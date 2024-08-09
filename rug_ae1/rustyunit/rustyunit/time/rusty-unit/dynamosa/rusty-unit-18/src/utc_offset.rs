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
	use std::clone::Clone;
	use std::ops::Neg;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7451() {
    rusty_monitor::set_test_id(7451);
    let mut month_0: month::Month = crate::month::Month::June;
    let mut i64_0: i64 = -132i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut u16_0: u16 = 82u16;
    let mut i32_0: i32 = 126i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut u32_0: u32 = 12u32;
    let mut u8_0: u8 = 66u8;
    let mut u8_1: u8 = 98u8;
    let mut u8_2: u8 = 8u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_1);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i32_1: i32 = 122i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut f32_0: f32 = 62.961492f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i8_0: i8 = 6i8;
    let mut i8_1: i8 = 75i8;
    let mut i8_2: i8 = -16i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_2, minutes: i8_1, seconds: i8_0};
    let mut i64_1: i64 = -60i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut u32_1: u32 = 36u32;
    let mut u8_3: u8 = 23u8;
    let mut u8_4: u8 = 48u8;
    let mut u8_5: u8 = 46u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_2: i32 = -2i32;
    let mut i64_2: i64 = 123i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_2);
    let mut duration_6: std::time::Duration = crate::duration::Duration::abs_std(duration_5);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut bool_0: bool = crate::utc_offset::UtcOffset::is_positive(utcoffset_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1495() {
    rusty_monitor::set_test_id(1495);
    let mut i8_0: i8 = -75i8;
    let mut i8_1: i8 = -105i8;
    let mut i8_2: i8 = 17i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_2, minutes: i8_1, seconds: i8_0};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_0_ref_0: &crate::instant::Instant = &mut instant_0;
    let mut i8_3: i8 = 115i8;
    let mut i8_4: i8 = -37i8;
    let mut i8_5: i8 = 57i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = 119i8;
    let mut i8_7: i8 = 11i8;
    let mut i8_8: i8 = 122i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut u32_0: u32 = 60u32;
    let mut u8_0: u8 = 20u8;
    let mut u8_1: u8 = 35u8;
    let mut u8_2: u8 = 92u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_0: i64 = 0i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut u16_0: u16 = 82u16;
    let mut i32_0: i32 = 21i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_0, offset: utcoffset_2};
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_2, utcoffset_1);
    let mut i64_1: i64 = -6i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_4, duration_1);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_5);
    let mut month_0: month::Month = crate::date::Date::month(date_2);
    let mut i8_9: i8 = -77i8;
    let mut i8_10: i8 = -15i8;
    let mut i8_11: i8 = 91i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut utcoffset_4: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_3);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut i32_1: i32 = crate::offset_date_time::OffsetDateTime::to_julian_day(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7065() {
    rusty_monitor::set_test_id(7065);
    let mut i8_0: i8 = -22i8;
    let mut i8_1: i8 = -30i8;
    let mut i8_2: i8 = 15i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut i8_3: i8 = 7i8;
    let mut i8_4: i8 = -114i8;
    let mut i8_5: i8 = -2i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = 19i8;
    let mut i8_7: i8 = 40i8;
    let mut i8_8: i8 = -104i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_8, minutes: i8_7, seconds: i8_6};
    let mut u32_0: u32 = 63u32;
    let mut u8_0: u8 = 66u8;
    let mut u8_1: u8 = 57u8;
    let mut u8_2: u8 = 18u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 47u16;
    let mut i32_0: i32 = 37i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_2);
    let mut i64_0: i64 = 24i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut i64_1: i64 = 141i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8388() {
    rusty_monitor::set_test_id(8388);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut i64_0: i64 = -69i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut u32_0: u32 = 60u32;
    let mut u8_0: u8 = 39u8;
    let mut u8_1: u8 = 54u8;
    let mut u8_2: u8 = 10u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut i8_0: i8 = -100i8;
    let mut i8_1: i8 = -49i8;
    let mut i8_2: i8 = 59i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_2, minutes: i8_1, seconds: i8_0};
    let mut utcoffset_1: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_0);
    let mut i64_1: i64 = 30i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::abs(duration_1);
    let mut i8_3: i8 = 89i8;
    let mut i8_4: i8 = -15i8;
    let mut i8_5: i8 = 29i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_5, minutes: i8_4, seconds: i8_3};
    let mut i64_2: i64 = 45i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut i8_6: i8 = 62i8;
    let mut i8_7: i8 = 55i8;
    let mut i8_8: i8 = 12i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut utcoffset_4: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_3);
    let mut i8_9: i8 = 71i8;
    let mut i8_10: i8 = 29i8;
    let mut i8_11: i8 = 53i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut utcoffset_6: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_5);
    let mut u32_1: u32 = 89u32;
    let mut u8_3: u8 = 86u8;
    let mut u8_4: u8 = 29u8;
    let mut u8_5: u8 = 47u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_3: i64 = -7i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut i64_4: i64 = 3i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::minutes(i64_4);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_5, duration_4);
    let mut i8_12: i8 = -76i8;
    let mut i8_13: i8 = -24i8;
    let mut i8_14: i8 = -85i8;
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut f64_0: f64 = 102.715549f64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::abs(duration_7);
    let mut i8_15: i8 = -37i8;
    let mut i8_16: i8 = -3i8;
    let mut i8_17: i8 = -41i8;
    let mut utcoffset_8: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut i8_18: i8 = 67i8;
    let mut i8_19: i8 = 32i8;
    let mut i8_20: i8 = -12i8;
    let mut utcoffset_9: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_20, minutes: i8_19, seconds: i8_18};
    let mut utcoffset_10: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_9);
    let mut i8_21: i8 = -6i8;
    let mut i8_22: i8 = -25i8;
    let mut i8_23: i8 = -97i8;
    let mut utcoffset_11: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_23, minutes: i8_22, seconds: i8_21};
    let mut utcoffset_12: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_11);
    let mut i64_5: i64 = -17i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::minutes(i64_5);
    let mut i64_6: i64 = 34i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_6);
    let mut u32_2: u32 = 30u32;
    let mut u8_6: u8 = 11u8;
    let mut u8_7: u8 = 49u8;
    let mut u8_8: u8 = 29u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i8_24: i8 = 14i8;
    let mut i8_25: i8 = -17i8;
    let mut i8_26: i8 = -53i8;
    let mut utcoffset_13: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_26, minutes: i8_25, seconds: i8_24};
    let mut utcoffset_14: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_13);
    let mut i8_27: i8 = 0i8;
    let mut i8_28: i8 = 36i8;
    let mut i8_29: i8 = 40i8;
    let mut utcoffset_15: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_29, minutes: i8_28, seconds: i8_27};
    let mut i64_7: i64 = -24i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_7);
    let mut u32_3: u32 = 58u32;
    let mut u8_9: u8 = 68u8;
    let mut u8_10: u8 = 4u8;
    let mut u8_11: u8 = 80u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_3);
    let mut i8_30: i8 = -66i8;
    let mut i8_31: i8 = -32i8;
    let mut i8_32: i8 = -51i8;
    let mut utcoffset_16: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_32, i8_31, i8_30);
    let mut utcoffset_17: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_16);
    let mut i8_33: i8 = 34i8;
    let mut i8_34: i8 = -64i8;
    let mut i8_35: i8 = -46i8;
    let mut utcoffset_18: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_35, i8_34, i8_33);
    let mut i8_36: i8 = 6i8;
    let mut i8_37: i8 = -101i8;
    let mut i8_38: i8 = -42i8;
    let mut utcoffset_19: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_38, minutes: i8_37, seconds: i8_36};
    let mut i64_8: i64 = -263i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::seconds(i64_8);
    let mut i64_9: i64 = 137i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::days(i64_9);
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::abs(duration_13);
    let mut u32_4: u32 = 42u32;
    let mut u8_12: u8 = 64u8;
    let mut u8_13: u8 = 34u8;
    let mut u8_14: u8 = 51u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_14, u8_13, u8_12, u32_4);
    let mut i128_0: i128 = 34i128;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_15);
    let mut utcoffset_20: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_3);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_5: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_4);
    let mut i32_0: i32 = 264i32;
    let mut i64_10: i64 = -15i64;
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_10, i32_0);
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_16, duration_12);
    let mut i8_39: i8 = 52i8;
    let mut i8_40: i8 = -112i8;
    let mut i8_41: i8 = 94i8;
    let mut utcoffset_21: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_41, i8_40, i8_39);
    let mut utcoffset_22: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_21);
    let mut i64_11: i64 = -67i64;
    let mut duration_18: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_11);
    let mut i64_12: i64 = 23i64;
    let mut duration_19: crate::duration::Duration = crate::duration::Duration::minutes(i64_12);
    let mut duration_20: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_19, duration_18);
    let mut i8_42: i8 = 115i8;
    let mut i8_43: i8 = -37i8;
    let mut i8_44: i8 = 57i8;
    let mut utcoffset_23: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_44, i8_43, i8_42);
    let mut i8_45: i8 = 119i8;
    let mut i8_46: i8 = 11i8;
    let mut i8_47: i8 = 122i8;
    let mut utcoffset_24: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_47, i8_46, i8_45);
    let mut u32_5: u32 = 60u32;
    let mut u8_15: u8 = 20u8;
    let mut u8_16: u8 = 35u8;
    let mut u8_17: u8 = 92u8;
    let mut time_6: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_17, u8_16, u8_15, u32_5);
    let mut i64_13: i64 = 0i64;
    let mut duration_21: crate::duration::Duration = crate::duration::Duration::minutes(i64_13);
    let mut u16_0: u16 = 82u16;
    let mut i32_1: i32 = 21i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_21);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_3, time_6);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_0, offset: utcoffset_24};
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_5, utcoffset_23);
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut month_0: month::Month = crate::date::Date::month(date_1);
    let mut i8_48: i8 = -77i8;
    let mut i8_49: i8 = -15i8;
    let mut i8_50: i8 = 91i8;
    let mut utcoffset_25: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_50, i8_49, i8_48);
    let mut utcoffset_26: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_25);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_22: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_23: crate::duration::Duration = crate::duration::Duration::abs(duration_22);
    let mut duration_24: std::time::Duration = crate::duration::Duration::abs_std(duration_23);
    let mut f32_0: f32 = -101.944517f32;
    let mut duration_25: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i8_51: i8 = -128i8;
    let mut i8_52: i8 = -85i8;
    let mut i8_53: i8 = -96i8;
    let mut utcoffset_27: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_53, i8_52, i8_51);
    let mut i64_14: i64 = 6i64;
    let mut duration_26: crate::duration::Duration = crate::duration::Duration::minutes(i64_14);
    let mut duration_27: std::time::Duration = crate::duration::Duration::abs_std(duration_26);
    let mut i8_54: i8 = -34i8;
    let mut i8_55: i8 = -34i8;
    let mut i8_56: i8 = 62i8;
    let mut utcoffset_28: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_56, i8_55, i8_54);
    let mut i64_15: i64 = -245i64;
    let mut duration_28: crate::duration::Duration = crate::duration::Duration::weeks(i64_15);
    let mut u32_6: u32 = 75u32;
    let mut u8_18: u8 = 96u8;
    let mut u8_19: u8 = 7u8;
    let mut u8_20: u8 = 43u8;
    let mut time_7: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_20, u8_19, u8_18, u32_6);
    let mut i64_16: i64 = -156i64;
    let mut duration_29: crate::duration::Duration = crate::duration::Duration::minutes(i64_16);
    let mut i64_17: i64 = -95i64;
    let mut duration_30: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_17);
    let mut duration_31: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_30, duration_29);
    let mut i64_18: i64 = -31i64;
    let mut duration_32: crate::duration::Duration = crate::duration::Duration::days(i64_18);
    let mut offsetdatetime_8: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_9: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_6, duration_8);
    let mut time_8: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_9);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_33: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut i128_1: i128 = 11i128;
    let mut duration_34: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut i128_2: i128 = 52i128;
    let mut duration_35: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_2);
    let mut duration_36: std::time::Duration = crate::duration::Duration::abs_std(duration_35);
    let mut i8_57: i8 = 27i8;
    let mut i8_58: i8 = 68i8;
    let mut i8_59: i8 = -45i8;
    let mut utcoffset_29: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_59, i8_58, i8_57);
    let mut f64_1: f64 = -95.523134f64;
    let mut duration_37: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut i64_19: i64 = 85i64;
    let mut duration_38: crate::duration::Duration = crate::duration::Duration::microseconds(i64_19);
    let mut i64_20: i64 = -29i64;
    let mut duration_39: crate::duration::Duration = crate::duration::Duration::microseconds(i64_20);
    let mut duration_40: std::time::Duration = crate::duration::Duration::abs_std(duration_39);
    let mut i64_21: i64 = 182i64;
    let mut i8_60: i8 = -82i8;
    let mut i8_61: i8 = -67i8;
    let mut i8_62: i8 = -22i8;
    let mut utcoffset_30: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_62, i8_61, i8_60);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut i8_63: i8 = 4i8;
    let mut i8_64: i8 = -27i8;
    let mut i8_65: i8 = 44i8;
    let mut utcoffset_31: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_65, i8_64, i8_63);
    let mut f64_2: f64 = 8.033375f64;
    let mut duration_41: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_2);
    let mut i32_2: i32 = -16i32;
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_5: crate::date::Date = crate::date::Date::saturating_sub(date_4, duration_41);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_5);
    let mut f32_1: f32 = 156.677118f32;
    let mut i32_3: i32 = -22i32;
    let mut duration_42: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_21, i32_3);
    let mut duration_43: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut f64_3: f64 = -156.725604f64;
    let mut duration_44: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_3);
    let mut duration_45: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_44, duration_43);
    let mut i32_4: i32 = 30i32;
    let mut date_6: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut date_7: crate::date::Date = crate::date::Date::saturating_sub(date_6, duration_45);
    let mut i32_5: i32 = -4i32;
    let mut i64_22: i64 = -135i64;
    let mut duration_46: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_22, i32_5);
    let mut u16_1: u16 = 93u16;
    let mut i32_6: i32 = 102i32;
    let mut date_8: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_6, u16_1);
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    let mut offsetdatetime_10: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_8, utcoffset_30);
    let mut u8_21: u8 = crate::date::Date::iso_week(date_8);
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3275() {
    rusty_monitor::set_test_id(3275);
    let mut i64_0: i64 = 2i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut i64_1: i64 = 169i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut u32_0: u32 = 8u32;
    let mut u8_0: u8 = 42u8;
    let mut u8_1: u8 = 1u8;
    let mut u8_2: u8 = 30u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 25u16;
    let mut i32_0: i32 = 77i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_0);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_0);
    let mut utcoffset_1_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_1;
    let mut i128_0: i128 = -13i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i64_2: i64 = -132i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_3, duration_2);
    let mut u16_1: u16 = 82u16;
    let mut i32_1: i32 = 126i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_1);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_4);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut time_1: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_2);
    let mut u32_1: u32 = 12u32;
    let mut u8_3: u8 = 66u8;
    let mut u8_4: u8 = 98u8;
    let mut u8_5: u8 = 8u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_5: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_5);
    let mut time_3: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut i32_2: i32 = 122i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_3, time_3);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_3, time_2);
    let mut f32_0: f32 = 62.961492f32;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i8_0: i8 = 6i8;
    let mut i8_1: i8 = 75i8;
    let mut i8_2: i8 = -16i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_2, minutes: i8_1, seconds: i8_0};
    let mut i64_3: i64 = -60i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut duration_8: std::time::Duration = crate::duration::Duration::abs_std(duration_7);
    let mut u32_2: u32 = 36u32;
    let mut u8_6: u8 = 23u8;
    let mut u8_7: u8 = 48u8;
    let mut u8_8: u8 = 46u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i32_3: i32 = -2i32;
    let mut i64_4: i64 = 123i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_3);
    let mut duration_10: std::time::Duration = crate::duration::Duration::abs_std(duration_9);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut i32_4: i32 = 184i32;
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_4};
    let mut i8_3: i8 = -81i8;
    let mut i8_4: i8 = 73i8;
    let mut i8_5: i8 = -83i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_4, utcoffset_3);
    let mut time_5: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_5);
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u16_2: u16 = 67u16;
    let mut i32_5: i32 = -2i32;
    let mut date_5: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_5, u16_2);
    let mut utcoffset_4: crate::utc_offset::UtcOffset = std::clone::Clone::clone(utcoffset_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1593() {
    rusty_monitor::set_test_id(1593);
    let mut i8_0: i8 = -115i8;
    let mut i8_1: i8 = 43i8;
    let mut i8_2: i8 = -11i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut utcoffset_0_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_0;
    let mut i32_0: i32 = 75i32;
    let mut i32_1: i32 = -25i32;
    let mut i64_0: i64 = 41i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_1);
    let mut u16_0: u16 = 2u16;
    let mut i32_2: i32 = -71i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 6u32;
    let mut u8_0: u8 = 13u8;
    let mut u8_1: u8 = 26u8;
    let mut u8_2: u8 = 49u8;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut u16_1: u16 = 72u16;
    let mut i32_3: i32 = 57i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_1);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_1);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_1: u32 = 73u32;
    let mut u8_3: u8 = 11u8;
    let mut u8_4: u8 = 33u8;
    let mut u8_5: u8 = 33u8;
    let mut i8_3: i8 = -70i8;
    let mut i8_4: i8 = -17i8;
    let mut i8_5: i8 = 3i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i32_4: i32 = -54i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut i32_5: i32 = -118i32;
    let mut i64_1: i64 = 147i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_5);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut f32_0: f32 = -76.468316f32;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i64_2: i64 = -54i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_5, duration_4);
    let mut i32_6: i32 = 2i32;
    let mut date_5: crate::date::Date = crate::date::Date {value: i32_6};
    let mut u16_2: u16 = crate::util::days_in_year(i32_0);
    let mut utcoffset_1_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_1;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(utcoffset_1_ref_0, utcoffset_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7179() {
    rusty_monitor::set_test_id(7179);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut i64_0: i64 = -69i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut u32_0: u32 = 60u32;
    let mut u8_0: u8 = 39u8;
    let mut u8_1: u8 = 54u8;
    let mut u8_2: u8 = 10u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_1, time_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_2);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut i8_0: i8 = -100i8;
    let mut i8_1: i8 = -49i8;
    let mut i8_2: i8 = 59i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_2, minutes: i8_1, seconds: i8_0};
    let mut utcoffset_1: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_0);
    let mut i64_1: i64 = 30i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::abs(duration_1);
    let mut i8_3: i8 = 89i8;
    let mut i8_4: i8 = -15i8;
    let mut i8_5: i8 = 29i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_5, minutes: i8_4, seconds: i8_3};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7291() {
    rusty_monitor::set_test_id(7291);
    let mut month_0: month::Month = crate::month::Month::September;
    let mut u8_0: u8 = 89u8;
    let mut month_1: month::Month = crate::month::Month::June;
    let mut i32_0: i32 = -143i32;
    let mut i128_0: i128 = -13i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i64_0: i64 = -132i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_1, duration_0);
    let mut u16_0: u16 = 82u16;
    let mut i32_1: i32 = 126i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_0);
    let mut u32_0: u32 = 12u32;
    let mut u8_1: u8 = 66u8;
    let mut u8_2: u8 = 98u8;
    let mut u8_3: u8 = 8u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_3, u8_2, u8_1, u32_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_3: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_3);
    let mut time_2: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i32_2: i32 = 122i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_1, time_1);
    let mut f32_0: f32 = 62.961492f32;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i8_0: i8 = 6i8;
    let mut i8_1: i8 = 75i8;
    let mut i8_2: i8 = -16i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_2, minutes: i8_1, seconds: i8_0};
    let mut i64_1: i64 = -60i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut duration_6: std::time::Duration = crate::duration::Duration::abs_std(duration_5);
    let mut u32_1: u32 = 36u32;
    let mut u8_4: u8 = 23u8;
    let mut u8_5: u8 = 48u8;
    let mut u8_6: u8 = 46u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_6, u8_5, u8_4, u32_1);
    let mut i32_3: i32 = -2i32;
    let mut i64_2: i64 = 123i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_3);
    let mut duration_8: std::time::Duration = crate::duration::Duration::abs_std(duration_7);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut i32_4: i32 = 184i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_4};
    let mut i8_3: i8 = -81i8;
    let mut i8_4: i8 = 73i8;
    let mut i8_5: i8 = -83i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_2, utcoffset_1);
    let mut time_4: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u16_1: u16 = 67u16;
    let mut i32_5: i32 = -2i32;
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_5, u16_1);
    let mut u32_2: u32 = 96u32;
    let mut u8_7: u8 = 37u8;
    let mut u8_8: u8 = 10u8;
    let mut u8_9: u8 = 5u8;
    let mut time_5: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_9, u8_8, u8_7, u32_2);
    let mut i32_6: i32 = -139i32;
    let mut date_5: crate::date::Date = crate::date::Date {value: i32_6};
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_5, time_5);
    let mut month_2: month::Month = crate::month::Month::December;
    let mut u32_3: u32 = crate::primitive_date_time::PrimitiveDateTime::nanosecond(primitivedatetime_3);
    let mut month_3: month::Month = crate::month::Month::previous(month_0);
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::Next;
    let mut month_4: month::Month = crate::month::Month::February;
    let mut month_3_ref_0: &month::Month = &mut month_3;
    let mut bool_0: bool = crate::utc_offset::UtcOffset::is_negative(utcoffset_0);
    let mut i128_1: i128 = crate::duration::Duration::whole_nanoseconds(duration_4);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_2, time_0);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_calendar_date(i32_0, month_1, u8_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6239() {
    rusty_monitor::set_test_id(6239);
    let mut i64_0: i64 = -5i64;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut i64_1: i64 = 69i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut u32_0: u32 = 60u32;
    let mut u8_0: u8 = 39u8;
    let mut u8_1: u8 = 54u8;
    let mut u8_2: u8 = 10u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut i8_0: i8 = -100i8;
    let mut i8_1: i8 = -49i8;
    let mut i8_2: i8 = 59i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_2, minutes: i8_1, seconds: i8_0};
    let mut utcoffset_1: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_0);
    let mut i64_2: i64 = 30i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::abs(duration_1);
    let mut i8_3: i8 = 89i8;
    let mut i8_4: i8 = -15i8;
    let mut i8_5: i8 = 29i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_5, minutes: i8_4, seconds: i8_3};
    let mut u32_1: u32 = 18u32;
    let mut u8_3: u8 = 58u8;
    let mut u8_4: u8 = 97u8;
    let mut u8_5: u8 = 47u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_3: i64 = 45i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut i8_6: i8 = 62i8;
    let mut i8_7: i8 = 55i8;
    let mut i8_8: i8 = 12i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut utcoffset_4: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_3);
    let mut i8_9: i8 = 71i8;
    let mut i8_10: i8 = 29i8;
    let mut i8_11: i8 = 53i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut utcoffset_6: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_5);
    let mut u32_2: u32 = 89u32;
    let mut u8_6: u8 = 86u8;
    let mut u8_7: u8 = 29u8;
    let mut u8_8: u8 = 47u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i64_4: i64 = -7i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_5, duration_4);
    let mut i8_12: i8 = -76i8;
    let mut i8_13: i8 = -24i8;
    let mut i8_14: i8 = -85i8;
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut f64_0: f64 = 102.715549f64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::abs(duration_7);
    let mut i8_15: i8 = -37i8;
    let mut i8_16: i8 = -3i8;
    let mut i8_17: i8 = -41i8;
    let mut utcoffset_8: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut i8_18: i8 = 67i8;
    let mut i8_19: i8 = 32i8;
    let mut i8_20: i8 = -12i8;
    let mut utcoffset_9: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_20, minutes: i8_19, seconds: i8_18};
    let mut utcoffset_10: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_9);
    let mut i8_21: i8 = -6i8;
    let mut i8_22: i8 = -25i8;
    let mut i8_23: i8 = -97i8;
    let mut utcoffset_11: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_23, minutes: i8_22, seconds: i8_21};
    let mut utcoffset_12: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_11);
    let mut i64_5: i64 = -17i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::minutes(i64_5);
    let mut i64_6: i64 = 34i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_6);
    let mut u32_3: u32 = 30u32;
    let mut u8_9: u8 = 11u8;
    let mut u8_10: u8 = 49u8;
    let mut u8_11: u8 = 29u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_3);
    let mut i8_24: i8 = 14i8;
    let mut i8_25: i8 = -17i8;
    let mut i8_26: i8 = -53i8;
    let mut utcoffset_13: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_26, minutes: i8_25, seconds: i8_24};
    let mut utcoffset_14: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_13);
    let mut i8_27: i8 = 0i8;
    let mut i8_28: i8 = 36i8;
    let mut i8_29: i8 = 40i8;
    let mut utcoffset_15: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_29, minutes: i8_28, seconds: i8_27};
    let mut f64_1: f64 = -157.543180f64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut i64_7: i64 = -24i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_7);
    let mut u32_4: u32 = 58u32;
    let mut u8_12: u8 = 68u8;
    let mut u8_13: u8 = 4u8;
    let mut u8_14: u8 = 80u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_14, u8_13, u8_12, u32_4);
    let mut i8_30: i8 = -66i8;
    let mut i8_31: i8 = -32i8;
    let mut i8_32: i8 = -51i8;
    let mut utcoffset_16: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_32, i8_31, i8_30);
    let mut utcoffset_17: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_16);
    let mut i8_33: i8 = 34i8;
    let mut i8_34: i8 = -64i8;
    let mut i8_35: i8 = -46i8;
    let mut utcoffset_18: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_35, i8_34, i8_33);
    let mut i8_36: i8 = 6i8;
    let mut i8_37: i8 = -101i8;
    let mut i8_38: i8 = -42i8;
    let mut utcoffset_19: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_38, minutes: i8_37, seconds: i8_36};
    let mut i64_8: i64 = -263i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::seconds(i64_8);
    let mut i64_9: i64 = 137i64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::days(i64_9);
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::abs(duration_14);
    let mut u32_5: u32 = 42u32;
    let mut u8_15: u8 = 64u8;
    let mut u8_16: u8 = 34u8;
    let mut u8_17: u8 = 51u8;
    let mut time_5: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_17, u8_16, u8_15, u32_5);
    let mut i8_39: i8 = -2i8;
    let mut i8_40: i8 = 49i8;
    let mut i8_41: i8 = 63i8;
    let mut utcoffset_20: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset {hours: i8_41, minutes: i8_40, seconds: i8_39};
    let mut i128_0: i128 = 34i128;
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_16);
    let mut utcoffset_21: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_3);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_6: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_4);
    let mut i64_10: i64 = 118i64;
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::days(i64_10);
    let mut i32_0: i32 = 264i32;
    let mut i64_11: i64 = -15i64;
    let mut duration_18: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_11, i32_0);
    let mut duration_19: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_18, duration_17);
    let mut i8_42: i8 = 52i8;
    let mut i8_43: i8 = -112i8;
    let mut i8_44: i8 = 94i8;
    let mut utcoffset_22: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_44, i8_43, i8_42);
    let mut utcoffset_23: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_22);
    let mut i64_12: i64 = -67i64;
    let mut duration_20: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_12);
    let mut i64_13: i64 = 23i64;
    let mut duration_21: crate::duration::Duration = crate::duration::Duration::minutes(i64_13);
    let mut duration_22: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_21, duration_20);
    let mut i8_45: i8 = 115i8;
    let mut i8_46: i8 = -37i8;
    let mut i8_47: i8 = 57i8;
    let mut utcoffset_24: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_47, i8_46, i8_45);
    let mut i8_48: i8 = 119i8;
    let mut i8_49: i8 = 11i8;
    let mut i8_50: i8 = 122i8;
    let mut utcoffset_25: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_50, i8_49, i8_48);
    let mut u32_6: u32 = 60u32;
    let mut u8_18: u8 = 20u8;
    let mut u8_19: u8 = 35u8;
    let mut u8_20: u8 = 92u8;
    let mut time_7: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_20, u8_19, u8_18, u32_6);
    let mut i64_14: i64 = 0i64;
    let mut duration_23: crate::duration::Duration = crate::duration::Duration::minutes(i64_14);
    let mut u16_0: u16 = 82u16;
    let mut i32_1: i32 = 21i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_23);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_3, time_7);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_0, offset: utcoffset_25};
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_5, utcoffset_24);
    let mut i64_15: i64 = -6i64;
    let mut duration_24: crate::duration::Duration = crate::duration::Duration::microseconds(i64_15);
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_8: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_7, duration_24);
    let mut date_4: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_8);
    let mut month_0: month::Month = crate::date::Date::month(date_1);
    let mut i8_51: i8 = -77i8;
    let mut i8_52: i8 = -15i8;
    let mut i8_53: i8 = 91i8;
    let mut utcoffset_26: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_53, i8_52, i8_51);
    let mut utcoffset_27: crate::utc_offset::UtcOffset = std::ops::Neg::neg(utcoffset_26);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_25: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_26: crate::duration::Duration = crate::duration::Duration::abs(duration_25);
    let mut duration_27: std::time::Duration = crate::duration::Duration::abs_std(duration_26);
    let mut f32_0: f32 = -101.944517f32;
    let mut duration_28: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i8_54: i8 = -128i8;
    let mut i8_55: i8 = -85i8;
    let mut i8_56: i8 = -96i8;
    let mut utcoffset_28: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_56, i8_55, i8_54);
    let mut i64_16: i64 = 6i64;
    let mut duration_29: crate::duration::Duration = crate::duration::Duration::minutes(i64_16);
    let mut duration_30: std::time::Duration = crate::duration::Duration::abs_std(duration_29);
    let mut i8_57: i8 = -34i8;
    let mut i8_58: i8 = -34i8;
    let mut i8_59: i8 = 62i8;
    let mut utcoffset_29: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_59, i8_58, i8_57);
    let mut i64_17: i64 = -245i64;
    let mut duration_31: crate::duration::Duration = crate::duration::Duration::weeks(i64_17);
    let mut u32_7: u32 = 75u32;
    let mut u8_21: u8 = 96u8;
    let mut u8_22: u8 = 7u8;
    let mut u8_23: u8 = 43u8;
    let mut time_8: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_23, u8_22, u8_21, u32_7);
    let mut i64_18: i64 = -156i64;
    let mut duration_32: crate::duration::Duration = crate::duration::Duration::minutes(i64_18);
    let mut i64_19: i64 = -95i64;
    let mut duration_33: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_19);
    let mut duration_34: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_33, duration_32);
    let mut i64_20: i64 = -31i64;
    let mut duration_35: crate::duration::Duration = crate::duration::Duration::days(i64_20);
    let mut offsetdatetime_9: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_10: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_6, duration_8);
    let mut time_9: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_10);
    let mut i8_60: i8 = -26i8;
    let mut i8_61: i8 = -111i8;
    let mut i8_62: i8 = 63i8;
    let mut utcoffset_30: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_62, i8_61, i8_60);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_36: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut i128_1: i128 = 11i128;
    let mut duration_37: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut i128_2: i128 = 52i128;
    let mut duration_38: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_2);
    let mut duration_39: std::time::Duration = crate::duration::Duration::abs_std(duration_38);
    let mut i8_63: i8 = 27i8;
    let mut i8_64: i8 = 68i8;
    let mut i8_65: i8 = -45i8;
    let mut utcoffset_31: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_65, i8_64, i8_63);
    let mut f64_2: f64 = -95.523134f64;
    let mut duration_40: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_2);
    let mut i64_21: i64 = 85i64;
    let mut duration_41: crate::duration::Duration = crate::duration::Duration::microseconds(i64_21);
    let mut i64_22: i64 = -29i64;
    let mut duration_42: crate::duration::Duration = crate::duration::Duration::microseconds(i64_22);
    let mut duration_43: std::time::Duration = crate::duration::Duration::abs_std(duration_42);
    let mut i64_23: i64 = 182i64;
    let mut i8_66: i8 = -82i8;
    let mut i8_67: i8 = -67i8;
    let mut i8_68: i8 = -22i8;
    let mut utcoffset_32: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_68, i8_67, i8_66);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut i8_69: i8 = 4i8;
    let mut i8_70: i8 = -27i8;
    let mut i8_71: i8 = 44i8;
    let mut utcoffset_33: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_71, i8_70, i8_69);
    let mut f64_3: f64 = 8.033375f64;
    let mut duration_44: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_3);
    let mut i32_2: i32 = -16i32;
    let mut date_5: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_6: crate::date::Date = crate::date::Date::saturating_sub(date_5, duration_44);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_6);
    let mut f32_1: f32 = 156.677118f32;
    let mut i32_3: i32 = -22i32;
    let mut duration_45: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_23, i32_3);
    let mut duration_46: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut f64_4: f64 = -156.725604f64;
    let mut duration_47: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_4);
    let mut duration_48: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_47, duration_46);
    let mut i32_4: i32 = 30i32;
    let mut date_7: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut date_8: crate::date::Date = crate::date::Date::saturating_sub(date_7, duration_48);
    let mut i32_5: i32 = -4i32;
    let mut i64_24: i64 = -135i64;
    let mut duration_49: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_24, i32_5);
    let mut u16_1: u16 = 93u16;
    let mut i32_6: i32 = 102i32;
    let mut date_9: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_6, u16_1);
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    let mut offsetdatetime_11: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_9, utcoffset_32);
    let mut u8_24: u8 = crate::date::Date::iso_week(date_9);
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    panic!("From RustyUnit with love");
}
}