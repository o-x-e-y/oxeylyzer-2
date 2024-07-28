use itertools::Itertools;
use libdof::{
    // dofinitions::Hand,
    prelude::{Finger, PhysicalKey, Shape},
};
use std::sync::Arc;

use crate::{char_mapping::CharMapping, layout::PosPair, weights::FingerWeights};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CachedLayout {
    pub name: String,
    pub keys: Box<[u8]>,
    pub fingers: Box<[Finger]>,
    pub keyboard: Box<[PhysicalKey]>,
    pub shape: Shape,
    pub char_mapping: Arc<CharMapping>,
    pub possible_swaps: Box<[PosPair]>,
    pub sfb_indices: SfbIndices,
    pub weighted_bigrams: BigramCache,
    pub stretch_indices: StretchIndices,
    // pub stretch_bigrams: StretchCache,
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

#[derive(Debug, Clone, PartialEq)]
pub struct BigramPair {
    pub pair: PosPair,
    pub dist: i64,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SfbIndices {
    pub fingers: Box<[Box<[BigramPair]>; 10]>,
    pub all: Box<[BigramPair]>,
}

impl SfbIndices {
    pub fn get_finger(&self, finger: Finger) -> &[BigramPair] {
        &self.fingers[finger as usize]
    }

    pub fn new(
        fingers: &[Finger],
        keyboard: &[PhysicalKey],
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

        let fingers: Box<[Box<[BigramPair]>; 10]> = Finger::FINGERS
            .map(|finger| {
                fingers
                    .iter()
                    .zip(keyboard)
                    .zip(0u8..)
                    .filter_map(|((f, k), i)| (f == &finger).then_some((k, i)))
                    .tuple_combinations::<(_, _)>()
                    .map(|((k1, i1), (k2, i2))| BigramPair {
                        pair: PosPair(i1, i2),
                        dist: dist(k1, k2, &Finger::LP, &Finger::LP) as i64
                            * 100
                            * weights.get(finger),
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
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BigramCache {
    pub total: i64,
    pub per_finger: Box<[i64; 10]>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct StretchIndices {
    per_key: Box<[Box<[BigramPair]>]>,
}

impl StretchIndices {
    pub fn new(fingers: &[Finger], keyboard: &[PhysicalKey], keys: &[char]) -> Self {
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

        keyboard
            .iter()
            .zip(fingers)
            .zip(keys)
            .tuple_combinations::<(_, _)>()
            .filter(|(((_, f1), _), ((_, f2), _))| {
                f1 != f2 && (!((**f1 as u8) < 5) && !((**f2 as u8) < 5))
            })
            .for_each(|(((k1, f1), c1), ((k2, f2), c2))| {
                let fd = (*f1 as u8).abs_diff(*f2 as u8) as f64 * 1.3;
                // let fd = match f1.is_thumb() || f2.is_thumb() {
                //     true => fd + 4.2,
                //     false => fd,
                // };
                let fd = match f1.is_pinky() || f2.is_pinky() {
                    true => fd + 0.6,
                    false => fd,
                };

                // let td = dist(k1, k2, &Finger::LP, &Finger::LP);
                let fad = dist(k1, k2, f1, f2) - fd;

                if fad > 0.0 {
                    println!("{c1}{c2}  {:.2}", fad);
                }
            });

        // Self { per_key }

        #[cfg(not(target_arch = "wasm32"))]
        todo!();
        #[cfg(target_arch = "wasm32")]
        Self::default()
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct StretchCache {}

fn dist(k1: &PhysicalKey, k2: &PhysicalKey, f1: &Finger, f2: &Finger) -> f64 {
    let flen = |f: &Finger| match f {
        Finger::LP | Finger::RP => -0.15,
        Finger::LR | Finger::RR => 0.35,
        Finger::LM | Finger::RM => 0.25,
        Finger::LI | Finger::RI => -0.30,
        Finger::LT | Finger::RT => -1.80,
    };

    // let x_factor = |f: &Finger| match f {
    //     Finger::LP | Finger::RP => 1f64,
    //     Finger::LR | Finger::RR => 1.0,
    //     Finger::LM | Finger::RM => 1.0,
    //     Finger::LI | Finger::RI => 1.0,
    //     Finger::LT | Finger::RT => 1.4,
    // };

    // let y_factor = |f: &Finger| match f {
    //     Finger::LP | Finger::RP => 1f64,
    //     Finger::LR | Finger::RR => 1.0,
    //     Finger::LM | Finger::RM => 1.0,
    //     Finger::LI | Finger::RI => 1.0,
    //     Finger::LT | Finger::RT => 0.6,
    // };

    // let x_factor = x_factor(f1).max(x_factor(f2));
    // let y_factor = y_factor(f1).max(y_factor(f2));

    // let x_thumb_offset = match (f1, f2) {
    //     (Finger::LT, _) => (k2.x()),
    //     (_, Finger::LT) => ,
    //     (Finger::RT, _) | (_, Finger::RT)
    // }

    let l1 = k1.x() + 0.5;
    let r1 = k1.x() + 0.5 + (k1.width() - 1.0).max(0.0);
    let t1 = k1.y() + 0.5 + flen(f1);
    let b1 = k1.y() + 0.5 + (k1.height() - 1.0).max(0.0) + flen(f1);

    let l2 = k2.x() + 0.5;
    let r2 = k2.x() + 0.5 + (k2.width() - 1.0).max(0.0);
    let t2 = k2.y() + 0.5 + flen(f2);
    let b2 = k2.y() + 0.5 + (k2.height() - 1.0).max(0.0) + flen(f2);

    let dx = (l1.max(l2) - r1.min(r2)).max(0.0);
    let dy = (t1.max(t2) - b1.min(b2)).max(0.0);

    dx.hypot(dy)
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn thing() {
    fn print_key_info(layout: &crate::layout::Layout, c: char) {
        let i = match layout.keys.iter().position(|k| k == &c) {
            Some(i) => i,
            None => {
                println!("layout '{}' does not contain '{c}'", layout.name);
                return;
            }
        };

        let p = &layout.keyboard[i];
        let f = &layout.fingers[i];

        println!("{c} uses {f}\nkey: {p:?}")
    }

    let k1 = "6.25 3 1 1"
        .parse::<PhysicalKey>()
        .expect("couldn't create k1");

    let k2 = "3.75 4 6.25 1 "
        .parse::<PhysicalKey>()
        .expect("couldn't create k2");

    let d = dist(&k1, &k2, &Finger::LP, &Finger::LP);

    println!("dist: {d}");

    let layout = crate::layout::Layout::load("../layouts/qwerty.dof").unwrap();

    print_key_info(&layout, 'b');
    print_key_info(&layout, '␣');
}
