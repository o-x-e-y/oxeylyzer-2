use itertools::Itertools;
use libdof::prelude::{Finger, Shape};
use std::sync::Arc;

use crate::{
    char_mapping::CharMapping,
    layout::{PhysicalPos, PosPair},
    weights::FingerWeights,
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CachedLayout {
    pub name: String,
    pub keys: Box<[u8]>,
    pub fingers: Box<[Finger]>,
    pub keyboard: Box<[PhysicalPos]>,
    pub shape: Shape,
    pub char_mapping: Arc<CharMapping>,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SfbPair {
    pub pair: PosPair,
    pub dist: i64,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SfbIndices {
    pub fingers: Box<[Box<[SfbPair]>; 10]>,
    pub all: Box<[SfbPair]>,
}

impl SfbIndices {
    pub fn new(
        fingers: &[Finger],
        keyboard: &[PhysicalPos],
        finger_weights: &FingerWeights,
    ) -> Self {
        assert!(
            fingers.len() <= u8::MAX as usize,
            "Too many keys to index with u8, max is {}",
            u8::MAX
        );
        assert_eq!(
            fingers.len(),
            keyboard.len(),
            "finger len is not the same as keyboard len: "
        );

        let weights = finger_weights.normalized();

        let fingers: Box<[Box<[SfbPair]>; 10]> = Finger::FINGERS
            .map(|finger| {
                fingers
                    .iter()
                    .zip(keyboard)
                    .zip(0u8..)
                    .filter_map(|((f, k), i)| (f == &finger).then_some((k, i)))
                    .tuple_combinations::<(_, _)>()
                    .map(|((k1, i1), (k2, i2))| SfbPair {
                        pair: PosPair(i1, i2),
                        dist: k1.dist(k2) * weights.get(finger),
                    })
                    .collect::<Box<_>>()
            })
            .into();

        let all = fingers
            .iter()
            .flat_map(|f| f.iter())
            .cloned()
            .collect::<Box<_>>();

        Self { fingers, all }
    }

    pub fn get_finger(&self, finger: Finger) -> &[SfbPair] {
        &self.fingers[finger as usize]
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BigramCache {
    pub total: i64,
    pub per_finger: Box<[i64; 10]>,
}
