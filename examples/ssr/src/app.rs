use leptos::{
    component,
    tracing,
    update,
    view,
    Callback,
    IntoView,
    RwSignal,
    SignalUpdate,
    StoredValue,
};
use leptos_animated_for::AnimatedFor;
use leptos_meta::{provide_meta_context, Link, Stylesheet, Title};
use leptos_router::{Route, Router, Routes};
use rand::{seq::SliceRandom, thread_rng, Rng};

use crate::button::Button;

const INITIAL_ITEM_COUNT: usize = 40;

#[component]
fn Home() -> impl IntoView {
    let next_item = StoredValue::new(INITIAL_ITEM_COUNT);

    let items = RwSignal::new((0..INITIAL_ITEM_COUNT).collect::<Vec<_>>());

    let create_new_item = Callback::new(move |()| {
        let item = next_item();
        next_item.set_value(item + 1);
        item
    });

    let rng = StoredValue::new(thread_rng());

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
                items.insert(rng.gen_range(0..items.len()), item);
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
        <Title text="Leptos + Tailwindcss + AnimatedFor"/>
        <main class="min-h-screen py-8 bg-gray-800 font-bold text-white">
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
                    key=|item| *item
                    children=|item| {
                        view! {
                            <div class="rounded flex items-center justify-center bg-blue-500/70">
                                {item}
                            </div>
                        }
                    }

                    enter_from_class="opacity-0"
                    enter_class="duration-1000"
                    move_class="duration-1000"
                    leave_class="opacity-0 duration-1000"
                />

            </div>
        </main>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/app.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <Routes>
                <Route path="" view=move || view! { <Home/> }/>
            </Routes>
        </Router>
    }
}
