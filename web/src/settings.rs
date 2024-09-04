use std::{fmt::Display, str::FromStr};

use leptos::*;

use crate::{layouts::NamedDof, util::*};

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
fn ThemeSettings() -> impl IntoView {
    let HeatmapTheme {
        low,
        high,
        curve,
        max_freq,
    } = expect_context::<HeatmapTheme>();

    view! {
        <p class="text-lg">"Theme"</p>
        <div class="md:flex max-md:mb-2">
            <SettingGroup>
                <Setting description="Low freq color:">
                    <ColorInput affect=low/>
                </Setting>
                <Setting description="High freq color:">
                    <ColorInput affect=high/>
                </Setting>
                <ValueSetting description="Curve power:" affect=curve/>
                <ValueSetting description="Max color freq:" affect=max_freq/>
            </SettingGroup>
            <div class="m-auto md:w-[28rem] lg:w-[36rem]">
                <NamedDof name="noctum".into()/>
            </div>
        </div>
    }
}

#[component]
fn WeightSettings() -> impl IntoView {
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
        <div class="flex">
            <div class="mr-1 sm:mr-4">
                <p class="text-lg">"Weights"</p>
                <SettingGroup>
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
                <p class="text-lg">"Fingers"</p>
                <SettingGroup>
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
fn SettingGroup(children: Children) -> impl IntoView {
    view! {
        <div class="my-3 p-2 border border-white/30 rounded-lg w-fit">
            <table>
                <tbody>{children()}</tbody>
            </table>
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
fn ValueSetting<T>(description: &'static str, affect: RwSignal<T>) -> impl IntoView
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
fn ValueInput<T>(affect: RwSignal<T>) -> impl IntoView
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
            value=affect.get().to_string()
            class="bg-header border-2 border-[#ccc] rounded-md w-16"
        />
    }
}
