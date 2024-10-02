use crate::{
    analyze::{LayoutKeys, PhysicalLayout, RenderAnalyzeLayout},
    util::*,
};

use std::collections::HashMap;

use leptos::*;
use leptos_router::*;
use libdof::{Dof, Language};
use oxeylyzer_core::layout::Layout;
use rust_embed::Embed;
use serde::{Deserialize, Serialize};

#[derive(Embed)]
#[folder = "../layouts"]
#[include = "*.dof"]
pub struct LayoutsFolder;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HeatmapData {
    #[serde(flatten)]
    corpora: HashMap<String, HashMap<char, f64>>,
}

impl HeatmapData {
    pub fn get(&self, corpus: String, c: char) -> Option<f64> {
        match self.corpora.get(&corpus) {
            Some(data) => data.get(&c).copied(),
            None => None,
        }
    }
}

#[component]
pub fn LayoutLinks(names: Memo<Vec<String>>) -> impl IntoView {
    let page = create_rw_signal(0);
    let max_per_page = 12;
    let max_pages = create_memo(move |_| names().len() / 12);

    view! {
        <PaginateSearch page max_pages/>
        <div class="my-2 md:my-3 grid gap-2 md:gap-3 grid-cols-1 md:grid-cols-2 xl:grid-cols-3">
            <Paginate page max_per_page names/>
        </div>
        <PaginateSearch page max_pages/>
    }
}

#[component]
fn Paginate(
    page: RwSignal<usize>,
    max_per_page: usize,
    names: impl Fn() -> Vec<String> + 'static,
) -> impl IntoView {
    view! {
        {move || {
            names()
                .into_iter()
                .skip(page() * max_per_page)
                .take(max_per_page)
                .map(|name| view! { <LayoutLink name/> })
                .collect_view()
        }}
    }
}

#[component]
fn PaginateSearch(page: RwSignal<usize>, max_pages: Memo<usize>) -> impl IntoView {
    let show_left = move || page() > 0;
    let show_right = move || page() < max_pages();

    view! {
        {move || {
            if show_left() || show_right() {
                view! {
                    <div class="bg-black rounded-lg flex justify-center sm:my-2 p-2 sm:p-4">
                        <PaginateButton content="Prev" page page_diff=-1 show_when=show_left/>
                        <div class="w-20 mx-2 my-auto text-center">{page}</div>
                        <PaginateButton content="Next" page page_diff=1 show_when=show_right/>
                    </div>
                }
                    .into_view()
            } else {
                view! {}.into_view()
            }
        }}
    }
}

#[component]
fn PaginateButton(
    content: &'static str,
    page: RwSignal<usize>,
    page_diff: isize,
    show_when: impl Fn() -> bool + 'static,
) -> impl IntoView {
    view! {
        {move || {
            if show_when() {
                view! {
                    <button
                        on:click=move |_| page.update(|v| *v = (*v as isize + page_diff) as usize)
                        class="
                        w-20 mx-2 px-2 py-3 text-center border-2 border-header rounded-lg
                        hover:bg-hovered"
                    >
                        {content}
                    </button>
                }
                    .into_view()
            } else {
                view! { <div class="w-20 mx-2 px-2 py-3"></div> }.into_view()
            }
        }}
    }
}

#[component]
pub fn LayoutLink(name: String) -> impl IntoView {
    view! {
        <div class="p-4 rounded-lg bg-black hover:bg-[#141414]
        duration-75
        ">
            <A href=format!("/layouts/{name}")>
                <p>{name.clone()}</p>
                <div class="pt-2">
                    <NamedDof name/>
                </div>
            </A>
        </div>
    }
}

#[component]
pub fn NamedDof(name: String) -> impl IntoView {
    let (name, _) = create_signal(name);

    let dof = create_resource(move || format!("/layouts/{}.dof", name()), load_json::<Dof>);

    view! {
        {move || match dof.get() {
            Some(Ok(dof)) => {
                view! { <RenderDof dof/> }
            }
            Some(Err(e)) => {
                view! { <p>"Error encountered for '" {name} ":' " {e.to_string()}</p> }.into_view()
            }
            None => {
                view! {
                    // "Loading..."
                    <div class="animate-pulse mx-auto mt-24"></div>
                }
                    .into_view()
            }
        }}
    }
}

#[component]
pub fn RenderDof(dof: Dof) -> impl IntoView {
    let language = match dof.languages().first() {
        Some(l) => l.language.clone().to_lowercase(),
        None => Language::default().language.to_lowercase(),
    };

    let Layout {
        name,
        keys,
        fingers,
        keyboard,
        shape,
    } = Layout::from(dof);

    let keys = LayoutKeys(
        keys.iter()
            .map(|c| create_rw_signal(*c))
            .collect::<Box<_>>(),
    );

    let phys = PhysicalLayout {
        name,
        fingers,
        keyboard,
        shape,
    };

    logging::log!("language: {language}");

    view! { <RenderAnalyzeLayout phys keys language/> }
}
