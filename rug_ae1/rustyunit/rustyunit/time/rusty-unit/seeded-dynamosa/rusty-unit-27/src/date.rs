//! The [`Date`] struct and its associated `impl`s.

use core::fmt;
use core::ops::{Add, Sub};
use core::time::Duration as StdDuration;
#[cfg(feature = "formatting")]
use std::io;

#[cfg(feature = "formatting")]
use crate::formatting::Formattable;
#[cfg(feature = "parsing")]
use crate::parsing::Parsable;
use crate::util::{days_in_year, days_in_year_month, is_leap_year, weeks_in_year};
use crate::{error, Duration, Month, PrimitiveDateTime, Time, Weekday};

/// The minimum valid year.
#[cfg(feature = "large-dates")]
pub(crate) const MIN_YEAR: i32 = -999_999;
/// The maximum valid year.
#[cfg(feature = "large-dates")]
pub(crate) const MAX_YEAR: i32 = 999_999;

/// The minimum valid year.
#[cfg(not(feature = "large-dates"))]
pub(crate) const MIN_YEAR: i32 = -9999;
/// The maximum valid year.
#[cfg(not(feature = "large-dates"))]
pub(crate) const MAX_YEAR: i32 = 9999;

/// Date in the proleptic Gregorian calendar.
///
/// By default, years between ±9999 inclusive are representable. This can be expanded to ±999,999
/// inclusive by enabling the `large-dates` crate feature. Doing so has performance implications
/// and introduces some ambiguities when parsing.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Date {
    /// Bitpacked field containing both the year and ordinal.
    // |     xx     | xxxxxxxxxxxxxxxxxxxxx | xxxxxxxxx |
    // |   2 bits   |        21 bits        |  9 bits   |
    // | unassigned |         year          |  ordinal  |
    // The year is 15 bits when `large-dates` is not enabled.
    pub(crate) value: i32,
}

impl fmt::Debug for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.debug_struct("Date")
            .field("year", &self.year())
            .field("ordinal", &self.ordinal())
            .finish()
    }
}

impl Date {
    /// The minimum valid `Date`.
    ///
    /// The value of this may vary depending on the feature flags enabled.
    pub const MIN: Self = Self::__from_ordinal_date_unchecked(MIN_YEAR, 1);

    /// The maximum valid `Date`.
    ///
    /// The value of this may vary depending on the feature flags enabled.
    pub const MAX: Self = Self::__from_ordinal_date_unchecked(MAX_YEAR, days_in_year(MAX_YEAR));

