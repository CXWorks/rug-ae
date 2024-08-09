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
#[timeout(30000)]fn rusty_test_8207() {
//    rusty_monitor::set_test_id(8207);
    let mut u16_0: u16 = 74u16;
    let mut f32_0: f32 = 1065353216.000000f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut u32_0: u32 = 10u32;
    let mut u8_0: u8 = 2u8;
    let mut u8_1: u8 = 79u8;
    let mut u8_2: u8 = 7u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_0: i8 = 23i8;
    let mut i8_1: i8 = 23i8;
    let mut i8_2: i8 = 6i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u16_1: u16 = 0u16;
    let mut i32_0: i32 = 86399i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_1);
    let mut i32_1: i32 = 235i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_0};
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut offsetdatetime_1_ref_0: &mut crate::offset_date_time::OffsetDateTime = &mut offsetdatetime_1;
    let mut i8_3: i8 = 6i8;
    let mut i8_4: i8 = 10i8;
    let mut i8_5: i8 = 4i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = 60i8;
    let mut i8_7: i8 = 5i8;
    let mut i8_8: i8 = 60i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut u32_1: u32 = 999999u32;
    let mut u8_3: u8 = 99u8;
    let mut u8_4: u8 = 30u8;
    let mut u8_5: u8 = 3u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut u32_2: u32 = 0u32;
    let mut u8_6: u8 = 96u8;
    let mut u8_7: u8 = 20u8;
    let mut u8_8: u8 = 91u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i32_2: i32 = 280i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_2, time_2);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_2, time_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_3, utcoffset_2);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_2, utcoffset_1);
    let mut f32_1: f32 = 1315859240.000000f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut u16_2: u16 = 87u16;
    let mut i32_3: i32 = 111i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_2);
    let mut i64_0: i64 = 1000000i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut i8_9: i8 = 23i8;
    let mut i8_10: i8 = 0i8;
    let mut i8_11: i8 = 5i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i32_4: i32 = 134i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut i64_1: i64 = -107i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_4, duration_3);
//    panic!("From RustyUnit with love");
}
}