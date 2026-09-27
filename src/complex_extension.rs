use core::ops::{Add, Div, Mul, Sub};

use num_complex::Complex;
use num_traits::Num;

pub trait ComplexExtension {}

pub trait Map<I, O> {
    fn map(self, f: impl FnMut(I) -> O) -> Complex<O>;
}
impl<T, U> Map<T, U> for Complex<T> {
    fn map(self, mut f: impl FnMut(T) -> U) -> Complex<U> {
        Complex {
            re: f(self.re),
            im: f(self.im),
        }
    }
}

pub trait ZipWith<L, R, O> {
    fn zip_with(self, other: Complex<R>, f: impl FnMut(L, R) -> O) -> Complex<O>;
}
impl<T, U, V> ZipWith<T, U, V> for Complex<T> {
    fn zip_with(self, other: Complex<U>, mut f: impl FnMut(T, U) -> V) -> Complex<V> {
        Complex {
            re: f(self.re, other.re),
            im: f(self.im, other.im),
        }
    }
}

pub trait ElementwiseMultiply<L, R, O> {
    fn elementwise_multiply(self, other: Complex<R>) -> Complex<O>;
}
impl<L, R, O> ElementwiseMultiply<L, R, O> for Complex<L>
where
    L: Mul<R, Output = O>,
{
    fn elementwise_multiply(self, other: Complex<R>) -> Complex<O> {
        self.zip_with(other, L::mul)
    }
}

pub trait ElementwiseDivide<L, R, O> {
    fn elementwise_divide(self, other: Complex<R>) -> Complex<O>;
}
impl<L, R, O> ElementwiseDivide<L, R, O> for Complex<L>
where
    L: Div<R, Output = O>,
{
    fn elementwise_divide(self, other: Complex<R>) -> Complex<O> {
        self.zip_with(other, L::div)
    }
}

pub trait ElementwiseAdd<L, R, O> {
    fn elementwise_add(self, other: Complex<R>) -> Complex<O>;
}
impl<L, R, O> ElementwiseAdd<L, R, O> for Complex<L>
where
    L: Add<R, Output = O>,
{
    fn elementwise_add(self, other: Complex<R>) -> Complex<O> {
        self.zip_with(other, L::add)
    }
}

pub trait ElementwiseSubtract<L, R, O> {
    fn elementwise_subtract(self, other: Complex<R>) -> Complex<O>;
}
impl<L, R, O> ElementwiseSubtract<L, R, O> for Complex<L>
where
    L: Sub<R, Output = O>,
{
    fn elementwise_subtract(self, other: Complex<R>) -> Complex<O> {
        self.zip_with(other, L::sub)
    }
}

pub trait ComplexZero {
    fn zero() -> Self;
}
impl<N: Num> ComplexZero for Complex<N> {
    fn zero() -> Self {
        Complex {
            re: N::zero(),
            im: N::zero(),
        }
    }
}
