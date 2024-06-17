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
        let heatmap = cache.heatmap.as_ref().map(|h| h.total).unwrap_or(0);
        let weighted_bigrams = cache.weighted_bigrams.total;

        heatmap * self.weights.heatmap + weighted_bigrams
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

        let fingers = layout.fingers;
        let shape = layout.shape;
        let char_mapping = self.data.mapping.clone();

        let possible_swaps = (0..(keys.len() as u8))
            .tuple_combinations::<(_, _)>()
            .map(Into::into)
            .collect();

        let sfb_indices = SfbIndices::new(&fingers);

        let heatmap = match layout.heatmap {
            Some(heatmap) => {
                let per_key = keys
                    .iter()
                    .zip(heatmap.iter())
                    .map(|(&k, &h)| self.data.get_char_u(k) * h)
                    .collect::<Box<_>>();

                let total = per_key.iter().sum();

                Some(HeatmapCache {
                    total,
                    per_key,
                    map: heatmap,
                })
            }
            None => None,
        };

        let mut cache = CachedLayout {
            keys,
            fingers,
            possible_swaps,
            sfb_indices,
            heatmap,
            shape,
            char_mapping,
            ..Default::default()
        };

        cache.weighted_bigrams = BigramCache {
            total: self.weighted_bigrams(&cache),
            per_finger: Finger::FINGERS
                .map(|f| self.finger_weighted_bigrams(&cache, f))
                .into(),
        };

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
            .map(|&PosPair(a, b)| {
                let u1 = cache.keys[a as usize];
                let u2 = cache.keys[b as usize];

                self.data.get_weighted_bigram_u([u1, u2])
                    + self.data.get_weighted_bigram_u([u2, u1])
            })
            .sum()
    }

    pub(crate) fn update_cache(&self, cache: &mut CachedLayout, swap: PosPair) {
        if swap.0 == swap.1 {
            return;
        }

        if self.weights.heatmap != 0 {
            self.update_cache_heatmap(cache, swap);
        }

        if self.analyze_bigrams {
            self.update_cache_weighted_bigrams(cache, swap);
        }
    }

    fn update_cache_heatmap(&self, cache: &mut CachedLayout, PosPair(a, b): PosPair) {
        if let Some(hc) = cache.heatmap.as_mut() {
            let u1 = cache.keys[a as usize];
            let u2 = cache.keys[b as usize];

            let new1 = self.data.get_char_u(u1) * hc.map[a as usize];
            let new2 = self.data.get_char_u(u2) * hc.map[b as usize];

            let prev1 = hc.per_key[a as usize];
            let prev2 = hc.per_key[b as usize];

            hc.total += new1 + new2 - prev1 - prev2;
            hc.per_key[a as usize] = new1;
            hc.per_key[b as usize] = new2;
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
        self.score_swap_heatmap(cache, swap) + self.score_swap_weighted_bigrams(cache, swap)
    }

    fn score_swap_heatmap(&self, cache: &CachedLayout, PosPair(a, b): PosPair) -> i64 {
        match &cache.heatmap {
            Some(hc) if a == b => hc.total * self.weights.heatmap,
            Some(hc) if self.weights.heatmap != 0 => {
                let u1 = cache.keys[a as usize];
                let u2 = cache.keys[b as usize];

                let new1 = self.data.get_char_u(u1) * hc.map[a as usize];
                let new2 = self.data.get_char_u(u2) * hc.map[b as usize];

                let prev1 = hc.per_key[a as usize];
                let prev2 = hc.per_key[b as usize];

                (hc.total + new1 + new2 - prev1 - prev2) * self.weights.heatmap
            }
            _ => 0,
        }
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
            .map(|&PosPair(a, b)| {
                let u1 = cache.keys[a as usize];
                let u2 = cache.keys[b as usize];

                self.data.get_bigram_u([u1, u2]) + self.data.get_bigram_u([u2, u1])
            })
            .sum()
    }

    pub fn sfs(&self, cache: &CachedLayout) -> i64 {
        cache
            .sfb_indices
            .all
            .iter()
            .map(|&PosPair(a, b)| {
                let u1 = cache.keys[a as usize];
                let u2 = cache.keys[b as usize];

                self.data.get_skipgram_u([u1, u2]) + self.data.get_skipgram_u([u2, u1])
            })
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
                .map(|&PosPair(a, b)| {
                    let u1 = cache.keys[a as usize];
                    let u2 = cache.keys[b as usize];

                    self.data.get_bigram_u([u1, u2]) + self.data.get_bigram_u([u2, u1])
                })
                .sum()
        })
    }

    fn weighted_bigrams(&self, cache: &CachedLayout) -> i64 {
        cache
            .sfb_indices
            .all
            .iter()
            .map(|&PosPair(a, b)| {
                let u1 = cache.keys[a as usize];
                let u2 = cache.keys[b as usize];

                self.data.get_weighted_bigram_u([u1, u2])
                    + self.data.get_weighted_bigram_u([u2, u1])
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn analyzer_layout() -> (Analyzer, Layout) {
        let data = Data::load("./data/shai.json").expect("this should exist");

        let weights = Weights {
            heatmap: -1,
            sfbs: -5,
            sfs: -1,
        };

        let analyzer = Analyzer::new(data, weights);

        let layout = Layout::load("./layouts/rstn-oxey.dof")
            .expect("this layout is valid and exists, soooo");

        (analyzer, layout)
    }

    #[test]
    fn sfs_cache_eq() {
        let (analyzer, layout) = analyzer_layout();

        let cache = analyzer.cached_layout(layout);

        assert_eq!(
            cache.weighted_bigrams.total,
            cache.weighted_bigrams.per_finger.into_iter().sum::<i64>()
        );

        println!(
            "total: {}, sum: {}",
            cache.weighted_bigrams.total,
            cache.weighted_bigrams.per_finger.into_iter().sum::<i64>()
        )
    }

    #[test]
    fn update_cache_heatmap() {
        let (analyzer, layout) = analyzer_layout();

        let mut cache = analyzer.cached_layout(layout);
        let reference = cache.clone();

        let possible_swaps = std::mem::take(&mut cache.possible_swaps);

        for (i, &swap) in possible_swaps.iter().enumerate() {
            let initial = analyzer.score_cache(&cache);

            cache.swap(swap);
            analyzer.update_cache_heatmap(&mut cache, swap);

            cache.swap(swap);
            analyzer.update_cache_heatmap(&mut cache, swap);

            let returned = analyzer.score_cache(&cache);

            assert_eq!(cache.keys, reference.keys);

            let heatmap = cache.heatmap.as_ref().unwrap();
            let heatmap_ref = reference.heatmap.as_ref().unwrap();

            assert_eq!(
                heatmap.total, heatmap_ref.total,
                "heatmap totals not equal! iteration nr: {i}"
            );
            assert_eq!(
                heatmap.per_key, heatmap_ref.per_key,
                "per key not equal! iteration nr: {i}"
            );
            assert_eq!(
                initial, returned,
                "before and after scores not equal! swap: {swap:?}, iteration nr: {i}"
            );
            assert_eq!(cache, reference);
        }
    }

    #[test]
    fn update_cache_bigrams() {
        let (analyzer, layout) = analyzer_layout();

        let mut cache = analyzer.cached_layout(layout);
        let reference = cache.clone();

        let possible_swaps = std::mem::take(&mut cache.possible_swaps);

        for &swap in possible_swaps.iter() {
            let initial = analyzer.score_cache(&cache);

            cache.swap(swap);
            analyzer.update_cache_weighted_bigrams(&mut cache, swap);

            cache.swap(swap);
            analyzer.update_cache_weighted_bigrams(&mut cache, swap);

            let returned = analyzer.score_cache(&cache);

            assert_eq!(initial, returned);
            assert_eq!(cache, reference);
        }
    }
}
