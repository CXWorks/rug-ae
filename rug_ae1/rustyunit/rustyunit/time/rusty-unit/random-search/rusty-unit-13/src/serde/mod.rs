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
fn rusty_test_4853() {
    rusty_monitor::set_test_id(4853);
    let mut i128_0: i128 = -128i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i32_0: i32 = -59i32;
    let mut i64_0: i64 = 231i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_1, i32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_2);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i8_0: i8 = 51i8;
    let mut i8_1: i8 = 7i8;
    let mut i8_2: i8 = 23i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_1: i64 = 61i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut i64_2: i64 = 48i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut i32_1: i32 = 37i32;
    let mut i64_3: i64 = -139i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_1);
    let mut duration_7: std::time::Duration = crate::duration::Duration::abs_std(duration_6);
    let mut i8_3: i8 = 41i8;
    let mut i8_4: i8 = -25i8;
    let mut i8_5: i8 = -59i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut f64_0: f64 = 153.446166f64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i64_4: i64 = -32i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::abs(duration_9);
    let mut f64_1: f64 = -44.830960f64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut i8_6: i8 = 44i8;
    let mut i8_7: i8 = 50i8;
    let mut i8_8: i8 = 67i8;
    let mut u32_0: u32 = 42u32;
    let mut u8_0: u8 = 57u8;
    let mut u8_1: u8 = 1u8;
    let mut u8_2: u8 = 2u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_9: i8 = 25i8;
    let mut i8_10: i8 = -30i8;
    let mut i8_11: i8 = -112i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i64_5: i64 = -13i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::days(i64_5);
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::abs(duration_12);
    let mut u32_1: u32 = 68u32;
    let mut u8_3: u8 = 31u8;
    let mut u8_4: u8 = 42u8;
    let mut u8_5: u8 = 38u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut f64_2: f64 = 62.844148f64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_2);
    let mut duration_15: std::time::Duration = crate::duration::Duration::abs_std(duration_14);
    let mut i8_12: i8 = 86i8;
    let mut i8_13: i8 = 46i8;
    let mut i8_14: i8 = -4i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut i64_6: i64 = -22i64;
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::days(i64_6);
    let mut f64_3: f64 = -7.572826f64;
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_3);
    let mut i64_7: i64 = 96i64;
    let mut duration_18: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_7);
    let mut duration_19: crate::duration::Duration = crate::duration::Duration::abs(duration_18);
    let mut i8_15: i8 = 109i8;
    let mut i8_16: i8 = 80i8;
    let mut i8_17: i8 = -54i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut i64_8: i64 = 149i64;
    let mut duration_20: crate::duration::Duration = crate::duration::Duration::microseconds(i64_8);
    let mut duration_21: std::time::Duration = crate::duration::Duration::abs_std(duration_20);
    let mut i32_2: i32 = -20i32;
    let mut i64_9: i64 = -179i64;
    let mut duration_22: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_9, i32_2);
    let mut f64_4: f64 = -147.830099f64;
    let mut duration_23: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_4);
    let mut f32_0: f32 = -6.101015f32;
    let mut duration_24: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_25: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_24, duration_23);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_25);
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_3);
    let mut u32_2: u32 = 34u32;
    let mut u8_6: u8 = 40u8;
    let mut u8_7: u8 = 59u8;
    let mut u8_8: u8 = 22u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut f32_1: f32 = -56.062040f32;
    let mut duration_26: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut f32_2: f32 = 47.798172f32;
    let mut duration_27: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_2);
    let mut duration_28: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_27, duration_26);
    let mut duration_29: std::time::Duration = crate::duration::Duration::abs_std(duration_28);
    let mut i32_3: i32 = 75i32;
    let mut i128_1: i128 = -9i128;
    let mut duration_30: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut duration_31: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_30, i32_3);
    let mut i32_4: i32 = -103i32;
    let mut i64_10: i64 = -169i64;
    let mut duration_32: crate::duration::Duration = crate::duration::Duration::new(i64_10, i32_4);
    let mut i8_18: i8 = -127i8;
    let mut i8_19: i8 = -24i8;
    let mut i8_20: i8 = -86i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut i64_11: i64 = 11i64;
    let mut duration_33: crate::duration::Duration = crate::duration::Duration::seconds(i64_11);
    let mut duration_34: std::time::Duration = crate::duration::Duration::abs_std(duration_33);
    let mut i64_12: i64 = 174i64;
    let mut duration_35: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_12);
    panic!("From RustyUnit with love");
}
}