use leptos::{MaybeProp, Memo, SignalGet, SignalWithUntracked};

use crate::animated_el::AnimatedEl;

type Classes = Vec<String>;

#[derive(Clone, Copy)]
pub struct UntrackedClasses {
    enter_from: Memo<Classes>,
    enter: Memo<Classes>,
    r#move: Memo<Classes>,
    leave: Memo<Classes>,
}

fn use_class_memo(class: MaybeProp<String>) -> Memo<Classes> {
    Memo::new(move |_| {
        class
            .get()
            .map(|class| class.split_whitespace().map(ToOwned::to_owned).collect())
            .unwrap_or_default()
    })
}

impl UntrackedClasses {
    pub fn new(
        raw_enter_from: MaybeProp<String>,
        raw_enter: MaybeProp<String>,
        raw_move: MaybeProp<String>,
        raw_leave: MaybeProp<String>,
    ) -> Self {
        let enter_from = use_class_memo(raw_enter_from);
        Self {
            enter_from,
            enter: use_class_memo(raw_enter),
            r#move: use_class_memo(raw_move),
            leave: use_class_memo(raw_leave),
        }
    }

    pub fn add_enter_from(&self, el: &web_sys::HtmlElement) -> Classes {
        self.enter_from
            .with_untracked(|classes| el.add_unique_classes(classes))
    }

    pub fn add_enter(&self, el: &web_sys::HtmlElement) -> Classes {
        self.enter
            .with_untracked(|classes| el.add_unique_classes(classes))
    }

    pub fn add_move(&self, el: &web_sys::HtmlElement) -> Classes {
        self.r#move
            .with_untracked(|classes| el.add_unique_classes(classes))
    }

    pub fn add_leave(&self, el: &web_sys::HtmlElement) {
        self.leave.with_untracked(|classes| el.add_classes(classes));
    }
}
