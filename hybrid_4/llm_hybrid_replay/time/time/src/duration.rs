//! The [`Duration`] struct and its associated `impl`s.
use core::cmp::Ordering;
use core::fmt;
use core::iter::Sum;
use core::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};
use core::time::Duration as StdDuration;
use crate::convert::*;
use crate::error;
#[cfg(feature = "std")]
use crate::Instant;
/// By explicitly inserting this enum where padding is expected, the compiler is able to better
/// perform niche value optimization.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) enum Padding {
    #[allow(clippy::missing_docs_in_private_items)]
    Optimize,
}
impl Default for Padding {
    fn default() -> Self {
        Self::Optimize
    }
}
/// A span of time with nanosecond precision.
///
/// Each `Duration` is composed of a whole number of seconds and a fractional part represented in
/// nanoseconds.
///
/// This implementation allows for negative durations, unlike [`core::time::Duration`].
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Duration {
    /// Number of whole seconds.
    seconds: i64,
    /// Number of nanoseconds within the second. The sign always matches the `seconds` field.
    nanoseconds: i32,
    #[allow(clippy::missing_docs_in_private_items)]
    padding: Padding,
}
impl fmt::Debug for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Duration")
            .field("seconds", &self.seconds)
            .field("nanoseconds", &self.nanoseconds)
            .finish()
    }
}
/// This is adapted from the `std` implementation, which uses mostly bit
/// operations to ensure the highest precision:
/// https://github.com/rust-lang/rust/blob/3a37c2f0523c87147b64f1b8099fc9df22e8c53e/library/core/src/time.rs#L1262-L1340
/// Changes from `std` are marked and explained below.
#[rustfmt::skip]
macro_rules! try_from_secs {
    (
        secs = $secs:expr, mantissa_bits = $mant_bits:literal, exponent_bits =
        $exp_bits:literal, offset = $offset:literal, bits_ty = $bits_ty:ty,
        bits_ty_signed = $bits_ty_signed:ty, double_ty = $double_ty:ty, float_ty =
        $float_ty:ty, is_nan = $is_nan:expr, is_overflow = $is_overflow:expr,
    ) => {
        { 'value : { const MIN_EXP : i16 = 1 - (1i16 << $exp_bits) / 2; const MANT_MASK :
        $bits_ty = (1 << $mant_bits) - 1; const EXP_MASK : $bits_ty = (1 << $exp_bits) -
        1; let bits = $secs .to_bits(); let mant = (bits & MANT_MASK) | (MANT_MASK + 1);
        let exp = ((bits >> $mant_bits) & EXP_MASK) as i16 + MIN_EXP; let (secs, nanos) =
        if exp < - 31 { (0u64, 0u32) } else if exp < 0 { let t = <$double_ty
        >::from(mant) << ($offset + exp); let nanos_offset = $mant_bits + $offset; let
        nanos_tmp = u128::from(Nanosecond.per(Second)) * u128::from(t); let nanos =
        (nanos_tmp >> nanos_offset) as u32; let rem_mask = (1 << nanos_offset) - 1; let
        rem_msb_mask = 1 << (nanos_offset - 1); let rem = nanos_tmp & rem_mask; let
        is_tie = rem == rem_msb_mask; let is_even = (nanos & 1) == 0; let rem_msb =
        nanos_tmp & rem_msb_mask == 0; let add_ns = ! (rem_msb || (is_even && is_tie));
        let nanos = nanos + add_ns as u32; if ($mant_bits == 23) || (nanos != Nanosecond
        .per(Second)) { (0, nanos) } else { (1, 0) } } else if exp < $mant_bits { let
        secs = u64::from(mant >> ($mant_bits - exp)); let t = <$double_ty >::from((mant
        << exp) & MANT_MASK); let nanos_offset = $mant_bits; let nanos_tmp = <$double_ty
        >::from(Nanosecond.per(Second)) * t; let nanos = (nanos_tmp >> nanos_offset) as
        u32; let rem_mask = (1 << nanos_offset) - 1; let rem_msb_mask = 1 <<
        (nanos_offset - 1); let rem = nanos_tmp & rem_mask; let is_tie = rem ==
        rem_msb_mask; let is_even = (nanos & 1) == 0; let rem_msb = nanos_tmp &
        rem_msb_mask == 0; let add_ns = ! (rem_msb || (is_even && is_tie)); let nanos =
        nanos + add_ns as u32; if ($mant_bits == 23) || (nanos != Nanosecond.per(Second))
        { (secs, nanos) } else { (secs + 1, 0) } } else if exp < 63 { let secs =
        u64::from(mant) << (exp - $mant_bits); (secs, 0) } else if bits == (i64::MIN as
        $float_ty).to_bits() { break 'value Self::new_unchecked(i64::MIN, 0); } else if
        $secs .is_nan() { $is_nan } else { $is_overflow }; let mask = (bits as
        $bits_ty_signed) >> ($mant_bits + $exp_bits); #[allow(trivial_numeric_casts)] let
        secs_signed = ((secs as i64) ^ (mask as i64)) - (mask as i64);
        #[allow(trivial_numeric_casts)] let nanos_signed = ((nanos as i32) ^ (mask as
        i32)) - (mask as i32); Self::new_unchecked(secs_signed, nanos_signed) } }
    };
}
impl Duration {
    /// Equivalent to `0.seconds()`.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(Duration::ZERO, 0.seconds());
    /// ```
    pub const ZERO: Self = Self::seconds(0);
    /// Equivalent to `1.nanoseconds()`.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(Duration::NANOSECOND, 1.nanoseconds());
    /// ```
    pub const NANOSECOND: Self = Self::nanoseconds(1);
    /// Equivalent to `1.microseconds()`.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(Duration::MICROSECOND, 1.microseconds());
    /// ```
    pub const MICROSECOND: Self = Self::microseconds(1);
    /// Equivalent to `1.milliseconds()`.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(Duration::MILLISECOND, 1.milliseconds());
    /// ```
    pub const MILLISECOND: Self = Self::milliseconds(1);
    /// Equivalent to `1.seconds()`.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(Duration::SECOND, 1.seconds());
    /// ```
    pub const SECOND: Self = Self::seconds(1);
    /// Equivalent to `1.minutes()`.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(Duration::MINUTE, 1.minutes());
    /// ```
    pub const MINUTE: Self = Self::minutes(1);
    /// Equivalent to `1.hours()`.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(Duration::HOUR, 1.hours());
    /// ```
    pub const HOUR: Self = Self::hours(1);
    /// Equivalent to `1.days()`.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(Duration::DAY, 1.days());
    /// ```
    pub const DAY: Self = Self::days(1);
    /// Equivalent to `1.weeks()`.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(Duration::WEEK, 1.weeks());
    /// ```
    pub const WEEK: Self = Self::weeks(1);
    /// The minimum possible duration. Adding any negative duration to this will cause an overflow.
    pub const MIN: Self = Self::new_unchecked(
        i64::MIN,
        -((Nanosecond.per(Second) - 1) as i32),
    );
    /// The maximum possible duration. Adding any positive duration to this will cause an overflow.
    pub const MAX: Self = Self::new_unchecked(
        i64::MAX,
        (Nanosecond.per(Second) - 1) as _,
    );
    /// Check if a duration is exactly zero.
    ///
    /// ```rust
    /// # use time::ext::NumericalDuration;
    /// assert!(0.seconds().is_zero());
    /// assert!(!1.nanoseconds().is_zero());
    /// ```
    pub const fn is_zero(self) -> bool {
        self.seconds == 0 && self.nanoseconds == 0
    }
    /// Check if a duration is negative.
    ///
    /// ```rust
    /// # use time::ext::NumericalDuration;
    /// assert!((-1).seconds().is_negative());
    /// assert!(!0.seconds().is_negative());
    /// assert!(!1.seconds().is_negative());
    /// ```
    pub const fn is_negative(self) -> bool {
        self.seconds < 0 || self.nanoseconds < 0
    }
    /// Check if a duration is positive.
    ///
    /// ```rust
    /// # use time::ext::NumericalDuration;
    /// assert!(1.seconds().is_positive());
    /// assert!(!0.seconds().is_positive());
    /// assert!(!(-1).seconds().is_positive());
    /// ```
    pub const fn is_positive(self) -> bool {
        self.seconds > 0 || self.nanoseconds > 0
    }
    /// Get the absolute value of the duration.
    ///
    /// This method saturates the returned value if it would otherwise overflow.
    ///
    /// ```rust
    /// # use time::ext::NumericalDuration;
    /// assert_eq!(1.seconds().abs(), 1.seconds());
    /// assert_eq!(0.seconds().abs(), 0.seconds());
    /// assert_eq!((-1).seconds().abs(), 1.seconds());
    /// ```
    pub const fn abs(self) -> Self {
        match self.seconds.checked_abs() {
            Some(seconds) => Self::new_unchecked(seconds, self.nanoseconds.abs()),
            None => Self::MAX,
        }
    }
    /// Convert the existing `Duration` to a `std::time::Duration` and its sign. This returns a
    /// [`std::time::Duration`] and does not saturate the returned value (unlike [`Duration::abs`]).
    ///
    /// ```rust
    /// # use time::ext::{NumericalDuration, NumericalStdDuration};
    /// assert_eq!(1.seconds().unsigned_abs(), 1.std_seconds());
    /// assert_eq!(0.seconds().unsigned_abs(), 0.std_seconds());
    /// assert_eq!((-1).seconds().unsigned_abs(), 1.std_seconds());
    /// ```
    pub const fn unsigned_abs(self) -> StdDuration {
        StdDuration::new(self.seconds.unsigned_abs(), self.nanoseconds.unsigned_abs())
    }
    /// Create a new `Duration` without checking the validity of the components.
    pub(crate) const fn new_unchecked(seconds: i64, nanoseconds: i32) -> Self {
        if seconds < 0 {
            debug_assert!(nanoseconds <= 0);
            debug_assert!(nanoseconds > - (Nanosecond.per(Second) as i32));
        } else if seconds > 0 {
            debug_assert!(nanoseconds >= 0);
            debug_assert!(nanoseconds < Nanosecond.per(Second) as _);
        } else {
            debug_assert!(nanoseconds.unsigned_abs() < Nanosecond.per(Second));
        }
        Self {
            seconds,
            nanoseconds,
            padding: Padding::Optimize,
        }
    }
    /// Create a new `Duration` with the provided seconds and nanoseconds. If nanoseconds is at
    /// least ±10<sup>9</sup>, it will wrap to the number of seconds.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(Duration::new(1, 0), 1.seconds());
    /// assert_eq!(Duration::new(-1, 0), (-1).seconds());
    /// assert_eq!(Duration::new(1, 2_000_000_000), 3.seconds());
    /// ```
    pub const fn new(mut seconds: i64, mut nanoseconds: i32) -> Self {
        seconds = expect_opt!(
            seconds.checked_add(nanoseconds as i64 / Nanosecond.per(Second) as i64),
            "overflow constructing `time::Duration`"
        );
        nanoseconds %= Nanosecond.per(Second) as i32;
        if seconds > 0 && nanoseconds < 0 {
            seconds -= 1;
            nanoseconds += Nanosecond.per(Second) as i32;
        } else if seconds < 0 && nanoseconds > 0 {
            seconds += 1;
            nanoseconds -= Nanosecond.per(Second) as i32;
        }
        Self::new_unchecked(seconds, nanoseconds)
    }
    /// Create a new `Duration` with the given number of weeks. Equivalent to
    /// `Duration::seconds(weeks * 604_800)`.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(Duration::weeks(1), 604_800.seconds());
    /// ```
    pub const fn weeks(weeks: i64) -> Self {
        Self::seconds(
            expect_opt!(
                weeks.checked_mul(Second.per(Week) as _),
                "overflow constructing `time::Duration`"
            ),
        )
    }
    /// Create a new `Duration` with the given number of days. Equivalent to
    /// `Duration::seconds(days * 86_400)`.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(Duration::days(1), 86_400.seconds());
    /// ```
    pub const fn days(days: i64) -> Self {
        Self::seconds(
            expect_opt!(
                days.checked_mul(Second.per(Day) as _),
                "overflow constructing `time::Duration`"
            ),
        )
    }
    /// Create a new `Duration` with the given number of hours. Equivalent to
    /// `Duration::seconds(hours * 3_600)`.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(Duration::hours(1), 3_600.seconds());
    /// ```
    pub const fn hours(hours: i64) -> Self {
        Self::seconds(
            expect_opt!(
                hours.checked_mul(Second.per(Hour) as _),
                "overflow constructing `time::Duration`"
            ),
        )
    }
    /// Create a new `Duration` with the given number of minutes. Equivalent to
    /// `Duration::seconds(minutes * 60)`.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(Duration::minutes(1), 60.seconds());
    /// ```
    pub const fn minutes(minutes: i64) -> Self {
        Self::seconds(
            expect_opt!(
                minutes.checked_mul(Second.per(Minute) as _),
                "overflow constructing `time::Duration`"
            ),
        )
    }
    /// Create a new `Duration` with the given number of seconds.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(Duration::seconds(1), 1_000.milliseconds());
    /// ```
    pub const fn seconds(seconds: i64) -> Self {
        Self::new_unchecked(seconds, 0)
    }
    /// Creates a new `Duration` from the specified number of seconds represented as `f64`.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(Duration::seconds_f64(0.5), 0.5.seconds());
    /// assert_eq!(Duration::seconds_f64(-0.5), -0.5.seconds());
    /// ```
    pub fn seconds_f64(seconds: f64) -> Self {
        try_from_secs!(
            secs = seconds, mantissa_bits = 52, exponent_bits = 11, offset = 44, bits_ty
            = u64, bits_ty_signed = i64, double_ty = u128, float_ty = f64, is_nan = crate
            ::expect_failed("passed NaN to `time::Duration::seconds_f64`"), is_overflow =
            crate ::expect_failed("overflow constructing `time::Duration`"),
        )
    }
    /// Creates a new `Duration` from the specified number of seconds represented as `f32`.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(Duration::seconds_f32(0.5), 0.5.seconds());
    /// assert_eq!(Duration::seconds_f32(-0.5), (-0.5).seconds());
    /// ```
    pub fn seconds_f32(seconds: f32) -> Self {
        try_from_secs!(
            secs = seconds, mantissa_bits = 23, exponent_bits = 8, offset = 41, bits_ty =
            u32, bits_ty_signed = i32, double_ty = u64, float_ty = f32, is_nan = crate
            ::expect_failed("passed NaN to `time::Duration::seconds_f32`"), is_overflow =
            crate ::expect_failed("overflow constructing `time::Duration`"),
        )
    }
    /// Creates a new `Duration` from the specified number of seconds
    /// represented as `f64`. Any values that are out of bounds are saturated at
    /// the minimum or maximum respectively. `NaN` gets turned into a `Duration`
    /// of 0 seconds.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(Duration::saturating_seconds_f64(0.5), 0.5.seconds());
    /// assert_eq!(Duration::saturating_seconds_f64(-0.5), -0.5.seconds());
    /// assert_eq!(
    ///     Duration::saturating_seconds_f64(f64::NAN),
    ///     Duration::new(0, 0),
    /// );
    /// assert_eq!(
    ///     Duration::saturating_seconds_f64(f64::NEG_INFINITY),
    ///     Duration::MIN,
    /// );
    /// assert_eq!(
    ///     Duration::saturating_seconds_f64(f64::INFINITY),
    ///     Duration::MAX,
    /// );
    /// ```
    pub fn saturating_seconds_f64(seconds: f64) -> Self {
        try_from_secs!(
            secs = seconds, mantissa_bits = 52, exponent_bits = 11, offset = 44, bits_ty
            = u64, bits_ty_signed = i64, double_ty = u128, float_ty = f64, is_nan =
            return Self::ZERO, is_overflow = return if seconds < 0.0 { Self::MIN } else {
            Self::MAX },
        )
    }
    /// Creates a new `Duration` from the specified number of seconds
    /// represented as `f32`. Any values that are out of bounds are saturated at
    /// the minimum or maximum respectively. `NaN` gets turned into a `Duration`
    /// of 0 seconds.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(Duration::saturating_seconds_f32(0.5), 0.5.seconds());
    /// assert_eq!(Duration::saturating_seconds_f32(-0.5), (-0.5).seconds());
    /// assert_eq!(
    ///     Duration::saturating_seconds_f32(f32::NAN),
    ///     Duration::new(0, 0),
    /// );
    /// assert_eq!(
    ///     Duration::saturating_seconds_f32(f32::NEG_INFINITY),
    ///     Duration::MIN,
    /// );
    /// assert_eq!(
    ///     Duration::saturating_seconds_f32(f32::INFINITY),
    ///     Duration::MAX,
    /// );
    /// ```
    pub fn saturating_seconds_f32(seconds: f32) -> Self {
        try_from_secs!(
            secs = seconds, mantissa_bits = 23, exponent_bits = 8, offset = 41, bits_ty =
            u32, bits_ty_signed = i32, double_ty = u64, float_ty = f32, is_nan = return
            Self::ZERO, is_overflow = return if seconds < 0.0 { Self::MIN } else {
            Self::MAX },
        )
    }
    /// Creates a new `Duration` from the specified number of seconds
    /// represented as `f64`. Returns `None` if the `Duration` can't be
    /// represented.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(Duration::checked_seconds_f64(0.5), Some(0.5.seconds()));
    /// assert_eq!(Duration::checked_seconds_f64(-0.5), Some(-0.5.seconds()));
    /// assert_eq!(Duration::checked_seconds_f64(f64::NAN), None);
    /// assert_eq!(Duration::checked_seconds_f64(f64::NEG_INFINITY), None);
    /// assert_eq!(Duration::checked_seconds_f64(f64::INFINITY), None);
    /// ```
    pub fn checked_seconds_f64(seconds: f64) -> Option<Self> {
        Some(
            try_from_secs!(
                secs = seconds, mantissa_bits = 52, exponent_bits = 11, offset = 44,
                bits_ty = u64, bits_ty_signed = i64, double_ty = u128, float_ty = f64,
                is_nan = return None, is_overflow = return None,
            ),
        )
    }
    /// Creates a new `Duration` from the specified number of seconds
    /// represented as `f32`. Returns `None` if the `Duration` can't be
    /// represented.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(Duration::checked_seconds_f32(0.5), Some(0.5.seconds()));
    /// assert_eq!(Duration::checked_seconds_f32(-0.5), Some(-0.5.seconds()));
    /// assert_eq!(Duration::checked_seconds_f32(f32::NAN), None);
    /// assert_eq!(Duration::checked_seconds_f32(f32::NEG_INFINITY), None);
    /// assert_eq!(Duration::checked_seconds_f32(f32::INFINITY), None);
    /// ```
    pub fn checked_seconds_f32(seconds: f32) -> Option<Self> {
        Some(
            try_from_secs!(
                secs = seconds, mantissa_bits = 23, exponent_bits = 8, offset = 41,
                bits_ty = u32, bits_ty_signed = i32, double_ty = u64, float_ty = f32,
                is_nan = return None, is_overflow = return None,
            ),
        )
    }
    /// Create a new `Duration` with the given number of milliseconds.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(Duration::milliseconds(1), 1_000.microseconds());
    /// assert_eq!(Duration::milliseconds(-1), (-1_000).microseconds());
    /// ```
    pub const fn milliseconds(milliseconds: i64) -> Self {
        Self::new_unchecked(
            milliseconds / Millisecond.per(Second) as i64,
            (milliseconds % Millisecond.per(Second) as i64
                * Nanosecond.per(Millisecond) as i64) as _,
        )
    }
    /// Create a new `Duration` with the given number of microseconds.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(Duration::microseconds(1), 1_000.nanoseconds());
    /// assert_eq!(Duration::microseconds(-1), (-1_000).nanoseconds());
    /// ```
    pub const fn microseconds(microseconds: i64) -> Self {
        Self::new_unchecked(
            microseconds / Microsecond.per(Second) as i64,
            (microseconds % Microsecond.per(Second) as i64
                * Nanosecond.per(Microsecond) as i64) as _,
        )
    }
    /// Create a new `Duration` with the given number of nanoseconds.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(Duration::nanoseconds(1), 1.microseconds() / 1_000);
    /// assert_eq!(Duration::nanoseconds(-1), (-1).microseconds() / 1_000);
    /// ```
    pub const fn nanoseconds(nanoseconds: i64) -> Self {
        Self::new_unchecked(
            nanoseconds / Nanosecond.per(Second) as i64,
            (nanoseconds % Nanosecond.per(Second) as i64) as _,
        )
    }
    /// Create a new `Duration` with the given number of nanoseconds.
    ///
    /// As the input range cannot be fully mapped to the output, this should only be used where it's
    /// known to result in a valid value.
    pub(crate) const fn nanoseconds_i128(nanoseconds: i128) -> Self {
        let seconds = nanoseconds / Nanosecond.per(Second) as i128;
        let nanoseconds = nanoseconds % Nanosecond.per(Second) as i128;
        if seconds > i64::MAX as i128 || seconds < i64::MIN as i128 {
            crate::expect_failed("overflow constructing `time::Duration`");
        }
        Self::new_unchecked(seconds as _, nanoseconds as _)
    }
    /// Get the number of whole weeks in the duration.
    ///
    /// ```rust
    /// # use time::ext::NumericalDuration;
    /// assert_eq!(1.weeks().whole_weeks(), 1);
    /// assert_eq!((-1).weeks().whole_weeks(), -1);
    /// assert_eq!(6.days().whole_weeks(), 0);
    /// assert_eq!((-6).days().whole_weeks(), 0);
    /// ```
    pub const fn whole_weeks(self) -> i64 {
        self.whole_seconds() / Second.per(Week) as i64
    }
    /// Get the number of whole days in the duration.
    ///
    /// ```rust
    /// # use time::ext::NumericalDuration;
    /// assert_eq!(1.days().whole_days(), 1);
    /// assert_eq!((-1).days().whole_days(), -1);
    /// assert_eq!(23.hours().whole_days(), 0);
    /// assert_eq!((-23).hours().whole_days(), 0);
    /// ```
    pub const fn whole_days(self) -> i64 {
        self.whole_seconds() / Second.per(Day) as i64
    }
    /// Get the number of whole hours in the duration.
    ///
    /// ```rust
    /// # use time::ext::NumericalDuration;
    /// assert_eq!(1.hours().whole_hours(), 1);
    /// assert_eq!((-1).hours().whole_hours(), -1);
    /// assert_eq!(59.minutes().whole_hours(), 0);
    /// assert_eq!((-59).minutes().whole_hours(), 0);
    /// ```
    pub const fn whole_hours(self) -> i64 {
        self.whole_seconds() / Second.per(Hour) as i64
    }
    /// Get the number of whole minutes in the duration.
    ///
    /// ```rust
    /// # use time::ext::NumericalDuration;
    /// assert_eq!(1.minutes().whole_minutes(), 1);
    /// assert_eq!((-1).minutes().whole_minutes(), -1);
    /// assert_eq!(59.seconds().whole_minutes(), 0);
    /// assert_eq!((-59).seconds().whole_minutes(), 0);
    /// ```
    pub const fn whole_minutes(self) -> i64 {
        self.whole_seconds() / Second.per(Minute) as i64
    }
    /// Get the number of whole seconds in the duration.
    ///
    /// ```rust
    /// # use time::ext::NumericalDuration;
    /// assert_eq!(1.seconds().whole_seconds(), 1);
    /// assert_eq!((-1).seconds().whole_seconds(), -1);
    /// assert_eq!(1.minutes().whole_seconds(), 60);
    /// assert_eq!((-1).minutes().whole_seconds(), -60);
    /// ```
    pub const fn whole_seconds(self) -> i64 {
        self.seconds
    }
    /// Get the number of fractional seconds in the duration.
    ///
    /// ```rust
    /// # use time::ext::NumericalDuration;
    /// assert_eq!(1.5.seconds().as_seconds_f64(), 1.5);
    /// assert_eq!((-1.5).seconds().as_seconds_f64(), -1.5);
    /// ```
    pub fn as_seconds_f64(self) -> f64 {
        self.seconds as f64 + self.nanoseconds as f64 / Nanosecond.per(Second) as f64
    }
    /// Get the number of fractional seconds in the duration.
    ///
    /// ```rust
    /// # use time::ext::NumericalDuration;
    /// assert_eq!(1.5.seconds().as_seconds_f32(), 1.5);
    /// assert_eq!((-1.5).seconds().as_seconds_f32(), -1.5);
    /// ```
    pub fn as_seconds_f32(self) -> f32 {
        self.seconds as f32 + self.nanoseconds as f32 / Nanosecond.per(Second) as f32
    }
    /// Get the number of whole milliseconds in the duration.
    ///
    /// ```rust
    /// # use time::ext::NumericalDuration;
    /// assert_eq!(1.seconds().whole_milliseconds(), 1_000);
    /// assert_eq!((-1).seconds().whole_milliseconds(), -1_000);
    /// assert_eq!(1.milliseconds().whole_milliseconds(), 1);
    /// assert_eq!((-1).milliseconds().whole_milliseconds(), -1);
    /// ```
    pub const fn whole_milliseconds(self) -> i128 {
        self.seconds as i128 * Millisecond.per(Second) as i128
            + self.nanoseconds as i128 / Nanosecond.per(Millisecond) as i128
    }
    /// Get the number of milliseconds past the number of whole seconds.
    ///
    /// Always in the range `-1_000..1_000`.
    ///
    /// ```rust
    /// # use time::ext::NumericalDuration;
    /// assert_eq!(1.4.seconds().subsec_milliseconds(), 400);
    /// assert_eq!((-1.4).seconds().subsec_milliseconds(), -400);
    /// ```
    pub const fn subsec_milliseconds(self) -> i16 {
        (self.nanoseconds / Nanosecond.per(Millisecond) as i32) as _
    }
    /// Get the number of whole microseconds in the duration.
    ///
    /// ```rust
    /// # use time::ext::NumericalDuration;
    /// assert_eq!(1.milliseconds().whole_microseconds(), 1_000);
    /// assert_eq!((-1).milliseconds().whole_microseconds(), -1_000);
    /// assert_eq!(1.microseconds().whole_microseconds(), 1);
    /// assert_eq!((-1).microseconds().whole_microseconds(), -1);
    /// ```
    pub const fn whole_microseconds(self) -> i128 {
        self.seconds as i128 * Microsecond.per(Second) as i128
            + self.nanoseconds as i128 / Nanosecond.per(Microsecond) as i128
    }
    /// Get the number of microseconds past the number of whole seconds.
    ///
    /// Always in the range `-1_000_000..1_000_000`.
    ///
    /// ```rust
    /// # use time::ext::NumericalDuration;
    /// assert_eq!(1.0004.seconds().subsec_microseconds(), 400);
    /// assert_eq!((-1.0004).seconds().subsec_microseconds(), -400);
    /// ```
    pub const fn subsec_microseconds(self) -> i32 {
        self.nanoseconds / Nanosecond.per(Microsecond) as i32
    }
    /// Get the number of nanoseconds in the duration.
    ///
    /// ```rust
    /// # use time::ext::NumericalDuration;
    /// assert_eq!(1.microseconds().whole_nanoseconds(), 1_000);
    /// assert_eq!((-1).microseconds().whole_nanoseconds(), -1_000);
    /// assert_eq!(1.nanoseconds().whole_nanoseconds(), 1);
    /// assert_eq!((-1).nanoseconds().whole_nanoseconds(), -1);
    /// ```
    pub const fn whole_nanoseconds(self) -> i128 {
        self.seconds as i128 * Nanosecond.per(Second) as i128 + self.nanoseconds as i128
    }
    /// Get the number of nanoseconds past the number of whole seconds.
    ///
    /// The returned value will always be in the range `-1_000_000_000..1_000_000_000`.
    ///
    /// ```rust
    /// # use time::ext::NumericalDuration;
    /// assert_eq!(1.000_000_400.seconds().subsec_nanoseconds(), 400);
    /// assert_eq!((-1.000_000_400).seconds().subsec_nanoseconds(), -400);
    /// ```
    pub const fn subsec_nanoseconds(self) -> i32 {
        self.nanoseconds
    }
    /// Computes `self + rhs`, returning `None` if an overflow occurred.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(5.seconds().checked_add(5.seconds()), Some(10.seconds()));
    /// assert_eq!(Duration::MAX.checked_add(1.nanoseconds()), None);
    /// assert_eq!((-5).seconds().checked_add(5.seconds()), Some(0.seconds()));
    /// ```
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        let mut seconds = const_try_opt!(self.seconds.checked_add(rhs.seconds));
        let mut nanoseconds = self.nanoseconds + rhs.nanoseconds;
        if nanoseconds >= Nanosecond.per(Second) as _ || seconds < 0 && nanoseconds > 0 {
            nanoseconds -= Nanosecond.per(Second) as i32;
            seconds = const_try_opt!(seconds.checked_add(1));
        } else if nanoseconds <= -(Nanosecond.per(Second) as i32)
            || seconds > 0 && nanoseconds < 0
        {
            nanoseconds += Nanosecond.per(Second) as i32;
            seconds = const_try_opt!(seconds.checked_sub(1));
        }
        Some(Self::new_unchecked(seconds, nanoseconds))
    }
    /// Computes `self - rhs`, returning `None` if an overflow occurred.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(5.seconds().checked_sub(5.seconds()), Some(Duration::ZERO));
    /// assert_eq!(Duration::MIN.checked_sub(1.nanoseconds()), None);
    /// assert_eq!(5.seconds().checked_sub(10.seconds()), Some((-5).seconds()));
    /// ```
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        let mut seconds = const_try_opt!(self.seconds.checked_sub(rhs.seconds));
        let mut nanoseconds = self.nanoseconds - rhs.nanoseconds;
        if nanoseconds >= Nanosecond.per(Second) as _ || seconds < 0 && nanoseconds > 0 {
            nanoseconds -= Nanosecond.per(Second) as i32;
            seconds = const_try_opt!(seconds.checked_add(1));
        } else if nanoseconds <= -(Nanosecond.per(Second) as i32)
            || seconds > 0 && nanoseconds < 0
        {
            nanoseconds += Nanosecond.per(Second) as i32;
            seconds = const_try_opt!(seconds.checked_sub(1));
        }
        Some(Self::new_unchecked(seconds, nanoseconds))
    }
    /// Computes `self * rhs`, returning `None` if an overflow occurred.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(5.seconds().checked_mul(2), Some(10.seconds()));
    /// assert_eq!(5.seconds().checked_mul(-2), Some((-10).seconds()));
    /// assert_eq!(5.seconds().checked_mul(0), Some(0.seconds()));
    /// assert_eq!(Duration::MAX.checked_mul(2), None);
    /// assert_eq!(Duration::MIN.checked_mul(2), None);
    /// ```
    pub const fn checked_mul(self, rhs: i32) -> Option<Self> {
        let total_nanos = self.nanoseconds as i64 * rhs as i64;
        let extra_secs = total_nanos / Nanosecond.per(Second) as i64;
        let nanoseconds = (total_nanos % Nanosecond.per(Second) as i64) as _;
        let seconds = const_try_opt!(
            const_try_opt!(self.seconds.checked_mul(rhs as _)) .checked_add(extra_secs)
        );
        Some(Self::new_unchecked(seconds, nanoseconds))
    }
    /// Computes `self / rhs`, returning `None` if `rhs == 0` or if the result would overflow.
    ///
    /// ```rust
    /// # use time::ext::NumericalDuration;
    /// assert_eq!(10.seconds().checked_div(2), Some(5.seconds()));
    /// assert_eq!(10.seconds().checked_div(-2), Some((-5).seconds()));
    /// assert_eq!(1.seconds().checked_div(0), None);
    /// ```
    pub const fn checked_div(self, rhs: i32) -> Option<Self> {
        let seconds = const_try_opt!(self.seconds.checked_div(rhs as i64));
        let carry = self.seconds - seconds * (rhs as i64);
        let extra_nanos = const_try_opt!(
            (carry * Nanosecond.per(Second) as i64).checked_div(rhs as i64)
        );
        let nanoseconds = const_try_opt!(self.nanoseconds.checked_div(rhs))
            + (extra_nanos as i32);
        Some(Self::new_unchecked(seconds, nanoseconds))
    }
    /// Computes `self + rhs`, saturating if an overflow occurred.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(5.seconds().saturating_add(5.seconds()), 10.seconds());
    /// assert_eq!(Duration::MAX.saturating_add(1.nanoseconds()), Duration::MAX);
    /// assert_eq!(
    ///     Duration::MIN.saturating_add((-1).nanoseconds()),
    ///     Duration::MIN
    /// );
    /// assert_eq!((-5).seconds().saturating_add(5.seconds()), Duration::ZERO);
    /// ```
    pub const fn saturating_add(self, rhs: Self) -> Self {
        let (mut seconds, overflow) = self.seconds.overflowing_add(rhs.seconds);
        if overflow {
            if self.seconds > 0 {
                return Self::MAX;
            }
            return Self::MIN;
        }
        let mut nanoseconds = self.nanoseconds + rhs.nanoseconds;
        if nanoseconds >= Nanosecond.per(Second) as _ || seconds < 0 && nanoseconds > 0 {
            nanoseconds -= Nanosecond.per(Second) as i32;
            seconds = match seconds.checked_add(1) {
                Some(seconds) => seconds,
                None => return Self::MAX,
            };
        } else if nanoseconds <= -(Nanosecond.per(Second) as i32)
            || seconds > 0 && nanoseconds < 0
        {
            nanoseconds += Nanosecond.per(Second) as i32;
            seconds = match seconds.checked_sub(1) {
                Some(seconds) => seconds,
                None => return Self::MIN,
            };
        }
        Self::new_unchecked(seconds, nanoseconds)
    }
    /// Computes `self - rhs`, saturating if an overflow occurred.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(5.seconds().saturating_sub(5.seconds()), Duration::ZERO);
    /// assert_eq!(Duration::MIN.saturating_sub(1.nanoseconds()), Duration::MIN);
    /// assert_eq!(
    ///     Duration::MAX.saturating_sub((-1).nanoseconds()),
    ///     Duration::MAX
    /// );
    /// assert_eq!(5.seconds().saturating_sub(10.seconds()), (-5).seconds());
    /// ```
    pub const fn saturating_sub(self, rhs: Self) -> Self {
        let (mut seconds, overflow) = self.seconds.overflowing_sub(rhs.seconds);
        if overflow {
            if self.seconds > 0 {
                return Self::MAX;
            }
            return Self::MIN;
        }
        let mut nanoseconds = self.nanoseconds - rhs.nanoseconds;
        if nanoseconds >= Nanosecond.per(Second) as _ || seconds < 0 && nanoseconds > 0 {
            nanoseconds -= Nanosecond.per(Second) as i32;
            seconds = match seconds.checked_add(1) {
                Some(seconds) => seconds,
                None => return Self::MAX,
            };
        } else if nanoseconds <= -(Nanosecond.per(Second) as i32)
            || seconds > 0 && nanoseconds < 0
        {
            nanoseconds += Nanosecond.per(Second) as i32;
            seconds = match seconds.checked_sub(1) {
                Some(seconds) => seconds,
                None => return Self::MIN,
            };
        }
        Self::new_unchecked(seconds, nanoseconds)
    }
    /// Computes `self * rhs`, saturating if an overflow occurred.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(5.seconds().saturating_mul(2), 10.seconds());
    /// assert_eq!(5.seconds().saturating_mul(-2), (-10).seconds());
    /// assert_eq!(5.seconds().saturating_mul(0), Duration::ZERO);
    /// assert_eq!(Duration::MAX.saturating_mul(2), Duration::MAX);
    /// assert_eq!(Duration::MIN.saturating_mul(2), Duration::MIN);
    /// assert_eq!(Duration::MAX.saturating_mul(-2), Duration::MIN);
    /// assert_eq!(Duration::MIN.saturating_mul(-2), Duration::MAX);
    /// ```
    pub const fn saturating_mul(self, rhs: i32) -> Self {
        let total_nanos = self.nanoseconds as i64 * rhs as i64;
        let extra_secs = total_nanos / Nanosecond.per(Second) as i64;
        let nanoseconds = (total_nanos % Nanosecond.per(Second) as i64) as _;
        let (seconds, overflow1) = self.seconds.overflowing_mul(rhs as _);
        if overflow1 {
            if self.seconds > 0 && rhs > 0 || self.seconds < 0 && rhs < 0 {
                return Self::MAX;
            }
            return Self::MIN;
        }
        let (seconds, overflow2) = seconds.overflowing_add(extra_secs);
        if overflow2 {
            if self.seconds > 0 && rhs > 0 {
                return Self::MAX;
            }
            return Self::MIN;
        }
        Self::new_unchecked(seconds, nanoseconds)
    }
    /// Runs a closure, returning the duration of time it took to run. The return value of the
    /// closure is provided in the second part of the tuple.
    #[cfg(feature = "std")]
    pub fn time_fn<T>(f: impl FnOnce() -> T) -> (Self, T) {
        let start = Instant::now();
        let return_value = f();
        let end = Instant::now();
        (end - start, return_value)
    }
}
/// The format returned by this implementation is not stable and must not be relied upon.
///
/// By default this produces an exact, full-precision printout of the duration.
/// For a concise, rounded printout instead, you can use the `.N` format specifier:
///
/// ```
/// # use time::Duration;
/// #
/// let duration = Duration::new(123456, 789011223);
/// println!("{duration:.3}");
/// ```
///
/// For the purposes of this implementation, a day is exactly 24 hours and a minute is exactly 60
/// seconds.
impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_negative() {
            f.write_str("-")?;
        }
        if let Some(_precision) = f.precision() {
            if self.is_zero() {
                return (0.).fmt(f).and_then(|_| f.write_str("s"));
            }
            /// Format the first item that produces a value greater than 1 and then break.
            macro_rules! item {
                ($name:literal, $value:expr) => {
                    let value = $value; if value >= 1.0 { return value.fmt(f).and_then(|
                    _ | f.write_str($name)); }
                };
            }
            let seconds = self.unsigned_abs().as_secs_f64();
            item!("d", seconds / Second.per(Day) as f64);
            item!("h", seconds / Second.per(Hour) as f64);
            item!("m", seconds / Second.per(Minute) as f64);
            item!("s", seconds);
            item!("ms", seconds * Millisecond.per(Second) as f64);
            item!("µs", seconds * Microsecond.per(Second) as f64);
            item!("ns", seconds * Nanosecond.per(Second) as f64);
        } else {
            if self.is_zero() {
                return f.write_str("0s");
            }
            /// Format a single item.
            macro_rules! item {
                ($name:literal, $value:expr) => {
                    match $value { 0 => Ok(()), value => value.fmt(f).and_then(| _ | f
                    .write_str($name)), }
                };
            }
            let seconds = self.seconds.unsigned_abs();
            let nanoseconds = self.nanoseconds.unsigned_abs();
            item!("d", seconds / Second.per(Day) as u64)?;
            item!("h", seconds / Second.per(Hour) as u64 % Hour.per(Day) as u64)?;
            item!("m", seconds / Second.per(Minute) as u64 % Minute.per(Hour) as u64)?;
            item!("s", seconds % Second.per(Minute) as u64)?;
            item!("ms", nanoseconds / Nanosecond.per(Millisecond))?;
            item!(
                "µs", nanoseconds / Nanosecond.per(Microsecond) as u32 % Microsecond
                .per(Millisecond) as u32
            )?;
            item!("ns", nanoseconds % Nanosecond.per(Microsecond) as u32)?;
        }
        Ok(())
    }
}
impl TryFrom<StdDuration> for Duration {
    type Error = error::ConversionRange;
    fn try_from(original: StdDuration) -> Result<Self, error::ConversionRange> {
        Ok(
            Self::new(
                original.as_secs().try_into().map_err(|_| error::ConversionRange)?,
                original.subsec_nanos() as _,
            ),
        )
    }
}
impl TryFrom<Duration> for StdDuration {
    type Error = error::ConversionRange;
    fn try_from(duration: Duration) -> Result<Self, error::ConversionRange> {
        Ok(
            Self::new(
                duration.seconds.try_into().map_err(|_| error::ConversionRange)?,
                duration.nanoseconds.try_into().map_err(|_| error::ConversionRange)?,
            ),
        )
    }
}
impl Add for Duration {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self.checked_add(rhs).expect("overflow when adding durations")
    }
}
impl Add<StdDuration> for Duration {
    type Output = Self;
    fn add(self, std_duration: StdDuration) -> Self::Output {
        self
            + Self::try_from(std_duration)
                .expect("overflow converting `std::time::Duration` to `time::Duration`")
    }
}
impl Add<Duration> for StdDuration {
    type Output = Duration;
    fn add(self, rhs: Duration) -> Self::Output {
        rhs + self
    }
}
impl_add_assign!(Duration : Self, StdDuration);
impl AddAssign<Duration> for StdDuration {
    fn add_assign(&mut self, rhs: Duration) {
        *self = (*self + rhs)
            .try_into()
            .expect(
                "Cannot represent a resulting duration in std. Try `let x = x + rhs;`, which will \
             change the type.",
            );
    }
}
impl Neg for Duration {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new_unchecked(-self.seconds, -self.nanoseconds)
    }
}
impl Sub for Duration {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self.checked_sub(rhs).expect("overflow when subtracting durations")
    }
}
impl Sub<StdDuration> for Duration {
    type Output = Self;
    fn sub(self, rhs: StdDuration) -> Self::Output {
        self
            - Self::try_from(rhs)
                .expect("overflow converting `std::time::Duration` to `time::Duration`")
    }
}
impl Sub<Duration> for StdDuration {
    type Output = Duration;
    fn sub(self, rhs: Duration) -> Self::Output {
        Duration::try_from(self)
            .expect("overflow converting `std::time::Duration` to `time::Duration`")
            - rhs
    }
}
impl_sub_assign!(Duration : Self, StdDuration);
impl SubAssign<Duration> for StdDuration {
    fn sub_assign(&mut self, rhs: Duration) {
        *self = (*self - rhs)
            .try_into()
            .expect(
                "Cannot represent a resulting duration in std. Try `let x = x - rhs;`, which will \
             change the type.",
            );
    }
}
/// Implement `Mul` (reflexively) and `Div` for `Duration` for various types.
macro_rules! duration_mul_div_int {
    ($($type:ty),+) => {
        $(impl Mul <$type > for Duration { type Output = Self; fn mul(self, rhs : $type)
        -> Self::Output { Self::nanoseconds_i128(self.whole_nanoseconds().checked_mul(rhs
        as _).expect("overflow when multiplying duration")) } } impl Mul < Duration > for
        $type { type Output = Duration; fn mul(self, rhs : Duration) -> Self::Output {
        rhs * self } } impl Div <$type > for Duration { type Output = Self; fn div(self,
        rhs : $type) -> Self::Output { Self::nanoseconds_i128(self.whole_nanoseconds() /
        rhs as i128) } })+
    };
}
duration_mul_div_int![i8, i16, i32, u8, u16, u32];
impl Mul<f32> for Duration {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self::seconds_f32(self.as_seconds_f32() * rhs)
    }
}
impl Mul<Duration> for f32 {
    type Output = Duration;
    fn mul(self, rhs: Duration) -> Self::Output {
        rhs * self
    }
}
impl Mul<f64> for Duration {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::seconds_f64(self.as_seconds_f64() * rhs)
    }
}
impl Mul<Duration> for f64 {
    type Output = Duration;
    fn mul(self, rhs: Duration) -> Self::Output {
        rhs * self
    }
}
impl_mul_assign!(Duration : i8, i16, i32, u8, u16, u32, f32, f64);
impl Div<f32> for Duration {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Self::seconds_f32(self.as_seconds_f32() / rhs)
    }
}
impl Div<f64> for Duration {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self::seconds_f64(self.as_seconds_f64() / rhs)
    }
}
impl_div_assign!(Duration : i8, i16, i32, u8, u16, u32, f32, f64);
impl Div for Duration {
    type Output = f64;
    fn div(self, rhs: Self) -> Self::Output {
        self.as_seconds_f64() / rhs.as_seconds_f64()
    }
}
impl Div<StdDuration> for Duration {
    type Output = f64;
    fn div(self, rhs: StdDuration) -> Self::Output {
        self.as_seconds_f64() / rhs.as_secs_f64()
    }
}
impl Div<Duration> for StdDuration {
    type Output = f64;
    fn div(self, rhs: Duration) -> Self::Output {
        self.as_secs_f64() / rhs.as_seconds_f64()
    }
}
impl PartialEq<StdDuration> for Duration {
    fn eq(&self, rhs: &StdDuration) -> bool {
        Ok(*self) == Self::try_from(*rhs)
    }
}
impl PartialEq<Duration> for StdDuration {
    fn eq(&self, rhs: &Duration) -> bool {
        rhs == self
    }
}
impl PartialOrd<StdDuration> for Duration {
    fn partial_cmp(&self, rhs: &StdDuration) -> Option<Ordering> {
        if rhs.as_secs() > i64::MAX as _ {
            return Some(Ordering::Less);
        }
        Some(
            self
                .seconds
                .cmp(&(rhs.as_secs() as _))
                .then_with(|| self.nanoseconds.cmp(&(rhs.subsec_nanos() as _))),
        )
    }
}
impl PartialOrd<Duration> for StdDuration {
    fn partial_cmp(&self, rhs: &Duration) -> Option<Ordering> {
        rhs.partial_cmp(self).map(Ordering::reverse)
    }
}
impl Sum for Duration {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|a, b| a + b).unwrap_or_default()
    }
}
impl<'a> Sum<&'a Self> for Duration {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().sum()
    }
}
#[cfg(test)]
mod tests_llm_16_32 {
    use crate::Duration;
    use std::iter::Sum;
    #[test]
    fn sum_empty() {
        let _rug_st_tests_llm_16_32_rrrruuuugggg_sum_empty = 0;
        let durations: Vec<Duration> = Vec::new();
        debug_assert_eq!(Duration::sum(durations.into_iter()), Duration::ZERO);
        let _rug_ed_tests_llm_16_32_rrrruuuugggg_sum_empty = 0;
    }
    #[test]
    fn sum_single_element() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let durations = vec![Duration::seconds(rug_fuzz_0)];
        debug_assert_eq!(Duration::sum(durations.into_iter()), Duration::seconds(5));
             }
}
}
}    }
    #[test]
    fn sum_multiple_elements() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let durations = vec![
            Duration::seconds(rug_fuzz_0), Duration::seconds(10), Duration::seconds(15)
        ];
        debug_assert_eq!(Duration::sum(durations.into_iter()), Duration::seconds(30));
             }
}
}
}    }
    #[test]
    fn sum_negative_and_positive() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let durations = vec![
            Duration::seconds(rug_fuzz_0), Duration::seconds(- 3), Duration::seconds(2)
        ];
        debug_assert_eq!(Duration::sum(durations.into_iter()), Duration::seconds(4));
             }
}
}
}    }
    #[test]
    fn sum_with_nanoseconds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let durations = vec![
            Duration::nanoseconds(rug_fuzz_0), Duration::nanoseconds(10),
            Duration::nanoseconds(15)
        ];
        debug_assert_eq!(
            Duration::sum(durations.into_iter()), Duration::nanoseconds(30)
        );
             }
}
}
}    }
    #[test]
    fn sum_with_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let max_safe_seconds = i64::MAX / rug_fuzz_0;
        let durations = vec![
            Duration::seconds(max_safe_seconds), Duration::seconds(max_safe_seconds)
        ];
        debug_assert_eq!(
            Duration::sum(durations.into_iter()), Duration::seconds(max_safe_seconds * 2)
        );
             }
}
}
}    }
    #[test]
    fn sum_with_underflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let min_safe_seconds = i64::MIN / rug_fuzz_0;
        let durations = vec![
            Duration::seconds(min_safe_seconds), Duration::seconds(min_safe_seconds)
        ];
        debug_assert_eq!(
            Duration::sum(durations.into_iter()), Duration::seconds(min_safe_seconds * 2)
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_37_llm_16_37 {
    use super::*;
    use crate::*;
    use crate::*;
    use std::ops::Div;
    use std::convert::TryInto;
    #[test]
    fn div_by_positive() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let divisor = rug_fuzz_1;
        let result = duration.div(divisor);
        debug_assert_eq!(result, Duration::seconds(5));
             }
}
}
}    }
    #[test]
    fn div_by_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let divisor = -rug_fuzz_1;
        let result = duration.div(divisor);
        debug_assert_eq!(result, Duration::seconds(- 5));
             }
}
}
}    }
    #[test]
    fn div_by_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let divisor = rug_fuzz_1;
        let result = duration.div(divisor);
        debug_assert_eq!(result, Duration::seconds_f32(f32::INFINITY));
             }
}
}
}    }
    #[test]
    fn div_by_fraction() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let divisor = rug_fuzz_1;
        let result = duration.div(divisor);
        debug_assert_eq!(result, Duration::seconds_f32(4.0));
             }
}
}
}    }
    #[test]
    fn div_by_large_number() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, f32, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let divisor = rug_fuzz_1;
        let result = duration.div(divisor);
        debug_assert!(result < Duration::seconds_f32(rug_fuzz_2));
             }
}
}
}    }
    #[test]
    fn div_by_small_fraction() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let divisor = rug_fuzz_1;
        let result = duration.div(divisor);
        debug_assert_eq!(result, Duration::seconds(100));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_38 {
    use super::*;
    use crate::*;
    use std::ops::Div;
    #[test]
    fn div_duration_by_positive_float() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let divisor = rug_fuzz_1;
        let result = duration.div(divisor);
        debug_assert_eq!(result, Duration::seconds(5));
             }
}
}
}    }
    #[test]
    fn div_duration_by_negative_float() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let divisor = -rug_fuzz_1;
        let result = duration.div(divisor);
        debug_assert_eq!(result, Duration::seconds(- 5));
             }
}
}
}    }
    #[test]
    fn div_duration_by_one() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let result = duration.div(rug_fuzz_1);
        debug_assert_eq!(result, duration);
             }
}
}
}    }
    #[test]
    #[should_panic(expected = "division by zero")]
    fn div_duration_by_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let _result = duration.div(rug_fuzz_1);
             }
}
}
}    }
    #[test]
    fn div_max_duration_by_float() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::MAX;
        let divisor = rug_fuzz_0;
        let result = duration.div(divisor);
        debug_assert_eq!(result, Duration::new(i64::MAX / 2, 499_999_999));
             }
}
}
}    }
    #[test]
    fn div_min_duration_by_float() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::MIN;
        let divisor = rug_fuzz_0;
        let result = duration.div(divisor);
        debug_assert_eq!(result, Duration::new(i64::MIN / 2, - 500_000_000));
             }
}
}
}    }
    #[test]
    fn div_duration_with_fractional_part_by_float() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i32, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::new(rug_fuzz_0, rug_fuzz_1);
        let divisor = rug_fuzz_2;
        let result = duration.div(divisor);
        debug_assert_eq!(result, Duration::new(5, 250_000_000));
             }
}
}
}    }
    #[test]
    fn div_negative_duration_with_fractional_part_by_float() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i32, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::new(-rug_fuzz_0, -rug_fuzz_1);
        let divisor = rug_fuzz_2;
        let result = duration.div(divisor);
        debug_assert_eq!(result, Duration::new(- 5, - 250_000_000));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_39 {
    use super::*;
    use crate::*;
    use std::ops::Div;
    #[test]
    fn test_div_duration_by_i16() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i64, i32, i16, i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let test_cases = vec![
            (Duration::new(rug_fuzz_0, rug_fuzz_1), rug_fuzz_2, Duration::new(rug_fuzz_3,
            rug_fuzz_4)), (Duration::new(- 2, 0), 2i16, Duration::new(- 1, 0)),
            (Duration::new(2, 500_000_000), 2i16, Duration::new(1, 250_000_000)),
            (Duration::new(1, 0), 0i16, Duration::new(0, 0)), (Duration::new(1, 0), -
            1i16, Duration::new(- 1, 0)),
            (Duration::nanoseconds_i128(2_000_000_000_000i128), 2i16,
            Duration::seconds(1))
        ];
        for (duration, divisor, expected) in test_cases {
            let result = duration.div(divisor);
            debug_assert_eq!(
                result, expected, "Dividing {:?} by {} failed", duration, divisor
            );
        }
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_41 {
    use super::*;
    use crate::*;
    use std::ops::Div;
    #[test]
    fn test_div_positive_duration_by_positive_i8() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i8, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let divisor: i8 = rug_fuzz_1;
        let result = Duration::seconds(rug_fuzz_2);
        debug_assert_eq!(duration.div(divisor), result);
             }
}
}
}    }
    #[test]
    fn test_div_positive_duration_by_negative_i8() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i8, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let divisor: i8 = -rug_fuzz_1;
        let result = Duration::seconds(-rug_fuzz_2);
        debug_assert_eq!(duration.div(divisor), result);
             }
}
}
}    }
    #[test]
    fn test_div_negative_duration_by_positive_i8() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i8, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(-rug_fuzz_0);
        let divisor: i8 = rug_fuzz_1;
        let result = Duration::seconds(-rug_fuzz_2);
        debug_assert_eq!(duration.div(divisor), result);
             }
}
}
}    }
    #[test]
    fn test_div_negative_duration_by_negative_i8() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i8, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(-rug_fuzz_0);
        let divisor: i8 = -rug_fuzz_1;
        let result = Duration::seconds(rug_fuzz_2);
        debug_assert_eq!(duration.div(divisor), result);
             }
}
}
}    }
    #[test]
    fn test_div_duration_by_zero_i8() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let divisor: i8 = rug_fuzz_1;
        let result = duration.div(divisor);
        debug_assert!(result.is_negative());
             }
}
}
}    }
    #[test]
    fn test_div_max_duration_by_i8() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::MAX;
        let divisor: i8 = rug_fuzz_0;
        debug_assert_eq!(duration.div(divisor), Duration::MAX);
             }
}
}
}    }
    #[test]
    fn test_div_min_duration_by_i8() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::MIN;
        let divisor: i8 = rug_fuzz_0;
        debug_assert_eq!(duration.div(divisor), Duration::MIN);
             }
}
}
}    }
    #[test]
    fn test_div_duration_by_max_i8() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let divisor: i8 = i8::MAX;
        debug_assert_eq!(duration.div(divisor), Duration::seconds(1));
             }
}
}
}    }
    #[test]
    fn test_div_duration_by_min_i8() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let divisor: i8 = i8::MIN;
        debug_assert_eq!(duration.div(divisor), Duration::seconds(- 1));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_43 {
    use super::*;
    use crate::*;
    use std::ops::Div;
    #[test]
    fn div_by_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let result = std::panic::catch_unwind(|| {
            let _ = duration.div(rug_fuzz_1);
        });
        debug_assert!(result.is_err(), "Dividing duration by zero should panic");
             }
}
}
}    }
    #[test]
    fn positive_duration_division() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let divisor: u16 = rug_fuzz_1;
        let result = duration.div(divisor);
        debug_assert_eq!(result, Duration::seconds(5), "10s divided by 2 should be 5s");
             }
}
}
}    }
    #[test]
    fn negative_duration_division() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(-rug_fuzz_0);
        let divisor: u16 = rug_fuzz_1;
        let result = duration.div(divisor);
        debug_assert_eq!(
            result, Duration::seconds(- 5), "(-10)s divided by 2 should be (-5)s"
        );
             }
}
}
}    }
    #[test]
    fn division_with_nanoseconds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i32, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::new(rug_fuzz_0, rug_fuzz_1);
        let divisor: u16 = rug_fuzz_2;
        let result = duration.div(divisor);
        debug_assert_eq!(
            result, Duration::new(5, 250_000_000), "10.5s divided by 2 should be 5.25s"
        );
             }
}
}
}    }
    #[test]
    fn duration_division_with_fractional_result() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let divisor: u16 = rug_fuzz_1;
        let result = duration.div(divisor);
        debug_assert_eq!(
            result, Duration::new(3, 333_333_333),
            "10s divided by 3 should be approximately 3.333333333s"
        );
             }
}
}
}    }
    #[test]
    fn large_duration_division() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(i64::MAX);
        let divisor: u16 = u16::MAX;
        let result = duration.div(divisor);
        let expected_seconds = i64::MAX / u16::MAX as i64;
        let expected_nanoseconds = ((i64::MAX % u16::MAX as i64) * rug_fuzz_0
            / u16::MAX as i64) as i32;
        let expected_duration = Duration::new(expected_seconds, expected_nanoseconds);
        debug_assert_eq!(
            result, expected_duration,
            "Division of maximum i64 seconds by maximum u16 should be correct"
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_44 {
    use super::*;
    use crate::*;
    use std::ops::Div;
    #[test]
    fn test_div_duration_by_u32_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let divisor: u32 = rug_fuzz_1;
        let result = std::panic::catch_unwind(|| duration.div(divisor));
        debug_assert!(result.is_err());
             }
}
}
}    }
    #[test]
    fn test_div_duration_by_u32_non_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let divisor: u32 = rug_fuzz_1;
        let result = duration.div(divisor);
        debug_assert_eq!(result, Duration::seconds(5));
             }
}
}
}    }
    #[test]
    fn test_div_duration_by_u32_with_nanoseconds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::new(rug_fuzz_0, rug_fuzz_1);
        let divisor: u32 = rug_fuzz_2;
        let result = duration.div(divisor);
        debug_assert_eq!(result, Duration::new(5, 250_000_000));
             }
}
}
}    }
    #[test]
    fn test_div_duration_by_u32_with_negative_duration() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(-rug_fuzz_0);
        let divisor: u32 = rug_fuzz_1;
        let result = duration.div(divisor);
        debug_assert_eq!(result, Duration::seconds(- 5));
             }
}
}
}    }
    #[test]
    fn test_div_duration_by_u32_with_negative_duration_and_nanoseconds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::new(-rug_fuzz_0, -rug_fuzz_1);
        let divisor: u32 = rug_fuzz_2;
        let result = duration.div(divisor);
        debug_assert_eq!(result, Duration::new(- 5, - 250_000_000));
             }
}
}
}    }
    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn test_div_duration_by_zero_panics() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let _result = duration.div(rug_fuzz_1);
             }
}
}
}    }
    #[test]
    fn test_div_duration_by_one() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        debug_assert_eq!(duration.div(rug_fuzz_1), Duration::seconds(1));
             }
}
}
}    }
    #[test]
    fn test_div_duration_with_fractional_result() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::new(rug_fuzz_0, rug_fuzz_1);
        debug_assert_eq!(duration.div(rug_fuzz_2), Duration::new(0, 100_000_000));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_45 {
    use super::*;
    use crate::*;
    #[test]
    fn div_duration_by_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let result = std::panic::catch_unwind(|| {
            let _ = duration.div(rug_fuzz_1);
        });
        debug_assert!(result.is_err(), "Division by zero should panic");
             }
}
}
}    }
    #[test]
    fn div_duration_by_one() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        debug_assert_eq!(
            duration.div(rug_fuzz_1), duration,
            "Division by 1 should yield the original duration"
        );
             }
}
}
}    }
    #[test]
    fn div_duration_integer() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        debug_assert_eq!(
            duration.div(rug_fuzz_1), Duration::seconds(5), "10s / 2 should be 5s"
        );
             }
}
}
}    }
    #[test]
    fn div_duration_result_positive() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        debug_assert!(
            duration.div(rug_fuzz_1).is_positive(), "Division result should be positive"
        );
             }
}
}
}    }
    #[test]
    fn div_duration_result_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(-rug_fuzz_0);
        debug_assert!(
            duration.div(rug_fuzz_1).is_negative(), "Division result should be negative"
        );
             }
}
}
}    }
    #[test]
    fn div_duration_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::new(i64::MAX, rug_fuzz_0);
        let result = std::panic::catch_unwind(|| {
            let _ = duration.div(rug_fuzz_1);
        });
        debug_assert!(result.is_err(), "Division leading to overflow should panic");
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_46 {
    use super::*;
    use crate::*;
    use std::ops::Div;
    #[test]
    fn zero_div_zero() {
        let _rug_st_tests_llm_16_46_rrrruuuugggg_zero_div_zero = 0;
        let zero = Duration::ZERO;
        debug_assert!(zero.div(zero).is_nan());
        let _rug_ed_tests_llm_16_46_rrrruuuugggg_zero_div_zero = 0;
    }
    #[test]
    fn zero_div_nonzero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let zero = Duration::ZERO;
        let nonzero = Duration::seconds(rug_fuzz_0);
        debug_assert_eq!(zero.div(nonzero), 0.0);
             }
}
}
}    }
    #[test]
    fn nonzero_div_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let zero = Duration::ZERO;
        let nonzero = Duration::seconds(rug_fuzz_0);
        let result = nonzero.div(zero);
        debug_assert!(result.is_infinite());
        debug_assert!(result.is_sign_positive());
             }
}
}
}    }
    #[test]
    fn nonzero_div_nonzero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration1 = Duration::seconds(rug_fuzz_0);
        let duration2 = Duration::seconds(rug_fuzz_1);
        debug_assert_eq!(duration1.div(duration2), 3.0);
             }
}
}
}    }
    #[test]
    fn negative_div_positive() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let negative = Duration::seconds(-rug_fuzz_0);
        let positive = Duration::seconds(rug_fuzz_1);
        debug_assert_eq!(negative.div(positive), - 3.0);
             }
}
}
}    }
    #[test]
    fn positive_div_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let positive = Duration::seconds(rug_fuzz_0);
        let negative = Duration::seconds(-rug_fuzz_1);
        debug_assert_eq!(positive.div(negative), - 3.0);
             }
}
}
}    }
    #[test]
    fn negative_div_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration1 = Duration::seconds(-rug_fuzz_0);
        let duration2 = Duration::seconds(-rug_fuzz_1);
        debug_assert_eq!(duration1.div(duration2), 3.0);
             }
}
}
}    }
    #[test]
    fn div_with_nanoseconds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i64, i32, i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration1 = Duration::new(rug_fuzz_0, rug_fuzz_1);
        let duration2 = Duration::new(rug_fuzz_2, rug_fuzz_3);
        debug_assert_eq!(duration1.div(duration2), 3.0);
             }
}
}
}    }
    #[test]
    fn div_with_negative_nanoseconds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i64, i32, i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration1 = Duration::new(-rug_fuzz_0, -rug_fuzz_1);
        let duration2 = Duration::new(rug_fuzz_2, -rug_fuzz_3);
        debug_assert_eq!(duration1.div(duration2), 3.0);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_55 {
    use super::*;
    use crate::*;
    use std::ops::Mul;
    #[test]
    fn mul_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let result = duration.mul(rug_fuzz_1);
        debug_assert_eq!(result, Duration::seconds(0));
             }
}
}
}    }
    #[test]
    fn mul_one() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let result = duration.mul(rug_fuzz_1);
        debug_assert_eq!(result, duration);
             }
}
}
}    }
    #[test]
    fn mul_fraction() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let result = duration.mul(rug_fuzz_1);
        debug_assert_eq!(result, Duration::seconds_f32(2.5));
             }
}
}
}    }
    #[test]
    fn mul_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let result = duration.mul(-rug_fuzz_1);
        debug_assert_eq!(result, Duration::seconds(- 5));
             }
}
}
}    }
    #[test]
    fn mul_large() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let result = duration.mul(f32::MAX);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_56 {
    use super::*;
    use crate::*;
    use std::ops::Mul;
    #[test]
    fn test_mul_with_positive() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let multiplier: f64 = rug_fuzz_1;
        let result = duration.mul(multiplier);
        debug_assert_eq!(result, Duration::seconds(12) + Duration::milliseconds(500));
             }
}
}
}    }
    #[test]
    fn test_mul_with_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let multiplier: f64 = -rug_fuzz_1;
        let result = duration.mul(multiplier);
        debug_assert_eq!(result, - Duration::seconds(12) - Duration::milliseconds(500));
             }
}
}
}    }
    #[test]
    fn test_mul_with_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let multiplier: f64 = rug_fuzz_1;
        let result = duration.mul(multiplier);
        debug_assert_eq!(result, Duration::ZERO);
             }
}
}
}    }
    #[test]
    fn test_mul_with_fraction() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::milliseconds(rug_fuzz_0);
        let multiplier: f64 = rug_fuzz_1;
        let result = duration.mul(multiplier);
        debug_assert_eq!(result, Duration::milliseconds(500));
             }
}
}
}    }
    #[test]
    fn test_mul_with_large_multiplier() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let multiplier: f64 = rug_fuzz_1;
        let result = duration.mul(multiplier);
        debug_assert_eq!(result, Duration::seconds(5e12 as i64));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_57 {
    use super::*;
    use crate::*;
    use std::ops::Mul;
    #[test]
    fn test_mul_with_positive() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let result = duration.mul(rug_fuzz_1);
        debug_assert_eq!(result, Duration::seconds(10));
             }
}
}
}    }
    #[test]
    fn test_mul_with_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let result = duration.mul(-rug_fuzz_1);
        debug_assert_eq!(result, Duration::seconds(- 10));
             }
}
}
}    }
    #[test]
    fn test_mul_with_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let result = duration.mul(rug_fuzz_1);
        debug_assert_eq!(result, Duration::seconds(0));
             }
}
}
}    }
    #[test]
    #[should_panic(expected = "overflow when multiplying duration")]
    fn test_mul_with_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(i64::MAX);
        let _ = duration.mul(rug_fuzz_0);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_58 {
    use crate::Duration;
    use std::ops::Mul;
    #[test]
    fn mul_with_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let multiplier = rug_fuzz_1;
        debug_assert_eq!(
            duration.clone().mul(multiplier), Duration::seconds(0),
            "Multiplying with zero should yield zero duration."
        );
             }
}
}
}    }
    #[test]
    fn mul_with_positive() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let multiplier = rug_fuzz_1;
        debug_assert_eq!(
            duration.clone().mul(multiplier), Duration::seconds(10),
            "Multiplying duration should be equivalent to duration * multiplier."
        );
             }
}
}
}    }
    #[test]
    fn mul_with_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let multiplier = -rug_fuzz_1;
        debug_assert_eq!(
            duration.clone().mul(multiplier), Duration::seconds(- 10),
            "Multiplying with negative should yield a negative duration."
        );
             }
}
}
}    }
    #[test]
    #[should_panic(expected = "overflow when multiplying duration")]
    fn mul_with_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(i64::MAX);
        let multiplier = rug_fuzz_0;
        let _result = duration.mul(multiplier);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_59 {
    use super::*;
    use crate::*;
    use std::ops::Mul;
    #[test]
    fn test_mul_positive() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let result = duration.mul(rug_fuzz_1);
        debug_assert_eq!(result, Duration::seconds(10));
             }
}
}
}    }
    #[test]
    fn test_mul_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let result = duration.mul(-rug_fuzz_1);
        debug_assert_eq!(result, Duration::seconds(- 10));
             }
}
}
}    }
    #[test]
    fn test_mul_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let result = duration.mul(rug_fuzz_1);
        debug_assert_eq!(result, Duration::seconds(0));
             }
}
}
}    }
    #[test]
    #[should_panic(expected = "overflow when multiplying duration")]
    fn test_mul_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::new(i64::MAX, rug_fuzz_0);
        let _result = duration.mul(i8::MAX);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_60 {
    use super::*;
    use crate::*;
    use std::ops::Mul;
    #[test]
    fn mul_with_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let result = duration.mul(rug_fuzz_1);
        debug_assert_eq!(result, Duration::seconds(0));
             }
}
}
}    }
    #[test]
    fn mul_with_pos() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let result = duration.mul(rug_fuzz_1);
        debug_assert_eq!(result, Duration::seconds(10));
             }
}
}
}    }
    #[test]
    #[should_panic(expected = "overflow when multiplying duration")]
    fn mul_with_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(i64::MAX);
        let _result = duration.mul(rug_fuzz_0);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_61 {
    use super::*;
    use crate::*;
    use std::ops::Mul;
    #[test]
    fn mul_with_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let multiplier: u32 = rug_fuzz_1;
        debug_assert_eq!(duration.mul(multiplier), Duration::seconds(0));
             }
}
}
}    }
    #[test]
    fn mul_with_positive() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let multiplier: u32 = rug_fuzz_1;
        debug_assert_eq!(duration.mul(multiplier), Duration::seconds(10));
             }
}
}
}    }
    #[test]
    fn mul_with_large_number() {
        let _rug_st_tests_llm_16_61_rrrruuuugggg_mul_with_large_number = 0;
        let duration = Duration::MAX;
        let multiplier: u32 = u32::MAX;
        let result = duration.mul(multiplier);
        debug_assert!(result.is_positive() && result > duration);
        let _rug_ed_tests_llm_16_61_rrrruuuugggg_mul_with_large_number = 0;
    }
    #[test]
    #[should_panic(expected = "overflow when multiplying duration")]
    fn mul_with_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(i64::MAX);
        let multiplier: u32 = rug_fuzz_0;
        let _result = duration.mul(multiplier);
             }
}
}
}    }
    #[test]
    fn mul_with_one() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let multiplier: u32 = rug_fuzz_1;
        debug_assert_eq!(duration.mul(multiplier), Duration::seconds(5));
             }
}
}
}    }
    #[test]
    fn mul_with_max_value() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let multiplier: u32 = u32::MAX;
        let result = duration.mul(multiplier);
        debug_assert!(result.is_positive() && result > duration);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_62 {
    use super::*;
    use crate::*;
    use std::ops::Mul;
    use crate::Duration;
    #[test]
    fn test_mul_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let result = duration.mul(rug_fuzz_1);
        debug_assert_eq!(result, Duration::seconds(0));
             }
}
}
}    }
    #[test]
    fn test_mul_one() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let result = duration.mul(rug_fuzz_1);
        debug_assert_eq!(result, Duration::seconds(5));
             }
}
}
}    }
    #[test]
    fn test_mul_two() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let result = duration.mul(rug_fuzz_1);
        debug_assert_eq!(result, Duration::seconds(10));
             }
}
}
}    }
    #[test]
    fn test_mul_with_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(i64::MAX);
        let result = std::panic::catch_unwind(|| duration.mul(rug_fuzz_0));
        debug_assert!(result.is_err());
             }
}
}
}    }
    #[test]
    fn test_mul_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let result = duration.mul(-rug_fuzz_1);
        debug_assert_eq!(result, Duration::seconds(- 5));
             }
}
}
}    }
    #[test]
    fn test_multiply_fractions() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::milliseconds(rug_fuzz_0);
        let result = duration.mul(rug_fuzz_1);
        debug_assert_eq!(result, Duration::seconds(1));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_71 {
    use super::*;
    use crate::*;
    #[test]
    fn neg_zero() {
        let _rug_st_tests_llm_16_71_rrrruuuugggg_neg_zero = 0;
        debug_assert_eq!(- Duration::ZERO, Duration::ZERO);
        let _rug_ed_tests_llm_16_71_rrrruuuugggg_neg_zero = 0;
    }
    #[test]
    fn neg_positive() {
        let _rug_st_tests_llm_16_71_rrrruuuugggg_neg_positive = 0;
        debug_assert_eq!(- Duration::SECOND, Duration::new(- 1, 0));
        let _rug_ed_tests_llm_16_71_rrrruuuugggg_neg_positive = 0;
    }
    #[test]
    fn neg_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(- Duration::new(- rug_fuzz_0, rug_fuzz_1), Duration::SECOND);
             }
}
}
}    }
    #[test]
    fn neg_edge_cases() {
        let _rug_st_tests_llm_16_71_rrrruuuugggg_neg_edge_cases = 0;
        debug_assert_eq!(- Duration::MIN, Duration::MIN);
        debug_assert_eq!(- Duration::MAX, Duration::MAX);
        let _rug_ed_tests_llm_16_71_rrrruuuugggg_neg_edge_cases = 0;
    }
    #[test]
    fn neg_nanosecond() {
        let _rug_st_tests_llm_16_71_rrrruuuugggg_neg_nanosecond = 0;
        debug_assert_eq!(- Duration::NANOSECOND, Duration::new(0, - 1));
        let _rug_ed_tests_llm_16_71_rrrruuuugggg_neg_nanosecond = 0;
    }
    #[test]
    fn neg_microsecond() {
        let _rug_st_tests_llm_16_71_rrrruuuugggg_neg_microsecond = 0;
        debug_assert_eq!(- Duration::MICROSECOND, Duration::new(0, - 1_000));
        let _rug_ed_tests_llm_16_71_rrrruuuugggg_neg_microsecond = 0;
    }
    #[test]
    fn neg_millisecond() {
        let _rug_st_tests_llm_16_71_rrrruuuugggg_neg_millisecond = 0;
        debug_assert_eq!(- Duration::MILLISECOND, Duration::new(0, - 1_000_000));
        let _rug_ed_tests_llm_16_71_rrrruuuugggg_neg_millisecond = 0;
    }
    #[test]
    fn neg_mixed() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            - Duration::new(rug_fuzz_0, rug_fuzz_1), Duration::new(- 1, - 500_000_000)
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_73 {
    use super::*;
    use crate::*;
    use std::ops::Sub;
    #[test]
    fn test_sub_positive_durations() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i64, i32, i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration_a = Duration::new(rug_fuzz_0, rug_fuzz_1);
        let duration_b = Duration::new(rug_fuzz_2, rug_fuzz_3);
        let duration_c = duration_a - duration_b;
        debug_assert_eq!(duration_c, Duration::new(2, 0));
             }
}
}
}    }
    #[test]
    fn test_sub_negative_durations() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i64, i32, i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration_a = Duration::new(-rug_fuzz_0, rug_fuzz_1);
        let duration_b = Duration::new(-rug_fuzz_2, rug_fuzz_3);
        let duration_c = duration_a - duration_b;
        debug_assert_eq!(duration_c, Duration::new(- 2, 0));
             }
}
}
}    }
    #[test]
    fn test_sub_mixed_sign_durations() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i64, i32, i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration_a = Duration::new(rug_fuzz_0, rug_fuzz_1);
        let duration_b = Duration::new(-rug_fuzz_2, rug_fuzz_3);
        let duration_c = duration_a - duration_b;
        debug_assert_eq!(duration_c, Duration::new(8, 0));
        let duration_d = duration_b - duration_a;
        debug_assert_eq!(duration_d, Duration::new(- 8, 0));
             }
}
}
}    }
    #[test]
    fn test_sub_with_nanoseconds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i64, i32, i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration_a = Duration::new(rug_fuzz_0, rug_fuzz_1);
        let duration_b = Duration::new(rug_fuzz_2, rug_fuzz_3);
        let duration_c = duration_a - duration_b;
        debug_assert_eq!(duration_c, Duration::new(0, 200_000_000));
             }
}
}
}    }
    #[test]
    #[should_panic]
    fn test_sub_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let _ = Duration::MIN - Duration::new(rug_fuzz_0, rug_fuzz_1);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_274 {
    use crate::Duration;
    use std::ops::Mul;
    #[test]
    fn mul_by_positive_float() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let multiplier = rug_fuzz_1;
        let result = multiplier.mul(duration);
        debug_assert_eq!(result, Duration::seconds(12) + Duration::milliseconds(500));
             }
}
}
}    }
    #[test]
    fn mul_by_negative_float() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let multiplier = -rug_fuzz_1;
        let result = multiplier.mul(duration);
        debug_assert_eq!(result, Duration::seconds(- 10));
             }
}
}
}    }
    #[test]
    fn mul_by_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let multiplier = rug_fuzz_1;
        let result = multiplier.mul(duration);
        debug_assert_eq!(result, Duration::seconds(0));
             }
}
}
}    }
    #[test]
    fn mul_by_one() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let multiplier = rug_fuzz_1;
        let result = multiplier.mul(duration);
        debug_assert_eq!(result, duration);
             }
}
}
}    }
    #[test]
    fn mul_with_fraction_result() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::milliseconds(rug_fuzz_0);
        let multiplier = rug_fuzz_1;
        let result = multiplier.mul(duration);
        debug_assert_eq!(result, Duration::milliseconds(500));
             }
}
}
}    }
    #[test]
    fn mul_with_large_multiplier() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let multiplier = rug_fuzz_1;
        let result = multiplier.mul(duration);
        debug_assert_eq!(result, Duration::seconds(1_000_000_000));
             }
}
}
}    }
    #[test]
    fn mul_with_large_duration() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(i64::MAX);
        let multiplier = rug_fuzz_0;
        let result = multiplier.mul(duration);
        debug_assert_eq!(result, Duration::seconds(1));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_275 {
    use super::*;
    use crate::*;
    #[test]
    fn mul_duration_by_i16() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i16, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let dur = Duration::seconds(rug_fuzz_0);
        let multiplier: i16 = rug_fuzz_1;
        let expected = Duration::seconds(rug_fuzz_2);
        debug_assert_eq!(multiplier.mul(dur), expected);
             }
}
}
}    }
    #[test]
    fn mul_duration_by_i16_with_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i64, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let dur = Duration::seconds(i64::MAX / rug_fuzz_0 + rug_fuzz_1);
        let multiplier: i16 = rug_fuzz_2;
        let expected = Duration::MAX;
        debug_assert_eq!(multiplier.mul(dur), expected);
             }
}
}
}    }
    #[test]
    fn mul_duration_by_i16_with_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i16, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let dur = Duration::seconds(rug_fuzz_0);
        let multiplier: i16 = -rug_fuzz_1;
        let expected = Duration::seconds(-rug_fuzz_2);
        debug_assert_eq!(multiplier.mul(dur), expected);
             }
}
}
}    }
    #[test]
    fn mul_duration_by_i16_with_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i16, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let dur = Duration::seconds(rug_fuzz_0);
        let multiplier: i16 = rug_fuzz_1;
        let expected = Duration::seconds(rug_fuzz_2);
        debug_assert_eq!(multiplier.mul(dur), expected);
             }
}
}
}    }
    #[test]
    fn mul_duration_by_i16_with_negative_duration() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i16, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let dur = Duration::seconds(-rug_fuzz_0);
        let multiplier: i16 = rug_fuzz_1;
        let expected = Duration::seconds(-rug_fuzz_2);
        debug_assert_eq!(multiplier.mul(dur), expected);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_276 {
    use super::*;
    use crate::*;
    use std::ops::Mul;
    #[test]
    fn test_mul_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(rug_fuzz_0.mul(Duration::ZERO), Duration::ZERO);
             }
}
}
}    }
    #[test]
    fn test_mul_positive() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(rug_fuzz_0.mul(Duration::SECOND), Duration::seconds(2));
             }
}
}
}    }
    #[test]
    fn test_mul_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!((- rug_fuzz_0).mul(Duration::SECOND), Duration::seconds(- 2));
             }
}
}
}    }
    #[test]
    fn test_mul_max() {
        let _rug_st_tests_llm_16_276_rrrruuuugggg_test_mul_max = 0;
        debug_assert_eq!(
            i32::MAX.mul(Duration::SECOND), Duration::seconds(i32::MAX as i64)
        );
        let _rug_ed_tests_llm_16_276_rrrruuuugggg_test_mul_max = 0;
    }
    #[test]
    fn test_mul_min() {
        let _rug_st_tests_llm_16_276_rrrruuuugggg_test_mul_min = 0;
        debug_assert_eq!(
            i32::MIN.mul(Duration::SECOND), Duration::seconds(i32::MIN as i64)
        );
        let _rug_ed_tests_llm_16_276_rrrruuuugggg_test_mul_min = 0;
    }
    #[test]
    fn test_mul_overflow() {
        let _rug_st_tests_llm_16_276_rrrruuuugggg_test_mul_overflow = 0;
        debug_assert!((i32::MAX).mul(Duration::MAX).is_positive());
        debug_assert!((i32::MIN).mul(Duration::MIN).is_positive());
        let _rug_ed_tests_llm_16_276_rrrruuuugggg_test_mul_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_277 {
    use crate::Duration;
    use std::ops::Mul;
    #[test]
    fn mul_duration_by_i8() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8)) = <(i64, i8, i64, i64, i8, i64, i64, i8, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let multiplier: i8 = rug_fuzz_1;
        let expected = Duration::seconds(rug_fuzz_2);
        debug_assert_eq!(multiplier.mul(duration), expected);
        let duration = Duration::seconds(rug_fuzz_3);
        let multiplier: i8 = rug_fuzz_4;
        let expected = Duration::seconds(rug_fuzz_5);
        debug_assert_eq!(multiplier.mul(duration), expected);
        let duration = Duration::seconds(rug_fuzz_6);
        let multiplier: i8 = -rug_fuzz_7;
        let expected = Duration::seconds(-rug_fuzz_8);
        debug_assert_eq!(multiplier.mul(duration), expected);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_278 {
    use crate::Duration;
    use std::ops::Mul;
    #[test]
    fn mul_duration_by_u16() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, u16, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let multiplier: u16 = rug_fuzz_1;
        let result = multiplier.mul(duration);
        debug_assert_eq!(Duration::seconds(rug_fuzz_2), result);
             }
}
}
}    }
    #[test]
    fn mul_duration_by_u16_with_nanoseconds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i64, i32, u16, i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::new(rug_fuzz_0, rug_fuzz_1);
        let multiplier: u16 = rug_fuzz_2;
        let result = multiplier.mul(duration);
        debug_assert_eq!(Duration::new(rug_fuzz_3, rug_fuzz_4), result);
             }
}
}
}    }
    #[test]
    fn mul_duration_by_u16_with_negative_duration() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i64, i32, u16, i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::new(-rug_fuzz_0, rug_fuzz_1);
        let multiplier: u16 = rug_fuzz_2;
        let result = multiplier.mul(duration);
        debug_assert_eq!(Duration::new(- rug_fuzz_3, rug_fuzz_4), result);
             }
}
}
}    }
    #[test]
    fn mul_duration_by_u16_with_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u16, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::new(i64::MAX, rug_fuzz_0);
        let multiplier: u16 = rug_fuzz_1;
        let result = multiplier.mul(duration);
        debug_assert!(result.is_negative());
        debug_assert_eq!(Duration::new(i64::MAX, rug_fuzz_2), result);
             }
}
}
}    }
    #[test]
    fn mul_duration_by_u16_with_underflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u16, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::new(i64::MIN, rug_fuzz_0);
        let multiplier: u16 = rug_fuzz_1;
        let result = multiplier.mul(duration);
        debug_assert!(result.is_negative());
        debug_assert_eq!(Duration::new(i64::MIN, rug_fuzz_2), result);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_279 {
    use crate::Duration;
    use std::ops::Mul;
    #[test]
    fn test_u32_mul_duration() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, u32, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::seconds(rug_fuzz_0);
        let scalar = rug_fuzz_1;
        let expected = Duration::seconds(rug_fuzz_2);
        debug_assert_eq!(scalar.mul(duration), expected);
             }
}
}
}    }
    #[test]
    fn test_u32_mul_duration_with_nanoseconds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i64, i32, u32, i64, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::new(rug_fuzz_0, rug_fuzz_1);
        let scalar = rug_fuzz_2;
        let expected = Duration::new(rug_fuzz_3, rug_fuzz_4 * rug_fuzz_5);
        debug_assert_eq!(scalar.mul(duration), expected);
             }
}
}
}    }
    #[test]
    fn test_u32_mul_duration_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::new(i64::MAX, rug_fuzz_0);
        let scalar = rug_fuzz_1;
        debug_assert!(scalar.mul(duration).is_positive());
             }
}
}
}    }
    #[test]
    fn test_u32_mul_duration_underflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::new(i64::MIN, -rug_fuzz_0);
        let scalar = rug_fuzz_1;
        debug_assert!(scalar.mul(duration).is_negative());
             }
}
}
}    }
    #[test]
    fn test_u32_mul_zero_duration() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::ZERO;
        let scalar = rug_fuzz_0;
        let expected = Duration::ZERO;
        debug_assert_eq!(scalar.mul(duration), expected);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_280 {
    use super::*;
    use crate::*;
    #[test]
    fn mul_duration_by_u8() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(u8, i64, u8, i64, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            (rug_fuzz_0).mul(Duration::seconds(rug_fuzz_1)), Duration::seconds(5)
        );
        debug_assert_eq!(
            (rug_fuzz_2).mul(Duration::seconds(rug_fuzz_3)), Duration::ZERO
        );
        debug_assert_eq!((rug_fuzz_4).mul(Duration::MIN), Duration::MIN);
        debug_assert_eq!((rug_fuzz_5).mul(Duration::MAX), Duration::MAX);
        debug_assert_eq!(
            (rug_fuzz_6).mul(Duration::MIN), Duration::MIN.saturating_mul(2)
        );
        debug_assert_eq!(
            (u8::MAX).mul(Duration::NANOSECOND), Duration::nanoseconds(u8::MAX as i64)
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_283 {
    use super::*;
    use crate::*;
    use crate::ext::NumericalDuration;
    #[test]
    fn test_abs_positive() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(rug_fuzz_0.seconds().abs(), 1.seconds());
             }
}
}
}    }
    #[test]
    fn test_abs_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(rug_fuzz_0.seconds().abs(), 0.seconds());
             }
}
}
}    }
    #[test]
    fn test_abs_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!((- rug_fuzz_0).seconds().abs(), 1.seconds());
             }
}
}
}    }
    #[test]
    fn test_abs_edge_case() {
        let _rug_st_tests_llm_16_283_rrrruuuugggg_test_abs_edge_case = 0;
        debug_assert_eq!(Duration::MIN.abs(), Duration::MAX);
        let _rug_ed_tests_llm_16_283_rrrruuuugggg_test_abs_edge_case = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_284 {
    use super::*;
    use crate::*;
    #[test]
    fn test_as_seconds_f32() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13)) = <(i64, i32, i64, i32, i64, i32, i64, i32, i64, i32, i64, i32, i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Duration::ZERO.as_seconds_f32(), 0.0);
        debug_assert_eq!(Duration::new(rug_fuzz_0, rug_fuzz_1).as_seconds_f32(), 0.5);
        debug_assert_eq!(Duration::new(rug_fuzz_2, rug_fuzz_3).as_seconds_f32(), 1.0);
        debug_assert_eq!(Duration::new(rug_fuzz_4, rug_fuzz_5).as_seconds_f32(), 1.25);
        debug_assert_eq!(
            Duration::new(- rug_fuzz_6, rug_fuzz_7).as_seconds_f32(), - 1.0
        );
        debug_assert_eq!(
            Duration::new(- rug_fuzz_8, - rug_fuzz_9).as_seconds_f32(), - 1.25
        );
        debug_assert_eq!(Duration::new(rug_fuzz_10, rug_fuzz_11).as_seconds_f32(), 1.0);
        debug_assert_eq!(
            Duration::new(rug_fuzz_12, - rug_fuzz_13).as_seconds_f32(), - 1.0
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_285 {
    use super::*;
    use crate::*;
    #[test]
    fn test_as_seconds_f64() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i64, i32, i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration_positive = Duration::new(rug_fuzz_0, rug_fuzz_1);
        let duration_negative = Duration::new(-rug_fuzz_2, -rug_fuzz_3);
        let duration_zero = Duration::ZERO;
        debug_assert_eq!(duration_positive.as_seconds_f64(), 5.5);
        debug_assert_eq!(duration_negative.as_seconds_f64(), - 5.5);
        debug_assert_eq!(duration_zero.as_seconds_f64(), 0.0);
             }
}
}
}    }
    #[test]
    fn test_as_seconds_f64_with_extreme_values() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration_max = Duration::new(i64::MAX, rug_fuzz_0);
        let duration_min = Duration::new(i64::MIN, -rug_fuzz_1);
        debug_assert_eq!(
            duration_max.as_seconds_f64(), i64::MAX as f64 + 999_999_999 as f64 /
            1_000_000_000 as f64
        );
        debug_assert_eq!(
            duration_min.as_seconds_f64(), i64::MIN as f64 - 999_999_999 as f64 /
            1_000_000_000 as f64
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_286 {
    use super::*;
    use crate::*;
    use crate::{Duration, ext::NumericalDuration};
    #[test]
    fn checked_add_with_no_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17)) = <(i64, i32, i64, i32, i64, i32, i64, i32, i64, i32, i64, i32, i32, i64, i32, i32, i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Duration::new(rug_fuzz_0, rug_fuzz_1).checked_add(Duration::new(rug_fuzz_2,
            rug_fuzz_3)), Some(Duration::new(3, 750_000_000))
        );
        debug_assert_eq!(
            Duration::new(- rug_fuzz_4, - rug_fuzz_5)
            .checked_add(Duration::new(rug_fuzz_6, rug_fuzz_7)), Some(Duration::new(0, -
            250_000_000))
        );
        debug_assert_eq!(
            Duration::new(rug_fuzz_8, rug_fuzz_9).checked_add(Duration::new(rug_fuzz_10,
            rug_fuzz_11)), Some(Duration::new(2, 0))
        );
        debug_assert_eq!(
            Duration::new(i64::MAX, rug_fuzz_12).checked_add(Duration::new(rug_fuzz_13,
            rug_fuzz_14)), Some(Duration::new(i64::MAX, 999_999_999))
        );
        debug_assert_eq!(
            Duration::new(i64::MIN, - rug_fuzz_15).checked_add(Duration::new(rug_fuzz_16,
            rug_fuzz_17)), Some(Duration::new(i64::MIN, - 999_999_999))
        );
             }
}
}
}    }
    #[test]
    fn checked_add_with_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, i64, i32, i32, i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Duration::new(i64::MAX, rug_fuzz_0).checked_add(Duration::new(rug_fuzz_1,
            rug_fuzz_2)), None
        );
        debug_assert_eq!(
            Duration::new(i64::MIN, - rug_fuzz_3).checked_add(Duration::new(rug_fuzz_4, -
            rug_fuzz_5)), None
        );
             }
}
}
}    }
    #[test]
    fn checked_add_with_underflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, i64, i32, i32, i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Duration::new(i64::MIN, rug_fuzz_0).checked_add(Duration::new(- rug_fuzz_1,
            rug_fuzz_2)), None
        );
        debug_assert_eq!(
            Duration::new(i64::MAX, rug_fuzz_3).checked_add(Duration::new(rug_fuzz_4,
            rug_fuzz_5)), None
        );
             }
}
}
}    }
    #[test]
    fn checked_add_with_edges() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, i64, i32, i32, i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Duration::new(i64::MAX, rug_fuzz_0).checked_add(Duration::new(rug_fuzz_1,
            rug_fuzz_2)), Some(Duration::new(i64::MAX, 999_999_999))
        );
        debug_assert_eq!(
            Duration::new(i64::MIN, rug_fuzz_3).checked_add(Duration::new(rug_fuzz_4, -
            rug_fuzz_5)), Some(Duration::new(i64::MIN, - 999_999_999))
        );
             }
}
}
}    }
    #[test]
    fn checked_add_with_cross_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(i64, i32, i64, i32, i64, i32, i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Duration::new(- rug_fuzz_0, rug_fuzz_1).checked_add(Duration::new(rug_fuzz_2,
            rug_fuzz_3)), Some(Duration::new(0, 0))
        );
        debug_assert_eq!(
            Duration::new(rug_fuzz_4, - rug_fuzz_5).checked_add(Duration::new(rug_fuzz_6,
            - rug_fuzz_7)), Some(Duration::new(0, 0))
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_287 {
    use super::*;
    use crate::*;
    #[test]
    fn checked_div_positive() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Duration::new(rug_fuzz_0, rug_fuzz_1).checked_div(rug_fuzz_2),
            Some(Duration::new(5, 0))
        );
             }
}
}
}    }
    #[test]
    fn checked_div_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Duration::new(- rug_fuzz_0, rug_fuzz_1).checked_div(rug_fuzz_2),
            Some(Duration::new(- 5, 0))
        );
             }
}
}
}    }
    #[test]
    fn checked_div_zero_divisor() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Duration::new(rug_fuzz_0, rug_fuzz_1).checked_div(rug_fuzz_2), None
        );
             }
}
}
}    }
    #[test]
    fn checked_div_fractional() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Duration::new(rug_fuzz_0, rug_fuzz_1).checked_div(rug_fuzz_2),
            Some(Duration::new(5, 250_000_000))
        );
             }
}
}
}    }
    #[test]
    fn checked_div_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10)) = <(i32, i32, i32, i32, i32, i32, i64, i32, i32, i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Duration::new(i64::MAX, rug_fuzz_0).checked_div(rug_fuzz_1),
            Some(Duration::new(i64::MAX, 0))
        );
        debug_assert_eq!(
            Duration::new(i64::MAX, rug_fuzz_2).checked_div(rug_fuzz_3),
            Some(Duration::new(i64::MAX, 999_999_999))
        );
        debug_assert_eq!(
            Duration::new(i64::MIN, rug_fuzz_4).checked_div(- rug_fuzz_5), None
        );
        debug_assert_eq!(
            Duration::new(i64::MIN + rug_fuzz_6, rug_fuzz_7).checked_div(- rug_fuzz_8),
            Some(Duration::new(i64::MAX, 0))
        );
        debug_assert_eq!(
            Duration::new(rug_fuzz_9, rug_fuzz_10).checked_div(i32::MIN), None
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_288 {
    use super::*;
    use crate::*;
    use crate::ext::NumericalDuration;
    #[test]
    fn checked_mul_with_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Duration::MAX.checked_mul(rug_fuzz_0), None);
        debug_assert_eq!(Duration::MIN.checked_mul(rug_fuzz_1), None);
             }
}
}
}    }
    #[test]
    fn checked_mul_without_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(i64, i32, i64, i32, i64, i32, i64, i32, i32, i64, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            rug_fuzz_0.seconds().checked_mul(rug_fuzz_1), Some(10.seconds())
        );
        debug_assert_eq!(
            rug_fuzz_2.seconds().checked_mul(- rug_fuzz_3), Some((- 10).seconds())
        );
        debug_assert_eq!(
            rug_fuzz_4.seconds().checked_mul(rug_fuzz_5), Some(0.seconds())
        );
        debug_assert_eq!(
            Duration::new(rug_fuzz_6, rug_fuzz_7).checked_mul(rug_fuzz_8),
            Some(Duration::new(2, 0))
        );
        debug_assert_eq!(
            Duration::new(- rug_fuzz_9, rug_fuzz_10).checked_mul(rug_fuzz_11),
            Some(Duration::new(- 2, 0))
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_289 {
    use crate::Duration;
    use crate::ext::NumericalDuration;
    #[test]
    fn test_checked_seconds_f32() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f32, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Duration::checked_seconds_f32(rug_fuzz_0), Some(Duration::seconds(0) +
            Duration::milliseconds(500))
        );
        debug_assert_eq!(
            Duration::checked_seconds_f32(- rug_fuzz_1), Some(Duration::seconds(0) -
            Duration::milliseconds(500))
        );
        debug_assert_eq!(Duration::checked_seconds_f32(f32::NAN), None);
        debug_assert_eq!(Duration::checked_seconds_f32(f32::NEG_INFINITY), None);
        debug_assert_eq!(Duration::checked_seconds_f32(f32::INFINITY), None);
        debug_assert_eq!(
            Duration::checked_seconds_f32(f32::MAX), Some(Duration::seconds(f32::MAX as
            i64))
        );
        debug_assert_eq!(
            Duration::checked_seconds_f32(f32::MIN), Some(Duration::seconds(f32::MIN as
            i64))
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_290 {
    use crate::Duration;
    use crate::ext::NumericalDuration;
    #[test]
    fn checked_seconds_f64_valid() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(f64, f64, f64, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Duration::checked_seconds_f64(rug_fuzz_0), Some(0.5.seconds()));
        debug_assert_eq!(
            Duration::checked_seconds_f64(- rug_fuzz_1), Some(- 0.5.seconds())
        );
        debug_assert_eq!(
            Duration::checked_seconds_f64(rug_fuzz_2), Some(1.0e7.seconds())
        );
        debug_assert_eq!(
            Duration::checked_seconds_f64(- rug_fuzz_3), Some(- 1.0e7.seconds())
        );
             }
}
}
}    }
    #[test]
    fn checked_seconds_f64_invalid() {
        let _rug_st_tests_llm_16_290_rrrruuuugggg_checked_seconds_f64_invalid = 0;
        debug_assert_eq!(Duration::checked_seconds_f64(f64::NAN), None);
        debug_assert_eq!(Duration::checked_seconds_f64(f64::NEG_INFINITY), None);
        debug_assert_eq!(Duration::checked_seconds_f64(f64::INFINITY), None);
        debug_assert_eq!(Duration::checked_seconds_f64(f64::MAX), None);
        debug_assert_eq!(Duration::checked_seconds_f64(f64::MIN), None);
        let _rug_ed_tests_llm_16_290_rrrruuuugggg_checked_seconds_f64_invalid = 0;
    }
    #[test]
    fn checked_seconds_f64_edge_cases() {
        let _rug_st_tests_llm_16_290_rrrruuuugggg_checked_seconds_f64_edge_cases = 0;
        debug_assert_eq!(
            Duration::checked_seconds_f64(f64::EPSILON), Some(f64::EPSILON.seconds())
        );
        debug_assert_eq!(
            Duration::checked_seconds_f64(- f64::EPSILON), Some((- f64::EPSILON)
            .seconds())
        );
        let _rug_ed_tests_llm_16_290_rrrruuuugggg_checked_seconds_f64_edge_cases = 0;
    }
}
#[cfg(test)]
mod checked_sub_tests {
    use super::*;
    use crate::*;
    use crate::{Duration, ext::NumericalDuration};
    #[test]
    fn zero_duration() {
        assert_eq!(Duration::ZERO.checked_sub(Duration::ZERO), Some(Duration::ZERO));
    }
    #[test]
    fn positive_duration() {
        assert_eq!(5.seconds().checked_sub(5.seconds()), Some(Duration::ZERO));
        assert_eq!(5.seconds().checked_sub(1.seconds()), Some(4.seconds()));
        assert_eq!(1.seconds().checked_sub(5.seconds()), Some((- 4).seconds()));
    }
    #[test]
    fn negative_duration() {
        assert_eq!((- 5).seconds().checked_sub((- 5).seconds()), Some(Duration::ZERO));
        assert_eq!((- 5).seconds().checked_sub((- 1).seconds()), Some((- 4).seconds()));
        assert_eq!((- 1).seconds().checked_sub((- 5).seconds()), Some(4.seconds()));
    }
    #[test]
    fn mixed_duration() {
        assert_eq!(5.seconds().checked_sub((- 5).seconds()), Some(10.seconds()));
        assert_eq!((- 5).seconds().checked_sub(5.seconds()), Some((- 10).seconds()));
    }
    #[test]
    fn overflow_duration() {
        assert_eq!(Duration::MIN.checked_sub(1.nanoseconds()), None);
        assert_eq!(Duration::MIN.checked_sub((- 1).seconds()), None);
        assert_eq!(Duration::MAX.checked_sub((- 1).nanoseconds()), None);
        assert_eq!(Duration::MAX.checked_sub(1.seconds()), None);
    }
}
#[cfg(test)]
mod tests_llm_16_292 {
    use super::*;
    use crate::*;
    use duration::Duration;
    #[test]
    fn days_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Duration::days(rug_fuzz_0), Duration::ZERO);
             }
}
}
}    }
    #[test]
    fn days_single() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Duration::days(rug_fuzz_0), Duration::DAY);
             }
}
}
}    }
    #[test]
    fn days_multiple() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Duration::days(rug_fuzz_0), Duration::days(1) * 10);
             }
}
}
}    }
    #[test]
    fn days_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Duration::days(- rug_fuzz_0), - Duration::DAY);
             }
}
}
}    }
    #[test]
    fn days_arbitrary() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Duration::days(rug_fuzz_0), Duration::seconds(365 * 86_400));
             }
}
}
}    }
    #[test]
    fn days_min() {
        let _rug_st_tests_llm_16_292_rrrruuuugggg_days_min = 0;
        debug_assert_eq!(Duration::days(i64::MIN), Duration::MIN);
        let _rug_ed_tests_llm_16_292_rrrruuuugggg_days_min = 0;
    }
    #[test]
    fn days_max() {
        let _rug_st_tests_llm_16_292_rrrruuuugggg_days_max = 0;
        debug_assert_eq!(Duration::days(i64::MAX), Duration::MAX);
        let _rug_ed_tests_llm_16_292_rrrruuuugggg_days_max = 0;
    }
    #[test]
    fn days_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(Duration::days(i64::MAX / rug_fuzz_0 + rug_fuzz_1).is_negative());
             }
}
}
}    }
    #[test]
    #[should_panic(expected = "overflow constructing `crate::Duration`")]
    fn days_overflow_panic() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let _ = Duration::days(i64::MAX / rug_fuzz_0 + rug_fuzz_1);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_293 {
    use super::*;
    use crate::*;
    #[test]
    fn hours_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Duration::hours(rug_fuzz_0), Duration::seconds(0));
             }
}
}
}    }
    #[test]
    fn hours_pos() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Duration::hours(rug_fuzz_0), Duration::seconds(3600));
        debug_assert_eq!(Duration::hours(rug_fuzz_1), Duration::seconds(7200));
        debug_assert_eq!(Duration::hours(rug_fuzz_2), Duration::seconds(86400));
             }
}
}
}    }
    #[test]
    fn hours_neg() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Duration::hours(- rug_fuzz_0), Duration::seconds(- 3600));
        debug_assert_eq!(Duration::hours(- rug_fuzz_1), Duration::seconds(- 7200));
        debug_assert_eq!(Duration::hours(- rug_fuzz_2), Duration::seconds(- 86400));
             }
}
}
}    }
    #[test]
    #[should_panic(expected = "overflow constructing `crate::Duration`")]
    fn hours_overflow() {
        let _rug_st_tests_llm_16_293_rrrruuuugggg_hours_overflow = 0;
        let _ = Duration::hours(i64::MAX);
        let _rug_ed_tests_llm_16_293_rrrruuuugggg_hours_overflow = 0;
    }
    #[test]
    #[should_panic(expected = "overflow constructing `crate::Duration`")]
    fn hours_underflow() {
        let _rug_st_tests_llm_16_293_rrrruuuugggg_hours_underflow = 0;
        let _ = Duration::hours(i64::MIN);
        let _rug_ed_tests_llm_16_293_rrrruuuugggg_hours_underflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_294 {
    use crate::Duration;
    #[test]
    fn test_is_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17)) = <(i64, i32, i64, i32, i64, i32, i64, i32, i64, i32, i64, i32, i64, i32, i64, i32, i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(Duration::new(- rug_fuzz_0, rug_fuzz_1).is_negative());
        debug_assert!(Duration::new(- rug_fuzz_2, - rug_fuzz_3).is_negative());
        debug_assert!(Duration::new(rug_fuzz_4, - rug_fuzz_5).is_negative());
        debug_assert!(Duration::new(- rug_fuzz_6, rug_fuzz_7).is_negative());
        debug_assert!(! Duration::new(rug_fuzz_8, rug_fuzz_9).is_negative());
        debug_assert!(! Duration::new(rug_fuzz_10, rug_fuzz_11).is_negative());
        debug_assert!(! Duration::new(rug_fuzz_12, - rug_fuzz_13).is_negative());
        debug_assert!(! Duration::new(rug_fuzz_14, rug_fuzz_15).is_negative());
        debug_assert!(! Duration::new(rug_fuzz_16, rug_fuzz_17).is_negative());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_295 {
    use crate::Duration;
    #[test]
    fn test_is_positive() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17, mut rug_fuzz_18, mut rug_fuzz_19, mut rug_fuzz_20)) = <(i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i32, i64, i32, i64, i32, i64, i32, i64, i32, i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(Duration::seconds(rug_fuzz_0).is_positive());
        debug_assert!(Duration::milliseconds(rug_fuzz_1).is_positive());
        debug_assert!(Duration::nanoseconds(rug_fuzz_2).is_positive());
        debug_assert!(! Duration::seconds(rug_fuzz_3).is_positive());
        debug_assert!(! Duration::milliseconds(rug_fuzz_4).is_positive());
        debug_assert!(! Duration::nanoseconds(rug_fuzz_5).is_positive());
        debug_assert!(! Duration::seconds(- rug_fuzz_6).is_positive());
        debug_assert!(! Duration::milliseconds(- rug_fuzz_7).is_positive());
        debug_assert!(! Duration::nanoseconds(- rug_fuzz_8).is_positive());
        debug_assert!(Duration::new(rug_fuzz_9, rug_fuzz_10).is_positive());
        debug_assert!(Duration::new(rug_fuzz_11, rug_fuzz_12).is_positive());
        debug_assert!(Duration::new(- rug_fuzz_13, rug_fuzz_14).is_positive());
        debug_assert!(! Duration::new(rug_fuzz_15, - rug_fuzz_16).is_positive());
        debug_assert!(! Duration::new(rug_fuzz_17, - rug_fuzz_18).is_positive());
        debug_assert!(! Duration::new(- rug_fuzz_19, - rug_fuzz_20).is_positive());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_296_llm_16_296 {
    use super::*;
    use crate::*;
    #[test]
    fn is_zero_with_zero_duration() {
        let _rug_st_tests_llm_16_296_llm_16_296_rrrruuuugggg_is_zero_with_zero_duration = 0;
        debug_assert!(Duration::ZERO.is_zero());
        let _rug_ed_tests_llm_16_296_llm_16_296_rrrruuuugggg_is_zero_with_zero_duration = 0;
    }
    #[test]
    fn is_zero_with_nonzero_duration() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(! Duration::nanoseconds(rug_fuzz_0).is_zero());
        debug_assert!(! Duration::seconds(rug_fuzz_1).is_zero());
             }
}
}
}    }
    #[test]
    fn is_zero_with_negative_duration() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(! Duration::seconds(- rug_fuzz_0).is_zero());
             }
}
}
}    }
    #[test]
    fn is_zero_with_complex_duration() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(Duration::new(rug_fuzz_0, rug_fuzz_1).is_zero());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_297 {
    use super::*;
    use crate::*;
    #[test]
    fn microseconds_new() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i64, i64, i64, i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Duration::microseconds(rug_fuzz_0), Duration::nanoseconds(1000)
        );
        debug_assert_eq!(Duration::microseconds(rug_fuzz_1), Duration::milliseconds(1));
        debug_assert_eq!(Duration::microseconds(rug_fuzz_2), Duration::seconds(1));
        debug_assert_eq!(
            Duration::microseconds(- rug_fuzz_3), Duration::nanoseconds(- 1000)
        );
        debug_assert_eq!(
            Duration::microseconds(- rug_fuzz_4), Duration::milliseconds(- 1)
        );
        debug_assert_eq!(Duration::microseconds(- rug_fuzz_5), Duration::seconds(- 1));
             }
}
}
}    }
    #[test]
    fn microseconds_bounds() {
        let _rug_st_tests_llm_16_297_rrrruuuugggg_microseconds_bounds = 0;
        debug_assert_eq!(Duration::microseconds(i64::MAX), Duration::MAX);
        debug_assert_eq!(Duration::microseconds(i64::MIN), Duration::MIN);
        let _rug_ed_tests_llm_16_297_rrrruuuugggg_microseconds_bounds = 0;
    }
    #[test]
    fn microseconds_properties() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let pos_duration = Duration::microseconds(rug_fuzz_0);
        let neg_duration = Duration::microseconds(-rug_fuzz_1);
        debug_assert!(pos_duration.is_positive());
        debug_assert!(neg_duration.is_negative());
        debug_assert!(! pos_duration.is_negative());
        debug_assert!(! neg_duration.is_positive());
             }
}
}
}    }
    #[test]
    fn microseconds_arithmetic() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(i64, i64, i64, i64, i64, i32, i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Duration::microseconds(rug_fuzz_0) + Duration::microseconds(rug_fuzz_1),
            Duration::milliseconds(1)
        );
        debug_assert_eq!(
            Duration::microseconds(rug_fuzz_2) - Duration::microseconds(rug_fuzz_3),
            Duration::microseconds(200)
        );
        debug_assert_eq!(
            Duration::microseconds(rug_fuzz_4) * rug_fuzz_5, Duration::microseconds(500)
        );
        debug_assert_eq!(
            Duration::microseconds(rug_fuzz_6) / rug_fuzz_7, Duration::microseconds(500)
        );
             }
}
}
}    }
    #[test]
    fn microseconds_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Duration::microseconds(- rug_fuzz_0), Duration::ZERO);
        debug_assert_eq!(Duration::microseconds(rug_fuzz_1), Duration::ZERO);
        debug_assert!(Duration::microseconds(rug_fuzz_2).is_zero());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_298_llm_16_298 {
    use super::*;
    use crate::*;
    use crate::duration::Duration;
    #[test]
    fn milliseconds_positive() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Duration::milliseconds(rug_fuzz_0), Duration::new(1, 500_000_000),
            "1500 milliseconds should be 1 second and 500 million nanoseconds"
        );
             }
}
}
}    }
    #[test]
    fn milliseconds_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Duration::milliseconds(- rug_fuzz_0), Duration::new(- 1, - 500_000_000),
            "Negative 1500 milliseconds should be -1 second and -500 million nanoseconds"
        );
             }
}
}
}    }
    #[test]
    fn milliseconds_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Duration::milliseconds(rug_fuzz_0), Duration::new(0, 0),
            "0 milliseconds should be 0 seconds and 0 nanoseconds"
        );
             }
}
}
}    }
    #[test]
    fn milliseconds_one() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Duration::milliseconds(rug_fuzz_0), Duration::new(0, 1_000_000),
            "1 millisecond should be 0 seconds and 1 million nanoseconds"
        );
             }
}
}
}    }
    #[test]
    fn milliseconds_max() {
        let _rug_st_tests_llm_16_298_llm_16_298_rrrruuuugggg_milliseconds_max = 0;
        debug_assert_eq!(
            Duration::milliseconds(i64::MAX), Duration::new(i64::MAX / 1_000, ((i64::MAX
            % 1_000) * 1_000_000) as i32),
            "Maximum i64 milliseconds should be calculated correctly"
        );
        let _rug_ed_tests_llm_16_298_llm_16_298_rrrruuuugggg_milliseconds_max = 0;
    }
    #[test]
    fn milliseconds_min() {
        let _rug_st_tests_llm_16_298_llm_16_298_rrrruuuugggg_milliseconds_min = 0;
        debug_assert_eq!(
            Duration::milliseconds(i64::MIN), Duration::new(i64::MIN / 1_000, ((i64::MIN
            % 1_000) * 1_000_000) as i32),
            "Minimum i64 milliseconds should be calculated correctly"
        );
        let _rug_ed_tests_llm_16_298_llm_16_298_rrrruuuugggg_milliseconds_min = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_299 {
    use crate::Duration;
    #[test]
    fn duration_minutes() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i64, i64, i64, i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Duration::minutes(rug_fuzz_0), Duration::ZERO);
        debug_assert_eq!(Duration::minutes(rug_fuzz_1), Duration::SECOND * 60);
        debug_assert_eq!(Duration::minutes(rug_fuzz_2), Duration::HOUR);
        debug_assert_eq!(Duration::minutes(rug_fuzz_3), Duration::DAY);
        debug_assert_eq!(Duration::minutes(- rug_fuzz_4), Duration::SECOND * - 60);
        debug_assert_eq!(Duration::minutes(- rug_fuzz_5), Duration::HOUR * - 1);
             }
}
}
}    }
    #[test]
    #[should_panic(expected = "overflow constructing `crate::Duration`")]
    fn duration_minutes_overflow_positive() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let _ = Duration::minutes(i64::MAX / rug_fuzz_0 + rug_fuzz_1);
             }
}
}
}    }
    #[test]
    #[should_panic(expected = "overflow constructing `crate::Duration`")]
    fn duration_minutes_overflow_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let _ = Duration::minutes(i64::MIN / rug_fuzz_0 - rug_fuzz_1);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_300 {
    use super::*;
    use crate::*;
    #[test]
    fn test_nanoseconds_positive() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let nanos = rug_fuzz_0;
        let duration = Duration::nanoseconds(nanos);
        debug_assert_eq!(duration.whole_seconds(), 1);
        debug_assert_eq!(duration.subsec_nanoseconds(), 234_567_890);
             }
}
}
}    }
    #[test]
    fn test_nanoseconds_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let nanos = -rug_fuzz_0;
        let duration = Duration::nanoseconds(nanos);
        debug_assert_eq!(duration.whole_seconds(), - 2);
        debug_assert_eq!(duration.subsec_nanoseconds(), 765_432_110);
             }
}
}
}    }
    #[test]
    fn test_nanoseconds_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let nanos = rug_fuzz_0;
        let duration = Duration::nanoseconds(nanos);
        debug_assert_eq!(duration.whole_seconds(), 0);
        debug_assert_eq!(duration.subsec_nanoseconds(), 0);
             }
}
}
}    }
    #[test]
    fn test_nanoseconds_one() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let nanos = rug_fuzz_0;
        let duration = Duration::nanoseconds(nanos);
        debug_assert_eq!(duration.whole_seconds(), 0);
        debug_assert_eq!(duration.subsec_nanoseconds(), 1);
             }
}
}
}    }
    #[test]
    fn test_nanoseconds_one_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let nanos = -rug_fuzz_0;
        let duration = Duration::nanoseconds(nanos);
        debug_assert_eq!(duration.whole_seconds(), 0);
        debug_assert_eq!(duration.subsec_nanoseconds(), - 1);
             }
}
}
}    }
    #[test]
    fn test_nanoseconds_max() {
        let _rug_st_tests_llm_16_300_rrrruuuugggg_test_nanoseconds_max = 0;
        let nanos = i64::MAX;
        let duration = Duration::nanoseconds(nanos);
        debug_assert_eq!(duration.whole_seconds(), i64::MAX / 1_000_000_000);
        debug_assert_eq!(
            duration.subsec_nanoseconds(), (i64::MAX % 1_000_000_000) as i32
        );
        let _rug_ed_tests_llm_16_300_rrrruuuugggg_test_nanoseconds_max = 0;
    }
    #[test]
    fn test_nanoseconds_min() {
        let _rug_st_tests_llm_16_300_rrrruuuugggg_test_nanoseconds_min = 0;
        let nanos = i64::MIN;
        let duration = Duration::nanoseconds(nanos);
        debug_assert_eq!(duration.whole_seconds(), i64::MIN / 1_000_000_000);
        debug_assert_eq!(
            duration.subsec_nanoseconds(), (i64::MIN % 1_000_000_000) as i32
        );
        let _rug_ed_tests_llm_16_300_rrrruuuugggg_test_nanoseconds_min = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_301 {
    use crate::Duration;
    #[test]
    fn nanoseconds_i128_within_bounds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i128, i128, i128, i128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Duration::nanoseconds_i128(rug_fuzz_0), Duration::seconds(1));
        debug_assert_eq!(
            Duration::nanoseconds_i128(- rug_fuzz_1), Duration::seconds(- 1)
        );
        debug_assert_eq!(
            Duration::nanoseconds_i128(i64::MAX as i128 * rug_fuzz_2), Duration::MAX
        );
        debug_assert_eq!(
            Duration::nanoseconds_i128(i64::MIN as i128 * rug_fuzz_3), Duration::MIN
        );
             }
}
}
}    }
    #[test]
    #[should_panic]
    fn nanoseconds_i128_overflow() {
        let _rug_st_tests_llm_16_301_rrrruuuugggg_nanoseconds_i128_overflow = 0;
        Duration::nanoseconds_i128(i128::MAX);
        let _rug_ed_tests_llm_16_301_rrrruuuugggg_nanoseconds_i128_overflow = 0;
    }
    #[test]
    #[should_panic]
    fn nanoseconds_i128_underflow() {
        let _rug_st_tests_llm_16_301_rrrruuuugggg_nanoseconds_i128_underflow = 0;
        Duration::nanoseconds_i128(i128::MIN);
        let _rug_ed_tests_llm_16_301_rrrruuuugggg_nanoseconds_i128_underflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_302 {
    use super::*;
    use crate::*;
    #[test]
    fn new_with_no_wrap() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(i64, i32, i64, i32, i64, i32, i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Duration::new(rug_fuzz_0, rug_fuzz_1), Duration::seconds(1));
        debug_assert_eq!(
            Duration::new(- rug_fuzz_2, rug_fuzz_3), Duration::seconds(- 1)
        );
        debug_assert_eq!(
            Duration::new(rug_fuzz_4, rug_fuzz_5), Duration::milliseconds(500)
        );
        debug_assert_eq!(
            Duration::new(- rug_fuzz_6, - rug_fuzz_7), Duration::seconds(- 3)
        );
             }
}
}
}    }
    #[test]
    fn new_with_nanosecond_wrapping() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(i64, i32, i64, i32, i64, i32, i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Duration::new(rug_fuzz_0, rug_fuzz_1), Duration::seconds(2));
        debug_assert_eq!(Duration::new(rug_fuzz_2, - rug_fuzz_3), Duration::seconds(0));
        debug_assert_eq!(Duration::new(- rug_fuzz_4, rug_fuzz_5), Duration::seconds(1));
        debug_assert_eq!(
            Duration::new(- rug_fuzz_6, - rug_fuzz_7), Duration::seconds(- 4)
        );
             }
}
}
}    }
    #[test]
    fn new_with_nanosecond_carry() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i64, i32, i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Duration::new(rug_fuzz_0, rug_fuzz_1), Duration::seconds(2) +
            Duration::milliseconds(500)
        );
        debug_assert_eq!(
            Duration::new(- rug_fuzz_2, - rug_fuzz_3), Duration::seconds(- 2) -
            Duration::milliseconds(500)
        );
             }
}
}
}    }
    #[test]
    fn new_with_maximum_values() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Duration::new(i64::MAX, rug_fuzz_0), Duration::MAX);
        debug_assert_eq!(Duration::new(i64::MIN, - rug_fuzz_1), Duration::MIN);
             }
}
}
}    }
    #[test]
    #[should_panic(expected = "overflow constructing `crate::Duration`")]
    fn new_with_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let _ = Duration::new(i64::MAX, rug_fuzz_0);
             }
}
}
}    }
    #[test]
    #[should_panic(expected = "overflow constructing `crate::Duration`")]
    fn new_with_underflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let _ = Duration::new(i64::MIN, -rug_fuzz_0);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_303 {
    use crate::Duration;
    use crate::duration::Padding;
    #[test]
    fn test_new_unchecked_positive() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::new_unchecked(rug_fuzz_0, rug_fuzz_1);
        debug_assert_eq!(duration.seconds, 5);
        debug_assert_eq!(duration.nanoseconds, 100);
        debug_assert_eq!(duration.padding, Padding::Optimize);
             }
}
}
}    }
    #[test]
    fn test_new_unchecked_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::new_unchecked(-rug_fuzz_0, -rug_fuzz_1);
        debug_assert_eq!(duration.seconds, - 5);
        debug_assert_eq!(duration.nanoseconds, - 100);
        debug_assert_eq!(duration.padding, Padding::Optimize);
             }
}
}
}    }
    #[test]
    fn test_new_unchecked_zero_positive() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::new_unchecked(rug_fuzz_0, rug_fuzz_1);
        debug_assert_eq!(duration.seconds, 0);
        debug_assert_eq!(duration.nanoseconds, 100);
        debug_assert_eq!(duration.padding, Padding::Optimize);
             }
}
}
}    }
    #[test]
    fn test_new_unchecked_zero_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::new_unchecked(rug_fuzz_0, -rug_fuzz_1);
        debug_assert_eq!(duration.seconds, 0);
        debug_assert_eq!(duration.nanoseconds, - 100);
        debug_assert_eq!(duration.padding, Padding::Optimize);
             }
}
}
}    }
    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_new_unchecked_panic_positive_nanoseconds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let _duration = Duration::new_unchecked(rug_fuzz_0, rug_fuzz_1);
             }
}
}
}    }
    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_new_unchecked_panic_negative_nanoseconds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let _duration = Duration::new_unchecked(-rug_fuzz_0, -rug_fuzz_1);
             }
}
}
}    }
    #[test]
    fn test_new_unchecked_edge_positive_nanoseconds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::new_unchecked(rug_fuzz_0, rug_fuzz_1);
        debug_assert_eq!(duration.seconds, 5);
        debug_assert_eq!(duration.nanoseconds, 999_999_999);
        debug_assert_eq!(duration.padding, Padding::Optimize);
             }
}
}
}    }
    #[test]
    fn test_new_unchecked_edge_negative_nanoseconds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::new_unchecked(-rug_fuzz_0, -rug_fuzz_1);
        debug_assert_eq!(duration.seconds, - 5);
        debug_assert_eq!(duration.nanoseconds, - 999_999_999);
        debug_assert_eq!(duration.padding, Padding::Optimize);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_304 {
    use super::*;
    use crate::*;
    #[test]
    fn saturating_add_with_no_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i64, i64, i64, i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Duration::seconds(rug_fuzz_0).saturating_add(Duration::seconds(rug_fuzz_1)),
            Duration::seconds(60)
        );
        debug_assert_eq!(
            Duration::nanoseconds(rug_fuzz_2)
            .saturating_add(Duration::nanoseconds(rug_fuzz_3)),
            Duration::nanoseconds(1000)
        );
        debug_assert_eq!(
            Duration::seconds(- rug_fuzz_4)
            .saturating_add(Duration::seconds(rug_fuzz_5)), Duration::seconds(0)
        );
             }
}
}
}    }
    #[test]
    fn saturating_add_with_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Duration::MAX.saturating_add(Duration::seconds(rug_fuzz_0)), Duration::MAX
        );
        debug_assert_eq!(Duration::MAX.saturating_add(Duration::MAX), Duration::MAX);
             }
}
}
}    }
    #[test]
    fn saturating_add_with_underflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Duration::MIN.saturating_add(Duration::seconds(- rug_fuzz_0)), Duration::MIN
        );
        debug_assert_eq!(Duration::MIN.saturating_add(Duration::MIN), Duration::MIN);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_306 {
    use super::*;
    use crate::*;
    #[test]
    fn saturating_seconds_f32_returns_zero_for_nan() {
        let _rug_st_tests_llm_16_306_rrrruuuugggg_saturating_seconds_f32_returns_zero_for_nan = 0;
        let duration = Duration::saturating_seconds_f32(f32::NAN);
        debug_assert_eq!(duration, Duration::ZERO);
        let _rug_ed_tests_llm_16_306_rrrruuuugggg_saturating_seconds_f32_returns_zero_for_nan = 0;
    }
    #[test]
    fn saturating_seconds_f32_returns_max_for_infinity() {
        let _rug_st_tests_llm_16_306_rrrruuuugggg_saturating_seconds_f32_returns_max_for_infinity = 0;
        let duration = Duration::saturating_seconds_f32(f32::INFINITY);
        debug_assert_eq!(duration, Duration::MAX);
        let _rug_ed_tests_llm_16_306_rrrruuuugggg_saturating_seconds_f32_returns_max_for_infinity = 0;
    }
    #[test]
    fn saturating_seconds_f32_returns_min_for_negative_infinity() {
        let _rug_st_tests_llm_16_306_rrrruuuugggg_saturating_seconds_f32_returns_min_for_negative_infinity = 0;
        let duration = Duration::saturating_seconds_f32(f32::NEG_INFINITY);
        debug_assert_eq!(duration, Duration::MIN);
        let _rug_ed_tests_llm_16_306_rrrruuuugggg_saturating_seconds_f32_returns_min_for_negative_infinity = 0;
    }
    #[test]
    fn saturating_seconds_f32_handles_positive_values() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::saturating_seconds_f32(rug_fuzz_0);
        debug_assert_eq!(duration, Duration::seconds_f32(0.5));
             }
}
}
}    }
    #[test]
    fn saturating_seconds_f32_handles_negative_values() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::saturating_seconds_f32(-rug_fuzz_0);
        debug_assert_eq!(duration, Duration::seconds_f32(- 0.5));
             }
}
}
}    }
    #[test]
    fn saturating_seconds_f32_saturates_positive_overflow() {
        let _rug_st_tests_llm_16_306_rrrruuuugggg_saturating_seconds_f32_saturates_positive_overflow = 0;
        let large_positive = f32::MAX;
        let duration = Duration::saturating_seconds_f32(large_positive);
        debug_assert_eq!(duration, Duration::MAX);
        let _rug_ed_tests_llm_16_306_rrrruuuugggg_saturating_seconds_f32_saturates_positive_overflow = 0;
    }
    #[test]
    fn saturating_seconds_f32_saturates_negative_overflow() {
        let _rug_st_tests_llm_16_306_rrrruuuugggg_saturating_seconds_f32_saturates_negative_overflow = 0;
        let large_negative = f32::MIN;
        let duration = Duration::saturating_seconds_f32(large_negative);
        debug_assert_eq!(duration, Duration::MIN);
        let _rug_ed_tests_llm_16_306_rrrruuuugggg_saturating_seconds_f32_saturates_negative_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_307_llm_16_307 {
    use super::*;
    use crate::*;
    use std::f64;
    #[test]
    fn test_saturating_seconds_f64_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Duration::saturating_seconds_f64(rug_fuzz_0), Duration::ZERO);
             }
}
}
}    }
    #[test]
    fn test_saturating_seconds_f64_half() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Duration::saturating_seconds_f64(rug_fuzz_0), Duration::seconds_f64(0.5)
        );
             }
}
}
}    }
    #[test]
    fn test_saturating_seconds_f64_negative_half() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Duration::saturating_seconds_f64(- rug_fuzz_0), Duration::seconds_f64(- 0.5)
        );
             }
}
}
}    }
    #[test]
    fn test_saturating_seconds_f64_nan() {
        let _rug_st_tests_llm_16_307_llm_16_307_rrrruuuugggg_test_saturating_seconds_f64_nan = 0;
        debug_assert_eq!(Duration::saturating_seconds_f64(f64::NAN), Duration::ZERO);
        let _rug_ed_tests_llm_16_307_llm_16_307_rrrruuuugggg_test_saturating_seconds_f64_nan = 0;
    }
    #[test]
    fn test_saturating_seconds_f64_infinity() {
        let _rug_st_tests_llm_16_307_llm_16_307_rrrruuuugggg_test_saturating_seconds_f64_infinity = 0;
        debug_assert_eq!(Duration::saturating_seconds_f64(f64::INFINITY), Duration::MAX);
        let _rug_ed_tests_llm_16_307_llm_16_307_rrrruuuugggg_test_saturating_seconds_f64_infinity = 0;
    }
    #[test]
    fn test_saturating_seconds_f64_negative_infinity() {
        let _rug_st_tests_llm_16_307_llm_16_307_rrrruuuugggg_test_saturating_seconds_f64_negative_infinity = 0;
        debug_assert_eq!(
            Duration::saturating_seconds_f64(f64::NEG_INFINITY), Duration::MIN
        );
        let _rug_ed_tests_llm_16_307_llm_16_307_rrrruuuugggg_test_saturating_seconds_f64_negative_infinity = 0;
    }
    #[test]
    fn test_saturating_seconds_f64_max() {
        let _rug_st_tests_llm_16_307_llm_16_307_rrrruuuugggg_test_saturating_seconds_f64_max = 0;
        debug_assert_eq!(
            Duration::saturating_seconds_f64(i64::MAX as f64),
            Duration::seconds(i64::MAX)
        );
        let _rug_ed_tests_llm_16_307_llm_16_307_rrrruuuugggg_test_saturating_seconds_f64_max = 0;
    }
    #[test]
    fn test_saturating_seconds_f64_min() {
        let _rug_st_tests_llm_16_307_llm_16_307_rrrruuuugggg_test_saturating_seconds_f64_min = 0;
        debug_assert_eq!(
            Duration::saturating_seconds_f64(i64::MIN as f64),
            Duration::seconds(i64::MIN)
        );
        let _rug_ed_tests_llm_16_307_llm_16_307_rrrruuuugggg_test_saturating_seconds_f64_min = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_308 {
    use crate::Duration;
    #[test]
    fn saturating_sub_with_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Duration::seconds(rug_fuzz_0).saturating_sub(Duration::seconds(rug_fuzz_1)),
            Duration::ZERO
        );
        debug_assert_eq!(
            Duration::ZERO.saturating_sub(Duration::seconds(rug_fuzz_2)),
            Duration::seconds(- 5)
        );
             }
}
}
}    }
    #[test]
    fn saturating_sub_with_min() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Duration::MIN.saturating_sub(Duration::seconds(rug_fuzz_0)), Duration::MIN
        );
        debug_assert_eq!(Duration::MIN.saturating_sub(Duration::MIN), Duration::ZERO);
             }
}
}
}    }
    #[test]
    fn saturating_sub_with_max() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Duration::MAX.saturating_sub(Duration::seconds(- rug_fuzz_0)), Duration::MAX
        );
        debug_assert_eq!(Duration::MAX.saturating_sub(Duration::MAX), Duration::ZERO);
             }
}
}
}    }
    #[test]
    fn saturating_sub_with_positive_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i64, i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Duration::seconds(rug_fuzz_0).saturating_sub(Duration::seconds(-
            rug_fuzz_1)), Duration::seconds(15)
        );
        debug_assert_eq!(
            Duration::seconds(- rug_fuzz_2)
            .saturating_sub(Duration::seconds(rug_fuzz_3)), Duration::seconds(- 15)
        );
             }
}
}
}    }
    #[test]
    fn saturating_sub_with_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Duration::seconds(i64::MAX).saturating_sub(Duration::seconds(- rug_fuzz_0)),
            Duration::MAX
        );
        debug_assert_eq!(
            Duration::seconds(i64::MIN).saturating_sub(Duration::seconds(rug_fuzz_1)),
            Duration::MIN
        );
             }
}
}
}    }
    #[test]
    fn saturating_sub_with_underflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i64, i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Duration::seconds(- rug_fuzz_0)
            .saturating_sub(Duration::seconds(rug_fuzz_1)), Duration::seconds(- 15)
        );
        debug_assert_eq!(
            Duration::seconds(rug_fuzz_2).saturating_sub(Duration::seconds(rug_fuzz_3)),
            Duration::seconds(- 5)
        );
             }
}
}
}    }
    #[test]
    fn saturating_sub_with_nanoseconds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i64, i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Duration::nanoseconds(rug_fuzz_0)
            .saturating_sub(Duration::nanoseconds(rug_fuzz_1)), Duration::nanoseconds(-
            500)
        );
        debug_assert_eq!(
            Duration::nanoseconds(- rug_fuzz_2).saturating_sub(Duration::nanoseconds(-
            rug_fuzz_3)), Duration::nanoseconds(500)
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_309 {
    use super::*;
    use crate::*;
    #[test]
    fn seconds_positive() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Duration::seconds(rug_fuzz_0), Duration::new(1, 0));
        debug_assert_eq!(Duration::seconds(rug_fuzz_1), Duration::new(5, 0));
             }
}
}
}    }
    #[test]
    fn seconds_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Duration::seconds(- rug_fuzz_0), Duration::new(- 1, 0));
        debug_assert_eq!(Duration::seconds(- rug_fuzz_1), Duration::new(- 5, 0));
             }
}
}
}    }
    #[test]
    fn seconds_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Duration::seconds(rug_fuzz_0), Duration::new(0, 0));
             }
}
}
}    }
    #[test]
    fn seconds_boundary() {
        let _rug_st_tests_llm_16_309_rrrruuuugggg_seconds_boundary = 0;
        debug_assert_eq!(Duration::seconds(i64::MAX), Duration::new(i64::MAX, 0));
        debug_assert_eq!(Duration::seconds(i64::MIN), Duration::new(i64::MIN, 0));
        let _rug_ed_tests_llm_16_309_rrrruuuugggg_seconds_boundary = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_311 {
    use super::*;
    use crate::*;
    #[test]
    fn test_zero_seconds_f64() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Duration::seconds_f64(rug_fuzz_0), Duration { seconds : 0, nanoseconds : 0,
            padding : Padding::Optimize, }
        );
             }
}
}
}    }
    #[test]
    fn test_positive_seconds_f64() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Duration::seconds_f64(rug_fuzz_0), Duration { seconds : 2, nanoseconds :
            700_000_000, padding : Padding::Optimize, }
        );
             }
}
}
}    }
    #[test]
    fn test_negative_seconds_f64() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Duration::seconds_f64(- rug_fuzz_0), Duration { seconds : - 2, nanoseconds :
            - 700_000_000, padding : Padding::Optimize, }
        );
             }
}
}
}    }
    #[test]
    fn test_subsecond_positive_seconds_f64() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Duration::seconds_f64(rug_fuzz_0), Duration { seconds : 0, nanoseconds : 123,
            padding : Padding::Optimize, }
        );
             }
}
}
}    }
    #[test]
    fn test_subsecond_negative_seconds_f64() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Duration::seconds_f64(- rug_fuzz_0), Duration { seconds : 0, nanoseconds : -
            123, padding : Padding::Optimize, }
        );
             }
}
}
}    }
    #[test]
    fn test_large_number_seconds_f64() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Duration::seconds_f64(rug_fuzz_0), Duration { seconds : 10_000_000_000,
            nanoseconds : 0, padding : Padding::Optimize, }
        );
             }
}
}
}    }
    #[test]
    fn test_small_number_seconds_f64() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Duration::seconds_f64(rug_fuzz_0), Duration { seconds : 0, nanoseconds : 0,
            padding : Padding::Optimize, }
        );
             }
}
}
}    }
    #[test]
    #[should_panic(expected = "overflow constructing `crate::Duration`")]
    fn test_overflow_seconds_f64() {
        let _rug_st_tests_llm_16_311_rrrruuuugggg_test_overflow_seconds_f64 = 0;
        let _ = Duration::seconds_f64(f64::MAX);
        let _rug_ed_tests_llm_16_311_rrrruuuugggg_test_overflow_seconds_f64 = 0;
    }
    #[test]
    #[should_panic(expected = "overflow constructing `crate::Duration`")]
    fn test_underflow_seconds_f64() {
        let _rug_st_tests_llm_16_311_rrrruuuugggg_test_underflow_seconds_f64 = 0;
        let _ = Duration::seconds_f64(f64::MIN);
        let _rug_ed_tests_llm_16_311_rrrruuuugggg_test_underflow_seconds_f64 = 0;
    }
    #[test]
    #[should_panic(expected = "passed NaN to `crate::Duration::seconds_f64`")]
    fn test_nan_seconds_f64() {
        let _rug_st_tests_llm_16_311_rrrruuuugggg_test_nan_seconds_f64 = 0;
        let _ = Duration::seconds_f64(f64::NAN);
        let _rug_ed_tests_llm_16_311_rrrruuuugggg_test_nan_seconds_f64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_312_llm_16_312 {
    use crate::Duration;
    #[test]
    fn subsec_microseconds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14)) = <(i64, i64, i32, i64, i32, i64, i32, i64, i32, i64, i32, i64, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Duration::seconds(rug_fuzz_0).subsec_microseconds(), 0);
        debug_assert_eq!(Duration::new(rug_fuzz_1, rug_fuzz_2).subsec_microseconds(), 0);
        debug_assert_eq!(Duration::new(rug_fuzz_3, rug_fuzz_4).subsec_microseconds(), 1);
        debug_assert_eq!(
            Duration::new(- rug_fuzz_5, - rug_fuzz_6).subsec_microseconds(), - 1
        );
        debug_assert_eq!(
            Duration::new(- rug_fuzz_7, rug_fuzz_8).subsec_microseconds(), 0
        );
        debug_assert_eq!(
            Duration::new(rug_fuzz_9, rug_fuzz_10).subsec_microseconds(), 1_000
        );
        debug_assert_eq!(
            Duration::new(rug_fuzz_11, - rug_fuzz_12).subsec_microseconds(), - 1_000
        );
        debug_assert_eq!(Duration::new(i64::MIN, rug_fuzz_13).subsec_microseconds(), 0);
        debug_assert_eq!(
            Duration::new(i64::MAX, rug_fuzz_14).subsec_microseconds(), 999_999
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_313 {
    use crate::Duration;
    #[test]
    fn test_subsec_milliseconds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(i64, i64, i64, i64, i64, i64, i64, i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Duration::milliseconds(rug_fuzz_0).subsec_milliseconds(), 500);
        debug_assert_eq!(Duration::milliseconds(rug_fuzz_1).subsec_milliseconds(), 0);
        debug_assert_eq!(
            Duration::milliseconds(- rug_fuzz_2).subsec_milliseconds(), - 500
        );
        debug_assert_eq!(Duration::milliseconds(- rug_fuzz_3).subsec_milliseconds(), 0);
        debug_assert_eq!(Duration::seconds(rug_fuzz_4).subsec_milliseconds(), 0);
        debug_assert_eq!(Duration::seconds(- rug_fuzz_5).subsec_milliseconds(), 0);
        debug_assert_eq!(Duration::milliseconds(rug_fuzz_6).subsec_milliseconds(), 500);
        debug_assert_eq!(
            Duration::milliseconds(- rug_fuzz_7).subsec_milliseconds(), - 500
        );
        debug_assert_eq!(Duration::nanoseconds(rug_fuzz_8).subsec_milliseconds(), 500);
        debug_assert_eq!(
            Duration::nanoseconds(- rug_fuzz_9).subsec_milliseconds(), - 500
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_317 {
    use super::*;
    use crate::*;
    #[test]
    fn test_weeks_positive() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Duration::weeks(rug_fuzz_0), Duration::seconds(604_800));
        debug_assert_eq!(Duration::weeks(rug_fuzz_1), Duration::seconds(2 * 604_800));
        debug_assert_eq!(Duration::weeks(rug_fuzz_2), Duration::seconds(52 * 604_800));
             }
}
}
}    }
    #[test]
    fn test_weeks_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Duration::weeks(rug_fuzz_0), Duration::seconds(0));
             }
}
}
}    }
    #[test]
    fn test_weeks_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Duration::weeks(- rug_fuzz_0), Duration::seconds(- 604_800));
        debug_assert_eq!(
            Duration::weeks(- rug_fuzz_1), Duration::seconds(- 2 * 604_800)
        );
        debug_assert_eq!(
            Duration::weeks(- rug_fuzz_2), Duration::seconds(- 52 * 604_800)
        );
             }
}
}
}    }
    #[test]
    #[should_panic(expected = "overflow constructing `crate::Duration`")]
    fn test_weeks_overflow_positive() {
        let _rug_st_tests_llm_16_317_rrrruuuugggg_test_weeks_overflow_positive = 0;
        let _ = Duration::weeks(i64::MAX);
        let _rug_ed_tests_llm_16_317_rrrruuuugggg_test_weeks_overflow_positive = 0;
    }
    #[test]
    #[should_panic(expected = "overflow constructing `crate::Duration`")]
    fn test_weeks_overflow_negative() {
        let _rug_st_tests_llm_16_317_rrrruuuugggg_test_weeks_overflow_negative = 0;
        let _ = Duration::weeks(i64::MIN);
        let _rug_ed_tests_llm_16_317_rrrruuuugggg_test_weeks_overflow_negative = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_318 {
    use super::*;
    use crate::*;
    use crate::ext::NumericalDuration;
    #[test]
    fn test_whole_days() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17, mut rug_fuzz_18, mut rug_fuzz_19, mut rug_fuzz_20, mut rug_fuzz_21, mut rug_fuzz_22, mut rug_fuzz_23, mut rug_fuzz_24, mut rug_fuzz_25)) = <(i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Duration::days(rug_fuzz_0).whole_days(), 1);
        debug_assert_eq!(Duration::hours(rug_fuzz_1).whole_days(), 1);
        debug_assert_eq!(Duration::hours(rug_fuzz_2).whole_days(), 2);
        debug_assert_eq!(Duration::hours(rug_fuzz_3).whole_days(), 1);
        debug_assert_eq!(Duration::hours(rug_fuzz_4).whole_days(), 0);
        debug_assert_eq!(Duration::minutes(rug_fuzz_5).whole_days(), 1);
        debug_assert_eq!(Duration::minutes(rug_fuzz_6).whole_days(), 2);
        debug_assert_eq!(Duration::minutes(rug_fuzz_7).whole_days(), 1);
        debug_assert_eq!(Duration::minutes(rug_fuzz_8).whole_days(), 0);
        debug_assert_eq!(Duration::seconds(rug_fuzz_9).whole_days(), 1);
        debug_assert_eq!(Duration::seconds(rug_fuzz_10).whole_days(), 2);
        debug_assert_eq!(Duration::seconds(rug_fuzz_11).whole_days(), 1);
        debug_assert_eq!(Duration::seconds(rug_fuzz_12).whole_days(), 0);
        debug_assert_eq!(Duration::days(- rug_fuzz_13).whole_days(), - 1);
        debug_assert_eq!(Duration::hours(- rug_fuzz_14).whole_days(), - 1);
        debug_assert_eq!(Duration::hours(- rug_fuzz_15).whole_days(), - 2);
        debug_assert_eq!(Duration::hours(- rug_fuzz_16).whole_days(), - 1);
        debug_assert_eq!(Duration::hours(- rug_fuzz_17).whole_days(), 0);
        debug_assert_eq!(Duration::minutes(- rug_fuzz_18).whole_days(), - 1);
        debug_assert_eq!(Duration::minutes(- rug_fuzz_19).whole_days(), - 2);
        debug_assert_eq!(Duration::minutes(- rug_fuzz_20).whole_days(), - 1);
        debug_assert_eq!(Duration::minutes(- rug_fuzz_21).whole_days(), 0);
        debug_assert_eq!(Duration::seconds(- rug_fuzz_22).whole_days(), - 1);
        debug_assert_eq!(Duration::seconds(- rug_fuzz_23).whole_days(), - 2);
        debug_assert_eq!(Duration::seconds(- rug_fuzz_24).whole_days(), - 1);
        debug_assert_eq!(Duration::seconds(- rug_fuzz_25).whole_days(), 0);
        debug_assert_eq!(Duration::MIN.whole_days(), i64::MIN);
        debug_assert_eq!(Duration::MAX.whole_days(), i64::MAX);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_319 {
    use super::*;
    use crate::*;
    #[test]
    fn whole_hours_zero() {
        let _rug_st_tests_llm_16_319_rrrruuuugggg_whole_hours_zero = 0;
        debug_assert_eq!(Duration::ZERO.whole_hours(), 0);
        let _rug_ed_tests_llm_16_319_rrrruuuugggg_whole_hours_zero = 0;
    }
    #[test]
    fn whole_hours_single() {
        let _rug_st_tests_llm_16_319_rrrruuuugggg_whole_hours_single = 0;
        debug_assert_eq!(Duration::HOUR.whole_hours(), 1);
        debug_assert_eq!((- Duration::HOUR).whole_hours(), - 1);
        let _rug_ed_tests_llm_16_319_rrrruuuugggg_whole_hours_single = 0;
    }
    #[test]
    fn whole_hours_multiple() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Duration::hours(rug_fuzz_0).whole_hours(), 24);
        debug_assert_eq!(Duration::hours(- rug_fuzz_1).whole_hours(), - 24);
             }
}
}
}    }
    #[test]
    fn whole_hours_partially() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i64, i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Duration::minutes(rug_fuzz_0).whole_hours(), 1);
        debug_assert_eq!(Duration::minutes(- rug_fuzz_1).whole_hours(), - 1);
        debug_assert_eq!(Duration::minutes(rug_fuzz_2).whole_hours(), 2);
        debug_assert_eq!(Duration::minutes(- rug_fuzz_3).whole_hours(), - 2);
             }
}
}
}    }
    #[test]
    fn whole_hours_limits() {
        let _rug_st_tests_llm_16_319_rrrruuuugggg_whole_hours_limits = 0;
        debug_assert_eq!(Duration::MIN.whole_hours(), i64::MIN);
        debug_assert_eq!(Duration::MAX.whole_hours(), i64::MAX);
        let _rug_ed_tests_llm_16_319_rrrruuuugggg_whole_hours_limits = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_320 {
    use super::*;
    use crate::*;
    #[test]
    fn whole_microseconds_zero() {
        let _rug_st_tests_llm_16_320_rrrruuuugggg_whole_microseconds_zero = 0;
        debug_assert_eq!(Duration::ZERO.whole_microseconds(), 0);
        let _rug_ed_tests_llm_16_320_rrrruuuugggg_whole_microseconds_zero = 0;
    }
    #[test]
    fn whole_microseconds_one_second() {
        let _rug_st_tests_llm_16_320_rrrruuuugggg_whole_microseconds_one_second = 0;
        debug_assert_eq!(Duration::SECOND.whole_microseconds(), 1_000_000);
        let _rug_ed_tests_llm_16_320_rrrruuuugggg_whole_microseconds_one_second = 0;
    }
    #[test]
    fn whole_microseconds_minus_one_second() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Duration::seconds(- rug_fuzz_0).whole_microseconds(), - 1_000_000
        );
             }
}
}
}    }
    #[test]
    fn whole_microseconds_one_microsecond() {
        let _rug_st_tests_llm_16_320_rrrruuuugggg_whole_microseconds_one_microsecond = 0;
        debug_assert_eq!(Duration::MICROSECOND.whole_microseconds(), 1);
        let _rug_ed_tests_llm_16_320_rrrruuuugggg_whole_microseconds_one_microsecond = 0;
    }
    #[test]
    fn whole_microseconds_minus_one_microsecond() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Duration::microseconds(- rug_fuzz_0).whole_microseconds(), - 1);
             }
}
}
}    }
    #[test]
    fn whole_microseconds_max_value() {
        let _rug_st_tests_llm_16_320_rrrruuuugggg_whole_microseconds_max_value = 0;
        debug_assert_eq!(Duration::MAX.whole_microseconds(), i128::MAX);
        let _rug_ed_tests_llm_16_320_rrrruuuugggg_whole_microseconds_max_value = 0;
    }
    #[test]
    fn whole_microseconds_min_value() {
        let _rug_st_tests_llm_16_320_rrrruuuugggg_whole_microseconds_min_value = 0;
        debug_assert_eq!(Duration::MIN.whole_microseconds(), i128::MIN + 1);
        let _rug_ed_tests_llm_16_320_rrrruuuugggg_whole_microseconds_min_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_321 {
    use crate::duration::Duration;
    #[test]
    fn whole_milliseconds_zero_duration() {
        let _rug_st_tests_llm_16_321_rrrruuuugggg_whole_milliseconds_zero_duration = 0;
        debug_assert_eq!(Duration::ZERO.whole_milliseconds(), 0);
        let _rug_ed_tests_llm_16_321_rrrruuuugggg_whole_milliseconds_zero_duration = 0;
    }
    #[test]
    fn whole_milliseconds_one_second() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Duration::seconds(rug_fuzz_0).whole_milliseconds(), 1_000);
             }
}
}
}    }
    #[test]
    fn whole_milliseconds_minus_one_second() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Duration::seconds(- rug_fuzz_0).whole_milliseconds(), - 1_000);
             }
}
}
}    }
    #[test]
    fn whole_milliseconds_one_millisecond() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Duration::milliseconds(rug_fuzz_0).whole_milliseconds(), 1);
             }
}
}
}    }
    #[test]
    fn whole_milliseconds_minus_one_millisecond() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Duration::milliseconds(- rug_fuzz_0).whole_milliseconds(), - 1);
             }
}
}
}    }
    #[test]
    fn whole_milliseconds_one_hour() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Duration::hours(rug_fuzz_0).whole_milliseconds(), 3_600_000);
             }
}
}
}    }
    #[test]
    fn whole_milliseconds_one_nano() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Duration::nanoseconds(rug_fuzz_0).whole_milliseconds(), 1);
             }
}
}
}    }
    #[test]
    fn whole_milliseconds_complex() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::new(rug_fuzz_0, rug_fuzz_1);
        debug_assert_eq!(duration.whole_milliseconds(), 5_750);
             }
}
}
}    }
    #[test]
    fn whole_milliseconds_max() {
        let _rug_st_tests_llm_16_321_rrrruuuugggg_whole_milliseconds_max = 0;
        debug_assert_eq!(Duration::MAX.whole_milliseconds(), i128::MAX);
        let _rug_ed_tests_llm_16_321_rrrruuuugggg_whole_milliseconds_max = 0;
    }
    #[test]
    fn whole_milliseconds_min() {
        let _rug_st_tests_llm_16_321_rrrruuuugggg_whole_milliseconds_min = 0;
        debug_assert_eq!(Duration::MIN.whole_milliseconds(), i128::MIN);
        let _rug_ed_tests_llm_16_321_rrrruuuugggg_whole_milliseconds_min = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_322 {
    use super::*;
    use crate::*;
    #[test]
    fn test_whole_minutes() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8)) = <(i64, i64, i64, i64, i64, i64, i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        use crate::duration::Duration;
        debug_assert_eq!(Duration::minutes(rug_fuzz_0).whole_minutes(), 1);
        debug_assert_eq!(Duration::minutes(- rug_fuzz_1).whole_minutes(), - 1);
        debug_assert_eq!(Duration::seconds(rug_fuzz_2).whole_minutes(), 0);
        debug_assert_eq!(Duration::seconds(- rug_fuzz_3).whole_minutes(), 0);
        debug_assert_eq!(Duration::hours(rug_fuzz_4).whole_minutes(), 60);
        debug_assert_eq!(Duration::hours(- rug_fuzz_5).whole_minutes(), - 60);
        debug_assert_eq!(Duration::seconds(rug_fuzz_6).whole_minutes(), 2);
        debug_assert_eq!(Duration::milliseconds(rug_fuzz_7).whole_minutes(), 1);
        debug_assert_eq!(Duration::milliseconds(- rug_fuzz_8).whole_minutes(), - 1);
        debug_assert_eq!(Duration::ZERO.whole_minutes(), 0);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_323 {
    use crate::Duration;
    #[test]
    fn whole_nanoseconds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13)) = <(i64, i32, i64, i32, i64, i32, i64, i32, i64, i32, i64, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::new(rug_fuzz_0, rug_fuzz_1);
        let nanoseconds = duration.whole_nanoseconds();
        debug_assert_eq!(nanoseconds, 1_500_000_000);
        let duration = Duration::new(-rug_fuzz_2, -rug_fuzz_3);
        let nanoseconds = duration.whole_nanoseconds();
        debug_assert_eq!(nanoseconds, - 1_500_000_000);
        let duration = Duration::new(rug_fuzz_4, rug_fuzz_5);
        let nanoseconds = duration.whole_nanoseconds();
        debug_assert_eq!(nanoseconds, 1);
        let duration = Duration::new(rug_fuzz_6, -rug_fuzz_7);
        let nanoseconds = duration.whole_nanoseconds();
        debug_assert_eq!(nanoseconds, - 1);
        let duration = Duration::new(rug_fuzz_8, rug_fuzz_9);
        let nanoseconds = duration.whole_nanoseconds();
        debug_assert_eq!(nanoseconds, 2_000_000_000);
        let duration = Duration::new(rug_fuzz_10, -rug_fuzz_11);
        let nanoseconds = duration.whole_nanoseconds();
        debug_assert_eq!(nanoseconds, 1_000_000_000);
        let duration = Duration::new(i64::MAX, rug_fuzz_12);
        let nanoseconds = duration.whole_nanoseconds();
        debug_assert_eq!(nanoseconds, i64::MAX as i128 * 1_000_000_000);
        let duration = Duration::new(i64::MIN, rug_fuzz_13);
        let nanoseconds = duration.whole_nanoseconds();
        debug_assert_eq!(nanoseconds, i64::MIN as i128 * 1_000_000_000);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_324 {
    use crate::Duration;
    #[test]
    fn test_whole_seconds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17, mut rug_fuzz_18, mut rug_fuzz_19, mut rug_fuzz_20, mut rug_fuzz_21)) = <(i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i32, i64, i32, i64, i32, i64, i32, i64, i32, i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Duration::seconds(rug_fuzz_0).whole_seconds(), 1);
        debug_assert_eq!(Duration::seconds(- rug_fuzz_1).whole_seconds(), - 1);
        debug_assert_eq!(Duration::minutes(rug_fuzz_2).whole_seconds(), 60);
        debug_assert_eq!(Duration::minutes(- rug_fuzz_3).whole_seconds(), - 60);
        debug_assert_eq!(Duration::hours(rug_fuzz_4).whole_seconds(), 3_600);
        debug_assert_eq!(Duration::hours(- rug_fuzz_5).whole_seconds(), - 3_600);
        debug_assert_eq!(Duration::days(rug_fuzz_6).whole_seconds(), 86_400);
        debug_assert_eq!(Duration::days(- rug_fuzz_7).whole_seconds(), - 86_400);
        debug_assert_eq!(Duration::weeks(rug_fuzz_8).whole_seconds(), 604_800);
        debug_assert_eq!(Duration::weeks(- rug_fuzz_9).whole_seconds(), - 604_800);
        debug_assert_eq!(Duration::new(rug_fuzz_10, rug_fuzz_11).whole_seconds(), 1);
        debug_assert_eq!(
            Duration::new(- rug_fuzz_12, - rug_fuzz_13).whole_seconds(), - 1
        );
        debug_assert_eq!(Duration::new(rug_fuzz_14, rug_fuzz_15).whole_seconds(), 2);
        debug_assert_eq!(
            Duration::new(- rug_fuzz_16, - rug_fuzz_17).whole_seconds(), - 2
        );
        debug_assert_eq!(Duration::new(rug_fuzz_18, - rug_fuzz_19).whole_seconds(), 0);
        debug_assert_eq!(Duration::new(- rug_fuzz_20, rug_fuzz_21).whole_seconds(), 0);
        debug_assert_eq!(Duration::ZERO.whole_seconds(), 0);
        debug_assert_eq!(Duration::MIN.whole_seconds(), i64::MIN);
        debug_assert_eq!(Duration::MAX.whole_seconds(), i64::MAX);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_325 {
    use super::*;
    use crate::*;
    use crate::ext::NumericalDuration;
    #[test]
    fn whole_weeks_zero() {
        let _rug_st_tests_llm_16_325_rrrruuuugggg_whole_weeks_zero = 0;
        debug_assert_eq!(Duration::ZERO.whole_weeks(), 0);
        let _rug_ed_tests_llm_16_325_rrrruuuugggg_whole_weeks_zero = 0;
    }
    #[test]
    fn whole_weeks_single_week() {
        let _rug_st_tests_llm_16_325_rrrruuuugggg_whole_weeks_single_week = 0;
        debug_assert_eq!(Duration::WEEK.whole_weeks(), 1);
        debug_assert_eq!((- Duration::WEEK).whole_weeks(), - 1);
        let _rug_ed_tests_llm_16_325_rrrruuuugggg_whole_weeks_single_week = 0;
    }
    #[test]
    fn whole_weeks_multiple_weeks() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Duration::WEEK.whole_weeks() * rug_fuzz_0, 7);
        debug_assert_eq!((- Duration::WEEK).whole_weeks() * rug_fuzz_1, - 7);
             }
}
}
}    }
    #[test]
    fn whole_weeks_days() {
        let _rug_st_tests_llm_16_325_rrrruuuugggg_whole_weeks_days = 0;
        debug_assert_eq!(Duration::DAY.whole_weeks(), 0);
        debug_assert_eq!((- Duration::DAY).whole_weeks(), 0);
        let _rug_ed_tests_llm_16_325_rrrruuuugggg_whole_weeks_days = 0;
    }
    #[test]
    fn whole_weeks_days_multiple() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Duration::DAY.whole_weeks() * rug_fuzz_0, 0);
        debug_assert_eq!((- Duration::DAY).whole_weeks() * rug_fuzz_1, 0);
             }
}
}
}    }
    #[test]
    fn whole_weeks_less_than_week() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(rug_fuzz_0.days().whole_weeks(), 0);
        debug_assert_eq!((- rug_fuzz_1).days().whole_weeks(), 0);
             }
}
}
}    }
    #[test]
    fn whole_weeks_more_than_week() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(rug_fuzz_0.days().whole_weeks(), 1);
        debug_assert_eq!((- rug_fuzz_1).days().whole_weeks(), - 1);
             }
}
}
}    }
    #[test]
    fn whole_weeks_mixed() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i64, i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!((rug_fuzz_0.weeks() + rug_fuzz_1.days()).whole_weeks(), 1);
        debug_assert_eq!((rug_fuzz_2.weeks() - rug_fuzz_3.days()).whole_weeks(), 0);
             }
}
}
}    }
    #[test]
    fn whole_weeks_extremes() {
        let _rug_st_tests_llm_16_325_rrrruuuugggg_whole_weeks_extremes = 0;
        debug_assert_eq!(
            Duration::MIN.whole_weeks(), Duration::MIN.whole_seconds() / 604800
        );
        debug_assert_eq!(
            Duration::MAX.whole_weeks(), Duration::MAX.whole_seconds() / 604800
        );
        let _rug_ed_tests_llm_16_325_rrrruuuugggg_whole_weeks_extremes = 0;
    }
}
#[cfg(test)]
mod tests_rug_148 {
    use crate::Duration;
    use std::time::Duration as StdDuration;
    use crate::ext::NumericalDuration;
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Duration::minutes(rug_fuzz_0);
        debug_assert_eq!(Duration::unsigned_abs(p0), StdDuration::from_secs(5 * 60));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_149 {
    use super::*;
    #[test]
    fn test_seconds_f32() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: f32 = rug_fuzz_0;
        debug_assert_eq!(< Duration > ::seconds_f32(p0), Duration::seconds(1) / 2);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_150 {
    use crate::Duration;
    #[test]
    fn test_subsec_nanoseconds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: Duration = Duration::minutes(rug_fuzz_0);
        debug_assert_eq!(Duration::subsec_nanoseconds(p0), 0);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_151 {
    use super::*;
    use crate::{Duration, ext::NumericalDuration};
    #[test]
    fn test_saturating_mul() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i64, i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0 = Duration::minutes(rug_fuzz_0);
        let p1: i32 = rug_fuzz_1;
        debug_assert_eq!(p0.saturating_mul(p1), Duration::minutes(10));
        debug_assert_eq!(p0.saturating_mul(rug_fuzz_2), Duration::ZERO);
        debug_assert_eq!(Duration::MAX.saturating_mul(p1), Duration::MAX);
        debug_assert_eq!(Duration::MIN.saturating_mul(p1), Duration::MIN);
        debug_assert_eq!(Duration::MAX.saturating_mul(- rug_fuzz_3), Duration::MIN);
        debug_assert_eq!(Duration::MIN.saturating_mul(- rug_fuzz_4), Duration::MAX);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_152 {
    use super::*;
    use crate::duration::Duration;
    use std::time::Instant;
    #[test]
    fn test_duration_time_fn() {
        let _rug_st_tests_rug_152_rrrruuuugggg_test_duration_time_fn = 0;
        struct A;
        struct B;
        struct Function;
        pub struct ConstFnMutClosure<T, F>(T, F);
        impl<T, F> ConstFnMutClosure<T, F> {
            pub fn new(t: T, f: F) -> Self {
                Self(t, f)
            }
        }
        let mut a = A;
        let mut b = B;
        let function = Function;
        let mut p0 = ConstFnMutClosure::new((&mut a, &mut b), function);
        let _result = Duration::time_fn(|| p0);
        let _rug_ed_tests_rug_152_rrrruuuugggg_test_duration_time_fn = 0;
    }
}
#[cfg(test)]
mod tests_rug_153 {
    use std::convert::TryFrom;
    use std::time::Duration as StdDuration;
    use crate::duration::Duration;
    use crate::error;
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: StdDuration = StdDuration::new(rug_fuzz_0, rug_fuzz_1);
        debug_assert!(Duration::try_from(p0).is_ok());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_154 {
    use super::*;
    use std::convert::TryFrom;
    use crate::{Duration, error::ConversionRange};
    #[test]
    fn test_try_from() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Duration::minutes(rug_fuzz_0);
        let result = std::time::Duration::try_from(p0);
        debug_assert!(result.is_ok());
        debug_assert_eq!(result.unwrap(), std::time::Duration::new(300, 0));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_155 {
    use super::*;
    use std::ops::Add;
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Duration::minutes(rug_fuzz_0);
        let mut p1 = Duration::minutes(rug_fuzz_1);
        <Duration as Add>::add(p0, p1);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_156 {
    use crate::Duration;
    use std::time::Duration as StdDuration;
    use std::ops::Add;
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, u64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Duration::minutes(rug_fuzz_0);
        let mut p1 = StdDuration::new(rug_fuzz_1, rug_fuzz_2);
        <Duration as Add<StdDuration>>::add(p0, p1);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_157 {
    use std::ops::Add;
    use std::time::Duration;
    use crate::duration::Duration as TimeDuration;
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u64, u32, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Duration::new(rug_fuzz_0, rug_fuzz_1);
        let mut p1 = TimeDuration::minutes(rug_fuzz_2);
        <Duration as Add<TimeDuration>>::add(p0, p1);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_158 {
    use super::*;
    use std::ops::AddAssign;
    use std::time::Duration as StdDuration;
    use crate::Duration as TimeDuration;
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u64, u32, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: StdDuration = StdDuration::new(rug_fuzz_0, rug_fuzz_1);
        let p1: TimeDuration = TimeDuration::minutes(rug_fuzz_2);
        StdDuration::add_assign(&mut p0, p1);
        debug_assert_eq!(p0, StdDuration::new(305, 0));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_159 {
    use super::*;
    use std::ops::Sub;
    use std::time::Duration as StdDuration;
    use crate::Duration;
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, u64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Duration::minutes(rug_fuzz_0);
        let mut p1 = StdDuration::new(rug_fuzz_1, rug_fuzz_2);
        <Duration as Sub<StdDuration>>::sub(p0, p1);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_160 {
    use super::*;
    use std::ops::Sub;
    use std::time::Duration as StdDuration;
    use crate::Duration;
    #[test]
    fn test_sub() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u64, u32, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = StdDuration::new(rug_fuzz_0, rug_fuzz_1);
        let mut p1 = Duration::minutes(rug_fuzz_2);
        let _result = StdDuration::sub(p0, p1);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_161 {
    use std::ops::SubAssign;
    use std::time::Duration as StdDuration;
    use crate::Duration as TimeDuration;
    #[test]
    fn test_sub_assign() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u64, u32, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: StdDuration = StdDuration::new(rug_fuzz_0, rug_fuzz_1);
        let p1: TimeDuration = TimeDuration::minutes(rug_fuzz_2);
        StdDuration::sub_assign(&mut p0, p1);
        debug_assert_eq!(p0, StdDuration::new(2, 0));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_162 {
    use super::*;
    use std::ops::Div;
    use crate::Duration;
    #[test]
    fn test_div() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: Duration = Duration::minutes(rug_fuzz_0);
        let mut p1: i32 = rug_fuzz_1;
        let result = <Duration as Div<i32>>::div(p0, p1);
        debug_assert_eq!(result, Duration::minutes(5 / 2));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_163 {
    use super::*;
    use std::ops::Mul;
    use crate::Duration;
    #[test]
    fn test_mul() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f32, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: f32 = rug_fuzz_0;
        let mut p1 = Duration::minutes(rug_fuzz_1);
        <f32 as Mul<Duration>>::mul(p0, p1);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_164 {
    use super::*;
    use std::ops::Div;
    use std::time::Duration as StdDuration;
    use crate::Duration;
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, u64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Duration::minutes(rug_fuzz_0);
        let mut p1 = StdDuration::new(rug_fuzz_1, rug_fuzz_2);
        let result = Duration::div(p0, p1);
        debug_assert!(result.is_finite());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_165 {
    use std::ops::Div;
    use std::time::Duration as StdDuration;
    use crate::Duration as TimeDuration;
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u64, u32, i64, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: StdDuration = StdDuration::new(rug_fuzz_0, rug_fuzz_1);
        let mut p1: TimeDuration = TimeDuration::minutes(rug_fuzz_2);
        let result = <StdDuration as Div<TimeDuration>>::div(p0, p1);
        debug_assert!(result > rug_fuzz_3);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_166 {
    use super::*;
    use std::cmp::PartialEq;
    use std::time::Duration as StdDuration;
    use crate::Duration;
    #[test]
    fn test_eq() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, u64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0 = Duration::minutes(rug_fuzz_0);
        let p1 = StdDuration::new(rug_fuzz_1, rug_fuzz_2);
        debug_assert!(< Duration as PartialEq < StdDuration > > ::eq(& p0, & p1));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_167 {
    use super::*;
    use std::time::Duration as StdDuration;
    use crate::Duration;
    #[test]
    fn test_eq() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u64, u32, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: StdDuration = StdDuration::new(rug_fuzz_0, rug_fuzz_1);
        let p1: Duration = Duration::minutes(rug_fuzz_2);
        debug_assert!(< StdDuration as PartialEq < Duration > > ::eq(& p0, & p1));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_168 {
    use super::*;
    use std::cmp::{Ordering, PartialOrd};
    use std::time::Duration as StdDuration;
    use crate::Duration;
    #[test]
    fn test_partial_cmp() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, u64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Duration::minutes(rug_fuzz_0);
        let mut p1 = StdDuration::new(rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(
            < Duration as PartialOrd < StdDuration > > ::partial_cmp(& p0, & p1),
            Some(Ordering::Equal)
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_169 {
    use super::*;
    use std::cmp::Ordering;
    use std::time::Duration;
    use crate::duration;
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u64, u32, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Duration::new(rug_fuzz_0, rug_fuzz_1);
        let mut p1 = duration::Duration::minutes(rug_fuzz_2);
        let result = <Duration as PartialOrd<duration::Duration>>::partial_cmp(&p0, &p1);
        debug_assert_eq!(result, Some(Ordering::Equal));
             }
}
}
}    }
}
