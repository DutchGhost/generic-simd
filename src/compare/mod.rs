use crate::*;
use core::arch::x86_64::*;

pub trait All<Rhs = Self> {
    fn all(&self, rhs: &Rhs) -> bool;
}

macro_rules! impl_all {
    ($tgt:ty, $size:expr, $self:ident, $rhs:ident, $operation:block) => {
        impl $crate::compare::All for Simd<$tgt, $size> {
            #[inline(always)]
            fn all(&self, rhs: &Self) -> bool {
                let $self = self;
                let $rhs = rhs;

                $operation
            }
        }
    }
}