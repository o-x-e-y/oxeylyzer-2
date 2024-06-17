use nanorand::{RandomGen, Rng, WyRand};

use crate::{
    analyze::Analyzer,
    cached_layout::CachedLayout,
    layout::{Layout, PosPair},
};

impl Analyzer {
    pub fn annealing_improve(
        &self,
        layout: Layout,
        initial_temperature: f64,
        cooling_rate: f64,
        max_iterations: usize,
    ) -> (Layout, i64) {
        let mut cache = self.cached_layout(layout);
        let mut rng = WyRand::new();

        let mut current_score = self.score_cache(&cache);
        let mut temperature = initial_temperature;

        for _ in 0..max_iterations {
            let swap = random_swap(&cache, &mut rng);

            cache.swap(swap);
            let new_score = self.score_cached_swap(&cache, swap);

            let ap = acceptance_probability(current_score, new_score, temperature);

            if ap > f64::random(&mut rng) {
                self.update_cache(&mut cache, swap);
                current_score = new_score;
            } else {
                cache.swap(swap);
            }

            temperature *= cooling_rate;
        }

        (cache.into(), current_score)
    }

    pub fn annealing_depth2_improve(
        &self,
        layout: Layout,
        initial_temperature: f64,
        cooling_rate: f64,
        max_iterations: usize,
    ) -> (Layout, i64) {
        let mut cache = self.cached_layout(layout);
        let mut rng = WyRand::new();

        let mut current_score = self.score_cache(&cache);
        let mut temperature = initial_temperature;

        for _ in 0..max_iterations {
            let [swap1, swap2] = random_swap2(&cache, &mut rng);

            cache.swap(swap1);
            self.update_cache(&mut cache, swap1);
            cache.swap(swap2);
            let new_score = self.score_cached_swap(&cache, swap2);

            let ap = acceptance_probability(current_score, new_score, temperature);

            if ap > f64::random(&mut rng) {
                self.update_cache(&mut cache, swap2);
                current_score = new_score;
            } else {
                cache.swap(swap2);
                cache.swap(swap1);
                self.update_cache(&mut cache, swap1);
            }

            temperature *= cooling_rate;
        }

        (cache.into(), current_score)
    }
}

#[inline]
fn random_swap(cache: &CachedLayout, rng: &mut WyRand) -> PosPair {
    cache.possible_swaps[rng.generate_range(0..(cache.possible_swaps.len()))]
}

#[inline]
fn random_swap2(cache: &CachedLayout, rng: &mut WyRand) -> [PosPair; 2] {
    [random_swap(cache, rng), random_swap(cache, rng)]
}

#[inline]
fn acceptance_probability(current_score: i64, new_score: i64, temperature: f64) -> f64 {
    if new_score > current_score {
        1.0
    } else {
        ((new_score - current_score) as f64 / temperature).exp()
    }
    // println!(
    //     "diff: {:<15} temp: {temperature:<20} ap: {ap}",
    //     new_score - current_score
    // );
}
