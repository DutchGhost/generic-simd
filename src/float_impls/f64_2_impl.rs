use crate::*;
use core::{arch::x86_64::*, ops::*};

type SimdType = Simd<f64, 2>;

impl SimdExt for SimdType {
    type Vector = __m128d;

    #[inline(always)]
    fn new() -> Self {
        Self {
            inner: unsafe { _mm_setzero_pd() },
        }
    }
}