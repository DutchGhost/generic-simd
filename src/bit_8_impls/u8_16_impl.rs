use crate::*;
use core::arch::x86_64::*;

impl SimdExt for Simd<u8, 16> {
    type Vector = __m128i;

    fn new() -> Self {
        Self {
            inner: unsafe { _mm_setzero_si128() },
        }
    }
}
