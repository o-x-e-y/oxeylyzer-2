use std::{collections::HashSet, path::PathBuf};

use gloo_net::http::Request;
use leptos::*;
use libdof::prelude::{Finger, PhysicalKey};
use oxeylyzer_core::prelude::Layout;
use rust_embed::Embed;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::HeatmapTheme;

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

pub fn embedded_names<R: Embed>() -> impl Iterator<Item = String> {
    R::iter()
        .flat_map(|s| {
            PathBuf::from(s.to_string())
                .file_stem()
                .map(ToOwned::to_owned)
        })
        .flat_map(|os| os.into_string())
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

pub fn heatmap_gradient(freq: f64, theme: HeatmapTheme) -> String {
    let freq = freq.powf(theme.curve.get()).min(theme.max_freq.get()).max(0.0);

    // #90ccca
    // #72d7f1 to #e04546
    // #9890e3 to #b1f4cf

    let factor = freq.powf(theme.curve.get()) / theme.max_freq.get().powf(theme.curve.get());
    let start = theme.low.get(); //(66.0 * 1.05, 120.0 * 1.05, 128.0 * 1.05);
    let end = theme.high.get(); //(255.0, 16.0, 16.0);

    // let start = (114.0, 215.0, 241.0);
    // let end = (224.0, 69.0, 70.0);

    // let end = (255.0, 32.0, 32.0);
    // let start = (end.0 / 2.4, end.1 / 2.4, end.2 / 2.4);

    // let start = (201.0, 159.0, 179.0);
    // let end = (87.0, 14.0, 75.0);

    // let start = (152.0 / 1.2, 144.0 / 1.2, 227.0 / 1.2);
    // let end = (177.0 / 1.2, 244.0 / 1.2, 207.0 / 1.2);

    // let start = (160.0, 160.0, 160.0);
    // let end = (255.0, 0.0, 0.0);
    // let end = (177.0 * 1.1, 244.0 * 1.1, 207.0 * 1.1);

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