    // region: constructors
    /// Construct a `Date` from the year and ordinal values, the validity of which must be
    /// guaranteed by the caller.
    #[doc(hidden)]
    pub const fn __from_ordinal_date_unchecked(year: i32, ordinal: u16) -> Self {
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
        ensure_value_in_range!(day conditionally in 1 => days_in_year_month(year, month));

        Ok(Self::__from_ordinal_date_unchecked(
            year,
            DAYS_CUMULATIVE_COMMON_LEAP[is_leap_year(year) as usize][month as usize - 1]
                + day as u16,
        ))
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
    pub const fn from_ordinal_date(year: i32, ordinal: u16) -> Result<Self, error::ComponentRange> {
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

        Ok(if ordinal <= 0 {
            Self::__from_ordinal_date_unchecked(
                year - 1,
                (ordinal as u16).wrapping_add(days_in_year(year - 1)),
            )
        } else if ordinal > days_in_year(year) as i16 {
            Self::__from_ordinal_date_unchecked(year + 1, ordinal as u16 - days_in_year(year))
        } else {
            Self::__from_ordinal_date_unchecked(year, ordinal as _)
        })
    }

    /// Create a `Date` from the Julian day.
    ///
    /// The algorithm to perform this conversion is derived from one provided by Peter Baum; it is
    /// freely available [here](https://www.researchgate.net/publication/316558298_Date_Algorithms).
    ///
    /// ```rust
    /// # use time::{Date, macros::date};
    /// assert_eq!(Date::from_julian_day(0), Ok(date!(-4713 - 11 - 24)));
    /// assert_eq!(Date::from_julian_day(2_451_545), Ok(date!(2000 - 01 - 01)));
    /// assert_eq!(Date::from_julian_day(2_458_485), Ok(date!(2019 - 01 - 01)));
    /// assert_eq!(Date::from_julian_day(2_458_849), Ok(date!(2019 - 12 - 31)));
    /// ```
    #[doc(alias = "from_julian_date")]
    pub const fn from_julian_day(julian_day: i32) -> Result<Self, error::ComponentRange> {
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
        #![allow(trivial_numeric_casts)] // cast depends on type alias

        /// A type that is either `i32` or `i64`. This subtle difference allows for optimization
        /// based on the valid values.
        #[cfg(feature = "large-dates")]
        type MaybeWidened = i64;
        #[allow(clippy::missing_docs_in_private_items)]
        #[cfg(not(feature = "large-dates"))]
        type MaybeWidened = i32;

        // To avoid a potential overflow, the value may need to be widened for some arithmetic.

        let z = julian_day - 1_721_119;
        let g = 100 * z as MaybeWidened - 25;
        let a = (g / 3_652_425) as i32;
        let b = a - a / 4;
        let mut year = div_floor!(100 * b as MaybeWidened + g, 36525) as i32;
        let mut ordinal = (b + z - div_floor!(36525 * year as MaybeWidened, 100) as i32) as _;

        if is_leap_year(year) {
            ordinal += 60;
            cascade!(ordinal in 1..367 => year);
        } else {
            ordinal += 59;
            cascade!(ordinal in 1..366 => year);
        }

        Self::__from_ordinal_date_unchecked(year, ordinal)
    }
    // endregion constructors

    // region: getters
    /// Get the year of the date.
    ///
    /// ```rust
    /// # use time::macros::date;
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
    /// # use time::{macros::date, Month};
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
    /// # use time::macros::date;
    /// assert_eq!(date!(2019 - 01 - 01).day(), 1);
    /// assert_eq!(date!(2019 - 12 - 31).day(), 31);
    /// ```
    pub const fn day(self) -> u8 {
        self.month_day().1
    }

    /// Get the month and day. This is more efficient than fetching the components individually.
    // For whatever reason, rustc has difficulty optimizing this function. It's significantly faster
    // to write the statements out by hand.
    pub(crate) const fn month_day(self) -> (Month, u8) {
        /// The number of days up to and including the given month. Common years
        /// are first, followed by leap years.
        const CUMULATIVE_DAYS_IN_MONTH_COMMON_LEAP: [[u16; 11]; 2] = [
            [31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334],
            [31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335],
        ];

        let days = CUMULATIVE_DAYS_IN_MONTH_COMMON_LEAP[is_leap_year(self.year()) as usize];
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
    /// # use time::macros::date;
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
    /// # use time::macros::date;
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
    /// # use time::macros::date;
    /// assert_eq!(date!(2019 - 01 - 01).sunday_based_week(), 0);
    /// assert_eq!(date!(2020 - 01 - 01).sunday_based_week(), 0);
    /// assert_eq!(date!(2020 - 12 - 31).sunday_based_week(), 52);
    /// assert_eq!(date!(2021 - 01 - 01).sunday_based_week(), 0);
    /// ```
    pub const fn sunday_based_week(self) -> u8 {
        ((self.ordinal() as i16 - self.weekday().number_days_from_sunday() as i16 + 6) / 7) as _
    }

    /// Get the week number where week 1 begins on the first Monday.
    ///
    /// The returned value will always be in the range `0..=53`.
    ///
    /// ```rust
    /// # use time::macros::date;
    /// assert_eq!(date!(2019 - 01 - 01).monday_based_week(), 0);
    /// assert_eq!(date!(2020 - 01 - 01).monday_based_week(), 0);
    /// assert_eq!(date!(2020 - 12 - 31).monday_based_week(), 52);
    /// assert_eq!(date!(2021 - 01 - 01).monday_based_week(), 0);
    /// ```
    pub const fn monday_based_week(self) -> u8 {
        ((self.ordinal() as i16 - self.weekday().number_days_from_monday() as i16 + 6) / 7) as _
    }

    /// Get the year, month, and day.
    ///
    /// ```rust
    /// # use time::{macros::date, Month};
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
    /// # use time::macros::date;
    /// assert_eq!(date!(2019 - 01 - 01).to_ordinal_date(), (2019, 1));
    /// ```
    pub const fn to_ordinal_date(self) -> (i32, u16) {
        (self.year(), self.ordinal())
    }

    /// Get the ISO 8601 year, week number, and weekday.
    ///
    /// ```rust
    /// # use time::{Weekday::*, macros::date};
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
    /// # use time::{Weekday::*, macros::date};
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
            _ => Weekday::Monday,
        }
    }

    /// Get the next calendar date.
    ///
    /// ```rust
    /// # use time::{Date, macros::date};
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
        if self.ordinal() == 366 || (self.ordinal() == 365 && !is_leap_year(self.year())) {
            if self.value == Self::MAX.value {
                None
            } else {
                Some(Self::__from_ordinal_date_unchecked(self.year() + 1, 1))
            }
        } else {
            Some(Self {
                value: self.value + 1,
            })
        }
    }

    /// Get the previous calendar date.
    ///
    /// ```rust
    /// # use time::{Date, macros::date};
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
            Some(Self {
                value: self.value - 1,
            })
        } else if self.value == Self::MIN.value {
            None
        } else {
            Some(Self::__from_ordinal_date_unchecked(
                self.year() - 1,
                days_in_year(self.year() - 1),
            ))
        }
    }

    /// Get the Julian day for the date.
    ///
    /// The algorithm to perform this conversion is derived from one provided by Peter Baum; it is
    /// freely available [here](https://www.researchgate.net/publication/316558298_Date_Algorithms).
    ///
    /// ```rust
    /// # use time::macros::date;
    /// assert_eq!(date!(-4713 - 11 - 24).to_julian_day(), 0);
    /// assert_eq!(date!(2000 - 01 - 01).to_julian_day(), 2_451_545);
    /// assert_eq!(date!(2019 - 01 - 01).to_julian_day(), 2_458_485);
    /// assert_eq!(date!(2019 - 12 - 31).to_julian_day(), 2_458_849);
    /// ```
    pub const fn to_julian_day(self) -> i32 {
        let year = self.year() - 1;
        let ordinal = self.ordinal() as i32;

        ordinal + 365 * year + div_floor!(year, 4) - div_floor!(year, 100)
            + div_floor!(year, 400)
            + 1_721_425
    }
    // endregion getters

    // region: checked arithmetic
    /// Computes `self + duration`, returning `None` if an overflow occurred.
    ///
    /// ```rust
    /// # use time::{Date, ext::NumericalDuration, macros::date};
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
    /// # use time::{Date, ext::NumericalDuration, macros::date};
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

        let julian_day = const_try_opt!(self.to_julian_day().checked_add(whole_days as _));
        if let Ok(date) = Self::from_julian_day(julian_day) {
            Some(date)
        } else {
            None
        }
    }

    /// Computes `self - duration`, returning `None` if an overflow occurred.
    ///
    /// ```
    /// # use time::{Date, ext::NumericalDuration, macros::date};
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
    /// # use time::{Date, ext::NumericalDuration, macros::date};
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

        let julian_day = const_try_opt!(self.to_julian_day().checked_sub(whole_days as _));
        if let Ok(date) = Self::from_julian_day(julian_day) {
            Some(date)
        } else {
            None
        }
    }
    // endregion: checked arithmetic

    // region: saturating arithmetic
    /// Computes `self + duration`, saturating value on overflow.
    ///
    /// ```rust
    /// # use time::{Date, ext::NumericalDuration, macros::date};
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
    /// # use time::{ext::NumericalDuration, macros::date};
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
            Self::MAX
        }
    }

    /// Computes `self - duration`, saturating value on overflow.
    ///
    /// ```
    /// # use time::{Date, ext::NumericalDuration, macros::date};
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
    /// # use time::{ext::NumericalDuration, macros::date};
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
            Self::MIN
        }
    }
    // region: saturating arithmetic
}

