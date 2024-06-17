use std::path::Path;

use libdof::prelude::{Dof, Finger, Shape};
use nanorand::{tls_rng, Rng as _};

use crate::{cached_layout::CachedLayout, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PosPair(pub u8, pub u8);

impl<U: Into<u8>> From<(U, U)> for PosPair {
    fn from((p1, p2): (U, U)) -> Self {
        PosPair(p1.into(), p2.into())
    }
}

#[derive(Debug, Clone)]
pub struct Layout {
    pub name: String,
    pub keys: Box<[char]>,
    pub fingers: Box<[Finger]>,
    pub heatmap: Option<Box<[i64]>>,
    pub shape: Shape,
}

#[inline]
fn shuffle_pins<T>(slice: &mut [T], pins: &[usize]) {
    let mapping = (0..slice.len())
        .filter(|x| !pins.contains(x))
        .collect::<Vec<_>>();
    let mut rng = tls_rng();

    for (m, &swap1) in mapping.iter().enumerate() {
        let swap2 = rng.generate_range(m..mapping.len());
        slice.swap(swap1, mapping[swap2]);
    }
}

impl Layout {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let s =
            std::fs::read_to_string(path)?;

        serde_json::from_str::<Dof>(&s)
            .map(Into::into)
            .map_err(Into::into)
    }

    pub fn random(&self) -> Self {
        self.random_with_pins(&[])
    }

    pub fn random_with_pins(&self, pins: &[usize]) -> Self {
        let shape = self.shape.clone();
        let fingers = self.fingers.clone();
        let heatmap = self.heatmap.clone();

        let mut keys = self.keys.clone();
        shuffle_pins(&mut keys, pins);

        Self {
            name: keys.iter().collect(),
            keys,
            fingers,
            heatmap,
            shape,
        }
    }
}

impl From<Dof> for Layout {
    fn from(dof: Dof) -> Self {
        let shape = dof
            .main_layer()
            .rows()
            .map(|r| r.len())
            .collect::<Vec<_>>()
            .into();

        let mut keys = Vec::new();
        let mut fingers = Vec::new();

        dof.main_layer()
            .keys()
            .zip(dof.fingering().keys().copied())
            .filter_map(|(k, f)| k.char_output().map(|c| (c, f)))
            .for_each(|(c, f)| {
                keys.push(c);
                fingers.push(f);
            });

        let heatmap = dof.heatmap().map(|h| h.keys().copied().collect::<Box<_>>());

        Layout {
            name: dof.name().into(),
            keys: keys.into(),
            fingers: fingers.into(),
            heatmap,
            shape,
        }
    }
}

impl From<CachedLayout> for Layout {
    fn from(layout: CachedLayout) -> Self {
        Self {
            name: layout.name,
            keys: layout
                .keys
                .iter()
                .map(|&u| layout.char_mapping.get_c(u))
                .collect(),
            fingers: layout.fingers,
            shape: layout.shape,
            heatmap: layout.heatmap.map(|h| h.map),
        }
    }
}

impl std::fmt::Display for Layout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.name)?;

        let mut iter = self.keys.iter();

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
