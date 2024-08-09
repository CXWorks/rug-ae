//! The [`Date`] struct and its associated `impl`s.
use core::fmt;
use core::ops::{Add, Sub};
use core::time::Duration as StdDuration;
#[cfg(feature = "formatting")]
use std::io;
use crate::convert::*;
#[cfg(feature = "formatting")]
use crate::formatting::Formattable;
#[cfg(feature = "parsing")]
use crate::parsing::Parsable;
use crate::util::{days_in_year, days_in_year_month, is_leap_year, weeks_in_year};
use crate::{error, Duration, Month, PrimitiveDateTime, Time, Weekday};
/// The minimum valid year.
pub(crate) const MIN_YEAR: i32 = if cfg!(feature = "large-dates") {
    -999_999
} else {
    -9999
};
/// The maximum valid year.
pub(crate) const MAX_YEAR: i32 = if cfg!(feature = "large-dates") {
    999_999
} else {
    9999
};
/// Date in the proleptic Gregorian calendar.
///
/// By default, years between ±9999 inclusive are representable. This can be expanded to ±999,999
/// inclusive by enabling the `large-dates` crate feature. Doing so has performance implications
/// and introduces some ambiguities when parsing.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Date {
    /// Bitpacked field containing both the year and ordinal.
    value: i32,
}
impl Date {
    /// The minimum valid `Date`.
    ///
    /// The value of this may vary depending on the feature flags enabled.
    pub const MIN: Self = Self::__from_ordinal_date_unchecked(MIN_YEAR, 1);
    /// The maximum valid `Date`.
    ///
    /// The value of this may vary depending on the feature flags enabled.
    pub const MAX: Self = Self::__from_ordinal_date_unchecked(
        MAX_YEAR,
        days_in_year(MAX_YEAR),
    );
    /// Construct a `Date` from the year and ordinal values, the validity of which must be
    /// guaranteed by the caller.
    #[doc(hidden)]
    pub const fn __from_ordinal_date_unchecked(year: i32, ordinal: u16) -> Self {
        debug_assert!(year >= MIN_YEAR);
        debug_assert!(year <= MAX_YEAR);
        debug_assert!(ordinal != 0);
        debug_assert!(ordinal <= days_in_year(year));
        Self {
            value: (year << 9) | ordinal as i32,
        }
    }
    /// Attempt to create a `Date` from the year, month, and day.
    ///
    /// ```rust
    /// # use time::{Date, Month};
    /// assert!(Date::from_calendar_date(2019, Month::January, 1).is_ok());
    /// assert!(Date::from_calendar_date(2019, Month::December, 31).is_ok());
    /// ```
    ///
    /// ```rust
    /// # use time::{Date, Month};
    /// assert!(Date::from_calendar_date(2019, Month::February, 29).is_err()); // 2019 isn't a leap year.
    /// ```
    pub const fn from_calendar_date(
        year: i32,
        month: Month,
        day: u8,
    ) -> Result<Self, error::ComponentRange> {
        /// Cumulative days through the beginning of a month in both common and leap years.
        const DAYS_CUMULATIVE_COMMON_LEAP: [[u16; 12]; 2] = [
            [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334],
            [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335],
        ];
        ensure_value_in_range!(year in MIN_YEAR => MAX_YEAR);
        ensure_value_in_range!(
            day conditionally in 1 => days_in_year_month(year, month)
        );
        Ok(
            Self::__from_ordinal_date_unchecked(
                year,
                DAYS_CUMULATIVE_COMMON_LEAP[is_leap_year(year)
                    as usize][month as usize - 1] + day as u16,
            ),
        )
    }
    /// Attempt to create a `Date` from the year and ordinal day number.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert!(Date::from_ordinal_date(2019, 1).is_ok());
    /// assert!(Date::from_ordinal_date(2019, 365).is_ok());
    /// ```
    ///
    /// ```rust
    /// # use time::Date;
    /// assert!(Date::from_ordinal_date(2019, 366).is_err()); // 2019 isn't a leap year.
    /// ```
    pub const fn from_ordinal_date(
        year: i32,
        ordinal: u16,
    ) -> Result<Self, error::ComponentRange> {
        ensure_value_in_range!(year in MIN_YEAR => MAX_YEAR);
        ensure_value_in_range!(ordinal conditionally in 1 => days_in_year(year));
        Ok(Self::__from_ordinal_date_unchecked(year, ordinal))
    }
    /// Attempt to create a `Date` from the ISO year, week, and weekday.
    ///
    /// ```rust
    /// # use time::{Date, Weekday::*};
    /// assert!(Date::from_iso_week_date(2019, 1, Monday).is_ok());
    /// assert!(Date::from_iso_week_date(2019, 1, Tuesday).is_ok());
    /// assert!(Date::from_iso_week_date(2020, 53, Friday).is_ok());
    /// ```
    ///
    /// ```rust
    /// # use time::{Date, Weekday::*};
    /// assert!(Date::from_iso_week_date(2019, 53, Monday).is_err()); // 2019 doesn't have 53 weeks.
    /// ```
    pub const fn from_iso_week_date(
        year: i32,
        week: u8,
        weekday: Weekday,
    ) -> Result<Self, error::ComponentRange> {
        ensure_value_in_range!(year in MIN_YEAR => MAX_YEAR);
        ensure_value_in_range!(week conditionally in 1 => weeks_in_year(year));
        let adj_year = year - 1;
        let raw = 365 * adj_year + div_floor!(adj_year, 4) - div_floor!(adj_year, 100)
            + div_floor!(adj_year, 400);
        let jan_4 = match (raw % 7) as i8 {
            -6 | 1 => 8,
            -5 | 2 => 9,
            -4 | 3 => 10,
            -3 | 4 => 4,
            -2 | 5 => 5,
            -1 | 6 => 6,
            _ => 7,
        };
        let ordinal = week as i16 * 7 + weekday.number_from_monday() as i16 - jan_4;
        Ok(
            if ordinal <= 0 {
                Self::__from_ordinal_date_unchecked(
                    year - 1,
                    (ordinal as u16).wrapping_add(days_in_year(year - 1)),
                )
            } else if ordinal > days_in_year(year) as i16 {
                Self::__from_ordinal_date_unchecked(
                    year + 1,
                    ordinal as u16 - days_in_year(year),
                )
            } else {
                Self::__from_ordinal_date_unchecked(year, ordinal as _)
            },
        )
    }
    /// Create a `Date` from the Julian day.
    ///
    /// The algorithm to perform this conversion is derived from one provided by Peter Baum; it is
    /// freely available [here](https://www.researchgate.net/publication/316558298_Date_Algorithms).
    ///
    /// ```rust
    /// # use time::Date;
    /// # use time_macros::date;
    /// assert_eq!(Date::from_julian_day(0), Ok(date!(-4713 - 11 - 24)));
    /// assert_eq!(Date::from_julian_day(2_451_545), Ok(date!(2000 - 01 - 01)));
    /// assert_eq!(Date::from_julian_day(2_458_485), Ok(date!(2019 - 01 - 01)));
    /// assert_eq!(Date::from_julian_day(2_458_849), Ok(date!(2019 - 12 - 31)));
    /// ```
    #[doc(alias = "from_julian_date")]
    pub const fn from_julian_day(
        julian_day: i32,
    ) -> Result<Self, error::ComponentRange> {
        ensure_value_in_range!(
            julian_day in Self::MIN.to_julian_day() => Self::MAX.to_julian_day()
        );
        Ok(Self::from_julian_day_unchecked(julian_day))
    }
    /// Create a `Date` from the Julian day.
    ///
    /// This does not check the validity of the provided Julian day, and as such may result in an
    /// internally invalid value.
    #[doc(alias = "from_julian_date_unchecked")]
    pub(crate) const fn from_julian_day_unchecked(julian_day: i32) -> Self {
        debug_assert!(julian_day >= Self::MIN.to_julian_day());
        debug_assert!(julian_day <= Self::MAX.to_julian_day());
        let z = julian_day - 1_721_119;
        let (mut year, mut ordinal) = if julian_day < -19_752_948
            || julian_day > 23_195_514
        {
            let g = 100 * z as i64 - 25;
            let a = (g / 3_652_425) as i32;
            let b = a - a / 4;
            let year = div_floor!(100 * b as i64 + g, 36525) as i32;
            let ordinal = (b + z - div_floor!(36525 * year as i64, 100) as i32) as _;
            (year, ordinal)
        } else {
            let g = 100 * z - 25;
            let a = g / 3_652_425;
            let b = a - a / 4;
            let year = div_floor!(100 * b + g, 36525);
            let ordinal = (b + z - div_floor!(36525 * year, 100)) as _;
            (year, ordinal)
        };
        if is_leap_year(year) {
            ordinal += 60;
            cascade!(ordinal in 1..367 => year);
        } else {
            ordinal += 59;
            cascade!(ordinal in 1..366 => year);
        }
        Self::__from_ordinal_date_unchecked(year, ordinal)
    }
    /// Get the year of the date.
    ///
    /// ```rust
    /// # use time_macros::date;
    /// assert_eq!(date!(2019 - 01 - 01).year(), 2019);
    /// assert_eq!(date!(2019 - 12 - 31).year(), 2019);
    /// assert_eq!(date!(2020 - 01 - 01).year(), 2020);
    /// ```
    pub const fn year(self) -> i32 {
        self.value >> 9
    }
    /// Get the month.
    ///
    /// ```rust
    /// # use time::Month;
    /// # use time_macros::date;
    /// assert_eq!(date!(2019 - 01 - 01).month(), Month::January);
    /// assert_eq!(date!(2019 - 12 - 31).month(), Month::December);
    /// ```
    pub const fn month(self) -> Month {
        self.month_day().0
    }
    /// Get the day of the month.
    ///
    /// The returned value will always be in the range `1..=31`.
    ///
    /// ```rust
    /// # use time_macros::date;
    /// assert_eq!(date!(2019 - 01 - 01).day(), 1);
    /// assert_eq!(date!(2019 - 12 - 31).day(), 31);
    /// ```
    pub const fn day(self) -> u8 {
        self.month_day().1
    }
    /// Get the month and day. This is more efficient than fetching the components individually.
    pub(crate) const fn month_day(self) -> (Month, u8) {
        /// The number of days up to and including the given month. Common years
        /// are first, followed by leap years.
        const CUMULATIVE_DAYS_IN_MONTH_COMMON_LEAP: [[u16; 11]; 2] = [
            [31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334],
            [31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335],
        ];
        let days = CUMULATIVE_DAYS_IN_MONTH_COMMON_LEAP[is_leap_year(self.year())
            as usize];
        let ordinal = self.ordinal();
        if ordinal > days[10] {
            (Month::December, (ordinal - days[10]) as _)
        } else if ordinal > days[9] {
            (Month::November, (ordinal - days[9]) as _)
        } else if ordinal > days[8] {
            (Month::October, (ordinal - days[8]) as _)
        } else if ordinal > days[7] {
            (Month::September, (ordinal - days[7]) as _)
        } else if ordinal > days[6] {
            (Month::August, (ordinal - days[6]) as _)
        } else if ordinal > days[5] {
            (Month::July, (ordinal - days[5]) as _)
        } else if ordinal > days[4] {
            (Month::June, (ordinal - days[4]) as _)
        } else if ordinal > days[3] {
            (Month::May, (ordinal - days[3]) as _)
        } else if ordinal > days[2] {
            (Month::April, (ordinal - days[2]) as _)
        } else if ordinal > days[1] {
            (Month::March, (ordinal - days[1]) as _)
        } else if ordinal > days[0] {
            (Month::February, (ordinal - days[0]) as _)
        } else {
            (Month::January, ordinal as _)
        }
    }
    /// Get the day of the year.
    ///
    /// The returned value will always be in the range `1..=366` (`1..=365` for common years).
    ///
    /// ```rust
    /// # use time_macros::date;
    /// assert_eq!(date!(2019 - 01 - 01).ordinal(), 1);
    /// assert_eq!(date!(2019 - 12 - 31).ordinal(), 365);
    /// ```
    pub const fn ordinal(self) -> u16 {
        (self.value & 0x1FF) as _
    }
    /// Get the ISO 8601 year and week number.
    pub(crate) const fn iso_year_week(self) -> (i32, u8) {
        let (year, ordinal) = self.to_ordinal_date();
        match ((ordinal + 10 - self.weekday().number_from_monday() as u16) / 7) as _ {
            0 => (year - 1, weeks_in_year(year - 1)),
            53 if weeks_in_year(year) == 52 => (year + 1, 1),
            week => (year, week),
        }
    }
    /// Get the ISO week number.
    ///
    /// The returned value will always be in the range `1..=53`.
    ///
    /// ```rust
    /// # use time_macros::date;
    /// assert_eq!(date!(2019 - 01 - 01).iso_week(), 1);
    /// assert_eq!(date!(2019 - 10 - 04).iso_week(), 40);
    /// assert_eq!(date!(2020 - 01 - 01).iso_week(), 1);
    /// assert_eq!(date!(2020 - 12 - 31).iso_week(), 53);
    /// assert_eq!(date!(2021 - 01 - 01).iso_week(), 53);
    /// ```
    pub const fn iso_week(self) -> u8 {
        self.iso_year_week().1
    }
    /// Get the week number where week 1 begins on the first Sunday.
    ///
    /// The returned value will always be in the range `0..=53`.
    ///
    /// ```rust
    /// # use time_macros::date;
    /// assert_eq!(date!(2019 - 01 - 01).sunday_based_week(), 0);
    /// assert_eq!(date!(2020 - 01 - 01).sunday_based_week(), 0);
    /// assert_eq!(date!(2020 - 12 - 31).sunday_based_week(), 52);
    /// assert_eq!(date!(2021 - 01 - 01).sunday_based_week(), 0);
    /// ```
    pub const fn sunday_based_week(self) -> u8 {
        ((self.ordinal() as i16 - self.weekday().number_days_from_sunday() as i16 + 6)
            / 7) as _
    }
    /// Get the week number where week 1 begins on the first Monday.
    ///
    /// The returned value will always be in the range `0..=53`.
    ///
    /// ```rust
    /// # use time_macros::date;
    /// assert_eq!(date!(2019 - 01 - 01).monday_based_week(), 0);
    /// assert_eq!(date!(2020 - 01 - 01).monday_based_week(), 0);
    /// assert_eq!(date!(2020 - 12 - 31).monday_based_week(), 52);
    /// assert_eq!(date!(2021 - 01 - 01).monday_based_week(), 0);
    /// ```
    pub const fn monday_based_week(self) -> u8 {
        ((self.ordinal() as i16 - self.weekday().number_days_from_monday() as i16 + 6)
            / 7) as _
    }
    /// Get the year, month, and day.
    ///
    /// ```rust
    /// # use time::Month;
    /// # use time_macros::date;
    /// assert_eq!(
    ///     date!(2019 - 01 - 01).to_calendar_date(),
    ///     (2019, Month::January, 1)
    /// );
    /// ```
    pub const fn to_calendar_date(self) -> (i32, Month, u8) {
        let (month, day) = self.month_day();
        (self.year(), month, day)
    }
    /// Get the year and ordinal day number.
    ///
    /// ```rust
    /// # use time_macros::date;
    /// assert_eq!(date!(2019 - 01 - 01).to_ordinal_date(), (2019, 1));
    /// ```
    pub const fn to_ordinal_date(self) -> (i32, u16) {
        (self.year(), self.ordinal())
    }
    /// Get the ISO 8601 year, week number, and weekday.
    ///
    /// ```rust
    /// # use time::Weekday::*;
    /// # use time_macros::date;
    /// assert_eq!(date!(2019 - 01 - 01).to_iso_week_date(), (2019, 1, Tuesday));
    /// assert_eq!(date!(2019 - 10 - 04).to_iso_week_date(), (2019, 40, Friday));
    /// assert_eq!(
    ///     date!(2020 - 01 - 01).to_iso_week_date(),
    ///     (2020, 1, Wednesday)
    /// );
    /// assert_eq!(
    ///     date!(2020 - 12 - 31).to_iso_week_date(),
    ///     (2020, 53, Thursday)
    /// );
    /// assert_eq!(date!(2021 - 01 - 01).to_iso_week_date(), (2020, 53, Friday));
    /// ```
    pub const fn to_iso_week_date(self) -> (i32, u8, Weekday) {
        let (year, ordinal) = self.to_ordinal_date();
        let weekday = self.weekday();
        match ((ordinal + 10 - self.weekday().number_from_monday() as u16) / 7) as _ {
            0 => (year - 1, weeks_in_year(year - 1), weekday),
            53 if weeks_in_year(year) == 52 => (year + 1, 1, weekday),
            week => (year, week, weekday),
        }
    }
    /// Get the weekday.
    ///
    /// ```rust
    /// # use time::Weekday::*;
    /// # use time_macros::date;
    /// assert_eq!(date!(2019 - 01 - 01).weekday(), Tuesday);
    /// assert_eq!(date!(2019 - 02 - 01).weekday(), Friday);
    /// assert_eq!(date!(2019 - 03 - 01).weekday(), Friday);
    /// assert_eq!(date!(2019 - 04 - 01).weekday(), Monday);
    /// assert_eq!(date!(2019 - 05 - 01).weekday(), Wednesday);
    /// assert_eq!(date!(2019 - 06 - 01).weekday(), Saturday);
    /// assert_eq!(date!(2019 - 07 - 01).weekday(), Monday);
    /// assert_eq!(date!(2019 - 08 - 01).weekday(), Thursday);
    /// assert_eq!(date!(2019 - 09 - 01).weekday(), Sunday);
    /// assert_eq!(date!(2019 - 10 - 01).weekday(), Tuesday);
    /// assert_eq!(date!(2019 - 11 - 01).weekday(), Friday);
    /// assert_eq!(date!(2019 - 12 - 01).weekday(), Sunday);
    /// ```
    pub const fn weekday(self) -> Weekday {
        match self.to_julian_day() % 7 {
            -6 | 1 => Weekday::Tuesday,
            -5 | 2 => Weekday::Wednesday,
            -4 | 3 => Weekday::Thursday,
            -3 | 4 => Weekday::Friday,
            -2 | 5 => Weekday::Saturday,
            -1 | 6 => Weekday::Sunday,
            val => {
                debug_assert!(val == 0);
                Weekday::Monday
            }
        }
    }
    /// Get the next calendar date.
    ///
    /// ```rust
    /// # use time::Date;
    /// # use time_macros::date;
    /// assert_eq!(
    ///     date!(2019 - 01 - 01).next_day(),
    ///     Some(date!(2019 - 01 - 02))
    /// );
    /// assert_eq!(
    ///     date!(2019 - 01 - 31).next_day(),
    ///     Some(date!(2019 - 02 - 01))
    /// );
    /// assert_eq!(
    ///     date!(2019 - 12 - 31).next_day(),
    ///     Some(date!(2020 - 01 - 01))
    /// );
    /// assert_eq!(Date::MAX.next_day(), None);
    /// ```
    pub const fn next_day(self) -> Option<Self> {
        if self.ordinal() == 366 || (self.ordinal() == 365 && !is_leap_year(self.year()))
        {
            if self.value == Self::MAX.value {
                None
            } else {
                Some(Self::__from_ordinal_date_unchecked(self.year() + 1, 1))
            }
        } else {
            Some(Self { value: self.value + 1 })
        }
    }
    /// Get the previous calendar date.
    ///
    /// ```rust
    /// # use time::Date;
    /// # use time_macros::date;
    /// assert_eq!(
    ///     date!(2019 - 01 - 02).previous_day(),
    ///     Some(date!(2019 - 01 - 01))
    /// );
    /// assert_eq!(
    ///     date!(2019 - 02 - 01).previous_day(),
    ///     Some(date!(2019 - 01 - 31))
    /// );
    /// assert_eq!(
    ///     date!(2020 - 01 - 01).previous_day(),
    ///     Some(date!(2019 - 12 - 31))
    /// );
    /// assert_eq!(Date::MIN.previous_day(), None);
    /// ```
    pub const fn previous_day(self) -> Option<Self> {
        if self.ordinal() != 1 {
            Some(Self { value: self.value - 1 })
        } else if self.value == Self::MIN.value {
            None
        } else {
            Some(
                Self::__from_ordinal_date_unchecked(
                    self.year() - 1,
                    days_in_year(self.year() - 1),
                ),
            )
        }
    }
    /// Get the Julian day for the date.
    ///
    /// The algorithm to perform this conversion is derived from one provided by Peter Baum; it is
    /// freely available [here](https://www.researchgate.net/publication/316558298_Date_Algorithms).
    ///
    /// ```rust
    /// # use time_macros::date;
    /// assert_eq!(date!(-4713 - 11 - 24).to_julian_day(), 0);
    /// assert_eq!(date!(2000 - 01 - 01).to_julian_day(), 2_451_545);
    /// assert_eq!(date!(2019 - 01 - 01).to_julian_day(), 2_458_485);
    /// assert_eq!(date!(2019 - 12 - 31).to_julian_day(), 2_458_849);
    /// ```
    pub const fn to_julian_day(self) -> i32 {
        let year = self.year() - 1;
        let ordinal = self.ordinal() as i32;
        ordinal + 365 * year + div_floor!(year, 4) - div_floor!(year, 100)
            + div_floor!(year, 400) + 1_721_425
    }
    /// Computes `self + duration`, returning `None` if an overflow occurred.
    ///
    /// ```rust
    /// # use time::{Date, ext::NumericalDuration};
    /// # use time_macros::date;
    /// assert_eq!(Date::MAX.checked_add(1.days()), None);
    /// assert_eq!(Date::MIN.checked_add((-2).days()), None);
    /// assert_eq!(
    ///     date!(2020 - 12 - 31).checked_add(2.days()),
    ///     Some(date!(2021 - 01 - 02))
    /// );
    /// ```
    ///
    /// # Note
    ///
    /// This function only takes whole days into account.
    ///
    /// ```rust
    /// # use time::{Date, ext::NumericalDuration};
    /// # use time_macros::date;
    /// assert_eq!(Date::MAX.checked_add(23.hours()), Some(Date::MAX));
    /// assert_eq!(Date::MIN.checked_add((-23).hours()), Some(Date::MIN));
    /// assert_eq!(
    ///     date!(2020 - 12 - 31).checked_add(23.hours()),
    ///     Some(date!(2020 - 12 - 31))
    /// );
    /// assert_eq!(
    ///     date!(2020 - 12 - 31).checked_add(47.hours()),
    ///     Some(date!(2021 - 01 - 01))
    /// );
    /// ```
    pub const fn checked_add(self, duration: Duration) -> Option<Self> {
        let whole_days = duration.whole_days();
        if whole_days < i32::MIN as i64 || whole_days > i32::MAX as i64 {
            return None;
        }
        let julian_day = const_try_opt!(
            self.to_julian_day().checked_add(whole_days as _)
        );
        if let Ok(date) = Self::from_julian_day(julian_day) { Some(date) } else { None }
    }
    /// Computes `self - duration`, returning `None` if an overflow occurred.
    ///
    /// ```
    /// # use time::{Date, ext::NumericalDuration};
    /// # use time_macros::date;
    /// assert_eq!(Date::MAX.checked_sub((-2).days()), None);
    /// assert_eq!(Date::MIN.checked_sub(1.days()), None);
    /// assert_eq!(
    ///     date!(2020 - 12 - 31).checked_sub(2.days()),
    ///     Some(date!(2020 - 12 - 29))
    /// );
    /// ```
    ///
    /// # Note
    ///
    /// This function only takes whole days into account.
    ///
    /// ```
    /// # use time::{Date, ext::NumericalDuration};
    /// # use time_macros::date;
    /// assert_eq!(Date::MAX.checked_sub((-23).hours()), Some(Date::MAX));
    /// assert_eq!(Date::MIN.checked_sub(23.hours()), Some(Date::MIN));
    /// assert_eq!(
    ///     date!(2020 - 12 - 31).checked_sub(23.hours()),
    ///     Some(date!(2020 - 12 - 31))
    /// );
    /// assert_eq!(
    ///     date!(2020 - 12 - 31).checked_sub(47.hours()),
    ///     Some(date!(2020 - 12 - 30))
    /// );
    /// ```
    pub const fn checked_sub(self, duration: Duration) -> Option<Self> {
        let whole_days = duration.whole_days();
        if whole_days < i32::MIN as i64 || whole_days > i32::MAX as i64 {
            return None;
        }
        let julian_day = const_try_opt!(
            self.to_julian_day().checked_sub(whole_days as _)
        );
        if let Ok(date) = Self::from_julian_day(julian_day) { Some(date) } else { None }
    }
    /// Computes `self + duration`, saturating value on overflow.
    ///
    /// ```rust
    /// # use time::{Date, ext::NumericalDuration};
    /// # use time_macros::date;
    /// assert_eq!(Date::MAX.saturating_add(1.days()), Date::MAX);
    /// assert_eq!(Date::MIN.saturating_add((-2).days()), Date::MIN);
    /// assert_eq!(
    ///     date!(2020 - 12 - 31).saturating_add(2.days()),
    ///     date!(2021 - 01 - 02)
    /// );
    /// ```
    ///
    /// # Note
    ///
    /// This function only takes whole days into account.
    ///
    /// ```rust
    /// # use time::ext::NumericalDuration;
    /// # use time_macros::date;
    /// assert_eq!(
    ///     date!(2020 - 12 - 31).saturating_add(23.hours()),
    ///     date!(2020 - 12 - 31)
    /// );
    /// assert_eq!(
    ///     date!(2020 - 12 - 31).saturating_add(47.hours()),
    ///     date!(2021 - 01 - 01)
    /// );
    /// ```
    pub const fn saturating_add(self, duration: Duration) -> Self {
        if let Some(datetime) = self.checked_add(duration) {
            datetime
        } else if duration.is_negative() {
            Self::MIN
        } else {
            debug_assert!(duration.is_positive());
            Self::MAX
        }
    }
    /// Computes `self - duration`, saturating value on overflow.
    ///
    /// ```
    /// # use time::{Date, ext::NumericalDuration};
    /// # use time_macros::date;
    /// assert_eq!(Date::MAX.saturating_sub((-2).days()), Date::MAX);
    /// assert_eq!(Date::MIN.saturating_sub(1.days()), Date::MIN);
    /// assert_eq!(
    ///     date!(2020 - 12 - 31).saturating_sub(2.days()),
    ///     date!(2020 - 12 - 29)
    /// );
    /// ```
    ///
    /// # Note
    ///
    /// This function only takes whole days into account.
    ///
    /// ```
    /// # use time::ext::NumericalDuration;
    /// # use time_macros::date;
    /// assert_eq!(
    ///     date!(2020 - 12 - 31).saturating_sub(23.hours()),
    ///     date!(2020 - 12 - 31)
    /// );
    /// assert_eq!(
    ///     date!(2020 - 12 - 31).saturating_sub(47.hours()),
    ///     date!(2020 - 12 - 30)
    /// );
    /// ```
    pub const fn saturating_sub(self, duration: Duration) -> Self {
        if let Some(datetime) = self.checked_sub(duration) {
            datetime
        } else if duration.is_negative() {
            Self::MAX
        } else {
            debug_assert!(duration.is_positive());
            Self::MIN
        }
    }
    /// Replace the year. The month and day will be unchanged.
    ///
    /// ```rust
    /// # use time_macros::date;
    /// assert_eq!(
    ///     date!(2022 - 02 - 18).replace_year(2019),
    ///     Ok(date!(2019 - 02 - 18))
    /// );
    /// assert!(date!(2022 - 02 - 18).replace_year(-1_000_000_000).is_err()); // -1_000_000_000 isn't a valid year
    /// assert!(date!(2022 - 02 - 18).replace_year(1_000_000_000).is_err()); // 1_000_000_000 isn't a valid year
    /// ```
    #[must_use = "This method does not mutate the original `Date`."]
    pub const fn replace_year(self, year: i32) -> Result<Self, error::ComponentRange> {
        ensure_value_in_range!(year in MIN_YEAR => MAX_YEAR);
        let ordinal = self.ordinal();
        if ordinal <= 59 {
            return Ok(Self::__from_ordinal_date_unchecked(year, ordinal));
        }
        match (is_leap_year(self.year()), is_leap_year(year)) {
            (false, false) | (true, true) => {
                Ok(Self::__from_ordinal_date_unchecked(year, ordinal))
            }
            (true, false) if ordinal == 60 => {
                Err(error::ComponentRange {
                    name: "day",
                    value: 29,
                    minimum: 1,
                    maximum: 28,
                    conditional_range: true,
                })
            }
            (false, true) => Ok(Self::__from_ordinal_date_unchecked(year, ordinal + 1)),
            (true, false) => Ok(Self::__from_ordinal_date_unchecked(year, ordinal - 1)),
        }
    }
    /// Replace the month of the year.
    ///
    /// ```rust
    /// # use time_macros::date;
    /// # use time::Month;
    /// assert_eq!(
    ///     date!(2022 - 02 - 18).replace_month(Month::January),
    ///     Ok(date!(2022 - 01 - 18))
    /// );
    /// assert!(
    ///     date!(2022 - 01 - 30)
    ///         .replace_month(Month::February)
    ///         .is_err()
    /// ); // 30 isn't a valid day in February
    /// ```
    #[must_use = "This method does not mutate the original `Date`."]
    pub const fn replace_month(
        self,
        month: Month,
    ) -> Result<Self, error::ComponentRange> {
        let (year, _, day) = self.to_calendar_date();
        Self::from_calendar_date(year, month, day)
    }
    /// Replace the day of the month.
    ///
    /// ```rust
    /// # use time_macros::date;
    /// assert_eq!(
    ///     date!(2022 - 02 - 18).replace_day(1),
    ///     Ok(date!(2022 - 02 - 01))
    /// );
    /// assert!(date!(2022 - 02 - 18).replace_day(0).is_err()); // 0 isn't a valid day
    /// assert!(date!(2022 - 02 - 18).replace_day(30).is_err()); // 30 isn't a valid day in February
    /// ```
    #[must_use = "This method does not mutate the original `Date`."]
    pub const fn replace_day(self, day: u8) -> Result<Self, error::ComponentRange> {
        if day == 0 || day >= 29 {
            ensure_value_in_range!(
                day conditionally in 1 => days_in_year_month(self.year(), self.month())
            );
        }
        Ok(
            Self::__from_ordinal_date_unchecked(
                self.year(),
                (self.ordinal() as i16 - self.day() as i16 + day as i16) as _,
            ),
        )
    }
}
/// Methods to add a [`Time`] component, resulting in a [`PrimitiveDateTime`].
impl Date {
    /// Create a [`PrimitiveDateTime`] using the existing date. The [`Time`] component will be set
    /// to midnight.
    ///
    /// ```rust
    /// # use time_macros::{date, datetime};
    /// assert_eq!(date!(1970-01-01).midnight(), datetime!(1970-01-01 0:00));
    /// ```
    pub const fn midnight(self) -> PrimitiveDateTime {
        PrimitiveDateTime::new(self, Time::MIDNIGHT)
    }
    /// Create a [`PrimitiveDateTime`] using the existing date and the provided [`Time`].
    ///
    /// ```rust
    /// # use time_macros::{date, datetime, time};
    /// assert_eq!(
    ///     date!(1970-01-01).with_time(time!(0:00)),
    ///     datetime!(1970-01-01 0:00),
    /// );
    /// ```
    pub const fn with_time(self, time: Time) -> PrimitiveDateTime {
        PrimitiveDateTime::new(self, time)
    }
    /// Attempt to create a [`PrimitiveDateTime`] using the existing date and the provided time.
    ///
    /// ```rust
    /// # use time_macros::date;
    /// assert!(date!(1970 - 01 - 01).with_hms(0, 0, 0).is_ok());
    /// assert!(date!(1970 - 01 - 01).with_hms(24, 0, 0).is_err());
    /// ```
    pub const fn with_hms(
        self,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> Result<PrimitiveDateTime, error::ComponentRange> {
        Ok(
            PrimitiveDateTime::new(
                self,
                const_try!(Time::from_hms(hour, minute, second)),
            ),
        )
    }
    /// Attempt to create a [`PrimitiveDateTime`] using the existing date and the provided time.
    ///
    /// ```rust
    /// # use time_macros::date;
    /// assert!(date!(1970 - 01 - 01).with_hms_milli(0, 0, 0, 0).is_ok());
    /// assert!(date!(1970 - 01 - 01).with_hms_milli(24, 0, 0, 0).is_err());
    /// ```
    pub const fn with_hms_milli(
        self,
        hour: u8,
        minute: u8,
        second: u8,
        millisecond: u16,
    ) -> Result<PrimitiveDateTime, error::ComponentRange> {
        Ok(
            PrimitiveDateTime::new(
                self,
                const_try!(Time::from_hms_milli(hour, minute, second, millisecond)),
            ),
        )
    }
    /// Attempt to create a [`PrimitiveDateTime`] using the existing date and the provided time.
    ///
    /// ```rust
    /// # use time_macros::date;
    /// assert!(date!(1970 - 01 - 01).with_hms_micro(0, 0, 0, 0).is_ok());
    /// assert!(date!(1970 - 01 - 01).with_hms_micro(24, 0, 0, 0).is_err());
    /// ```
    pub const fn with_hms_micro(
        self,
        hour: u8,
        minute: u8,
        second: u8,
        microsecond: u32,
    ) -> Result<PrimitiveDateTime, error::ComponentRange> {
        Ok(
            PrimitiveDateTime::new(
                self,
                const_try!(Time::from_hms_micro(hour, minute, second, microsecond)),
            ),
        )
    }
    /// Attempt to create a [`PrimitiveDateTime`] using the existing date and the provided time.
    ///
    /// ```rust
    /// # use time_macros::date;
    /// assert!(date!(1970 - 01 - 01).with_hms_nano(0, 0, 0, 0).is_ok());
    /// assert!(date!(1970 - 01 - 01).with_hms_nano(24, 0, 0, 0).is_err());
    /// ```
    pub const fn with_hms_nano(
        self,
        hour: u8,
        minute: u8,
        second: u8,
        nanosecond: u32,
    ) -> Result<PrimitiveDateTime, error::ComponentRange> {
        Ok(
            PrimitiveDateTime::new(
                self,
                const_try!(Time::from_hms_nano(hour, minute, second, nanosecond)),
            ),
        )
    }
}
#[cfg(feature = "formatting")]
impl Date {
    /// Format the `Date` using the provided [format description](crate::format_description).
    pub fn format_into(
        self,
        output: &mut impl io::Write,
        format: &(impl Formattable + ?Sized),
    ) -> Result<usize, error::Format> {
        format.format_into(output, Some(self), None, None)
    }
    /// Format the `Date` using the provided [format description](crate::format_description).
    ///
    /// ```rust
    /// # use time::{format_description};
    /// # use time_macros::date;
    /// let format = format_description::parse("[year]-[month]-[day]")?;
    /// assert_eq!(date!(2020 - 01 - 02).format(&format)?, "2020-01-02");
    /// # Ok::<_, time::Error>(())
    /// ```
    pub fn format(
        self,
        format: &(impl Formattable + ?Sized),
    ) -> Result<String, error::Format> {
        format.format(Some(self), None, None)
    }
}
#[cfg(feature = "parsing")]
impl Date {
    /// Parse a `Date` from the input using the provided [format
    /// description](crate::format_description).
    ///
    /// ```rust
    /// # use time::Date;
    /// # use time_macros::{date, format_description};
    /// let format = format_description!("[year]-[month]-[day]");
    /// assert_eq!(Date::parse("2020-01-02", &format)?, date!(2020 - 01 - 02));
    /// # Ok::<_, time::Error>(())
    /// ```
    pub fn parse(
        input: &str,
        description: &(impl Parsable + ?Sized),
    ) -> Result<Self, error::Parse> {
        description.parse_date(input.as_bytes())
    }
}
impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if cfg!(feature = "large-dates") && self.year().abs() >= 10_000 {
            write!(f, "{:+}-{:02}-{:02}", self.year(), self.month() as u8, self.day())
        } else {
            write!(
                f, "{:0width$}-{:02}-{:02}", self.year(), self.month() as u8, self.day(),
                width = 4 + (self.year() < 0) as usize
            )
        }
    }
}
impl fmt::Debug for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        fmt::Display::fmt(self, f)
    }
}
impl Add<Duration> for Date {
    type Output = Self;
    fn add(self, duration: Duration) -> Self::Output {
        self.checked_add(duration).expect("overflow adding duration to date")
    }
}
impl Add<StdDuration> for Date {
    type Output = Self;
    fn add(self, duration: StdDuration) -> Self::Output {
        Self::from_julian_day(
                self.to_julian_day()
                    + (duration.as_secs() / Second.per(Day) as u64) as i32,
            )
            .expect("overflow adding duration to date")
    }
}
impl_add_assign!(Date : Duration, StdDuration);
impl Sub<Duration> for Date {
    type Output = Self;
    fn sub(self, duration: Duration) -> Self::Output {
        self.checked_sub(duration).expect("overflow subtracting duration from date")
    }
}
impl Sub<StdDuration> for Date {
    type Output = Self;
    fn sub(self, duration: StdDuration) -> Self::Output {
        Self::from_julian_day(
                self.to_julian_day()
                    - (duration.as_secs() / Second.per(Day) as u64) as i32,
            )
            .expect("overflow subtracting duration from date")
    }
}
impl_sub_assign!(Date : Duration, StdDuration);
impl Sub for Date {
    type Output = Duration;
    fn sub(self, other: Self) -> Self::Output {
        Duration::days((self.to_julian_day() - other.to_julian_day()) as _)
    }
}
#[cfg(test)]
mod tests_llm_16_1_llm_16_1 {
    use crate::{Date, Duration};
    use crate::ext::NumericalDuration;
    use std::ops::Add;
    #[test]
    fn add_duration_to_date() {
        let _rug_st_tests_llm_16_1_llm_16_1_rrrruuuugggg_add_duration_to_date = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 15;
        let rug_fuzz_2 = "Failed to construct Date";
        let rug_fuzz_3 = 300;
        let rug_fuzz_4 = 2024;
        let rug_fuzz_5 = 9;
        let rug_fuzz_6 = "Failed to construct expected Date";
        let rug_fuzz_7 = 300;
        let rug_fuzz_8 = 2022;
        let rug_fuzz_9 = 21;
        let rug_fuzz_10 = "Failed to construct expected Date";
        let rug_fuzz_11 = 1;
        let rug_fuzz_12 = 1;
        let rug_fuzz_13 = 2023;
        let rug_fuzz_14 = 28;
        let rug_fuzz_15 = "Failed to construct Date";
        let rug_fuzz_16 = 1;
        let rug_fuzz_17 = 2023;
        let rug_fuzz_18 = 1;
        let rug_fuzz_19 = "Failed to construct expected Date";
        let rug_fuzz_20 = 2023;
        let rug_fuzz_21 = 28;
        let rug_fuzz_22 = "Failed to construct Date";
        let rug_fuzz_23 = 2;
        let rug_fuzz_24 = 2023;
        let rug_fuzz_25 = 2;
        let rug_fuzz_26 = "Failed to construct expected Date";
        let rug_fuzz_27 = 2023;
        let rug_fuzz_28 = 1;
        let rug_fuzz_29 = "Failed to construct Date";
        let rug_fuzz_30 = 1;
        let rug_fuzz_31 = 2023;
        let rug_fuzz_32 = 28;
        let rug_fuzz_33 = "Failed to construct expected Date";
        let rug_fuzz_34 = 2023;
        let rug_fuzz_35 = 1;
        let rug_fuzz_36 = "Failed to construct Date";
        let rug_fuzz_37 = 0;
        let date = Date::from_calendar_date(rug_fuzz_0, crate::Month::April, rug_fuzz_1)
            .expect(rug_fuzz_2);
        let duration = rug_fuzz_3.days();
        let expected = Date::from_calendar_date(
                rug_fuzz_4,
                crate::Month::February,
                rug_fuzz_5,
            )
            .expect(rug_fuzz_6);
        debug_assert_eq!(date.add(duration), expected);
        debug_assert_eq!(date.add(Duration::ZERO), date);
        let duration = (-rug_fuzz_7).days();
        let expected = Date::from_calendar_date(
                rug_fuzz_8,
                crate::Month::June,
                rug_fuzz_9,
            )
            .expect(rug_fuzz_10);
        debug_assert_eq!(date.add(duration), expected);
        let date = Date::MAX;
        let duration = rug_fuzz_11.days();
        let result = std::panic::catch_unwind(|| date.add(duration));
        debug_assert!(result.is_err());
        let date = Date::MIN;
        let duration = (-rug_fuzz_12).days();
        let result = std::panic::catch_unwind(|| date.add(duration));
        debug_assert!(result.is_err());
        let date = Date::from_calendar_date(
                rug_fuzz_13,
                crate::Month::February,
                rug_fuzz_14,
            )
            .expect(rug_fuzz_15);
        let duration = rug_fuzz_16.days();
        let expected = Date::from_calendar_date(
                rug_fuzz_17,
                crate::Month::March,
                rug_fuzz_18,
            )
            .expect(rug_fuzz_19);
        debug_assert_eq!(date.add(duration), expected);
        let date = Date::from_calendar_date(
                rug_fuzz_20,
                crate::Month::February,
                rug_fuzz_21,
            )
            .expect(rug_fuzz_22);
        let duration = rug_fuzz_23.days();
        let expected = Date::from_calendar_date(
                rug_fuzz_24,
                crate::Month::March,
                rug_fuzz_25,
            )
            .expect(rug_fuzz_26);
        debug_assert_eq!(date.add(duration), expected);
        let date = Date::from_calendar_date(
                rug_fuzz_27,
                crate::Month::March,
                rug_fuzz_28,
            )
            .expect(rug_fuzz_29);
        let duration = (-rug_fuzz_30).days();
        let expected = Date::from_calendar_date(
                rug_fuzz_31,
                crate::Month::February,
                rug_fuzz_32,
            )
            .expect(rug_fuzz_33);
        debug_assert_eq!(date.add(duration), expected);
        let date = Date::from_calendar_date(
                rug_fuzz_34,
                crate::Month::March,
                rug_fuzz_35,
            )
            .expect(rug_fuzz_36);
        let duration = Duration::new(i64::MAX, rug_fuzz_37);
        let result = std::panic::catch_unwind(|| date.add(duration));
        debug_assert!(result.is_err());
        let _rug_ed_tests_llm_16_1_llm_16_1_rrrruuuugggg_add_duration_to_date = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_5 {
    use super::*;
    use crate::*;
    use crate::ext::NumericalDuration;
    use crate::ext::NumericalStdDuration;
    #[test]
    fn test_sub_positive_duration() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i32, u16, i64, i32, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = Date::from_ordinal_date(rug_fuzz_0, rug_fuzz_1).unwrap();
        let duration = rug_fuzz_2.days();
        let expected = Date::from_ordinal_date(rug_fuzz_3, rug_fuzz_4).unwrap();
        debug_assert_eq!(date - duration, expected);
             }
});    }
    #[test]
    fn test_sub_negative_duration() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i32, u16, i64, i32, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = Date::from_ordinal_date(rug_fuzz_0, rug_fuzz_1).unwrap();
        let duration = (-rug_fuzz_2).days();
        let expected = Date::from_ordinal_date(rug_fuzz_3, rug_fuzz_4).unwrap();
        debug_assert_eq!(date - duration, expected);
             }
});    }
    #[test]
    #[should_panic(expected = "overflow subtracting duration from date")]
    fn test_sub_overflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = Date::MIN;
        let duration = rug_fuzz_0.days();
        let _ = date - duration;
             }
});    }
    #[test]
    fn test_sub_std_duration() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i32, u16, u64, i32, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = Date::from_ordinal_date(rug_fuzz_0, rug_fuzz_1).unwrap();
        let duration = rug_fuzz_2.std_hours();
        let expected = Date::from_ordinal_date(rug_fuzz_3, rug_fuzz_4).unwrap();
        debug_assert_eq!(date - duration, expected);
             }
});    }
    #[test]
    #[should_panic(expected = "overflow subtracting duration from date")]
    fn test_sub_std_duration_overflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = Date::MIN;
        let duration = rug_fuzz_0.std_hours();
        let _ = date - duration;
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_7 {
    use super::*;
    use crate::*;
    use crate::Duration;
    #[test]
    fn test_sub_same_date() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date1 = Date::from_calendar_date(rug_fuzz_0, Month::December, rug_fuzz_1)
            .unwrap();
        let date2 = date1;
        debug_assert_eq!(date1 - date2, Duration::ZERO);
             }
});    }
    #[test]
    fn test_sub_different_date() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u8, i32, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date1 = Date::from_calendar_date(rug_fuzz_0, Month::December, rug_fuzz_1)
            .unwrap();
        let date2 = Date::from_calendar_date(rug_fuzz_2, Month::January, rug_fuzz_3)
            .unwrap();
        debug_assert_eq!(date1 - date2, Duration::days(364));
             }
});    }
    #[test]
    fn test_sub_across_years() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u8, i32, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date1 = Date::from_calendar_date(rug_fuzz_0, Month::January, rug_fuzz_1)
            .unwrap();
        let date2 = Date::from_calendar_date(rug_fuzz_2, Month::January, rug_fuzz_3)
            .unwrap();
        debug_assert_eq!(date1 - date2, Duration::days(365));
             }
});    }
    #[test]
    fn test_sub_negative_result() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u8, i32, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date1 = Date::from_calendar_date(rug_fuzz_0, Month::January, rug_fuzz_1)
            .unwrap();
        let date2 = Date::from_calendar_date(rug_fuzz_2, Month::December, rug_fuzz_3)
            .unwrap();
        debug_assert_eq!(date1 - date2, Duration::days(- 364));
             }
});    }
    #[test]
    fn test_sub_with_leap_year() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u8, i32, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date1 = Date::from_calendar_date(rug_fuzz_0, Month::December, rug_fuzz_1)
            .unwrap();
        let date2 = Date::from_calendar_date(rug_fuzz_2, Month::January, rug_fuzz_3)
            .unwrap();
        debug_assert_eq!(date1 - date2, Duration::days(365));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_164 {
    use super::*;
    use crate::*;
    use crate::error::ComponentRange;
    #[test]
    #[allow(unused_imports)]
    fn test_from_ordinal_date_unchecked() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u16, i32, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let valid_date = Date::__from_ordinal_date_unchecked(rug_fuzz_0, rug_fuzz_1);
        debug_assert_eq!(valid_date.year(), 2021);
        debug_assert_eq!(valid_date.ordinal(), 1);
        let year = rug_fuzz_2;
        let ordinal = rug_fuzz_3;
        let max_ordinal = days_in_year(year);
        if cfg!(debug_assertions) {
            let result = std::panic::catch_unwind(|| {
                Date::__from_ordinal_date_unchecked(year, ordinal)
            });
            debug_assert!(result.is_ok());
            if ordinal <= max_ordinal {
                debug_assert_eq!(result.unwrap().ordinal(), ordinal);
            }
        }
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_168 {
    use super::*;
    use crate::*;
    use crate::date::Date;
    use crate::Month;
    use crate::error::ComponentRange;
    #[test]
    fn from_calendar_date_valid_dates() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(i32, u8, i32, u8, i32, u8, i32, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(
            Date::from_calendar_date(rug_fuzz_0, Month::January, rug_fuzz_1).is_ok()
        );
        debug_assert!(
            Date::from_calendar_date(rug_fuzz_2, Month::December, rug_fuzz_3).is_ok()
        );
        debug_assert!(
            Date::from_calendar_date(rug_fuzz_4, Month::February, rug_fuzz_5).is_ok()
        );
        debug_assert!(
            Date::from_calendar_date(- rug_fuzz_6, Month::January, rug_fuzz_7).is_ok()
        );
             }
});    }
    #[test]
    fn from_calendar_date_invalid_dates() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(i32, u8, i32, u8, i32, u8, i32, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(
            Date::from_calendar_date(rug_fuzz_0, Month::February, rug_fuzz_1).is_err()
        );
        debug_assert!(
            Date::from_calendar_date(rug_fuzz_2, Month::April, rug_fuzz_3).is_err()
        );
        debug_assert!(
            Date::from_calendar_date(rug_fuzz_4, Month::December, rug_fuzz_5).is_err()
        );
        debug_assert!(
            matches!(Date::from_calendar_date(rug_fuzz_6, Month::February, rug_fuzz_7)
            .unwrap_err(), ComponentRange { name, minimum : 1, maximum : _, value : 29,
            conditional_range : true })
        );
             }
});    }
    #[test]
    fn from_calendar_date_edge_cases() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(
            Date::from_calendar_date(i32::MIN, Month::January, rug_fuzz_0).is_err()
        );
        debug_assert!(
            Date::from_calendar_date(i32::MAX, Month::January, rug_fuzz_1).is_err()
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_169 {
    use super::*;
    use crate::*;
    use crate::Weekday::{Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday};
    #[test]
    fn test_from_iso_week_date() {
        let _rug_st_tests_llm_16_169_rrrruuuugggg_test_from_iso_week_date = 0;
        let rug_fuzz_0 = 2019;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 2019;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 2019;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 2019;
        let rug_fuzz_7 = 1;
        let rug_fuzz_8 = 2019;
        let rug_fuzz_9 = 1;
        let rug_fuzz_10 = 2019;
        let rug_fuzz_11 = 1;
        let rug_fuzz_12 = 2019;
        let rug_fuzz_13 = 1;
        let rug_fuzz_14 = 2020;
        let rug_fuzz_15 = 53;
        let rug_fuzz_16 = 2019;
        let rug_fuzz_17 = 53;
        let rug_fuzz_18 = 2019;
        let rug_fuzz_19 = 0;
        let rug_fuzz_20 = 1;
        let rug_fuzz_21 = 1;
        let rug_fuzz_22 = 1;
        let rug_fuzz_23 = 1;
        let rug_fuzz_24 = 1;
        let rug_fuzz_25 = 1;
        let rug_fuzz_26 = 1;
        let rug_fuzz_27 = 1;
        debug_assert!(Date::from_iso_week_date(rug_fuzz_0, rug_fuzz_1, Monday).is_ok());
        debug_assert!(Date::from_iso_week_date(rug_fuzz_2, rug_fuzz_3, Tuesday).is_ok());
        debug_assert!(
            Date::from_iso_week_date(rug_fuzz_4, rug_fuzz_5, Wednesday).is_ok()
        );
        debug_assert!(
            Date::from_iso_week_date(rug_fuzz_6, rug_fuzz_7, Thursday).is_ok()
        );
        debug_assert!(Date::from_iso_week_date(rug_fuzz_8, rug_fuzz_9, Friday).is_ok());
        debug_assert!(
            Date::from_iso_week_date(rug_fuzz_10, rug_fuzz_11, Saturday).is_ok()
        );
        debug_assert!(
            Date::from_iso_week_date(rug_fuzz_12, rug_fuzz_13, Sunday).is_ok()
        );
        debug_assert!(
            Date::from_iso_week_date(rug_fuzz_14, rug_fuzz_15, Friday).is_ok()
        );
        debug_assert!(
            Date::from_iso_week_date(rug_fuzz_16, rug_fuzz_17, Monday).is_err()
        );
        debug_assert!(
            Date::from_iso_week_date(rug_fuzz_18, rug_fuzz_19, Monday).is_err()
        );
        debug_assert!(
            Date::from_iso_week_date(crate ::date::MIN_YEAR, rug_fuzz_20, Monday).is_ok()
        );
        debug_assert!(
            Date::from_iso_week_date(crate ::date::MIN_YEAR, rug_fuzz_21, Sunday).is_ok()
        );
        debug_assert!(
            Date::from_iso_week_date(crate ::date::MAX_YEAR, rug_fuzz_22, Monday).is_ok()
        );
        debug_assert!(
            Date::from_iso_week_date(crate ::date::MAX_YEAR, rug_fuzz_23, Sunday).is_ok()
        );
        debug_assert!(
            Date::from_iso_week_date(crate ::date::MIN_YEAR - rug_fuzz_24, rug_fuzz_25,
            Monday).is_err()
        );
        debug_assert!(
            Date::from_iso_week_date(crate ::date::MAX_YEAR + rug_fuzz_26, rug_fuzz_27,
            Sunday).is_err()
        );
        let _rug_ed_tests_llm_16_169_rrrruuuugggg_test_from_iso_week_date = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_171 {
    use super::*;
    use crate::*;
    #[test]
    fn test_from_julian_day_unchecked() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = Date::from_julian_day_unchecked(rug_fuzz_0);
        debug_assert_eq!(date.to_calendar_date(), (2000, Month::January, 1));
        let date = Date::from_julian_day_unchecked(rug_fuzz_1);
        debug_assert_eq!(date.to_calendar_date(), (2019, Month::January, 1));
        let date = Date::from_julian_day_unchecked(rug_fuzz_2);
        debug_assert_eq!(date.to_calendar_date(), (2019, Month::December, 31));
        let date = Date::from_julian_day_unchecked(rug_fuzz_3);
        debug_assert_eq!(date.to_calendar_date(), (- 4713, Month::November, 24));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_173 {
    use super::*;
    use crate::*;
    use crate::ext::NumericalDuration;
    use crate::Date;
    use time_macros::date;
    #[test]
    fn iso_week_number() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(i32, u8, i32, u8, i32, u8, i32, u8, i32, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Date::from_calendar_date(rug_fuzz_0, Month::January, rug_fuzz_1).unwrap()
            .iso_week(), 1
        );
        debug_assert_eq!(
            Date::from_calendar_date(rug_fuzz_2, Month::October, rug_fuzz_3).unwrap()
            .iso_week(), 40
        );
        debug_assert_eq!(
            Date::from_calendar_date(rug_fuzz_4, Month::January, rug_fuzz_5).unwrap()
            .iso_week(), 1
        );
        debug_assert_eq!(
            Date::from_calendar_date(rug_fuzz_6, Month::December, rug_fuzz_7).unwrap()
            .iso_week(), 53
        );
        debug_assert_eq!(
            Date::from_calendar_date(rug_fuzz_8, Month::January, rug_fuzz_9).unwrap()
            .iso_week(), 53
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_177 {
    use super::*;
    use crate::*;
    use crate::Month::{
        January, February, March, April, May, June, July, August, September, October,
        November, December,
    };
    use crate::Date;
    #[test]
    fn test_month() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17, mut rug_fuzz_18, mut rug_fuzz_19, mut rug_fuzz_20, mut rug_fuzz_21, mut rug_fuzz_22, mut rug_fuzz_23)) = <(i32, u8, i32, u8, i32, u8, i32, u8, i32, u8, i32, u8, i32, u8, i32, u8, i32, u8, i32, u8, i32, u8, i32, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Date::from_calendar_date(rug_fuzz_0, January, rug_fuzz_1).unwrap().month(),
            January
        );
        debug_assert_eq!(
            Date::from_calendar_date(rug_fuzz_2, February, rug_fuzz_3).unwrap().month(),
            February
        );
        debug_assert_eq!(
            Date::from_calendar_date(rug_fuzz_4, March, rug_fuzz_5).unwrap().month(),
            March
        );
        debug_assert_eq!(
            Date::from_calendar_date(rug_fuzz_6, April, rug_fuzz_7).unwrap().month(),
            April
        );
        debug_assert_eq!(
            Date::from_calendar_date(rug_fuzz_8, May, rug_fuzz_9).unwrap().month(), May
        );
        debug_assert_eq!(
            Date::from_calendar_date(rug_fuzz_10, June, rug_fuzz_11).unwrap().month(),
            June
        );
        debug_assert_eq!(
            Date::from_calendar_date(rug_fuzz_12, July, rug_fuzz_13).unwrap().month(),
            July
        );
        debug_assert_eq!(
            Date::from_calendar_date(rug_fuzz_14, August, rug_fuzz_15).unwrap().month(),
            August
        );
        debug_assert_eq!(
            Date::from_calendar_date(rug_fuzz_16, September, rug_fuzz_17).unwrap()
            .month(), September
        );
        debug_assert_eq!(
            Date::from_calendar_date(rug_fuzz_18, October, rug_fuzz_19).unwrap().month(),
            October
        );
        debug_assert_eq!(
            Date::from_calendar_date(rug_fuzz_20, November, rug_fuzz_21).unwrap()
            .month(), November
        );
        debug_assert_eq!(
            Date::from_calendar_date(rug_fuzz_22, December, rug_fuzz_23).unwrap()
            .month(), December
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_178 {
    use super::*;
    use crate::*;
    use crate::Month::*;
    #[test]
    fn test_month_day() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(i32, u16, i32, u16, i32, u16, i32, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let common_year_date = Date::__from_ordinal_date_unchecked(
            rug_fuzz_0,
            rug_fuzz_1,
        );
        let leap_year_date = Date::__from_ordinal_date_unchecked(rug_fuzz_2, rug_fuzz_3);
        let end_of_year_date = Date::__from_ordinal_date_unchecked(
            rug_fuzz_4,
            rug_fuzz_5,
        );
        let start_of_year_date = Date::__from_ordinal_date_unchecked(
            rug_fuzz_6,
            rug_fuzz_7,
        );
        debug_assert_eq!(common_year_date.month_day(), (February, 28));
        debug_assert_eq!(leap_year_date.month_day(), (February, 29));
        debug_assert_eq!(end_of_year_date.month_day(), (December, 31));
        debug_assert_eq!(start_of_year_date.month_day(), (January, 1));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_182_llm_16_182 {
    use crate::Date;
    use crate::error::ComponentRange;
    use crate::Month;
    use time_macros::date;
    #[test]
    fn replace_day_valid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(i32, u8, u8, i32, u8, u8, i32, u8, u8, i32, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Date::from_calendar_date(rug_fuzz_0, Month::February, rug_fuzz_1).unwrap()
            .replace_day(rug_fuzz_2), Ok(Date::from_calendar_date(2022, Month::February,
            1).unwrap())
        );
        debug_assert_eq!(
            Date::from_calendar_date(rug_fuzz_3, Month::February, rug_fuzz_4).unwrap()
            .replace_day(rug_fuzz_5), Ok(Date::from_calendar_date(2022, Month::February,
            28).unwrap())
        );
        debug_assert_eq!(
            Date::from_calendar_date(rug_fuzz_6, Month::January, rug_fuzz_7).unwrap()
            .replace_day(rug_fuzz_8), Ok(Date::from_calendar_date(2022, Month::January,
            31).unwrap())
        );
        debug_assert_eq!(
            Date::from_calendar_date(rug_fuzz_9, Month::February, rug_fuzz_10).unwrap()
            .replace_day(rug_fuzz_11), Ok(Date::from_calendar_date(2020, Month::February,
            29).unwrap())
        );
             }
});    }
    #[test]
    fn replace_day_invalid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(i32, u8, u8, i32, u8, u8, i32, u8, u8, i32, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Date::from_calendar_date(rug_fuzz_0, Month::February, rug_fuzz_1).unwrap()
            .replace_day(rug_fuzz_2).unwrap_err(), ComponentRange { name : "day", value :
            0, minimum : 1, maximum : 29, conditional_range : true, }
        );
        debug_assert_eq!(
            Date::from_calendar_date(rug_fuzz_3, Month::February, rug_fuzz_4).unwrap()
            .replace_day(rug_fuzz_5).unwrap_err(), ComponentRange { name : "day", value :
            30, minimum : 1, maximum : 29, conditional_range : true, }
        );
        debug_assert_eq!(
            Date::from_calendar_date(rug_fuzz_6, Month::April, rug_fuzz_7).unwrap()
            .replace_day(rug_fuzz_8).unwrap_err(), ComponentRange { name : "day", value :
            31, minimum : 1, maximum : 30, conditional_range : true, }
        );
        debug_assert_eq!(
            Date::from_calendar_date(rug_fuzz_9, Month::January, rug_fuzz_10).unwrap()
            .replace_day(rug_fuzz_11).unwrap_err(), ComponentRange { name : "day", value
            : 32, minimum : 1, maximum : 31, conditional_range : true, }
        );
             }
});    }
    #[test]
    fn replace_day_edge_cases() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8)) = <(i32, u8, u8, i32, u8, u8, i32, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Date::from_calendar_date(rug_fuzz_0, Month::December, rug_fuzz_1).unwrap()
            .replace_day(rug_fuzz_2), Ok(Date::from_calendar_date(2022, Month::December,
            1).unwrap())
        );
        debug_assert_eq!(
            Date::from_calendar_date(rug_fuzz_3, Month::February, rug_fuzz_4).unwrap()
            .replace_day(rug_fuzz_5), Ok(Date::from_calendar_date(2020, Month::February,
            28).unwrap())
        );
        debug_assert_eq!(
            Date::from_calendar_date(rug_fuzz_6, Month::February, rug_fuzz_7).unwrap()
            .replace_day(rug_fuzz_8).unwrap_err(), ComponentRange { name : "day", value :
            29, minimum : 1, maximum : 28, conditional_range : true, }
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_184_llm_16_184 {
    use crate::Date;
    use crate::error::ComponentRange;
    use time_macros::date;
    #[test]
    fn replace_valid_year() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u8, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Date::from_calendar_date(rug_fuzz_0, crate ::Month::February, rug_fuzz_1)
            .unwrap().replace_year(rug_fuzz_2), Ok(Date::from_calendar_date(2019, crate
            ::Month::February, 18).unwrap())
        );
             }
});    }
    #[test]
    fn replace_invalid_year() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u8, i32, i32, u8, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(
            Date::from_calendar_date(rug_fuzz_0, crate ::Month::February, rug_fuzz_1)
            .unwrap().replace_year(- rug_fuzz_2).is_err()
        );
        debug_assert!(
            Date::from_calendar_date(rug_fuzz_3, crate ::Month::February, rug_fuzz_4)
            .unwrap().replace_year(rug_fuzz_5).is_err()
        );
             }
});    }
    #[test]
    fn replace_leap_year_to_common() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u8, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Date::from_calendar_date(rug_fuzz_0, crate ::Month::February, rug_fuzz_1)
            .unwrap().replace_year(rug_fuzz_2), Err(ComponentRange { name : "day", value
            : 29, minimum : 1, maximum : 28, conditional_range : true, })
        );
             }
});    }
    #[test]
    fn replace_common_year_to_leap() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u8, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Date::from_calendar_date(rug_fuzz_0, crate ::Month::March, rug_fuzz_1)
            .unwrap().replace_year(rug_fuzz_2), Ok(Date::from_calendar_date(2020, crate
            ::Month::March, 1).unwrap())
        );
             }
});    }
    #[test]
    fn replace_leap_year_to_leap() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u8, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Date::from_calendar_date(rug_fuzz_0, crate ::Month::February, rug_fuzz_1)
            .unwrap().replace_year(rug_fuzz_2), Ok(Date::from_calendar_date(2016, crate
            ::Month::February, 29).unwrap())
        );
             }
});    }
    #[test]
    fn replace_common_year_to_common() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u8, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Date::from_calendar_date(rug_fuzz_0, crate ::Month::January, rug_fuzz_1)
            .unwrap().replace_year(rug_fuzz_2), Ok(Date::from_calendar_date(2018, crate
            ::Month::January, 15).unwrap())
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_185 {
    use crate::{ext::NumericalDuration, Date, Duration};
    #[test]
    fn saturating_add_positive_no_overflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u8, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = Date::from_calendar_date(
                rug_fuzz_0,
                crate::Month::January,
                rug_fuzz_1,
            )
            .unwrap();
        debug_assert_eq!(
            date.saturating_add(rug_fuzz_2.days()), Date::from_calendar_date(2020, crate
            ::Month::January, 2).unwrap()
        );
             }
});    }
    #[test]
    fn saturating_add_positive_overflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = Date::MAX;
        debug_assert_eq!(date.saturating_add(rug_fuzz_0.days()), Date::MAX);
             }
});    }
    #[test]
    fn saturating_add_negative_no_overflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u8, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = Date::from_calendar_date(
                rug_fuzz_0,
                crate::Month::January,
                rug_fuzz_1,
            )
            .unwrap();
        debug_assert_eq!(
            date.saturating_add((- rug_fuzz_2).days()), Date::from_calendar_date(2020,
            crate ::Month::January, 1).unwrap()
        );
             }
});    }
    #[test]
    fn saturating_add_negative_overflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = Date::MIN;
        debug_assert_eq!(date.saturating_add((- rug_fuzz_0).days()), Date::MIN);
             }
});    }
    #[test]
    fn saturating_add_large_duration() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u8, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = Date::from_calendar_date(
                rug_fuzz_0,
                crate::Month::January,
                rug_fuzz_1,
            )
            .unwrap();
        let duration = Duration::new(i64::MAX, rug_fuzz_2);
        debug_assert_eq!(date.saturating_add(duration), Date::MAX);
             }
});    }
    #[test]
    fn saturating_add_with_hours() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u8, i64, i32, u8, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Date::from_calendar_date(rug_fuzz_0, crate ::Month::December, rug_fuzz_1)
            .unwrap().saturating_add(rug_fuzz_2.hours()), Date::from_calendar_date(2020,
            crate ::Month::December, 31).unwrap()
        );
        debug_assert_eq!(
            Date::from_calendar_date(rug_fuzz_3, crate ::Month::December, rug_fuzz_4)
            .unwrap().saturating_add(rug_fuzz_5.hours()), Date::from_calendar_date(2021,
            crate ::Month::January, 1).unwrap()
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_186 {
    use super::*;
    use crate::*;
    use crate::ext::NumericalDuration;
    use crate::Date;
    use crate::Duration;
    #[test]
    fn saturating_sub_underflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Date::MIN.saturating_sub(rug_fuzz_0.days()), Date::MIN);
             }
});    }
    #[test]
    fn saturating_sub_no_underflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Date::MIN.saturating_sub((- rug_fuzz_0).days()), Date::MIN.checked_add(1
            .days()).unwrap()
        );
             }
});    }
    #[test]
    fn saturating_sub_overflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Date::MAX.saturating_sub((- rug_fuzz_0).days()), Date::MAX);
             }
});    }
    #[test]
    fn saturating_sub_no_overflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Date::MAX.saturating_sub(rug_fuzz_0.days()), Date::MAX.checked_sub(1.days())
            .unwrap()
        );
             }
});    }
    #[test]
    fn saturating_sub_days() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u8, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Date::from_calendar_date(rug_fuzz_0, Month::January, rug_fuzz_1).unwrap()
            .saturating_sub(rug_fuzz_2.days()), Date::from_calendar_date(2019,
            Month::December, 30).unwrap()
        );
             }
});    }
    #[test]
    fn saturating_sub_hours_no_change() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u8, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Date::from_calendar_date(rug_fuzz_0, Month::January, rug_fuzz_1).unwrap()
            .saturating_sub(rug_fuzz_2.hours()), Date::from_calendar_date(2020,
            Month::January, 1).unwrap()
        );
             }
});    }
    #[test]
    fn saturating_sub_hours_change() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u8, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Date::from_calendar_date(rug_fuzz_0, Month::January, rug_fuzz_1).unwrap()
            .saturating_sub(rug_fuzz_2.hours()), Date::from_calendar_date(2019,
            Month::December, 31).unwrap()
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_188 {
    use super::*;
    use crate::*;
    use crate::Month::*;
    use crate::error::ComponentRange;
    #[test]
    fn to_calendar_date_works() {
        let _rug_st_tests_llm_16_188_rrrruuuugggg_to_calendar_date_works = 0;
        let rug_fuzz_0 = 2019;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 2019;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 2020;
        let rug_fuzz_5 = 29;
        let rug_fuzz_6 = 2020;
        let rug_fuzz_7 = 29;
        let rug_fuzz_8 = 2019;
        let rug_fuzz_9 = 31;
        let rug_fuzz_10 = 2019;
        let rug_fuzz_11 = 31;
        let rug_fuzz_12 = 2000;
        let rug_fuzz_13 = 29;
        let rug_fuzz_14 = 2000;
        let rug_fuzz_15 = 29;
        let rug_fuzz_16 = 1900;
        let rug_fuzz_17 = 28;
        let rug_fuzz_18 = 1900;
        let rug_fuzz_19 = 28;
        let rug_fuzz_20 = 1970;
        let rug_fuzz_21 = 1;
        let rug_fuzz_22 = 1970;
        let rug_fuzz_23 = 1;
        let rug_fuzz_24 = 1;
        let rug_fuzz_25 = 31;
        let rug_fuzz_26 = 1;
        let rug_fuzz_27 = 31;
        let rug_fuzz_28 = 2021;
        let rug_fuzz_29 = 12;
        let rug_fuzz_30 = 2021;
        let rug_fuzz_31 = 12;
        let cases = [
            (rug_fuzz_0, January, rug_fuzz_1, rug_fuzz_2, January, rug_fuzz_3),
            (rug_fuzz_4, February, rug_fuzz_5, rug_fuzz_6, February, rug_fuzz_7),
            (rug_fuzz_8, December, rug_fuzz_9, rug_fuzz_10, December, rug_fuzz_11),
            (rug_fuzz_12, February, rug_fuzz_13, rug_fuzz_14, February, rug_fuzz_15),
            (rug_fuzz_16, February, rug_fuzz_17, rug_fuzz_18, February, rug_fuzz_19),
            (rug_fuzz_20, June, rug_fuzz_21, rug_fuzz_22, June, rug_fuzz_23),
            (-rug_fuzz_24, December, rug_fuzz_25, -rug_fuzz_26, December, rug_fuzz_27),
            (rug_fuzz_28, May, rug_fuzz_29, rug_fuzz_30, May, rug_fuzz_31),
        ];
        for (year, month, day, expected_year, expected_month, expected_day) in cases {
            let date = Date::from_calendar_date(year, month, day).unwrap();
            let (year, month, day) = date.to_calendar_date();
            debug_assert_eq!(
                (year, month, day), (expected_year, expected_month, expected_day)
            );
        }
        let _rug_ed_tests_llm_16_188_rrrruuuugggg_to_calendar_date_works = 0;
    }
    #[test]
    fn to_calendar_date_failures() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(i32, u8, i32, u8, i32, u8, i32, u8, i32, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let cases = [
            (rug_fuzz_0, January, rug_fuzz_1),
            (rug_fuzz_2, January, rug_fuzz_3),
            (-rug_fuzz_4, January, rug_fuzz_5),
            (rug_fuzz_6, February, rug_fuzz_7),
            (rug_fuzz_8, February, rug_fuzz_9),
        ];
        for (year, month, day) in cases {
            let date = Date::from_calendar_date(year, month, day);
            debug_assert!(matches!(date, Err(ComponentRange { .. })));
        }
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_195_llm_16_195 {
    use crate::{
        error::{self, ComponentRange},
        Date, PrimitiveDateTime, Time, ext::NumericalDuration,
    };
    #[test]
    fn with_hms_milli_valid_time() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(i32, u8, u8, u8, u8, u16, u8, u8, u8, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let valid_date = Date::from_calendar_date(
                rug_fuzz_0,
                crate::Month::March,
                rug_fuzz_1,
            )
            .unwrap();
        let valid_time = valid_date
            .with_time(
                Time::from_hms_milli(rug_fuzz_2, rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
                    .unwrap(),
            );
        debug_assert_eq!(
            valid_date.with_hms_milli(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8, rug_fuzz_9),
            Ok(valid_time)
        );
             }
});    }
    #[test]
    fn with_hms_milli_invalid_hour() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u8, u8, u8, u8, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(
            matches!(Date::from_calendar_date(rug_fuzz_0, crate ::Month::March,
            rug_fuzz_1).unwrap().with_hms_milli(rug_fuzz_2, rug_fuzz_3, rug_fuzz_4,
            rug_fuzz_5), Err(ComponentRange { .. }))
        );
             }
});    }
    #[test]
    fn with_hms_milli_invalid_minute() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u8, u8, u8, u8, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(
            matches!(Date::from_calendar_date(rug_fuzz_0, crate ::Month::March,
            rug_fuzz_1).unwrap().with_hms_milli(rug_fuzz_2, rug_fuzz_3, rug_fuzz_4,
            rug_fuzz_5), Err(ComponentRange { .. }))
        );
             }
});    }
    #[test]
    fn with_hms_milli_invalid_second() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u8, u8, u8, u8, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(
            matches!(Date::from_calendar_date(rug_fuzz_0, crate ::Month::March,
            rug_fuzz_1).unwrap().with_hms_milli(rug_fuzz_2, rug_fuzz_3, rug_fuzz_4,
            rug_fuzz_5), Err(ComponentRange { .. }))
        );
             }
});    }
    #[test]
    fn with_hms_milli_invalid_millisecond() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u8, u8, u8, u8, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(
            matches!(Date::from_calendar_date(rug_fuzz_0, crate ::Month::March,
            rug_fuzz_1).unwrap().with_hms_milli(rug_fuzz_2, rug_fuzz_3, rug_fuzz_4,
            rug_fuzz_5), Err(ComponentRange { .. }))
        );
             }
});    }
    #[test]
    fn with_hms_milli_min_value() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(i32, u8, u8, u8, u8, u16, u8, u8, u8, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let valid_date = Date::from_calendar_date(
                rug_fuzz_0,
                crate::Month::March,
                rug_fuzz_1,
            )
            .unwrap();
        let valid_time = valid_date
            .with_time(
                Time::from_hms_milli(rug_fuzz_2, rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
                    .unwrap(),
            );
        debug_assert_eq!(
            valid_date.with_hms_milli(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8, rug_fuzz_9),
            Ok(valid_time)
        );
             }
});    }
    #[test]
    fn with_hms_milli_max_value() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(i32, u8, u8, u8, u8, u16, u8, u8, u8, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let valid_date = Date::from_calendar_date(
                rug_fuzz_0,
                crate::Month::March,
                rug_fuzz_1,
            )
            .unwrap();
        let valid_time = valid_date
            .with_time(
                Time::from_hms_milli(rug_fuzz_2, rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
                    .unwrap(),
            );
        debug_assert_eq!(
            valid_date.with_hms_milli(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8, rug_fuzz_9),
            Ok(valid_time)
        );
             }
});    }
}
