use libdof::prelude::{Finger, PhysicalKey, Shape};
use std::sync::Arc;

use crate::{char_mapping::CharMapping, layout::PosPair, stat_caches::*, REPLACEMENT_CHAR};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CachedLayout {
    pub name: String,
    pub keys: Box<[u8]>,
    pub fingers: Box<[Finger]>,
    pub keyboard: Box<[PhysicalKey]>,
    pub shape: Shape,
    pub char_mapping: Arc<CharMapping>,
    pub possible_swaps: Box<[PosPair]>,
    pub weighted_sfb_indices: SfbIndices,
    pub unweighted_sfb_indices: SfbIndices,
    pub weighted_bigrams: BigramCache,
    pub stretch_bigrams: StretchCache,
}

impl CachedLayout {
    #[inline]
    pub fn swap(&mut self, PosPair(k1, k2): PosPair) {
        self.keys.swap(k1 as usize, k2 as usize);
    }

    pub fn char(&self, pos: u8) -> Option<char> {
        let u = self.keys.get(pos as usize)?;

        match self.char_mapping.get_c(*u) {
            REPLACEMENT_CHAR => None,
            c => Some(c),
        }
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
