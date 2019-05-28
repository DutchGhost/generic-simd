use crate::*;
use core::arch::x86_64::*;

impl SimdExt for Simd<i8, 32> {
    type Vector = __m256i;

    fn new() -> Self {
        Self {
            inner: unsafe { _mm256_setzero_si256() },
        }
    }
}
