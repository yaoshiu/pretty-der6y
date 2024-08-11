use std::{collections::VecDeque, time::Duration};

use leptos::*;
use log::Level;
use wasm_bindgen::JsCast;
use web_sys::HtmlButtonElement;

#[component]
pub fn Input(
    #[prop(default = "text")] elemtype: &'static str,
    #[prop(optional)] placeholder: &'static str,
    #[prop(optional)] required: bool,
    #[prop(optional)] maxlength: Option<usize>,
    value: RwSignal<String>,
) -> impl IntoView {
    view! {
        <input
            class="w-full px-4 py-2 border rounded-lg focus:ring-2 focus:ring-blue-500 focus:outline-none"
            type=elemtype
            placeholder=placeholder
            prop:value=value
            required=required
            maxlength=maxlength
            on:input=move |ev| value.set(event_target_value(&ev))
        />
    }
}

#[component]
pub fn Button(
    #[prop(default = "button")] elemtype: &'static str,
    #[prop(default=|_|())] onclick: fn(ev::MouseEvent),
    #[prop(optional)] disabled: bool,
    children: Children,
) -> impl IntoView {
    #[derive(Clone, Copy, PartialEq)]
    struct Ripple {
        x: i32,
        y: i32,
        id: u32,
        size: i32,
    }

    let (ripples, set_ripples) = create_signal(VecDeque::<Ripple>::new());

    let mut id = 0;

    let on_click = move |ev: ev::MouseEvent| {
        if let Some(target) = ev.target() {
            if let Some(element) = target.dyn_ref::<HtmlButtonElement>() {
                let rect = element.get_bounding_client_rect();
                let size = rect.width().max(rect.height()) as i32;
                let x = ev.client_x() - rect.left() as i32 - size / 2;
                let y = ev.client_y() - rect.top() as i32 - size / 2;

                let ripple = Ripple { x, y, id, size };
                id += 1;

                set_ripples.update(|ripples| ripples.push_back(ripple));

                set_timeout(
                    move || {
                        set_ripples.update(|ripples| {
                            if ripples.front() == Some(&ripple) {
                                ripples.pop_front();
                            }
                        });
                    },
                    Duration::from_millis(1000),
                );
            }
        }
        onclick(ev);
    };

    view! {
        <button
            class="w-full bg-blue-500 text-white py-2 rounded-lg hover:bg-blue-600 relative overflow-hidden"
            type=elemtype
            prop:disabled=disabled
            on:click=on_click
        >
            {children()}
            <For
                each=ripples
                key=|ripple| ripple.id
                children=move |Ripple { x, y, id: _, size }| {
                    view! {
                        <span
                            class="ripple"
                            style:top=move || format!("{}px", y)
                            style:left=move || format!("{}px", x)
                            style:width=move || format!("{}px", size)
                            style:height=move || format!("{}px", size)
                        />
                    }
                }
            />
        </button>
    }
}

#[component]
pub fn Popup(
    level: impl Fn() -> Level + 'static,
    message: impl Fn() -> String + 'static,
    show: RwSignal<bool>,
) -> impl IntoView {
    view! {
        <Show when=show>
            <div
                class="fixed flex justify-between items-center rounded-lg text-sm text-white top-4 left-1/2 transform -translate-x-1/2 px-4 py-2 shadow-lg z-50"
                class=("bg-blue-500", level() == Level::Info)
                class=("bg-yellow-500", level() == Level::Warn)
                class=("bg-red-500", level() == Level::Error)
            >
                {message()}
                <button
                    class="ml-4 text-xl text-white font-bold focus:outline-none"
                    on:click=move |_| show.set(false)
                >
                    "×"
                </button>
            </div>
        </Show>
    }
}
