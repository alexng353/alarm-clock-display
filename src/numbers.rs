use crate::segment::SegmentMask as M;

pub const ZERO: M = M::T
    .union(M::TR)
    .union(M::BR)
    .union(M::B)
    .union(M::BL)
    .union(M::TL);
pub const ONE: M = M::TR.union(M::BR);
pub const TWO: M = M::T.union(M::TR).union(M::M).union(M::BL).union(M::B);
pub const THREE: M = M::T.union(M::TR).union(M::BR).union(M::B).union(M::M);
pub const FOUR: M = M::TL.union(M::M).union(M::TR).union(M::BR);
pub const FIVE: M = M::T.union(M::TL).union(M::M).union(M::BR).union(M::B);
pub const SIX: M = M::T
    .union(M::TL)
    .union(M::BL)
    .union(M::B)
    .union(M::BR)
    .union(M::M);
pub const SEVEN: M = M::T.union(M::TR).union(M::BR);
pub const EIGHT: M = M::T
    .union(M::TR)
    .union(M::BR)
    .union(M::B)
    .union(M::BL)
    .union(M::TL)
    .union(M::M);
pub const NINE: M = M::T
    .union(M::TR)
    .union(M::BR)
    .union(M::B)
    .union(M::TL)
    .union(M::M);

pub const NUMBERS: [M; 10] = [ZERO, ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE];
