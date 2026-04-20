use dioxus::prelude::*;
use std::fmt::{Debug, Display};
use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;

/// Generates a random ID for an element.
pub fn new_id() -> String {
    let value = rand::random::<u32>();
    let hex_value = format!("{value:x}");
    hex_value
}

pub fn use_id() -> String {
    use_hook(new_id)
}

/// A trait for numeric values.
pub trait Number:
    PartialEq
    + PartialOrd
    + FromStr
    + ToString
    + Default
    + Clone
    + Copy
    + Display
    + Debug
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
{
    /// The minimum possible value for this number type.
    const NUMBER_MIN: Self;

    /// The maximum possible value for this number type.
    const NUMBER_MAX: Self;

    /// The step amount for this number type.
    const NUMBER_STEP: Self;

    /// Can this number type represent decimal values?
    const DECIMAL: bool;

    /// Represent this numeric value as an `f64`.
    fn as_f64(self) -> f64;
}

/// Implements the `Number` trait for integer primitives that can be losslessly
/// converted into an `f64`.
macro_rules! impl_number_int_lossless {
    ( $($ty:ty)* ) => {
        $(
            impl Number for $ty {
                const NUMBER_MIN: Self = Self::MIN;
                const NUMBER_MAX: Self = Self::MAX;
                const NUMBER_STEP: Self = 1 as Self;
                const DECIMAL: bool = false;

                fn as_f64(self) -> f64 {
                    f64::from(self)
                }
            }
        )*
    };
}

/// Implements the `Number` trait for integer primitives that cannot be
/// loslessly converted into an `f64`.
macro_rules! impl_number_int_lossy {
    ( $($ty:ty)* ) => {
        $(
            impl Number for $ty {
                const NUMBER_MIN: Self = Self::MIN;
                const NUMBER_MAX: Self = Self::MAX;
                const NUMBER_STEP: Self = 1 as Self;
                const DECIMAL: bool = false;

                #[allow(clippy::cast_precision_loss)]
                fn as_f64(self) -> f64 {
                    self as f64
                }
            }
        )*
    };
}

/// Implements the `Number` trait for floating point primitives.
macro_rules! impl_number_float {
    ( $($ty:ty)* ) => {
        $(
            impl Number for $ty {
                const NUMBER_MIN: Self = Self::MIN;
                const NUMBER_MAX: Self = Self::MAX;
                const NUMBER_STEP: Self = 1.0 as Self;
                const DECIMAL: bool = true;

                fn as_f64(self) -> f64 {
                    f64::from(self)
                }
            }
        )*
    };
}

impl_number_int_lossless!(i8 i16 i32 u8 u16 u32);

impl_number_int_lossy!(i64 i128 isize u64 u128 usize);

impl_number_float!(f32 f64);
