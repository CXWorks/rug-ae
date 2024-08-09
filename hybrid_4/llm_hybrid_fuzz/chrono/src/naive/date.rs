//! ISO 8601 calendar date without timezone.
#[cfg(any(feature = "alloc", feature = "std", test))]
use core::borrow::Borrow;
use core::convert::TryFrom;
use core::ops::{Add, AddAssign, RangeInclusive, Sub, SubAssign};
use core::{fmt, str};
#[cfg(feature = "rkyv")]
use rkyv::{Archive, Deserialize, Serialize};
/// L10n locales.
#[cfg(feature = "unstable-locales")]
use pure_rust_locales::Locale;
#[cfg(any(feature = "alloc", feature = "std", test))]
use crate::format::DelayedFormat;
use crate::format::{
    parse, write_hundreds, ParseError, ParseResult, Parsed, StrftimeItems,
};
use crate::format::{Item, Numeric, Pad};
use crate::month::Months;
use crate::naive::{IsoWeek, NaiveDateTime, NaiveTime};
use crate::{Datelike, TimeDelta, Weekday};
use super::internals::{self, DateImpl, Mdf, Of, YearFlags};
use super::isoweek;
const MAX_YEAR: i32 = internals::MAX_YEAR;
const MIN_YEAR: i32 = internals::MIN_YEAR;
#[cfg(test)]
const MAX_DAYS_FROM_YEAR_0: i32 = MAX_YEAR * 365 + MAX_YEAR / 4 - MAX_YEAR / 100
    + MAX_YEAR / 400 + 365;
#[cfg(test)]
const MIN_DAYS_FROM_YEAR_0: i32 = (MIN_YEAR + 400_000) * 365 + (MIN_YEAR + 400_000) / 4
    - (MIN_YEAR + 400_000) / 100 + (MIN_YEAR + 400_000) / 400 - 146_097_000;
