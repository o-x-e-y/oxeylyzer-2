mod analyze;
mod dof;
mod layouts;
mod posts;
mod search;
mod util;

use layouts::HeatmapData;
use leptos::*;
use leptos_meta::provide_meta_context;
use leptos_router::*;

pub fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(App)
}

#[derive(Clone, Copy, Debug)]
struct EnableHeatmap(RwSignal<bool>);

#[component]
fn App() -> impl IntoView {
    provide_meta_context();

    let heatmap_data = create_resource(
        move || "/public/heatmap_data.json".into(),
        util::load_json::<HeatmapData>,
    );

    let enable_heatmap = EnableHeatmap(create_rw_signal(true));

    provide_context(heatmap_data);
    provide_context(enable_heatmap);

    view! {
        <Router trailing_slash=leptos_router::TrailingSlash::Redirect>
            <main class="text-txt">
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
                    <Route path="/posts" view=|| view! { <Outlet/> }>
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
        <div class="w-full h-1/2 my-auto flex content-center">
            <div class="w-11/12 h-1/2 mx-auto text-1xl sm:grid sm:gap-16 sm:grid-cols-homepage">
                <div class="w-full mt-12">
                    <p>"An online keyboard layout analyzer and generator."</p>
                // <div class="br md:w-2/3 flex">
                // <div class="br my-10 mx-auto">
                // "Get started"
                // </div>
                // </div>
                </div>
                <div class="w-full mt-12">
                    <dof::RenderNamedDof name="noctum".to_string()></dof::RenderNamedDof>
                </div>
            </div>
        </div>
    }
}

#[component]
fn Navigation() -> impl IntoView {
    let (dots_clicked, set_dots_clicked) = create_signal(false);

    view! {
        <header class="w-full bg-header p-4">
            <nav class="w-[97%] m-auto flex">
                <A class="visited:text-txt" href="/">
                    <h1 class="text-4xl min-w-[11ch] text-center">"Oxeylyzer 2"</h1>
                </A>
                <div class="hidden sm:flex sm:w-full justify-right">
                    <div class="
                    w-full flex flex-col sm:gap-4 sm:flex-row sm:items-center
                    sm:justify-end sm:mt-0 sm:pl-5">
                        <NavElem text="Layouts" href="/layouts"/>
                        <NavElem text="Posts" href="/posts"/>
                        <NavElem text="Analyze" href="/analyze"/>
                        <Github/>
                        <ToggleHeatmap/>
                    </div>
                </div>
                <div class="sm:hidden flex w-full justify-end my-auto max-h-fit">
                    <button
                        class="p-1 mr-1 hover:bg-hovered rounded-lg"
                        on:click=move |_| set_dots_clicked(true)
                    >
                        <img class="h-6 w-auto" src="../public/images/three-dots.svg" alt="Menu"/>
                    </button>
                    <ToggleHeatmap/>
                </div>
                <div
                    class="fixed inset-0 bg-black/20 z-[9000] backdrop-blur-sm"
                    hidden=move || !dots_clicked()
                    on:click=move |_| set_dots_clicked(false)
                >
                    <div class="flex justify-end">
                        <div class="bg-header rounded-xl m-4 px-5 py-4 w-5/12">
                            <ThreeDotsElem text="Layouts" href="/layouts"/>
                            <ThreeDotsElem text="Posts" href="/posts"/>
                            <ThreeDotsElem text="Analyze" href="/analyze"/>
                            <ThreeDotsElem
                                text="Github"
                                href="https://github.com/o-x-e-y/oxeylyzer-2"
                            />
                        </div>
                    </div>
                </div>
            </nav>
        </header>
    }
}

#[component]
fn Github() -> impl IntoView {
    view! {
        <a class="flex" href="https://github.com/o-x-e-y/oxeylyzer-2">
            <div class="p-1 hover:bg-hovered rounded-full my-auto">
                <img class="h-6 w-auto" src="../public/images/github-logo.svg" alt="Github"/>
            </div>
        </a>
    }
}

#[component]
fn NavElem(text: &'static str, href: &'static str) -> impl IntoView {
    view! {
        <A class="text-xl text-[#ccc] visited:text-[#ccc]" href>
            <div class="hover:bg-hovered rounded-lg">{text}</div>
        </A>
    }
}

#[component]
fn ThreeDotsElem(text: &'static str, href: &'static str) -> impl IntoView {
    view! {
        <A class="text-xl text-txt visited:text-txt" href>
            <div class="hover:bg-hovered rounded-md">{text}</div>
        </A>
    }
}

#[component]
fn ToggleHeatmap() -> impl IntoView {
    let enable_heatmap = expect_context::<EnableHeatmap>().0;

    let gradient = move || match enable_heatmap() {
        true => "bg-heatmap-gradient",
        false => "bg-fingermap-gradient",
    };

    let style = move || format!("{} w-8 h-8 rounded-[0.25rem]", gradient());

    view! { <button class=style on:click=move |_| enable_heatmap.update(|b| *b = !*b)></button> }
}
