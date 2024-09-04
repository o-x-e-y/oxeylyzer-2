use leptos::*;
use leptos_router::*;
use strsim::jaro_winkler;

use crate::{
    layouts::{LayoutLinks, LayoutsFolder},
    util::embedded_names,
    LayoutNames,
};

#[derive(Debug, Clone, Copy)]
struct DisplaySearch(RwSignal<bool>);

#[component]
pub fn QuerySearch() -> impl IntoView {
    let params = use_params_map();
    let query = move || params.with(|p| p.get("query").cloned().unwrap_or_default());

    let possible_results = move || match use_context::<LayoutNames>() {
        Some(names) => names.0,
        None => embedded_names::<LayoutsFolder>(),
    };

    let names = create_memo(move |_| search(&possible_results(), &query(), 24));

    view! {
        <div class="mx-4">
            {move || match names().is_empty() {
                false => view! {
                    <p class="text-2xl text-center py-4">"Layouts matching '" {query} "'"</p>
                    <LayoutLinks names/>
                }.into_view(),
                true => view! {
                    <p class="text-2xl text-center py-4">"No matches for '" {query} "' :("</p>
                }.into_view()
            }}
        </div>
    }

    // view! {
    //     <div class="mx-4">
    //         {move || match names().is_empty() {
    //             false => {
    //                 view! {
    //                     <p class="text-2xl text-center py-4">"Layouts matching '" {query} "'"</p>
    //                     <LayoutLinks names/>
    //                 }
    //                     .into_view()
    //             }
    //             true => {
    //                 view! {
    //                     <p class="text-2xl text-center py-4">"No matches for '" {query} "' :("</p>
    //                 }
    //                     .into_view()
    //             }
    //         }}

    //     </div>
    // }
}

#[component]
pub fn NavSearch(possible_results: Vec<String>) -> impl IntoView {
    let display_search = create_rw_signal(false);

    provide_context(DisplaySearch(display_search));

    let is_window_md = leptos_use::use_media_query("(min-width: 768px)");

    view! {
        <div class="my-auto">
            {move || {
                if is_window_md() {
                    view! {
                        <div class="hidden md:block">
                            <SearchBar
                                possible_results=possible_results.clone()
                                width="25ch"
                                input_ref=None
                            />
                        </div>
                    }
                } else {
                    view! {
                        <div class="md:hidden">
                            <SmallSearchBar possible_results=possible_results.clone()/>
                        </div>
                    }
                }
            }}

        </div>
    }
}

#[component]
fn SmallSearchBar(possible_results: Vec<String>) -> impl IntoView {
    let display_search = expect_context::<DisplaySearch>().0;
    let input_ref = create_node_ref::<html::Input>();

    view! {
        <button
            on:click=move |_| {
                display_search.set(true);
                if let Some(node) = input_ref() {
                    let _ = node.focus();
                }
            }

            class="hover:bg-hovered -mr-1 p-1 rounded-lg"
        >
            <img class="h-6 w-auto text-lg" src="../public/images/search.svg" alt="Search"/>
        </button>
        <div hidden=move || !display_search() class="fixed inset-0 w-screen z-[9001]">
            <div class="w-full h-[4.5rem] bg-header flex justify-center">
                <div class="my-auto">
                    <SearchBar
                        possible_results=possible_results.clone()
                        width="80vw"
                        input_ref=Some(input_ref)
                    />
                </div>
            </div>
        </div>
    }
}

pub fn search(possible_results: &[String], search: &str, max_results: usize) -> Vec<String> {
    let search = search.to_lowercase();

    let mut results = possible_results
        .into_iter()
        .map(|s| (jaro_winkler(&s.to_lowercase(), &search), s))
        .filter(|(d, _)| *d >= 0.55)
        .collect::<Vec<_>>();

    results.sort_by(|(d, _), (d2, _)| d2.total_cmp(d));

    results
        .into_iter()
        .take(max_results)
        .map(|(_, s)| s.to_owned())
        .collect()
}

#[component]
fn SearchBar(
    possible_results: Vec<String>,
    width: &'static str,
    input_ref: Option<NodeRef<html::Input>>,
) -> impl IntoView {
    let mut next_search_id = 0u32;

    let (search_results, set_search_results) = create_signal(Vec::new());
    let (display_results, set_display_results) = create_signal(false);
    let (focused, set_focused) = create_signal(false);

    let display_search = use_context::<DisplaySearch>().map(|s| s.0);

    let border_focus_color = move || match focused() {
        true => "rgb(100, 160, 240)",
        false => "#ccc",
    };

    let input_ref = match input_ref {
        Some(node) => node,
        None => create_node_ref::<html::Input>(),
    };

    // generate a search result with a unique ID
    let mut new_search_result = move |r: String| {
        let res = (next_search_id, r);
        next_search_id += 1;
        res
    };

    // Generates new search suggestions when the user is typing
    let on_input_searchbox = move |ev| {
        let current_search = event_target_value(&ev);
        if current_search.is_empty() {
            set_display_results(false);
            return;
        }

        let results = search(&possible_results, &current_search, 8)
            .into_iter()
            .map(|r| new_search_result(r))
            .collect::<Vec<_>>();

        if !results.is_empty() {
            set_display_results(true);
            set_search_results(results);
        }
    };

    // Shows search results when searchbox is in focus
    let on_focus_searchbox = move |_| {
        if !search_results().is_empty() {
            set_display_results(true)
        }
        set_focused(true);
    };

    // Hides search results after searchbox loses focus
    let on_blur_searchbox = move |_| {
        if let Some(display_search) = display_search {
            display_search.set(false);
        }
        set_display_results(false);
        set_focused(false);
    };

    // redirect to `search/<query>` based on the contents of the search box
    let nav_to_search = move || {
        if let Some(input) = input_ref() {
            let _ = input.blur();
            let query = input.value();
            input.set_value("");

            let navigate = use_navigate();
            navigate(&format!("/search/{query}"), NavigateOptions::default());
        }
    };

    // Navigate to a search page when pressing enter or blur the search box when pressing esc
    let on_keydown_searchbox = move |ev: ev::KeyboardEvent| match (ev.key().as_str(), input_ref()) {
        ("Escape", Some(input)) => {
            let _ = input.blur();
        }
        ("Enter", Some(_)) => nav_to_search(),
        _ => {}
    };

    view! {
        <div
            style:width=width
            style:border-color=border_focus_color
            class="grid grid-cols-[1fr_2.4rem] bg-darker border-2 rounded-full"
        >
            <div class="ml-4">
                <label name="search layouts">
                    <input
                        node_ref=input_ref

                        on:input=on_input_searchbox
                        on:focus=on_focus_searchbox
                        on:blur=on_blur_searchbox
                        on:keydown=on_keydown_searchbox

                        type="text"
                        placeholder="Search layouts..."
                        class="w-full h-8 bg-darker outline-none"
                    />
                </label>
            </div>
            <button
                on:mousedown=move |ev| ev.prevent_default()
                on:click=move |_| nav_to_search()

                class="min-w-fit hover:bg-hovered pl-[0.4rem] rounded-r-full"
            >
                <img class="h-6 w-auto" src="../public/images/search.svg" alt="Search"/>
            </button>
        </div>
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
                children=move |(_, result)| view! { <SearchResult result/> }
            />
        </ul>
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
