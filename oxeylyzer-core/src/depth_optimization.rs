use crate::{
    analyze::Analyzer,
    cached_layout::CachedLayout,
    layout::{Layout, PosPair},
};

impl Analyzer {
    pub fn always_better_swap(&self, layout: Layout, pins: &[usize]) -> (Layout, i64) {
        let mut cache = self.cached_layout(layout, pins);
        let mut best_score = self.score_cache(&cache);

        let swaps = std::mem::take(&mut cache.possible_swaps);

        loop {
            let mut best_loop_score = i64::MIN;

            for &pair in swaps.iter() {
                cache.swap(pair);
                let score = self.score_cached_swap(&cache, pair);

                if score > best_score {
                    best_loop_score = score;
                    self.update_cache(&mut cache, pair);
                    break;
                }
                cache.swap(pair);
            }

            if best_loop_score <= best_score {
                break;
            }

            best_score = best_loop_score;
        }

        (cache.into(), best_score)
    }

    pub fn alternative_d3(&self, layout: Layout, pins: &[usize]) -> (Layout, i64) {
        let (layout, _) = self.always_better_swap(layout, pins);
        self.greedy_depth3_improve(layout, pins)
    }

    pub fn optimize_depth3(&self, layout: Layout, pins: &[usize]) -> (Layout, i64) {
        let (layout, _) = self.greedy_improve(layout, pins);
        let (layout, _) = self.greedy_depth2_improve(layout, pins);
        self.greedy_depth3_improve(layout, pins)
    }

    pub fn optimize_depth4(&self, layout: Layout, pins: &[usize]) -> (Layout, i64) {
        let (layout, _) = self.greedy_improve(layout, pins);
        let (layout, _) = self.greedy_depth2_improve(layout, pins);
        let (layout, _) = self.greedy_depth3_improve(layout, pins);
        self.greedy_depth4_improve(layout, pins)
    }

    pub fn greedy_depth2_improve(&self, layout: Layout, pins: &[usize]) -> (Layout, i64) {
        let mut cache = self.cached_layout(layout, pins);
        let mut best_score = self.score_cache(&cache);

        while let Some((swaps, score)) = self.best_swap_depth2(&mut cache) {
            if score <= best_score {
                break;
            }

            best_score = score;
            for &swap in swaps.iter() {
                cache.swap(swap);
                self.update_cache(&mut cache, swap);
            }
        }

        (cache.into(), best_score)
    }

    pub fn greedy_depth4_improve(&self, layout: Layout, pins: &[usize]) -> (Layout, i64) {
        let mut cache = self.cached_layout(layout, pins);
        let mut best_score = self.score_cache(&cache);

        while let Some((swaps, score)) = self.best_swap_depth4(&mut cache) {
            if score <= best_score {
                break;
            }

            best_score = score;
            for &swap in swaps.iter() {
                cache.swap(swap);
                self.update_cache(&mut cache, swap);
            }
        }

        (cache.into(), best_score)
    }

    pub fn greedy_depth3_improve(&self, layout: Layout, pins: &[usize]) -> (Layout, i64) {
        let mut cache = self.cached_layout(layout, pins);
        let mut best_score = self.score_cache(&cache);

        while let Some((swaps, score)) = self.best_swap_depth3(&mut cache) {
            if score <= best_score {
                break;
            }

            best_score = score;
            for &swap in swaps.iter() {
                cache.swap(swap);
                self.update_cache(&mut cache, swap);
            }
        }

        (cache.into(), best_score)
    }

