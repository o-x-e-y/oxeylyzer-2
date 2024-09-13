use crate::{layouts::HeatmapData, util::*};

use ev::{DragEvent, MouseEvent};
use fxhash::FxHashSet;
use leptos::*;
use leptos_router::*;
use libdof::prelude::{Dof, Finger, PhysicalKey, Shape};
use oxeylyzer_core::prelude::{Analyzer, Data, Layout};

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

pub type FingerStat = Memo<[Memo<f64>; 10]>;

#[derive(Debug, Copy, Clone)]
pub struct FingerStats {
    finger_use: FingerStat,
    finger_sfbs: FingerStat,
    weighted_finger_distance: FingerStat,
    unweighted_finger_distance: FingerStat,
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
            Some(Err(e)) => {
                view! { <p>"Error encountered for '" {name} ":'" {e.to_string()}</p> }.into_view()
            }
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
            batch(move || {
                let help = source_key();
                source_key.set(target_key());
                target_key.set(help);
            });

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
            let x = (pos.x() - lx) * kw + 0.15;
            let y = (pos.y() - ly) * kw * ym + 0.15 * ym;

            let width = (pos.width()) * kw - 0.3;
            let height = ((pos.height()) * kw - 0.3) * ym;

            let pos = PhysicalKey::xywh(x, y, width, height);

            view! {
                <div
                    class="select-none"
                    draggable=draggable
                    on:dragstart=move |ev| on_drag_start(ev, k)
                    on:drop=move |ev| on_drop(ev, k)
                    on:dragover=on_drag_over
                    on:contextmenu=move |ev| on_contextmenu(ev, i)
                >
                    <Key k f pos i/>
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
fn Key(k: Key, f: Finger, pos: PhysicalKey, i: usize) -> impl IntoView {
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
        }
        false => fingermap_colors(f).to_owned(),
    };
    let title = move || format!("Key usage: {:.2}%", freq());

