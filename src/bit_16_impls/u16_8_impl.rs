use crate::*;
use core::arch::x86_64::*;

impl SimdExt for Simd<u16, 8> {
    type Vector = __m128i;

    #[inline(always)]
    fn new() -> Self {
        Self {
            inner: unsafe { _mm_setzero_si128() },
        }
    }
}
