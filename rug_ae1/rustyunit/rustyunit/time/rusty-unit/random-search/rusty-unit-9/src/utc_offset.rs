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
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4010() {
    rusty_monitor::set_test_id(4010);
    let mut u16_0: u16 = 18u16;
    let mut i32_0: i32 = -13i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut u32_0: u32 = 34u32;
    let mut u8_0: u8 = 65u8;
    let mut u8_1: u8 = 37u8;
    let mut u8_2: u8 = 11u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = 62i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_0};
    let mut i8_0: i8 = 60i8;
    let mut i8_1: i8 = -60i8;
    let mut i8_2: i8 = 62i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_2, minutes: i8_1, seconds: i8_0};
    let mut bool_0: bool = crate::utc_offset::UtcOffset::is_positive(utcoffset_0);
    let mut u8_3: u8 = crate::primitive_date_time::PrimitiveDateTime::sunday_based_week(primitivedatetime_0);
    let mut tuple_0: (month::Month, u8) = crate::date::Date::month_day(date_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1820() {
    rusty_monitor::set_test_id(1820);
    let mut i8_0: i8 = 7i8;
    let mut i8_1: i8 = 51i8;
    let mut i8_2: i8 = -3i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_0: i64 = 53i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut u32_0: u32 = 71u32;
    let mut u8_0: u8 = 2u8;
    let mut u8_1: u8 = 37u8;
    let mut u8_2: u8 = 51u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 78i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_0);
    let mut i8_3: i8 = -91i8;
    let mut i8_4: i8 = 7i8;
    let mut i8_5: i8 = -36i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_5, minutes: i8_4, seconds: i8_3};
    let mut bool_0: bool = crate::utc_offset::UtcOffset::is_negative(utcoffset_1);
    let mut u16_0: u16 = crate::offset_date_time::OffsetDateTime::ordinal(offsetdatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2589() {
    rusty_monitor::set_test_id(2589);
    let mut i8_0: i8 = -43i8;
    let mut i8_1: i8 = -69i8;
    let mut i8_2: i8 = 4i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_2, minutes: i8_1, seconds: i8_0};
    let mut i64_0: i64 = -2i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut u32_0: u32 = 45u32;
    let mut u8_0: u8 = 4u8;
    let mut u8_1: u8 = 48u8;
    let mut u8_2: u8 = 18u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_1: i64 = 92i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut i32_0: i32 = 2i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut i8_3: i8 = -80i8;
    let mut i8_4: i8 = 98i8;
    let mut i8_5: i8 = 41i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_5, minutes: i8_4, seconds: i8_3};
    let mut utcoffset_2: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_1);
    let mut i8_6: i8 = crate::utc_offset::UtcOffset::whole_hours(utcoffset_2);
    let mut u8_3: u8 = crate::offset_date_time::OffsetDateTime::minute(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_62() {
    rusty_monitor::set_test_id(62);
    let mut i8_0: i8 = -31i8;
    let mut i8_1: i8 = -91i8;
    let mut i8_2: i8 = 81i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_2, minutes: i8_1, seconds: i8_0};
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_0: i64 = 134i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut i32_0: i32 = -158i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut month_0: month::Month = crate::month::Month::July;
    let mut month_0_ref_0: &month::Month = &mut month_0;
    let mut i8_3: i8 = -31i8;
    let mut i8_4: i8 = 14i8;
    let mut i8_5: i8 = 64i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut bool_0: bool = crate::utc_offset::UtcOffset::is_negative(utcoffset_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4449() {
    rusty_monitor::set_test_id(4449);
    let mut u32_0: u32 = 48u32;
    let mut u8_0: u8 = 70u8;
    let mut u8_1: u8 = 98u8;
    let mut u8_2: u8 = 48u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 13i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut i8_0: i8 = 27i8;
    let mut i8_1: i8 = 64i8;
    let mut i8_2: i8 = -33i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_2, minutes: i8_1, seconds: i8_0};
    let mut i8_3: i8 = -81i8;
    let mut i8_4: i8 = -15i8;
    let mut i8_5: i8 = 68i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut utcoffset_2: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_1);
    let mut utcoffset_2_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_2;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(utcoffset_2_ref_0);
    let mut i32_1: i32 = crate::utc_offset::UtcOffset::whole_seconds(utcoffset_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2963() {
    rusty_monitor::set_test_id(2963);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i8_0: i8 = -5i8;
    let mut i8_1: i8 = -44i8;
    let mut i8_2: i8 = 13i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_2, utcoffset_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut i8_3: i8 = -5i8;
    let mut i8_4: i8 = 10i8;
    let mut i8_5: i8 = 94i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = -90i8;
    let mut i8_7: i8 = 80i8;
    let mut i8_8: i8 = -34i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_8, minutes: i8_7, seconds: i8_6};
    let mut i64_0: i64 = 174i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut i32_0: i32 = -170i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_1, utcoffset_2);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_2, offset: utcoffset_1};
    let mut month_0: month::Month = crate::offset_date_time::OffsetDateTime::month(offsetdatetime_4);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut month_1_ref_0: &month::Month = &mut month_1;
    let mut tuple_0: (i32, month::Month, u8) = crate::primitive_date_time::PrimitiveDateTime::to_calendar_date(primitivedatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2450() {
    rusty_monitor::set_test_id(2450);
    let mut i8_0: i8 = -17i8;
    let mut i8_1: i8 = 108i8;
    let mut i8_2: i8 = 65i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_2, minutes: i8_1, seconds: i8_0};
    let mut i64_0: i64 = -160i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut u32_0: u32 = 83u32;
    let mut u8_0: u8 = 85u8;
    let mut u8_1: u8 = 73u8;
    let mut u8_2: u8 = 23u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 106i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut i8_3: i8 = 37i8;
    let mut i8_4: i8 = -12i8;
    let mut i8_5: i8 = 87i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_5, minutes: i8_4, seconds: i8_3};
    let mut utcoffset_1_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_1;
    let mut u8_3: u8 = crate::offset_date_time::OffsetDateTime::hour(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1533() {
    rusty_monitor::set_test_id(1533);
    let mut i8_0: i8 = -8i8;
    let mut i8_1: i8 = -36i8;
    let mut i8_2: i8 = 26i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_2, minutes: i8_1, seconds: i8_0};
    let mut utcoffset_0_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_0;
    let mut u16_0: u16 = 72u16;
    let mut i32_0: i32 = -5i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut u32_0: u32 = 41u32;
    let mut u8_0: u8 = 19u8;
    let mut u8_1: u8 = 12u8;
    let mut u8_2: u8 = 10u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut f64_0: f64 = -164.546322f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i64_0: i64 = -191i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_0);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut utcoffset_1_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_1;
    let mut option_0: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(utcoffset_1_ref_0, utcoffset_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1168() {
    rusty_monitor::set_test_id(1168);
    let mut i64_0: i64 = -74i64;
    let mut i8_0: i8 = -47i8;
    let mut i8_1: i8 = 70i8;
    let mut i8_2: i8 = -37i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_2, minutes: i8_1, seconds: i8_0};
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::previous(weekday_1);
    let mut u8_0: u8 = 58u8;
    let mut i32_0: i32 = 91i32;
    let mut i32_1: i32 = -54i32;
    let mut i64_1: i64 = 240i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_1);
    let mut i32_2: i32 = -34i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_2};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_1);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut utcoffset_2: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_1);
    let mut i8_3: i8 = crate::utc_offset::UtcOffset::seconds_past_minute(utcoffset_2);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_0, u8_0, weekday_2);
    let mut date_1: crate::date::Date = std::result::Result::unwrap(result_0);
    let mut u8_1: u8 = crate::weekday::Weekday::number_days_from_monday(weekday_0);
    let mut i8_4: i8 = crate::utc_offset::UtcOffset::seconds_past_minute(utcoffset_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_132() {
    rusty_monitor::set_test_id(132);
    let mut i32_0: i32 = -47i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut u16_0: u16 = 42u16;
    let mut i32_1: i32 = 84i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_0};
    let mut i64_0: i64 = -47i64;
    let mut i8_0: i8 = -106i8;
    let mut i8_1: i8 = -5i8;
    let mut i8_2: i8 = 17i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_2, minutes: i8_1, seconds: i8_0};
    let mut utcoffset_0_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_0;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut u16_1: u16 = crate::primitive_date_time::PrimitiveDateTime::millisecond(primitivedatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4558() {
    rusty_monitor::set_test_id(4558);
    let mut f32_0: f32 = -12.894284f32;
    let mut i32_0: i32 = -77i32;
    let mut i64_0: i64 = -10i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i8_0: i8 = 29i8;
    let mut i8_1: i8 = 44i8;
    let mut i8_2: i8 = 85i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_0);
    let mut utcoffset_1_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_1;
    let mut i8_3: i8 = 110i8;
    let mut i8_4: i8 = -117i8;
    let mut i8_5: i8 = -43i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut utcoffset_2_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_2;
    let mut bool_0: bool = std::cmp::PartialEq::ne(utcoffset_2_ref_0, utcoffset_1_ref_0);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_add(duration_1, duration_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3971() {
    rusty_monitor::set_test_id(3971);
    let mut i64_0: i64 = -106i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut i32_0: i32 = -135i32;
    let mut i64_1: i64 = 97i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut u16_0: u16 = 37u16;
    let mut i32_1: i32 = 236i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_1);
    let mut i8_0: i8 = -88i8;
    let mut i8_1: i8 = 20i8;
    let mut i8_2: i8 = 11i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_2, minutes: i8_1, seconds: i8_0};
    let mut utcoffset_0_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_0;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(utcoffset_0_ref_0);
    let mut tuple_1: (i32, u16) = crate::date::Date::to_ordinal_date(date_1);
    let mut month_0: month::Month = crate::month::Month::March;
    let mut i128_0: i128 = crate::duration::Duration::whole_nanoseconds(duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3798() {
    rusty_monitor::set_test_id(3798);
    let mut i8_0: i8 = -28i8;
    let mut i8_1: i8 = 44i8;
    let mut i8_2: i8 = 0i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_2, minutes: i8_1, seconds: i8_0};
    let mut utcoffset_0_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_0;
    let mut i128_0: i128 = 141i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i64_0: i64 = -6i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_1, duration_0);
    let mut u16_0: u16 = 12u16;
    let mut i32_0: i32 = -39i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut u16_1: u16 = 53u16;
    let mut i32_1: i32 = -71i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2021() {
    rusty_monitor::set_test_id(2021);
    let mut f64_0: f64 = -86.591666f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i32_0: i32 = 46i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut f64_1: f64 = 45.343924f64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut i32_1: i32 = -39i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_1);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut i8_0: i8 = 68i8;
    let mut i8_1: i8 = -46i8;
    let mut i8_2: i8 = 76i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_1);
    let mut month_0: month::Month = crate::month::Month::July;
    let mut u8_0: u8 = crate::offset_date_time::OffsetDateTime::sunday_based_week(offsetdatetime_1);
    let mut u16_0: u16 = crate::time::Time::millisecond(time_0);
    let mut tuple_0: (month::Month, u8) = crate::date::Date::month_day(date_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4438() {
    rusty_monitor::set_test_id(4438);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut i64_0: i64 = 107i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut i64_1: i64 = -38i64;
    let mut i8_0: i8 = -74i8;
    let mut i8_1: i8 = -53i8;
    let mut i8_2: i8 = -19i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_2, minutes: i8_1, seconds: i8_0};
    let mut utcoffset_1: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_0);
    let mut i16_0: i16 = crate::utc_offset::UtcOffset::whole_minutes(utcoffset_1);
    let mut month_0: month::Month = crate::month::Month::April;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut bool_0: bool = crate::duration::Duration::is_positive(duration_2);
    let mut i64_2: i64 = crate::duration::Duration::whole_seconds(duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2519() {
    rusty_monitor::set_test_id(2519);
    let mut u32_0: u32 = 54u32;
    let mut u8_0: u8 = 1u8;
    let mut u8_1: u8 = 81u8;
    let mut u8_2: u8 = 27u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 0i32;
    let mut i64_0: i64 = -45i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut i32_1: i32 = -11i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_0);
    let mut i8_0: i8 = 22i8;
    let mut i8_1: i8 = 113i8;
    let mut i8_2: i8 = -27i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_2, minutes: i8_1, seconds: i8_0};
    let mut utcoffset_0_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_0;
    let mut u8_3: u8 = crate::primitive_date_time::PrimitiveDateTime::iso_week(primitivedatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2489() {
    rusty_monitor::set_test_id(2489);
    let mut i32_0: i32 = 35i32;
    let mut i64_0: i64 = 58i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut i64_1: i64 = -13i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut u16_0: u16 = 50u16;
    let mut i32_1: i32 = 102i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut i8_0: i8 = 85i8;
    let mut i8_1: i8 = -80i8;
    let mut i8_2: i8 = 13i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_2, minutes: i8_1, seconds: i8_0};
    let mut tuple_0: (i8, i8, i8) = crate::utc_offset::UtcOffset::as_hms(utcoffset_0);
    let mut month_0: month::Month = crate::month::Month::June;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut u8_0: u8 = crate::offset_date_time::OffsetDateTime::iso_week(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_665() {
    rusty_monitor::set_test_id(665);
    let mut i32_0: i32 = 31i32;
    let mut i64_0: i64 = 178i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut i64_1: i64 = 36i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_1);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_0);
    let mut u8_0: u8 = 18u8;
    let mut i64_2: i64 = -204i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut i8_0: i8 = -31i8;
    let mut i8_1: i8 = -77i8;
    let mut i8_2: i8 = 39i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut bool_0: bool = crate::utc_offset::UtcOffset::is_utc(utcoffset_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1002() {
    rusty_monitor::set_test_id(1002);
    let mut i8_0: i8 = -62i8;
    let mut i8_1: i8 = -17i8;
    let mut i8_2: i8 = 61i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut utcoffset_0_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_0;
    let mut i8_3: i8 = 7i8;
    let mut i8_4: i8 = 13i8;
    let mut i8_5: i8 = 49i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut utcoffset_1_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_1;
    let mut f64_0: f64 = -137.765550f64;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: std::time::Instant = crate::instant::Instant::into_inner(instant_2);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut bool_0: bool = std::cmp::PartialEq::ne(utcoffset_1_ref_0, utcoffset_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3352() {
    rusty_monitor::set_test_id(3352);
    let mut bool_0: bool = true;
    let mut i64_0: i64 = -89i64;
    let mut i64_1: i64 = -57i64;
    let mut i64_2: i64 = -93i64;
    let mut str_0: &str = "bPIj5wOOeyQSSG";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i8_0: i8 = -39i8;
    let mut i8_1: i8 = 54i8;
    let mut i8_2: i8 = -21i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_2, minutes: i8_1, seconds: i8_0};
    let mut utcoffset_0_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_0;
    let mut i8_3: i8 = -79i8;
    let mut i8_4: i8 = 59i8;
    let mut i8_5: i8 = 78i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut utcoffset_1_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_1;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(utcoffset_1_ref_0, utcoffset_0_ref_0);
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3088() {
    rusty_monitor::set_test_id(3088);
    let mut i8_0: i8 = -66i8;
    let mut i8_1: i8 = 38i8;
    let mut i8_2: i8 = 43i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_2, minutes: i8_1, seconds: i8_0};
    let mut i8_3: i8 = 2i8;
    let mut i8_4: i8 = 33i8;
    let mut i8_5: i8 = -4i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut utcoffset_1_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_1;
    let mut bool_0: bool = true;
    let mut i64_0: i64 = -14i64;
    let mut i64_1: i64 = -5i64;
    let mut i64_2: i64 = -28i64;
    let mut str_0: &str = "PD9SvCuwyfIHRb";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut utcoffset_2: crate::utc_offset::UtcOffset = std::clone::Clone::clone(utcoffset_1_ref_0);
    let mut utcoffset_3: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2094() {
    rusty_monitor::set_test_id(2094);
    let mut i32_0: i32 = 54i32;
    let mut i64_0: i64 = -48i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut u16_0: u16 = 7u16;
    let mut i32_1: i32 = 60i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut i64_1: i64 = 92i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut i64_2: i64 = -94i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_2, duration_1);
    let mut u16_1: u16 = 47u16;
    let mut i32_2: i32 = -16i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_1);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_0);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut utcoffset_0_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_0;
    let mut i64_3: i64 = 54i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_4);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_3);
    let mut utcoffset_1_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_1;
    let mut month_0: month::Month = crate::month::Month::May;
    let mut bool_0: bool = std::cmp::PartialEq::ne(utcoffset_1_ref_0, utcoffset_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2126() {
    rusty_monitor::set_test_id(2126);
    let mut i8_0: i8 = 5i8;
    let mut i8_1: i8 = 12i8;
    let mut i8_2: i8 = -105i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_2, minutes: i8_1, seconds: i8_0};
    let mut utcoffset_1: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_0);
    let mut i64_0: i64 = 2i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i128_0: i128 = -174i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut i32_0: i32 = -84i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date_time(offsetdatetime_0, primitivedatetime_1);
    let mut i8_3: i8 = 42i8;
    let mut i8_4: i8 = -111i8;
    let mut i8_5: i8 = 19i8;
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_5, i8_4, i8_3);
    let mut tuple_0: (u8, u8, u8, u16) = crate::offset_date_time::OffsetDateTime::to_hms_milli(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_512() {
    rusty_monitor::set_test_id(512);
    let mut u32_0: u32 = 95u32;
    let mut u8_0: u8 = 49u8;
    let mut u8_1: u8 = 9u8;
    let mut u8_2: u8 = 96u8;
    let mut i32_0: i32 = -52i32;
    let mut i64_0: i64 = -101i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut i64_1: i64 = 56i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut i32_1: i32 = 151i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_2);
    let mut i8_0: i8 = 13i8;
    let mut i8_1: i8 = -3i8;
    let mut i8_2: i8 = -4i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_0);
    let mut utcoffset_1_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_1;
    let mut i8_3: i8 = 19i8;
    let mut i8_4: i8 = -36i8;
    let mut i8_5: i8 = 21i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut utcoffset_3: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_2);
    let mut utcoffset_3_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_3;
    let mut bool_0: bool = std::cmp::PartialEq::eq(utcoffset_3_ref_0, utcoffset_1_ref_0);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_micro(date_1, u8_2, u8_1, u8_0, u32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4059() {
    rusty_monitor::set_test_id(4059);
    let mut u16_0: u16 = 52u16;
    let mut u8_0: u8 = 2u8;
    let mut u8_1: u8 = 49u8;
    let mut u8_2: u8 = 15u8;
    let mut u16_1: u16 = 63u16;
    let mut i32_0: i32 = 47i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_1);
    let mut i8_0: i8 = 3i8;
    let mut i8_1: i8 = -31i8;
    let mut i8_2: i8 = 16i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i128_0: i128 = -98i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut f32_0: f32 = -100.822060f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_3: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::abs(duration_3);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut f32_1: f32 = 118.774518f32;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut i64_0: i64 = -18i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut i64_1: i64 = -271i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut utcoffset_2: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_1);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_milli(date_0, u8_2, u8_1, u8_0, u16_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_826() {
    rusty_monitor::set_test_id(826);
    let mut i8_0: i8 = 26i8;
    let mut i8_1: i8 = 23i8;
    let mut i8_2: i8 = 10i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_0: i32 = -48i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut i32_1: i32 = 84i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut i128_0: i128 = -37i128;
    let mut i8_3: i8 = 10i8;
    let mut i8_4: i8 = -45i8;
    let mut i8_5: i8 = -31i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_5, minutes: i8_4, seconds: i8_3};
    let mut utcoffset_2: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_1);
    let mut utcoffset_3: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_2);
    let mut result_0: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp_nanos(i128_0);
    let mut i8_6: i8 = crate::utc_offset::UtcOffset::seconds_past_minute(utcoffset_3);
    let mut u32_0: u32 = crate::time::Time::microsecond(time_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1436() {
    rusty_monitor::set_test_id(1436);
    let mut i64_0: i64 = -87i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut u16_0: u16 = 61u16;
    let mut i32_0: i32 = -51i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut i64_1: i64 = -113i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut i32_1: i32 = -29i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_1);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_0);
    let mut utcoffset_1_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_1;
    let mut u8_0: u8 = 77u8;
    let mut month_0: month::Month = crate::month::Month::September;
    let mut i32_2: i32 = 72i32;
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_calendar_date(i32_2, month_0, u8_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_413() {
    rusty_monitor::set_test_id(413);
    let mut f64_0: f64 = 88.033792f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i64_0: i64 = 143i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_1, duration_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_0);
    let mut i32_0: i32 = -112i32;
    let mut i64_1: i64 = 21i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_1, duration_3);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_2);
    let mut utcoffset_0_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_0;
    let mut i32_1: i32 = 16i32;
    let mut i64_2: i64 = -120i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_1);
    let mut i32_2: i32 = -12i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_4);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_3, date_2);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_4);
    let mut utcoffset_1_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_1;
    let mut option_0: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(utcoffset_1_ref_0, utcoffset_0_ref_0);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3660() {
    rusty_monitor::set_test_id(3660);
    let mut i64_0: i64 = 114i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut i32_0: i32 = -47i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_1);
    let mut u32_0: u32 = 31u32;
    let mut u8_0: u8 = 64u8;
    let mut u8_1: u8 = 32u8;
    let mut u8_2: u8 = 62u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = 61i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_2, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_0);
    let mut utcoffset_1_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_1;
    let mut i32_2: i32 = 32i32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_2, i32_2);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_1, duration_3);
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_2);
    let mut utcoffset_2_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_2;
    let mut bool_0: bool = std::cmp::PartialEq::eq(utcoffset_2_ref_0, utcoffset_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4392() {
    rusty_monitor::set_test_id(4392);
    let mut i8_0: i8 = -19i8;
    let mut i8_1: i8 = -102i8;
    let mut i8_2: i8 = -7i8;
    let mut i64_0: i64 = 6i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut f64_0: f64 = 37.682210f64;
    let mut i64_1: i64 = -248i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut i8_3: i8 = 62i8;
    let mut i8_4: i8 = -21i8;
    let mut i8_5: i8 = -29i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_1);
    let mut bool_0: bool = crate::duration::Duration::is_zero(duration_0);
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3943() {
    rusty_monitor::set_test_id(3943);
    let mut u32_0: u32 = 74u32;
    let mut u8_0: u8 = 23u8;
    let mut u8_1: u8 = 24u8;
    let mut u8_2: u8 = 0u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_0: i64 = -74i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut i32_0: i32 = -97i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_0);
    let mut i32_1: i32 = 76i32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_1, i32_1);
    let mut i32_2: i32 = 231i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_2);
    let mut i32_3: i32 = -34i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_1, date_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_2);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date_time(offsetdatetime_0, primitivedatetime_0);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut utcoffset_0_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_0;
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_2);
    let mut utcoffset_1_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_1;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(utcoffset_1_ref_0, utcoffset_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4307() {
    rusty_monitor::set_test_id(4307);
    let mut i8_0: i8 = -8i8;
    let mut i8_1: i8 = -43i8;
    let mut i8_2: i8 = -114i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 27u32;
    let mut u8_0: u8 = 41u8;
    let mut u8_1: u8 = 38u8;
    let mut u8_2: u8 = 74u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 96u16;
    let mut i32_0: i32 = -10i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_0);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut utcoffset_2: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_1);
    let mut tuple_0: (i8, i8, i8) = crate::utc_offset::UtcOffset::as_hms(utcoffset_2);
    let mut tuple_1: (u8, u8, u8) = crate::time::Time::as_hms(time_0);
    let mut i16_0: i16 = crate::utc_offset::UtcOffset::whole_minutes(utcoffset_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2095() {
    rusty_monitor::set_test_id(2095);
    let mut i8_0: i8 = -71i8;
    let mut i8_1: i8 = -55i8;
    let mut i8_2: i8 = 53i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_0);
    let mut i8_3: i8 = -4i8;
    let mut i8_4: i8 = -94i8;
    let mut i8_5: i8 = -5i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i32_0: i32 = 27i32;
    let mut i64_0: i64 = 88i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut u32_0: u32 = 15u32;
    let mut u8_0: u8 = 55u8;
    let mut u8_1: u8 = 26u8;
    let mut u8_2: u8 = 69u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 8u16;
    let mut i32_1: i32 = -13i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::previous(weekday_0);
    let mut u8_3: u8 = crate::weekday::Weekday::number_from_monday(weekday_1);
    panic!("From RustyUnit with love");
}
}