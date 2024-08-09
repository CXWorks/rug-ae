//! Serde visitor for various types.

use core::fmt;
use core::marker::PhantomData;

use serde::de;
#[cfg(feature = "serde-well-known")]
use serde::Deserializer;

#[cfg(feature = "parsing")]
use super::{
    DATE_FORMAT, OFFSET_DATE_TIME_FORMAT, PRIMITIVE_DATE_TIME_FORMAT, TIME_FORMAT,
    UTC_OFFSET_FORMAT,
};
use crate::error::ComponentRange;
#[cfg(feature = "serde-well-known")]
use crate::format_description::well_known;
use crate::{Date, Duration, Month, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset, Weekday};

/// A serde visitor for various types.
pub(super) struct Visitor<T: ?Sized>(pub(super) PhantomData<T>);

impl<'a> de::Visitor<'a> for Visitor<Date> {
    type Value = Date;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a `Date`")
    }

    #[cfg(feature = "parsing")]
    fn visit_str<E: de::Error>(self, value: &str) -> Result<Date, E> {
        Date::parse(value, &DATE_FORMAT).map_err(E::custom)
    }

    fn visit_seq<A: de::SeqAccess<'a>>(self, mut seq: A) -> Result<Date, A::Error> {
        let year = item!(seq, "year")?;
        let ordinal = item!(seq, "day of year")?;
        Date::from_ordinal_date(year, ordinal).map_err(ComponentRange::into_de_error)
    }
}

impl<'a> de::Visitor<'a> for Visitor<Duration> {
    type Value = Duration;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a `Duration`")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Duration, E> {
        let (seconds, nanoseconds) = value.split_once('.').ok_or_else(|| {
            de::Error::invalid_value(de::Unexpected::Str(value), &"a decimal point")
        })?;

        let seconds = seconds
            .parse()
            .map_err(|_| de::Error::invalid_value(de::Unexpected::Str(seconds), &"seconds"))?;
        let mut nanoseconds = nanoseconds.parse().map_err(|_| {
            de::Error::invalid_value(de::Unexpected::Str(nanoseconds), &"nanoseconds")
        })?;

        if seconds < 0 {
            nanoseconds *= -1;
        }

        Ok(Duration::new(seconds, nanoseconds))
    }

    fn visit_seq<A: de::SeqAccess<'a>>(self, mut seq: A) -> Result<Duration, A::Error> {
        let seconds = item!(seq, "seconds")?;
        let nanoseconds = item!(seq, "nanoseconds")?;
        Ok(Duration::new(seconds, nanoseconds))
    }
}

impl<'a> de::Visitor<'a> for Visitor<OffsetDateTime> {
    type Value = OffsetDateTime;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an `OffsetDateTime`")
    }

    #[cfg(feature = "parsing")]
    fn visit_str<E: de::Error>(self, value: &str) -> Result<OffsetDateTime, E> {
        OffsetDateTime::parse(value, &OFFSET_DATE_TIME_FORMAT).map_err(E::custom)
    }

    fn visit_seq<A: de::SeqAccess<'a>>(self, mut seq: A) -> Result<OffsetDateTime, A::Error> {
        let year = item!(seq, "year")?;
        let ordinal = item!(seq, "day of year")?;
        let hour = item!(seq, "hour")?;
        let minute = item!(seq, "minute")?;
        let second = item!(seq, "second")?;
        let nanosecond = item!(seq, "nanosecond")?;
        let offset_hours = item!(seq, "offset hours")?;
        let offset_minutes = item!(seq, "offset minutes")?;
        let offset_seconds = item!(seq, "offset seconds")?;

        Date::from_ordinal_date(year, ordinal)
            .and_then(|date| date.with_hms_nano(hour, minute, second, nanosecond))
            .and_then(|datetime| {
                UtcOffset::from_hms(offset_hours, offset_minutes, offset_seconds)
                    .map(|offset| datetime.assume_offset(offset))
            })
            .map_err(ComponentRange::into_de_error)
    }
}

