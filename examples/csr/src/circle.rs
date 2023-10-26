use leptos::{component, view, Children, IntoView, MaybeProp};

#[component]
pub fn Circle(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let class = move || {
        [
            "rounded-full flex items-center justify-center".to_owned(),
            class().unwrap_or_default(),
        ]
        .join(" ")
    };

    view! { <div class=class>{children()}</div> }
}
