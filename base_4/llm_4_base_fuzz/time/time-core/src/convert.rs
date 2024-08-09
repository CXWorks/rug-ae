#![allow(clippy::missing_docs_in_private_items)]
macro_rules! declare_structs {
    ($($t:ident)*) => {
        $(#[derive(Debug, Copy, Clone)] pub struct $t; impl $t { pub const fn per < T >
        (self, _ : T) -> < (Self, T) as Per >::Output where (Self, T) : Per, T : Copy, {
        < (Self, T) >::VALUE } })*
    };
}
declare_structs! {
    Nanosecond Microsecond Millisecond Second Minute Hour Day Week
}
mod sealed {
    pub trait Sealed {}
}
pub trait Per: sealed::Sealed {
    type Output;
    const VALUE: Self::Output;
}
macro_rules! impl_per {
    ($($t:ty : $x:ident in $y:ident = $val:expr)*) => {
        $(impl sealed::Sealed for ($x, $y) {} impl Per for ($x, $y) { type Output = $t;
        const VALUE : $t = $val; })*
    };
}
impl_per! {
    u16 : Nanosecond in Microsecond = 1_000 u32 : Nanosecond in Millisecond = 1_000_000
    u32 : Nanosecond in Second = 1_000_000_000 u64 : Nanosecond in Minute =
    60_000_000_000 u64 : Nanosecond in Hour = 3_600_000_000_000 u64 : Nanosecond in Day =
    86_400_000_000_000 u64 : Nanosecond in Week = 604_800_000_000_000 u16 : Microsecond
    in Millisecond = 1_000 u32 : Microsecond in Second = 1_000_000 u32 : Microsecond in
    Minute = 60_000_000 u32 : Microsecond in Hour = 3_600_000_000 u64 : Microsecond in
    Day = 86_400_000_000 u64 : Microsecond in Week = 604_800_000_000 u16 : Millisecond in
    Second = 1_000 u16 : Millisecond in Minute = 60_000 u32 : Millisecond in Hour =
    3_600_000 u32 : Millisecond in Day = 86_400_000 u32 : Millisecond in Week =
    604_800_000 u8 : Second in Minute = 60 u16 : Second in Hour = 3_600 u32 : Second in
    Day = 86_400 u32 : Second in Week = 604_800 u8 : Minute in Hour = 60 u16 : Minute in
    Day = 1_440 u16 : Minute in Week = 10_080 u8 : Hour in Day = 24 u8 : Hour in Week =
    168 u8 : Day in Week = 7
}
#[cfg(test)]
mod tests_llm_16_2 {
    use crate::convert::{Hour, Day, Per};
    #[test]
    fn hour_per_day() {
        let _rug_st_tests_llm_16_2_rrrruuuugggg_hour_per_day = 0;
        let hour = Hour;
        let result: <(Hour, Day) as Per>::Output = hour.per(Day);
        debug_assert_eq!(result, < (Hour, Day) > ::VALUE);
        let _rug_ed_tests_llm_16_2_rrrruuuugggg_hour_per_day = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_3_llm_16_3 {
    use super::*;
    use crate::*;
    use crate::convert::{Day, Hour, Microsecond, Millisecond, Minute, Per, Second, Week};
    #[test]
    fn microseconds_per_day() {
        let _rug_st_tests_llm_16_3_llm_16_3_rrrruuuugggg_microseconds_per_day = 0;
        let micros_per_day = Microsecond.per(Day);
        debug_assert_eq!(micros_per_day, < (Microsecond, Day) as Per > ::VALUE);
        let _rug_ed_tests_llm_16_3_llm_16_3_rrrruuuugggg_microseconds_per_day = 0;
    }
    #[test]
    fn microseconds_per_hour() {
        let _rug_st_tests_llm_16_3_llm_16_3_rrrruuuugggg_microseconds_per_hour = 0;
        let micros_per_hour = Microsecond.per(Hour);
        debug_assert_eq!(micros_per_hour, < (Microsecond, Hour) as Per > ::VALUE);
        let _rug_ed_tests_llm_16_3_llm_16_3_rrrruuuugggg_microseconds_per_hour = 0;
    }
    #[test]
    fn microseconds_per_millisecond() {
        let _rug_st_tests_llm_16_3_llm_16_3_rrrruuuugggg_microseconds_per_millisecond = 0;
        let micros_per_millisecond = Microsecond.per(Millisecond);
        debug_assert_eq!(
            micros_per_millisecond, < (Microsecond, Millisecond) as Per > ::VALUE
        );
        let _rug_ed_tests_llm_16_3_llm_16_3_rrrruuuugggg_microseconds_per_millisecond = 0;
    }
    #[test]
    fn microseconds_per_minute() {
        let _rug_st_tests_llm_16_3_llm_16_3_rrrruuuugggg_microseconds_per_minute = 0;
        let micros_per_minute = Microsecond.per(Minute);
        debug_assert_eq!(micros_per_minute, < (Microsecond, Minute) as Per > ::VALUE);
        let _rug_ed_tests_llm_16_3_llm_16_3_rrrruuuugggg_microseconds_per_minute = 0;
    }
    #[test]
    fn microseconds_per_second() {
        let _rug_st_tests_llm_16_3_llm_16_3_rrrruuuugggg_microseconds_per_second = 0;
        let micros_per_second = Microsecond.per(Second);
        debug_assert_eq!(micros_per_second, < (Microsecond, Second) as Per > ::VALUE);
        let _rug_ed_tests_llm_16_3_llm_16_3_rrrruuuugggg_microseconds_per_second = 0;
    }
    #[test]
    fn microseconds_per_week() {
        let _rug_st_tests_llm_16_3_llm_16_3_rrrruuuugggg_microseconds_per_week = 0;
        let micros_per_week = Microsecond.per(Week);
        debug_assert_eq!(micros_per_week, < (Microsecond, Week) as Per > ::VALUE);
        let _rug_ed_tests_llm_16_3_llm_16_3_rrrruuuugggg_microseconds_per_week = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_6_llm_16_6 {
    use super::*;
    use crate::*;
    use crate::convert::{Day, Nanosecond, Per};
    #[test]
    fn nanoseconds_per_day() {
        let _rug_st_tests_llm_16_6_llm_16_6_rrrruuuugggg_nanoseconds_per_day = 0;
        debug_assert_eq!(Nanosecond.per(Day), < (Nanosecond, Day) as Per > ::VALUE);
        let _rug_ed_tests_llm_16_6_llm_16_6_rrrruuuugggg_nanoseconds_per_day = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_7_llm_16_7 {
    use crate::convert::{Second, Minute, Hour, Day, Week};
    #[test]
    fn test_seconds_per_minute() {
        let _rug_st_tests_llm_16_7_llm_16_7_rrrruuuugggg_test_seconds_per_minute = 0;
        debug_assert_eq!(Second.per(Minute), 60);
        let _rug_ed_tests_llm_16_7_llm_16_7_rrrruuuugggg_test_seconds_per_minute = 0;
    }
    #[test]
    fn test_seconds_per_hour() {
        let _rug_st_tests_llm_16_7_llm_16_7_rrrruuuugggg_test_seconds_per_hour = 0;
        debug_assert_eq!(Second.per(Hour), 3600);
        let _rug_ed_tests_llm_16_7_llm_16_7_rrrruuuugggg_test_seconds_per_hour = 0;
    }
    #[test]
    fn test_seconds_per_day() {
        let _rug_st_tests_llm_16_7_llm_16_7_rrrruuuugggg_test_seconds_per_day = 0;
        debug_assert_eq!(Second.per(Day), 86400);
        let _rug_ed_tests_llm_16_7_llm_16_7_rrrruuuugggg_test_seconds_per_day = 0;
    }
    #[test]
    fn test_seconds_per_week() {
        let _rug_st_tests_llm_16_7_llm_16_7_rrrruuuugggg_test_seconds_per_week = 0;
        debug_assert_eq!(Second.per(Week), 604800);
        let _rug_ed_tests_llm_16_7_llm_16_7_rrrruuuugggg_test_seconds_per_week = 0;
    }
}
