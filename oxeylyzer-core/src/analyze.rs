use itertools::Itertools;
use libdof::dofinitions::Finger;

use crate::{
    analyzer_data::AnalyzerData, cached_layout::*, char_mapping::CharMapping, data::Data,
    layout::*, weights::Weights,
};

#[derive(Debug, Clone)]
pub struct Analyzer {
    pub data: AnalyzerData,
    pub weights: Weights,
    pub analyze_bigrams: bool,
}

impl Analyzer {
    pub fn new(data: Data, weights: Weights) -> Self {
        let data = AnalyzerData::new(data, &weights);
        let analyze_bigrams = weights.sfbs != 0 || weights.sfs != 0;

        Self {
            data,
            weights,
            analyze_bigrams,
        }
    }

    pub fn score(&self, layout: &Layout) -> i64 {
        let cache = self.cached_layout(layout.clone());

        self.score_cache(&cache)
    }

    pub fn score_cache(&self, cache: &CachedLayout) -> i64 {
        // more metrics will obviously also go here
        cache.weighted_bigrams.total
    }

    pub fn mapping(&self) -> &CharMapping {
        &self.data.mapping
    }

    pub fn cached_layout(&self, layout: Layout) -> CachedLayout {
        let keys = layout
            .keys
            .iter()
            .map(|&c| self.data.mapping.get_u(c))
            .collect::<Box<_>>();

        let name = layout.name;
        let fingers = layout.fingers;
        let shape = layout.shape;
        let char_mapping = self.data.mapping.clone();
        let keyboard = layout.keyboard;

        let possible_swaps = (0..(keys.len() as u8))
            .tuple_combinations::<(_, _)>()
            .map(Into::into)
            .collect();

        let sfb_indices = SfbIndices::new(&fingers, &keyboard, &self.weights.fingers);

        let mut cache = CachedLayout {
            name,
            keys,
            fingers,
            keyboard,
            possible_swaps,
            sfb_indices,
            shape,
            char_mapping,
            ..Default::default()
        };

        let per_finger = Box::new(Finger::FINGERS.map(|f| self.finger_weighted_bigrams(&cache, f)));
        let total = per_finger.iter().sum();

        cache.weighted_bigrams = BigramCache { total, per_finger };

        cache
    }

    pub fn greedy_improve(&self, layout: Layout) -> (Layout, i64) {
        let mut cache = self.cached_layout(layout);
        let mut best_score = self.score_cache(&cache);

        while let Some((swap, score)) = self.best_swap(&mut cache) {
            if score <= best_score {
                break;
            }

            best_score = score;
            cache.swap(swap);
            self.update_cache(&mut cache, swap);
        }

        (cache.into(), best_score)
    }

    pub fn best_swap(&self, cache: &mut CachedLayout) -> Option<(PosPair, i64)> {
        let swaps = std::mem::take(&mut cache.possible_swaps);

        let res = swaps
            .iter()
            .map(|&pair| {
                cache.swap(pair);
                let score = self.score_cached_swap(cache, pair);
                cache.swap(pair);
                (pair, score)
            })
            .max_by(|(_, s1), (_, s2)| s1.cmp(s2));

        cache.possible_swaps = swaps;

        res
    }

    pub fn finger_weighted_bigrams(&self, cache: &CachedLayout, f: Finger) -> i64 {
        cache
            .sfb_indices
            .get_finger(f)
            .iter()
            .map(
                |SfbPair {
                     pair: PosPair(a, b),
                     dist,
                 }| {
                    let u1 = cache.keys[*a as usize];
                    let u2 = cache.keys[*b as usize];

                    (self.data.get_weighted_bigram_u([u1, u2])
                        + self.data.get_weighted_bigram_u([u2, u1]))
                        * dist
                },
            )
            .sum()
    }

    pub(crate) fn update_cache(&self, cache: &mut CachedLayout, swap: PosPair) {
        if swap.0 == swap.1 {
            return;
        }

        if self.analyze_bigrams {
            self.update_cache_weighted_bigrams(cache, swap);
        }
    }

    fn update_cache_weighted_bigrams(&self, cache: &mut CachedLayout, PosPair(a, b): PosPair) {
        let f1 = cache.fingers[a as usize];
        let f2 = cache.fingers[b as usize];

        if f1 == f2 {
            let b1 = self.finger_weighted_bigrams(cache, f1);

            let cache1 = cache.weighted_bigrams.per_finger[f1 as usize];

            cache.weighted_bigrams.total += b1 - cache1;
            cache.weighted_bigrams.per_finger[f1 as usize] = b1;
        } else {
            let b1 = self.finger_weighted_bigrams(cache, f1);
            let b2 = self.finger_weighted_bigrams(cache, f2);

            let cache1 = cache.weighted_bigrams.per_finger[f1 as usize];
            let cache2 = cache.weighted_bigrams.per_finger[f2 as usize];

            cache.weighted_bigrams.total += b1 + b2 - cache1 - cache2;
            cache.weighted_bigrams.per_finger[f1 as usize] = b1;
            cache.weighted_bigrams.per_finger[f2 as usize] = b2;
        }
    }