// region: attach time
/// Methods to add a [`Time`] component, resulting in a [`PrimitiveDateTime`].
impl Date {
    /// Create a [`PrimitiveDateTime`] using the existing date. The [`Time`] component will be set
    /// to midnight.
    ///
    /// ```rust
    /// # use time::macros::{date, datetime};
    /// assert_eq!(date!(1970-01-01).midnight(), datetime!(1970-01-01 0:00));
    /// ```
    pub const fn midnight(self) -> PrimitiveDateTime {
        PrimitiveDateTime::new(self, Time::MIDNIGHT)
    }

    /// Create a [`PrimitiveDateTime`] using the existing date and the provided [`Time`].
    ///
    /// ```rust
    /// # use time::macros::{date, datetime, time};
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
    /// # use time::macros::date;
    /// assert!(date!(1970 - 01 - 01).with_hms(0, 0, 0).is_ok());
    /// assert!(date!(1970 - 01 - 01).with_hms(24, 0, 0).is_err());
    /// ```
    pub const fn with_hms(
        self,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> Result<PrimitiveDateTime, error::ComponentRange> {
        Ok(PrimitiveDateTime::new(
            self,
            const_try!(Time::from_hms(hour, minute, second)),
        ))
    }

    /// Attempt to create a [`PrimitiveDateTime`] using the existing date and the provided time.
    ///
    /// ```rust
    /// # use time::macros::date;
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
        Ok(PrimitiveDateTime::new(
            self,
            const_try!(Time::from_hms_milli(hour, minute, second, millisecond)),
        ))
    }

    /// Attempt to create a [`PrimitiveDateTime`] using the existing date and the provided time.
    ///
    /// ```rust
    /// # use time::macros::date;
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
        Ok(PrimitiveDateTime::new(
            self,
            const_try!(Time::from_hms_micro(hour, minute, second, microsecond)),
        ))
    }

    /// Attempt to create a [`PrimitiveDateTime`] using the existing date and the provided time.
    ///
    /// ```rust
    /// # use time::macros::date;
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
        Ok(PrimitiveDateTime::new(
            self,
            const_try!(Time::from_hms_nano(hour, minute, second, nanosecond)),
        ))
    }
}
// endregion attach time

// region: formatting & parsing
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
    /// # use time::{format_description, macros::date};
    /// let format = format_description::parse("[year]-[month]-[day]")?;
    /// assert_eq!(date!(2020 - 01 - 02).format(&format)?, "2020-01-02");
    /// # Ok::<_, time::Error>(())
    /// ```
    pub fn format(self, format: &(impl Formattable + ?Sized)) -> Result<String, error::Format> {
        format.format(Some(self), None, None)
    }
}

#[cfg(feature = "parsing")]
impl Date {
    /// Parse a `Date` from the input using the provided [format
    /// description](crate::format_description).
    ///
    /// ```rust
    /// # use time::{format_description, macros::date, Date};
    /// let format = format_description::parse("[year]-[month]-[day]")?;
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
            write!(
                f,
                "{:+}-{:02}-{:02}",
                self.year(),
                self.month() as u8,
                self.day()
            )
        } else {
            write!(
                f,
                "{:0width$}-{:02}-{:02}",
                self.year(),
                self.month() as u8,
                self.day(),
                width = 4 + (self.year() < 0) as usize
            )
        }
    }
}
// endregion formatting & parsing

// region: trait impls
impl Add<Duration> for Date {
    type Output = Self;

    fn add(self, duration: Duration) -> Self::Output {
        self.checked_add(duration)
            .expect("overflow adding duration to date")
    }
}

impl Add<StdDuration> for Date {
    type Output = Self;

    fn add(self, duration: StdDuration) -> Self::Output {
        Self::from_julian_day(self.to_julian_day() + (duration.as_secs() / 86_400) as i32)
            .expect("overflow adding duration to date")
    }
}