impl<'a> de::Visitor<'a> for Visitor<PrimitiveDateTime> {
    type Value = PrimitiveDateTime;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a `PrimitiveDateTime`")
    }

    #[cfg(feature = "parsing")]
    fn visit_str<E: de::Error>(self, value: &str) -> Result<PrimitiveDateTime, E> {
        PrimitiveDateTime::parse(value, &PRIMITIVE_DATE_TIME_FORMAT).map_err(E::custom)
    }

    fn visit_seq<A: de::SeqAccess<'a>>(self, mut seq: A) -> Result<PrimitiveDateTime, A::Error> {
        let year = item!(seq, "year")?;
        let ordinal = item!(seq, "day of year")?;
        let hour = item!(seq, "hour")?;
        let minute = item!(seq, "minute")?;
        let second = item!(seq, "second")?;
        let nanosecond = item!(seq, "nanosecond")?;

        Date::from_ordinal_date(year, ordinal)
            .and_then(|date| date.with_hms_nano(hour, minute, second, nanosecond))
            .map_err(ComponentRange::into_de_error)
    }
}

impl<'a> de::Visitor<'a> for Visitor<Time> {
    type Value = Time;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a `Time`")
    }

    #[cfg(feature = "parsing")]
    fn visit_str<E: de::Error>(self, value: &str) -> Result<Time, E> {
        Time::parse(value, &TIME_FORMAT).map_err(E::custom)
    }

    fn visit_seq<A: de::SeqAccess<'a>>(self, mut seq: A) -> Result<Time, A::Error> {
        let hour = item!(seq, "hour")?;
        let minute = item!(seq, "minute")?;
        let second = item!(seq, "second")?;
        let nanosecond = item!(seq, "nanosecond")?;

        Time::from_hms_nano(hour, minute, second, nanosecond).map_err(ComponentRange::into_de_error)
    }
}

impl<'a> de::Visitor<'a> for Visitor<UtcOffset> {
    type Value = UtcOffset;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a `UtcOffset`")
    }

    #[cfg(feature = "parsing")]
    fn visit_str<E: de::Error>(self, value: &str) -> Result<UtcOffset, E> {
        UtcOffset::parse(value, &UTC_OFFSET_FORMAT).map_err(E::custom)
    }

    fn visit_seq<A: de::SeqAccess<'a>>(self, mut seq: A) -> Result<UtcOffset, A::Error> {
        let hours = item!(seq, "offset hours")?;
        let minutes = item!(seq, "offset minutes")?;
        let seconds = item!(seq, "offset seconds")?;

        UtcOffset::from_hms(hours, minutes, seconds).map_err(ComponentRange::into_de_error)
    }
}

impl<'a> de::Visitor<'a> for Visitor<Weekday> {
    type Value = Weekday;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a `Weekday`")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Weekday, E> {
        match value {
            "Monday" => Ok(Weekday::Monday),
            "Tuesday" => Ok(Weekday::Tuesday),
            "Wednesday" => Ok(Weekday::Wednesday),
            "Thursday" => Ok(Weekday::Thursday),
            "Friday" => Ok(Weekday::Friday),
            "Saturday" => Ok(Weekday::Saturday),
            "Sunday" => Ok(Weekday::Sunday),
            _ => Err(E::invalid_value(de::Unexpected::Str(value), &"a `Weekday`")),
        }
    }

    fn visit_u8<E: de::Error>(self, value: u8) -> Result<Weekday, E> {
        match value {
            1 => Ok(Weekday::Monday),
            2 => Ok(Weekday::Tuesday),
            3 => Ok(Weekday::Wednesday),
            4 => Ok(Weekday::Thursday),
            5 => Ok(Weekday::Friday),
            6 => Ok(Weekday::Saturday),
            7 => Ok(Weekday::Sunday),
            _ => Err(E::invalid_value(
                de::Unexpected::Unsigned(value.into()),
                &"a value in the range 1..=7",
            )),
        }
    }
}

