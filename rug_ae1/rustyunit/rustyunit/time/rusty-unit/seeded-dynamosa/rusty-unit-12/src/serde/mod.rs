//! Differential formats for serde.
// This also includes the serde implementations for all types. This doesn't need to be externally
// documented, though.

// Types with guaranteed stable serde representations. Strings are avoided to allow for optimal
// representations in various binary forms.

/// Consume the next item in a sequence.
macro_rules! item {
    ($seq:expr, $name:literal) => {
        $seq.next_element()?
            .ok_or_else(|| <A::Error as serde::de::Error>::custom(concat!("expected ", $name)))
    };
}

#[cfg(feature = "serde-well-known")]
pub mod rfc2822;
#[cfg(feature = "serde-well-known")]
pub mod rfc3339;
pub mod timestamp;
mod visitor;

use core::marker::PhantomData;

#[cfg(feature = "serde-human-readable")]
use serde::ser::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use self::visitor::Visitor;
#[cfg(feature = "parsing")]
use crate::format_description::{modifier, Component, FormatItem};
use crate::{Date, Duration, Month, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset, Weekday};

// region: Date
/// The format used when serializing and deserializing a human-readable `Date`.
#[cfg(feature = "parsing")]
const DATE_FORMAT: &[FormatItem<'_>] = &[
    FormatItem::Component(Component::Year(modifier::Year::default())),
    FormatItem::Literal(b"-"),
    FormatItem::Component(Component::Month(modifier::Month::default())),
    FormatItem::Literal(b"-"),
    FormatItem::Component(Component::Day(modifier::Day::default())),
];

impl Serialize for Date {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[cfg(feature = "serde-human-readable")]
        if serializer.is_human_readable() {
            return serializer.serialize_str(&match self.format(&DATE_FORMAT) {
                Ok(s) => s,
                Err(_) => return Err(S::Error::custom("failed formatting `Date`")),
            });
        }

        (self.year(), self.ordinal()).serialize(serializer)
    }
}

impl<'a> Deserialize<'a> for Date {
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(Visitor::<Self>(PhantomData))
    }
}
// endregion date

// region: Duration
impl Serialize for Duration {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[cfg(feature = "serde-human-readable")]
        if serializer.is_human_readable() {
            return serializer.collect_str(&format_args!(
                "{}.{:>09}",
                self.whole_seconds(),
                self.subsec_nanoseconds().abs()
            ));
        }

        (self.whole_seconds(), self.subsec_nanoseconds()).serialize(serializer)
    }
}

impl<'a> Deserialize<'a> for Duration {
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(Visitor::<Self>(PhantomData))
    }
}
// endregion Duration

// region: OffsetDateTime
/// The format used when serializing and deserializing a human-readable `OffsetDateTime`.
#[cfg(feature = "parsing")]
const OFFSET_DATE_TIME_FORMAT: &[FormatItem<'_>] = &[
    FormatItem::Compound(DATE_FORMAT),
    FormatItem::Literal(b" "),
    FormatItem::Compound(TIME_FORMAT),
    FormatItem::Literal(b" "),
    FormatItem::Compound(UTC_OFFSET_FORMAT),
];

impl Serialize for OffsetDateTime {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[cfg(feature = "serde-human-readable")]
        if serializer.is_human_readable() {
            return serializer.serialize_str(&match self.format(&OFFSET_DATE_TIME_FORMAT) {
                Ok(s) => s,
                Err(_) => return Err(S::Error::custom("failed formatting `OffsetDateTime`")),
            });
        }

        (
            self.year(),
            self.ordinal(),
            self.hour(),
            self.minute(),
            self.second(),
            self.nanosecond(),
            self.offset.whole_hours(),
            self.offset.minutes_past_hour(),
            self.offset.seconds_past_minute(),
        )
            .serialize(serializer)
    }
}

impl<'a> Deserialize<'a> for OffsetDateTime {
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(Visitor::<Self>(PhantomData))
    }
}
// endregion OffsetDateTime

// region: PrimitiveDateTime
/// The format used when serializing and deserializing a human-readable `PrimitiveDateTime`.
#[cfg(feature = "parsing")]
const PRIMITIVE_DATE_TIME_FORMAT: &[FormatItem<'_>] = &[
    FormatItem::Compound(DATE_FORMAT),
    FormatItem::Literal(b" "),
    FormatItem::Compound(TIME_FORMAT),
];

impl Serialize for PrimitiveDateTime {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[cfg(feature = "serde-human-readable")]
        if serializer.is_human_readable() {
            return serializer.serialize_str(&match self.format(&PRIMITIVE_DATE_TIME_FORMAT) {
                Ok(s) => s,
                Err(_) => return Err(<S::Error>::custom("failed formatting `PrimitiveDateTime`")),
            });
        }

        (
            self.year(),
            self.ordinal(),
            self.hour(),
            self.minute(),
            self.second(),
            self.nanosecond(),
        )
            .serialize(serializer)
    }
}

impl<'a> Deserialize<'a> for PrimitiveDateTime {
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(Visitor::<Self>(PhantomData))
    }
}
// endregion PrimitiveDateTime

// region: Time
/// The format used when serializing and deserializing a human-readable `Time`.
#[cfg(feature = "parsing")]
const TIME_FORMAT: &[FormatItem<'_>] = &[
    FormatItem::Component(Component::Hour(<modifier::Hour>::default())),
    FormatItem::Literal(b":"),
    FormatItem::Component(Component::Minute(<modifier::Minute>::default())),
    FormatItem::Literal(b":"),
    FormatItem::Component(Component::Second(<modifier::Second>::default())),
    FormatItem::Literal(b"."),
    FormatItem::Component(Component::Subsecond(<modifier::Subsecond>::default())),
];

