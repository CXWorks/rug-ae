//! The [`UtcOffset`] struct and its associated `impl`s.

use core::fmt;
use core::ops::Neg;
#[cfg(feature = "formatting")]
use std::io;

use crate::error;
#[cfg(feature = "formatting")]
use crate::formatting::Formattable;
#[cfg(feature = "parsing")]
use crate::parsing::Parsable;
#[cfg(feature = "local-offset")]
use crate::sys::local_offset_at;
#[cfg(feature = "local-offset")]
use crate::OffsetDateTime;

/// An offset from UTC.
///
/// This struct can store values up to ±23:59:59. If you need support outside this range, please
/// file an issue with your use case.
// All three components _must_ have the same sign.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UtcOffset {
    #[allow(clippy::missing_docs_in_private_items)]
    hours: i8,
    #[allow(clippy::missing_docs_in_private_items)]
    minutes: i8,
    #[allow(clippy::missing_docs_in_private_items)]
    seconds: i8,
}

impl UtcOffset {
    /// A `UtcOffset` that is UTC.
    ///
    /// ```rust
    /// # use time::{UtcOffset, macros::offset};
    /// assert_eq!(UtcOffset::UTC, offset!(UTC));
    /// ```
    pub const UTC: Self = Self::__from_hms_unchecked(0, 0, 0);

    // region: constructors
    /// Create a `UtcOffset` representing an offset of the hours, minutes, and seconds provided, the
    /// validity of which must be guaranteed by the caller. All three parameters must have the same
    /// sign.
    #[doc(hidden)]
    pub const fn __from_hms_unchecked(hours: i8, minutes: i8, seconds: i8) -> Self {
        Self {
            hours,
            minutes,
            seconds,
        }
    }

    /// Create a `UtcOffset` representing an offset by the number of hours, minutes, and seconds
    /// provided.
    ///
    /// The sign of all three components should match. If they do not, all smaller components will
    /// have their signs flipped.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::from_hms(1, 2, 3)?.as_hms(), (1, 2, 3));
    /// assert_eq!(UtcOffset::from_hms(1, -2, -3)?.as_hms(), (1, 2, 3));
    /// # Ok::<_, time::Error>(())
    /// ```
    pub const fn from_hms(
        hours: i8,
        mut minutes: i8,
        mut seconds: i8,
    ) -> Result<Self, error::ComponentRange> {
        ensure_value_in_range!(hours in -23 => 23);
        ensure_value_in_range!(minutes in -59 => 59);
        ensure_value_in_range!(seconds in -59 => 59);

        if (hours > 0 && minutes < 0) || (hours < 0 && minutes > 0) {
            minutes *= -1;
        }
        if (hours > 0 && seconds < 0)
            || (hours < 0 && seconds > 0)
            || (minutes > 0 && seconds < 0)
            || (minutes < 0 && seconds > 0)
        {
            seconds *= -1;
        }

        Ok(Self::__from_hms_unchecked(hours, minutes, seconds))
    }

    /// Create a `UtcOffset` representing an offset by the number of seconds provided.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::from_whole_seconds(3_723)?.as_hms(), (1, 2, 3));
    /// # Ok::<_, time::Error>(())
    /// ```
    pub const fn from_whole_seconds(seconds: i32) -> Result<Self, error::ComponentRange> {
        ensure_value_in_range!(seconds in -86_399 => 86_399);

        Ok(Self::__from_hms_unchecked(
            (seconds / 3_600) as _,
            ((seconds / 60) % 60) as _,
            (seconds % 60) as _,
        ))
    }
    // endregion constructors

    // region: getters
    /// Obtain the UTC offset as its hours, minutes, and seconds. The sign of all three components
    /// will always match. A positive value indicates an offset to the east; a negative to the west.
    ///
    /// ```rust
    /// # use time::macros::offset;
    /// assert_eq!(offset!(+1:02:03).as_hms(), (1, 2, 3));
    /// assert_eq!(offset!(-1:02:03).as_hms(), (-1, -2, -3));
    /// ```
    pub const fn as_hms(self) -> (i8, i8, i8) {
        (self.hours, self.minutes, self.seconds)
    }

    /// Obtain the number of whole hours the offset is from UTC. A positive value indicates an
    /// offset to the east; a negative to the west.
    ///
    /// ```rust
    /// # use time::macros::offset;
    /// assert_eq!(offset!(+1:02:03).whole_hours(), 1);
    /// assert_eq!(offset!(-1:02:03).whole_hours(), -1);
    /// ```
    pub const fn whole_hours(self) -> i8 {
        self.hours
    }

    /// Obtain the number of whole minutes the offset is from UTC. A positive value indicates an
    /// offset to the east; a negative to the west.
    ///
    /// ```rust
    /// # use time::macros::offset;
    /// assert_eq!(offset!(+1:02:03).whole_minutes(), 62);
    /// assert_eq!(offset!(-1:02:03).whole_minutes(), -62);
    /// ```
    pub const fn whole_minutes(self) -> i16 {
        self.hours as i16 * 60 + self.minutes as i16
    }

    /// Obtain the number of minutes past the hour the offset is from UTC. A positive value
    /// indicates an offset to the east; a negative to the west.
    ///
    /// ```rust
    /// # use time::macros::offset;
    /// assert_eq!(offset!(+1:02:03).minutes_past_hour(), 2);
    /// assert_eq!(offset!(-1:02:03).minutes_past_hour(), -2);
    /// ```
    pub const fn minutes_past_hour(self) -> i8 {
        self.minutes
    }

    /// Obtain the number of whole seconds the offset is from UTC. A positive value indicates an
    /// offset to the east; a negative to the west.
    ///
    /// ```rust
    /// # use time::macros::offset;
    /// assert_eq!(offset!(+1:02:03).whole_seconds(), 3723);
    /// assert_eq!(offset!(-1:02:03).whole_seconds(), -3723);
    /// ```
    // This may be useful for anyone manually implementing arithmetic, as it
    // would let them construct a `Duration` directly.
    pub const fn whole_seconds(self) -> i32 {
        self.hours as i32 * 3_600 + self.minutes as i32 * 60 + self.seconds as i32
    }

