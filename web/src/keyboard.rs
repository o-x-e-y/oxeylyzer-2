use std::path::PathBuf;

use ev::{DragEvent, MouseEvent};
use leptos::*;
use leptos_router::*;
use libdof::prelude::{PhysicalKey, Dof, Finger};
use oxeylyzer_core::prelude::{Analyzer, Layout, Weights, Data};
use rust_embed::Embed;
use stylance::{classes, import_crate_style};

use crate::util::*;

import_crate_style!(css, "./css/keyboard.module.css");

#[derive(Embed)]
#[folder = "../layouts"]
#[include = "*.dof"]
struct LayoutsFolder;

#[component]
pub fn Layouts() -> impl IntoView {
    let url = |s: &str| format!("/layouts/{s}");

    view! {
        <ul>
            {LayoutsFolder::iter()
                .flat_map(|s| PathBuf::from(s.to_string()).file_stem().map(ToOwned::to_owned))
                .flat_map(|os| os.into_string())
                .map(|i| {
                    view! {
                        <li>
                            <A href=url(&i)>{i.to_string()}</A>
                        </li>
                    }
                })
                .collect_view()}

        </ul>
        <Outlet/>
    }
}

#[component]
pub fn Layout() -> impl IntoView {
    let params = use_params_map();
    let name = move || params.with(|p| p.get("name").cloned().unwrap_or_default());

    let dof = create_resource(move || format!("/layouts/{}.dof", name()), load_json::<Dof>);

    view! {
        <div>
            <MaybeViewLayout dof />
        </div>
    }
}

#[component]
fn MaybeViewLayout(dof: JsonResource<Dof>) -> impl IntoView {
    let navigate = use_navigate();

    let redirect = create_action(move |_: &()| {
        navigate("/layouts", Default::default());
        async {}
    });

    view!{
        {move || match dof.get() {
            Some(Ok(dof)) => {
                let layout = create_rw_signal(Layout::from(dof));

                provide_context(layout);

                let data = create_resource(move || format!("/data/shai.json"), load_json::<Data>);
                let weights = create_resource(move || format!("/weights/default.json"), load_json::<Weights>);
    
                view! {
                    <div>
                        <div class=css::layout_wrapper>
                            <ViewLayout layout/>
                        </div>
                        <div>
                            <div class=css::stats_wrapper>
                                <MaybeViewAnalysis data weights />
                            </div>
                        </div>
                    </div>
                }
            }
            Some(Err(_)) => {
                redirect.dispatch(());
                view! { <div>"Layout not found. Redirecting..."</div> }
            }
            None => view! { <div>"Loading..."</div> },
        }}
    }
}

#[component]
fn ViewLayout(layout: RwSignal<Layout>) -> impl IntoView {
    let (dragged_index, set_dragged_index) = create_signal(Some(0));

    let on_drag_start = move |_: DragEvent, index: usize| set_dragged_index(Some(index));

    let on_drop = move |_: DragEvent, target_index: usize| {
        if let Some(source_index) = dragged_index.get() {
            layout.update(|l| l.keys.swap(source_index, target_index));
            set_dragged_index(None);
        }
    };

    let on_drag_over = move |ev: DragEvent| {
        ev.prevent_default();
    };

    let (lx, hx) = layout.with_untracked(|l| minmax_x(l));
    let (ly, hy) = layout.with_untracked(|l| minmax_y(l));
    let (dx, dy) = (hx - lx, hy - ly);

    let w = 100.0;
    let kw = w / dx;
    let h = dy * kw;
    let ym = dx / dy;

    view! {
        <div class=css::layout_wrapper_inner>
            <div style=move || {
                format!("width: {w}cqw; height: {h}cqw")
            }>
                {move || {
                    layout.with(|l| l.keys.iter()
                    .copied()
                    .zip(layout.get().keyboard)
                    .enumerate()
                    .map(|(index, (c, pos))| {
                        view! {
                            <div
                                draggable="true"
                                on:dragstart=move |ev| on_drag_start(ev, index)
                                on:drop=move |ev| on_drop(ev, index)
                                on:dragover=on_drag_over
                            >
                                <Key c pos lx ly kw ym/>
                            </div>
                        }
                    })
                    .collect_view())
                }}
            </div>
        </div>
    }
}

#[component]
fn Key(c: char, pos: PhysicalKey, lx: f64, ly: f64, kw: f64, ym: f64) -> impl IntoView {
    let x = (pos.x() - lx) * kw;
    let y = (pos.y() - ly) * kw * ym;
    let width = (pos.width()) * kw;
    let height = (pos.height()) * kw * ym;

    view! {
        <div
            class=css::key
            style:left=format!("{}%", x)
            style:top=format!("{}%", y)
            style:width=format!("{}%", width)
            style:height=format!("{}%", height)
            style:z-index=format!("{}", (y * 10.0) as u16)
        >
            {c}
        </div>
    }
}

