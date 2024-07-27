// use std::{collections::HashSet, path::PathBuf};

// use ev::{DragEvent, MouseEvent};
// use fxhash::FxHashMap;
// use leptos::*;
// use leptos_router::*;
// use libdof::prelude::{Dof, Finger, PhysicalKey};
// use oxeylyzer_core::prelude::{Analyzer, Data, Layout, Weights};
// use rust_embed::Embed;
// use stylance::import_crate_style;

// use crate::util::*;

// import_crate_style!(css, "./css/keyboard.module.css");

// #[derive(Embed)]
// #[folder = "../layouts"]
// #[include = "*.dof"]
// struct LayoutsFolder;

// #[derive(Debug, Clone, Default)]
// struct Pins(HashSet<usize>);

// #[component]
// pub fn Layouts() -> impl IntoView {
//     let url = |s: &str| format!("/layouts/{s}");

//     view! {
//         <ul>
//             {LayoutsFolder::iter()
//                 .flat_map(|s| PathBuf::from(s.to_string()).file_stem().map(ToOwned::to_owned))
//                 .flat_map(|os| os.into_string())
//                 .map(|i| {
//                     view! {
//                         <li>
//                             <A href=url(&i)>{i.to_string()}</A>
//                         </li>
//                     }
//                 })
//                 .collect_view()}

//         </ul>
//     }
// }

// #[component]
// pub fn Layout() -> impl IntoView {
//     let params = use_params_map();
//     let name = move || params.with(|p| p.get("name").cloned().unwrap_or_default());

//     let dof = create_resource(move || format!("/layouts/{}.dof", name()), load_json::<Dof>);

//     let pins = create_rw_signal(Pins::default());
//     provide_context(pins);

//     let char_freq = create_rw_signal(FxHashMap::<char, f64>::default());
//     provide_context(char_freq);

//     view! {
//         <div>
//             <MaybeViewLayout dof />
//         </div>
//     }
// }

// #[component]
// fn MaybeViewLayout(dof: JsonResource<Dof>) -> impl IntoView {
//     let navigate = use_navigate();

//     let redirect = create_action(move |_: &()| {
//         navigate("/layouts", Default::default());
//         async {}
//     });

//     view! {
//         {move || match dof.get() {
//             Some(Ok(dof)) => {
//                 let layout = create_rw_signal(Layout::from(dof));

//                 provide_context(layout);

//                 let data = create_resource(move || format!("/data/shai.json"), load_json::<Data>);
//                 let weights = create_resource(move || format!("/public/weights/default.json"), load_json::<Weights>);

//                 view! {
//                     <div>
//                         <div class=css::layout_wrapper>
//                             <ViewLayout/>
//                         </div>
//                         <div>
//                             <div class=css::stats_wrapper>
//                                 <MaybeViewAnalysis data weights />
//                             </div>
//                         </div>
//                     </div>
//                 }
//             }
//             Some(Err(_)) => {
//                 redirect.dispatch(());
//                 view! { <div>"Layout not found. Redirecting..."</div> }
//             }
//             None => view! { <div>"Loading..."</div> },
//         }}
//     }
// }

// #[component]
// fn ViewLayout() -> impl IntoView {
//     let layout = expect_context::<RwSignal<Layout>>();

//     let (dragged_index, set_dragged_index) = create_signal(Some(0));

//     let on_drag_start = move |_: DragEvent, index: usize| set_dragged_index(Some(index));

//     let on_drop = move |_: DragEvent, target_index: usize| {
//         if let Some(source_index) = dragged_index.get() {
//             layout.update(|l| l.keys.swap(source_index, target_index));
//             set_dragged_index(None);
//         }
//     };

//     let on_drag_over = move |ev: DragEvent| {
//         ev.prevent_default();
//     };

//     let pins = expect_context::<RwSignal<Pins>>();

//     let on_contextmenu = move |ev: MouseEvent, i: usize| {
//         ev.prevent_default();

//         pins.update(|p| {
//             if !p.0.insert(i) {
//                 p.0.remove(&i);
//             }
//         });
//     };

