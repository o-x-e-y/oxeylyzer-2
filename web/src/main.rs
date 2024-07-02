mod keyboard;
mod util;

use leptos::*;
use leptos_router::*;
use stylance::import_crate_style;

import_crate_style!(css, "./css/main.module.css");

pub fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router
            // trailing_slash=leptos_router::TrailingSlash::Redirect
            base="/oxeylyzer-2"
        >
            <nav class=css::nav>
                <A href="/oxeylyzer-2">
                    <h3>{"Go Home"}</h3>
                </A>
            </nav>
            <Routes
                base="/oxeylyzer-2".to_string()
            >
                <Route path="/" view=Home/>
                <Route path="/layouts" view=LayoutsWrapper>
                    <Route path="" view=keyboard::Layouts/>
                    <Route path=":name" view=keyboard::Layout/>
                </Route>
            </Routes>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    "home"
}

#[component]
fn LayoutsWrapper() -> impl IntoView {
    view! {
        // <h1>Layouts</h1>
        <div style="margin-top: 1cqw"></div>
        <Outlet/>
    }
}