impl<'a> de::Visitor<'a> for Visitor<Month> {
    type Value = Month;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a `Month`")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Month, E> {
        match value {
            "January" => Ok(Month::January),
            "February" => Ok(Month::February),
            "March" => Ok(Month::March),
            "April" => Ok(Month::April),
            "May" => Ok(Month::May),
            "June" => Ok(Month::June),
            "July" => Ok(Month::July),
            "August" => Ok(Month::August),
            "September" => Ok(Month::September),
            "October" => Ok(Month::October),
            "November" => Ok(Month::November),
            "December" => Ok(Month::December),
            _ => Err(E::invalid_value(de::Unexpected::Str(value), &"a `Month`")),
        }
    }

    fn visit_u8<E: de::Error>(self, value: u8) -> Result<Month, E> {
        match value {
            1 => Ok(Month::January),
            2 => Ok(Month::February),
            3 => Ok(Month::March),
            4 => Ok(Month::April),
            5 => Ok(Month::May),
            6 => Ok(Month::June),
            7 => Ok(Month::July),
            8 => Ok(Month::August),
            9 => Ok(Month::September),
            10 => Ok(Month::October),
            11 => Ok(Month::November),
            12 => Ok(Month::December),
            _ => Err(E::invalid_value(
                de::Unexpected::Unsigned(value.into()),
                &"a value in the range 1..=12",
            )),
        }
    }
}

#[cfg(feature = "serde-well-known")]
impl<'a> de::Visitor<'a> for Visitor<well_known::Rfc2822> {
    type Value = OffsetDateTime;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an RFC2822-formatted `OffsetDateTime`")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<OffsetDateTime, E> {
        OffsetDateTime::parse(value, &well_known::Rfc2822).map_err(E::custom)
    }
}

#[cfg(feature = "serde-well-known")]
impl<'a> de::Visitor<'a> for Visitor<Option<well_known::Rfc2822>> {
    type Value = Option<OffsetDateTime>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an RFC2822-formatted `Option<OffsetDateTime>`")
    }

    fn visit_some<D: Deserializer<'a>>(
        self,
        deserializer: D,
    ) -> Result<Option<OffsetDateTime>, D::Error> {
        deserializer
            .deserialize_any(Visitor::<well_known::Rfc2822>(PhantomData))
            .map(Some)
    }

    fn visit_none<E: de::Error>(self) -> Result<Option<OffsetDateTime>, E> {
        Ok(None)
    }
}

#[cfg(feature = "serde-well-known")]
impl<'a> de::Visitor<'a> for Visitor<well_known::Rfc3339> {
    type Value = OffsetDateTime;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an RFC3339-formatted `OffsetDateTime`")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<OffsetDateTime, E> {
        OffsetDateTime::parse(value, &well_known::Rfc3339).map_err(E::custom)
    }
}

#[cfg(feature = "serde-well-known")]
impl<'a> de::Visitor<'a> for Visitor<Option<well_known::Rfc3339>> {
    type Value = Option<OffsetDateTime>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an RFC3339-formatted `Option<OffsetDateTime>`")
    }

    fn visit_some<D: Deserializer<'a>>(
        self,
        deserializer: D,
    ) -> Result<Option<OffsetDateTime>, D::Error> {
        deserializer
            .deserialize_any(Visitor::<well_known::Rfc3339>(PhantomData))
            .map(Some)
    }

    fn visit_none<E: de::Error>(self) -> Result<Option<OffsetDateTime>, E> {
        Ok(None)
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8939() {
//    rusty_monitor::set_test_id(8939);
    let mut u32_0: u32 = 1000u32;
    let mut u8_0: u8 = 9u8;
    let mut u8_1: u8 = 12u8;
    let mut u8_2: u8 = 5u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut time_0_ref_0: &crate::time::Time = &mut time_0;
    let mut i32_0: i32 = 314i32;
    let mut i64_0: i64 = 32i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i32_1: i32 = 381i32;
    let mut i64_1: i64 = 604800i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_1);
    let mut u16_0: u16 = 367u16;
    let mut i32_2: i32 = 26i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut i64_2: i64 = 604800i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut i64_3: i64 = 24i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut i8_0: i8 = 6i8;
    let mut i8_1: i8 = 24i8;
    let mut i8_2: i8 = 127i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i32_3: i32 = 25i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_3};
    let mut i64_4: i64 = 3600i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut i32_4: i32 = 9i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_4};
    let mut date_4: crate::date::Date = crate::date::Date::saturating_add(date_3, duration_5);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_1, date_2);
    let mut time_1: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_2);
    let mut time_1_ref_0: &crate::time::Time = &mut time_1;
//    panic!("From RustyUnit with love");
}
}