    pub fn best_swap_depth4(&self, cache: &mut CachedLayout) -> Option<(Box<[PosPair]>, i64)> {
        let depth1 = self.best_swap(cache).map(|(s, score)| ([s].into(), score));
        let depth2 = self.best_swap_depth2(cache);
        let depth3 = self.best_swap_depth3(cache);

        let possible_swaps = cache.possible_swaps.clone();

        let mut depth4_score = i64::MIN;
        let mut depth4_swap = None;

        for (&swap1, i) in possible_swaps.iter().zip(1..) {
            cache.swap(swap1);
            self.update_cache(cache, swap1);

            for (&swap2, j) in possible_swaps.iter().zip(1..).skip(i) {
                cache.swap(swap2);
                self.update_cache(cache, swap2);

                for (&swap3, k) in possible_swaps.iter().zip(1..).skip(j) {
                    cache.swap(swap3);
                    self.update_cache(cache, swap3);

                    for &swap4 in possible_swaps.iter().skip(k) {
                        cache.swap(swap4);
                        let current_score = self.score_cached_swap(cache, swap4);
                        cache.swap(swap4);

                        if current_score > depth4_score {
                            depth4_score = current_score;
                            depth4_swap = Some([swap1, swap2, swap3, swap4]);
                        }
                    }

                    cache.swap(swap3);
                    self.update_cache(cache, swap3);
                }

                cache.swap(swap2);
                self.update_cache(cache, swap2);
            }

            cache.swap(swap1);
            self.update_cache(cache, swap1);
        }

        let depth4 = depth4_swap.map(|s| (s.into(), depth4_score));

        [depth1, depth2, depth3, depth4]
            .into_iter()
            .flatten()
            .max_by(|(_, s1), (_, s2)| s1.cmp(s2))
    }

    pub fn best_swap_depth3(&self, cache: &mut CachedLayout) -> Option<(Box<[PosPair]>, i64)> {
        let depth1 = self.best_swap(cache).map(|(s, score)| ([s].into(), score));
        let depth2 = self.best_swap_depth2(cache);

        let possible_swaps = cache.possible_swaps.clone();

        let mut depth3_score = i64::MIN;
        let mut depth3_swap = None;

        for (&swap1, i) in possible_swaps.iter().zip(1..) {
            cache.swap(swap1);
            self.update_cache(cache, swap1);

            for (&swap2, j) in possible_swaps.iter().zip(1..).skip(i) {
                cache.swap(swap2);
                self.update_cache(cache, swap2);

                for &swap3 in possible_swaps.iter().skip(j) {
                    cache.swap(swap3);
                    let current_score = self.score_cached_swap(cache, swap3);
                    cache.swap(swap3);

                    if current_score > depth3_score {
                        depth3_score = current_score;
                        depth3_swap = Some([swap1, swap2, swap3]);
                    }
                }

                cache.swap(swap2);
                self.update_cache(cache, swap2);
            }

            cache.swap(swap1);
            self.update_cache(cache, swap1);
        }

        let depth3 = depth3_swap.map(|s| (s.into(), depth3_score));

        [depth1, depth2, depth3]
            .into_iter()
            .flatten()
            .max_by(|(_, s1), (_, s2)| s1.cmp(s2))
    }

    pub fn best_swap_depth2(&self, cache: &mut CachedLayout) -> Option<(Box<[PosPair]>, i64)> {
        let depth1 = self.best_swap(cache).map(|(s, score)| ([s].into(), score));

        let possible_swaps = cache.possible_swaps.clone();

        let mut depth2_score = i64::MIN;
        let mut depth2_swap = None;

        for (&swap1, i) in possible_swaps.iter().zip(1usize..) {
            cache.swap(swap1);
            self.update_cache(cache, swap1);

            for &swap2 in possible_swaps.iter().skip(i) {
                cache.swap(swap2);
                let current_score = self.score_cached_swap(cache, swap2);
                cache.swap(swap2);

                if current_score > depth2_score {
                    depth2_score = current_score;
                    depth2_swap = Some([swap1, swap2]);
                }
            }

            cache.swap(swap1);
            self.update_cache(cache, swap1);
        }

        let depth2 = depth2_swap.map(|s| (s.into(), depth2_score));

        [depth1, depth2]
            .into_iter()
            .flatten()
            .max_by(|(_, s1), (_, s2)| s1.cmp(s2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn analyzer_layout() -> (Analyzer, Layout) {
        let data = crate::prelude::Data::load("../data/shai.json").expect("this should exist");

        let weights = crate::weights::dummy_weights();

        let analyzer = Analyzer::new(data, weights);

        let layout = Layout::load("../layouts/rstn-oxey.dof")
            .expect("this layout is valid and exists, soooo");

        (analyzer, layout)
    }

    #[test]
    fn cache_intact() {
        let (analyzer, layout) = analyzer_layout();
        let mut cache = analyzer.cached_layout(layout, &[]);
        let reference = cache.clone();

        analyzer.best_swap(&mut cache);

        assert_eq!(cache, reference);

        analyzer.best_swap_depth2(&mut cache);

        assert_eq!(cache, reference);

        analyzer.best_swap_depth3(&mut cache);

        assert_eq!(cache, reference);
    }
}
