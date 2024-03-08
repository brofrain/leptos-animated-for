mod button;
mod circle;

use leptos::{
    component,
    mount_to_body,
    update,
    view,
    Callback,
    IntoView,
    RwSignal,
    SignalUpdate,
    StoredValue,
    View,
};
use leptos_animated_for::AnimatedFor;
use rand::{seq::SliceRandom, thread_rng, Rng};

use crate::{button::Button, circle::Circle};

#[derive(Clone)]
struct Item {
    id: usize,
    view: View,
}

const INITIAL_ITEM_COUNT: usize = 40;

#[component]
fn App() -> impl IntoView {
    let last_added_item_id = StoredValue::new(0);

    let rng = StoredValue::new(thread_rng());

    let create_new_item = Callback::new(move |()| {
        let id = last_added_item_id();
        last_added_item_id.set_value(id + 1);

        let mut circle = None;
        let mut bg_color_class = None;

        rng.update_value(|rng| {
            circle = Some(matches!(rng.gen_range(0..=1), 0));

            bg_color_class = Some(match rng.gen_range(0..=2) {
                0 => "bg-red-500/70",
                1 => "bg-green-500/70",
                _ => "bg-blue-500/70",
            });
        });

        let view = if circle.unwrap() {
            view! { <Circle class=bg_color_class.unwrap()>{id}</Circle> }
                .into_view()
        } else {
            view! {
                <div class=["rounded flex items-center justify-center", bg_color_class.unwrap()]
                    .join(" ")>{id}</div>
            }
            .into_view()
        };

        Item { id, view }
    });

    let items = RwSignal::new(
        (0..INITIAL_ITEM_COUNT)
            .map(|_| create_new_item(()))
            .collect::<Vec<_>>(),
    );

    let shuffle = move |_| {
        items.update(|items| {
            rng.update_value(|rng| {
                items.shuffle(rng);
            });
        });
    };

    let add_start = move |_| {
        update!(|items| {
            items.insert(0, create_new_item(()));
        });
    };

    let add_end = move |_| {
        update!(|items| {
            items.push(create_new_item(()));
        });
    };

    let add_random = move |_| {
        items.update(|items| {
            let item = create_new_item(());
            rng.update_value(|rng| {
                items.insert(rng.gen_range(0..=items.len()), item);
            });
        });
    };

    let remove_start = move |_| {
        update!(|items| {
            if items.is_empty() {
                return;
            }

            items.remove(0);
        });
    };

    let remove_end = move |_| {
        update!(|items| {
            items.pop();
        });
    };

    let remove_random = move |_| {
        items.update(|items| {
            if items.is_empty() {
                return;
            }

            rng.update_value(|rng| {
                items.remove(rng.gen_range(0..items.len()));
            });
        });
    };

    view! {
        <div class="min-h-screen py-8 bg-gray-800 font-bold text-white">
            <div class="flex flex-wrap gap-2 justify-center">
                <Button on:click=add_start>{"Add start"}</Button>
                <Button on:click=add_end>{"Add end"}</Button>
                <Button on:click=add_random>{"Add random"}</Button>
                <Button on:click=remove_start>{"Remove start"}</Button>
                <Button on:click=remove_end>{"Remove end"}</Button>
                <Button on:click=remove_random>{"Remove random"}</Button>
                <Button on:click=shuffle>{"Shuffle"}</Button>
            </div>

            <div class="mt-4 grid grid-cols-[repeat(5,auto)] gap-2 justify-center text-xl [&>*]:w-16 [&>*]:h-16">
                <AnimatedFor
                    each=items
                    key=|item| item.id
                    children=|item| item.view
                    enter_from_class="opacity-0"
                    enter_class="animate-fade-in-bottom-left"
                    move_class="duration-1000"
                    leave_class="opacity-0 duration-1000"
                    appear=true
                />

            </div>
        </div>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
