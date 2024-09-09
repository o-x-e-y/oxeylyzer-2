use leptos::*;
use leptos_router::*;
// use crate::util::*;

#[component]
pub fn Tools() -> impl IntoView {
    view! {
        <div class="m-4 p-4 rounded-lg">
            <Tool name="Generate corpus data"/>
        </div>
    }
}

#[component]
fn Tool(name: &'static str) -> impl IntoView {
    let url = name.to_lowercase().replace(' ', "-");

    view! {
        <A href=url>
            <div class="w-full max-w-[32rem] h-48 content-end p-8 text-lg bg-black rounded-lg
            hover:bg-[#141414] duration-75">
                <div>{name}</div>
            </div>
        </A>
    }
}
