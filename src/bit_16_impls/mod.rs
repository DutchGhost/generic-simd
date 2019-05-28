use crate::*;
use core::{arch::x86_64::*, ops::*};

mod i16_16_impl;
mod i16_8_impl;
mod u16_16_impl;
mod u16_8_impl;

binops!(u16, 8, Add, add, _mm_add_epi16);
binops!(i16, 8, Add, add, _mm_add_epi16);
binops!(u16, 16, Add, add, _mm256_add_epi16);
binops!(i16, 16, Add, add, _mm256_add_epi16);

binops!(u16, 8, BitAnd, bitand, _mm_and_si128);
binops!(i16, 8, BitAnd, bitand, _mm_and_si128);
binops!(u16, 16, BitAnd, bitand, _mm256_and_si256);
binops!(i16, 16, BitAnd, bitand, _mm256_and_si256);

binops!(u16, 8, BitOr, bitor, _mm_or_si128);
binops!(i16, 8, BitOr, bitor, _mm_or_si128);
binops!(u16, 16, BitOr, bitor, _mm256_or_si256);
binops!(i16, 16, BitOr, bitor, _mm256_or_si256);

binops!(u16, 8, BitXor, bitxor, _mm_xor_si128);
binops!(i16, 8, BitXor, bitxor, _mm_xor_si128);
binops!(u16, 16, BitXor, bitxor, _mm256_xor_si256);
binops!(i16, 16, BitXor, bitxor, _mm256_xor_si256);

binops!(u16, 8, Mul, mul, _mm_mullo_epi16);
binops!(i16, 8, Mul, mul, _mm_mullo_epi16);
binops!(u16, 16, Mul, mul, _mm256_mullo_epi16);
binops!(i16, 16, Mul, mul, _mm256_mullo_epi16);

/*
binops!(u16, 8, Shr, shr, _mm_sll_epi16);
binops!(i16, 8, Shr, shr, _mm_sll_epi16);
binops!(u16, 16, Shr, shr, _mm256_sll_epi16);
binops!(i16, 16, Shr, shr, _mm256_sll_epi16);
*/

binops!(u16, 8, Sub, sub, _mm_sub_epi16);
binops!(i16, 8, Sub, sub, _mm_sub_epi16);
binops!(u16, 16, Sub, sub, _mm256_sub_epi16);
binops!(i16, 16, Sub, sub, _mm256_sub_epi16);

