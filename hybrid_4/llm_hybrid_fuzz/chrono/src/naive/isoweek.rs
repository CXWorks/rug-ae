//! ISO 8601 week.
use core::fmt;
use super::internals::{DateImpl, Of, YearFlags};
#[cfg(feature = "rkyv")]
use rkyv::{Archive, Deserialize, Serialize};
/// ISO 8601 week.
///
/// This type, combined with [`Weekday`](../enum.Weekday.html),
/// constitutes the ISO 8601 [week date](./struct.NaiveDate.html#week-date).
/// One can retrieve this type from the existing [`Datelike`](../trait.Datelike.html) types
/// via the [`Datelike::iso_week`](../trait.Datelike.html#tymethod.iso_week) method.
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
#[cfg_attr(feature = "rkyv", derive(Archive, Deserialize, Serialize))]
pub struct IsoWeek {
    ywf: DateImpl,
}
/// Returns the corresponding `IsoWeek` from the year and the `Of` internal value.
pub(super) fn iso_week_from_yof(year: i32, of: Of) -> IsoWeek {
    let (rawweek, _) = of.isoweekdate_raw();
    let (year, week) = if rawweek < 1 {
        let prevlastweek = YearFlags::from_year(year - 1).nisoweeks();
        (year - 1, prevlastweek)
    } else {
        let lastweek = of.flags().nisoweeks();
        if rawweek > lastweek { (year + 1, 1) } else { (year, rawweek) }
    };
    let flags = YearFlags::from_year(year);
    IsoWeek {
        ywf: (year << 10) | (week << 4) as DateImpl | DateImpl::from(flags.0),
    }
}
impl IsoWeek {
    /// Returns the year number for this ISO week.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, Datelike, Weekday};
    ///
    /// let d = NaiveDate::from_isoywd(2015, 1, Weekday::Mon);
    /// assert_eq!(d.iso_week().year(), 2015);
    /// ```
    ///
    /// This year number might not match the calendar year number.
    /// Continuing the example...
    ///
    /// ```
    /// # use chrono::{NaiveDate, Datelike, Weekday};
    /// # let d = NaiveDate::from_isoywd(2015, 1, Weekday::Mon);
    /// assert_eq!(d.year(), 2014);
    /// assert_eq!(d, NaiveDate::from_ymd_opt(2014, 12, 29).unwrap());
    /// ```
    #[inline]
    pub const fn year(&self) -> i32 {
        self.ywf >> 10
    }
    /// Returns the ISO week number starting from 1.
    ///
    /// The return value ranges from 1 to 53. (The last week of year differs by years.)
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, Datelike, Weekday};
    ///
    /// let d = NaiveDate::from_isoywd(2015, 15, Weekday::Mon);
    /// assert_eq!(d.iso_week().week(), 15);
    /// ```
    #[inline]
    pub const fn week(&self) -> u32 {
        ((self.ywf >> 4) & 0x3f) as u32
    }
    /// Returns the ISO week number starting from 0.
    ///
    /// The return value ranges from 0 to 52. (The last week of year differs by years.)
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, Datelike, Weekday};
    ///
    /// let d = NaiveDate::from_isoywd(2015, 15, Weekday::Mon);
    /// assert_eq!(d.iso_week().week0(), 14);
    /// ```
    #[inline]
    pub const fn week0(&self) -> u32 {
        ((self.ywf >> 4) & 0x3f) as u32 - 1
    }
}
/// The `Debug` output of the ISO week `w` is the same as
/// [`d.format("%G-W%V")`](../format/strftime/index.html)
/// where `d` is any `NaiveDate` value in that week.
///
/// # Example
///
/// ```
/// use chrono::{NaiveDate, Datelike};
///
/// assert_eq!(format!("{:?}", NaiveDate::from_ymd_opt(2015,  9,  5).unwrap().iso_week()), "2015-W36");
/// assert_eq!(format!("{:?}", NaiveDate::from_ymd_opt(   0,  1,  3).unwrap().iso_week()), "0000-W01");
/// assert_eq!(format!("{:?}", NaiveDate::from_ymd_opt(9999, 12, 31).unwrap().iso_week()), "9999-W52");
/// ```
///
/// ISO 8601 requires an explicit sign for years before 1 BCE or after 9999 CE.
///
/// ```
/// # use chrono::{NaiveDate, Datelike};
/// assert_eq!(format!("{:?}", NaiveDate::from_ymd_opt(    0,  1,  2).unwrap().iso_week()),  "-0001-W52");
/// assert_eq!(format!("{:?}", NaiveDate::from_ymd_opt(10000, 12, 31).unwrap().iso_week()), "+10000-W52");
/// ```
impl fmt::Debug for IsoWeek {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let year = self.year();
        let week = self.week();
        if (0..=9999).contains(&year) {
            write!(f, "{:04}-W{:02}", year, week)
        } else {
            write!(f, "{:+05}-W{:02}", year, week)
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::naive::{internals, NaiveDate};
    use crate::Datelike;
    #[test]
    fn test_iso_week_extremes() {
        let minweek = NaiveDate::MIN.iso_week();
        let maxweek = NaiveDate::MAX.iso_week();
        assert_eq!(minweek.year(), internals::MIN_YEAR);
        assert_eq!(minweek.week(), 1);
        assert_eq!(minweek.week0(), 0);
        assert_eq!(
            format!("{:?}", minweek), NaiveDate::MIN.format("%G-W%V").to_string()
        );
        assert_eq!(maxweek.year(), internals::MAX_YEAR + 1);
        assert_eq!(maxweek.week(), 1);
        assert_eq!(maxweek.week0(), 0);
        assert_eq!(
            format!("{:?}", maxweek), NaiveDate::MAX.format("%G-W%V").to_string()
        );
    }
    #[test]
    fn test_iso_week_equivalence_for_first_week() {
        let monday = NaiveDate::from_ymd_opt(2024, 12, 30).unwrap();
        let friday = NaiveDate::from_ymd_opt(2025, 1, 3).unwrap();
        assert_eq!(monday.iso_week(), friday.iso_week());
    }
    #[test]
    fn test_iso_week_equivalence_for_last_week() {
        let monday = NaiveDate::from_ymd_opt(2026, 12, 28).unwrap();
        let friday = NaiveDate::from_ymd_opt(2027, 1, 1).unwrap();
        assert_eq!(monday.iso_week(), friday.iso_week());
    }
    #[test]
    fn test_iso_week_ordering_for_first_week() {
        let monday = NaiveDate::from_ymd_opt(2024, 12, 30).unwrap();
        let friday = NaiveDate::from_ymd_opt(2025, 1, 3).unwrap();
        assert!(monday.iso_week() >= friday.iso_week());
        assert!(monday.iso_week() <= friday.iso_week());
    }
    #[test]
    fn test_iso_week_ordering_for_last_week() {
        let monday = NaiveDate::from_ymd_opt(2026, 12, 28).unwrap();
        let friday = NaiveDate::from_ymd_opt(2027, 1, 1).unwrap();
        assert!(monday.iso_week() >= friday.iso_week());
        assert!(monday.iso_week() <= friday.iso_week());
    }
}
#[cfg(test)]
mod tests_llm_16_456 {
    use crate::{Datelike, NaiveDate, Weekday};
    #[test]
    fn test_week() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let test_cases = vec![
            (rug_fuzz_0, rug_fuzz_1, Weekday::Mon, rug_fuzz_2), (2015, 52, Weekday::Sun,
            52), (2015, 53, Weekday::Sun, 53), (2014, 52, Weekday::Wed, 52), (2013, 1,
            Weekday::Tue, 1), (2013, 52, Weekday::Tue, 52), (2016, 53, Weekday::Sat, 53)
        ];
        for (year, week, weekday, expected) in test_cases {
            let date = NaiveDate::from_isoywd(year, week, weekday);
            let iso_week = date.iso_week().week();
            debug_assert_eq!(iso_week, expected);
        }
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_457_llm_16_457 {
    use crate::naive::{self, isoweek::IsoWeek};
    use crate::Datelike;
    fn new_iso_week(year: i32, week: u32) -> IsoWeek {
        let ywf = (year << 10) | ((week as i32) << 4);
        IsoWeek { ywf: ywf as i32 }
    }
    #[test]
    fn week0_test_week_1() {
        let week = new_iso_week(2023, 1);
        assert_eq!(week.week0(), 0);
    }
    #[test]
    fn week0_test_week_2() {
        let week = new_iso_week(2023, 2);
        assert_eq!(week.week0(), 1);
    }
    #[test]
    fn week0_test_week_52() {
        let week = new_iso_week(2023, 52);
        assert_eq!(week.week0(), 51);
    }
    #[test]
    fn week0_week_53_edge_case() {
        let week = new_iso_week(2023, 53);
        assert_eq!(week.week0(), 52);
    }
    #[test]
    fn week0_with_invalid_week() {
        let week = new_iso_week(2023, 54);
        assert_eq!(week.week0(), 53);
    }
}
#[cfg(test)]
mod tests_llm_16_458 {
    use super::*;
    use crate::*;
    #[test]
    fn test_iso_week_year() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let isoweek = IsoWeek {
            ywf: rug_fuzz_0 << rug_fuzz_1,
        };
        debug_assert_eq!(isoweek.year(), 2015);
        let isoweek = IsoWeek {
            ywf: rug_fuzz_2 << rug_fuzz_3,
        };
        debug_assert_eq!(isoweek.year(), 2014);
        let isoweek = IsoWeek {
            ywf: rug_fuzz_4 << rug_fuzz_5,
        };
        debug_assert_eq!(isoweek.year(), 1);
        let isoweek = IsoWeek {
            ywf: rug_fuzz_6 << rug_fuzz_7,
        };
        debug_assert_eq!(isoweek.year(), 0);
        let isoweek = IsoWeek {
            ywf: (-rug_fuzz_8) << rug_fuzz_9,
        };
        debug_assert_eq!(isoweek.year(), - 1);
        let isoweek = IsoWeek {
            ywf: (-rug_fuzz_10) << rug_fuzz_11,
        };
        debug_assert_eq!(isoweek.year(), - 2015);
             }
});    }
}
