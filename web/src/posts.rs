// use std::{collections::HashSet, path::PathBuf};

// use ev::{DragEvent, MouseEvent};
// use fxhash::FxHashMap;
// use leptos::*;
// use leptos_router::*;
// use leptos_meta::Link;
// use libdof::prelude::{Dof, Finger, PhysicalKey};
// use oxeylyzer_core::prelude::{Analyzer, Data, Layout, Weights};
// use rust_embed::Embed;
// use serde::{Deserialize, Serialize};
// use stylance::import_crate_style;

// use crate::util::*;

// import_crate_style!(css, "./css/posts.module.css");

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct Metadata {
//     pub title: String,
//     pub date: String,
//     pub description: String,
// }

// #[derive(Clone, Debug)]
// pub struct Post {
//     metadata: Metadata,
//     html: String
// }

// pub fn parse_post(content: &str) -> Option<Post> {
//     use gray_matter::engine::YAML;
//     use gray_matter::Matter;
//     use pulldown_cmark::{html, Options, Parser};

//     let mut options = Options::empty();
//     options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
//     let matter = Matter::<YAML>::new();

//     let post_data = matter
//         .parse_with_struct::<Metadata>(content)
//         .expect("Unable to parse md frontmatter");
//     let metadata = post_data.data;

//     let content = post_data.content;

//     let parser = Parser::new_ext(&content, options);

//     let mut html = String::new();
//     html::push_html(&mut html, parser);

//     Some(Post { metadata, html })
// }

// #[component]
// pub fn RenderPost() -> impl IntoView {
//     let params = use_params_map();
//     let name = move || params.with(|p| p.get("name").cloned().unwrap_or_default());

//     let posts_resource = create_resource(move || format!("/public/posts/{}.md", name()), load_text);

//     let navigate = use_navigate();

//     let redirect = create_action(move |_: &()| {
//         navigate("/", Default::default());
//         async {}
//     });

//     logging::log!("url: /public/posts/{}.md", name());

//     view! {
//         {move || match posts_resource.get() {
//             Some(Ok(post)) => {
//                 if let Some(post) = parse_post(&post) {
//                     view! {
//                         <div>
//                             <ViewPost post/>
//                         </div>
//                     }
//                 } else {
//                     view! {
//                         <div>
//                             {"Nuh uh! nuh uh"}
//                         </div>
//                     }
//                 }
//             }
//             Some(Err(_)) => {
//                 redirect.dispatch(());
//                 view! { <div>"Piece not found. Redirecting..."</div> }
//             }
//             None => view! { <div>"Loading..."</div> },
//         }}
//     }
// }

// #[component]
// fn ViewPost(post: Post) -> impl IntoView {
//     let html = post.html;

//     logging::log!("WHAT");

//     view! {
//         <Link rel="stylesheet" href="/highlighter/styles/github.min.css"/>
//         <script src="/highlighter/load_highlight.js"></script>
//         <div class=css::post_wrapper>
//             <div class="text-red-300" inner_html=html>
//             </div>
//         </div>
//     }
// }