//     let (lx, hx) = layout.with_untracked(|l| minmax_x(l));
//     let (ly, hy) = layout.with_untracked(|l| minmax_y(l));
//     let (dx, dy) = (hx - lx, hy - ly);

//     let w = 100.0;
//     let kw = w / dx;
//     let h = dy * kw;
//     let ym = dx / dy;

//     view! {
//         <div class=css::layout_wrapper_inner>
//             <div style=move || {
//                 format!("width: {w}cqw; height: {h}cqw")
//             }>
//                 {move || {
//                     layout.with(|l| l.keys.iter()
//                     .copied()
//                     .zip(layout.get().keyboard)
//                     .zip(layout.get().fingers)
//                     .enumerate()
//                     .map(|(index, ((c, pos), f))| {
//                         let pinned = pins.get().0.contains(&index);
//                         view! {
//                             <div
//                                 draggable="true"
//                                 on:dragstart=move |ev| on_drag_start(ev, index)
//                                 on:drop=move |ev| on_drop(ev, index)
//                                 on:dragover=on_drag_over
//                                 on:contextmenu=move |ev| on_contextmenu(ev, index)
//                             >
//                                 <Key c pos lx ly kw ym f pinned/>
//                             </div>
//                         }
//                     })
//                     .collect_view())
//                 }}
//             </div>
//         </div>
//     }
// }

// #[component]
// fn Key(c: char, pos: PhysicalKey, lx: f64, ly: f64, kw: f64, ym: f64, f: Finger, pinned: bool) -> impl IntoView {
//     let x = (pos.x() - lx) * kw;
//     let y = (pos.y() - ly) * kw * ym;
//     let width = (pos.width()) * kw;
//     let height = (pos.height()) * kw * ym;

//     // let char_freq = expect_context::<RwSignal<FxHashMap<char, f64>>>();
//     // let freq = move || char_freq.get().get(&c).copied().unwrap_or(0.0);
//     // let heatmap = move || format!("#{:02x}0000", ((freq() * 12.0) as usize).min(255));

//     let outline = match f {
//         Finger::LP => "#9e0142",
//         Finger::LR => "#d53e4f",
//         Finger::LM => "#f46d43",
//         Finger::LI => "#fdae61",
//         Finger::LT => "#fee08b",
//         Finger::RT => "#e6f598",
//         Finger::RI => "#abdda4",
//         Finger::RM => "#66c2a5",
//         Finger::RR => "#3288bd",
//         Finger::RP => "#5e4fa2",
//     };

//     let op = match pinned {
//         true => 1,
//         false => 0
//     };

//     view! {
//         <div
//             class=css::key_border
//             style:left=format!("{}%", x)
//             style:top=format!("{}%", y)
//             style:width=format!("{}%", width)
//             style:height=format!("{}%", height)
//             style:z-index=format!("{}", (y * 10.0) as u16 + 2)
//             style:border-color=outline
//         ></div>
//         <div
//             class=css::key
//             style:left=format!("{}%", x)
//             style:top=format!("{}%", y)
//             style:width=format!("{}%", width)
//             style:height=format!("{}%", height)
//             style:z-index=format!("{}", (y * 10.0) as u16)
//             // style:background-color=heatmap
//         >
//             <div
//                 class=css::pinned
//                 style:opacity=op
//                 style:border-top-color=outline
//                 style:border-right-color=outline
//             >
//             </div>
//             {c}
//         </div>
//     }
// }

// #[component]
// fn MaybeViewAnalysis(data: JsonResource<Data>, weights: JsonResource<Weights>) -> impl IntoView {
//     let err = move |e: &str| format!("Analysis failed: {}", e);

//     let view = move || match data.get() {
//         Some(Ok(data)) => {
//             let char_freq = expect_context::<RwSignal<FxHashMap<char, f64>>>();
//             char_freq.set(data.chars.clone());

