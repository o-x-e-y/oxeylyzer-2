use std::{collections::HashSet, path::PathBuf};

use gloo_net::http::Request;
use leptos::*;
use libdof::prelude::PhysicalKey;
use oxeylyzer_core::prelude::Layout;
use rust_embed::Embed;
use serde::{Deserialize, Serialize};
use thiserror::Error;

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

// pub fn font_mod(keys: &[PhysicalKey]) -> f64 {
//     let x = keys
//         .iter()
//         .map(|p| p.width())
//         .min_by(|a, b| a.total_cmp(b))
//         .unwrap_or_default();

//     let y = keys
//         .iter()
//         .map(|p| p.height())
//         .max_by(|a, b| a.total_cmp(b))
//         .unwrap_or_default();

//     x.min(y)
// }

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
