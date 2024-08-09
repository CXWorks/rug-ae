//! ISO 8601 calendar date with time zone.
#![allow(deprecated)]
#[cfg(any(feature = "alloc", feature = "std", test))]
use core::borrow::Borrow;
use core::cmp::Ordering;
use core::ops::{Add, AddAssign, Sub, SubAssign};
use core::{fmt, hash};
#[cfg(feature = "rkyv")]
use rkyv::{Archive, Deserialize, Serialize};
#[cfg(feature = "unstable-locales")]
use crate::format::Locale;
#[cfg(any(feature = "alloc", feature = "std", test))]
use crate::format::{DelayedFormat, Item, StrftimeItems};
use crate::naive::{IsoWeek, NaiveDate, NaiveTime};
use crate::offset::{TimeZone, Utc};
use crate::time_delta::TimeDelta;
use crate::DateTime;
use crate::{Datelike, Weekday};
/// ISO 8601 calendar date with time zone.
///
/// You almost certainly want to be using a [`NaiveDate`] instead of this type.
///
/// This type primarily exists to aid in the construction of DateTimes that
/// have a timezone by way of the [`TimeZone`] datelike constructors (e.g.
/// [`TimeZone::ymd`]).
///
/// This type should be considered ambiguous at best, due to the inherent lack
/// of precision required for the time zone resolution.
///
/// There are some guarantees on the usage of `Date<Tz>`:
///
/// - If properly constructed via [`TimeZone::ymd`] and others without an error,
///   the corresponding local date should exist for at least a moment.
///   (It may still have a gap from the offset changes.)
///
/// - The `TimeZone` is free to assign *any* [`Offset`](crate::offset::Offset) to the
///   local date, as long as that offset did occur in given day.
///
///   For example, if `2015-03-08T01:59-08:00` is followed by `2015-03-08T03:00-07:00`,
///   it may produce either `2015-03-08-08:00` or `2015-03-08-07:00`
///   but *not* `2015-03-08+00:00` and others.
///
/// - Once constructed as a full `DateTime`, [`DateTime::date`] and other associated
///   methods should return those for the original `Date`. For example, if `dt =
///   tz.ymd_opt(y,m,d).unwrap().hms(h,n,s)` were valid, `dt.date() == tz.ymd_opt(y,m,d).unwrap()`.
///
/// - The date is timezone-agnostic up to one day (i.e. practically always),
///   so the local date and UTC date should be equal for most cases
///   even though the raw calculation between `NaiveDate` and `Duration` may not.
#[deprecated(since = "0.4.23", note = "Use `NaiveDate` or `DateTime<Tz>` instead")]
#[derive(Clone)]
#[cfg_attr(feature = "rkyv", derive(Archive, Deserialize, Serialize))]
pub struct Date<Tz: TimeZone> {
    date: NaiveDate,
    offset: Tz::Offset,
}
/// The minimum possible `Date`.
#[allow(deprecated)]
#[deprecated(since = "0.4.20", note = "Use Date::MIN_UTC instead")]
pub const MIN_DATE: Date<Utc> = Date::<Utc>::MIN_UTC;
/// The maximum possible `Date`.
#[allow(deprecated)]
#[deprecated(since = "0.4.20", note = "Use Date::MAX_UTC instead")]
pub const MAX_DATE: Date<Utc> = Date::<Utc>::MAX_UTC;
impl<Tz: TimeZone> Date<Tz> {
    /// Makes a new `Date` with given *UTC* date and offset.
    /// The local date should be constructed via the `TimeZone` trait.
    #[inline]
    #[must_use]
    pub fn from_utc(date: NaiveDate, offset: Tz::Offset) -> Date<Tz> {
        Date { date, offset }
    }
    /// Makes a new `DateTime` from the current date and given `NaiveTime`.
    /// The offset in the current date is preserved.
    ///
    /// Panics on invalid datetime.
    #[inline]
    #[must_use]
    pub fn and_time(&self, time: NaiveTime) -> Option<DateTime<Tz>> {
        let localdt = self.naive_local().and_time(time);
        self.timezone().from_local_datetime(&localdt).single()
    }
    /// Makes a new `DateTime` from the current date, hour, minute and second.
    /// The offset in the current date is preserved.
    ///
    /// Panics on invalid hour, minute and/or second.
    #[deprecated(since = "0.4.23", note = "Use and_hms_opt() instead")]
    #[inline]
    #[must_use]
    pub fn and_hms(&self, hour: u32, min: u32, sec: u32) -> DateTime<Tz> {
        self.and_hms_opt(hour, min, sec).expect("invalid time")
    }
    /// Makes a new `DateTime` from the current date, hour, minute and second.
    /// The offset in the current date is preserved.
    ///
    /// Returns `None` on invalid hour, minute and/or second.
    #[inline]
    #[must_use]
    pub fn and_hms_opt(&self, hour: u32, min: u32, sec: u32) -> Option<DateTime<Tz>> {
        NaiveTime::from_hms_opt(hour, min, sec).and_then(|time| self.and_time(time))
    }
    /// Makes a new `DateTime` from the current date, hour, minute, second and millisecond.
    /// The millisecond part can exceed 1,000 in order to represent the leap second.
    /// The offset in the current date is preserved.
    ///
    /// Panics on invalid hour, minute, second and/or millisecond.
    #[deprecated(since = "0.4.23", note = "Use and_hms_milli_opt() instead")]
    #[inline]
    #[must_use]
    pub fn and_hms_milli(
        &self,
        hour: u32,
        min: u32,
        sec: u32,
        milli: u32,
    ) -> DateTime<Tz> {
        self.and_hms_milli_opt(hour, min, sec, milli).expect("invalid time")
    }
    /// Makes a new `DateTime` from the current date, hour, minute, second and millisecond.
    /// The millisecond part can exceed 1,000 in order to represent the leap second.
    /// The offset in the current date is preserved.
    ///
    /// Returns `None` on invalid hour, minute, second and/or millisecond.
    #[inline]
    #[must_use]
    pub fn and_hms_milli_opt(
        &self,
        hour: u32,
        min: u32,
        sec: u32,
        milli: u32,
    ) -> Option<DateTime<Tz>> {
        NaiveTime::from_hms_milli_opt(hour, min, sec, milli)
            .and_then(|time| self.and_time(time))
    }
    /// Makes a new `DateTime` from the current date, hour, minute, second and microsecond.
    /// The microsecond part can exceed 1,000,000 in order to represent the leap second.
    /// The offset in the current date is preserved.
    ///
    /// Panics on invalid hour, minute, second and/or microsecond.
    #[deprecated(since = "0.4.23", note = "Use and_hms_micro_opt() instead")]
    #[inline]
    #[must_use]
    pub fn and_hms_micro(
        &self,
        hour: u32,
        min: u32,
        sec: u32,
        micro: u32,
    ) -> DateTime<Tz> {
        self.and_hms_micro_opt(hour, min, sec, micro).expect("invalid time")
    }
    /// Makes a new `DateTime` from the current date, hour, minute, second and microsecond.
    /// The microsecond part can exceed 1,000,000 in order to represent the leap second.
    /// The offset in the current date is preserved.
    ///
    /// Returns `None` on invalid hour, minute, second and/or microsecond.
    #[inline]
    #[must_use]
    pub fn and_hms_micro_opt(
        &self,
        hour: u32,
        min: u32,
        sec: u32,
        micro: u32,
    ) -> Option<DateTime<Tz>> {
        NaiveTime::from_hms_micro_opt(hour, min, sec, micro)
            .and_then(|time| self.and_time(time))
    }
    /// Makes a new `DateTime` from the current date, hour, minute, second and nanosecond.
    /// The nanosecond part can exceed 1,000,000,000 in order to represent the leap second.
    /// The offset in the current date is preserved.
    ///
    /// Panics on invalid hour, minute, second and/or nanosecond.
    #[deprecated(since = "0.4.23", note = "Use and_hms_nano_opt() instead")]
    #[inline]
    #[must_use]
    pub fn and_hms_nano(
        &self,
        hour: u32,
        min: u32,
        sec: u32,
        nano: u32,
    ) -> DateTime<Tz> {
        self.and_hms_nano_opt(hour, min, sec, nano).expect("invalid time")
    }
    /// Makes a new `DateTime` from the current date, hour, minute, second and nanosecond.
    /// The nanosecond part can exceed 1,000,000,000 in order to represent the leap second.
    /// The offset in the current date is preserved.
    ///
    /// Returns `None` on invalid hour, minute, second and/or nanosecond.
    #[inline]
    #[must_use]
    pub fn and_hms_nano_opt(
        &self,
        hour: u32,
        min: u32,
        sec: u32,
        nano: u32,
    ) -> Option<DateTime<Tz>> {
        NaiveTime::from_hms_nano_opt(hour, min, sec, nano)
            .and_then(|time| self.and_time(time))
    }
    /// Makes a new `Date` for the next date.
    ///
    /// Panics when `self` is the last representable date.
    #[deprecated(since = "0.4.23", note = "Use succ_opt() instead")]
    #[inline]
    #[must_use]
    pub fn succ(&self) -> Date<Tz> {
        self.succ_opt().expect("out of bound")
    }
    /// Makes a new `Date` for the next date.
    ///
    /// Returns `None` when `self` is the last representable date.
    #[inline]
    #[must_use]
    pub fn succ_opt(&self) -> Option<Date<Tz>> {
        self.date.succ_opt().map(|date| Date::from_utc(date, self.offset.clone()))
    }
    /// Makes a new `Date` for the prior date.
    ///
    /// Panics when `self` is the first representable date.
    #[deprecated(since = "0.4.23", note = "Use pred_opt() instead")]
    #[inline]
    #[must_use]
    pub fn pred(&self) -> Date<Tz> {
        self.pred_opt().expect("out of bound")
    }
    /// Makes a new `Date` for the prior date.
    ///
    /// Returns `None` when `self` is the first representable date.
    #[inline]
    #[must_use]
    pub fn pred_opt(&self) -> Option<Date<Tz>> {
        self.date.pred_opt().map(|date| Date::from_utc(date, self.offset.clone()))
    }
    /// Retrieves an associated offset from UTC.
    #[inline]
    #[must_use]
    pub fn offset(&self) -> &Tz::Offset {
        &self.offset
    }
    /// Retrieves an associated time zone.
    #[inline]
    #[must_use]
    pub fn timezone(&self) -> Tz {
        TimeZone::from_offset(&self.offset)
    }
    /// Changes the associated time zone.
    /// This does not change the actual `Date` (but will change the string representation).
    #[inline]
    #[must_use]
    pub fn with_timezone<Tz2: TimeZone>(&self, tz: &Tz2) -> Date<Tz2> {
        tz.from_utc_date(&self.date)
    }
    /// Adds given `Duration` to the current date.
    ///
    /// Returns `None` when it will result in overflow.
    #[inline]
    #[must_use]
    pub fn checked_add_signed(self, rhs: TimeDelta) -> Option<Date<Tz>> {
        let date = self.date.checked_add_signed(rhs)?;
        Some(Date { date, offset: self.offset })
    }
    /// Subtracts given `Duration` from the current date.
    ///
    /// Returns `None` when it will result in overflow.
    #[inline]
    #[must_use]
    pub fn checked_sub_signed(self, rhs: TimeDelta) -> Option<Date<Tz>> {
        let date = self.date.checked_sub_signed(rhs)?;
        Some(Date { date, offset: self.offset })
    }
    /// Subtracts another `Date` from the current date.
    /// Returns a `Duration` of integral numbers.
    ///
    /// This does not overflow or underflow at all,
    /// as all possible output fits in the range of `Duration`.
    #[inline]
    #[must_use]
    pub fn signed_duration_since<Tz2: TimeZone>(self, rhs: Date<Tz2>) -> TimeDelta {
        self.date.signed_duration_since(rhs.date)
    }
    /// Returns a view to the naive UTC date.
    #[inline]
    #[must_use]
    pub fn naive_utc(&self) -> NaiveDate {
        self.date
    }
    /// Returns a view to the naive local date.
    ///
    /// This is technically the same as [`naive_utc`](#method.naive_utc)
    /// because the offset is restricted to never exceed one day,
    /// but provided for the consistency.
    #[inline]
    #[must_use]
    pub fn naive_local(&self) -> NaiveDate {
        self.date
    }
    /// Returns the number of whole years from the given `base` until `self`.
    #[must_use]
    pub fn years_since(&self, base: Self) -> Option<u32> {
        self.date.years_since(base.date)
    }
    /// The minimum possible `Date`.
    pub const MIN_UTC: Date<Utc> = Date {
        date: NaiveDate::MIN,
        offset: Utc,
    };
    /// The maximum possible `Date`.
    pub const MAX_UTC: Date<Utc> = Date {
        date: NaiveDate::MAX,
        offset: Utc,
    };
}
/// Maps the local date to other date with given conversion function.
fn map_local<Tz: TimeZone, F>(d: &Date<Tz>, mut f: F) -> Option<Date<Tz>>
where
    F: FnMut(NaiveDate) -> Option<NaiveDate>,
{
    f(d.naive_local()).and_then(|date| d.timezone().from_local_date(&date).single())
}
impl<Tz: TimeZone> Date<Tz>
where
    Tz::Offset: fmt::Display,
{
    /// Formats the date with the specified formatting items.
    #[cfg(any(feature = "alloc", feature = "std", test))]
    #[cfg_attr(docsrs, doc(cfg(any(feature = "alloc", feature = "std"))))]
    #[inline]
    #[must_use]
    pub fn format_with_items<'a, I, B>(&self, items: I) -> DelayedFormat<I>
    where
        I: Iterator<Item = B> + Clone,
        B: Borrow<Item<'a>>,
    {
        DelayedFormat::new_with_offset(
            Some(self.naive_local()),
            None,
            &self.offset,
            items,
        )
    }
    /// Formats the date with the specified format string.
    /// See the [`crate::format::strftime`] module
    /// on the supported escape sequences.
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
        DelayedFormat::new_with_offset_and_locale(
            Some(self.naive_local()),
            None,
            &self.offset,
            items,
            locale,
        )
    }
    /// Formats the date with the specified format string and locale.
    /// See the [`crate::format::strftime`] module
    /// on the supported escape sequences.
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
}
impl<Tz: TimeZone> Datelike for Date<Tz> {
    #[inline]
    fn year(&self) -> i32 {
        self.naive_local().year()
    }
    #[inline]
    fn month(&self) -> u32 {
        self.naive_local().month()
    }
    #[inline]
    fn month0(&self) -> u32 {
        self.naive_local().month0()
    }
    #[inline]
    fn day(&self) -> u32 {
        self.naive_local().day()
    }
    #[inline]
    fn day0(&self) -> u32 {
        self.naive_local().day0()
    }
    #[inline]
    fn ordinal(&self) -> u32 {
        self.naive_local().ordinal()
    }
    #[inline]
    fn ordinal0(&self) -> u32 {
        self.naive_local().ordinal0()
    }
    #[inline]
    fn weekday(&self) -> Weekday {
        self.naive_local().weekday()
    }
    #[inline]
    fn iso_week(&self) -> IsoWeek {
        self.naive_local().iso_week()
    }
    #[inline]
    fn with_year(&self, year: i32) -> Option<Date<Tz>> {
        map_local(self, |date| date.with_year(year))
    }
    #[inline]
    fn with_month(&self, month: u32) -> Option<Date<Tz>> {
        map_local(self, |date| date.with_month(month))
    }
    #[inline]
    fn with_month0(&self, month0: u32) -> Option<Date<Tz>> {
        map_local(self, |date| date.with_month0(month0))
    }
    #[inline]
    fn with_day(&self, day: u32) -> Option<Date<Tz>> {
        map_local(self, |date| date.with_day(day))
    }
    #[inline]
    fn with_day0(&self, day0: u32) -> Option<Date<Tz>> {
        map_local(self, |date| date.with_day0(day0))
    }
    #[inline]
    fn with_ordinal(&self, ordinal: u32) -> Option<Date<Tz>> {
        map_local(self, |date| date.with_ordinal(ordinal))
    }
    #[inline]
    fn with_ordinal0(&self, ordinal0: u32) -> Option<Date<Tz>> {
        map_local(self, |date| date.with_ordinal0(ordinal0))
    }
}
impl<Tz: TimeZone> Copy for Date<Tz>
where
    <Tz as TimeZone>::Offset: Copy,
{}
unsafe impl<Tz: TimeZone> Send for Date<Tz>
where
    <Tz as TimeZone>::Offset: Send,
{}
impl<Tz: TimeZone, Tz2: TimeZone> PartialEq<Date<Tz2>> for Date<Tz> {
    fn eq(&self, other: &Date<Tz2>) -> bool {
        self.date == other.date
    }
}
impl<Tz: TimeZone> Eq for Date<Tz> {}
impl<Tz: TimeZone> PartialOrd for Date<Tz> {
    fn partial_cmp(&self, other: &Date<Tz>) -> Option<Ordering> {
        self.date.partial_cmp(&other.date)
    }
}
impl<Tz: TimeZone> Ord for Date<Tz> {
    fn cmp(&self, other: &Date<Tz>) -> Ordering {
        self.date.cmp(&other.date)
    }
}
impl<Tz: TimeZone> hash::Hash for Date<Tz> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.date.hash(state)
    }
}
impl<Tz: TimeZone> Add<TimeDelta> for Date<Tz> {
    type Output = Date<Tz>;
    #[inline]
    fn add(self, rhs: TimeDelta) -> Date<Tz> {
        self.checked_add_signed(rhs).expect("`Date + Duration` overflowed")
    }
}
impl<Tz: TimeZone> AddAssign<TimeDelta> for Date<Tz> {
    #[inline]
    fn add_assign(&mut self, rhs: TimeDelta) {
        self
            .date = self
            .date
            .checked_add_signed(rhs)
            .expect("`Date + Duration` overflowed");
    }
}
impl<Tz: TimeZone> Sub<TimeDelta> for Date<Tz> {
    type Output = Date<Tz>;
    #[inline]
    fn sub(self, rhs: TimeDelta) -> Date<Tz> {
        self.checked_sub_signed(rhs).expect("`Date - Duration` overflowed")
    }
}
impl<Tz: TimeZone> SubAssign<TimeDelta> for Date<Tz> {
    #[inline]
    fn sub_assign(&mut self, rhs: TimeDelta) {
        self
            .date = self
            .date
            .checked_sub_signed(rhs)
            .expect("`Date - Duration` overflowed");
    }
}
impl<Tz: TimeZone> Sub<Date<Tz>> for Date<Tz> {
    type Output = TimeDelta;
    #[inline]
    fn sub(self, rhs: Date<Tz>) -> TimeDelta {
        self.signed_duration_since(rhs)
    }
}
impl<Tz: TimeZone> fmt::Debug for Date<Tz> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.naive_local().fmt(f)?;
        self.offset.fmt(f)
    }
}
impl<Tz: TimeZone> fmt::Display for Date<Tz>
where
    Tz::Offset: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.naive_local().fmt(f)?;
        self.offset.fmt(f)
    }
}
#[cfg(feature = "arbitrary")]
impl<'a, Tz> arbitrary::Arbitrary<'a> for Date<Tz>
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: arbitrary::Arbitrary<'a>,
{
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Date<Tz>> {
        let date = NaiveDate::arbitrary(u)?;
        let offset = <Tz as TimeZone>::Offset::arbitrary(u)?;
        Ok(Date::from_utc(date, offset))
    }
}
#[cfg(test)]
mod tests {
    use super::Date;
    use crate::time_delta::TimeDelta;
    use crate::{FixedOffset, NaiveDate, Utc};
    #[cfg(feature = "clock")]
    use crate::offset::{Local, TimeZone};
    #[test]
    #[cfg(feature = "clock")]
    fn test_years_elapsed() {
        const WEEKS_PER_YEAR: f32 = 52.1775;
        let one_year_ago = Utc::today()
            - TimeDelta::weeks((WEEKS_PER_YEAR * 1.5).ceil() as i64);
        let two_year_ago = Utc::today()
            - TimeDelta::weeks((WEEKS_PER_YEAR * 2.5).ceil() as i64);
        assert_eq!(Utc::today().years_since(one_year_ago), Some(1));
        assert_eq!(Utc::today().years_since(two_year_ago), Some(2));
        let future = Utc::today() + TimeDelta::weeks(12);
        assert_eq!(Utc::today().years_since(future), None);
    }
    #[test]
    fn test_date_add_assign() {
        let naivedate = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();
        let date = Date::<Utc>::from_utc(naivedate, Utc);
        let mut date_add = date;
        date_add += TimeDelta::days(5);
        assert_eq!(date_add, date + TimeDelta::days(5));
        let timezone = FixedOffset::east_opt(60 * 60).unwrap();
        let date = date.with_timezone(&timezone);
        let date_add = date_add.with_timezone(&timezone);
        assert_eq!(date_add, date + TimeDelta::days(5));
        let timezone = FixedOffset::west_opt(2 * 60 * 60).unwrap();
        let date = date.with_timezone(&timezone);
        let date_add = date_add.with_timezone(&timezone);
        assert_eq!(date_add, date + TimeDelta::days(5));
    }
    #[test]
    #[cfg(feature = "clock")]
    fn test_date_add_assign_local() {
        let naivedate = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();
        let date = Local.from_utc_date(&naivedate);
        let mut date_add = date;
        date_add += TimeDelta::days(5);
        assert_eq!(date_add, date + TimeDelta::days(5));
    }
    #[test]
    fn test_date_sub_assign() {
        let naivedate = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();
        let date = Date::<Utc>::from_utc(naivedate, Utc);
        let mut date_sub = date;
        date_sub -= TimeDelta::days(5);
        assert_eq!(date_sub, date - TimeDelta::days(5));
        let timezone = FixedOffset::east_opt(60 * 60).unwrap();
        let date = date.with_timezone(&timezone);
        let date_sub = date_sub.with_timezone(&timezone);
        assert_eq!(date_sub, date - TimeDelta::days(5));
        let timezone = FixedOffset::west_opt(2 * 60 * 60).unwrap();
        let date = date.with_timezone(&timezone);
        let date_sub = date_sub.with_timezone(&timezone);
        assert_eq!(date_sub, date - TimeDelta::days(5));
    }
    #[test]
    #[cfg(feature = "clock")]
    fn test_date_sub_assign_local() {
        let naivedate = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();
        let date = Local.from_utc_date(&naivedate);
        let mut date_sub = date;
        date_sub -= TimeDelta::days(5);
        assert_eq!(date_sub, date - TimeDelta::days(5));
    }
}
#[cfg(test)]
mod tests_llm_16_4 {
    use super::*;
    use crate::*;
    use crate::{Date, TimeZone, Utc, FixedOffset, NaiveDate};
    #[test]
    fn test_date_eq() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10)) = <(i32, i32, i32, u32, u32, i32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let fixed_offset_plus = FixedOffset::east(rug_fuzz_0);
        let fixed_offset_minus = FixedOffset::west(rug_fuzz_1);
        let utc_date = Utc.ymd(rug_fuzz_2, rug_fuzz_3, rug_fuzz_4);
        let fixed_plus_date = fixed_offset_plus.ymd(rug_fuzz_5, rug_fuzz_6, rug_fuzz_7);
        let fixed_minus_date = fixed_offset_minus
            .ymd(rug_fuzz_8, rug_fuzz_9, rug_fuzz_10);
        debug_assert!(
            utc_date.eq(& fixed_plus_date), "UTC and FixedOffset(+1) should be equal"
        );
        debug_assert!(
            utc_date.eq(& fixed_minus_date), "UTC and FixedOffset(-1) should be equal"
        );
             }
}
}
}    }
    #[test]
    fn test_date_ne() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, i32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let fixed_offset = FixedOffset::east(rug_fuzz_0);
        let utc_date = Utc.ymd(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        let fixed_date = fixed_offset.ymd(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        debug_assert!(
            ! utc_date.eq(& fixed_date), "Different dates should not be equal"
        );
             }
}
}
}    }
    #[test]
    fn test_date_eq_naive() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, i32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let fixed_offset = FixedOffset::east(rug_fuzz_0);
        let utc_date = Utc.ymd(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        let naive_date = NaiveDate::from_ymd(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        let fixed_naive_date = fixed_offset
            .ymd(naive_date.year(), naive_date.month(), naive_date.day());
        debug_assert!(
            utc_date.eq(& fixed_naive_date), "UTC Date should equal fixed naive date"
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_9 {
    use crate::{Date, TimeZone, NaiveDate, offset::FixedOffset, TimeDelta};
    use std::ops::Sub;
    #[test]
    fn test_sub() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8)) = <(i32, i32, i32, u32, u32, i64, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let fixed_offset = FixedOffset::east(rug_fuzz_0 * rug_fuzz_1);
        let date: Date<FixedOffset> = fixed_offset
            .ymd(rug_fuzz_2, rug_fuzz_3, rug_fuzz_4);
        let duration = TimeDelta::days(rug_fuzz_5);
        let expected: Date<FixedOffset> = fixed_offset
            .ymd(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8);
        let result = date.sub(duration);
        debug_assert_eq!(result, expected);
             }
}
}
}    }
    #[test]
    fn test_sub_with_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(i32, i32, u32, u32, i64, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let fixed_offset = FixedOffset::east(rug_fuzz_0);
        let date: Date<FixedOffset> = fixed_offset
            .ymd(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        let duration = TimeDelta::days(rug_fuzz_4);
        let expected: Date<FixedOffset> = fixed_offset
            .ymd(rug_fuzz_5, rug_fuzz_6, rug_fuzz_7);
        let result = date.sub(duration);
        debug_assert_eq!(result, expected);
             }
}
}
}    }
    #[test]
    fn test_sub_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8)) = <(i32, i32, i32, u32, u32, i64, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let fixed_offset = FixedOffset::west(rug_fuzz_0 * rug_fuzz_1);
        let date: Date<FixedOffset> = fixed_offset
            .ymd(rug_fuzz_2, rug_fuzz_3, rug_fuzz_4);
        let duration = TimeDelta::days(-rug_fuzz_5);
        let expected: Date<FixedOffset> = fixed_offset
            .ymd(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8);
        let result = date.sub(duration);
        debug_assert_eq!(result, expected);
             }
}
}
}    }
    #[test]
    #[should_panic(expected = "`Date - Duration` overflowed")]
    fn test_sub_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i32, i32, u32, u32, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let fixed_offset = FixedOffset::east(rug_fuzz_0);
        let date: Date<FixedOffset> = fixed_offset
            .ymd(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        let duration = TimeDelta::days(-rug_fuzz_4);
        date.sub(duration);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_16_llm_16_16 {
    use crate::{Date, FixedOffset, TimeZone, Utc};
    use crate::traits::Datelike;
    #[test]
    fn test_month0() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i32, i32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let fixed_offset = FixedOffset::east(rug_fuzz_0);
        let utc_dates = vec![
            Utc.ymd(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3), Utc.ymd(2023, 2, 15), Utc
            .ymd(2023, 3, 15), Utc.ymd(2023, 4, 15), Utc.ymd(2023, 5, 15), Utc.ymd(2023,
            6, 15), Utc.ymd(2023, 7, 15), Utc.ymd(2023, 8, 15), Utc.ymd(2023, 9, 15), Utc
            .ymd(2023, 10, 15), Utc.ymd(2023, 11, 15), Utc.ymd(2023, 12, 15)
        ];
        let fixed_dates: Vec<Date<FixedOffset>> = utc_dates
            .iter()
            .map(|&d| d.with_timezone(&fixed_offset))
            .collect();
        let expected_months: Vec<u32> = vec![
            rug_fuzz_4, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11
        ];
        for (fixed_date, &expected_month) in fixed_dates
            .iter()
            .zip(expected_months.iter())
        {
            debug_assert_eq!(
                fixed_date.month0(), expected_month, "Month0 should be {}, but was {}",
                expected_month, fixed_date.month0()
            );
        }
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_17 {
    use super::*;
    use crate::*;
    use crate::{Date, offset::Utc};
    #[test]
    fn test_ordinal() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(i32, u32, u32, i32, u32, u32, i32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date_utc: Date<Utc> = Utc.ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(date_utc.ordinal(), 1);
        let date_utc: Date<Utc> = Utc.ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        debug_assert_eq!(date_utc.ordinal(), 365);
        let date_utc: Date<Utc> = Utc.ymd(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8);
        debug_assert_eq!(date_utc.ordinal(), 60);
        let date_utc: Date<Utc> = Utc.ymd(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11);
        debug_assert_eq!(date_utc.ordinal(), 166);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_18 {
    use super::*;
    use crate::*;
    use crate::{Date, TimeZone, Utc};
    #[test]
    fn test_ordinal0() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(i32, u32, u32, i32, u32, u32, i32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date_utc: Date<Utc> = Utc.ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(date_utc.ordinal0(), 0);
        let date_utc: Date<Utc> = Utc.ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        debug_assert_eq!(date_utc.ordinal0(), 1);
        let date_utc: Date<Utc> = Utc.ymd(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8);
        debug_assert_eq!(date_utc.ordinal0(), 364);
        let date_utc: Date<Utc> = Utc.ymd(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11);
        debug_assert_eq!(date_utc.ordinal0(), 365);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_20 {
    use super::*;
    use crate::*;
    use crate::offset::TimeZone;
    use crate::offset::FixedOffset;
    #[test]
    fn test_with_day() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let tz = FixedOffset::east(rug_fuzz_0);
        let date = tz.ymd(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        let new_day = date.with_day(rug_fuzz_4);
        debug_assert_eq!(new_day, Some(tz.ymd(2023, 3, 15)));
        let new_day = date.with_day(rug_fuzz_5);
        debug_assert_eq!(new_day, None);
        let new_day = date.with_day(rug_fuzz_6);
        debug_assert_eq!(new_day, Some(tz.ymd(2023, 3, 28)));
             }
}
}
}    }
    #[test]
    fn test_with_day_edge_cases() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10)) = <(i32, i32, u32, u32, u32, i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let tz = FixedOffset::east(rug_fuzz_0);
        let date = tz.ymd(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        let new_day = date.with_day(rug_fuzz_4);
        debug_assert_eq!(new_day, Some(tz.ymd(2024, 2, 28)));
        let date = tz.ymd(rug_fuzz_5, rug_fuzz_6, rug_fuzz_7);
        let new_day = date.with_day(rug_fuzz_8);
        debug_assert_eq!(new_day, None);
        let date = tz.from_utc_date(&NaiveDate::MIN);
        let new_day = date.with_day(rug_fuzz_9);
        debug_assert_eq!(
            new_day, Some(tz.ymd(NaiveDate::MIN.year(), NaiveDate::MIN.month(), 2))
        );
        let date = tz.from_utc_date(&NaiveDate::MAX);
        let new_day = date.with_day(rug_fuzz_10);
        debug_assert_eq!(
            new_day, Some(tz.ymd(NaiveDate::MAX.year(), NaiveDate::MAX.month(), 1))
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_21 {
    use super::*;
    use crate::*;
    use crate::{FixedOffset, TimeZone};
    #[test]
    fn with_day0_valid_day() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i32, i32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let tz = FixedOffset::east(rug_fuzz_0);
        let date = tz.ymd(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        let result = date.with_day0(rug_fuzz_4);
        debug_assert_eq!(result, Some(tz.ymd(2023, 4, 1)));
             }
}
}
}    }
    #[test]
    fn with_day0_invalid_day() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i32, i32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let tz = FixedOffset::east(rug_fuzz_0);
        let date = tz.ymd(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        let result = date.with_day0(rug_fuzz_4);
        debug_assert_eq!(result, None);
             }
}
}
}    }
    #[test]
    fn with_day0_first_day_of_year() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i32, i32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let tz = FixedOffset::east(rug_fuzz_0);
        let date = tz.ymd(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        let result = date.with_day0(rug_fuzz_4);
        debug_assert_eq!(result, Some(tz.ymd(2023, 1, 1)));
             }
}
}
}    }
    #[test]
    fn with_day0_last_day_of_year() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i32, i32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let tz = FixedOffset::east(rug_fuzz_0);
        let date = tz.ymd(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        let result = date.with_day0(rug_fuzz_4);
        debug_assert_eq!(result, Some(tz.ymd(2023, 12, 31)));
             }
}
}
}    }
    #[test]
    fn with_day0_leap_year() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i32, i32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let tz = FixedOffset::east(rug_fuzz_0);
        let date = tz.ymd(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        let result = date.with_day0(rug_fuzz_4);
        debug_assert_eq!(result, Some(tz.ymd(2024, 2, 29)));
             }
}
}
}    }
    #[test]
    fn with_day0_non_leap_year() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i32, i32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let tz = FixedOffset::east(rug_fuzz_0);
        let date = tz.ymd(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        let result = date.with_day0(rug_fuzz_4);
        debug_assert_eq!(result, None);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_23 {
    use super::*;
    use crate::*;
    use crate::{Date, NaiveDate, TimeZone, Utc, Local, FixedOffset};
    use crate::offset::TimeZone as ChronoTimeZone;
    #[test]
    fn test_with_month0_valid() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = Utc.ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let new_month0 = rug_fuzz_3;
        let new_date = date.with_month0(new_month0);
        debug_assert_eq!(new_date, Some(Utc.ymd(2020, 5, 1)));
             }
}
}
}    }
    #[test]
    fn test_with_month0_invalid() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = Utc.ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let new_month0 = rug_fuzz_3;
        let new_date = date.with_month0(new_month0);
        debug_assert_eq!(new_date, None);
             }
}
}
}    }
    #[test]
    fn test_with_month0_edge_case() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = Utc.ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let new_month0 = rug_fuzz_3;
        let new_date = date.with_month0(new_month0);
        debug_assert_eq!(new_date, Some(Utc.ymd(2021, 12, 1)));
             }
}
}
}    }
    #[test]
    fn test_with_month0_using_fixed_offset() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, i32, i32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = FixedOffset::east(rug_fuzz_0 * rug_fuzz_1)
            .ymd(rug_fuzz_2, rug_fuzz_3, rug_fuzz_4);
        let new_month0 = rug_fuzz_5;
        let new_date = date.with_month0(new_month0);
        debug_assert_eq!(new_date, Some(FixedOffset::east(3600 * 9).ymd(2020, 5, 1)));
             }
}
}
}    }
    #[test]
    fn test_with_month0_using_local() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, u32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let local_date = Local.ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let new_month0 = rug_fuzz_3;
        let new_local_date = local_date.with_month0(new_month0);
        let expected_naive_date = NaiveDate::from_ymd(
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
        );
        debug_assert_eq!(
            new_local_date.map(| d | d.naive_local()), Some(expected_naive_date)
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_24 {
    use super::*;
    use crate::*;
    use crate::{Date, Datelike, Local, TimeZone};
    #[test]
    fn test_with_ordinal() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10)) = <(i32, u32, u32, u32, u32, u32, u32, i32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let tz = Local::now().timezone();
        let date = tz.ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        for ordinal in rug_fuzz_3..=rug_fuzz_4 {
            let with_ordinal = date.with_ordinal(ordinal);
            debug_assert!(with_ordinal.is_some(), "Ordinal {} should be valid", ordinal);
            debug_assert_eq!(
                with_ordinal.unwrap().ordinal(), ordinal, "Ordinal {} should match",
                ordinal
            );
        }
        debug_assert!(
            date.with_ordinal(rug_fuzz_5).is_none(), "Ordinal 0 should be invalid"
        );
        debug_assert!(
            date.with_ordinal(rug_fuzz_6).is_none(),
            "Ordinal 366 should be invalid for non-leap year"
        );
        let leap_date = tz.ymd(rug_fuzz_7, rug_fuzz_8, rug_fuzz_9);
        debug_assert!(
            leap_date.with_ordinal(rug_fuzz_10).is_some(),
            "Ordinal 366 should be valid for leap year"
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_25 {
    use super::*;
    use crate::*;
    use crate::prelude::*;
    use crate::Offset;
    #[test]
    fn test_with_ordinal0() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let tz = FixedOffset::east(rug_fuzz_0);
        let date = tz.ymd(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        let ordinal0 = rug_fuzz_4;
        let new_date = date.with_ordinal0(ordinal0);
        debug_assert!(new_date.is_some());
        debug_assert_eq!(new_date.unwrap(), tz.ymd(2020, 12, 31));
        let out_of_range_ordinal0 = rug_fuzz_5;
        debug_assert!(date.with_ordinal0(out_of_range_ordinal0).is_none());
        debug_assert!(date.with_ordinal0(rug_fuzz_6).is_none());
        debug_assert!(date.with_ordinal0(u32::MAX).is_none());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_26 {
    use crate::{Date, Datelike, TimeZone, Utc, FixedOffset};
    #[test]
    fn test_with_year_success() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u32, u32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let tz = Utc;
        let utc_date = tz.ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let new_date = utc_date.with_year(rug_fuzz_3);
        debug_assert_eq!(new_date.unwrap().year(), 2019);
             }
}
}
}    }
    #[test]
    fn test_with_year_success_leap_year() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u32, u32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let tz = Utc;
        let utc_date = tz.ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let new_date = utc_date.with_year(rug_fuzz_3);
        debug_assert!(new_date.is_none());
             }
}
}
}    }
    #[test]
    fn test_with_year_failure_out_of_range() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u32, u32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let tz = Utc;
        let utc_date = tz.ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let new_date = utc_date.with_year(rug_fuzz_3);
        debug_assert!(new_date.is_none());
             }
}
}
}    }
    #[test]
    fn test_with_year_fixed_offset() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, i32, i32, u32, u32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let tz = FixedOffset::east(rug_fuzz_0 * rug_fuzz_1);
        let fixed_offset_date = tz.ymd(rug_fuzz_2, rug_fuzz_3, rug_fuzz_4);
        let new_date = fixed_offset_date.with_year(rug_fuzz_5);
        debug_assert_eq!(new_date.unwrap().year(), 2019);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_210_llm_16_210 {
    use super::*;
    use crate::*;
    use crate::offset::{FixedOffset, Local, TimeZone, Utc, LocalResult};
    use crate::naive::NaiveDate;
    #[test]
    fn test_utc_and_hms() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = Utc.ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(
            date.and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5).timestamp(), 1678022788
        );
             }
}
}
}    }
    #[test]
    fn test_fixed_offset_east_and_hms() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(i32, i32, i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let eastern_offset = FixedOffset::east(rug_fuzz_0 * rug_fuzz_1);
        let date = eastern_offset.ymd(rug_fuzz_2, rug_fuzz_3, rug_fuzz_4);
        debug_assert_eq!(
            date.and_hms(rug_fuzz_5, rug_fuzz_6, rug_fuzz_7).timestamp(), 1690162500
        );
             }
}
}
}    }
    #[test]
    fn test_fixed_offset_west_and_hms() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(i32, i32, i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let western_offset = FixedOffset::west(rug_fuzz_0 * rug_fuzz_1);
        let date = western_offset.ymd(rug_fuzz_2, rug_fuzz_3, rug_fuzz_4);
        debug_assert_eq!(
            date.and_hms(rug_fuzz_5, rug_fuzz_6, rug_fuzz_7).timestamp(), 1683787200
        );
             }
}
}
}    }
    #[test]
    fn test_local_and_hms() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, u32, u32, u32, u32, u32, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = Local.ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let date_time = date.and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let local_result = Local.from_local_datetime(&date_time.naive_utc());
        match local_result {
            LocalResult::None => panic!("None result for local datetime"),
            LocalResult::Single(dt) => {
                debug_assert!(dt.timestamp() > rug_fuzz_6, "Invalid local datetime")
            }
            LocalResult::Ambiguous(_, _) => panic!("Ambiguous result for local datetime"),
        }
             }
}
}
}    }
    #[test]
    fn test_invalid_date() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(
            matches!(Utc.ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2), LocalResult::None)
        );
             }
}
}
}    }
    #[test]
    fn test_leap_second() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = Utc.ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        debug_assert!(
            date.and_hms_milli_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6)
            .is_none()
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_211 {
    use super::*;
    use crate::*;
    use crate::{DateTime, TimeZone, Utc, FixedOffset, Local, NaiveDate};
    #[test]
    fn test_and_hms_micro_valid_times() {
        let _rug_st_tests_llm_16_211_rrrruuuugggg_test_and_hms_micro_valid_times = 0;
        let rug_fuzz_0 = 2022;
        let rug_fuzz_1 = 4;
        let rug_fuzz_2 = 2;
        let rug_fuzz_3 = 12;
        let rug_fuzz_4 = 30;
        let rug_fuzz_5 = 45;
        let rug_fuzz_6 = 123456;
        let rug_fuzz_7 = 2022;
        let rug_fuzz_8 = 4;
        let rug_fuzz_9 = 2;
        let rug_fuzz_10 = 12;
        let rug_fuzz_11 = 30;
        let rug_fuzz_12 = 45;
        let rug_fuzz_13 = 123456;
        let rug_fuzz_14 = 3600;
        let rug_fuzz_15 = 2022;
        let rug_fuzz_16 = 4;
        let rug_fuzz_17 = 2;
        let rug_fuzz_18 = 12;
        let rug_fuzz_19 = 30;
        let rug_fuzz_20 = 45;
        let rug_fuzz_21 = 123456;
        let rug_fuzz_22 = 2022;
        let rug_fuzz_23 = 4;
        let rug_fuzz_24 = 2;
        let rug_fuzz_25 = 12;
        let rug_fuzz_26 = 30;
        let rug_fuzz_27 = 45;
        let rug_fuzz_28 = 123456;
        let rug_fuzz_29 = 2022;
        let rug_fuzz_30 = 4;
        let rug_fuzz_31 = 2;
        let rug_fuzz_32 = 12;
        let rug_fuzz_33 = 30;
        let rug_fuzz_34 = 45;
        let rug_fuzz_35 = 123456;
        let rug_fuzz_36 = 2022;
        let rug_fuzz_37 = 4;
        let rug_fuzz_38 = 2;
        let rug_fuzz_39 = 12;
        let rug_fuzz_40 = 30;
        let rug_fuzz_41 = 45;
        let rug_fuzz_42 = 123456;
        let date_utc: Date<Utc> = Utc.ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let datetime_utc: DateTime<Utc> = date_utc
            .and_hms_micro(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        let expected_utc: DateTime<Utc> = Utc
            .ymd(rug_fuzz_7, rug_fuzz_8, rug_fuzz_9)
            .and_hms_micro(rug_fuzz_10, rug_fuzz_11, rug_fuzz_12, rug_fuzz_13);
        debug_assert_eq!(datetime_utc, expected_utc);
        let offset = FixedOffset::east(rug_fuzz_14);
        let date_fixed: Date<FixedOffset> = offset
            .ymd(rug_fuzz_15, rug_fuzz_16, rug_fuzz_17);
        let datetime_fixed: DateTime<FixedOffset> = date_fixed
            .and_hms_micro(rug_fuzz_18, rug_fuzz_19, rug_fuzz_20, rug_fuzz_21);
        let expected_fixed: DateTime<FixedOffset> = offset
            .ymd(rug_fuzz_22, rug_fuzz_23, rug_fuzz_24)
            .and_hms_micro(rug_fuzz_25, rug_fuzz_26, rug_fuzz_27, rug_fuzz_28);
        debug_assert_eq!(datetime_fixed, expected_fixed);
        let date_local: Date<Local> = Local.ymd(rug_fuzz_29, rug_fuzz_30, rug_fuzz_31);
        let datetime_local: DateTime<Local> = date_local
            .and_hms_micro(rug_fuzz_32, rug_fuzz_33, rug_fuzz_34, rug_fuzz_35);
        let expected_local: DateTime<Local> = Local
            .ymd(rug_fuzz_36, rug_fuzz_37, rug_fuzz_38)
            .and_hms_micro(rug_fuzz_39, rug_fuzz_40, rug_fuzz_41, rug_fuzz_42);
        debug_assert_eq!(datetime_local, expected_local);
        let _rug_ed_tests_llm_16_211_rrrruuuugggg_test_and_hms_micro_valid_times = 0;
    }
    #[test]
    #[should_panic(expected = "invalid time")]
    fn test_and_hms_micro_invalid_time() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date: Date<Utc> = Utc.ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let _datetime: DateTime<Utc> = date
            .and_hms_micro(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
             }
}
}
}    }
    #[test]
    #[should_panic(expected = "invalid time")]
    fn test_and_hms_micro_invalid_microsecond() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date: Date<Utc> = Utc.ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let _datetime: DateTime<Utc> = date
            .and_hms_micro(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
             }
}
}
}    }
    #[test]
    fn test_and_hms_micro_leap_second() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(i32, u32, u32, u32, u32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date: Date<Utc> = Utc.ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let datetime: DateTime<Utc> = date
            .and_hms_micro(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        let expected: NaiveDate = NaiveDate::from_ymd(
            rug_fuzz_7,
            rug_fuzz_8,
            rug_fuzz_9,
        );
        debug_assert_eq!(datetime.naive_utc().date(), expected);
             }
}
}
}    }
    #[test]
    fn test_and_hms_micro_day_rollover() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(i32, u32, u32, u32, u32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date: Date<Utc> = Utc.ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let datetime: DateTime<Utc> = date
            .and_hms_micro(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        let expected: NaiveDate = NaiveDate::from_ymd(
            rug_fuzz_7,
            rug_fuzz_8,
            rug_fuzz_9,
        );
        debug_assert_eq!(datetime.naive_utc().date(), expected);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_214 {
    use crate::{DateTime, FixedOffset, Local, TimeZone, Utc};
    #[test]
    fn test_and_hms_milli_opt() {
        let _rug_st_tests_llm_16_214_rrrruuuugggg_test_and_hms_milli_opt = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 15;
        let rug_fuzz_3 = 23;
        let rug_fuzz_4 = 59;
        let rug_fuzz_5 = 59;
        let rug_fuzz_6 = 999;
        let rug_fuzz_7 = 24;
        let rug_fuzz_8 = 0;
        let rug_fuzz_9 = 0;
        let rug_fuzz_10 = 0;
        let rug_fuzz_11 = 23;
        let rug_fuzz_12 = 60;
        let rug_fuzz_13 = 0;
        let rug_fuzz_14 = 0;
        let rug_fuzz_15 = 23;
        let rug_fuzz_16 = 59;
        let rug_fuzz_17 = 60;
        let rug_fuzz_18 = 0;
        let rug_fuzz_19 = 23;
        let rug_fuzz_20 = 59;
        let rug_fuzz_21 = 59;
        let rug_fuzz_22 = 1000;
        let rug_fuzz_23 = 3600;
        let rug_fuzz_24 = 2023;
        let rug_fuzz_25 = 3;
        let rug_fuzz_26 = 15;
        let rug_fuzz_27 = 23;
        let rug_fuzz_28 = 59;
        let rug_fuzz_29 = 59;
        let rug_fuzz_30 = 999;
        let rug_fuzz_31 = 24;
        let rug_fuzz_32 = 0;
        let rug_fuzz_33 = 0;
        let rug_fuzz_34 = 0;
        let rug_fuzz_35 = 23;
        let rug_fuzz_36 = 60;
        let rug_fuzz_37 = 0;
        let rug_fuzz_38 = 0;
        let rug_fuzz_39 = 23;
        let rug_fuzz_40 = 59;
        let rug_fuzz_41 = 60;
        let rug_fuzz_42 = 0;
        let rug_fuzz_43 = 23;
        let rug_fuzz_44 = 59;
        let rug_fuzz_45 = 59;
        let rug_fuzz_46 = 1000;
        let rug_fuzz_47 = 2023;
        let rug_fuzz_48 = 3;
        let rug_fuzz_49 = 15;
        let rug_fuzz_50 = 23;
        let rug_fuzz_51 = 59;
        let rug_fuzz_52 = 59;
        let rug_fuzz_53 = 999;
        let rug_fuzz_54 = 24;
        let rug_fuzz_55 = 0;
        let rug_fuzz_56 = 0;
        let rug_fuzz_57 = 0;
        let rug_fuzz_58 = 23;
        let rug_fuzz_59 = 60;
        let rug_fuzz_60 = 0;
        let rug_fuzz_61 = 0;
        let rug_fuzz_62 = 23;
        let rug_fuzz_63 = 59;
        let rug_fuzz_64 = 60;
        let rug_fuzz_65 = 0;
        let rug_fuzz_66 = 23;
        let rug_fuzz_67 = 59;
        let rug_fuzz_68 = 59;
        let rug_fuzz_69 = 1000;
        let date_utc = Utc.ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        debug_assert!(
            date_utc.and_hms_milli_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6)
            .is_some()
        );
        debug_assert!(
            date_utc.and_hms_milli_opt(rug_fuzz_7, rug_fuzz_8, rug_fuzz_9, rug_fuzz_10)
            .is_none()
        );
        debug_assert!(
            date_utc.and_hms_milli_opt(rug_fuzz_11, rug_fuzz_12, rug_fuzz_13,
            rug_fuzz_14).is_none()
        );
        debug_assert!(
            date_utc.and_hms_milli_opt(rug_fuzz_15, rug_fuzz_16, rug_fuzz_17,
            rug_fuzz_18).is_none()
        );
        debug_assert!(
            date_utc.and_hms_milli_opt(rug_fuzz_19, rug_fuzz_20, rug_fuzz_21,
            rug_fuzz_22).is_none()
        );
        let date_fixed = FixedOffset::east(rug_fuzz_23)
            .ymd_opt(rug_fuzz_24, rug_fuzz_25, rug_fuzz_26)
            .unwrap();
        debug_assert!(
            date_fixed.and_hms_milli_opt(rug_fuzz_27, rug_fuzz_28, rug_fuzz_29,
            rug_fuzz_30).is_some()
        );
        debug_assert!(
            date_fixed.and_hms_milli_opt(rug_fuzz_31, rug_fuzz_32, rug_fuzz_33,
            rug_fuzz_34).is_none()
        );
        debug_assert!(
            date_fixed.and_hms_milli_opt(rug_fuzz_35, rug_fuzz_36, rug_fuzz_37,
            rug_fuzz_38).is_none()
        );
        debug_assert!(
            date_fixed.and_hms_milli_opt(rug_fuzz_39, rug_fuzz_40, rug_fuzz_41,
            rug_fuzz_42).is_none()
        );
        debug_assert!(
            date_fixed.and_hms_milli_opt(rug_fuzz_43, rug_fuzz_44, rug_fuzz_45,
            rug_fuzz_46).is_none()
        );
        let date_local = Local.ymd_opt(rug_fuzz_47, rug_fuzz_48, rug_fuzz_49).unwrap();
        debug_assert!(
            date_local.and_hms_milli_opt(rug_fuzz_50, rug_fuzz_51, rug_fuzz_52,
            rug_fuzz_53).is_some()
        );
        debug_assert!(
            date_local.and_hms_milli_opt(rug_fuzz_54, rug_fuzz_55, rug_fuzz_56,
            rug_fuzz_57).is_none()
        );
        debug_assert!(
            date_local.and_hms_milli_opt(rug_fuzz_58, rug_fuzz_59, rug_fuzz_60,
            rug_fuzz_61).is_none()
        );
        debug_assert!(
            date_local.and_hms_milli_opt(rug_fuzz_62, rug_fuzz_63, rug_fuzz_64,
            rug_fuzz_65).is_none()
        );
        debug_assert!(
            date_local.and_hms_milli_opt(rug_fuzz_66, rug_fuzz_67, rug_fuzz_68,
            rug_fuzz_69).is_none()
        );
        let _rug_ed_tests_llm_16_214_rrrruuuugggg_test_and_hms_milli_opt = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_215 {
    use super::*;
    use crate::*;
    use crate::{DateTime, TimeZone, Utc};
    #[test]
    fn test_and_hms_nano() {
        let _rug_st_tests_llm_16_215_rrrruuuugggg_test_and_hms_nano = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 5;
        let rug_fuzz_3 = 2023;
        let rug_fuzz_4 = 3;
        let rug_fuzz_5 = 5;
        let rug_fuzz_6 = 12;
        let rug_fuzz_7 = 30;
        let rug_fuzz_8 = 45;
        let rug_fuzz_9 = 1_000_000;
        let rug_fuzz_10 = 12;
        let rug_fuzz_11 = 30;
        let rug_fuzz_12 = 45;
        let rug_fuzz_13 = 1_000_000;
        let rug_fuzz_14 = 2023;
        let rug_fuzz_15 = 3;
        let rug_fuzz_16 = 5;
        let rug_fuzz_17 = 23;
        let rug_fuzz_18 = 59;
        let rug_fuzz_19 = 59;
        let rug_fuzz_20 = 1_999_999_999;
        let rug_fuzz_21 = 23;
        let rug_fuzz_22 = 59;
        let rug_fuzz_23 = 59;
        let rug_fuzz_24 = 1_999_999_999;
        let rug_fuzz_25 = 24;
        let rug_fuzz_26 = 30;
        let rug_fuzz_27 = 45;
        let rug_fuzz_28 = 1_000_000;
        let rug_fuzz_29 = 12;
        let rug_fuzz_30 = 60;
        let rug_fuzz_31 = 45;
        let rug_fuzz_32 = 1_000_000;
        let rug_fuzz_33 = 12;
        let rug_fuzz_34 = 30;
        let rug_fuzz_35 = 60;
        let rug_fuzz_36 = 1_000_000;
        let rug_fuzz_37 = 12;
        let rug_fuzz_38 = 30;
        let rug_fuzz_39 = 45;
        let rug_fuzz_40 = 2_000_000_000;
        let tz = Utc;
        let date = tz.ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let expected_normal = tz
            .ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .and_hms_nano_opt(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8, rug_fuzz_9)
            .unwrap();
        let result_normal = date
            .and_hms_nano(rug_fuzz_10, rug_fuzz_11, rug_fuzz_12, rug_fuzz_13);
        debug_assert_eq!(expected_normal, result_normal);
        let expected_end_of_day = tz
            .ymd(rug_fuzz_14, rug_fuzz_15, rug_fuzz_16)
            .and_hms_nano_opt(rug_fuzz_17, rug_fuzz_18, rug_fuzz_19, rug_fuzz_20)
            .unwrap();
        let result_end_of_day = date
            .and_hms_nano(rug_fuzz_21, rug_fuzz_22, rug_fuzz_23, rug_fuzz_24);
        debug_assert_eq!(expected_end_of_day, result_end_of_day);
        let result_invalid_hour = date
            .and_hms_nano_opt(rug_fuzz_25, rug_fuzz_26, rug_fuzz_27, rug_fuzz_28);
        debug_assert!(result_invalid_hour.is_none());
        let result_invalid_minute = date
            .and_hms_nano_opt(rug_fuzz_29, rug_fuzz_30, rug_fuzz_31, rug_fuzz_32);
        debug_assert!(result_invalid_minute.is_none());
        let result_invalid_second = date
            .and_hms_nano_opt(rug_fuzz_33, rug_fuzz_34, rug_fuzz_35, rug_fuzz_36);
        debug_assert!(result_invalid_second.is_none());
        let result_invalid_nano = date
            .and_hms_nano_opt(rug_fuzz_37, rug_fuzz_38, rug_fuzz_39, rug_fuzz_40);
        debug_assert!(result_invalid_nano.is_none());
        let _rug_ed_tests_llm_16_215_rrrruuuugggg_test_and_hms_nano = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_216 {
    use super::*;
    use crate::*;
    use crate::{FixedOffset, TimeZone, NaiveDate, NaiveTime};
    #[test]
    fn test_and_hms_nano_opt_valid() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(i32, i32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let timezone = FixedOffset::east_opt(rug_fuzz_0).unwrap();
        let date = NaiveDate::from_ymd_opt(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3).unwrap();
        let local_date = timezone.from_utc_date(&date);
        let opt_date_time = local_date
            .and_hms_nano_opt(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6, rug_fuzz_7);
        debug_assert!(opt_date_time.is_some());
        let date_time = opt_date_time.unwrap();
        debug_assert_eq!(
            date_time.time(), NaiveTime::from_hms_nano_opt(23, 59, 59, 1_000_000_000)
            .unwrap()
        );
             }
}
}
}    }
    #[test]
    fn test_and_hms_nano_opt_invalid_hour() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(i32, i32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let timezone = FixedOffset::east_opt(rug_fuzz_0).unwrap();
        let date = NaiveDate::from_ymd_opt(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3).unwrap();
        let local_date = timezone.from_utc_date(&date);
        debug_assert!(
            local_date.and_hms_nano_opt(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6, rug_fuzz_7)
            .is_none()
        );
             }
}
}
}    }
    #[test]
    fn test_and_hms_nano_opt_invalid_minute() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(i32, i32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let timezone = FixedOffset::east_opt(rug_fuzz_0).unwrap();
        let date = NaiveDate::from_ymd_opt(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3).unwrap();
        let local_date = timezone.from_utc_date(&date);
        debug_assert!(
            local_date.and_hms_nano_opt(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6, rug_fuzz_7)
            .is_none()
        );
             }
}
}
}    }
    #[test]
    fn test_and_hms_nano_opt_invalid_second() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(i32, i32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let timezone = FixedOffset::east_opt(rug_fuzz_0).unwrap();
        let date = NaiveDate::from_ymd_opt(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3).unwrap();
        let local_date = timezone.from_utc_date(&date);
        debug_assert!(
            local_date.and_hms_nano_opt(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6, rug_fuzz_7)
            .is_none()
        );
             }
}
}
}    }
    #[test]
    fn test_and_hms_nano_opt_invalid_nano() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(i32, i32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let timezone = FixedOffset::east_opt(rug_fuzz_0).unwrap();
        let date = NaiveDate::from_ymd_opt(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3).unwrap();
        let local_date = timezone.from_utc_date(&date);
        debug_assert!(
            local_date.and_hms_nano_opt(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6, rug_fuzz_7)
            .is_none()
        );
             }
}
}
}    }
    #[test]
    fn test_and_hms_nano_opt_leap_second() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(i32, i32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let timezone = FixedOffset::east_opt(rug_fuzz_0).unwrap();
        let date = NaiveDate::from_ymd_opt(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3).unwrap();
        let local_date = timezone.from_utc_date(&date);
        let opt_date_time = local_date
            .and_hms_nano_opt(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6, rug_fuzz_7);
        debug_assert!(opt_date_time.is_some());
        let date_time = opt_date_time.unwrap();
        debug_assert_eq!(
            date_time.time(), NaiveTime::from_hms_nano_opt(23, 59, 59, 1_000_000_001)
            .unwrap()
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_217 {
    use super::*;
    use crate::*;
    use crate::{Date, FixedOffset, Local, TimeZone, Utc};
    #[test]
    fn test_and_hms_opt_with_utc() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8)) = <(i32, u32, u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date_utc: Date<Utc> = Utc.ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(
            date_utc.and_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5), Some(Utc.ymd(2023,
            3, 14).and_hms(15, 0, 0))
        );
        debug_assert_eq!(date_utc.and_hms_opt(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8), None);
             }
}
}
}    }
    #[test]
    fn test_and_hms_opt_with_fixed_offset() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10)) = <(i32, i32, i32, u32, u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let fixed_offset = FixedOffset::east(rug_fuzz_0 * rug_fuzz_1);
        let date_fixed_offset: Date<FixedOffset> = fixed_offset
            .ymd(rug_fuzz_2, rug_fuzz_3, rug_fuzz_4);
        debug_assert_eq!(
            date_fixed_offset.and_hms_opt(rug_fuzz_5, rug_fuzz_6, rug_fuzz_7),
            Some(fixed_offset.ymd(2023, 3, 14).and_hms(15, 0, 0))
        );
        debug_assert_eq!(
            date_fixed_offset.and_hms_opt(rug_fuzz_8, rug_fuzz_9, rug_fuzz_10), None
        );
             }
}
}
}    }
    #[test]
    fn test_and_hms_opt_with_local() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8)) = <(i32, u32, u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date_local: Date<Local> = Local.ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        debug_assert!(
            date_local.and_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5).is_some()
        );
        debug_assert_eq!(
            date_local.and_hms_opt(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8), None
        );
             }
}
}
}    }
    #[test]
    fn test_and_hms_opt_with_leap_second() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date_utc: Date<Utc> = Utc.ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(date_utc.and_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5), None);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_219 {
    use super::*;
    use crate::*;
    use crate::offset::{TimeZone, FixedOffset, Utc};
    use crate::naive::NaiveDate;
    use crate::time_delta::TimeDelta;
    #[test]
    fn test_checked_add_signed_with_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = Utc.ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let duration = TimeDelta::max_value();
        debug_assert_eq!(date.checked_add_signed(duration), None);
             }
}
}
}    }
    #[test]
    fn test_checked_add_signed_without_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u32, u32, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = Utc.ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let duration = TimeDelta::days(rug_fuzz_3);
        debug_assert_eq!(
            date.checked_add_signed(duration), Utc.ymd_opt(2023, 1, 2).single()
        );
             }
}
}
}    }
    #[test]
    fn test_checked_add_signed_negative_without_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u32, u32, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = Utc.ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let duration = TimeDelta::days(-rug_fuzz_3);
        debug_assert_eq!(
            date.checked_add_signed(duration), Utc.ymd_opt(2023, 1, 1).single()
        );
             }
}
}
}    }
    #[test]
    fn test_checked_add_signed_with_dst_transition() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8)) = <(i32, i32, i32, u32, u32, u32, u32, u32, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let offset = FixedOffset::east(rug_fuzz_0 * rug_fuzz_1);
        let date = offset
            .ymd(rug_fuzz_2, rug_fuzz_3, rug_fuzz_4)
            .and_hms(rug_fuzz_5, rug_fuzz_6, rug_fuzz_7);
        let duration = TimeDelta::hours(rug_fuzz_8);
        debug_assert_eq!(
            date.checked_add_signed(duration), offset.ymd_opt(2023, 4, 1).and_hms_opt(2,
            30, 0).single()
        );
             }
}
}
}    }
    #[test]
    fn test_checked_add_signed_crossing_year_boundary() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u32, u32, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = Utc.ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let duration = TimeDelta::days(rug_fuzz_3);
        debug_assert_eq!(
            date.checked_add_signed(duration), Utc.ymd_opt(2024, 1, 1).single()
        );
             }
}
}
}    }
    #[test]
    fn test_checked_add_signed_crossing_century_boundary() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u32, u32, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = Utc.ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let duration = TimeDelta::days(rug_fuzz_3);
        debug_assert_eq!(
            date.checked_add_signed(duration), Utc.ymd_opt(2100, 1, 1).single()
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_221 {
    use crate::{Date, FixedOffset, TimeZone, Utc, NaiveDate};
    #[test]
    fn test_format_with_different_offsets() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8)) = <(i32, i32, i32, i32, i32, u32, u32, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let fixed_minus_5_hours = FixedOffset::west_opt(rug_fuzz_0 * rug_fuzz_1)
            .unwrap();
        let fixed_plus_3_hours = FixedOffset::east_opt(rug_fuzz_2 * rug_fuzz_3).unwrap();
        let date = Date::<
            Utc,
        >::from_utc(NaiveDate::from_ymd(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6), Utc)
            .with_timezone(&fixed_plus_3_hours);
        let formatted_minus_5 = date
            .with_timezone(&fixed_minus_5_hours)
            .format(rug_fuzz_7);
        let formatted_plus_3 = date
            .with_timezone(&fixed_plus_3_hours)
            .format(rug_fuzz_8);
        debug_assert_eq!(formatted_minus_5.to_string(), "2023-04-03 19:00:00 -0500");
        debug_assert_eq!(formatted_plus_3.to_string(), "2023-04-04 00:00:00 +0300");
             }
}
}
}    }
    #[test]
    fn test_format_with_different_dates_and_offsets() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10)) = <(i32, i32, i32, i32, u32, u32, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let fixed_plus_1_hour = FixedOffset::east_opt(rug_fuzz_0).unwrap();
        let fixed_plus_10_hours = FixedOffset::east_opt(rug_fuzz_1 * rug_fuzz_2)
            .unwrap();
        let dates_and_expected = vec![
            (NaiveDate::from_ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5), rug_fuzz_6),
            (NaiveDate::from_ymd(2024, 12, 31), "2024-12-31 01:00:00 +0100"),
            (NaiveDate::from_ymd(2025, 1, 1), "2025-01-01 01:00:00 +0100")
        ];
        for (naive_date, expected_str_plus_1_hour) in dates_and_expected {
            let date_plus_1_hour = Date::<Utc>::from_utc(naive_date, Utc)
                .with_timezone(&fixed_plus_1_hour);
            let date_plus_10_hours = Date::<Utc>::from_utc(naive_date, Utc)
                .with_timezone(&fixed_plus_10_hours);
            let formatted_plus_1_hour = date_plus_1_hour.format(rug_fuzz_7);
            let formatted_plus_10_hours = date_plus_10_hours
                .format(rug_fuzz_8)
                .to_string();
            let expected_str_plus_10_hours = expected_str_plus_1_hour
                .replace(rug_fuzz_9, rug_fuzz_10);
            debug_assert_eq!(
                formatted_plus_1_hour.to_string(), expected_str_plus_1_hour
            );
            debug_assert_eq!(formatted_plus_10_hours, expected_str_plus_10_hours);
        }
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_222 {
    use super::*;
    use crate::*;
    use crate::offset::FixedOffset;
    use crate::offset::TimeZone;
    #[test]
    fn test_format_with_items() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let tz = FixedOffset::east(rug_fuzz_0);
        let date = tz.ymd(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        let items = vec![
            format::Item::Fixed(format::Fixed::ShortWeekdayName),
            format::Item::Literal(" "), format::Item::Numeric(format::Numeric::Day,
            format::Pad::Zero), format::Item::Literal(" "),
            format::Item::Fixed(format::Fixed::ShortMonthName),
            format::Item::Literal(" "), format::Item::Numeric(format::Numeric::Year,
            format::Pad::Zero), format::Item::Literal(", "),
            format::Item::Fixed(format::Fixed::TimezoneName)
        ];
        let formatted = date.format_with_items(items.into_iter());
        debug_assert_eq!(formatted.to_string(), "Sun 30 Apr 2023, +0100");
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_224 {
    use crate::Date;
    use crate::{Datelike, NaiveDate, TimeZone, Utc, FixedOffset};
    #[test]
    fn test_naive_local_for_utc() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let naive_utc = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let utc_date = Utc.from_utc_date(&naive_utc);
        debug_assert_eq!(utc_date.naive_local(), naive_utc);
             }
}
}
}    }
    #[test]
    fn test_naive_local_for_fixed_offset() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u32, u32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let naive_utc = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let fixed_offset = FixedOffset::east(rug_fuzz_3);
        let date_with_offset = fixed_offset.from_utc_date(&naive_utc);
        debug_assert_eq!(date_with_offset.naive_local(), naive_utc);
             }
}
}
}    }
    #[test]
    fn test_naive_local_edge_cases() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let min_date = NaiveDate::from_ymd(-rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let max_date = NaiveDate::from_ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let utc_min_date = Utc.from_utc_date(&min_date);
        let utc_max_date = Utc.from_utc_date(&max_date);
        debug_assert_eq!(utc_min_date.naive_local(), min_date);
        debug_assert_eq!(utc_max_date.naive_local(), max_date);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_226 {
    use super::*;
    use crate::*;
    use crate::offset::TimeZone;
    #[test]
    fn test_fixed_offset_offset() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let offset = FixedOffset::east(rug_fuzz_0);
        let date = offset
            .ymd(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3)
            .and_hms(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        debug_assert_eq!(date.offset(), & offset);
             }
}
}
}    }
    #[test]
    #[should_panic(expected = "FixedOffset::east out of bounds")]
    fn test_fixed_offset_offset_panic() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        FixedOffset::east(rug_fuzz_0 * rug_fuzz_1);
             }
}
}
}    }
    #[test]
    fn test_fixed_offset_offset_opt() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let offset_opt = FixedOffset::east_opt(rug_fuzz_0);
        debug_assert!(offset_opt.is_some());
        let offset_none = FixedOffset::east_opt(rug_fuzz_1 * rug_fuzz_2);
        debug_assert!(offset_none.is_none());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_231 {
    use super::*;
    use crate::*;
    use crate::{offset::FixedOffset, TimeZone};
    #[test]
    fn test_succ_opt_with_fixed_offset() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, &str, i32, u32, u32, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let fixed_offset = FixedOffset::east_opt(rug_fuzz_0).expect(rug_fuzz_1);
        let date = fixed_offset.ymd_opt(rug_fuzz_2, rug_fuzz_3, rug_fuzz_4).unwrap();
        let next_date = date.succ_opt().expect(rug_fuzz_5);
        debug_assert_eq!(next_date, fixed_offset.ymd_opt(2023, 4, 1).unwrap());
             }
}
}
}    }
    #[test]
    fn test_succ_opt_with_date_before_dst() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, &str, i32, u32, u32, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let fixed_offset = FixedOffset::east_opt(rug_fuzz_0).expect(rug_fuzz_1);
        let date = fixed_offset.ymd_opt(rug_fuzz_2, rug_fuzz_3, rug_fuzz_4).unwrap();
        let next_date = date.succ_opt().expect(rug_fuzz_5);
        debug_assert_eq!(next_date, fixed_offset.ymd_opt(2023, 3, 27).unwrap());
             }
}
}
}    }
    #[test]
    fn test_succ_opt_with_date_after_dst() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, &str, i32, u32, u32, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let fixed_offset = FixedOffset::east_opt(rug_fuzz_0).expect(rug_fuzz_1);
        let date = fixed_offset.ymd_opt(rug_fuzz_2, rug_fuzz_3, rug_fuzz_4).unwrap();
        let next_date = date.succ_opt().expect(rug_fuzz_5);
        debug_assert_eq!(next_date, fixed_offset.ymd_opt(2023, 3, 29).unwrap());
             }
}
}
}    }
    #[test]
    fn test_succ_opt_with_last_representable_date() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i32, &str, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let fixed_offset = FixedOffset::east_opt(rug_fuzz_0).expect(rug_fuzz_1);
        let date = fixed_offset.ymd_opt(rug_fuzz_2, rug_fuzz_3, rug_fuzz_4).unwrap();
        let next_date = date.succ_opt();
        debug_assert!(next_date.is_none());
             }
}
}
}    }
    #[test]
    fn test_succ_opt_with_earliest_representable_date() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i32, &str, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let fixed_offset = FixedOffset::east_opt(rug_fuzz_0).expect(rug_fuzz_1);
        let date = fixed_offset.ymd_opt(-rug_fuzz_2, rug_fuzz_3, rug_fuzz_4).unwrap();
        let previous_date = date.pred_opt();
        debug_assert!(previous_date.is_none());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_2 {
    use crate::{Date, Utc, NaiveDate, TimeZone};
    #[test]
    fn test_map_local() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: Date<Utc> = Date::<
            Utc,
        >::from_utc(NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2), Utc);
        let mut p1 = |date: NaiveDate| Some(date);
        let result = crate::date::map_local(&mut p0, p1);
        debug_assert!(result.is_some());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_3 {
    use crate::{NaiveDate, FixedOffset, offset::TimeZone, Date};
    #[test]
    fn test_from_utc() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u32, u32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: NaiveDate = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let mut p1 = FixedOffset::east(rug_fuzz_3);
        Date::<FixedOffset>::from_utc(p0, p1);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_4 {
    use crate::prelude::*;
    use crate::naive;
    #[test]
    fn test_and_time() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Date::<
            Utc,
        >::from_utc(NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2), Utc);
        let mut p1 = NaiveTime::from_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let result = p0.and_time(p1);
        debug_assert!(result.is_some());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_5 {
    use super::*;
    use crate::prelude::*;
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Date::<
            Utc,
        >::from_utc(NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2), Utc);
        let mut p1: u32 = rug_fuzz_3;
        let mut p2: u32 = rug_fuzz_4;
        let mut p3: u32 = rug_fuzz_5;
        let mut p4: u32 = rug_fuzz_6;
        p0.and_hms_milli(p1, p2, p3, p4);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_6 {
    use super::*;
    use crate::prelude::*;
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8)) = <(i32, u32, u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Date::<
            Utc,
        >::from_utc(NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2), Utc);
        let mut p1: u32 = rug_fuzz_3;
        let mut p2: u32 = rug_fuzz_4;
        let mut p3: u32 = rug_fuzz_5;
        let mut p4: u32 = rug_fuzz_6;
        debug_assert!(p0.and_hms_micro_opt(p1, p2, p3, p4).is_some());
        let invalid_p1: u32 = rug_fuzz_7;
        debug_assert!(p0.and_hms_micro_opt(invalid_p1, p2, p3, p4).is_none());
        let leap_p4: u32 = rug_fuzz_8;
        debug_assert!(p0.and_hms_micro_opt(p1, p2, p3, leap_p4).is_some());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_7 {
    use crate::{Date, Utc, TimeZone, NaiveDate};
    #[test]
    fn test_succ() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: Date<Utc> = Utc.ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let _result = p0.succ();
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_8 {
    use crate::prelude::*;
    #[test]
    #[should_panic(expected = "out of bound")]
    fn test_pred() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Date::<
            Utc,
        >::from_utc(NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2), Utc);
        p0.pred();
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_9 {
    use crate::prelude::*;
    use crate::date::Date;
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0 = Date::<
            Utc,
        >::from_utc(NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2), Utc);
        let pred_date_option = Date::<Utc>::pred_opt(&p0);
        debug_assert!(pred_date_option.is_some());
        let pred_date = pred_date_option.unwrap();
        debug_assert_eq!(
            pred_date, Date:: < Utc > ::from_utc(NaiveDate::from_ymd(2023, 3, 31), Utc)
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_10 {
    use crate::prelude::*;
    #[test]
    fn test_timezone() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Date::<
            Utc,
        >::from_utc(NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2), Utc);
        let timezone = p0.timezone();
        debug_assert_eq!(timezone, Utc);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_11 {
    use crate::{Date, NaiveDate, TimeZone, Utc};
    #[test]
    fn test_with_timezone() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Date::<
            Utc,
        >::from_utc(NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2), Utc);
        let mut p1 = Utc;
        p0.with_timezone(&p1);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_12 {
    use crate::prelude::*;
    use crate::TimeDelta;
    use crate::date;
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u32, u32, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Date::<
            Utc,
        >::from_utc(NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2), Utc);
        let mut p1 = TimeDelta::seconds(rug_fuzz_3);
        debug_assert!(date::Date:: < Utc > ::checked_sub_signed(p0, p1).is_some());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_13 {
    use crate::prelude::*;
    #[test]
    fn test_signed_duration_since() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: Date<Utc> = Date::<
            Utc,
        >::from_utc(NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2), Utc);
        let mut p1: Date<Utc> = Date::<
            Utc,
        >::from_utc(NaiveDate::from_ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5), Utc);
        p0.signed_duration_since(p1);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_14 {
    use crate::prelude::*;
    #[test]
    fn test_naive_utc() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Date::<
            Utc,
        >::from_utc(NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2), Utc);
        let naive_utc_date = p0.naive_utc();
        debug_assert_eq!(naive_utc_date, NaiveDate::from_ymd(2023, 4, 1));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_15 {
    use crate::prelude::*;
    #[test]
    fn test_years_since() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Date::<
            Utc,
        >::from_utc(NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2), Utc);
        let mut p1 = Date::<
            Utc,
        >::from_utc(NaiveDate::from_ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5), Utc);
        debug_assert_eq!(p0.years_since(p1), Some(0));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_16 {
    use crate::prelude::*;
    use crate::Datelike;
    #[test]
    fn test_year() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Date::<
            Utc,
        >::from_utc(NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2), Utc);
        debug_assert_eq!(p0.year(), 2023);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_17 {
    use super::*;
    use crate::Datelike;
    use crate::prelude::*;
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: Date<Utc> = Date::<
            Utc,
        >::from_utc(NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2), Utc);
        debug_assert_eq!(< Date < Utc > as Datelike > ::month(& p0), 4);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_18 {
    use crate::{Date, Datelike, NaiveDate, Utc};
    #[test]
    fn test_day() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: Date<Utc> = Date::<
            Utc,
        >::from_utc(NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2), Utc);
        debug_assert_eq!(p0.day(), 1);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_19 {
    use crate::prelude::*;
    #[test]
    fn test_day0() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Date::<
            Utc,
        >::from_utc(NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2), Utc);
        debug_assert_eq!(p0.day0(), 0);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_20 {
    use crate::prelude::*;
    use crate::Date;
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: Date<Utc> = Date::<
            Utc,
        >::from_utc(NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2), Utc);
        let weekday = <Date<Utc> as Datelike>::weekday(&p0);
        debug_assert_eq!(weekday, Weekday::Sat);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_21 {
    use super::*;
    use crate::{Datelike, Date, Utc};
    use crate::naive::NaiveDate;
    use crate::offset::TimeZone;
    #[test]
    fn test_iso_week() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: Date<Utc> = Utc
            .from_utc_date(&NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2));
        let iso_week = <Date<Utc> as Datelike>::iso_week(&p0);
        debug_assert_eq!(iso_week.week(), 13);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_22 {
    use super::*;
    use crate::Datelike;
    use crate::{Date, Utc, TimeZone, NaiveDate};
    #[test]
    fn test_with_month() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0 = Date::<
            Utc,
        >::from_utc(NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2), Utc);
        let p1: u32 = rug_fuzz_3;
        debug_assert_eq!(
            p0.with_month(p1).unwrap(), Date:: < Utc >
            ::from_utc(NaiveDate::from_ymd(2023, 5, 1), Utc)
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_23 {
    use crate::prelude::*;
    use std::cmp::PartialOrd;
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Date::<
            Utc,
        >::from_utc(NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2), Utc);
        let mut p1 = Date::<
            Utc,
        >::from_utc(NaiveDate::from_ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5), Utc);
        debug_assert!(
            matches!(< Date < Utc > as PartialOrd > ::partial_cmp(& p0, & p1),
            Some(std::cmp::Ordering::Equal))
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_24 {
    use crate::prelude::*;
    use std::cmp::Ordering;
    #[test]
    fn test_cmp() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Date::<
            Utc,
        >::from_utc(NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2), Utc);
        let mut p1 = Date::<
            Utc,
        >::from_utc(NaiveDate::from_ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5), Utc);
        debug_assert_eq!(
            < Date < Utc > as std::cmp::Ord > ::cmp(& p0, & p1), Ordering::Less
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_25 {
    use super::*;
    use crate::prelude::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    #[test]
    fn test_hash() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Date::<
            Utc,
        >::from_utc(NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2), Utc);
        let mut p1 = DefaultHasher::new();
        <Date<Utc> as Hash>::hash(&p0, &mut p1);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_26 {
    use super::*;
    use crate::{Date, TimeDelta, Utc};
    use crate::prelude::*;
    use std::ops::Add;
    #[test]
    fn test_add() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u32, u32, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Date::<
            Utc,
        >::from_utc(NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2), Utc);
        let mut p1 = TimeDelta::seconds(rug_fuzz_3);
        p0.add(p1);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_27 {
    use crate::prelude::*;
    use crate::{Date, TimeDelta};
    use std::ops::AddAssign;
    #[test]
    fn test_add_assign() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u32, u32, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: Date<Utc> = Date::<
            Utc,
        >::from_utc(NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2), Utc);
        let p1: TimeDelta = TimeDelta::seconds(rug_fuzz_3);
        <Date<Utc> as AddAssign<TimeDelta>>::add_assign(&mut p0, p1);
        debug_assert_eq!(
            p0, Date:: < Utc > ::from_utc(NaiveDate::from_ymd(2023, 4, 1), Utc) +
            TimeDelta::seconds(10)
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_29 {
    use crate::prelude::*;
    use std::ops::Sub;
    #[test]
    fn test_sub() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Date::<
            Utc,
        >::from_utc(NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2), Utc);
        let mut p1 = Date::<
            Utc,
        >::from_utc(NaiveDate::from_ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5), Utc);
        <Date<Utc> as Sub>::sub(p0, p1);
             }
}
}
}    }
}
