//! Utility functions.
pub use time_core::util::{days_in_year, is_leap_year, weeks_in_year};
use crate::Month;
/// Whether to adjust the date, and in which direction. Useful when implementing arithmetic.
pub(crate) enum DateAdjustment {
    /// The previous day should be used.
    Previous,
    /// The next day should be used.
    Next,
    /// The date should be used as-is.
    None,
}
/// Get the number of days in the month of a given year.
///
/// ```rust
/// # use time::{Month, util};
/// assert_eq!(util::days_in_year_month(2020, Month::February), 29);
/// ```
pub const fn days_in_year_month(year: i32, month: Month) -> u8 {
    use Month::*;
    match month {
        January | March | May | July | August | October | December => 31,
        April | June | September | November => 30,
        February if is_leap_year(year) => 29,
        February => 28,
    }
}
#[cfg(feature = "local-offset")]
/// Utility functions relating to the local UTC offset.
pub mod local_offset {
    use core::sync::atomic::{AtomicBool, Ordering};
    /// Whether obtaining the local UTC offset is required to be sound.
    static LOCAL_OFFSET_IS_SOUND: AtomicBool = AtomicBool::new(true);
    /// The soundness of obtaining the local UTC offset.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Soundness {
        /// Obtaining the local UTC offset is required to be sound. Undefined behavior will never
        /// occur. This is the default.
        Sound,
        /// Obtaining the local UTC offset is allowed to invoke undefined behavior. **Setting this
        /// value is strongly discouraged.** To do so, you must comply with the safety requirements
        /// of [`time::local_offset::set_soundness`](set_soundness).
        Unsound,
    }
    /// Set whether obtaining the local UTC offset is allowed to invoke undefined behavior. **Use of
    /// this function is heavily discouraged.**
    ///
    /// # Safety
    ///
    /// If this method is called with [`Soundness::Sound`], the call is always sound. If this method
    /// is called with [`Soundness::Unsound`], the following conditions apply.
    ///
    /// - If the operating system provides a thread-safe environment, the call is sound.
    /// - If the process is single-threaded, the call is sound.
    /// - If the process is multi-threaded, no other thread may mutate the environment in any way at
    ///   the same time a call to a method that obtains the local UTC offset. This includes adding,
    ///   removing, or modifying an environment variable.
    ///
    /// The first two conditions are automatically checked by `time`, such that you do not need to
    /// declare your code unsound. Currently, the only known operating systems that does _not_
    /// provide a thread-safe environment are some Unix-like OS's. All other operating systems
    /// should succeed when attempting to obtain the local UTC offset.
    ///
    /// Note that you must not only verify this safety condition for your code, but for **all** code
    /// that will be included in the final binary. Notably, it applies to both direct and transitive
    /// dependencies and to both Rust and non-Rust code. **For this reason it is not possible to
    /// soundly pass [`Soundness::Unsound`] to this method if you are writing a library that may
    /// used by others.**
    ///
    /// If using this method is absolutely necessary, it is recommended to keep the time between
    /// setting the soundness to [`Soundness::Unsound`] and setting it back to [`Soundness::Sound`]
    /// as short as possible.
    ///
    /// The following methods currently obtain the local UTC offset:
    ///
    /// - [`OffsetDateTime::now_local`](crate::OffsetDateTime::now_local)
    /// - [`UtcOffset::local_offset_at`](crate::UtcOffset::local_offset_at)
    /// - [`UtcOffset::current_local_offset`](crate::UtcOffset::current_local_offset)
    pub unsafe fn set_soundness(soundness: Soundness) {
        LOCAL_OFFSET_IS_SOUND.store(soundness == Soundness::Sound, Ordering::SeqCst);
    }
    /// Obtains the soundness of obtaining the local UTC offset. If it is [`Soundness::Unsound`],
    /// it is allowed to invoke undefined behavior when obtaining the local UTC offset.
    pub fn get_soundness() -> Soundness {
        match LOCAL_OFFSET_IS_SOUND.load(Ordering::SeqCst) {
            false => Soundness::Unsound,
            true => Soundness::Sound,
        }
    }
}
#[cfg(test)]
mod tests_llm_16_482 {
    use super::*;
    use crate::*;
    use crate::Month::*;
    #[test]
    fn test_days_in_month_for_non_leap_year() {
        let _rug_st_tests_llm_16_482_rrrruuuugggg_test_days_in_month_for_non_leap_year = 0;
        let rug_fuzz_0 = 2021;
        let rug_fuzz_1 = 2021;
        let rug_fuzz_2 = 2021;
        let rug_fuzz_3 = 2021;
        let rug_fuzz_4 = 2021;
        let rug_fuzz_5 = 2021;
        let rug_fuzz_6 = 2021;
        let rug_fuzz_7 = 2021;
        let rug_fuzz_8 = 2021;
        let rug_fuzz_9 = 2021;
        let rug_fuzz_10 = 2021;
        let rug_fuzz_11 = 2021;
        debug_assert_eq!(days_in_year_month(rug_fuzz_0, January), 31);
        debug_assert_eq!(days_in_year_month(rug_fuzz_1, February), 28);
        debug_assert_eq!(days_in_year_month(rug_fuzz_2, March), 31);
        debug_assert_eq!(days_in_year_month(rug_fuzz_3, April), 30);
        debug_assert_eq!(days_in_year_month(rug_fuzz_4, May), 31);
        debug_assert_eq!(days_in_year_month(rug_fuzz_5, June), 30);
        debug_assert_eq!(days_in_year_month(rug_fuzz_6, July), 31);
        debug_assert_eq!(days_in_year_month(rug_fuzz_7, August), 31);
        debug_assert_eq!(days_in_year_month(rug_fuzz_8, September), 30);
        debug_assert_eq!(days_in_year_month(rug_fuzz_9, October), 31);
        debug_assert_eq!(days_in_year_month(rug_fuzz_10, November), 30);
        debug_assert_eq!(days_in_year_month(rug_fuzz_11, December), 31);
        let _rug_ed_tests_llm_16_482_rrrruuuugggg_test_days_in_month_for_non_leap_year = 0;
    }
    #[test]
    fn test_days_in_month_for_leap_year() {
        let _rug_st_tests_llm_16_482_rrrruuuugggg_test_days_in_month_for_leap_year = 0;
        let rug_fuzz_0 = 2020;
        let rug_fuzz_1 = 2020;
        let rug_fuzz_2 = 2020;
        let rug_fuzz_3 = 2020;
        let rug_fuzz_4 = 2020;
        let rug_fuzz_5 = 2020;
        let rug_fuzz_6 = 2020;
        let rug_fuzz_7 = 2020;
        let rug_fuzz_8 = 2020;
        let rug_fuzz_9 = 2020;
        let rug_fuzz_10 = 2020;
        let rug_fuzz_11 = 2020;
        debug_assert_eq!(days_in_year_month(rug_fuzz_0, January), 31);
        debug_assert_eq!(days_in_year_month(rug_fuzz_1, February), 29);
        debug_assert_eq!(days_in_year_month(rug_fuzz_2, March), 31);
        debug_assert_eq!(days_in_year_month(rug_fuzz_3, April), 30);
        debug_assert_eq!(days_in_year_month(rug_fuzz_4, May), 31);
        debug_assert_eq!(days_in_year_month(rug_fuzz_5, June), 30);
        debug_assert_eq!(days_in_year_month(rug_fuzz_6, July), 31);
        debug_assert_eq!(days_in_year_month(rug_fuzz_7, August), 31);
        debug_assert_eq!(days_in_year_month(rug_fuzz_8, September), 30);
        debug_assert_eq!(days_in_year_month(rug_fuzz_9, October), 31);
        debug_assert_eq!(days_in_year_month(rug_fuzz_10, November), 30);
        debug_assert_eq!(days_in_year_month(rug_fuzz_11, December), 31);
        let _rug_ed_tests_llm_16_482_rrrruuuugggg_test_days_in_month_for_leap_year = 0;
    }
    #[test]
    fn test_days_in_month_for_century_non_leap_year() {
        let _rug_st_tests_llm_16_482_rrrruuuugggg_test_days_in_month_for_century_non_leap_year = 0;
        let rug_fuzz_0 = 1900;
        debug_assert_eq!(days_in_year_month(rug_fuzz_0, February), 28);
        let _rug_ed_tests_llm_16_482_rrrruuuugggg_test_days_in_month_for_century_non_leap_year = 0;
    }
    #[test]
    fn test_days_in_month_for_century_leap_year() {
        let _rug_st_tests_llm_16_482_rrrruuuugggg_test_days_in_month_for_century_leap_year = 0;
        let rug_fuzz_0 = 2000;
        debug_assert_eq!(days_in_year_month(rug_fuzz_0, February), 29);
        let _rug_ed_tests_llm_16_482_rrrruuuugggg_test_days_in_month_for_century_leap_year = 0;
    }
}
