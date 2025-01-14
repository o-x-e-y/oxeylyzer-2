mod sfb_cache;
mod stretch_cache;

pub use sfb_cache::SfbIndices;
pub use stretch_cache::StretchCache;

use libdof::prelude::{Finger, PhysicalKey};

use crate::layout::PosPair;

const KEY_EDGE_OFFSET: f64 = 0.5;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BigramPair {
    pub pair: PosPair,
    pub dist: i64,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BigramCache {
    pub total: i64,
    pub per_finger: Box<[i64; 10]>,
}

fn x_finger_overlap(f1: Finger, f2: Finger) -> f64 {
    use Finger::*;

    match (f1, f2) {
        (LP, LR) => 0.8,
        (LR, LP) => 0.8,
        (LR, LM) => 0.4,
        (LM, LR) => 0.4,
        (LM, LI) => 0.1,
        (LI, LM) => 0.1,
        (LI, LT) => -2.5,
        (LT, LI) => -2.5,
        (RT, RI) => -2.5,
        (RI, RT) => -2.5,
        (RI, RM) => 0.1,
        (RM, RI) => 0.1,
        (RM, RR) => 0.4,
        (RR, RM) => 0.4,
        (RR, RP) => 0.8,
        (RP, RR) => 0.8,
        _ => 0.0,
    }
}

fn x_overlap(dx: f64, dy: f64, f1: Finger, f2: Finger) -> f64 {
    let x_offset = x_finger_overlap(f1, f2);

    let dx_offset = x_offset - dx * 1.3;
    let dy_offset = 0.3333 * dy;

    (dx_offset + dy_offset).max(0.0)
}

fn dx_dy(k1: &PhysicalKey, k2: &PhysicalKey, f1: Finger, f2: Finger) -> (f64, f64) {
    let flen = |f: Finger| match f {
        Finger::LP | Finger::RP => -0.15,
        Finger::LR | Finger::RR => 0.35,
        Finger::LM | Finger::RM => 0.25,
        Finger::LI | Finger::RI => -0.30,
        Finger::LT | Finger::RT => -1.80,
    };

    let ox1 = (k1.width() * KEY_EDGE_OFFSET).min(KEY_EDGE_OFFSET);
    let ox2 = (k1.width() * KEY_EDGE_OFFSET).min(KEY_EDGE_OFFSET);

    let oy1 = (k2.height() * KEY_EDGE_OFFSET).min(KEY_EDGE_OFFSET);
    let oy2 = (k2.height() * KEY_EDGE_OFFSET).min(KEY_EDGE_OFFSET);

    let l1 = k1.x() + ox1;
    let r1 = k1.x() - ox1 + k1.width();
    let t1 = k1.y() + oy1 + flen(f1);
    let b1 = k1.y() - oy1 + k1.height() + flen(f1);

    let l2 = k2.x() + ox2;
    let r2 = k2.x() - ox2 + k2.width();
    let t2 = k2.y() + oy2 + flen(f2);
    let b2 = k2.y() - oy2 + k2.height() + flen(f2);

    let dx = (l1.max(l2) - r1.min(r2)).max(0.0);
    let dy = (t1.max(t2) - b1.min(b2)).max(0.0);

    // Checks whether or not a finger is below or to the side of another finger, in which case the
    // distance is considered negative. To the side meaning, where the distance between qwerty `er`
    // pressed with middle and index is considered 1, if each key were pressed with the other
    // finger, the distance is negative (because who the fuck is doing that, that's not good).

    let xo = x_finger_overlap(f1, f2);

    // match (f1.hand(), f2.hand()) {
    //     (Hand::Left, Hand::Left) => match ((f1 as u8) > (f2 as u8), (f1 as u8) < (f2 as u8)) {
    //         (true, false) if r1 < l2 => (-dx, dy),
    //         (false, true) if l1 > r2 => (-dx, dy),
    //         _ => (dx, dy),
    //     },
    //     (Hand::Right, Hand::Right) => match ((f2 as u8) > (f1 as u8), (f2 as u8) < (f1 as u8)) {
    //         (true, false) if r1 > l2 => (-dx, dy),
    //         (false, true) if l1 < r2 => (-dx, dy),
    //         _ => (dx, dy),
    //     },
    //     _ => (dx, dy)
    // }
    match ((f1 as u8) > (f2 as u8), (f1 as u8) < (f2 as u8)) {
        (true, false) if r1 < l2 + xo => (-dx, dy),
        (false, true) if l1 + xo > r2 => (-dx, dy),
        _ => (dx, dy),
    }
}

pub(super) fn dist(k1: &PhysicalKey, k2: &PhysicalKey, f1: Finger, f2: Finger) -> f64 {
    let (dx, dy) = dx_dy(k1, k2, f1, f2);

    dx.hypot(dy)
}
