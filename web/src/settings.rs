use std::{fmt::Display, str::FromStr};

use leptos::*;

use web_components::{layouts::NamedDof, util::*};

#[component]
pub fn Settings() -> impl IntoView {
    view! {
        <h2 class="text-3xl my-3 sm:my-4 text-center">"Settings"</h2>
        <div class="mx-4 sm:mx-8 p-4 bg-black rounded-xl">
            <ThemeSettings/>
            <WeightSettings/>
        </div>
    }
}

#[component]
pub fn ThemeSettings() -> impl IntoView {
    let HeatmapTheme {
        low,
        high,
        curve,
        max_freq,
    } = expect_context::<HeatmapTheme>();

    view! {
        <div class="md:flex">
            <SettingGroup header="Theme">
                <Setting description="Low freq color:">
                    <ColorInput affect=low/>
                </Setting>
                <Setting description="High freq color:">
                    <ColorInput affect=high/>
                </Setting>
                <ValueSetting description="Curve power:" affect=curve/>
                <ValueSetting description="Max color freq:" affect=max_freq/>
            </SettingGroup>
            <div class="m-auto px-1 py-3 sm:px-3 md:w-[28rem] lg:w-[36rem]">
                <NamedDof name="noctum".into()/>
            </div>
        </div>
    }
}

#[component]
pub fn WeightSettings() -> impl IntoView {
    let GlobalWeights {
        sfbs,
        sfs,
        sft,
        inroll,
        outroll,
        alternate,
        redirect,
        onehandin,
        onehandout,
        thumb,
        fingers:
            GlobalFingerWeights {
                lp,
                lr,
                lm,
                li,
                lt,
                rt,
                ri,
                rm,
                rr,
                rp,
            },
    } = expect_context::<GlobalWeights>();

    view! {
        <div class="flex md:mt-2">
            <div class="mr-1 sm:mr-4">
                <SettingGroup header="Weights">
                    <ValueSetting description="sfbs:" affect=sfbs/>
                    <ValueSetting description="sfs:" affect=sfs/>
                    <ValueSetting description="sft:" affect=sft/>
                    <ValueSetting description="inroll:" affect=inroll/>
                    <ValueSetting description="outroll:" affect=outroll/>
                    <ValueSetting description="alternate:" affect=alternate/>
                    <ValueSetting description="redirect:" affect=redirect/>
                    <ValueSetting description="onehandin:" affect=onehandin/>
                    <ValueSetting description="onehandout:" affect=onehandout/>
                    <ValueSetting description="thumb:" affect=thumb/>
                </SettingGroup>
            </div>
            <div>
                <SettingGroup header="Fingers">
                    <ValueSetting description="lp:" affect=lp/>
                    <ValueSetting description="lr:" affect=lr/>
                    <ValueSetting description="lm:" affect=lm/>
                    <ValueSetting description="li:" affect=li/>
                    <ValueSetting description="lt:" affect=lt/>
                    <ValueSetting description="rt:" affect=rt/>
                    <ValueSetting description="ri:" affect=ri/>
                    <ValueSetting description="rm:" affect=rm/>
                    <ValueSetting description="rr:" affect=rr/>
                    <ValueSetting description="rp:" affect=rp/>
                </SettingGroup>
            </div>
        </div>
    }
}

#[component]
pub fn SettingGroup(children: Children, header: &'static str) -> impl IntoView {
    view! {
        <div>
            <h3 class="text-lg pb-2">{header}</h3>
            <div class="p-2 border border-white/30 rounded-lg">
                <table>
                    <tbody>{children()}</tbody>
                </table>
            </div>
        </div>
    }
}

#[component]
pub fn Setting(description: &'static str, children: Children) -> impl IntoView {
    view! {
        <tr>
            <td class="text-left align-center pr-4">{description}</td>
            <td>{children()}</td>
        </tr>
    }
}

#[component]
pub fn ValueSetting<T>(description: &'static str, affect: RwSignal<T>) -> impl IntoView
where
    T: Display + FromStr + Clone + 'static,
{
    view! {
        <Setting description>
            <ValueInput affect/>
        </Setting>
    }
}

#[component]
pub fn ColorInput(affect: RwSignal<(f64, f64, f64)>) -> impl IntoView {
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
            value=rgb_to_hex(affect.get_untracked())
            class="h-8 w-16 rounded-md border border-header"
        />
    }
}

#[component]
pub fn ValueInput<T>(affect: RwSignal<T>) -> impl IntoView
where
    T: Display + FromStr + Clone + 'static,
{
    view! {
        <input
            on:input=move |ev| {
                let value = event_target_value(&ev);
                if let Ok(val) = value.parse::<T>() {
                    affect.set(val);
                }
            }

            type="text"
            value=affect.get_untracked().to_string()
            class="w-16 px-1 bg-darker border-2 border-ccc/80 rounded-md"
        />
    }
}

#[component]
pub fn CheckboxInput(affect: RwSignal<bool>) -> impl IntoView {
    view! {
        <input
            type="checkbox"
            prop:checked=move || { affect() }
            on:click=move |_| affect.update(|v| *v = !*v)
            class="w-4 h-4 my-auto"
        />
    }
}
