use std::collections::HashSet;

use gloo_net::http::Request;
use leptos::*;
use oxeylyzer_core::prelude::{Data, Layout, Weights};
use serde::Deserialize;

pub type JsonResource<T> = Resource<String, Result<T, String>>;

pub async fn load_json<T: for<'a> Deserialize<'a>>(url: String) -> Result<T, String> {
    async fn load<T: for<'a> Deserialize<'a>>(url: String) -> Result<T, gloo_net::Error> {
        Request::get(&url).send().await?.json::<T>().await
    }

    load(url).await.map_err(|e| e.to_string())
}

pub async fn load_text(url: &str) -> Result<String, gloo_net::Error> {
    Request::get(url).send().await?.text().await
}

pub fn minmax_x(layout: &Layout) -> (f64, f64) {
    let min = layout
        .keyboard
        .iter()
        .map(|p| p.x())
        .min_by(|a, b| a.total_cmp(b))
        .unwrap_or_default();

    let max = layout
        .keyboard
        .iter()
        .map(|p| p.x() + p.width())
        .max_by(|a, b| a.total_cmp(b))
        .unwrap_or_default();

    (min, max)
}

pub fn minmax_y(layout: &Layout) -> (f64, f64) {
    let min = layout
        .keyboard
        .iter()
        .map(|p| p.y())
        .min_by(|a, b| a.total_cmp(b))
        .unwrap_or_default();

    let max = layout
        .keyboard
        .iter()
        .map(|p| p.y() + p.height())
        .max_by(|a, b| a.total_cmp(b))
        .unwrap_or_default();

    (min, max)
}

pub fn pin_positions(layout: &Layout, pin_chars: String) -> Vec<usize> {
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
