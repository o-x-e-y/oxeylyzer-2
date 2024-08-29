use std::collections::HashMap;

use crate::dof::{RenderDof, RenderNamedDof};
use crate::util::*;

use ev::MouseEvent;
use leptos::*;
use leptos_router::*;
use libdof::Dof;
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
pub fn RenderLayoutLinks(base_url: &'static str) -> impl IntoView {
    view! {
        <div class="flex justify-center">
            <div class=" bg-darker p-4 w-full md:grid md:grid-cols-2 xl:grid-cols-3">
                {embedded_names::<LayoutsFolder>()
                    .map(|name| {
                        view! { <RenderLayoutLink base_url name/> }
                    })
                    .collect_view()}
            </div>
        </div>
    }
}

#[component]
pub fn RenderLayoutLink(base_url: &'static str, name: String) -> impl IntoView {
    view! {
        <div class="p-3 m-2 rounded-lg bg-black container-inline-size hover:bg-header">
            <A href=format!("/{base_url}/{name}")>
                <p>{name.clone()}</p>
                <div class="p-2">
                    <RenderNamedDof name/>
                </div>
            </A>
        </div>
    }
}

#[component]
pub fn RenderLayout() -> impl IntoView {
    let params = use_params_map();
    let name = move || params.with(|p| p.get("name").cloned().unwrap_or_default());

    let dof = create_resource(move || format!("/layouts/{}.dof", name()), load_json::<Dof>);

    view! {
        {move || match dof.get() {
            Some(Ok(dof)) => {
                view! {
                    <div class="w-full sm:w-3/4 mx-auto">
                        <RenderMetadataDof dof/>
                    </div>
                }
                    .into_view()
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

fn collapse_data(data: RwSignal<Option<String>>, collapsed: ReadSignal<bool>) {
    if collapsed() {
        match data() {
            None => data.set(Some("Unknown".to_owned())),
            _ => {}
        };
    } else {
        match data().as_deref() {
            Some("Unknown") => data.set(None),
            _ => {}
        }
    }
}

#[component]
pub fn RenderMetadataDof(dof: Dof) -> impl IntoView {
    let name = Some(dof.name().to_owned());
    let authors = dof.authors().map(|v| v.join(", "));
    let description = dof.description().map(ToOwned::to_owned);
    let year = dof.year().map(|y| y.to_string());
    // let board = match dof.board_type() {
    //     KeyboardType::Custom(s) if s.is_empty() => None,
    //     b => Some(b.to_string())
    // };
    let langs_str = dof
        .languages()
        .iter()
        .map(|l| format!("{l:?}"))
        .collect::<Vec<_>>();
    let languages = Some(langs_str.join(", "));
    let link_base = dof.link().map(move |l| {
        let link = l.to_owned();
        view! { <a href=link.clone()>{link}</a> }.into_view()
    });

    let (info, set_info) = create_signal('𝅉');

    let name = create_rw_signal(name);
    let authors = create_rw_signal(authors);
    let description = create_rw_signal(description);
    let year = create_rw_signal(year);
    // let board = create_rw_signal(board);
    let languages = create_rw_signal(languages);

    let link = create_rw_signal(link_base.clone());

    let (collapsed, set_collapsed) = create_signal(true);

    let collapse = move |_: MouseEvent| {
        match collapsed() {
            true => {
                set_info('𝅏');
                match link() {
                    None if link_base.is_none() => link.set(Some("Unknown".into_view())),
                    _ => link.set(link_base.clone()),
                }
            }
            false => {
                set_info('𝅉');
                if link() == Some("Unknown".into_view()) {
                    link.set(None)
                }
            }
        }

        collapse_data(name, collapsed);
        collapse_data(authors, collapsed);
        collapse_data(description, collapsed);
        collapse_data(year, collapsed);
        // collapse_data(board, collapsed);
        collapse_data(languages, collapsed);

        set_collapsed.update(|b| *b = !*b);
    };

    view! {
        <div class="p-4">
            <RenderDof dof/>
        </div>
        <div class="flex justify-center">
            <table class="w-full m-4">
                <thead>
                    <tr class="bg-[#292929]">
                        <th class="text-left align-top px-2 py-1">
                            <label name="collapse-metadata">
                                <button on:click=collapse>
                                    <span>"Info"</span>
                                    <span class="absolute -mt-3 opacity-70">{move || info()}</span>
                                </button>
                            </label>
                        </th>
                        <th></th>
                    </tr>
                </thead>
                <tbody>
                    <Metadata name="Name" data=name/>
                    <Metadata name="Authors" data=authors/>
                    <Metadata name="Year" data=year/>
                    <Metadata name="Description" data=description/>
                    <Metadata name="Source" data=link/>
                    <Metadata name="Languages" data=languages/>
                // <Metadata name="Board" data=board />
                </tbody>
            </table>
        </div>
    }
}

#[component]
fn Metadata(
    name: &'static str,
    data: RwSignal<Option<impl IntoView + Clone + 'static>>,
) -> impl IntoView {
    view! {
        {move || match data() {
            Some(data) => {
                Some(
                    view! {
                        <tr class="even:bg-[#292929] grid grid-cols-metadata px-2">
                            <td class="text-left align-top py-1">{name}</td>
                            <td class="text-left align-top py-1">{data}</td>
                        </tr>
                    },
                )
            }
            None => None,
        }}
    }
}
