use crate::{layouts::HeatmapData, util::*, EnableHeatmap, HeatmapTheme};

use ev::{DragEvent, MouseEvent};
use fxhash::FxHashSet;
use leptos::*;
use leptos_router::*;
use libdof::prelude::{Dof, Finger, PhysicalKey, Shape};
use oxeylyzer_core::prelude::{Analyzer, Data, Layout, Weights};

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
    } = Layout::from(dof.clone());
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
        <div class="w-full">
            // <div class="my-4 grid grid-cols-[2fr_1fr]">
            // <div class="flex justify-center my-4">
            // <div class="w-2/3 sm:mr-[1%] md:mr-[2%] lg:mr-[3%]">
            <div class="p-4 xl:w-7/12 lg:w-2/3 md:w-3/4 sm:w-5/6 mx-auto">
                <RenderAnalyzeLayout phys keys=LayoutKeys(keys)/>
            </div>
            // </div>
            // <div class="sm:ml-[1%] md:ml-[2%] lg:ml-[3%]">
            // // <p>"Button uno"</p>
            // // <p>"Button dos"</p>
            // // <p>"Button tres"</p>
            // </div>
            // </div>
            <div class="mx-4">
                <div class="mb-2">
                    <MaybeRenderAnalysis/>
                </div>
                <div class="mb-4">
                    <DofMetadata dof/>
                </div>
            </div>
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
        <div class="container-inline-size">
            <div style=move || {
                format!("width: {width}cqw; height: {height}cqw; font-size: {font_size}cqw")
            }>{key_views}</div>
        </div>
    }
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
            Some(Ok(data)) => data.get("shai".to_owned(), k()).unwrap_or_default(),
            Some(Err(e)) => {
                logging::log!("{e}");
                0.0
            }
            None => 0.0,
        })
    };

    let enable_heatmap = expect_context::<EnableHeatmap>().0;
    let bg = move || match enable_heatmap() {
        true => {
            let theme = use_context::<HeatmapTheme>().unwrap_or_default();
            heatmap_gradient(freq(), theme)
        },
        false => fingermap_colors(f).to_owned(),
    };
    let title = move || format!("Key usage: {:.2}%", freq());

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
            title=title
        >
            <div
                class="
                absolute top-0 right-0 w-0 h-0
                border-l-[0.8ch] border-l-transparent border-b-[0.8ch] border-b-transparent
                border-r-[0.8ch] border-r-darker border-t-[0.8ch] border-t-darker
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
        <div class="mx-auto text-sm sm:text-base sm:grid sm:grid-cols-[1fr_2.5fr] sm:gap-2">
            <div class="p-4 bg-header rounded-t-xl sm:rounded-b-xl">
                <p class="font-bold text-lg">"Bigrams"</p>
                <RenderStat name="sfbs" stat=sfbs unit="%"/>
                <RenderStat name="sfs" stat=sfs unit="%"/>
                <p class="mt-2 font-bold text-lg">"Trigrams"</p>
                <RenderStat name="sft" stat=move || trigrams.with(|s| s.sft) unit="%"/>
                <RenderStat name="sfb" stat=move || trigrams.with(|s| s.sfb) unit="%"/>
                <RenderStat name="inroll" stat=move || trigrams.with(|s| s.inroll) unit="%"/>
                <RenderStat name="outroll" stat=move || trigrams.with(|s| s.outroll) unit="%"/>
                <RenderStat name="alternate" stat=move || trigrams.with(|s| s.alternate) unit="%"/>
                <RenderStat name="redirect" stat=move || trigrams.with(|s| s.redirect) unit="%"/>
                <RenderStat name="onehandin" stat=move || trigrams.with(|s| s.onehandin) unit="%"/>
                <RenderStat
                    name="onehandout"
                    stat=move || trigrams.with(|s| s.onehandout)
                    unit="%"
                />
                <RenderStat name="thumb" stat=move || trigrams.with(|s| s.thumb) unit="%"/>
                <RenderStat name="invalid" stat=move || trigrams.with(|s| s.invalid) unit="%"/>
            </div>
            <div class="p-4 bg-header rounded-b-xl sm:rounded-t-xl overflow-x-scroll">
                <table class="w-full text-left border-y border-y-hovered">
                    <tr class="text-darker">
                        <th class="border border-hovered"></th>
                        {Finger::FINGERS
                            .map(|f| {
                                let bg = fingermap_colors(f);
                                view! {
                                    <th class="border border-hovered" style:background-color=bg>
                                        {f.to_string()}
                                    </th>
                                }
                            })}

                    </tr>
                    <RenderFingerStat name="Finger usage" stat=finger_use/>
                    <RenderFingerStat name="Finger sfbs" stat=finger_sfbs/>
                    <RenderFingerStat name="Finger distance" stat=weighted_finger_distance/>
                    <RenderFingerStat
                        name="Unweighted finger distance"
                        stat=unweighted_finger_distance
                    />
                </table>
            </div>
        </div>
    }
}

