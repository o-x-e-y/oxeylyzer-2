use crate::{
    analyze::{LayoutKeys, PhysicalLayout, RenderAnalyzeLayout},
    util::*,
};

use std::collections::HashMap;

use leptos::*;
use leptos_router::*;
use libdof::Dof;
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
pub fn LayoutsWrapper() -> impl IntoView {
    view! { <Outlet/> }
}

#[component]
pub fn LayoutLinks(names: impl Fn() -> Vec<String> + 'static) -> impl IntoView {
    let page = create_rw_signal(0);
    let max_per_page = 12;

    view! {
        <div>
            <PaginateSearch page/>
            <div class="w-full md:grid md:grid-cols-2 xl:grid-cols-3 md:gap-3">
                <Paginate page max_per_page names/>
            </div>
            <PaginateSearch page/>
        </div>
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
                .map(|name| {
                    view! { <RenderLayoutLink name/> }
                })
                .take(max_per_page)
                .collect_view()
        }}
    }
}

#[component]
fn PaginateSearch(page: RwSignal<usize>) -> impl IntoView {
    view! {
        <div class="my-3 p-4 bg-black rounded-lg flex justify-center">
            <button
                on:click=move |_| page.update(|v| *v = v.saturating_sub(1))
                class="w-20 mx-2 p-2 text-center border-2 border-[#ccc] rounded-lg"
            >
                "<- "
                {move || page().saturating_sub(1)}
            </button>
            <div class="mx-2 py-2 px-4 text-center border-2 border-[#ccc] rounded-lg">
                Page {move || page()}
            </div>
            <button
                on:click=move |_| page.update(|v| *v += 1)
                class="w-20 mx-2 p-2 text-center border-2 border-[#ccc] rounded-lg"
            >
                {move || page() + 1}
                " ->"
            </button>
        </div>
    }
}

#[component]
pub fn RenderLayoutLink(name: String) -> impl IntoView {
    view! {
        <div class="p-4 rounded-lg bg-black container-inline-size hover:bg-header">
            <A href=format!("/layouts/{name}")>
                <p>{name.clone()}</p>
                <div class="pt-2">
                    <RenderNamedDof name/>
                </div>
            </A>
        </div>
    }
}

#[component]
pub fn RenderNamedDof(name: String) -> impl IntoView {
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

    view! { <RenderAnalyzeLayout phys keys/> }
}