    /// Obtain the number of seconds past the minute the offset is from UTC. A positive value
    /// indicates an offset to the east; a negative to the west.
    ///
    /// ```rust
    /// # use time::macros::offset;
    /// assert_eq!(offset!(+1:02:03).seconds_past_minute(), 3);
    /// assert_eq!(offset!(-1:02:03).seconds_past_minute(), -3);
    /// ```
    pub const fn seconds_past_minute(self) -> i8 {
        self.seconds
    }
    // endregion getters

    // region: is_{sign}
    /// Check if the offset is exactly UTC.
    ///
    ///
    /// ```rust
    /// # use time::macros::offset;
    /// assert!(!offset!(+1:02:03).is_utc());
    /// assert!(!offset!(-1:02:03).is_utc());
    /// assert!(offset!(UTC).is_utc());
    /// ```
    pub const fn is_utc(self) -> bool {
        self.hours == 0 && self.minutes == 0 && self.seconds == 0
    }

    /// Check if the offset is positive, or east of UTC.
    ///
    /// ```rust
    /// # use time::macros::offset;
    /// assert!(offset!(+1:02:03).is_positive());
    /// assert!(!offset!(-1:02:03).is_positive());
    /// assert!(!offset!(UTC).is_positive());
    /// ```
    pub const fn is_positive(self) -> bool {
        self.hours > 0 || self.minutes > 0 || self.seconds > 0
    }

    /// Check if the offset is negative, or west of UTC.
    ///
    /// ```rust
    /// # use time::macros::offset;
    /// assert!(!offset!(+1:02:03).is_negative());
    /// assert!(offset!(-1:02:03).is_negative());
    /// assert!(!offset!(UTC).is_negative());
    /// ```
    pub const fn is_negative(self) -> bool {
        self.hours < 0 || self.minutes < 0 || self.seconds < 0
    }
    // endregion is_{sign}

    // region: local offset
    /// Attempt to obtain the system's UTC offset at a known moment in time. If the offset cannot be
    /// determined, an error is returned.
    ///
    /// ```rust
    /// # use time::{UtcOffset, OffsetDateTime};
    /// let local_offset = UtcOffset::local_offset_at(OffsetDateTime::UNIX_EPOCH);
    /// # if false {
    /// assert!(local_offset.is_ok());
    /// # }
    /// ```
    #[cfg(feature = "local-offset")]
    pub fn local_offset_at(datetime: OffsetDateTime) -> Result<Self, error::IndeterminateOffset> {
        local_offset_at(datetime).ok_or(error::IndeterminateOffset)
    }

    /// Attempt to obtain the system's current UTC offset. If the offset cannot be determined, an
    /// error is returned.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// let local_offset = UtcOffset::current_local_offset();
    /// # if false {
    /// assert!(local_offset.is_ok());
    /// # }
    /// ```
    #[cfg(feature = "local-offset")]
    pub fn current_local_offset() -> Result<Self, error::IndeterminateOffset> {
        let now = OffsetDateTime::now_utc();
        local_offset_at(now).ok_or(error::IndeterminateOffset)
    }
    // endregion: local offset
}

// region: formatting & parsing
#[cfg(feature = "formatting")]
impl UtcOffset {
    /// Format the `UtcOffset` using the provided [format description](crate::format_description).
    pub fn format_into(
        self,
        output: &mut impl io::Write,
        format: &(impl Formattable + ?Sized),
    ) -> Result<usize, error::Format> {
        format.format_into(output, None, None, Some(self))
    }

    /// Format the `UtcOffset` using the provided [format description](crate::format_description).
    ///
    /// ```rust
    /// # use time::{format_description, macros::offset};
    /// let format = format_description::parse("[offset_hour sign:mandatory]:[offset_minute]")?;
    /// assert_eq!(offset!(+1).format(&format)?, "+01:00");
    /// # Ok::<_, time::Error>(())
    /// ```
    pub fn format(self, format: &(impl Formattable + ?Sized)) -> Result<String, error::Format> {
        format.format(None, None, Some(self))
    }
}

#[cfg(feature = "parsing")]
impl UtcOffset {
    /// Parse a `UtcOffset` from the input using the provided [format
    /// description](crate::format_description).
    ///
    /// ```rust
    /// # use time::{format_description, macros::offset, UtcOffset};
    /// let format = format_description::parse("[offset_hour]:[offset_minute]")?;
    /// assert_eq!(UtcOffset::parse("-03:42", &format)?, offset!(-3:42));
    /// # Ok::<_, time::Error>(())
    /// ```
    pub fn parse(
        input: &str,
        description: &(impl Parsable + ?Sized),
    ) -> Result<Self, error::Parse> {
        description.parse_offset(input.as_bytes())
    }
}

impl fmt::Display for UtcOffset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{:02}:{:02}:{:02}",
            if self.is_negative() { '-' } else { '+' },
            self.hours.abs(),
            self.minutes.abs(),
            self.seconds.abs()
        )
    }
}
// endregion formatting & parsing

