use futures::channel;
use leptos::{document, request_animation_frame, window, View};
use wasm_bindgen::JsCast;
use web_sys::DomRect;

use crate::animated_el::AnimatedEl;

pub fn force_reflow() {
    _ = window().document().unwrap().body().unwrap().offset_height();
}

pub async fn next_tick() {
    let (tx, rx) = channel::oneshot::channel();

    request_animation_frame(move || {
        tx.send(()).unwrap();
    });

    rx.await.unwrap();
}

pub fn extract_el_from_view(view: &View) -> Option<web_sys::HtmlElement> {
    match view {
        View::Component(component) => {
            let node_view = component.children.first()?.clone();

            let el = node_view
                .into_html_element()
                .ok()?
                .dyn_ref::<web_sys::HtmlElement>()?
                .clone();

            Some(el)
        }
        view => {
            let el = view
                .clone()
                .into_html_element()
                .ok()?
                .dyn_ref::<web_sys::HtmlElement>()?
                .clone();

            Some(el)
        }
    }
}

fn lock_fixed_position(
    el: &web_sys::HtmlElement,
    el_pos: &DomRect,
    document_pos: &DomRect,
) {
    let top = el_pos.top() - document_pos.top();
    let left = el_pos.left() - document_pos.left();
    let width = el_pos.width();
    let height = el_pos.height();

    el.set_important_style_property("position", "fixed");
    el.set_important_style_property("margin", "0px");
    el.set_important_style_property("top", &format!("{top}px"));
    el.set_important_style_property("left", &format!("{left}px"));
    el.set_important_style_property("width", &format!("{width}px"));
    el.set_important_style_property("height", &format!("{height}px"));

    el.enable_instant_transition();
}

pub fn prepare_leave(
    leaving_els_parent: &web_sys::Element,
    leaving_els_with_rects: &Vec<(web_sys::HtmlElement, DomRect)>,
) {
    let document_pos = document()
        .document_element()
        .expect("document to be Element")
        .get_bounding_client_rect();

    for (el, rect) in leaving_els_with_rects {
        lock_fixed_position(el, rect, &document_pos);
        leaving_els_parent.append_child(el).unwrap();
    }
}

pub fn check_if_moved_and_lock_previous_position(
    el: &web_sys::HtmlElement,
    old_pos: &DomRect,
) -> bool {
    let new_pos = el.get_bounding_client_rect();

    let dx = old_pos.left() - new_pos.left();
    let dy = old_pos.top() - new_pos.top();

    if dx != 0.0 || dy != 0.0 {
        el.set_important_style_property(
            "transform",
            &format!("translate({dx}px,{dy}px)"),
        );
        el.enable_instant_transition();

        return true;
    }

    false
}
