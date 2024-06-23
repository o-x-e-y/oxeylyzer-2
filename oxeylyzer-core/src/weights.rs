use libdof::dofinitions::Finger;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Weights {
    pub sfbs: i64,
    pub sfs: i64,
    // pub sft: i64,
    // pub inroll: i64,
    // pub outroll: i64,
    // pub alternate: i64,
    // pub redirect: i64,
    // pub onehandin: i64,
    // pub onehandout: i64,
    // pub thumb: i64,
    pub fingers: FingerWeights,
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

    #[inline]
    pub fn normalized(&self) -> Self {
        let max = [
            self.lp, self.lr, self.lm, self.li, self.lt, self.rt, self.ri, self.rm, self.rr,
            self.rp,
        ]
        .into_iter()
        .max()
        .unwrap_or_default();

        FingerWeights {
            lp: (max * 100) / self.lp,
            lr: (max * 100) / self.lr,
            lm: (max * 100) / self.lm,
            li: (max * 100) / self.li,
            lt: (max * 100) / self.lt,
            rt: (max * 100) / self.rt,
            ri: (max * 100) / self.ri,
            rm: (max * 100) / self.rm,
            rr: (max * 100) / self.rr,
            rp: (max * 100) / self.rp,
        }
    }
}

pub fn dummy_weights() -> Weights {
    Weights {
        sfbs: -60000,
        sfs: -8000,
        fingers: FingerWeights {
            lp: 15,
            lr: 36,
            lm: 48,
            li: 55,
            lt: 25,
            rt: 25,
            ri: 55,
            rm: 48,
            rr: 36,
            rp: 15,
        }, // sft = -10000
           // inroll = 12000
           // outroll = 11000
           // alternate = 9500
           // redirect = -3500
           // onehandin = 10
           // onehandout = 0
           // thumb = 0
    }
}
