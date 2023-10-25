use std::{collections::HashMap, hash::Hash, rc::Rc};

use leptos::{MaybeProp, StoredValue};
use wasm_bindgen::{prelude::Closure, JsCast};

use super::untracked_classes::UntrackedClasses;
use crate::animated_el::AnimatedEl;

fn set_cb_on_transition_end(el: &web_sys::HtmlElement, cb: &Rc<impl Fn() + 'static>) {
    let closure = Closure::<dyn FnMut(&web_sys::TransitionEvent)>::wrap(Box::new({
        let cb = Rc::clone(cb);
        move |_| cb()
    }));

    el.set_ontransitionend(Some(closure.as_ref().unchecked_ref()));
    el.set_onanimationend(Some(closure.as_ref().unchecked_ref()));
    closure.forget();
}

#[derive(Clone)]
pub struct Animator<Key>
where
    Key: 'static,
{
    classes: UntrackedClasses,
    transition_end_cbs: StoredValue<Vec<Rc<dyn Fn()>>>,
    enter_from_class_per_key: StoredValue<HashMap<Key, Vec<String>>>,
}

impl<Key> Copy for Animator<Key> where Key: Clone {}

impl<Key> Animator<Key>
where
    Key: Clone + Hash + Eq + PartialEq,
{
    pub fn new(
        raw_enter_from: MaybeProp<String>,
        raw_enter: MaybeProp<String>,
        raw_move: MaybeProp<String>,
        raw_leave: MaybeProp<String>,
    ) -> Self {
        Self {
            classes: UntrackedClasses::new(raw_enter_from, raw_enter, raw_move, raw_leave),
            transition_end_cbs: StoredValue::new(Vec::new()),
            enter_from_class_per_key: StoredValue::new(HashMap::new()),
        }
    }

    pub fn prepare_enter(&self, key: &Key, el: &web_sys::HtmlElement) {
        el.enable_instant_transition();
        let added_classes = self.classes.add_enter_from(el);
        self.enter_from_class_per_key.update_value(|v| {
            v.insert(key.clone(), added_classes);
        });
    }

    fn push_class_cleanup_on_transition_end(
        &self,
        el: &web_sys::HtmlElement,
        classes_to_remove: Vec<String>,
    ) {
        let clear_classes = Rc::new({
            let el = el.clone();
            move || {
                el.remove_classes(&classes_to_remove);
                el.set_ontransitionend(None);
                el.set_onanimationend(None);
            }
        });

        set_cb_on_transition_end(el, &clear_classes);

        self.transition_end_cbs.update_value(|v| {
            v.push(clear_classes);
        });
    }

    pub fn start_enter(&self, key: &Key, el: &web_sys::HtmlElement) {
        self.enter_from_class_per_key.update_value(|v| {
            let classes_to_remove = v.remove(key);

            if let Some(classes_to_remove) = classes_to_remove {
                el.remove_classes(&classes_to_remove);
            }
        });

        el.disable_instant_transition();

        let added_classes = self.classes.add_enter(el);
        self.push_class_cleanup_on_transition_end(el, added_classes);
    }

    pub fn start_move(&self, el: &web_sys::HtmlElement) {
        el.disable_instant_transition();
        el.clear_transform();

        let added_classes = self.classes.add_move(el);
        self.push_class_cleanup_on_transition_end(el, added_classes);
    }

    pub fn start_leave(&self, el: &web_sys::HtmlElement) {
        self.classes.add_leave(el);
        el.disable_instant_transition();

        let remove_el = Rc::new({
            let el = el.clone();
            move || el.remove()
        });

        set_cb_on_transition_end(el, &remove_el);
    }

    pub fn clear_transitions(&self) {
        self.transition_end_cbs.update_value(|v| {
            for cb in v.drain(..) {
                cb();
            }
        });
    }
}
