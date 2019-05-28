use crate::*;
use core::{arch::x86_64::*, ops::*};

mod i8_16_impl;
mod i8_32_impl;
mod u8_16_impl;
mod u8_32_impl;

macro_rules! binops {
    ($tgt:ty, $size:expr, $trait_name:ident, $fnname:ident, $operation:tt) => {
        impl $trait_name for Simd<$tgt, $size> {
            type Output = Self;

            #[inline(always)]
            fn $fnname(self, rhs: Self) -> Self::Output {
                Self {
                    inner: unsafe { $operation(self.inner, rhs.inner) }
                }
            }
        }

        impl <'a> $trait_name<Simd<$tgt, $size>> for &'a Simd<$tgt, $size> {
            type Output = <Simd<$tgt, $size> as $trait_name>::Output;

            #[inline(always)]
            fn $fnname(self, rhs: Simd<$tgt, $size>) -> Self::Output {
                (*self).$fnname(rhs)
            }
        }

        impl <'a> $trait_name<Simd<$tgt, $size>> for &'a mut Simd<$tgt, $size> {
            type Output = <Simd<$tgt, $size> as $trait_name>::Output;

            #[inline(always)]
            fn $fnname(self, rhs: Simd<$tgt, $size>) -> Self::Output {
                (*self).$fnname(rhs)
            }
        }

        impl <'a> $trait_name<&'a Simd<$tgt, $size>> for Simd<$tgt, $size> {
            type Output = <Simd<$tgt, $size> as $trait_name>::Output;

            #[inline(always)]
            fn $fnname(self, rhs: &'a Simd<$tgt, $size>) -> Self::Output {
                self.$fnname(*rhs)
            }
        }

        impl <'a> $trait_name<&'a mut Simd<$tgt, $size>> for Simd<$tgt, $size> {
            type Output = <Simd<$tgt, $size> as $trait_name>::Output;

            #[inline(always)]
            fn $fnname(self, rhs: &'a mut Simd<$tgt, $size>) -> Self::Output {
                self.$fnname(*rhs)
            }
        }

        impl <'a, 'b> $trait_name<&'a Simd<$tgt, $size>> for &'b Simd<$tgt, $size> {
            type Output = <Simd<$tgt, $size> as $trait_name>::Output;
            
            #[inline(always)]
            fn $fnname(self, rhs: &'a Simd<$tgt, $size>) -> Self::Output {
                (*self).$fnname(*rhs)
            }
        }

        impl <'a, 'b> $trait_name<&'a mut Simd<$tgt, $size>> for &'b Simd<$tgt, $size> {
            type Output = <Simd<$tgt, $size> as $trait_name>::Output;
            
            #[inline(always)]
            fn $fnname(self, rhs: &'a mut Simd<$tgt, $size>) -> Self::Output {
                (*self).$fnname(*rhs)
            }
        }

        impl <'a, 'b> $trait_name<&'a Simd<$tgt, $size>> for &'b mut Simd<$tgt, $size> {
            type Output = <Simd<$tgt, $size> as $trait_name>::Output;
            
            #[inline(always)]
            fn $fnname(self, rhs: &'a Simd<$tgt, $size>) -> Self::Output {
                (*self).$fnname(*rhs)
            }
        }

        impl <'a, 'b> $trait_name<&'a mut Simd<$tgt, $size>> for &'b mut Simd<$tgt, $size> {
            type Output = <Simd<$tgt, $size> as $trait_name>::Output;
            
            #[inline(always)]
            fn $fnname(self, rhs: &'a mut Simd<$tgt, $size>) -> Self::Output {
                (*self).$fnname(*rhs)
            }
        }
    }
}

binops!(u8, 16, Add, add, _mm_add_epi8);
binops!(i8, 16, Add, add, _mm_add_epi8);
binops!(u8, 32, Add, add, _mm256_add_epi8);
binops!(i8, 32, Add, add, _mm256_add_epi8);

binops!(u8, 16, Sub, sub, _mm_sub_epi8);
binops!(i8, 16, Sub, sub, _mm_sub_epi8);
binops!(u8, 32, Sub, sub, _mm256_sub_epi8);
binops!(i8, 32, Sub, sub, _mm256_sub_epi8);