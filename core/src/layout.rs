use libdof::prelude::{Dof, Finger, Keyboard, PhysicalKey, Shape};
use nanorand::{tls_rng, Rng as _};

use crate::{
    cached_layout::CachedLayout, Result, REPEAT_KEY, REPLACEMENT_CHAR, SHIFT_CHAR, SPACE_CHAR,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PosPair(pub u8, pub u8);

impl<U: Into<u8>> From<(U, U)> for PosPair {
    fn from((p1, p2): (U, U)) -> Self {
        Self(p1.into(), p2.into())
    }
}

// #[derive(Debug, Clone, Copy, PartialEq)]
// pub struct PhysicalPos(pub i64, pub i64);

// impl PhysicalPos {
//     pub fn dist(&self, other: &Self) -> i64 {
//         let x = self.0.abs_diff(other.0) as f64;
//         let y = self.1.abs_diff(other.1) as f64;

//         x.hypot(y) as i64
//     }

//     pub fn dist_squared(&self, other: &Self) -> i64 {
//         (self.0 - other.0).pow(2) + (self.1 - other.1).pow(2)
//     }
// }

// impl<F: Into<f64>> From<(F, F)> for PhysicalPos {
//     fn from((x, y): (F, F)) -> Self {
//         Self((x.into() * 100.0) as i64, (y.into() * 100.0) as i64)
//     }
// }

// impl From<PhysicalKey> for PhysicalPos {
//     fn from(pk: PhysicalKey) -> Self {
//         let x = pk.x() + (0.5 * pk.width());
//         let y = pk.y() + (0.5 * pk.height());

//         (x, y).into()
//     }
// }

#[derive(Debug, Clone, PartialEq)]
pub struct Layout {
    pub name: String,
    pub keys: Box<[char]>,
    pub fingers: Box<[Finger]>,
    pub keyboard: Box<[PhysicalKey]>,
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
    #[cfg(not(target_arch = "wasm32"))]
    pub fn load<P: AsRef<std::path::Path>>(path: P) -> Result<Self> {
        let s = std::fs::read_to_string(path)?;

        serde_json::from_str::<Dof>(&s)
            .map(Into::into)
            .map_err(Into::into)
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn load(url: &str) -> Result<Self> {
        let dof = gloo_net::http::Request::get(url)
            .send()
            .await?
            .json::<Dof>()
            .await?;

        Ok(dof.into())
    }

    pub fn random(&self) -> Self {
        self.random_with_pins(&[])
    }

    pub fn random_with_pins(&self, pins: &[usize]) -> Self {
        let shape = self.shape.clone();
        let fingers = self.fingers.clone();
        let keyboard = self.keyboard.clone();

        let mut keys = self.keys.clone();
        shuffle_pins(&mut keys, pins);

        Self {
            name: keys.iter().collect(),
            keys,
            fingers,
            keyboard,
            shape,
        }
    }
}

impl From<Dof> for Layout {
    fn from(dof: Dof) -> Self {
        use libdof::prelude::{Key, SpecialKey};

        let keys = dof
            .main_layer()
            .keys()
            .map(|k| match k {
                Key::Char(c) => *c,
                Key::Special(s) => match s {
                    SpecialKey::Repeat => REPEAT_KEY,
                    SpecialKey::Space => SPACE_CHAR,
                    SpecialKey::Shift => SHIFT_CHAR,
                    _ => REPLACEMENT_CHAR,
                },
                _ => REPLACEMENT_CHAR,
            })
            .collect();

        let name = dof.name().to_owned();
        let fingers = dof.fingering().keys().copied().collect();
        let keyboard = dof.board().keys().cloned().map(Into::into).collect();
        let shape = dof.main_layer().shape();

        Layout {
            name,
            keys,
            fingers,
            keyboard,
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
            keyboard: layout.keyboard,
            shape: layout.shape,
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
