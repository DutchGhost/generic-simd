use crate::*;
use core::{arch::x86_64::*, ops::*};

mod i8_16_impl;
mod i8_32_impl;
mod u8_16_impl;
mod u8_32_impl;

binops!(u8, 16, Add, add, _mm_add_epi8);
binops!(i8, 16, Add, add, _mm_add_epi8);
binops!(u8, 32, Add, add, _mm256_add_epi8);
binops!(i8, 32, Add, add, _mm256_add_epi8);

binops!(u8, 16, Sub, sub, _mm_sub_epi8);
binops!(i8, 16, Sub, sub, _mm_sub_epi8);
binops!(u8, 32, Sub, sub, _mm256_sub_epi8);
binops!(i8, 32, Sub, sub, _mm256_sub_epi8);