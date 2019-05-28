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

//assignops!(u8, 16, AddAssign, add_assign, add);

binops!(u8, 16, Sub, sub, _mm_sub_epi8);
binops!(i8, 16, Sub, sub, _mm_sub_epi8);
binops!(u8, 32, Sub, sub, _mm256_sub_epi8);
binops!(i8, 32, Sub, sub, _mm256_sub_epi8);

binops!(u8, 16, BitAnd, bitand, _mm_and_si128);
binops!(i8, 16, BitAnd, bitand, _mm_and_si128);
binops!(u8, 32, BitAnd, bitand, _mm256_and_si256);
binops!(i8, 32, BitAnd, bitand, _mm256_and_si256);