mod analyze;
mod dof;
mod layouts;
mod posts;
mod search;
mod util;

use leptos::*;
use leptos_meta::provide_meta_context;
use leptos_router::*;

pub fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router trailing_slash=leptos_router::TrailingSlash::Redirect>
            <main class="text-lg text-txt">
                <Navigation/>
                <Routes>
                    <Route path="/" view=Home/>
                    <Route path="/layouts" view=layouts::LayoutsWrapper>
                        <Route
                            path=""
                            view=|| {
                                view! {
                                    <layouts::RenderLayoutLinks base_url="layouts"></layouts::RenderLayoutLinks>
                                }
                            }
                        />

                        <Route path=":name" view=layouts::RenderLayout/>
                    </Route>
                    <Route path="/analyze" view=layouts::LayoutsWrapper>
                        <Route
                            path=""
                            view=|| {
                                view! {
                                    <layouts::RenderLayoutLinks base_url="analyze"></layouts::RenderLayoutLinks>
                                }
                            }
                        />

                        <Route path=":name" view=analyze::RenderAnalyzer/>
                    </Route>
                    <Route path="/posts" view=|| view! { <Outlet /> }>
                        <Route path="" view=posts::RenderPostLinks/>
                        <Route path=":name" view=posts::RenderPost/>
                    </Route>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <div class="flex content-center">
            <div class="px-16 py-32 my-auto text-1xl grid gap-16 grid-cols-homepage">
                <div class="">
                    <p class="animate-fadein-1 opacity-0">
                        "An online keyboard layout analyzer and generator."
                    </p>
                    <p class="animate-fadein-2 opacity-0">"Created by Oxey"</p>
                </div>
                <div>
                    <dof::RenderNamedDof name="noctum".to_string()></dof::RenderNamedDof>
                </div>
            </div>
        </div>
    }
}

#[component]
fn Navigation() -> impl IntoView {
    view! {
        <header class="w-full bg-header container-inline-size">
            <div class="relative flex items-center p-4">
                <A class="ml-16 visited:text-[#ddd]" href="/">
                    <h1 class="pl-5 text-4xl">"Oxeylyzer 2"</h1>
                </A>
                <div class="relative flex items-center ml-auto mr-16">
                    <nav class="flex">
                        <NavElem text="Layouts" href="/layouts"/>
                        <NavElem text="Posts" href="/posts"/>
                        <NavElem text="Analyze" href="/analyze"/>
                        <Github/>
                    </nav>
                </div>
            </div>
        </header>
    }
}

#[component]
fn Github() -> impl IntoView {
    view! {
        <a class="pl-5 flex" href="https://github.com/o-x-e-y/oxeylyzer-2">
            <div class="p-1 hover:bg-[#ffffff10] rounded-full my-auto">
                <img class="h-6 w-auto" src="../public/images/github-logo.svg" alt="Github"/>
            </div>
        </a>
    }
}
#[component]
fn NavElem(text: &'static str, href: &'static str) -> impl IntoView {
    view! {
        <A class="text-[#ccc] visited:text-[#ccc]" href>
            <div class="pl-5 text-2xl">
                <div class="p-1 hover:bg-[#ffffff10] rounded-lg">{text}</div>
            </div>
        </A>
    }
}
