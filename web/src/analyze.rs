use crate::{layouts::HeatmapData, util::*, EnableHeatmap};

use ev::{DragEvent, MouseEvent};
use fxhash::FxHashSet;
use leptos::*;
use leptos_router::*;
use libdof::prelude::{Dof, Finger, PhysicalKey, Shape};
use oxeylyzer_core::{
    prelude::{Analyzer, Data, Layout, Weights},
    stats::TrigramStats,
};

pub type Key = RwSignal<char>;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Pins(FxHashSet<usize>);

#[derive(Debug, Clone, PartialEq)]
pub struct PhysicalLayout {
    pub name: String,
    pub fingers: Box<[Finger]>,
    pub keyboard: Box<[PhysicalKey]>,
    pub shape: Shape,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LayoutKeys(pub Box<[Key]>);

impl LayoutKeys {
    pub fn swap(&self, i1: usize, i2: usize) {
        let k1 = self.0.get(i1);
        let k2 = self.0.get(i2);

        if let (Some(sig1), Some(sig2)) = (k1, k2) {
            let help = sig1.get();

            sig1.set(sig2.get());
            sig2.set(help);
        }
    }
}

#[component]
pub fn RenderAnalyzer() -> impl IntoView {
    let params = use_params_map();
    let name = move || params.with(|p| p.get("name").cloned().unwrap_or_default());

    let dof = create_resource(move || format!("/layouts/{}.dof", name()), load_json::<Dof>);

    view! {
        {move || match dof.get() {
            Some(Ok(dof)) => {
                view! { <RenderDofAnalyzer dof/> }
            }
            Some(Err(_)) => view! { <p>"Layout '" {name} "' doesn't exist :("</p> }.into_view(),
            None => {
                view! {
                    // "Loading..."
                    <div class="animate-pulse mx-auto mt-24"></div>
                }
                    .into_view()
            }
        }}
    }
}

#[component]
pub fn RenderDofAnalyzer(dof: Dof) -> impl IntoView {
    let Layout {
        name,
        keys,
        fingers,
        keyboard,
        shape,
    } = Layout::from(dof);
    let keys = keys
        .iter()
        .map(|c| create_rw_signal(*c))
        .collect::<Box<_>>();
    let phys = PhysicalLayout {
        name,
        fingers,
        keyboard,
        shape,
    };

    provide_context(phys.clone());
    provide_context(LayoutKeys(keys.iter().copied().collect::<Box<_>>()));
    provide_context(create_rw_signal(Pins::default()));

    view! {
        <div class="w-full sm:w-3/4 mx-auto">
            // <div class="my-4 grid grid-cols-[2fr_1fr]">
            // <div class="flex justify-center my-4">
                // <div class="w-2/3 sm:mr-[1%] md:mr-[2%] lg:mr-[3%]">
            <RenderAnalyzeLayout phys keys=LayoutKeys(keys)/>
                // </div>
            // <div class="sm:ml-[1%] md:ml-[2%] lg:ml-[3%]">
            // // <p>"Button uno"</p>
            // // <p>"Button dos"</p>
            // // <p>"Button tres"</p>
            // </div>
            // </div>
            <MaybeRenderAnalysis/>
        </div>
    }
}

#[component]
pub fn RenderAnalyzeLayout(phys: PhysicalLayout, keys: LayoutKeys) -> impl IntoView {
    let keys = keys.0;
    let pins = use_context::<RwSignal<Pins>>();

    let (lx, hx) = minmax_x(&phys.keyboard);
    let (ly, hy) = minmax_y(&phys.keyboard);
    let (dx, dy) = (hx - lx, hy - ly);

    let width = 100.0;
    let kw = width / dx;
    let height = dy * kw;
    let ym = dx / dy;

    let font_size = 2.8;

    let (dragged_sig, set_dragged_sig) = create_signal::<Option<Key>>(None);

    let on_drag_start = move |_: DragEvent, key: Key| set_dragged_sig(Some(key));

    let on_drop = move |_: DragEvent, target_key: Key| {
        if let Some(source_key) = dragged_sig.get() {
            let help = source_key();
            source_key.set(target_key());
            target_key.set(help);

            set_dragged_sig(None);
        }
    };

    let on_drag_over = move |ev: DragEvent| {
        ev.prevent_default();
    };

    let on_contextmenu = move |ev: MouseEvent, i: usize| {
        ev.prevent_default();

        if let Some(pins) = pins {
            pins.update(|p| {
                if !p.0.insert(i) {
                    p.0.remove(&i);
                }
            })
        };
    };

    let draggable = match pins {
        Some(_) => "true",
        None => "false",
    };

    let key_views = keys
        .iter()
        .copied()
        .zip(phys.keyboard)
        .zip(phys.fingers)
        .enumerate()
        .map(|(i, ((k, pos), f))| {
            view! {
                <div
                    class="select-none"
                    draggable=draggable
                    on:dragstart=move |ev| on_drag_start(ev, k)
                    on:drop=move |ev| on_drop(ev, k)
                    on:dragover=on_drag_over
                    on:contextmenu=move |ev| on_contextmenu(ev, i)
                >
                    <Key k pos lx ly kw ym f i/>
                </div>
            }
        })
        .collect::<Vec<_>>();

    view! {
        <div class="m-4 w-4/5 mx-auto container-inline-size">
            <div style=move || {
                format!("width: {width}cqw; height: {height}cqw; font-size: {font_size}cqw")
            }>{key_views}</div>
        </div>
    }
}

fn heatmap_gradient(freq: f64, curve: f64, max: f64) -> String {
    let freq = freq.powf(curve).min(max).max(0.0);

    let factor = freq / max;
    let start = (66.0, 120.0, 128.0);
    let end = (255.0, 32.0, 32.0);

    let r = (start.0 + factor * (end.0 - start.0)) as u16;
    let g = (start.1 + factor * (end.1 - start.1)) as u16;
    let b = (start.2 + factor * (end.2 - start.2)) as u16;

    format!("rgb({r}, {g}, {b})")
}

#[component]
fn Key(
    k: Key,
    pos: PhysicalKey,
    lx: f64,
    ly: f64,
    kw: f64,
    ym: f64,
    f: Finger,
    i: usize,
) -> impl IntoView {
    let width = (pos.width()) * kw;
    let height = (pos.height()) * kw * ym;

    let x = (pos.x() - lx) * kw + 0.15;
    let y = (pos.y() - ly) * kw * ym + 0.15 * ym;

    let width = width - 0.3;
    let height = height - 0.3 * ym;

    let op = move || match use_context::<RwSignal<Pins>>() {
        Some(pins) => match pins().0.contains(&i) {
            true => 1,
            false => 0,
        },
        None => 0,
    };

    let freq = move || {
        expect_context::<JsonResource<HeatmapData>>().with(|data| match data {
            Some(Ok(data)) => {
                data.get("shai".to_owned(), k()).unwrap_or_default()
            }
            Some(Err(e)) => {
                logging::log!("{e}");
                0.0
            }
            None => 0.0,
        })
    };

    let enable_heatmap = expect_context::<EnableHeatmap>().0;
    let bg = move || match enable_heatmap() {
        true => heatmap_gradient(freq(), 1.2, 12.0),
        false => match f {
            Finger::LP => "#b4014b", //"#9e0142",
            Finger::LR => "#d53e4f",
            Finger::LM => "#f46d43",
            Finger::LI => "#fdae61",
            Finger::LT => "#fee08b",
            Finger::RT => "#e6f598",
            Finger::RI => "#abdda4",
            Finger::RM => "#66c2a5",
            Finger::RR => "#3288bd",
            Finger::RP => "#6b5ab8", //"#5e4fa2",
        }
        .to_string(),
    };

    view! {
        <div
            class="
            absolute flex border-[0.3cqw] border-darker items-center justify-center
            bg-darker text-darker rounded-[1cqw] container-inline-size
            "
            style:left=format!("{}%", x)
            style:top=format!("{}%", y)
            style:width=format!("{}%", width)
            style:height=format!("{}%", height)
            style:background-color=bg
        >
            <div
                class="
                absolute top-0 right-0 w-0 h-0
                border-l-[13cqw] border-l-transparent border-b-[13cqw] border-b-transparent
                border-r-[13cqw] border-r-darker border-t-[13cqw] border-t-darker
                "
                style:opacity=op
            ></div>
            {k}
        </div>
    }
}

#[component]
fn MaybeRenderAnalysis() -> impl IntoView {
    let err = move |e: &str| format!("Analysis failed: {}", e);

    let data = create_resource(move || format!("/data/shai.json"), load_json::<Data>);
    let weights = create_resource(
        move || format!("/public/weights/default.json"),
        load_json::<Weights>,
    );

    view! {
        {move || {
            match data() {
                Some(Ok(data)) => {
                    match weights() {
                        Some(Ok(weights)) => view! { <RenderAnalysis data weights/> }.into_view(),
                        Some(Err(e)) => err(&e).into_view(),
                        None => "Loading weights...".into_view(),
                    }
                }
                Some(Err(e)) => err(&e).into_view(),
                None => "Loading data...".into_view(),
            }
        }}
    }
}

#[component]
fn RenderAnalysis(data: Data, weights: Weights) -> impl IntoView {
    let phys = expect_context::<PhysicalLayout>();
    let keys = expect_context::<LayoutKeys>().0;
    // let pins = use_context::<RwSignal<Pins>>();

    let (analyzer, _) = create_signal(Analyzer::new(data, weights));
    let layout_memo = create_memo(move |_| Layout {
        name: phys.name.clone(),
        keys: keys.iter().map(|s| s()).collect(),
        fingers: phys.fingers.clone(),
        keyboard: phys.keyboard.clone(),
        shape: phys.shape.clone(),
    });

    let stats_memo = create_memo(move |_| analyzer.with(|a| layout_memo.with(|l| a.stats(l))));

    let sfbs = create_memo(move |_| stats_memo.with(|s| s.sfbs));
    let sfs = create_memo(move |_| stats_memo.with(|s| s.sfs));
    let finger_use = create_memo(move |_| stats_memo.with(|s| s.finger_use));
    let finger_sfbs = create_memo(move |_| stats_memo.with(|s| s.finger_sfbs));

    let weighted_finger_distance =
        create_memo(move |_| stats_memo.with(|s| s.weighted_finger_distance));
    
    let unweighted_finger_distance =
        create_memo(move |_| stats_memo.with(|s| s.unweighted_finger_distance));
    
    let trigrams = create_memo(move |_| stats_memo.with(|s| s.trigrams.clone()));

    view! {
        <div class="w-full overflow-x-scroll">
            <table class="w-full m-4">
                <thead>
                </thead>
                <tbody class="grid">
                    <RenderStatRow stats=vec![("sfbs", sfbs), ("sfs", sfs)]/>
                    <RenderFingerStatRow name="sfbs" stat=finger_sfbs unit="%"/>
                    <RenderFingerStatRow name="use" stat=finger_use unit="%"/>
                    <RenderFingerStatRow name="dist" stat=unweighted_finger_distance unit=""/>
                    <RenderFingerStatRow name="weighted dist" stat=weighted_finger_distance unit=""/>
                    <RenderTrigrams trigrams/>
                </tbody>
            </table>
        </div>
    }
}

#[component]
fn RenderStatRow(stats: Vec<(&'static str, impl Fn() -> f64 + 'static)>) -> impl IntoView {
    let rows = stats
        .into_iter()
        .map(|(name, stat)| {
            view! {
                <td class="text-left align-top px-2 py-1">
                    {name} ": " {move || format!("{:.3}%", stat())}
                </td>
            }
        })
        .collect::<Vec<_>>();

    view! { <tr class="grid grid-flow-col even:bg-[#292929]">{rows}</tr> }
}

#[component]
fn RenderFingerStatRow(
    name: &'static str,
    stat: impl Fn() -> [f64; 10] + 'static,
    unit: &'static str,
) -> impl IntoView {
    let rows = move || {
        stat().into_iter().map(|v| {
        view! { <td class="text-left align-top px-2 py-1">{move || format!("{v:.3}{unit}")}</td> }
    }).collect::<Vec<_>>()
    };

    view! {
        <tr class="grid grid-flow-col even:bg-[#292929]">
            <td class="text-left align-top px-2 py-1">{name}</td>
            {rows}
        </tr>
    }
}

#[component]
fn RenderTrigrams(trigrams: Memo<TrigramStats>) -> impl IntoView {
    view! {
        <RenderStatRow stats=vec![("sft", move || trigrams.with(|s| s.sft))]/>
        <RenderStatRow stats=vec![("sfb", move || trigrams.with(|s| s.sfb))]/>
        <RenderStatRow stats=vec![("inroll", move || trigrams.with(|s| s.inroll))]/>
        <RenderStatRow stats=vec![("outroll", move || trigrams.with(|s| s.outroll))]/>
        <RenderStatRow stats=vec![("alternate", move || trigrams.with(|s| s.alternate))]/>
        <RenderStatRow stats=vec![("redirect", move || trigrams.with(|s| s.redirect))]/>
        <RenderStatRow stats=vec![("onehandin", move || trigrams.with(|s| s.onehandin))]/>
        <RenderStatRow stats=vec![("onehandout", move || trigrams.with(|s| s.onehandout))]/>
        <RenderStatRow stats=vec![("thumb", move || trigrams.with(|s| s.thumb))]/>
        <RenderStatRow stats=vec![("invalid", move || trigrams.with(|s| s.invalid))]/>
    }
}
