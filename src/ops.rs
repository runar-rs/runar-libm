//! This module contains additional operations that are not part of core::ops, but are useful for the runar-project.

pub trait Exp {
    /// The output type of the exponential operation.
    type Output;

    /// Computes the exponential of `self`.
    fn exp(self) -> Self::Output;
}

/// Trait for computing the natural logarithm of a value.
pub trait Ln {
    /// The output type of the natural logarithm operation.
    type Output;

    /// Computes the natural logarithm of `self`.
    fn ln(self) -> Self::Output;
}


pub trait Log {
    /// The output type of the logarithm operation.
    type Output;
    /// The base type of the logarithm operation.
    type Base;

    /// Computes the logarithm of `self` with respect to the given `base`.
    fn log(self, base: Self::Base) -> Self::Output;
}

pub trait Log2 {
    /// The output type of the logarithm base 2 operation.
    type Output;

    /// Computes the logarithm base 2 of `self`.
    fn log2(self) -> Self::Output;
}

/// A blanket implementation of the `Log2` trait for any type that implements the `Log` trait with a base of 2.
impl<T: From<i32>> Log2 for T
where
    T: Log<Base = T>,
{
    type Output = T::Output;

    fn log2(self) -> Self::Output {
        self.log(T::from(2))
    }
}

pub trait Log10 {
    /// The output type of the logarithm base 10 operation.
    type Output;

    /// Computes the logarithm base 10 of `self`.
    fn log10(self) -> Self::Output;
}

impl<T: From<i32>> Log10 for T
where
    T: Log<Base = T>,
{
    type Output = T::Output;

    fn log10(self) -> Self::Output {
        self.log(T::from(10))
    }
}

pub trait Sqrt {
    /// The output type of the square root operation.
    type Output;

    /// Computes the square root of `self`.
    fn sqrt(self) -> Self::Output;
}