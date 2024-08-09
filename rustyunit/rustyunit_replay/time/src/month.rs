//! The `Month` enum and its associated `impl`s.
use core::convert::TryFrom;
use core::fmt;
use core::num::NonZeroU8;
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
mod tests_llm_16_361 {
    use super::*;
    use crate::*;
    use std::convert::TryFrom;
    #[test]
    fn test_month_from() {
        let _rug_st_tests_llm_16_361_rrrruuuugggg_test_month_from = 0;
        debug_assert_eq!(u8::from(Month::January), 1);
        debug_assert_eq!(u8::from(Month::February), 2);
        debug_assert_eq!(u8::from(Month::March), 3);
        debug_assert_eq!(u8::from(Month::April), 4);
        debug_assert_eq!(u8::from(Month::May), 5);
        debug_assert_eq!(u8::from(Month::June), 6);
        debug_assert_eq!(u8::from(Month::July), 7);
        debug_assert_eq!(u8::from(Month::August), 8);
        debug_assert_eq!(u8::from(Month::September), 9);
        debug_assert_eq!(u8::from(Month::October), 10);
        debug_assert_eq!(u8::from(Month::November), 11);
        debug_assert_eq!(u8::from(Month::December), 12);
        let _rug_ed_tests_llm_16_361_rrrruuuugggg_test_month_from = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_363 {
    use crate::month::{Month, error::ComponentRange};
    use std::num::NonZeroU8;
    #[test]
    fn test_from_number() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13)) = <(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Month::from_number(NonZeroU8::new(rug_fuzz_0).unwrap()), Ok(Month::January)
        );
        debug_assert_eq!(
            Month::from_number(NonZeroU8::new(rug_fuzz_1).unwrap()), Ok(Month::February)
        );
        debug_assert_eq!(
            Month::from_number(NonZeroU8::new(rug_fuzz_2).unwrap()), Ok(Month::March)
        );
        debug_assert_eq!(
            Month::from_number(NonZeroU8::new(rug_fuzz_3).unwrap()), Ok(Month::April)
        );
        debug_assert_eq!(
            Month::from_number(NonZeroU8::new(rug_fuzz_4).unwrap()), Ok(Month::May)
        );
        debug_assert_eq!(
            Month::from_number(NonZeroU8::new(rug_fuzz_5).unwrap()), Ok(Month::June)
        );
        debug_assert_eq!(
            Month::from_number(NonZeroU8::new(rug_fuzz_6).unwrap()), Ok(Month::July)
        );
        debug_assert_eq!(
            Month::from_number(NonZeroU8::new(rug_fuzz_7).unwrap()), Ok(Month::August)
        );
        debug_assert_eq!(
            Month::from_number(NonZeroU8::new(rug_fuzz_8).unwrap()), Ok(Month::September)
        );
        debug_assert_eq!(
            Month::from_number(NonZeroU8::new(rug_fuzz_9).unwrap()), Ok(Month::October)
        );
        debug_assert_eq!(
            Month::from_number(NonZeroU8::new(rug_fuzz_10).unwrap()), Ok(Month::November)
        );
        debug_assert_eq!(
            Month::from_number(NonZeroU8::new(rug_fuzz_11).unwrap()), Ok(Month::December)
        );
        debug_assert_eq!(
            Month::from_number(NonZeroU8::new(rug_fuzz_12).unwrap()), Err(ComponentRange
            { name : "month", minimum : 1, maximum : 12, value : 0, conditional_range :
            false, })
        );
        debug_assert_eq!(
            Month::from_number(NonZeroU8::new(rug_fuzz_13).unwrap()), Err(ComponentRange
            { name : "month", minimum : 1, maximum : 12, value : 13, conditional_range :
            false, })
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_364 {
    use super::*;
    use crate::*;
    #[test]
    fn test_next() {
        let _rug_st_tests_llm_16_364_rrrruuuugggg_test_next = 0;
        debug_assert_eq!(Month::January.next(), Month::February);
        debug_assert_eq!(Month::February.next(), Month::March);
        debug_assert_eq!(Month::March.next(), Month::April);
        debug_assert_eq!(Month::April.next(), Month::May);
        debug_assert_eq!(Month::May.next(), Month::June);
        debug_assert_eq!(Month::June.next(), Month::July);
        debug_assert_eq!(Month::July.next(), Month::August);
        debug_assert_eq!(Month::August.next(), Month::September);
        debug_assert_eq!(Month::September.next(), Month::October);
        debug_assert_eq!(Month::October.next(), Month::November);
        debug_assert_eq!(Month::November.next(), Month::December);
        debug_assert_eq!(Month::December.next(), Month::January);
        let _rug_ed_tests_llm_16_364_rrrruuuugggg_test_next = 0;
    }
}
mod tests_llm_16_365 {
    use super::*;
    use crate::*;
    use std::convert::TryFrom;
    use crate::month::Month::*;
    #[test]
    fn test_previous() {
        let _rug_st_tests_llm_16_365_rrrruuuugggg_test_previous = 0;
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
        let _rug_ed_tests_llm_16_365_rrrruuuugggg_test_previous = 0;
    }
}
