mod analyze;
mod layouts;
mod posts;
mod search;
mod util;

use search::*;

use layouts::{HeatmapData, LayoutsFolder};
use leptos::*;
use leptos_meta::provide_meta_context;
use leptos_router::*;

pub fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(App)
}

#[derive(Clone, Copy, Debug)]
struct EnableHeatmap(RwSignal<bool>);

#[derive(Clone, Debug)]
struct LayoutNames(Vec<String>);

#[component]
fn App() -> impl IntoView {
    provide_meta_context();

    let heatmap_data = create_resource(
        move || "/public/heatmap_data.json".into(),
        util::load_json::<HeatmapData>,
    );

    let enable_heatmap = EnableHeatmap(create_rw_signal(true));

    let layout_names = LayoutNames(util::embedded_names::<LayoutsFolder>().collect::<Vec<_>>());

    provide_context(heatmap_data);
    provide_context(enable_heatmap);
    provide_context(layout_names);

    view! {
        <Router trailing_slash=leptos_router::TrailingSlash::Redirect>
            <main class="text-txt">
                <Navigation/>
                <Routes>
                    <Route path="/" view=Home/>
                    <Route path="/layouts" view=layouts::LayoutsWrapper>
                        <Route
                            path=""
                            view=move || {
                                let names = move || expect_context::<LayoutNames>().0;
                                view! {
                                    <div class="m-6">
                                        <layouts::RenderLayoutLinks names></layouts::RenderLayoutLinks>
                                    </div>
                                }
                            }
                        />

                        <Route path=":name" view=analyze::RenderAnalyzer/>
                    </Route>
                    <Route path="/posts" view=|| view! { <Outlet/> }>
                        <Route path="" view=posts::RenderPostLinks/>
                        <Route path=":name" view=posts::RenderPost/>
                    </Route>
                    <Route path="/search/:query" view=search::QuerySearch/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <div class="
        w-11/12 mx-auto text-1xl flex flex-col-reverse py-8 sm:grid sm:grid-cols-homepage
        sm:gap-6 sm:py-12">
            <div class="w-full">
                <p class="text-center sm:px-4 max-sm:py-6">
                    "An online keyboard layout analyzer and generator."
                </p>
            // <p class="flex justify-center">
            // <button
            // class="sm:my-8 p-6 bg-header rounded-2xl"
            // >
            // <p class="text-lg">"Get started"</p>
            // <p class="text-[#00caca]">"Analyze dvorak"</p>
            // </button>
            // </p>
            </div>
            <div class="w-full">
                <layouts::RenderNamedDof name="noctum".to_string()></layouts::RenderNamedDof>
            </div>
        </div>
    }
}

#[component]
fn Navigation() -> impl IntoView {
    view! {
        <header class="w-full bg-header">
            <nav class="flex p-4 pr-5 sm:pl-8">
                <A class="visited:text-txt text-nowrap" href="/">
                    <h1 class="text-4xl">"Oxeylyzer\u{00A0}2"</h1>
                </A>
                <NormalNav/>
                <SmallNav/>
            </nav>
        </header>
    }
}

#[component]
fn NormalNav() -> impl IntoView {
    let possible_results = expect_context::<LayoutNames>().0;

    view! {
        <ul class="hidden w-full justify-end list-none sm:flex sm:gap-5">
            <NavElem text="Posts" href="/posts"/>
            <NavElem text="Layouts" href="/layouts"/>
            <NavSearch possible_results/>
            // <GithubImage/>
            <ToggleHeatmap/>
        </ul>
    }
}

#[component]
fn SmallNav() -> impl IntoView {
    let possible_results = expect_context::<LayoutNames>().0;
    let (dots_clicked, set_dots_clicked) = create_signal(false);

    view! {
        <div class="flex gap-3 w-full justify-end sm:hidden">
            <NavSearch possible_results/>
            <button
                class="hover:bg-hovered py-1 rounded-lg"
                on:click=move |_| set_dots_clicked(true)
            >
                <img class="h-6 w-auto text-lg" src="../public/images/three-dots.svg" alt="Menu"/>
            </button>
            <ToggleHeatmap/>
        </div>
        <div
            class="fixed inset-0 w-screen h-screen bg-black/20 z-[9001] backdrop-blur-sm"
            hidden=move || !dots_clicked()
            on:click=move |_| set_dots_clicked(false)
        >
            <div class="flex justify-end">
                <ul class="m-4 p-4 pr-16 bg-header rounded-xl list-none">
                    <NavElem text="Layouts" href="/layouts"/>
                    <NavElem text="Posts" href="/posts"/>
                    <NavElem text="Analyze" href="/analyze"/>
                    <NavElem text="Github" href="https://github.com/o-x-e-y/oxeylyzer-2"/>
                </ul>
            </div>
        </div>
    }
}

#[component]
fn NavElem(text: &'static str, href: &'static str) -> impl IntoView {
    view! {
        <A class="my-auto text-xl text-[#ccc] visited:text-[#ccc] hover:text-txt" href>
            <ul>{text}</ul>
        </A>
    }
}

#[component]
fn GithubImage() -> impl IntoView {
    view! {
        <a
            class="my-auto hover:bg-hovered rounded-full"
            href="https://github.com/o-x-e-y/oxeylyzer-2"
        >
            <img class="h-7 w-auto" src="../public/images/github-logo.svg" alt="Github"/>
        </a>
    }
}

#[component]
fn ToggleHeatmap() -> impl IntoView {
    let enable_heatmap = expect_context::<EnableHeatmap>().0;

    let gradient = move || match enable_heatmap() {
        true => "bg-heatmap-gradient",
        false => "bg-fingermap-gradient",
    };

    let style = move || format!("{} my-auto w-8 h-8 rounded-[0.25rem]", gradient());

    view! { <button class=style on:click=move |_| enable_heatmap.update(|b| *b = !*b)></button> }
}