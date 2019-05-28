use crate::*;
use core::{arch::x86_64::*, ops::*};

type SimdType = Simd<i8, 16>;

impl SimdExt for SimdType {
    type Vector = __m128i;

    #[inline(always)]
    fn new() -> Self {
        Self {
            inner: unsafe { _mm_setzero_si128() },
        }
    }
}