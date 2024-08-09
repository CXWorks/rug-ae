//! Days of the week.
use core::fmt::{self, Display};
use core::str::FromStr;
use Weekday::*;
use crate::error;
/// Days of the week.
///
/// As order is dependent on context (Sunday could be either two days after or five days before
/// Friday), this type does not implement `PartialOrd` or `Ord`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Weekday {
    #[allow(clippy::missing_docs_in_private_items)]
    Monday,
    #[allow(clippy::missing_docs_in_private_items)]
    Tuesday,
    #[allow(clippy::missing_docs_in_private_items)]
    Wednesday,
    #[allow(clippy::missing_docs_in_private_items)]
    Thursday,
    #[allow(clippy::missing_docs_in_private_items)]
    Friday,
    #[allow(clippy::missing_docs_in_private_items)]
    Saturday,
    #[allow(clippy::missing_docs_in_private_items)]
    Sunday,
}
impl Weekday {
    /// Get the previous weekday.
    ///
    /// ```rust
    /// # use time::Weekday;
    /// assert_eq!(Weekday::Tuesday.previous(), Weekday::Monday);
    /// ```
    pub const fn previous(self) -> Self {
        match self {
            Monday => Sunday,
            Tuesday => Monday,
            Wednesday => Tuesday,
            Thursday => Wednesday,
            Friday => Thursday,
            Saturday => Friday,
            Sunday => Saturday,
        }
    }
    /// Get the next weekday.
    ///
    /// ```rust
    /// # use time::Weekday;
    /// assert_eq!(Weekday::Monday.next(), Weekday::Tuesday);
    /// ```
    pub const fn next(self) -> Self {
        match self {
            Monday => Tuesday,
            Tuesday => Wednesday,
            Wednesday => Thursday,
            Thursday => Friday,
            Friday => Saturday,
            Saturday => Sunday,
            Sunday => Monday,
        }
    }
    /// Get n-th next day.
    ///
    /// ```rust
    /// # use time::Weekday;
    /// assert_eq!(Weekday::Monday.nth_next(1), Weekday::Tuesday);
    /// assert_eq!(Weekday::Sunday.nth_next(10), Weekday::Wednesday);
    /// ```
    pub const fn nth_next(self, n: u8) -> Self {
        match (self.number_days_from_monday() + n % 7) % 7 {
            0 => Monday,
            1 => Tuesday,
            2 => Wednesday,
            3 => Thursday,
            4 => Friday,
            5 => Saturday,
            val => {
                debug_assert!(val == 6);
                Sunday
            }
        }
    }
    /// Get the one-indexed number of days from Monday.
    ///
    /// ```rust
    /// # use time::Weekday;
    /// assert_eq!(Weekday::Monday.number_from_monday(), 1);
    /// ```
    #[doc(alias = "iso_weekday_number")]
    pub const fn number_from_monday(self) -> u8 {
        self.number_days_from_monday() + 1
    }
    /// Get the one-indexed number of days from Sunday.
    ///
    /// ```rust
    /// # use time::Weekday;
    /// assert_eq!(Weekday::Monday.number_from_sunday(), 2);
    /// ```
    pub const fn number_from_sunday(self) -> u8 {
        self.number_days_from_sunday() + 1
    }
    /// Get the zero-indexed number of days from Monday.
    ///
    /// ```rust
    /// # use time::Weekday;
    /// assert_eq!(Weekday::Monday.number_days_from_monday(), 0);
    /// ```
    pub const fn number_days_from_monday(self) -> u8 {
        self as _
    }
    /// Get the zero-indexed number of days from Sunday.
    ///
    /// ```rust
    /// # use time::Weekday;
    /// assert_eq!(Weekday::Monday.number_days_from_sunday(), 1);
    /// ```
    pub const fn number_days_from_sunday(self) -> u8 {
        match self {
            Monday => 1,
            Tuesday => 2,
            Wednesday => 3,
            Thursday => 4,
            Friday => 5,
            Saturday => 6,
            Sunday => 0,
        }
    }
}
impl Display for Weekday {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(
            match self {
                Monday => "Monday",
                Tuesday => "Tuesday",
                Wednesday => "Wednesday",
                Thursday => "Thursday",
                Friday => "Friday",
                Saturday => "Saturday",
                Sunday => "Sunday",
            },
        )
    }
}
impl FromStr for Weekday {
    type Err = error::InvalidVariant;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Monday" => Ok(Monday),
            "Tuesday" => Ok(Tuesday),
            "Wednesday" => Ok(Wednesday),
            "Thursday" => Ok(Thursday),
            "Friday" => Ok(Friday),
            "Saturday" => Ok(Saturday),
            "Sunday" => Ok(Sunday),
            _ => Err(error::InvalidVariant),
        }
    }
}
#[cfg(test)]
mod tests_llm_16_163 {
    use super::*;
    use crate::*;
    use std::str::FromStr;
    #[test]
    fn test_from_str_valid_days() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(&str, &str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Weekday::from_str(rug_fuzz_0), Ok(Weekday::Monday));
        debug_assert_eq!(Weekday::from_str(rug_fuzz_1), Ok(Weekday::Tuesday));
        debug_assert_eq!(Weekday::from_str(rug_fuzz_2), Ok(Weekday::Wednesday));
        debug_assert_eq!(Weekday::from_str(rug_fuzz_3), Ok(Weekday::Thursday));
        debug_assert_eq!(Weekday::from_str(rug_fuzz_4), Ok(Weekday::Friday));
        debug_assert_eq!(Weekday::from_str(rug_fuzz_5), Ok(Weekday::Saturday));
        debug_assert_eq!(Weekday::from_str(rug_fuzz_6), Ok(Weekday::Sunday));
             }
});    }
    #[test]
    fn test_from_str_invalid_day() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(
            matches!(Weekday::from_str(rug_fuzz_0), Err(error::InvalidVariant))
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_483 {
    use super::*;
    use crate::*;
    #[test]
    fn test_next_weekday() {
        let _rug_st_tests_llm_16_483_rrrruuuugggg_test_next_weekday = 0;
        debug_assert_eq!(Weekday::Monday.next(), Weekday::Tuesday);
        debug_assert_eq!(Weekday::Tuesday.next(), Weekday::Wednesday);
        debug_assert_eq!(Weekday::Wednesday.next(), Weekday::Thursday);
        debug_assert_eq!(Weekday::Thursday.next(), Weekday::Friday);
        debug_assert_eq!(Weekday::Friday.next(), Weekday::Saturday);
        debug_assert_eq!(Weekday::Saturday.next(), Weekday::Sunday);
        debug_assert_eq!(Weekday::Sunday.next(), Weekday::Monday);
        let _rug_ed_tests_llm_16_483_rrrruuuugggg_test_next_weekday = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_484 {
    use super::*;
    use crate::*;
    #[test]
    fn test_nth_next() {
        let _rug_st_tests_llm_16_484_rrrruuuugggg_test_nth_next = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 7;
        let rug_fuzz_3 = 13;
        let rug_fuzz_4 = 14;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = 6;
        let rug_fuzz_8 = 12;
        let rug_fuzz_9 = 13;
        let rug_fuzz_10 = 0;
        let rug_fuzz_11 = 1;
        let rug_fuzz_12 = 5;
        let rug_fuzz_13 = 11;
        let rug_fuzz_14 = 12;
        let rug_fuzz_15 = 0;
        let rug_fuzz_16 = 1;
        let rug_fuzz_17 = 4;
        let rug_fuzz_18 = 10;
        let rug_fuzz_19 = 11;
        let rug_fuzz_20 = 0;
        let rug_fuzz_21 = 1;
        let rug_fuzz_22 = 3;
        let rug_fuzz_23 = 9;
        let rug_fuzz_24 = 10;
        let rug_fuzz_25 = 0;
        let rug_fuzz_26 = 1;
        let rug_fuzz_27 = 2;
        let rug_fuzz_28 = 8;
        let rug_fuzz_29 = 9;
        let rug_fuzz_30 = 0;
        let rug_fuzz_31 = 1;
        let rug_fuzz_32 = 1;
        let rug_fuzz_33 = 2;
        let rug_fuzz_34 = 7;
        use Weekday::*;
        debug_assert_eq!(Monday.nth_next(rug_fuzz_0), Monday);
        debug_assert_eq!(Monday.nth_next(rug_fuzz_1), Tuesday);
        debug_assert_eq!(Monday.nth_next(rug_fuzz_2), Monday);
        debug_assert_eq!(Monday.nth_next(rug_fuzz_3), Sunday);
        debug_assert_eq!(Monday.nth_next(rug_fuzz_4), Monday);
        debug_assert_eq!(Tuesday.nth_next(rug_fuzz_5), Tuesday);
        debug_assert_eq!(Tuesday.nth_next(rug_fuzz_6), Wednesday);
        debug_assert_eq!(Tuesday.nth_next(rug_fuzz_7), Monday);
        debug_assert_eq!(Tuesday.nth_next(rug_fuzz_8), Sunday);
        debug_assert_eq!(Tuesday.nth_next(rug_fuzz_9), Monday);
        debug_assert_eq!(Wednesday.nth_next(rug_fuzz_10), Wednesday);
        debug_assert_eq!(Wednesday.nth_next(rug_fuzz_11), Thursday);
        debug_assert_eq!(Wednesday.nth_next(rug_fuzz_12), Monday);
        debug_assert_eq!(Wednesday.nth_next(rug_fuzz_13), Sunday);
        debug_assert_eq!(Wednesday.nth_next(rug_fuzz_14), Monday);
        debug_assert_eq!(Thursday.nth_next(rug_fuzz_15), Thursday);
        debug_assert_eq!(Thursday.nth_next(rug_fuzz_16), Friday);
        debug_assert_eq!(Thursday.nth_next(rug_fuzz_17), Monday);
        debug_assert_eq!(Thursday.nth_next(rug_fuzz_18), Sunday);
        debug_assert_eq!(Thursday.nth_next(rug_fuzz_19), Monday);
        debug_assert_eq!(Friday.nth_next(rug_fuzz_20), Friday);
        debug_assert_eq!(Friday.nth_next(rug_fuzz_21), Saturday);
        debug_assert_eq!(Friday.nth_next(rug_fuzz_22), Monday);
        debug_assert_eq!(Friday.nth_next(rug_fuzz_23), Sunday);
        debug_assert_eq!(Friday.nth_next(rug_fuzz_24), Monday);
        debug_assert_eq!(Saturday.nth_next(rug_fuzz_25), Saturday);
        debug_assert_eq!(Saturday.nth_next(rug_fuzz_26), Sunday);
        debug_assert_eq!(Saturday.nth_next(rug_fuzz_27), Monday);
        debug_assert_eq!(Saturday.nth_next(rug_fuzz_28), Sunday);
        debug_assert_eq!(Saturday.nth_next(rug_fuzz_29), Monday);
        debug_assert_eq!(Sunday.nth_next(rug_fuzz_30), Sunday);
        debug_assert_eq!(Sunday.nth_next(rug_fuzz_31), Monday);
        debug_assert_eq!(Sunday.nth_next(rug_fuzz_32), Tuesday.nth_next(6));
        debug_assert_eq!(Sunday.nth_next(rug_fuzz_33), Wednesday.nth_next(5));
        debug_assert_eq!(Sunday.nth_next(rug_fuzz_34), Sunday);
        let _rug_ed_tests_llm_16_484_rrrruuuugggg_test_nth_next = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_485 {
    use super::*;
    use crate::*;
    #[test]
    fn test_number_days_from_monday() {
        let _rug_st_tests_llm_16_485_rrrruuuugggg_test_number_days_from_monday = 0;
        debug_assert_eq!(Weekday::Monday.number_days_from_monday(), 0);
        debug_assert_eq!(Weekday::Tuesday.number_days_from_monday(), 1);
        debug_assert_eq!(Weekday::Wednesday.number_days_from_monday(), 2);
        debug_assert_eq!(Weekday::Thursday.number_days_from_monday(), 3);
        debug_assert_eq!(Weekday::Friday.number_days_from_monday(), 4);
        debug_assert_eq!(Weekday::Saturday.number_days_from_monday(), 5);
        debug_assert_eq!(Weekday::Sunday.number_days_from_monday(), 6);
        let _rug_ed_tests_llm_16_485_rrrruuuugggg_test_number_days_from_monday = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_486 {
    use crate::Weekday;
    use crate::Weekday::*;
    #[test]
    fn test_number_days_from_sunday() {
        let _rug_st_tests_llm_16_486_rrrruuuugggg_test_number_days_from_sunday = 0;
        debug_assert_eq!(Sunday.number_days_from_sunday(), 0);
        debug_assert_eq!(Monday.number_days_from_sunday(), 1);
        debug_assert_eq!(Tuesday.number_days_from_sunday(), 2);
        debug_assert_eq!(Wednesday.number_days_from_sunday(), 3);
        debug_assert_eq!(Thursday.number_days_from_sunday(), 4);
        debug_assert_eq!(Friday.number_days_from_sunday(), 5);
        debug_assert_eq!(Saturday.number_days_from_sunday(), 6);
        let _rug_ed_tests_llm_16_486_rrrruuuugggg_test_number_days_from_sunday = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_487 {
    use crate::Weekday;
    #[test]
    fn number_from_monday_test() {
        let _rug_st_tests_llm_16_487_rrrruuuugggg_number_from_monday_test = 0;
        debug_assert_eq!(Weekday::Monday.number_from_monday(), 1);
        debug_assert_eq!(Weekday::Tuesday.number_from_monday(), 2);
        debug_assert_eq!(Weekday::Wednesday.number_from_monday(), 3);
        debug_assert_eq!(Weekday::Thursday.number_from_monday(), 4);
        debug_assert_eq!(Weekday::Friday.number_from_monday(), 5);
        debug_assert_eq!(Weekday::Saturday.number_from_monday(), 6);
        debug_assert_eq!(Weekday::Sunday.number_from_monday(), 7);
        let _rug_ed_tests_llm_16_487_rrrruuuugggg_number_from_monday_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_488 {
    use crate::Weekday::*;
    #[test]
    fn test_number_from_sunday() {
        let _rug_st_tests_llm_16_488_rrrruuuugggg_test_number_from_sunday = 0;
        debug_assert_eq!(
            Sunday.number_from_sunday(), 1, "Sunday should be 1 from Sunday"
        );
        debug_assert_eq!(
            Monday.number_from_sunday(), 2, "Monday should be 2 from Sunday"
        );
        debug_assert_eq!(
            Tuesday.number_from_sunday(), 3, "Tuesday should be 3 from Sunday"
        );
        debug_assert_eq!(
            Wednesday.number_from_sunday(), 4, "Wednesday should be 4 from Sunday"
        );
        debug_assert_eq!(
            Thursday.number_from_sunday(), 5, "Thursday should be 5 from Sunday"
        );
        debug_assert_eq!(
            Friday.number_from_sunday(), 6, "Friday should be 6 from Sunday"
        );
        debug_assert_eq!(
            Saturday.number_from_sunday(), 7, "Saturday should be 7 from Sunday"
        );
        let _rug_ed_tests_llm_16_488_rrrruuuugggg_test_number_from_sunday = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_489 {
    use crate::Weekday::*;
    use crate::Weekday;
    #[test]
    fn test_previous_weekday() {
        let _rug_st_tests_llm_16_489_rrrruuuugggg_test_previous_weekday = 0;
        debug_assert_eq!(Monday.previous(), Sunday);
        debug_assert_eq!(Tuesday.previous(), Monday);
        debug_assert_eq!(Wednesday.previous(), Tuesday);
        debug_assert_eq!(Thursday.previous(), Wednesday);
        debug_assert_eq!(Friday.previous(), Thursday);
        debug_assert_eq!(Saturday.previous(), Friday);
        debug_assert_eq!(Sunday.previous(), Saturday);
        let _rug_ed_tests_llm_16_489_rrrruuuugggg_test_previous_weekday = 0;
    }
}
