//! Days of the week.
use core::fmt::{self, Display};
use Weekday::*;
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
#[cfg(test)]
mod tests_llm_16_437 {
    use super::*;
    use crate::*;
    #[test]
    fn test_next_weekday() {
        let _rug_st_tests_llm_16_437_rrrruuuugggg_test_next_weekday = 0;
        debug_assert_eq!(Weekday::Monday.next(), Weekday::Tuesday);
        debug_assert_eq!(Weekday::Tuesday.next(), Weekday::Wednesday);
        debug_assert_eq!(Weekday::Wednesday.next(), Weekday::Thursday);
        debug_assert_eq!(Weekday::Thursday.next(), Weekday::Friday);
        debug_assert_eq!(Weekday::Friday.next(), Weekday::Saturday);
        debug_assert_eq!(Weekday::Saturday.next(), Weekday::Sunday);
        debug_assert_eq!(Weekday::Sunday.next(), Weekday::Monday);
        let _rug_ed_tests_llm_16_437_rrrruuuugggg_test_next_weekday = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_439_llm_16_438 {
    use crate::weekday::Weekday;
    #[test]
    fn test_number_days_from_monday() {
        let _rug_st_tests_llm_16_439_llm_16_438_rrrruuuugggg_test_number_days_from_monday = 0;
        debug_assert_eq!(Weekday::Monday.number_days_from_monday(), 0);
        debug_assert_eq!(Weekday::Tuesday.number_days_from_monday(), 1);
        debug_assert_eq!(Weekday::Wednesday.number_days_from_monday(), 2);
        debug_assert_eq!(Weekday::Thursday.number_days_from_monday(), 3);
        debug_assert_eq!(Weekday::Friday.number_days_from_monday(), 4);
        debug_assert_eq!(Weekday::Saturday.number_days_from_monday(), 5);
        debug_assert_eq!(Weekday::Sunday.number_days_from_monday(), 6);
        let _rug_ed_tests_llm_16_439_llm_16_438_rrrruuuugggg_test_number_days_from_monday = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_442 {
    use super::*;
    use crate::*;
    #[test]
    fn test_number_from_monday() {
        let _rug_st_tests_llm_16_442_rrrruuuugggg_test_number_from_monday = 0;
        debug_assert_eq!(Weekday::Monday.number_from_monday(), 1);
        debug_assert_eq!(Weekday::Tuesday.number_from_monday(), 2);
        debug_assert_eq!(Weekday::Wednesday.number_from_monday(), 3);
        debug_assert_eq!(Weekday::Thursday.number_from_monday(), 4);
        debug_assert_eq!(Weekday::Friday.number_from_monday(), 5);
        debug_assert_eq!(Weekday::Saturday.number_from_monday(), 6);
        debug_assert_eq!(Weekday::Sunday.number_from_monday(), 7);
        let _rug_ed_tests_llm_16_442_rrrruuuugggg_test_number_from_monday = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_443 {
    use crate::weekday::Weekday;
    #[test]
    fn test_number_from_sunday() {
        let _rug_st_tests_llm_16_443_rrrruuuugggg_test_number_from_sunday = 0;
        debug_assert_eq!(Weekday::Monday.number_from_sunday(), 2);
        debug_assert_eq!(Weekday::Tuesday.number_from_sunday(), 3);
        debug_assert_eq!(Weekday::Wednesday.number_from_sunday(), 4);
        debug_assert_eq!(Weekday::Thursday.number_from_sunday(), 5);
        debug_assert_eq!(Weekday::Friday.number_from_sunday(), 6);
        debug_assert_eq!(Weekday::Saturday.number_from_sunday(), 7);
        debug_assert_eq!(Weekday::Sunday.number_from_sunday(), 1);
        let _rug_ed_tests_llm_16_443_rrrruuuugggg_test_number_from_sunday = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_444 {
    use super::*;
    use crate::*;
    use crate::weekday::Weekday::*;
    #[test]
    fn test_previous() {
        let _rug_st_tests_llm_16_444_rrrruuuugggg_test_previous = 0;
        debug_assert_eq!(Monday.previous(), Sunday);
        debug_assert_eq!(Tuesday.previous(), Monday);
        debug_assert_eq!(Wednesday.previous(), Tuesday);
        debug_assert_eq!(Thursday.previous(), Wednesday);
        debug_assert_eq!(Friday.previous(), Thursday);
        debug_assert_eq!(Saturday.previous(), Friday);
        debug_assert_eq!(Sunday.previous(), Saturday);
        let _rug_ed_tests_llm_16_444_rrrruuuugggg_test_previous = 0;
    }
}
#[cfg(test)]
mod tests_rug_262 {
    use super::*;
    use crate::Weekday;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_262_rrrruuuugggg_test_rug = 0;
        let mut p0: Weekday = Weekday::Monday;
        p0.number_days_from_sunday();
        let _rug_ed_tests_rug_262_rrrruuuugggg_test_rug = 0;
    }
}
