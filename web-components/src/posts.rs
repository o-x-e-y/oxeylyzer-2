use chrono::NaiveDate;
use gray_matter::engine::YAML;
use gray_matter::Matter;
use leptos::*;
use leptos_meta::Link;
use leptos_router::*;
use pulldown_cmark::{html, Options, Parser};
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
    _metadata: Metadata,
    html: String,
}

#[derive(Embed)]
#[folder = "./../web/public/posts"]
#[include = "*.md"]
pub struct PostsFolder;

#[component]
pub fn RenderPostLinks() -> impl IntoView {
    let mut posts = embedded_names::<PostsFolder>()
        .into_iter()
        .zip(PostsFolder::iter())
        .map(|(name, path)| {
            let content =
                String::from_utf8_lossy(&PostsFolder::get(&path).unwrap().data).into_owned();
            let (metadata, _) = parse_gray_matter(&content);
            let date = NaiveDate::parse_from_str(&metadata.date, "%Y-%m-%d").unwrap_or_else(|e| {
                panic!(
                    "Couldn't parse date {} for post '{}': {e}",
                    metadata.date, name
                )
            });

            (name, metadata, date)
        })
        .collect::<Vec<_>>();

    posts.sort_by(|(_, _, d1), (_, _, d2)| d2.cmp(d1));

    let post_links = posts
        .into_iter()
        .map(|(name, metadata, _)| {
            view! { <RenderPostLink name metadata/> }
        })
        .collect::<Vec<_>>();

    view! {
        <div class="flex justify-center pt-5">
            <h2 class="text-4xl">"Posts"</h2>
        </div>
        <div class="flex justify-center">
            <div class="p-4 w-full md:grid md:grid-cols-2 xl:grid-cols-3">{post_links}</div>
        </div>
    }
}

#[component]
pub fn RenderPostLink(name: String, metadata: Metadata) -> impl IntoView {
    view! {
        <div class="p-4 m-2 rounded-lg bg-black hover:bg-header">
            <A href=format!("/posts/{name}")>
                <p class="text-lg md:text-base">{metadata.title}</p>
                <p class="text-lg md:text-base text-[#aaa]">{metadata.date}</p>
            </A>
        </div>
    }
}

pub fn parse_gray_matter(content: &str) -> (Metadata, String) {
    let matter = Matter::<YAML>::new();

    let gray_matter = matter
        .parse_with_struct::<Metadata>(content)
        .expect("Unable to parse md frontmatter");

    (gray_matter.data, gray_matter.content)
}

pub fn parse_post(content: &str) -> Option<Post> {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    let (metadata, content) = parse_gray_matter(content);

    let parser = Parser::new_ext(&content, options);

    let mut html = String::new();
    html::push_html(&mut html, parser);

    Some(Post {
        _metadata: metadata,
        html,
    })
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

    // logging::log!("url: /public/posts/{}.md", name());

    view! {
        {move || match posts_resource() {
            Some(Ok(post)) => {
                if let Some(post) = parse_post(&post) {
                    view! { <ViewPost post/> }.into_view()
                } else {
                    view! {
                        <div class="flex justify-center">
                            <div class="mx-auto my-12">
                                "Encountered an error while parsing post :("
                            </div>
                        </div>
                    }
                        .into_view()
                }
            }
            Some(Err(_)) => {
                redirect.dispatch(());
                "Piece not found. Redirecting...".into_view()
            }
            None => ().into_view(),
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
                class="
                overflow-auto mx-auto prose prose-posts sm:text-2xl text-2xl md:text-xl lg:text-base
                prose-pre:p-0 prose-pre:m-0 prose-pre:rounded-lg prose-code:sm:text-2xl
                prose-code:md:text-2xl prose-code:lg:text-base
                "
                inner_html=post.html
            ></div>
        </div>
    }
}
