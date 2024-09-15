use leptos::*;
use leptos_router::*;
// use crate::util::*;

#[component]
pub fn Tools() -> impl IntoView {
    view! {
        <div class="-mt-2 sm:-mt-2 p-4 rounded-lg">
            <Tool name="Generate corpus data"/>
        </div>
    }
}

#[component]
fn Tool(name: &'static str) -> impl IntoView {
    let url = name.to_lowercase().replace(' ', "-");

    view! {
        <div class="max-w-[32rem] h-48 content-end p-8 text-lg bg-black rounded-lg
            hover:bg-[#141414] duration-75 my-2 sm:my-4"
        >
            <A href=url>
                <div class="w-full h-full">{name}</div>
            </A>
        </div>
    }
}
