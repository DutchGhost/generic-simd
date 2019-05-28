use crate::*;
use core::arch::x86_64::*;

impl SimdExt for Simd<i16, 16> {
    type Vector = __m256i;

    #[inline(always)]
    fn new() -> Self {
        Self {
            inner: unsafe { _mm256_setzero_si256() },
        }
    }
}
