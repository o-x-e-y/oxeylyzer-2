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
        >
            <nav class=css::nav>
                {
                    let loc = use_location();
                    let href = move || format!("{}/..", loc.pathname.get());
                    
                    view! {
                        <A href>
                            <h3>{"Go Back"}</h3>
                        </A>
                    }
                }
            </nav>
            <Routes>
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
    view! {
        <div>"Home page"</div>
        <div style="margin-top: 1cqw">
            <A href="/layouts">{"layouts"}</A>
        </div>
    }
}

#[component]
fn LayoutsWrapper() -> impl IntoView {
    view! {
        <div style="margin-top: 1cqw"></div>
        <Outlet/>
    }
}