//             match weights.get() {
//                 Some(Ok(weights)) => {
//                     view! {
//                         <div>
//                             <ViewAnalysis data weights />
//                         </div>
//                     }
//                 }
//                 Some(Err(e)) => {
//                     view! { <div>{move || err(&e)}</div> }
//                 }
//                 None => view! { <div>"Loading..."</div> },
//             }
//         },
//         Some(Err(e)) => {
//             view! { <div>{move || err(&e)}</div> }
//         }
//         None => view! { <div>"Loading..."</div> },
//     };

//     view! {
//         <div>{view}</div>
//     }
// }

// async fn generate(analyzer: Analyzer, layout: Layout, pins: Pins) -> Layout {
//     let pin_vec = pins.0.into_iter().collect::<Vec<_>>();

//     (0..100)
//         .into_iter()
//         .map(|_| {
//             analyzer.annealing_depth2_improve(
//                 layout.clone(),
//                 &pin_vec,
//                 10_000_000_000.0,
//                 0.999,
//                 6000,
//             )
//         })
//         .max_by(|(_, s1), (_, s2)| s1.cmp(s2))
//         .map(|(layout, _)| layout)
//         .unwrap()
// }

// #[component]
// fn ViewAnalysis(data: Data, weights: Weights) -> impl IntoView {
//     let (analyzer, _) = create_signal(Analyzer::new(data, weights));

//     let layout = expect_context::<RwSignal<Layout>>();
//     let pins = expect_context::<RwSignal<Pins>>();

//     let stats = move || layout.with(|l| analyzer().stats(l));

//     let finger_use = create_memo(move |_| stats().finger_use);
//     let finger_sfbs = create_memo(move |_| stats().finger_sfbs);
//     let finger_distance = create_memo(move |_| stats().finger_distance);
//     let score = move || layout.with(|l| analyzer().score(l));

//     let optimize = create_action(move |_: &()| {
//         let a = analyzer.get_untracked();
//         let l = layout.get_untracked();
//         let p = pins.get_untracked();

//         async move {
//             let l = generate(a, l, p).await;
//             layout.set(l);
//         }
//     });

//     let stat_fmt = move |v| format!("{:.3}%", v);

//     view! {
//         <table class=css::stats_table>
//             <tbody>
//                 <tr>
//                     <td class=css::stat_td>{"sfbs"}</td>
//                     <td class=css::stat_td>{move || stat_fmt(stats().sfbs)}</td>
//                     <td class=css::stat_td>{"sfs"}</td>
//                     <td class=css::stat_td>{move || stat_fmt(stats().sfs)}</td>
//                     <td class=css::stat_td>{"score"}</td>
//                     <td class=css::stat_td>{score}</td>
//                 </tr>
//             </tbody>
//         </table>
//         <table class=css::stats_table>
//             <tr>
//                 <td class=css::stat_td/>
//                 {move || {
//                     Finger::FINGERS
//                         .iter()
//                         .copied()
//                         .map(|f| view! { <td class=css::stat_td>{f.to_string()}</td> })
//                         .collect_view()
//                 }}
//             </tr>
//             <FingerStat stat=finger_sfbs name="sfbs"/>
//             <FingerStat stat=finger_use name="usage"/>
//             <FingerStat stat=finger_distance name="dist"/>
//         </table>
//         <div class=css::optimize_button_wrapper>
//             <div class=css::optimize_button>
//                 <label>
//                     <button
//                         class=css::optimize_button
//                         on:click=move |_| optimize.dispatch(())
//                         disable=optimize.pending()
//                     >
//                         {"Optimize"}
//                     </button>
//                 </label>
//             </div>
//         </div>
//     }
// }

// #[component]
// fn FingerStat(stat: Memo<[f64; 10]>, name: &'static str) -> impl IntoView {
//     let fmt = move |v| format!("{:.3}", v);

//     view! {
//         <tr>
//             <td class=css::stat_td>{name}</td>
//             {move || {
//                 stat()
//                     .iter()
//                     .copied()
//                     .map(|v| view! {
//                         <td class=css::stat_td>{fmt(v)}</td>
//                     })
//                     .collect_view()
//             }}
//         </tr>
//     }
// }
