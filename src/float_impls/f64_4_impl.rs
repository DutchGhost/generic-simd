use crate::*;
use core::{arch::x86_64::*, ops::*};

type SimdType = Simd<f64, 4>;

impl SimdExt for SimdType {
    type Vector = __m256d;

    #[inline(always)]
    fn new() -> Self {
        Self {
            inner: unsafe { _mm256_setzero_pd() },
        }
    }
}
