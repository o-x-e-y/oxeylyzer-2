use nanorand::{RandomGen, Rng, WyRand};

use crate::{
    analyze::Analyzer,
    cached_layout::CachedLayout,
    layout::{Layout, PosPair},
};
// use plotters::prelude::*;

impl Analyzer {
    pub fn annealing_improve(
        &self,
        layout: Layout,
        pins: &[usize],
        initial_temperature: f64,
        final_temperature: f64,
        max_iterations: usize,
    ) -> (Layout, i64) {
        assert!(
            initial_temperature > final_temperature,
            "Final temperature must be less than initial temperature"
        );
        assert!(
            initial_temperature > 0.0,
            "Initial temperature must be positive"
        );
        assert!(
            final_temperature > 0.0,
            "Final temperature must be positive"
        );

        let mut cache = self.cached_layout(layout, pins);
        let mut rng = WyRand::new();
        let mut best = cache.clone();

        let mut best_score = self.score_cache(&cache);
        let mut current_score = best_score;
        let mut worst_score = current_score;
        let mut temperature = initial_temperature;

        // Calculate cooling rate based on final temperature
        let cooling_rate =
            (final_temperature / initial_temperature).powf(1.0 / max_iterations as f64);
        assert!(cooling_rate < 1.0, "Cooling rate must be < 1.0");

        // let mut scores = Vec::<i64>::new();

        for _ in 0..max_iterations {
            let swap = random_swap(&cache, &mut rng);

            // Test the swap
            cache.swap(swap);
            self.update_cache(&mut cache, swap);
            let new_score = self.score_cache(&cache);

            if new_score < worst_score {
                worst_score = new_score;
            }
            if new_score > best_score {
                best_score = new_score;
                best = cache.clone();
            }

            // Odds we take this swap
            let ap = acceptance_probability(current_score, new_score, worst_score, temperature);

            if ap > f64::random(&mut rng) {
                current_score = new_score;
            } else {
                // Undo the swap
                cache.swap(swap);
                self.update_cache(&mut cache, swap);
            }
            // scores.push(current_score);

            temperature *= cooling_rate;
        }
        // plot_line_chart(scores).expect("Failed to plot line chart");

        (best.into(), best_score)
    }
}

// fn plot_line_chart(data: Vec<i64>) -> Result<(), Box<dyn std::error::Error>> {
//     // Create a drawing area with the specified dimensions
//     let root = BitMapBackend::new("line_plot.png", (800, 600)).into_drawing_area();
//     root.fill(&WHITE)?;

//     // Create a chart builder
//     let mut chart = ChartBuilder::on(&root)
//         .caption("Line Plot", ("sans-serif", 50))
//         .margin(5)
//         .x_label_area_size(40)
//         .y_label_area_size(40)
//         .build_cartesian_2d(
//             0..data.len() as i64,
//             *data.iter().min().unwrap()..*data.iter().max().unwrap(),
//         )?;

//     // Draw a line plot
//     chart.draw_series(LineSeries::new(
//         data.iter().enumerate().map(|(x, &y)| (x as i64, y)),
//         &RED,
//     ))?;

//     // Add a grid and labels for x and y axes
//     chart.configure_mesh().draw()?;

//     // Save the plot
//     Ok(())
// }

#[inline]
fn random_swap(cache: &CachedLayout, rng: &mut WyRand) -> PosPair {
    cache.possible_swaps[rng.generate_range(0..(cache.possible_swaps.len()))]
}

#[inline]
fn acceptance_probability(
    current_score: i64,
    new_score: i64,
    worst_score: i64,
    temperature: f64,
) -> f64 {
    // Good scores are less negative (greater) than bad scores
    // ie. qwerty: -721853925651
    //     sturdy: -169325667674
    if new_score > current_score {
        // Always take the new layout if it's better
        1.0
    } else {
        // If worse, odds of taking new layout vary with temperature
        // Use worst score to normalize the objective function (or temperature values don't make sense)
        (((new_score - current_score) as f64 / worst_score.abs() as f64) / temperature).exp()
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::Data;
    use crate::weights::dummy_weights;

    #[test]
    fn test_acceptance_probability() {
        // New score is better
        let current_score = -100;
        let new_score = -90;
        let worst_score = -100;
        let temperature = 10.0;
        let ap = acceptance_probability(current_score, new_score, worst_score, temperature);
        assert_eq!(ap, 1.0);

        // new score is worse
        let current_score = -90;
        let new_score = -100;
        let worst_score = -100;
        let temperature = 10.0;
        let ap = acceptance_probability(current_score, new_score, worst_score, temperature);
        let target = (-0.01f64).exp();
        assert_eq!(ap, target);
    }

    #[test]
    fn test_annealing_improve() {
        let data = Data::load("../data/english.json").expect("this should exist");
        let weights = dummy_weights();
        let analyzer = Analyzer::new(data, weights);
        let layout = Layout::load(format!("../layouts/sturdy.dof"))
            .expect("this layout is valid and exists, soooo");
        let pins = vec![];
        let initial_temperature = 0.0001;
        let max_iterations = 1000;

        let qwerty_score = analyzer.score(&layout);
        let (new_layout, score) =
            analyzer.annealing_improve(layout, &pins, initial_temperature, 0.95, max_iterations);
        assert_eq!(score, analyzer.score(&new_layout));

        // After 1000 iterations, this better have made it better
        assert!(score > qwerty_score);
    }
}
