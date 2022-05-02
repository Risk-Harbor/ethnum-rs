//! Module contains conversions for [`I256`] to and from primimitive types.

use super::I256;
use crate::{error::tfie, uint::U256};
use core::num::TryFromIntError;

macro_rules! impl_from {
    ($($t:ty),* $(,)?) => {$(
        impl From<$t> for I256 {
            #[inline]
            fn from(value: $t) -> Self {
                value.as_i256()
            }
        }
    )*};
}

impl_from! {
    bool,
    i8, i16, i32, i64, i128,
    u8, u16, u32, u64, u128,
}

impl TryFrom<U256> for I256 {
    type Error = TryFromIntError;

    fn try_from(value: U256) -> Result<Self, Self::Error> {
        if value > I256::MAX.as_u256() {
            return Err(tfie());
        }
        Ok(value.as_i256())
    }
}

/// This trait defines `as` conversions (casting) from primitive types to
/// [`I256`].
///
/// [`I256`]: struct.I256.html
///
/// # Examples
///
/// Casting a floating point value to an integer is a saturating operation,
/// with `NaN` converting to `0`. So:
///
/// ```
/// # use ethnum::{I256, AsI256};
/// assert_eq!((-1i32).as_i256(), -I256::ONE);
/// assert_eq!(u32::MAX.as_i256(), 0xffffffff);
///
/// assert_eq!(-13.37f64.as_i256(), -13);
/// assert_eq!(42.0f64.as_i256(), 42);
/// assert_eq!(
///     f32::MAX.as_i256(),
///     0xffffff00000000000000000000000000u128.as_i256(),
/// );
/// assert_eq!(
///     f32::MIN.as_i256(),
///     -0xffffff00000000000000000000000000u128.as_i256(),
/// );
///
/// assert_eq!(f64::NEG_INFINITY.as_i256(), I256::MIN);
/// assert_eq!((-2.0f64.powi(256)).as_i256(), I256::MIN);
/// assert_eq!(f64::INFINITY.as_i256(), I256::MAX);
/// assert_eq!(2.0f64.powi(256).as_i256(), I256::MAX);
/// assert_eq!(f64::NAN.as_i256(), 0);
/// ```
pub trait AsI256 {
    /// Perform an `as` conversion to a [`I256`].
    ///
    /// [`I256`]: struct.I256.html
    #[allow(clippy::wrong_self_convention)]
    fn as_i256(self) -> I256;
}

impl AsI256 for I256 {
    #[inline]
    fn as_i256(self) -> I256 {
        self
    }
}

impl AsI256 for U256 {
    #[inline]
    fn as_i256(self) -> I256 {
        U256::as_i256(self)
    }
}

macro_rules! impl_as_i256 {
    ($($t:ty),* $(,)?) => {$(
        impl AsI256 for $t {
            #[inline]
            fn as_i256(self) -> I256 {
                #[allow(unused_comparisons)]
                let hi = if self >= 0 { 0 } else { !0 };
                I256::from_words(hi, self as _)
            }
        }
    )*};
}

impl_as_i256! {
    i8, i16, i32, i64, i128,
    u8, u16, u32, u64, u128,
    isize, usize,
}

impl AsI256 for bool {
    #[inline]
    fn as_i256(self) -> I256 {
        I256::new(self as _)
    }
}

macro_rules! impl_try_into {
    ($($t:ty),* $(,)?) => {$(
        impl TryFrom<I256> for $t {
            type Error = TryFromIntError;

            #[inline]
            fn try_from(x: I256) -> Result<Self, Self::Error> {
                if x <= <$t>::MAX.as_i256() {
                    Ok(*x.low() as _)
                } else {
                    Err(tfie())
                }
            }
        }
    )*};
}

impl_try_into! {
    i8, i16, i32, i64, i128,
    u8, u16, u32, u64, u128,
    isize, usize,
}
