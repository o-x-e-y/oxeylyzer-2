use std::{collections::HashSet, path::PathBuf};

use ev::{DragEvent, MouseEvent};
use fxhash::FxHashMap;
use leptos::*;
use leptos_router::*;
use leptos_meta::Link;
use libdof::prelude::{Dof, Finger, PhysicalKey};
use oxeylyzer_core::prelude::{Analyzer, Data, Layout, Weights};
use rust_embed::Embed;
use serde::{Deserialize, Serialize};

use crate::util::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub title: String,
    pub date: String,
    pub description: String,
}

#[derive(Clone, Debug)]
pub struct Post {
    metadata: Metadata,
    html: String
}

#[derive(Embed)]
#[folder = "./public/posts"]
#[include = "*.md"]
pub struct PostsFolder;

#[component]
pub fn RenderPostLinks() -> impl IntoView {
    view! {
        <div class="flex justify-center">
            <div class=" bg-darker p-4 w-full grid grid-cols-3">
                {embedded_names::<PostsFolder>()
                    .map(|name| {
                        view! { <RenderPostLink name/> }
                    })
                    .collect_view()}
            </div>
        </div>
    }
}

#[component]
pub fn RenderPostLink(name: String) -> impl IntoView {
    view! {
        <div class="p-3 m-2 rounded-lg bg-black container-inline-size hover:bg-header">
            <A href=format!("/posts/{name}")>
                <p>{name.clone()}</p>
                // <div>
                //     <RenderNamedDof name/>
                // </div>
            </A>
        </div>
    }
}

pub fn parse_post(content: &str) -> Option<Post> {
    use gray_matter::engine::YAML;
    use gray_matter::Matter;
    use pulldown_cmark::{html, Options, Parser};

    let mut options = Options::empty();
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    let matter = Matter::<YAML>::new();

    let post_data = matter
        .parse_with_struct::<Metadata>(content)
        .expect("Unable to parse md frontmatter");
    let metadata = post_data.data;

    let content = post_data.content;

    let parser = Parser::new_ext(&content, options);

    let mut html = String::new();
    html::push_html(&mut html, parser);

    Some(Post { metadata, html })
}

#[component]
pub fn RenderPost() -> impl IntoView {
    let params = use_params_map();
    let name = move || params.with_untracked(|p| p.get("name").cloned().unwrap_or_default());

    let posts_resource = create_resource(move || format!("/public/posts/{}.md", name()), load_text);

    let navigate = use_navigate();

    let redirect = create_action(move |_: &()| {
        navigate("/", Default::default());
        async {}
    });

    logging::log!("url: /public/posts/{}.md", name());

    view! {
        {move || match posts_resource() {
            Some(Ok(post)) => {
                if let Some(post) = parse_post(&post) {
                    view! {
                        <ViewPost post/>
                    }
                } else {
                    "Nuh uh! nuh uh".into_view()
                }
            }
            Some(Err(_)) => {
                redirect.dispatch(());
                "Piece not found. Redirecting...".into_view()
            }
            None => "Loading...".into_view(),
        }}
    }
}

#[component]
fn ViewPost(post: Post) -> impl IntoView {
    view! {
        <Link rel="stylesheet" href="/public/highlighter/styles/gruvbox-dark-medium.min.css"/>
        <script src="/public/highlighter/load_highlight.js"></script>
        <div class="flex justify-center my-6">
            <div
                class="prose prose-blog overflow-auto mx-auto prose-pre:p-0 prose-pre:m-0 prose-pre:rounded-lg"
                inner_html=post.html
            />
        </div>
    }
}
