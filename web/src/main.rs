mod analyze;
mod dof;
mod keyboard;
mod layouts;
mod posts;
mod search;
mod util;

use leptos::*;
use leptos_router::*;
use libdof::Dof;
use rust_embed::Embed;
use util::*;

pub fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router trailing_slash=leptos_router::TrailingSlash::Redirect>
            <main class="text-lg text-[#ddd]">
                <Navigation/>
                <Routes>
                    <Route path="/" view=Home/>
                    <Route path="/layouts" view=LayoutsWrapper>
                        <Route
                            path=""
                            view=|| {
                                view! { <layouts::Layouts base_url="layouts"></layouts::Layouts> }
                            }
                        />

                        <Route path=":name" view=layouts::RenderLayout/>
                    </Route>
                    <Route path="/analyze" view=LayoutsWrapper>
                        <Route
                            path=""
                            view=|| {
                                view! { <layouts::Layouts base_url="analyze"></layouts::Layouts> }
                            }
                        />

                        <Route path=":name" view=analyze::RenderAnalyzer/>
                    </Route>
                // <Route path="/posts" view=LayoutsWrapper>
                // <Route path="" view=Home/>
                // <Route path=":name" view=posts::RenderPost/>
                // </Route>
                    </Routes>
            </main>
        </Router>
    }
}

#[derive(Embed)]
#[folder = "./public/posts"]
#[include = "*.md"]
struct PostsFolder;

#[component]
fn Home() -> impl IntoView {
    view! {
        <div class="absolute top-0 w-full h-full bg-darker -z-50"></div>
        <div class="flex content-center bg-darker">
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
        <header class="w-full bg-background container-inline-size">
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

#[component]
fn LayoutsWrapper() -> impl IntoView {
    view! {
        <div style="margin-top: 1cqw"></div>
        <Outlet/>
    }
}
