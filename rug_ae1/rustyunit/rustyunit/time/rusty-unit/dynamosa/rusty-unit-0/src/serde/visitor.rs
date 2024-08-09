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

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7792() {
    rusty_monitor::set_test_id(7792);
    let mut u32_0: u32 = 89u32;
    let mut u8_0: u8 = 2u8;
    let mut u8_1: u8 = 73u8;
    let mut u8_2: u8 = 4u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = -43i32;
    let mut i64_0: i64 = 15i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_0);
    let mut i32_1: i32 = 14i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_1);
    let mut i64_1: i64 = 27i64;
    let mut i32_2: i32 = -113i32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_2, i32_2);
    let mut i64_2: i64 = 76i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut i32_3: i32 = -70i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_3};
    let mut i32_4: i32 = 20i32;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    let mut u8_3: u8 = 57u8;
    let mut i32_5: i32 = -88i32;
    let mut u32_1: u32 = 29u32;
    let mut u8_4: u8 = 2u8;
    let mut u8_5: u8 = 3u8;
    let mut u8_6: u8 = 9u8;
    let mut i64_3: i64 = 68i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::abs(duration_6);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_3: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_0);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_add(date_3, duration_7);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut f64_0: f64 = 106.341160f64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i64_4: i64 = 64i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::minutes(i64_4);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_9);
    let mut date_5: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut date_6: crate::date::Date = crate::date::Date::saturating_add(date_5, duration_8);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_6);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_1);
    let mut i64_5: i64 = -30i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::weeks(i64_5);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut i8_0: i8 = 39i8;
    let mut i8_1: i8 = 51i8;
    let mut i8_2: i8 = 91i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut f64_1: f64 = 64.196562f64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut u16_0: u16 = 0u16;
    let mut i32_6: i32 = 37i32;
    let mut date_7: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_6, u16_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_7);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_2, duration_11);
    let mut i8_3: i8 = 38i8;
    let mut i8_4: i8 = -21i8;
    let mut i8_5: i8 = 65i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u32_2: u32 = 98u32;
    let mut u8_7: u8 = 45u8;
    let mut u8_8: u8 = 45u8;
    let mut u8_9: u8 = 37u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_9, u8_8, u8_7, u32_2);
    let mut u32_3: u32 = 21u32;
    let mut u8_10: u8 = 0u8;
    let mut u8_11: u8 = 59u8;
    let mut u8_12: u8 = 56u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_12, u8_11, u8_10, u32_3);
    let mut i32_7: i32 = 18i32;
    let mut date_8: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_7);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_0);
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_4, time_2);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_5, utcoffset_1);
    let mut i64_6: i64 = -80i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_6);
    let mut month_0: month::Month = crate::month::Month::December;
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut bool_0: bool = crate::duration::Duration::is_zero(duration_12);
    let mut bool_1: bool = crate::utc_offset::UtcOffset::is_negative(utcoffset_0);
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_1, duration_10);
    let mut u16_1: u16 = crate::primitive_date_time::PrimitiveDateTime::millisecond(primitivedatetime_1);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_4, u8_6, u8_5, u8_4, u32_1);
    let mut result_1: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_5, u8_3, weekday_1);
    let mut u8_13: u8 = crate::util::weeks_in_year(i32_4);
    panic!("From RustyUnit with love");
}
}