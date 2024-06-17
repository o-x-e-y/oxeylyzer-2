use crate::{analyze::Analyzer, layout::Layout};

pub trait OptimizeLayout {
    fn optimize(a: &Analyzer, layout: Layout) -> (Layout, i64);
}

#[derive(Debug, Clone)]
pub struct Greedy;

#[derive(Debug, Clone)]
pub struct GreedyDepth2;

#[derive(Debug, Clone)]
pub struct GreedyDepth3;

#[derive(Debug, Clone)]
pub struct GreedyDepth4;

#[derive(Debug, Clone)]
pub struct GreedyAlternative;

#[derive(Debug, Clone)]
pub struct SimulatedAnnealing;

#[derive(Debug, Clone)]
pub struct SimulatedAnnealingDepth2;

#[derive(Clone, Copy, Debug)]
pub enum OptimizationMethod {
    Greedy,
    GreedyDepth2,
    GreedyDepth3,
    GreedyDepth4,
    GreedyAlternative,
}

impl OptimizationMethod {
    pub fn optimize(&self, a: &Analyzer, layout: Layout) -> (Layout, i64) {
        match self {
            OptimizationMethod::Greedy => Greedy::optimize(a, layout),
            OptimizationMethod::GreedyDepth2 => GreedyDepth2::optimize(a, layout),
            OptimizationMethod::GreedyDepth3 => GreedyDepth3::optimize(a, layout),
            OptimizationMethod::GreedyDepth4 => GreedyDepth4::optimize(a, layout),
            OptimizationMethod::GreedyAlternative => GreedyAlternative::optimize(a, layout),
        }
    }
}

impl OptimizeLayout for Greedy {
    fn optimize(a: &Analyzer, layout: Layout) -> (Layout, i64) {
        a.greedy_improve(layout)
    }
}

impl OptimizeLayout for GreedyDepth2 {
    fn optimize(a: &Analyzer, layout: Layout) -> (Layout, i64) {
        a.greedy_depth2_improve(layout)
    }
}

impl OptimizeLayout for GreedyDepth3 {
    fn optimize(a: &Analyzer, layout: Layout) -> (Layout, i64) {
        a.greedy_depth3_improve(layout)
    }
}

impl OptimizeLayout for GreedyDepth4 {
    fn optimize(a: &Analyzer, layout: Layout) -> (Layout, i64) {
        a.optimize_depth4(layout)
    }
}

impl OptimizeLayout for GreedyAlternative {
    fn optimize(a: &Analyzer, layout: Layout) -> (Layout, i64) {
        a.always_better_swap(layout)
    }
}

// impl OptimizeLayout<3> for SimulatedAnnealing {
//     fn optimize(a: &Analyzer, layout: Layout, [initial_temperature, cooling_rate, max_iterations]: [f64; 3]) -> (Layout, i64) {
//         a.annealing_improve(layout, initial_temperature, cooling_rate, max_iterations as usize)
//     }
// }

// impl OptimizeLayout<3> for SimulatedAnnealingDepth2 {
//     fn optimize(a: &Analyzer, layout: Layout, [initial_temperature, cooling_rate, max_iterations]: [f64; 3]) -> (Layout, i64) {
//         a.annealing_depth2_improve(layout, initial_temperature, cooling_rate, max_iterations as usize)
//     }
// }