    pub fn score_cached_swap(&self, cache: &CachedLayout, swap: PosPair) -> i64 {
        self.score_swap_weighted_bigrams(cache, swap)
    }

    fn score_swap_weighted_bigrams(&self, cache: &CachedLayout, PosPair(a, b): PosPair) -> i64 {
        if self.weights.sfbs == 0 {
            return 0;
        }
        if a == b {
            return cache.weighted_bigrams.total * self.weights.sfbs;
        }

        let f1 = cache.fingers[a as usize];
        let f2 = cache.fingers[b as usize];

        if f1 == f2 {
            let b1 = self.finger_weighted_bigrams(cache, f1);

            let cache1 = cache.weighted_bigrams.per_finger[f1 as usize];

            cache.weighted_bigrams.total + b1 - cache1
        } else {
            let b1 = self.finger_weighted_bigrams(cache, f1);
            let b2 = self.finger_weighted_bigrams(cache, f2);

            let cache1 = cache.weighted_bigrams.per_finger[f1 as usize];
            let cache2 = cache.weighted_bigrams.per_finger[f2 as usize];

            cache.weighted_bigrams.total + b1 + b2 - cache1 - cache2
        }
    }

    pub fn sfbs(&self, cache: &CachedLayout) -> i64 {
        cache
            .sfb_indices
            .all
            .iter()
            .map(
                |SfbPair {
                     pair: PosPair(a, b),
                     ..
                 }| {
                    let u1 = cache.keys[*a as usize];
                    let u2 = cache.keys[*b as usize];

                    self.data.get_bigram_u([u1, u2]) + self.data.get_bigram_u([u2, u1])
                },
            )
            .sum()
    }

    pub fn sfs(&self, cache: &CachedLayout) -> i64 {
        cache
            .sfb_indices
            .all
            .iter()
            .map(
                |SfbPair {
                     pair: PosPair(a, b),
                     ..
                 }| {
                    let u1 = cache.keys[*a as usize];
                    let u2 = cache.keys[*b as usize];

                    self.data.get_skipgram_u([u1, u2]) + self.data.get_skipgram_u([u2, u1])
                },
            )
            .sum()
    }

    pub fn finger_use(&self, cache: &CachedLayout) -> [i64; 10] {
        let mut res = [0; 10];

        for (&k, &f) in cache.keys.iter().zip(cache.fingers.iter()) {
            res[f as usize] += self.data.get_char_u(k);
        }

        res
    }

    pub fn finger_sfbs(&self, cache: &CachedLayout) -> [i64; 10] {
        cache.sfb_indices.fingers.clone().map(|pairs| {
            pairs
                .iter()
                .map(
                    |SfbPair {
                         pair: PosPair(a, b),
                         ..
                     }| {
                        let u1 = cache.keys[*a as usize];
                        let u2 = cache.keys[*b as usize];

                        self.data.get_bigram_u([u1, u2]) + self.data.get_bigram_u([u2, u1])
                    },
                )
                .sum()
        })
    }

    pub fn weighted_bigrams(&self, cache: &CachedLayout) -> i64 {
        Finger::FINGERS
            .into_iter()
            .map(|f| self.finger_weighted_bigrams(cache, f))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::weights::dummy_weights;

    use super::*;

    fn analyzer_layout() -> (Analyzer, Layout) {
        let data = Data::load("../data/shai.json").expect("this should exist");

        let weights = dummy_weights();

        let analyzer = Analyzer::new(data, weights);

        let layout = Layout::load("../layouts/rstn-oxey.dof")
            .expect("this layout is valid and exists, soooo");

        (analyzer, layout)
    }

    #[test]
    fn update_cache_bigrams() {
        let (analyzer, layout) = analyzer_layout();

        let mut cache = analyzer.cached_layout(layout);
        let reference = cache.clone();

        let possible_swaps = cache.possible_swaps.clone();

        for (i, &swap) in possible_swaps.iter().enumerate() {
            let initial = analyzer.score_cache(&cache);

            cache.swap(swap);
            analyzer.update_cache_weighted_bigrams(&mut cache, swap);

            cache.swap(swap);
            analyzer.update_cache_weighted_bigrams(&mut cache, swap);

            let returned = analyzer.score_cache(&cache);

            assert_eq!(initial, returned, "iteration {i}: ");
            assert_eq!(cache, reference, "iteration {i}: ");
        }
    }
}
