use leptos::*;
use leptos_router::*;
use crate::{
    search::NavSearch,
    util::*,
    *
};

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="
        w-11/12 mx-auto text-1xl flex flex-col-reverse py-8 sm:grid sm:grid-cols-homepage
        sm:gap-6 sm:py-12">
            <div class="w-full">
                <p class="text-center sm:px-4 max-sm:py-6">
                    "An online keyboard layout analyzer and generator."
                </p>
            // <p class="flex justify-center">
            // <button
            // class="sm:my-8 p-6 bg-header rounded-2xl"
            // >
            // <p class="text-lg">"Get started"</p>
            // <p class="text-[#00caca]">"Analyze dvorak"</p>
            // </button>
            // </p>
            </div>
            <div class="w-full">
                <layouts::NamedDof name="noctum".to_string()></layouts::NamedDof>
            </div>
        </div>
    }
}

#[component]
pub fn Navigation() -> impl IntoView {
    let is_window_sm = leptos_use::use_media_query("(min-width: 640px)");

    view! {
        <header class="w-full bg-header">
            <nav class="flex p-4 pr-5 sm:pl-8">
                <A class="visited:text-txt text-nowrap" href="/">
                    <h1 class="text-4xl">"Oxeylyzer\u{00A0}2"</h1>
                </A>
                {move || {
                    if is_window_sm() {
                        view! { <NormalNav/> }
                    } else {
                        view! { <SmallNav/> }
                    }
                }}

            </nav>
        </header>
    }
}

#[component]
fn GithubImage() -> impl IntoView {
    view! {
        <a
            class="my-auto hover:bg-hovered rounded-full"
            href="https://github.com/o-x-e-y/oxeylyzer-2"
        >
            <img class="h-7 w-auto" src="../public/images/github-logo.svg" alt="Github"/>
        </a>
    }
}

#[component]
fn NavSettings() -> impl IntoView {
    view! {
        <A class="my-auto -mx-2 p-1 hover:bg-hovered rounded-full" href="/settings">
            <img
                class="h-9 w-auto -ml-[0.04rem] leading-9 text-lg"
                src="../public/images/settings.svg"
                alt="Settings"
            />
        </A>
    }
}

#[component]
fn ToggleHeatmap() -> impl IntoView {
    let enable_heatmap = expect_context::<EnableHeatmap>().0;
    let theme = use_context::<HeatmapTheme>().unwrap_or_default();

    let gradient = move || match enable_heatmap() {
        true => {
            let low = theme.low.get();
            let high = theme.high.get();
            format!("linear-gradient(to right top, rgb{:?}, rgb{:?})", high, low)
        }
        false => r#"
            linear-gradient(to right top, #b4014b, #d53e4f, #f46d43, #fdae61,
            #fee08b, #e6f598, #abdda4, #66c2a5, #3288bd, #6b5ab8)"#
            .to_owned(),
    };

    view! {
        <button
            on:click=move |_| enable_heatmap.update(|b| *b = !*b)
            class="my-auto p-1 -mx-1 rounded-md hover:bg-hovered"
        >
            <div style:background-image=gradient class="w-8 h-8 rounded-[0.25rem]"></div>
        </button>
    }
}

#[component]
fn NormalNav() -> impl IntoView {
    let possible_results = expect_context::<LayoutNames>().0;

    view! {
        <ul class="hidden w-full justify-end list-none sm:flex sm:gap-5">
            <NavElem text="Layouts" href="/layouts"/>
            <NavElem text="Tools" href="/tools"/>
            <NavElem text="Posts" href="/posts"/>
            <NavSearch possible_results/>
            <NavSettings/>
            <ToggleHeatmap/>
        // <GithubImage/>
        </ul>
    }
}

#[component]
fn SmallNav() -> impl IntoView {
    let (dots_clicked, set_dots_clicked) = create_signal(false);
    let possible_results = expect_context::<LayoutNames>().0;

    view! {
        <div class="flex gap-5 w-full justify-end sm:hidden">
            <NavSearch possible_results/>
            <button
                class="hover:bg-hovered py-1 -mx-2 rounded-lg"
                on:click=move |_| set_dots_clicked(true)
            >
                <img class="h-6 w-auto text-lg" src="../public/images/three-dots.svg" alt="Menu"/>
            </button>
            <NavSettings/>
            <ToggleHeatmap/>
        </div>
        <div
            class="fixed inset-0 w-screen h-screen bg-black/20 z-[9001] backdrop-blur-sm"
            hidden=move || !dots_clicked()
            on:click=move |_| set_dots_clicked(false)
        >
            <div class="flex justify-end">
                <ul class="m-4 p-4 pr-16 bg-header rounded-xl list-none">
                    <NavElem text="Layouts" href="/layouts"/>
                    <NavElem text="Tools" href="/tools"/>
                    <NavElem text="Posts" href="/posts"/>
                    <NavElem text="Github" href="https://github.com/o-x-e-y/oxeylyzer-2"/>
                </ul>
            </div>
        </div>
    }
}

#[component]
fn NavElem(text: &'static str, href: &'static str) -> impl IntoView {
    view! {
        <A class="my-auto text-xl text-ccc visited:text-ccc hover:text-txt" href>
            <ul>{text}</ul>
        </A>
    }
}
