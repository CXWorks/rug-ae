//! ISO 8601 time without timezone.
#[cfg(any(feature = "alloc", feature = "std", test))]
use core::borrow::Borrow;
use core::ops::{Add, AddAssign, Sub, SubAssign};
use core::{fmt, str};
#[cfg(feature = "rkyv")]
use rkyv::{Archive, Deserialize, Serialize};
#[cfg(any(feature = "alloc", feature = "std", test))]
use crate::format::DelayedFormat;
use crate::format::{
    parse, write_hundreds, ParseError, ParseResult, Parsed, StrftimeItems,
};
use crate::format::{Fixed, Item, Numeric, Pad};
use crate::{TimeDelta, Timelike};
#[cfg(feature = "serde")]
mod serde;
#[cfg(test)]
mod tests;
/// ISO 8601 time without timezone.
/// Allows for the nanosecond precision and optional leap second representation.
///
/// # Leap Second Handling
///
/// Since 1960s, the manmade atomic clock has been so accurate that
/// it is much more accurate than Earth's own motion.
/// It became desirable to define the civil time in terms of the atomic clock,
/// but that risks the desynchronization of the civil time from Earth.
/// To account for this, the designers of the Coordinated Universal Time (UTC)
/// made that the UTC should be kept within 0.9 seconds of the observed Earth-bound time.
/// When the mean solar day is longer than the ideal (86,400 seconds),
/// the error slowly accumulates and it is necessary to add a **leap second**
/// to slow the UTC down a bit.
/// (We may also remove a second to speed the UTC up a bit, but it never happened.)
/// The leap second, if any, follows 23:59:59 of June 30 or December 31 in the UTC.
///
/// Fast forward to the 21st century,
/// we have seen 26 leap seconds from January 1972 to December 2015.
/// Yes, 26 seconds. Probably you can read this paragraph within 26 seconds.
/// But those 26 seconds, and possibly more in the future, are never predictable,
/// and whether to add a leap second or not is known only before 6 months.
/// Internet-based clocks (via NTP) do account for known leap seconds,
/// but the system API normally doesn't (and often can't, with no network connection)
/// and there is no reliable way to retrieve leap second information.
///
/// Chrono does not try to accurately implement leap seconds; it is impossible.
/// Rather, **it allows for leap seconds but behaves as if there are *no other* leap seconds.**
/// Various operations will ignore any possible leap second(s)
/// except when any of the operands were actually leap seconds.
///
/// If you cannot tolerate this behavior,
/// you must use a separate `TimeZone` for the International Atomic Time (TAI).
/// TAI is like UTC but has no leap seconds, and thus slightly differs from UTC.
/// Chrono does not yet provide such implementation, but it is planned.
///
/// ## Representing Leap Seconds
///
/// The leap second is indicated via fractional seconds more than 1 second.
/// This makes possible to treat a leap second as the prior non-leap second
/// if you don't care about sub-second accuracy.
/// You should use the proper formatting to get the raw leap second.
///
/// All methods accepting fractional seconds will accept such values.
///
/// ```
/// use chrono::{NaiveDate, NaiveTime, Utc, TimeZone};
///
/// let t = NaiveTime::from_hms_milli_opt(8, 59, 59, 1_000).unwrap();
///
/// let dt1 = NaiveDate::from_ymd_opt(2015, 7, 1).unwrap().and_hms_micro_opt(8, 59, 59, 1_000_000).unwrap();
///
/// let dt2 = NaiveDate::from_ymd_opt(2015, 6, 30).unwrap().and_hms_nano_opt(23, 59, 59, 1_000_000_000).unwrap().and_local_timezone(Utc).unwrap();
/// # let _ = (t, dt1, dt2);
/// ```
///
/// Note that the leap second can happen anytime given an appropriate time zone;
/// 2015-07-01 01:23:60 would be a proper leap second if UTC+01:24 had existed.
/// Practically speaking, though, by the time of the first leap second on 1972-06-30,
/// every time zone offset around the world has standardized to the 5-minute alignment.
///
/// ## Date And Time Arithmetics
///
/// As a concrete example, let's assume that `03:00:60` and `04:00:60` are leap seconds.
/// In reality, of course, leap seconds are separated by at least 6 months.
/// We will also use some intuitive concise notations for the explanation.
///
/// `Time + TimeDelta`
/// (short for [`NaiveTime::overflowing_add_signed`](#method.overflowing_add_signed)):
///
/// - `03:00:00 + 1s = 03:00:01`.
/// - `03:00:59 + 60s = 03:02:00`.
/// - `03:00:59 + 1s = 03:01:00`.
/// - `03:00:60 + 1s = 03:01:00`.
///   Note that the sum is identical to the previous.
/// - `03:00:60 + 60s = 03:01:59`.
/// - `03:00:60 + 61s = 03:02:00`.
/// - `03:00:60.1 + 0.8s = 03:00:60.9`.
///
/// `Time - TimeDelta`
/// (short for [`NaiveTime::overflowing_sub_signed`](#method.overflowing_sub_signed)):
///
/// - `03:00:00 - 1s = 02:59:59`.
/// - `03:01:00 - 1s = 03:00:59`.
/// - `03:01:00 - 60s = 03:00:00`.
/// - `03:00:60 - 60s = 03:00:00`.
///   Note that the result is identical to the previous.
/// - `03:00:60.7 - 0.4s = 03:00:60.3`.
/// - `03:00:60.7 - 0.9s = 03:00:59.8`.
///
/// `Time - Time`
/// (short for [`NaiveTime::signed_duration_since`](#method.signed_duration_since)):
///
/// - `04:00:00 - 03:00:00 = 3600s`.
/// - `03:01:00 - 03:00:00 = 60s`.
/// - `03:00:60 - 03:00:00 = 60s`.
///   Note that the difference is identical to the previous.
/// - `03:00:60.6 - 03:00:59.4 = 1.2s`.
/// - `03:01:00 - 03:00:59.8 = 0.2s`.
/// - `03:01:00 - 03:00:60.5 = 0.5s`.
///   Note that the difference is larger than the previous,
///   even though the leap second clearly follows the previous whole second.
/// - `04:00:60.9 - 03:00:60.1 =
///   (04:00:60.9 - 04:00:00) + (04:00:00 - 03:01:00) + (03:01:00 - 03:00:60.1) =
///   60.9s + 3540s + 0.9s = 3601.8s`.
///
/// In general,
///
/// - `Time + TimeDelta` unconditionally equals to `TimeDelta + Time`.
///
/// - `Time - TimeDelta` unconditionally equals to `Time + (-TimeDelta)`.
///
/// - `Time1 - Time2` unconditionally equals to `-(Time2 - Time1)`.
///
/// - Associativity does not generally hold, because
///   `(Time + TimeDelta1) - TimeDelta2` no longer equals to `Time + (TimeDelta1 - TimeDelta2)`
///   for two positive durations.
///
///     - As a special case, `(Time + TimeDelta) - TimeDelta` also does not equal to `Time`.
///
///     - If you can assume that all durations have the same sign, however,
///       then the associativity holds:
///       `(Time + TimeDelta1) + TimeDelta2` equals to `Time + (TimeDelta1 + TimeDelta2)`
///       for two positive durations.
///
/// ## Reading And Writing Leap Seconds
///
/// The "typical" leap seconds on the minute boundary are
/// correctly handled both in the formatting and parsing.
/// The leap second in the human-readable representation
/// will be represented as the second part being 60, as required by ISO 8601.
///
/// ```
/// use chrono::{Utc, TimeZone, NaiveDate};
///
/// let dt = NaiveDate::from_ymd_opt(2015, 6, 30).unwrap().and_hms_milli_opt(23, 59, 59, 1_000).unwrap().and_local_timezone(Utc).unwrap();
/// assert_eq!(format!("{:?}", dt), "2015-06-30T23:59:60Z");
/// ```
///
/// There are hypothetical leap seconds not on the minute boundary
/// nevertheless supported by Chrono.
/// They are allowed for the sake of completeness and consistency;
/// there were several "exotic" time zone offsets with fractional minutes prior to UTC after all.
/// For such cases the human-readable representation is ambiguous
/// and would be read back to the next non-leap second.
///
/// ```
/// use chrono::{DateTime, Utc, TimeZone, NaiveDate};
///
/// let dt = NaiveDate::from_ymd_opt(2015, 6, 30).unwrap().and_hms_milli_opt(23, 56, 4, 1_000).unwrap().and_local_timezone(Utc).unwrap();
/// assert_eq!(format!("{:?}", dt), "2015-06-30T23:56:05Z");
///
/// let dt = Utc.with_ymd_and_hms(2015, 6, 30, 23, 56, 5).unwrap();
/// assert_eq!(format!("{:?}", dt), "2015-06-30T23:56:05Z");
/// assert_eq!(DateTime::<Utc>::parse_from_rfc3339("2015-06-30T23:56:05Z").unwrap(), dt);
/// ```
///
/// Since Chrono alone cannot determine any existence of leap seconds,
/// **there is absolutely no guarantee that the leap second read has actually happened**.
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Clone)]
#[cfg_attr(feature = "rkyv", derive(Archive, Deserialize, Serialize))]
pub struct NaiveTime {
    secs: u32,
    frac: u32,
}
#[cfg(feature = "arbitrary")]
impl arbitrary::Arbitrary<'_> for NaiveTime {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<NaiveTime> {
        let secs = u.int_in_range(0..=86_399)?;
        let nano = u.int_in_range(0..=1_999_999_999)?;
        let time = NaiveTime::from_num_seconds_from_midnight_opt(secs, nano)
            .expect(
                "Could not generate a valid chrono::NaiveTime. It looks like implementation of Arbitrary for NaiveTime is erroneous.",
            );
        Ok(time)
    }
}
impl NaiveTime {
    /// Makes a new `NaiveTime` from hour, minute and second.
    ///
    /// No [leap second](#leap-second-handling) is allowed here;
    /// use `NaiveTime::from_hms_*` methods with a subsecond parameter instead.
    ///
    /// Panics on invalid hour, minute and/or second.
    #[deprecated(since = "0.4.23", note = "use `from_hms_opt()` instead")]
    #[inline]
    #[must_use]
    pub fn from_hms(hour: u32, min: u32, sec: u32) -> NaiveTime {
        NaiveTime::from_hms_opt(hour, min, sec).expect("invalid time")
    }
    /// Makes a new `NaiveTime` from hour, minute and second.
    ///
    /// No [leap second](#leap-second-handling) is allowed here;
    /// use `NaiveTime::from_hms_*_opt` methods with a subsecond parameter instead.
    ///
    /// Returns `None` on invalid hour, minute and/or second.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::NaiveTime;
    ///
    /// let from_hms_opt = NaiveTime::from_hms_opt;
    ///
    /// assert!(from_hms_opt(0, 0, 0).is_some());
    /// assert!(from_hms_opt(23, 59, 59).is_some());
    /// assert!(from_hms_opt(24, 0, 0).is_none());
    /// assert!(from_hms_opt(23, 60, 0).is_none());
    /// assert!(from_hms_opt(23, 59, 60).is_none());
    /// ```
    #[inline]
    #[must_use]
    pub const fn from_hms_opt(hour: u32, min: u32, sec: u32) -> Option<NaiveTime> {
        NaiveTime::from_hms_nano_opt(hour, min, sec, 0)
    }
    /// Makes a new `NaiveTime` from hour, minute, second and millisecond.
    ///
    /// The millisecond part can exceed 1,000
    /// in order to represent the [leap second](#leap-second-handling).
    ///
    /// Panics on invalid hour, minute, second and/or millisecond.
    #[deprecated(since = "0.4.23", note = "use `from_hms_milli_opt()` instead")]
    #[inline]
    #[must_use]
    pub fn from_hms_milli(hour: u32, min: u32, sec: u32, milli: u32) -> NaiveTime {
        NaiveTime::from_hms_milli_opt(hour, min, sec, milli).expect("invalid time")
    }
    /// Makes a new `NaiveTime` from hour, minute, second and millisecond.
    ///
    /// The millisecond part can exceed 1,000
    /// in order to represent the [leap second](#leap-second-handling).
    ///
    /// Returns `None` on invalid hour, minute, second and/or millisecond.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::NaiveTime;
    ///
    /// let from_hmsm_opt = NaiveTime::from_hms_milli_opt;
    ///
    /// assert!(from_hmsm_opt(0, 0, 0, 0).is_some());
    /// assert!(from_hmsm_opt(23, 59, 59, 999).is_some());
    /// assert!(from_hmsm_opt(23, 59, 59, 1_999).is_some()); // a leap second after 23:59:59
    /// assert!(from_hmsm_opt(24, 0, 0, 0).is_none());
    /// assert!(from_hmsm_opt(23, 60, 0, 0).is_none());
    /// assert!(from_hmsm_opt(23, 59, 60, 0).is_none());
    /// assert!(from_hmsm_opt(23, 59, 59, 2_000).is_none());
    /// ```
    #[inline]
    #[must_use]
    pub fn from_hms_milli_opt(
        hour: u32,
        min: u32,
        sec: u32,
        milli: u32,
    ) -> Option<NaiveTime> {
        milli
            .checked_mul(1_000_000)
            .and_then(|nano| NaiveTime::from_hms_nano_opt(hour, min, sec, nano))
    }
    /// Makes a new `NaiveTime` from hour, minute, second and microsecond.
    ///
    /// The microsecond part can exceed 1,000,000
    /// in order to represent the [leap second](#leap-second-handling).
    ///
    /// Panics on invalid hour, minute, second and/or microsecond.
    #[deprecated(since = "0.4.23", note = "use `from_hms_micro_opt()` instead")]
    #[inline]
    #[must_use]
    pub fn from_hms_micro(hour: u32, min: u32, sec: u32, micro: u32) -> NaiveTime {
        NaiveTime::from_hms_micro_opt(hour, min, sec, micro).expect("invalid time")
    }
    /// Makes a new `NaiveTime` from hour, minute, second and microsecond.
    ///
    /// The microsecond part can exceed 1,000,000
    /// in order to represent the [leap second](#leap-second-handling).
    ///
    /// Returns `None` on invalid hour, minute, second and/or microsecond.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::NaiveTime;
    ///
    /// let from_hmsu_opt = NaiveTime::from_hms_micro_opt;
    ///
    /// assert!(from_hmsu_opt(0, 0, 0, 0).is_some());
    /// assert!(from_hmsu_opt(23, 59, 59, 999_999).is_some());
    /// assert!(from_hmsu_opt(23, 59, 59, 1_999_999).is_some()); // a leap second after 23:59:59
    /// assert!(from_hmsu_opt(24, 0, 0, 0).is_none());
    /// assert!(from_hmsu_opt(23, 60, 0, 0).is_none());
    /// assert!(from_hmsu_opt(23, 59, 60, 0).is_none());
    /// assert!(from_hmsu_opt(23, 59, 59, 2_000_000).is_none());
    /// ```
    #[inline]
    #[must_use]
    pub fn from_hms_micro_opt(
        hour: u32,
        min: u32,
        sec: u32,
        micro: u32,
    ) -> Option<NaiveTime> {
        micro
            .checked_mul(1_000)
            .and_then(|nano| NaiveTime::from_hms_nano_opt(hour, min, sec, nano))
    }
    /// Makes a new `NaiveTime` from hour, minute, second and nanosecond.
    ///
    /// The nanosecond part can exceed 1,000,000,000
    /// in order to represent the [leap second](#leap-second-handling).
    ///
    /// Panics on invalid hour, minute, second and/or nanosecond.
    #[deprecated(since = "0.4.23", note = "use `from_hms_nano_opt()` instead")]
    #[inline]
    #[must_use]
    pub fn from_hms_nano(hour: u32, min: u32, sec: u32, nano: u32) -> NaiveTime {
        NaiveTime::from_hms_nano_opt(hour, min, sec, nano).expect("invalid time")
    }
    /// Makes a new `NaiveTime` from hour, minute, second and nanosecond.
    ///
    /// The nanosecond part can exceed 1,000,000,000
    /// in order to represent the [leap second](#leap-second-handling).
    ///
    /// Returns `None` on invalid hour, minute, second and/or nanosecond.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::NaiveTime;
    ///
    /// let from_hmsn_opt = NaiveTime::from_hms_nano_opt;
    ///
    /// assert!(from_hmsn_opt(0, 0, 0, 0).is_some());
    /// assert!(from_hmsn_opt(23, 59, 59, 999_999_999).is_some());
    /// assert!(from_hmsn_opt(23, 59, 59, 1_999_999_999).is_some()); // a leap second after 23:59:59
    /// assert!(from_hmsn_opt(24, 0, 0, 0).is_none());
    /// assert!(from_hmsn_opt(23, 60, 0, 0).is_none());
    /// assert!(from_hmsn_opt(23, 59, 60, 0).is_none());
    /// assert!(from_hmsn_opt(23, 59, 59, 2_000_000_000).is_none());
    /// ```
    #[inline]
    #[must_use]
    pub const fn from_hms_nano_opt(
        hour: u32,
        min: u32,
        sec: u32,
        nano: u32,
    ) -> Option<NaiveTime> {
        if hour >= 24 || min >= 60 || sec >= 60 || nano >= 2_000_000_000 {
            return None;
        }
        let secs = hour * 3600 + min * 60 + sec;
        Some(NaiveTime { secs, frac: nano })
    }
    /// Makes a new `NaiveTime` from the number of seconds since midnight and nanosecond.
    ///
    /// The nanosecond part can exceed 1,000,000,000
    /// in order to represent the [leap second](#leap-second-handling).
    ///
    /// Panics on invalid number of seconds and/or nanosecond.
    #[deprecated(
        since = "0.4.23",
        note = "use `from_num_seconds_from_midnight_opt()` instead"
    )]
    #[inline]
    #[must_use]
    pub fn from_num_seconds_from_midnight(secs: u32, nano: u32) -> NaiveTime {
        NaiveTime::from_num_seconds_from_midnight_opt(secs, nano).expect("invalid time")
    }
    /// Makes a new `NaiveTime` from the number of seconds since midnight and nanosecond.
    ///
    /// The nanosecond part can exceed 1,000,000,000
    /// in order to represent the [leap second](#leap-second-handling).
    ///
    /// Returns `None` on invalid number of seconds and/or nanosecond.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::NaiveTime;
    ///
    /// let from_nsecs_opt = NaiveTime::from_num_seconds_from_midnight_opt;
    ///
    /// assert!(from_nsecs_opt(0, 0).is_some());
    /// assert!(from_nsecs_opt(86399, 999_999_999).is_some());
    /// assert!(from_nsecs_opt(86399, 1_999_999_999).is_some()); // a leap second after 23:59:59
    /// assert!(from_nsecs_opt(86_400, 0).is_none());
    /// assert!(from_nsecs_opt(86399, 2_000_000_000).is_none());
    /// ```
    #[inline]
    #[must_use]
    pub const fn from_num_seconds_from_midnight_opt(
        secs: u32,
        nano: u32,
    ) -> Option<NaiveTime> {
        if secs >= 86_400 || nano >= 2_000_000_000 {
            return None;
        }
        Some(NaiveTime { secs, frac: nano })
    }
    /// Parses a string with the specified format string and returns a new `NaiveTime`.
    /// See the [`format::strftime` module](../format/strftime/index.html)
    /// on the supported escape sequences.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::NaiveTime;
    ///
    /// let parse_from_str = NaiveTime::parse_from_str;
    ///
    /// assert_eq!(parse_from_str("23:56:04", "%H:%M:%S"),
    ///            Ok(NaiveTime::from_hms_opt(23, 56, 4).unwrap()));
    /// assert_eq!(parse_from_str("pm012345.6789", "%p%I%M%S%.f"),
    ///            Ok(NaiveTime::from_hms_micro_opt(13, 23, 45, 678_900).unwrap()));
    /// ```
    ///
    /// Date and offset is ignored for the purpose of parsing.
    ///
    /// ```
    /// # use chrono::NaiveTime;
    /// # let parse_from_str = NaiveTime::parse_from_str;
    /// assert_eq!(parse_from_str("2014-5-17T12:34:56+09:30", "%Y-%m-%dT%H:%M:%S%z"),
    ///            Ok(NaiveTime::from_hms_opt(12, 34, 56).unwrap()));
    /// ```
    ///
    /// [Leap seconds](#leap-second-handling) are correctly handled by
    /// treating any time of the form `hh:mm:60` as a leap second.
    /// (This equally applies to the formatting, so the round trip is possible.)
    ///
    /// ```
    /// # use chrono::NaiveTime;
    /// # let parse_from_str = NaiveTime::parse_from_str;
    /// assert_eq!(parse_from_str("08:59:60.123", "%H:%M:%S%.f"),
    ///            Ok(NaiveTime::from_hms_milli_opt(8, 59, 59, 1_123).unwrap()));
    /// ```
    ///
    /// Missing seconds are assumed to be zero,
    /// but out-of-bound times or insufficient fields are errors otherwise.
    ///
    /// ```
    /// # use chrono::NaiveTime;
    /// # let parse_from_str = NaiveTime::parse_from_str;
    /// assert_eq!(parse_from_str("7:15", "%H:%M"),
    ///            Ok(NaiveTime::from_hms_opt(7, 15, 0).unwrap()));
    ///
    /// assert!(parse_from_str("04m33s", "%Mm%Ss").is_err());
    /// assert!(parse_from_str("12", "%H").is_err());
    /// assert!(parse_from_str("17:60", "%H:%M").is_err());
    /// assert!(parse_from_str("24:00:00", "%H:%M:%S").is_err());
    /// ```
    ///
    /// All parsed fields should be consistent to each other, otherwise it's an error.
    /// Here `%H` is for 24-hour clocks, unlike `%I`,
    /// and thus can be independently determined without AM/PM.
    ///
    /// ```
    /// # use chrono::NaiveTime;
    /// # let parse_from_str = NaiveTime::parse_from_str;
    /// assert!(parse_from_str("13:07 AM", "%H:%M %p").is_err());
    /// ```
    pub fn parse_from_str(s: &str, fmt: &str) -> ParseResult<NaiveTime> {
        let mut parsed = Parsed::new();
        parse(&mut parsed, s, StrftimeItems::new(fmt))?;
        parsed.to_naive_time()
    }
    /// Adds given `TimeDelta` to the current time,
    /// and also returns the number of *seconds*
    /// in the integral number of days ignored from the addition.
    /// (We cannot return `TimeDelta` because it is subject to overflow or underflow.)
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{TimeDelta, NaiveTime};
    ///
    /// let from_hms = NaiveTime::from_hms;
    ///
    /// assert_eq!(from_hms(3, 4, 5).overflowing_add_signed(TimeDelta::hours(11)),
    ///            (from_hms(14, 4, 5), 0));
    /// assert_eq!(from_hms(3, 4, 5).overflowing_add_signed(TimeDelta::hours(23)),
    ///            (from_hms(2, 4, 5), 86_400));
    /// assert_eq!(from_hms(3, 4, 5).overflowing_add_signed(TimeDelta::hours(-7)),
    ///            (from_hms(20, 4, 5), -86_400));
    /// ```
    #[must_use]
    pub fn overflowing_add_signed(&self, mut rhs: TimeDelta) -> (NaiveTime, i64) {
        let mut secs = self.secs;
        let mut frac = self.frac;
        if frac >= 1_000_000_000 {
            let rfrac = 2_000_000_000 - frac;
            if rhs >= TimeDelta::nanoseconds(i64::from(rfrac)) {
                rhs = rhs - TimeDelta::nanoseconds(i64::from(rfrac));
                secs += 1;
                frac = 0;
            } else if rhs < TimeDelta::nanoseconds(-i64::from(frac)) {
                rhs = rhs + TimeDelta::nanoseconds(i64::from(frac));
                frac = 0;
            } else {
                frac = (i64::from(frac) + rhs.num_nanoseconds().unwrap()) as u32;
                debug_assert!(frac < 2_000_000_000);
                return (NaiveTime { secs, frac }, 0);
            }
        }
        debug_assert!(secs <= 86_400);
        debug_assert!(frac < 1_000_000_000);
        let rhssecs = rhs.num_seconds();
        let rhsfrac = (rhs - TimeDelta::seconds(rhssecs)).num_nanoseconds().unwrap();
        debug_assert_eq!(
            TimeDelta::seconds(rhssecs) + TimeDelta::nanoseconds(rhsfrac), rhs
        );
        let rhssecsinday = rhssecs % 86_400;
        let mut morerhssecs = rhssecs - rhssecsinday;
        let rhssecs = rhssecsinday as i32;
        let rhsfrac = rhsfrac as i32;
        debug_assert!(- 86_400 < rhssecs && rhssecs < 86_400);
        debug_assert_eq!(morerhssecs % 86_400, 0);
        debug_assert!(- 1_000_000_000 < rhsfrac && rhsfrac < 1_000_000_000);
        let mut secs = secs as i32 + rhssecs;
        let mut frac = frac as i32 + rhsfrac;
        debug_assert!(- 86_400 < secs && secs < 2 * 86_400);
        debug_assert!(- 1_000_000_000 < frac && frac < 2_000_000_000);
        if frac < 0 {
            frac += 1_000_000_000;
            secs -= 1;
        } else if frac >= 1_000_000_000 {
            frac -= 1_000_000_000;
            secs += 1;
        }
        debug_assert!((- 86_400..2 * 86_400).contains(& secs));
        debug_assert!((0..1_000_000_000).contains(& frac));
        if secs < 0 {
            secs += 86_400;
            morerhssecs -= 86_400;
        } else if secs >= 86_400 {
            secs -= 86_400;
            morerhssecs += 86_400;
        }
        debug_assert!((0..86_400).contains(& secs));
        (
            NaiveTime {
                secs: secs as u32,
                frac: frac as u32,
            },
            morerhssecs,
        )
    }
    /// Subtracts given `TimeDelta` from the current time,
    /// and also returns the number of *seconds*
    /// in the integral number of days ignored from the subtraction.
    /// (We cannot return `TimeDelta` because it is subject to overflow or underflow.)
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{TimeDelta, NaiveTime};
    ///
    /// let from_hms = NaiveTime::from_hms;
    ///
    /// assert_eq!(from_hms(3, 4, 5).overflowing_sub_signed(TimeDelta::hours(2)),
    ///            (from_hms(1, 4, 5), 0));
    /// assert_eq!(from_hms(3, 4, 5).overflowing_sub_signed(TimeDelta::hours(17)),
    ///            (from_hms(10, 4, 5), 86_400));
    /// assert_eq!(from_hms(3, 4, 5).overflowing_sub_signed(TimeDelta::hours(-22)),
    ///            (from_hms(1, 4, 5), -86_400));
    /// ```
    #[inline]
    #[must_use]
    pub fn overflowing_sub_signed(&self, rhs: TimeDelta) -> (NaiveTime, i64) {
        let (time, rhs) = self.overflowing_add_signed(-rhs);
        (time, -rhs)
    }
    /// Subtracts another `NaiveTime` from the current time.
    /// Returns a `TimeDelta` within +/- 1 day.
    /// This does not overflow or underflow at all.
    ///
    /// As a part of Chrono's [leap second handling](#leap-second-handling),
    /// the subtraction assumes that **there is no leap second ever**,
    /// except when any of the `NaiveTime`s themselves represents a leap second
    /// in which case the assumption becomes that
    /// **there are exactly one (or two) leap second(s) ever**.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{TimeDelta, NaiveTime};
    ///
    /// let from_hmsm = NaiveTime::from_hms_milli;
    /// let since = NaiveTime::signed_duration_since;
    ///
    /// assert_eq!(since(from_hmsm(3, 5, 7, 900), from_hmsm(3, 5, 7, 900)),
    ///            TimeDelta::zero());
    /// assert_eq!(since(from_hmsm(3, 5, 7, 900), from_hmsm(3, 5, 7, 875)),
    ///            TimeDelta::milliseconds(25));
    /// assert_eq!(since(from_hmsm(3, 5, 7, 900), from_hmsm(3, 5, 6, 925)),
    ///            TimeDelta::milliseconds(975));
    /// assert_eq!(since(from_hmsm(3, 5, 7, 900), from_hmsm(3, 5, 0, 900)),
    ///            TimeDelta::seconds(7));
    /// assert_eq!(since(from_hmsm(3, 5, 7, 900), from_hmsm(3, 0, 7, 900)),
    ///            TimeDelta::seconds(5 * 60));
    /// assert_eq!(since(from_hmsm(3, 5, 7, 900), from_hmsm(0, 5, 7, 900)),
    ///            TimeDelta::seconds(3 * 3600));
    /// assert_eq!(since(from_hmsm(3, 5, 7, 900), from_hmsm(4, 5, 7, 900)),
    ///            TimeDelta::seconds(-3600));
    /// assert_eq!(since(from_hmsm(3, 5, 7, 900), from_hmsm(2, 4, 6, 800)),
    ///            TimeDelta::seconds(3600 + 60 + 1) + TimeDelta::milliseconds(100));
    /// ```
    ///
    /// Leap seconds are handled, but the subtraction assumes that
    /// there were no other leap seconds happened.
    ///
    /// ```
    /// # use chrono::{TimeDelta, NaiveTime};
    /// # let from_hmsm = NaiveTime::from_hms_milli;
    /// # let since = NaiveTime::signed_duration_since;
    /// assert_eq!(since(from_hmsm(3, 0, 59, 1_000), from_hmsm(3, 0, 59, 0)),
    ///            TimeDelta::seconds(1));
    /// assert_eq!(since(from_hmsm(3, 0, 59, 1_500), from_hmsm(3, 0, 59, 0)),
    ///            TimeDelta::milliseconds(1500));
    /// assert_eq!(since(from_hmsm(3, 0, 59, 1_000), from_hmsm(3, 0, 0, 0)),
    ///            TimeDelta::seconds(60));
    /// assert_eq!(since(from_hmsm(3, 0, 0, 0), from_hmsm(2, 59, 59, 1_000)),
    ///            TimeDelta::seconds(1));
    /// assert_eq!(since(from_hmsm(3, 0, 59, 1_000), from_hmsm(2, 59, 59, 1_000)),
    ///            TimeDelta::seconds(61));
    /// ```
    #[must_use]
    pub fn signed_duration_since(self, rhs: NaiveTime) -> TimeDelta {
        use core::cmp::Ordering;
        let secs = i64::from(self.secs) - i64::from(rhs.secs);
        let frac = i64::from(self.frac) - i64::from(rhs.frac);
        let adjust = match self.secs.cmp(&rhs.secs) {
            Ordering::Greater => i64::from(rhs.frac >= 1_000_000_000),
            Ordering::Equal => 0,
            Ordering::Less => if self.frac >= 1_000_000_000 { -1 } else { 0 }
        };
        TimeDelta::seconds(secs + adjust) + TimeDelta::nanoseconds(frac)
    }
    /// Formats the time with the specified formatting items.
    /// Otherwise it is the same as the ordinary [`format`](#method.format) method.
    ///
    /// The `Iterator` of items should be `Clone`able,
    /// since the resulting `DelayedFormat` value may be formatted multiple times.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::NaiveTime;
    /// use chrono::format::strftime::StrftimeItems;
    ///
    /// let fmt = StrftimeItems::new("%H:%M:%S");
    /// let t = NaiveTime::from_hms_opt(23, 56, 4).unwrap();
    /// assert_eq!(t.format_with_items(fmt.clone()).to_string(), "23:56:04");
    /// assert_eq!(t.format("%H:%M:%S").to_string(),             "23:56:04");
    /// ```
    ///
    /// The resulting `DelayedFormat` can be formatted directly via the `Display` trait.
    ///
    /// ```
    /// # use chrono::NaiveTime;
    /// # use chrono::format::strftime::StrftimeItems;
    /// # let fmt = StrftimeItems::new("%H:%M:%S").clone();
    /// # let t = NaiveTime::from_hms_opt(23, 56, 4).unwrap();
    /// assert_eq!(format!("{}", t.format_with_items(fmt)), "23:56:04");
    /// ```
    #[cfg(any(feature = "alloc", feature = "std", test))]
    #[cfg_attr(docsrs, doc(cfg(any(feature = "alloc", feature = "std"))))]
    #[inline]
    #[must_use]
    pub fn format_with_items<'a, I, B>(&self, items: I) -> DelayedFormat<I>
    where
        I: Iterator<Item = B> + Clone,
        B: Borrow<Item<'a>>,
    {
        DelayedFormat::new(None, Some(*self), items)
    }
    /// Formats the time with the specified format string.
    /// See the [`format::strftime` module](../format/strftime/index.html)
    /// on the supported escape sequences.
    ///
    /// This returns a `DelayedFormat`,
    /// which gets converted to a string only when actual formatting happens.
    /// You may use the `to_string` method to get a `String`,
    /// or just feed it into `print!` and other formatting macros.
    /// (In this way it avoids the redundant memory allocation.)
    ///
    /// A wrong format string does *not* issue an error immediately.
    /// Rather, converting or formatting the `DelayedFormat` fails.
    /// You are recommended to immediately use `DelayedFormat` for this reason.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::NaiveTime;
    ///
    /// let t = NaiveTime::from_hms_nano_opt(23, 56, 4, 12_345_678).unwrap();
    /// assert_eq!(t.format("%H:%M:%S").to_string(), "23:56:04");
    /// assert_eq!(t.format("%H:%M:%S%.6f").to_string(), "23:56:04.012345");
    /// assert_eq!(t.format("%-I:%M %p").to_string(), "11:56 PM");
    /// ```
    ///
    /// The resulting `DelayedFormat` can be formatted directly via the `Display` trait.
    ///
    /// ```
    /// # use chrono::NaiveTime;
    /// # let t = NaiveTime::from_hms_nano_opt(23, 56, 4, 12_345_678).unwrap();
    /// assert_eq!(format!("{}", t.format("%H:%M:%S")), "23:56:04");
    /// assert_eq!(format!("{}", t.format("%H:%M:%S%.6f")), "23:56:04.012345");
    /// assert_eq!(format!("{}", t.format("%-I:%M %p")), "11:56 PM");
    /// ```
    #[cfg(any(feature = "alloc", feature = "std", test))]
    #[cfg_attr(docsrs, doc(cfg(any(feature = "alloc", feature = "std"))))]
    #[inline]
    #[must_use]
    pub fn format<'a>(&self, fmt: &'a str) -> DelayedFormat<StrftimeItems<'a>> {
        self.format_with_items(StrftimeItems::new(fmt))
    }
    /// Returns a triple of the hour, minute and second numbers.
    fn hms(&self) -> (u32, u32, u32) {
        let sec = self.secs % 60;
        let mins = self.secs / 60;
        let min = mins % 60;
        let hour = mins / 60;
        (hour, min, sec)
    }
    /// The earliest possible `NaiveTime`
    pub const MIN: Self = Self { secs: 0, frac: 0 };
    pub(super) const MAX: Self = Self {
        secs: 23 * 3600 + 59 * 60 + 59,
        frac: 999_999_999,
    };
}
impl Timelike for NaiveTime {
    /// Returns the hour number from 0 to 23.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveTime, Timelike};
    ///
    /// assert_eq!(NaiveTime::from_hms_opt(0, 0, 0).unwrap().hour(), 0);
    /// assert_eq!(NaiveTime::from_hms_nano_opt(23, 56, 4, 12_345_678).unwrap().hour(), 23);
    /// ```
    #[inline]
    fn hour(&self) -> u32 {
        self.hms().0
    }
    /// Returns the minute number from 0 to 59.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveTime, Timelike};
    ///
    /// assert_eq!(NaiveTime::from_hms_opt(0, 0, 0).unwrap().minute(), 0);
    /// assert_eq!(NaiveTime::from_hms_nano_opt(23, 56, 4, 12_345_678).unwrap().minute(), 56);
    /// ```
    #[inline]
    fn minute(&self) -> u32 {
        self.hms().1
    }
    /// Returns the second number from 0 to 59.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveTime, Timelike};
    ///
    /// assert_eq!(NaiveTime::from_hms_opt(0, 0, 0).unwrap().second(), 0);
    /// assert_eq!(NaiveTime::from_hms_nano_opt(23, 56, 4, 12_345_678).unwrap().second(), 4);
    /// ```
    ///
    /// This method never returns 60 even when it is a leap second.
    /// ([Why?](#leap-second-handling))
    /// Use the proper [formatting method](#method.format) to get a human-readable representation.
    ///
    #[cfg_attr(not(feature = "std"), doc = "```ignore")]
    #[cfg_attr(feature = "std", doc = "```")]
    /// # use chrono::{NaiveTime, Timelike};
    /// let leap = NaiveTime::from_hms_milli_opt(23, 59, 59, 1_000).unwrap();
    /// assert_eq!(leap.second(), 59);
    /// assert_eq!(leap.format("%H:%M:%S").to_string(), "23:59:60");
    /// ```
    #[inline]
    fn second(&self) -> u32 {
        self.hms().2
    }
    /// Returns the number of nanoseconds since the whole non-leap second.
    /// The range from 1,000,000,000 to 1,999,999,999 represents
    /// the [leap second](#leap-second-handling).
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveTime, Timelike};
    ///
    /// assert_eq!(NaiveTime::from_hms_opt(0, 0, 0).unwrap().nanosecond(), 0);
    /// assert_eq!(NaiveTime::from_hms_nano_opt(23, 56, 4, 12_345_678).unwrap().nanosecond(), 12_345_678);
    /// ```
    ///
    /// Leap seconds may have seemingly out-of-range return values.
    /// You can reduce the range with `time.nanosecond() % 1_000_000_000`, or
    /// use the proper [formatting method](#method.format) to get a human-readable representation.
    ///
    #[cfg_attr(not(feature = "std"), doc = "```ignore")]
    #[cfg_attr(feature = "std", doc = "```")]
    /// # use chrono::{NaiveTime, Timelike};
    /// let leap = NaiveTime::from_hms_milli_opt(23, 59, 59, 1_000).unwrap();
    /// assert_eq!(leap.nanosecond(), 1_000_000_000);
    /// assert_eq!(leap.format("%H:%M:%S%.9f").to_string(), "23:59:60.000000000");
    /// ```
    #[inline]
    fn nanosecond(&self) -> u32 {
        self.frac
    }
    /// Makes a new `NaiveTime` with the hour number changed.
    ///
    /// Returns `None` when the resulting `NaiveTime` would be invalid.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveTime, Timelike};
    ///
    /// let dt = NaiveTime::from_hms_nano_opt(23, 56, 4, 12_345_678).unwrap();
    /// assert_eq!(dt.with_hour(7), Some(NaiveTime::from_hms_nano_opt(7, 56, 4, 12_345_678).unwrap()));
    /// assert_eq!(dt.with_hour(24), None);
    /// ```
    #[inline]
    fn with_hour(&self, hour: u32) -> Option<NaiveTime> {
        if hour >= 24 {
            return None;
        }
        let secs = hour * 3600 + self.secs % 3600;
        Some(NaiveTime { secs, ..*self })
    }
    /// Makes a new `NaiveTime` with the minute number changed.
    ///
    /// Returns `None` when the resulting `NaiveTime` would be invalid.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveTime, Timelike};
    ///
    /// let dt = NaiveTime::from_hms_nano_opt(23, 56, 4, 12_345_678).unwrap();
    /// assert_eq!(dt.with_minute(45), Some(NaiveTime::from_hms_nano_opt(23, 45, 4, 12_345_678).unwrap()));
    /// assert_eq!(dt.with_minute(60), None);
    /// ```
    #[inline]
    fn with_minute(&self, min: u32) -> Option<NaiveTime> {
        if min >= 60 {
            return None;
        }
        let secs = self.secs / 3600 * 3600 + min * 60 + self.secs % 60;
        Some(NaiveTime { secs, ..*self })
    }
    /// Makes a new `NaiveTime` with the second number changed.
    ///
    /// Returns `None` when the resulting `NaiveTime` would be invalid.
    /// As with the [`second`](#method.second) method,
    /// the input range is restricted to 0 through 59.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveTime, Timelike};
    ///
    /// let dt = NaiveTime::from_hms_nano_opt(23, 56, 4, 12_345_678).unwrap();
    /// assert_eq!(dt.with_second(17), Some(NaiveTime::from_hms_nano_opt(23, 56, 17, 12_345_678).unwrap()));
    /// assert_eq!(dt.with_second(60), None);
    /// ```
    #[inline]
    fn with_second(&self, sec: u32) -> Option<NaiveTime> {
        if sec >= 60 {
            return None;
        }
        let secs = self.secs / 60 * 60 + sec;
        Some(NaiveTime { secs, ..*self })
    }
    /// Makes a new `NaiveTime` with nanoseconds since the whole non-leap second changed.
    ///
    /// Returns `None` when the resulting `NaiveTime` would be invalid.
    /// As with the [`nanosecond`](#method.nanosecond) method,
    /// the input range can exceed 1,000,000,000 for leap seconds.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveTime, Timelike};
    ///
    /// let dt = NaiveTime::from_hms_nano_opt(23, 56, 4, 12_345_678).unwrap();
    /// assert_eq!(dt.with_nanosecond(333_333_333),
    ///            Some(NaiveTime::from_hms_nano_opt(23, 56, 4, 333_333_333).unwrap()));
    /// assert_eq!(dt.with_nanosecond(2_000_000_000), None);
    /// ```
    ///
    /// Leap seconds can theoretically follow *any* whole second.
    /// The following would be a proper leap second at the time zone offset of UTC-00:03:57
    /// (there are several historical examples comparable to this "non-sense" offset),
    /// and therefore is allowed.
    ///
    /// ```
    /// # use chrono::{NaiveTime, Timelike};
    /// # let dt = NaiveTime::from_hms_nano_opt(23, 56, 4, 12_345_678).unwrap();
    /// assert_eq!(dt.with_nanosecond(1_333_333_333),
    ///            Some(NaiveTime::from_hms_nano_opt(23, 56, 4, 1_333_333_333).unwrap()));
    /// ```
    #[inline]
    fn with_nanosecond(&self, nano: u32) -> Option<NaiveTime> {
        if nano >= 2_000_000_000 {
            return None;
        }
        Some(NaiveTime { frac: nano, ..*self })
    }
    /// Returns the number of non-leap seconds past the last midnight.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveTime, Timelike};
    ///
    /// assert_eq!(NaiveTime::from_hms_opt(1, 2, 3).unwrap().num_seconds_from_midnight(),
    ///            3723);
    /// assert_eq!(NaiveTime::from_hms_nano_opt(23, 56, 4, 12_345_678).unwrap().num_seconds_from_midnight(),
    ///            86164);
    /// assert_eq!(NaiveTime::from_hms_milli_opt(23, 59, 59, 1_000).unwrap().num_seconds_from_midnight(),
    ///            86399);
    /// ```
    #[inline]
    fn num_seconds_from_midnight(&self) -> u32 {
        self.secs
    }
}
/// An addition of `TimeDelta` to `NaiveTime` wraps around and never overflows or underflows.
/// In particular the addition ignores integral number of days.
///
/// As a part of Chrono's [leap second handling], the addition assumes that **there is no leap
/// second ever**, except when the `NaiveTime` itself represents a leap second in which case the
/// assumption becomes that **there is exactly a single leap second ever**.
///
/// # Example
///
/// ```
/// use chrono::{TimeDelta, NaiveTime};
///
/// let from_hmsm = NaiveTime::from_hms_milli;
///
/// assert_eq!(from_hmsm(3, 5, 7, 0) + TimeDelta::zero(),                  from_hmsm(3, 5, 7, 0));
/// assert_eq!(from_hmsm(3, 5, 7, 0) + TimeDelta::seconds(1),              from_hmsm(3, 5, 8, 0));
/// assert_eq!(from_hmsm(3, 5, 7, 0) + TimeDelta::seconds(-1),             from_hmsm(3, 5, 6, 0));
/// assert_eq!(from_hmsm(3, 5, 7, 0) + TimeDelta::seconds(60 + 4),         from_hmsm(3, 6, 11, 0));
/// assert_eq!(from_hmsm(3, 5, 7, 0) + TimeDelta::seconds(7*60*60 - 6*60), from_hmsm(9, 59, 7, 0));
/// assert_eq!(from_hmsm(3, 5, 7, 0) + TimeDelta::milliseconds(80),        from_hmsm(3, 5, 7, 80));
/// assert_eq!(from_hmsm(3, 5, 7, 950) + TimeDelta::milliseconds(280),     from_hmsm(3, 5, 8, 230));
/// assert_eq!(from_hmsm(3, 5, 7, 950) + TimeDelta::milliseconds(-980),    from_hmsm(3, 5, 6, 970));
/// ```
///
/// The addition wraps around.
///
/// ```
/// # use chrono::{TimeDelta, NaiveTime};
/// # let from_hmsm = NaiveTime::from_hms_milli;
/// assert_eq!(from_hmsm(3, 5, 7, 0) + TimeDelta::seconds(22*60*60), from_hmsm(1, 5, 7, 0));
/// assert_eq!(from_hmsm(3, 5, 7, 0) + TimeDelta::seconds(-8*60*60), from_hmsm(19, 5, 7, 0));
/// assert_eq!(from_hmsm(3, 5, 7, 0) + TimeDelta::days(800),         from_hmsm(3, 5, 7, 0));
/// ```
///
/// Leap seconds are handled, but the addition assumes that it is the only leap second happened.
///
/// ```
/// # use chrono::{TimeDelta, NaiveTime};
/// # let from_hmsm = NaiveTime::from_hms_milli;
/// let leap = from_hmsm(3, 5, 59, 1_300);
/// assert_eq!(leap + TimeDelta::zero(),             from_hmsm(3, 5, 59, 1_300));
/// assert_eq!(leap + TimeDelta::milliseconds(-500), from_hmsm(3, 5, 59, 800));
/// assert_eq!(leap + TimeDelta::milliseconds(500),  from_hmsm(3, 5, 59, 1_800));
/// assert_eq!(leap + TimeDelta::milliseconds(800),  from_hmsm(3, 6, 0, 100));
/// assert_eq!(leap + TimeDelta::seconds(10),        from_hmsm(3, 6, 9, 300));
/// assert_eq!(leap + TimeDelta::seconds(-10),       from_hmsm(3, 5, 50, 300));
/// assert_eq!(leap + TimeDelta::days(1),            from_hmsm(3, 5, 59, 300));
/// ```
///
/// [leap second handling]: crate::NaiveTime#leap-second-handling
impl Add<TimeDelta> for NaiveTime {
    type Output = NaiveTime;
    #[inline]
    fn add(self, rhs: TimeDelta) -> NaiveTime {
        self.overflowing_add_signed(rhs).0
    }
}
impl AddAssign<TimeDelta> for NaiveTime {
    #[inline]
    fn add_assign(&mut self, rhs: TimeDelta) {
        *self = self.add(rhs);
    }
}
/// A subtraction of `TimeDelta` from `NaiveTime` wraps around and never overflows or underflows.
/// In particular the addition ignores integral number of days.
/// It is the same as the addition with a negated `TimeDelta`.
///
/// As a part of Chrono's [leap second handling], the subtraction assumes that **there is no leap
/// second ever**, except when the `NaiveTime` itself represents a leap second in which case the
/// assumption becomes that **there is exactly a single leap second ever**.
///
/// # Example
///
/// ```
/// use chrono::{TimeDelta, NaiveTime};
///
/// let from_hmsm = NaiveTime::from_hms_milli;
///
/// assert_eq!(from_hmsm(3, 5, 7, 0) - TimeDelta::zero(),                  from_hmsm(3, 5, 7, 0));
/// assert_eq!(from_hmsm(3, 5, 7, 0) - TimeDelta::seconds(1),              from_hmsm(3, 5, 6, 0));
/// assert_eq!(from_hmsm(3, 5, 7, 0) - TimeDelta::seconds(60 + 5),         from_hmsm(3, 4, 2, 0));
/// assert_eq!(from_hmsm(3, 5, 7, 0) - TimeDelta::seconds(2*60*60 + 6*60), from_hmsm(0, 59, 7, 0));
/// assert_eq!(from_hmsm(3, 5, 7, 0) - TimeDelta::milliseconds(80),        from_hmsm(3, 5, 6, 920));
/// assert_eq!(from_hmsm(3, 5, 7, 950) - TimeDelta::milliseconds(280),     from_hmsm(3, 5, 7, 670));
/// ```
///
/// The subtraction wraps around.
///
/// ```
/// # use chrono::{TimeDelta, NaiveTime};
/// # let from_hmsm = NaiveTime::from_hms_milli;
/// assert_eq!(from_hmsm(3, 5, 7, 0) - TimeDelta::seconds(8*60*60), from_hmsm(19, 5, 7, 0));
/// assert_eq!(from_hmsm(3, 5, 7, 0) - TimeDelta::days(800),        from_hmsm(3, 5, 7, 0));
/// ```
///
/// Leap seconds are handled, but the subtraction assumes that it is the only leap second happened.
///
/// ```
/// # use chrono::{TimeDelta, NaiveTime};
/// # let from_hmsm = NaiveTime::from_hms_milli;
/// let leap = from_hmsm(3, 5, 59, 1_300);
/// assert_eq!(leap - TimeDelta::zero(),            from_hmsm(3, 5, 59, 1_300));
/// assert_eq!(leap - TimeDelta::milliseconds(200), from_hmsm(3, 5, 59, 1_100));
/// assert_eq!(leap - TimeDelta::milliseconds(500), from_hmsm(3, 5, 59, 800));
/// assert_eq!(leap - TimeDelta::seconds(60),       from_hmsm(3, 5, 0, 300));
/// assert_eq!(leap - TimeDelta::days(1),           from_hmsm(3, 6, 0, 300));
/// ```
///
/// [leap second handling]: crate::NaiveTime#leap-second-handling
impl Sub<TimeDelta> for NaiveTime {
    type Output = NaiveTime;
    #[inline]
    fn sub(self, rhs: TimeDelta) -> NaiveTime {
        self.overflowing_sub_signed(rhs).0
    }
}
impl SubAssign<TimeDelta> for NaiveTime {
    #[inline]
    fn sub_assign(&mut self, rhs: TimeDelta) {
        *self = self.sub(rhs);
    }
}
/// Subtracts another `NaiveTime` from the current time.
/// Returns a `TimeDelta` within +/- 1 day.
/// This does not overflow or underflow at all.
///
/// As a part of Chrono's [leap second handling](#leap-second-handling),
/// the subtraction assumes that **there is no leap second ever**,
/// except when any of the `NaiveTime`s themselves represents a leap second
/// in which case the assumption becomes that
/// **there are exactly one (or two) leap second(s) ever**.
///
/// The implementation is a wrapper around
/// [`NaiveTime::signed_duration_since`](#method.signed_duration_since).
///
/// # Example
///
/// ```
/// use chrono::{TimeDelta, NaiveTime};
///
/// let from_hmsm = NaiveTime::from_hms_milli;
///
/// assert_eq!(from_hmsm(3, 5, 7, 900) - from_hmsm(3, 5, 7, 900), TimeDelta::zero());
/// assert_eq!(from_hmsm(3, 5, 7, 900) - from_hmsm(3, 5, 7, 875), TimeDelta::milliseconds(25));
/// assert_eq!(from_hmsm(3, 5, 7, 900) - from_hmsm(3, 5, 6, 925), TimeDelta::milliseconds(975));
/// assert_eq!(from_hmsm(3, 5, 7, 900) - from_hmsm(3, 5, 0, 900), TimeDelta::seconds(7));
/// assert_eq!(from_hmsm(3, 5, 7, 900) - from_hmsm(3, 0, 7, 900), TimeDelta::seconds(5 * 60));
/// assert_eq!(from_hmsm(3, 5, 7, 900) - from_hmsm(0, 5, 7, 900), TimeDelta::seconds(3 * 3600));
/// assert_eq!(from_hmsm(3, 5, 7, 900) - from_hmsm(4, 5, 7, 900), TimeDelta::seconds(-3600));
/// assert_eq!(from_hmsm(3, 5, 7, 900) - from_hmsm(2, 4, 6, 800),
///            TimeDelta::seconds(3600 + 60 + 1) + TimeDelta::milliseconds(100));
/// ```
///
/// Leap seconds are handled, but the subtraction assumes that
/// there were no other leap seconds happened.
///
/// ```
/// # use chrono::{TimeDelta, NaiveTime};
/// # let from_hmsm = NaiveTime::from_hms_milli;
/// assert_eq!(from_hmsm(3, 0, 59, 1_000) - from_hmsm(3, 0, 59, 0), TimeDelta::seconds(1));
/// assert_eq!(from_hmsm(3, 0, 59, 1_500) - from_hmsm(3, 0, 59, 0),
///            TimeDelta::milliseconds(1500));
/// assert_eq!(from_hmsm(3, 0, 59, 1_000) - from_hmsm(3, 0, 0, 0), TimeDelta::seconds(60));
/// assert_eq!(from_hmsm(3, 0, 0, 0) - from_hmsm(2, 59, 59, 1_000), TimeDelta::seconds(1));
/// assert_eq!(from_hmsm(3, 0, 59, 1_000) - from_hmsm(2, 59, 59, 1_000),
///            TimeDelta::seconds(61));
/// ```
impl Sub<NaiveTime> for NaiveTime {
    type Output = TimeDelta;
    #[inline]
    fn sub(self, rhs: NaiveTime) -> TimeDelta {
        self.signed_duration_since(rhs)
    }
}
/// The `Debug` output of the naive time `t` is the same as
/// [`t.format("%H:%M:%S%.f")`](../format/strftime/index.html).
///
/// The string printed can be readily parsed via the `parse` method on `str`.
///
/// It should be noted that, for leap seconds not on the minute boundary,
/// it may print a representation not distinguishable from non-leap seconds.
/// This doesn't matter in practice, since such leap seconds never happened.
/// (By the time of the first leap second on 1972-06-30,
/// every time zone offset around the world has standardized to the 5-minute alignment.)
///
/// # Example
///
/// ```
/// use chrono::NaiveTime;
///
/// assert_eq!(format!("{:?}", NaiveTime::from_hms_opt(23, 56, 4).unwrap()),              "23:56:04");
/// assert_eq!(format!("{:?}", NaiveTime::from_hms_milli_opt(23, 56, 4, 12).unwrap()),    "23:56:04.012");
/// assert_eq!(format!("{:?}", NaiveTime::from_hms_micro_opt(23, 56, 4, 1234).unwrap()),  "23:56:04.001234");
/// assert_eq!(format!("{:?}", NaiveTime::from_hms_nano_opt(23, 56, 4, 123456).unwrap()), "23:56:04.000123456");
/// ```
///
/// Leap seconds may also be used.
///
/// ```
/// # use chrono::NaiveTime;
/// assert_eq!(format!("{:?}", NaiveTime::from_hms_milli_opt(6, 59, 59, 1_500).unwrap()), "06:59:60.500");
/// ```
impl fmt::Debug for NaiveTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (hour, min, sec) = self.hms();
        let (sec, nano) = if self.frac >= 1_000_000_000 {
            (sec + 1, self.frac - 1_000_000_000)
        } else {
            (sec, self.frac)
        };
        use core::fmt::Write;
        write_hundreds(f, hour as u8)?;
        f.write_char(':')?;
        write_hundreds(f, min as u8)?;
        f.write_char(':')?;
        write_hundreds(f, sec as u8)?;
        if nano == 0 {
            Ok(())
        } else if nano % 1_000_000 == 0 {
            write!(f, ".{:03}", nano / 1_000_000)
        } else if nano % 1_000 == 0 {
            write!(f, ".{:06}", nano / 1_000)
        } else {
            write!(f, ".{:09}", nano)
        }
    }
}
/// The `Display` output of the naive time `t` is the same as
/// [`t.format("%H:%M:%S%.f")`](../format/strftime/index.html).
///
/// The string printed can be readily parsed via the `parse` method on `str`.
///
/// It should be noted that, for leap seconds not on the minute boundary,
/// it may print a representation not distinguishable from non-leap seconds.
/// This doesn't matter in practice, since such leap seconds never happened.
/// (By the time of the first leap second on 1972-06-30,
/// every time zone offset around the world has standardized to the 5-minute alignment.)
///
/// # Example
///
/// ```
/// use chrono::NaiveTime;
///
/// assert_eq!(format!("{}", NaiveTime::from_hms_opt(23, 56, 4).unwrap()),              "23:56:04");
/// assert_eq!(format!("{}", NaiveTime::from_hms_milli_opt(23, 56, 4, 12).unwrap()),    "23:56:04.012");
/// assert_eq!(format!("{}", NaiveTime::from_hms_micro_opt(23, 56, 4, 1234).unwrap()),  "23:56:04.001234");
/// assert_eq!(format!("{}", NaiveTime::from_hms_nano_opt(23, 56, 4, 123456).unwrap()), "23:56:04.000123456");
/// ```
///
/// Leap seconds may also be used.
///
/// ```
/// # use chrono::NaiveTime;
/// assert_eq!(format!("{}", NaiveTime::from_hms_milli_opt(6, 59, 59, 1_500).unwrap()), "06:59:60.500");
/// ```
impl fmt::Display for NaiveTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
/// Parsing a `str` into a `NaiveTime` uses the same format,
/// [`%H:%M:%S%.f`](../format/strftime/index.html), as in `Debug` and `Display`.
///
/// # Example
///
/// ```
/// use chrono::NaiveTime;
///
/// let t = NaiveTime::from_hms_opt(23, 56, 4).unwrap();
/// assert_eq!("23:56:04".parse::<NaiveTime>(), Ok(t));
///
/// let t = NaiveTime::from_hms_nano_opt(23, 56, 4, 12_345_678).unwrap();
/// assert_eq!("23:56:4.012345678".parse::<NaiveTime>(), Ok(t));
///
/// let t = NaiveTime::from_hms_nano_opt(23, 59, 59, 1_234_567_890).unwrap(); // leap second
/// assert_eq!("23:59:60.23456789".parse::<NaiveTime>(), Ok(t));
///
/// assert!("foo".parse::<NaiveTime>().is_err());
/// ```
impl str::FromStr for NaiveTime {
    type Err = ParseError;
    fn from_str(s: &str) -> ParseResult<NaiveTime> {
        const ITEMS: &[Item<'static>] = &[
            Item::Numeric(Numeric::Hour, Pad::Zero),
            Item::Literal(":"),
            Item::Numeric(Numeric::Minute, Pad::Zero),
            Item::Literal(":"),
            Item::Numeric(Numeric::Second, Pad::Zero),
            Item::Fixed(Fixed::Nanosecond),
        ];
        let mut parsed = Parsed::new();
        parse(&mut parsed, s, ITEMS.iter())?;
        parsed.to_naive_time()
    }
}
/// The default value for a NaiveTime is midnight, 00:00:00 exactly.
///
/// # Example
///
/// ```rust
/// use chrono::NaiveTime;
///
/// let default_time = NaiveTime::default();
/// assert_eq!(default_time, NaiveTime::from_hms_opt(0, 0, 0).unwrap());
/// ```
impl Default for NaiveTime {
    fn default() -> Self {
        NaiveTime::from_hms_opt(0, 0, 0).unwrap()
    }
}
#[cfg(all(test, feature = "serde"))]
fn test_encodable_json<F, E>(to_string: F)
where
    F: Fn(&NaiveTime) -> Result<String, E>,
    E: ::std::fmt::Debug,
{
    assert_eq!(
        to_string(& NaiveTime::from_hms_opt(0, 0, 0).unwrap()).ok(), Some(r#""00:00:00""#
        .into())
    );
    assert_eq!(
        to_string(& NaiveTime::from_hms_milli_opt(0, 0, 0, 950).unwrap()).ok(),
        Some(r#""00:00:00.950""#.into())
    );
    assert_eq!(
        to_string(& NaiveTime::from_hms_milli_opt(0, 0, 59, 1_000).unwrap()).ok(),
        Some(r#""00:00:60""#.into())
    );
    assert_eq!(
        to_string(& NaiveTime::from_hms_opt(0, 1, 2).unwrap()).ok(), Some(r#""00:01:02""#
        .into())
    );
    assert_eq!(
        to_string(& NaiveTime::from_hms_nano_opt(3, 5, 7, 98765432).unwrap()).ok(),
        Some(r#""03:05:07.098765432""#.into())
    );
    assert_eq!(
        to_string(& NaiveTime::from_hms_opt(7, 8, 9).unwrap()).ok(), Some(r#""07:08:09""#
        .into())
    );
    assert_eq!(
        to_string(& NaiveTime::from_hms_micro_opt(12, 34, 56, 789).unwrap()).ok(),
        Some(r#""12:34:56.000789""#.into())
    );
    assert_eq!(
        to_string(& NaiveTime::from_hms_nano_opt(23, 59, 59, 1_999_999_999).unwrap())
        .ok(), Some(r#""23:59:60.999999999""#.into())
    );
}
#[cfg(all(test, feature = "serde"))]
fn test_decodable_json<F, E>(from_str: F)
where
    F: Fn(&str) -> Result<NaiveTime, E>,
    E: ::std::fmt::Debug,
{
    assert_eq!(
        from_str(r#""00:00:00""#).ok(), Some(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
    );
    assert_eq!(
        from_str(r#""0:0:0""#).ok(), Some(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
    );
    assert_eq!(
        from_str(r#""00:00:00.950""#).ok(), Some(NaiveTime::from_hms_milli_opt(0, 0, 0,
        950).unwrap())
    );
    assert_eq!(
        from_str(r#""0:0:0.95""#).ok(), Some(NaiveTime::from_hms_milli_opt(0, 0, 0, 950)
        .unwrap())
    );
    assert_eq!(
        from_str(r#""00:00:60""#).ok(), Some(NaiveTime::from_hms_milli_opt(0, 0, 59,
        1_000).unwrap())
    );
    assert_eq!(
        from_str(r#""00:01:02""#).ok(), Some(NaiveTime::from_hms_opt(0, 1, 2).unwrap())
    );
    assert_eq!(
        from_str(r#""03:05:07.098765432""#).ok(), Some(NaiveTime::from_hms_nano_opt(3, 5,
        7, 98765432).unwrap())
    );
    assert_eq!(
        from_str(r#""07:08:09""#).ok(), Some(NaiveTime::from_hms_opt(7, 8, 9).unwrap())
    );
    assert_eq!(
        from_str(r#""12:34:56.000789""#).ok(), Some(NaiveTime::from_hms_micro_opt(12, 34,
        56, 789).unwrap())
    );
    assert_eq!(
        from_str(r#""23:59:60.999999999""#).ok(), Some(NaiveTime::from_hms_nano_opt(23,
        59, 59, 1_999_999_999).unwrap())
    );
    assert_eq!(
        from_str(r#""23:59:60.9999999999997""#).ok(),
        Some(NaiveTime::from_hms_nano_opt(23, 59, 59, 1_999_999_999).unwrap())
    );
    assert!(from_str(r#""""#).is_err());
    assert!(from_str(r#""000000""#).is_err());
    assert!(from_str(r#""00:00:61""#).is_err());
    assert!(from_str(r#""00:60:00""#).is_err());
    assert!(from_str(r#""24:00:00""#).is_err());
    assert!(from_str(r#""23:59:59,1""#).is_err());
    assert!(from_str(r#""012:34:56""#).is_err());
    assert!(from_str(r#""hh:mm:ss""#).is_err());
    assert!(from_str(r#"0"#).is_err());
    assert!(from_str(r#"86399"#).is_err());
    assert!(from_str(r#"{}"#).is_err());
    assert!(from_str(r#"{"secs":0,"frac":0}"#).is_err());
    assert!(from_str(r#"null"#).is_err());
}
#[cfg(test)]
mod tests_llm_16_154 {
    use crate::naive::time::NaiveTime;
    use std::default::Default;
    #[test]
    fn test_naive_time_default() {
        let _rug_st_tests_llm_16_154_rrrruuuugggg_test_naive_time_default = 0;
        let default_time = NaiveTime::default();
        debug_assert_eq!(default_time, NaiveTime::from_hms(0, 0, 0));
        let _rug_ed_tests_llm_16_154_rrrruuuugggg_test_naive_time_default = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_155_llm_16_155 {
    use crate::NaiveTime;
    use crate::time_delta::TimeDelta;
    use std::ops::Add;
    #[test]
    fn test_add_timedelta_to_naive_time() {
        let _rug_st_tests_llm_16_155_llm_16_155_rrrruuuugggg_test_add_timedelta_to_naive_time = 0;
        let rug_fuzz_0 = 10;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 90;
        let rug_fuzz_5 = 30;
        let rug_fuzz_6 = 15;
        let rug_fuzz_7 = 24;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 30;
        let rug_fuzz_10 = 500_000_000;
        let rug_fuzz_11 = 11;
        let rug_fuzz_12 = 86400;
        let rug_fuzz_13 = 1_000_000_000;
        let time = NaiveTime::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let delta_one_hour = TimeDelta::hours(rug_fuzz_3);
        debug_assert_eq!(time + delta_one_hour, NaiveTime::from_hms(11, 0, 0));
        let delta_ninety_minutes = TimeDelta::minutes(rug_fuzz_4);
        debug_assert_eq!(time + delta_ninety_minutes, NaiveTime::from_hms(11, 30, 0));
        let delta_minus_thirty_minutes = TimeDelta::minutes(-rug_fuzz_5);
        debug_assert_eq!(
            time + delta_minus_thirty_minutes, NaiveTime::from_hms(9, 30, 0)
        );
        let delta_to_next_day = TimeDelta::hours(rug_fuzz_6);
        debug_assert_eq!(time + delta_to_next_day, NaiveTime::from_hms(1, 0, 0));
        let delta_full_day = TimeDelta::hours(rug_fuzz_7);
        debug_assert_eq!(time + delta_full_day, time);
        let delta_seconds_nanos = TimeDelta::minutes(rug_fuzz_8)
            + TimeDelta::seconds(rug_fuzz_9) + TimeDelta::nanoseconds(rug_fuzz_10);
        debug_assert_eq!(
            time + delta_seconds_nanos, NaiveTime::from_hms_nano(10, 1, 30, 500_000_000)
        );
        let delta_negative_time = TimeDelta::hours(-rug_fuzz_11);
        debug_assert_eq!(time + delta_negative_time, NaiveTime::from_hms(23, 0, 0));
        let delta_leap_second = TimeDelta::seconds(rug_fuzz_12)
            + TimeDelta::nanoseconds(rug_fuzz_13);
        debug_assert_eq!(
            time + delta_leap_second, NaiveTime::from_hms_nano(10, 0, 1, 0)
        );
        let _rug_ed_tests_llm_16_155_llm_16_155_rrrruuuugggg_test_add_timedelta_to_naive_time = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_156 {
    use crate::naive::time::NaiveTime;
    use crate::time_delta::TimeDelta;
    use std::ops::AddAssign;
    #[test]
    fn test_add_assign_with_positive_timedelta() {
        let _rug_st_tests_llm_16_156_rrrruuuugggg_test_add_assign_with_positive_timedelta = 0;
        let rug_fuzz_0 = 12;
        let rug_fuzz_1 = 34;
        let rug_fuzz_2 = 56;
        let rug_fuzz_3 = 789;
        let rug_fuzz_4 = 10_000;
        let mut time = NaiveTime::from_hms_milli_opt(
                rug_fuzz_0,
                rug_fuzz_1,
                rug_fuzz_2,
                rug_fuzz_3,
            )
            .unwrap();
        let time_delta = TimeDelta::milliseconds(rug_fuzz_4);
        time.add_assign(time_delta);
        debug_assert_eq!(time, NaiveTime::from_hms_milli_opt(12, 35, 6, 789).unwrap());
        let _rug_ed_tests_llm_16_156_rrrruuuugggg_test_add_assign_with_positive_timedelta = 0;
    }
    #[test]
    fn test_add_assign_with_negative_timedelta() {
        let _rug_st_tests_llm_16_156_rrrruuuugggg_test_add_assign_with_negative_timedelta = 0;
        let rug_fuzz_0 = 12;
        let rug_fuzz_1 = 34;
        let rug_fuzz_2 = 56;
        let rug_fuzz_3 = 789;
        let rug_fuzz_4 = 10_000;
        let mut time = NaiveTime::from_hms_milli_opt(
                rug_fuzz_0,
                rug_fuzz_1,
                rug_fuzz_2,
                rug_fuzz_3,
            )
            .unwrap();
        let time_delta = TimeDelta::milliseconds(-rug_fuzz_4);
        time.add_assign(time_delta);
        debug_assert_eq!(time, NaiveTime::from_hms_milli_opt(12, 34, 46, 789).unwrap());
        let _rug_ed_tests_llm_16_156_rrrruuuugggg_test_add_assign_with_negative_timedelta = 0;
    }
    #[test]
    fn test_add_assign_with_zero_timedelta() {
        let _rug_st_tests_llm_16_156_rrrruuuugggg_test_add_assign_with_zero_timedelta = 0;
        let rug_fuzz_0 = 12;
        let rug_fuzz_1 = 34;
        let rug_fuzz_2 = 56;
        let rug_fuzz_3 = 789;
        let rug_fuzz_4 = 0;
        let mut time = NaiveTime::from_hms_milli_opt(
                rug_fuzz_0,
                rug_fuzz_1,
                rug_fuzz_2,
                rug_fuzz_3,
            )
            .unwrap();
        let time_delta = TimeDelta::milliseconds(rug_fuzz_4);
        time.add_assign(time_delta);
        debug_assert_eq!(time, NaiveTime::from_hms_milli_opt(12, 34, 56, 789).unwrap());
        let _rug_ed_tests_llm_16_156_rrrruuuugggg_test_add_assign_with_zero_timedelta = 0;
    }
    #[test]
    fn test_add_assign_leap_second() {
        let _rug_st_tests_llm_16_156_rrrruuuugggg_test_add_assign_leap_second = 0;
        let rug_fuzz_0 = 23;
        let rug_fuzz_1 = 59;
        let rug_fuzz_2 = 59;
        let rug_fuzz_3 = 1_000;
        let rug_fuzz_4 = 1;
        let mut time = NaiveTime::from_hms_milli_opt(
                rug_fuzz_0,
                rug_fuzz_1,
                rug_fuzz_2,
                rug_fuzz_3,
            )
            .unwrap();
        let time_delta = TimeDelta::seconds(rug_fuzz_4);
        time.add_assign(time_delta);
        debug_assert_eq!(time, NaiveTime::from_hms_milli_opt(0, 0, 0, 0).unwrap());
        let _rug_ed_tests_llm_16_156_rrrruuuugggg_test_add_assign_leap_second = 0;
    }
    #[test]
    fn test_add_assign_with_large_timedelta() {
        let _rug_st_tests_llm_16_156_rrrruuuugggg_test_add_assign_with_large_timedelta = 0;
        let rug_fuzz_0 = 23;
        let rug_fuzz_1 = 59;
        let rug_fuzz_2 = 59;
        let rug_fuzz_3 = 999;
        let rug_fuzz_4 = 86400;
        let mut time = NaiveTime::from_hms_milli_opt(
                rug_fuzz_0,
                rug_fuzz_1,
                rug_fuzz_2,
                rug_fuzz_3,
            )
            .unwrap();
        let time_delta = TimeDelta::seconds(rug_fuzz_4);
        time.add_assign(time_delta);
        debug_assert_eq!(time, NaiveTime::from_hms_milli_opt(23, 59, 59, 999).unwrap());
        let _rug_ed_tests_llm_16_156_rrrruuuugggg_test_add_assign_with_large_timedelta = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_157_llm_16_157 {
    use crate::naive::time::NaiveTime;
    use crate::time_delta::TimeDelta;
    use std::ops::Sub;
    #[test]
    fn test_sub_right_on_leap_second() {
        let _rug_st_tests_llm_16_157_llm_16_157_rrrruuuugggg_test_sub_right_on_leap_second = 0;
        let rug_fuzz_0 = 23;
        let rug_fuzz_1 = 59;
        let rug_fuzz_2 = 59;
        let rug_fuzz_3 = 1_500_000_000;
        let rug_fuzz_4 = 1500;
        let rug_fuzz_5 = 23;
        let rug_fuzz_6 = 59;
        let rug_fuzz_7 = 59;
        let rug_fuzz_8 = 0;
        let time = NaiveTime::from_hms_nano_opt(
                rug_fuzz_0,
                rug_fuzz_1,
                rug_fuzz_2,
                rug_fuzz_3,
            )
            .unwrap();
        let delta = TimeDelta::milliseconds(rug_fuzz_4);
        let result = time.sub(delta);
        let expected = NaiveTime::from_hms_nano_opt(
                rug_fuzz_5,
                rug_fuzz_6,
                rug_fuzz_7,
                rug_fuzz_8,
            )
            .unwrap();
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_157_llm_16_157_rrrruuuugggg_test_sub_right_on_leap_second = 0;
    }
    #[test]
    fn test_sub_rolling_over_midnight() {
        let _rug_st_tests_llm_16_157_llm_16_157_rrrruuuugggg_test_sub_rolling_over_midnight = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 15;
        let rug_fuzz_3 = 30;
        let rug_fuzz_4 = 23;
        let rug_fuzz_5 = 59;
        let rug_fuzz_6 = 45;
        let time = NaiveTime::from_hms_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        let delta = TimeDelta::seconds(rug_fuzz_3);
        let result = time.sub(delta);
        let expected = NaiveTime::from_hms_opt(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6)
            .unwrap();
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_157_llm_16_157_rrrruuuugggg_test_sub_rolling_over_midnight = 0;
    }
    #[test]
    fn test_sub_with_negative_timedelta() {
        let _rug_st_tests_llm_16_157_llm_16_157_rrrruuuugggg_test_sub_with_negative_timedelta = 0;
        let rug_fuzz_0 = 12;
        let rug_fuzz_1 = 30;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 300;
        let rug_fuzz_4 = 12;
        let rug_fuzz_5 = 35;
        let rug_fuzz_6 = 0;
        let time = NaiveTime::from_hms_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        let delta = TimeDelta::seconds(-rug_fuzz_3);
        let result = time.sub(delta);
        let expected = NaiveTime::from_hms_opt(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6)
            .unwrap();
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_157_llm_16_157_rrrruuuugggg_test_sub_with_negative_timedelta = 0;
    }
    #[test]
    fn test_sub_subseconds() {
        let _rug_st_tests_llm_16_157_llm_16_157_rrrruuuugggg_test_sub_subseconds = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 500_000;
        let rug_fuzz_4 = 250_000;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 2;
        let rug_fuzz_7 = 3;
        let rug_fuzz_8 = 250_000;
        let time = NaiveTime::from_hms_micro_opt(
                rug_fuzz_0,
                rug_fuzz_1,
                rug_fuzz_2,
                rug_fuzz_3,
            )
            .unwrap();
        let delta = TimeDelta::microseconds(rug_fuzz_4);
        let result = time.sub(delta);
        let expected = NaiveTime::from_hms_micro_opt(
                rug_fuzz_5,
                rug_fuzz_6,
                rug_fuzz_7,
                rug_fuzz_8,
            )
            .unwrap();
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_157_llm_16_157_rrrruuuugggg_test_sub_subseconds = 0;
    }
    #[test]
    fn test_sub_full_day() {
        let _rug_st_tests_llm_16_157_llm_16_157_rrrruuuugggg_test_sub_full_day = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 30;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 24;
        let time = NaiveTime::from_hms_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        let delta = TimeDelta::hours(-rug_fuzz_3);
        let result = time.sub(delta);
        debug_assert_eq!(result, time);
        let _rug_ed_tests_llm_16_157_llm_16_157_rrrruuuugggg_test_sub_full_day = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_158 {
    use super::*;
    use crate::*;
    use crate::naive::time::NaiveTime;
    use crate::time_delta::TimeDelta;
    #[test]
    fn test_sub_time() {
        let _rug_st_tests_llm_16_158_rrrruuuugggg_test_sub_time = 0;
        let rug_fuzz_0 = 10;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 9;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 10;
        let rug_fuzz_7 = 30;
        let rug_fuzz_8 = 45;
        let rug_fuzz_9 = 9;
        let rug_fuzz_10 = 45;
        let rug_fuzz_11 = 15;
        let rug_fuzz_12 = 10;
        let rug_fuzz_13 = 0;
        let rug_fuzz_14 = 0;
        let rug_fuzz_15 = 0;
        let rug_fuzz_16 = 9;
        let rug_fuzz_17 = 0;
        let rug_fuzz_18 = 0;
        let rug_fuzz_19 = 500_000_000;
        let rug_fuzz_20 = 23;
        let rug_fuzz_21 = 59;
        let rug_fuzz_22 = 59;
        let rug_fuzz_23 = 1_000_000_000;
        let rug_fuzz_24 = 23;
        let rug_fuzz_25 = 59;
        let rug_fuzz_26 = 59;
        let rug_fuzz_27 = 9;
        let rug_fuzz_28 = 0;
        let rug_fuzz_29 = 0;
        let rug_fuzz_30 = 10;
        let rug_fuzz_31 = 0;
        let rug_fuzz_32 = 0;
        let time1 = NaiveTime::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let time2 = NaiveTime::from_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        debug_assert_eq!(time1.sub(time2), TimeDelta::hours(1));
        let time1 = NaiveTime::from_hms(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8);
        let time2 = NaiveTime::from_hms(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11);
        debug_assert_eq!(
            time1.sub(time2), TimeDelta::minutes(45) + TimeDelta::seconds(30)
        );
        let time1 = NaiveTime::from_hms_nano(
            rug_fuzz_12,
            rug_fuzz_13,
            rug_fuzz_14,
            rug_fuzz_15,
        );
        let time2 = NaiveTime::from_hms_nano(
            rug_fuzz_16,
            rug_fuzz_17,
            rug_fuzz_18,
            rug_fuzz_19,
        );
        debug_assert_eq!(
            time1.sub(time2), TimeDelta::hours(1) - TimeDelta::nanoseconds(500_000_000)
        );
        let time1 = NaiveTime::from_hms_nano(
            rug_fuzz_20,
            rug_fuzz_21,
            rug_fuzz_22,
            rug_fuzz_23,
        );
        let time2 = NaiveTime::from_hms(rug_fuzz_24, rug_fuzz_25, rug_fuzz_26);
        debug_assert_eq!(time1.sub(time2), TimeDelta::seconds(1));
        let time1 = NaiveTime::from_hms(rug_fuzz_27, rug_fuzz_28, rug_fuzz_29);
        let time2 = NaiveTime::from_hms(rug_fuzz_30, rug_fuzz_31, rug_fuzz_32);
        debug_assert_eq!(time1.sub(time2), TimeDelta::hours(- 1));
        let _rug_ed_tests_llm_16_158_rrrruuuugggg_test_sub_time = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_159 {
    use super::*;
    use crate::*;
    use crate::naive::time::NaiveTime;
    use crate::time_delta::TimeDelta;
    #[test]
    fn test_sub_assign_positive_delta() {
        let _rug_st_tests_llm_16_159_rrrruuuugggg_test_sub_assign_positive_delta = 0;
        let rug_fuzz_0 = 12;
        let rug_fuzz_1 = 34;
        let rug_fuzz_2 = 56;
        let rug_fuzz_3 = 1234;
        let rug_fuzz_4 = 12;
        let rug_fuzz_5 = 14;
        let rug_fuzz_6 = 42;
        let mut time = NaiveTime::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let delta = TimeDelta::seconds(rug_fuzz_3);
        let expected = NaiveTime::from_hms(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        time -= delta;
        debug_assert_eq!(time, expected);
        let _rug_ed_tests_llm_16_159_rrrruuuugggg_test_sub_assign_positive_delta = 0;
    }
    #[test]
    fn test_sub_assign_negative_delta() {
        let _rug_st_tests_llm_16_159_rrrruuuugggg_test_sub_assign_negative_delta = 0;
        let rug_fuzz_0 = 12;
        let rug_fuzz_1 = 34;
        let rug_fuzz_2 = 56;
        let rug_fuzz_3 = 1234;
        let rug_fuzz_4 = 12;
        let rug_fuzz_5 = 55;
        let rug_fuzz_6 = 10;
        let mut time = NaiveTime::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let delta = TimeDelta::seconds(-rug_fuzz_3);
        let expected = NaiveTime::from_hms(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        time -= delta;
        debug_assert_eq!(time, expected);
        let _rug_ed_tests_llm_16_159_rrrruuuugggg_test_sub_assign_negative_delta = 0;
    }
    #[test]
    fn test_sub_assign_overflow() {
        let _rug_st_tests_llm_16_159_rrrruuuugggg_test_sub_assign_overflow = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 86400;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 0;
        let mut time = NaiveTime::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let delta = TimeDelta::seconds(rug_fuzz_3);
        let expected = NaiveTime::from_hms(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        time -= delta;
        debug_assert_eq!(time, expected);
        let _rug_ed_tests_llm_16_159_rrrruuuugggg_test_sub_assign_overflow = 0;
    }
    #[test]
    fn test_sub_assign_underflow() {
        let _rug_st_tests_llm_16_159_rrrruuuugggg_test_sub_assign_underflow = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 86400;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 0;
        let mut time = NaiveTime::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let delta = TimeDelta::seconds(-rug_fuzz_3);
        let expected = NaiveTime::from_hms(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        time -= delta;
        debug_assert_eq!(time, expected);
        let _rug_ed_tests_llm_16_159_rrrruuuugggg_test_sub_assign_underflow = 0;
    }
    #[test]
    fn test_sub_assign_leap_second() {
        let _rug_st_tests_llm_16_159_rrrruuuugggg_test_sub_assign_leap_second = 0;
        let rug_fuzz_0 = 23;
        let rug_fuzz_1 = 59;
        let rug_fuzz_2 = 59;
        let rug_fuzz_3 = 1_000_000_000;
        let rug_fuzz_4 = 60;
        let rug_fuzz_5 = 23;
        let rug_fuzz_6 = 59;
        let rug_fuzz_7 = 59;
        let mut time = NaiveTime::from_hms_nano(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
        );
        let delta = TimeDelta::seconds(rug_fuzz_4);
        let expected = NaiveTime::from_hms(rug_fuzz_5, rug_fuzz_6, rug_fuzz_7);
        time -= delta;
        debug_assert_eq!(time, expected);
        let _rug_ed_tests_llm_16_159_rrrruuuugggg_test_sub_assign_leap_second = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_160 {
    use crate::naive::time::NaiveTime;
    use std::str::FromStr;
    #[test]
    fn from_str_valid_times() {
        let _rug_st_tests_llm_16_160_rrrruuuugggg_from_str_valid_times = 0;
        let rug_fuzz_0 = "23:59:59";
        let rug_fuzz_1 = "00:00:00";
        let rug_fuzz_2 = "12:34:56";
        let rug_fuzz_3 = "23:59:59.999999999";
        debug_assert_eq!(
            NaiveTime::from_str(rug_fuzz_0).unwrap(), NaiveTime::from_hms(23, 59, 59)
        );
        debug_assert_eq!(
            NaiveTime::from_str(rug_fuzz_1).unwrap(), NaiveTime::from_hms(0, 0, 0)
        );
        debug_assert_eq!(
            NaiveTime::from_str(rug_fuzz_2).unwrap(), NaiveTime::from_hms(12, 34, 56)
        );
        debug_assert_eq!(
            NaiveTime::from_str(rug_fuzz_3).unwrap(), NaiveTime::from_hms_nano(23, 59,
            59, 999999999)
        );
        let _rug_ed_tests_llm_16_160_rrrruuuugggg_from_str_valid_times = 0;
    }
    #[test]
    fn from_str_invalid_times() {
        let _rug_st_tests_llm_16_160_rrrruuuugggg_from_str_invalid_times = 0;
        let rug_fuzz_0 = "24:00:00";
        let rug_fuzz_1 = "23:60:00";
        let rug_fuzz_2 = "23:59:60";
        let rug_fuzz_3 = "23:59::";
        let rug_fuzz_4 = "::";
        let rug_fuzz_5 = "23:59";
        let rug_fuzz_6 = "asdf";
        let rug_fuzz_7 = "23:59:59:";
        let rug_fuzz_8 = "23:59:59.9999999999";
        debug_assert!(NaiveTime::from_str(rug_fuzz_0).is_err());
        debug_assert!(NaiveTime::from_str(rug_fuzz_1).is_err());
        debug_assert!(NaiveTime::from_str(rug_fuzz_2).is_err());
        debug_assert!(NaiveTime::from_str(rug_fuzz_3).is_err());
        debug_assert!(NaiveTime::from_str(rug_fuzz_4).is_err());
        debug_assert!(NaiveTime::from_str(rug_fuzz_5).is_err());
        debug_assert!(NaiveTime::from_str(rug_fuzz_6).is_err());
        debug_assert!(NaiveTime::from_str(rug_fuzz_7).is_err());
        debug_assert!(NaiveTime::from_str(rug_fuzz_8).is_err());
        let _rug_ed_tests_llm_16_160_rrrruuuugggg_from_str_invalid_times = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_161 {
    use crate::NaiveTime;
    use crate::Timelike;
    #[test]
    fn test_hour() {
        let _rug_st_tests_llm_16_161_rrrruuuugggg_test_hour = 0;
        let rug_fuzz_0 = 12;
        let rug_fuzz_1 = 34;
        let rug_fuzz_2 = 56;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 23;
        let rug_fuzz_7 = 59;
        let rug_fuzz_8 = 59;
        let t = NaiveTime::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(t.hour(), 12);
        let t = NaiveTime::from_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        debug_assert_eq!(t.hour(), 0);
        let t = NaiveTime::from_hms(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8);
        debug_assert_eq!(t.hour(), 23);
        let _rug_ed_tests_llm_16_161_rrrruuuugggg_test_hour = 0;
    }
    #[test]
    fn test_hour_leap_second() {
        let _rug_st_tests_llm_16_161_rrrruuuugggg_test_hour_leap_second = 0;
        let rug_fuzz_0 = 23;
        let rug_fuzz_1 = 59;
        let rug_fuzz_2 = 59;
        let rug_fuzz_3 = 1_000;
        let t = NaiveTime::from_hms_milli(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
        );
        debug_assert_eq!(t.hour(), 23);
        let _rug_ed_tests_llm_16_161_rrrruuuugggg_test_hour_leap_second = 0;
    }
    #[test]
    #[should_panic]
    fn test_hour_invalid_time() {
        let _rug_st_tests_llm_16_161_rrrruuuugggg_test_hour_invalid_time = 0;
        let rug_fuzz_0 = 24;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        NaiveTime::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let _rug_ed_tests_llm_16_161_rrrruuuugggg_test_hour_invalid_time = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_162 {
    use crate::naive::time::NaiveTime;
    use crate::Timelike;
    #[test]
    fn test_minute() {
        let _rug_st_tests_llm_16_162_rrrruuuugggg_test_minute = 0;
        let rug_fuzz_0 = 12;
        let rug_fuzz_1 = 30;
        let rug_fuzz_2 = 45;
        let rug_fuzz_3 = 23;
        let rug_fuzz_4 = 59;
        let rug_fuzz_5 = 59;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 0;
        let rug_fuzz_9 = 23;
        let rug_fuzz_10 = 45;
        let rug_fuzz_11 = 11;
        let rug_fuzz_12 = 7;
        let rug_fuzz_13 = 15;
        let rug_fuzz_14 = 23;
        let rug_fuzz_15 = 7;
        let rug_fuzz_16 = 59;
        let rug_fuzz_17 = 59;
        let time = NaiveTime::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(time.minute(), 30);
        let time = NaiveTime::from_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        debug_assert_eq!(time.minute(), 59);
        let time = NaiveTime::from_hms(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8);
        debug_assert_eq!(time.minute(), 0);
        let time = NaiveTime::from_hms(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11);
        debug_assert_eq!(time.minute(), 45);
        let time = NaiveTime::from_hms(rug_fuzz_12, rug_fuzz_13, rug_fuzz_14);
        debug_assert_eq!(time.minute(), 15);
        let time = NaiveTime::from_hms(rug_fuzz_15, rug_fuzz_16, rug_fuzz_17);
        debug_assert_eq!(time.minute(), 59);
        let _rug_ed_tests_llm_16_162_rrrruuuugggg_test_minute = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_163 {
    use crate::naive::time::NaiveTime;
    use crate::Timelike;
    #[test]
    fn test_nanosecond() {
        let _rug_st_tests_llm_16_163_rrrruuuugggg_test_nanosecond = 0;
        let rug_fuzz_0 = 10;
        let rug_fuzz_1 = 20;
        let rug_fuzz_2 = 30;
        let rug_fuzz_3 = 40;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 123;
        let rug_fuzz_8 = 23;
        let rug_fuzz_9 = 59;
        let rug_fuzz_10 = 59;
        let rug_fuzz_11 = 999_999_999;
        let rug_fuzz_12 = 23;
        let rug_fuzz_13 = 59;
        let rug_fuzz_14 = 59;
        let rug_fuzz_15 = 1_000_000_000;
        let rug_fuzz_16 = 23;
        let rug_fuzz_17 = 59;
        let rug_fuzz_18 = 59;
        let rug_fuzz_19 = 1_999_999_999;
        let rug_fuzz_20 = 23;
        let rug_fuzz_21 = 59;
        let rug_fuzz_22 = 59;
        let rug_fuzz_23 = 2_000_000_000;
        debug_assert_eq!(
            NaiveTime::from_hms_nano_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3)
            .unwrap().nanosecond(), 40
        );
        debug_assert_eq!(
            NaiveTime::from_hms_nano_opt(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6, rug_fuzz_7)
            .unwrap().nanosecond(), 123
        );
        debug_assert_eq!(
            NaiveTime::from_hms_nano_opt(rug_fuzz_8, rug_fuzz_9, rug_fuzz_10,
            rug_fuzz_11).unwrap().nanosecond(), 999_999_999
        );
        debug_assert_eq!(
            NaiveTime::from_hms_nano_opt(rug_fuzz_12, rug_fuzz_13, rug_fuzz_14,
            rug_fuzz_15).unwrap().nanosecond(), 1_000_000_000
        );
        debug_assert_eq!(
            NaiveTime::from_hms_nano_opt(rug_fuzz_16, rug_fuzz_17, rug_fuzz_18,
            rug_fuzz_19).unwrap().nanosecond(), 1_999_999_999
        );
        debug_assert!(
            NaiveTime::from_hms_nano_opt(rug_fuzz_20, rug_fuzz_21, rug_fuzz_22,
            rug_fuzz_23).is_none(), "Expected invalid NaiveTime due to large nanosecond"
        );
        let _rug_ed_tests_llm_16_163_rrrruuuugggg_test_nanosecond = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_164 {
    use crate::{Timelike, NaiveTime};
    #[test]
    fn test_num_seconds_from_midnight() {
        let _rug_st_tests_llm_16_164_rrrruuuugggg_test_num_seconds_from_midnight = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 23;
        let rug_fuzz_7 = 59;
        let rug_fuzz_8 = 59;
        let rug_fuzz_9 = 23;
        let rug_fuzz_10 = 59;
        let rug_fuzz_11 = 59;
        let rug_fuzz_12 = 1_000_000_000;
        let time1 = NaiveTime::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(time1.num_seconds_from_midnight(), 0);
        let time2 = NaiveTime::from_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        debug_assert_eq!(time2.num_seconds_from_midnight(), 3600);
        let time3 = NaiveTime::from_hms(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8);
        debug_assert_eq!(time3.num_seconds_from_midnight(), 86399);
        let time4 = NaiveTime::from_hms_nano(
            rug_fuzz_9,
            rug_fuzz_10,
            rug_fuzz_11,
            rug_fuzz_12,
        );
        debug_assert_eq!(time4.num_seconds_from_midnight(), 86399);
        let _rug_ed_tests_llm_16_164_rrrruuuugggg_test_num_seconds_from_midnight = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_165 {
    use super::*;
    use crate::*;
    use crate::Timelike;
    #[test]
    fn test_second() {
        let _rug_st_tests_llm_16_165_rrrruuuugggg_test_second = 0;
        let rug_fuzz_0 = 12;
        let rug_fuzz_1 = 30;
        let rug_fuzz_2 = 45;
        let rug_fuzz_3 = 23;
        let rug_fuzz_4 = 59;
        let rug_fuzz_5 = 59;
        let rug_fuzz_6 = 23;
        let rug_fuzz_7 = 59;
        let rug_fuzz_8 = 59;
        let rug_fuzz_9 = 1_000;
        let rug_fuzz_10 = 23;
        let rug_fuzz_11 = 59;
        let rug_fuzz_12 = 59;
        let rug_fuzz_13 = 1_999_999_999;
        let rug_fuzz_14 = 0;
        let rug_fuzz_15 = 0;
        let rug_fuzz_16 = 0;
        let rug_fuzz_17 = 0;
        let time = NaiveTime::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(time.second(), 45);
        let time = NaiveTime::from_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        debug_assert_eq!(time.second(), 59);
        let time = NaiveTime::from_hms_milli(
            rug_fuzz_6,
            rug_fuzz_7,
            rug_fuzz_8,
            rug_fuzz_9,
        );
        debug_assert_eq!(time.second(), 59);
        let time = NaiveTime::from_hms_nano(
            rug_fuzz_10,
            rug_fuzz_11,
            rug_fuzz_12,
            rug_fuzz_13,
        );
        debug_assert_eq!(time.second(), 59);
        let time = NaiveTime::from_hms_nano(
            rug_fuzz_14,
            rug_fuzz_15,
            rug_fuzz_16,
            rug_fuzz_17,
        );
        debug_assert_eq!(time.second(), 0);
        let _rug_ed_tests_llm_16_165_rrrruuuugggg_test_second = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_166 {
    use crate::NaiveTime;
    use crate::Timelike;
    #[test]
    fn test_with_hour() {
        let _rug_st_tests_llm_16_166_rrrruuuugggg_test_with_hour = 0;
        let rug_fuzz_0 = 10;
        let rug_fuzz_1 = 20;
        let rug_fuzz_2 = 30;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 23;
        let rug_fuzz_5 = 24;
        let time = NaiveTime::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(
            time.with_hour(rug_fuzz_3), Some(NaiveTime::from_hms(0, 20, 30))
        );
        debug_assert_eq!(
            time.with_hour(rug_fuzz_4), Some(NaiveTime::from_hms(23, 20, 30))
        );
        debug_assert_eq!(time.with_hour(rug_fuzz_5), None);
        let _rug_ed_tests_llm_16_166_rrrruuuugggg_test_with_hour = 0;
    }
    #[test]
    fn test_with_hour_edge_cases() {
        let _rug_st_tests_llm_16_166_rrrruuuugggg_test_with_hour_edge_cases = 0;
        let rug_fuzz_0 = 23;
        let rug_fuzz_1 = 59;
        let rug_fuzz_2 = 59;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 23;
        let rug_fuzz_5 = 24;
        let time = NaiveTime::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(
            time.with_hour(rug_fuzz_3), Some(NaiveTime::from_hms(0, 59, 59))
        );
        debug_assert_eq!(
            time.with_hour(rug_fuzz_4), Some(NaiveTime::from_hms(23, 59, 59))
        );
        debug_assert_eq!(time.with_hour(rug_fuzz_5), None);
        let _rug_ed_tests_llm_16_166_rrrruuuugggg_test_with_hour_edge_cases = 0;
    }
    #[test]
    fn test_with_hour_leap_second() {
        let _rug_st_tests_llm_16_166_rrrruuuugggg_test_with_hour_leap_second = 0;
        let rug_fuzz_0 = 23;
        let rug_fuzz_1 = 59;
        let rug_fuzz_2 = 59;
        let rug_fuzz_3 = 1_500;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 23;
        let rug_fuzz_6 = 24;
        let time = NaiveTime::from_hms_milli(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
        );
        debug_assert_eq!(
            time.with_hour(rug_fuzz_4), Some(NaiveTime::from_hms_milli(0, 59, 59, 1_500))
        );
        debug_assert_eq!(
            time.with_hour(rug_fuzz_5), Some(NaiveTime::from_hms_milli(23, 59, 59,
            1_500))
        );
        debug_assert_eq!(time.with_hour(rug_fuzz_6), None);
        let _rug_ed_tests_llm_16_166_rrrruuuugggg_test_with_hour_leap_second = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_167 {
    use crate::{NaiveTime, Timelike};
    #[test]
    fn test_with_minute() {
        let _rug_st_tests_llm_16_167_rrrruuuugggg_test_with_minute = 0;
        let rug_fuzz_0 = 12;
        let rug_fuzz_1 = 30;
        let rug_fuzz_2 = 45;
        let rug_fuzz_3 = 15;
        let rug_fuzz_4 = 60;
        let rug_fuzz_5 = 61;
        let original_time = NaiveTime::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        if let Some(updated_time) = original_time.with_minute(rug_fuzz_3) {
            debug_assert_eq!(updated_time, NaiveTime::from_hms(12, 15, 45));
        } else {
            panic!("with_minute(15) should not return None");
        }
        debug_assert!(
            original_time.with_minute(rug_fuzz_4).is_none(),
            "with_minute(60) should return None"
        );
        debug_assert!(
            original_time.with_minute(rug_fuzz_5).is_none(),
            "with_minute(61) should return None"
        );
        let _rug_ed_tests_llm_16_167_rrrruuuugggg_test_with_minute = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_168 {
    use super::*;
    use crate::*;
    use crate::Timelike;
    #[test]
    fn test_with_nanosecond_valid() {
        let _rug_st_tests_llm_16_168_rrrruuuugggg_test_with_nanosecond_valid = 0;
        let rug_fuzz_0 = 23;
        let rug_fuzz_1 = 56;
        let rug_fuzz_2 = 4;
        let rug_fuzz_3 = 12_345_678;
        let rug_fuzz_4 = 333_333_333;
        let time = NaiveTime::from_hms_nano_opt(
                rug_fuzz_0,
                rug_fuzz_1,
                rug_fuzz_2,
                rug_fuzz_3,
            )
            .unwrap();
        let new_nano = rug_fuzz_4;
        let time_with_nano = time.with_nanosecond(new_nano).unwrap();
        debug_assert_eq!(time_with_nano.nanosecond(), new_nano);
        let _rug_ed_tests_llm_16_168_rrrruuuugggg_test_with_nanosecond_valid = 0;
    }
    #[test]
    fn test_with_nanosecond_none() {
        let _rug_st_tests_llm_16_168_rrrruuuugggg_test_with_nanosecond_none = 0;
        let rug_fuzz_0 = 23;
        let rug_fuzz_1 = 56;
        let rug_fuzz_2 = 4;
        let rug_fuzz_3 = 12_345_678;
        let rug_fuzz_4 = 2_000_000_000;
        let time = NaiveTime::from_hms_nano_opt(
                rug_fuzz_0,
                rug_fuzz_1,
                rug_fuzz_2,
                rug_fuzz_3,
            )
            .unwrap();
        let new_nano = rug_fuzz_4;
        debug_assert!(time.with_nanosecond(new_nano).is_none());
        let _rug_ed_tests_llm_16_168_rrrruuuugggg_test_with_nanosecond_none = 0;
    }
    #[test]
    fn test_with_nanosecond_leap_second() {
        let _rug_st_tests_llm_16_168_rrrruuuugggg_test_with_nanosecond_leap_second = 0;
        let rug_fuzz_0 = 23;
        let rug_fuzz_1 = 56;
        let rug_fuzz_2 = 4;
        let rug_fuzz_3 = 12_345_678;
        let rug_fuzz_4 = 1_333_333_333;
        let time = NaiveTime::from_hms_nano_opt(
                rug_fuzz_0,
                rug_fuzz_1,
                rug_fuzz_2,
                rug_fuzz_3,
            )
            .unwrap();
        let new_nano = rug_fuzz_4;
        let time_with_nano = time.with_nanosecond(new_nano).unwrap();
        debug_assert_eq!(time_with_nano.nanosecond(), new_nano);
        let _rug_ed_tests_llm_16_168_rrrruuuugggg_test_with_nanosecond_leap_second = 0;
    }
    #[test]
    fn test_with_nanosecond_edge_case() {
        let _rug_st_tests_llm_16_168_rrrruuuugggg_test_with_nanosecond_edge_case = 0;
        let rug_fuzz_0 = 23;
        let rug_fuzz_1 = 56;
        let rug_fuzz_2 = 4;
        let rug_fuzz_3 = 12_345_678;
        let rug_fuzz_4 = 1_000_000_000;
        let time = NaiveTime::from_hms_nano_opt(
                rug_fuzz_0,
                rug_fuzz_1,
                rug_fuzz_2,
                rug_fuzz_3,
            )
            .unwrap();
        let new_nano = rug_fuzz_4;
        let time_with_nano = time.with_nanosecond(new_nano).unwrap();
        debug_assert_eq!(time_with_nano.nanosecond(), new_nano);
        let _rug_ed_tests_llm_16_168_rrrruuuugggg_test_with_nanosecond_edge_case = 0;
    }
    #[test]
    fn test_with_nanosecond_maximum() {
        let _rug_st_tests_llm_16_168_rrrruuuugggg_test_with_nanosecond_maximum = 0;
        let rug_fuzz_0 = 23;
        let rug_fuzz_1 = 56;
        let rug_fuzz_2 = 4;
        let rug_fuzz_3 = 12_345_678;
        let rug_fuzz_4 = 1_999_999_999;
        let time = NaiveTime::from_hms_nano_opt(
                rug_fuzz_0,
                rug_fuzz_1,
                rug_fuzz_2,
                rug_fuzz_3,
            )
            .unwrap();
        let new_nano = rug_fuzz_4;
        let time_with_nano = time.with_nanosecond(new_nano).unwrap();
        debug_assert_eq!(time_with_nano.nanosecond(), new_nano);
        let _rug_ed_tests_llm_16_168_rrrruuuugggg_test_with_nanosecond_maximum = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_169 {
    use super::*;
    use crate::*;
    use crate::{NaiveTime, Timelike};
    #[test]
    fn test_with_second_valid() {
        let _rug_st_tests_llm_16_169_rrrruuuugggg_test_with_second_valid = 0;
        let rug_fuzz_0 = 23;
        let rug_fuzz_1 = 59;
        let rug_fuzz_2 = 45;
        let rug_fuzz_3 = 30;
        let time = NaiveTime::from_hms_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        let result = time.with_second(rug_fuzz_3);
        debug_assert_eq!(result, Some(NaiveTime::from_hms_opt(23, 59, 30).unwrap()));
        let _rug_ed_tests_llm_16_169_rrrruuuugggg_test_with_second_valid = 0;
    }
    #[test]
    fn test_with_second_invalid() {
        let _rug_st_tests_llm_16_169_rrrruuuugggg_test_with_second_invalid = 0;
        let rug_fuzz_0 = 23;
        let rug_fuzz_1 = 59;
        let rug_fuzz_2 = 45;
        let rug_fuzz_3 = 60;
        let time = NaiveTime::from_hms_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        let result = time.with_second(rug_fuzz_3);
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_169_rrrruuuugggg_test_with_second_invalid = 0;
    }
    #[test]
    fn test_with_second_boundary() {
        let _rug_st_tests_llm_16_169_rrrruuuugggg_test_with_second_boundary = 0;
        let rug_fuzz_0 = 23;
        let rug_fuzz_1 = 59;
        let rug_fuzz_2 = 45;
        let rug_fuzz_3 = 59;
        let time = NaiveTime::from_hms_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        let result = time.with_second(rug_fuzz_3);
        debug_assert_eq!(result, Some(NaiveTime::from_hms_opt(23, 59, 59).unwrap()));
        let _rug_ed_tests_llm_16_169_rrrruuuugggg_test_with_second_boundary = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_460 {
    use super::*;
    use crate::*;
    use crate::NaiveTime;
    #[test]
    fn test_naive_time_format() {
        let _rug_st_tests_llm_16_460_rrrruuuugggg_test_naive_time_format = 0;
        let rug_fuzz_0 = 23;
        let rug_fuzz_1 = 56;
        let rug_fuzz_2 = 4;
        let rug_fuzz_3 = 12_345_678;
        let rug_fuzz_4 = "%H:%M:%S";
        let rug_fuzz_5 = "%H:%M:%S%.6f";
        let rug_fuzz_6 = "%-I:%M %p";
        let rug_fuzz_7 = "%H:%M";
        let rug_fuzz_8 = "%-H:%-M:%-S";
        let rug_fuzz_9 = "%H:%M:%S %P";
        let time = NaiveTime::from_hms_nano_opt(
                rug_fuzz_0,
                rug_fuzz_1,
                rug_fuzz_2,
                rug_fuzz_3,
            )
            .unwrap();
        debug_assert_eq!(time.format(rug_fuzz_4).to_string(), "23:56:04");
        debug_assert_eq!(time.format(rug_fuzz_5).to_string(), "23:56:04.012345");
        debug_assert_eq!(time.format(rug_fuzz_6).to_string(), "11:56 PM");
        debug_assert_eq!(time.format(rug_fuzz_7).to_string(), "23:56");
        debug_assert_eq!(time.format(rug_fuzz_8).to_string(), "23:56:4");
        debug_assert_eq!(time.format(rug_fuzz_9).to_string(), "23:56:04 pm");
        let _rug_ed_tests_llm_16_460_rrrruuuugggg_test_naive_time_format = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_461 {
    use super::*;
    use crate::*;
    use crate::format::strftime::StrftimeItems;
    #[test]
    fn test_format_with_items() {
        let _rug_st_tests_llm_16_461_rrrruuuugggg_test_format_with_items = 0;
        let rug_fuzz_0 = 10;
        let rug_fuzz_1 = 11;
        let rug_fuzz_2 = 12;
        let rug_fuzz_3 = "%H:%M:%S";
        let t = NaiveTime::from_hms_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        let fmt = StrftimeItems::new(rug_fuzz_3);
        debug_assert_eq!(t.format_with_items(fmt.clone()).to_string(), "10:11:12");
        let _rug_ed_tests_llm_16_461_rrrruuuugggg_test_format_with_items = 0;
    }
    #[test]
    fn test_format_with_items_leap_second() {
        let _rug_st_tests_llm_16_461_rrrruuuugggg_test_format_with_items_leap_second = 0;
        let rug_fuzz_0 = 23;
        let rug_fuzz_1 = 59;
        let rug_fuzz_2 = 59;
        let rug_fuzz_3 = 1_000_000_000;
        let rug_fuzz_4 = "%H:%M:%S";
        let t = NaiveTime::from_hms_nano_opt(
                rug_fuzz_0,
                rug_fuzz_1,
                rug_fuzz_2,
                rug_fuzz_3,
            )
            .unwrap();
        let fmt = StrftimeItems::new(rug_fuzz_4);
        debug_assert_eq!(t.format_with_items(fmt.clone()).to_string(), "23:59:60");
        let _rug_ed_tests_llm_16_461_rrrruuuugggg_test_format_with_items_leap_second = 0;
    }
    #[test]
    fn test_format_with_items_padding() {
        let _rug_st_tests_llm_16_461_rrrruuuugggg_test_format_with_items_padding = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = "%H:%M:%S";
        let t = NaiveTime::from_hms_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        let fmt = StrftimeItems::new(rug_fuzz_3);
        debug_assert_eq!(t.format_with_items(fmt.clone()).to_string(), "01:02:03");
        let _rug_ed_tests_llm_16_461_rrrruuuugggg_test_format_with_items_padding = 0;
    }
    #[test]
    fn test_format_with_items_24h_edge() {
        let _rug_st_tests_llm_16_461_rrrruuuugggg_test_format_with_items_24h_edge = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = "%H:%M:%S";
        let t = NaiveTime::from_hms_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        let fmt = StrftimeItems::new(rug_fuzz_3);
        debug_assert_eq!(t.format_with_items(fmt.clone()).to_string(), "00:00:00");
        let _rug_ed_tests_llm_16_461_rrrruuuugggg_test_format_with_items_24h_edge = 0;
    }
    #[test]
    #[should_panic]
    fn test_format_with_items_invalid_time() {
        let _rug_st_tests_llm_16_461_rrrruuuugggg_test_format_with_items_invalid_time = 0;
        let rug_fuzz_0 = 24;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = "%H:%M:%S";
        let t = NaiveTime::from_hms_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        let fmt = StrftimeItems::new(rug_fuzz_3);
        let _ = t.format_with_items(fmt.clone()).to_string();
        let _rug_ed_tests_llm_16_461_rrrruuuugggg_test_format_with_items_invalid_time = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_462 {
    use crate::NaiveTime;
    use crate::Timelike;
    #[test]
    fn test_from_hms_valid() {
        let _rug_st_tests_llm_16_462_rrrruuuugggg_test_from_hms_valid = 0;
        let rug_fuzz_0 = 8;
        let rug_fuzz_1 = 30;
        let rug_fuzz_2 = 45;
        let time = NaiveTime::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(time.hour(), 8);
        debug_assert_eq!(time.minute(), 30);
        debug_assert_eq!(time.second(), 45);
        let _rug_ed_tests_llm_16_462_rrrruuuugggg_test_from_hms_valid = 0;
    }
    #[test]
    #[should_panic(expected = "invalid time")]
    fn test_from_hms_invalid_hour() {
        let _rug_st_tests_llm_16_462_rrrruuuugggg_test_from_hms_invalid_hour = 0;
        let rug_fuzz_0 = 24;
        let rug_fuzz_1 = 30;
        let rug_fuzz_2 = 45;
        NaiveTime::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let _rug_ed_tests_llm_16_462_rrrruuuugggg_test_from_hms_invalid_hour = 0;
    }
    #[test]
    #[should_panic(expected = "invalid time")]
    fn test_from_hms_invalid_minute() {
        let _rug_st_tests_llm_16_462_rrrruuuugggg_test_from_hms_invalid_minute = 0;
        let rug_fuzz_0 = 8;
        let rug_fuzz_1 = 60;
        let rug_fuzz_2 = 45;
        NaiveTime::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let _rug_ed_tests_llm_16_462_rrrruuuugggg_test_from_hms_invalid_minute = 0;
    }
    #[test]
    #[should_panic(expected = "invalid time")]
    fn test_from_hms_invalid_second() {
        let _rug_st_tests_llm_16_462_rrrruuuugggg_test_from_hms_invalid_second = 0;
        let rug_fuzz_0 = 8;
        let rug_fuzz_1 = 30;
        let rug_fuzz_2 = 60;
        NaiveTime::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let _rug_ed_tests_llm_16_462_rrrruuuugggg_test_from_hms_invalid_second = 0;
    }
    #[test]
    fn test_from_hms_opt_valid() {
        let _rug_st_tests_llm_16_462_rrrruuuugggg_test_from_hms_opt_valid = 0;
        let rug_fuzz_0 = 8;
        let rug_fuzz_1 = 30;
        let rug_fuzz_2 = 45;
        let time_opt = NaiveTime::from_hms_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        debug_assert!(time_opt.is_some());
        let time = time_opt.unwrap();
        debug_assert_eq!(time.hour(), 8);
        debug_assert_eq!(time.minute(), 30);
        debug_assert_eq!(time.second(), 45);
        let _rug_ed_tests_llm_16_462_rrrruuuugggg_test_from_hms_opt_valid = 0;
    }
    #[test]
    fn test_from_hms_opt_invalid_hour() {
        let _rug_st_tests_llm_16_462_rrrruuuugggg_test_from_hms_opt_invalid_hour = 0;
        let rug_fuzz_0 = 24;
        let rug_fuzz_1 = 30;
        let rug_fuzz_2 = 45;
        debug_assert!(
            NaiveTime::from_hms_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).is_none()
        );
        let _rug_ed_tests_llm_16_462_rrrruuuugggg_test_from_hms_opt_invalid_hour = 0;
    }
    #[test]
    fn test_from_hms_opt_invalid_minute() {
        let _rug_st_tests_llm_16_462_rrrruuuugggg_test_from_hms_opt_invalid_minute = 0;
        let rug_fuzz_0 = 8;
        let rug_fuzz_1 = 60;
        let rug_fuzz_2 = 45;
        debug_assert!(
            NaiveTime::from_hms_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).is_none()
        );
        let _rug_ed_tests_llm_16_462_rrrruuuugggg_test_from_hms_opt_invalid_minute = 0;
    }
    #[test]
    fn test_from_hms_opt_invalid_second() {
        let _rug_st_tests_llm_16_462_rrrruuuugggg_test_from_hms_opt_invalid_second = 0;
        let rug_fuzz_0 = 8;
        let rug_fuzz_1 = 30;
        let rug_fuzz_2 = 60;
        debug_assert!(
            NaiveTime::from_hms_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).is_none()
        );
        let _rug_ed_tests_llm_16_462_rrrruuuugggg_test_from_hms_opt_invalid_second = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_463 {
    use crate::naive::time::NaiveTime;
    #[test]
    fn test_from_hms_micro_valid() {
        let _rug_st_tests_llm_16_463_rrrruuuugggg_test_from_hms_micro_valid = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = 20;
        let rug_fuzz_3 = 304000;
        debug_assert_eq!(
            NaiveTime::from_hms_micro(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3),
            NaiveTime::from_hms_micro_opt(5, 10, 20, 304000).unwrap()
        );
        let _rug_ed_tests_llm_16_463_rrrruuuugggg_test_from_hms_micro_valid = 0;
    }
    #[test]
    #[should_panic(expected = "invalid time")]
    fn test_from_hms_micro_invalid() {
        let _rug_st_tests_llm_16_463_rrrruuuugggg_test_from_hms_micro_invalid = 0;
        let rug_fuzz_0 = 25;
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = 20;
        let rug_fuzz_3 = 304000;
        NaiveTime::from_hms_micro(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        let _rug_ed_tests_llm_16_463_rrrruuuugggg_test_from_hms_micro_invalid = 0;
    }
    #[test]
    fn test_from_hms_micro_leap_second() {
        let _rug_st_tests_llm_16_463_rrrruuuugggg_test_from_hms_micro_leap_second = 0;
        let rug_fuzz_0 = 23;
        let rug_fuzz_1 = 59;
        let rug_fuzz_2 = 59;
        let rug_fuzz_3 = 2000000;
        debug_assert_eq!(
            NaiveTime::from_hms_micro(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3),
            NaiveTime::from_hms_micro_opt(23, 59, 59, 2000000).unwrap()
        );
        let _rug_ed_tests_llm_16_463_rrrruuuugggg_test_from_hms_micro_leap_second = 0;
    }
    #[test]
    fn test_from_hms_micro_boundary_conditions() {
        let _rug_st_tests_llm_16_463_rrrruuuugggg_test_from_hms_micro_boundary_conditions = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 23;
        let rug_fuzz_5 = 59;
        let rug_fuzz_6 = 59;
        let rug_fuzz_7 = 999999;
        debug_assert_eq!(
            NaiveTime::from_hms_micro(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3),
            NaiveTime::from_hms_micro_opt(0, 0, 0, 0).unwrap()
        );
        debug_assert_eq!(
            NaiveTime::from_hms_micro(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6, rug_fuzz_7),
            NaiveTime::from_hms_micro_opt(23, 59, 59, 999999).unwrap()
        );
        let _rug_ed_tests_llm_16_463_rrrruuuugggg_test_from_hms_micro_boundary_conditions = 0;
    }
    #[test]
    #[should_panic(expected = "invalid time")]
    fn test_from_hms_micro_invalid_hour() {
        let _rug_st_tests_llm_16_463_rrrruuuugggg_test_from_hms_micro_invalid_hour = 0;
        let rug_fuzz_0 = 24;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        NaiveTime::from_hms_micro(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        let _rug_ed_tests_llm_16_463_rrrruuuugggg_test_from_hms_micro_invalid_hour = 0;
    }
    #[test]
    #[should_panic(expected = "invalid time")]
    fn test_from_hms_micro_invalid_minute() {
        let _rug_st_tests_llm_16_463_rrrruuuugggg_test_from_hms_micro_invalid_minute = 0;
        let rug_fuzz_0 = 23;
        let rug_fuzz_1 = 60;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        NaiveTime::from_hms_micro(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        let _rug_ed_tests_llm_16_463_rrrruuuugggg_test_from_hms_micro_invalid_minute = 0;
    }
    #[test]
    #[should_panic(expected = "invalid time")]
    fn test_from_hms_micro_invalid_second() {
        let _rug_st_tests_llm_16_463_rrrruuuugggg_test_from_hms_micro_invalid_second = 0;
        let rug_fuzz_0 = 23;
        let rug_fuzz_1 = 59;
        let rug_fuzz_2 = 60;
        let rug_fuzz_3 = 0;
        NaiveTime::from_hms_micro(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        let _rug_ed_tests_llm_16_463_rrrruuuugggg_test_from_hms_micro_invalid_second = 0;
    }
    #[test]
    #[should_panic(expected = "invalid time")]
    fn test_from_hms_micro_invalid_micro() {
        let _rug_st_tests_llm_16_463_rrrruuuugggg_test_from_hms_micro_invalid_micro = 0;
        let rug_fuzz_0 = 23;
        let rug_fuzz_1 = 59;
        let rug_fuzz_2 = 59;
        let rug_fuzz_3 = 2000000;
        NaiveTime::from_hms_micro(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        let _rug_ed_tests_llm_16_463_rrrruuuugggg_test_from_hms_micro_invalid_micro = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_464 {
    use super::*;
    use crate::*;
    #[test]
    fn test_from_hms_micro_opt_valid_times() {
        let _rug_st_tests_llm_16_464_rrrruuuugggg_test_from_hms_micro_opt_valid_times = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 23;
        let rug_fuzz_5 = 59;
        let rug_fuzz_6 = 59;
        let rug_fuzz_7 = 999_999;
        let rug_fuzz_8 = 23;
        let rug_fuzz_9 = 59;
        let rug_fuzz_10 = 59;
        let rug_fuzz_11 = 1_999_999;
        debug_assert!(
            NaiveTime::from_hms_micro_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3)
            .is_some()
        );
        debug_assert!(
            NaiveTime::from_hms_micro_opt(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6, rug_fuzz_7)
            .is_some()
        );
        debug_assert!(
            NaiveTime::from_hms_micro_opt(rug_fuzz_8, rug_fuzz_9, rug_fuzz_10,
            rug_fuzz_11).is_some()
        );
        let _rug_ed_tests_llm_16_464_rrrruuuugggg_test_from_hms_micro_opt_valid_times = 0;
    }
    #[test]
    fn test_from_hms_micro_opt_invalid_times() {
        let _rug_st_tests_llm_16_464_rrrruuuugggg_test_from_hms_micro_opt_invalid_times = 0;
        let rug_fuzz_0 = 24;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 23;
        let rug_fuzz_5 = 60;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 23;
        let rug_fuzz_9 = 59;
        let rug_fuzz_10 = 60;
        let rug_fuzz_11 = 0;
        let rug_fuzz_12 = 23;
        let rug_fuzz_13 = 59;
        let rug_fuzz_14 = 59;
        let rug_fuzz_15 = 2_000_000;
        debug_assert!(
            NaiveTime::from_hms_micro_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3)
            .is_none()
        );
        debug_assert!(
            NaiveTime::from_hms_micro_opt(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6, rug_fuzz_7)
            .is_none()
        );
        debug_assert!(
            NaiveTime::from_hms_micro_opt(rug_fuzz_8, rug_fuzz_9, rug_fuzz_10,
            rug_fuzz_11).is_none()
        );
        debug_assert!(
            NaiveTime::from_hms_micro_opt(rug_fuzz_12, rug_fuzz_13, rug_fuzz_14,
            rug_fuzz_15).is_none()
        );
        let _rug_ed_tests_llm_16_464_rrrruuuugggg_test_from_hms_micro_opt_invalid_times = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_465_llm_16_465 {
    use crate::naive::time::NaiveTime;
    use crate::Timelike;
    #[test]
    fn test_from_hms_milli_valid() {
        let _rug_st_tests_llm_16_465_llm_16_465_rrrruuuugggg_test_from_hms_milli_valid = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 23;
        let rug_fuzz_5 = 59;
        let rug_fuzz_6 = 59;
        let rug_fuzz_7 = 999;
        let rug_fuzz_8 = 23;
        let rug_fuzz_9 = 59;
        let rug_fuzz_10 = 59;
        let rug_fuzz_11 = 1000;
        debug_assert_eq!(
            NaiveTime::from_hms_milli(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3),
            NaiveTime::from_hms_nano_opt(0, 0, 0, 0).unwrap()
        );
        debug_assert_eq!(
            NaiveTime::from_hms_milli(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6, rug_fuzz_7),
            NaiveTime::from_hms_nano_opt(23, 59, 59, 999_000_000).unwrap()
        );
        debug_assert_eq!(
            NaiveTime::from_hms_milli(rug_fuzz_8, rug_fuzz_9, rug_fuzz_10, rug_fuzz_11)
            .nanosecond(), 1000_000_000
        );
        let _rug_ed_tests_llm_16_465_llm_16_465_rrrruuuugggg_test_from_hms_milli_valid = 0;
    }
    #[test]
    #[should_panic(expected = "invalid time")]
    fn test_from_hms_milli_invalid_hour() {
        let _rug_st_tests_llm_16_465_llm_16_465_rrrruuuugggg_test_from_hms_milli_invalid_hour = 0;
        let rug_fuzz_0 = 24;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        NaiveTime::from_hms_milli(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        let _rug_ed_tests_llm_16_465_llm_16_465_rrrruuuugggg_test_from_hms_milli_invalid_hour = 0;
    }
    #[test]
    #[should_panic(expected = "invalid time")]
    fn test_from_hms_milli_invalid_minute() {
        let _rug_st_tests_llm_16_465_llm_16_465_rrrruuuugggg_test_from_hms_milli_invalid_minute = 0;
        let rug_fuzz_0 = 23;
        let rug_fuzz_1 = 60;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        NaiveTime::from_hms_milli(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        let _rug_ed_tests_llm_16_465_llm_16_465_rrrruuuugggg_test_from_hms_milli_invalid_minute = 0;
    }
    #[test]
    #[should_panic(expected = "invalid time")]
    fn test_from_hms_milli_invalid_second() {
        let _rug_st_tests_llm_16_465_llm_16_465_rrrruuuugggg_test_from_hms_milli_invalid_second = 0;
        let rug_fuzz_0 = 23;
        let rug_fuzz_1 = 59;
        let rug_fuzz_2 = 60;
        let rug_fuzz_3 = 0;
        NaiveTime::from_hms_milli(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        let _rug_ed_tests_llm_16_465_llm_16_465_rrrruuuugggg_test_from_hms_milli_invalid_second = 0;
    }
    #[test]
    #[should_panic(expected = "invalid time")]
    fn test_from_hms_milli_invalid_milli() {
        let _rug_st_tests_llm_16_465_llm_16_465_rrrruuuugggg_test_from_hms_milli_invalid_milli = 0;
        let rug_fuzz_0 = 23;
        let rug_fuzz_1 = 59;
        let rug_fuzz_2 = 59;
        let rug_fuzz_3 = 2000;
        NaiveTime::from_hms_milli(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        let _rug_ed_tests_llm_16_465_llm_16_465_rrrruuuugggg_test_from_hms_milli_invalid_milli = 0;
    }
    #[test]
    fn test_from_hms_milli_edge_cases() {
        let _rug_st_tests_llm_16_465_llm_16_465_rrrruuuugggg_test_from_hms_milli_edge_cases = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 999;
        let rug_fuzz_4 = 23;
        let rug_fuzz_5 = 59;
        let rug_fuzz_6 = 59;
        let rug_fuzz_7 = 1000;
        debug_assert_eq!(
            NaiveTime::from_hms_milli(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3),
            NaiveTime::from_hms_nano_opt(0, 0, 0, 999_000_000).unwrap()
        );
        debug_assert_eq!(
            NaiveTime::from_hms_milli(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6, rug_fuzz_7)
            .nanosecond(), 1000_000_000
        );
        let _rug_ed_tests_llm_16_465_llm_16_465_rrrruuuugggg_test_from_hms_milli_edge_cases = 0;
    }
    #[test]
    fn test_from_hms_milli_leap_second() {
        let _rug_st_tests_llm_16_465_llm_16_465_rrrruuuugggg_test_from_hms_milli_leap_second = 0;
        let rug_fuzz_0 = 23;
        let rug_fuzz_1 = 59;
        let rug_fuzz_2 = 59;
        let rug_fuzz_3 = 1500;
        let leap_second = NaiveTime::from_hms_milli(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
        );
        debug_assert_eq!(leap_second.hour(), 23);
        debug_assert_eq!(leap_second.minute(), 59);
        debug_assert_eq!(leap_second.second(), 59);
        debug_assert_eq!(leap_second.nanosecond(), 1500_000_000);
        let _rug_ed_tests_llm_16_465_llm_16_465_rrrruuuugggg_test_from_hms_milli_leap_second = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_466 {
    use crate::NaiveTime;
    #[test]
    fn test_from_hms_milli_opt() {
        let _rug_st_tests_llm_16_466_rrrruuuugggg_test_from_hms_milli_opt = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 23;
        let rug_fuzz_5 = 59;
        let rug_fuzz_6 = 59;
        let rug_fuzz_7 = 999;
        let rug_fuzz_8 = 23;
        let rug_fuzz_9 = 59;
        let rug_fuzz_10 = 59;
        let rug_fuzz_11 = 1_999;
        let rug_fuzz_12 = 24;
        let rug_fuzz_13 = 0;
        let rug_fuzz_14 = 0;
        let rug_fuzz_15 = 0;
        let rug_fuzz_16 = 23;
        let rug_fuzz_17 = 60;
        let rug_fuzz_18 = 0;
        let rug_fuzz_19 = 0;
        let rug_fuzz_20 = 23;
        let rug_fuzz_21 = 59;
        let rug_fuzz_22 = 60;
        let rug_fuzz_23 = 0;
        let rug_fuzz_24 = 23;
        let rug_fuzz_25 = 59;
        let rug_fuzz_26 = 59;
        let rug_fuzz_27 = 2_000;
        let rug_fuzz_28 = 12;
        let rug_fuzz_29 = 30;
        let rug_fuzz_30 = 30;
        let rug_fuzz_31 = 500;
        let rug_fuzz_32 = 0;
        let rug_fuzz_33 = 0;
        let rug_fuzz_34 = 59;
        let rug_fuzz_35 = 999;
        let rug_fuzz_36 = 25;
        let rug_fuzz_37 = 0;
        let rug_fuzz_38 = 0;
        let rug_fuzz_39 = 0;
        let rug_fuzz_40 = 0;
        let rug_fuzz_41 = 0;
        let rug_fuzz_42 = 0;
        let rug_fuzz_43 = 23;
        let rug_fuzz_44 = 61;
        let rug_fuzz_45 = 0;
        let rug_fuzz_46 = 0;
        let rug_fuzz_47 = 23;
        let rug_fuzz_48 = 0;
        let rug_fuzz_49 = 0;
        let rug_fuzz_50 = 23;
        let rug_fuzz_51 = 59;
        let rug_fuzz_52 = 61;
        let rug_fuzz_53 = 0;
        let rug_fuzz_54 = 23;
        let rug_fuzz_55 = 59;
        let rug_fuzz_56 = 0;
        let rug_fuzz_57 = 23;
        let rug_fuzz_58 = 59;
        let rug_fuzz_59 = 59;
        let rug_fuzz_60 = 1_000;
        let rug_fuzz_61 = 23;
        let rug_fuzz_62 = 59;
        let rug_fuzz_63 = 59;
        debug_assert!(
            NaiveTime::from_hms_milli_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3)
            .is_some()
        );
        debug_assert!(
            NaiveTime::from_hms_milli_opt(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6, rug_fuzz_7)
            .is_some()
        );
        debug_assert!(
            NaiveTime::from_hms_milli_opt(rug_fuzz_8, rug_fuzz_9, rug_fuzz_10,
            rug_fuzz_11).is_some()
        );
        debug_assert!(
            NaiveTime::from_hms_milli_opt(rug_fuzz_12, rug_fuzz_13, rug_fuzz_14,
            rug_fuzz_15).is_none()
        );
        debug_assert!(
            NaiveTime::from_hms_milli_opt(rug_fuzz_16, rug_fuzz_17, rug_fuzz_18,
            rug_fuzz_19).is_none()
        );
        debug_assert!(
            NaiveTime::from_hms_milli_opt(rug_fuzz_20, rug_fuzz_21, rug_fuzz_22,
            rug_fuzz_23).is_none()
        );
        debug_assert!(
            NaiveTime::from_hms_milli_opt(rug_fuzz_24, rug_fuzz_25, rug_fuzz_26,
            rug_fuzz_27).is_none()
        );
        debug_assert!(
            NaiveTime::from_hms_milli_opt(rug_fuzz_28, rug_fuzz_29, rug_fuzz_30,
            rug_fuzz_31).is_some()
        );
        debug_assert!(
            NaiveTime::from_hms_milli_opt(rug_fuzz_32, rug_fuzz_33, rug_fuzz_34,
            rug_fuzz_35).is_some()
        );
        debug_assert!(
            NaiveTime::from_hms_milli_opt(rug_fuzz_36, rug_fuzz_37, rug_fuzz_38,
            rug_fuzz_39).is_none()
        );
        debug_assert!(
            NaiveTime::from_hms_milli_opt(u32::MAX, rug_fuzz_40, rug_fuzz_41,
            rug_fuzz_42).is_none()
        );
        debug_assert!(
            NaiveTime::from_hms_milli_opt(rug_fuzz_43, rug_fuzz_44, rug_fuzz_45,
            rug_fuzz_46).is_none()
        );
        debug_assert!(
            NaiveTime::from_hms_milli_opt(rug_fuzz_47, u32::MAX, rug_fuzz_48,
            rug_fuzz_49).is_none()
        );
        debug_assert!(
            NaiveTime::from_hms_milli_opt(rug_fuzz_50, rug_fuzz_51, rug_fuzz_52,
            rug_fuzz_53).is_none()
        );
        debug_assert!(
            NaiveTime::from_hms_milli_opt(rug_fuzz_54, rug_fuzz_55, u32::MAX,
            rug_fuzz_56).is_none()
        );
        debug_assert!(
            NaiveTime::from_hms_milli_opt(rug_fuzz_57, rug_fuzz_58, rug_fuzz_59,
            rug_fuzz_60).is_some()
        );
        debug_assert!(
            NaiveTime::from_hms_milli_opt(rug_fuzz_61, rug_fuzz_62, rug_fuzz_63,
            u32::MAX).is_none()
        );
        let _rug_ed_tests_llm_16_466_rrrruuuugggg_test_from_hms_milli_opt = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_467 {
    use super::*;
    use crate::*;
    use crate::NaiveTime;
    #[test]
    fn test_from_hms_nano_valid() {
        let _rug_st_tests_llm_16_467_rrrruuuugggg_test_from_hms_nano_valid = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 23;
        let rug_fuzz_5 = 59;
        let rug_fuzz_6 = 59;
        let rug_fuzz_7 = 999_999_999;
        let rug_fuzz_8 = 23;
        let rug_fuzz_9 = 59;
        let rug_fuzz_10 = 59;
        let rug_fuzz_11 = 1_500_000_000;
        let rug_fuzz_12 = 23;
        let rug_fuzz_13 = 59;
        let rug_fuzz_14 = 59;
        let rug_fuzz_15 = 1_999_999_999;
        debug_assert_eq!(
            NaiveTime::from_hms_nano(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3),
            NaiveTime::from_hms_nano_opt(0, 0, 0, 0).unwrap()
        );
        debug_assert_eq!(
            NaiveTime::from_hms_nano(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6, rug_fuzz_7),
            NaiveTime::from_hms_nano_opt(23, 59, 59, 999_999_999).unwrap()
        );
        debug_assert_eq!(
            NaiveTime::from_hms_nano(rug_fuzz_8, rug_fuzz_9, rug_fuzz_10, rug_fuzz_11),
            NaiveTime::from_hms_nano_opt(23, 59, 59, 1_500_000_000).unwrap()
        );
        debug_assert_eq!(
            NaiveTime::from_hms_nano(rug_fuzz_12, rug_fuzz_13, rug_fuzz_14, rug_fuzz_15),
            NaiveTime::from_hms_nano_opt(23, 59, 59, 1_999_999_999).unwrap()
        );
        let _rug_ed_tests_llm_16_467_rrrruuuugggg_test_from_hms_nano_valid = 0;
    }
    #[test]
    #[should_panic(expected = "invalid time")]
    fn test_from_hms_nano_panic_hour() {
        let _rug_st_tests_llm_16_467_rrrruuuugggg_test_from_hms_nano_panic_hour = 0;
        let rug_fuzz_0 = 24;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        NaiveTime::from_hms_nano(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        let _rug_ed_tests_llm_16_467_rrrruuuugggg_test_from_hms_nano_panic_hour = 0;
    }
    #[test]
    #[should_panic(expected = "invalid time")]
    fn test_from_hms_nano_panic_minute() {
        let _rug_st_tests_llm_16_467_rrrruuuugggg_test_from_hms_nano_panic_minute = 0;
        let rug_fuzz_0 = 23;
        let rug_fuzz_1 = 60;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        NaiveTime::from_hms_nano(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        let _rug_ed_tests_llm_16_467_rrrruuuugggg_test_from_hms_nano_panic_minute = 0;
    }
    #[test]
    #[should_panic(expected = "invalid time")]
    fn test_from_hms_nano_panic_second() {
        let _rug_st_tests_llm_16_467_rrrruuuugggg_test_from_hms_nano_panic_second = 0;
        let rug_fuzz_0 = 23;
        let rug_fuzz_1 = 59;
        let rug_fuzz_2 = 60;
        let rug_fuzz_3 = 0;
        NaiveTime::from_hms_nano(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        let _rug_ed_tests_llm_16_467_rrrruuuugggg_test_from_hms_nano_panic_second = 0;
    }
    #[test]
    #[should_panic(expected = "invalid time")]
    fn test_from_hms_nano_panic_nano() {
        let _rug_st_tests_llm_16_467_rrrruuuugggg_test_from_hms_nano_panic_nano = 0;
        let rug_fuzz_0 = 23;
        let rug_fuzz_1 = 59;
        let rug_fuzz_2 = 59;
        let rug_fuzz_3 = 2_000_000_000;
        NaiveTime::from_hms_nano(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        let _rug_ed_tests_llm_16_467_rrrruuuugggg_test_from_hms_nano_panic_nano = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_468 {
    use crate::NaiveTime;
    #[test]
    fn test_from_hms_nano_opt() {
        let _rug_st_tests_llm_16_468_rrrruuuugggg_test_from_hms_nano_opt = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 23;
        let rug_fuzz_5 = 59;
        let rug_fuzz_6 = 59;
        let rug_fuzz_7 = 999_999_999;
        let rug_fuzz_8 = 23;
        let rug_fuzz_9 = 59;
        let rug_fuzz_10 = 59;
        let rug_fuzz_11 = 1_999_999_999;
        let rug_fuzz_12 = 24;
        let rug_fuzz_13 = 0;
        let rug_fuzz_14 = 0;
        let rug_fuzz_15 = 0;
        let rug_fuzz_16 = 23;
        let rug_fuzz_17 = 60;
        let rug_fuzz_18 = 0;
        let rug_fuzz_19 = 0;
        let rug_fuzz_20 = 23;
        let rug_fuzz_21 = 59;
        let rug_fuzz_22 = 60;
        let rug_fuzz_23 = 0;
        let rug_fuzz_24 = 23;
        let rug_fuzz_25 = 59;
        let rug_fuzz_26 = 59;
        let rug_fuzz_27 = 2_000_000_000;
        let rug_fuzz_28 = 23;
        let rug_fuzz_29 = 59;
        let rug_fuzz_30 = 59;
        let rug_fuzz_31 = 0;
        let rug_fuzz_32 = 0;
        let rug_fuzz_33 = 0;
        let rug_fuzz_34 = 0;
        let rug_fuzz_35 = 1_999_999_999;
        let rug_fuzz_36 = 24;
        let rug_fuzz_37 = 0;
        let rug_fuzz_38 = 0;
        let rug_fuzz_39 = 1;
        let rug_fuzz_40 = 0;
        let rug_fuzz_41 = 60;
        let rug_fuzz_42 = 0;
        let rug_fuzz_43 = 0;
        let rug_fuzz_44 = 0;
        let rug_fuzz_45 = 0;
        let rug_fuzz_46 = 60;
        let rug_fuzz_47 = 0;
        let rug_fuzz_48 = 0;
        let rug_fuzz_49 = 0;
        let rug_fuzz_50 = 0;
        let rug_fuzz_51 = 2_000_000_000;
        debug_assert!(
            NaiveTime::from_hms_nano_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3)
            .is_some()
        );
        debug_assert!(
            NaiveTime::from_hms_nano_opt(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6, rug_fuzz_7)
            .is_some()
        );
        debug_assert!(
            NaiveTime::from_hms_nano_opt(rug_fuzz_8, rug_fuzz_9, rug_fuzz_10,
            rug_fuzz_11).is_some()
        );
        debug_assert!(
            NaiveTime::from_hms_nano_opt(rug_fuzz_12, rug_fuzz_13, rug_fuzz_14,
            rug_fuzz_15).is_none()
        );
        debug_assert!(
            NaiveTime::from_hms_nano_opt(rug_fuzz_16, rug_fuzz_17, rug_fuzz_18,
            rug_fuzz_19).is_none()
        );
        debug_assert!(
            NaiveTime::from_hms_nano_opt(rug_fuzz_20, rug_fuzz_21, rug_fuzz_22,
            rug_fuzz_23).is_none()
        );
        debug_assert!(
            NaiveTime::from_hms_nano_opt(rug_fuzz_24, rug_fuzz_25, rug_fuzz_26,
            rug_fuzz_27).is_none()
        );
        debug_assert!(
            NaiveTime::from_hms_nano_opt(rug_fuzz_28, rug_fuzz_29, rug_fuzz_30,
            rug_fuzz_31).is_some()
        );
        debug_assert!(
            NaiveTime::from_hms_nano_opt(rug_fuzz_32, rug_fuzz_33, rug_fuzz_34,
            rug_fuzz_35).is_some()
        );
        debug_assert!(
            NaiveTime::from_hms_nano_opt(rug_fuzz_36, rug_fuzz_37, rug_fuzz_38,
            rug_fuzz_39).is_none()
        );
        debug_assert!(
            NaiveTime::from_hms_nano_opt(rug_fuzz_40, rug_fuzz_41, rug_fuzz_42,
            rug_fuzz_43).is_none()
        );
        debug_assert!(
            NaiveTime::from_hms_nano_opt(rug_fuzz_44, rug_fuzz_45, rug_fuzz_46,
            rug_fuzz_47).is_none()
        );
        debug_assert!(
            NaiveTime::from_hms_nano_opt(rug_fuzz_48, rug_fuzz_49, rug_fuzz_50,
            rug_fuzz_51).is_none()
        );
        let _rug_ed_tests_llm_16_468_rrrruuuugggg_test_from_hms_nano_opt = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_469 {
    use crate::NaiveTime;
    #[test]
    fn test_from_hms_opt_valid_times() {
        let _rug_st_tests_llm_16_469_rrrruuuugggg_test_from_hms_opt_valid_times = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 23;
        let rug_fuzz_4 = 59;
        let rug_fuzz_5 = 59;
        let rug_fuzz_6 = 12;
        let rug_fuzz_7 = 30;
        let rug_fuzz_8 = 45;
        let rug_fuzz_9 = 1;
        let rug_fuzz_10 = 2;
        let rug_fuzz_11 = 3;
        debug_assert!(
            NaiveTime::from_hms_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).is_some()
        );
        debug_assert!(
            NaiveTime::from_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5).is_some()
        );
        debug_assert!(
            NaiveTime::from_hms_opt(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8).is_some()
        );
        debug_assert!(
            NaiveTime::from_hms_opt(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11).is_some()
        );
        let _rug_ed_tests_llm_16_469_rrrruuuugggg_test_from_hms_opt_valid_times = 0;
    }
    #[test]
    fn test_from_hms_opt_invalid_hours() {
        let _rug_st_tests_llm_16_469_rrrruuuugggg_test_from_hms_opt_invalid_hours = 0;
        let rug_fuzz_0 = 24;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 25;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 0;
        debug_assert!(
            NaiveTime::from_hms_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).is_none()
        );
        debug_assert!(
            NaiveTime::from_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5).is_none()
        );
        debug_assert!(
            NaiveTime::from_hms_opt(u32::MAX, rug_fuzz_6, rug_fuzz_7).is_none()
        );
        let _rug_ed_tests_llm_16_469_rrrruuuugggg_test_from_hms_opt_invalid_hours = 0;
    }
    #[test]
    fn test_from_hms_opt_invalid_minutes() {
        let _rug_st_tests_llm_16_469_rrrruuuugggg_test_from_hms_opt_invalid_minutes = 0;
        let rug_fuzz_0 = 23;
        let rug_fuzz_1 = 60;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 61;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 0;
        debug_assert!(
            NaiveTime::from_hms_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).is_none()
        );
        debug_assert!(
            NaiveTime::from_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5).is_none()
        );
        debug_assert!(
            NaiveTime::from_hms_opt(rug_fuzz_6, u32::MAX, rug_fuzz_7).is_none()
        );
        let _rug_ed_tests_llm_16_469_rrrruuuugggg_test_from_hms_opt_invalid_minutes = 0;
    }
    #[test]
    fn test_from_hms_opt_invalid_seconds() {
        let _rug_st_tests_llm_16_469_rrrruuuugggg_test_from_hms_opt_invalid_seconds = 0;
        let rug_fuzz_0 = 23;
        let rug_fuzz_1 = 59;
        let rug_fuzz_2 = 60;
        let rug_fuzz_3 = 12;
        let rug_fuzz_4 = 30;
        let rug_fuzz_5 = 61;
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = 2;
        debug_assert!(
            NaiveTime::from_hms_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).is_none()
        );
        debug_assert!(
            NaiveTime::from_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5).is_none()
        );
        debug_assert!(
            NaiveTime::from_hms_opt(rug_fuzz_6, rug_fuzz_7, u32::MAX).is_none()
        );
        let _rug_ed_tests_llm_16_469_rrrruuuugggg_test_from_hms_opt_invalid_seconds = 0;
    }
    #[test]
    fn test_from_hms_opt_boundary_values() {
        let _rug_st_tests_llm_16_469_rrrruuuugggg_test_from_hms_opt_boundary_values = 0;
        let rug_fuzz_0 = 23;
        let rug_fuzz_1 = 59;
        let rug_fuzz_2 = 59;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        debug_assert!(
            NaiveTime::from_hms_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).is_some()
        );
        debug_assert!(
            NaiveTime::from_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5).is_some()
        );
        let _rug_ed_tests_llm_16_469_rrrruuuugggg_test_from_hms_opt_boundary_values = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_470 {
    use crate::naive::time::NaiveTime;
    use crate::Timelike;
    #[test]
    fn test_from_num_seconds_from_midnight() {
        let _rug_st_tests_llm_16_470_rrrruuuugggg_test_from_num_seconds_from_midnight = 0;
        let rug_fuzz_0 = 3661;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 86399;
        let rug_fuzz_3 = 1_500_000_000;
        let rug_fuzz_4 = 86400;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 86399;
        let rug_fuzz_7 = 2_000_000_000;
        let time = NaiveTime::from_num_seconds_from_midnight(rug_fuzz_0, rug_fuzz_1);
        debug_assert_eq!(time.hour(), 1);
        debug_assert_eq!(time.minute(), 1);
        debug_assert_eq!(time.second(), 1);
        debug_assert_eq!(time.nanosecond(), 0);
        let leap_time = NaiveTime::from_num_seconds_from_midnight(
            rug_fuzz_2,
            rug_fuzz_3,
        );
        debug_assert_eq!(leap_time.hour(), 23);
        debug_assert_eq!(leap_time.minute(), 59);
        debug_assert_eq!(leap_time.second(), 59);
        debug_assert_eq!(leap_time.nanosecond(), 1_500_000_000);
        #[should_panic(expected = "invalid time")]
        NaiveTime::from_num_seconds_from_midnight(rug_fuzz_4, rug_fuzz_5);
        #[should_panic(expected = "invalid time")]
        NaiveTime::from_num_seconds_from_midnight(rug_fuzz_6, rug_fuzz_7);
        let _rug_ed_tests_llm_16_470_rrrruuuugggg_test_from_num_seconds_from_midnight = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_471 {
    use super::*;
    use crate::*;
    #[test]
    fn test_from_num_seconds_from_midnight_opt_valid_times() {
        let _rug_st_tests_llm_16_471_rrrruuuugggg_test_from_num_seconds_from_midnight_opt_valid_times = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 86399;
        let rug_fuzz_3 = 999_999_999;
        let rug_fuzz_4 = 86399;
        let rug_fuzz_5 = 1_999_999_999;
        debug_assert!(
            NaiveTime::from_num_seconds_from_midnight_opt(rug_fuzz_0, rug_fuzz_1)
            .is_some()
        );
        debug_assert!(
            NaiveTime::from_num_seconds_from_midnight_opt(rug_fuzz_2, rug_fuzz_3)
            .is_some()
        );
        debug_assert!(
            NaiveTime::from_num_seconds_from_midnight_opt(rug_fuzz_4, rug_fuzz_5)
            .is_some()
        );
        let _rug_ed_tests_llm_16_471_rrrruuuugggg_test_from_num_seconds_from_midnight_opt_valid_times = 0;
    }
    #[test]
    fn test_from_num_seconds_from_midnight_opt_edge_cases() {
        let _rug_st_tests_llm_16_471_rrrruuuugggg_test_from_num_seconds_from_midnight_opt_edge_cases = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 86399;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 1_999_999_999;
        debug_assert!(
            NaiveTime::from_num_seconds_from_midnight_opt(rug_fuzz_0, rug_fuzz_1)
            .is_some()
        );
        debug_assert!(
            NaiveTime::from_num_seconds_from_midnight_opt(rug_fuzz_2, rug_fuzz_3)
            .is_some()
        );
        debug_assert!(
            NaiveTime::from_num_seconds_from_midnight_opt(rug_fuzz_4, rug_fuzz_5)
            .is_some()
        );
        let _rug_ed_tests_llm_16_471_rrrruuuugggg_test_from_num_seconds_from_midnight_opt_edge_cases = 0;
    }
    #[test]
    fn test_from_num_seconds_from_midnight_opt_invalid_times() {
        let _rug_st_tests_llm_16_471_rrrruuuugggg_test_from_num_seconds_from_midnight_opt_invalid_times = 0;
        let rug_fuzz_0 = 86_400;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 86399;
        let rug_fuzz_3 = 2_000_000_000;
        let rug_fuzz_4 = 86_400;
        let rug_fuzz_5 = 2_000_000_000;
        debug_assert!(
            NaiveTime::from_num_seconds_from_midnight_opt(rug_fuzz_0, rug_fuzz_1)
            .is_none()
        );
        debug_assert!(
            NaiveTime::from_num_seconds_from_midnight_opt(rug_fuzz_2, rug_fuzz_3)
            .is_none()
        );
        debug_assert!(
            NaiveTime::from_num_seconds_from_midnight_opt(rug_fuzz_4, rug_fuzz_5)
            .is_none()
        );
        let _rug_ed_tests_llm_16_471_rrrruuuugggg_test_from_num_seconds_from_midnight_opt_invalid_times = 0;
    }
    #[test]
    fn test_from_num_seconds_from_midnight_opt_boundary_conditions() {
        let _rug_st_tests_llm_16_471_rrrruuuugggg_test_from_num_seconds_from_midnight_opt_boundary_conditions = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1_999_999_999;
        let rug_fuzz_2 = 86399;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 86_400;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 2_000_000_000;
        debug_assert!(
            NaiveTime::from_num_seconds_from_midnight_opt(rug_fuzz_0, rug_fuzz_1)
            .is_some()
        );
        debug_assert!(
            NaiveTime::from_num_seconds_from_midnight_opt(rug_fuzz_2, rug_fuzz_3)
            .is_some()
        );
        debug_assert!(
            NaiveTime::from_num_seconds_from_midnight_opt(rug_fuzz_4, rug_fuzz_5)
            .is_none()
        );
        debug_assert!(
            NaiveTime::from_num_seconds_from_midnight_opt(rug_fuzz_6, rug_fuzz_7)
            .is_none()
        );
        let _rug_ed_tests_llm_16_471_rrrruuuugggg_test_from_num_seconds_from_midnight_opt_boundary_conditions = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_472 {
    use crate::NaiveTime;
    #[test]
    fn test_hms() {
        let _rug_st_tests_llm_16_472_rrrruuuugggg_test_hms = 0;
        let rug_fuzz_0 = 3661;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 86399;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 3600;
        let rug_fuzz_7 = 0;
        let time = NaiveTime {
            secs: rug_fuzz_0,
            frac: rug_fuzz_1,
        };
        debug_assert_eq!(time.hms(), (1, 1, 1));
        let time = NaiveTime {
            secs: rug_fuzz_2,
            frac: rug_fuzz_3,
        };
        debug_assert_eq!(time.hms(), (0, 0, 0));
        let time = NaiveTime {
            secs: rug_fuzz_4,
            frac: rug_fuzz_5,
        };
        debug_assert_eq!(time.hms(), (23, 59, 59));
        let time = NaiveTime {
            secs: rug_fuzz_6,
            frac: rug_fuzz_7,
        };
        debug_assert_eq!(time.hms(), (1, 0, 0));
        let _rug_ed_tests_llm_16_472_rrrruuuugggg_test_hms = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_473 {
    use crate::NaiveTime;
    use crate::time_delta::TimeDelta;
    #[test]
    fn test_overflowing_add_signed() {
        let _rug_st_tests_llm_16_473_rrrruuuugggg_test_overflowing_add_signed = 0;
        let rug_fuzz_0 = 3;
        let rug_fuzz_1 = 4;
        let rug_fuzz_2 = 5;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 11;
        let rug_fuzz_5 = 23;
        let rug_fuzz_6 = 59;
        let rug_fuzz_7 = 59;
        let rug_fuzz_8 = 0;
        let rug_fuzz_9 = 2;
        let rug_fuzz_10 = 0;
        let rug_fuzz_11 = 0;
        let rug_fuzz_12 = 1;
        let rug_fuzz_13 = 0;
        let rug_fuzz_14 = 2;
        let rug_fuzz_15 = 23;
        let rug_fuzz_16 = 59;
        let rug_fuzz_17 = 59;
        let rug_fuzz_18 = 1_000_000_000;
        let rug_fuzz_19 = 1;
        let rug_fuzz_20 = 23;
        let rug_fuzz_21 = 59;
        let rug_fuzz_22 = 59;
        let rug_fuzz_23 = 0;
        let rug_fuzz_24 = 1_000_000_000;
        let from_hms_nano = NaiveTime::from_hms_nano;
        debug_assert_eq!(
            from_hms_nano(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3)
            .overflowing_add_signed(TimeDelta::seconds(rug_fuzz_4)), (from_hms_nano(3, 4,
            16, 0), 0)
        );
        debug_assert_eq!(
            from_hms_nano(rug_fuzz_5, rug_fuzz_6, rug_fuzz_7, rug_fuzz_8)
            .overflowing_add_signed(TimeDelta::seconds(rug_fuzz_9)), (from_hms_nano(0, 0,
            1, 0), 86_400)
        );
        debug_assert_eq!(
            from_hms_nano(rug_fuzz_10, rug_fuzz_11, rug_fuzz_12, rug_fuzz_13)
            .overflowing_add_signed(TimeDelta::seconds(- rug_fuzz_14)),
            (from_hms_nano(23, 59, 59, 0), - 86_400)
        );
        debug_assert_eq!(
            from_hms_nano(rug_fuzz_15, rug_fuzz_16, rug_fuzz_17, rug_fuzz_18)
            .overflowing_add_signed(TimeDelta::seconds(rug_fuzz_19)), (from_hms_nano(0,
            0, 0, 0), 86_400)
        );
        debug_assert_eq!(
            from_hms_nano(rug_fuzz_20, rug_fuzz_21, rug_fuzz_22, rug_fuzz_23)
            .overflowing_add_signed(TimeDelta::nanoseconds(rug_fuzz_24)),
            (from_hms_nano(23, 59, 59, 1_000_000_000), 0)
        );
        let _rug_ed_tests_llm_16_473_rrrruuuugggg_test_overflowing_add_signed = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_475 {
    use crate::naive::time::NaiveTime;
    use crate::ParseResult;
    #[test]
    fn test_parse_from_str_valid_times() {
        let _rug_st_tests_llm_16_475_rrrruuuugggg_test_parse_from_str_valid_times = 0;
        let rug_fuzz_0 = "23:56:04";
        let rug_fuzz_1 = "%H:%M:%S";
        let rug_fuzz_2 = "pm012345.6789";
        let rug_fuzz_3 = "%p%I%M%S%.f";
        let rug_fuzz_4 = "2014-5-17T12:34:56+09:30";
        let rug_fuzz_5 = "%Y-%m-%dT%H:%M:%S%z";
        let rug_fuzz_6 = "08:59:60.123";
        let rug_fuzz_7 = "%H:%M:%S%.f";
        let rug_fuzz_8 = "7:15";
        let rug_fuzz_9 = "%H:%M";
        debug_assert_eq!(
            NaiveTime::parse_from_str(rug_fuzz_0, rug_fuzz_1), Ok(NaiveTime::from_hms(23,
            56, 4))
        );
        debug_assert_eq!(
            NaiveTime::parse_from_str(rug_fuzz_2, rug_fuzz_3),
            Ok(NaiveTime::from_hms_micro(13, 23, 45, 678900))
        );
        debug_assert_eq!(
            NaiveTime::parse_from_str(rug_fuzz_4, rug_fuzz_5), Ok(NaiveTime::from_hms(12,
            34, 56))
        );
        debug_assert_eq!(
            NaiveTime::parse_from_str(rug_fuzz_6, rug_fuzz_7),
            Ok(NaiveTime::from_hms_milli(8, 59, 59, 1123))
        );
        debug_assert_eq!(
            NaiveTime::parse_from_str(rug_fuzz_8, rug_fuzz_9), Ok(NaiveTime::from_hms(7,
            15, 0))
        );
        let _rug_ed_tests_llm_16_475_rrrruuuugggg_test_parse_from_str_valid_times = 0;
    }
    #[test]
    fn test_parse_from_str_invalid_times() {
        let _rug_st_tests_llm_16_475_rrrruuuugggg_test_parse_from_str_invalid_times = 0;
        let rug_fuzz_0 = "04m33s";
        let rug_fuzz_1 = "%Mm%Ss";
        let rug_fuzz_2 = "12";
        let rug_fuzz_3 = "%H";
        let rug_fuzz_4 = "17:60";
        let rug_fuzz_5 = "%H:%M";
        let rug_fuzz_6 = "24:00:00";
        let rug_fuzz_7 = "%H:%M:%S";
        let rug_fuzz_8 = "13:07 AM";
        let rug_fuzz_9 = "%H:%M %p";
        debug_assert!(NaiveTime::parse_from_str(rug_fuzz_0, rug_fuzz_1).is_err());
        debug_assert!(NaiveTime::parse_from_str(rug_fuzz_2, rug_fuzz_3).is_err());
        debug_assert!(NaiveTime::parse_from_str(rug_fuzz_4, rug_fuzz_5).is_err());
        debug_assert!(NaiveTime::parse_from_str(rug_fuzz_6, rug_fuzz_7).is_err());
        debug_assert!(NaiveTime::parse_from_str(rug_fuzz_8, rug_fuzz_9).is_err());
        let _rug_ed_tests_llm_16_475_rrrruuuugggg_test_parse_from_str_invalid_times = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_476 {
    use crate::naive::time::NaiveTime;
    use crate::time_delta::TimeDelta;
    use crate::Timelike;
    #[test]
    fn test_signed_duration_since() {
        let _rug_st_tests_llm_16_476_rrrruuuugggg_test_signed_duration_since = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 60;
        let rug_fuzz_2 = 3_600;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 1;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 3;
        let rug_fuzz_7 = 5;
        let rug_fuzz_8 = 7;
        let rug_fuzz_9 = 900;
        let rug_fuzz_10 = 3;
        let rug_fuzz_11 = 5;
        let rug_fuzz_12 = 7;
        let rug_fuzz_13 = 900;
        let rug_fuzz_14 = 3;
        let rug_fuzz_15 = 5;
        let rug_fuzz_16 = 7;
        let rug_fuzz_17 = 900;
        let rug_fuzz_18 = 3;
        let rug_fuzz_19 = 5;
        let rug_fuzz_20 = 7;
        let rug_fuzz_21 = 875;
        let rug_fuzz_22 = 3;
        let rug_fuzz_23 = 5;
        let rug_fuzz_24 = 7;
        let rug_fuzz_25 = 900;
        let rug_fuzz_26 = 3;
        let rug_fuzz_27 = 5;
        let rug_fuzz_28 = 6;
        let rug_fuzz_29 = 925;
        let rug_fuzz_30 = 3;
        let rug_fuzz_31 = 5;
        let rug_fuzz_32 = 7;
        let rug_fuzz_33 = 900;
        let rug_fuzz_34 = 3;
        let rug_fuzz_35 = 5;
        let rug_fuzz_36 = 0;
        let rug_fuzz_37 = 900;
        let rug_fuzz_38 = 3;
        let rug_fuzz_39 = 5;
        let rug_fuzz_40 = 7;
        let rug_fuzz_41 = 900;
        let rug_fuzz_42 = 3;
        let rug_fuzz_43 = 0;
        let rug_fuzz_44 = 7;
        let rug_fuzz_45 = 900;
        let rug_fuzz_46 = 3;
        let rug_fuzz_47 = 5;
        let rug_fuzz_48 = 7;
        let rug_fuzz_49 = 900;
        let rug_fuzz_50 = 0;
        let rug_fuzz_51 = 5;
        let rug_fuzz_52 = 7;
        let rug_fuzz_53 = 900;
        let rug_fuzz_54 = 3;
        let rug_fuzz_55 = 5;
        let rug_fuzz_56 = 7;
        let rug_fuzz_57 = 900;
        let rug_fuzz_58 = 4;
        let rug_fuzz_59 = 5;
        let rug_fuzz_60 = 7;
        let rug_fuzz_61 = 900;
        let rug_fuzz_62 = 3;
        let rug_fuzz_63 = 5;
        let rug_fuzz_64 = 7;
        let rug_fuzz_65 = 900;
        let rug_fuzz_66 = 2;
        let rug_fuzz_67 = 4;
        let rug_fuzz_68 = 6;
        let rug_fuzz_69 = 800;
        let rug_fuzz_70 = 3;
        let rug_fuzz_71 = 0;
        let rug_fuzz_72 = 59;
        let rug_fuzz_73 = 1000;
        let rug_fuzz_74 = 3;
        let rug_fuzz_75 = 0;
        let rug_fuzz_76 = 59;
        let rug_fuzz_77 = 0;
        let rug_fuzz_78 = 3;
        let rug_fuzz_79 = 0;
        let rug_fuzz_80 = 59;
        let rug_fuzz_81 = 1500;
        let rug_fuzz_82 = 3;
        let rug_fuzz_83 = 0;
        let rug_fuzz_84 = 59;
        let rug_fuzz_85 = 0;
        let rug_fuzz_86 = 3;
        let rug_fuzz_87 = 0;
        let rug_fuzz_88 = 59;
        let rug_fuzz_89 = 1000;
        let rug_fuzz_90 = 3;
        let rug_fuzz_91 = 0;
        let rug_fuzz_92 = 0;
        let rug_fuzz_93 = 0;
        let rug_fuzz_94 = 3;
        let rug_fuzz_95 = 0;
        let rug_fuzz_96 = 0;
        let rug_fuzz_97 = 0;
        let rug_fuzz_98 = 2;
        let rug_fuzz_99 = 59;
        let rug_fuzz_100 = 59;
        let rug_fuzz_101 = 1000;
        let rug_fuzz_102 = 3;
        let rug_fuzz_103 = 0;
        let rug_fuzz_104 = 59;
        let rug_fuzz_105 = 1000;
        let rug_fuzz_106 = 2;
        let rug_fuzz_107 = 59;
        let rug_fuzz_108 = 59;
        let rug_fuzz_109 = 1000;
        let rug_fuzz_110 = 0;
        let rug_fuzz_111 = 0;
        let rug_fuzz_112 = 0;
        let rug_fuzz_113 = 23;
        let rug_fuzz_114 = 59;
        let rug_fuzz_115 = 59;
        let rug_fuzz_116 = 999_999_999;
        let rug_fuzz_117 = 12;
        let rug_fuzz_118 = 0;
        let rug_fuzz_119 = 0;
        let rug_fuzz_120 = 0;
        let rug_fuzz_121 = 0;
        let rug_fuzz_122 = 0;
        let rug_fuzz_123 = 1;
        let rug_fuzz_124 = 23;
        let rug_fuzz_125 = 59;
        let rug_fuzz_126 = 59;
        let zero_duration = TimeDelta::zero();
        let one_second = TimeDelta::seconds(rug_fuzz_0);
        let one_minute = TimeDelta::seconds(rug_fuzz_1);
        let one_hour = TimeDelta::seconds(rug_fuzz_2);
        let one_milli = TimeDelta::milliseconds(rug_fuzz_3);
        let one_micro = TimeDelta::microseconds(rug_fuzz_4);
        let one_nano = TimeDelta::nanoseconds(rug_fuzz_5);
        debug_assert_eq!(
            NaiveTime::from_hms_milli(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8, rug_fuzz_9)
            .signed_duration_since(NaiveTime::from_hms_milli(rug_fuzz_10, rug_fuzz_11,
            rug_fuzz_12, rug_fuzz_13)), zero_duration
        );
        debug_assert_eq!(
            NaiveTime::from_hms_milli(rug_fuzz_14, rug_fuzz_15, rug_fuzz_16, rug_fuzz_17)
            .signed_duration_since(NaiveTime::from_hms_milli(rug_fuzz_18, rug_fuzz_19,
            rug_fuzz_20, rug_fuzz_21)), one_milli * 25
        );
        debug_assert_eq!(
            NaiveTime::from_hms_milli(rug_fuzz_22, rug_fuzz_23, rug_fuzz_24, rug_fuzz_25)
            .signed_duration_since(NaiveTime::from_hms_milli(rug_fuzz_26, rug_fuzz_27,
            rug_fuzz_28, rug_fuzz_29)), one_second - one_milli * 25
        );
        debug_assert_eq!(
            NaiveTime::from_hms_milli(rug_fuzz_30, rug_fuzz_31, rug_fuzz_32, rug_fuzz_33)
            .signed_duration_since(NaiveTime::from_hms_milli(rug_fuzz_34, rug_fuzz_35,
            rug_fuzz_36, rug_fuzz_37)), one_minute * 7
        );
        debug_assert_eq!(
            NaiveTime::from_hms_milli(rug_fuzz_38, rug_fuzz_39, rug_fuzz_40, rug_fuzz_41)
            .signed_duration_since(NaiveTime::from_hms_milli(rug_fuzz_42, rug_fuzz_43,
            rug_fuzz_44, rug_fuzz_45)), one_hour - one_minute * 5
        );
        debug_assert_eq!(
            NaiveTime::from_hms_milli(rug_fuzz_46, rug_fuzz_47, rug_fuzz_48, rug_fuzz_49)
            .signed_duration_since(NaiveTime::from_hms_milli(rug_fuzz_50, rug_fuzz_51,
            rug_fuzz_52, rug_fuzz_53)), one_hour * 3
        );
        debug_assert_eq!(
            NaiveTime::from_hms_milli(rug_fuzz_54, rug_fuzz_55, rug_fuzz_56, rug_fuzz_57)
            .signed_duration_since(NaiveTime::from_hms_milli(rug_fuzz_58, rug_fuzz_59,
            rug_fuzz_60, rug_fuzz_61)), - one_hour
        );
        debug_assert_eq!(
            NaiveTime::from_hms_milli(rug_fuzz_62, rug_fuzz_63, rug_fuzz_64, rug_fuzz_65)
            .signed_duration_since(NaiveTime::from_hms_milli(rug_fuzz_66, rug_fuzz_67,
            rug_fuzz_68, rug_fuzz_69)), one_hour + one_minute + one_second + one_milli *
            100
        );
        debug_assert_eq!(
            NaiveTime::from_hms_milli(rug_fuzz_70, rug_fuzz_71, rug_fuzz_72, rug_fuzz_73)
            .signed_duration_since(NaiveTime::from_hms_milli(rug_fuzz_74, rug_fuzz_75,
            rug_fuzz_76, rug_fuzz_77)), one_second
        );
        debug_assert_eq!(
            NaiveTime::from_hms_milli(rug_fuzz_78, rug_fuzz_79, rug_fuzz_80, rug_fuzz_81)
            .signed_duration_since(NaiveTime::from_hms_milli(rug_fuzz_82, rug_fuzz_83,
            rug_fuzz_84, rug_fuzz_85)), one_milli * 1500
        );
        debug_assert_eq!(
            NaiveTime::from_hms_milli(rug_fuzz_86, rug_fuzz_87, rug_fuzz_88, rug_fuzz_89)
            .signed_duration_since(NaiveTime::from_hms_milli(rug_fuzz_90, rug_fuzz_91,
            rug_fuzz_92, rug_fuzz_93)), one_minute
        );
        debug_assert_eq!(
            NaiveTime::from_hms_milli(rug_fuzz_94, rug_fuzz_95, rug_fuzz_96, rug_fuzz_97)
            .signed_duration_since(NaiveTime::from_hms_milli(rug_fuzz_98, rug_fuzz_99,
            rug_fuzz_100, rug_fuzz_101)), one_second
        );
        debug_assert_eq!(
            NaiveTime::from_hms_milli(rug_fuzz_102, rug_fuzz_103, rug_fuzz_104,
            rug_fuzz_105).signed_duration_since(NaiveTime::from_hms_milli(rug_fuzz_106,
            rug_fuzz_107, rug_fuzz_108, rug_fuzz_109)), one_minute + one_second
        );
        let midnight = NaiveTime::from_hms(rug_fuzz_110, rug_fuzz_111, rug_fuzz_112);
        let almost_midnight = NaiveTime::from_hms_nano(
            rug_fuzz_113,
            rug_fuzz_114,
            rug_fuzz_115,
            rug_fuzz_116,
        );
        debug_assert_eq!(midnight.signed_duration_since(almost_midnight), one_nano);
        debug_assert_eq!(
            almost_midnight.signed_duration_since(midnight), - (one_second - one_nano)
        );
        let noon = NaiveTime::from_hms(rug_fuzz_117, rug_fuzz_118, rug_fuzz_119);
        debug_assert_eq!(midnight.signed_duration_since(noon), - one_hour * 12);
        debug_assert_eq!(noon.signed_duration_since(midnight), one_hour * 12);
        let just_after_midnight = NaiveTime::from_hms_nano(
            rug_fuzz_120,
            rug_fuzz_121,
            rug_fuzz_122,
            rug_fuzz_123,
        );
        debug_assert_eq!(just_after_midnight.signed_duration_since(midnight), one_nano);
        debug_assert_eq!(
            midnight.signed_duration_since(just_after_midnight), - one_nano
        );
        let just_before_midnight = NaiveTime::from_hms(
            rug_fuzz_124,
            rug_fuzz_125,
            rug_fuzz_126,
        );
        debug_assert_eq!(
            midnight.signed_duration_since(just_before_midnight), one_second
        );
        debug_assert_eq!(
            just_before_midnight.signed_duration_since(midnight), - (one_hour * 24 -
            one_second)
        );
        let _rug_ed_tests_llm_16_476_rrrruuuugggg_test_signed_duration_since = 0;
    }
}