impl Neg for UtcOffset {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::__from_hms_unchecked(-self.hours, -self.minutes, -self.seconds)
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::cmp::Ord;
	use std::cmp::PartialEq;
	use std::clone::Clone;
	use std::cmp::PartialOrd;
	use std::ops::Neg;
	use std::cmp::Eq;
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7001() {
//    rusty_monitor::set_test_id(7001);
    let mut u16_0: u16 = 367u16;
    let mut u8_0: u8 = 28u8;
    let mut u8_1: u8 = 6u8;
    let mut u8_2: u8 = 10u8;
    let mut i32_0: i32 = 3600i32;
    let mut i64_0: i64 = 0i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut i32_1: i32 = 54i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut i8_0: i8 = 23i8;
    let mut i8_1: i8 = 2i8;
    let mut i8_2: i8 = 3i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_2, minutes: i8_1, seconds: i8_0};
    let mut i64_1: i64 = 60i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut i64_2: i64 = 24i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_3, duration_2);
    let mut u32_0: u32 = 2u32;
    let mut u8_3: u8 = 52u8;
    let mut u8_4: u8 = 4u8;
    let mut u8_5: u8 = 92u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_0);
    let mut u16_1: u16 = 10u16;
    let mut i32_2: i32 = 285i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_4);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut i128_0: i128 = -52i128;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_1, duration_5);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_2);
    let mut utcoffset_0_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_0;
    let mut i8_3: i8 = 59i8;
    let mut i8_4: i8 = 1i8;
    let mut i8_5: i8 = 2i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_5, minutes: i8_4, seconds: i8_3};
    let mut utcoffset_2_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_2;
    let mut i8_6: i8 = 0i8;
    let mut i8_7: i8 = 127i8;
    let mut i8_8: i8 = 24i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut utcoffset_3_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_3;
    let mut i8_9: i8 = 26i8;
    let mut i8_10: i8 = 23i8;
    let mut i8_11: i8 = 2i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_11, minutes: i8_10, seconds: i8_9};
    let mut utcoffset_5: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_4);
    let mut utcoffset_5_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_5;
    let mut bool_0: bool = std::cmp::PartialEq::eq(utcoffset_5_ref_0, utcoffset_3_ref_0);
    let mut bool_1: bool = std::cmp::PartialEq::eq(utcoffset_2_ref_0, utcoffset_0_ref_0);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_milli(date_1, u8_2, u8_1, u8_0, u16_0);
    let mut u8_6: u8 = crate::offset_date_time::OffsetDateTime::monday_based_week(offsetdatetime_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2494() {
//    rusty_monitor::set_test_id(2494);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::previous(weekday_0);
    let mut u8_0: u8 = 99u8;
    let mut month_0: month::Month = crate::month::Month::June;
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut i32_0: i32 = 54i32;
    let mut i8_0: i8 = 12i8;
    let mut i8_1: i8 = -54i8;
    let mut i8_2: i8 = 0i8;
    let mut i8_3: i8 = 0i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_3, minutes: i8_2, seconds: i8_1};
    let mut utcoffset_0_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_0;
    let mut i64_0: i64 = 12i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut i64_1: i64 = 60i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut i64_2: i64 = 24i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_2, duration_1);
    let mut u32_0: u32 = 999999999u32;
    let mut u8_1: u8 = 52u8;
    let mut u8_2: u8 = 4u8;
    let mut u8_3: u8 = 92u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_3, u8_2, u8_1, u32_0);
    let mut u16_0: u16 = 10u16;
    let mut i32_1: i32 = 285i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut utcoffset_2: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_1);
    let mut utcoffset_2_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_2;
    let mut i128_0: i128 = -52i128;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_4);
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_3);
    let mut utcoffset_3_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_3;
    let mut i8_4: i8 = 1i8;
    let mut i8_5: i8 = 2i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_5, minutes: i8_4, seconds: i8_0};
    let mut utcoffset_4_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_4;
    let mut i8_6: i8 = 0i8;
    let mut i8_7: i8 = 127i8;
    let mut i8_8: i8 = 14i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut utcoffset_5_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_5;
    let mut i8_9: i8 = 26i8;
    let mut i8_10: i8 = 23i8;
    let mut i8_11: i8 = 2i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_11, minutes: i8_10, seconds: i8_9};
    let mut utcoffset_7: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_6);
    let mut utcoffset_7_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_7;
    let mut bool_0: bool = std::cmp::PartialEq::eq(utcoffset_7_ref_0, utcoffset_5_ref_0);
    let mut bool_1: bool = std::cmp::PartialEq::eq(utcoffset_4_ref_0, utcoffset_3_ref_0);
    let mut bool_2: bool = std::cmp::PartialEq::eq(utcoffset_2_ref_0, utcoffset_0_ref_0);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_calendar_date(i32_0, month_1, u8_0);
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::next(weekday_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1151() {
//    rusty_monitor::set_test_id(1151);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut i8_0: i8 = -54i8;
    let mut i8_1: i8 = 0i8;
    let mut i8_2: i8 = 5i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_2, minutes: i8_1, seconds: i8_0};
    let mut utcoffset_0_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_0;
    let mut i64_0: i64 = 12i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut i64_1: i64 = 60i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut i64_2: i64 = 24i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_2, duration_1);
    let mut u32_0: u32 = 999999999u32;
    let mut u8_0: u8 = 52u8;
    let mut u8_1: u8 = 4u8;
    let mut u8_2: u8 = 92u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 10u16;
    let mut i32_0: i32 = 285i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut utcoffset_2: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_1);
    let mut utcoffset_2_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_2;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_407() {
//    rusty_monitor::set_test_id(407);
    let mut i8_0: i8 = 1i8;
    let mut i8_1: i8 = 2i8;
    let mut i8_2: i8 = 0i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_2, minutes: i8_1, seconds: i8_0};
    let mut i8_3: i8 = -35i8;
    let mut i8_4: i8 = 6i8;
    let mut i8_5: i8 = -5i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_5, minutes: i8_4, seconds: i8_3};
    let mut utcoffset_2: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_1);
    let mut i8_6: i8 = 24i8;
    let mut i8_7: i8 = 24i8;
    let mut i8_8: i8 = 6i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i8_9: i8 = -21i8;
    let mut i8_10: i8 = -120i8;
    let mut i8_11: i8 = 4i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut utcoffset_5: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_4);
    let mut i8_12: i8 = 127i8;
    let mut i8_13: i8 = -88i8;
    let mut i8_14: i8 = 1i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_14, minutes: i8_13, seconds: i8_12};
    let mut utcoffset_7: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_6);
    let mut i8_15: i8 = 59i8;
    let mut i8_16: i8 = 0i8;
    let mut i8_17: i8 = 3i8;
    let mut utcoffset_8: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_17, minutes: i8_16, seconds: i8_15};
    let mut bool_0: bool = crate::utc_offset::UtcOffset::is_negative(utcoffset_8);
    let mut bool_1: bool = crate::utc_offset::UtcOffset::is_negative(utcoffset_7);
    let mut bool_2: bool = crate::utc_offset::UtcOffset::is_negative(utcoffset_5);
    let mut bool_3: bool = crate::utc_offset::UtcOffset::is_negative(utcoffset_3);
    let mut bool_4: bool = crate::utc_offset::UtcOffset::is_negative(utcoffset_2);
    let mut bool_5: bool = crate::utc_offset::UtcOffset::is_negative(utcoffset_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1146() {
//    rusty_monitor::set_test_id(1146);
    let mut i8_0: i8 = 24i8;
    let mut i8_1: i8 = 127i8;
    let mut i8_2: i8 = 127i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_2, minutes: i8_1, seconds: i8_0};
    let mut utcoffset_0_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_0;
    let mut i8_3: i8 = 59i8;
    let mut i8_4: i8 = 6i8;
    let mut i8_5: i8 = 127i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_5, minutes: i8_4, seconds: i8_3};
    let mut utcoffset_1_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_1;
    let mut i32_0: i32 = 303i32;
    let mut i64_0: i64 = 604800i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut i64_1: i64 = 2440588i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_1, duration_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i8_6: i8 = 127i8;
    let mut i8_7: i8 = 3i8;
    let mut i8_8: i8 = 6i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut utcoffset_2_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_2;
    let mut bool_0: bool = std::cmp::PartialEq::ne(utcoffset_1_ref_0, utcoffset_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_405() {
//    rusty_monitor::set_test_id(405);
    let mut i8_0: i8 = 10i8;
    let mut i8_1: i8 = 3i8;
    let mut i8_2: i8 = 24i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_2, minutes: i8_1, seconds: i8_0};
    let mut i8_3: i8 = -120i8;
    let mut i8_4: i8 = 59i8;
    let mut i8_5: i8 = 23i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_5, minutes: i8_4, seconds: i8_3};
    let mut i8_6: i8 = 60i8;
    let mut i8_7: i8 = 59i8;
    let mut i8_8: i8 = -11i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i8_9: i8 = 59i8;
    let mut i8_10: i8 = 23i8;
    let mut i8_11: i8 = 5i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_11, minutes: i8_10, seconds: i8_9};
    let mut i8_12: i8 = 2i8;
    let mut i8_13: i8 = 127i8;
    let mut i8_14: i8 = 60i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_14, minutes: i8_13, seconds: i8_12};
    let mut utcoffset_5: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_4);
    let mut i8_15: i8 = 6i8;
    let mut i8_16: i8 = 127i8;
    let mut i8_17: i8 = 4i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_17, minutes: i8_16, seconds: i8_15};
    let mut bool_0: bool = crate::utc_offset::UtcOffset::is_utc(utcoffset_6);
    let mut bool_1: bool = crate::utc_offset::UtcOffset::is_utc(utcoffset_5);
    let mut bool_2: bool = crate::utc_offset::UtcOffset::is_utc(utcoffset_3);
    let mut bool_3: bool = crate::utc_offset::UtcOffset::is_utc(utcoffset_2);
    let mut bool_4: bool = crate::utc_offset::UtcOffset::is_utc(utcoffset_1);
    let mut bool_5: bool = crate::utc_offset::UtcOffset::is_utc(utcoffset_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_324() {
//    rusty_monitor::set_test_id(324);
    let mut i32_0: i32 = 59i32;
    let mut i64_0: i64 = 126i64;
    let mut i32_1: i32 = 235i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut i8_0: i8 = 1i8;
    let mut i8_1: i8 = 127i8;
    let mut i8_2: i8 = 127i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_2, minutes: i8_1, seconds: i8_0};
    let mut utcoffset_1: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_0);
    let mut u32_0: u32 = 999999u32;
    let mut u8_0: u8 = 59u8;
    let mut u8_1: u8 = 4u8;
    let mut u8_2: u8 = 59u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_2: i32 = 336i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_581() {
//    rusty_monitor::set_test_id(581);
    let mut i32_0: i32 = 240i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut u32_0: u32 = 75u32;
    let mut u8_0: u8 = 60u8;
    let mut u8_1: u8 = 91u8;
    let mut u8_2: u8 = 0u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 365u16;
    let mut i32_1: i32 = 5i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut utcoffset_0_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_0;
    let mut i8_0: i8 = 24i8;
    let mut i8_1: i8 = 5i8;
    let mut i8_2: i8 = 4i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_2, minutes: i8_1, seconds: i8_0};
    let mut utcoffset_1_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_1;
    let mut i8_3: i8 = 24i8;
    let mut i8_4: i8 = 4i8;
    let mut i8_5: i8 = 60i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_5, minutes: i8_4, seconds: i8_3};
    let mut utcoffset_2_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_2;
    let mut i8_6: i8 = 4i8;
    let mut i8_7: i8 = 6i8;
    let mut i8_8: i8 = -6i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_8, minutes: i8_7, seconds: i8_6};
    let mut utcoffset_3_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_3;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(utcoffset_3_ref_0, utcoffset_2_ref_0);
    let mut ordering_1: std::cmp::Ordering = std::cmp::Ord::cmp(utcoffset_1_ref_0, utcoffset_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8118() {
//    rusty_monitor::set_test_id(8118);
    let mut f32_0: f32 = 59.026492f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_0);
    let mut i8_0: i8 = 0i8;
    let mut i8_1: i8 = 6i8;
    let mut i8_2: i8 = 4i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_2, utcoffset_2);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_1);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut i32_0: i32 = 133i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_0};
    let mut i64_0: i64 = 253402300799i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i32_1: i32 = 7i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut i8_3: i8 = 23i8;
    let mut i8_4: i8 = 1i8;
    let mut i8_5: i8 = 24i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut utcoffset_4: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_3);
    let mut bool_0: bool = crate::utc_offset::UtcOffset::is_positive(utcoffset_4);
    let mut tuple_0: (u8, u8, u8, u16) = crate::time::Time::as_hms_milli(time_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1001() {
//    rusty_monitor::set_test_id(1001);
    let mut i64_0: i64 = 12i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut i64_1: i64 = 24i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut u32_0: u32 = 999999999u32;
    let mut u8_0: u8 = 52u8;
    let mut u8_1: u8 = 4u8;
    let mut u8_2: u8 = 92u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 10u16;
    let mut i32_0: i32 = 285i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut i128_0: i128 = -52i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_2);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut utcoffset_0_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_0;
    let mut i8_0: i8 = 0i8;
    let mut i8_1: i8 = 127i8;
    let mut i8_2: i8 = 24i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut utcoffset_1_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_1;
    let mut i8_3: i8 = 26i8;
    let mut i8_4: i8 = 23i8;
    let mut i8_5: i8 = 2i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_5, minutes: i8_4, seconds: i8_3};
    let mut utcoffset_3: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_2);
    let mut utcoffset_3_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_3;
    let mut bool_0: bool = std::cmp::PartialEq::eq(utcoffset_3_ref_0, utcoffset_1_ref_0);
    let mut i64_2: i64 = crate::duration::Duration::whole_hours(duration_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_839() {
//    rusty_monitor::set_test_id(839);
    let mut i8_0: i8 = 3i8;
    let mut i8_1: i8 = 5i8;
    let mut i8_2: i8 = 2i8;
    let mut i8_3: i8 = -54i8;
    let mut i8_4: i8 = 0i8;
    let mut i8_5: i8 = 5i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_5, minutes: i8_4, seconds: i8_3};
    let mut utcoffset_0_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_0;
    let mut i64_0: i64 = 12i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut i64_1: i64 = 60i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut i64_2: i64 = 24i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_2, duration_1);
    let mut u32_0: u32 = 999999999u32;
    let mut u8_0: u8 = 52u8;
    let mut u8_1: u8 = 4u8;
    let mut u8_2: u8 = 92u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 10u16;
    let mut i32_0: i32 = 285i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut utcoffset_2: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_1);
    let mut utcoffset_2_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_2;
    let mut i128_0: i128 = -52i128;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_4);
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_3);
    let mut utcoffset_3_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_3;
    let mut i8_6: i8 = 59i8;
    let mut i8_7: i8 = 1i8;
    let mut i8_8: i8 = 2i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_8, minutes: i8_7, seconds: i8_6};
    let mut utcoffset_4_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_4;
    let mut i8_9: i8 = 0i8;
    let mut i8_10: i8 = 127i8;
    let mut i8_11: i8 = 24i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut utcoffset_5_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_5;
    let mut i8_12: i8 = 26i8;
    let mut i8_13: i8 = 23i8;
    let mut i8_14: i8 = 2i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_14, minutes: i8_13, seconds: i8_12};
    let mut utcoffset_7: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_6);
    let mut utcoffset_7_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_7;
    let mut bool_0: bool = std::cmp::PartialEq::eq(utcoffset_7_ref_0, utcoffset_5_ref_0);
    let mut bool_1: bool = std::cmp::PartialEq::eq(utcoffset_4_ref_0, utcoffset_3_ref_0);
    let mut bool_2: bool = std::cmp::PartialEq::eq(utcoffset_2_ref_0, utcoffset_0_ref_0);
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_2, i8_1, i8_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_421() {
//    rusty_monitor::set_test_id(421);
    let mut i8_0: i8 = 1i8;
    let mut i8_1: i8 = 4i8;
    let mut i8_2: i8 = 3i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_0);
    let mut i8_3: i8 = 0i8;
    let mut i8_4: i8 = 3i8;
    let mut i8_5: i8 = 1i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i32_0: i32 = -16i32;
    let mut i64_0: i64 = 253402300799i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut u16_0: u16 = 7u16;
    let mut i32_1: i32 = 218i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut u32_0: u32 = 0u32;
    let mut u8_0: u8 = 57u8;
    let mut u8_1: u8 = 5u8;
    let mut u8_2: u8 = 39u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_1: u16 = 53u16;
    let mut i32_2: i32 = 172i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_2};
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_1);
    let mut weekday_0: weekday::Weekday = crate::offset_date_time::OffsetDateTime::weekday(offsetdatetime_1);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Sunday;
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::previous(weekday_1);
    let mut weekday_3: weekday::Weekday = crate::weekday::Weekday::Sunday;
    let mut weekday_4: weekday::Weekday = crate::weekday::Weekday::previous(weekday_3);
    let mut u8_3: u8 = crate::weekday::Weekday::number_days_from_monday(weekday_4);
    let mut u8_4: u8 = crate::weekday::Weekday::number_days_from_monday(weekday_2);
    let mut u8_5: u8 = crate::weekday::Weekday::number_days_from_monday(weekday_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1905() {
//    rusty_monitor::set_test_id(1905);
    let mut i8_0: i8 = 6i8;
    let mut i8_1: i8 = 4i8;
    let mut i8_2: i8 = 27i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut f32_0: f32 = 1065353216.000000f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i8_3: i8 = 59i8;
    let mut i8_4: i8 = 6i8;
    let mut i8_5: i8 = 127i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_5, minutes: i8_4, seconds: i8_3};
    let mut utcoffset_1_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_1;
    let mut i64_0: i64 = 2440588i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut u32_0: u32 = 10000000u32;
    let mut u8_0: u8 = 29u8;
    let mut u8_1: u8 = 0u8;
    let mut u8_2: u8 = 53u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 3i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i8_6: i8 = 127i8;
    let mut i8_7: i8 = 3i8;
    let mut i8_8: i8 = 6i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut utcoffset_2_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_2;
    let mut i64_1: i64 = crate::duration::Duration::whole_days(duration_0);
    let mut i16_0: i16 = crate::utc_offset::UtcOffset::whole_minutes(utcoffset_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_403() {
//    rusty_monitor::set_test_id(403);
    let mut i8_0: i8 = 60i8;
    let mut i8_1: i8 = 4i8;
    let mut i8_2: i8 = 59i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_2, minutes: i8_1, seconds: i8_0};
    let mut i8_3: i8 = 23i8;
    let mut i8_4: i8 = 60i8;
    let mut i8_5: i8 = 23i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = -75i8;
    let mut i8_7: i8 = 2i8;
    let mut i8_8: i8 = 6i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i8_9: i8 = 23i8;
    let mut i8_10: i8 = 5i8;
    let mut i8_11: i8 = 0i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i8_12: i8 = 59i8;
    let mut i8_13: i8 = 4i8;
    let mut i8_14: i8 = 60i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut i32_0: i32 = -41i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut i32_1: i32 = crate::utc_offset::UtcOffset::whole_seconds(utcoffset_5);
    let mut i32_2: i32 = crate::utc_offset::UtcOffset::whole_seconds(utcoffset_4);
    let mut i32_3: i32 = crate::utc_offset::UtcOffset::whole_seconds(utcoffset_3);
    let mut i32_4: i32 = crate::utc_offset::UtcOffset::whole_seconds(utcoffset_2);
    let mut i32_5: i32 = crate::utc_offset::UtcOffset::whole_seconds(utcoffset_1);
    let mut i32_6: i32 = crate::utc_offset::UtcOffset::whole_seconds(utcoffset_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2422() {
//    rusty_monitor::set_test_id(2422);
    let mut f64_0: f64 = 0.000000f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut u16_0: u16 = 0u16;
    let mut i32_0: i32 = 325i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut i64_0: i64 = 2440588i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut i8_0: i8 = 0i8;
    let mut i8_1: i8 = 6i8;
    let mut i8_2: i8 = 6i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_1: i32 = 186i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut u32_0: u32 = 10000u32;
    let mut u8_0: u8 = 2u8;
    let mut u8_1: u8 = 60u8;
    let mut u8_2: u8 = 9u8;
    let mut u16_1: u16 = 365u16;
    let mut i32_2: i32 = 364i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_1);
    let mut f32_0: f32 = 59.026492f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_2);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut utcoffset_2: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_1);
    let mut i8_3: i8 = 0i8;
    let mut i8_4: i8 = 6i8;
    let mut i8_5: i8 = 4i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_2, utcoffset_3);
    let mut date_4: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_2, utcoffset_2);
    let mut time_1: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_3);
    let mut i32_3: i32 = 133i32;
    let mut date_5: crate::date::Date = crate::date::Date {value: i32_3};
    let mut u32_1: u32 = 10000u32;
    let mut u8_3: u8 = 25u8;
    let mut u8_4: u8 = 5u8;
    let mut u8_5: u8 = 59u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_1: i64 = 253402300799i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut i32_4: i32 = 7i32;
    let mut date_6: crate::date::Date = crate::date::Date {value: i32_4};
    let mut date_7: crate::date::Date = crate::date::Date::saturating_add(date_6, duration_3);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_7);
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_4, time_2);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_5);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_4, date_5);
    let mut i8_6: i8 = 23i8;
    let mut i8_7: i8 = 1i8;
    let mut i8_8: i8 = 24i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut utcoffset_5: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_4);
    let mut bool_0: bool = crate::utc_offset::UtcOffset::is_positive(utcoffset_5);
    let mut time_3: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_5);
    let mut tuple_0: (u8, u8, u8, u16) = crate::time::Time::as_hms_milli(time_1);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_3, u8_2, u8_1, u8_0, u32_0);
    let mut tuple_1: (util::DateAdjustment, crate::time::Time) = crate::time::Time::adjusting_sub(time_0, duration_1);
    let mut tuple_2: (i32, u8, weekday::Weekday) = crate::date::Date::to_iso_week_date(date_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_676() {
//    rusty_monitor::set_test_id(676);
    let mut u32_0: u32 = 1000000000u32;
    let mut u8_0: u8 = 11u8;
    let mut u8_1: u8 = 6u8;
    let mut u8_2: u8 = 16u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_0: i8 = 14i8;
    let mut i8_1: i8 = 60i8;
    let mut i8_2: i8 = 1i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_2, minutes: i8_1, seconds: i8_0};
    let mut i32_0: i32 = 296i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut u32_1: u32 = 0u32;
    let mut u8_3: u8 = 59u8;
    let mut u8_4: u8 = 7u8;
    let mut u8_5: u8 = 31u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_1: i32 = 9999i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut f64_0: f64 = 4696837146684686336.000000f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_0);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_3);
    let mut utcoffset_2: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_1);
    let mut i8_3: i8 = crate::utc_offset::UtcOffset::minutes_past_hour(utcoffset_2);
    let mut weekday_0: weekday::Weekday = crate::offset_date_time::OffsetDateTime::weekday(offsetdatetime_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_325() {
//    rusty_monitor::set_test_id(325);
    let mut i8_0: i8 = 4i8;
    let mut i8_1: i8 = 59i8;
    let mut i8_2: i8 = 127i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_2, minutes: i8_1, seconds: i8_0};
    let mut utcoffset_1: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_0);
    let mut i8_3: i8 = 23i8;
    let mut i8_4: i8 = 23i8;
    let mut i8_5: i8 = 3i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_5, minutes: i8_4, seconds: i8_3};
    let mut utcoffset_3: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_2);
    let mut u32_0: u32 = 100u32;
    let mut u8_0: u8 = 2u8;
    let mut u8_1: u8 = 9u8;
    let mut u8_2: u8 = 47u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 5i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_1};
    let mut offsetdatetime_0_ref_0: &crate::offset_date_time::OffsetDateTime = &mut offsetdatetime_0;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_766() {
//    rusty_monitor::set_test_id(766);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    let mut u32_0: u32 = 1000000000u32;
    let mut u8_0: u8 = 11u8;
    let mut u8_1: u8 = 6u8;
    let mut u8_2: u8 = 16u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_0: i8 = 14i8;
    let mut i8_1: i8 = 60i8;
    let mut i8_2: i8 = 1i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_2, minutes: i8_1, seconds: i8_0};
    let mut i32_0: i32 = 296i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut u32_1: u32 = 0u32;
    let mut u8_3: u8 = 59u8;
    let mut u8_4: u8 = 7u8;
    let mut u8_5: u8 = 31u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_1: i32 = 9999i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut f64_0: f64 = 4696837146684686336.000000f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_598() {
//    rusty_monitor::set_test_id(598);
    let mut i8_0: i8 = 6i8;
    let mut i8_1: i8 = -5i8;
    let mut i8_2: i8 = 127i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_0: i32 = 7i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut i64_0: i64 = 604800i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut i128_0: i128 = 1000000000i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_1, duration_0);
    let mut u16_0: u16 = 367u16;
    let mut i32_1: i32 = 71i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_0);
    let mut weekday_0: weekday::Weekday = crate::offset_date_time::OffsetDateTime::weekday(offsetdatetime_0);
    let mut u8_0: u8 = 8u8;
    let mut i32_2: i32 = 161i32;
    let mut i8_3: i8 = 1i8;
    let mut i8_4: i8 = 60i8;
    let mut i8_5: i8 = 0i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_5, minutes: i8_4, seconds: i8_3};
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_1, utcoffset_1);
    let mut weekday_1: weekday::Weekday = crate::offset_date_time::OffsetDateTime::weekday(offsetdatetime_2);
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::previous(weekday_1);
    let mut u8_1: u8 = 4u8;
    let mut i32_3: i32 = 9999i32;
    let mut weekday_3: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut weekday_4: weekday::Weekday = crate::weekday::Weekday::next(weekday_3);
    let mut u8_2: u8 = 6u8;
    let mut i32_4: i32 = 320i32;
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_4, u8_2, weekday_4);
    let mut result_1: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_3, u8_1, weekday_2);
    let mut result_2: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_2, u8_0, weekday_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6586() {
//    rusty_monitor::set_test_id(6586);
    let mut u32_0: u32 = 10000u32;
    let mut u8_0: u8 = 12u8;
    let mut u8_1: u8 = 4u8;
    let mut u8_2: u8 = 2u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_0: i64 = 24i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut u16_0: u16 = 365u16;
    let mut i32_0: i32 = 263i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_0};
    let mut i8_0: i8 = 59i8;
    let mut i8_1: i8 = 23i8;
    let mut i8_2: i8 = 24i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut utcoffset_0_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_0;
    let mut u8_3: u8 = 9u8;
    let mut month_0: month::Month = crate::month::Month::April;
    let mut i32_1: i32 = 7i32;
    let mut f64_0: f64 = 4696837146684686336.000000f64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut i8_3: i8 = 2i8;
    let mut i64_1: i64 = 60i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::abs(duration_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_0);
    let mut i8_4: i8 = 60i8;
    let mut i8_5: i8 = 60i8;
    let mut i8_6: i8 = 24i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_6, i8_5, i8_4);
    let mut i8_7: i8 = 4i8;
    let mut i8_8: i8 = 12i8;
    let mut i8_9: i8 = 127i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_9, i8_8, i8_7);
    let mut u32_1: u32 = 100000u32;
    let mut u8_4: u8 = 4u8;
    let mut u8_5: u8 = 28u8;
    let mut u8_6: u8 = 9u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_6, u8_5, u8_4, u32_1);
    let mut i8_10: i8 = 4i8;
    let mut i8_11: i8 = 5i8;
    let mut i8_12: i8 = 127i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_12, i8_11, i8_10);
    let mut i64_2: i64 = 1000i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut f32_0: f32 = 1065353216.000000f32;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i8_13: i8 = 5i8;
    let mut i8_14: i8 = 3i8;
    let mut i8_15: i8 = 6i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_15, i8_14, i8_13);
    let mut u32_2: u32 = 10u32;
    let mut u8_7: u8 = 52u8;
    let mut u8_8: u8 = 34u8;
    let mut u8_9: u8 = 0u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_9, u8_8, u8_7, u32_2);
    let mut i8_16: i8 = 59i8;
    let mut i8_17: i8 = -2i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_3);
    let mut i64_3: i64 = 9223372036854775807i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_7: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_0, duration_1);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_calendar_date(i32_1, month_0, u8_3);
    let mut utcoffset_1_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_1;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(utcoffset_1_ref_0, utcoffset_0_ref_0);
    let mut u32_3: u32 = crate::primitive_date_time::PrimitiveDateTime::microsecond(primitivedatetime_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_565() {
//    rusty_monitor::set_test_id(565);
    let mut i8_0: i8 = 59i8;
    let mut i8_1: i8 = -76i8;
    let mut i8_2: i8 = 24i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_2, minutes: i8_1, seconds: i8_0};
    let mut i32_0: i32 = 2147483647i32;
    let mut i64_0: i64 = 1i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut u16_0: u16 = 7u16;
    let mut i32_1: i32 = -97i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut time_0_ref_0: &crate::time::Time = &mut time_0;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2933() {
//    rusty_monitor::set_test_id(2933);
    let mut i8_0: i8 = 59i8;
    let mut i8_1: i8 = 6i8;
    let mut i8_2: i8 = 127i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_2, minutes: i8_1, seconds: i8_0};
    let mut utcoffset_0_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_0;
    let mut i32_0: i32 = 303i32;
    let mut i64_0: i64 = 604800i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut i64_1: i64 = 2440588i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_1, duration_0);
    let mut u32_0: u32 = 10000000u32;
    let mut u8_0: u8 = 29u8;
    let mut u8_1: u8 = 0u8;
    let mut u8_2: u8 = 53u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = 3i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date_time(offsetdatetime_0, primitivedatetime_1);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut utcoffset_2: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_1);
    let mut utcoffset_2_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_2;
    let mut i8_3: i8 = 127i8;
    let mut i8_4: i8 = 3i8;
    let mut i8_5: i8 = 6i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut utcoffset_3_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_3;
    let mut bool_0: bool = std::cmp::PartialEq::ne(utcoffset_3_ref_0, utcoffset_2_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_293() {
//    rusty_monitor::set_test_id(293);
    let mut i8_0: i8 = 59i8;
    let mut i8_1: i8 = 23i8;
    let mut i8_2: i8 = 24i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_2, minutes: i8_1, seconds: i8_0};
    let mut utcoffset_1: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_0);
    let mut u32_0: u32 = 999999999u32;
    let mut u8_0: u8 = 6u8;
    let mut u8_1: u8 = 30u8;
    let mut u8_2: u8 = 29u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 0u16;
    let mut i32_0: i32 = 93i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_0, utcoffset_1);
    let mut i8_3: i8 = 23i8;
    let mut i8_4: i8 = 1i8;
    let mut i8_5: i8 = 4i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_5, minutes: i8_4, seconds: i8_3};
    let mut i8_6: i8 = 4i8;
    let mut i8_7: i8 = 0i8;
    let mut i8_8: i8 = 60i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_8, minutes: i8_7, seconds: i8_6};
    let mut utcoffset_4: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_3);
    let mut u32_1: u32 = 100000000u32;
    let mut u8_3: u8 = 24u8;
    let mut u8_4: u8 = 45u8;
    let mut u8_5: u8 = 9u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut u16_1: u16 = 92u16;
    let mut i32_1: i32 = 1721425i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_1, utcoffset_4);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_2, offset: utcoffset_2};
    let mut u8_6: u8 = crate::offset_date_time::OffsetDateTime::sunday_based_week(offsetdatetime_1);
    let mut u8_7: u8 = crate::offset_date_time::OffsetDateTime::sunday_based_week(offsetdatetime_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_575() {
//    rusty_monitor::set_test_id(575);
    let mut f64_0: f64 = 4768169126130614272.000000f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_0);
    let mut utcoffset_1_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_1;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut u16_0: u16 = 367u16;
    let mut i32_0: i32 = 6i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_1);
    let mut i32_1: i32 = 178i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_2);
    let mut utcoffset_2_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_2;
    let mut i8_0: i8 = 5i8;
    let mut i8_1: i8 = 3i8;
    let mut i8_2: i8 = 23i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut utcoffset_3_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_3;
    let mut i8_3: i8 = 23i8;
    let mut i8_4: i8 = 49i8;
    let mut i8_5: i8 = 5i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut utcoffset_4_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_4;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = std::clone::Clone::clone(utcoffset_4_ref_0);
    let mut utcoffset_6: crate::utc_offset::UtcOffset = std::clone::Clone::clone(utcoffset_3_ref_0);
    let mut utcoffset_7: crate::utc_offset::UtcOffset = std::clone::Clone::clone(utcoffset_2_ref_0);
    let mut utcoffset_8: crate::utc_offset::UtcOffset = std::clone::Clone::clone(utcoffset_1_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3972() {
//    rusty_monitor::set_test_id(3972);
    let mut i8_0: i8 = 6i8;
    let mut i8_1: i8 = 2i8;
    let mut i8_2: i8 = 63i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut utcoffset_0_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_0;
    let mut i32_0: i32 = 3i32;
    let mut i64_0: i64 = -116i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut i32_1: i32 = 26i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut i32_2: i32 = 160i32;
    let mut i64_1: i64 = 9223372036854775807i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_2);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::abs(duration_1);
    let mut u32_0: u32 = 999999999u32;
    let mut u8_0: u8 = 2u8;
    let mut u8_1: u8 = 60u8;
    let mut u8_2: u8 = 23u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i32_3: i32 = 308i32;
    let mut i64_2: i64 = 60i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_3);
    let mut i32_4: i32 = 392i32;
    let mut i32_5: i32 = 314i32;
    let mut i64_3: i64 = -195i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_5);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_4, i32_4);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_5);
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_3);
    let mut u32_1: u32 = 100u32;
    let mut u8_3: u8 = 53u8;
    let mut u8_4: u8 = 9u8;
    let mut u8_5: u8 = 24u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_6: i32 = 320i32;
    let mut i64_4: i64 = 1000000000i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_6);
    let mut duration_7: std::time::Duration = crate::duration::Duration::abs_std(duration_6);
    let mut i32_7: i32 = 246i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_7);
    let mut u16_0: u16 = 365u16;
    let mut i32_8: i32 = 111i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_8, u16_0);
    let mut tuple_0: (i32, u8) = crate::date::Date::iso_year_week(date_2);
    let mut utcoffset_2_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_2;
    let mut option_0: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(utcoffset_2_ref_0, utcoffset_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_542() {
//    rusty_monitor::set_test_id(542);
    let mut i8_0: i8 = -89i8;
    let mut i8_1: i8 = -69i8;
    let mut i8_2: i8 = 2i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_0);
    let mut i32_0: i32 = 65i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_1);
    let mut primitivedatetime_1_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_1;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2576() {
//    rusty_monitor::set_test_id(2576);
    let mut i32_0: i32 = 37i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_0);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i8_0: i8 = -54i8;
    let mut i8_1: i8 = 0i8;
    let mut i8_2: i8 = 5i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_2, minutes: i8_1, seconds: i8_0};
    let mut utcoffset_0_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_0;
    let mut i64_0: i64 = 12i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut i64_1: i64 = 60i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut i64_2: i64 = 19i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_2, duration_1);
    let mut u32_0: u32 = 999999999u32;
    let mut u8_0: u8 = 52u8;
    let mut u8_1: u8 = 4u8;
    let mut u8_2: u8 = 92u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 10u16;
    let mut i32_1: i32 = 285i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_0);
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_3);
    let mut utcoffset_3: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_2);
    let mut utcoffset_3_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_3;
    let mut i128_0: i128 = -52i128;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_4, duration_4);
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_5);
    let mut utcoffset_4_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_4;
    let mut i8_3: i8 = 59i8;
    let mut i8_4: i8 = 1i8;
    let mut i8_5: i8 = 2i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_5, minutes: i8_4, seconds: i8_3};
    let mut utcoffset_5_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_5;
    let mut i8_6: i8 = 26i8;
    let mut i8_7: i8 = 23i8;
    let mut i8_8: i8 = 2i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_8, minutes: i8_7, seconds: i8_6};
    let mut utcoffset_7: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_6);
    let mut utcoffset_7_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_7;
    let mut bool_0: bool = std::cmp::PartialEq::eq(utcoffset_5_ref_0, utcoffset_4_ref_0);
    let mut bool_1: bool = std::cmp::PartialEq::eq(utcoffset_3_ref_0, utcoffset_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_104() {
//    rusty_monitor::set_test_id(104);
    let mut i32_0: i32 = 320i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut u16_0: u16 = 365u16;
    let mut i32_1: i32 = 353i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut u8_0: u8 = 12u8;
    let mut month_0: month::Month = crate::month::Month::January;
    let mut i32_2: i32 = 144i32;
    let mut i8_0: i8 = -105i8;
    let mut i8_1: i8 = 23i8;
    let mut i8_2: i8 = 23i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_2, minutes: i8_1, seconds: i8_0};
    let mut utcoffset_1: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_0);
    let mut i128_0: i128 = 1i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut u32_0: u32 = 100000u32;
    let mut u8_1: u8 = 88u8;
    let mut u8_2: u8 = 7u8;
    let mut u8_3: u8 = 16u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_3, u8_2, u8_1, u32_0);
    let mut i32_3: i32 = 268i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_1};
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_calendar_date(i32_2, month_0, u8_0);
    let mut u16_1: u16 = crate::primitive_date_time::PrimitiveDateTime::ordinal(primitivedatetime_2);
    let mut tuple_0: (i32, month::Month, u8) = crate::offset_date_time::OffsetDateTime::to_calendar_date(offsetdatetime_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_578() {
//    rusty_monitor::set_test_id(578);
    let mut i64_0: i64 = 107i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut i32_0: i32 = -23i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut f32_0: f32 = 1065353216.000000f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut u32_0: u32 = 10000u32;
    let mut u8_0: u8 = 52u8;
    let mut u8_1: u8 = 11u8;
    let mut u8_2: u8 = 30u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = 365i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_1);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_2, duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_3);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_0);
    let mut utcoffset_1_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_1;
    let mut i64_1: i64 = 12i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_2);
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_3);
    let mut utcoffset_3: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_2);
    let mut utcoffset_3_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_3;
    let mut i64_2: i64 = 60i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut i32_2: i32 = 263i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_3);
    let mut i64_3: i64 = 2147483647i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut i32_3: i32 = 387i32;
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_3};
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(utcoffset_3_ref_0);
    let mut tuple_1: () = std::cmp::Eq::assert_receiver_is_total_eq(utcoffset_1_ref_0);
//    panic!("From RustyUnit with love");
}
}