#![no_std]
#![feature(const_generics)]

use core::arch::x86_64::*;

pub unsafe trait Sealed {}
unsafe impl Sealed for u8 {}
unsafe impl Sealed for u16 {}
unsafe impl Sealed for u32 {}
unsafe impl Sealed for u64 {}
unsafe impl Sealed for u128 {}

unsafe impl Sealed for i8 {}
unsafe impl Sealed for i16 {}
unsafe impl Sealed for i32 {}
unsafe impl Sealed for i64 {}
unsafe impl Sealed for i128 {}

pub trait Num: Sealed {}
impl Num for u8 {}
impl Num for u16 {}
impl Num for u32 {}
impl Num for u64 {}
impl Num for u128 {}

impl Num for i8 {}
impl Num for i16 {}
impl Num for i32 {}
impl Num for i64 {}
impl Num for i128 {}

pub struct Simd<T: Num, const WIDTH: usize>
where
    Self: SimdExt,
{
    inner: <Self as SimdExt>::Vector,
}

impl<T: Num, const WIDTH: usize> Simd<T, { WIDTH }>
where
    Self: SimdExt,
{
    /// Returns the number of bits the vector contains.
    pub fn bitwidth() -> usize {
        use core::mem;

        mem::size_of::<<Self as SimdExt>::Vector>() * 8
    }
}
// - u8/i8: 16, 32, 64
// - u16/i16: 8, 16, 32
// - u32/i32/f32: 4, 8, 16
// - u64/i64/f64: 2, 4, 8
// - u128/i128: 1, 2, 4
pub trait SimdExt {
    type Vector: Copy;

    fn new() -> Self;
}

impl<T: Num> Copy for Simd<T, 8> where Self: SimdExt {}
impl<T: Num> Copy for Simd<T, 16> where Self: SimdExt {}
impl<T: Num> Copy for Simd<T, 32> where Self: SimdExt {}

impl<T: Num> Clone for Simd<T, 8>
where
    Self: SimdExt,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Num> Clone for Simd<T, 16>
where
    Self: SimdExt,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Num> Clone for Simd<T, 32>
where
    Self: SimdExt,
{
    fn clone(&self) -> Self {
        *self
    }
}

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


mod bit_16_impls;
mod bit_8_impls;
