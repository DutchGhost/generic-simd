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

binops!(u16, 8, Sub, sub, _mm_sub_epi16);
binops!(i16, 8, Sub, sub, _mm_sub_epi16);
binops!(u16, 16, Sub, sub, _mm256_sub_epi16);
binops!(i16, 16, Sub, sub, _mm256_sub_epi16);

binops!(u16, 8, Mul, mul, _mm_mullo_epi16);
binops!(i16, 8, Mul, mul, _mm_mullo_epi16);
binops!(u16, 16, Mul, mul, _mm256_mullo_epi16);
binops!(i16, 16, Mul, mul, _mm256_mullo_epi16);