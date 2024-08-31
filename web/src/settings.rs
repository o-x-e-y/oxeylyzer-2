use leptos::*;

use crate::{
    HeatmapTheme,
    util::*
};

#[component]
pub fn Settings() -> impl IntoView {
    let HeatmapTheme { low, high, curve, max_freq } = expect_context::<HeatmapTheme>();

    view! {
        <h2 class="text-3xl my-4 text-center">"Settings"</h2>
        <div class="mx-8 p-4 bg-black rounded-xl">
            <p class="text-lg">"Theme:"</p>
            <table class="py-3">
                <tbody>
                    <Setting description="Low freq color:">
                        <ColorInput affect=low/>
                    </Setting>
                    <Setting description="High freq color:">
                        <ColorInput affect=high/>
                    </Setting>
                    <Setting description="Curve power:">
                        <ValueInput affect=curve/>
                    </Setting>
                    <Setting description="Max color freq:">
                        <ValueInput affect=max_freq/>
                    </Setting>
                </tbody>
            </table>
        </div>
    }
}

#[component]
fn Setting(description: &'static str, children: Children) -> impl IntoView {
    view! {
        <tr class="py-1">
            <td class="text-left align-center">{description}</td>
            <td>{children()}</td>
        </tr>
    }
}

#[component]
fn ColorInput(affect: RwSignal<(f64, f64, f64)>) -> impl IntoView {
    view! {
        <input
            on:change=move |ev| {
                let value = event_target_value(&ev);
                match hex_to_rgb(&value) {
                    Some(rgb) => affect.set(rgb),
                    None => logging::error!("couldn't parse value! {value}"),
                }
            }

            type="color"
            value=rgb_to_hex(affect())
            class="h-8 rounded-md border border-header"
        />
    }
}

#[component]
fn ValueInput(affect: RwSignal<f64>) -> impl IntoView {
    view! { <input type="text" value=affect() class="bg-header border-2 border-[#ccc] rounded-md"/> }
}