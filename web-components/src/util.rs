use std::{collections::HashSet, path::PathBuf};

use gloo_net::http::Request;
use leptos::*;
use libdof::prelude::{Finger, PhysicalKey};
use oxeylyzer_core::{
    prelude::Layout,
    weights::{FingerWeights, Weights},
};
use rust_embed::Embed;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Copy, Debug)]
pub struct EnableHeatmap(pub RwSignal<bool>);

#[derive(Clone, Debug)]
pub struct LayoutNames(pub Vec<String>);

#[derive(Clone, Debug)]
pub struct HeatmapTheme {
    pub low: RwSignal<(f64, f64, f64)>,
    pub high: RwSignal<(f64, f64, f64)>,
    pub curve: RwSignal<f64>,
    pub max_freq: RwSignal<f64>,
}

impl Default for HeatmapTheme {
    fn default() -> Self {
        Self {
            low: create_rw_signal((140.0, 140.0, 140.0)),
            high: create_rw_signal((255.0, 0.0, 0.0)),
            max_freq: create_rw_signal(14.0),
            curve: create_rw_signal(1.0),
        }
    }
}

#[derive(Clone, Debug)]
pub struct GlobalWeights {
    pub sfbs: RwSignal<i64>,
    pub sfs: RwSignal<i64>,
    pub sft: RwSignal<i64>,
    pub inroll: RwSignal<i64>,
    pub outroll: RwSignal<i64>,
    pub alternate: RwSignal<i64>,
    pub redirect: RwSignal<i64>,
    pub onehandin: RwSignal<i64>,
    pub onehandout: RwSignal<i64>,
    pub thumb: RwSignal<i64>,
    pub fingers: GlobalFingerWeights,
}

#[derive(Clone, Debug)]
pub struct GlobalFingerWeights {
    pub lp: RwSignal<i64>,
    pub lr: RwSignal<i64>,
    pub lm: RwSignal<i64>,
    pub li: RwSignal<i64>,
    pub lt: RwSignal<i64>,
    pub rt: RwSignal<i64>,
    pub ri: RwSignal<i64>,
    pub rm: RwSignal<i64>,
    pub rr: RwSignal<i64>,
    pub rp: RwSignal<i64>,
}

impl Default for GlobalWeights {
    fn default() -> Self {
        Self {
            sfbs: create_rw_signal(-7),
            sfs: create_rw_signal(-1),
            sft: create_rw_signal(-12),
            inroll: create_rw_signal(5),
            outroll: create_rw_signal(4),
            alternate: create_rw_signal(4),
            redirect: create_rw_signal(-1),
            onehandin: create_rw_signal(1),
            onehandout: create_rw_signal(0),
            thumb: create_rw_signal(0),
            fingers: GlobalFingerWeights {
                lp: create_rw_signal(77),
                lr: create_rw_signal(32),
                lm: create_rw_signal(24),
                li: create_rw_signal(21),
                lt: create_rw_signal(46),
                rt: create_rw_signal(46),
                ri: create_rw_signal(21),
                rm: create_rw_signal(24),
                rr: create_rw_signal(32),
                rp: create_rw_signal(77),
            },
        }
    }
}

impl From<GlobalWeights> for Weights {
    fn from(w: GlobalWeights) -> Self {
        Self {
            sfbs: w.sfbs.get(),
            sfs: w.sfs.get(),
            sft: w.sft.get(),
            inroll: w.inroll.get(),
            outroll: w.outroll.get(),
            alternate: w.alternate.get(),
            redirect: w.redirect.get(),
            onehandin: w.onehandin.get(),
            onehandout: w.onehandout.get(),
            thumb: w.thumb.get(),
            fingers: FingerWeights {
                lp: w.fingers.lp.get(),
                lr: w.fingers.lr.get(),
                lm: w.fingers.lm.get(),
                li: w.fingers.li.get(),
                lt: w.fingers.lt.get(),
                rt: w.fingers.rt.get(),
                ri: w.fingers.ri.get(),
                rm: w.fingers.rm.get(),
                rr: w.fingers.rr.get(),
                rp: w.fingers.rp.get(),
            },
        }
    }
}

#[derive(Debug, Clone, Error, Serialize, Deserialize, PartialEq)]
pub struct RequestError(String);

