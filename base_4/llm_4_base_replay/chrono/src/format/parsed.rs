//! A collection of parsed date and time items.
//! They can be constructed incrementally while being checked for consistency.
use core::convert::TryFrom;
use super::{ParseResult, IMPOSSIBLE, NOT_ENOUGH, OUT_OF_RANGE};
use crate::naive::{NaiveDate, NaiveDateTime, NaiveTime};
use crate::offset::{FixedOffset, LocalResult, Offset, TimeZone};
use crate::{DateTime, Datelike, TimeDelta, Timelike, Weekday};
/// Parsed parts of date and time. There are two classes of methods:
///
/// - `set_*` methods try to set given field(s) while checking for the consistency.
///   It may or may not check for the range constraint immediately (for efficiency reasons).
///
/// - `to_*` methods try to make a concrete date and time value out of set fields.
///   It fully checks any remaining out-of-range conditions and inconsistent/impossible fields.
#[non_exhaustive]
#[derive(Clone, PartialEq, Eq, Debug, Default, Hash)]
pub struct Parsed {
    /// Year.
    ///
    /// This can be negative unlike [`year_div_100`](#structfield.year_div_100)
    /// and [`year_mod_100`](#structfield.year_mod_100) fields.
    pub year: Option<i32>,
    /// Year divided by 100. Implies that the year is >= 1 BCE when set.
    ///
    /// Due to the common usage, if this field is missing but
    /// [`year_mod_100`](#structfield.year_mod_100) is present,
    /// it is inferred to 19 when `year_mod_100 >= 70` and 20 otherwise.
    pub year_div_100: Option<i32>,
    /// Year modulo 100. Implies that the year is >= 1 BCE when set.
    pub year_mod_100: Option<i32>,
    /// Year in the [ISO week date](../naive/struct.NaiveDate.html#week-date).
    ///
    /// This can be negative unlike [`isoyear_div_100`](#structfield.isoyear_div_100) and
    /// [`isoyear_mod_100`](#structfield.isoyear_mod_100) fields.
    pub isoyear: Option<i32>,
    /// Year in the [ISO week date](../naive/struct.NaiveDate.html#week-date), divided by 100.
    /// Implies that the year is >= 1 BCE when set.
    ///
    /// Due to the common usage, if this field is missing but
    /// [`isoyear_mod_100`](#structfield.isoyear_mod_100) is present,
    /// it is inferred to 19 when `isoyear_mod_100 >= 70` and 20 otherwise.
    pub isoyear_div_100: Option<i32>,
    /// Year in the [ISO week date](../naive/struct.NaiveDate.html#week-date), modulo 100.
    /// Implies that the year is >= 1 BCE when set.
    pub isoyear_mod_100: Option<i32>,
    /// Month (1--12).
    pub month: Option<u32>,
    /// Week number, where the week 1 starts at the first Sunday of January
    /// (0--53, 1--53 or 1--52 depending on the year).
    pub week_from_sun: Option<u32>,
    /// Week number, where the week 1 starts at the first Monday of January
    /// (0--53, 1--53 or 1--52 depending on the year).
    pub week_from_mon: Option<u32>,
    /// [ISO week number](../naive/struct.NaiveDate.html#week-date)
    /// (1--52 or 1--53 depending on the year).
    pub isoweek: Option<u32>,
    /// Day of the week.
    pub weekday: Option<Weekday>,
    /// Day of the year (1--365 or 1--366 depending on the year).
    pub ordinal: Option<u32>,
    /// Day of the month (1--28, 1--29, 1--30 or 1--31 depending on the month).
    pub day: Option<u32>,
    /// Hour number divided by 12 (0--1). 0 indicates AM and 1 indicates PM.
    pub hour_div_12: Option<u32>,
    /// Hour number modulo 12 (0--11).
    pub hour_mod_12: Option<u32>,
    /// Minute number (0--59).
    pub minute: Option<u32>,
    /// Second number (0--60, accounting for leap seconds).
    pub second: Option<u32>,
    /// The number of nanoseconds since the whole second (0--999,999,999).
    pub nanosecond: Option<u32>,
    /// The number of non-leap seconds since the midnight UTC on January 1, 1970.
    ///
    /// This can be off by one if [`second`](#structfield.second) is 60 (a leap second).
    pub timestamp: Option<i64>,
    /// Offset from the local time to UTC, in seconds.
    pub offset: Option<i32>,
}
/// Checks if `old` is either empty or has the same value as `new` (i.e. "consistent"),
/// and if it is empty, set `old` to `new` as well.
#[inline]
fn set_if_consistent<T: PartialEq>(old: &mut Option<T>, new: T) -> ParseResult<()> {
    if let Some(ref old) = *old {
        if *old == new { Ok(()) } else { Err(IMPOSSIBLE) }
    } else {
        *old = Some(new);
        Ok(())
    }
}
impl Parsed {
    /// Returns the initial value of parsed parts.
    #[must_use]
    pub fn new() -> Parsed {
        Parsed::default()
    }
    /// Tries to set the [`year`](#structfield.year) field from given value.
    #[inline]
    pub fn set_year(&mut self, value: i64) -> ParseResult<()> {
        set_if_consistent(
            &mut self.year,
            i32::try_from(value).map_err(|_| OUT_OF_RANGE)?,
        )
    }
    /// Tries to set the [`year_div_100`](#structfield.year_div_100) field from given value.
    #[inline]
    pub fn set_year_div_100(&mut self, value: i64) -> ParseResult<()> {
        if value < 0 {
            return Err(OUT_OF_RANGE);
        }
        set_if_consistent(
            &mut self.year_div_100,
            i32::try_from(value).map_err(|_| OUT_OF_RANGE)?,
        )
    }
    /// Tries to set the [`year_mod_100`](#structfield.year_mod_100) field from given value.
    #[inline]
    pub fn set_year_mod_100(&mut self, value: i64) -> ParseResult<()> {
        if value < 0 {
            return Err(OUT_OF_RANGE);
        }
        set_if_consistent(
            &mut self.year_mod_100,
            i32::try_from(value).map_err(|_| OUT_OF_RANGE)?,
        )
    }
    /// Tries to set the [`isoyear`](#structfield.isoyear) field from given value.
    #[inline]
    pub fn set_isoyear(&mut self, value: i64) -> ParseResult<()> {
        set_if_consistent(
            &mut self.isoyear,
            i32::try_from(value).map_err(|_| OUT_OF_RANGE)?,
        )
    }
    /// Tries to set the [`isoyear_div_100`](#structfield.isoyear_div_100) field from given value.
    #[inline]
    pub fn set_isoyear_div_100(&mut self, value: i64) -> ParseResult<()> {
        if value < 0 {
            return Err(OUT_OF_RANGE);
        }
        set_if_consistent(
            &mut self.isoyear_div_100,
            i32::try_from(value).map_err(|_| OUT_OF_RANGE)?,
        )
    }
    /// Tries to set the [`isoyear_mod_100`](#structfield.isoyear_mod_100) field from given value.
    #[inline]
    pub fn set_isoyear_mod_100(&mut self, value: i64) -> ParseResult<()> {
        if value < 0 {
            return Err(OUT_OF_RANGE);
        }
        set_if_consistent(
            &mut self.isoyear_mod_100,
            i32::try_from(value).map_err(|_| OUT_OF_RANGE)?,
        )
    }
    /// Tries to set the [`month`](#structfield.month) field from given value.
    #[inline]
    pub fn set_month(&mut self, value: i64) -> ParseResult<()> {
        set_if_consistent(
            &mut self.month,
            u32::try_from(value).map_err(|_| OUT_OF_RANGE)?,
        )
    }
    /// Tries to set the [`week_from_sun`](#structfield.week_from_sun) field from given value.
    #[inline]
    pub fn set_week_from_sun(&mut self, value: i64) -> ParseResult<()> {
        set_if_consistent(
            &mut self.week_from_sun,
            u32::try_from(value).map_err(|_| OUT_OF_RANGE)?,
        )
    }
    /// Tries to set the [`week_from_mon`](#structfield.week_from_mon) field from given value.
    #[inline]
    pub fn set_week_from_mon(&mut self, value: i64) -> ParseResult<()> {
        set_if_consistent(
            &mut self.week_from_mon,
            u32::try_from(value).map_err(|_| OUT_OF_RANGE)?,
        )
    }
    /// Tries to set the [`isoweek`](#structfield.isoweek) field from given value.
    #[inline]
    pub fn set_isoweek(&mut self, value: i64) -> ParseResult<()> {
        set_if_consistent(
            &mut self.isoweek,
            u32::try_from(value).map_err(|_| OUT_OF_RANGE)?,
        )
    }
    /// Tries to set the [`weekday`](#structfield.weekday) field from given value.
    #[inline]
    pub fn set_weekday(&mut self, value: Weekday) -> ParseResult<()> {
        set_if_consistent(&mut self.weekday, value)
    }
    /// Tries to set the [`ordinal`](#structfield.ordinal) field from given value.
    #[inline]
    pub fn set_ordinal(&mut self, value: i64) -> ParseResult<()> {
        set_if_consistent(
            &mut self.ordinal,
            u32::try_from(value).map_err(|_| OUT_OF_RANGE)?,
        )
    }
    /// Tries to set the [`day`](#structfield.day) field from given value.
    #[inline]
    pub fn set_day(&mut self, value: i64) -> ParseResult<()> {
        set_if_consistent(&mut self.day, u32::try_from(value).map_err(|_| OUT_OF_RANGE)?)
    }
    /// Tries to set the [`hour_div_12`](#structfield.hour_div_12) field from given value.
    /// (`false` for AM, `true` for PM)
    #[inline]
    pub fn set_ampm(&mut self, value: bool) -> ParseResult<()> {
        set_if_consistent(&mut self.hour_div_12, u32::from(value))
    }
    /// Tries to set the [`hour_mod_12`](#structfield.hour_mod_12) field from
    /// given hour number in 12-hour clocks.
    #[inline]
    pub fn set_hour12(&mut self, value: i64) -> ParseResult<()> {
        if !(1..=12).contains(&value) {
            return Err(OUT_OF_RANGE);
        }
        set_if_consistent(&mut self.hour_mod_12, value as u32 % 12)
    }
    /// Tries to set both [`hour_div_12`](#structfield.hour_div_12) and
    /// [`hour_mod_12`](#structfield.hour_mod_12) fields from given value.
    #[inline]
    pub fn set_hour(&mut self, value: i64) -> ParseResult<()> {
        let v = u32::try_from(value).map_err(|_| OUT_OF_RANGE)?;
        set_if_consistent(&mut self.hour_div_12, v / 12)?;
        set_if_consistent(&mut self.hour_mod_12, v % 12)?;
        Ok(())
    }
    /// Tries to set the [`minute`](#structfield.minute) field from given value.
    #[inline]
    pub fn set_minute(&mut self, value: i64) -> ParseResult<()> {
        set_if_consistent(
            &mut self.minute,
            u32::try_from(value).map_err(|_| OUT_OF_RANGE)?,
        )
    }
    /// Tries to set the [`second`](#structfield.second) field from given value.
    #[inline]
    pub fn set_second(&mut self, value: i64) -> ParseResult<()> {
        set_if_consistent(
            &mut self.second,
            u32::try_from(value).map_err(|_| OUT_OF_RANGE)?,
        )
    }
    /// Tries to set the [`nanosecond`](#structfield.nanosecond) field from given value.
    #[inline]
    pub fn set_nanosecond(&mut self, value: i64) -> ParseResult<()> {
        set_if_consistent(
            &mut self.nanosecond,
            u32::try_from(value).map_err(|_| OUT_OF_RANGE)?,
        )
    }
    /// Tries to set the [`timestamp`](#structfield.timestamp) field from given value.
    #[inline]
    pub fn set_timestamp(&mut self, value: i64) -> ParseResult<()> {
        set_if_consistent(&mut self.timestamp, value)
    }
    /// Tries to set the [`offset`](#structfield.offset) field from given value.
    #[inline]
    pub fn set_offset(&mut self, value: i64) -> ParseResult<()> {
        set_if_consistent(
            &mut self.offset,
            i32::try_from(value).map_err(|_| OUT_OF_RANGE)?,
        )
    }
    /// Returns a parsed naive date out of given fields.
    ///
    /// This method is able to determine the date from given subset of fields:
    ///
    /// - Year, month, day.
    /// - Year, day of the year (ordinal).
    /// - Year, week number counted from Sunday or Monday, day of the week.
    /// - ISO week date.
    ///
    /// Gregorian year and ISO week date year can have their century number (`*_div_100`) omitted,
    /// the two-digit year is used to guess the century number then.
    pub fn to_naive_date(&self) -> ParseResult<NaiveDate> {
        fn resolve_year(
            y: Option<i32>,
            q: Option<i32>,
            r: Option<i32>,
        ) -> ParseResult<Option<i32>> {
            match (y, q, r) {
                (y, None, None) => Ok(y),
                (Some(y), q, r @ Some(0..=99)) | (Some(y), q, r @ None) => {
                    if y < 0 {
                        return Err(OUT_OF_RANGE);
                    }
                    let q_ = y / 100;
                    let r_ = y % 100;
                    if q.unwrap_or(q_) == q_ && r.unwrap_or(r_) == r_ {
                        Ok(Some(y))
                    } else {
                        Err(IMPOSSIBLE)
                    }
                }
                (None, Some(q), Some(r @ 0..=99)) => {
                    if q < 0 {
                        return Err(OUT_OF_RANGE);
                    }
                    let y = q.checked_mul(100).and_then(|v| v.checked_add(r));
                    Ok(Some(y.ok_or(OUT_OF_RANGE)?))
                }
                (None, None, Some(r @ 0..=99)) => {
                    Ok(Some(r + if r < 70 { 2000 } else { 1900 }))
                }
                (None, Some(_), None) => Err(NOT_ENOUGH),
                (_, _, Some(_)) => Err(OUT_OF_RANGE),
            }
        }
        let given_year = resolve_year(self.year, self.year_div_100, self.year_mod_100)?;
        let given_isoyear = resolve_year(
            self.isoyear,
            self.isoyear_div_100,
            self.isoyear_mod_100,
        )?;
        let verify_ymd = |date: NaiveDate| {
            let year = date.year();
            let (year_div_100, year_mod_100) = if year >= 0 {
                (Some(year / 100), Some(year % 100))
            } else {
                (None, None)
            };
            let month = date.month();
            let day = date.day();
            self.year.unwrap_or(year) == year
                && self.year_div_100.or(year_div_100) == year_div_100
                && self.year_mod_100.or(year_mod_100) == year_mod_100
                && self.month.unwrap_or(month) == month && self.day.unwrap_or(day) == day
        };
        let verify_isoweekdate = |date: NaiveDate| {
            let week = date.iso_week();
            let isoyear = week.year();
            let isoweek = week.week();
            let weekday = date.weekday();
            let (isoyear_div_100, isoyear_mod_100) = if isoyear >= 0 {
                (Some(isoyear / 100), Some(isoyear % 100))
            } else {
                (None, None)
            };
            self.isoyear.unwrap_or(isoyear) == isoyear
                && self.isoyear_div_100.or(isoyear_div_100) == isoyear_div_100
                && self.isoyear_mod_100.or(isoyear_mod_100) == isoyear_mod_100
                && self.isoweek.unwrap_or(isoweek) == isoweek
                && self.weekday.unwrap_or(weekday) == weekday
        };
        let verify_ordinal = |date: NaiveDate| {
            let ordinal = date.ordinal();
            let week_from_sun = date.weeks_from(Weekday::Sun);
            let week_from_mon = date.weeks_from(Weekday::Mon);
            self.ordinal.unwrap_or(ordinal) == ordinal
                && self.week_from_sun.map_or(week_from_sun, |v| v as i32)
                    == week_from_sun
                && self.week_from_mon.map_or(week_from_mon, |v| v as i32)
                    == week_from_mon
        };
        let (verified, parsed_date) = match (given_year, given_isoyear, self) {
            (Some(year), _, &Parsed { month: Some(month), day: Some(day), .. }) => {
                let date = NaiveDate::from_ymd_opt(year, month, day)
                    .ok_or(OUT_OF_RANGE)?;
                (verify_isoweekdate(date) && verify_ordinal(date), date)
            }
            (Some(year), _, &Parsed { ordinal: Some(ordinal), .. }) => {
                let date = NaiveDate::from_yo_opt(year, ordinal).ok_or(OUT_OF_RANGE)?;
                (
                    verify_ymd(date) && verify_isoweekdate(date) && verify_ordinal(date),
                    date,
                )
            }
            (
                Some(year),
                _,
                &Parsed {
                    week_from_sun: Some(week_from_sun),
                    weekday: Some(weekday),
                    ..
                },
            ) => {
                let newyear = NaiveDate::from_yo_opt(year, 1).ok_or(OUT_OF_RANGE)?;
                let firstweek = match newyear.weekday() {
                    Weekday::Sun => 0,
                    Weekday::Mon => 6,
                    Weekday::Tue => 5,
                    Weekday::Wed => 4,
                    Weekday::Thu => 3,
                    Weekday::Fri => 2,
                    Weekday::Sat => 1,
                };
                if week_from_sun > 53 {
                    return Err(OUT_OF_RANGE);
                }
                let ndays = firstweek + (week_from_sun as i32 - 1) * 7
                    + weekday.num_days_from_sunday() as i32;
                let date = newyear
                    .checked_add_signed(TimeDelta::days(i64::from(ndays)))
                    .ok_or(OUT_OF_RANGE)?;
                if date.year() != year {
                    return Err(OUT_OF_RANGE);
                }
                (
                    verify_ymd(date) && verify_isoweekdate(date) && verify_ordinal(date),
                    date,
                )
            }
            (
                Some(year),
                _,
                &Parsed {
                    week_from_mon: Some(week_from_mon),
                    weekday: Some(weekday),
                    ..
                },
            ) => {
                let newyear = NaiveDate::from_yo_opt(year, 1).ok_or(OUT_OF_RANGE)?;
                let firstweek = match newyear.weekday() {
                    Weekday::Sun => 1,
                    Weekday::Mon => 0,
                    Weekday::Tue => 6,
                    Weekday::Wed => 5,
                    Weekday::Thu => 4,
                    Weekday::Fri => 3,
                    Weekday::Sat => 2,
                };
                if week_from_mon > 53 {
                    return Err(OUT_OF_RANGE);
                }
                let ndays = firstweek + (week_from_mon as i32 - 1) * 7
                    + weekday.num_days_from_monday() as i32;
                let date = newyear
                    .checked_add_signed(TimeDelta::days(i64::from(ndays)))
                    .ok_or(OUT_OF_RANGE)?;
                if date.year() != year {
                    return Err(OUT_OF_RANGE);
                }
                (
                    verify_ymd(date) && verify_isoweekdate(date) && verify_ordinal(date),
                    date,
                )
            }
            (
                _,
                Some(isoyear),
                &Parsed { isoweek: Some(isoweek), weekday: Some(weekday), .. },
            ) => {
                let date = NaiveDate::from_isoywd_opt(isoyear, isoweek, weekday);
                let date = date.ok_or(OUT_OF_RANGE)?;
                (verify_ymd(date) && verify_ordinal(date), date)
            }
            (_, _, _) => return Err(NOT_ENOUGH),
        };
        if verified { Ok(parsed_date) } else { Err(IMPOSSIBLE) }
    }
    /// Returns a parsed naive time out of given fields.
    ///
    /// This method is able to determine the time from given subset of fields:
    ///
    /// - Hour, minute. (second and nanosecond assumed to be 0)
    /// - Hour, minute, second. (nanosecond assumed to be 0)
    /// - Hour, minute, second, nanosecond.
    ///
    /// It is able to handle leap seconds when given second is 60.
    pub fn to_naive_time(&self) -> ParseResult<NaiveTime> {
        let hour_div_12 = match self.hour_div_12 {
            Some(v @ 0..=1) => v,
            Some(_) => return Err(OUT_OF_RANGE),
            None => return Err(NOT_ENOUGH),
        };
        let hour_mod_12 = match self.hour_mod_12 {
            Some(v @ 0..=11) => v,
            Some(_) => return Err(OUT_OF_RANGE),
            None => return Err(NOT_ENOUGH),
        };
        let hour = hour_div_12 * 12 + hour_mod_12;
        let minute = match self.minute {
            Some(v @ 0..=59) => v,
            Some(_) => return Err(OUT_OF_RANGE),
            None => return Err(NOT_ENOUGH),
        };
        let (second, mut nano) = match self.second.unwrap_or(0) {
            v @ 0..=59 => (v, 0),
            60 => (59, 1_000_000_000),
            _ => return Err(OUT_OF_RANGE),
        };
        nano
            += match self.nanosecond {
                Some(v @ 0..=999_999_999) if self.second.is_some() => v,
                Some(0..=999_999_999) => return Err(NOT_ENOUGH),
                Some(_) => return Err(OUT_OF_RANGE),
                None => 0,
            };
        NaiveTime::from_hms_nano_opt(hour, minute, second, nano).ok_or(OUT_OF_RANGE)
    }
    /// Returns a parsed naive date and time out of given fields,
    /// except for the [`offset`](#structfield.offset) field (assumed to have a given value).
    /// This is required for parsing a local time or other known-timezone inputs.
    ///
    /// This method is able to determine the combined date and time
    /// from date and time fields or a single [`timestamp`](#structfield.timestamp) field.
    /// Either way those fields have to be consistent to each other.
    pub fn to_naive_datetime_with_offset(
        &self,
        offset: i32,
    ) -> ParseResult<NaiveDateTime> {
        let date = self.to_naive_date();
        let time = self.to_naive_time();
        if let (Ok(date), Ok(time)) = (date, time) {
            let datetime = date.and_time(time);
            let timestamp = datetime.timestamp() - i64::from(offset);
            if let Some(given_timestamp) = self.timestamp {
                if given_timestamp != timestamp
                    && !(datetime.nanosecond() >= 1_000_000_000
                        && given_timestamp == timestamp + 1)
                {
                    return Err(IMPOSSIBLE);
                }
            }
            Ok(datetime)
        } else if let Some(timestamp) = self.timestamp {
            use super::ParseError as PE;
            use super::ParseErrorKind::{Impossible, OutOfRange};
            match (date, time) {
                (Err(PE(OutOfRange)), _) | (_, Err(PE(OutOfRange))) => {
                    return Err(OUT_OF_RANGE);
                }
                (Err(PE(Impossible)), _) | (_, Err(PE(Impossible))) => {
                    return Err(IMPOSSIBLE);
                }
                (_, _) => {}
            }
            let ts = timestamp.checked_add(i64::from(offset)).ok_or(OUT_OF_RANGE)?;
            let datetime = NaiveDateTime::from_timestamp_opt(ts, 0);
            let mut datetime = datetime.ok_or(OUT_OF_RANGE)?;
            let mut parsed = self.clone();
            if parsed.second == Some(60) {
                match datetime.second() {
                    59 => {}
                    0 => {
                        datetime -= TimeDelta::seconds(1);
                    }
                    _ => return Err(IMPOSSIBLE),
                }
            } else {
                parsed.set_second(i64::from(datetime.second()))?;
            }
            parsed.set_year(i64::from(datetime.year()))?;
            parsed.set_ordinal(i64::from(datetime.ordinal()))?;
            parsed.set_hour(i64::from(datetime.hour()))?;
            parsed.set_minute(i64::from(datetime.minute()))?;
            let date = parsed.to_naive_date()?;
            let time = parsed.to_naive_time()?;
            Ok(date.and_time(time))
        } else {
            date?;
            time?;
            unreachable!()
        }
    }
    /// Returns a parsed fixed time zone offset out of given fields.
    pub fn to_fixed_offset(&self) -> ParseResult<FixedOffset> {
        self.offset.and_then(FixedOffset::east_opt).ok_or(OUT_OF_RANGE)
    }
    /// Returns a parsed timezone-aware date and time out of given fields.
    ///
    /// This method is able to determine the combined date and time
    /// from date and time fields or a single [`timestamp`](#structfield.timestamp) field,
    /// plus a time zone offset.
    /// Either way those fields have to be consistent to each other.
    pub fn to_datetime(&self) -> ParseResult<DateTime<FixedOffset>> {
        let offset = self.offset.ok_or(NOT_ENOUGH)?;
        let datetime = self.to_naive_datetime_with_offset(offset)?;
        let offset = FixedOffset::east_opt(offset).ok_or(OUT_OF_RANGE)?;
        datetime
            .checked_sub_signed(TimeDelta::seconds(i64::from(offset.local_minus_utc())))
            .ok_or(OUT_OF_RANGE)?;
        match offset.from_local_datetime(&datetime) {
            LocalResult::None => Err(IMPOSSIBLE),
            LocalResult::Single(t) => Ok(t),
            LocalResult::Ambiguous(..) => Err(NOT_ENOUGH),
        }
    }
    /// Returns a parsed timezone-aware date and time out of given fields,
    /// with an additional `TimeZone` used to interpret and validate the local date.
    ///
    /// This method is able to determine the combined date and time
    /// from date and time fields or a single [`timestamp`](#structfield.timestamp) field,
    /// plus a time zone offset.
    /// Either way those fields have to be consistent to each other.
    /// If parsed fields include an UTC offset, it also has to be consistent to
    /// [`offset`](#structfield.offset).
    pub fn to_datetime_with_timezone<Tz: TimeZone>(
        &self,
        tz: &Tz,
    ) -> ParseResult<DateTime<Tz>> {
        let mut guessed_offset = 0;
        if let Some(timestamp) = self.timestamp {
            let nanosecond = self.nanosecond.unwrap_or(0);
            let dt = NaiveDateTime::from_timestamp_opt(timestamp, nanosecond);
            let dt = dt.ok_or(OUT_OF_RANGE)?;
            guessed_offset = tz.offset_from_utc_datetime(&dt).fix().local_minus_utc();
        }
        let check_offset = |dt: &DateTime<Tz>| {
            if let Some(offset) = self.offset {
                dt.offset().fix().local_minus_utc() == offset
            } else {
                true
            }
        };
        let datetime = self.to_naive_datetime_with_offset(guessed_offset)?;
        match tz.from_local_datetime(&datetime) {
            LocalResult::None => Err(IMPOSSIBLE),
            LocalResult::Single(t) => {
                if check_offset(&t) { Ok(t) } else { Err(IMPOSSIBLE) }
            }
            LocalResult::Ambiguous(min, max) => {
                match (check_offset(&min), check_offset(&max)) {
                    (false, false) => Err(IMPOSSIBLE),
                    (false, true) => Ok(max),
                    (true, false) => Ok(min),
                    (true, true) => Err(NOT_ENOUGH),
                }
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::super::{IMPOSSIBLE, NOT_ENOUGH, OUT_OF_RANGE};
    use super::Parsed;
    use crate::naive::{NaiveDate, NaiveTime};
    use crate::offset::{FixedOffset, TimeZone, Utc};
    use crate::Datelike;
    use crate::Weekday::*;
    #[test]
    fn test_parsed_set_fields() {
        let mut p = Parsed::new();
        assert_eq!(p.set_year(1987), Ok(()));
        assert_eq!(p.set_year(1986), Err(IMPOSSIBLE));
        assert_eq!(p.set_year(1988), Err(IMPOSSIBLE));
        assert_eq!(p.set_year(1987), Ok(()));
        assert_eq!(p.set_year_div_100(20), Ok(()));
        assert_eq!(p.set_year_div_100(21), Err(IMPOSSIBLE));
        assert_eq!(p.set_year_div_100(19), Err(IMPOSSIBLE));
        assert_eq!(p.set_year_mod_100(37), Ok(()));
        assert_eq!(p.set_year_mod_100(38), Err(IMPOSSIBLE));
        assert_eq!(p.set_year_mod_100(36), Err(IMPOSSIBLE));
        let mut p = Parsed::new();
        assert_eq!(p.set_year(0), Ok(()));
        assert_eq!(p.set_year_div_100(0), Ok(()));
        assert_eq!(p.set_year_mod_100(0), Ok(()));
        let mut p = Parsed::new();
        assert_eq!(p.set_year_div_100(- 1), Err(OUT_OF_RANGE));
        assert_eq!(p.set_year_mod_100(- 1), Err(OUT_OF_RANGE));
        assert_eq!(p.set_year(- 1), Ok(()));
        assert_eq!(p.set_year(- 2), Err(IMPOSSIBLE));
        assert_eq!(p.set_year(0), Err(IMPOSSIBLE));
        let mut p = Parsed::new();
        assert_eq!(p.set_year_div_100(0x1_0000_0008), Err(OUT_OF_RANGE));
        assert_eq!(p.set_year_div_100(8), Ok(()));
        assert_eq!(p.set_year_div_100(0x1_0000_0008), Err(OUT_OF_RANGE));
        let mut p = Parsed::new();
        assert_eq!(p.set_month(7), Ok(()));
        assert_eq!(p.set_month(1), Err(IMPOSSIBLE));
        assert_eq!(p.set_month(6), Err(IMPOSSIBLE));
        assert_eq!(p.set_month(8), Err(IMPOSSIBLE));
        assert_eq!(p.set_month(12), Err(IMPOSSIBLE));
        let mut p = Parsed::new();
        assert_eq!(p.set_month(8), Ok(()));
        assert_eq!(p.set_month(0x1_0000_0008), Err(OUT_OF_RANGE));
        let mut p = Parsed::new();
        assert_eq!(p.set_hour(12), Ok(()));
        assert_eq!(p.set_hour(11), Err(IMPOSSIBLE));
        assert_eq!(p.set_hour(13), Err(IMPOSSIBLE));
        assert_eq!(p.set_hour(12), Ok(()));
        assert_eq!(p.set_ampm(false), Err(IMPOSSIBLE));
        assert_eq!(p.set_ampm(true), Ok(()));
        assert_eq!(p.set_hour12(12), Ok(()));
        assert_eq!(p.set_hour12(0), Err(OUT_OF_RANGE));
        assert_eq!(p.set_hour12(1), Err(IMPOSSIBLE));
        assert_eq!(p.set_hour12(11), Err(IMPOSSIBLE));
        let mut p = Parsed::new();
        assert_eq!(p.set_ampm(true), Ok(()));
        assert_eq!(p.set_hour12(7), Ok(()));
        assert_eq!(p.set_hour(7), Err(IMPOSSIBLE));
        assert_eq!(p.set_hour(18), Err(IMPOSSIBLE));
        assert_eq!(p.set_hour(19), Ok(()));
        let mut p = Parsed::new();
        assert_eq!(p.set_timestamp(1_234_567_890), Ok(()));
        assert_eq!(p.set_timestamp(1_234_567_889), Err(IMPOSSIBLE));
        assert_eq!(p.set_timestamp(1_234_567_891), Err(IMPOSSIBLE));
    }
    #[test]
    fn test_parsed_to_naive_date() {
        macro_rules! parse {
            ($($k:ident : $v:expr),*) => {
                Parsed { $($k : Some($v),)* ..Parsed::new() } .to_naive_date()
            };
        }
        let ymd = |y, m, d| Ok(NaiveDate::from_ymd_opt(y, m, d).unwrap());
        assert_eq!(parse!(), Err(NOT_ENOUGH));
        assert_eq!(parse!(year : 1984), Err(NOT_ENOUGH));
        assert_eq!(parse!(year : 1984, month : 1), Err(NOT_ENOUGH));
        assert_eq!(parse!(year : 1984, month : 1, day : 2), ymd(1984, 1, 2));
        assert_eq!(parse!(year : 1984, day : 2), Err(NOT_ENOUGH));
        assert_eq!(parse!(year_div_100 : 19), Err(NOT_ENOUGH));
        assert_eq!(parse!(year_div_100 : 19, year_mod_100 : 84), Err(NOT_ENOUGH));
        assert_eq!(
            parse!(year_div_100 : 19, year_mod_100 : 84, month : 1), Err(NOT_ENOUGH)
        );
        assert_eq!(
            parse!(year_div_100 : 19, year_mod_100 : 84, month : 1, day : 2), ymd(1984,
            1, 2)
        );
        assert_eq!(
            parse!(year_div_100 : 19, year_mod_100 : 84, day : 2), Err(NOT_ENOUGH)
        );
        assert_eq!(parse!(year_div_100 : 19, month : 1, day : 2), Err(NOT_ENOUGH));
        assert_eq!(parse!(year_mod_100 : 70, month : 1, day : 2), ymd(1970, 1, 2));
        assert_eq!(parse!(year_mod_100 : 69, month : 1, day : 2), ymd(2069, 1, 2));
        assert_eq!(
            parse!(year_div_100 : 19, year_mod_100 : 84, month : 2, day : 29), ymd(1984,
            2, 29)
        );
        assert_eq!(
            parse!(year_div_100 : 19, year_mod_100 : 83, month : 2, day : 29),
            Err(OUT_OF_RANGE)
        );
        assert_eq!(
            parse!(year_div_100 : 19, year_mod_100 : 83, month : 13, day : 1),
            Err(OUT_OF_RANGE)
        );
        assert_eq!(
            parse!(year_div_100 : 19, year_mod_100 : 83, month : 12, day : 31), ymd(1983,
            12, 31)
        );
        assert_eq!(
            parse!(year_div_100 : 19, year_mod_100 : 83, month : 12, day : 32),
            Err(OUT_OF_RANGE)
        );
        assert_eq!(
            parse!(year_div_100 : 19, year_mod_100 : 83, month : 12, day : 0),
            Err(OUT_OF_RANGE)
        );
        assert_eq!(
            parse!(year_div_100 : 19, year_mod_100 : 100, month : 1, day : 1),
            Err(OUT_OF_RANGE)
        );
        assert_eq!(
            parse!(year_div_100 : 19, year_mod_100 : - 1, month : 1, day : 1),
            Err(OUT_OF_RANGE)
        );
        assert_eq!(
            parse!(year_div_100 : 0, year_mod_100 : 0, month : 1, day : 1), ymd(0, 1, 1)
        );
        assert_eq!(
            parse!(year_div_100 : - 1, year_mod_100 : 42, month : 1, day : 1),
            Err(OUT_OF_RANGE)
        );
        let max_year = NaiveDate::MAX.year();
        assert_eq!(
            parse!(year_div_100 : max_year / 100, year_mod_100 : max_year % 100, month :
            1, day : 1), ymd(max_year, 1, 1)
        );
        assert_eq!(
            parse!(year_div_100 : (max_year + 1) / 100, year_mod_100 : (max_year + 1) %
            100, month : 1, day : 1), Err(OUT_OF_RANGE)
        );
        assert_eq!(
            parse!(year : 1984, year_div_100 : 19, month : 1, day : 1), ymd(1984, 1, 1)
        );
        assert_eq!(
            parse!(year : 1984, year_div_100 : 20, month : 1, day : 1), Err(IMPOSSIBLE)
        );
        assert_eq!(
            parse!(year : 1984, year_mod_100 : 84, month : 1, day : 1), ymd(1984, 1, 1)
        );
        assert_eq!(
            parse!(year : 1984, year_mod_100 : 83, month : 1, day : 1), Err(IMPOSSIBLE)
        );
        assert_eq!(
            parse!(year : 1984, year_div_100 : 19, year_mod_100 : 84, month : 1, day :
            1), ymd(1984, 1, 1)
        );
        assert_eq!(
            parse!(year : 1984, year_div_100 : 18, year_mod_100 : 94, month : 1, day :
            1), Err(IMPOSSIBLE)
        );
        assert_eq!(
            parse!(year : 1984, year_div_100 : 18, year_mod_100 : 184, month : 1, day :
            1), Err(OUT_OF_RANGE)
        );
        assert_eq!(
            parse!(year : - 1, year_div_100 : 0, year_mod_100 : - 1, month : 1, day : 1),
            Err(OUT_OF_RANGE)
        );
        assert_eq!(
            parse!(year : - 1, year_div_100 : - 1, year_mod_100 : 99, month : 1, day :
            1), Err(OUT_OF_RANGE)
        );
        assert_eq!(
            parse!(year : - 1, year_div_100 : 0, month : 1, day : 1), Err(OUT_OF_RANGE)
        );
        assert_eq!(
            parse!(year : - 1, year_mod_100 : 99, month : 1, day : 1), Err(OUT_OF_RANGE)
        );
        assert_eq!(parse!(year : 2000, week_from_mon : 0), Err(NOT_ENOUGH));
        assert_eq!(parse!(year : 2000, week_from_sun : 0), Err(NOT_ENOUGH));
        assert_eq!(parse!(year : 2000, weekday : Sun), Err(NOT_ENOUGH));
        assert_eq!(
            parse!(year : 2000, week_from_mon : 0, weekday : Fri), Err(OUT_OF_RANGE)
        );
        assert_eq!(
            parse!(year : 2000, week_from_sun : 0, weekday : Fri), Err(OUT_OF_RANGE)
        );
        assert_eq!(
            parse!(year : 2000, week_from_mon : 0, weekday : Sat), ymd(2000, 1, 1)
        );
        assert_eq!(
            parse!(year : 2000, week_from_sun : 0, weekday : Sat), ymd(2000, 1, 1)
        );
        assert_eq!(
            parse!(year : 2000, week_from_mon : 0, weekday : Sun), ymd(2000, 1, 2)
        );
        assert_eq!(
            parse!(year : 2000, week_from_sun : 1, weekday : Sun), ymd(2000, 1, 2)
        );
        assert_eq!(
            parse!(year : 2000, week_from_mon : 1, weekday : Mon), ymd(2000, 1, 3)
        );
        assert_eq!(
            parse!(year : 2000, week_from_sun : 1, weekday : Mon), ymd(2000, 1, 3)
        );
        assert_eq!(
            parse!(year : 2000, week_from_mon : 1, weekday : Sat), ymd(2000, 1, 8)
        );
        assert_eq!(
            parse!(year : 2000, week_from_sun : 1, weekday : Sat), ymd(2000, 1, 8)
        );
        assert_eq!(
            parse!(year : 2000, week_from_mon : 1, weekday : Sun), ymd(2000, 1, 9)
        );
        assert_eq!(
            parse!(year : 2000, week_from_sun : 2, weekday : Sun), ymd(2000, 1, 9)
        );
        assert_eq!(
            parse!(year : 2000, week_from_mon : 2, weekday : Mon), ymd(2000, 1, 10)
        );
        assert_eq!(
            parse!(year : 2000, week_from_sun : 52, weekday : Sat), ymd(2000, 12, 30)
        );
        assert_eq!(
            parse!(year : 2000, week_from_sun : 53, weekday : Sun), ymd(2000, 12, 31)
        );
        assert_eq!(
            parse!(year : 2000, week_from_sun : 53, weekday : Mon), Err(OUT_OF_RANGE)
        );
        assert_eq!(
            parse!(year : 2000, week_from_sun : 0xffffffff, weekday : Mon),
            Err(OUT_OF_RANGE)
        );
        assert_eq!(
            parse!(year : 2006, week_from_sun : 0, weekday : Sat), Err(OUT_OF_RANGE)
        );
        assert_eq!(
            parse!(year : 2006, week_from_sun : 1, weekday : Sun), ymd(2006, 1, 1)
        );
        assert_eq!(
            parse!(year : 2000, week_from_mon : 1, week_from_sun : 1, weekday : Sat),
            ymd(2000, 1, 8)
        );
        assert_eq!(
            parse!(year : 2000, week_from_mon : 1, week_from_sun : 2, weekday : Sun),
            ymd(2000, 1, 9)
        );
        assert_eq!(
            parse!(year : 2000, week_from_mon : 1, week_from_sun : 1, weekday : Sun),
            Err(IMPOSSIBLE)
        );
        assert_eq!(
            parse!(year : 2000, week_from_mon : 2, week_from_sun : 2, weekday : Sun),
            Err(IMPOSSIBLE)
        );
        assert_eq!(parse!(isoyear : 2004, isoweek : 53), Err(NOT_ENOUGH));
        assert_eq!(
            parse!(isoyear : 2004, isoweek : 53, weekday : Fri), ymd(2004, 12, 31)
        );
        assert_eq!(parse!(isoyear : 2004, isoweek : 53, weekday : Sat), ymd(2005, 1, 1));
        assert_eq!(
            parse!(isoyear : 2004, isoweek : 0xffffffff, weekday : Sat),
            Err(OUT_OF_RANGE)
        );
        assert_eq!(
            parse!(isoyear : 2005, isoweek : 0, weekday : Thu), Err(OUT_OF_RANGE)
        );
        assert_eq!(parse!(isoyear : 2005, isoweek : 5, weekday : Thu), ymd(2005, 2, 3));
        assert_eq!(parse!(isoyear : 2005, weekday : Thu), Err(NOT_ENOUGH));
        assert_eq!(parse!(ordinal : 123), Err(NOT_ENOUGH));
        assert_eq!(parse!(year : 2000, ordinal : 0), Err(OUT_OF_RANGE));
        assert_eq!(parse!(year : 2000, ordinal : 1), ymd(2000, 1, 1));
        assert_eq!(parse!(year : 2000, ordinal : 60), ymd(2000, 2, 29));
        assert_eq!(parse!(year : 2000, ordinal : 61), ymd(2000, 3, 1));
        assert_eq!(parse!(year : 2000, ordinal : 366), ymd(2000, 12, 31));
        assert_eq!(parse!(year : 2000, ordinal : 367), Err(OUT_OF_RANGE));
        assert_eq!(parse!(year : 2000, ordinal : 0xffffffff), Err(OUT_OF_RANGE));
        assert_eq!(parse!(year : 2100, ordinal : 0), Err(OUT_OF_RANGE));
        assert_eq!(parse!(year : 2100, ordinal : 1), ymd(2100, 1, 1));
        assert_eq!(parse!(year : 2100, ordinal : 59), ymd(2100, 2, 28));
        assert_eq!(parse!(year : 2100, ordinal : 60), ymd(2100, 3, 1));
        assert_eq!(parse!(year : 2100, ordinal : 365), ymd(2100, 12, 31));
        assert_eq!(parse!(year : 2100, ordinal : 366), Err(OUT_OF_RANGE));
        assert_eq!(parse!(year : 2100, ordinal : 0xffffffff), Err(OUT_OF_RANGE));
        assert_eq!(
            parse!(year : 2014, month : 12, day : 31, ordinal : 365, isoyear : 2015,
            isoweek : 1, week_from_sun : 52, week_from_mon : 52, weekday : Wed),
            ymd(2014, 12, 31)
        );
        assert_eq!(
            parse!(year : 2014, month : 12, ordinal : 365, isoyear : 2015, isoweek : 1,
            week_from_sun : 52, week_from_mon : 52), ymd(2014, 12, 31)
        );
        assert_eq!(
            parse!(year : 2014, month : 12, day : 31, ordinal : 365, isoyear : 2014,
            isoweek : 53, week_from_sun : 52, week_from_mon : 52, weekday : Wed),
            Err(IMPOSSIBLE)
        );
        assert_eq!(
            parse!(year : 2012, isoyear : 2015, isoweek : 1, week_from_sun : 52,
            week_from_mon : 52), Err(NOT_ENOUGH)
        );
        assert_eq!(
            parse!(year_div_100 : 20, isoyear_mod_100 : 15, ordinal : 366),
            Err(NOT_ENOUGH)
        );
    }
    #[test]
    fn test_parsed_to_naive_time() {
        macro_rules! parse {
            ($($k:ident : $v:expr),*) => {
                Parsed { $($k : Some($v),)* ..Parsed::new() } .to_naive_time()
            };
        }
        let hms = |h, m, s| Ok(NaiveTime::from_hms_opt(h, m, s).unwrap());
        let hmsn = |h, m, s, n| Ok(NaiveTime::from_hms_nano_opt(h, m, s, n).unwrap());
        assert_eq!(parse!(), Err(NOT_ENOUGH));
        assert_eq!(parse!(hour_div_12 : 0), Err(NOT_ENOUGH));
        assert_eq!(parse!(hour_div_12 : 0, hour_mod_12 : 1), Err(NOT_ENOUGH));
        assert_eq!(parse!(hour_div_12 : 0, hour_mod_12 : 1, minute : 23), hms(1, 23, 0));
        assert_eq!(
            parse!(hour_div_12 : 0, hour_mod_12 : 1, minute : 23, second : 45), hms(1,
            23, 45)
        );
        assert_eq!(
            parse!(hour_div_12 : 0, hour_mod_12 : 1, minute : 23, second : 45, nanosecond
            : 678_901_234), hmsn(1, 23, 45, 678_901_234)
        );
        assert_eq!(
            parse!(hour_div_12 : 1, hour_mod_12 : 11, minute : 45, second : 6), hms(23,
            45, 6)
        );
        assert_eq!(parse!(hour_mod_12 : 1, minute : 23), Err(NOT_ENOUGH));
        assert_eq!(
            parse!(hour_div_12 : 0, hour_mod_12 : 1, minute : 23, nanosecond :
            456_789_012), Err(NOT_ENOUGH)
        );
        assert_eq!(
            parse!(hour_div_12 : 2, hour_mod_12 : 0, minute : 0), Err(OUT_OF_RANGE)
        );
        assert_eq!(
            parse!(hour_div_12 : 1, hour_mod_12 : 12, minute : 0), Err(OUT_OF_RANGE)
        );
        assert_eq!(
            parse!(hour_div_12 : 0, hour_mod_12 : 1, minute : 60), Err(OUT_OF_RANGE)
        );
        assert_eq!(
            parse!(hour_div_12 : 0, hour_mod_12 : 1, minute : 23, second : 61),
            Err(OUT_OF_RANGE)
        );
        assert_eq!(
            parse!(hour_div_12 : 0, hour_mod_12 : 1, minute : 23, second : 34, nanosecond
            : 1_000_000_000), Err(OUT_OF_RANGE)
        );
        assert_eq!(
            parse!(hour_div_12 : 0, hour_mod_12 : 1, minute : 23, second : 60), hmsn(1,
            23, 59, 1_000_000_000)
        );
        assert_eq!(
            parse!(hour_div_12 : 0, hour_mod_12 : 1, minute : 23, second : 60, nanosecond
            : 999_999_999), hmsn(1, 23, 59, 1_999_999_999)
        );
    }
    #[test]
    fn test_parsed_to_naive_datetime_with_offset() {
        macro_rules! parse {
            (offset = $offset:expr; $($k:ident : $v:expr),*) => {
                Parsed { $($k : Some($v),)* ..Parsed::new() }
                .to_naive_datetime_with_offset($offset)
            };
            ($($k:ident : $v:expr),*) => {
                parse!(offset = 0; $($k : $v),*)
            };
        }
        let ymdhms = |y, m, d, h, n, s| {
            Ok(NaiveDate::from_ymd_opt(y, m, d).unwrap().and_hms_opt(h, n, s).unwrap())
        };
        let ymdhmsn = |y, m, d, h, n, s, nano| {
            Ok(
                NaiveDate::from_ymd_opt(y, m, d)
                    .unwrap()
                    .and_hms_nano_opt(h, n, s, nano)
                    .unwrap(),
            )
        };
        assert_eq!(parse!(), Err(NOT_ENOUGH));
        assert_eq!(
            parse!(year : 2015, month : 1, day : 30, hour_div_12 : 1, hour_mod_12 : 2,
            minute : 38), ymdhms(2015, 1, 30, 14, 38, 0)
        );
        assert_eq!(
            parse!(year : 1997, month : 1, day : 30, hour_div_12 : 1, hour_mod_12 : 2,
            minute : 38, second : 5), ymdhms(1997, 1, 30, 14, 38, 5)
        );
        assert_eq!(
            parse!(year : 2012, ordinal : 34, hour_div_12 : 0, hour_mod_12 : 5, minute :
            6, second : 7, nanosecond : 890_123_456), ymdhmsn(2012, 2, 3, 5, 6, 7,
            890_123_456)
        );
        assert_eq!(parse!(timestamp : 0), ymdhms(1970, 1, 1, 0, 0, 0));
        assert_eq!(parse!(timestamp : 1, nanosecond : 0), ymdhms(1970, 1, 1, 0, 0, 1));
        assert_eq!(
            parse!(timestamp : 1, nanosecond : 1), ymdhmsn(1970, 1, 1, 0, 0, 1, 1)
        );
        assert_eq!(parse!(timestamp : 1_420_000_000), ymdhms(2014, 12, 31, 4, 26, 40));
        assert_eq!(
            parse!(timestamp : - 0x1_0000_0000), ymdhms(1833, 11, 24, 17, 31, 44)
        );
        assert_eq!(
            parse!(year : 2014, year_div_100 : 20, year_mod_100 : 14, month : 12, day :
            31, ordinal : 365, isoyear : 2015, isoyear_div_100 : 20, isoyear_mod_100 :
            15, isoweek : 1, week_from_sun : 52, week_from_mon : 52, weekday : Wed,
            hour_div_12 : 0, hour_mod_12 : 4, minute : 26, second : 40, nanosecond :
            12_345_678, timestamp : 1_420_000_000), ymdhmsn(2014, 12, 31, 4, 26, 40,
            12_345_678)
        );
        assert_eq!(
            parse!(year : 2014, year_div_100 : 20, year_mod_100 : 14, month : 12, day :
            31, ordinal : 365, isoyear : 2015, isoyear_div_100 : 20, isoyear_mod_100 :
            15, isoweek : 1, week_from_sun : 52, week_from_mon : 52, weekday : Wed,
            hour_div_12 : 0, hour_mod_12 : 4, minute : 26, second : 40, nanosecond :
            12_345_678, timestamp : 1_419_999_999), Err(IMPOSSIBLE)
        );
        assert_eq!(
            parse!(offset = 32400; year : 2014, year_div_100 : 20, year_mod_100 : 14,
            month : 12, day : 31, ordinal : 365, isoyear : 2015, isoyear_div_100 : 20,
            isoyear_mod_100 : 15, isoweek : 1, week_from_sun : 52, week_from_mon : 52,
            weekday : Wed, hour_div_12 : 0, hour_mod_12 : 4, minute : 26, second : 40,
            nanosecond : 12_345_678, timestamp : 1_419_967_600), ymdhmsn(2014, 12, 31, 4,
            26, 40, 12_345_678)
        );
        let max_days_from_year_1970 = NaiveDate::MAX
            .signed_duration_since(NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());
        let year_0_from_year_1970 = NaiveDate::from_ymd_opt(0, 1, 1)
            .unwrap()
            .signed_duration_since(NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());
        let min_days_from_year_1970 = NaiveDate::MIN
            .signed_duration_since(NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());
        assert_eq!(
            parse!(timestamp : min_days_from_year_1970.num_seconds()),
            ymdhms(NaiveDate::MIN.year(), 1, 1, 0, 0, 0)
        );
        assert_eq!(
            parse!(timestamp : year_0_from_year_1970.num_seconds()), ymdhms(0, 1, 1, 0,
            0, 0)
        );
        assert_eq!(
            parse!(timestamp : max_days_from_year_1970.num_seconds() + 86399),
            ymdhms(NaiveDate::MAX.year(), 12, 31, 23, 59, 59)
        );
        assert_eq!(parse!(second : 59, timestamp : 1_341_100_798), Err(IMPOSSIBLE));
        assert_eq!(
            parse!(second : 59, timestamp : 1_341_100_799), ymdhms(2012, 6, 30, 23, 59,
            59)
        );
        assert_eq!(parse!(second : 59, timestamp : 1_341_100_800), Err(IMPOSSIBLE));
        assert_eq!(
            parse!(second : 60, timestamp : 1_341_100_799), ymdhmsn(2012, 6, 30, 23, 59,
            59, 1_000_000_000)
        );
        assert_eq!(
            parse!(second : 60, timestamp : 1_341_100_800), ymdhmsn(2012, 6, 30, 23, 59,
            59, 1_000_000_000)
        );
        assert_eq!(
            parse!(second : 0, timestamp : 1_341_100_800), ymdhms(2012, 7, 1, 0, 0, 0)
        );
        assert_eq!(parse!(second : 1, timestamp : 1_341_100_800), Err(IMPOSSIBLE));
        assert_eq!(parse!(second : 60, timestamp : 1_341_100_801), Err(IMPOSSIBLE));
        assert_eq!(
            parse!(year : 2012, ordinal : 182, hour_div_12 : 1, hour_mod_12 : 11, minute
            : 59, second : 59, timestamp : 1_341_100_798), Err(IMPOSSIBLE)
        );
        assert_eq!(
            parse!(year : 2012, ordinal : 182, hour_div_12 : 1, hour_mod_12 : 11, minute
            : 59, second : 59, timestamp : 1_341_100_799), ymdhms(2012, 6, 30, 23, 59,
            59)
        );
        assert_eq!(
            parse!(year : 2012, ordinal : 182, hour_div_12 : 1, hour_mod_12 : 11, minute
            : 59, second : 59, timestamp : 1_341_100_800), Err(IMPOSSIBLE)
        );
        assert_eq!(
            parse!(year : 2012, ordinal : 182, hour_div_12 : 1, hour_mod_12 : 11, minute
            : 59, second : 60, timestamp : 1_341_100_799), ymdhmsn(2012, 6, 30, 23, 59,
            59, 1_000_000_000)
        );
        assert_eq!(
            parse!(year : 2012, ordinal : 182, hour_div_12 : 1, hour_mod_12 : 11, minute
            : 59, second : 60, timestamp : 1_341_100_800), ymdhmsn(2012, 6, 30, 23, 59,
            59, 1_000_000_000)
        );
        assert_eq!(
            parse!(year : 2012, ordinal : 183, hour_div_12 : 0, hour_mod_12 : 0, minute :
            0, second : 0, timestamp : 1_341_100_800), ymdhms(2012, 7, 1, 0, 0, 0)
        );
        assert_eq!(
            parse!(year : 2012, ordinal : 183, hour_div_12 : 0, hour_mod_12 : 0, minute :
            0, second : 1, timestamp : 1_341_100_800), Err(IMPOSSIBLE)
        );
        assert_eq!(
            parse!(year : 2012, ordinal : 182, hour_div_12 : 1, hour_mod_12 : 11, minute
            : 59, second : 60, timestamp : 1_341_100_801), Err(IMPOSSIBLE)
        );
        assert_eq!(
            parse!(year : 2015, month : 1, day : 20, weekday : Tue, hour_div_12 : 2,
            hour_mod_12 : 1, minute : 35, second : 20), Err(OUT_OF_RANGE)
        );
    }
    #[test]
    fn test_parsed_to_datetime() {
        macro_rules! parse {
            ($($k:ident : $v:expr),*) => {
                Parsed { $($k : Some($v),)* ..Parsed::new() } .to_datetime()
            };
        }
        let ymdhmsn = |y, m, d, h, n, s, nano, off| {
            Ok(
                FixedOffset::east_opt(off)
                    .unwrap()
                    .from_local_datetime(
                        &NaiveDate::from_ymd_opt(y, m, d)
                            .unwrap()
                            .and_hms_nano_opt(h, n, s, nano)
                            .unwrap(),
                    )
                    .unwrap(),
            )
        };
        assert_eq!(parse!(offset : 0), Err(NOT_ENOUGH));
        assert_eq!(
            parse!(year : 2014, ordinal : 365, hour_div_12 : 0, hour_mod_12 : 4, minute :
            26, second : 40, nanosecond : 12_345_678), Err(NOT_ENOUGH)
        );
        assert_eq!(
            parse!(year : 2014, ordinal : 365, hour_div_12 : 0, hour_mod_12 : 4, minute :
            26, second : 40, nanosecond : 12_345_678, offset : 0), ymdhmsn(2014, 12, 31,
            4, 26, 40, 12_345_678, 0)
        );
        assert_eq!(
            parse!(year : 2014, ordinal : 365, hour_div_12 : 1, hour_mod_12 : 1, minute :
            26, second : 40, nanosecond : 12_345_678, offset : 32400), ymdhmsn(2014, 12,
            31, 13, 26, 40, 12_345_678, 32400)
        );
        assert_eq!(
            parse!(year : 2014, ordinal : 365, hour_div_12 : 0, hour_mod_12 : 1, minute :
            42, second : 4, nanosecond : 12_345_678, offset : - 9876), ymdhmsn(2014, 12,
            31, 1, 42, 4, 12_345_678, - 9876)
        );
        assert_eq!(
            parse!(year : 2015, ordinal : 1, hour_div_12 : 0, hour_mod_12 : 4, minute :
            26, second : 40, nanosecond : 12_345_678, offset : 86_400), Err(OUT_OF_RANGE)
        );
    }
    #[test]
    fn test_parsed_to_datetime_with_timezone() {
        macro_rules! parse {
            ($tz:expr; $($k:ident : $v:expr),*) => {
                Parsed { $($k : Some($v),)* ..Parsed::new() }
                .to_datetime_with_timezone(&$tz)
            };
        }
        assert_eq!(
            parse!(Utc; year : 2014, ordinal : 365, hour_div_12 : 0, hour_mod_12 : 4,
            minute : 26, second : 40, nanosecond : 12_345_678, offset : 0), Ok(Utc
            .from_local_datetime(& NaiveDate::from_ymd_opt(2014, 12, 31).unwrap()
            .and_hms_nano_opt(4, 26, 40, 12_345_678).unwrap()).unwrap())
        );
        assert_eq!(
            parse!(Utc; year : 2014, ordinal : 365, hour_div_12 : 1, hour_mod_12 : 1,
            minute : 26, second : 40, nanosecond : 12_345_678, offset : 32400),
            Err(IMPOSSIBLE)
        );
        assert_eq!(
            parse!(FixedOffset::east_opt(32400).unwrap(); year : 2014, ordinal : 365,
            hour_div_12 : 0, hour_mod_12 : 4, minute : 26, second : 40, nanosecond :
            12_345_678, offset : 0), Err(IMPOSSIBLE)
        );
        assert_eq!(
            parse!(FixedOffset::east_opt(32400).unwrap(); year : 2014, ordinal : 365,
            hour_div_12 : 1, hour_mod_12 : 1, minute : 26, second : 40, nanosecond :
            12_345_678, offset : 32400), Ok(FixedOffset::east_opt(32400).unwrap()
            .from_local_datetime(& NaiveDate::from_ymd_opt(2014, 12, 31).unwrap()
            .and_hms_nano_opt(13, 26, 40, 12_345_678).unwrap()).unwrap())
        );
        assert_eq!(
            parse!(Utc; timestamp : 1_420_000_000, offset : 0), Ok(Utc
            .with_ymd_and_hms(2014, 12, 31, 4, 26, 40).unwrap())
        );
        assert_eq!(
            parse!(Utc; timestamp : 1_420_000_000, offset : 32400), Err(IMPOSSIBLE)
        );
        assert_eq!(
            parse!(FixedOffset::east_opt(32400).unwrap(); timestamp : 1_420_000_000,
            offset : 0), Err(IMPOSSIBLE)
        );
        assert_eq!(
            parse!(FixedOffset::east_opt(32400).unwrap(); timestamp : 1_420_000_000,
            offset : 32400), Ok(FixedOffset::east_opt(32400).unwrap()
            .with_ymd_and_hms(2014, 12, 31, 13, 26, 40).unwrap())
        );
    }
    #[test]
    fn issue_551() {
        use crate::Weekday;
        let mut parsed = Parsed::new();
        parsed.year = Some(2002);
        parsed.week_from_mon = Some(22);
        parsed.weekday = Some(Weekday::Mon);
        assert_eq!(
            NaiveDate::from_ymd_opt(2002, 6, 3).unwrap(), parsed.to_naive_date().unwrap()
        );
        parsed.year = Some(2001);
        assert_eq!(
            NaiveDate::from_ymd_opt(2001, 5, 28).unwrap(), parsed.to_naive_date()
            .unwrap()
        );
    }
}
#[cfg(test)]
mod tests_llm_16_291 {
    use super::*;
    use crate::*;
    #[test]
    fn test_new() {
        let _rug_st_tests_llm_16_291_rrrruuuugggg_test_new = 0;
        let parsed = Parsed::new();
        debug_assert_eq!(parsed, Parsed::default());
        let _rug_ed_tests_llm_16_291_rrrruuuugggg_test_new = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_292 {
    use super::*;
    use crate::*;
    use crate::format::parsed::{ParseResult, Parsed};
    #[test]
    fn test_set_ampm() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(bool, bool, u32, bool, u32, bool, u32, bool, u32, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        debug_assert_eq!(parsed.set_ampm(rug_fuzz_0), Ok(()));
        debug_assert_eq!(parsed.hour_div_12, Some(0));
        let mut parsed = Parsed::new();
        debug_assert_eq!(parsed.set_ampm(rug_fuzz_1), Ok(()));
        debug_assert_eq!(parsed.hour_div_12, Some(1));
        let mut parsed = Parsed::new();
        parsed.hour_div_12 = Some(rug_fuzz_2);
        debug_assert_eq!(parsed.set_ampm(rug_fuzz_3), Ok(()));
        debug_assert_eq!(parsed.hour_div_12, Some(0));
        let mut parsed = Parsed::new();
        parsed.hour_div_12 = Some(rug_fuzz_4);
        debug_assert_eq!(parsed.set_ampm(rug_fuzz_5), Ok(()));
        debug_assert_eq!(parsed.hour_div_12, Some(1));
        let mut parsed = Parsed::new();
        parsed.hour_div_12 = Some(rug_fuzz_6);
        debug_assert!(parsed.set_ampm(rug_fuzz_7).is_err());
        let mut parsed = Parsed::new();
        parsed.hour_div_12 = Some(rug_fuzz_8);
        debug_assert!(parsed.set_ampm(rug_fuzz_9).is_err());
             }
}
}
}    }
}
#[cfg(test)]
mod set_day_tests {
    use crate::format::parsed::Parsed;
    use crate::format::ParseResult;
    #[test]
    fn set_day_within_valid_range() -> ParseResult<()> {
        let mut parsed = Parsed::new();
        parsed.set_day(1)?;
        assert_eq!(parsed.day, Some(1));
        parsed.set_day(31)?;
        assert_eq!(parsed.day, Some(31));
        Ok(())
    }
    #[test]
    fn set_day_invalid_negative() -> ParseResult<()> {
        let mut parsed = Parsed::new();
        assert!(parsed.set_day(- 1).is_err());
        assert_eq!(parsed.day, None);
        Ok(())
    }
    #[test]
    fn set_day_invalid_overflow() -> ParseResult<()> {
        let mut parsed = Parsed::new();
        assert!(parsed.set_day(1_000_000_000).is_err());
        assert_eq!(parsed.day, None);
        Ok(())
    }
    #[test]
    fn set_day_already_set_consistent() -> ParseResult<()> {
        let mut parsed = Parsed::new();
        parsed.set_day(15)?;
        parsed.set_day(15)?;
        assert_eq!(parsed.day, Some(15));
        Ok(())
    }
    #[test]
    fn set_day_already_set_inconsistent() -> ParseResult<()> {
        let mut parsed = Parsed::new();
        parsed.set_day(15)?;
        assert!(parsed.set_day(16).is_err());
        assert_eq!(parsed.day, Some(15));
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_294_llm_16_294 {
    use super::*;
    use crate::*;
    use crate::format::parsed::{Parsed, ParseResult};
    use crate::format::ParseErrorKind::{Impossible, OutOfRange};
    #[test]
    fn test_set_hour_valid_values() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i64, i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        debug_assert!(parsed.set_hour(rug_fuzz_0).is_ok());
        debug_assert_eq!(parsed.hour_div_12, Some(0));
        debug_assert_eq!(parsed.hour_mod_12, Some(0));
        let mut parsed = Parsed::new();
        debug_assert!(parsed.set_hour(rug_fuzz_1).is_ok());
        debug_assert_eq!(parsed.hour_div_12, Some(0));
        debug_assert_eq!(parsed.hour_mod_12, Some(11));
        let mut parsed = Parsed::new();
        debug_assert!(parsed.set_hour(rug_fuzz_2).is_ok());
        debug_assert_eq!(parsed.hour_div_12, Some(1));
        debug_assert_eq!(parsed.hour_mod_12, Some(0));
        let mut parsed = Parsed::new();
        debug_assert!(parsed.set_hour(rug_fuzz_3).is_ok());
        debug_assert_eq!(parsed.hour_div_12, Some(1));
        debug_assert_eq!(parsed.hour_mod_12, Some(11));
             }
}
}
}    }
    #[test]
    fn test_set_hour_invalid_values() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        debug_assert_eq!(parsed.set_hour(- rug_fuzz_0).unwrap_err().kind(), OutOfRange);
        let mut parsed = Parsed::new();
        debug_assert_eq!(parsed.set_hour(rug_fuzz_1).unwrap_err().kind(), OutOfRange);
        let mut parsed = Parsed::new();
        debug_assert_eq!(parsed.set_hour(i64::MAX).unwrap_err().kind(), OutOfRange);
        let mut parsed = Parsed::new();
        debug_assert_eq!(parsed.set_hour(i64::MIN).unwrap_err().kind(), OutOfRange);
             }
}
}
}    }
    #[test]
    fn test_set_hour_inconsistent_state() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u32, i64, u32, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        parsed.hour_div_12 = Some(rug_fuzz_0);
        debug_assert_eq!(parsed.set_hour(rug_fuzz_1).unwrap_err().kind(), Impossible);
        let mut parsed = Parsed::new();
        parsed.hour_mod_12 = Some(rug_fuzz_2);
        debug_assert_eq!(parsed.set_hour(rug_fuzz_3).unwrap_err().kind(), Impossible);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_295 {
    use super::*;
    use crate::*;
    use crate::format::ParseResult;
    use std::result::Result::{Err, Ok};
    fn new_parsed() -> Parsed {
        Parsed::new()
    }
    #[test]
    fn test_set_hour12() {
        let mut parsed = new_parsed();
        for hour in 1..=12 {
            assert_eq!(parsed.set_hour12(hour), Ok(()));
            assert_eq!(parsed.hour_mod_12, Some(hour as u32 % 12));
        }
        for hour in [0, 13, 24, -1, -12].iter() {
            assert_eq!(parsed.set_hour12(* hour), Err(OUT_OF_RANGE));
        }
    }
}
#[cfg(test)]
mod tests_llm_16_296 {
    use super::*;
    use crate::*;
    use crate::format::parsed::Parsed;
    use crate::format::ParseResult;
    use std::convert::TryFrom;
    #[test]
    fn test_set_isoweek_valid() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        debug_assert_eq!(parsed.set_isoweek(rug_fuzz_0), Ok(()));
        debug_assert_eq!(parsed.isoweek, Some(1));
             }
}
}
}    }
    #[test]
    fn test_set_isoweek_invalid() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        debug_assert!(parsed.set_isoweek(- rug_fuzz_0).is_err());
        debug_assert_eq!(parsed.isoweek, None);
             }
}
}
}    }
    #[test]
    fn test_set_isoweek_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        let result = parsed.set_isoweek(i64::from(u32::MAX) + rug_fuzz_0);
        debug_assert!(result.is_err());
        debug_assert_eq!(parsed.isoweek, None);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_297_llm_16_297 {
    use crate::format::parsed::Parsed;
    use crate::format::ParseResult;
    use crate::format::{ParseError, ParseErrorKind::{OutOfRange, Impossible}};
    #[test]
    fn test_set_isoyear_valid_values() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        debug_assert_eq!(parsed.set_isoyear(rug_fuzz_0), Ok(()));
        debug_assert_eq!(parsed.isoyear, Some(2019));
        debug_assert_eq!(parsed.set_isoyear(- rug_fuzz_1), Ok(()));
        debug_assert_eq!(parsed.isoyear, Some(- 50));
        debug_assert_eq!(parsed.set_isoyear(rug_fuzz_2), Ok(()));
        debug_assert_eq!(parsed.isoyear, Some(0));
             }
}
}
}    }
    #[test]
    fn test_set_isoyear_invalid_values() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        debug_assert_eq!(
            parsed.set_isoyear(i64::from(i32::MAX) + rug_fuzz_0),
            Err(ParseError(OutOfRange))
        );
        debug_assert_eq!(parsed.isoyear, None);
        debug_assert_eq!(
            parsed.set_isoyear(i64::from(i32::MIN) - rug_fuzz_1),
            Err(ParseError(OutOfRange))
        );
        debug_assert_eq!(parsed.isoyear, None);
             }
}
}
}    }
    #[test]
    fn test_set_isoyear_consistency() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        debug_assert_eq!(parsed.set_isoyear(rug_fuzz_0), Ok(()));
        debug_assert_eq!(parsed.set_isoyear(rug_fuzz_1), Ok(()));
        debug_assert_eq!(parsed.set_isoyear(rug_fuzz_2), Err(ParseError(Impossible)));
        debug_assert_eq!(parsed.isoyear, Some(2020));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_300 {
    use super::*;
    use crate::*;
    use crate::format::parsed::Parsed;
    use crate::format::ParseResult;
    #[test]
    fn test_set_minute_valid() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        debug_assert!(parsed.set_minute(rug_fuzz_0).is_ok());
        debug_assert_eq!(parsed.minute, Some(30));
             }
}
}
}    }
    #[test]
    fn test_set_minute_invalid_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        debug_assert!(parsed.set_minute(- rug_fuzz_0).is_err());
        debug_assert_eq!(parsed.minute, None);
             }
}
}
}    }
    #[test]
    fn test_set_minute_invalid_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        debug_assert!(parsed.set_minute(rug_fuzz_0).is_err());
        debug_assert_eq!(parsed.minute, None);
             }
}
}
}    }
    #[test]
    fn test_set_minute_already_set() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        debug_assert!(parsed.set_minute(rug_fuzz_0).is_ok());
        debug_assert!(parsed.set_minute(rug_fuzz_1).is_err());
        debug_assert_eq!(parsed.minute, Some(25));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_301 {
    use super::*;
    use crate::*;
    use crate::format::parsed::Parsed;
    use crate::format::ParseResult;
    #[test]
    fn test_set_month_valid() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        debug_assert_eq!(parsed.set_month(rug_fuzz_0), Ok(()));
        debug_assert_eq!(parsed.month, Some(1));
        debug_assert_eq!(parsed.set_month(rug_fuzz_1), Ok(()));
        debug_assert_eq!(parsed.month, Some(12));
             }
}
}
}    }
    #[test]
    fn test_set_month_invalid() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        debug_assert!(parsed.set_month(rug_fuzz_0).is_err());
        debug_assert_eq!(parsed.month, None);
        debug_assert!(parsed.set_month(rug_fuzz_1).is_err());
        debug_assert_eq!(parsed.month, None);
             }
}
}
}    }
    #[test]
    fn test_set_month_consistency() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u32, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        parsed.month = Some(rug_fuzz_0);
        debug_assert!(parsed.set_month(rug_fuzz_1).is_err());
        debug_assert_eq!(parsed.month, Some(5));
        debug_assert!(parsed.set_month(rug_fuzz_2).is_err());
        debug_assert_eq!(parsed.month, Some(5));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_302_llm_16_302 {
    use super::*;
    use crate::*;
    use crate::format::parsed::Parsed;
    use crate::format::ParseError;
    use crate::format::ParseErrorKind::OutOfRange;
    use std::convert::TryFrom;
    #[test]
    fn set_nanosecond_within_range() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        debug_assert_eq!(parsed.set_nanosecond(rug_fuzz_0), Ok(()));
        debug_assert_eq!(parsed.nanosecond, Some(999_999_999));
             }
}
}
}    }
    #[test]
    fn set_nanosecond_below_range() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        debug_assert_eq!(
            parsed.set_nanosecond(- rug_fuzz_0), Err(ParseError(OutOfRange))
        );
        debug_assert_eq!(parsed.nanosecond, None);
             }
}
}
}    }
    #[test]
    fn set_nanosecond_above_range() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        debug_assert_eq!(parsed.set_nanosecond(rug_fuzz_0), Err(ParseError(OutOfRange)));
        debug_assert_eq!(parsed.nanosecond, None);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_303 {
    use super::*;
    use crate::*;
    use crate::format::parsed::{Parsed, ParseResult};
    #[test]
    fn test_set_offset_within_range() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        let res = parsed.set_offset(rug_fuzz_0);
        debug_assert!(res.is_ok());
        debug_assert_eq!(parsed.offset, Some(3600));
             }
}
}
}    }
    #[test]
    fn test_set_offset_out_of_range() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        let res = parsed.set_offset(i64::from(i32::MAX) + rug_fuzz_0);
        debug_assert!(res.is_err());
        debug_assert_eq!(parsed.offset, None);
             }
}
}
}    }
    #[test]
    fn test_set_offset_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        let res = parsed.set_offset(-rug_fuzz_0);
        debug_assert!(res.is_ok());
        debug_assert_eq!(parsed.offset, Some(- 3600));
             }
}
}
}    }
    #[test]
    fn test_set_offset_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        let res = parsed.set_offset(rug_fuzz_0);
        debug_assert!(res.is_ok());
        debug_assert_eq!(parsed.offset, Some(0));
             }
}
}
}    }
    #[test]
    fn test_set_offset_edge_cases() {
        let _rug_st_tests_llm_16_303_rrrruuuugggg_test_set_offset_edge_cases = 0;
        let mut parsed = Parsed::new();
        let res = parsed.set_offset(i64::from(i32::MIN));
        debug_assert!(res.is_ok());
        debug_assert_eq!(parsed.offset, Some(i32::MIN));
        let res = parsed.set_offset(i64::from(i32::MAX));
        debug_assert!(res.is_ok());
        debug_assert_eq!(parsed.offset, Some(i32::MAX));
        let _rug_ed_tests_llm_16_303_rrrruuuugggg_test_set_offset_edge_cases = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_305 {
    use super::*;
    use crate::*;
    use crate::format::ParseError;
    use crate::format::ParseResult;
    #[test]
    fn test_set_second_valid() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        debug_assert!(parsed.set_second(rug_fuzz_0).is_ok());
        debug_assert_eq!(parsed.second, Some(59));
             }
}
}
}    }
    #[test]
    fn test_set_second_invalid_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        let result = parsed.set_second(-rug_fuzz_0);
        debug_assert!(result.is_err());
        debug_assert_eq!(parsed.second, None);
             }
}
}
}    }
    #[test]
    fn test_set_second_invalid_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        let result = parsed.set_second(rug_fuzz_0);
        debug_assert!(result.is_err());
        debug_assert_eq!(parsed.second, None);
             }
}
}
}    }
    #[test]
    fn test_set_second_on_the_edge() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        debug_assert!(parsed.set_second(rug_fuzz_0).is_ok());
        debug_assert_eq!(parsed.second, Some(60));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_306 {
    use crate::format::parsed::Parsed;
    use crate::format::ParseResult;
    #[test]
    fn set_timestamp_with_none_existing() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        debug_assert!(parsed.timestamp.is_none());
        let result = parsed.set_timestamp(rug_fuzz_0);
        debug_assert!(result.is_ok());
        debug_assert_eq!(parsed.timestamp, Some(1611700024));
             }
}
}
}    }
    #[test]
    fn set_timestamp_with_existing_consistent() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        parsed.timestamp = Some(rug_fuzz_0);
        let result = parsed.set_timestamp(rug_fuzz_1);
        debug_assert!(result.is_ok());
        debug_assert_eq!(parsed.timestamp, Some(1611700024));
             }
}
}
}    }
    #[test]
    fn set_timestamp_with_existing_inconsistent() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        parsed.timestamp = Some(rug_fuzz_0);
        let result = parsed.set_timestamp(rug_fuzz_1);
        debug_assert!(result.is_err());
        debug_assert_eq!(parsed.timestamp, Some(1611700024));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_307_llm_16_307 {
    use super::*;
    use crate::*;
    use crate::format::parsed::Parsed;
    use crate::format::ParseErrorKind::*;
    use crate::format::ParseError;
    #[test]
    fn test_set_week_from_mon_within_range() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        debug_assert_eq!(parsed.set_week_from_mon(rug_fuzz_0), Ok(()));
        debug_assert_eq!(parsed.week_from_mon, Some(1));
        debug_assert_eq!(parsed.set_week_from_mon(rug_fuzz_1), Ok(()));
        debug_assert_eq!(parsed.week_from_mon, Some(53));
             }
}
}
}    }
    #[test]
    fn test_set_week_from_mon_below_range() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        debug_assert_eq!(
            parsed.set_week_from_mon(rug_fuzz_0), Err(ParseError(OutOfRange))
        );
             }
}
}
}    }
    #[test]
    fn test_set_week_from_mon_above_range() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        debug_assert_eq!(
            parsed.set_week_from_mon(i64::from(u32::MAX) + rug_fuzz_0),
            Err(ParseError(OutOfRange))
        );
             }
}
}
}    }
    #[test]
    fn test_set_week_from_mon_negative_value() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        debug_assert_eq!(
            parsed.set_week_from_mon(- rug_fuzz_0), Err(ParseError(OutOfRange))
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_308_llm_16_308 {
    use super::*;
    use crate::*;
    use crate::format::ParseError;
    use crate::format::ParseErrorKind::{Impossible, OutOfRange};
    #[test]
    fn test_set_week_from_sun_with_valid_value() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        debug_assert!(parsed.set_week_from_sun(rug_fuzz_0).is_ok());
        debug_assert_eq!(parsed.week_from_sun, Some(32));
             }
}
}
}    }
    #[test]
    fn test_set_week_from_sun_with_value_too_large() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        let result = parsed.set_week_from_sun(rug_fuzz_0);
        debug_assert_eq!(result, Err(ParseError(OutOfRange)));
        debug_assert!(parsed.week_from_sun.is_none());
             }
}
}
}    }
    #[test]
    fn test_set_week_from_sun_with_value_too_small() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        let result = parsed.set_week_from_sun(rug_fuzz_0);
        debug_assert_eq!(result, Err(ParseError(OutOfRange)));
        debug_assert!(parsed.week_from_sun.is_none());
             }
}
}
}    }
    #[test]
    fn test_set_week_from_sun_with_negative_value() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        let result = parsed.set_week_from_sun(-rug_fuzz_0);
        debug_assert_eq!(result, Err(ParseError(OutOfRange)));
        debug_assert!(parsed.week_from_sun.is_none());
             }
}
}
}    }
    #[test]
    fn test_set_week_from_sun_with_inconsistent_value() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        parsed.set_week_from_sun(rug_fuzz_0).unwrap();
        let result = parsed.set_week_from_sun(rug_fuzz_1);
        debug_assert_eq!(result, Err(ParseError(Impossible)));
        debug_assert_eq!(parsed.week_from_sun, Some(10));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_309 {
    use super::*;
    use crate::*;
    use crate::Weekday;
    #[test]
    fn test_set_weekday() {
        let _rug_st_tests_llm_16_309_rrrruuuugggg_test_set_weekday = 0;
        let mut parsed = Parsed::new();
        let set_result = parsed.set_weekday(Weekday::Wed);
        debug_assert!(set_result.is_ok());
        debug_assert_eq!(parsed.weekday, Some(Weekday::Wed));
        let set_result = parsed.set_weekday(Weekday::Sun);
        debug_assert!(set_result.is_ok());
        debug_assert_eq!(parsed.weekday, Some(Weekday::Sun));
        let _rug_ed_tests_llm_16_309_rrrruuuugggg_test_set_weekday = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_310_llm_16_310 {
    use crate::format::parsed::Parsed;
    use crate::format::ParseErrorKind::{Impossible, OutOfRange};
    use crate::format::{ParseError, ParseResult};
    #[test]
    fn test_set_year_valid() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        debug_assert_eq!(parsed.set_year(rug_fuzz_0), Ok(()));
        debug_assert_eq!(parsed.year, Some(2023));
             }
}
}
}    }
    #[test]
    fn test_set_year_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        debug_assert_eq!(
            parsed.set_year(i64::from(i32::MAX) + rug_fuzz_0),
            Err(ParseError(OutOfRange))
        );
             }
}
}
}    }
    #[test]
    fn test_set_year_underflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        debug_assert_eq!(
            parsed.set_year(i64::from(i32::MIN) - rug_fuzz_0),
            Err(ParseError(OutOfRange))
        );
             }
}
}
}    }
    #[test]
    fn test_set_year_previous_value() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        parsed.year = Some(rug_fuzz_0);
        debug_assert_eq!(parsed.set_year(rug_fuzz_1), Ok(()));
        debug_assert_eq!(parsed.year, Some(1980));
             }
}
}
}    }
    #[test]
    fn test_set_year_inconsistent() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        parsed.year = Some(rug_fuzz_0);
        debug_assert_eq!(parsed.set_year(rug_fuzz_1), Err(ParseError(Impossible)));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_312 {
    use super::*;
    use crate::*;
    use crate::format::parsed::{Parsed, OUT_OF_RANGE};
    use crate::format::ParseResult;
    #[test]
    fn set_year_mod_100_within_range() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        debug_assert_eq!(parsed.set_year_mod_100(rug_fuzz_0), Ok(()));
        debug_assert_eq!(parsed.year_mod_100, Some(99));
             }
}
}
}    }
    #[test]
    fn set_year_mod_100_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        debug_assert_eq!(parsed.set_year_mod_100(- rug_fuzz_0), Err(OUT_OF_RANGE));
        debug_assert_eq!(parsed.year_mod_100, None);
             }
}
}
}    }
    #[test]
    fn set_year_mod_100_overflow() {
        let _rug_st_tests_llm_16_312_rrrruuuugggg_set_year_mod_100_overflow = 0;
        let mut parsed = Parsed::new();
        debug_assert_eq!(parsed.set_year_mod_100(i64::max_value()), Err(OUT_OF_RANGE));
        debug_assert_eq!(parsed.year_mod_100, None);
        let _rug_ed_tests_llm_16_312_rrrruuuugggg_set_year_mod_100_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_313 {
    use super::*;
    use crate::*;
    use crate::offset::{FixedOffset, TimeZone};
    use crate::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};
    use format::parsed::Parsed;
    use std::fmt::Write;
    #[test]
    fn test_to_datetime() {
        let _rug_st_tests_llm_16_313_rrrruuuugggg_test_to_datetime = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 14;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 58;
        let rug_fuzz_5 = 53;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 0;
        let rug_fuzz_9 = 2023;
        let rug_fuzz_10 = 3;
        let rug_fuzz_11 = 14;
        let rug_fuzz_12 = 1;
        let rug_fuzz_13 = 58;
        let rug_fuzz_14 = 53;
        let rug_fuzz_15 = 0;
        let rug_fuzz_16 = 2023;
        let rug_fuzz_17 = 3;
        let rug_fuzz_18 = 14;
        let rug_fuzz_19 = 1;
        let rug_fuzz_20 = 58;
        let rug_fuzz_21 = 53;
        let rug_fuzz_22 = 0;
        let rug_fuzz_23 = 3600;
        let rug_fuzz_24 = 1998;
        let rug_fuzz_25 = 12;
        let rug_fuzz_26 = 31;
        let rug_fuzz_27 = 23;
        let rug_fuzz_28 = 59;
        let rug_fuzz_29 = 60;
        let rug_fuzz_30 = 0;
        let rug_fuzz_31 = 0;
        let rug_fuzz_32 = 1998;
        let rug_fuzz_33 = 12;
        let rug_fuzz_34 = 31;
        let rug_fuzz_35 = 23;
        let rug_fuzz_36 = 59;
        let rug_fuzz_37 = 59;
        let rug_fuzz_38 = 1_000_000_000;
        let mut parsed = Parsed::new();
        parsed.set_year(rug_fuzz_0).unwrap();
        parsed.set_month(rug_fuzz_1).unwrap();
        parsed.set_day(rug_fuzz_2).unwrap();
        parsed.set_hour(rug_fuzz_3).unwrap();
        parsed.set_minute(rug_fuzz_4).unwrap();
        parsed.set_second(rug_fuzz_5).unwrap();
        parsed.set_nanosecond(rug_fuzz_6).unwrap();
        parsed.set_offset(rug_fuzz_7).unwrap();
        let expected = FixedOffset::east(rug_fuzz_8)
            .ymd(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11)
            .and_hms_nano(rug_fuzz_12, rug_fuzz_13, rug_fuzz_14, rug_fuzz_15);
        debug_assert_eq!(parsed.to_datetime().unwrap(), expected);
        let mut parsed = Parsed::new();
        parsed.set_year(rug_fuzz_16).unwrap();
        parsed.set_month(rug_fuzz_17).unwrap();
        parsed.set_day(rug_fuzz_18).unwrap();
        parsed.set_hour(rug_fuzz_19).unwrap();
        parsed.set_minute(rug_fuzz_20).unwrap();
        parsed.set_second(rug_fuzz_21).unwrap();
        parsed.set_nanosecond(rug_fuzz_22).unwrap();
        parsed.set_offset(rug_fuzz_23).unwrap();
        debug_assert!(parsed.to_datetime().is_err());
        let parsed = Parsed::new();
        debug_assert!(parsed.to_datetime().is_err());
        let mut parsed = Parsed::new();
        parsed.set_year(rug_fuzz_24).unwrap();
        parsed.set_month(rug_fuzz_25).unwrap();
        parsed.set_day(rug_fuzz_26).unwrap();
        parsed.set_hour(rug_fuzz_27).unwrap();
        parsed.set_minute(rug_fuzz_28).unwrap();
        parsed.set_second(rug_fuzz_29).unwrap();
        parsed.set_offset(rug_fuzz_30).unwrap();
        let expected = FixedOffset::east(rug_fuzz_31)
            .ymd(rug_fuzz_32, rug_fuzz_33, rug_fuzz_34)
            .and_hms_nano(rug_fuzz_35, rug_fuzz_36, rug_fuzz_37, rug_fuzz_38);
        debug_assert_eq!(parsed.to_datetime().unwrap(), expected);
        let _rug_ed_tests_llm_16_313_rrrruuuugggg_test_to_datetime = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_315 {
    use crate::{FixedOffset, format::parsed::Parsed, format::ParseResult};
    use crate::offset::TimeZone;
    #[test]
    fn test_to_fixed_offset_none() {
        let _rug_st_tests_llm_16_315_rrrruuuugggg_test_to_fixed_offset_none = 0;
        let parsed = Parsed {
            offset: None,
            ..Parsed::new()
        };
        debug_assert!(parsed.to_fixed_offset().is_err());
        let _rug_ed_tests_llm_16_315_rrrruuuugggg_test_to_fixed_offset_none = 0;
    }
    #[test]
    fn test_to_fixed_offset_valid() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let parsed = Parsed {
            offset: Some(rug_fuzz_0),
            ..Parsed::new()
        };
        debug_assert_eq!(parsed.to_fixed_offset().unwrap(), FixedOffset::east(3600));
             }
}
}
}    }
    #[test]
    fn test_to_fixed_offset_invalid() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let parsed = Parsed {
            offset: Some(rug_fuzz_0),
            ..Parsed::new()
        };
        debug_assert!(parsed.to_fixed_offset().is_err());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_316 {
    use super::*;
    use crate::*;
    use crate::{NaiveDate, Weekday};
    #[test]
    fn test_to_naive_date_year_month_day() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u32, u32, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        parsed.year = Some(rug_fuzz_0);
        parsed.month = Some(rug_fuzz_1);
        parsed.day = Some(rug_fuzz_2);
        let date = parsed.to_naive_date().expect(rug_fuzz_3);
        debug_assert_eq!(date, NaiveDate::from_ymd(2023, 4, 5));
             }
}
}
}    }
    #[test]
    fn test_to_naive_date_year_ordinal() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        parsed.year = Some(rug_fuzz_0);
        parsed.ordinal = Some(rug_fuzz_1);
        let date = parsed.to_naive_date().expect(rug_fuzz_2);
        debug_assert_eq!(date, NaiveDate::from_yo(2023, 95));
             }
}
}
}    }
    #[test]
    fn test_to_naive_date_year_week_day() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i32, u32, &str, i32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        parsed.year = Some(rug_fuzz_0);
        parsed.week_from_sun = Some(rug_fuzz_1);
        parsed.weekday = Some(Weekday::Wed);
        let date = parsed.to_naive_date().expect(rug_fuzz_2);
        let expected_date = NaiveDate::from_yo(rug_fuzz_3, rug_fuzz_4)
            .succ_opt()
            .unwrap()
            .succ_opt()
            .unwrap()
            .succ_opt()
            .unwrap()
            .succ_opt()
            .unwrap()
            .succ_opt()
            .unwrap()
            .succ_opt()
            .unwrap()
            .succ_opt()
            .unwrap()
            .succ_opt()
            .unwrap()
            .succ_opt()
            .unwrap()
            .succ_opt()
            .unwrap()
            .succ_opt()
            .unwrap()
            .succ_opt()
            .unwrap()
            .succ_opt()
            .unwrap()
            .succ_opt()
            .unwrap()
            .succ_opt()
            .unwrap();
        debug_assert_eq!(date, expected_date);
             }
}
}
}    }
    #[test]
    fn test_to_naive_date_year_week_from_monday() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i32, u32, &str, i32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        parsed.year = Some(rug_fuzz_0);
        parsed.week_from_mon = Some(rug_fuzz_1);
        parsed.weekday = Some(Weekday::Mon);
        let date = parsed.to_naive_date().expect(rug_fuzz_2);
        let expected_date = NaiveDate::from_yo(rug_fuzz_3, rug_fuzz_4)
            .succ_opt()
            .unwrap()
            .succ_opt()
            .unwrap();
        debug_assert_eq!(date, expected_date);
             }
}
}
}    }
    #[test]
    fn test_to_naive_date_iso_year_week_weekday() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i32, u32, &str, i32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parsed = Parsed::new();
        parsed.isoyear = Some(rug_fuzz_0);
        parsed.isoweek = Some(rug_fuzz_1);
        parsed.weekday = Some(Weekday::Wed);
        let date = parsed.to_naive_date().expect(rug_fuzz_2);
        let expected_date = NaiveDate::from_isoywd(rug_fuzz_3, rug_fuzz_4, Weekday::Wed);
        debug_assert_eq!(date, expected_date);
             }
}
}
}    }
}
