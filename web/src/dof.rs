use crate::{
    analyze::{LayoutKeys, PhysicalLayout, RenderAnalyzeLayout},
    util::*,
};

use leptos::*;
use libdof::Dof;
use oxeylyzer_core::layout::Layout;

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
    let keys = keys
        .iter()
        .map(|c| create_rw_signal(*c))
        .collect::<Box<_>>();
    let phys = PhysicalLayout {
        name,
        fingers,
        keyboard,
        shape,
    };

    view! { <RenderAnalyzeLayout phys keys=LayoutKeys(keys)/> }
}