#[component]
fn RenderStat(
    name: &'static str,
    stat: impl Fn() -> f64 + 'static,
    unit: &'static str,
) -> impl IntoView {
    let rendered = move || format!("{:.3}{unit}", stat());

    view! { <p>{name} : {rendered}</p> }
}

#[component]
fn RenderFingerStat(name: &'static str, stat: impl Fn() -> [f64; 10] + 'static) -> impl IntoView {
    let rows = move || {
        stat()
            .into_iter()
            // .zip(Finger::FINGERS)
            .map(|v| {
                // let bg = fingermap_colors(f);
                view! { <td class="p-1 w-[8.5%] border border-hovered">{move || format!("{v:.2}")}</td> }
            })
            .collect::<Vec<_>>()
    };

    view! {
        <tr>
            <th class="p-1 border border-hovered">{name}</th>
            {rows}
        </tr>
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

fn collapse_data(data: RwSignal<Option<String>>, collapsed: ReadSignal<bool>) {
    if collapsed() {
        match data() {
            None => data.set(Some("Unknown".to_owned())),
            _ => {}
        };
    } else {
        match data().as_deref() {
            Some("Unknown") => data.set(None),
            _ => {}
        }
    }
}

#[component]
pub fn DofMetadata(dof: Dof) -> impl IntoView {
    let name = Some(dof.name().to_owned());
    let authors = dof.authors().map(|v| v.join(", "));
    let description = dof.description().map(ToOwned::to_owned);
    let year = dof.year().map(|y| y.to_string());
    let langs_str = dof
        .languages()
        .iter()
        .map(|l| format!("{l:?}"))
        .collect::<Vec<_>>();
    let languages = Some(langs_str.join(", "));
    let link_base = dof.link().map(move |l| {
        let link = l.to_owned();
        view! { <a href=link.clone()>{link}</a> }.into_view()
    });

    let (info, set_info) = create_signal('𝅉');

    let name = create_rw_signal(name);
    let authors = create_rw_signal(authors);
    let description = create_rw_signal(description);
    let year = create_rw_signal(year);
    let languages = create_rw_signal(languages);
    let link = create_rw_signal(link_base.clone());

    let (collapsed, set_collapsed) = create_signal(true);

    let collapse = move |_: MouseEvent| {
        match collapsed() {
            true => {
                set_info('𝅏');
                match link() {
                    None if link_base.is_none() => link.set(Some("Unknown".into_view())),
                    _ => link.set(link_base.clone()),
                }
            }
            false => {
                set_info('𝅉');
                if link() == Some("Unknown".into_view()) {
                    link.set(None)
                }
            }
        }

        collapse_data(name, collapsed);
        collapse_data(authors, collapsed);
        collapse_data(description, collapsed);
        collapse_data(year, collapsed);
        collapse_data(languages, collapsed);

        set_collapsed.update(|b| *b = !*b);
    };

    view! {
        <div class="sm:p-4 border border-hovered rounded-lg">
            <table class="w-full">
                <thead>
                    <tr class="bg-header">
                        <th class="text-left align-top px-2 py-1">
                            <label name="collapse-metadata">
                                <button on:click=collapse>
                                    <span>"Info"</span>
                                    <span class="absolute -mt-3 opacity-70">{move || info()}</span>
                                </button>
                            </label>
                        </th>
                        <th></th>
                    </tr>
                </thead>
                <tbody>
                    <Metadata name="Name" data=name/>
                    <Metadata name="Authors" data=authors/>
                    <Metadata name="Year" data=year/>
                    <Metadata name="Description" data=description/>
                    <Metadata name="Source" data=link/>
                    <Metadata name="Languages" data=languages/>
                // <Metadata name="Board" data=board />
                </tbody>
            </table>
        </div>
    }
}

#[component]
fn Metadata(
    name: &'static str,
    data: RwSignal<Option<impl IntoView + Clone + 'static>>,
) -> impl IntoView {
    view! {
        {move || match data() {
            Some(data) => {
                Some(
                    view! {
                        <tr class="even:bg-header px-2">
                            <th class="text-left align-top py-1 pr-3">{name}</th>
                            <td class="text-left align-top py-1 pl-3">{data}</td>
                        </tr>
                    },
                )
            }
            None => None,
        }}
    }
}