#[component]
fn MaybeViewAnalysis(data: JsonResource<Data>, weights: JsonResource<Weights>) -> impl IntoView {
    let err = move |e: &str| format!("Analysis failed: {}", e);
    
    let view = move || match data.get() {
        Some(Ok(data)) => {
            match weights.get() {
                Some(Ok(weights)) => {
                    view! {
                        <div>
                            <ViewAnalysis data weights />
                        </div>
                    }
                }
                Some(Err(e)) => {
                    view! { <div>{move || err(&e)}</div> }
                }
                None => view! { <div>"Loading..."</div> },
            }
        }
        Some(Err(e)) => {
            view! { <div>{move || err(&e)}</div> }
        }
        None => view! { <div>"Loading..."</div> },
    };
    
    view!{
        <div>{view}</div>
    }
}

#[component]
fn ViewAnalysis(data: Data, weights: Weights) -> impl IntoView {
    let (analyzer, _) = create_signal(Analyzer::new(data, weights));

    let layout = use_context::<RwSignal<Layout>>()
        .expect("Couldn't get layout for whatever reason");

    let stats = move || layout.with(|l| analyzer().stats(l));

    let finger_use = create_memo(move |_| stats().finger_use);
    let finger_sfbs = create_memo(move |_| stats().finger_sfbs);
    let score = move || layout.with(|l| analyzer().score(l));

    let (pins, set_pins) = create_signal(vec![]);

    let generate = move |_: MouseEvent| {
        let start = wasm_timer::Instant::now();

        // let (l, _) = analyzer().annealing_depth2_improve(layout.get(), &pins(), 10_000_000_000.0, 0.9999, 20000);
        let (l, _) = analyzer().alternative_d3(layout.get(), &pins());

        logging::log!("generated a layout in {:.2}s", start.elapsed().as_secs_f64());

        layout.set(l);
    };

    view! {
        <div class=css::optimize_button_wrapper>
            <div class=css::optimize_button>
                <label>
                    <input
                        type="text"
                        placeholder="pins..."
                        on:input=move |ev| set_pins(pin_positions(&layout(), event_target_value(&ev)))
                    />
                </label>
            </div>
        </div>
        <div class=css::optimize_button_wrapper>
            <div class=css::optimize_button>
                <label>
                    <button
                        class=css::optimize_button
                        on:click=generate
                    >
                        {"Generate"}
                    </button>
                </label>
            </div>
        </div>
        <table class=css::stats_table>
            <tbody>
                <StatRow text="score" f=move || score() />
                <StatRow text="sfbs" f=move || format!("{:.3}%", stats().sfbs) />
                <StatRow text="sfs" f=move || format!("{:.3}%", stats().sfs) />
            </tbody>
        </table>
        <p class=css::stat_table_header>{"Sfbs per finger"}</p>
        <table class=css::stats_table>
            {
                (0..5)
                    .into_iter()
                    .map(|i| {
                        view! {
                            <FingerStatRow  stat={finger_sfbs} i />
                        }
                    })
                    .collect_view()
            }
        </table>
        <p class=css::stat_table_header>{"Finger usage"}</p>
        <table class=css::stats_table>
            {
                (0..5)
                    .into_iter()
                    .map(|i| {
                        view! {
                            <FingerStatRow  stat={finger_use} i />
                        }
                    })
                    .collect_view()
            }
        </table>
    }
}

#[component]
fn StatRow<T, F>(text: &'static str, f: F) -> impl IntoView
    where T: IntoView + 'static,
          F: Fn() -> T + 'static
{
    view! {
        <tr class=css::stat_row>
            <td class=css::stat_td>{text}</td>
            <td class=css::stat_td>{f}</td>
        </tr>
    }
}

#[component]
fn FingerStatRow(stat: Memo<[f64; 10]>, i: usize) -> impl IntoView {
    let (v1, v2) = (move || format!("{:.3}%", stat()[i]), move || format!("{:.3}%", stat()[i + 5]));
    let (f1, f2) = (Finger::FINGERS[i], Finger::FINGERS[i + 5]);

    view! {
        <tr>
            <td class=css::stat_finger>{f1.to_string()}</td>
            <td class=css::stat_td>{v1}</td>
            <td class=css::stat_finger>{f2.to_string()}</td>
            <td class=css::stat_td>{v2}</td>
        </tr>
    }
}