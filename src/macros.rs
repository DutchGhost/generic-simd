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

macro_rules! assignops {
    ($size:expr, $trait_name:ident, $fnname:ident, $forwarding_trait:ident, $forwardingfn:ident) => {
        impl <T: Num> $trait_name for Simd<T, $size>
        where
            Self: $forwarding_trait<Output = Self> + SimdExt,
        {
            fn $fnname(&mut self, rhs: Simd<T, $size>) {
                *self = (*self).$forwardingfn(rhs)
            }
        }

        impl <'a, T: Num> $trait_name<&'a Simd<T, $size>> for Simd<T, $size>
        where
            Self: $forwarding_trait<Output = Self> + SimdExt,
        {
            fn $fnname(&mut self, rhs: &'a Simd<T, $size>) {
                *self = (*self).$forwardingfn(*rhs);
            }
        }

        impl <'a, T: Num> $trait_name<&'a mut Simd<T, $size>> for Simd<T, $size>
        where
            Self: $forwarding_trait<Output = Self> + SimdExt,
        {
            fn $fnname(&mut self, rhs: &'a mut Simd<T, $size>) {
                *self = (*self).$forwardingfn(*rhs);
            }
        }

        impl <'a, T: Num> $trait_name<Simd<T, $size>> for &'a mut Simd<T, $size>
        where
            Simd<T, $size>: $forwarding_trait<Output = Simd<T, $size>> + SimdExt,
        {
            fn $fnname(&mut self, rhs: Simd<T, $size>) {
                (**self) = (**self).$forwardingfn(rhs);
            }
        }

        impl <'a, 'b, T: Num> $trait_name<&'b Simd<T, $size>> for &'a mut Simd<T, $size>
        where
            Simd<T, $size>: $forwarding_trait<Output = Simd<T, $size>> + SimdExt,
        {
            fn $fnname(&mut self, rhs: &'b Simd<T, $size>) {
                (**self) = (**self).$forwardingfn(*rhs);
            }
        }

        impl <'a, 'b, T: Num> $trait_name<&'b mut Simd<T, $size>> for &'a mut Simd<T, $size>
        where
            Simd<T, $size>: $forwarding_trait<Output = Simd<T, $size>> + SimdExt,
        {
            fn $fnname(&mut self, rhs: &'b mut Simd<T, $size>) {
                (**self) = (**self).$forwardingfn(*rhs);
            }
        }
    }
}