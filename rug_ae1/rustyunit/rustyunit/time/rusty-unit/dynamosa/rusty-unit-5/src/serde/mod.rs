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

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7448() {
    rusty_monitor::set_test_id(7448);
    let mut i32_0: i32 = -100i32;
    let mut i64_0: i64 = 127i64;
    let mut i8_0: i8 = 36i8;
    let mut i8_1: i8 = -93i8;
    let mut i8_2: i8 = -73i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_1: i32 = 201i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_0, utcoffset_0);
    let mut i64_1: i64 = -187i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut u32_0: u32 = 89u32;
    let mut u8_0: u8 = 15u8;
    let mut u8_1: u8 = 42u8;
    let mut u8_2: u8 = 21u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_2: i32 = 211i32;
    let mut i64_2: i64 = -122i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_2);
    let mut i32_3: i32 = 150i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_2, time_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_1, duration_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_2);
    let mut i8_3: i8 = 60i8;
    let mut i8_4: i8 = 24i8;
    let mut i8_5: i8 = -53i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = -10i8;
    let mut i8_7: i8 = -18i8;
    let mut i8_8: i8 = -112i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut u16_0: u16 = 89u16;
    let mut i32_4: i32 = -22i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_0);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_3, utcoffset_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_4, offset: utcoffset_1};
    let mut i64_3: i64 = -155i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut f32_0: f32 = -86.331360f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_5: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut u16_1: u16 = 10u16;
    let mut i32_5: i32 = -42i32;
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_5, u16_1);
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut f32_1: f32 = crate::duration::Duration::as_seconds_f32(duration_2);
    let mut tuple_0: (i32, u16) = crate::offset_date_time::OffsetDateTime::to_ordinal_date(offsetdatetime_2);
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut u16_2: u16 = crate::offset_date_time::OffsetDateTime::ordinal(offsetdatetime_0);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_0);
    panic!("From RustyUnit with love");
}
}