#[cfg(test)]
const MAX_BITS: usize = 44;
/// A week represented by a [`NaiveDate`] and a [`Weekday`] which is the first
/// day of the week.
#[derive(Debug)]
pub struct NaiveWeek {
    date: NaiveDate,
    start: Weekday,
}
impl NaiveWeek {
    /// Returns a date representing the first day of the week.
    ///
    /// # Examples
    ///
    /// ```
    /// use chrono::{NaiveDate, Weekday};
    ///
    /// let date = NaiveDate::from_ymd_opt(2022, 4, 18).unwrap();
    /// let week = date.week(Weekday::Mon);
    /// assert!(week.first_day() <= date);
    /// ```
    #[inline]
    #[must_use]
    pub fn first_day(&self) -> NaiveDate {
        let start = self.start.num_days_from_monday();
        let end = self.date.weekday().num_days_from_monday();
        let days = if start > end { 7 - start + end } else { end - start };
        self.date - TimeDelta::days(days.into())
    }
    /// Returns a date representing the last day of the week.
    ///
    /// # Examples
    ///
    /// ```
    /// use chrono::{NaiveDate, Weekday};
    ///
    /// let date = NaiveDate::from_ymd_opt(2022, 4, 18).unwrap();
    /// let week = date.week(Weekday::Mon);
    /// assert!(week.last_day() >= date);
    /// ```
    #[inline]
    #[must_use]
    pub fn last_day(&self) -> NaiveDate {
        self.first_day() + TimeDelta::days(6)
    }
    /// Returns a [`RangeInclusive<T>`] representing the whole week bounded by
    /// [first_day](./struct.NaiveWeek.html#method.first_day) and
    /// [last_day](./struct.NaiveWeek.html#method.last_day) functions.
    ///
    /// # Examples
    ///
    /// ```
    /// use chrono::{NaiveDate, Weekday};
    ///
    /// let date = NaiveDate::from_ymd_opt(2022, 4, 18).unwrap();
    /// let week = date.week(Weekday::Mon);
    /// let days = week.days();
    /// assert!(days.contains(&date));
    /// ```
    #[inline]
    #[must_use]
    pub fn days(&self) -> RangeInclusive<NaiveDate> {
        self.first_day()..=self.last_day()
    }
}
/// A duration in calendar days.
///
/// This is useful because when using `TimeDelta` it is possible
/// that adding `TimeDelta::days(1)` doesn't increment the day value as expected due to it being a
/// fixed number of seconds. This difference applies only when dealing with `DateTime<TimeZone>` data types
/// and in other cases `TimeDelta::days(n)` and `Days::new(n)` are equivalent.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub struct Days(pub(crate) u64);
impl Days {
    /// Construct a new `Days` from a number of days
    pub const fn new(num: u64) -> Self {
        Self(num)
    }
}
/// ISO 8601 calendar date without timezone.
/// Allows for every [proleptic Gregorian date] from Jan 1, 262145 BCE to Dec 31, 262143 CE.
/// Also supports the conversion from ISO 8601 ordinal and week date.
///
/// # Calendar Date
///
/// The ISO 8601 **calendar date** follows the proleptic Gregorian calendar.
/// It is like a normal civil calendar but note some slight differences:
///
/// * Dates before the Gregorian calendar's inception in 1582 are defined via the extrapolation.
///   Be careful, as historical dates are often noted in the Julian calendar and others
///   and the transition to Gregorian may differ across countries (as late as early 20C).
///
///   (Some example: Both Shakespeare from Britain and Cervantes from Spain seemingly died
///   on the same calendar date---April 23, 1616---but in the different calendar.
///   Britain used the Julian calendar at that time, so Shakespeare's death is later.)
///
/// * ISO 8601 calendars has the year 0, which is 1 BCE (a year before 1 CE).
///   If you need a typical BCE/BC and CE/AD notation for year numbers,
///   use the [`Datelike::year_ce`](../trait.Datelike.html#method.year_ce) method.
///
/// # Week Date
///
/// The ISO 8601 **week date** is a triple of year number, week number
/// and [day of the week](../enum.Weekday.html) with the following rules:
///
/// * A week consists of Monday through Sunday, and is always numbered within some year.
///   The week number ranges from 1 to 52 or 53 depending on the year.
///
/// * The week 1 of given year is defined as the first week containing January 4 of that year,
///   or equivalently, the first week containing four or more days in that year.
///
/// * The year number in the week date may *not* correspond to the actual Gregorian year.
///   For example, January 3, 2016 (Sunday) was on the last (53rd) week of 2015.
///
/// Chrono's date types default to the ISO 8601 [calendar date](#calendar-date),
/// but [`Datelike::iso_week`](../trait.Datelike.html#tymethod.iso_week) and
/// [`Datelike::weekday`](../trait.Datelike.html#tymethod.weekday) methods
/// can be used to get the corresponding week date.
///
/// # Ordinal Date
///
/// The ISO 8601 **ordinal date** is a pair of year number and day of the year ("ordinal").
/// The ordinal number ranges from 1 to 365 or 366 depending on the year.
/// The year number is the same as that of the [calendar date](#calendar-date).
///
/// This is currently the internal format of Chrono's date types.
///
/// [proleptic Gregorian date]: crate::NaiveDate#calendar-date
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Clone)]
#[cfg_attr(feature = "rkyv", derive(Archive, Deserialize, Serialize))]
pub struct NaiveDate {
    ymdf: DateImpl,
}
/// The minimum possible `NaiveDate` (January 1, 262145 BCE).
#[deprecated(since = "0.4.20", note = "Use NaiveDate::MIN instead")]
pub const MIN_DATE: NaiveDate = NaiveDate::MIN;
/// The maximum possible `NaiveDate` (December 31, 262143 CE).
#[deprecated(since = "0.4.20", note = "Use NaiveDate::MAX instead")]
pub const MAX_DATE: NaiveDate = NaiveDate::MAX;
#[cfg(feature = "arbitrary")]
impl arbitrary::Arbitrary<'_> for NaiveDate {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<NaiveDate> {
        let year = u.int_in_range(MIN_YEAR..=MAX_YEAR)?;
        let max_days = YearFlags::from_year(year).ndays();
        let ord = u.int_in_range(1..=max_days)?;
        NaiveDate::from_yo_opt(year, ord).ok_or(arbitrary::Error::IncorrectFormat)
    }
}
#[test]
fn test_date_bounds() {
    let calculated_min = NaiveDate::from_ymd_opt(MIN_YEAR, 1, 1).unwrap();
    let calculated_max = NaiveDate::from_ymd_opt(MAX_YEAR, 12, 31).unwrap();
    assert!(
        NaiveDate::MIN == calculated_min,
        "`NaiveDate::MIN` should have a year flag {:?}", calculated_min.of().flags()
    );
    assert!(
        NaiveDate::MAX == calculated_max,
        "`NaiveDate::MAX` should have a year flag {:?}", calculated_max.of().flags()
    );
    let maxsecs = NaiveDate::MAX.signed_duration_since(NaiveDate::MIN).num_seconds();
    let maxsecs = maxsecs + 86401;
    assert!(
        maxsecs < (1 << MAX_BITS),
        "The entire `NaiveDate` range somehow exceeds 2^{} seconds", MAX_BITS
    );
}
impl NaiveDate {
    pub(crate) fn weeks_from(&self, day: Weekday) -> i32 {
        (self.ordinal() as i32 - self.weekday().num_days_from(day) as i32 + 6) / 7
    }
    /// Makes a new `NaiveDate` from year and packed ordinal-flags, with a verification.
    fn from_of(year: i32, of: Of) -> Option<NaiveDate> {
        if (MIN_YEAR..=MAX_YEAR).contains(&year) && of.valid() {
            let Of(of) = of;
            Some(NaiveDate {
                ymdf: (year << 13) | (of as DateImpl),
            })
        } else {
            None
        }
    }
    /// Makes a new `NaiveDate` from year and packed month-day-flags, with a verification.
    fn from_mdf(year: i32, mdf: Mdf) -> Option<NaiveDate> {
        NaiveDate::from_of(year, mdf.to_of())
    }
    /// Makes a new `NaiveDate` from the [calendar date](#calendar-date)
    /// (year, month and day).
    ///
    /// Panics on the out-of-range date, invalid month and/or day.
    #[deprecated(since = "0.4.23", note = "use `from_ymd_opt()` instead")]
    #[must_use]
    pub fn from_ymd(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).expect("invalid or out-of-range date")
    }
    /// Makes a new `NaiveDate` from the [calendar date](#calendar-date)
    /// (year, month and day).
    ///
    /// Returns `None` on the out-of-range date, invalid month and/or day.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::NaiveDate;
    ///
    /// let from_ymd_opt = NaiveDate::from_ymd_opt;
    ///
    /// assert!(from_ymd_opt(2015, 3, 14).is_some());
    /// assert!(from_ymd_opt(2015, 0, 14).is_none());
    /// assert!(from_ymd_opt(2015, 2, 29).is_none());
    /// assert!(from_ymd_opt(-4, 2, 29).is_some()); // 5 BCE is a leap year
    /// assert!(from_ymd_opt(400000, 1, 1).is_none());
    /// assert!(from_ymd_opt(-400000, 1, 1).is_none());
    /// ```
    #[must_use]
    pub fn from_ymd_opt(year: i32, month: u32, day: u32) -> Option<NaiveDate> {
        let flags = YearFlags::from_year(year);
        NaiveDate::from_mdf(year, Mdf::new(month, day, flags)?)
    }
    /// Makes a new `NaiveDate` from the [ordinal date](#ordinal-date)
    /// (year and day of the year).
    ///
    /// Panics on the out-of-range date and/or invalid day of year.
    #[deprecated(since = "0.4.23", note = "use `from_yo_opt()` instead")]
    #[must_use]
    pub fn from_yo(year: i32, ordinal: u32) -> NaiveDate {
        NaiveDate::from_yo_opt(year, ordinal).expect("invalid or out-of-range date")
    }
    /// Makes a new `NaiveDate` from the [ordinal date](#ordinal-date)
    /// (year and day of the year).
    ///
    /// Returns `None` on the out-of-range date and/or invalid day of year.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::NaiveDate;
    ///
    /// let from_yo_opt = NaiveDate::from_yo_opt;
    ///
    /// assert!(from_yo_opt(2015, 100).is_some());
    /// assert!(from_yo_opt(2015, 0).is_none());
    /// assert!(from_yo_opt(2015, 365).is_some());
    /// assert!(from_yo_opt(2015, 366).is_none());
    /// assert!(from_yo_opt(-4, 366).is_some()); // 5 BCE is a leap year
    /// assert!(from_yo_opt(400000, 1).is_none());
    /// assert!(from_yo_opt(-400000, 1).is_none());
    /// ```
    #[must_use]
    pub fn from_yo_opt(year: i32, ordinal: u32) -> Option<NaiveDate> {
        let flags = YearFlags::from_year(year);
        NaiveDate::from_of(year, Of::new(ordinal, flags)?)
    }
    /// Makes a new `NaiveDate` from the [ISO week date](#week-date)
    /// (year, week number and day of the week).
    /// The resulting `NaiveDate` may have a different year from the input year.
    ///
    /// Panics on the out-of-range date and/or invalid week number.
    #[deprecated(since = "0.4.23", note = "use `from_isoywd_opt()` instead")]
    #[must_use]
    pub fn from_isoywd(year: i32, week: u32, weekday: Weekday) -> NaiveDate {
        NaiveDate::from_isoywd_opt(year, week, weekday)
            .expect("invalid or out-of-range date")
    }
    /// Makes a new `NaiveDate` from the [ISO week date](#week-date)
    /// (year, week number and day of the week).
    /// The resulting `NaiveDate` may have a different year from the input year.
    ///
    /// Returns `None` on the out-of-range date and/or invalid week number.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, Weekday};
    ///
    /// let from_ymd = NaiveDate::from_ymd;
    /// let from_isoywd_opt = NaiveDate::from_isoywd_opt;
    ///
    /// assert_eq!(from_isoywd_opt(2015, 0, Weekday::Sun), None);
    /// assert_eq!(from_isoywd_opt(2015, 10, Weekday::Sun), Some(from_ymd(2015, 3, 8)));
    /// assert_eq!(from_isoywd_opt(2015, 30, Weekday::Mon), Some(from_ymd(2015, 7, 20)));
    /// assert_eq!(from_isoywd_opt(2015, 60, Weekday::Mon), None);
    ///
    /// assert_eq!(from_isoywd_opt(400000, 10, Weekday::Fri), None);
    /// assert_eq!(from_isoywd_opt(-400000, 10, Weekday::Sat), None);
    /// ```
    ///
    /// The year number of ISO week date may differ from that of the calendar date.
    ///
    /// ```
    /// # use chrono::{NaiveDate, Weekday};
    /// # let from_ymd = NaiveDate::from_ymd;
    /// # let from_isoywd_opt = NaiveDate::from_isoywd_opt;
    /// //           Mo Tu We Th Fr Sa Su
    /// // 2014-W52  22 23 24 25 26 27 28    has 4+ days of new year,
    /// // 2015-W01  29 30 31  1  2  3  4 <- so this is the first week
    /// assert_eq!(from_isoywd_opt(2014, 52, Weekday::Sun), Some(from_ymd(2014, 12, 28)));
    /// assert_eq!(from_isoywd_opt(2014, 53, Weekday::Mon), None);
    /// assert_eq!(from_isoywd_opt(2015, 1, Weekday::Mon), Some(from_ymd(2014, 12, 29)));
    ///
    /// // 2015-W52  21 22 23 24 25 26 27    has 4+ days of old year,
    /// // 2015-W53  28 29 30 31  1  2  3 <- so this is the last week
    /// // 2016-W01   4  5  6  7  8  9 10
    /// assert_eq!(from_isoywd_opt(2015, 52, Weekday::Sun), Some(from_ymd(2015, 12, 27)));
    /// assert_eq!(from_isoywd_opt(2015, 53, Weekday::Sun), Some(from_ymd(2016, 1, 3)));
    /// assert_eq!(from_isoywd_opt(2015, 54, Weekday::Mon), None);
    /// assert_eq!(from_isoywd_opt(2016, 1, Weekday::Mon), Some(from_ymd(2016, 1, 4)));
    /// ```
    #[must_use]
    pub fn from_isoywd_opt(year: i32, week: u32, weekday: Weekday) -> Option<NaiveDate> {
        let flags = YearFlags::from_year(year);
        let nweeks = flags.nisoweeks();
        if 1 <= week && week <= nweeks {
            let weekord = week * 7 + weekday as u32;
            let delta = flags.isoweek_delta();
            if weekord <= delta {
                let prevflags = YearFlags::from_year(year - 1);
                NaiveDate::from_of(
                    year - 1,
                    Of::new(weekord + prevflags.ndays() - delta, prevflags)?,
                )
            } else {
                let ordinal = weekord - delta;
                let ndays = flags.ndays();
                if ordinal <= ndays {
                    NaiveDate::from_of(year, Of::new(ordinal, flags)?)
                } else {
                    let nextflags = YearFlags::from_year(year + 1);
                    NaiveDate::from_of(year + 1, Of::new(ordinal - ndays, nextflags)?)
                }
            }
        } else {
            None
        }
    }
    /// Makes a new `NaiveDate` from a day's number in the proleptic Gregorian calendar, with
    /// January 1, 1 being day 1.
    ///
    /// Panics if the date is out of range.
    #[deprecated(since = "0.4.23", note = "use `from_num_days_from_ce_opt()` instead")]
    #[inline]
    #[must_use]
    pub fn from_num_days_from_ce(days: i32) -> NaiveDate {
        NaiveDate::from_num_days_from_ce_opt(days).expect("out-of-range date")
    }
    /// Makes a new `NaiveDate` from a day's number in the proleptic Gregorian calendar, with
    /// January 1, 1 being day 1.
    ///
    /// Returns `None` if the date is out of range.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::NaiveDate;
    ///
    /// let from_ndays_opt = NaiveDate::from_num_days_from_ce_opt;
    /// let from_ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();
    ///
    /// assert_eq!(from_ndays_opt(730_000),      Some(from_ymd(1999, 9, 3)));
    /// assert_eq!(from_ndays_opt(1),            Some(from_ymd(1, 1, 1)));
    /// assert_eq!(from_ndays_opt(0),            Some(from_ymd(0, 12, 31)));
    /// assert_eq!(from_ndays_opt(-1),           Some(from_ymd(0, 12, 30)));
    /// assert_eq!(from_ndays_opt(100_000_000),  None);
    /// assert_eq!(from_ndays_opt(-100_000_000), None);
    /// ```
    #[must_use]
    pub fn from_num_days_from_ce_opt(days: i32) -> Option<NaiveDate> {
        let days = days.checked_add(365)?;
        let (year_div_400, cycle) = div_mod_floor(days, 146_097);
        let (year_mod_400, ordinal) = internals::cycle_to_yo(cycle as u32);
        let flags = YearFlags::from_year_mod_400(year_mod_400 as i32);
        NaiveDate::from_of(
            year_div_400 * 400 + year_mod_400 as i32,
            Of::new(ordinal, flags)?,
        )
    }
    /// Makes a new `NaiveDate` by counting the number of occurrences of a particular day-of-week
    /// since the beginning of the given month.  For instance, if you want the 2nd Friday of March
    /// 2017, you would use `NaiveDate::from_weekday_of_month(2017, 3, Weekday::Fri, 2)`.
    ///
    /// # Panics
    ///
    /// The resulting `NaiveDate` is guaranteed to be in `month`.  If `n` is larger than the number
    /// of `weekday` in `month` (eg. the 6th Friday of March 2017) then this function will panic.
    ///
    /// `n` is 1-indexed.  Passing `n=0` will cause a panic.
    #[deprecated(since = "0.4.23", note = "use `from_weekday_of_month_opt()` instead")]
    #[must_use]
    pub fn from_weekday_of_month(
        year: i32,
        month: u32,
        weekday: Weekday,
        n: u8,
    ) -> NaiveDate {
        NaiveDate::from_weekday_of_month_opt(year, month, weekday, n)
            .expect("out-of-range date")
    }
    /// Makes a new `NaiveDate` by counting the number of occurrences of a particular day-of-week
    /// since the beginning of the given month.  For instance, if you want the 2nd Friday of March
    /// 2017, you would use `NaiveDate::from_weekday_of_month(2017, 3, Weekday::Fri, 2)`.  `n` is 1-indexed.
    ///
    /// ```
    /// use chrono::{NaiveDate, Weekday};
    /// assert_eq!(NaiveDate::from_weekday_of_month_opt(2017, 3, Weekday::Fri, 2),
    ///            NaiveDate::from_ymd_opt(2017, 3, 10))
    /// ```
    ///
    /// Returns `None` if `n` out-of-range; ie. if `n` is larger than the number of `weekday` in
    /// `month` (eg. the 6th Friday of March 2017), or if `n == 0`.
    #[must_use]
    pub fn from_weekday_of_month_opt(
        year: i32,
        month: u32,
        weekday: Weekday,
        n: u8,
    ) -> Option<NaiveDate> {
        if n == 0 {
            return None;
        }
        let first = NaiveDate::from_ymd_opt(year, month, 1)?.weekday();
        let first_to_dow = (7 + weekday.number_from_monday()
            - first.number_from_monday()) % 7;
        let day = (u32::from(n) - 1) * 7 + first_to_dow + 1;
        NaiveDate::from_ymd_opt(year, month, day)
    }
    /// Parses a string with the specified format string and returns a new `NaiveDate`.
    /// See the [`format::strftime` module](../format/strftime/index.html)
    /// on the supported escape sequences.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::NaiveDate;
    ///
    /// let parse_from_str = NaiveDate::parse_from_str;
    ///
    /// assert_eq!(parse_from_str("2015-09-05", "%Y-%m-%d"),
    ///            Ok(NaiveDate::from_ymd_opt(2015, 9, 5).unwrap()));
    /// assert_eq!(parse_from_str("5sep2015", "%d%b%Y"),
    ///            Ok(NaiveDate::from_ymd_opt(2015, 9, 5).unwrap()));
    /// ```
    ///
    /// Time and offset is ignored for the purpose of parsing.
    ///
    /// ```
    /// # use chrono::NaiveDate;
    /// # let parse_from_str = NaiveDate::parse_from_str;
    /// assert_eq!(parse_from_str("2014-5-17T12:34:56+09:30", "%Y-%m-%dT%H:%M:%S%z"),
    ///            Ok(NaiveDate::from_ymd_opt(2014, 5, 17).unwrap()));
    /// ```
    ///
    /// Out-of-bound dates or insufficient fields are errors.
    ///
    /// ```
    /// # use chrono::NaiveDate;
    /// # let parse_from_str = NaiveDate::parse_from_str;
    /// assert!(parse_from_str("2015/9", "%Y/%m").is_err());
    /// assert!(parse_from_str("2015/9/31", "%Y/%m/%d").is_err());
    /// ```
    ///
    /// All parsed fields should be consistent to each other, otherwise it's an error.
    ///
    /// ```
    /// # use chrono::NaiveDate;
    /// # let parse_from_str = NaiveDate::parse_from_str;
    /// assert!(parse_from_str("Sat, 09 Aug 2013", "%a, %d %b %Y").is_err());
    /// ```
    pub fn parse_from_str(s: &str, fmt: &str) -> ParseResult<NaiveDate> {
        let mut parsed = Parsed::new();
        parse(&mut parsed, s, StrftimeItems::new(fmt))?;
        parsed.to_naive_date()
    }
    /// Add a duration in [`Months`] to the date
    ///
    /// If the day would be out of range for the resulting month, use the last day for that month.
    ///
    /// Returns `None` if the resulting date would be out of range.
    ///
    /// ```
    /// # use chrono::{NaiveDate, Months};
    /// assert_eq!(
    ///     NaiveDate::from_ymd_opt(2022, 2, 20).unwrap().checked_add_months(Months::new(6)),
    ///     Some(NaiveDate::from_ymd_opt(2022, 8, 20).unwrap())
    /// );
    /// assert_eq!(
    ///     NaiveDate::from_ymd_opt(2022, 7, 31).unwrap().checked_add_months(Months::new(2)),
    ///     Some(NaiveDate::from_ymd_opt(2022, 9, 30).unwrap())
    /// );
    /// ```
    #[must_use]
    pub fn checked_add_months(self, months: Months) -> Option<Self> {
        if months.0 == 0 {
            return Some(self);
        }
        match months.0 <= core::i32::MAX as u32 {
            true => self.diff_months(months.0 as i32),
            false => None,
        }
    }
    /// Subtract a duration in [`Months`] from the date
    ///
    /// If the day would be out of range for the resulting month, use the last day for that month.
    ///
    /// Returns `None` if the resulting date would be out of range.
    ///
    /// ```
    /// # use chrono::{NaiveDate, Months};
    /// assert_eq!(
    ///     NaiveDate::from_ymd_opt(2022, 2, 20).unwrap().checked_sub_months(Months::new(6)),
    ///     Some(NaiveDate::from_ymd_opt(2021, 8, 20).unwrap())
    /// );
    ///
    /// assert_eq!(
    ///     NaiveDate::from_ymd_opt(2014, 1, 1).unwrap()
    ///         .checked_sub_months(Months::new(core::i32::MAX as u32 + 1)),
    ///     None
    /// );
    /// ```
    #[must_use]
    pub fn checked_sub_months(self, months: Months) -> Option<Self> {
        if months.0 == 0 {
            return Some(self);
        }
        match months.0 <= 2_147_483_647 {
            true => self.diff_months(-(months.0 as i32)),
            false => None,
        }
    }
    fn diff_months(self, months: i32) -> Option<Self> {
        let (years, left) = ((months / 12), (months % 12));
        let year = if (years > 0 && years > (MAX_YEAR - self.year()))
            || (years < 0 && years < (MIN_YEAR - self.year()))
        {
            return None;
        } else {
            self.year() + years
        };
        let month = self.month() as i32 + left;
        let (year, month) = if month <= 0 {
            if year == MIN_YEAR {
                return None;
            }
            (year - 1, month + 12)
        } else if month > 12 {
            if year == MAX_YEAR {
                return None;
            }
            (year + 1, month - 12)
        } else {
            (year, month)
        };
        let flags = YearFlags::from_year(year);
        let feb_days = if flags.ndays() == 366 { 29 } else { 28 };
        let days = [31, feb_days, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let day = Ord::min(self.day(), days[(month - 1) as usize]);
        NaiveDate::from_mdf(year, Mdf::new(month as u32, day, flags)?)
    }
    /// Add a duration in [`Days`] to the date
    ///
    /// Returns `None` if the resulting date would be out of range.
    ///
    /// ```
    /// # use chrono::{NaiveDate, Days};
    /// assert_eq!(
    ///     NaiveDate::from_ymd_opt(2022, 2, 20).unwrap().checked_add_days(Days::new(9)),
    ///     Some(NaiveDate::from_ymd_opt(2022, 3, 1).unwrap())
    /// );
    /// assert_eq!(
    ///     NaiveDate::from_ymd_opt(2022, 7, 31).unwrap().checked_add_days(Days::new(2)),
    ///     Some(NaiveDate::from_ymd_opt(2022, 8, 2).unwrap())
    /// );
    /// assert_eq!(
    ///     NaiveDate::from_ymd_opt(2022, 7, 31).unwrap().checked_add_days(Days::new(1000000000000)),
    ///     None
    /// );
    /// ```
    #[must_use]
    pub fn checked_add_days(self, days: Days) -> Option<Self> {
        if days.0 == 0 {
            return Some(self);
        }
        i64::try_from(days.0).ok().and_then(|d| self.diff_days(d))
    }
    /// Subtract a duration in [`Days`] from the date
    ///
    /// Returns `None` if the resulting date would be out of range.
    ///
    /// ```
    /// # use chrono::{NaiveDate, Days};
    /// assert_eq!(
    ///     NaiveDate::from_ymd_opt(2022, 2, 20).unwrap().checked_sub_days(Days::new(6)),
    ///     Some(NaiveDate::from_ymd_opt(2022, 2, 14).unwrap())
    /// );
    /// assert_eq!(
    ///     NaiveDate::from_ymd_opt(2022, 2, 20).unwrap().checked_sub_days(Days::new(1000000000000)),
    ///     None
    /// );
    /// ```
    #[must_use]
    pub fn checked_sub_days(self, days: Days) -> Option<Self> {
        if days.0 == 0 {
            return Some(self);
        }
        i64::try_from(days.0).ok().and_then(|d| self.diff_days(-d))
    }
    fn diff_days(self, days: i64) -> Option<Self> {
        let secs = days.checked_mul(86400)?;
        if secs >= core::i64::MAX / 1000 || secs <= core::i64::MIN / 1000 {
            return None;
        }
        self.checked_add_signed(TimeDelta::seconds(secs))
    }
    /// Makes a new `NaiveDateTime` from the current date and given `NaiveTime`.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, NaiveTime, NaiveDateTime};
    ///
    /// let d = NaiveDate::from_ymd_opt(2015, 6, 3).unwrap();
    /// let t = NaiveTime::from_hms_milli_opt(12, 34, 56, 789).unwrap();
    ///
    /// let dt: NaiveDateTime = d.and_time(t);
    /// assert_eq!(dt.date(), d);
    /// assert_eq!(dt.time(), t);
    /// ```
    #[inline]
    #[must_use]
    pub const fn and_time(&self, time: NaiveTime) -> NaiveDateTime {
        NaiveDateTime::new(*self, time)
    }
    /// Makes a new `NaiveDateTime` from the current date, hour, minute and second.
    ///
    /// No [leap second](./struct.NaiveTime.html#leap-second-handling) is allowed here;
    /// use `NaiveDate::and_hms_*` methods with a subsecond parameter instead.
    ///
    /// Panics on invalid hour, minute and/or second.
    #[deprecated(since = "0.4.23", note = "use `and_hms_opt()` instead")]
    #[inline]
    #[must_use]
    pub fn and_hms(&self, hour: u32, min: u32, sec: u32) -> NaiveDateTime {
        self.and_hms_opt(hour, min, sec).expect("invalid time")
    }
    /// Makes a new `NaiveDateTime` from the current date, hour, minute and second.
    ///
    /// No [leap second](./struct.NaiveTime.html#leap-second-handling) is allowed here;
    /// use `NaiveDate::and_hms_*_opt` methods with a subsecond parameter instead.
    ///
    /// Returns `None` on invalid hour, minute and/or second.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::NaiveDate;
    ///
    /// let d = NaiveDate::from_ymd_opt(2015, 6, 3).unwrap();
    /// assert!(d.and_hms_opt(12, 34, 56).is_some());
    /// assert!(d.and_hms_opt(12, 34, 60).is_none()); // use `and_hms_milli_opt` instead
    /// assert!(d.and_hms_opt(12, 60, 56).is_none());
    /// assert!(d.and_hms_opt(24, 34, 56).is_none());
    /// ```
    #[inline]
    #[must_use]
    pub fn and_hms_opt(&self, hour: u32, min: u32, sec: u32) -> Option<NaiveDateTime> {
        NaiveTime::from_hms_opt(hour, min, sec).map(|time| self.and_time(time))
    }
    /// Makes a new `NaiveDateTime` from the current date, hour, minute, second and millisecond.
    ///
    /// The millisecond part can exceed 1,000
    /// in order to represent the [leap second](./struct.NaiveTime.html#leap-second-handling).
    ///
    /// Panics on invalid hour, minute, second and/or millisecond.
    #[deprecated(since = "0.4.23", note = "use `and_hms_milli_opt()` instead")]
    #[inline]
    #[must_use]
    pub fn and_hms_milli(
        &self,
        hour: u32,
        min: u32,
        sec: u32,
        milli: u32,
    ) -> NaiveDateTime {
        self.and_hms_milli_opt(hour, min, sec, milli).expect("invalid time")
    }
    /// Makes a new `NaiveDateTime` from the current date, hour, minute, second and millisecond.
    ///
    /// The millisecond part can exceed 1,000
    /// in order to represent the [leap second](./struct.NaiveTime.html#leap-second-handling).
    ///
    /// Returns `None` on invalid hour, minute, second and/or millisecond.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::NaiveDate;
    ///
    /// let d = NaiveDate::from_ymd_opt(2015, 6, 3).unwrap();
    /// assert!(d.and_hms_milli_opt(12, 34, 56,   789).is_some());
    /// assert!(d.and_hms_milli_opt(12, 34, 59, 1_789).is_some()); // leap second
    /// assert!(d.and_hms_milli_opt(12, 34, 59, 2_789).is_none());
    /// assert!(d.and_hms_milli_opt(12, 34, 60,   789).is_none());
    /// assert!(d.and_hms_milli_opt(12, 60, 56,   789).is_none());
    /// assert!(d.and_hms_milli_opt(24, 34, 56,   789).is_none());
    /// ```
    #[inline]
    #[must_use]
    pub fn and_hms_milli_opt(
        &self,
        hour: u32,
        min: u32,
        sec: u32,
        milli: u32,
    ) -> Option<NaiveDateTime> {
        NaiveTime::from_hms_milli_opt(hour, min, sec, milli)
            .map(|time| self.and_time(time))
    }
    /// Makes a new `NaiveDateTime` from the current date, hour, minute, second and microsecond.
    ///
    /// The microsecond part can exceed 1,000,000
    /// in order to represent the [leap second](./struct.NaiveTime.html#leap-second-handling).
    ///
    /// Panics on invalid hour, minute, second and/or microsecond.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, NaiveDateTime, Datelike, Timelike, Weekday};
    ///
    /// let d = NaiveDate::from_ymd_opt(2015, 6, 3).unwrap();
    ///
    /// let dt: NaiveDateTime = d.and_hms_micro(12, 34, 56, 789_012);
    /// assert_eq!(dt.year(), 2015);
    /// assert_eq!(dt.weekday(), Weekday::Wed);
    /// assert_eq!(dt.second(), 56);
    /// assert_eq!(dt.nanosecond(), 789_012_000);
    /// ```
    #[deprecated(since = "0.4.23", note = "use `and_hms_micro_opt()` instead")]
    #[inline]
    #[must_use]
    pub fn and_hms_micro(
        &self,
        hour: u32,
        min: u32,
        sec: u32,
        micro: u32,
    ) -> NaiveDateTime {
        self.and_hms_micro_opt(hour, min, sec, micro).expect("invalid time")
    }
    /// Makes a new `NaiveDateTime` from the current date, hour, minute, second and microsecond.
    ///
    /// The microsecond part can exceed 1,000,000
    /// in order to represent the [leap second](./struct.NaiveTime.html#leap-second-handling).
    ///
    /// Returns `None` on invalid hour, minute, second and/or microsecond.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::NaiveDate;
    ///
    /// let d = NaiveDate::from_ymd_opt(2015, 6, 3).unwrap();
    /// assert!(d.and_hms_micro_opt(12, 34, 56,   789_012).is_some());
    /// assert!(d.and_hms_micro_opt(12, 34, 59, 1_789_012).is_some()); // leap second
    /// assert!(d.and_hms_micro_opt(12, 34, 59, 2_789_012).is_none());
    /// assert!(d.and_hms_micro_opt(12, 34, 60,   789_012).is_none());
    /// assert!(d.and_hms_micro_opt(12, 60, 56,   789_012).is_none());
    /// assert!(d.and_hms_micro_opt(24, 34, 56,   789_012).is_none());
    /// ```
    #[inline]
    #[must_use]
    pub fn and_hms_micro_opt(
        &self,
        hour: u32,
        min: u32,
        sec: u32,
        micro: u32,
    ) -> Option<NaiveDateTime> {
        NaiveTime::from_hms_micro_opt(hour, min, sec, micro)
            .map(|time| self.and_time(time))
    }
    /// Makes a new `NaiveDateTime` from the current date, hour, minute, second and nanosecond.
    ///
    /// The nanosecond part can exceed 1,000,000,000
    /// in order to represent the [leap second](./struct.NaiveTime.html#leap-second-handling).
    ///
    /// Panics on invalid hour, minute, second and/or nanosecond.
    #[deprecated(since = "0.4.23", note = "use `and_hms_nano_opt()` instead")]
    #[inline]
    #[must_use]
    pub fn and_hms_nano(
        &self,
        hour: u32,
        min: u32,
        sec: u32,
        nano: u32,
    ) -> NaiveDateTime {
        self.and_hms_nano_opt(hour, min, sec, nano).expect("invalid time")
    }
    /// Makes a new `NaiveDateTime` from the current date, hour, minute, second and nanosecond.
    ///
    /// The nanosecond part can exceed 1,000,000,000
    /// in order to represent the [leap second](./struct.NaiveTime.html#leap-second-handling).
    ///
    /// Returns `None` on invalid hour, minute, second and/or nanosecond.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::NaiveDate;
    ///
    /// let d = NaiveDate::from_ymd_opt(2015, 6, 3).unwrap();
    /// assert!(d.and_hms_nano_opt(12, 34, 56,   789_012_345).is_some());
    /// assert!(d.and_hms_nano_opt(12, 34, 59, 1_789_012_345).is_some()); // leap second
    /// assert!(d.and_hms_nano_opt(12, 34, 59, 2_789_012_345).is_none());
    /// assert!(d.and_hms_nano_opt(12, 34, 60,   789_012_345).is_none());
    /// assert!(d.and_hms_nano_opt(12, 60, 56,   789_012_345).is_none());
    /// assert!(d.and_hms_nano_opt(24, 34, 56,   789_012_345).is_none());
    /// ```
    #[inline]
    #[must_use]
    pub fn and_hms_nano_opt(
        &self,
        hour: u32,
        min: u32,
        sec: u32,
        nano: u32,
    ) -> Option<NaiveDateTime> {
        NaiveTime::from_hms_nano_opt(hour, min, sec, nano)
            .map(|time| self.and_time(time))
    }
    /// Returns the packed month-day-flags.
    #[inline]
    fn mdf(&self) -> Mdf {
        self.of().to_mdf()
    }
    /// Returns the packed ordinal-flags.
    #[inline]
    const fn of(&self) -> Of {
        Of((self.ymdf & 0b1_1111_1111_1111) as u32)
    }
    /// Makes a new `NaiveDate` with the packed month-day-flags changed.
    ///
    /// Returns `None` when the resulting `NaiveDate` would be invalid.
    #[inline]
    fn with_mdf(&self, mdf: Mdf) -> Option<NaiveDate> {
        self.with_of(mdf.to_of())
    }
    /// Makes a new `NaiveDate` with the packed ordinal-flags changed.
    ///
    /// Returns `None` when the resulting `NaiveDate` would be invalid.
    #[inline]
    fn with_of(&self, of: Of) -> Option<NaiveDate> {
        if of.valid() {
            let Of(of) = of;
            Some(NaiveDate {
                ymdf: (self.ymdf & !0b1_1111_1111_1111) | of as DateImpl,
            })
        } else {
            None
        }
    }
    /// Makes a new `NaiveDate` for the next calendar date.
    ///
    /// Panics when `self` is the last representable date.
    #[deprecated(since = "0.4.23", note = "use `succ_opt()` instead")]
    #[inline]
    #[must_use]
    pub fn succ(&self) -> NaiveDate {
        self.succ_opt().expect("out of bound")
    }
    /// Makes a new `NaiveDate` for the next calendar date.
    ///
    /// Returns `None` when `self` is the last representable date.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::NaiveDate;
    ///
    /// assert_eq!(NaiveDate::from_ymd_opt(2015, 6, 3).unwrap().succ_opt(),
    ///            Some(NaiveDate::from_ymd_opt(2015, 6, 4).unwrap()));
    /// assert_eq!(NaiveDate::MAX.succ_opt(), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn succ_opt(&self) -> Option<NaiveDate> {
        self.with_of(self.of().succ())
            .or_else(|| NaiveDate::from_ymd_opt(self.year() + 1, 1, 1))
    }
    /// Makes a new `NaiveDate` for the previous calendar date.
    ///
    /// Panics when `self` is the first representable date.
    #[deprecated(since = "0.4.23", note = "use `pred_opt()` instead")]
    #[inline]
    #[must_use]
    pub fn pred(&self) -> NaiveDate {
        self.pred_opt().expect("out of bound")
    }
    /// Makes a new `NaiveDate` for the previous calendar date.
    ///
    /// Returns `None` when `self` is the first representable date.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::NaiveDate;
    ///
    /// assert_eq!(NaiveDate::from_ymd_opt(2015, 6, 3).unwrap().pred_opt(),
    ///            Some(NaiveDate::from_ymd_opt(2015, 6, 2).unwrap()));
    /// assert_eq!(NaiveDate::MIN.pred_opt(), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn pred_opt(&self) -> Option<NaiveDate> {
        self.with_of(self.of().pred())
            .or_else(|| NaiveDate::from_ymd_opt(self.year() - 1, 12, 31))
    }
    /// Adds the `days` part of given `Duration` to the current date.
    ///
    /// Returns `None` when it will result in overflow.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{TimeDelta, NaiveDate};
    ///
    /// let d = NaiveDate::from_ymd_opt(2015, 9, 5).unwrap();
    /// assert_eq!(d.checked_add_signed(TimeDelta::days(40)),
    ///            Some(NaiveDate::from_ymd_opt(2015, 10, 15).unwrap()));
    /// assert_eq!(d.checked_add_signed(TimeDelta::days(-40)),
    ///            Some(NaiveDate::from_ymd_opt(2015, 7, 27).unwrap()));
    /// assert_eq!(d.checked_add_signed(TimeDelta::days(1_000_000_000)), None);
    /// assert_eq!(d.checked_add_signed(TimeDelta::days(-1_000_000_000)), None);
    /// assert_eq!(NaiveDate::MAX.checked_add_signed(TimeDelta::days(1)), None);
    /// ```
    #[must_use]
    pub fn checked_add_signed(self, rhs: TimeDelta) -> Option<NaiveDate> {
        let year = self.year();
        let (mut year_div_400, year_mod_400) = div_mod_floor(year, 400);
        let cycle = internals::yo_to_cycle(year_mod_400 as u32, self.of().ordinal());
        let cycle = (cycle as i32).checked_add(i32::try_from(rhs.num_days()).ok()?)?;
        let (cycle_div_400y, cycle) = div_mod_floor(cycle, 146_097);
        year_div_400 += cycle_div_400y;
        let (year_mod_400, ordinal) = internals::cycle_to_yo(cycle as u32);
        let flags = YearFlags::from_year_mod_400(year_mod_400 as i32);
        NaiveDate::from_of(
            year_div_400 * 400 + year_mod_400 as i32,
            Of::new(ordinal, flags)?,
        )
    }
    /// Subtracts the `days` part of given `TimeDelta` from the current date.
    ///
    /// Returns `None` when it will result in overflow.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{TimeDelta, NaiveDate};
    ///
    /// let d = NaiveDate::from_ymd_opt(2015, 9, 5).unwrap();
    /// assert_eq!(d.checked_sub_signed(TimeDelta::days(40)),
    ///            Some(NaiveDate::from_ymd_opt(2015, 7, 27).unwrap()));
    /// assert_eq!(d.checked_sub_signed(TimeDelta::days(-40)),
    ///            Some(NaiveDate::from_ymd_opt(2015, 10, 15).unwrap()));
    /// assert_eq!(d.checked_sub_signed(TimeDelta::days(1_000_000_000)), None);
    /// assert_eq!(d.checked_sub_signed(TimeDelta::days(-1_000_000_000)), None);
    /// assert_eq!(NaiveDate::MIN.checked_sub_signed(TimeDelta::days(1)), None);
    /// ```
    #[must_use]
    pub fn checked_sub_signed(self, rhs: TimeDelta) -> Option<NaiveDate> {
        let year = self.year();
        let (mut year_div_400, year_mod_400) = div_mod_floor(year, 400);
        let cycle = internals::yo_to_cycle(year_mod_400 as u32, self.of().ordinal());
        let cycle = (cycle as i32).checked_sub(i32::try_from(rhs.num_days()).ok()?)?;
        let (cycle_div_400y, cycle) = div_mod_floor(cycle, 146_097);
        year_div_400 += cycle_div_400y;
        let (year_mod_400, ordinal) = internals::cycle_to_yo(cycle as u32);
        let flags = YearFlags::from_year_mod_400(year_mod_400 as i32);
        NaiveDate::from_of(
            year_div_400 * 400 + year_mod_400 as i32,
            Of::new(ordinal, flags)?,
        )
    }
    /// Subtracts another `NaiveDate` from the current date.
    /// Returns a `TimeDelta` of integral numbers.
    ///
    /// This does not overflow or underflow at all,
    /// as all possible output fits in the range of `TimeDelta`.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{TimeDelta, NaiveDate};
    ///
    /// let from_ymd = NaiveDate::from_ymd;
    /// let since = NaiveDate::signed_duration_since;
    ///
    /// assert_eq!(since(from_ymd(2014, 1, 1), from_ymd(2014, 1, 1)), TimeDelta::zero());
    /// assert_eq!(since(from_ymd(2014, 1, 1), from_ymd(2013, 12, 31)), TimeDelta::days(1));
    /// assert_eq!(since(from_ymd(2014, 1, 1), from_ymd(2014, 1, 2)), TimeDelta::days(-1));
    /// assert_eq!(since(from_ymd(2014, 1, 1), from_ymd(2013, 9, 23)), TimeDelta::days(100));
    /// assert_eq!(since(from_ymd(2014, 1, 1), from_ymd(2013, 1, 1)), TimeDelta::days(365));
    /// assert_eq!(since(from_ymd(2014, 1, 1), from_ymd(2010, 1, 1)), TimeDelta::days(365*4 + 1));
    /// assert_eq!(since(from_ymd(2014, 1, 1), from_ymd(1614, 1, 1)), TimeDelta::days(365*400 + 97));
    /// ```
    #[must_use]
    pub fn signed_duration_since(self, rhs: NaiveDate) -> TimeDelta {
        let year1 = self.year();
        let year2 = rhs.year();
        let (year1_div_400, year1_mod_400) = div_mod_floor(year1, 400);
        let (year2_div_400, year2_mod_400) = div_mod_floor(year2, 400);
        let cycle1 = i64::from(
            internals::yo_to_cycle(year1_mod_400 as u32, self.of().ordinal()),
        );
        let cycle2 = i64::from(
            internals::yo_to_cycle(year2_mod_400 as u32, rhs.of().ordinal()),
        );
        TimeDelta::days(
            (i64::from(year1_div_400) - i64::from(year2_div_400)) * 146_097
                + (cycle1 - cycle2),
        )
    }
    /// Returns the number of whole years from the given `base` until `self`.
    #[must_use]
    pub fn years_since(&self, base: Self) -> Option<u32> {
        let mut years = self.year() - base.year();
        if (self.month(), self.day()) < (base.month(), base.day()) {
            years -= 1;
        }
        match years >= 0 {
            true => Some(years as u32),
            false => None,
        }
    }
    /// Formats the date with the specified formatting items.
    /// Otherwise it is the same as the ordinary `format` method.
    ///
    /// The `Iterator` of items should be `Clone`able,
    /// since the resulting `DelayedFormat` value may be formatted multiple times.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::NaiveDate;
    /// use chrono::format::strftime::StrftimeItems;
    ///
    /// let fmt = StrftimeItems::new("%Y-%m-%d");
    /// let d = NaiveDate::from_ymd_opt(2015, 9, 5).unwrap();
    /// assert_eq!(d.format_with_items(fmt.clone()).to_string(), "2015-09-05");
    /// assert_eq!(d.format("%Y-%m-%d").to_string(),             "2015-09-05");
    /// ```
    ///
    /// The resulting `DelayedFormat` can be formatted directly via the `Display` trait.
    ///
    /// ```
    /// # use chrono::NaiveDate;
    /// # use chrono::format::strftime::StrftimeItems;
    /// # let fmt = StrftimeItems::new("%Y-%m-%d").clone();
    /// # let d = NaiveDate::from_ymd_opt(2015, 9, 5).unwrap();
    /// assert_eq!(format!("{}", d.format_with_items(fmt)), "2015-09-05");
    /// ```
    #[cfg(any(feature = "alloc", feature = "std", test))]
    #[cfg_attr(docsrs, doc(cfg(any(feature = "alloc", feature = "std"))))]
    #[inline]
    #[must_use]
    pub fn format_with_items<'a, I, B>(&self, items: I) -> DelayedFormat<I>
    where
        I: Iterator<Item = B> + Clone,
        B: Borrow<Item<'a>>,
    {
        DelayedFormat::new(Some(*self), None, items)
    }
    /// Formats the date with the specified format string.
    /// See the [`format::strftime` module](../format/strftime/index.html)
    /// on the supported escape sequences.
    ///
    /// This returns a `DelayedFormat`,
    /// which gets converted to a string only when actual formatting happens.
    /// You may use the `to_string` method to get a `String`,
    /// or just feed it into `print!` and other formatting macros.
    /// (In this way it avoids the redundant memory allocation.)
    ///
    /// A wrong format string does *not* issue an error immediately.
    /// Rather, converting or formatting the `DelayedFormat` fails.
    /// You are recommended to immediately use `DelayedFormat` for this reason.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::NaiveDate;
    ///
    /// let d = NaiveDate::from_ymd_opt(2015, 9, 5).unwrap();
    /// assert_eq!(d.format("%Y-%m-%d").to_string(), "2015-09-05");
    /// assert_eq!(d.format("%A, %-d %B, %C%y").to_string(), "Saturday, 5 September, 2015");
    /// ```
    ///
    /// The resulting `DelayedFormat` can be formatted directly via the `Display` trait.
    ///
    /// ```
    /// # use chrono::NaiveDate;
    /// # let d = NaiveDate::from_ymd_opt(2015, 9, 5).unwrap();
    /// assert_eq!(format!("{}", d.format("%Y-%m-%d")), "2015-09-05");
    /// assert_eq!(format!("{}", d.format("%A, %-d %B, %C%y")), "Saturday, 5 September, 2015");
    /// ```
    #[cfg(any(feature = "alloc", feature = "std", test))]
    #[cfg_attr(docsrs, doc(cfg(any(feature = "alloc", feature = "std"))))]
    #[inline]
    #[must_use]
    pub fn format<'a>(&self, fmt: &'a str) -> DelayedFormat<StrftimeItems<'a>> {
        self.format_with_items(StrftimeItems::new(fmt))
    }
    /// Formats the date with the specified formatting items and locale.
    #[cfg(feature = "unstable-locales")]
    #[cfg_attr(docsrs, doc(cfg(feature = "unstable-locales")))]
    #[inline]
    #[must_use]
    pub fn format_localized_with_items<'a, I, B>(
        &self,
        items: I,
        locale: Locale,
    ) -> DelayedFormat<I>
    where
        I: Iterator<Item = B> + Clone,
        B: Borrow<Item<'a>>,
    {
        DelayedFormat::new_with_locale(Some(*self), None, items, locale)
    }
    /// Formats the date with the specified format string and locale.
    ///
    /// See the [`crate::format::strftime`] module on the supported escape
    /// sequences.
    #[cfg(feature = "unstable-locales")]
    #[cfg_attr(docsrs, doc(cfg(feature = "unstable-locales")))]
    #[inline]
    #[must_use]
    pub fn format_localized<'a>(
        &self,
        fmt: &'a str,
        locale: Locale,
    ) -> DelayedFormat<StrftimeItems<'a>> {
        self.format_localized_with_items(
            StrftimeItems::new_with_locale(fmt, locale),
            locale,
        )
    }
    /// Returns an iterator that steps by days across all representable dates.
    ///
    /// # Example
    ///
    /// ```
    /// # use chrono::NaiveDate;
    ///
    /// let expected = [
    ///     NaiveDate::from_ymd_opt(2016, 2, 27).unwrap(),
    ///     NaiveDate::from_ymd_opt(2016, 2, 28).unwrap(),
    ///     NaiveDate::from_ymd_opt(2016, 2, 29).unwrap(),
    ///     NaiveDate::from_ymd_opt(2016, 3, 1).unwrap(),
    /// ];
    ///
    /// let mut count = 0;
    /// for (idx, d) in NaiveDate::from_ymd_opt(2016, 2, 27).unwrap().iter_days().take(4).enumerate() {
    ///    assert_eq!(d, expected[idx]);
    ///    count += 1;
    /// }
    /// assert_eq!(count, 4);
    ///
    /// for d in NaiveDate::from_ymd_opt(2016, 3, 1).unwrap().iter_days().rev().take(4) {
    ///     count -= 1;
    ///     assert_eq!(d, expected[count]);
    /// }
    /// ```
    #[inline]
    pub const fn iter_days(&self) -> NaiveDateDaysIterator {
        NaiveDateDaysIterator {
            value: *self,
        }
    }
    /// Returns an iterator that steps by weeks across all representable dates.
    ///
    /// # Example
    ///
    /// ```
    /// # use chrono::NaiveDate;
    ///
    /// let expected = [
    ///     NaiveDate::from_ymd_opt(2016, 2, 27).unwrap(),
    ///     NaiveDate::from_ymd_opt(2016, 3, 5).unwrap(),
    ///     NaiveDate::from_ymd_opt(2016, 3, 12).unwrap(),
    ///     NaiveDate::from_ymd_opt(2016, 3, 19).unwrap(),
    /// ];
    ///
    /// let mut count = 0;
    /// for (idx, d) in NaiveDate::from_ymd_opt(2016, 2, 27).unwrap().iter_weeks().take(4).enumerate() {
    ///    assert_eq!(d, expected[idx]);
    ///    count += 1;
    /// }
    /// assert_eq!(count, 4);
    ///
    /// for d in NaiveDate::from_ymd_opt(2016, 3, 19).unwrap().iter_weeks().rev().take(4) {
    ///     count -= 1;
    ///     assert_eq!(d, expected[count]);
    /// }
    /// ```
    #[inline]
    pub const fn iter_weeks(&self) -> NaiveDateWeeksIterator {
        NaiveDateWeeksIterator {
            value: *self,
        }
    }
    /// Returns the [`NaiveWeek`] that the date belongs to, starting with the [`Weekday`]
    /// specified.
    #[inline]
    pub const fn week(&self, start: Weekday) -> NaiveWeek {
        NaiveWeek { date: *self, start }
    }
    /// The minimum possible `NaiveDate` (January 1, 262145 BCE).
    pub const MIN: NaiveDate = NaiveDate {
        ymdf: (MIN_YEAR << 13) | (1 << 4) | 0o07,
    };
    /// The maximum possible `NaiveDate` (December 31, 262143 CE).
    pub const MAX: NaiveDate = NaiveDate {
        ymdf: (MAX_YEAR << 13) | (365 << 4) | 0o17,
    };
}
impl Datelike for NaiveDate {
    /// Returns the year number in the [calendar date](#calendar-date).
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, Datelike};
    ///
    /// assert_eq!(NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().year(), 2015);
    /// assert_eq!(NaiveDate::from_ymd_opt(-308, 3, 14).unwrap().year(), -308); // 309 BCE
    /// ```
    #[inline]
    fn year(&self) -> i32 {
        self.ymdf >> 13
    }
    /// Returns the month number starting from 1.
    ///
    /// The return value ranges from 1 to 12.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, Datelike};
    ///
    /// assert_eq!(NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().month(), 9);
    /// assert_eq!(NaiveDate::from_ymd_opt(-308, 3, 14).unwrap().month(), 3);
    /// ```
    #[inline]
    fn month(&self) -> u32 {
        self.mdf().month()
    }
    /// Returns the month number starting from 0.
    ///
    /// The return value ranges from 0 to 11.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, Datelike};
    ///
    /// assert_eq!(NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().month0(), 8);
    /// assert_eq!(NaiveDate::from_ymd_opt(-308, 3, 14).unwrap().month0(), 2);
    /// ```
    #[inline]
    fn month0(&self) -> u32 {
        self.mdf().month() - 1
    }
    /// Returns the day of month starting from 1.
    ///
    /// The return value ranges from 1 to 31. (The last day of month differs by months.)
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, Datelike};
    ///
    /// assert_eq!(NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().day(), 8);
    /// assert_eq!(NaiveDate::from_ymd_opt(-308, 3, 14).unwrap().day(), 14);
    /// ```
    ///
    /// Combined with [`NaiveDate::pred`](#method.pred),
    /// one can determine the number of days in a particular month.
    /// (Note that this panics when `year` is out of range.)
    ///
    /// ```
    /// use chrono::{NaiveDate, Datelike};
    ///
    /// fn ndays_in_month(year: i32, month: u32) -> u32 {
    ///     // the first day of the next month...
    ///     let (y, m) = if month == 12 { (year + 1, 1) } else { (year, month + 1) };
    ///     let d = NaiveDate::from_ymd_opt(y, m, 1).unwrap();
    ///
    ///     // ...is preceded by the last day of the original month
    ///     d.pred().day()
    /// }
    ///
    /// assert_eq!(ndays_in_month(2015, 8), 31);
    /// assert_eq!(ndays_in_month(2015, 9), 30);
    /// assert_eq!(ndays_in_month(2015, 12), 31);
    /// assert_eq!(ndays_in_month(2016, 2), 29);
    /// assert_eq!(ndays_in_month(2017, 2), 28);
    /// ```
    #[inline]
    fn day(&self) -> u32 {
        self.mdf().day()
    }
    /// Returns the day of month starting from 0.
    ///
    /// The return value ranges from 0 to 30. (The last day of month differs by months.)
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, Datelike};
    ///
    /// assert_eq!(NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().day0(), 7);
    /// assert_eq!(NaiveDate::from_ymd_opt(-308, 3, 14).unwrap().day0(), 13);
    /// ```
    #[inline]
    fn day0(&self) -> u32 {
        self.mdf().day() - 1
    }
    /// Returns the day of year starting from 1.
    ///
    /// The return value ranges from 1 to 366. (The last day of year differs by years.)
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, Datelike};
    ///
    /// assert_eq!(NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().ordinal(), 251);
    /// assert_eq!(NaiveDate::from_ymd_opt(-308, 3, 14).unwrap().ordinal(), 74);
    /// ```
    ///
    /// Combined with [`NaiveDate::pred`](#method.pred),
    /// one can determine the number of days in a particular year.
    /// (Note that this panics when `year` is out of range.)
    ///
    /// ```
    /// use chrono::{NaiveDate, Datelike};
    ///
    /// fn ndays_in_year(year: i32) -> u32 {
    ///     // the first day of the next year...
    ///     let d = NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap();
    ///
    ///     // ...is preceded by the last day of the original year
    ///     d.pred().ordinal()
    /// }
    ///
    /// assert_eq!(ndays_in_year(2015), 365);
    /// assert_eq!(ndays_in_year(2016), 366);
    /// assert_eq!(ndays_in_year(2017), 365);
    /// assert_eq!(ndays_in_year(2000), 366);
    /// assert_eq!(ndays_in_year(2100), 365);
    /// ```
    #[inline]
    fn ordinal(&self) -> u32 {
        self.of().ordinal()
    }
    /// Returns the day of year starting from 0.
    ///
    /// The return value ranges from 0 to 365. (The last day of year differs by years.)
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, Datelike};
    ///
    /// assert_eq!(NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().ordinal0(), 250);
    /// assert_eq!(NaiveDate::from_ymd_opt(-308, 3, 14).unwrap().ordinal0(), 73);
    /// ```
    #[inline]
    fn ordinal0(&self) -> u32 {
        self.of().ordinal() - 1
    }
    /// Returns the day of week.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, Datelike, Weekday};
    ///
    /// assert_eq!(NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().weekday(), Weekday::Tue);
    /// assert_eq!(NaiveDate::from_ymd_opt(-308, 3, 14).unwrap().weekday(), Weekday::Fri);
    /// ```
    #[inline]
    fn weekday(&self) -> Weekday {
        self.of().weekday()
    }
    #[inline]
    fn iso_week(&self) -> IsoWeek {
        isoweek::iso_week_from_yof(self.year(), self.of())
    }
    /// Makes a new `NaiveDate` with the year number changed.
    ///
    /// Returns `None` when the resulting `NaiveDate` would be invalid.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, Datelike};
    ///
    /// assert_eq!(NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().with_year(2016),
    ///            Some(NaiveDate::from_ymd_opt(2016, 9, 8).unwrap()));
    /// assert_eq!(NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().with_year(-308),
    ///            Some(NaiveDate::from_ymd_opt(-308, 9, 8).unwrap()));
    /// ```
    ///
    /// A leap day (February 29) is a good example that this method can return `None`.
    ///
    /// ```
    /// # use chrono::{NaiveDate, Datelike};
    /// assert!(NaiveDate::from_ymd_opt(2016, 2, 29).unwrap().with_year(2015).is_none());
    /// assert!(NaiveDate::from_ymd_opt(2016, 2, 29).unwrap().with_year(2020).is_some());
    /// ```
    #[inline]
    fn with_year(&self, year: i32) -> Option<NaiveDate> {
        let mdf = self.mdf();
        let flags = YearFlags::from_year(year);
        let mdf = mdf.with_flags(flags);
        NaiveDate::from_mdf(year, mdf)
    }
    /// Makes a new `NaiveDate` with the month number (starting from 1) changed.
    ///
    /// Returns `None` when the resulting `NaiveDate` would be invalid.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, Datelike};
    ///
    /// assert_eq!(NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().with_month(10),
    ///            Some(NaiveDate::from_ymd_opt(2015, 10, 8).unwrap()));
    /// assert_eq!(NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().with_month(13), None); // no month 13
    /// assert_eq!(NaiveDate::from_ymd_opt(2015, 9, 30).unwrap().with_month(2), None); // no February 30
    /// ```
    #[inline]
    fn with_month(&self, month: u32) -> Option<NaiveDate> {
        self.with_mdf(self.mdf().with_month(month)?)
    }
    /// Makes a new `NaiveDate` with the month number (starting from 0) changed.
    ///
    /// Returns `None` when the resulting `NaiveDate` would be invalid.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, Datelike};
    ///
    /// assert_eq!(NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().with_month0(9),
    ///            Some(NaiveDate::from_ymd_opt(2015, 10, 8).unwrap()));
    /// assert_eq!(NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().with_month0(12), None); // no month 13
    /// assert_eq!(NaiveDate::from_ymd_opt(2015, 9, 30).unwrap().with_month0(1), None); // no February 30
    /// ```
    #[inline]
    fn with_month0(&self, month0: u32) -> Option<NaiveDate> {
        let month = month0.checked_add(1)?;
        self.with_mdf(self.mdf().with_month(month)?)
    }
    /// Makes a new `NaiveDate` with the day of month (starting from 1) changed.
    ///
    /// Returns `None` when the resulting `NaiveDate` would be invalid.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, Datelike};
    ///
    /// assert_eq!(NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().with_day(30),
    ///            Some(NaiveDate::from_ymd_opt(2015, 9, 30).unwrap()));
    /// assert_eq!(NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().with_day(31),
    ///            None); // no September 31
    /// ```
    #[inline]
    fn with_day(&self, day: u32) -> Option<NaiveDate> {
        self.with_mdf(self.mdf().with_day(day)?)
    }
    /// Makes a new `NaiveDate` with the day of month (starting from 0) changed.
    ///
    /// Returns `None` when the resulting `NaiveDate` would be invalid.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, Datelike};
    ///
    /// assert_eq!(NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().with_day0(29),
    ///            Some(NaiveDate::from_ymd_opt(2015, 9, 30).unwrap()));
    /// assert_eq!(NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().with_day0(30),
    ///            None); // no September 31
    /// ```
    #[inline]
    fn with_day0(&self, day0: u32) -> Option<NaiveDate> {
        let day = day0.checked_add(1)?;
        self.with_mdf(self.mdf().with_day(day)?)
    }
    /// Makes a new `NaiveDate` with the day of year (starting from 1) changed.
    ///
    /// Returns `None` when the resulting `NaiveDate` would be invalid.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, Datelike};
    ///
    /// assert_eq!(NaiveDate::from_ymd_opt(2015, 1, 1).unwrap().with_ordinal(60),
    ///            Some(NaiveDate::from_ymd_opt(2015, 3, 1).unwrap()));
    /// assert_eq!(NaiveDate::from_ymd_opt(2015, 1, 1).unwrap().with_ordinal(366),
    ///            None); // 2015 had only 365 days
    ///
    /// assert_eq!(NaiveDate::from_ymd_opt(2016, 1, 1).unwrap().with_ordinal(60),
    ///            Some(NaiveDate::from_ymd_opt(2016, 2, 29).unwrap()));
    /// assert_eq!(NaiveDate::from_ymd_opt(2016, 1, 1).unwrap().with_ordinal(366),
    ///            Some(NaiveDate::from_ymd_opt(2016, 12, 31).unwrap()));
    /// ```
    #[inline]
    fn with_ordinal(&self, ordinal: u32) -> Option<NaiveDate> {
        self.with_of(self.of().with_ordinal(ordinal)?)
    }
    /// Makes a new `NaiveDate` with the day of year (starting from 0) changed.
    ///
    /// Returns `None` when the resulting `NaiveDate` would be invalid.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, Datelike};
    ///
    /// assert_eq!(NaiveDate::from_ymd_opt(2015, 1, 1).unwrap().with_ordinal0(59),
    ///            Some(NaiveDate::from_ymd_opt(2015, 3, 1).unwrap()));
    /// assert_eq!(NaiveDate::from_ymd_opt(2015, 1, 1).unwrap().with_ordinal0(365),
    ///            None); // 2015 had only 365 days
    ///
    /// assert_eq!(NaiveDate::from_ymd_opt(2016, 1, 1).unwrap().with_ordinal0(59),
    ///            Some(NaiveDate::from_ymd_opt(2016, 2, 29).unwrap()));
    /// assert_eq!(NaiveDate::from_ymd_opt(2016, 1, 1).unwrap().with_ordinal0(365),
    ///            Some(NaiveDate::from_ymd_opt(2016, 12, 31).unwrap()));
    /// ```
    #[inline]
    fn with_ordinal0(&self, ordinal0: u32) -> Option<NaiveDate> {
        let ordinal = ordinal0.checked_add(1)?;
        self.with_of(self.of().with_ordinal(ordinal)?)
    }
}
/// An addition of `Duration` to `NaiveDate` discards the fractional days,
/// rounding to the closest integral number of days towards `Duration::zero()`.
///
/// Panics on underflow or overflow. Use [`NaiveDate::checked_add_signed`] to detect that.
///
/// # Example
///
/// ```
/// use chrono::{TimeDelta, NaiveDate};
///
/// let from_ymd = NaiveDate::from_ymd;
///
/// assert_eq!(from_ymd(2014, 1, 1) + TimeDelta::zero(),             from_ymd(2014, 1, 1));
/// assert_eq!(from_ymd(2014, 1, 1) + TimeDelta::seconds(86399),     from_ymd(2014, 1, 1));
/// assert_eq!(from_ymd(2014, 1, 1) + TimeDelta::seconds(-86399),    from_ymd(2014, 1, 1));
/// assert_eq!(from_ymd(2014, 1, 1) + TimeDelta::days(1),            from_ymd(2014, 1, 2));
/// assert_eq!(from_ymd(2014, 1, 1) + TimeDelta::days(-1),           from_ymd(2013, 12, 31));
/// assert_eq!(from_ymd(2014, 1, 1) + TimeDelta::days(364),          from_ymd(2014, 12, 31));
/// assert_eq!(from_ymd(2014, 1, 1) + TimeDelta::days(365*4 + 1),    from_ymd(2018, 1, 1));
/// assert_eq!(from_ymd(2014, 1, 1) + TimeDelta::days(365*400 + 97), from_ymd(2414, 1, 1));
/// ```
///
/// [`NaiveDate::checked_add_signed`]: crate::NaiveDate::checked_add_signed
impl Add<TimeDelta> for NaiveDate {
    type Output = NaiveDate;
    #[inline]
    fn add(self, rhs: TimeDelta) -> NaiveDate {
        self.checked_add_signed(rhs).expect("`NaiveDate + TimeDelta` overflowed")
    }
}
impl AddAssign<TimeDelta> for NaiveDate {
    #[inline]
    fn add_assign(&mut self, rhs: TimeDelta) {
        *self = self.add(rhs);
    }
}
impl Add<Months> for NaiveDate {
    type Output = NaiveDate;
    /// An addition of months to `NaiveDate` clamped to valid days in resulting month.
    ///
    /// # Panics
    ///
    /// Panics if the resulting date would be out of range.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{TimeDelta, NaiveDate, Months};
    ///
    /// let from_ymd = NaiveDate::from_ymd;
    ///
    /// assert_eq!(from_ymd(2014, 1, 1) + Months::new(1), from_ymd(2014, 2, 1));
    /// assert_eq!(from_ymd(2014, 1, 1) + Months::new(11), from_ymd(2014, 12, 1));
    /// assert_eq!(from_ymd(2014, 1, 1) + Months::new(12), from_ymd(2015, 1, 1));
    /// assert_eq!(from_ymd(2014, 1, 1) + Months::new(13), from_ymd(2015, 2, 1));
    /// assert_eq!(from_ymd(2014, 1, 31) + Months::new(1), from_ymd(2014, 2, 28));
    /// assert_eq!(from_ymd(2020, 1, 31) + Months::new(1), from_ymd(2020, 2, 29));
    /// ```
    fn add(self, months: Months) -> Self::Output {
        self.checked_add_months(months).unwrap()
    }
}
impl Sub<Months> for NaiveDate {
    type Output = NaiveDate;
    /// A subtraction of Months from `NaiveDate` clamped to valid days in resulting month.
    ///
    /// # Panics
    ///
    /// Panics if the resulting date would be out of range.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{TimeDelta, NaiveDate, Months};
    ///
    /// let from_ymd = NaiveDate::from_ymd;
    ///
    /// assert_eq!(from_ymd(2014, 1, 1) - Months::new(11), from_ymd(2013, 2, 1));
    /// assert_eq!(from_ymd(2014, 1, 1) - Months::new(12), from_ymd(2013, 1, 1));
    /// assert_eq!(from_ymd(2014, 1, 1) - Months::new(13), from_ymd(2012, 12, 1));
    /// ```
    fn sub(self, months: Months) -> Self::Output {
        self.checked_sub_months(months).unwrap()
    }
}
impl Add<Days> for NaiveDate {
    type Output = NaiveDate;
    fn add(self, days: Days) -> Self::Output {
        self.checked_add_days(days).unwrap()
    }
}
impl Sub<Days> for NaiveDate {
    type Output = NaiveDate;
    fn sub(self, days: Days) -> Self::Output {
        self.checked_sub_days(days).unwrap()
    }
}
/// A subtraction of `TimeDelta` from `NaiveDate` discards the fractional days,
/// rounding to the closest integral number of days towards `TimeDelta::zero()`.
/// It is the same as the addition with a negated `TimeDelta`.
///
/// Panics on underflow or overflow. Use [`NaiveDate::checked_sub_signed`] to detect that.
///
/// # Example
///
/// ```
/// use chrono::{TimeDelta, NaiveDate};
///
/// let from_ymd = NaiveDate::from_ymd;
///
/// assert_eq!(from_ymd(2014, 1, 1) - TimeDelta::zero(),             from_ymd(2014, 1, 1));
/// assert_eq!(from_ymd(2014, 1, 1) - TimeDelta::seconds(86399),     from_ymd(2014, 1, 1));
/// assert_eq!(from_ymd(2014, 1, 1) - TimeDelta::seconds(-86399),    from_ymd(2014, 1, 1));
/// assert_eq!(from_ymd(2014, 1, 1) - TimeDelta::days(1),            from_ymd(2013, 12, 31));
/// assert_eq!(from_ymd(2014, 1, 1) - TimeDelta::days(-1),           from_ymd(2014, 1, 2));
/// assert_eq!(from_ymd(2014, 1, 1) - TimeDelta::days(364),          from_ymd(2013, 1, 2));
/// assert_eq!(from_ymd(2014, 1, 1) - TimeDelta::days(365*4 + 1),    from_ymd(2010, 1, 1));
/// assert_eq!(from_ymd(2014, 1, 1) - TimeDelta::days(365*400 + 97), from_ymd(1614, 1, 1));
/// ```
///
/// [`NaiveDate::checked_sub_signed`]: crate::NaiveDate::checked_sub_signed
impl Sub<TimeDelta> for NaiveDate {
    type Output = NaiveDate;
    #[inline]
    fn sub(self, rhs: TimeDelta) -> NaiveDate {
        self.checked_sub_signed(rhs).expect("`NaiveDate - TimeDelta` overflowed")
    }
}
impl SubAssign<TimeDelta> for NaiveDate {
    #[inline]
    fn sub_assign(&mut self, rhs: TimeDelta) {
        *self = self.sub(rhs);
    }
}
/// Subtracts another `NaiveDate` from the current date.
/// Returns a `TimeDelta` of integral numbers.
///
/// This does not overflow or underflow at all,
/// as all possible output fits in the range of `TimeDelta`.
///
/// The implementation is a wrapper around
/// [`NaiveDate::signed_duration_since`](#method.signed_duration_since).
///
/// # Example
///
/// ```
/// use chrono::{TimeDelta, NaiveDate};
///
/// let from_ymd = NaiveDate::from_ymd;
///
/// assert_eq!(from_ymd(2014, 1, 1) - from_ymd(2014, 1, 1), TimeDelta::zero());
/// assert_eq!(from_ymd(2014, 1, 1) - from_ymd(2013, 12, 31), TimeDelta::days(1));
/// assert_eq!(from_ymd(2014, 1, 1) - from_ymd(2014, 1, 2), TimeDelta::days(-1));
/// assert_eq!(from_ymd(2014, 1, 1) - from_ymd(2013, 9, 23), TimeDelta::days(100));
/// assert_eq!(from_ymd(2014, 1, 1) - from_ymd(2013, 1, 1), TimeDelta::days(365));
/// assert_eq!(from_ymd(2014, 1, 1) - from_ymd(2010, 1, 1), TimeDelta::days(365*4 + 1));
/// assert_eq!(from_ymd(2014, 1, 1) - from_ymd(1614, 1, 1), TimeDelta::days(365*400 + 97));
/// ```
impl Sub<NaiveDate> for NaiveDate {
    type Output = TimeDelta;
    #[inline]
    fn sub(self, rhs: NaiveDate) -> TimeDelta {
        self.signed_duration_since(rhs)
    }
}
impl From<NaiveDateTime> for NaiveDate {
    fn from(naive_datetime: NaiveDateTime) -> Self {
        naive_datetime.date()
    }
}
/// Iterator over `NaiveDate` with a step size of one day.
#[derive(Debug, Copy, Clone, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub struct NaiveDateDaysIterator {
    value: NaiveDate,
}
impl Iterator for NaiveDateDaysIterator {
    type Item = NaiveDate;
    fn next(&mut self) -> Option<Self::Item> {
        if self.value == NaiveDate::MAX {
            return None;
        }
        let current = self.value;
        self.value = current.succ_opt().unwrap();
        Some(current)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let exact_size = NaiveDate::MAX.signed_duration_since(self.value).num_days();
        (exact_size as usize, Some(exact_size as usize))
    }
}
impl ExactSizeIterator for NaiveDateDaysIterator {}
impl DoubleEndedIterator for NaiveDateDaysIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.value == NaiveDate::MIN {
            return None;
        }
        let current = self.value;
        self.value = current.pred_opt().unwrap();
        Some(current)
    }
}
#[derive(Debug, Copy, Clone, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub struct NaiveDateWeeksIterator {
    value: NaiveDate,
}
impl Iterator for NaiveDateWeeksIterator {
    type Item = NaiveDate;
    fn next(&mut self) -> Option<Self::Item> {
        if NaiveDate::MAX - self.value < TimeDelta::weeks(1) {
            return None;
        }
        let current = self.value;
        self.value = current + TimeDelta::weeks(1);
        Some(current)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let exact_size = NaiveDate::MAX.signed_duration_since(self.value).num_weeks();
        (exact_size as usize, Some(exact_size as usize))
    }
}
impl ExactSizeIterator for NaiveDateWeeksIterator {}
impl DoubleEndedIterator for NaiveDateWeeksIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.value - NaiveDate::MIN < TimeDelta::weeks(1) {
            return None;
        }
        let current = self.value;
        self.value = current - TimeDelta::weeks(1);
        Some(current)
    }
}
/// The `Debug` output of the naive date `d` is the same as
/// [`d.format("%Y-%m-%d")`](../format/strftime/index.html).
///
/// The string printed can be readily parsed via the `parse` method on `str`.
///
/// # Example
///
/// ```
/// use chrono::NaiveDate;
///
/// assert_eq!(format!("{:?}", NaiveDate::from_ymd_opt(2015,  9,  5).unwrap()), "2015-09-05");
/// assert_eq!(format!("{:?}", NaiveDate::from_ymd_opt(   0,  1,  1).unwrap()), "0000-01-01");
/// assert_eq!(format!("{:?}", NaiveDate::from_ymd_opt(9999, 12, 31).unwrap()), "9999-12-31");
/// ```
///
/// ISO 8601 requires an explicit sign for years before 1 BCE or after 9999 CE.
///
/// ```
/// # use chrono::NaiveDate;
/// assert_eq!(format!("{:?}", NaiveDate::from_ymd_opt(   -1,  1,  1).unwrap()),  "-0001-01-01");
/// assert_eq!(format!("{:?}", NaiveDate::from_ymd_opt(10000, 12, 31).unwrap()), "+10000-12-31");
/// ```
impl fmt::Debug for NaiveDate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use core::fmt::Write;
        let year = self.year();
        let mdf = self.mdf();
        if (0..=9999).contains(&year) {
            write_hundreds(f, (year / 100) as u8)?;
            write_hundreds(f, (year % 100) as u8)?;
        } else {
            write!(f, "{:+05}", year)?;
        }
        f.write_char('-')?;
        write_hundreds(f, mdf.month() as u8)?;
        f.write_char('-')?;
        write_hundreds(f, mdf.day() as u8)
    }
}
/// The `Display` output of the naive date `d` is the same as
/// [`d.format("%Y-%m-%d")`](../format/strftime/index.html).
///
/// The string printed can be readily parsed via the `parse` method on `str`.
///
/// # Example
///
/// ```
/// use chrono::NaiveDate;
///
/// assert_eq!(format!("{}", NaiveDate::from_ymd_opt(2015,  9,  5).unwrap()), "2015-09-05");
/// assert_eq!(format!("{}", NaiveDate::from_ymd_opt(   0,  1,  1).unwrap()), "0000-01-01");
/// assert_eq!(format!("{}", NaiveDate::from_ymd_opt(9999, 12, 31).unwrap()), "9999-12-31");
/// ```
///
/// ISO 8601 requires an explicit sign for years before 1 BCE or after 9999 CE.
///
/// ```
/// # use chrono::NaiveDate;
/// assert_eq!(format!("{}", NaiveDate::from_ymd_opt(   -1,  1,  1).unwrap()),  "-0001-01-01");
/// assert_eq!(format!("{}", NaiveDate::from_ymd_opt(10000, 12, 31).unwrap()), "+10000-12-31");
/// ```
impl fmt::Display for NaiveDate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
/// Parsing a `str` into a `NaiveDate` uses the same format,
/// [`%Y-%m-%d`](../format/strftime/index.html), as in `Debug` and `Display`.
///
/// # Example
///
/// ```
/// use chrono::NaiveDate;
///
/// let d = NaiveDate::from_ymd_opt(2015, 9, 18).unwrap();
/// assert_eq!("2015-09-18".parse::<NaiveDate>(), Ok(d));
///
/// let d = NaiveDate::from_ymd_opt(12345, 6, 7).unwrap();
/// assert_eq!("+12345-6-7".parse::<NaiveDate>(), Ok(d));
///
/// assert!("foo".parse::<NaiveDate>().is_err());
/// ```
impl str::FromStr for NaiveDate {
    type Err = ParseError;
    fn from_str(s: &str) -> ParseResult<NaiveDate> {
        const ITEMS: &[Item<'static>] = &[
            Item::Numeric(Numeric::Year, Pad::Zero),
            Item::Literal("-"),
            Item::Numeric(Numeric::Month, Pad::Zero),
            Item::Literal("-"),
            Item::Numeric(Numeric::Day, Pad::Zero),
        ];
        let mut parsed = Parsed::new();
        parse(&mut parsed, s, ITEMS.iter())?;
        parsed.to_naive_date()
    }
}
/// The default value for a NaiveDate is 1st of January 1970.
///
/// # Example
///
/// ```rust
/// use chrono::NaiveDate;
///
/// let default_date = NaiveDate::default();
/// assert_eq!(default_date, NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());
/// ```
impl Default for NaiveDate {
    fn default() -> Self {
        NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()
    }
}
fn div_mod_floor(val: i32, div: i32) -> (i32, i32) {
    (val.div_euclid(div), val.rem_euclid(div))
}
#[cfg(all(test, feature = "serde"))]
fn test_encodable_json<F, E>(to_string: F)
where
    F: Fn(&NaiveDate) -> Result<String, E>,
    E: ::std::fmt::Debug,
{
    assert_eq!(
        to_string(& NaiveDate::from_ymd_opt(2014, 7, 24).unwrap()).ok(),
        Some(r#""2014-07-24""#.into())
    );
    assert_eq!(
        to_string(& NaiveDate::from_ymd_opt(0, 1, 1).unwrap()).ok(),
        Some(r#""0000-01-01""#.into())
    );
    assert_eq!(
        to_string(& NaiveDate::from_ymd_opt(- 1, 12, 31).unwrap()).ok(),
        Some(r#""-0001-12-31""#.into())
    );
    assert_eq!(to_string(& NaiveDate::MIN).ok(), Some(r#""-262144-01-01""#.into()));
    assert_eq!(to_string(& NaiveDate::MAX).ok(), Some(r#""+262143-12-31""#.into()));
}
#[cfg(all(test, feature = "serde"))]
fn test_decodable_json<F, E>(from_str: F)
where
    F: Fn(&str) -> Result<NaiveDate, E>,
    E: ::std::fmt::Debug,
{
    use std::{i32, i64};
    assert_eq!(
        from_str(r#""2016-07-08""#).ok(), Some(NaiveDate::from_ymd_opt(2016, 7, 8)
        .unwrap())
    );
    assert_eq!(
        from_str(r#""2016-7-8""#).ok(), Some(NaiveDate::from_ymd_opt(2016, 7, 8)
        .unwrap())
    );
    assert_eq!(from_str(r#""+002016-07-08""#).ok(), NaiveDate::from_ymd_opt(2016, 7, 8));
    assert_eq!(
        from_str(r#""0000-01-01""#).ok(), Some(NaiveDate::from_ymd_opt(0, 1, 1).unwrap())
    );
    assert_eq!(
        from_str(r#""0-1-1""#).ok(), Some(NaiveDate::from_ymd_opt(0, 1, 1).unwrap())
    );
    assert_eq!(
        from_str(r#""-0001-12-31""#).ok(), Some(NaiveDate::from_ymd_opt(- 1, 12, 31)
        .unwrap())
    );
    assert_eq!(from_str(r#""-262144-01-01""#).ok(), Some(NaiveDate::MIN));
    assert_eq!(from_str(r#""+262143-12-31""#).ok(), Some(NaiveDate::MAX));
    assert!(from_str(r#""""#).is_err());
    assert!(from_str(r#""20001231""#).is_err());
    assert!(from_str(r#""2000-00-00""#).is_err());
    assert!(from_str(r#""2000-02-30""#).is_err());
    assert!(from_str(r#""2001-02-29""#).is_err());
    assert!(from_str(r#""2002-002-28""#).is_err());
    assert!(from_str(r#""yyyy-mm-dd""#).is_err());
    assert!(from_str(r#"0"#).is_err());
    assert!(from_str(r#"20.01"#).is_err());
    assert!(from_str(& i32::MIN.to_string()).is_err());
    assert!(from_str(& i32::MAX.to_string()).is_err());
    assert!(from_str(& i64::MIN.to_string()).is_err());
    assert!(from_str(& i64::MAX.to_string()).is_err());
    assert!(from_str(r#"{}"#).is_err());
    assert!(from_str(r#"{"ymdf":20}"#).is_err());
    assert!(from_str(r#"null"#).is_err());
}
#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
mod serde {
    use super::NaiveDate;
    use core::fmt;
    use serde::{de, ser};
    impl ser::Serialize for NaiveDate {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
        {
            struct FormatWrapped<'a, D: 'a> {
                inner: &'a D,
            }
            impl<'a, D: fmt::Debug> fmt::Display for FormatWrapped<'a, D> {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    self.inner.fmt(f)
                }
            }
            serializer.collect_str(&FormatWrapped { inner: &self })
        }
    }
    struct NaiveDateVisitor;
    impl<'de> de::Visitor<'de> for NaiveDateVisitor {
        type Value = NaiveDate;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a formatted date string")
        }
        #[cfg(any(feature = "std", test))]
        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            value.parse().map_err(E::custom)
        }
        #[cfg(not(any(feature = "std", test)))]
        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            value.parse().map_err(E::custom)
        }
    }
    impl<'de> de::Deserialize<'de> for NaiveDate {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            deserializer.deserialize_str(NaiveDateVisitor)
        }
    }
    #[test]
    fn test_serde_serialize() {
        super::test_encodable_json(serde_json::to_string);
    }
    #[test]
    fn test_serde_deserialize() {
        super::test_decodable_json(|input| serde_json::from_str(input));
    }
    #[test]
    fn test_serde_bincode() {
        use bincode::{deserialize, serialize};
        let d = NaiveDate::from_ymd_opt(2014, 7, 24).unwrap();
        let encoded = serialize(&d).unwrap();
        let decoded: NaiveDate = deserialize(&encoded).unwrap();
        assert_eq!(d, decoded);
    }
}
#[cfg(test)]
mod tests {
    use super::{
        Days, Months, NaiveDate, MAX_DAYS_FROM_YEAR_0, MAX_YEAR, MIN_DAYS_FROM_YEAR_0,
        MIN_YEAR,
    };
    use crate::time_delta::TimeDelta;
    use crate::{Datelike, Weekday};
    use std::{
        convert::{TryFrom, TryInto},
        i32, u32,
    };
    #[test]
    fn diff_months() {
        assert_eq!(
            NaiveDate::from_ymd_opt(2022, 8, 3).unwrap()
            .checked_add_months(Months::new(0)), Some(NaiveDate::from_ymd_opt(2022, 8, 3)
            .unwrap())
        );
        assert_eq!(
            NaiveDate::from_ymd_opt(2022, 8, 3).unwrap()
            .checked_add_months(Months::new(i32::MAX as u32 + 1)), None
        );
        assert_eq!(
            NaiveDate::from_ymd_opt(2022, 8, 3).unwrap()
            .checked_sub_months(Months::new(i32::MIN.unsigned_abs() + 1)), None
        );
        assert_eq!(NaiveDate::MAX.checked_add_months(Months::new(1)), None);
        assert_eq!(NaiveDate::MIN.checked_sub_months(Months::new(1)), None);
        assert_eq!(
            NaiveDate::from_ymd_opt(2022, 8, 3).unwrap()
            .checked_sub_months(Months::new(2050 * 12)), Some(NaiveDate::from_ymd_opt(-
            28, 8, 3).unwrap())
        );
        assert_eq!(
            NaiveDate::from_ymd_opt(2022, 8, 3).unwrap()
            .checked_add_months(Months::new(6)), Some(NaiveDate::from_ymd_opt(2023, 2, 3)
            .unwrap())
        );
        assert_eq!(
            NaiveDate::from_ymd_opt(2022, 8, 3).unwrap()
            .checked_sub_months(Months::new(10)), Some(NaiveDate::from_ymd_opt(2021, 10,
            3).unwrap())
        );
        assert_eq!(
            NaiveDate::from_ymd_opt(2022, 1, 29).unwrap()
            .checked_add_months(Months::new(1)), Some(NaiveDate::from_ymd_opt(2022, 2,
            28).unwrap())
        );
        assert_eq!(
            NaiveDate::from_ymd_opt(2022, 10, 29).unwrap()
            .checked_add_months(Months::new(16)), Some(NaiveDate::from_ymd_opt(2024, 2,
            29).unwrap())
        );
        assert_eq!(
            NaiveDate::from_ymd_opt(2022, 10, 31).unwrap()
            .checked_add_months(Months::new(2)), Some(NaiveDate::from_ymd_opt(2022, 12,
            31).unwrap())
        );
        assert_eq!(
            NaiveDate::from_ymd_opt(2022, 10, 31).unwrap()
            .checked_sub_months(Months::new(10)), Some(NaiveDate::from_ymd_opt(2021, 12,
            31).unwrap())
        );
        assert_eq!(
            NaiveDate::from_ymd_opt(2022, 8, 3).unwrap()
            .checked_add_months(Months::new(5)), Some(NaiveDate::from_ymd_opt(2023, 1, 3)
            .unwrap())
        );
        assert_eq!(
            NaiveDate::from_ymd_opt(2022, 8, 3).unwrap()
            .checked_sub_months(Months::new(7)), Some(NaiveDate::from_ymd_opt(2022, 1, 3)
            .unwrap())
        );
    }
    #[test]
    fn test_readme_doomsday() {
        use num_iter::range_inclusive;
        for y in range_inclusive(NaiveDate::MIN.year(), NaiveDate::MAX.year()) {
            let d4 = NaiveDate::from_ymd_opt(y, 4, 4).unwrap();
            let d6 = NaiveDate::from_ymd_opt(y, 6, 6).unwrap();
            let d8 = NaiveDate::from_ymd_opt(y, 8, 8).unwrap();
            let d10 = NaiveDate::from_ymd_opt(y, 10, 10).unwrap();
            let d12 = NaiveDate::from_ymd_opt(y, 12, 12).unwrap();
            let d59 = NaiveDate::from_ymd_opt(y, 5, 9).unwrap();
            let d95 = NaiveDate::from_ymd_opt(y, 9, 5).unwrap();
            let d711 = NaiveDate::from_ymd_opt(y, 7, 11).unwrap();
            let d117 = NaiveDate::from_ymd_opt(y, 11, 7).unwrap();
            let d30 = NaiveDate::from_ymd_opt(y, 3, 1).unwrap().pred_opt().unwrap();
            let weekday = d30.weekday();
            let other_dates = [d4, d6, d8, d10, d12, d59, d95, d711, d117];
            assert!(other_dates.iter().all(| d | d.weekday() == weekday));
        }
    }
    #[test]
    fn test_date_from_ymd() {
        let ymd_opt = NaiveDate::from_ymd_opt;
        assert!(ymd_opt(2012, 0, 1).is_none());
        assert!(ymd_opt(2012, 1, 1).is_some());
        assert!(ymd_opt(2012, 2, 29).is_some());
        assert!(ymd_opt(2014, 2, 29).is_none());
        assert!(ymd_opt(2014, 3, 0).is_none());
        assert!(ymd_opt(2014, 3, 1).is_some());
        assert!(ymd_opt(2014, 3, 31).is_some());
        assert!(ymd_opt(2014, 3, 32).is_none());
        assert!(ymd_opt(2014, 12, 31).is_some());
        assert!(ymd_opt(2014, 13, 1).is_none());
    }
    #[test]
    fn test_date_from_yo() {
        let yo_opt = NaiveDate::from_yo_opt;
        let ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();
        assert_eq!(yo_opt(2012, 0), None);
        assert_eq!(yo_opt(2012, 1), Some(ymd(2012, 1, 1)));
        assert_eq!(yo_opt(2012, 2), Some(ymd(2012, 1, 2)));
        assert_eq!(yo_opt(2012, 32), Some(ymd(2012, 2, 1)));
        assert_eq!(yo_opt(2012, 60), Some(ymd(2012, 2, 29)));
        assert_eq!(yo_opt(2012, 61), Some(ymd(2012, 3, 1)));
        assert_eq!(yo_opt(2012, 100), Some(ymd(2012, 4, 9)));
        assert_eq!(yo_opt(2012, 200), Some(ymd(2012, 7, 18)));
        assert_eq!(yo_opt(2012, 300), Some(ymd(2012, 10, 26)));
        assert_eq!(yo_opt(2012, 366), Some(ymd(2012, 12, 31)));
        assert_eq!(yo_opt(2012, 367), None);
        assert_eq!(yo_opt(2014, 0), None);
        assert_eq!(yo_opt(2014, 1), Some(ymd(2014, 1, 1)));
        assert_eq!(yo_opt(2014, 2), Some(ymd(2014, 1, 2)));
        assert_eq!(yo_opt(2014, 32), Some(ymd(2014, 2, 1)));
        assert_eq!(yo_opt(2014, 59), Some(ymd(2014, 2, 28)));
        assert_eq!(yo_opt(2014, 60), Some(ymd(2014, 3, 1)));
        assert_eq!(yo_opt(2014, 100), Some(ymd(2014, 4, 10)));
        assert_eq!(yo_opt(2014, 200), Some(ymd(2014, 7, 19)));
        assert_eq!(yo_opt(2014, 300), Some(ymd(2014, 10, 27)));
        assert_eq!(yo_opt(2014, 365), Some(ymd(2014, 12, 31)));
        assert_eq!(yo_opt(2014, 366), None);
    }
    #[test]
    fn test_date_from_isoywd() {
        let isoywd_opt = NaiveDate::from_isoywd_opt;
        let ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();
        assert_eq!(isoywd_opt(2004, 0, Weekday::Sun), None);
        assert_eq!(isoywd_opt(2004, 1, Weekday::Mon), Some(ymd(2003, 12, 29)));
        assert_eq!(isoywd_opt(2004, 1, Weekday::Sun), Some(ymd(2004, 1, 4)));
        assert_eq!(isoywd_opt(2004, 2, Weekday::Mon), Some(ymd(2004, 1, 5)));
        assert_eq!(isoywd_opt(2004, 2, Weekday::Sun), Some(ymd(2004, 1, 11)));
        assert_eq!(isoywd_opt(2004, 52, Weekday::Mon), Some(ymd(2004, 12, 20)));
        assert_eq!(isoywd_opt(2004, 52, Weekday::Sun), Some(ymd(2004, 12, 26)));
        assert_eq!(isoywd_opt(2004, 53, Weekday::Mon), Some(ymd(2004, 12, 27)));
        assert_eq!(isoywd_opt(2004, 53, Weekday::Sun), Some(ymd(2005, 1, 2)));
        assert_eq!(isoywd_opt(2004, 54, Weekday::Mon), None);
        assert_eq!(isoywd_opt(2011, 0, Weekday::Sun), None);
        assert_eq!(isoywd_opt(2011, 1, Weekday::Mon), Some(ymd(2011, 1, 3)));
        assert_eq!(isoywd_opt(2011, 1, Weekday::Sun), Some(ymd(2011, 1, 9)));
        assert_eq!(isoywd_opt(2011, 2, Weekday::Mon), Some(ymd(2011, 1, 10)));
        assert_eq!(isoywd_opt(2011, 2, Weekday::Sun), Some(ymd(2011, 1, 16)));
        assert_eq!(isoywd_opt(2018, 51, Weekday::Mon), Some(ymd(2018, 12, 17)));
        assert_eq!(isoywd_opt(2018, 51, Weekday::Sun), Some(ymd(2018, 12, 23)));
        assert_eq!(isoywd_opt(2018, 52, Weekday::Mon), Some(ymd(2018, 12, 24)));
        assert_eq!(isoywd_opt(2018, 52, Weekday::Sun), Some(ymd(2018, 12, 30)));
        assert_eq!(isoywd_opt(2018, 53, Weekday::Mon), None);
    }
    #[test]
    fn test_date_from_isoywd_and_iso_week() {
        for year in 2000..2401 {
            for week in 1..54 {
                for &weekday in [
                    Weekday::Mon,
                    Weekday::Tue,
                    Weekday::Wed,
                    Weekday::Thu,
                    Weekday::Fri,
                    Weekday::Sat,
                    Weekday::Sun,
                ]
                    .iter()
                {
                    let d = NaiveDate::from_isoywd_opt(year, week, weekday);
                    if let Some(d) = d {
                        assert_eq!(d.weekday(), weekday);
                        let w = d.iso_week();
                        assert_eq!(w.year(), year);
                        assert_eq!(w.week(), week);
                    }
                }
            }
        }
        for year in 2000..2401 {
            for month in 1..13 {
                for day in 1..32 {
                    let d = NaiveDate::from_ymd_opt(year, month, day);
                    if let Some(d) = d {
                        let w = d.iso_week();
                        let d_ = NaiveDate::from_isoywd_opt(
                            w.year(),
                            w.week(),
                            d.weekday(),
                        );
                        assert_eq!(d, d_.unwrap());
                    }
                }
            }
        }
    }
    #[test]
    fn test_date_from_num_days_from_ce() {
        let from_ndays_from_ce = NaiveDate::from_num_days_from_ce_opt;
        assert_eq!(
            from_ndays_from_ce(1), Some(NaiveDate::from_ymd_opt(1, 1, 1).unwrap())
        );
        assert_eq!(
            from_ndays_from_ce(2), Some(NaiveDate::from_ymd_opt(1, 1, 2).unwrap())
        );
        assert_eq!(
            from_ndays_from_ce(31), Some(NaiveDate::from_ymd_opt(1, 1, 31).unwrap())
        );
        assert_eq!(
            from_ndays_from_ce(32), Some(NaiveDate::from_ymd_opt(1, 2, 1).unwrap())
        );
        assert_eq!(
            from_ndays_from_ce(59), Some(NaiveDate::from_ymd_opt(1, 2, 28).unwrap())
        );
        assert_eq!(
            from_ndays_from_ce(60), Some(NaiveDate::from_ymd_opt(1, 3, 1).unwrap())
        );
        assert_eq!(
            from_ndays_from_ce(365), Some(NaiveDate::from_ymd_opt(1, 12, 31).unwrap())
        );
        assert_eq!(
            from_ndays_from_ce(365 + 1), Some(NaiveDate::from_ymd_opt(2, 1, 1).unwrap())
        );
        assert_eq!(
            from_ndays_from_ce(365 * 2 + 1), Some(NaiveDate::from_ymd_opt(3, 1, 1)
            .unwrap())
        );
        assert_eq!(
            from_ndays_from_ce(365 * 3 + 1), Some(NaiveDate::from_ymd_opt(4, 1, 1)
            .unwrap())
        );
        assert_eq!(
            from_ndays_from_ce(365 * 4 + 2), Some(NaiveDate::from_ymd_opt(5, 1, 1)
            .unwrap())
        );
        assert_eq!(
            from_ndays_from_ce(146097 + 1), Some(NaiveDate::from_ymd_opt(401, 1, 1)
            .unwrap())
        );
        assert_eq!(
            from_ndays_from_ce(146097 * 5 + 1), Some(NaiveDate::from_ymd_opt(2001, 1, 1)
            .unwrap())
        );
        assert_eq!(
            from_ndays_from_ce(719163), Some(NaiveDate::from_ymd_opt(1970, 1, 1)
            .unwrap())
        );
        assert_eq!(
            from_ndays_from_ce(0), Some(NaiveDate::from_ymd_opt(0, 12, 31).unwrap())
        );
        assert_eq!(
            from_ndays_from_ce(- 365), Some(NaiveDate::from_ymd_opt(0, 1, 1).unwrap())
        );
        assert_eq!(
            from_ndays_from_ce(- 366), Some(NaiveDate::from_ymd_opt(- 1, 12, 31)
            .unwrap())
        );
        for days in (-9999..10001).map(|x| x * 100) {
            assert_eq!(
                from_ndays_from_ce(days).map(| d | d.num_days_from_ce()), Some(days)
            );
        }
        assert_eq!(
            from_ndays_from_ce(NaiveDate::MIN.num_days_from_ce()), Some(NaiveDate::MIN)
        );
        assert_eq!(from_ndays_from_ce(NaiveDate::MIN.num_days_from_ce() - 1), None);
        assert_eq!(
            from_ndays_from_ce(NaiveDate::MAX.num_days_from_ce()), Some(NaiveDate::MAX)
        );
        assert_eq!(from_ndays_from_ce(NaiveDate::MAX.num_days_from_ce() + 1), None);
        assert_eq!(from_ndays_from_ce(i32::MIN), None);
        assert_eq!(from_ndays_from_ce(i32::MAX), None);
    }
    #[test]
    fn test_date_from_weekday_of_month_opt() {
        let ymwd = NaiveDate::from_weekday_of_month_opt;
        assert_eq!(ymwd(2018, 8, Weekday::Tue, 0), None);
        assert_eq!(
            ymwd(2018, 8, Weekday::Wed, 1), Some(NaiveDate::from_ymd_opt(2018, 8, 1)
            .unwrap())
        );
        assert_eq!(
            ymwd(2018, 8, Weekday::Thu, 1), Some(NaiveDate::from_ymd_opt(2018, 8, 2)
            .unwrap())
        );
        assert_eq!(
            ymwd(2018, 8, Weekday::Sun, 1), Some(NaiveDate::from_ymd_opt(2018, 8, 5)
            .unwrap())
        );
        assert_eq!(
            ymwd(2018, 8, Weekday::Mon, 1), Some(NaiveDate::from_ymd_opt(2018, 8, 6)
            .unwrap())
        );
        assert_eq!(
            ymwd(2018, 8, Weekday::Tue, 1), Some(NaiveDate::from_ymd_opt(2018, 8, 7)
            .unwrap())
        );
        assert_eq!(
            ymwd(2018, 8, Weekday::Wed, 2), Some(NaiveDate::from_ymd_opt(2018, 8, 8)
            .unwrap())
        );
        assert_eq!(
            ymwd(2018, 8, Weekday::Sun, 2), Some(NaiveDate::from_ymd_opt(2018, 8, 12)
            .unwrap())
        );
        assert_eq!(
            ymwd(2018, 8, Weekday::Thu, 3), Some(NaiveDate::from_ymd_opt(2018, 8, 16)
            .unwrap())
        );
        assert_eq!(
            ymwd(2018, 8, Weekday::Thu, 4), Some(NaiveDate::from_ymd_opt(2018, 8, 23)
            .unwrap())
        );
        assert_eq!(
            ymwd(2018, 8, Weekday::Thu, 5), Some(NaiveDate::from_ymd_opt(2018, 8, 30)
            .unwrap())
        );
        assert_eq!(
            ymwd(2018, 8, Weekday::Fri, 5), Some(NaiveDate::from_ymd_opt(2018, 8, 31)
            .unwrap())
        );
        assert_eq!(ymwd(2018, 8, Weekday::Sat, 5), None);
    }
    #[test]
    fn test_date_fields() {
        fn check(year: i32, month: u32, day: u32, ordinal: u32) {
            let d1 = NaiveDate::from_ymd_opt(year, month, day).unwrap();
            assert_eq!(d1.year(), year);
            assert_eq!(d1.month(), month);
            assert_eq!(d1.day(), day);
            assert_eq!(d1.ordinal(), ordinal);
            let d2 = NaiveDate::from_yo_opt(year, ordinal).unwrap();
            assert_eq!(d2.year(), year);
            assert_eq!(d2.month(), month);
            assert_eq!(d2.day(), day);
            assert_eq!(d2.ordinal(), ordinal);
            assert_eq!(d1, d2);
        }
        check(2012, 1, 1, 1);
        check(2012, 1, 2, 2);
        check(2012, 2, 1, 32);
        check(2012, 2, 29, 60);
        check(2012, 3, 1, 61);
        check(2012, 4, 9, 100);
        check(2012, 7, 18, 200);
        check(2012, 10, 26, 300);
        check(2012, 12, 31, 366);
        check(2014, 1, 1, 1);
        check(2014, 1, 2, 2);
        check(2014, 2, 1, 32);
        check(2014, 2, 28, 59);
        check(2014, 3, 1, 60);
        check(2014, 4, 10, 100);
        check(2014, 7, 19, 200);
        check(2014, 10, 27, 300);
        check(2014, 12, 31, 365);
    }
    #[test]
    fn test_date_weekday() {
        assert_eq!(
            NaiveDate::from_ymd_opt(1582, 10, 15).unwrap().weekday(), Weekday::Fri
        );
        assert_eq!(
            NaiveDate::from_ymd_opt(1875, 5, 20).unwrap().weekday(), Weekday::Thu
        );
        assert_eq!(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap().weekday(), Weekday::Sat);
    }
    #[test]
    fn test_date_with_fields() {
        let d = NaiveDate::from_ymd_opt(2000, 2, 29).unwrap();
        assert_eq!(
            d.with_year(- 400), Some(NaiveDate::from_ymd_opt(- 400, 2, 29).unwrap())
        );
        assert_eq!(d.with_year(- 100), None);
        assert_eq!(
            d.with_year(1600), Some(NaiveDate::from_ymd_opt(1600, 2, 29).unwrap())
        );
        assert_eq!(d.with_year(1900), None);
        assert_eq!(
            d.with_year(2000), Some(NaiveDate::from_ymd_opt(2000, 2, 29).unwrap())
        );
        assert_eq!(d.with_year(2001), None);
        assert_eq!(
            d.with_year(2004), Some(NaiveDate::from_ymd_opt(2004, 2, 29).unwrap())
        );
        assert_eq!(d.with_year(i32::MAX), None);
        let d = NaiveDate::from_ymd_opt(2000, 4, 30).unwrap();
        assert_eq!(d.with_month(0), None);
        assert_eq!(d.with_month(1), Some(NaiveDate::from_ymd_opt(2000, 1, 30).unwrap()));
        assert_eq!(d.with_month(2), None);
        assert_eq!(d.with_month(3), Some(NaiveDate::from_ymd_opt(2000, 3, 30).unwrap()));
        assert_eq!(d.with_month(4), Some(NaiveDate::from_ymd_opt(2000, 4, 30).unwrap()));
        assert_eq!(
            d.with_month(12), Some(NaiveDate::from_ymd_opt(2000, 12, 30).unwrap())
        );
        assert_eq!(d.with_month(13), None);
        assert_eq!(d.with_month(u32::MAX), None);
        let d = NaiveDate::from_ymd_opt(2000, 2, 8).unwrap();
        assert_eq!(d.with_day(0), None);
        assert_eq!(d.with_day(1), Some(NaiveDate::from_ymd_opt(2000, 2, 1).unwrap()));
        assert_eq!(d.with_day(29), Some(NaiveDate::from_ymd_opt(2000, 2, 29).unwrap()));
        assert_eq!(d.with_day(30), None);
        assert_eq!(d.with_day(u32::MAX), None);
        let d = NaiveDate::from_ymd_opt(2000, 5, 5).unwrap();
        assert_eq!(d.with_ordinal(0), None);
        assert_eq!(
            d.with_ordinal(1), Some(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap())
        );
        assert_eq!(
            d.with_ordinal(60), Some(NaiveDate::from_ymd_opt(2000, 2, 29).unwrap())
        );
        assert_eq!(
            d.with_ordinal(61), Some(NaiveDate::from_ymd_opt(2000, 3, 1).unwrap())
        );
        assert_eq!(
            d.with_ordinal(366), Some(NaiveDate::from_ymd_opt(2000, 12, 31).unwrap())
        );
        assert_eq!(d.with_ordinal(367), None);
        assert_eq!(d.with_ordinal(u32::MAX), None);
    }
    #[test]
    fn test_date_num_days_from_ce() {
        assert_eq!(NaiveDate::from_ymd_opt(1, 1, 1).unwrap().num_days_from_ce(), 1);
        for year in -9999..10001 {
            assert_eq!(
                NaiveDate::from_ymd_opt(year, 1, 1).unwrap().num_days_from_ce(),
                NaiveDate::from_ymd_opt(year - 1, 12, 31).unwrap().num_days_from_ce() + 1
            );
        }
    }
    #[test]
    fn test_date_succ() {
        let ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();
        assert_eq!(ymd(2014, 5, 6).succ_opt(), Some(ymd(2014, 5, 7)));
        assert_eq!(ymd(2014, 5, 31).succ_opt(), Some(ymd(2014, 6, 1)));
        assert_eq!(ymd(2014, 12, 31).succ_opt(), Some(ymd(2015, 1, 1)));
        assert_eq!(ymd(2016, 2, 28).succ_opt(), Some(ymd(2016, 2, 29)));
        assert_eq!(ymd(NaiveDate::MAX.year(), 12, 31).succ_opt(), None);
    }
    #[test]
    fn test_date_pred() {
        let ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();
        assert_eq!(ymd(2016, 3, 1).pred_opt(), Some(ymd(2016, 2, 29)));
        assert_eq!(ymd(2015, 1, 1).pred_opt(), Some(ymd(2014, 12, 31)));
        assert_eq!(ymd(2014, 6, 1).pred_opt(), Some(ymd(2014, 5, 31)));
        assert_eq!(ymd(2014, 5, 7).pred_opt(), Some(ymd(2014, 5, 6)));
        assert_eq!(ymd(NaiveDate::MIN.year(), 1, 1).pred_opt(), None);
    }
    #[test]
    fn test_date_add() {
        fn check(
            (y1, m1, d1): (i32, u32, u32),
            rhs: TimeDelta,
            ymd: Option<(i32, u32, u32)>,
        ) {
            let lhs = NaiveDate::from_ymd_opt(y1, m1, d1).unwrap();
            let sum = ymd.map(|(y, m, d)| NaiveDate::from_ymd_opt(y, m, d).unwrap());
            assert_eq!(lhs.checked_add_signed(rhs), sum);
            assert_eq!(lhs.checked_sub_signed(- rhs), sum);
        }
        check((2014, 1, 1), TimeDelta::zero(), Some((2014, 1, 1)));
        check((2014, 1, 1), TimeDelta::seconds(86399), Some((2014, 1, 1)));
        check((2014, 1, 1), TimeDelta::seconds(-86399), Some((2014, 1, 1)));
        check((2014, 1, 1), TimeDelta::days(1), Some((2014, 1, 2)));
        check((2014, 1, 1), TimeDelta::days(-1), Some((2013, 12, 31)));
        check((2014, 1, 1), TimeDelta::days(364), Some((2014, 12, 31)));
        check((2014, 1, 1), TimeDelta::days(365 * 4 + 1), Some((2018, 1, 1)));
        check((2014, 1, 1), TimeDelta::days(365 * 400 + 97), Some((2414, 1, 1)));
        check((-7, 1, 1), TimeDelta::days(365 * 12 + 3), Some((5, 1, 1)));
        check(
            (0, 1, 1),
            TimeDelta::days(MAX_DAYS_FROM_YEAR_0 as i64),
            Some((MAX_YEAR, 12, 31)),
        );
        check((0, 1, 1), TimeDelta::days(MAX_DAYS_FROM_YEAR_0 as i64 + 1), None);
        check((0, 1, 1), TimeDelta::max_value(), None);
        check(
            (0, 1, 1),
            TimeDelta::days(MIN_DAYS_FROM_YEAR_0 as i64),
            Some((MIN_YEAR, 1, 1)),
        );
        check((0, 1, 1), TimeDelta::days(MIN_DAYS_FROM_YEAR_0 as i64 - 1), None);
        check((0, 1, 1), TimeDelta::min_value(), None);
    }
    #[test]
    fn test_date_sub() {
        fn check(
            (y1, m1, d1): (i32, u32, u32),
            (y2, m2, d2): (i32, u32, u32),
            diff: TimeDelta,
        ) {
            let lhs = NaiveDate::from_ymd_opt(y1, m1, d1).unwrap();
            let rhs = NaiveDate::from_ymd_opt(y2, m2, d2).unwrap();
            assert_eq!(lhs.signed_duration_since(rhs), diff);
            assert_eq!(rhs.signed_duration_since(lhs), - diff);
        }
        check((2014, 1, 1), (2014, 1, 1), TimeDelta::zero());
        check((2014, 1, 2), (2014, 1, 1), TimeDelta::days(1));
        check((2014, 12, 31), (2014, 1, 1), TimeDelta::days(364));
        check((2015, 1, 3), (2014, 1, 1), TimeDelta::days(365 + 2));
        check((2018, 1, 1), (2014, 1, 1), TimeDelta::days(365 * 4 + 1));
        check((2414, 1, 1), (2014, 1, 1), TimeDelta::days(365 * 400 + 97));
        check(
            (MAX_YEAR, 12, 31),
            (0, 1, 1),
            TimeDelta::days(MAX_DAYS_FROM_YEAR_0 as i64),
        );
        check((MIN_YEAR, 1, 1), (0, 1, 1), TimeDelta::days(MIN_DAYS_FROM_YEAR_0 as i64));
    }
    #[test]
    fn test_date_add_days() {
        fn check(
            (y1, m1, d1): (i32, u32, u32),
            rhs: Days,
            ymd: Option<(i32, u32, u32)>,
        ) {
            let lhs = NaiveDate::from_ymd_opt(y1, m1, d1).unwrap();
            let sum = ymd.map(|(y, m, d)| NaiveDate::from_ymd_opt(y, m, d).unwrap());
            assert_eq!(lhs.checked_add_days(rhs), sum);
        }
        check((2014, 1, 1), Days::new(0), Some((2014, 1, 1)));
        check((2014, 1, 1), Days::new(1), Some((2014, 1, 2)));
        check((2014, 1, 1), Days::new(364), Some((2014, 12, 31)));
        check((2014, 1, 1), Days::new(365 * 4 + 1), Some((2018, 1, 1)));
        check((2014, 1, 1), Days::new(365 * 400 + 97), Some((2414, 1, 1)));
        check((-7, 1, 1), Days::new(365 * 12 + 3), Some((5, 1, 1)));
        check(
            (0, 1, 1),
            Days::new(MAX_DAYS_FROM_YEAR_0.try_into().unwrap()),
            Some((MAX_YEAR, 12, 31)),
        );
        check(
            (0, 1, 1),
            Days::new(u64::try_from(MAX_DAYS_FROM_YEAR_0).unwrap() + 1),
            None,
        );
    }
    #[test]
    fn test_date_sub_days() {
        fn check(
            (y1, m1, d1): (i32, u32, u32),
            (y2, m2, d2): (i32, u32, u32),
            diff: Days,
        ) {
            let lhs = NaiveDate::from_ymd_opt(y1, m1, d1).unwrap();
            let rhs = NaiveDate::from_ymd_opt(y2, m2, d2).unwrap();
            assert_eq!(lhs - diff, rhs);
        }
        check((2014, 1, 1), (2014, 1, 1), Days::new(0));
        check((2014, 1, 2), (2014, 1, 1), Days::new(1));
        check((2014, 12, 31), (2014, 1, 1), Days::new(364));
        check((2015, 1, 3), (2014, 1, 1), Days::new(365 + 2));
        check((2018, 1, 1), (2014, 1, 1), Days::new(365 * 4 + 1));
        check((2414, 1, 1), (2014, 1, 1), Days::new(365 * 400 + 97));
        check(
            (MAX_YEAR, 12, 31),
            (0, 1, 1),
            Days::new(MAX_DAYS_FROM_YEAR_0.try_into().unwrap()),
        );
        check(
            (0, 1, 1),
            (MIN_YEAR, 1, 1),
            Days::new((-MIN_DAYS_FROM_YEAR_0).try_into().unwrap()),
        );
    }
    #[test]
    fn test_date_addassignment() {
        let ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();
        let mut date = ymd(2016, 10, 1);
        date += TimeDelta::days(10);
        assert_eq!(date, ymd(2016, 10, 11));
        date += TimeDelta::days(30);
        assert_eq!(date, ymd(2016, 11, 10));
    }
    #[test]
    fn test_date_subassignment() {
        let ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();
        let mut date = ymd(2016, 10, 11);
        date -= TimeDelta::days(10);
        assert_eq!(date, ymd(2016, 10, 1));
        date -= TimeDelta::days(2);
        assert_eq!(date, ymd(2016, 9, 29));
    }
    #[test]
    fn test_date_fmt() {
        assert_eq!(
            format!("{:?}", NaiveDate::from_ymd_opt(2012, 3, 4).unwrap()), "2012-03-04"
        );
        assert_eq!(
            format!("{:?}", NaiveDate::from_ymd_opt(0, 3, 4).unwrap()), "0000-03-04"
        );
        assert_eq!(
            format!("{:?}", NaiveDate::from_ymd_opt(- 307, 3, 4).unwrap()), "-0307-03-04"
        );
        assert_eq!(
            format!("{:?}", NaiveDate::from_ymd_opt(12345, 3, 4).unwrap()),
            "+12345-03-04"
        );
        assert_eq!(
            NaiveDate::from_ymd_opt(2012, 3, 4).unwrap().to_string(), "2012-03-04"
        );
        assert_eq!(NaiveDate::from_ymd_opt(0, 3, 4).unwrap().to_string(), "0000-03-04");
        assert_eq!(
            NaiveDate::from_ymd_opt(- 307, 3, 4).unwrap().to_string(), "-0307-03-04"
        );
        assert_eq!(
            NaiveDate::from_ymd_opt(12345, 3, 4).unwrap().to_string(), "+12345-03-04"
        );
        assert_eq!(
            format!("{:+30?}", NaiveDate::from_ymd_opt(1234, 5, 6).unwrap()),
            "1234-05-06"
        );
        assert_eq!(
            format!("{:30?}", NaiveDate::from_ymd_opt(12345, 6, 7).unwrap()),
            "+12345-06-07"
        );
    }
    #[test]
    fn test_date_from_str() {
        let valid = [
            "-0000000123456-1-2",
            "-123456-1-2",
            "-12345-1-2",
            "-1234-12-31",
            "-7-6-5",
            "350-2-28",
            "360-02-29",
            "0360-02-29",
            "2015-2-18",
            "2015-02-18",
            "+70-2-18",
            "+70000-2-18",
            "+00007-2-18",
        ];
        for &s in &valid {
            eprintln!("test_date_from_str valid {:?}", s);
            let d = match s.parse::<NaiveDate>() {
                Ok(d) => d,
                Err(e) => panic!("parsing `{}` has failed: {}", s, e),
            };
            eprintln!("d {:?} (NaiveDate)", d);
            let s_ = format!("{:?}", d);
            eprintln!("s_ {:?}", s_);
            let d_ = match s_.parse::<NaiveDate>() {
                Ok(d) => d,
                Err(e) => {
                    panic!(
                        "`{}` is parsed into `{:?}`, but reparsing that has failed: {}",
                        s, d, e
                    )
                }
            };
            eprintln!("d_ {:?} (NaiveDate)", d_);
            assert!(
                d == d_,
                "`{}` is parsed into `{:?}`, but reparsed result \
                              `{:?}` does not match",
                s, d, d_
            );
        }
        let invalid = [
            "",
            "x",
            "Fri, 09 Aug 2013 GMT",
            "Sat Jun 30 2012",
            "1441497364.649",
            "+1441497364.649",
            "+1441497364",
            "2014/02/03",
            "2014",
            "2014-01",
            "2014-01-00",
            "2014-11-32",
            "2014-13-01",
            "2014-13-57",
            "2001 -02-03",
            "2001- 02-03",
            "2001 - 02-03",
            "2001-02 -03",
            "2001-02- 03",
            "2001-02 - 03",
            "2001-02-03 ",
            " 2001-02-03",
            "    -123456 - 1 - 2    ",
            "9999999-9-9",
        ];
        for &s in &invalid {
            eprintln!("test_date_from_str invalid {:?}", s);
            assert!(s.parse::< NaiveDate > ().is_err());
        }
    }
    #[test]
    fn test_date_parse_from_str() {
        let ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();
        assert_eq!(
            NaiveDate::parse_from_str("2014-5-7T12:34:56+09:30", "%Y-%m-%dT%H:%M:%S%z"),
            Ok(ymd(2014, 5, 7))
        );
        assert_eq!(
            NaiveDate::parse_from_str("2015-W06-1=2015-033", "%G-W%V-%u=%Y-%j"),
            Ok(ymd(2015, 2, 2))
        );
        assert_eq!(
            NaiveDate::parse_from_str("Fri, 09 Aug 13", "%a, %d %b %y"), Ok(ymd(2013, 8,
            9))
        );
        assert!(NaiveDate::parse_from_str("Sat, 09 Aug 2013", "%a, %d %b %Y").is_err());
        assert!(NaiveDate::parse_from_str("2014-57", "%Y-%m-%d").is_err());
        assert!(NaiveDate::parse_from_str("2014", "%Y").is_err());
        assert_eq!(
            NaiveDate::parse_from_str("2020-01-0", "%Y-%W-%w").ok(),
            NaiveDate::from_ymd_opt(2020, 1, 12),
        );
        assert_eq!(
            NaiveDate::parse_from_str("2019-01-0", "%Y-%W-%w").ok(),
            NaiveDate::from_ymd_opt(2019, 1, 13),
        );
    }
    #[test]
    fn test_date_format() {
        let d = NaiveDate::from_ymd_opt(2012, 3, 4).unwrap();
        assert_eq!(d.format("%Y,%C,%y,%G,%g").to_string(), "2012,20,12,2012,12");
        assert_eq!(d.format("%m,%b,%h,%B").to_string(), "03,Mar,Mar,March");
        assert_eq!(d.format("%d,%e").to_string(), "04, 4");
        assert_eq!(d.format("%U,%W,%V").to_string(), "10,09,09");
        assert_eq!(d.format("%a,%A,%w,%u").to_string(), "Sun,Sunday,0,7");
        assert_eq!(d.format("%j").to_string(), "064");
        assert_eq!(d.format("%D,%x").to_string(), "03/04/12,03/04/12");
        assert_eq!(d.format("%F").to_string(), "2012-03-04");
        assert_eq!(d.format("%v").to_string(), " 4-Mar-2012");
        assert_eq!(d.format("%t%n%%%n%t").to_string(), "\t\n%\n\t");
        assert_eq!(
            NaiveDate::from_ymd_opt(12345, 1, 1).unwrap().format("%Y").to_string(),
            "+12345"
        );
        assert_eq!(
            NaiveDate::from_ymd_opt(1234, 1, 1).unwrap().format("%Y").to_string(), "1234"
        );
        assert_eq!(
            NaiveDate::from_ymd_opt(123, 1, 1).unwrap().format("%Y").to_string(), "0123"
        );
        assert_eq!(
            NaiveDate::from_ymd_opt(12, 1, 1).unwrap().format("%Y").to_string(), "0012"
        );
        assert_eq!(
            NaiveDate::from_ymd_opt(1, 1, 1).unwrap().format("%Y").to_string(), "0001"
        );
        assert_eq!(
            NaiveDate::from_ymd_opt(0, 1, 1).unwrap().format("%Y").to_string(), "0000"
        );
        assert_eq!(
            NaiveDate::from_ymd_opt(- 1, 1, 1).unwrap().format("%Y").to_string(), "-0001"
        );
        assert_eq!(
            NaiveDate::from_ymd_opt(- 12, 1, 1).unwrap().format("%Y").to_string(),
            "-0012"
        );
        assert_eq!(
            NaiveDate::from_ymd_opt(- 123, 1, 1).unwrap().format("%Y").to_string(),
            "-0123"
        );
        assert_eq!(
            NaiveDate::from_ymd_opt(- 1234, 1, 1).unwrap().format("%Y").to_string(),
            "-1234"
        );
        assert_eq!(
            NaiveDate::from_ymd_opt(- 12345, 1, 1).unwrap().format("%Y").to_string(),
            "-12345"
        );
        assert_eq!(
            NaiveDate::from_ymd_opt(2007, 12, 31).unwrap().format("%G,%g,%U,%W,%V")
            .to_string(), "2008,08,52,53,01"
        );
        assert_eq!(
            NaiveDate::from_ymd_opt(2010, 1, 3).unwrap().format("%G,%g,%U,%W,%V")
            .to_string(), "2009,09,01,00,53"
        );
    }
    #[test]
    fn test_day_iterator_limit() {
        assert_eq!(
            NaiveDate::from_ymd_opt(262143, 12, 29).unwrap().iter_days().take(4).count(),
            2
        );
        assert_eq!(
            NaiveDate::from_ymd_opt(- 262144, 1, 3).unwrap().iter_days().rev().take(4)
            .count(), 2
        );
    }
    #[test]
    fn test_week_iterator_limit() {
        assert_eq!(
            NaiveDate::from_ymd_opt(262143, 12, 12).unwrap().iter_weeks().take(4)
            .count(), 2
        );
        assert_eq!(
            NaiveDate::from_ymd_opt(- 262144, 1, 15).unwrap().iter_weeks().rev().take(4)
            .count(), 2
        );
    }
    #[test]
    fn test_naiveweek() {
        let date = NaiveDate::from_ymd_opt(2022, 5, 18).unwrap();
        let asserts = vec![
            (Weekday::Mon, "2022-05-16", "2022-05-22"), (Weekday::Tue, "2022-05-17",
            "2022-05-23"), (Weekday::Wed, "2022-05-18", "2022-05-24"), (Weekday::Thu,
            "2022-05-12", "2022-05-18"), (Weekday::Fri, "2022-05-13", "2022-05-19"),
            (Weekday::Sat, "2022-05-14", "2022-05-20"), (Weekday::Sun, "2022-05-15",
            "2022-05-21"),
        ];
        for (start, first_day, last_day) in asserts {
            let week = date.week(start);
            let days = week.days();
            assert_eq!(
                Ok(week.first_day()), NaiveDate::parse_from_str(first_day, "%Y-%m-%d")
            );
            assert_eq!(
                Ok(week.last_day()), NaiveDate::parse_from_str(last_day, "%Y-%m-%d")
            );
            assert!(days.contains(& date));
        }
    }
    #[test]
    fn test_weeks_from() {
        assert_eq!(
            NaiveDate::parse_from_str("2020-01-0", "%Y-%W-%w").ok(),
            NaiveDate::from_ymd_opt(2020, 1, 12),
        );
        assert_eq!(
            NaiveDate::parse_from_str("2019-01-0", "%Y-%W-%w").ok(),
            NaiveDate::from_ymd_opt(2019, 1, 13),
        );
        for (y, starts_on) in &[
            (2019, Weekday::Tue),
            (2020, Weekday::Wed),
            (2021, Weekday::Fri),
            (2022, Weekday::Sat),
            (2023, Weekday::Sun),
            (2024, Weekday::Mon),
            (2025, Weekday::Wed),
            (2026, Weekday::Thu),
        ] {
            for day in &[
                Weekday::Mon,
                Weekday::Tue,
                Weekday::Wed,
                Weekday::Thu,
                Weekday::Fri,
                Weekday::Sat,
                Weekday::Sun,
            ] {
                assert_eq!(
                    NaiveDate::from_ymd_opt(* y, 1, 1).map(| d | d.weeks_from(* day)),
                    Some(if day == starts_on { 1 } else { 0 })
                );
                assert!(
                    [52, 53].contains(& NaiveDate::from_ymd_opt(* y, 12, 31).unwrap()
                    .weeks_from(* day)),
                );
            }
        }
        let base = NaiveDate::from_ymd_opt(2019, 1, 1).unwrap();
        for day in &[
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
            Weekday::Fri,
            Weekday::Sat,
            Weekday::Sun,
        ] {
            for dplus in 1..(400 * 366) {
                assert!((base + Days::new(dplus)).weeks_from(* day) < 54)
            }
        }
    }
    #[test]
    fn test_with_0_overflow() {
        let dt = NaiveDate::from_ymd_opt(2023, 4, 18).unwrap();
        assert!(dt.with_month0(4294967295).is_none());
        assert!(dt.with_day0(4294967295).is_none());
        assert!(dt.with_ordinal0(4294967295).is_none());
    }
}
#[cfg(test)]
mod tests_llm_16_83 {
    use super::*;
    use crate::*;
    use crate::NaiveDateTime;
    #[test]
    fn test_naive_datetime_from_naive_date() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let naive_date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let naive_datetime = NaiveDateTime::new(
            naive_date,
            NaiveTime::from_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5),
        );
        let result = NaiveDate::from(naive_datetime);
        debug_assert_eq!(naive_date, result);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_84 {
    use crate::NaiveDate;
    use std::default::Default;
    #[test]
    fn test_naive_date_default() {
        let _rug_st_tests_llm_16_84_rrrruuuugggg_test_naive_date_default = 0;
        let default_date = NaiveDate::default();
        debug_assert_eq!(default_date, NaiveDate::from_ymd(1970, 1, 1));
        let _rug_ed_tests_llm_16_84_rrrruuuugggg_test_naive_date_default = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_85 {
    use super::*;
    use crate::*;
    use crate::Months;
    #[test]
    fn test_add_months() {
        let _rug_st_tests_llm_16_85_rrrruuuugggg_test_add_months = 0;
        let rug_fuzz_0 = 2014;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 2014;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = 11;
        let rug_fuzz_8 = 2014;
        let rug_fuzz_9 = 1;
        let rug_fuzz_10 = 1;
        let rug_fuzz_11 = 12;
        let rug_fuzz_12 = 2014;
        let rug_fuzz_13 = 1;
        let rug_fuzz_14 = 1;
        let rug_fuzz_15 = 13;
        let rug_fuzz_16 = 2014;
        let rug_fuzz_17 = 1;
        let rug_fuzz_18 = 31;
        let rug_fuzz_19 = 1;
        let rug_fuzz_20 = 2020;
        let rug_fuzz_21 = 1;
        let rug_fuzz_22 = 31;
        let rug_fuzz_23 = 1;
        let rug_fuzz_24 = 2020;
        let rug_fuzz_25 = 1;
        let rug_fuzz_26 = 31;
        let rug_fuzz_27 = 13;
        let from_ymd = NaiveDate::from_ymd;
        debug_assert_eq!(
            from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2) + Months::new(rug_fuzz_3),
            from_ymd(2014, 2, 1)
        );
        debug_assert_eq!(
            from_ymd(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6) + Months::new(rug_fuzz_7),
            from_ymd(2014, 12, 1)
        );
        debug_assert_eq!(
            from_ymd(rug_fuzz_8, rug_fuzz_9, rug_fuzz_10) + Months::new(rug_fuzz_11),
            from_ymd(2015, 1, 1)
        );
        debug_assert_eq!(
            from_ymd(rug_fuzz_12, rug_fuzz_13, rug_fuzz_14) + Months::new(rug_fuzz_15),
            from_ymd(2015, 2, 1)
        );
        debug_assert_eq!(
            from_ymd(rug_fuzz_16, rug_fuzz_17, rug_fuzz_18) + Months::new(rug_fuzz_19),
            from_ymd(2014, 2, 28)
        );
        debug_assert_eq!(
            from_ymd(rug_fuzz_20, rug_fuzz_21, rug_fuzz_22) + Months::new(rug_fuzz_23),
            from_ymd(2020, 2, 29)
        );
        debug_assert_eq!(
            from_ymd(rug_fuzz_24, rug_fuzz_25, rug_fuzz_26) + Months::new(rug_fuzz_27),
            from_ymd(2021, 2, 28)
        );
        let _rug_ed_tests_llm_16_85_rrrruuuugggg_test_add_months = 0;
    }
    #[test]
    #[should_panic]
    fn test_add_months_panic() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let _ = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            + Months::new(u32::MAX);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_86 {
    use super::*;
    use crate::*;
    use crate::naive::date::NaiveDate;
    use crate::naive::date::Days;
    #[test]
    fn test_addition_of_days_to_date() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(i32, u32, u32, u64, i32, u32, u32, u64, i32, u32, u32, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date1 = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        let added_days1 = Days::new(rug_fuzz_3);
        let result_date1 = date1 + added_days1;
        debug_assert_eq!(result_date1, NaiveDate::from_ymd_opt(2023, 3, 24).unwrap());
        let date2 = NaiveDate::from_ymd_opt(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6).unwrap();
        let added_days2 = Days::new(rug_fuzz_7);
        let result_date2 = date2 + added_days2;
        debug_assert_eq!(result_date2, NaiveDate::from_ymd_opt(2023, 4, 4).unwrap());
        let date3 = NaiveDate::from_ymd_opt(rug_fuzz_8, rug_fuzz_9, rug_fuzz_10)
            .unwrap();
        let added_days3 = Days::new(rug_fuzz_11);
        let result_date3 = date3 + added_days3;
        debug_assert_eq!(result_date3, NaiveDate::from_ymd_opt(2023, 1, 4).unwrap());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_87 {
    use super::*;
    use crate::*;
    use crate::{NaiveDate, TimeDelta};
    #[test]
    fn test_add_time_delta_overflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u32, u32, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let delta = TimeDelta::days(rug_fuzz_3);
        let result = date.add(delta);
        debug_assert_eq!(result, NaiveDate::from_ymd(262143, 12, 31));
             }
});    }
    #[test]
    fn test_add_time_delta_underflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u32, u32, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(-rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let delta = TimeDelta::days(-rug_fuzz_3);
        let result = date.add(delta);
        debug_assert_eq!(result, NaiveDate::from_ymd(- 262144, 1, 1));
             }
});    }
    #[test]
    fn test_add_time_delta_no_overflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u32, u32, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let delta = TimeDelta::days(rug_fuzz_3);
        let result = date.add(delta);
        debug_assert_eq!(result, NaiveDate::from_ymd(2000, 1, 31));
             }
});    }
    #[test]
    #[should_panic(expected = "`NaiveDate + TimeDelta` overflowed")]
    fn test_add_time_delta_overflow_panic() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u32, u32, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let delta = TimeDelta::days(rug_fuzz_3);
        let _result = date.add(delta);
             }
});    }
    #[test]
    #[should_panic(expected = "`NaiveDate + TimeDelta` overflowed")]
    fn test_add_time_delta_underflow_panic() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u32, u32, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(-rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let delta = TimeDelta::days(-rug_fuzz_3);
        let _result = date.add(delta);
             }
});    }
}
#[test]
fn test_add_assign_overflow() {
    let mut date = NaiveDate::from_ymd_opt(2023, 3, 15).unwrap();
    let delta = TimeDelta::days(i64::MAX);
    let result = std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| {
            AddAssign::add_assign(&mut date, delta);
        }),
    );
    assert!(result.is_err());
}
#[cfg(test)]
mod tests_llm_16_89 {
    use crate::{NaiveDate, Months};
    #[test]
    fn test_sub_months() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15)) = <(i32, u32, u32, u32, i32, u32, u32, u32, i32, u32, u32, u32, i32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let from_ymd = NaiveDate::from_ymd;
        debug_assert_eq!(
            from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2) - Months::new(rug_fuzz_3),
            from_ymd(2014, 1, 1)
        );
        debug_assert_eq!(
            from_ymd(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6) - Months::new(rug_fuzz_7),
            from_ymd(2013, 12, 1)
        );
        debug_assert_eq!(
            from_ymd(rug_fuzz_8, rug_fuzz_9, rug_fuzz_10) - Months::new(rug_fuzz_11),
            from_ymd(2015, 3, 1)
        );
        debug_assert_eq!(
            from_ymd(rug_fuzz_12, rug_fuzz_13, rug_fuzz_14) - Months::new(rug_fuzz_15),
            from_ymd(2014, 2, 28)
        );
             }
});    }
    #[test]
    #[should_panic]
    fn test_sub_months_panic() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let _ = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            - Months::new(rug_fuzz_3);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_90 {
    use super::*;
    use crate::*;
    use crate::naive::date::Days;
    #[test]
    fn test_sub_days() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u32, u32, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let two_days = Days::new(rug_fuzz_3);
        let result_date = date - two_days;
        debug_assert_eq!(result_date, NaiveDate::from_ymd(2023, 4, 8));
             }
});    }
    #[test]
    #[should_panic(expected = "`NaiveDate - Days` overflowed")]
    fn test_sub_days_panic() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u32, u32, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let one_day = Days::new(rug_fuzz_3);
        let _ = date - one_day;
             }
});    }
    #[test]
    fn test_sub_days_boundary() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u32, u32, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let one_day = Days::new(rug_fuzz_3);
        let result_date = date - one_day;
        debug_assert_eq!(result_date, NaiveDate::from_ymd(262145, 1, 1));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_91 {
    use super::*;
    use crate::*;
    use crate::{NaiveDate, TimeDelta, Months, Days};
    #[test]
    fn test_sub_timedelta() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, u32, u32, i64, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let timedelta = TimeDelta::days(rug_fuzz_3);
        let expected = NaiveDate::from_ymd(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        debug_assert_eq!(date - timedelta, expected);
             }
});    }
    #[test]
    #[should_panic]
    fn test_sub_timedelta_panic_on_underflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u32, u32, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let timedelta = TimeDelta::days(rug_fuzz_3);
        let _ = date - timedelta;
             }
});    }
    #[test]
    fn test_sub_months() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, u32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let months = Months::new(rug_fuzz_3);
        let expected = NaiveDate::from_ymd(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        debug_assert_eq!(date - months, expected);
             }
});    }
    #[test]
    fn test_sub_months_end_of_month() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, u32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let months = Months::new(rug_fuzz_3);
        let expected = NaiveDate::from_ymd(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        debug_assert_eq!(date - months, expected);
             }
});    }
    #[test]
    fn test_sub_days() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, u32, u32, u64, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let days = Days::new(rug_fuzz_3);
        let expected = NaiveDate::from_ymd(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        debug_assert_eq!(date - days, expected);
             }
});    }
    #[test]
    fn test_sub_days_subtract_leap_day() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, u32, u32, u64, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let days = Days::new(rug_fuzz_3);
        let expected = NaiveDate::from_ymd(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        debug_assert_eq!(date - days, expected);
             }
});    }
    #[test]
    #[should_panic]
    fn test_sub_days_panic_on_underflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u32, u32, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let days = Days::new(rug_fuzz_3);
        let _ = date - days;
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_92 {
    use super::*;
    use crate::*;
    use crate::{NaiveDate, TimeDelta};
    #[test]
    fn test_subtract_dates() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8)) = <(i32, u32, u32, i32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date1 = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let date2 = NaiveDate::from_ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let date3 = NaiveDate::from_ymd(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8);
        let delta1 = date1.sub(date2);
        let delta2 = date1.sub(date3);
        debug_assert_eq!(delta1, TimeDelta::days(26));
        debug_assert_eq!(delta2, TimeDelta::days(- 10));
             }
});    }
    #[test]
    fn test_subtract_dates_inverse() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date1 = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let date2 = NaiveDate::from_ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let delta = date2.sub(date1);
        debug_assert_eq!(delta, TimeDelta::days(- 26));
             }
});    }
    #[test]
    fn test_subtract_dates_with_overflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date1 = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let date2 = NaiveDate::from_ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let delta = date1.sub(date2);
        debug_assert_eq!(delta, TimeDelta::days(- 95747736));
             }
});    }
    #[test]
    fn test_subtract_dates_with_underflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date1 = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let date2 = NaiveDate::from_ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let delta = date1.sub(date2);
        debug_assert_eq!(delta, TimeDelta::days(95747736));
             }
});    }
    #[test]
    #[should_panic(expected = "`NaiveDate - TimeDelta` overflowed")]
    fn test_subtract_dates_with_panic() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date1 = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let date2 = NaiveDate::from_ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let _ = date2.sub(date1);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_93 {
    use crate::NaiveDate;
    use crate::TimeDelta;
    #[test]
    fn test_sub_assign_timedelta() {
        let _rug_st_tests_llm_16_93_rrrruuuugggg_test_sub_assign_timedelta = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 4;
        let rug_fuzz_2 = 10;
        let rug_fuzz_3 = 5;
        let rug_fuzz_4 = 2023;
        let rug_fuzz_5 = 4;
        let rug_fuzz_6 = 5;
        let rug_fuzz_7 = 2023;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 1;
        let rug_fuzz_10 = 1;
        let rug_fuzz_11 = 2022;
        let rug_fuzz_12 = 12;
        let rug_fuzz_13 = 31;
        let rug_fuzz_14 = 2023;
        let rug_fuzz_15 = 3;
        let rug_fuzz_16 = 1;
        let rug_fuzz_17 = 1;
        let rug_fuzz_18 = 2023;
        let rug_fuzz_19 = 2;
        let rug_fuzz_20 = 22;
        let rug_fuzz_21 = 2024;
        let rug_fuzz_22 = 2;
        let rug_fuzz_23 = 29;
        let rug_fuzz_24 = 365;
        let rug_fuzz_25 = 2023;
        let rug_fuzz_26 = 2;
        let rug_fuzz_27 = 28;
        let mut date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        date -= TimeDelta::days(rug_fuzz_3);
        debug_assert_eq!(NaiveDate::from_ymd(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6), date);
        let mut date = NaiveDate::from_ymd(rug_fuzz_7, rug_fuzz_8, rug_fuzz_9);
        date -= TimeDelta::days(rug_fuzz_10);
        debug_assert_eq!(
            NaiveDate::from_ymd(rug_fuzz_11, rug_fuzz_12, rug_fuzz_13), date
        );
        let mut date = NaiveDate::from_ymd(rug_fuzz_14, rug_fuzz_15, rug_fuzz_16);
        date -= TimeDelta::weeks(rug_fuzz_17);
        debug_assert_eq!(
            NaiveDate::from_ymd(rug_fuzz_18, rug_fuzz_19, rug_fuzz_20), date
        );
        let mut date = NaiveDate::from_ymd(rug_fuzz_21, rug_fuzz_22, rug_fuzz_23);
        date -= TimeDelta::days(rug_fuzz_24);
        debug_assert_eq!(
            NaiveDate::from_ymd(rug_fuzz_25, rug_fuzz_26, rug_fuzz_27), date
        );
        let _rug_ed_tests_llm_16_93_rrrruuuugggg_test_sub_assign_timedelta = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_95 {
    use crate::{NaiveDate, Datelike};
    #[test]
    fn test_day() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14)) = <(i32, u32, u32, i32, u32, u32, i32, u32, u32, i32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date1 = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(date1.day(), 5);
        let date2 = NaiveDate::from_ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        debug_assert_eq!(date2.day(), 29);
        let date3 = NaiveDate::from_ymd(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8);
        debug_assert_eq!(date3.day(), 31);
        let date4 = NaiveDate::from_ymd(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11);
        debug_assert_eq!(date4.day(), 28);
        let date5 = NaiveDate::from_ymd(rug_fuzz_12, rug_fuzz_13, rug_fuzz_14);
        debug_assert_eq!(date5.day(), 29);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_96 {
    use super::*;
    use crate::*;
    use crate::NaiveDate;
    #[test]
    fn test_day0() {
        let _rug_st_tests_llm_16_96_rrrruuuugggg_test_day0 = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 2023;
        let rug_fuzz_4 = 1;
        let rug_fuzz_5 = 31;
        let rug_fuzz_6 = 2023;
        let rug_fuzz_7 = 2;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 2023;
        let rug_fuzz_10 = 12;
        let rug_fuzz_11 = 31;
        let rug_fuzz_12 = 2023;
        let rug_fuzz_13 = 2;
        let rug_fuzz_14 = 28;
        let rug_fuzz_15 = 2023;
        let rug_fuzz_16 = 3;
        let rug_fuzz_17 = 15;
        let rug_fuzz_18 = 2023;
        let rug_fuzz_19 = 4;
        let rug_fuzz_20 = 30;
        let rug_fuzz_21 = 2023;
        let rug_fuzz_22 = 7;
        let rug_fuzz_23 = 4;
        let rug_fuzz_24 = 2023;
        let rug_fuzz_25 = 10;
        let rug_fuzz_26 = 10;
        let rug_fuzz_27 = 2024;
        let rug_fuzz_28 = 2;
        let rug_fuzz_29 = 29;
        debug_assert_eq!(
            NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).day0(), 0
        );
        debug_assert_eq!(
            NaiveDate::from_ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5).day0(), 30
        );
        debug_assert_eq!(
            NaiveDate::from_ymd(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8).day0(), 0
        );
        debug_assert_eq!(
            NaiveDate::from_ymd(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11).day0(), 30
        );
        debug_assert_eq!(
            NaiveDate::from_ymd(rug_fuzz_12, rug_fuzz_13, rug_fuzz_14).day0(), 27
        );
        debug_assert_eq!(
            NaiveDate::from_ymd(rug_fuzz_15, rug_fuzz_16, rug_fuzz_17).day0(), 14
        );
        debug_assert_eq!(
            NaiveDate::from_ymd(rug_fuzz_18, rug_fuzz_19, rug_fuzz_20).day0(), 29
        );
        debug_assert_eq!(
            NaiveDate::from_ymd(rug_fuzz_21, rug_fuzz_22, rug_fuzz_23).day0(), 3
        );
        debug_assert_eq!(
            NaiveDate::from_ymd(rug_fuzz_24, rug_fuzz_25, rug_fuzz_26).day0(), 9
        );
        debug_assert_eq!(
            NaiveDate::from_ymd(rug_fuzz_27, rug_fuzz_28, rug_fuzz_29).day0(), 28
        );
        let _rug_ed_tests_llm_16_96_rrrruuuugggg_test_day0 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_97_date_iso_week {
    use crate::{Datelike, NaiveDate, Weekday};
    #[test]
    fn test_iso_week() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let test_date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let iso_week = test_date.iso_week();
        debug_assert_eq!(iso_week.year(), 2015);
        debug_assert_eq!(iso_week.week(), 48);
        debug_assert_eq!(iso_week.week0(), 47);
             }
});    }
    #[test]
    fn test_iso_week_first_week() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let test_date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let iso_week = test_date.iso_week();
        debug_assert_eq!(iso_week.year(), 2009);
        debug_assert_eq!(iso_week.week(), 53);
        debug_assert_eq!(iso_week.week0(), 52);
             }
});    }
    #[test]
    fn test_iso_week_year_boundary() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let test_date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let iso_week = test_date.iso_week();
        debug_assert_eq!(iso_week.year(), 2013);
        debug_assert_eq!(iso_week.week(), 1);
        debug_assert_eq!(iso_week.week0(), 0);
             }
});    }
    #[test]
    fn test_iso_week_week_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let test_date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let iso_week = test_date.iso_week();
        debug_assert_eq!(iso_week.year(), 2019);
        debug_assert_eq!(iso_week.week(), 1);
        debug_assert_eq!(iso_week.week0(), 0);
             }
});    }
    #[test]
    fn test_iso_week_end_year() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let test_date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let iso_week = test_date.iso_week();
        debug_assert_eq!(iso_week.year(), 2020);
        debug_assert_eq!(iso_week.week(), 53);
        debug_assert_eq!(iso_week.week0(), 52);
             }
});    }
    #[test]
    fn test_iso_week_last_possible_day() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let test_date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let iso_week = test_date.iso_week();
        debug_assert_eq!(iso_week.year(), 262143);
        debug_assert_eq!(iso_week.week(), 52);
        debug_assert_eq!(iso_week.week0(), 51);
             }
});    }
    #[test]
    fn test_iso_week_out_of_range_date() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let test_date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let iso_week = test_date.iso_week();
        debug_assert_eq!(iso_week.year(), 262145);
        debug_assert_eq!(iso_week.week(), 1);
        debug_assert_eq!(iso_week.week0(), 0);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_98 {
    use super::*;
    use crate::*;
    use crate::{NaiveDate, Datelike};
    #[test]
    fn test_month() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14)) = <(i32, u32, u32, i32, u32, u32, i32, u32, u32, i32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).month(), 1
        );
        debug_assert_eq!(
            NaiveDate::from_ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5).month(), 12
        );
        debug_assert_eq!(
            NaiveDate::from_ymd(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8).month(), 6
        );
        debug_assert_eq!(
            NaiveDate::from_ymd(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11).month(), 2
        );
        debug_assert_eq!(
            NaiveDate::from_ymd(rug_fuzz_12, rug_fuzz_13, rug_fuzz_14).month(), 3
        );
             }
});    }
    #[test]
    #[should_panic]
    fn test_month_panic_below_range() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).month();
             }
});    }
    #[test]
    #[should_panic]
    fn test_month_panic_above_range() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).month();
             }
});    }
    #[test]
    fn test_month_edge_cases() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).month(), 1
        );
        debug_assert_eq!(
            NaiveDate::from_ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5).month(), 2
        );
             }
});    }
    #[test]
    fn test_month_on_leap_year() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8)) = <(i32, u32, u32, i32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).month(), 2
        );
        debug_assert_eq!(
            NaiveDate::from_ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5).month(), 2
        );
        debug_assert_eq!(
            NaiveDate::from_ymd(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8).month(), 3
        );
             }
});    }
    #[test]
    fn test_month_with_negative_year() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            NaiveDate::from_ymd(- rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).month(), 1
        );
        debug_assert_eq!(
            NaiveDate::from_ymd(- rug_fuzz_3, rug_fuzz_4, rug_fuzz_5).month(), 2
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_99 {
    use super::*;
    use crate::*;
    use crate::Datelike;
    #[test]
    fn test_month0() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17, mut rug_fuzz_18, mut rug_fuzz_19, mut rug_fuzz_20)) = <(i32, u32, u32, i32, u32, u32, i32, u32, u32, i32, u32, u32, i32, u32, u32, i32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date1 = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(date1.month0(), 0);
        let date2 = NaiveDate::from_ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        debug_assert_eq!(date2.month0(), 5);
        let date3 = NaiveDate::from_ymd(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8);
        debug_assert_eq!(date3.month0(), 11);
        let date4 = NaiveDate::from_ymd(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11);
        debug_assert_eq!(date4.month0(), 0);
        let date5 = NaiveDate::from_ymd(-rug_fuzz_12, rug_fuzz_13, rug_fuzz_14);
        debug_assert_eq!(date5.month0(), 11);
        let date6 = NaiveDate::from_ymd(rug_fuzz_15, rug_fuzz_16, rug_fuzz_17);
        debug_assert_eq!(date6.month0(), 1);
        let date7 = NaiveDate::from_ymd(rug_fuzz_18, rug_fuzz_19, rug_fuzz_20);
        debug_assert_eq!(date7.month0(), 1);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_100 {
    use super::*;
    use crate::*;
    use crate::{Datelike, NaiveDate};
    #[test]
    fn test_ordinal() {
        let _rug_st_tests_llm_16_100_rrrruuuugggg_test_ordinal = 0;
        let rug_fuzz_0 = 2021;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 2021;
        let rug_fuzz_4 = 12;
        let rug_fuzz_5 = 31;
        let rug_fuzz_6 = 2020;
        let rug_fuzz_7 = 1;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 2020;
        let rug_fuzz_10 = 2;
        let rug_fuzz_11 = 29;
        let rug_fuzz_12 = 2020;
        let rug_fuzz_13 = 12;
        let rug_fuzz_14 = 31;
        let rug_fuzz_15 = 2020;
        let rug_fuzz_16 = 3;
        let rug_fuzz_17 = 1;
        let rug_fuzz_18 = 2021;
        let rug_fuzz_19 = 1;
        let rug_fuzz_20 = 1;
        let rug_fuzz_21 = 2021;
        let rug_fuzz_22 = 12;
        let rug_fuzz_23 = 31;
        let rug_fuzz_24 = 0;
        let rug_fuzz_25 = 1;
        let rug_fuzz_26 = 1;
        let rug_fuzz_27 = 1;
        let rug_fuzz_28 = 1;
        let rug_fuzz_29 = 1;
        let date_common = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(date_common.ordinal(), 1);
        let date_common_later = NaiveDate::from_ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        debug_assert_eq!(date_common_later.ordinal(), 365);
        let date_leap = NaiveDate::from_ymd(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8);
        debug_assert_eq!(date_leap.ordinal(), 1);
        let date_leap_feb29 = NaiveDate::from_ymd(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11);
        debug_assert_eq!(date_leap_feb29.ordinal(), 60);
        let date_leap_later = NaiveDate::from_ymd(rug_fuzz_12, rug_fuzz_13, rug_fuzz_14);
        debug_assert_eq!(date_leap_later.ordinal(), 366);
        let date_after_leap = NaiveDate::from_ymd(rug_fuzz_15, rug_fuzz_16, rug_fuzz_17);
        debug_assert_eq!(date_after_leap.ordinal(), 61);
        let date_beginning = NaiveDate::from_ymd(rug_fuzz_18, rug_fuzz_19, rug_fuzz_20);
        debug_assert_eq!(date_beginning.ordinal(), 1);
        let date_end = NaiveDate::from_ymd(rug_fuzz_21, rug_fuzz_22, rug_fuzz_23);
        debug_assert_eq!(date_end.ordinal(), 365);
        let date_year_zero = NaiveDate::from_ymd(rug_fuzz_24, rug_fuzz_25, rug_fuzz_26);
        debug_assert_eq!(date_year_zero.ordinal(), 1);
        let date_negative_year = NaiveDate::from_ymd(
            -rug_fuzz_27,
            rug_fuzz_28,
            rug_fuzz_29,
        );
        debug_assert_eq!(date_negative_year.ordinal(), 1);
        let _rug_ed_tests_llm_16_100_rrrruuuugggg_test_ordinal = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_101 {
    use crate::NaiveDate;
    use crate::Datelike;
    #[test]
    fn test_ordinal0() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17, mut rug_fuzz_18, mut rug_fuzz_19)) = <(i32, u32, u32, i32, u32, u32, i32, u32, u32, i32, u32, u32, i32, u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date1 = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        let date2 = NaiveDate::from_ymd_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5).unwrap();
        let date3 = NaiveDate::from_ymd_opt(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8).unwrap();
        let date4 = NaiveDate::from_ymd_opt(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11)
            .unwrap();
        let date5 = NaiveDate::from_ymd_opt(rug_fuzz_12, rug_fuzz_13, rug_fuzz_14)
            .unwrap();
        debug_assert_eq!(rug_fuzz_15, date1.ordinal0());
        debug_assert_eq!(rug_fuzz_16, date2.ordinal0());
        debug_assert_eq!(rug_fuzz_17, date3.ordinal0());
        debug_assert_eq!(rug_fuzz_18, date4.ordinal0());
        debug_assert_eq!(rug_fuzz_19, date5.ordinal0());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_102 {
    use crate::{Weekday, NaiveDate, Datelike};
    #[test]
    fn test_weekday_for_weekday_monday() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(date.weekday(), Weekday::Mon);
             }
});    }
    #[test]
    fn test_weekday_for_weekday_tuesday() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(date.weekday(), Weekday::Tue);
             }
});    }
    #[test]
    fn test_weekday_for_weekday_wednesday() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(date.weekday(), Weekday::Wed);
             }
});    }
    #[test]
    fn test_weekday_for_weekday_thursday() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(date.weekday(), Weekday::Thu);
             }
});    }
    #[test]
    fn test_weekday_for_weekday_friday() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(date.weekday(), Weekday::Fri);
             }
});    }
    #[test]
    fn test_weekday_for_weekday_saturday() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(date.weekday(), Weekday::Sat);
             }
});    }
    #[test]
    fn test_weekday_for_weekday_sunday() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(date.weekday(), Weekday::Sun);
             }
});    }
    #[test]
    fn test_weekday_for_leap_year() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(date.weekday(), Weekday::Thu);
             }
});    }
    #[test]
    fn test_weekday_for_non_leap_year() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(date.weekday(), Weekday::Tue);
             }
});    }
    #[test]
    fn test_weekday_for_earliest_possible_date() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(-rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(date.weekday(), Weekday::Tue);
             }
});    }
    #[test]
    fn test_weekday_for_latest_possible_date() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(date.weekday(), Weekday::Fri);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_103 {
    use super::*;
    use crate::*;
    use crate::{NaiveDate, Datelike};
    #[test]
    fn test_with_day() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17, mut rug_fuzz_18, mut rug_fuzz_19, mut rug_fuzz_20, mut rug_fuzz_21, mut rug_fuzz_22)) = <(i32, u32, u32, u32, i32, u32, u32, u32, u32, i32, u32, u32, u32, u32, u32, i32, u32, u32, u32, i32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(
            date.with_day(rug_fuzz_3), Some(NaiveDate::from_ymd(2023, 3, 20))
        );
        debug_assert_eq!(
            NaiveDate::from_ymd(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6).with_day(rug_fuzz_7),
            None
        );
        debug_assert_eq!(
            date.with_day(rug_fuzz_8), Some(NaiveDate::from_ymd(2023, 3, 15))
        );
        debug_assert_eq!(
            NaiveDate::from_ymd(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11)
            .with_day(rug_fuzz_12), Some(NaiveDate::from_ymd(2023, 5, 30))
        );
        debug_assert_eq!(date.with_day(rug_fuzz_13), None);
        debug_assert_eq!(date.with_day(rug_fuzz_14), None);
        debug_assert_eq!(
            NaiveDate::from_ymd(rug_fuzz_15, rug_fuzz_16, rug_fuzz_17)
            .with_day(rug_fuzz_18), Some(NaiveDate::from_ymd(2024, 2, 29))
        );
        debug_assert_eq!(
            NaiveDate::from_ymd(rug_fuzz_19, rug_fuzz_20, rug_fuzz_21)
            .with_day(rug_fuzz_22), None
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_104 {
    use super::*;
    use crate::*;
    use crate::{NaiveDate, Datelike};
    #[test]
    fn test_with_day0_success() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, u32, u32, i32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        let expected = NaiveDate::from_ymd_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .unwrap();
        debug_assert_eq!(date.with_day0(rug_fuzz_6), Some(expected));
             }
});    }
    #[test]
    fn test_with_day0_last_day() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, u32, u32, i32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        let expected = NaiveDate::from_ymd_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .unwrap();
        debug_assert_eq!(date.with_day0(rug_fuzz_6), Some(expected));
             }
});    }
    #[test]
    fn test_with_day0_overflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        debug_assert_eq!(date.with_day0(rug_fuzz_3), None);
             }
});    }
    #[test]
    fn test_with_day0_underflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        debug_assert_eq!(date.with_day0(u32::MAX), None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_105 {
    use super::*;
    use crate::*;
    use crate::naive::date::NaiveDate;
    #[test]
    fn with_month_changes_month() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        debug_assert_eq!(
            date.with_month(rug_fuzz_3).unwrap(), NaiveDate::from_ymd(2015, 5, 14)
        );
             }
});    }
    #[test]
    fn with_month_handles_incorrect_month() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        debug_assert!(date.with_month(rug_fuzz_3).is_none());
        debug_assert!(date.with_month(rug_fuzz_4).is_none());
             }
});    }
    #[test]
    fn with_month_preserves_day() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        debug_assert_eq!(
            date.with_month(rug_fuzz_3).unwrap(), NaiveDate::from_ymd(2015, 4, 30)
        );
             }
});    }
    #[test]
    fn with_month_handles_last_day_of_month() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        debug_assert!(date.with_month(rug_fuzz_3).is_none());
        debug_assert_eq!(
            date.with_month(rug_fuzz_4).unwrap(), NaiveDate::from_ymd(2015, 5, 31)
        );
             }
});    }
    #[test]
    fn with_month_handles_leap_year() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, u32, u32, u32, u32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        debug_assert_eq!(
            date.with_month(rug_fuzz_3).unwrap(), NaiveDate::from_ymd(2016, 2, 29)
        );
        debug_assert!(
            date.with_month(rug_fuzz_4).unwrap().year() % rug_fuzz_5 == rug_fuzz_6
        );
             }
});    }
    #[test]
    fn with_month_handles_non_leap_year() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        debug_assert!(date.with_month(rug_fuzz_3).is_none());
        debug_assert!(date.with_month(rug_fuzz_4).is_none());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_106 {
    use super::*;
    use crate::*;
    use crate::Datelike;
    #[test]
    fn test_with_month0() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(i32, u32, u32, i32, u32, u32, i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let naive_date1 = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .unwrap();
        let naive_date2 = NaiveDate::from_ymd_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .unwrap();
        let naive_date3 = NaiveDate::from_ymd_opt(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8)
            .unwrap();
        debug_assert_eq!(naive_date1.with_month0(rug_fuzz_9), Some(naive_date2));
        debug_assert_eq!(naive_date1.with_month0(rug_fuzz_10), None);
        debug_assert_eq!(naive_date3.with_month0(rug_fuzz_11), None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_107 {
    use super::*;
    use crate::*;
    use crate::NaiveDate;
    #[test]
    fn test_with_ordinal() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17, mut rug_fuzz_18, mut rug_fuzz_19, mut rug_fuzz_20, mut rug_fuzz_21, mut rug_fuzz_22, mut rug_fuzz_23)) = <(i32, u32, u32, u32, i32, u32, u32, u32, i32, u32, u32, u32, i32, u32, u32, u32, i32, u32, u32, u32, i32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap()
            .with_ordinal(rug_fuzz_3), Some(NaiveDate::from_ymd_opt(2015, 3, 1).unwrap())
        );
        debug_assert_eq!(
            NaiveDate::from_ymd_opt(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6).unwrap()
            .with_ordinal(rug_fuzz_7), Some(NaiveDate::from_ymd_opt(2015, 12, 31)
            .unwrap())
        );
        debug_assert_eq!(
            NaiveDate::from_ymd_opt(rug_fuzz_8, rug_fuzz_9, rug_fuzz_10).unwrap()
            .with_ordinal(rug_fuzz_11), None
        );
        debug_assert_eq!(
            NaiveDate::from_ymd_opt(rug_fuzz_12, rug_fuzz_13, rug_fuzz_14).unwrap()
            .with_ordinal(rug_fuzz_15), Some(NaiveDate::from_ymd_opt(2016, 2, 29)
            .unwrap())
        );
        debug_assert_eq!(
            NaiveDate::from_ymd_opt(rug_fuzz_16, rug_fuzz_17, rug_fuzz_18).unwrap()
            .with_ordinal(rug_fuzz_19), Some(NaiveDate::from_ymd_opt(2016, 12, 31)
            .unwrap())
        );
        debug_assert_eq!(
            NaiveDate::from_ymd_opt(rug_fuzz_20, rug_fuzz_21, rug_fuzz_22).unwrap()
            .with_ordinal(rug_fuzz_23), None
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_108 {
    use super::*;
    use crate::*;
    use crate::naive::date::NaiveDate;
    use crate::traits::Datelike;
    #[test]
    fn test_with_ordinal0() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12)) = <(i32, u32, u32, u32, u32, u32, i32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        debug_assert_eq!(
            date.with_ordinal0(rug_fuzz_3), Some(NaiveDate::from_ymd_opt(2022, 1, 1)
            .unwrap())
        );
        debug_assert_eq!(
            date.with_ordinal0(rug_fuzz_4), Some(NaiveDate::from_ymd_opt(2022, 2, 1)
            .unwrap())
        );
        debug_assert_eq!(
            date.with_ordinal0(rug_fuzz_5), Some(NaiveDate::from_ymd_opt(2022, 3, 1)
            .unwrap())
        );
        let leap_date = NaiveDate::from_ymd_opt(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8)
            .unwrap();
        debug_assert_eq!(
            leap_date.with_ordinal0(rug_fuzz_9), Some(NaiveDate::from_ymd_opt(2020, 12,
            31).unwrap())
        );
        debug_assert_eq!(
            leap_date.with_ordinal0(rug_fuzz_10), Some(NaiveDate::from_ymd_opt(2020, 2,
            29).unwrap())
        );
        debug_assert_eq!(date.with_ordinal0(rug_fuzz_11), None);
        debug_assert_eq!(date.with_ordinal0(rug_fuzz_12), None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_110 {
    use super::*;
    use crate::*;
    use crate::Datelike;
    #[test]
    fn test_year() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17, mut rug_fuzz_18, mut rug_fuzz_19, mut rug_fuzz_20, mut rug_fuzz_21, mut rug_fuzz_22, mut rug_fuzz_23)) = <(i32, u32, u32, i32, u32, u32, i32, u32, u32, i32, u32, u32, i32, u32, u32, i32, u32, u32, i32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(date.year(), 2022);
        let date = NaiveDate::from_ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        debug_assert_eq!(date.year(), 2022);
        let date = NaiveDate::from_ymd(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8);
        debug_assert_eq!(date.year(), 2022);
        let date = NaiveDate::from_ymd(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11);
        debug_assert_eq!(date.year(), 2020);
        let date = NaiveDate::from_ymd(rug_fuzz_12, rug_fuzz_13, rug_fuzz_14);
        debug_assert_eq!(date.year(), 2021);
        let date = NaiveDate::from_ymd(rug_fuzz_15, rug_fuzz_16, rug_fuzz_17);
        debug_assert_eq!(date.year(), 0);
        let date = NaiveDate::from_ymd(-rug_fuzz_18, rug_fuzz_19, rug_fuzz_20);
        debug_assert_eq!(date.year(), - 1);
        let date = NaiveDate::from_ymd(-rug_fuzz_21, rug_fuzz_22, rug_fuzz_23);
        debug_assert_eq!(date.year(), - 9999);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_111 {
    use crate::NaiveDate;
    use std::iter::DoubleEndedIterator;
    use crate::naive::date::NaiveDateDaysIterator;
    use crate::Datelike;
    use crate::Weekday;
    #[test]
    fn test_next_back() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut iter = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .iter_days();
        debug_assert_eq!(iter.next_back(), Some(NaiveDate::from_ymd(2023, 2, 28)));
        debug_assert_eq!(iter.next_back(), Some(NaiveDate::from_ymd(2023, 2, 27)));
        debug_assert_eq!(iter.next_back(), Some(NaiveDate::from_ymd(2023, 2, 26)));
             }
});    }
    #[test]
    fn test_next_back_at_min_date() {
        let _rug_st_tests_llm_16_111_rrrruuuugggg_test_next_back_at_min_date = 0;
        let mut iter = NaiveDateDaysIterator {
            value: NaiveDate::MIN,
        };
        debug_assert_eq!(iter.next_back(), None);
        let _rug_ed_tests_llm_16_111_rrrruuuugggg_test_next_back_at_min_date = 0;
    }
    #[test]
    fn test_next_back_around_end_of_year() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut iter = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .iter_days();
        debug_assert_eq!(iter.next_back(), Some(NaiveDate::from_ymd(2022, 12, 31)));
        debug_assert_eq!(iter.next_back(), Some(NaiveDate::from_ymd(2022, 12, 30)));
        debug_assert_eq!(iter.next_back(), Some(NaiveDate::from_ymd(2022, 12, 29)));
             }
});    }
    #[test]
    fn test_next_back_around_leap_year() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut iter = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .iter_days();
        debug_assert_eq!(iter.next_back(), Some(NaiveDate::from_ymd(2024, 2, 29)));
        debug_assert_eq!(iter.next_back(), Some(NaiveDate::from_ymd(2024, 2, 28)));
        debug_assert_eq!(iter.next_back(), Some(NaiveDate::from_ymd(2024, 2, 27)));
             }
});    }
    #[test]
    fn test_next_back_on_weekday() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut iter = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .iter_days();
        debug_assert_eq!(iter.next_back().map(| d | d.weekday()), Some(Weekday::Tue));
        debug_assert_eq!(iter.next_back().map(| d | d.weekday()), Some(Weekday::Mon));
        debug_assert_eq!(iter.next_back().map(| d | d.weekday()), Some(Weekday::Sun));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_112 {
    use super::*;
    use crate::*;
    use crate::NaiveDate;
    #[test]
    fn test_next_for_last_day() {
        let _rug_st_tests_llm_16_112_rrrruuuugggg_test_next_for_last_day = 0;
        let mut iterator = NaiveDate::MAX.iter_days();
        debug_assert_eq!(iterator.next(), None);
        let _rug_ed_tests_llm_16_112_rrrruuuugggg_test_next_for_last_day = 0;
    }
    #[test]
    fn test_next_for_typical_day() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut iterator = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .iter_days();
        debug_assert_eq!(iterator.next(), Some(NaiveDate::from_ymd(2023, 3, 14)));
        debug_assert_eq!(iterator.next(), Some(NaiveDate::from_ymd(2023, 3, 15)));
             }
});    }
    #[test]
    fn test_next_for_new_year() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut iterator = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .iter_days();
        debug_assert_eq!(iterator.next(), Some(NaiveDate::from_ymd(2022, 12, 31)));
        debug_assert_eq!(iterator.next(), Some(NaiveDate::from_ymd(2023, 1, 1)));
             }
});    }
    #[test]
    fn test_next_for_leap_year() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut iterator = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .iter_days();
        debug_assert_eq!(iterator.next(), Some(NaiveDate::from_ymd(2020, 2, 28)));
        debug_assert_eq!(iterator.next(), Some(NaiveDate::from_ymd(2020, 2, 29)));
        debug_assert_eq!(iterator.next(), Some(NaiveDate::from_ymd(2020, 3, 1)));
             }
});    }
    #[test]
    fn test_next_back_for_first_day() {
        let _rug_st_tests_llm_16_112_rrrruuuugggg_test_next_back_for_first_day = 0;
        let mut iterator = NaiveDate::MIN.iter_days();
        debug_assert_eq!(iterator.next_back(), None);
        let _rug_ed_tests_llm_16_112_rrrruuuugggg_test_next_back_for_first_day = 0;
    }
    #[test]
    fn test_next_back_for_typical_day() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut iterator = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .iter_days();
        debug_assert_eq!(iterator.next_back(), Some(NaiveDate::from_ymd(2023, 3, 14)));
        debug_assert_eq!(iterator.next_back(), Some(NaiveDate::from_ymd(2023, 3, 13)));
             }
});    }
    #[test]
    fn test_next_back_for_new_year() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut iterator = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .iter_days();
        debug_assert_eq!(iterator.next_back(), Some(NaiveDate::from_ymd(2023, 1, 1)));
        debug_assert_eq!(iterator.next_back(), Some(NaiveDate::from_ymd(2022, 12, 31)));
             }
});    }
    #[test]
    fn test_next_back_for_leap_year() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut iterator = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .iter_days();
        debug_assert_eq!(iterator.next_back(), Some(NaiveDate::from_ymd(2020, 3, 1)));
        debug_assert_eq!(iterator.next_back(), Some(NaiveDate::from_ymd(2020, 2, 29)));
        debug_assert_eq!(iterator.next_back(), Some(NaiveDate::from_ymd(2020, 2, 28)));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_114 {
    use super::*;
    use crate::*;
    use crate::{NaiveDate, TimeDelta};
    use std::iter::DoubleEndedIterator;
    #[test]
    fn test_naive_date_weeks_iterator_next_back() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(i32, u32, u32, i32, u32, u32, i32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut it = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .unwrap()
            .iter_weeks();
        debug_assert_eq!(
            it.next_back(), Some(NaiveDate::from_ymd_opt(262144, 12, 18).unwrap())
        );
        debug_assert_eq!(
            it.next_back(), Some(NaiveDate::from_ymd_opt(262144, 12, 11).unwrap())
        );
        debug_assert_eq!(
            it.next_back(), Some(NaiveDate::from_ymd_opt(262144, 12, 4).unwrap())
        );
        let mut it = NaiveDate::from_ymd_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .unwrap()
            .iter_weeks();
        debug_assert_eq!(it.next_back(), None);
        let mut it = NaiveDate::from_ymd_opt(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8)
            .unwrap()
            .iter_weeks();
        debug_assert_eq!(
            it.next_back(), Some(NaiveDate::from_ymd_opt(262145, 1, 1).unwrap())
        );
        let mut it = NaiveDate::from_ymd_opt(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11)
            .unwrap()
            .iter_weeks();
        it.next_back();
        it.next_back();
        debug_assert_eq!(it.next_back(), None);
             }
});    }
    #[test]
    fn test_naive_date_weeks_iterator_next() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut it = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .unwrap()
            .iter_weeks();
        debug_assert_eq!(
            it.next(), Some(NaiveDate::from_ymd_opt(262143, 12, 31).unwrap())
        );
        debug_assert_eq!(it.next(), None);
        let mut it = NaiveDate::from_ymd_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .unwrap()
            .iter_weeks();
        it.next();
        it.next();
        debug_assert_eq!(it.next(), None);
             }
});    }
    #[test]
    fn test_naive_date_weeks_iterator_size_hint() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let begin = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        let end = NaiveDate::from_ymd_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5).unwrap();
        let it = begin.iter_weeks();
        let range = end.signed_duration_since(begin).num_weeks();
        debug_assert_eq!(it.size_hint(), (range as usize, Some(range as usize)));
             }
});    }
    #[test]
    fn test_naive_date_weeks_iterator_double_ended() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut it = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .unwrap()
            .iter_weeks();
        debug_assert_eq!(
            it.next_back(), Some(NaiveDate::from_ymd_opt(262143, 12, 24).unwrap())
        );
        debug_assert_eq!(
            it.next(), Some(NaiveDate::from_ymd_opt(262143, 12, 10).unwrap())
        );
        debug_assert_eq!(
            it.next_back(), Some(NaiveDate::from_ymd_opt(262143, 12, 17).unwrap())
        );
        debug_assert_eq!(
            it.next(), Some(NaiveDate::from_ymd_opt(262143, 12, 3).unwrap())
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_115 {
    use super::*;
    use crate::*;
    use crate::{NaiveDate, TimeDelta};
    #[test]
    fn test_naive_date_weeks_iterator_next() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let start_date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let mut iter = NaiveDateWeeksIterator {
            value: start_date,
        };
        debug_assert_eq!(iter.next(), Some(NaiveDate::from_ymd(2023, 1, 1)));
        debug_assert_eq!(iter.next(), Some(NaiveDate::from_ymd(2023, 1, 8)));
        debug_assert_eq!(iter.next(), Some(NaiveDate::from_ymd(2023, 1, 15)));
             }
});    }
    #[test]
    fn test_naive_date_weeks_iterator_next_at_max() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut iter = NaiveDateWeeksIterator {
            value: NaiveDate::MAX - TimeDelta::weeks(rug_fuzz_0),
        };
        debug_assert!(iter.next().is_some());
        debug_assert!(iter.next().is_none());
             }
});    }
    #[test]
    fn test_naive_date_weeks_iterator_size_hint() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let start_date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let iter = NaiveDateWeeksIterator {
            value: start_date,
        };
        let weeks_until_max = NaiveDate::MAX
            .signed_duration_since(start_date)
            .num_weeks();
        let (lower_bound, upper_bound) = iter.size_hint();
        debug_assert_eq!(lower_bound, upper_bound.unwrap());
        debug_assert_eq!(lower_bound as i64, weeks_until_max);
             }
});    }
    #[test]
    fn test_naive_date_weeks_iterator_next_back() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let start_date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let mut iter = NaiveDateWeeksIterator {
            value: start_date,
        };
        debug_assert_eq!(iter.next_back(), Some(NaiveDate::from_ymd(2023, 1, 15)));
        debug_assert_eq!(iter.next_back(), Some(NaiveDate::from_ymd(2023, 1, 8)));
        debug_assert_eq!(iter.next_back(), Some(NaiveDate::from_ymd(2023, 1, 1)));
             }
});    }
    #[test]
    fn test_naive_date_weeks_iterator_next_back_at_min() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut iter = NaiveDateWeeksIterator {
            value: NaiveDate::MIN + TimeDelta::weeks(rug_fuzz_0),
        };
        debug_assert!(iter.next_back().is_some());
        debug_assert!(iter.next_back().is_none());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_116 {
    use crate::NaiveDate;
    use crate::TimeDelta;
    use std::iter::Iterator;
    #[test]
    fn test_size_hint() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let start_date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let end_date = NaiveDate::from_ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let week_iter = start_date.iter_weeks();
        let duration = end_date.signed_duration_since(start_date);
        let num_weeks = duration.num_weeks() as usize;
        let (lower, upper) = week_iter.size_hint();
        debug_assert_eq!(lower, num_weeks);
        debug_assert_eq!(upper, Some(num_weeks));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_354_llm_16_354 {
    use crate::Days;
    #[test]
    fn test_days_new() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let num_days = rug_fuzz_0;
        let days = Days::new(num_days);
        debug_assert_eq!(days.0, num_days);
             }
});    }
    #[test]
    fn test_days_clone() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let days = Days::new(rug_fuzz_0);
        let days_clone = days.clone();
        debug_assert_eq!(days, days_clone);
             }
});    }
    #[test]
    fn test_days_eq() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let days_a = Days::new(rug_fuzz_0);
        let days_b = Days::new(rug_fuzz_1);
        debug_assert_eq!(days_a, days_b);
             }
});    }
    #[test]
    fn test_days_partial_eq() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let days_a = Days::new(rug_fuzz_0);
        let days_b = Days::new(rug_fuzz_1);
        debug_assert_eq!(days_a, days_b);
             }
});    }
    #[test]
    fn test_days_partial_ord() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let days_a = Days::new(rug_fuzz_0);
        let days_b = Days::new(rug_fuzz_1);
        debug_assert!(days_a < days_b);
             }
});    }
    #[test]
    fn test_days_debug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let days = Days::new(rug_fuzz_0);
        debug_assert_eq!(format!("{:?}", days), "Days(35)");
             }
});    }
    #[test]
    fn test_days_hash() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let days = Days::new(rug_fuzz_0);
        let mut hasher = DefaultHasher::new();
        days.hash(&mut hasher);
        let hashed_days = hasher.finish();
        let mut hasher2 = DefaultHasher::new();
        Days::new(rug_fuzz_1).hash(&mut hasher2);
        let hashed_days2 = hasher2.finish();
        debug_assert_eq!(hashed_days, hashed_days2);
             }
});    }
    #[test]
    fn test_days_copy() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let days = Days::new(rug_fuzz_0);
        let days_copied = days;
        debug_assert_eq!(days, days_copied);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_355_llm_16_355 {
    use crate::{NaiveDate, NaiveDateTime, NaiveTime};
    #[test]
    fn test_and_hms() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let datetime = date.and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        debug_assert_eq!(
            datetime, NaiveDateTime::new(date, NaiveTime::from_hms(12, 30, 45))
        );
             }
});    }
    #[test]
    #[should_panic]
    fn test_and_hms_panic_hour() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let _ = date.and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
             }
});    }
    #[test]
    #[should_panic]
    fn test_and_hms_panic_minute() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let _ = date.and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
             }
});    }
    #[test]
    #[should_panic]
    fn test_and_hms_panic_second() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let _ = date.and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_356 {
    use super::*;
    use crate::*;
    use crate::NaiveDate;
    #[test]
    fn test_and_hms_micro_valid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        let datetime = date
            .and_hms_micro(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        debug_assert_eq!(datetime.year(), 2020);
        debug_assert_eq!(datetime.month(), 5);
        debug_assert_eq!(datetime.day(), 15);
        debug_assert_eq!(datetime.hour(), 10);
        debug_assert_eq!(datetime.minute(), 20);
        debug_assert_eq!(datetime.second(), 30);
        debug_assert_eq!(datetime.nanosecond(), 123_456_000);
             }
});    }
    #[test]
    #[should_panic]
    fn test_and_hms_micro_invalid_hour() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        date.and_hms_micro(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
             }
});    }
    #[test]
    #[should_panic]
    fn test_and_hms_micro_invalid_minute() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        date.and_hms_micro(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
             }
});    }
    #[test]
    #[should_panic]
    fn test_and_hms_micro_invalid_second() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        date.and_hms_micro(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
             }
});    }
    #[test]
    #[should_panic]
    fn test_and_hms_micro_invalid_microsecond() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        date.and_hms_micro(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
             }
});    }
    #[test]
    fn test_and_hms_micro_leap_second() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        let datetime = date
            .and_hms_micro(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        debug_assert_eq!(datetime.second(), 30);
        debug_assert_eq!(datetime.nanosecond(), 1_123_456_000);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_357 {
    use super::*;
    use crate::*;
    use crate::NaiveDate;
    #[test]
    fn test_and_hms_micro_opt() {
        let _rug_st_tests_llm_16_357_rrrruuuugggg_test_and_hms_micro_opt = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 4;
        let rug_fuzz_2 = 15;
        let rug_fuzz_3 = 12;
        let rug_fuzz_4 = 30;
        let rug_fuzz_5 = 45;
        let rug_fuzz_6 = 123_456;
        let rug_fuzz_7 = 23;
        let rug_fuzz_8 = 59;
        let rug_fuzz_9 = 59;
        let rug_fuzz_10 = 1_000_000;
        let rug_fuzz_11 = 12;
        let rug_fuzz_12 = 30;
        let rug_fuzz_13 = 60;
        let rug_fuzz_14 = 123_456;
        let rug_fuzz_15 = 12;
        let rug_fuzz_16 = 60;
        let rug_fuzz_17 = 45;
        let rug_fuzz_18 = 123_456;
        let rug_fuzz_19 = 24;
        let rug_fuzz_20 = 30;
        let rug_fuzz_21 = 45;
        let rug_fuzz_22 = 123_456;
        let rug_fuzz_23 = 12;
        let rug_fuzz_24 = 30;
        let rug_fuzz_25 = 45;
        let rug_fuzz_26 = 1_000_001;
        let rug_fuzz_27 = 99;
        let rug_fuzz_28 = 99;
        let rug_fuzz_29 = 99;
        let rug_fuzz_30 = 99_999_999;
        let date = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        debug_assert_eq!(
            date.and_hms_micro_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6),
            Some(NaiveDateTime::new(date, NaiveTime::from_hms_micro(12, 30, 45,
            123_456)))
        );
        debug_assert_eq!(
            date.and_hms_micro_opt(rug_fuzz_7, rug_fuzz_8, rug_fuzz_9, rug_fuzz_10),
            Some(NaiveDateTime::new(date, NaiveTime::from_hms_micro(23, 59, 59,
            1_000_000)))
        );
        debug_assert_eq!(
            date.and_hms_micro_opt(rug_fuzz_11, rug_fuzz_12, rug_fuzz_13, rug_fuzz_14),
            None
        );
        debug_assert_eq!(
            date.and_hms_micro_opt(rug_fuzz_15, rug_fuzz_16, rug_fuzz_17, rug_fuzz_18),
            None
        );
        debug_assert_eq!(
            date.and_hms_micro_opt(rug_fuzz_19, rug_fuzz_20, rug_fuzz_21, rug_fuzz_22),
            None
        );
        debug_assert_eq!(
            date.and_hms_micro_opt(rug_fuzz_23, rug_fuzz_24, rug_fuzz_25, rug_fuzz_26),
            None
        );
        debug_assert_eq!(
            date.and_hms_micro_opt(rug_fuzz_27, rug_fuzz_28, rug_fuzz_29, rug_fuzz_30),
            None
        );
        let _rug_ed_tests_llm_16_357_rrrruuuugggg_test_and_hms_micro_opt = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_359 {
    use super::*;
    use crate::*;
    use crate::naive::date::NaiveDate;
    #[test]
    fn test_and_hms_milli_opt() {
        let _rug_st_tests_llm_16_359_rrrruuuugggg_test_and_hms_milli_opt = 0;
        let rug_fuzz_0 = 2015;
        let rug_fuzz_1 = 6;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 12;
        let rug_fuzz_4 = 34;
        let rug_fuzz_5 = 56;
        let rug_fuzz_6 = 789;
        let rug_fuzz_7 = 12;
        let rug_fuzz_8 = 34;
        let rug_fuzz_9 = 59;
        let rug_fuzz_10 = 1789;
        let rug_fuzz_11 = 12;
        let rug_fuzz_12 = 34;
        let rug_fuzz_13 = 59;
        let rug_fuzz_14 = 2789;
        let rug_fuzz_15 = 12;
        let rug_fuzz_16 = 34;
        let rug_fuzz_17 = 60;
        let rug_fuzz_18 = 789;
        let rug_fuzz_19 = 12;
        let rug_fuzz_20 = 60;
        let rug_fuzz_21 = 56;
        let rug_fuzz_22 = 789;
        let rug_fuzz_23 = 24;
        let rug_fuzz_24 = 34;
        let rug_fuzz_25 = 56;
        let rug_fuzz_26 = 789;
        let d = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        debug_assert_eq!(
            d.and_hms_milli_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6),
            Some(NaiveDateTime::new(d, NaiveTime::from_hms_milli(12, 34, 56, 789)))
        );
        debug_assert_eq!(
            d.and_hms_milli_opt(rug_fuzz_7, rug_fuzz_8, rug_fuzz_9, rug_fuzz_10),
            Some(NaiveDateTime::new(d, NaiveTime::from_hms_milli(12, 34, 59, 1789)))
        );
        debug_assert!(
            d.and_hms_milli_opt(rug_fuzz_11, rug_fuzz_12, rug_fuzz_13, rug_fuzz_14)
            .is_none()
        );
        debug_assert!(
            d.and_hms_milli_opt(rug_fuzz_15, rug_fuzz_16, rug_fuzz_17, rug_fuzz_18)
            .is_none()
        );
        debug_assert!(
            d.and_hms_milli_opt(rug_fuzz_19, rug_fuzz_20, rug_fuzz_21, rug_fuzz_22)
            .is_none()
        );
        debug_assert!(
            d.and_hms_milli_opt(rug_fuzz_23, rug_fuzz_24, rug_fuzz_25, rug_fuzz_26)
            .is_none()
        );
        let _rug_ed_tests_llm_16_359_rrrruuuugggg_test_and_hms_milli_opt = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_360 {
    use super::*;
    use crate::*;
    use crate::NaiveDate;
    #[test]
    fn test_and_hms_nano_valid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let naive_date_time = date
            .and_hms_nano(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        debug_assert_eq!(
            naive_date_time, NaiveDate::from_ymd(2023, 4, 1).and_hms_nano(12, 30, 45,
            1_000_000_000)
        );
             }
});    }
    #[test]
    #[should_panic]
    fn test_and_hms_nano_panic_hour() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let _ = date.and_hms_nano(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
             }
});    }
    #[test]
    #[should_panic]
    fn test_and_hms_nano_panic_minute() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let _ = date.and_hms_nano(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
             }
});    }
    #[test]
    #[should_panic]
    fn test_and_hms_nano_panic_second() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let _ = date.and_hms_nano(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
             }
});    }
    #[test]
    #[should_panic]
    fn test_and_hms_nano_panic_nanosecond() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let _ = date.and_hms_nano(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_361 {
    use crate::NaiveDate;
    use crate::NaiveTime;
    use crate::Datelike;
    use crate::naive::MAX_DATE;
    use crate::naive::MIN_DATE;
    #[test]
    fn test_and_hms_nano_opt() {
        let _rug_st_tests_llm_16_361_rrrruuuugggg_test_and_hms_nano_opt = 0;
        let rug_fuzz_0 = 2015;
        let rug_fuzz_1 = 6;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 12;
        let rug_fuzz_4 = 34;
        let rug_fuzz_5 = 56;
        let rug_fuzz_6 = 789_012_345;
        let rug_fuzz_7 = 12;
        let rug_fuzz_8 = 34;
        let rug_fuzz_9 = 59;
        let rug_fuzz_10 = 1_789_012_345;
        let rug_fuzz_11 = 12;
        let rug_fuzz_12 = 34;
        let rug_fuzz_13 = 59;
        let rug_fuzz_14 = 2_789_012_345;
        let rug_fuzz_15 = 12;
        let rug_fuzz_16 = 34;
        let rug_fuzz_17 = 60;
        let rug_fuzz_18 = 789_012_345;
        let rug_fuzz_19 = 12;
        let rug_fuzz_20 = 60;
        let rug_fuzz_21 = 56;
        let rug_fuzz_22 = 789_012_345;
        let rug_fuzz_23 = 24;
        let rug_fuzz_24 = 34;
        let rug_fuzz_25 = 56;
        let rug_fuzz_26 = 789_012_345;
        let d = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        debug_assert!(
            d.and_hms_nano_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6).is_some()
        );
        debug_assert!(
            d.and_hms_nano_opt(rug_fuzz_7, rug_fuzz_8, rug_fuzz_9, rug_fuzz_10).is_some()
        );
        debug_assert!(
            d.and_hms_nano_opt(rug_fuzz_11, rug_fuzz_12, rug_fuzz_13, rug_fuzz_14)
            .is_none()
        );
        debug_assert!(
            d.and_hms_nano_opt(rug_fuzz_15, rug_fuzz_16, rug_fuzz_17, rug_fuzz_18)
            .is_none()
        );
        debug_assert!(
            d.and_hms_nano_opt(rug_fuzz_19, rug_fuzz_20, rug_fuzz_21, rug_fuzz_22)
            .is_none()
        );
        debug_assert!(
            d.and_hms_nano_opt(rug_fuzz_23, rug_fuzz_24, rug_fuzz_25, rug_fuzz_26)
            .is_none()
        );
        let _rug_ed_tests_llm_16_361_rrrruuuugggg_test_and_hms_nano_opt = 0;
    }
    #[test]
    fn test_and_hms_nano_opt_min_and_max() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let d_min = NaiveDate::from_ymd_opt(MIN_DATE.year(), rug_fuzz_0, rug_fuzz_1)
            .unwrap();
        let d_max = NaiveDate::from_ymd_opt(MAX_DATE.year(), rug_fuzz_2, rug_fuzz_3)
            .unwrap();
        debug_assert!(
            d_min.and_hms_nano_opt(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6, rug_fuzz_7)
            .is_some()
        );
        debug_assert!(
            d_max.and_hms_nano_opt(rug_fuzz_8, rug_fuzz_9, rug_fuzz_10, rug_fuzz_11)
            .is_some()
        );
             }
});    }
    #[test]
    fn test_and_hms_nano_opt_with_time() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10)) = <(i32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let d = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let t = NaiveTime::from_hms_nano(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        debug_assert_eq!(
            d.and_hms_nano_opt(rug_fuzz_7, rug_fuzz_8, rug_fuzz_9, rug_fuzz_10), Some(d
            .and_time(t))
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_362 {
    use crate::NaiveDate;
    use crate::NaiveTime;
    use crate::NaiveDateTime;
    #[test]
    fn test_and_hms_opt() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17)) = <(i32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        debug_assert_eq!(
            date.and_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5),
            Some(NaiveDateTime::new(date, NaiveTime::from_hms(0, 0, 0)))
        );
        debug_assert_eq!(
            date.and_hms_opt(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8),
            Some(NaiveDateTime::new(date, NaiveTime::from_hms(23, 59, 59)))
        );
        debug_assert!(date.and_hms_opt(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11).is_none());
        debug_assert!(date.and_hms_opt(rug_fuzz_12, rug_fuzz_13, rug_fuzz_14).is_none());
        debug_assert!(date.and_hms_opt(rug_fuzz_15, rug_fuzz_16, rug_fuzz_17).is_none());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_363 {
    use super::*;
    use crate::*;
    use crate::naive::NaiveTime;
    #[test]
    fn test_and_time() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13)) = <(i32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        let time = NaiveTime::from_hms_milli_opt(
                rug_fuzz_3,
                rug_fuzz_4,
                rug_fuzz_5,
                rug_fuzz_6,
            )
            .unwrap();
        let datetime = date.and_time(time);
        debug_assert_eq!(datetime.date(), date);
        debug_assert_eq!(datetime.time(), time);
        let leap_time = NaiveTime::from_hms_milli_opt(
                rug_fuzz_7,
                rug_fuzz_8,
                rug_fuzz_9,
                rug_fuzz_10,
            )
            .unwrap();
        let leap_datetime = date.and_time(leap_time);
        debug_assert_eq!(leap_datetime.date(), date);
        debug_assert_eq!(leap_datetime.time().hour(), 23);
        debug_assert_eq!(leap_datetime.time().minute(), 59);
        debug_assert_eq!(leap_datetime.time().second(), 59);
        debug_assert_eq!(leap_datetime.time().nanosecond(), 1_000_000_000);
        let invalid_time = NaiveTime::from_hms_opt(
            rug_fuzz_11,
            rug_fuzz_12,
            rug_fuzz_13,
        );
        debug_assert!(invalid_time.is_none());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_364 {
    use super::*;
    use crate::*;
    use crate::naive::date::Days;
    #[test]
    fn test_checked_add_days() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(i32, u32, u32, u64, i32, u32, u32, u64, i32, u32, u32, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let added_days = Days::new(rug_fuzz_3);
        let result = date.checked_add_days(added_days);
        debug_assert_eq!(result, Some(NaiveDate::from_ymd(2023, 5, 1)));
        let date = NaiveDate::from_ymd(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        let added_days = Days::new(rug_fuzz_7);
        let result = date.checked_add_days(added_days);
        debug_assert_eq!(result, Some(NaiveDate::from_ymd(2024, 4, 29)));
        let date = NaiveDate::from_ymd(rug_fuzz_8, rug_fuzz_9, rug_fuzz_10);
        let added_days = Days::new(rug_fuzz_11);
        let result = date.checked_add_days(added_days);
        debug_assert_eq!(result, None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_365 {
    use super::*;
    use crate::*;
    use crate::Month;
    #[test]
    fn test_checked_add_months() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13)) = <(i32, u32, u32, u32, u32, u32, i32, u32, u32, u32, i32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        debug_assert_eq!(
            date.checked_add_months(Months::new(rug_fuzz_3)),
            Some(NaiveDate::from_ymd_opt(2022, 8, 20).unwrap())
        );
        debug_assert_eq!(
            date.checked_add_months(Months::new(rug_fuzz_4)),
            Some(NaiveDate::from_ymd_opt(2023, 2, 20).unwrap())
        );
        debug_assert_eq!(
            date.checked_add_months(Months::new(rug_fuzz_5)),
            Some(NaiveDate::from_ymd_opt(2024, 2, 20).unwrap())
        );
        debug_assert_eq!(date.checked_add_months(Months::new(u32::MAX)), None);
        debug_assert_eq!(
            NaiveDate::from_ymd_opt(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8).unwrap()
            .checked_add_months(Months::new(rug_fuzz_9)),
            Some(NaiveDate::from_ymd_opt(2023, 2, 28).unwrap())
        );
        debug_assert_eq!(
            NaiveDate::from_ymd_opt(rug_fuzz_10, rug_fuzz_11, rug_fuzz_12).unwrap()
            .checked_add_months(Months::new(rug_fuzz_13)),
            Some(NaiveDate::from_ymd_opt(2022, 2, 28).unwrap())
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_366 {
    use super::*;
    use crate::*;
    use crate::naive::date::NaiveDate;
    use crate::time_delta::TimeDelta;
    #[test]
    fn test_checked_add_signed() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, u32, u32, i64, i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let d = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        debug_assert_eq!(
            d.checked_add_signed(TimeDelta::days(rug_fuzz_3)),
            Some(NaiveDate::from_ymd_opt(2020, 3, 1).unwrap())
        );
        debug_assert_eq!(
            d.checked_add_signed(TimeDelta::days(- rug_fuzz_4)),
            Some(NaiveDate::from_ymd_opt(2020, 2, 28).unwrap())
        );
        debug_assert_eq!(
            d.checked_add_signed(TimeDelta::days(rug_fuzz_5)),
            Some(NaiveDate::from_ymd_opt(2021, 2, 28).unwrap())
        );
        debug_assert_eq!(d.checked_add_signed(TimeDelta::days(- rug_fuzz_6)), None);
             }
});    }
    #[test]
    fn test_checked_add_signed_overflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i32, u32, u32, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let d = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        debug_assert_eq!(d.checked_add_signed(TimeDelta::days(rug_fuzz_3)), None);
        debug_assert_eq!(d.checked_add_signed(TimeDelta::days(- rug_fuzz_4)), None);
             }
});    }
    #[test]
    fn test_checked_add_signed_boundaries() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i64, i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let min = NaiveDate::MIN;
        let max = NaiveDate::MAX;
        debug_assert_eq!(
            min.checked_add_signed(TimeDelta::days(rug_fuzz_0)),
            Some(NaiveDate::from_ymd_opt(- 262_144, 1, 2).unwrap())
        );
        debug_assert_eq!(min.checked_add_signed(TimeDelta::days(- rug_fuzz_1)), None);
        debug_assert_eq!(max.checked_add_signed(TimeDelta::days(rug_fuzz_2)), None);
        debug_assert_eq!(
            max.checked_add_signed(TimeDelta::days(- rug_fuzz_3)),
            Some(NaiveDate::from_ymd_opt(262_143, 12, 30).unwrap())
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_367 {
    use super::*;
    use crate::*;
    use crate::naive::date::NaiveDate;
    use crate::naive::date::Days;
    #[test]
    fn test_checked_sub_days() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17, mut rug_fuzz_18, mut rug_fuzz_19, mut rug_fuzz_20, mut rug_fuzz_21, mut rug_fuzz_22)) = <(i32, u32, u32, u64, i32, u32, u32, u64, i32, u32, u32, u64, i32, u32, u32, u64, i32, u32, u32, u64, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap()
            .checked_sub_days(Days::new(rug_fuzz_3)), Some(NaiveDate::from_ymd_opt(2023,
            7, 5).unwrap())
        );
        debug_assert_eq!(
            NaiveDate::from_ymd_opt(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6).unwrap()
            .checked_sub_days(Days::new(rug_fuzz_7)), Some(NaiveDate::from_ymd_opt(2023,
            6, 30).unwrap())
        );
        debug_assert_eq!(
            NaiveDate::from_ymd_opt(rug_fuzz_8, rug_fuzz_9, rug_fuzz_10).unwrap()
            .checked_sub_days(Days::new(rug_fuzz_11)), Some(NaiveDate::from_ymd_opt(2022,
            12, 31).unwrap())
        );
        debug_assert_eq!(
            NaiveDate::from_ymd_opt(rug_fuzz_12, rug_fuzz_13, rug_fuzz_14).unwrap()
            .checked_sub_days(Days::new(rug_fuzz_15)), Some(NaiveDate::from_ymd_opt(2023,
            5, 10).unwrap())
        );
        debug_assert_eq!(
            NaiveDate::from_ymd_opt(- rug_fuzz_16, rug_fuzz_17, rug_fuzz_18).unwrap()
            .checked_sub_days(Days::new(rug_fuzz_19)), None
        );
        debug_assert_eq!(
            NaiveDate::from_ymd_opt(rug_fuzz_20, rug_fuzz_21, rug_fuzz_22).unwrap()
            .checked_sub_days(Days::new(u64::MAX)), None
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_368 {
    use super::*;
    use crate::*;
    use crate::naive::date::NaiveDate;
    use crate::Months;
    #[test]
    fn test_checked_sub_months() {
        let _rug_st_tests_llm_16_368_rrrruuuugggg_test_checked_sub_months = 0;
        let rug_fuzz_0 = 2022;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 20;
        let rug_fuzz_3 = 6;
        let rug_fuzz_4 = 2022;
        let rug_fuzz_5 = 3;
        let rug_fuzz_6 = 31;
        let rug_fuzz_7 = 1;
        let rug_fuzz_8 = 2022;
        let rug_fuzz_9 = 1;
        let rug_fuzz_10 = 1;
        let rug_fuzz_11 = 2;
        let rug_fuzz_12 = 2022;
        let rug_fuzz_13 = 1;
        let rug_fuzz_14 = 31;
        let rug_fuzz_15 = 1;
        let rug_fuzz_16 = 2022;
        let rug_fuzz_17 = 1;
        let rug_fuzz_18 = 31;
        let rug_fuzz_19 = 3;
        let rug_fuzz_20 = 1;
        let rug_fuzz_21 = 1;
        let rug_fuzz_22 = 1;
        let rug_fuzz_23 = 12;
        let rug_fuzz_24 = 1;
        let rug_fuzz_25 = 1;
        let rug_fuzz_26 = 1;
        let rug_fuzz_27 = 13;
        let rug_fuzz_28 = 1;
        let rug_fuzz_29 = 1;
        let rug_fuzz_30 = 1;
        let rug_fuzz_31 = 1;
        debug_assert_eq!(
            NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap()
            .checked_sub_months(Months::new(rug_fuzz_3)),
            Some(NaiveDate::from_ymd_opt(2021, 8, 20).unwrap())
        );
        debug_assert_eq!(
            NaiveDate::from_ymd_opt(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6).unwrap()
            .checked_sub_months(Months::new(rug_fuzz_7)),
            Some(NaiveDate::from_ymd_opt(2022, 2, 29).unwrap())
        );
        debug_assert_eq!(
            NaiveDate::from_ymd_opt(rug_fuzz_8, rug_fuzz_9, rug_fuzz_10).unwrap()
            .checked_sub_months(Months::new(rug_fuzz_11)),
            Some(NaiveDate::from_ymd_opt(2021, 11, 1).unwrap())
        );
        debug_assert_eq!(
            NaiveDate::from_ymd_opt(rug_fuzz_12, rug_fuzz_13, rug_fuzz_14).unwrap()
            .checked_sub_months(Months::new(rug_fuzz_15)),
            Some(NaiveDate::from_ymd_opt(2021, 12, 31).unwrap())
        );
        debug_assert_eq!(
            NaiveDate::from_ymd_opt(rug_fuzz_16, rug_fuzz_17, rug_fuzz_18).unwrap()
            .checked_sub_months(Months::new(rug_fuzz_19)),
            Some(NaiveDate::from_ymd_opt(2021, 10, 31).unwrap())
        );
        debug_assert_eq!(
            NaiveDate::from_ymd_opt(rug_fuzz_20, rug_fuzz_21, rug_fuzz_22).unwrap()
            .checked_sub_months(Months::new(rug_fuzz_23)),
            Some(NaiveDate::from_ymd_opt(0, 1, 1).unwrap())
        );
        debug_assert_eq!(
            NaiveDate::from_ymd_opt(rug_fuzz_24, rug_fuzz_25, rug_fuzz_26).unwrap()
            .checked_sub_months(Months::new(rug_fuzz_27)),
            Some(NaiveDate::from_ymd_opt(0, 12, 1).unwrap())
        );
        debug_assert_eq!(
            NaiveDate::from_ymd_opt(rug_fuzz_28, rug_fuzz_29, rug_fuzz_30).unwrap()
            .checked_sub_months(Months::new(i32::MAX as u32 + rug_fuzz_31)), None
        );
        let _rug_ed_tests_llm_16_368_rrrruuuugggg_test_checked_sub_months = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_369 {
    use crate::NaiveDate;
    use crate::time_delta::TimeDelta;
    #[test]
    fn test_checked_sub_signed() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16)) = <(i32, u32, u32, i64, i32, u32, u32, i64, i32, u32, u32, i64, i32, u32, u32, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        let days_to_subtract = TimeDelta::days(rug_fuzz_3);
        debug_assert_eq!(
            date.checked_sub_signed(days_to_subtract), Some(NaiveDate::from_ymd_opt(2023,
            4, 5).unwrap())
        );
        let date = NaiveDate::from_ymd_opt(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6).unwrap();
        let days_to_subtract = TimeDelta::days(rug_fuzz_7);
        debug_assert_eq!(
            date.checked_sub_signed(days_to_subtract), Some(NaiveDate::from_ymd_opt(2022,
            12, 31).unwrap())
        );
        let date = NaiveDate::from_ymd_opt(rug_fuzz_8, rug_fuzz_9, rug_fuzz_10).unwrap();
        let days_to_subtract = TimeDelta::days(rug_fuzz_11);
        debug_assert_eq!(date.checked_sub_signed(days_to_subtract), None);
        let date = NaiveDate::from_ymd_opt(rug_fuzz_12, rug_fuzz_13, rug_fuzz_14)
            .unwrap();
        let days_to_subtract = TimeDelta::days(-rug_fuzz_15);
        debug_assert_eq!(
            date.checked_sub_signed(days_to_subtract), Some(NaiveDate::from_ymd_opt(2023,
            4, 15).unwrap())
        );
        let date = NaiveDate::MIN;
        let days_to_subtract = TimeDelta::days(rug_fuzz_16);
        debug_assert_eq!(date.checked_sub_signed(days_to_subtract), None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_370 {
    use crate::NaiveDate;
    use crate::TimeDelta;
    #[test]
    fn test_diff_days() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16)) = <(i32, u32, u32, i64, i64, i64, i64, i64, i64, i32, u32, u32, i32, u32, u32, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(
            date.diff_days(rug_fuzz_3), Some(NaiveDate::from_ymd(2023, 4, 10))
        );
        debug_assert_eq!(
            date.diff_days(rug_fuzz_4), Some(NaiveDate::from_ymd(2023, 4, 11))
        );
        debug_assert_eq!(
            date.diff_days(- rug_fuzz_5), Some(NaiveDate::from_ymd(2023, 4, 9))
        );
        debug_assert_eq!(
            date.diff_days(rug_fuzz_6), Some(NaiveDate::from_ymd(2023, 4, 30))
        );
        debug_assert_eq!(
            date.diff_days(- rug_fuzz_7), Some(NaiveDate::from_ymd(2023, 3, 31))
        );
        let boundary_days = i64::MAX / rug_fuzz_8;
        let date_max = NaiveDate::from_ymd(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11);
        debug_assert_eq!(
            date_max.diff_days(- boundary_days), Some(NaiveDate::from_ymd(1, 1, 1))
        );
        let date_min = NaiveDate::from_ymd(-rug_fuzz_12, rug_fuzz_13, rug_fuzz_14);
        debug_assert_eq!(
            date_min.diff_days(boundary_days), Some(NaiveDate::from_ymd(1, 1, 1))
        );
        debug_assert_eq!(date.diff_days(boundary_days), None);
        debug_assert_eq!(date.diff_days(- boundary_days), None);
        debug_assert_eq!(date_max.diff_days(rug_fuzz_15), None);
        debug_assert_eq!(date_min.diff_days(- rug_fuzz_16), None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_372 {
    use super::*;
    use crate::*;
    use crate::NaiveDate;
    #[test]
    fn test_format_with_simple_format() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u32, u32, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let formatted = date.format(rug_fuzz_3).to_string();
        debug_assert_eq!(formatted, "2021-03-14");
             }
});    }
    #[test]
    fn test_format_with_complex_format() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u32, u32, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let formatted = date.format(rug_fuzz_3).to_string();
        debug_assert_eq!(formatted, "Sunday 14 March 2021");
             }
});    }
    #[test]
    fn test_format_with_padding() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u32, u32, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let formatted = date.format(rug_fuzz_3).to_string();
        debug_assert_eq!(formatted, "2021-03-04");
             }
});    }
    #[test]
    fn test_format_with_no_padding() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u32, u32, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let formatted = date.format(rug_fuzz_3).to_string();
        debug_assert_eq!(formatted, "2021-3-4");
             }
});    }
    #[test]
    fn test_format_with_locale_specific_format() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u32, u32, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let formatted = date.format(rug_fuzz_3).to_string();
        debug_assert_eq!(formatted, "2021-03-14");
             }
});    }
    #[test]
    fn test_format_with_nonexistent_date() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u32, u32, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let formatted = date.format(rug_fuzz_3).to_string();
        debug_assert_eq!(formatted, "2021-02-30");
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_373 {
    use super::*;
    use crate::*;
    use crate::format::strftime::StrftimeItems;
    use crate::NaiveDate;
    #[test]
    fn test_format_with_items() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let fmt = StrftimeItems::new(rug_fuzz_0);
        let date = NaiveDate::from_ymd_opt(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3).unwrap();
        let formatted = date.format_with_items(fmt.clone()).to_string();
        debug_assert_eq!(formatted, "2015-09-05");
             }
});    }
    #[test]
    fn test_format_with_items_using_date_format_directly() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let fmt = StrftimeItems::new(rug_fuzz_0);
        let date = NaiveDate::from_ymd_opt(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3).unwrap();
        let formatted = date.format_with_items(fmt.clone()).to_string();
        debug_assert_eq!(formatted, "Friday, 15 April 2022");
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_374 {
    use crate::NaiveDate;
    use crate::Weekday;
    use std::str::FromStr;
    #[test]
    fn test_from_isoywd_with_valid_dates() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(i32, u32, i32, u32, i32, u32, i32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let first_week_monday = NaiveDate::from_isoywd(
            rug_fuzz_0,
            rug_fuzz_1,
            Weekday::Mon,
        );
        debug_assert_eq!(first_week_monday, NaiveDate::from_ymd(2023, 1, 2));
        let arbitrary_date = NaiveDate::from_isoywd(
            rug_fuzz_2,
            rug_fuzz_3,
            Weekday::Wed,
        );
        debug_assert_eq!(arbitrary_date, NaiveDate::from_ymd(2020, 11, 4));
        let first_week = NaiveDate::from_isoywd(rug_fuzz_4, rug_fuzz_5, Weekday::Sun);
        debug_assert_eq!(first_week, NaiveDate::from_ymd(2020, 12, 27));
        let last_week = NaiveDate::from_isoywd(rug_fuzz_6, rug_fuzz_7, Weekday::Fri);
        debug_assert_eq!(last_week, NaiveDate::from_ymd(2021, 12, 31));
             }
});    }
    #[test]
    #[should_panic]
    fn test_from_isoywd_with_invalid_week() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let _ = NaiveDate::from_isoywd(rug_fuzz_0, rug_fuzz_1, Weekday::Mon);
             }
});    }
    #[test]
    #[should_panic]
    fn test_from_isoywd_with_week_out_of_range() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let _ = NaiveDate::from_isoywd(rug_fuzz_0, rug_fuzz_1, Weekday::Mon);
             }
});    }
    #[test]
    #[should_panic]
    fn test_from_isoywd_with_invalid_weekday() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, i32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let invalid_weekday = Weekday::from_str(rug_fuzz_0).unwrap_or(Weekday::Mon);
        let _ = NaiveDate::from_isoywd(rug_fuzz_1, rug_fuzz_2, invalid_weekday);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_375 {
    use super::*;
    use crate::*;
    use crate::naive::date::NaiveDate;
    use crate::weekday::Weekday;
    #[test]
    fn test_from_isoywd_opt() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17, mut rug_fuzz_18, mut rug_fuzz_19, mut rug_fuzz_20, mut rug_fuzz_21, mut rug_fuzz_22, mut rug_fuzz_23, mut rug_fuzz_24, mut rug_fuzz_25)) = <(i32, u32, i32, u32, i32, u32, i32, u32, i32, u32, i32, u32, i32, u32, i32, u32, i32, u32, i32, u32, i32, u32, i32, u32, i32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            NaiveDate::from_isoywd_opt(rug_fuzz_0, rug_fuzz_1, Weekday::Sun), None
        );
        debug_assert_eq!(
            NaiveDate::from_isoywd_opt(rug_fuzz_2, rug_fuzz_3, Weekday::Sun),
            Some(NaiveDate::from_ymd(2015, 3, 8))
        );
        debug_assert_eq!(
            NaiveDate::from_isoywd_opt(rug_fuzz_4, rug_fuzz_5, Weekday::Mon),
            Some(NaiveDate::from_ymd(2015, 7, 20))
        );
        debug_assert_eq!(
            NaiveDate::from_isoywd_opt(rug_fuzz_6, rug_fuzz_7, Weekday::Mon), None
        );
        debug_assert_eq!(
            NaiveDate::from_isoywd_opt(rug_fuzz_8, rug_fuzz_9, Weekday::Fri), None
        );
        debug_assert_eq!(
            NaiveDate::from_isoywd_opt(- rug_fuzz_10, rug_fuzz_11, Weekday::Sat), None
        );
        debug_assert_eq!(
            NaiveDate::from_isoywd_opt(rug_fuzz_12, rug_fuzz_13, Weekday::Sun),
            Some(NaiveDate::from_ymd(2014, 12, 28))
        );
        debug_assert_eq!(
            NaiveDate::from_isoywd_opt(rug_fuzz_14, rug_fuzz_15, Weekday::Mon), None
        );
        debug_assert_eq!(
            NaiveDate::from_isoywd_opt(rug_fuzz_16, rug_fuzz_17, Weekday::Mon),
            Some(NaiveDate::from_ymd(2014, 12, 29))
        );
        debug_assert_eq!(
            NaiveDate::from_isoywd_opt(rug_fuzz_18, rug_fuzz_19, Weekday::Sun),
            Some(NaiveDate::from_ymd(2015, 12, 27))
        );
        debug_assert_eq!(
            NaiveDate::from_isoywd_opt(rug_fuzz_20, rug_fuzz_21, Weekday::Sun),
            Some(NaiveDate::from_ymd(2016, 1, 3))
        );
        debug_assert_eq!(
            NaiveDate::from_isoywd_opt(rug_fuzz_22, rug_fuzz_23, Weekday::Mon), None
        );
        debug_assert_eq!(
            NaiveDate::from_isoywd_opt(rug_fuzz_24, rug_fuzz_25, Weekday::Mon),
            Some(NaiveDate::from_ymd(2016, 1, 4))
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_376 {
    use crate::NaiveDate;
    use crate::naive::internals::{Mdf, YearFlags, Of};
    #[test]
    fn test_from_mdf_valid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i32, u8, u32, u32, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let year = rug_fuzz_0;
        let flags = YearFlags(rug_fuzz_1);
        let month = rug_fuzz_2;
        let day = rug_fuzz_3;
        let mdf = Mdf::new(month, day, flags).expect(rug_fuzz_4);
        let date = NaiveDate::from_mdf(year, mdf);
        debug_assert!(date.is_some());
             }
});    }
    #[test]
    fn test_from_mdf_invalid_month() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u8, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let year = rug_fuzz_0;
        let flags = YearFlags(rug_fuzz_1);
        let month = rug_fuzz_2;
        let day = rug_fuzz_3;
        let mdf = Mdf::new(month, day, flags);
        debug_assert!(mdf.is_none());
             }
});    }
    #[test]
    fn test_from_mdf_invalid_day() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u8, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let year = rug_fuzz_0;
        let flags = YearFlags(rug_fuzz_1);
        let month = rug_fuzz_2;
        let day = rug_fuzz_3;
        let mdf = Mdf::new(month, day, flags);
        debug_assert!(mdf.is_none());
             }
});    }
    #[test]
    fn test_from_mdf_leap_year() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i32, u8, u32, u32, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let year = rug_fuzz_0;
        let flags = YearFlags(rug_fuzz_1);
        let month = rug_fuzz_2;
        let day = rug_fuzz_3;
        let mdf = Mdf::new(month, day, flags).expect(rug_fuzz_4);
        let date = NaiveDate::from_mdf(year, mdf);
        debug_assert!(date.is_some());
             }
});    }
    #[test]
    fn test_from_mdf_non_leap_year() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u8, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let year = rug_fuzz_0;
        let flags = YearFlags(rug_fuzz_1);
        let month = rug_fuzz_2;
        let day = rug_fuzz_3;
        let mdf = Mdf::new(month, day, flags);
        debug_assert!(mdf.is_none());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_377 {
    use super::*;
    use crate::*;
    use crate::naive::date::NaiveDate;
    #[test]
    fn test_from_num_days_from_ce_valid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date_1_ce = NaiveDate::from_num_days_from_ce(rug_fuzz_0);
        debug_assert_eq!(date_1_ce, NaiveDate::from_ymd(1, 1, 1));
        let date_1970 = NaiveDate::from_num_days_from_ce(rug_fuzz_1);
        debug_assert_eq!(date_1970, NaiveDate::from_ymd(1970, 1, 1));
        let date_2000 = NaiveDate::from_num_days_from_ce(rug_fuzz_2);
        debug_assert_eq!(date_2000, NaiveDate::from_ymd(2000, 1, 1));
             }
});    }
    #[test]
    #[should_panic(expected = "out-of-range date")]
    fn test_from_num_days_from_ce_invalid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let _ = NaiveDate::from_num_days_from_ce(-rug_fuzz_0);
             }
});    }
    #[test]
    fn test_from_num_days_from_ce_edge_cases() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let min_days = NaiveDate::MIN
            .signed_duration_since(
                NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2),
            )
            .num_days();
        let min_date = NaiveDate::from_num_days_from_ce(min_days as i32);
        debug_assert_eq!(min_date, NaiveDate::MIN);
        let max_days = NaiveDate::MAX
            .signed_duration_since(
                NaiveDate::from_ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5),
            )
            .num_days();
        let max_date = NaiveDate::from_num_days_from_ce(max_days as i32);
        debug_assert_eq!(max_date, NaiveDate::MAX);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_378 {
    use super::*;
    use crate::*;
    #[test]
    fn test_from_num_days_from_ce_opt() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, i32, i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let from_ndays_opt = NaiveDate::from_num_days_from_ce_opt;
        let from_ymd_opt = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();
        debug_assert_eq!(from_ndays_opt(rug_fuzz_0), Some(from_ymd_opt(1999, 9, 3)));
        debug_assert_eq!(from_ndays_opt(rug_fuzz_1), Some(from_ymd_opt(1, 1, 1)));
        debug_assert_eq!(from_ndays_opt(rug_fuzz_2), Some(from_ymd_opt(0, 12, 31)));
        debug_assert_eq!(from_ndays_opt(- rug_fuzz_3), Some(from_ymd_opt(0, 12, 30)));
        debug_assert_eq!(from_ndays_opt(rug_fuzz_4), None);
        debug_assert_eq!(from_ndays_opt(- rug_fuzz_5), None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_380 {
    use super::*;
    use crate::*;
    use crate::Weekday;
    #[test]
    fn test_from_weekday_of_month_valid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17)) = <(i32, u32, u32, i32, u32, u8, i32, u32, u32, i32, u32, u8, i32, u32, u32, i32, u32, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let expected_date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let result_date = NaiveDate::from_weekday_of_month(
            rug_fuzz_3,
            rug_fuzz_4,
            Weekday::Mon,
            rug_fuzz_5,
        );
        debug_assert_eq!(
            result_date, expected_date,
            "1st Monday of February 2023 should be February 6, 2023"
        );
        let expected_date = NaiveDate::from_ymd(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8);
        let result_date = NaiveDate::from_weekday_of_month(
            rug_fuzz_9,
            rug_fuzz_10,
            Weekday::Wed,
            rug_fuzz_11,
        );
        debug_assert_eq!(
            result_date, expected_date,
            "3rd Wednesday of March 2023 should be March 15, 2023"
        );
        let expected_date = NaiveDate::from_ymd(rug_fuzz_12, rug_fuzz_13, rug_fuzz_14);
        let result_date = NaiveDate::from_weekday_of_month(
            rug_fuzz_15,
            rug_fuzz_16,
            Weekday::Fri,
            rug_fuzz_17,
        );
        debug_assert_eq!(
            result_date, expected_date,
            "2nd Friday of December 2023 should be December 8, 2023"
        );
             }
});    }
    #[test]
    #[should_panic(expected = "out-of-range date")]
    fn test_from_weekday_of_month_panic_n_0() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        NaiveDate::from_weekday_of_month(
            rug_fuzz_0,
            rug_fuzz_1,
            Weekday::Mon,
            rug_fuzz_2,
        );
             }
});    }
    #[test]
    #[should_panic(expected = "out-of-range date")]
    fn test_from_weekday_of_month_panic_invalid_weekday() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        NaiveDate::from_weekday_of_month(
            rug_fuzz_0,
            rug_fuzz_1,
            Weekday::Mon,
            rug_fuzz_2,
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_381 {
    use crate::NaiveDate;
    use crate::Weekday;
    #[test]
    fn test_from_weekday_of_month_opt() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17, mut rug_fuzz_18, mut rug_fuzz_19, mut rug_fuzz_20, mut rug_fuzz_21, mut rug_fuzz_22, mut rug_fuzz_23)) = <(i32, u32, u8, i32, u32, u8, i32, u32, u8, i32, u32, u8, i32, u32, u8, i32, u32, u8, i32, u32, u8, i32, u32, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            NaiveDate::from_weekday_of_month_opt(rug_fuzz_0, rug_fuzz_1, Weekday::Fri,
            rug_fuzz_2), NaiveDate::from_ymd_opt(2017, 3, 10)
        );
        debug_assert_eq!(
            NaiveDate::from_weekday_of_month_opt(rug_fuzz_3, rug_fuzz_4, Weekday::Fri,
            rug_fuzz_5), None
        );
        debug_assert_eq!(
            NaiveDate::from_weekday_of_month_opt(rug_fuzz_6, rug_fuzz_7, Weekday::Fri,
            rug_fuzz_8), None
        );
        debug_assert_eq!(
            NaiveDate::from_weekday_of_month_opt(rug_fuzz_9, rug_fuzz_10, Weekday::Wed,
            rug_fuzz_11), NaiveDate::from_ymd_opt(2017, 2, 22)
        );
        debug_assert_eq!(
            NaiveDate::from_weekday_of_month_opt(rug_fuzz_12, rug_fuzz_13, Weekday::Wed,
            rug_fuzz_14), NaiveDate::from_ymd_opt(2017, 3, 1)
        );
        debug_assert_eq!(
            NaiveDate::from_weekday_of_month_opt(rug_fuzz_15, rug_fuzz_16, Weekday::Mon,
            rug_fuzz_17), None
        );
        debug_assert_eq!(
            NaiveDate::from_weekday_of_month_opt(rug_fuzz_18, rug_fuzz_19, Weekday::Sun,
            rug_fuzz_20), NaiveDate::from_ymd_opt(2017, 10, 1)
        );
        debug_assert_eq!(
            NaiveDate::from_weekday_of_month_opt(rug_fuzz_21, rug_fuzz_22, Weekday::Mon,
            rug_fuzz_23), NaiveDate::from_ymd_opt(2016, 2, 29)
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_382 {
    use crate::NaiveDate;
    use crate::Datelike;
    #[test]
    fn test_from_ymd_valid_dates() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17)) = <(i32, u32, u32, i32, u32, u32, i32, u32, u32, i32, u32, u32, i32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).year(), 2000
        );
        debug_assert_eq!(
            NaiveDate::from_ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5).month(), 2
        );
        debug_assert_eq!(
            NaiveDate::from_ymd(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8).day(), 29
        );
        debug_assert_eq!(
            NaiveDate::from_ymd(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11).year(), 2023
        );
        debug_assert_eq!(
            NaiveDate::from_ymd(rug_fuzz_12, rug_fuzz_13, rug_fuzz_14).month(), 3
        );
        debug_assert_eq!(
            NaiveDate::from_ymd(rug_fuzz_15, rug_fuzz_16, rug_fuzz_17).day(), 30
        );
             }
});    }
    #[test]
    #[should_panic(expected = "invalid or out-of-range date")]
    fn test_from_ymd_invalid_month() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
             }
});    }
    #[test]
    #[should_panic(expected = "invalid or out-of-range date")]
    fn test_from_ymd_invalid_day() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
             }
});    }
    #[test]
    #[should_panic(expected = "invalid or out-of-range date")]
    fn test_from_ymd_out_of_range_year() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        NaiveDate::from_ymd(-rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_383 {
    use super::*;
    use crate::*;
    use crate::naive::date::NaiveDate;
    #[test]
    fn test_from_ymd_opt() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17)) = <(i32, u32, u32, i32, u32, u32, i32, u32, u32, i32, u32, u32, i32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(
            NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).is_some()
        );
        debug_assert!(
            NaiveDate::from_ymd_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5).is_none()
        );
        debug_assert!(
            NaiveDate::from_ymd_opt(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8).is_none()
        );
        debug_assert!(
            NaiveDate::from_ymd_opt(- rug_fuzz_9, rug_fuzz_10, rug_fuzz_11).is_some()
        );
        debug_assert!(
            NaiveDate::from_ymd_opt(rug_fuzz_12, rug_fuzz_13, rug_fuzz_14).is_none()
        );
        debug_assert!(
            NaiveDate::from_ymd_opt(- rug_fuzz_15, rug_fuzz_16, rug_fuzz_17).is_none()
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_384 {
    use super::*;
    use crate::*;
    use crate::{Datelike, NaiveDate};
    #[test]
    fn test_from_yo_valid_dates() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(i32, u32, i32, u32, i32, u32, i32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            NaiveDate::from_yo(rug_fuzz_0, rug_fuzz_1), NaiveDate::from_ymd(2023, 1, 1)
        );
        debug_assert_eq!(
            NaiveDate::from_yo(rug_fuzz_2, rug_fuzz_3), NaiveDate::from_ymd(2023, 12, 31)
        );
        debug_assert_eq!(
            NaiveDate::from_yo(rug_fuzz_4, rug_fuzz_5), NaiveDate::from_ymd(2024, 1, 1)
        );
        debug_assert_eq!(
            NaiveDate::from_yo(rug_fuzz_6, rug_fuzz_7), NaiveDate::from_ymd(2024, 12, 31)
        );
             }
});    }
    #[test]
    #[should_panic(expected = "invalid or out-of-range date")]
    fn test_from_yo_panic_before_min() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        NaiveDate::from_yo(rug_fuzz_0, rug_fuzz_1);
             }
});    }
    #[test]
    #[should_panic(expected = "invalid or out-of-range date")]
    fn test_from_yo_panic_after_max() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        NaiveDate::from_yo(rug_fuzz_0, rug_fuzz_1);
             }
});    }
    #[test]
    #[should_panic(expected = "invalid or out-of-range date")]
    fn test_from_yo_panic_leap_year() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        NaiveDate::from_yo(rug_fuzz_0, rug_fuzz_1);
             }
});    }
    #[test]
    #[should_panic(expected = "invalid or out-of-range date")]
    fn test_from_yo_panic_year_out_of_range() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        NaiveDate::from_yo(rug_fuzz_0, rug_fuzz_1);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_385 {
    use super::*;
    use crate::*;
    #[test]
    fn test_from_yo_opt() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13)) = <(i32, u32, i32, u32, i32, u32, i32, u32, i32, u32, i32, u32, i32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(NaiveDate::from_yo_opt(rug_fuzz_0, rug_fuzz_1).is_some());
        debug_assert!(NaiveDate::from_yo_opt(rug_fuzz_2, rug_fuzz_3).is_none());
        debug_assert!(NaiveDate::from_yo_opt(rug_fuzz_4, rug_fuzz_5).is_some());
        debug_assert!(NaiveDate::from_yo_opt(rug_fuzz_6, rug_fuzz_7).is_none());
        debug_assert!(NaiveDate::from_yo_opt(- rug_fuzz_8, rug_fuzz_9).is_some());
        debug_assert!(NaiveDate::from_yo_opt(rug_fuzz_10, rug_fuzz_11).is_none());
        debug_assert!(NaiveDate::from_yo_opt(- rug_fuzz_12, rug_fuzz_13).is_none());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_386 {
    use super::*;
    use crate::*;
    use crate::naive::date::NaiveDate;
    #[test]
    fn test_iter_days() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u32, u32, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let start_date = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .unwrap();
        let iterator = start_date.iter_days();
        for (idx, date) in iterator.take(rug_fuzz_3).enumerate() {
            debug_assert_eq!(
                date, NaiveDate::from_ymd_opt(2020, 1, idx as u32 + 1).unwrap()
            );
        }
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_388 {
    use crate::NaiveDate;
    use crate::naive::internals::{Mdf, YearFlags};
    use crate::naive::date::MIN_YEAR;
    #[test]
    fn test_mdf_returns_correct_month_day_flags() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let naive_date_ymd = NaiveDate::from_ymd_opt(MIN_YEAR, rug_fuzz_0, rug_fuzz_1)
            .unwrap();
        let naive_date_mdf = naive_date_ymd.mdf();
        let expected_flags = YearFlags::from_year(MIN_YEAR);
        let expected_mdf = Mdf::new(rug_fuzz_2, rug_fuzz_3, expected_flags).unwrap();
        debug_assert_eq!(naive_date_mdf, expected_mdf);
             }
});    }
    #[test]
    fn test_mdf_for_leap_year() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let naive_date_ymd = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .unwrap();
        let naive_date_mdf = naive_date_ymd.mdf();
        let expected_flags = YearFlags::from_year(rug_fuzz_3);
        let expected_mdf = Mdf::new(rug_fuzz_4, rug_fuzz_5, expected_flags).unwrap();
        debug_assert_eq!(naive_date_mdf, expected_mdf);
             }
});    }
    #[test]
    fn test_mdf_for_non_leap_year() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let naive_date_ymd = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .unwrap();
        let naive_date_mdf = naive_date_ymd.mdf();
        let expected_flags = YearFlags::from_year(rug_fuzz_3);
        let expected_mdf = Mdf::new(rug_fuzz_4, rug_fuzz_5, expected_flags).unwrap();
        debug_assert_eq!(naive_date_mdf, expected_mdf);
             }
});    }
    #[test]
    fn test_mdf_for_last_day_of_month() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let naive_date_ymd = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .unwrap();
        let naive_date_mdf = naive_date_ymd.mdf();
        let expected_flags = YearFlags::from_year(rug_fuzz_3);
        let expected_mdf = Mdf::new(rug_fuzz_4, rug_fuzz_5, expected_flags).unwrap();
        debug_assert_eq!(naive_date_mdf, expected_mdf);
             }
});    }
    #[test]
    fn test_mdf_for_end_of_year() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let naive_date_ymd = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .unwrap();
        let naive_date_mdf = naive_date_ymd.mdf();
        let expected_flags = YearFlags::from_year(rug_fuzz_3);
        let expected_mdf = Mdf::new(rug_fuzz_4, rug_fuzz_5, expected_flags).unwrap();
        debug_assert_eq!(naive_date_mdf, expected_mdf);
             }
});    }
    #[test]
    fn test_mdf_for_start_of_year() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let naive_date_ymd = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .unwrap();
        let naive_date_mdf = naive_date_ymd.mdf();
        let expected_flags = YearFlags::from_year(rug_fuzz_3);
        let expected_mdf = Mdf::new(rug_fuzz_4, rug_fuzz_5, expected_flags).unwrap();
        debug_assert_eq!(naive_date_mdf, expected_mdf);
             }
});    }
    #[test]
    fn test_mdf_for_nonexistent_date() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let expected_flags = YearFlags::from_year(rug_fuzz_0);
        let naive_date_mdf = Mdf::new(rug_fuzz_1, rug_fuzz_2, expected_flags);
        debug_assert!(naive_date_mdf.is_none());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_389 {
    use crate::NaiveDate;
    use crate::naive::internals::Of;
    use crate::naive::date::NaiveDateDaysIterator;
    use crate::Datelike;
    #[test]
    fn test_naive_date_of() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let test_date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let extracted_of = test_date.of();
        debug_assert_eq!(extracted_of, Of((257 << 4) | 0b011));
             }
});    }
    #[test]
    fn test_naive_date_of_min() {
        let _rug_st_tests_llm_16_389_rrrruuuugggg_test_naive_date_of_min = 0;
        let test_date = NaiveDate::MIN;
        let extracted_of = test_date.of();
        debug_assert_eq!(extracted_of, Of((1 << 4) | 0b111));
        let _rug_ed_tests_llm_16_389_rrrruuuugggg_test_naive_date_of_min = 0;
    }
    #[test]
    fn test_naive_date_of_max() {
        let _rug_st_tests_llm_16_389_rrrruuuugggg_test_naive_date_of_max = 0;
        let test_date = NaiveDate::MAX;
        let extracted_of = test_date.of();
        debug_assert_eq!(extracted_of, Of((365 << 4) | 0b111));
        let _rug_ed_tests_llm_16_389_rrrruuuugggg_test_naive_date_of_max = 0;
    }
    #[test]
    fn test_naive_date_of_ordinal() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let test_date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let extracted_of = test_date.of();
        debug_assert_eq!(extracted_of.ordinal(), 257);
             }
});    }
    #[test]
    fn test_naive_date_of_succ() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let test_date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let next_day_of = test_date.of().succ();
        let next_day_date = NaiveDate::from_ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        debug_assert_eq!(next_day_of, next_day_date.of());
             }
});    }
    #[test]
    fn test_naive_date_of_pred() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let test_date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let prev_day_of = test_date.of().pred();
        let prev_day_date = NaiveDate::from_ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        debug_assert_eq!(prev_day_of, prev_day_date.of());
             }
});    }
    #[test]
    fn test_naive_date_iter() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let start_date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let end_date = NaiveDate::from_ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let mut iter = start_date.iter_days();
        debug_assert_eq!(iter.next(), Some(NaiveDate::from_ymd(2023, 9, 1)));
        debug_assert_eq!(iter.next(), Some(NaiveDate::from_ymd(2023, 9, 2)));
        debug_assert_eq!(iter.next(), Some(NaiveDate::from_ymd(2023, 9, 3)));
        debug_assert_eq!(iter.next(), Some(NaiveDate::from_ymd(2023, 9, 4)));
        debug_assert_eq!(iter.next(), Some(NaiveDate::from_ymd(2023, 9, 5)));
        debug_assert_eq!(iter.next(), Some(NaiveDate::from_ymd(2023, 9, 6)));
        debug_assert_eq!(iter.next(), Some(NaiveDate::from_ymd(2023, 9, 7)));
             }
});    }
    #[test]
    fn test_naive_date_of_the_days_iterator() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let mut iter = date.iter_days();
        debug_assert_eq!(iter.next(), Some(NaiveDate::from_ymd(2023, 9, 14)));
        debug_assert_eq!(iter.next(), Some(NaiveDate::from_ymd(2023, 9, 15)));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_390 {
    use crate::naive::date::NaiveDate;
    use crate::ParseResult;
    #[test]
    fn test_parse_from_str_valid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            NaiveDate::parse_from_str(rug_fuzz_0, rug_fuzz_1),
            Ok(NaiveDate::from_ymd(2015, 9, 5))
        );
        debug_assert_eq!(
            NaiveDate::parse_from_str(rug_fuzz_2, rug_fuzz_3),
            Ok(NaiveDate::from_ymd(2015, 9, 5))
        );
        debug_assert_eq!(
            NaiveDate::parse_from_str(rug_fuzz_4, rug_fuzz_5),
            Ok(NaiveDate::from_ymd(2014, 5, 17))
        );
             }
});    }
    #[test]
    fn test_parse_from_str_invalid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(NaiveDate::parse_from_str(rug_fuzz_0, rug_fuzz_1).is_err());
        debug_assert!(NaiveDate::parse_from_str(rug_fuzz_2, rug_fuzz_3).is_err());
        debug_assert!(NaiveDate::parse_from_str(rug_fuzz_4, rug_fuzz_5).is_err());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_391 {
    use crate::NaiveDate;
    use crate::Datelike;
    #[test]
    fn test_pred_with_valid_date() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let expected = NaiveDate::from_ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        debug_assert_eq!(date.pred(), expected);
             }
});    }
    #[test]
    #[should_panic(expected = "out of bound")]
    fn test_pred_with_first_date() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(-rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let _ = date.pred();
             }
});    }
    #[test]
    fn test_pred_with_leap_year() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let expected = NaiveDate::from_ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        debug_assert_eq!(date.pred(), expected);
             }
});    }
    #[test]
    fn test_pred_with_new_year() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let expected = NaiveDate::from_ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        debug_assert_eq!(date.pred(), expected);
             }
});    }
    #[test]
    fn test_pred_with_common_year() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let expected = NaiveDate::from_ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        debug_assert_eq!(date.pred(), expected);
             }
});    }
    #[test]
    fn test_pred_with_year_boundary() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let expected = NaiveDate::from_ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        debug_assert_eq!(date.pred(), expected);
             }
});    }
    #[test]
    fn test_pred_with_century_boundary() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let expected = NaiveDate::from_ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        debug_assert_eq!(date.pred(), expected);
             }
});    }
    #[test]
    fn test_pred_with_non_century_leap_year() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let expected = NaiveDate::from_ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        debug_assert_eq!(date.pred(), expected);
             }
});    }
    #[test]
    fn test_pred_with_century_non_leap_year() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let expected = NaiveDate::from_ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        debug_assert_eq!(date.pred(), expected);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_392 {
    use super::*;
    use crate::*;
    use crate::NaiveDate;
    #[test]
    fn test_pred_opt() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(i32, u32, u32, i32, u32, u32, i32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap()
            .pred_opt(), Some(NaiveDate::from_ymd_opt(2015, 6, 2).unwrap())
        );
        debug_assert_eq!(
            NaiveDate::from_ymd_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5).unwrap()
            .pred_opt(), Some(NaiveDate::from_ymd_opt(2014, 12, 31).unwrap())
        );
        debug_assert_eq!(
            NaiveDate::from_ymd_opt(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8).unwrap()
            .pred_opt(), Some(NaiveDate::from_ymd_opt(2015, 2, 28).unwrap())
        );
        debug_assert_eq!(
            NaiveDate::from_ymd_opt(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11).unwrap()
            .pred_opt(), Some(NaiveDate::from_ymd_opt(2016, 2, 29).unwrap())
        );
        debug_assert_eq!(NaiveDate::MIN.pred_opt(), None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_393 {
    use crate::NaiveDate;
    use crate::time_delta::TimeDelta;
    #[test]
    fn test_signed_duration_since() {
        let _rug_st_tests_llm_16_393_rrrruuuugggg_test_signed_duration_since = 0;
        let rug_fuzz_0 = 2014;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 2014;
        let rug_fuzz_4 = 1;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 2014;
        let rug_fuzz_7 = 1;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 2013;
        let rug_fuzz_10 = 12;
        let rug_fuzz_11 = 31;
        let rug_fuzz_12 = 2014;
        let rug_fuzz_13 = 1;
        let rug_fuzz_14 = 1;
        let rug_fuzz_15 = 2014;
        let rug_fuzz_16 = 1;
        let rug_fuzz_17 = 2;
        let rug_fuzz_18 = 2014;
        let rug_fuzz_19 = 1;
        let rug_fuzz_20 = 1;
        let rug_fuzz_21 = 2013;
        let rug_fuzz_22 = 9;
        let rug_fuzz_23 = 23;
        let rug_fuzz_24 = 2014;
        let rug_fuzz_25 = 1;
        let rug_fuzz_26 = 1;
        let rug_fuzz_27 = 2013;
        let rug_fuzz_28 = 1;
        let rug_fuzz_29 = 1;
        let rug_fuzz_30 = 2014;
        let rug_fuzz_31 = 1;
        let rug_fuzz_32 = 1;
        let rug_fuzz_33 = 2010;
        let rug_fuzz_34 = 1;
        let rug_fuzz_35 = 1;
        let rug_fuzz_36 = 2014;
        let rug_fuzz_37 = 1;
        let rug_fuzz_38 = 1;
        let rug_fuzz_39 = 1614;
        let rug_fuzz_40 = 1;
        let rug_fuzz_41 = 1;
        let from_ymd = NaiveDate::from_ymd;
        debug_assert_eq!(
            NaiveDate::signed_duration_since(from_ymd(rug_fuzz_0, rug_fuzz_1,
            rug_fuzz_2), from_ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)), TimeDelta::zero()
        );
        debug_assert_eq!(
            NaiveDate::signed_duration_since(from_ymd(rug_fuzz_6, rug_fuzz_7,
            rug_fuzz_8), from_ymd(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11)),
            TimeDelta::days(1)
        );
        debug_assert_eq!(
            NaiveDate::signed_duration_since(from_ymd(rug_fuzz_12, rug_fuzz_13,
            rug_fuzz_14), from_ymd(rug_fuzz_15, rug_fuzz_16, rug_fuzz_17)),
            TimeDelta::days(- 1)
        );
        debug_assert_eq!(
            NaiveDate::signed_duration_since(from_ymd(rug_fuzz_18, rug_fuzz_19,
            rug_fuzz_20), from_ymd(rug_fuzz_21, rug_fuzz_22, rug_fuzz_23)),
            TimeDelta::days(100)
        );
        debug_assert_eq!(
            NaiveDate::signed_duration_since(from_ymd(rug_fuzz_24, rug_fuzz_25,
            rug_fuzz_26), from_ymd(rug_fuzz_27, rug_fuzz_28, rug_fuzz_29)),
            TimeDelta::days(365)
        );
        debug_assert_eq!(
            NaiveDate::signed_duration_since(from_ymd(rug_fuzz_30, rug_fuzz_31,
            rug_fuzz_32), from_ymd(rug_fuzz_33, rug_fuzz_34, rug_fuzz_35)),
            TimeDelta::days(365 * 4 + 1)
        );
        debug_assert_eq!(
            NaiveDate::signed_duration_since(from_ymd(rug_fuzz_36, rug_fuzz_37,
            rug_fuzz_38), from_ymd(rug_fuzz_39, rug_fuzz_40, rug_fuzz_41)),
            TimeDelta::days(365 * 400 + 97)
        );
        let _rug_ed_tests_llm_16_393_rrrruuuugggg_test_signed_duration_since = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_394 {
    use crate::naive::date::NaiveDate;
    use crate::Datelike;
    #[test]
    fn test_succ() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let next_date = date.succ();
        debug_assert_eq!(next_date.year(), 2023);
        debug_assert_eq!(next_date.month(), 3);
        debug_assert_eq!(next_date.day(), 15);
             }
});    }
    #[test]
    #[should_panic(expected = "out of bound")]
    fn test_succ_panics_at_max_date() {
        let _rug_st_tests_llm_16_394_rrrruuuugggg_test_succ_panics_at_max_date = 0;
        let max_date = NaiveDate::MAX;
        max_date.succ();
        let _rug_ed_tests_llm_16_394_rrrruuuugggg_test_succ_panics_at_max_date = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_395 {
    use super::*;
    use crate::*;
    use crate::naive::date::NaiveDate;
    #[test]
    fn test_succ_opt() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        let next_date = date.succ_opt();
        debug_assert_eq!(next_date, Some(NaiveDate::from_ymd_opt(2023, 3, 15).unwrap()));
        let date = NaiveDate::from_ymd_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5).unwrap();
        let next_date = date.succ_opt();
        debug_assert_eq!(next_date, Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()));
        let date = NaiveDate::MAX;
        let next_date = date.succ_opt();
        debug_assert_eq!(next_date, None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_396 {
    use crate::{NaiveDate, NaiveWeek, Weekday};
    use std::ops::RangeInclusive;
    #[test]
    fn test_naive_week_methods() {
        let _rug_st_tests_llm_16_396_rrrruuuugggg_test_naive_week_methods = 0;
        let rug_fuzz_0 = 2022;
        let rug_fuzz_1 = 4;
        let rug_fuzz_2 = 18;
        let rug_fuzz_3 = 2022;
        let rug_fuzz_4 = 4;
        let rug_fuzz_5 = 18;
        let rug_fuzz_6 = 2022;
        let rug_fuzz_7 = 4;
        let rug_fuzz_8 = 24;
        let rug_fuzz_9 = 2022;
        let rug_fuzz_10 = 4;
        let rug_fuzz_11 = 18;
        let rug_fuzz_12 = 2022;
        let rug_fuzz_13 = 4;
        let rug_fuzz_14 = 19;
        let rug_fuzz_15 = 2022;
        let rug_fuzz_16 = 4;
        let rug_fuzz_17 = 20;
        let rug_fuzz_18 = 2022;
        let rug_fuzz_19 = 4;
        let rug_fuzz_20 = 21;
        let rug_fuzz_21 = 2022;
        let rug_fuzz_22 = 4;
        let rug_fuzz_23 = 22;
        let rug_fuzz_24 = 2022;
        let rug_fuzz_25 = 4;
        let rug_fuzz_26 = 23;
        let rug_fuzz_27 = 2022;
        let rug_fuzz_28 = 4;
        let rug_fuzz_29 = 24;
        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let week = date.week(Weekday::Mon);
        debug_assert_eq!(week.first_day(), NaiveDate::from_ymd(2022, 4, 18));
        debug_assert_eq!(week.last_day(), NaiveDate::from_ymd(2022, 4, 24));
        let expected_days: RangeInclusive<NaiveDate> = NaiveDate::from_ymd(
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
        )..=NaiveDate::from_ymd(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8);
        let week_days = week.days();
        debug_assert_eq!(week_days, expected_days);
        let mut days = week_days;
        debug_assert!(
            days.contains(& NaiveDate::from_ymd(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11))
        );
        debug_assert!(
            days.contains(& NaiveDate::from_ymd(rug_fuzz_12, rug_fuzz_13, rug_fuzz_14))
        );
        debug_assert!(
            days.contains(& NaiveDate::from_ymd(rug_fuzz_15, rug_fuzz_16, rug_fuzz_17))
        );
        debug_assert!(
            days.contains(& NaiveDate::from_ymd(rug_fuzz_18, rug_fuzz_19, rug_fuzz_20))
        );
        debug_assert!(
            days.contains(& NaiveDate::from_ymd(rug_fuzz_21, rug_fuzz_22, rug_fuzz_23))
        );
        debug_assert!(
            days.contains(& NaiveDate::from_ymd(rug_fuzz_24, rug_fuzz_25, rug_fuzz_26))
        );
        debug_assert!(
            days.contains(& NaiveDate::from_ymd(rug_fuzz_27, rug_fuzz_28, rug_fuzz_29))
        );
        let _rug_ed_tests_llm_16_396_rrrruuuugggg_test_naive_week_methods = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_397 {
    use super::*;
    use crate::*;
    #[test]
    fn test_weeks_from() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        use crate::Weekday::*;
        use crate::NaiveDate;
        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(date.weeks_from(Mon), 1);
        debug_assert_eq!(date.weeks_from(Tue), 0);
        debug_assert_eq!(date.weeks_from(Wed), 0);
        debug_assert_eq!(date.weeks_from(Thu), 0);
        debug_assert_eq!(date.weeks_from(Fri), 0);
        debug_assert_eq!(date.weeks_from(Sat), 0);
        debug_assert_eq!(date.weeks_from(Sun), 0);
        let date = NaiveDate::from_ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        debug_assert_eq!(date.weeks_from(Mon), 1);
        debug_assert_eq!(date.weeks_from(Tue), 1);
        debug_assert_eq!(date.weeks_from(Wed), 1);
        debug_assert_eq!(date.weeks_from(Thu), 0);
        debug_assert_eq!(date.weeks_from(Fri), 0);
        debug_assert_eq!(date.weeks_from(Sat), 0);
        debug_assert_eq!(date.weeks_from(Sun), 0);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_399 {
    use super::*;
    use crate::*;
    use crate::naive::date::NaiveDate;
    use crate::naive::internals::Of;
    use crate::naive::NaiveDateTime;
    #[test]
    fn test_with_of() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(i32, u32, u32, u32, u32, u32, i32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let of = Of::new(rug_fuzz_3, date.of().flags()).unwrap();
        let new_date = date.with_of(of);
        debug_assert_eq!(new_date, Some(NaiveDate::from_ymd(2015, 3, 1)));
        let of_invalid = Of::new(rug_fuzz_4, date.of().flags());
        debug_assert!(date.with_of(of_invalid.unwrap()).is_none());
        let of_end_year = Of::new(rug_fuzz_5, date.of().flags()).unwrap();
        let new_date_end_year = date.with_of(of_end_year);
        debug_assert_eq!(new_date_end_year, Some(NaiveDate::from_ymd(2015, 12, 31)));
        let leap_date = NaiveDate::from_ymd(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8);
        let of_leap = Of::new(rug_fuzz_9, leap_date.of().flags()).unwrap();
        let new_date_leap_year = leap_date.with_of(of_leap);
        debug_assert_eq!(new_date_leap_year, Some(NaiveDate::from_ymd(2016, 12, 31)));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_400 {
    use super::*;
    use crate::*;
    use crate::NaiveDate;
    #[test]
    fn test_years_since() {
        let _rug_st_tests_llm_16_400_rrrruuuugggg_test_years_since = 0;
        let rug_fuzz_0 = 2022;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 2020;
        let rug_fuzz_4 = 1;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 2022;
        let rug_fuzz_7 = 1;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 2022;
        let rug_fuzz_10 = 1;
        let rug_fuzz_11 = 1;
        let rug_fuzz_12 = 2022;
        let rug_fuzz_13 = 12;
        let rug_fuzz_14 = 31;
        let rug_fuzz_15 = 2020;
        let rug_fuzz_16 = 1;
        let rug_fuzz_17 = 1;
        let rug_fuzz_18 = 2020;
        let rug_fuzz_19 = 1;
        let rug_fuzz_20 = 1;
        let rug_fuzz_21 = 2022;
        let rug_fuzz_22 = 1;
        let rug_fuzz_23 = 1;
        let rug_fuzz_24 = 2021;
        let rug_fuzz_25 = 12;
        let rug_fuzz_26 = 31;
        let rug_fuzz_27 = 2020;
        let rug_fuzz_28 = 1;
        let rug_fuzz_29 = 1;
        let rug_fuzz_30 = 2020;
        let rug_fuzz_31 = 2;
        let rug_fuzz_32 = 29;
        let rug_fuzz_33 = 2019;
        let rug_fuzz_34 = 3;
        let rug_fuzz_35 = 1;
        let date1 = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let date2 = NaiveDate::from_ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        debug_assert_eq!(date1.years_since(date2), Some(2));
        let date1 = NaiveDate::from_ymd(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8);
        let date2 = NaiveDate::from_ymd(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11);
        debug_assert_eq!(date1.years_since(date2), Some(0));
        let date1 = NaiveDate::from_ymd(rug_fuzz_12, rug_fuzz_13, rug_fuzz_14);
        let date2 = NaiveDate::from_ymd(rug_fuzz_15, rug_fuzz_16, rug_fuzz_17);
        debug_assert_eq!(date1.years_since(date2), Some(2));
        let date1 = NaiveDate::from_ymd(rug_fuzz_18, rug_fuzz_19, rug_fuzz_20);
        let date2 = NaiveDate::from_ymd(rug_fuzz_21, rug_fuzz_22, rug_fuzz_23);
        debug_assert_eq!(date1.years_since(date2), None);
        let date1 = NaiveDate::from_ymd(rug_fuzz_24, rug_fuzz_25, rug_fuzz_26);
        let date2 = NaiveDate::from_ymd(rug_fuzz_27, rug_fuzz_28, rug_fuzz_29);
        debug_assert_eq!(date1.years_since(date2), Some(1));
        let date1 = NaiveDate::from_ymd(rug_fuzz_30, rug_fuzz_31, rug_fuzz_32);
        let date2 = NaiveDate::from_ymd(rug_fuzz_33, rug_fuzz_34, rug_fuzz_35);
        debug_assert_eq!(date1.years_since(date2), Some(0));
        let _rug_ed_tests_llm_16_400_rrrruuuugggg_test_years_since = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_401 {
    use super::*;
    use crate::*;
    use crate::{NaiveDate, TimeDelta, Weekday};
    #[test]
    fn test_week_days() {
        let _rug_st_tests_llm_16_401_rrrruuuugggg_test_week_days = 0;
        let rug_fuzz_0 = 2022;
        let rug_fuzz_1 = 4;
        let rug_fuzz_2 = 18;
        let rug_fuzz_3 = 2022;
        let rug_fuzz_4 = 4;
        let rug_fuzz_5 = 18;
        let rug_fuzz_6 = 2022;
        let rug_fuzz_7 = 4;
        let rug_fuzz_8 = 19;
        let rug_fuzz_9 = 2022;
        let rug_fuzz_10 = 4;
        let rug_fuzz_11 = 20;
        let rug_fuzz_12 = 2022;
        let rug_fuzz_13 = 4;
        let rug_fuzz_14 = 21;
        let rug_fuzz_15 = 2022;
        let rug_fuzz_16 = 4;
        let rug_fuzz_17 = 22;
        let rug_fuzz_18 = 2022;
        let rug_fuzz_19 = 4;
        let rug_fuzz_20 = 23;
        let rug_fuzz_21 = 2022;
        let rug_fuzz_22 = 4;
        let rug_fuzz_23 = 24;
        let rug_fuzz_24 = 2022;
        let rug_fuzz_25 = 4;
        let rug_fuzz_26 = 25;
        let date = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        let week = date.week(Weekday::Mon);
        let days = week.days();
        debug_assert!(
            days.contains(& NaiveDate::from_ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5))
        );
        debug_assert!(
            days.contains(& NaiveDate::from_ymd(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8))
        );
        debug_assert!(
            days.contains(& NaiveDate::from_ymd(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11))
        );
        debug_assert!(
            days.contains(& NaiveDate::from_ymd(rug_fuzz_12, rug_fuzz_13, rug_fuzz_14))
        );
        debug_assert!(
            days.contains(& NaiveDate::from_ymd(rug_fuzz_15, rug_fuzz_16, rug_fuzz_17))
        );
        debug_assert!(
            days.contains(& NaiveDate::from_ymd(rug_fuzz_18, rug_fuzz_19, rug_fuzz_20))
        );
        debug_assert!(
            days.contains(& NaiveDate::from_ymd(rug_fuzz_21, rug_fuzz_22, rug_fuzz_23))
        );
        debug_assert!(
            ! days.contains(& NaiveDate::from_ymd(rug_fuzz_24, rug_fuzz_25, rug_fuzz_26))
        );
        let _rug_ed_tests_llm_16_401_rrrruuuugggg_test_week_days = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_402 {
    use super::*;
    use crate::*;
    use crate::{NaiveDate, Weekday};
    #[test]
    fn test_first_day() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8)) = <(i32, u32, u32, i32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let week = date.week(Weekday::Mon);
        debug_assert_eq!(week.first_day(), NaiveDate::from_ymd(2022, 4, 18));
        let date = NaiveDate::from_ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let week = date.week(Weekday::Mon);
        debug_assert_eq!(week.first_day(), NaiveDate::from_ymd(2022, 4, 18));
        let date = NaiveDate::from_ymd(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8);
        let week = date.week(Weekday::Mon);
        debug_assert_eq!(week.first_day(), NaiveDate::from_ymd(2022, 4, 11));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_403 {
    use super::*;
    use crate::*;
    use crate::{NaiveDate, Weekday};
    #[test]
    fn test_last_day_monday_start() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        let week = date.week(Weekday::Mon);
        debug_assert_eq!(week.last_day(), NaiveDate::from_ymd_opt(2022, 4, 24).unwrap());
             }
});    }
    #[test]
    fn test_last_day_sunday_start() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        let week = date.week(Weekday::Sun);
        debug_assert_eq!(week.last_day(), NaiveDate::from_ymd_opt(2022, 4, 23).unwrap());
             }
});    }
    #[test]
    fn test_last_day_middle_of_week() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        let week = date.week(Weekday::Wed);
        debug_assert_eq!(week.last_day(), NaiveDate::from_ymd_opt(2022, 4, 26).unwrap());
             }
});    }
    #[test]
    fn test_last_day_edge_case_beginning() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd_opt(NaiveDate::MIN.year(), rug_fuzz_0, rug_fuzz_1)
            .unwrap();
        let week = date.week(Weekday::Mon);
        debug_assert_eq!(
            week.last_day(), NaiveDate::from_ymd_opt(NaiveDate::MIN.year(), 1, 1 + 6)
            .unwrap()
        );
             }
});    }
    #[test]
    fn test_last_day_edge_case_end() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd_opt(NaiveDate::MAX.year(), rug_fuzz_0, rug_fuzz_1)
            .unwrap();
        let week = date.week(Weekday::Mon);
        debug_assert_eq!(
            week.last_day(), NaiveDate::from_ymd_opt(NaiveDate::MAX.year(), 12, 31)
            .unwrap()
        );
             }
});    }
    #[test]
    #[should_panic]
    fn test_last_day_invalid_date() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        let week = date.week(Weekday::Mon);
        week.last_day();
             }
});    }
}
#[cfg(test)]
mod tests_rug_89 {
    use super::*;
    #[test]
    fn test_div_mod_floor() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: i32 = rug_fuzz_0;
        let p1: i32 = rug_fuzz_1;
        let result = crate::naive::date::div_mod_floor(p0, p1);
        debug_assert_eq!(result, (3, 1));
             }
});    }
}
#[cfg(test)]
mod tests_rug_91 {
    use crate::naive::date::NaiveDate;
    use crate::naive;
    #[test]
    fn test_diff_months() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u32, u32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let mut p1 = rug_fuzz_3;
        <naive::date::NaiveDate>::diff_months(p0, p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_92 {
    use crate::NaiveDate;
    use crate::naive::date::NaiveDate as Date;
    #[test]
    fn test_and_hms_milli() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let mut p1: u32 = rug_fuzz_3;
        let mut p2: u32 = rug_fuzz_4;
        let mut p3: u32 = rug_fuzz_5;
        let mut p4: u32 = rug_fuzz_6;
        Date::and_hms_milli(&p0, p1, p2, p3, p4);
             }
});    }
}
#[cfg(test)]
mod tests_rug_94 {
    use crate::NaiveDate;
    #[test]
    fn test_iter_weeks() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u32, u32, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0 = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let weeks = NaiveDate::iter_weeks(&p0).take(rug_fuzz_3).collect::<Vec<_>>();
        debug_assert_eq!(
            weeks, [NaiveDate::from_ymd(2023, 4, 5), NaiveDate::from_ymd(2023, 4, 12),
            NaiveDate::from_ymd(2023, 4, 19), NaiveDate::from_ymd(2023, 4, 26),]
        );
             }
});    }
}
#[cfg(test)]
mod tests_rug_95 {
    use crate::{Datelike, naive::date::NaiveDate};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u32, u32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: NaiveDate = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let mut p1: i32 = rug_fuzz_3;
        <NaiveDate as Datelike>::with_year(&p0, p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_96 {
    use super::*;
    use crate::naive::NaiveDate;
    use crate::Datelike;
    use std::iter::Iterator;
    #[test]
    fn test_size_hint() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u32, u32, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let start_date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let mut p0 = start_date.iter_days();
        let size_hint = p0.size_hint();
        debug_assert_eq!(size_hint.0, size_hint.1.unwrap());
        debug_assert!(size_hint.0 > rug_fuzz_3);
             }
});    }
}