impl Serialize for Time {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[cfg(feature = "serde-human-readable")]
        if serializer.is_human_readable() {
            return serializer.serialize_str(&match self.format(&TIME_FORMAT) {
                Ok(s) => s,
                Err(_) => return Err(S::Error::custom("failed formatting `Time`")),
            });
        }

        (self.hour(), self.minute(), self.second(), self.nanosecond()).serialize(serializer)
    }
}

impl<'a> Deserialize<'a> for Time {
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(Visitor::<Self>(PhantomData))
    }
}
// endregion Time

// region: UtcOffset
/// The format used when serializing and deserializing a human-readable `UtcOffset`.
#[cfg(feature = "parsing")]
const UTC_OFFSET_FORMAT: &[FormatItem<'_>] = &[
    FormatItem::Component(Component::OffsetHour(modifier::OffsetHour::default())),
    FormatItem::Literal(b":"),
    FormatItem::Component(Component::OffsetMinute(modifier::OffsetMinute::default())),
    FormatItem::Literal(b":"),
    FormatItem::Component(Component::OffsetSecond(modifier::OffsetSecond::default())),
];

impl Serialize for UtcOffset {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[cfg(feature = "serde-human-readable")]
        if serializer.is_human_readable() {
            return serializer.serialize_str(&match self.format(&UTC_OFFSET_FORMAT) {
                Ok(s) => s,
                Err(_) => return Err(S::Error::custom("failed formatting `UtcOffset`")),
            });
        }

        (
            self.whole_hours(),
            self.minutes_past_hour(),
            self.seconds_past_minute(),
        )
            .serialize(serializer)
    }
}

impl<'a> Deserialize<'a> for UtcOffset {
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(Visitor::<Self>(PhantomData))
    }
}
// endregion UtcOffset

// region: Weekday
impl Serialize for Weekday {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[cfg(feature = "serde-human-readable")]
        if serializer.is_human_readable() {
            #[cfg(not(feature = "std"))]
            use alloc::string::ToString;
            return self.to_string().serialize(serializer);
        }

        self.number_from_monday().serialize(serializer)
    }
}

impl<'a> Deserialize<'a> for Weekday {
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(Visitor::<Self>(PhantomData))
    }
}
// endregion Weekday

// region: Month
impl Serialize for Month {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[cfg(feature = "serde-human-readable")]
        if serializer.is_human_readable() {
            #[cfg(not(feature = "std"))]
            use alloc::string::String;
            return self.to_string().serialize(serializer);
        }

        (*self as u8).serialize(serializer)
    }
}

impl<'a> Deserialize<'a> for Month {
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(Visitor::<Self>(PhantomData))
    }
}
// endregion Month

#[cfg(test)]
mod rusty_tests {
	use crate::*;

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7211() {
//    rusty_monitor::set_test_id(7211);
    let mut i64_0: i64 = 2440588i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut i64_1: i64 = 0i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut i32_0: i32 = 257i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut i8_0: i8 = 5i8;
    let mut i8_1: i8 = 23i8;
    let mut i8_2: i8 = 60i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_1: i32 = 122i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_0, utcoffset_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_0);
    let mut u8_0: u8 = 3u8;
    let mut u8_1: u8 = 88u8;
    let mut u8_2: u8 = 29u8;
    let mut month_0: month::Month = crate::month::Month::June;
    let mut i32_2: i32 = 10i32;
    let mut month_1: month::Month = crate::month::Month::August;
    let mut i32_3: i32 = -27i32;
    let mut month_2: month::Month = crate::month::Month::October;
    let mut i32_4: i32 = 128i32;
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i8_3: i8 = 1i8;
    let mut i8_4: i8 = 3i8;
    let mut i8_5: i8 = 24i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_2, utcoffset_1);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut u16_0: u16 = 10u16;
    let mut i32_5: i32 = 139i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_5, u16_0);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_4);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_3, time_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_1, duration_3);
    let mut month_3: month::Month = crate::month::Month::March;
    let mut i32_6: i32 = -91i32;
    let mut i128_0: i128 = 1000000000i128;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut u16_1: u16 = 60u16;
    let mut i32_7: i32 = 320i32;
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_7, u16_1);
    let mut str_0: &str = "February";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_0: bool = false;
    let mut i64_2: i64 = -161i64;
    let mut i64_3: i64 = 1000i64;
    let mut i64_4: i64 = 1000000i64;
    let mut str_1: &str = "microsecond";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_4, maximum: i64_3, value: i64_2, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut f32_1: f32 = 1315859240.000000f32;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut month_4: month::Month = crate::primitive_date_time::PrimitiveDateTime::month(primitivedatetime_2);
    let mut i32_8: i32 = 212i32;
    let mut u8_3: u8 = crate::util::days_in_year_month(i32_6, month_4);
    let mut u8_4: u8 = crate::util::days_in_year_month(i32_8, month_3);
    let mut u8_5: u8 = crate::util::days_in_year_month(i32_4, month_2);
    let mut u8_6: u8 = crate::util::days_in_year_month(i32_3, month_1);
    let mut u8_7: u8 = crate::util::days_in_year_month(i32_2, month_0);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms(u8_2, u8_1, u8_0);
    let mut option_0: std::option::Option<crate::offset_date_time::OffsetDateTime> = crate::offset_date_time::OffsetDateTime::checked_add(offsetdatetime_1, duration_2);
//    panic!("From RustyUnit with love");
}
}