//! General utilities.

use dioxus::core::{AttributeValue, IntoAttributeValue};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::fmt::{Debug, Display};
use std::ops::{Add, Deref, Div, Mul, Sub};
use std::str::FromStr;
use std::time::Duration;

/// An element ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Id([u8; 8]);

impl From<[u8; 8]> for Id {
    fn from(value: [u8; 8]) -> Self {
        Self(value)
    }
}

impl Deref for Id {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        str::from_utf8(&self.0).unwrap()
    }
}

impl Borrow<str> for Id {
    fn borrow(&self) -> &str {
        str::from_utf8(&self.0).unwrap()
    }
}

impl AsRef<str> for Id {
    fn as_ref(&self) -> &str {
        str::from_utf8(&self.0).unwrap()
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self)
    }
}

impl IntoAttributeValue for Id {
    fn into_value(self) -> AttributeValue {
        AttributeValue::Text(self.to_string())
    }
}

/// Generates a random ID for an element.
pub fn new_id() -> Id {
    let value = rand::random::<u32>();
    let hex_value = format!("{value:08x}");
    Id(hex_value.bytes().collect::<Vec<_>>().try_into().unwrap())
}

/// Generates a random ID for an element, which will remain consistent across
/// renders.
pub fn use_id() -> Id {
    #[cfg(feature = "fullstack")]
    {
        use_server_cached(new_id)
    }
    #[cfg(not(feature = "fullstack"))]
    {
        use_hook(new_id)
    }
}

/// Generates random IDs for `n` distinct elements, which will remain consistent
/// across renders.
#[allow(dead_code)]
pub fn use_ids(n: usize) -> Vec<Id> {
    use_hook(|| (0..n).map(|_| new_id()).collect())
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

/// Sleep wrapper function, generalized to work in both wasm and non-wasm
/// environments.
pub async fn sleep(duration: Duration) {
    #[cfg(not(target_arch = "wasm32"))]
    use tokio::time::sleep as sleep_async;

    #[cfg(target_arch = "wasm32")]
    use wasmtimer::tokio::sleep as sleep_async;

    sleep_async(duration).await;
}
