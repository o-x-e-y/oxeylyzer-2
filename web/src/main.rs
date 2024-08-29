mod analyze;
mod dof;
mod layouts;
mod posts;
mod search;
mod util;

use layouts::{HeatmapData, LayoutsFolder};
use leptos::*;
use leptos_meta::provide_meta_context;
use leptos_router::*;
use strsim::jaro_winkler;

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
                <dof::RenderNamedDof name="noctum".to_string()></dof::RenderNamedDof>
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
    let possible_results = util::embedded_names::<LayoutsFolder>().collect();

    view! {
        <ul class="hidden w-full justify-end list-none sm:flex sm:gap-5">
            <NavElem text="Layouts" href="/layouts"/>
            <NavElem text="Posts" href="/posts"/>
            <NavElem text="Analyze" href="/analyze"/>
            <SearchBar possible_results/>
            <ToggleHeatmap/>
        // <GithubImage/>
        </ul>
    }
}

#[component]
fn SmallNav() -> impl IntoView {
    let possible_results = util::embedded_names::<LayoutsFolder>().collect();
    let (dots_clicked, set_dots_clicked) = create_signal(false);

    view! {
        <div class="flex gap-3 w-full justify-end sm:hidden">
            <SearchBar possible_results/>
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

#[component]
fn SearchBar(possible_results: Vec<String>) -> impl IntoView {
    view! {
        <div class="my-auto hidden md:block">
            <SearchBarElem possible_results=possible_results.clone() width="19ch"/>
        </div>
        <div class="my-auto md:hidden">
            <SmallSearchBar possible_results width="80%"/>
        </div>
    }
}

fn search<'a>(
    possible_results: &'a [String],
    search: &'a str,
) -> impl Iterator<Item = String> + 'a {
    let mut results = possible_results
        .into_iter()
        .map(|s| (jaro_winkler(&s, search), s))
        .filter(|(d, _)| *d >= 0.3)
        .collect::<Vec<_>>();

    results.sort_by(|(d, _), (d2, _)| d2.total_cmp(d));

    results.into_iter().take(10).map(|(_, s)| s.to_owned())
}

#[component]
fn SmallSearchBar(possible_results: Vec<String>, width: &'static str) -> impl IntoView {
    let mut next_search_id = 0;

    let (search_results, set_search_results) = create_signal(Vec::new());

    let (display_results, set_display_results) = create_signal(false);
    let (display_search, set_display_search) = create_signal(false);

    let input_ref = create_node_ref::<html::Input>();

    let mut new_search_result = move |r: String| {
        let res = (next_search_id, r);
        next_search_id += 1;
        res
    };

    let on_input_search = move |ev: ev::Event| {
        let current_search = event_target_value(&ev);
        if current_search.is_empty() {
            set_display_results(false);
            return;
        }

        let results = search(&possible_results, &current_search)
            .map(|r| new_search_result(r))
            .collect::<Vec<_>>();

        if !results.is_empty() {
            set_display_results(true);
            set_search_results(results);
        }
    };

    view! {
        <button
            class="hover:bg-hovered -mr-1 p-1 rounded-lg"
            on:click=move |_| {
                set_display_search(true);
                if let Some(node) = input_ref() {
                    let _ = node.focus();
                }
            }
        >

            <img class="h-6 w-auto text-lg" src="../public/images/search.svg" alt="Search"/>
        </button>
        <div hidden=move || !display_search() class="fixed inset-0 w-screen h-screen z-[9001]">
            <div class="h-[4.5rem] bg-header flex justify-center align-middle">
                <div style:width=width class="my-auto">
                    <label name="search layouts">
                        <input
                            node_ref=input_ref
                            on:input=on_input_search
                            on:focus=move |_| {
                                if !search_results().is_empty() {
                                    set_display_results(true)
                                }
                            }

                            on:blur=move |_| {
                                set_display_search(false);
                                set_display_results(false)
                            }

                            on:keydown=move |ev| {
                                if ev.key() == "Escape" {
                                    if let Some(input) = input_ref() {
                                        let _ = input.blur();
                                    }
                                }
                            }

                            type="text"
                            placeholder="Search layouts..."
                            class="
                            w-full h-8 pl-3 bg-darker border-2
                            border-white-[#ccc] rounded-full text-[#ccc]"
                        />
                    </label>
                    <ul
                        style:width=width
                        hidden=move || !display_results()
                        on:mousedown=move |ev| ev.prevent_default()
                        on:click=move |_| {
                            if let Some(node) = input_ref() {
                                let _ = node.blur();
                            }
                        }

                        class="
                        absolute z-[9001] mt-3 text-[#ccc] list-none bg-header border border-hovered rounded-lg"
                    >
                        <For
                            each=search_results
                            key=|result| result.0
                            children=move |(_, result)| {
                                view! { <SearchResult result/> }
                            }
                        />

                    </ul>
                </div>
            </div>
        </div>
    }
}

#[component]
fn SearchBarElem(possible_results: Vec<String>, width: &'static str) -> impl IntoView {
    let mut next_search_id = 0;

    let (search_results, set_search_results) = create_signal(Vec::new());
    let (display_results, set_display_results) = create_signal(false);

    let input_ref = create_node_ref::<html::Input>();

    let mut new_search_result = move |r: String| {
        let res = (next_search_id, r);
        next_search_id += 1;
        res
    };

    let on_input_search = move |ev: ev::Event| {
        let current_search = event_target_value(&ev);
        if current_search.is_empty() {
            set_display_results(false);
            return;
        }

        let results = search(&possible_results, &current_search)
            .map(|r| new_search_result(r))
            .collect::<Vec<_>>();

        if !results.is_empty() {
            set_display_results(true);
            set_search_results(results);
        }
    };

    view! {
        <div style:width=width class="my-auto">
            <label name="search layouts">
                <input
                    node_ref=input_ref
                    on:input=on_input_search
                    on:focus=move |_| {
                        if !search_results().is_empty() {
                            set_display_results(true)
                        }
                    }

                    on:blur=move |_| set_display_results(false)
                    on:keydown=move |ev| {
                        if ev.key() == "Escape" {
                            if let Some(input) = input_ref() {
                                let _ = input.blur();
                            }
                        }
                    }

                    type="text"
                    placeholder="Search layouts..."
                    class="
                    w-full h-8 pl-3 bg-darker border-2
                    border-white-[#ccc] rounded-full text-[#ccc]"
                />
            </label>
            <ul
                style:width=width
                hidden=move || !display_results()
                on:mousedown=move |ev| ev.prevent_default()
                on:click=move |_| {
                    if let Some(node) = input_ref() {
                        let _ = node.blur();
                    }
                }

                class="
                absolute z-[9001] mt-3 text-[#ccc] list-none bg-header border border-hovered rounded-lg"
            >
                <For
                    each=search_results
                    key=|result| result.0
                    children=move |(_, result)| {
                        view! { <SearchResult result/> }
                    }
                />

            </ul>
        </div>
    }
}

#[component]
fn SearchResult(result: String) -> impl IntoView {
    view! {
        <A href=format!("/layouts/{result}")>
            <li class="p-1 mx-1 hover:text-txt">{result}</li>
        </A>
    }
}
