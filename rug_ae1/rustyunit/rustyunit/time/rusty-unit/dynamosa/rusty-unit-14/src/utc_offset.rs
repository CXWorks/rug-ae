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
	use std::clone::Clone;
	use std::cmp::PartialEq;
	use std::cmp::PartialOrd;
	use std::ops::Neg;
	use std::cmp::Eq;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1767() {
    rusty_monitor::set_test_id(1767);
    let mut i8_0: i8 = -106i8;
    let mut i8_1: i8 = -4i8;
    let mut i8_2: i8 = 82i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut utcoffset_0_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_0;
    let mut f32_0: f32 = -138.587574f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i64_0: i64 = -137i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i32_0: i32 = 37i32;
    let mut i64_1: i64 = -30i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_0);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_3, duration_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_4);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i32_1: i32 = 56i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_1);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut i8_3: i8 = -1i8;
    let mut i8_4: i8 = -102i8;
    let mut i8_5: i8 = -58i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_5, minutes: i8_4, seconds: i8_3};
    let mut i128_0: i128 = -6i128;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_6: std::time::Duration = crate::duration::Duration::abs_std(duration_5);
    let mut i64_2: i64 = 6i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::abs(duration_7);
    let mut i32_2: i32 = -18i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_8);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut utcoffset_3: crate::utc_offset::UtcOffset = std::clone::Clone::clone(utcoffset_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2005() {
    rusty_monitor::set_test_id(2005);
    let mut f32_0: f32 = -138.587574f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i64_0: i64 = -137i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i32_0: i32 = 37i32;
    let mut i64_1: i64 = -30i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_0);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_3, duration_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_4);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i8_0: i8 = -1i8;
    let mut i8_1: i8 = -102i8;
    let mut i8_2: i8 = -58i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_2, minutes: i8_1, seconds: i8_0};
    let mut i128_0: i128 = -6i128;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i64_2: i64 = 6i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::abs(duration_6);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5862() {
    rusty_monitor::set_test_id(5862);
    let mut i32_0: i32 = -79i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut i32_1: i32 = -127i32;
    let mut i64_0: i64 = 60i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_1);
    let mut i8_0: i8 = 85i8;
    let mut i8_1: i8 = -94i8;
    let mut i8_2: i8 = -29i8;
    let mut f32_0: f32 = 172.137804f32;
    let mut i8_3: i8 = -54i8;
    let mut i8_4: i8 = -27i8;
    let mut i8_5: i8 = -68i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_5, minutes: i8_4, seconds: i8_3};
    let mut i64_1: i64 = 212i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut u32_0: u32 = 58u32;
    let mut u8_0: u8 = 82u8;
    let mut u8_1: u8 = 16u8;
    let mut u8_2: u8 = 7u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_2: i64 = 12i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut i32_2: i32 = -85i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_0);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_2, i8_1, i8_0);
    let mut bool_0: bool = crate::duration::Duration::is_positive(duration_0);
    let mut month_1: month::Month = crate::month::Month::May;
    let mut tuple_0: (month::Month, u8) = crate::date::Date::month_day(date_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4292() {
    rusty_monitor::set_test_id(4292);
    let mut u16_0: u16 = 53u16;
    let mut i32_0: i32 = 39i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut i8_0: i8 = 25i8;
    let mut i8_1: i8 = 35i8;
    let mut i8_2: i8 = 43i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_2, minutes: i8_1, seconds: i8_0};
    let mut utcoffset_1: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_1);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i32_1: i32 = 18i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_0);
    let mut f32_0: f32 = -138.587574f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i64_0: i64 = -137i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i32_2: i32 = 37i32;
    let mut i64_1: i64 = -30i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_2);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_3, duration_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_4);
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_3);
    let mut i32_3: i32 = 56i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_3};
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_2, utcoffset_2);
    let mut time_1: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_3);
    let mut i8_3: i8 = -1i8;
    let mut i8_4: i8 = -102i8;
    let mut i8_5: i8 = -58i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_5, minutes: i8_4, seconds: i8_3};
    let mut i128_0: i128 = -6i128;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_6: std::time::Duration = crate::duration::Duration::abs_std(duration_5);
    let mut i64_2: i64 = 6i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::abs(duration_7);
    let mut i32_4: i32 = -18i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_add(date_3, duration_8);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut tuple_0: (i32, month::Month, u8) = crate::primitive_date_time::PrimitiveDateTime::to_calendar_date(primitivedatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6915() {
    rusty_monitor::set_test_id(6915);
    let mut i8_0: i8 = -62i8;
    let mut i8_1: i8 = -34i8;
    let mut i8_2: i8 = -14i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_2, minutes: i8_1, seconds: i8_0};
    let mut utcoffset_0_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_0;
    let mut i8_3: i8 = 16i8;
    let mut i8_4: i8 = -14i8;
    let mut i8_5: i8 = 56i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut utcoffset_1_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_1;
    let mut i32_0: i32 = -127i32;
    let mut i64_0: i64 = 60i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut i8_6: i8 = 85i8;
    let mut i8_7: i8 = -94i8;
    let mut i8_8: i8 = -29i8;
    let mut f32_0: f32 = 172.137804f32;
    let mut i8_9: i8 = -54i8;
    let mut i8_10: i8 = -27i8;
    let mut i8_11: i8 = -68i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_11, minutes: i8_10, seconds: i8_9};
    let mut i64_1: i64 = 212i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut u32_0: u32 = 58u32;
    let mut u8_0: u8 = 82u8;
    let mut u8_1: u8 = 16u8;
    let mut u8_2: u8 = 7u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_2: i64 = 12i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut i32_1: i32 = -85i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_8, i8_7, i8_6);
    let mut bool_0: bool = crate::duration::Duration::is_positive(duration_0);
    let mut month_1: month::Month = crate::month::Month::May;
    let mut bool_1: bool = std::cmp::PartialEq::eq(utcoffset_1_ref_0, utcoffset_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1702() {
    rusty_monitor::set_test_id(1702);
    let mut month_0: month::Month = crate::month::Month::April;
    let mut f32_0: f32 = -138.587574f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i64_0: i64 = -137i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i32_0: i32 = 37i32;
    let mut i64_1: i64 = -30i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_0);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_3, duration_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_4);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i32_1: i32 = 56i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut i8_0: i8 = -1i8;
    let mut i8_1: i8 = -102i8;
    let mut i8_2: i8 = -58i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_2, minutes: i8_1, seconds: i8_0};
    let mut i128_0: i128 = -6i128;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_6: std::time::Duration = crate::duration::Duration::abs_std(duration_5);
    let mut i64_2: i64 = 6i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::abs(duration_7);
    let mut i32_2: i32 = -18i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_8);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8205() {
    rusty_monitor::set_test_id(8205);
    let mut i32_0: i32 = -53i32;
    let mut i64_0: i64 = 66i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i32_1: i32 = -118i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut f32_0: f32 = -138.512535f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut u16_0: u16 = 77u16;
    let mut i32_2: i32 = 323i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut i8_0: i8 = 112i8;
    let mut i8_1: i8 = 36i8;
    let mut i8_2: i8 = 95i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_1: i64 = -52i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut i32_3: i32 = -84i32;
    let mut i64_2: i64 = 11i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_4);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_3);
    let mut i64_3: i64 = 96i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut u32_0: u32 = 6u32;
    let mut u8_0: u8 = 73u8;
    let mut u8_1: u8 = 59u8;
    let mut u8_2: u8 = 27u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_4: i32 = -9i32;
    let mut u32_1: u32 = 38u32;
    let mut u8_3: u8 = 98u8;
    let mut u8_4: u8 = 32u8;
    let mut u8_5: u8 = 96u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut u32_2: u32 = 40u32;
    let mut u8_6: u8 = 75u8;
    let mut u8_7: u8 = 72u8;
    let mut u8_8: u8 = 15u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut u16_1: u16 = 58u16;
    let mut i32_5: i32 = 63i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_5, u16_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_3);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_1, time_2);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_2);
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_4);
    let mut i8_3: i8 = -21i8;
    let mut i8_4: i8 = 21i8;
    let mut i8_5: i8 = 10i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u16_2: u16 = 28u16;
    let mut i32_6: i32 = 50i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_6, u16_2);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_3, time: time_1};
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_3);
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut month_0: month::Month = crate::month::Month::February;
    let mut u32_3: u32 = crate::primitive_date_time::PrimitiveDateTime::microsecond(primitivedatetime_0);
    let mut utcoffset_0_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_0;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(utcoffset_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1406() {
    rusty_monitor::set_test_id(1406);
    let mut i128_0: i128 = -3i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i8_0: i8 = 104i8;
    let mut i8_1: i8 = 14i8;
    let mut i8_2: i8 = 63i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_0);
    let mut utcoffset_1_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_1;
    let mut i32_0: i32 = 125i32;
    let mut i64_0: i64 = -104i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut u32_0: u32 = 68u32;
    let mut u8_0: u8 = 89u8;
    let mut u8_1: u8 = 37u8;
    let mut u8_2: u8 = 17u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_1: i64 = 66i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut i32_1: i32 = 5i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_1);
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut utcoffset_2_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_2;
    let mut f32_0: f32 = -138.587574f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut i64_2: i64 = -137i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut i32_2: i32 = 37i32;
    let mut i64_3: i64 = -30i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_2);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_6, duration_5);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_7);
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_3);
    let mut i32_3: i32 = 56i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_3};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_1, utcoffset_3);
    let mut time_1: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_2);
    let mut i8_3: i8 = -1i8;
    let mut i8_4: i8 = -102i8;
    let mut i8_5: i8 = -58i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_5, minutes: i8_4, seconds: i8_3};
    let mut i128_1: i128 = -6i128;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut duration_9: std::time::Duration = crate::duration::Duration::abs_std(duration_8);
    let mut i64_4: i64 = 6i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_4);
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::abs(duration_10);
    let mut i32_4: i32 = -18i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_add(date_3, duration_11);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut option_0: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(utcoffset_2_ref_0, utcoffset_1_ref_0);
    let mut f64_0: f64 = crate::duration::Duration::as_seconds_f64(duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1509() {
    rusty_monitor::set_test_id(1509);
    let mut f32_0: f32 = -138.587574f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i64_0: i64 = -137i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i32_0: i32 = 37i32;
    let mut i64_1: i64 = -30i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_0);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_3, duration_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_4);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i32_1: i32 = 56i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut i8_0: i8 = -1i8;
    let mut i8_1: i8 = -102i8;
    let mut i8_2: i8 = -58i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_2, minutes: i8_1, seconds: i8_0};
    let mut i128_0: i128 = -6i128;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_6: std::time::Duration = crate::duration::Duration::abs_std(duration_5);
    let mut i32_2: i32 = -18i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1639() {
    rusty_monitor::set_test_id(1639);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut f32_0: f32 = -138.587574f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut i64_0: i64 = -137i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i32_0: i32 = 37i32;
    let mut i64_1: i64 = -30i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_0);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_0, duration_4);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_5);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i32_1: i32 = 56i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut i8_0: i8 = -1i8;
    let mut i8_1: i8 = -102i8;
    let mut i8_2: i8 = -58i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_2, minutes: i8_1, seconds: i8_0};
    let mut i128_0: i128 = -6i128;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_7: std::time::Duration = crate::duration::Duration::abs_std(duration_6);
    let mut i64_2: i64 = 6i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::abs(duration_8);
    let mut i32_2: i32 = -18i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_9);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2686() {
    rusty_monitor::set_test_id(2686);
    let mut i32_0: i32 = 42i32;
    let mut i64_0: i64 = -63i64;
    let mut i32_1: i32 = -127i32;
    let mut i64_1: i64 = 60i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_1);
    let mut f32_0: f32 = 172.137804f32;
    let mut i8_0: i8 = -54i8;
    let mut i8_1: i8 = -27i8;
    let mut i8_2: i8 = -68i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_2, minutes: i8_1, seconds: i8_0};
    let mut i64_2: i64 = 212i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut u32_0: u32 = 58u32;
    let mut u8_0: u8 = 82u8;
    let mut u8_1: u8 = 16u8;
    let mut u8_2: u8 = 7u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_3: i64 = 12i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut i32_2: i32 = -85i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_3);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut bool_0: bool = crate::duration::Duration::is_positive(duration_0);
    let mut month_1: month::Month = crate::month::Month::May;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut u8_3: u8 = crate::util::weeks_in_year(i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6709() {
    rusty_monitor::set_test_id(6709);
    let mut i8_0: i8 = -56i8;
    let mut i8_1: i8 = 88i8;
    let mut i8_2: i8 = 101i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_2, minutes: i8_1, seconds: i8_0};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut i128_0: i128 = -36i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i32_0: i32 = 12i32;
    let mut i64_0: i64 = 79i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut i8_3: i8 = 17i8;
    let mut i8_4: i8 = 32i8;
    let mut i8_5: i8 = -70i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_2, utcoffset_1);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut u32_0: u32 = 6u32;
    let mut u8_0: u8 = 74u8;
    let mut u8_1: u8 = 77u8;
    let mut u8_2: u8 = 50u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut f32_0: f32 = -16.926092f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_5, duration_3);
    let mut i32_1: i32 = 46i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut i64_1: i64 = crate::duration::Duration::whole_seconds(duration_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_1, duration_0);
    let mut u32_1: u32 = crate::offset_date_time::OffsetDateTime::microsecond(offsetdatetime_1);
    let mut tuple_0: (u8, u8, u8) = crate::primitive_date_time::PrimitiveDateTime::as_hms(primitivedatetime_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7258() {
    rusty_monitor::set_test_id(7258);
    let mut i32_0: i32 = -127i32;
    let mut i64_0: i64 = 60i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut i8_0: i8 = 85i8;
    let mut i8_1: i8 = -94i8;
    let mut i8_2: i8 = -29i8;
    let mut i8_3: i8 = -54i8;
    let mut i8_4: i8 = -27i8;
    let mut i8_5: i8 = -68i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_5, minutes: i8_4, seconds: i8_3};
    let mut i64_1: i64 = 212i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut i64_2: i64 = 12i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut i32_1: i32 = -85i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_3);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_2, i8_1, i8_0);
    let mut bool_0: bool = crate::duration::Duration::is_positive(duration_0);
    let mut month_1: month::Month = crate::month::Month::May;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_30() {
    rusty_monitor::set_test_id(30);
    let mut i8_0: i8 = 28i8;
    let mut i8_1: i8 = 7i8;
    let mut i8_2: i8 = 47i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_0: i64 = 164i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut i32_0: i32 = -4i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut u32_0: u32 = 8u32;
    let mut u8_0: u8 = 87u8;
    let mut u8_1: u8 = 25u8;
    let mut u8_2: u8 = 7u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = 83i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_0};
    let mut i8_3: i8 = -30i8;
    let mut i8_4: i8 = -63i8;
    let mut i8_5: i8 = 33i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_5, minutes: i8_4, seconds: i8_3};
    let mut utcoffset_2_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_2;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Sunday;
    let mut u8_3: u8 = crate::weekday::Weekday::number_from_monday(weekday_0);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_2, utcoffset_1);
    let mut tuple_0: (u8, u8, u8) = crate::primitive_date_time::PrimitiveDateTime::as_hms(primitivedatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2050() {
    rusty_monitor::set_test_id(2050);
    let mut i32_0: i32 = 60i32;
    let mut i64_0: i64 = -135i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut i32_1: i32 = -127i32;
    let mut i64_1: i64 = 60i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_1);
    let mut i8_0: i8 = 85i8;
    let mut i8_1: i8 = -94i8;
    let mut i8_2: i8 = -29i8;
    let mut f32_0: f32 = 172.137804f32;
    let mut i8_3: i8 = -54i8;
    let mut i8_4: i8 = -27i8;
    let mut i8_5: i8 = -68i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_5, minutes: i8_4, seconds: i8_3};
    let mut i64_2: i64 = 212i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut u32_0: u32 = 58u32;
    let mut u8_0: u8 = 82u8;
    let mut u8_1: u8 = 16u8;
    let mut u8_2: u8 = 7u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_3: i64 = 12i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut i32_2: i32 = -85i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_4);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_2, i8_1, i8_0);
    let mut bool_0: bool = crate::duration::Duration::is_positive(duration_1);
    let mut month_1: month::Month = crate::month::Month::May;
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_sub(instant_0, duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4928() {
    rusty_monitor::set_test_id(4928);
    let mut u32_0: u32 = 62u32;
    let mut u8_0: u8 = 73u8;
    let mut u8_1: u8 = 33u8;
    let mut u8_2: u8 = 75u8;
    let mut i32_0: i32 = -127i32;
    let mut i64_0: i64 = 60i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut i8_0: i8 = 85i8;
    let mut i8_1: i8 = -94i8;
    let mut i8_2: i8 = -29i8;
    let mut f32_0: f32 = 172.137804f32;
    let mut i8_3: i8 = -54i8;
    let mut i8_4: i8 = -27i8;
    let mut i8_5: i8 = -68i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_5, minutes: i8_4, seconds: i8_3};
    let mut i64_1: i64 = 212i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut u32_1: u32 = 58u32;
    let mut u8_3: u8 = 82u8;
    let mut u8_4: u8 = 16u8;
    let mut u8_5: u8 = 7u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_2: i64 = 12i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut i32_1: i32 = -85i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_2, i8_1, i8_0);
    let mut bool_0: bool = crate::duration::Duration::is_positive(duration_0);
    let mut month_1: month::Month = crate::month::Month::May;
    let mut result_1: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_nano(u8_2, u8_1, u8_0, u32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7524() {
    rusty_monitor::set_test_id(7524);
    let mut i8_0: i8 = -48i8;
    let mut i8_1: i8 = 2i8;
    let mut i8_2: i8 = -26i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i32_0: i32 = -83i32;
    let mut i64_0: i64 = 128i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut f64_0: f64 = -102.045325f64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i32_1: i32 = -248i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut u32_0: u32 = 24u32;
    let mut u8_0: u8 = 73u8;
    let mut u8_1: u8 = 88u8;
    let mut u8_2: u8 = 65u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_2: i32 = 49i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_2, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_1);
    let mut i32_3: i32 = -127i32;
    let mut i64_1: i64 = 60i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_3);
    let mut i8_3: i8 = 85i8;
    let mut i8_4: i8 = -94i8;
    let mut i8_5: i8 = -29i8;
    let mut f32_0: f32 = 172.137804f32;
    let mut i8_6: i8 = -54i8;
    let mut i8_7: i8 = -27i8;
    let mut i8_8: i8 = -68i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_8, minutes: i8_7, seconds: i8_6};
    let mut i64_2: i64 = 212i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut u32_1: u32 = 58u32;
    let mut u8_3: u8 = 82u8;
    let mut u8_4: u8 = 16u8;
    let mut u8_5: u8 = 7u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_3: i64 = 12i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut i32_4: i32 = -85i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_4};
    let mut date_4: crate::date::Date = crate::date::Date::saturating_sub(date_3, duration_5);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_4, time_1);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_5, i8_4, i8_3);
    let mut bool_0: bool = crate::duration::Duration::is_positive(duration_2);
    let mut month_1: month::Month = crate::month::Month::May;
    let mut tuple_0: (u8, u8, u8, u32) = crate::primitive_date_time::PrimitiveDateTime::as_hms_nano(primitivedatetime_1);
    let mut f64_1: f64 = crate::duration::Duration::as_seconds_f64(duration_0);
    let mut tuple_1: (i32, month::Month, u8) = crate::offset_date_time::OffsetDateTime::to_calendar_date(offsetdatetime_0);
    let mut i32_5: i32 = crate::utc_offset::UtcOffset::whole_seconds(utcoffset_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4392() {
    rusty_monitor::set_test_id(4392);
    let mut i8_0: i8 = -74i8;
    let mut i8_1: i8 = 92i8;
    let mut i8_2: i8 = 98i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_2, minutes: i8_1, seconds: i8_0};
    let mut utcoffset_1: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_0);
    let mut i64_0: i64 = -83i64;
    let mut f32_0: f32 = -138.587574f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i32_0: i32 = 37i32;
    let mut i64_1: i64 = -30i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_0);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_3, duration_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_4);
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i32_1: i32 = 56i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_1);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut i8_3: i8 = -1i8;
    let mut i8_4: i8 = -102i8;
    let mut i8_5: i8 = -58i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_5, minutes: i8_4, seconds: i8_3};
    let mut i128_0: i128 = -6i128;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_6: std::time::Duration = crate::duration::Duration::abs_std(duration_5);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3285() {
    rusty_monitor::set_test_id(3285);
    let mut i64_0: i64 = -156i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut u32_0: u32 = 87u32;
    let mut u8_0: u8 = 41u8;
    let mut u8_1: u8 = 20u8;
    let mut u8_2: u8 = 95u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut f32_0: f32 = -138.587574f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut i64_1: i64 = -137i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut i32_0: i32 = 37i32;
    let mut i64_2: i64 = -30i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_0);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_4, duration_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_1, duration_0);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_3);
    let mut i32_1: i32 = 56i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_0);
    let mut time_1: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut i8_0: i8 = -1i8;
    let mut i8_1: i8 = -102i8;
    let mut i8_2: i8 = -58i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_2, minutes: i8_1, seconds: i8_0};
    let mut i128_0: i128 = -6i128;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_7: std::time::Duration = crate::duration::Duration::abs_std(duration_6);
    let mut i64_3: i64 = 6i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::abs(duration_8);
    let mut i32_2: i32 = -18i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_9);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3336() {
    rusty_monitor::set_test_id(3336);
    let mut f32_0: f32 = -138.587574f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i64_0: i64 = -137i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i32_0: i32 = 37i32;
    let mut i64_1: i64 = -30i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_0);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_3, duration_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_4);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i8_0: i8 = -1i8;
    let mut i8_1: i8 = -102i8;
    let mut i8_2: i8 = -58i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_2, minutes: i8_1, seconds: i8_0};
    let mut i64_2: i64 = 6i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::abs(duration_5);
    let mut i32_1: i32 = -18i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_6);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2850() {
    rusty_monitor::set_test_id(2850);
    let mut i64_0: i64 = -55i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut f32_0: f32 = -138.587574f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut i64_1: i64 = -137i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i32_0: i32 = 56i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut i8_0: i8 = -1i8;
    let mut i8_1: i8 = -102i8;
    let mut i8_2: i8 = -58i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_2, minutes: i8_1, seconds: i8_0};
    let mut i128_0: i128 = -6i128;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut i64_2: i64 = 6i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::abs(duration_6);
    let mut i32_1: i32 = -18i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_7);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut u16_0: u16 = crate::offset_date_time::OffsetDateTime::millisecond(offsetdatetime_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8136() {
    rusty_monitor::set_test_id(8136);
    let mut i64_0: i64 = -78i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut u32_0: u32 = 69u32;
    let mut u8_0: u8 = 28u8;
    let mut u8_1: u8 = 50u8;
    let mut u8_2: u8 = 60u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = -127i32;
    let mut i64_1: i64 = 60i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut i8_0: i8 = 85i8;
    let mut i8_1: i8 = -94i8;
    let mut i8_2: i8 = -29i8;
    let mut f32_0: f32 = 172.137804f32;
    let mut i8_3: i8 = -54i8;
    let mut i8_4: i8 = -27i8;
    let mut i8_5: i8 = -68i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_5, minutes: i8_4, seconds: i8_3};
    let mut i64_2: i64 = 212i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut u32_1: u32 = 58u32;
    let mut u8_3: u8 = 82u8;
    let mut u8_4: u8 = 16u8;
    let mut u8_5: u8 = 7u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_3: i64 = 12i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut i32_1: i32 = -85i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_4);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_1);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_2, i8_1, i8_0);
    let mut bool_0: bool = crate::duration::Duration::is_positive(duration_1);
    let mut month_1: month::Month = crate::month::Month::May;
    let mut tuple_0: (util::DateAdjustment, crate::time::Time) = crate::time::Time::adjusting_sub(time_0, duration_0);
    panic!("From RustyUnit with love");
}
}