    view! {
        <div
            class="
            absolute flex border-[0.3cqw] border-darker items-center justify-center
            bg-darker text-darker rounded-[1cqw] container-inline-size
            "
            style:left=format!("{}%", pos.x())
            style:top=format!("{}%", pos.y())
            style:width=format!("{}%", pos.width())
            style:height=format!("{}%", pos.height())
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

    let data = create_resource(move || "/data/shai.json".to_owned(), load_json::<Data>);
    let weights = move || use_context::<GlobalWeights>().unwrap_or_default();

    view! {
        {move || {
            match data() {
                Some(Ok(data)) => view! { <RenderAnalysis data weights/> }.into_view(),
                Some(Err(e)) => err(&e).into_view(),
                None => "Loading data...".into_view(),
            }
        }}
    }
}

fn fmt_f64_stat(stat: f64, unit: &'static str) -> String {
    format!("{:.3}{unit}", stat)
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_owned()
}

#[component]
fn RenderAnalysis(data: Data, weights: impl Fn() -> GlobalWeights + 'static) -> impl IntoView {
    let phys = expect_context::<PhysicalLayout>();
    let keys = expect_context::<LayoutKeys>().0;
    // let pins = use_context::<RwSignal<Pins>>();

    let analyzer = create_memo(move |_| Analyzer::new(data.clone(), weights().into()));
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
    let score = create_memo(move |_| analyzer.with(|a| layout_memo.with(|l| a.score(l))));

    let finger_use =
        create_memo(move |_| stats_memo.with(|s| s.finger_use.map(|v| create_memo(move |_| v))));
    let finger_sfbs =
        create_memo(move |_| stats_memo.with(|s| s.finger_sfbs.map(|v| create_memo(move |_| v))));
    let weighted_finger_distance = create_memo(move |_| {
        stats_memo.with(|s| s.weighted_finger_distance.map(|v| create_memo(move |_| v)))
    });
    let unweighted_finger_distance = create_memo(move |_| {
        stats_memo.with(|s| {
            s.unweighted_finger_distance
                .map(|v| create_memo(move |_| v))
        })
    });

    let finger_stats = FingerStats {
        finger_use,
        finger_sfbs,
        weighted_finger_distance,
        unweighted_finger_distance,
    };

    let t_sft = create_memo(move |_| stats_memo.with(|s| s.trigrams.sft));
    let t_sfb = create_memo(move |_| stats_memo.with(|s| s.trigrams.sfb));
    let t_inroll = create_memo(move |_| stats_memo.with(|s| s.trigrams.inroll));
    let t_outroll = create_memo(move |_| stats_memo.with(|s| s.trigrams.outroll));
    let t_alternate = create_memo(move |_| stats_memo.with(|s| s.trigrams.alternate));
    let t_redirect = create_memo(move |_| stats_memo.with(|s| s.trigrams.redirect));
    let t_onehandin = create_memo(move |_| stats_memo.with(|s| s.trigrams.onehandin));
    let t_onehandout = create_memo(move |_| stats_memo.with(|s| s.trigrams.onehandout));
    let t_thumb = create_memo(move |_| stats_memo.with(|s| s.trigrams.thumb));
    let t_invalid = create_memo(move |_| stats_memo.with(|s| s.trigrams.invalid));

    // let is_window_lg = leptos_use::use_media_query("(min-width: 1024px)");

    view! {
        <div class="mx-auto sm:flex text-xs sm:text-sm md:text-base lg:text-lg">
            <div class="p-4 bg-header rounded-t-xl sm:rounded-b-xl">
                <StatGroup description="Bigrams">
                    <F64Stat name="sfbs:" stat=sfbs unit="%"/>
                    <F64Stat name="sfs:" stat=sfs unit="%"/>
                </StatGroup>
                <StatGroup description="Trigrams">
                    <F64Stat name="sft:" stat=t_sft unit="%"/>
                    <F64Stat name="sfb:" stat=t_sfb unit="%"/>
                    <F64Stat name="inroll:" stat=t_inroll unit="%"/>
                    <F64Stat name="outroll:" stat=t_outroll unit="%"/>
                    <F64Stat name="alternate:" stat=t_alternate unit="%"/>
                    <F64Stat name="redirect:" stat=t_redirect unit="%"/>
                    <F64Stat name="onehandin:" stat=t_onehandin unit="%"/>
                    <F64Stat name="onehandout:" stat=t_onehandout unit="%"/>
                    <F64Stat name="thumb:" stat=t_thumb unit="%"/>
                    <F64Stat name="invalid:" stat=t_invalid unit="%"/>
                </StatGroup>
                <Stat name="score:" stat=move || score().to_string()/>
            </div>
            // {move || {
            // if is_window_lg() {
            // view! {
            // <div class="hidden lg:block">
            // <HorizontalFingerStats stats=finger_stats/>
            // </div>
            // }
            // } else {
            // view! {
            // <div class="lg:hidden">
            // <VerticalFingerStats stats=finger_stats/>
            // </div>
            // }
            // }
            // }}
            <VerticalFingerStats stats=finger_stats/>
        </div>
    }
}

#[component]
fn StatGroup(description: &'static str, children: Children) -> impl IntoView {
    view! {
        <p class="font-bold">{description}</p>
        <div class="w-fit">
            <table>
                <tbody>{children()}</tbody>
            </table>
        </div>
    }
}

#[component]
fn F64Stat(
    name: &'static str,
    stat: impl Fn() -> f64 + 'static,
    unit: &'static str,
) -> impl IntoView {
    let stat = move || fmt_f64_stat(stat(), unit);

    view! {
        <tr class="py-1">
            <td class="text-left align-center">{name}</td>
            <td class="pl-3">{stat}</td>
        </tr>
    }
}

#[component]
fn Stat(name: &'static str, stat: impl Fn() -> String + 'static) -> impl IntoView {
    view! {
        <tr class="py-1">
            <td class="text-left align-center">{name}</td>
            <td class="pl-3">{stat}</td>
        </tr>
    }
}

#[component]
fn HorizontalFingerStats(stats: FingerStats) -> impl IntoView {
    view! {
        <div class="p-4 ml-2 bg-header rounded-b-xl sm:rounded-t-xl overflow-x-scroll">
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
                <HorizontalFingerStat name="Finger usage" stat=stats.finger_use/>
                <HorizontalFingerStat name="Finger sfbs" stat=stats.finger_sfbs/>
                <HorizontalFingerStat name="Finger distance" stat=stats.weighted_finger_distance/>
                <HorizontalFingerStat
                    name="Unweighted finger distance"
                    stat=stats.unweighted_finger_distance
                />
            </table>
        </div>
    }
}

#[component]
fn HorizontalFingerStat(name: &'static str, stat: FingerStat) -> impl IntoView {
    let rows = move || {
        stat()
            .into_iter()
            .map(|v| {
                view! { <td class="p-1 border border-hovered">{move || format!("{:.2}", v())}</td> }
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
fn VerticalFingerStats(stats: FingerStats) -> impl IntoView {
    let FingerStats {
        finger_use,
        finger_sfbs,
        weighted_finger_distance,
        unweighted_finger_distance,
    } = stats;

    view! {
        <div class="p-4 sm:ml-2 bg-header rounded-b-xl sm:rounded-t-xl overflow-x-scroll">
            <table class="w-full text-left border-y border-y-hovered">
                <tr>
                    <th class="p-1 border border-hovered"></th>
                    <th class="p-1 border border-hovered">"Finger usage"</th>
                    <th class="p-1 border border-hovered">"Finger sfbs"</th>
                    <th class="p-1 border border-hovered">"Finger distance"</th>
                    <th class="p-1 border border-hovered">"Unweighted finger distance"</th>
                </tr>
                {move || {
                    finger_use()
                        .into_iter()
                        .zip(finger_sfbs())
                        .zip(weighted_finger_distance())
                        .zip(unweighted_finger_distance())
                        .zip(Finger::FINGERS)
                        .map(|((((fu, fs), wfd), ufd), f)| {
                            let bg = fingermap_colors(f);
                            view! {
                                <tr>
                                    <th
                                        style:background-color=bg
                                        class="text-darker border border-hovered"
                                    >
                                        {f.to_string()}
                                    </th>
                                    <VerticalFingerStat stat=fu/>
                                    <VerticalFingerStat stat=fs/>
                                    <VerticalFingerStat stat=wfd/>
                                    <VerticalFingerStat stat=ufd/>
                                </tr>
                            }
                        })
                        .collect_view()
                }}

            </table>
        </div>
    }
}

#[component]
fn VerticalFingerStat(stat: Memo<f64>) -> impl IntoView {
    let fmt = move || format!("{:.2}", stat());

    view! { <td class="p-1 border border-hovered">{fmt}</td> }
}

fn collapse_data(data: RwSignal<Option<String>>, collapsed: ReadSignal<bool>) {
    if collapsed() {
        if data().is_none() { data.set(Some("Unknown".to_owned())) };
    } else if let Some("Unknown") = data().as_deref() { data.set(None) }
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
        <div class="sm:p-2 border border-hovered rounded-lg">
            <table class="w-full">
                <thead>
                    <tr class="bg-header">
                        <th class="text-left align-top px-2 py-1">
                            <label name="collapse-metadata">
                                <button on:click=collapse>
                                    <span>"Info"</span>
                                    <span class="absolute -mt-3 opacity-70">{info}</span>
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
        {move || data().map(|data| view! {
            <tr class="even:bg-header px-2">
                <th class="text-left align-top py-1 pr-3">{name}</th>
                <td class="text-left align-top py-1 pl-3">{data}</td>
            </tr>
        })}
    }
}
