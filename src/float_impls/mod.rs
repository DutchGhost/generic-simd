use crate::*;
use core::{arch::x86_64::*, ops::*};

mod f32_4_impl;
mod f32_8_impl;
mod f64_2_impl;
mod f64_4_impl;

binops!(f32, 4, Add, add, _mm_add_ps);
binops!(f32, 8, Add, add, _mm256_add_ps);
binops!(f64, 2, Add, add, _mm_add_pd);
binops!(f64, 4, Add, add, _mm256_add_pd);

binops!(f32, 4, BitAnd, bitand, _mm_and_ps);
binops!(f32, 8, BitAnd, bitand, _mm256_and_ps);
binops!(f64, 2, BitAnd, bitand, _mm_and_pd);
binops!(f64, 4, BitAnd, bitand, _mm256_and_pd);

binops!(f32, 4, BitOr, bitor, _mm_or_ps);
binops!(f32, 8, BitOr, bitor, _mm256_or_ps);
binops!(f64, 2, BitOr, bitor, _mm_or_pd);
binops!(f64, 4, BitOr, bitor, _mm256_or_pd);

binops!(f32, 4, BitXor, bitxor, _mm_xor_ps);
binops!(f32, 8, BitXor, bitxor, _mm256_xor_ps);
binops!(f64, 2, BitXor, bitxor, _mm_xor_pd);
binops!(f64, 4, BitXor, bitxor, _mm256_xor_pd);

binops!(f32, 4, Mul, mul, _mm_mul_ps);
binops!(f32, 8, Mul, mul, _mm256_mul_ps);
binops!(f64, 2, Mul, mul, _mm_mul_pd);
binops!(f64, 4, Mul, mul, _mm256_mul_pd);

/*
binops!(u16, 8, Shr, shr, _mm_sll_epi16);
binops!(i16, 8, Shr, shr, _mm_sll_epi16);
binops!(u16, 16, Shr, shr, _mm256_sll_epi16);
binops!(i16, 16, Shr, shr, _mm256_sll_epi16);
*/

binops!(f32, 4, Sub, sub, _mm_sub_ps);
binops!(f32, 8, Sub, sub, _mm256_sub_ps);
binops!(f64, 2, Sub, sub, _mm_sub_pd);
binops!(f64, 4, Sub, sub, _mm256_sub_pd);