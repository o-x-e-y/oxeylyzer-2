use libdof::prelude::{Finger as DofFinger, Finger::*};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TrigramType {
    Sft,
    Sfb,
    // Repeat,
    Inroll,
    Outroll,
    Alternate,
    Redirect,
    OnehandIn,
    OnehandOut,
    Thumb,
    Invalid,
}

#[derive(Debug, Clone, Copy)]
enum Hand {
    Left,
    Right,
}

impl Hand {
    const fn eq(self, rhs: Self) -> bool {
        self as u8 == rhs as u8
    }
}

#[derive(Debug, Clone, Copy)]
struct Finger(DofFinger);

impl Finger {
    const fn eq(self, rhs: Self) -> bool {
        self.0 as u8 == rhs.0 as u8
    }

    const fn hand(self) -> Hand {
        match self.0 {
            LP | LR | LM | LI | LT => Hand::Left,
            RP | RR | RM | RI | RT => Hand::Right,
        }
    }

    const fn is_thumb(self) -> bool {
        matches!(self.0, RT | LT)
    }

    const fn _is_index(self) -> bool {
        matches!(self.0, LI | RI)
    }

    const fn _is_non_index(self) -> bool {
        !(self.is_thumb() || self._is_index())
    }

    const fn is_inward(self, rhs: Self) -> bool {
        matches!(
            (self.0, rhs.0),
            (LP, LR | LM | LI | LT)
                | (RP, RP | RM | RI | RT)
                | (LR, LM | LI | LT)
                | (RR, RM | RI | RT)
                | (LM, LI | LT)
                | (RM, RI | RT)
                | (LI, LT)
                | (RI, RT)
        )
    }
}

#[derive(Debug, Clone, Copy)]
struct Trigram([Finger; 3]);

impl Trigram {
    const fn new([f1, f2, f3]: [DofFinger; 3]) -> Self {
        Self([Finger(f1), Finger(f2), Finger(f3)])
    }

    const fn is_sft(&self) -> bool {
        let [f1, f2, f3] = self.0;

        f1.eq(f2) && f2.eq(f3)
    }

    const fn is_sfb(&self) -> bool {
        let [f1, f2, f3] = self.0;

        !self.is_sft() && (f1.eq(f2) || f2.eq(f3))
    }

    const fn is_inroll(&self) -> bool {
        let [f1, f2, f3] = self.0;
        let [h1, h2, h3] = [f1.hand(), f2.hand(), f3.hand()];

        h1.eq(h2) && !h2.eq(h3) && f1.is_inward(f2) || h2.eq(h3) && !h1.eq(h2) && f2.is_inward(f3)
    }

    const fn is_outroll(&self) -> bool {
        let [f1, f2, f3] = self.0;

        f1.hand().eq(f2.hand()) && !f2.hand().eq(f3.hand()) && !f1.is_inward(f2)
            || f2.hand().eq(f3.hand()) && !f1.hand().eq(f2.hand()) && !f2.is_inward(f3)
    }

    const fn is_alternate(&self) -> bool {
        let [f1, f2, f3] = self.0;

        !f1.hand().eq(f2.hand()) && !f2.hand().eq(f3.hand())
    }

    const fn is_redirect(&self) -> bool {
        let [f1, f2, f3] = self.0;

        (f1.is_inward(f2) && !f2.is_inward(f3)) || (!f1.is_inward(f2) && f2.is_inward(f3))
    }

    const fn is_onehandin(&self) -> bool {
        let [f1, f2, f3] = self.0;

        f1.is_inward(f2) && f2.is_inward(f3)
    }

    const fn is_onehandout(&self) -> bool {
        let [f1, f2, f3] = self.0;

        !(f1.is_inward(f2) || f2.is_inward(f3))
    }

    const fn is_thumb(&self) -> bool {
        let [f1, f2, f3] = self.0;

        f1.is_thumb() || f2.is_thumb() || f3.is_thumb()
    }
}

pub const fn trigrams() -> [TrigramType; 1000] {
    use TrigramType::*;

    let mut res = [Invalid; 1000];

    let mut i = 0;
    while i < 10 {
        let mut j = 0;
        while j < 10 {
            let mut k = 0;
            while k < 10 {
                let fs = Trigram::new([
                    DofFinger::FINGERS[i],
                    DofFinger::FINGERS[j],
                    DofFinger::FINGERS[k],
                ]);

                res[i * 100 + j * 10 + k] = if fs.is_thumb() {
                    Thumb
                } else if fs.is_sft() {
                    Sft
                } else if fs.is_sfb() {
                    Sfb
                } else if fs.is_inroll() {
                    Inroll
                } else if fs.is_outroll() {
                    Outroll
                } else if fs.is_alternate() {
                    Alternate
                } else if fs.is_redirect() {
                    Redirect
                } else if fs.is_onehandin() {
                    OnehandIn
                } else if fs.is_onehandout() {
                    OnehandOut
                } else {
                    Invalid
                };

                k += 1;
            }

            j += 1;
        }

        i += 1;
    }

    res
}

pub const TRIGRAMS: [TrigramType; 1000] = trigrams();

#[test]
fn print() {
    for f1 in DofFinger::FINGERS {
        for f2 in DofFinger::FINGERS {
            for f3 in DofFinger::FINGERS {
                let t = TRIGRAMS[f1 as usize * 100 + f2 as usize * 10 + f3 as usize];
                println!("{f1} {f2} {f3}: {t:?}");
            }
        }
    }
}
