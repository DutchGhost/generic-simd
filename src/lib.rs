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

unsafe impl Sealed for f32 {}
unsafe impl Sealed for f64 {}

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

impl Num for f32 {}
impl Num for f64 {}

#[repr(transparent)]
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

use core::ops::*;

#[macro_use]
mod macros;

copy_clone!(2, 4, 8, 16, 32);

mod bit_16_impls;
mod bit_8_impls;
mod float_impls;

assignops!([2, 4, 8, 16, 32], AddAssign, add_assign, Add, add);
assignops!(
    [2, 4, 8, 16, 32],
    BitAndAssign,
    bitand_assign,
    BitAnd,
    bitand
);
assignops!([2, 4, 8, 16, 32], BitOrAssign, bitor_assign, BitOr, bitor);
assignops!(
    [2, 4, 8, 16, 32],
    BitXorAssign,
    bitxor_assign,
    BitXor,
    bitxor
);
assignops!([2, 4, 8, 16, 32], DivAssign, div_assign, Div, div);
assignops!([2, 4, 8, 16, 32], MulAssign, mul_assign, Mul, mul);
assignops!([2, 4, 8, 16, 32], RemAssign, rem_assign, Rem, rem);
assignops!([2, 4, 8, 16, 32], ShlAssign, shl_assign, Shl, shl);
assignops!([2, 4, 8, 16, 32], SubAssign, sub_assign, Sub, sub);

#[cfg(test)]
mod tests {
    use super::*;

    fn is_send<T: Send>() {}

    #[test]
    fn test_is_send() {
        is_send::<Simd<u8, 16>>();
    }
}
