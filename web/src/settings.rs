use leptos::*;

use crate::{layouts::RenderNamedDof, util::*, HeatmapTheme};

#[component]
pub fn Settings() -> impl IntoView {
    let HeatmapTheme {
        low,
        high,
        curve,
        max_freq,
    } = expect_context::<HeatmapTheme>();

    view! {
        <h2 class="text-3xl my-4 text-center">"Settings"</h2>
        <div class="mx-8 p-4 bg-black rounded-xl">
            <p class="text-lg">"Theme:"</p>
            <div class="sm:flex">
                <div class="my-3 p-2 border border-white/30 rounded-lg w-fit">
                    <table>
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
                <div class="m-auto w-4/5 sm:w-[20rem] md:w-[28rem] lg:w-[36rem] px-4 md:px-8 py-2 md:py-4">
                    <RenderNamedDof name="noctum".into()/>
                </div>
            </div>
        </div>
    }
}

#[component]
fn Setting(description: &'static str, children: Children) -> impl IntoView {
    view! {
        <tr class="py-1">
            <td class="text-left align-center">{description}</td>
            <td class="px-2">{children()}</td>
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
            class="h-8 w-16 rounded-md border border-header"
        />
    }
}

#[component]
fn ValueInput(affect: RwSignal<f64>) -> impl IntoView {
    view! {
        <input
            on:input=move |ev| {
                let value = event_target_value(&ev);
                if let Ok(val) = value.parse::<f64>() {
                    affect.set(val);
                }
            }

            type="text"
            value=affect()
            class="bg-header border-2 border-[#ccc] rounded-md w-16"
        />
    }
}
