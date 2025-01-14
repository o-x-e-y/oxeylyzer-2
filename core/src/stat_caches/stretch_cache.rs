use fxhash::FxHashMap as HashMap;
use itertools::Itertools;
use libdof::prelude::{Finger, PhysicalKey};

use crate::{analyzer_data::AnalyzerData, layout::PosPair, stat_caches::*};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct StretchCache {
    pub all_pairs: Box<[BigramPair]>,
    pub per_keypair: HashMap<PosPair, Box<[BigramPair]>>,
    pub total: i64,
}

impl StretchCache {
    pub fn new(
        keys: &[char],
        fingers: &[Finger],
        keyboard: &[PhysicalKey],
        data: &AnalyzerData,
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

        let all_pairs = keyboard
            .iter()
            .zip(fingers)
            .zip(keys)
            .enumerate()
            .tuple_combinations::<(_, _)>()
            .filter(|((_, ((_, &f1), _)), (_, ((_, &f2), _)))| f1 != f2 && (f1.hand() == f2.hand()))
            .filter_map(|((i1, ((k1, &f1), _c1)), (i2, ((k2, &f2), _c2)))| {
                let diff = (f1 as u8).abs_diff(f2 as u8) as f64;
                let fd = diff * 1.35;
                // let minimum_diff = diff * 0.9;
                let (dx, dy) = dx_dy(k1, k2, f1, f2);
                let negative_lsb = 0.0; //(minimum_diff - dx.abs() - 1.0).max(0.0) * 2.0;
                let dist = dx.hypot(dy);

                let xo = x_overlap(dx, dy, f1, f2);

                let stretch = dist + xo + negative_lsb - fd;

                // if stretch > 0.001 {
                //     println!("{_c1}{_c2}: {}", (stretch * 100.0) as i64);
                // }

                (stretch > 0.001).then_some(BigramPair {
                    pair: PosPair(i1 as u8, i2 as u8),
                    dist: (stretch * 100.0) as i64,
                })
            })
            .collect::<Box<[_]>>();

        // println!("pair count: {}", all_pairs.len());

        let per_keypair = (0..(fingers.len() as u8))
            .cartesian_product(0..(fingers.len() as u8))
            .map(|(i1, i2)| {
                let is = [i1, i2];

                let pairs = all_pairs
                    .iter()
                    .filter(move |b| is.contains(&b.pair.0) || is.contains(&b.pair.1))
                    .copied()
                    .collect::<Box<[_]>>();

                (PosPair(i1, i2), pairs)
            })
            .collect::<HashMap<_, _>>();

        let total = all_pairs
            .iter()
            .map(
                |BigramPair {
                     dist,
                     pair: PosPair(a, b),
                 }| {
                    let u1 = keys[*a as usize];
                    let u2 = keys[*b as usize];

                    (data.get_stretch_weighted_bigram([u1, u2])
                        + data.get_stretch_weighted_bigram([u2, u1]))
                        * dist
                },
            )
            .sum();

        Self {
            all_pairs,
            per_keypair,
            total,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_dist() {
        let k1 = "1 0 0 0"
            .parse::<PhysicalKey>()
            .expect("couldn't create k1");

        let k2 = "2 1 0 0"
            .parse::<PhysicalKey>()
            .expect("couldn't create k2");

        let d = dist(&k1, &k2, Finger::RP, Finger::RP);

        approx::assert_abs_diff_eq!(d, 2f64.sqrt(), epsilon = 1e-9);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn test_square_shapes() {
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

            println!("{c} uses {f}, key: {p:?}")
        }

        let k1 = "6.25 3 1 1"
            .parse::<PhysicalKey>()
            .expect("couldn't create k1");

        let k2 = "3.75 4 6.25 1 "
            .parse::<PhysicalKey>()
            .expect("couldn't create k2");

        let d = dist(&k1, &k2, Finger::LP, Finger::LP);

        approx::assert_abs_diff_eq!(d, 1.0, epsilon = 1e-9);

        let layout = crate::layout::Layout::load("../layouts/qwerty.dof").unwrap();

        print_key_info(&layout, 'b');
        print_key_info(&layout, '␣');
    }
}
