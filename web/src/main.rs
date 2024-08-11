mod components;
mod logger;

use components::*;
use ev::SubmitEvent;
use leptos::*;
use log::*;
use logger::*;

#[component]
pub fn App() -> impl IntoView {
    let message = create_rw_signal(String::new());
    let show = create_rw_signal(false);
    let level = create_rw_signal(Level::Info);
    let logger = Box::new(Logger::new(3000, show, level, message));
    set_boxed_logger(logger).unwrap();
    set_max_level(LevelFilter::Warn);

    let username = create_rw_signal(String::new());
    let password = create_rw_signal(String::new());
    let (islogged, set_islogged) = create_signal(false);

    view! {
        <div class="flex items-center justify-center min-h-screen bg-gray-100">
            <Popup level=level show=show message=message />
            <Show when=move || !islogged() fallback=move || view! { <Main /> }>
                <form
                    class="w-full max-w-sm"
                    on:submit=move |ev: SubmitEvent| {
                        ev.prevent_default();
                        warn!("Login: {}", username());
                    }
                >
                    <div class="bg-white p-8 rounded-lg shadow-lg space-y-4">
                        <Input
                            elemtype="tel"
                            placeholder="Account"
                            maxlength=11
                            value=username
                            required=true
                        />
                        <Input
                            elemtype="password"
                            placeholder="Password"
                            maxlength=16
                            value=password
                            required=true
                        />
                        <Button elemtype="submit">Login</Button>
                    </div>
                </form>
            </Show>
        </div>
    }
}

#[component]
fn Main() -> impl IntoView {
    view! { <h1>Welcome</h1> }
}

fn main() {
    mount_to_body(|| view! { <App /> })
}
