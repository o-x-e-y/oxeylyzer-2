use itertools::Itertools;
use libdof::prelude::{Finger, Shape};
use std::sync::Arc;

use crate::{char_mapping::CharMapping, layout::PosPair};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CachedLayout {
    pub name: String,
    pub keys: Box<[u8]>,
    pub fingers: Box<[Finger]>,
    pub shape: Shape,
    pub char_mapping: Arc<CharMapping>,
    pub heatmap: Option<HeatmapCache>,
    pub possible_swaps: Box<[PosPair]>,
    pub sfb_indices: SfbIndices,
    pub weighted_bigrams: BigramCache,
}

impl CachedLayout {
    #[inline]
    pub fn swap(&mut self, PosPair(k1, k2): PosPair) {
        self.keys.swap(k1 as usize, k2 as usize);
    }
}

impl std::fmt::Display for CachedLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.keys.iter().map(|&u| self.char_mapping.get_c(u));

        for l in self.shape.inner().iter() {
            let mut i = 0;
            for c in iter.by_ref() {
                write!(f, "{c} ")?;
                i += 1;

                if *l == i {
                    break;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SfbIndices {
    pub fingers: Box<[Box<[PosPair]>; 10]>,
    pub all: Box<[PosPair]>,
}

impl SfbIndices {
    pub fn new(fingers: &[Finger]) -> Self {
        assert!(
            fingers.len() <= u8::MAX as usize,
            "Too many keys to index with u8, max is {}",
            u8::MAX
        );

        let fingers: Box<[Box<[PosPair]>; 10]> = Finger::FINGERS
            .map(|finger| {
                fingers
                    .iter()
                    .zip(0u8..)
                    .filter_map(|(f, i)| (f == &finger).then_some(i))
                    .tuple_combinations::<(_, _)>()
                    .map(PosPair::from)
                    .collect::<Box<_>>()
            })
            .into();

        let all = fingers
            .iter()
            .flat_map(|f| f.iter())
            .copied()
            .collect::<Box<_>>();

        Self { fingers, all }
    }

    pub fn get_finger(&self, finger: Finger) -> &[PosPair] {
        &self.fingers[finger as usize]
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HeatmapCache {
    pub total: i64,
    pub per_key: Box<[i64]>,
    pub map: Box<[i64]>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BigramCache {
    pub total: i64,
    pub per_finger: Box<[i64; 10]>,
}