impl std::fmt::Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<gloo_net::Error> for RequestError {
    fn from(value: gloo_net::Error) -> Self {
        Self(value.to_string())
    }
}

impl std::ops::Deref for RequestError {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub type JsonResource<T> = Resource<String, Result<T, RequestError>>;
// pub type TextResource = Resource<String, Result<String, RequestError>>;

pub async fn load_json<T: for<'a> Deserialize<'a>>(url: String) -> Result<T, RequestError> {
    let text = Request::get(&url).send().await?.json::<T>().await?;

    Ok(text)
}

pub async fn load_text(url: String) -> Result<String, RequestError> {
    let text = Request::get(&url).send().await?.text().await?;

    Ok(text)
}

pub fn embedded_names<R: Embed>() -> Vec<String> {
    let mut names = R::iter()
        .flat_map(|s| {
            PathBuf::from(s.to_string())
                .file_stem()
                .map(ToOwned::to_owned)
        })
        .flat_map(|os| os.into_string())
        .map(|s| (s.to_lowercase(), s))
        .collect::<Vec<_>>();

    names.sort_by(|(l1, _), (l2, _)| l1.cmp(l2));

    names.into_iter().map(|(_, s)| s).collect()
}

pub fn fingermap_colors(f: Finger) -> &'static str {
    match f {
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
}

pub fn hex_to_rgb(hex: &str) -> Option<(f64, f64, f64)> {
    let hex = hex.trim().trim_start_matches('#');

    match (hex.get(..2), hex.get(2..4), hex.get(4..)) {
        (Some(s1), Some(s2), Some(s3)) => {
            match (
                u8::from_str_radix(s1, 16),
                u8::from_str_radix(s2, 16),
                u8::from_str_radix(s3, 16),
            ) {
                (Ok(n1), Ok(n2), Ok(n3)) => Some((n1 as f64, n2 as f64, n3 as f64)),
                _ => None,
            }
        }
        _ => None,
    }
}

pub fn rgb_to_hex((r, g, b): (f64, f64, f64)) -> String {
    let n1 = r.clamp(0.0, 255.0) as u8;
    let n2 = g.clamp(0.0, 255.0) as u8;
    let n3 = b.clamp(0.0, 255.0) as u8;

    format!("#{n1:02x}{n2:02x}{n3:02x}")
}

pub fn heatmap_gradient(freq: f64, theme: HeatmapTheme) -> String {
    let freq = freq
        .powf(theme.curve.get())
        .min(theme.max_freq.get())
        .max(0.0);

    let factor = freq / theme.max_freq.get();
    let start = theme.low.get();
    let end = theme.high.get();

    let r = (start.0 + factor * (end.0 - start.0)) as u16;
    let g = (start.1 + factor * (end.1 - start.1)) as u16;
    let b = (start.2 + factor * (end.2 - start.2)) as u16;

    format!("rgb({r}, {g}, {b})")
}

pub fn minmax_x(keys: &[PhysicalKey]) -> (f64, f64) {
    let min = keys
        .iter()
        .map(|p| p.x())
        .min_by(|a, b| a.total_cmp(b))
        .unwrap_or_default();

    let max = keys
        .iter()
        .map(|p| p.x() + p.width())
        .max_by(|a, b| a.total_cmp(b))
        .unwrap_or_default();

    (min, max)
}

pub fn minmax_y(keys: &[PhysicalKey]) -> (f64, f64) {
    let min = keys
        .iter()
        .map(|p| p.y())
        .min_by(|a, b| a.total_cmp(b))
        .unwrap_or_default();

    let max = keys
        .iter()
        .map(|p| p.y() + p.height())
        .max_by(|a, b| a.total_cmp(b))
        .unwrap_or_default();

    (min, max)
}

pub fn _pin_positions(layout: &Layout, pin_chars: String) -> Vec<usize> {
    match pin_chars.len() {
        0 => vec![],
        1 => {
            let find = &pin_chars.chars().next().unwrap();

            match layout.keys.iter().position(|c| find == c) {
                Some(i) => vec![i],
                None => vec![],
            }
        }
        _ => {
            let m = HashSet::<char>::from_iter(pin_chars.chars());

            layout
                .keys
                .iter()
                .enumerate()
                .filter_map(|(i, k)| m.contains(k).then_some(i))
                .collect()
        }
    }
}
