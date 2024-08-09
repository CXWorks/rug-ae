//! Extension traits.
use core::time::Duration as StdDuration;
use crate::Duration;
/// Sealed trait to prevent downstream implementations.
mod sealed {
    /// A trait that cannot be implemented by downstream users.
    pub trait Sealed {}
    impl Sealed for i64 {}
    impl Sealed for u64 {}
    impl Sealed for f64 {}
}
/// Create [`Duration`]s from numeric literals.
///
/// # Examples
///
/// Basic construction of [`Duration`]s.
///
/// ```rust
/// # use time::{Duration, ext::NumericalDuration};
/// assert_eq!(5.nanoseconds(), Duration::nanoseconds(5));
/// assert_eq!(5.microseconds(), Duration::microseconds(5));
/// assert_eq!(5.milliseconds(), Duration::milliseconds(5));
/// assert_eq!(5.seconds(), Duration::seconds(5));
/// assert_eq!(5.minutes(), Duration::minutes(5));
/// assert_eq!(5.hours(), Duration::hours(5));
/// assert_eq!(5.days(), Duration::days(5));
/// assert_eq!(5.weeks(), Duration::weeks(5));
/// ```
///
/// Signed integers work as well!
///
/// ```rust
/// # use time::{Duration, ext::NumericalDuration};
/// assert_eq!((-5).nanoseconds(), Duration::nanoseconds(-5));
/// assert_eq!((-5).microseconds(), Duration::microseconds(-5));
/// assert_eq!((-5).milliseconds(), Duration::milliseconds(-5));
/// assert_eq!((-5).seconds(), Duration::seconds(-5));
/// assert_eq!((-5).minutes(), Duration::minutes(-5));
/// assert_eq!((-5).hours(), Duration::hours(-5));
/// assert_eq!((-5).days(), Duration::days(-5));
/// assert_eq!((-5).weeks(), Duration::weeks(-5));
/// ```
///
/// Just like any other [`Duration`], they can be added, subtracted, etc.
///
/// ```rust
/// # use time::ext::NumericalDuration;
/// assert_eq!(2.seconds() + 500.milliseconds(), 2_500.milliseconds());
/// assert_eq!(2.seconds() - 500.milliseconds(), 1_500.milliseconds());
/// ```
///
/// When called on floating point values, any remainder of the floating point value will be
/// truncated. Keep in mind that floating point numbers are inherently imprecise and have limited
/// capacity.
pub trait NumericalDuration: sealed::Sealed {
    /// Create a [`Duration`] from the number of nanoseconds.
    fn nanoseconds(self) -> Duration;
    /// Create a [`Duration`] from the number of microseconds.
    fn microseconds(self) -> Duration;
    /// Create a [`Duration`] from the number of milliseconds.
    fn milliseconds(self) -> Duration;
    /// Create a [`Duration`] from the number of seconds.
    fn seconds(self) -> Duration;
    /// Create a [`Duration`] from the number of minutes.
    fn minutes(self) -> Duration;
    /// Create a [`Duration`] from the number of hours.
    fn hours(self) -> Duration;
    /// Create a [`Duration`] from the number of days.
    fn days(self) -> Duration;
    /// Create a [`Duration`] from the number of weeks.
    fn weeks(self) -> Duration;
}
impl NumericalDuration for i64 {
    fn nanoseconds(self) -> Duration {
        Duration::nanoseconds(self)
    }
    fn microseconds(self) -> Duration {
        Duration::microseconds(self)
    }
    fn milliseconds(self) -> Duration {
        Duration::milliseconds(self)
    }
    fn seconds(self) -> Duration {
        Duration::seconds(self)
    }
    fn minutes(self) -> Duration {
        Duration::minutes(self)
    }
    fn hours(self) -> Duration {
        Duration::hours(self)
    }
    fn days(self) -> Duration {
        Duration::days(self)
    }
    fn weeks(self) -> Duration {
        Duration::weeks(self)
    }
}
impl NumericalDuration for f64 {
    fn nanoseconds(self) -> Duration {
        Duration::nanoseconds(self as _)
    }
    fn microseconds(self) -> Duration {
        Duration::nanoseconds((self * 1_000.) as _)
    }
    fn milliseconds(self) -> Duration {
        Duration::nanoseconds((self * 1_000_000.) as _)
    }
    fn seconds(self) -> Duration {
        Duration::nanoseconds((self * 1_000_000_000.) as _)
    }
    fn minutes(self) -> Duration {
        Duration::nanoseconds((self * 60_000_000_000.) as _)
    }
    fn hours(self) -> Duration {
        Duration::nanoseconds((self * 3_600_000_000_000.) as _)
    }
    fn days(self) -> Duration {
        Duration::nanoseconds((self * 86_400_000_000_000.) as _)
    }
    fn weeks(self) -> Duration {
        Duration::nanoseconds((self * 604_800_000_000_000.) as _)
    }
}
/// Create [`std::time::Duration`]s from numeric literals.
///
/// # Examples
///
/// Basic construction of [`std::time::Duration`]s.
///
/// ```rust
/// # use time::ext::NumericalStdDuration;
/// # use core::time::Duration;
/// assert_eq!(5.std_nanoseconds(), Duration::from_nanos(5));
/// assert_eq!(5.std_microseconds(), Duration::from_micros(5));
/// assert_eq!(5.std_milliseconds(), Duration::from_millis(5));
/// assert_eq!(5.std_seconds(), Duration::from_secs(5));
/// assert_eq!(5.std_minutes(), Duration::from_secs(5 * 60));
/// assert_eq!(5.std_hours(), Duration::from_secs(5 * 3_600));
/// assert_eq!(5.std_days(), Duration::from_secs(5 * 86_400));
/// assert_eq!(5.std_weeks(), Duration::from_secs(5 * 604_800));
/// ```
///
/// Just like any other [`std::time::Duration`], they can be added, subtracted, etc.
///
/// ```rust
/// # use time::ext::NumericalStdDuration;
/// assert_eq!(
///     2.std_seconds() + 500.std_milliseconds(),
///     2_500.std_milliseconds()
/// );
/// assert_eq!(
///     2.std_seconds() - 500.std_milliseconds(),
///     1_500.std_milliseconds()
/// );
/// ```
///
/// When called on floating point values, any remainder of the floating point value will be
/// truncated. Keep in mind that floating point numbers are inherently imprecise and have limited
/// capacity.
pub trait NumericalStdDuration: sealed::Sealed {
    /// Create a [`std::time::Duration`] from the number of nanoseconds.
    fn std_nanoseconds(self) -> StdDuration;
    /// Create a [`std::time::Duration`] from the number of microseconds.
    fn std_microseconds(self) -> StdDuration;
    /// Create a [`std::time::Duration`] from the number of milliseconds.
    fn std_milliseconds(self) -> StdDuration;
    /// Create a [`std::time::Duration`] from the number of seconds.
    fn std_seconds(self) -> StdDuration;
    /// Create a [`std::time::Duration`] from the number of minutes.
    fn std_minutes(self) -> StdDuration;
    /// Create a [`std::time::Duration`] from the number of hours.
    fn std_hours(self) -> StdDuration;
    /// Create a [`std::time::Duration`] from the number of days.
    fn std_days(self) -> StdDuration;
    /// Create a [`std::time::Duration`] from the number of weeks.
    fn std_weeks(self) -> StdDuration;
}
impl NumericalStdDuration for u64 {
    fn std_nanoseconds(self) -> StdDuration {
        StdDuration::from_nanos(self)
    }
    fn std_microseconds(self) -> StdDuration {
        StdDuration::from_micros(self)
    }
    fn std_milliseconds(self) -> StdDuration {
        StdDuration::from_millis(self)
    }
    fn std_seconds(self) -> StdDuration {
        StdDuration::from_secs(self)
    }
    fn std_minutes(self) -> StdDuration {
        StdDuration::from_secs(self * 60)
    }
    fn std_hours(self) -> StdDuration {
        StdDuration::from_secs(self * 3_600)
    }
    fn std_days(self) -> StdDuration {
        StdDuration::from_secs(self * 86_400)
    }
    fn std_weeks(self) -> StdDuration {
        StdDuration::from_secs(self * 604_800)
    }
}
impl NumericalStdDuration for f64 {
    fn std_nanoseconds(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos(self as _)
    }
    fn std_microseconds(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos((self * 1_000.) as _)
    }
    fn std_milliseconds(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos((self * 1_000_000.) as _)
    }
    fn std_seconds(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos((self * 1_000_000_000.) as _)
    }
    fn std_minutes(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos((self * 60_000_000_000.) as _)
    }
    fn std_hours(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos((self * 3_600_000_000_000.) as _)
    }
    fn std_days(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos((self * 86_400_000_000_000.) as _)
    }
    fn std_weeks(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos((self * 604_800_000_000_000.) as _)
    }
}
#[cfg(test)]
mod tests_llm_16_107 {
    use super::*;
    use crate::*;
    use crate::Duration;
    #[test]
    fn test_milliseconds() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::milliseconds(rug_fuzz_0);
        debug_assert_eq!(duration.whole_milliseconds(), 1000);
        debug_assert_eq!(duration.subsec_milliseconds(), 0);
        debug_assert_eq!(duration.whole_nanoseconds(), 1_000_000_000);
        debug_assert_eq!(duration.subsec_nanoseconds(), 0);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_113_llm_16_112 {
    use super::*;
    use crate::*;
    #[test]
    fn test_seconds() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(f64, f64, f64, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < f64 as ext::NumericalDuration > ::seconds(rug_fuzz_0), Duration::seconds(0)
        );
        debug_assert_eq!(
            < f64 as ext::NumericalDuration > ::seconds(rug_fuzz_1),
            Duration::seconds_f64(1.5)
        );
        debug_assert_eq!(
            < f64 as ext::NumericalDuration > ::seconds(rug_fuzz_2),
            Duration::seconds_f64(0.5)
        );
        debug_assert_eq!(
            < f64 as ext::NumericalDuration > ::seconds(- rug_fuzz_3),
            Duration::seconds_f64(- 1.5)
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_114 {
    use super::*;
    use crate::*;
    use crate::ext::NumericalDuration;
    #[test]
    fn test_weeks() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(rug_fuzz_0.weeks(), 604_800.seconds());
        debug_assert_eq!(rug_fuzz_1.weeks(), 1.weeks() + 1.weeks());
        debug_assert_eq!(rug_fuzz_2.weeks(), Duration::ZERO);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_133 {
    use super::*;
    use crate::*;
    use std::convert::TryInto;
    #[test]
    fn test_hours() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = Duration::hours(rug_fuzz_0);
        let expected = Duration::seconds(rug_fuzz_1 * rug_fuzz_2);
        debug_assert_eq!(duration, expected);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_136 {
    use super::*;
    use crate::*;
    #[test]
    fn test_milliseconds() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i64, i64, i64, i64, i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Duration::milliseconds(rug_fuzz_0), 1.seconds());
        debug_assert_eq!(Duration::milliseconds(- rug_fuzz_1), (- 1).seconds());
        debug_assert_eq!(Duration::milliseconds(rug_fuzz_2), 0.seconds());
        debug_assert_eq!(Duration::milliseconds(rug_fuzz_3), 1.milliseconds());
        debug_assert_eq!(Duration::milliseconds(- rug_fuzz_4), (- 1).milliseconds());
        debug_assert_eq!(Duration::milliseconds(rug_fuzz_5), 1001.milliseconds());
        debug_assert_eq!(Duration::milliseconds(- rug_fuzz_6), (- 1001).milliseconds());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_138 {
    use super::*;
    use crate::*;
    use crate::duration::Duration;
    use crate::ext::NumericalDuration;
    #[test]
    fn test_minutes() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Duration::minutes(rug_fuzz_0), 60.seconds());
        debug_assert_eq!(Duration::minutes(- rug_fuzz_1), (- 60).seconds());
        debug_assert_eq!(Duration::minutes(rug_fuzz_2), 0.seconds());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_143 {
    use super::*;
    use crate::*;
    #[test]
    fn test_weeks() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i64, i64, i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Duration::weeks(rug_fuzz_0), Duration::seconds(604800));
        debug_assert_eq!(Duration::weeks(rug_fuzz_1), Duration::seconds(1209600));
        debug_assert_eq!(Duration::weeks(rug_fuzz_2), Duration::seconds(0));
        debug_assert_eq!(Duration::weeks(- rug_fuzz_3), Duration::seconds(- 604800));
        debug_assert_eq!(Duration::weeks(- rug_fuzz_4), Duration::seconds(- 1209600));
             }
});    }
}
#[cfg(test)]
mod tests_rug_3 {
    use super::*;
    use crate::ext::NumericalDuration;
    use std::time::Duration;
    #[test]
    fn test_nanoseconds() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: i64 = rug_fuzz_0;
        <i64 as NumericalDuration>::nanoseconds(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_5 {
    use super::*;
    use crate::ext::NumericalDuration;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: i64 = rug_fuzz_0;
        p0.seconds();
             }
});    }
}
#[cfg(test)]
mod tests_rug_6 {
    use super::*;
    use crate::ext::NumericalDuration;
    use crate::ext::Duration;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: i64 = rug_fuzz_0;
        i64::days(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_7 {
    use super::*;
    use crate::ext::NumericalDuration;
    use std::time::Duration;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: f64 = rug_fuzz_0;
        p0.nanoseconds();
             }
});    }
}
#[cfg(test)]
mod tests_rug_8 {
    use super::*;
    use crate::ext::NumericalDuration;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: f64 = rug_fuzz_0;
        p0.microseconds();
             }
});    }
}
#[cfg(test)]
mod unit_tests {
    use super::*;
    use crate::ext::NumericalDuration;
    use std::time::Duration;
    #[test]
    fn test_minutes() {
        let p0: f64 = 5.5;
        <f64 as NumericalDuration>::minutes(p0);
    }
}
#[cfg(test)]
mod tests_rug_10 {
    use super::*;
    use crate::ext::NumericalDuration;
    #[test]
    fn test_hours() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: f64 = rug_fuzz_0;
        p0.hours();
             }
});    }
}
#[cfg(test)]
mod tests_rug_11 {
    use super::*;
    use crate::ext::NumericalDuration;
    #[test]
    fn test_days() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: f64 = rug_fuzz_0;
        p0.days();
             }
});    }
}
#[cfg(test)]
mod tests_rug_12 {
    use super::*;
    use std::time::Duration;
    use crate::ext::NumericalStdDuration;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: u64 = rug_fuzz_0;
        <u64>::std_nanoseconds(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_13 {
    use super::*;
    use crate::ext::NumericalStdDuration;
    #[test]
    fn test_std_microseconds() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: u64 = rug_fuzz_0;
        <u64>::std_microseconds(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_14 {
    use super::*;
    use crate::ext::NumericalStdDuration;
    #[test]
    fn test_std_milliseconds() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: u64 = rug_fuzz_0;
        <u64 as NumericalStdDuration>::std_milliseconds(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_15 {
    use super::*;
    use crate::ext::NumericalStdDuration;
    use std::time::Duration as StdDuration;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: u64 = rug_fuzz_0;
        <u64>::std_seconds(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_16 {
    use super::*;
    use crate::ext::NumericalStdDuration;
    use std::time::Duration;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: u64 = rug_fuzz_0;
        <u64 as NumericalStdDuration>::std_minutes(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_17 {
    use super::*;
    use crate::ext::NumericalStdDuration;
    const P0: u64 = 100;
    #[test]
    fn test_std_hours() {
        let _rug_st_tests_rug_17_rrrruuuugggg_test_std_hours = 0;
        let p0 = P0;
        <u64 as NumericalStdDuration>::std_hours(p0);
        let _rug_ed_tests_rug_17_rrrruuuugggg_test_std_hours = 0;
    }
}
#[cfg(test)]
mod tests_rug_18 {
    use super::*;
    use crate::ext::NumericalStdDuration;
    use std::time::Duration;
    #[test]
    fn test_std_days() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: u64 = rug_fuzz_0;
        let result = p0.std_days();
        let expected = Duration::from_secs(p0 * rug_fuzz_1);
        debug_assert_eq!(result, expected);
             }
});    }
}
#[cfg(test)]
mod tests_rug_19 {
    use super::*;
    use crate::ext::NumericalStdDuration;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: u64 = rug_fuzz_0;
        p0.std_weeks();
             }
});    }
}
#[cfg(test)]
mod tests_rug_20 {
    use super::*;
    use crate::ext::NumericalStdDuration;
    use std::time::Duration;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: f64 = rug_fuzz_0;
        p0.std_nanoseconds();
             }
});    }
}
#[cfg(test)]
mod tests_rug_21 {
    use super::*;
    use crate::ext::NumericalStdDuration;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: f64 = rug_fuzz_0;
        <f64>::std_microseconds(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_22 {
    use super::*;
    use crate::ext::NumericalStdDuration;
    #[test]
    fn test_std_milliseconds() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: f64 = rug_fuzz_0;
        let result = p0.std_milliseconds();
        debug_assert_eq!(result.as_millis(), 1234);
             }
});    }
}
#[cfg(test)]
mod tests_rug_23 {
    use super::*;
    use crate::ext::NumericalStdDuration;
    use std::time::Duration;
    #[test]
    fn test_std_seconds() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: f64 = rug_fuzz_0;
        <f64 as NumericalStdDuration>::std_seconds(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_24 {
    use super::*;
    use crate::ext::NumericalStdDuration;
    use std::time::Duration;
    #[test]
    fn test_std_minutes() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: f64 = rug_fuzz_0;
        let result = p0.std_minutes();
        let expected_result = Duration::from_secs(rug_fuzz_1);
        debug_assert_eq!(result, expected_result);
             }
});    }
}
#[cfg(test)]
mod tests_rug_25 {
    use super::*;
    use crate::ext::NumericalStdDuration;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: f64 = rug_fuzz_0;
        <f64 as NumericalStdDuration>::std_hours(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_26 {
    use super::*;
    use crate::ext::NumericalStdDuration;
    use std::time::Duration;
    #[test]
    fn test_std_days() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: f64 = rug_fuzz_0;
        let result = p0.std_days();
        debug_assert_eq!(result, Duration::from_secs(1066656000));
             }
});    }
}
#[cfg(test)]
mod tests_rug_27 {
    use super::*;
    use crate::ext::NumericalStdDuration;
    use std::time::Duration;
    #[test]
    fn test_std_weeks() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: f64 = rug_fuzz_0;
        p0.std_weeks();
             }
});    }
}
