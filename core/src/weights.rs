use libdof::dofinitions::Finger;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Weights {
    pub sfbs: i64,
    pub sfs: i64,
    pub sft: i64,
    pub inroll: i64,
    pub outroll: i64,
    pub alternate: i64,
    pub redirect: i64,
    pub onehandin: i64,
    pub onehandout: i64,
    pub thumb: i64,
    pub fingers: FingerWeights,
}

impl Weights {
    pub const fn has_bigram_weights(&self) -> bool {
        self.sfbs != 0 || self.sfs != 0
    }

    pub const fn has_trigram_weights(&self) -> bool {
        self.sft != 0
            || self.inroll != 0
            || self.outroll != 0
            || self.alternate != 0
            || self.redirect != 0
            || self.onehandin != 0
            || self.onehandout != 0
            || self.thumb != 0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FingerWeights {
    lp: i64,
    lr: i64,
    lm: i64,
    li: i64,
    lt: i64,
    rt: i64,
    ri: i64,
    rm: i64,
    rr: i64,
    rp: i64,
}

impl FingerWeights {
    #[inline]
    pub const fn get(&self, f: Finger) -> i64 {
        use Finger::*;

        match f {
            LP => self.lp,
            LR => self.lr,
            LM => self.lm,
            LI => self.li,
            LT => self.lt,
            RT => self.rt,
            RI => self.ri,
            RM => self.rm,
            RR => self.rr,
            RP => self.rp,
        }
    }
}

impl Default for FingerWeights {
    fn default() -> Self {
        Self {
            lp: 1,
            lr: 1,
            lm: 1,
            li: 1,
            lt: 1,
            rt: 1,
            ri: 1,
            rm: 1,
            rr: 1,
            rp: 1,
        }
    }
}

pub fn dummy_weights() -> Weights {
    Weights {
        sfbs: -7,
        sfs: -1,
        sft: -12,
        inroll: 5,
        outroll: 4,
        alternate: 4,
        redirect: -1,
        onehandin: 1,
        onehandout: 0,
        thumb: 0,
        fingers: FingerWeights {
            lp: 77,
            lr: 32,
            lm: 24,
            li: 21,
            lt: 46,
            rt: 46,
            ri: 21,
            rm: 24,
            rr: 32,
            rp: 77,
        },
    }
}
