use crate::{
    analyze::TrigramData,
    prelude::{Analyzer, Layout},
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Stats {
    pub finger_use: [f64; 10],
    pub finger_sfbs: [f64; 10],
    pub weighted_finger_distance: [f64; 10],
    pub unweighted_finger_distance: [f64; 10],
    pub sfbs: f64,
    pub sfs: f64,
    pub trigrams: TrigramStats,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrigramStats {
    pub sft: f64,
    pub sfb: f64,
    pub inroll: f64,
    pub outroll: f64,
    pub alternate: f64,
    pub redirect: f64,
    pub onehandin: f64,
    pub onehandout: f64,
    pub thumb: f64,
    pub invalid: f64,
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

        let weighted_finger_distance = self
            .weighted_finger_distance(&cache)
            .map(|s| s as f64 / ((self.data.bigram_total + self.data.skipgram_total) * 100.0));

        let unweighted_finger_distance = self
            .unweighted_finger_distance(&cache)
            .map(|s| s as f64 / ((self.data.bigram_total + self.data.skipgram_total) * 100.0));

        let sfbs = self.sfbs(&cache) as f64 / self.data.char_total;
        let sfs = self.sfs(&cache) as f64 / self.data.bigram_total;

        let trigrams = self.trigram_stats(self.trigrams(&cache));

        Stats {
            finger_use,
            finger_sfbs,
            weighted_finger_distance,
            unweighted_finger_distance,
            sfbs,
            sfs,
            trigrams,
        }
    }

    pub fn trigram_stats(&self, trigrams: TrigramData) -> TrigramStats {
        TrigramStats {
            sft: trigrams.sft as f64 / self.data.trigram_total,
            sfb: trigrams.sfb as f64 / self.data.trigram_total,
            inroll: trigrams.inroll as f64 / self.data.trigram_total,
            outroll: trigrams.outroll as f64 / self.data.trigram_total,
            alternate: trigrams.alternate as f64 / self.data.trigram_total,
            redirect: trigrams.redirect as f64 / self.data.trigram_total,
            onehandin: trigrams.onehandin as f64 / self.data.trigram_total,
            onehandout: trigrams.onehandout as f64 / self.data.trigram_total,
            thumb: trigrams.thumb as f64 / self.data.trigram_total,
            invalid: trigrams.invalid as f64 / self.data.trigram_total,
        }
    }
}
