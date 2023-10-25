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
};
use rand::{seq::SliceRandom, thread_rng, Rng};

struct Item {
    id: usize,
}

const INITIAL_ITEM_COUNT: usize = 20;

#[component]
fn App() -> impl IntoView {
    let last_added_item_id = StoredValue::new(INITIAL_ITEM_COUNT);
    let items = RwSignal::new(
        (1..INITIAL_ITEM_COUNT)
            .map(|id| Item { id })
            .collect::<Vec<_>>(),
    );

    let rng = StoredValue::new(thread_rng());

    let create_new_item = Callback::new(move |()| {
        let id = last_added_item_id();
        last_added_item_id.set_value(id + 1);
        Item { id }
    });

    let shuffle = move || {
        items.update(|items| {
            rng.update_value(|rng| {
                items.shuffle(rng);
            });
        });
    };

    let add_start = {
        move || {
            update!(|items| {
                items.insert(0, create_new_item(()));
            });
        }
    };

    let add_end = {
        move || {
            update!(|items| {
                items.push(create_new_item(()));
            });
        }
    };

    let add_random = {
        move || {
            items.update(|items| {
                rng.update_value(|rng| {
                    items.insert(
                        rng.gen_range(0..items.len()),
                        create_new_item(()),
                    );
                });
            });
        }
    };

    let remove_start = {
        move || {
            update!(|items| {
                items.remove(0);
            });
        }
    };

    let remove_end = {
        move || {
            update!(|items| {
                items.pop();
            });
        }
    };

    let remove_random = {
        move || {
            items.update(|items| {
                rng.update_value(|rng| {
                    items.remove(rng.gen_range(0..items.len()));
                });
            });
        }
    };

    view! { <div></div> }
}

fn main() {
    mount_to_body(App);
}
