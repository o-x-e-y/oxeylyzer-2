use crate::prelude::{Analyzer, Layout};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Stats {
    pub finger_use: [f64; 10],
    pub finger_sfbs: [f64; 10],
    pub sfbs: f64,
    pub sfs: f64,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrigramStats {
    sft: f64,
    sfb: f64,
    inroll: f64,
    outroll: f64,
    alternate: f64,
    redirect: f64,
    onehandin: f64,
    onehandout: f64,
    thumb: f64,
    invalid: f64,
}

impl Analyzer {
    pub fn stats(&self, layout: &Layout) -> Stats {
        let cache = self.cached_layout(layout.clone(), &[]);

        let finger_use = self
            .finger_use(&cache)
            .map(|u| u as f64 / self.data.char_total);

        let finger_sfbs = self
            .finger_sfbs(&cache)
            .map(|s| s as f64 / self.data.bigram_total);

        let sfbs = self.sfbs(&cache) as f64 / self.data.char_total;
        let sfs = self.sfs(&cache) as f64 / self.data.bigram_total;

        Stats {
            finger_use,
            finger_sfbs,
            sfbs,
            sfs,
        }
    }
}