impl_add_assign!(Date: Duration, StdDuration);

impl Sub<Duration> for Date {
    type Output = Self;

    fn sub(self, duration: Duration) -> Self::Output {
        self.checked_sub(duration)
            .expect("overflow subtracting duration from date")
    }
}

impl Sub<StdDuration> for Date {
    type Output = Self;

    fn sub(self, duration: StdDuration) -> Self::Output {
        Self::from_julian_day(self.to_julian_day() - (duration.as_secs() / 86_400) as i32)
            .expect("overflow subtracting duration from date")
    }
}

impl_sub_assign!(Date: Duration, StdDuration);

impl Sub for Date {
    type Output = Duration;

    fn sub(self, other: Self) -> Self::Output {
        Duration::days((self.to_julian_day() - other.to_julian_day()) as _)
    }
}
// endregion trait impls

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::clone::Clone;
	use std::ops::Add;
	use std::ops::Sub;
	use std::cmp::Eq;
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_87() {
//    rusty_monitor::set_test_id(87);
    let mut i32_0: i32 = 398i32;
    let mut i64_0: i64 = 604800i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut i64_1: i64 = 1000000000i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_1, duration_0);
    let mut i64_2: i64 = 181i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_3);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut u16_0: u16 = 7u16;
    let mut i32_1: i32 = 178i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_0);
    let mut u16_1: u16 = 10u16;
    let mut i32_2: i32 = -112i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_1);
    let mut i64_3: i64 = -96i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::minutes(i64_3);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut i32_3: i32 = 0i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut date_3: crate::date::Date = std::ops::Sub::sub(date_2, duration_5);
    let mut duration_6: crate::duration::Duration = std::ops::Sub::sub(date_3, date_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_1, duration_2);
    let mut i64_4: i64 = crate::duration::Duration::whole_days(duration_6);
    let mut u8_0: u8 = crate::util::weeks_in_year(i32_0);
    let mut u8_1: u8 = crate::primitive_date_time::PrimitiveDateTime::day(primitivedatetime_2);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_180() {
//    rusty_monitor::set_test_id(180);
    let mut i8_0: i8 = 3i8;
    let mut i8_1: i8 = 117i8;
    let mut i8_2: i8 = 24i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = 24i8;
    let mut i8_4: i8 = 5i8;
    let mut i8_5: i8 = 70i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i32_0: i32 = -19i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_0};
    let mut i64_0: i64 = -37i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut i8_6: i8 = 23i8;
    let mut i8_7: i8 = 3i8;
    let mut i8_8: i8 = 2i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i64_1: i64 = 0i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut i32_1: i32 = 240i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_3: crate::date::Date = std::ops::Sub::sub(date_2, duration_2);
    let mut u32_0: u32 = 10u32;
    let mut u8_0: u8 = 1u8;
    let mut u8_1: u8 = 23u8;
    let mut u8_2: u8 = 28u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 0u16;
    let mut i32_2: i32 = 1721119i32;
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_4, time_0);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_2, date_3);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_3, utcoffset_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_1, duration_1);
    let mut u8_3: u8 = crate::offset_date_time::OffsetDateTime::monday_based_week(offsetdatetime_2);
    let mut u8_4: u8 = crate::offset_date_time::OffsetDateTime::monday_based_week(offsetdatetime_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1862() {
//    rusty_monitor::set_test_id(1862);
    let mut u16_0: u16 = 999u16;
    let mut u8_0: u8 = 86u8;
    let mut u8_1: u8 = 0u8;
    let mut u8_2: u8 = 31u8;
    let mut i64_0: i64 = 3i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut i32_0: i32 = 336i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = std::ops::Add::add(date_0, duration_1);
    let mut i64_1: i64 = 60i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut i128_0: i128 = 1000000000i128;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i32_1: i32 = 274i32;
    let mut i64_2: i64 = 2440588i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_4, i32_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_5);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_milli(date_1, u8_2, u8_1, u8_0, u16_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_636() {
//    rusty_monitor::set_test_id(636);
    let mut i32_0: i32 = 55i32;
    let mut f64_0: f64 = 4741671816366391296.000000f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_0);
    let mut u32_0: u32 = 100000000u32;
    let mut u8_0: u8 = 11u8;
    let mut u8_1: u8 = 23u8;
    let mut u8_2: u8 = 0u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_0: i8 = 127i8;
    let mut i8_1: i8 = 60i8;
    let mut i8_2: i8 = 5i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_1);
    let mut i64_0: i64 = 60i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i32_1: i32 = 25i32;
    let mut i64_1: i64 = 2i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_1);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_3, duration_2);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut i32_2: i32 = 25i32;
    let mut i64_2: i64 = 253402300799i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_2);
    let mut duration_6: std::time::Duration = crate::duration::Duration::abs_std(duration_5);
    let mut i32_3: i32 = 0i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut date_2: crate::date::Date = std::ops::Sub::sub(date_1, duration_6);
    let mut tuple_0: (i32, u16) = crate::primitive_date_time::PrimitiveDateTime::to_ordinal_date(primitivedatetime_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_515() {
//    rusty_monitor::set_test_id(515);
    let mut i32_0: i32 = 314i32;
    let mut i64_0: i64 = 24i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i32_1: i32 = 100i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_1: crate::date::Date = std::ops::Sub::sub(date_0, duration_1);
    let mut i8_0: i8 = 5i8;
    let mut i8_1: i8 = 60i8;
    let mut i8_2: i8 = 4i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i128_0: i128 = 1i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i64_1: i64 = 44i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut i32_2: i32 = 3i32;
    let mut i64_2: i64 = 2147483647i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_2);
    let mut i64_3: i64 = 24i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut f64_0: f64 = 4741671816366391296.000000f64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::abs(duration_7);
    let mut i64_4: i64 = 1000i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::microseconds(i64_4);
    let mut duration_10: std::time::Duration = crate::duration::Duration::abs_std(duration_9);
    let mut i32_3: i32 = 212i32;
    let mut i32_4: i32 = 268i32;
    let mut i64_5: i64 = 2440588i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::new(i64_5, i32_4);
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_11, i32_3);
    let mut duration_13: std::time::Duration = crate::duration::Duration::abs_std(duration_12);
    let mut u32_0: u32 = 100u32;
    let mut u8_0: u8 = 5u8;
    let mut u8_1: u8 = 10u8;
    let mut u8_2: u8 = 52u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_3: i8 = 6i8;
    let mut i8_4: i8 = 80i8;
    let mut i8_5: i8 = 2i8;
    let mut u16_0: u16 = 59u16;
    let mut i32_5: i32 = 86399i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_5, u16_0);
    let mut i32_6: i32 = 314i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_6};
    let mut i32_7: i32 = 314i32;
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_7};
    let mut month_0: month::Month = crate::date::Date::month(date_4);
    let mut month_1: month::Month = crate::date::Date::month(date_3);
    let mut month_2: month::Month = crate::date::Date::month(date_2);
    let mut month_3: month::Month = crate::date::Date::month(date_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3090() {
//    rusty_monitor::set_test_id(3090);
    let mut i64_0: i64 = 3600i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut i128_0: i128 = 1000000i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i32_0: i32 = 16i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = std::ops::Sub::sub(date_0, duration_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_1);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i64_1: i64 = 1i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut i32_1: i32 = 139i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_3, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_1);
    let mut weekday_0: weekday::Weekday = crate::primitive_date_time::PrimitiveDateTime::weekday(primitivedatetime_1);
    let mut i32_2: i32 = 139i32;
    let mut i64_2: i64 = -38i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_4, i32_2);
    let mut u16_0: u16 = 7u16;
    let mut i32_3: i32 = 105i32;
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_0);
    let mut i8_0: i8 = -28i8;
    let mut i8_1: i8 = 0i8;
    let mut i8_2: i8 = 4i8;
    let mut i32_4: i32 = 0i32;
    let mut i64_3: i64 = 253402300799i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_6, i32_4);
    let mut i32_5: i32 = 291i32;
    let mut date_5: crate::date::Date = crate::date::Date {value: i32_5};
    let mut date_6: crate::date::Date = std::ops::Sub::sub(date_5, duration_7);
    let mut i8_3: i8 = 5i8;
    let mut i8_4: i8 = 23i8;
    let mut i8_5: i8 = 127i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u32_0: u32 = 100000000u32;
    let mut u8_0: u8 = 31u8;
    let mut u8_1: u8 = 53u8;
    let mut u8_2: u8 = 7u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_4: i64 = 86400i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::seconds(i64_4);
    let mut i32_6: i32 = -31i32;
    let mut date_7: crate::date::Date = crate::date::Date {value: i32_6};
    let mut date_8: crate::date::Date = crate::date::Date::saturating_sub(date_7, duration_8);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_8, time: time_1};
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_2, utcoffset_0);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_2, date_6);
    let mut weekday_1: weekday::Weekday = crate::offset_date_time::OffsetDateTime::weekday(offsetdatetime_3);
    let mut f32_0: f32 = -244.024200f32;
    let mut u8_3: u8 = 9u8;
    let mut month_0: month::Month = crate::month::Month::November;
    let mut i32_7: i32 = 0i32;
    let mut i64_5: i64 = 9223372036854775807i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_5);
    let mut duration_10: std::time::Duration = crate::duration::Duration::abs_std(duration_9);
    let mut i8_6: i8 = 6i8;
    let mut i8_7: i8 = 24i8;
    let mut i8_8: i8 = 59i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i8_9: i8 = 23i8;
    let mut i8_10: i8 = 24i8;
    let mut i8_11: i8 = 2i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut u16_1: u16 = 366u16;
    let mut i32_8: i32 = 16i32;
    let mut date_9: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_8, u16_1);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_9);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_3, utcoffset_2);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_4, offset: utcoffset_1};
    let mut time_2: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_4);
    let mut i128_1: i128 = -52i128;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::abs(duration_11);
    let mut duration_13: std::time::Duration = crate::duration::Duration::abs_std(duration_12);
    let mut i64_6: i64 = 0i64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::minutes(i64_6);
    let mut i32_9: i32 = 20i32;
    let mut date_10: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_9);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_7: i64 = crate::duration::Duration::whole_days(duration_14);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_calendar_date(i32_7, month_0, u8_3);
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut result_1: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_2, i8_1, i8_0);
    let mut date_11: crate::date::Date = std::ops::Sub::sub(date_4, duration_5);
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2711() {
//    rusty_monitor::set_test_id(2711);
    let mut i128_0: i128 = 1i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut i32_0: i32 = 140i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = std::ops::Add::add(date_0, duration_2);
    let mut i32_1: i32 = 0i32;
    let mut i64_0: i64 = 253402300799i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_3, i32_1);
    let mut i32_2: i32 = 291i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_3: crate::date::Date = std::ops::Sub::sub(date_2, duration_4);
    let mut i8_0: i8 = 5i8;
    let mut i8_1: i8 = 23i8;
    let mut i8_2: i8 = 127i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 100000000u32;
    let mut u8_0: u8 = 31u8;
    let mut u8_1: u8 = 53u8;
    let mut u8_2: u8 = 7u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_1: i64 = 86400i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut i32_3: i32 = -31i32;
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_3};
    let mut date_5: crate::date::Date = crate::date::Date::saturating_sub(date_4, duration_5);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_5, time: time_0};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_0, utcoffset_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_3);
    let mut weekday_0: weekday::Weekday = crate::offset_date_time::OffsetDateTime::weekday(offsetdatetime_1);
    let mut month_0: month::Month = crate::month::Month::November;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_446() {
//    rusty_monitor::set_test_id(446);
    let mut u32_0: u32 = 10u32;
    let mut u8_0: u8 = 95u8;
    let mut u8_1: u8 = 8u8;
    let mut u8_2: u8 = 76u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i8_0: i8 = 3i8;
    let mut i8_1: i8 = 127i8;
    let mut i8_2: i8 = 0i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = std::ops::Sub::sub(date_0, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut month_0: month::Month = crate::primitive_date_time::PrimitiveDateTime::month(primitivedatetime_1);
    let mut month_1: month::Month = crate::month::Month::July;
    let mut month_2: month::Month = crate::month::Month::April;
    let mut month_3: month::Month = crate::month::Month::September;
    let mut month_4: month::Month = crate::month::Month::September;
    let mut month_5: month::Month = crate::month::Month::January;
    let mut month_6: month::Month = crate::month::Month::September;
    let mut month_7: month::Month = crate::month::Month::next(month_6);
    let mut month_8: month::Month = crate::month::Month::July;
    let mut month_9: month::Month = crate::month::Month::November;
    let mut month_10: month::Month = crate::month::Month::next(month_9);
    let mut month_11: month::Month = crate::month::Month::next(month_8);
    let mut month_12: month::Month = crate::month::Month::next(month_7);
    let mut month_13: month::Month = crate::month::Month::next(month_5);
    let mut month_14: month::Month = crate::month::Month::next(month_4);
    let mut month_15: month::Month = crate::month::Month::next(month_3);
    let mut month_16: month::Month = crate::month::Month::next(month_2);
    let mut month_17: month::Month = crate::month::Month::next(month_1);
    let mut month_18: month::Month = crate::month::Month::next(month_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_415() {
//    rusty_monitor::set_test_id(415);
    let mut i8_0: i8 = 91i8;
    let mut i8_1: i8 = -33i8;
    let mut i8_2: i8 = 1i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_0: i64 = 2440588i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut i128_0: i128 = 1000i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_1, duration_0);
    let mut i32_0: i32 = 3i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = std::ops::Add::add(date_0, duration_2);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    let mut weekday_1_ref_0: &weekday::Weekday = &mut weekday_1;
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::Sunday;
    let mut weekday_2_ref_0: &weekday::Weekday = &mut weekday_2;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_359() {
//    rusty_monitor::set_test_id(359);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut i32_0: i32 = -84i32;
    let mut i128_0: i128 = 1i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut i32_1: i32 = 88i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_1: crate::date::Date = std::ops::Add::add(date_0, duration_2);
    let mut weekday_1: weekday::Weekday = crate::date::Date::weekday(date_1);
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::next(weekday_1);
    let mut i32_2: i32 = 218i32;
    let mut i64_0: i64 = 604800i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_2);
    let mut i64_1: i64 = 86400i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::abs(duration_4);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_5);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i64_2: i64 = 24i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut i8_0: i8 = 1i8;
    let mut i8_1: i8 = 23i8;
    let mut i8_2: i8 = 3i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = 24i8;
    let mut i8_4: i8 = 24i8;
    let mut i8_5: i8 = 3i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut weekday_3: weekday::Weekday = crate::weekday::Weekday::previous(weekday_2);
    let mut weekday_4: weekday::Weekday = crate::weekday::Weekday::previous(weekday_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4054() {
//    rusty_monitor::set_test_id(4054);
    let mut u32_0: u32 = 999999999u32;
    let mut u8_0: u8 = 28u8;
    let mut u8_1: u8 = 9u8;
    let mut u8_2: u8 = 94u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_0_ref_0: &crate::date::Date = &mut date_0;
    let mut u16_0: u16 = 999u16;
    let mut u8_3: u8 = 86u8;
    let mut u8_4: u8 = 0u8;
    let mut u8_5: u8 = 31u8;
    let mut i64_0: i64 = 3i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut i32_0: i32 = 336i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_2: crate::date::Date = std::ops::Add::add(date_1, duration_1);
    let mut i8_0: i8 = 60i8;
    let mut i8_1: i8 = 2i8;
    let mut i8_2: i8 = 4i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_1: u32 = 10000000u32;
    let mut u8_6: u8 = 4u8;
    let mut u8_7: u8 = 52u8;
    let mut u8_8: u8 = 29u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_1);
    let mut i64_1: i64 = 60i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut i128_0: i128 = 1000000000i128;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i32_1: i32 = 274i32;
    let mut i64_2: i64 = 2440588i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_4, i32_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_5);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_3);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_milli(date_2, u8_5, u8_4, u8_3, u16_0);
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(date_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_548() {
//    rusty_monitor::set_test_id(548);
    let mut i64_0: i64 = 253402300799i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut u16_0: u16 = 60u16;
    let mut i32_0: i32 = 54i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = std::ops::Sub::sub(date_0, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_0_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_0;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3289() {
//    rusty_monitor::set_test_id(3289);
    let mut i32_0: i32 = 348i32;
    let mut i64_0: i64 = 253402300799i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i32_1: i32 = 60i32;
    let mut i64_1: i64 = 1000000i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_2);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = std::ops::Sub::sub(date_0, duration_1);
    let mut i8_0: i8 = 60i8;
    let mut i8_1: i8 = 5i8;
    let mut i8_2: i8 = -4i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_2: i32 = 150i32;
    let mut i64_2: i64 = 9223372036854775807i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_2);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5468() {
//    rusty_monitor::set_test_id(5468);
    let mut i32_0: i32 = 16i32;
    let mut i64_0: i64 = 1i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut u16_0: u16 = 366u16;
    let mut i32_1: i32 = 2i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_1: crate::date::Date = std::ops::Sub::sub(date_0, duration_0);
    let mut i8_0: i8 = 59i8;
    let mut i8_1: i8 = 23i8;
    let mut i8_2: i8 = 60i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_1: i64 = -60i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut i32_2: i32 = 48i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut date_3: crate::date::Date = std::ops::Sub::sub(date_2, duration_1);
    let mut u32_0: u32 = 59u32;
    let mut u8_0: u8 = 28u8;
    let mut u8_1: u8 = 59u8;
    let mut u8_2: u8 = 7u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_3: i32 = 9i32;
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_3};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_4, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_1);
    let mut i128_0: i128 = 1000i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i32_4: i32 = 20i32;
    let mut date_5: crate::date::Date = crate::date::Date {value: i32_4};
    let mut date_6: crate::date::Date = crate::date::Date::saturating_add(date_5, duration_2);
    let mut i64_2: i64 = 24i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u16_1: u16 = 1u16;
    let mut i32_5: i32 = 400i32;
    let mut date_7: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_5, u16_1);
    let mut i32_6: i32 = crate::offset_date_time::OffsetDateTime::year(offsetdatetime_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4977() {
//    rusty_monitor::set_test_id(4977);
    let mut u32_0: u32 = 100u32;
    let mut u8_0: u8 = 59u8;
    let mut u8_1: u8 = 51u8;
    let mut u8_2: u8 = 24u8;
    let mut u16_0: u16 = 999u16;
    let mut i32_0: i32 = 156i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut u32_1: u32 = 12u32;
    let mut u8_3: u8 = 8u8;
    let mut u8_4: u8 = 58u8;
    let mut u8_5: u8 = 24u8;
    let mut i128_0: i128 = -46i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i32_1: i32 = 122i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_2: crate::date::Date = std::ops::Sub::sub(date_1, duration_1);
    let mut u32_2: u32 = 10000000u32;
    let mut u8_6: u8 = 3u8;
    let mut u8_7: u8 = 6u8;
    let mut u8_8: u8 = 0u8;
    let mut i32_2: i32 = 201i32;
    let mut i64_0: i64 = 2440588i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_2);
    let mut i64_1: i64 = 1000000i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_3, duration_2);
    let mut i32_3: i32 = 5i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_3};
    let mut date_4: crate::date::Date = std::ops::Sub::sub(date_3, duration_4);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_4, u8_8, u8_7, u8_6, u32_2);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_2, u8_5, u8_4, u8_3, u32_1);
    let mut result_2: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_0, u8_2, u8_1, u8_0, u32_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_147() {
//    rusty_monitor::set_test_id(147);
    let mut u32_0: u32 = 1000000000u32;
    let mut u8_0: u8 = 4u8;
    let mut u8_1: u8 = 52u8;
    let mut u8_2: u8 = 28u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 336i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i64_0: i64 = 2440588i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut i8_0: i8 = 60i8;
    let mut i8_1: i8 = 3i8;
    let mut i8_2: i8 = 1i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut i8_3: i8 = 59i8;
    let mut i8_4: i8 = 4i8;
    let mut i8_5: i8 = 3i8;
    let mut i64_1: i64 = 1000000i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut u16_0: u16 = 999u16;
    let mut i32_1: i32 = 9i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_2: crate::date::Date = std::ops::Add::add(date_1, duration_4);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut date_3: crate::date::Date = crate::primitive_date_time::PrimitiveDateTime::date(primitivedatetime_1);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut option_0: std::option::Option<crate::offset_date_time::OffsetDateTime> = crate::offset_date_time::OffsetDateTime::checked_sub(offsetdatetime_1, duration_2);
    let mut u8_3: u8 = crate::primitive_date_time::PrimitiveDateTime::sunday_based_week(primitivedatetime_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2067() {
//    rusty_monitor::set_test_id(2067);
    let mut u32_0: u32 = 100u32;
    let mut u8_0: u8 = 59u8;
    let mut u8_1: u8 = 51u8;
    let mut u8_2: u8 = 24u8;
    let mut u16_0: u16 = 999u16;
    let mut i32_0: i32 = 156i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut i128_0: i128 = -46i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i32_1: i32 = 201i32;
    let mut i64_0: i64 = 2440588i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_1);
    let mut i64_1: i64 = 1000000i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_3, duration_2);
    let mut i32_2: i32 = 5i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_2: crate::date::Date = std::ops::Sub::sub(date_1, duration_4);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_0, u8_2, u8_1, u8_0, u32_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_173() {
//    rusty_monitor::set_test_id(173);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i64_0: i64 = 12i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut u16_0: u16 = 0u16;
    let mut i32_0: i32 = 20i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = std::ops::Sub::sub(date_0, duration_3);
    let mut date_1_ref_0: &mut crate::date::Date = &mut date_1;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4528() {
//    rusty_monitor::set_test_id(4528);
    let mut u32_0: u32 = 100u32;
    let mut u8_0: u8 = 59u8;
    let mut u8_1: u8 = 51u8;
    let mut u8_2: u8 = 24u8;
    let mut u16_0: u16 = 999u16;
    let mut i32_0: i32 = 156i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut i128_0: i128 = -46i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i32_1: i32 = 122i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut u32_1: u32 = 10000000u32;
    let mut u8_3: u8 = 92u8;
    let mut u8_4: u8 = 6u8;
    let mut u8_5: u8 = 0u8;
    let mut i32_2: i32 = 201i32;
    let mut i64_0: i64 = 2440588i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_2);
    let mut i64_1: i64 = 1000000i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_2, duration_1);
    let mut i32_3: i32 = 5i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_3};
    let mut date_3: crate::date::Date = std::ops::Sub::sub(date_2, duration_3);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_3, u8_5, u8_4, u8_3, u32_1);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_0, u8_2, u8_1, u8_0, u32_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1661() {
//    rusty_monitor::set_test_id(1661);
    let mut u16_0: u16 = 366u16;
    let mut i32_0: i32 = 2i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut i64_0: i64 = -60i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i32_1: i32 = 48i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_2: crate::date::Date = std::ops::Sub::sub(date_1, duration_0);
    let mut u32_0: u32 = 59u32;
    let mut u8_0: u8 = 28u8;
    let mut u8_1: u8 = 59u8;
    let mut u8_2: u8 = 7u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_2: i32 = 9i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_2};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_3, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_2);
    let mut i128_0: i128 = 1000i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i32_3: i32 = 20i32;
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_3};
    let mut date_5: crate::date::Date = crate::date::Date::saturating_add(date_4, duration_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1621() {
//    rusty_monitor::set_test_id(1621);
    let mut i32_0: i32 = 303i32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_0);
    let mut u16_0: u16 = 366u16;
    let mut i32_1: i32 = -286i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_1: crate::date::Date = std::ops::Add::add(date_0, duration_1);
    let mut date_1_ref_0: &crate::date::Date = &mut date_1;
    let mut i64_0: i64 = 24i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_2);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i128_0: i128 = 9223372036854775807i128;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i64_1: i64 = 2147483647i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut i8_0: i8 = 2i8;
    let mut i8_1: i8 = 1i8;
    let mut i8_2: i8 = 5i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i128_1: i128 = 121i128;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut i32_2: i32 = 172i32;
    let mut i64_2: i64 = 604800i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_2);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_6, duration_5);
    let mut i8_3: i8 = 23i8;
    let mut i8_4: i8 = 23i8;
    let mut i8_5: i8 = 119i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut date_2: crate::date::Date = std::clone::Clone::clone(date_1_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_371() {
//    rusty_monitor::set_test_id(371);
    let mut i64_0: i64 = 2147483647i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i32_0: i32 = 342i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = std::ops::Sub::sub(date_0, duration_0);
    let mut i32_1: i32 = 91i32;
    let mut i64_1: i64 = -56i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_1);
    let mut u16_0: u16 = 59u16;
    let mut i32_2: i32 = 364i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut date_3: crate::date::Date = std::ops::Sub::sub(date_2, duration_1);
    let mut i8_0: i8 = -32i8;
    let mut i8_1: i8 = 4i8;
    let mut i8_2: i8 = 3i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut f32_0: f32 = 1065353216.000000f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i32_3: i32 = 252i32;
    let mut i64_2: i64 = 86400i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_3);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i64_3: i64 = 3600i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_4);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_3);
    let mut f64_0: f64 = 4652007308841189376.000000f64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut tuple_0: (i32, month::Month, u8) = crate::date::Date::to_calendar_date(date_3);
    let mut tuple_1: (i32, month::Month, u8) = crate::date::Date::to_calendar_date(date_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_370() {
//    rusty_monitor::set_test_id(370);
    let mut i32_0: i32 = 1000i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut i32_1: i32 = 98i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut duration_0: crate::duration::Duration = std::ops::Sub::sub(date_1, date_0);
    let mut duration_0_ref_0: &crate::duration::Duration = &mut duration_0;
    let mut i128_0: i128 = 1000000000i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_1_ref_0: &crate::duration::Duration = &mut duration_1;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3172() {
//    rusty_monitor::set_test_id(3172);
    let mut i64_0: i64 = 59i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut f32_0: f32 = -226.006618f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut i8_0: i8 = 1i8;
    let mut i8_1: i8 = 127i8;
    let mut i8_2: i8 = 4i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = std::ops::Add::add(date_0, duration_2);
//    panic!("From RustyUnit with love");
}
}