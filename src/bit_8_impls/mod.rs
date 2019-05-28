use crate::*;
use core::{arch::x86_64::*, ops::*};

#[cfg(target_feature = "sse2")]
mod i8_16_impl;
#[cfg(target_feature = "avx2")]
mod i8_32_impl;
#[cfg(target_feature = "sse2")]
mod u8_16_impl;
#[cfg(target_feature = "avx2")]
mod u8_32_impl;

// ------------------------------------
// ---------- impl add ----------------
//-------------------------------------
#[cfg(target_feature = "sse2")]
binops!(u8, 16, Add, add, _mm_add_epi8);
#[cfg(target_feature = "sse2")]
binops!(i8, 16, Add, add, _mm_add_epi8);
#[cfg(target_feature = "avx2")]
binops!(u8, 32, Add, add, _mm256_add_epi8);
#[cfg(target_feature = "avx2")]
binops!(i8, 32, Add, add, _mm256_add_epi8);

// ------------------------------------
// ---------- impl bitand -------------
//-------------------------------------
#[cfg(target_feature = "sse2")]
binops!(u8, 16, BitAnd, bitand, _mm_and_si128);
#[cfg(target_feature = "sse2")]
binops!(i8, 16, BitAnd, bitand, _mm_and_si128);
#[cfg(target_feature = "avx2")]
binops!(u8, 32, BitAnd, bitand, _mm256_and_si256);
#[cfg(target_feature = "avx2")]
binops!(i8, 32, BitAnd, bitand, _mm256_and_si256);

// ------------------------------------
// ---------- impl bitor --------------
//-------------------------------------
#[cfg(target_feature = "sse2")]
binops!(u8, 16, BitOr, bitor, _mm_or_si128);
#[cfg(target_feature = "sse2")]
binops!(i8, 16, BitOr, bitor, _mm_or_si128);
#[cfg(target_feature = "avx2")]
binops!(u8, 32, BitOr, bitor, _mm256_or_si256);
#[cfg(target_feature = "avx2")]
binops!(i8, 32, BitOr, bitor, _mm256_or_si256);

// ------------------------------------
// ---------- impl bitxor -------------
//-------------------------------------
#[cfg(target_feature = "sse2")]
binops!(u8, 16, BitXor, bitxor, _mm_xor_si128);
#[cfg(target_feature = "sse2")]
binops!(i8, 16, BitXor, bitxor, _mm_xor_si128);
#[cfg(target_feature = "avx2")]
binops!(u8, 32, BitXor, bitxor, _mm256_xor_si256);
#[cfg(target_feature = "avx2")]
binops!(i8, 32, BitXor, bitxor, _mm256_xor_si256);

// ------------------------------------
// ---------- impl sub ----------------
//-------------------------------------
#[cfg(target_feature = "sse2")]
binops!(u8, 16, Sub, sub, _mm_sub_epi8);
#[cfg(target_feature = "sse2")]
binops!(i8, 16, Sub, sub, _mm_sub_epi8);
#[cfg(target_feature = "avx2")]
binops!(u8, 32, Sub, sub, _mm256_sub_epi8);
#[cfg(target_feature = "avx2")]
binops!(i8, 32, Sub, sub, _mm256_sub_epi8);


#[cfg(target_feature = "sse4.1")]
impl_all!(u8, 16, this, other, {
    unsafe { _mm_testc_si128(this.inner, other.inner) != 0 }
});

#[cfg(all(target_feature = "sse2", not(target_feature = "sse4.1")))]
impl_all!(u8, 16, this, other, {
    unsafe {
        let result = _mm_cmpeq_epi8(this.inner, other.inner);
        _mm_movemask_epi8(result) == 0
    }
});