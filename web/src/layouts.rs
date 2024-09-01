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
pub fn RenderLayoutLinks(names: impl Fn() -> Vec<String> + 'static) -> impl IntoView {
    view! {
        <div class="w-full md:grid md:grid-cols-2 xl:grid-cols-3 md:gap-3">
            {move || {
                names()
                    .into_iter()
                    .map(|name| {
                        view! { <RenderLayoutLink name/> }
                    })
                    .collect_view()
            }}

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
            Some(Err(_)) => view! { <p>"Layout '" {name} "' doesn't exist :("</p> }.into_view(),
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
