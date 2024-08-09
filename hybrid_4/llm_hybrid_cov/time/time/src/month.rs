//! The `Month` enum and its associated `impl`s.
use core::fmt;
use core::num::NonZeroU8;
use core::str::FromStr;
use self::Month::*;
use crate::error;
/// Months of the year.
#[allow(clippy::missing_docs_in_private_items)]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Month {
    January = 1,
    February = 2,
    March = 3,
    April = 4,
    May = 5,
    June = 6,
    July = 7,
    August = 8,
    September = 9,
    October = 10,
    November = 11,
    December = 12,
}
impl Month {
    /// Create a `Month` from its numerical value.
    pub(crate) const fn from_number(
        n: NonZeroU8,
    ) -> Result<Self, error::ComponentRange> {
        match n.get() {
            1 => Ok(January),
            2 => Ok(February),
            3 => Ok(March),
            4 => Ok(April),
            5 => Ok(May),
            6 => Ok(June),
            7 => Ok(July),
            8 => Ok(August),
            9 => Ok(September),
            10 => Ok(October),
            11 => Ok(November),
            12 => Ok(December),
            n => {
                Err(error::ComponentRange {
                    name: "month",
                    minimum: 1,
                    maximum: 12,
                    value: n as _,
                    conditional_range: false,
                })
            }
        }
    }
    /// Get the previous month.
    ///
    /// ```rust
    /// # use time::Month;
    /// assert_eq!(Month::January.previous(), Month::December);
    /// ```
    pub const fn previous(self) -> Self {
        match self {
            January => December,
            February => January,
            March => February,
            April => March,
            May => April,
            June => May,
            July => June,
            August => July,
            September => August,
            October => September,
            November => October,
            December => November,
        }
    }
    /// Get the next month.
    ///
    /// ```rust
    /// # use time::Month;
    /// assert_eq!(Month::January.next(), Month::February);
    /// ```
    pub const fn next(self) -> Self {
        match self {
            January => February,
            February => March,
            March => April,
            April => May,
            May => June,
            June => July,
            July => August,
            August => September,
            September => October,
            October => November,
            November => December,
            December => January,
        }
    }
}
impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(
            match self {
                January => "January",
                February => "February",
                March => "March",
                April => "April",
                May => "May",
                June => "June",
                July => "July",
                August => "August",
                September => "September",
                October => "October",
                November => "November",
                December => "December",
            },
        )
    }
}
impl FromStr for Month {
    type Err = error::InvalidVariant;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "January" => Ok(January),
            "February" => Ok(February),
            "March" => Ok(March),
            "April" => Ok(April),
            "May" => Ok(May),
            "June" => Ok(June),
            "July" => Ok(July),
            "August" => Ok(August),
            "September" => Ok(September),
            "October" => Ok(October),
            "November" => Ok(November),
            "December" => Ok(December),
            _ => Err(error::InvalidVariant),
        }
    }
}
impl From<Month> for u8 {
    fn from(month: Month) -> Self {
        month as _
    }
}
impl TryFrom<u8> for Month {
    type Error = error::ComponentRange;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match NonZeroU8::new(value) {
            Some(value) => Self::from_number(value),
            None => {
                Err(error::ComponentRange {
                    name: "month",
                    minimum: 1,
                    maximum: 12,
                    value: 0,
                    conditional_range: false,
                })
            }
        }
    }
}
#[cfg(test)]
mod tests_llm_16_121 {
    use super::*;
    use crate::*;
    use std::convert::TryFrom;
    #[test]
    fn test_try_from_valid_month() {
        let _rug_st_tests_llm_16_121_rrrruuuugggg_test_try_from_valid_month = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 12;
        for i in rug_fuzz_0..=rug_fuzz_1 {
            debug_assert!(Month::try_from(i).is_ok());
        }
        let _rug_ed_tests_llm_16_121_rrrruuuugggg_test_try_from_valid_month = 0;
    }
    #[test]
    fn test_try_from_invalid_month() {
        let _rug_st_tests_llm_16_121_rrrruuuugggg_test_try_from_invalid_month = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 13;
        for i in rug_fuzz_0..u8::MIN {
            debug_assert!(Month::try_from(i).is_err());
        }
        for i in rug_fuzz_1..u8::MAX {
            debug_assert!(Month::try_from(i).is_err());
        }
        let _rug_ed_tests_llm_16_121_rrrruuuugggg_test_try_from_invalid_month = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_122 {
    use super::*;
    use crate::*;
    use std::str::FromStr;
    #[test]
    fn test_from_str_valid() {
        let _rug_st_tests_llm_16_122_rrrruuuugggg_test_from_str_valid = 0;
        let rug_fuzz_0 = "January";
        let rug_fuzz_1 = "February";
        let rug_fuzz_2 = "March";
        let rug_fuzz_3 = "April";
        let rug_fuzz_4 = "May";
        let rug_fuzz_5 = "June";
        let rug_fuzz_6 = "July";
        let rug_fuzz_7 = "August";
        let rug_fuzz_8 = "September";
        let rug_fuzz_9 = "October";
        let rug_fuzz_10 = "November";
        let rug_fuzz_11 = "December";
        debug_assert_eq!(Month::from_str(rug_fuzz_0).unwrap(), Month::January);
        debug_assert_eq!(Month::from_str(rug_fuzz_1).unwrap(), Month::February);
        debug_assert_eq!(Month::from_str(rug_fuzz_2).unwrap(), Month::March);
        debug_assert_eq!(Month::from_str(rug_fuzz_3).unwrap(), Month::April);
        debug_assert_eq!(Month::from_str(rug_fuzz_4).unwrap(), Month::May);
        debug_assert_eq!(Month::from_str(rug_fuzz_5).unwrap(), Month::June);
        debug_assert_eq!(Month::from_str(rug_fuzz_6).unwrap(), Month::July);
        debug_assert_eq!(Month::from_str(rug_fuzz_7).unwrap(), Month::August);
        debug_assert_eq!(Month::from_str(rug_fuzz_8).unwrap(), Month::September);
        debug_assert_eq!(Month::from_str(rug_fuzz_9).unwrap(), Month::October);
        debug_assert_eq!(Month::from_str(rug_fuzz_10).unwrap(), Month::November);
        debug_assert_eq!(Month::from_str(rug_fuzz_11).unwrap(), Month::December);
        let _rug_ed_tests_llm_16_122_rrrruuuugggg_test_from_str_valid = 0;
    }
    #[test]
    fn test_from_str_invalid() {
        let _rug_st_tests_llm_16_122_rrrruuuugggg_test_from_str_invalid = 0;
        let rug_fuzz_0 = "NotAMonth";
        debug_assert!(Month::from_str(rug_fuzz_0).is_err());
        let _rug_ed_tests_llm_16_122_rrrruuuugggg_test_from_str_invalid = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_346 {
    use crate::Month;
    use std::convert::From;
    #[test]
    fn from_january() {
        let _rug_st_tests_llm_16_346_rrrruuuugggg_from_january = 0;
        let month_u8 = u8::from(Month::January);
        debug_assert_eq!(month_u8, 1);
        let _rug_ed_tests_llm_16_346_rrrruuuugggg_from_january = 0;
    }
    #[test]
    fn from_february() {
        let _rug_st_tests_llm_16_346_rrrruuuugggg_from_february = 0;
        let month_u8 = u8::from(Month::February);
        debug_assert_eq!(month_u8, 2);
        let _rug_ed_tests_llm_16_346_rrrruuuugggg_from_february = 0;
    }
    #[test]
    fn from_march() {
        let _rug_st_tests_llm_16_346_rrrruuuugggg_from_march = 0;
        let month_u8 = u8::from(Month::March);
        debug_assert_eq!(month_u8, 3);
        let _rug_ed_tests_llm_16_346_rrrruuuugggg_from_march = 0;
    }
    #[test]
    fn from_april() {
        let _rug_st_tests_llm_16_346_rrrruuuugggg_from_april = 0;
        let month_u8 = u8::from(Month::April);
        debug_assert_eq!(month_u8, 4);
        let _rug_ed_tests_llm_16_346_rrrruuuugggg_from_april = 0;
    }
    #[test]
    fn from_may() {
        let _rug_st_tests_llm_16_346_rrrruuuugggg_from_may = 0;
        let month_u8 = u8::from(Month::May);
        debug_assert_eq!(month_u8, 5);
        let _rug_ed_tests_llm_16_346_rrrruuuugggg_from_may = 0;
    }
    #[test]
    fn from_june() {
        let _rug_st_tests_llm_16_346_rrrruuuugggg_from_june = 0;
        let month_u8 = u8::from(Month::June);
        debug_assert_eq!(month_u8, 6);
        let _rug_ed_tests_llm_16_346_rrrruuuugggg_from_june = 0;
    }
    #[test]
    fn from_july() {
        let _rug_st_tests_llm_16_346_rrrruuuugggg_from_july = 0;
        let month_u8 = u8::from(Month::July);
        debug_assert_eq!(month_u8, 7);
        let _rug_ed_tests_llm_16_346_rrrruuuugggg_from_july = 0;
    }
    #[test]
    fn from_august() {
        let _rug_st_tests_llm_16_346_rrrruuuugggg_from_august = 0;
        let month_u8 = u8::from(Month::August);
        debug_assert_eq!(month_u8, 8);
        let _rug_ed_tests_llm_16_346_rrrruuuugggg_from_august = 0;
    }
    #[test]
    fn from_september() {
        let _rug_st_tests_llm_16_346_rrrruuuugggg_from_september = 0;
        let month_u8 = u8::from(Month::September);
        debug_assert_eq!(month_u8, 9);
        let _rug_ed_tests_llm_16_346_rrrruuuugggg_from_september = 0;
    }
    #[test]
    fn from_october() {
        let _rug_st_tests_llm_16_346_rrrruuuugggg_from_october = 0;
        let month_u8 = u8::from(Month::October);
        debug_assert_eq!(month_u8, 10);
        let _rug_ed_tests_llm_16_346_rrrruuuugggg_from_october = 0;
    }
    #[test]
    fn from_november() {
        let _rug_st_tests_llm_16_346_rrrruuuugggg_from_november = 0;
        let month_u8 = u8::from(Month::November);
        debug_assert_eq!(month_u8, 11);
        let _rug_ed_tests_llm_16_346_rrrruuuugggg_from_november = 0;
    }
    #[test]
    fn from_december() {
        let _rug_st_tests_llm_16_346_rrrruuuugggg_from_december = 0;
        let month_u8 = u8::from(Month::December);
        debug_assert_eq!(month_u8, 12);
        let _rug_ed_tests_llm_16_346_rrrruuuugggg_from_december = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_347 {
    use super::*;
    use crate::*;
    use crate::error::ComponentRange;
    use core::num::NonZeroU8;
    use crate::Month::*;
    use crate::month;
    #[test]
    fn test_from_number_valid() {
        let _rug_st_tests_llm_16_347_rrrruuuugggg_test_from_number_valid = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 5;
        let rug_fuzz_5 = 6;
        let rug_fuzz_6 = 7;
        let rug_fuzz_7 = 8;
        let rug_fuzz_8 = 9;
        let rug_fuzz_9 = 10;
        let rug_fuzz_10 = 11;
        let rug_fuzz_11 = 12;
        debug_assert_eq!(
            month::Month::from_number(NonZeroU8::new(rug_fuzz_0).unwrap()), Ok(January)
        );
        debug_assert_eq!(
            month::Month::from_number(NonZeroU8::new(rug_fuzz_1).unwrap()), Ok(February)
        );
        debug_assert_eq!(
            month::Month::from_number(NonZeroU8::new(rug_fuzz_2).unwrap()), Ok(March)
        );
        debug_assert_eq!(
            month::Month::from_number(NonZeroU8::new(rug_fuzz_3).unwrap()), Ok(April)
        );
        debug_assert_eq!(
            month::Month::from_number(NonZeroU8::new(rug_fuzz_4).unwrap()), Ok(May)
        );
        debug_assert_eq!(
            month::Month::from_number(NonZeroU8::new(rug_fuzz_5).unwrap()), Ok(June)
        );
        debug_assert_eq!(
            month::Month::from_number(NonZeroU8::new(rug_fuzz_6).unwrap()), Ok(July)
        );
        debug_assert_eq!(
            month::Month::from_number(NonZeroU8::new(rug_fuzz_7).unwrap()), Ok(August)
        );
        debug_assert_eq!(
            month::Month::from_number(NonZeroU8::new(rug_fuzz_8).unwrap()), Ok(September)
        );
        debug_assert_eq!(
            month::Month::from_number(NonZeroU8::new(rug_fuzz_9).unwrap()), Ok(October)
        );
        debug_assert_eq!(
            month::Month::from_number(NonZeroU8::new(rug_fuzz_10).unwrap()), Ok(November)
        );
        debug_assert_eq!(
            month::Month::from_number(NonZeroU8::new(rug_fuzz_11).unwrap()), Ok(December)
        );
        let _rug_ed_tests_llm_16_347_rrrruuuugggg_test_from_number_valid = 0;
    }
    #[test]
    fn test_from_number_invalid() {
        let _rug_st_tests_llm_16_347_rrrruuuugggg_test_from_number_invalid = 0;
        let rug_fuzz_0 = 0_u8;
        let rug_fuzz_1 = 13;
        let rug_fuzz_2 = 255;
        let rug_fuzz_3 = 0;
        for value in [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2] {
            match NonZeroU8::new(value) {
                Some(non_zero_value) => {
                    debug_assert!(
                        matches!(month::Month::from_number(non_zero_value),
                        Err(ComponentRange { name, minimum, maximum, value : _,
                        conditional_range : false }) if name == "month" && minimum == 1
                        && maximum == 12)
                    );
                }
                None => {
                    debug_assert!(value == rug_fuzz_3, "Invalid test value: {}", value)
                }
            }
        }
        let _rug_ed_tests_llm_16_347_rrrruuuugggg_test_from_number_invalid = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_348 {
    use crate::Month::*;
    use crate::month::Month;
    #[test]
    fn test_next_month() {
        let _rug_st_tests_llm_16_348_rrrruuuugggg_test_next_month = 0;
        debug_assert_eq!(January.next(), February);
        debug_assert_eq!(February.next(), March);
        debug_assert_eq!(March.next(), April);
        debug_assert_eq!(April.next(), May);
        debug_assert_eq!(May.next(), June);
        debug_assert_eq!(June.next(), July);
        debug_assert_eq!(July.next(), August);
        debug_assert_eq!(August.next(), September);
        debug_assert_eq!(September.next(), October);
        debug_assert_eq!(October.next(), November);
        debug_assert_eq!(November.next(), December);
        debug_assert_eq!(December.next(), January);
        let _rug_ed_tests_llm_16_348_rrrruuuugggg_test_next_month = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_349 {
    use crate::Month;
    #[test]
    fn previous_month() {
        let _rug_st_tests_llm_16_349_rrrruuuugggg_previous_month = 0;
        debug_assert_eq!(Month::January.previous(), Month::December);
        debug_assert_eq!(Month::February.previous(), Month::January);
        debug_assert_eq!(Month::March.previous(), Month::February);
        debug_assert_eq!(Month::April.previous(), Month::March);
        debug_assert_eq!(Month::May.previous(), Month::April);
        debug_assert_eq!(Month::June.previous(), Month::May);
        debug_assert_eq!(Month::July.previous(), Month::June);
        debug_assert_eq!(Month::August.previous(), Month::July);
        debug_assert_eq!(Month::September.previous(), Month::August);
        debug_assert_eq!(Month::October.previous(), Month::September);
        debug_assert_eq!(Month::November.previous(), Month::October);
        debug_assert_eq!(Month::December.previous(), Month::November);
        let _rug_ed_tests_llm_16_349_rrrruuuugggg_previous_month = 0;
    }
}
