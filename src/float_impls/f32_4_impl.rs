use crate::*;
use core::{arch::x86_64::*, ops::*};

type SimdType = Simd<f32, 4>;

impl SimdExt for SimdType {
    type Vector = __m128;

    #[inline(always)]
    fn new() -> Self {
        Self {
            inner: unsafe { _mm_setzero_ps() },
        }
    }
}