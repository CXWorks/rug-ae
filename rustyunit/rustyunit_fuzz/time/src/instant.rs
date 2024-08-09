//! The [`Instant`] struct and its associated `impl`s.
use core::cmp::{Ord, Ordering, PartialEq, PartialOrd};
use core::convert::{TryFrom, TryInto};
use core::ops::{Add, Sub};
use core::time::Duration as StdDuration;
use std::borrow::Borrow;
use std::time::Instant as StdInstant;
use crate::Duration;
/// A measurement of a monotonically non-decreasing clock. Opaque and useful only with [`Duration`].
///
/// Instants are always guaranteed to be no less than any previously measured instant when created,
/// and are often useful for tasks such as measuring benchmarks or timing how long an operation
/// takes.
///
/// Note, however, that instants are not guaranteed to be **steady**. In other words, each tick of
/// the underlying clock may not be the same length (e.g. some seconds may be longer than others).
/// An instant may jump forwards or experience time dilation (slow down or speed up), but it will
/// never go backwards.
///
/// Instants are opaque types that can only be compared to one another. There is no method to get
/// "the number of seconds" from an instant. Instead, it only allows measuring the duration between
/// two instants (or comparing two instants).
///
/// This implementation allows for operations with signed [`Duration`]s, but is otherwise identical
/// to [`std::time::Instant`].
#[cfg_attr(__time_03_docs, doc(cfg(feature = "std")))]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instant(pub StdInstant);
impl Instant {
    /// Returns an `Instant` corresponding to "now".
    ///
    /// ```rust
    /// # use time::Instant;
    /// println!("{:?}", Instant::now());
    /// ```
    pub fn now() -> Self {
        Self(StdInstant::now())
    }
    /// Returns the amount of time elapsed since this instant was created. The duration will always
    /// be nonnegative if the instant is not synthetically created.
    ///
    /// ```rust
    /// # use time::{Instant, ext::{NumericalStdDuration, NumericalDuration}};
    /// # use std::thread;
    /// let instant = Instant::now();
    /// thread::sleep(1.std_milliseconds());
    /// assert!(instant.elapsed() >= 1.milliseconds());
    /// ```
    pub fn elapsed(self) -> Duration {
        Self::now() - self
    }
    /// Returns `Some(t)` where `t` is the time `self + duration` if `t` can be represented as
    /// `Instant` (which means it's inside the bounds of the underlying data structure), `None`
    /// otherwise.
    ///
    /// ```rust
    /// # use time::{Instant, ext::NumericalDuration};
    /// let now = Instant::now();
    /// assert_eq!(now.checked_add(5.seconds()), Some(now + 5.seconds()));
    /// assert_eq!(now.checked_add((-5).seconds()), Some(now + (-5).seconds()));
    /// ```
    pub fn checked_add(self, duration: Duration) -> Option<Self> {
        if duration.is_zero() {
            Some(self)
        } else if duration.is_positive() {
            self.0.checked_add(duration.abs_std()).map(Self)
        } else {
            debug_assert!(duration.is_negative());
            self.0.checked_sub(duration.abs_std()).map(Self)
        }
    }
    /// Returns `Some(t)` where `t` is the time `self - duration` if `t` can be represented as
    /// `Instant` (which means it's inside the bounds of the underlying data structure), `None`
    /// otherwise.
    ///
    /// ```rust
    /// # use time::{Instant, ext::NumericalDuration};
    /// let now = Instant::now();
    /// assert_eq!(now.checked_sub(5.seconds()), Some(now - 5.seconds()));
    /// assert_eq!(now.checked_sub((-5).seconds()), Some(now - (-5).seconds()));
    /// ```
    pub fn checked_sub(self, duration: Duration) -> Option<Self> {
        if duration.is_zero() {
            Some(self)
        } else if duration.is_positive() {
            self.0.checked_sub(duration.abs_std()).map(Self)
        } else {
            debug_assert!(duration.is_negative());
            self.0.checked_add(duration.abs_std()).map(Self)
        }
    }
    /// Obtain the inner [`std::time::Instant`].
    ///
    /// ```rust
    /// # use time::Instant;
    /// let now = Instant::now();
    /// assert_eq!(now.into_inner(), now.0);
    /// ```
    pub const fn into_inner(self) -> StdInstant {
        self.0
    }
}
impl From<StdInstant> for Instant {
    fn from(instant: StdInstant) -> Self {
        Self(instant)
    }
}
impl From<Instant> for StdInstant {
    fn from(instant: Instant) -> Self {
        instant.0
    }
}
impl Sub for Instant {
    type Output = Duration;
    fn sub(self, other: Self) -> Self::Output {
        match self.0.cmp(&other.0) {
            Ordering::Equal => Duration::ZERO,
            Ordering::Greater => {
                (self.0 - other.0)
                    .try_into()
                    .expect(
                        "overflow converting `std::time::Duration` to `time::Duration`",
                    )
            }
            Ordering::Less => {
                -Duration::try_from(other.0 - self.0)
                    .expect(
                        "overflow converting `std::time::Duration` to `time::Duration`",
                    )
            }
        }
    }
}
impl Sub<StdInstant> for Instant {
    type Output = Duration;
    fn sub(self, other: StdInstant) -> Self::Output {
        self - Self(other)
    }
}
impl Sub<Instant> for StdInstant {
    type Output = Duration;
    fn sub(self, other: Instant) -> Self::Output {
        Instant(self) - other
    }
}
impl Add<Duration> for Instant {
    type Output = Self;
    fn add(self, duration: Duration) -> Self::Output {
        if duration.is_positive() {
            Self(self.0 + duration.abs_std())
        } else if duration.is_negative() {
            Self(self.0 - duration.abs_std())
        } else {
            self
        }
    }
}
impl Add<Duration> for StdInstant {
    type Output = Self;
    fn add(self, duration: Duration) -> Self::Output {
        (Instant(self) + duration).0
    }
}
impl Add<StdDuration> for Instant {
    type Output = Self;
    fn add(self, duration: StdDuration) -> Self::Output {
        Self(self.0 + duration)
    }
}
impl_add_assign!(Instant : Duration, StdDuration);
impl_add_assign!(StdInstant : Duration);
impl Sub<Duration> for Instant {
    type Output = Self;
    fn sub(self, duration: Duration) -> Self::Output {
        if duration.is_positive() {
            Self(self.0 - duration.abs_std())
        } else if duration.is_negative() {
            Self(self.0 + duration.abs_std())
        } else {
            self
        }
    }
}
impl Sub<Duration> for StdInstant {
    type Output = Self;
    fn sub(self, duration: Duration) -> Self::Output {
        (Instant(self) - duration).0
    }
}
impl Sub<StdDuration> for Instant {
    type Output = Self;
    fn sub(self, duration: StdDuration) -> Self::Output {
        Self(self.0 - duration)
    }
}
impl_sub_assign!(Instant : Duration, StdDuration);
impl_sub_assign!(StdInstant : Duration);
impl PartialEq<StdInstant> for Instant {
    fn eq(&self, rhs: &StdInstant) -> bool {
        self.0.eq(rhs)
    }
}
impl PartialEq<Instant> for StdInstant {
    fn eq(&self, rhs: &Instant) -> bool {
        self.eq(&rhs.0)
    }
}
impl PartialOrd<StdInstant> for Instant {
    fn partial_cmp(&self, rhs: &StdInstant) -> Option<Ordering> {
        self.0.partial_cmp(rhs)
    }
}
impl PartialOrd<Instant> for StdInstant {
    fn partial_cmp(&self, rhs: &Instant) -> Option<Ordering> {
        self.partial_cmp(&rhs.0)
    }
}
impl AsRef<StdInstant> for Instant {
    fn as_ref(&self) -> &StdInstant {
        &self.0
    }
}
impl Borrow<StdInstant> for Instant {
    fn borrow(&self) -> &StdInstant {
        &self.0
    }
}
#[cfg(test)]
mod tests_llm_16_148 {
    use crate::instant::*;
    use std::cmp::Ordering;
    #[test]
    fn test_partial_cmp() {
        let _rug_st_tests_llm_16_148_rrrruuuugggg_test_partial_cmp = 0;
        let instant1 = Instant::from(StdInstant::now());
        let instant2 = Instant::from(StdInstant::now());
        let result = instant1.partial_cmp(&instant2);
        debug_assert_eq!(result, Some(Ordering::Equal));
        let _rug_ed_tests_llm_16_148_rrrruuuugggg_test_partial_cmp = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_151 {
    use super::*;
    use crate::*;
    #[test]
    fn test_from() {
        let _rug_st_tests_llm_16_151_rrrruuuugggg_test_from = 0;
        let std_instant = StdInstant::now();
        let instant = Instant::from(std_instant);
        debug_assert_eq!(instant.0, std_instant);
        let _rug_ed_tests_llm_16_151_rrrruuuugggg_test_from = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_160 {
    use super::*;
    use crate::*;
    #[test]
    fn test_sub_positive_duration() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let instant = Instant::from(StdInstant::now());
        let duration = Duration::seconds(rug_fuzz_0);
        let result = instant - duration;
        let expected = Instant::from(StdInstant::now() - duration.abs_std());
        debug_assert_eq!(result, expected);
             }
});    }
    #[test]
    fn test_sub_negative_duration() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let instant = Instant::from(StdInstant::now());
        let duration = Duration::seconds(-rug_fuzz_0);
        let result = instant - duration;
        let expected = Instant::from(StdInstant::now() + duration.abs_std());
        debug_assert_eq!(result, expected);
             }
});    }
    #[test]
    fn test_sub_zero_duration() {
        let _rug_st_tests_llm_16_160_rrrruuuugggg_test_sub_zero_duration = 0;
        let instant = Instant::from(StdInstant::now());
        let duration = Duration::ZERO;
        let result = instant - duration;
        debug_assert_eq!(result, instant);
        let _rug_ed_tests_llm_16_160_rrrruuuugggg_test_sub_zero_duration = 0;
    }
    #[test]
    fn test_checked_sub_positive_duration() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let instant = Instant::from(StdInstant::now());
        let duration = Duration::seconds(rug_fuzz_0);
        let result = instant.checked_sub(duration);
        let expected = Some(Instant::from(StdInstant::now() - duration.abs_std()));
        debug_assert_eq!(result, expected);
             }
});    }
    #[test]
    fn test_checked_sub_negative_duration() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let instant = Instant::from(StdInstant::now());
        let duration = Duration::seconds(-rug_fuzz_0);
        let result = instant.checked_sub(duration);
        let expected = Some(Instant::from(StdInstant::now() + duration.abs_std()));
        debug_assert_eq!(result, expected);
             }
});    }
    #[test]
    fn test_checked_sub_zero_duration() {
        let _rug_st_tests_llm_16_160_rrrruuuugggg_test_checked_sub_zero_duration = 0;
        let instant = Instant::from(StdInstant::now());
        let duration = Duration::ZERO;
        let result = instant.checked_sub(duration);
        debug_assert_eq!(result, Some(instant));
        let _rug_ed_tests_llm_16_160_rrrruuuugggg_test_checked_sub_zero_duration = 0;
    }
    #[test]
    fn test_into_inner() {
        let _rug_st_tests_llm_16_160_rrrruuuugggg_test_into_inner = 0;
        let instant = Instant::from(StdInstant::now());
        let result = instant.into_inner();
        debug_assert_eq!(result, instant.0);
        let _rug_ed_tests_llm_16_160_rrrruuuugggg_test_into_inner = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_341 {
    use super::*;
    use crate::*;
    use crate::{Instant, Duration};
    use std::cmp::PartialEq;
    #[test]
    fn test_eq_returns_true_if_self_is_equal_to_rhs() {
        let _rug_st_tests_llm_16_341_rrrruuuugggg_test_eq_returns_true_if_self_is_equal_to_rhs = 0;
        let lhs = Instant::now();
        let rhs = lhs;
        debug_assert_eq!(lhs.eq(& rhs), true);
        let _rug_ed_tests_llm_16_341_rrrruuuugggg_test_eq_returns_true_if_self_is_equal_to_rhs = 0;
    }
    #[test]
    fn test_eq_returns_false_if_self_is_not_equal_to_rhs() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let lhs = Instant::now();
        let rhs = lhs + Duration::seconds(rug_fuzz_0);
        debug_assert_eq!(lhs.eq(& rhs), false);
             }
});    }
    #[test]
    fn test_eq_returns_true_if_self_is_equal_to_rhs_std_instant() {
        let _rug_st_tests_llm_16_341_rrrruuuugggg_test_eq_returns_true_if_self_is_equal_to_rhs_std_instant = 0;
        let lhs = Instant::now();
        let rhs = lhs.into_inner();
        debug_assert_eq!(lhs.eq(& rhs), true);
        let _rug_ed_tests_llm_16_341_rrrruuuugggg_test_eq_returns_true_if_self_is_equal_to_rhs_std_instant = 0;
    }
    #[test]
    fn test_eq_returns_false_if_self_is_not_equal_to_rhs_std_instant() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let lhs = Instant::now();
        let rhs = lhs.into_inner() + Duration::seconds(rug_fuzz_0);
        debug_assert_eq!(lhs.eq(& rhs), false);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_342 {
    use super::*;
    use crate::*;
    use std::cmp::Ordering;
    #[test]
    fn test_partial_cmp() {
        let _rug_st_tests_llm_16_342_rrrruuuugggg_test_partial_cmp = 0;
        let instant1 = Instant::now();
        let instant2 = Instant::now();
        let result = instant1.partial_cmp(&instant2);
        debug_assert!(
            result == Some(Ordering::Less) || result == Some(Ordering::Equal) || result
            == Some(Ordering::Greater)
        );
        let _rug_ed_tests_llm_16_342_rrrruuuugggg_test_partial_cmp = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_346 {
    use super::*;
    use crate::*;
    #[test]
    fn test_add() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let instant = Instant::now();
        let duration = Duration::seconds(rug_fuzz_0);
        let result = instant.add(duration);
        debug_assert_eq!(result, instant + duration);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_357 {
    use super::*;
    use crate::*;
    #[test]
    fn test_elapsed() {
        let _rug_st_tests_llm_16_357_rrrruuuugggg_test_elapsed = 0;
        let instant = Instant::now();
        let elapsed = instant.elapsed();
        debug_assert!(elapsed >= Duration::ZERO);
        let _rug_ed_tests_llm_16_357_rrrruuuugggg_test_elapsed = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_360 {
    use crate::{Instant, Duration};
    #[test]
    fn test_now() {
        let _rug_st_tests_llm_16_360_rrrruuuugggg_test_now = 0;
        let now = Instant::now();
        debug_assert!(now.elapsed() >= Duration::ZERO);
        let _rug_ed_tests_llm_16_360_rrrruuuugggg_test_now = 0;
    }
}
#[cfg(test)]
mod tests_rug_127 {
    use super::*;
    use crate::{Instant, Duration, ext::NumericalDuration};
    #[test]
    fn test_checked_add() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Instant::now();
        let mut p1 = Duration::new(rug_fuzz_0, rug_fuzz_1);
        p0.checked_add(p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_128 {
    use super::*;
    use crate::{Instant, Duration, ext::NumericalDuration};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Instant::now();
        let mut p1 = Duration::new(rug_fuzz_0, rug_fuzz_1);
        <Instant>::checked_sub(p0, p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_129 {
    use super::*;
    use crate::Instant;
    use std::time::Instant as StdInstant;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_129_rrrruuuugggg_test_rug = 0;
        let mut p0 = Instant::now();
        <Instant>::into_inner(p0);
        let _rug_ed_tests_rug_129_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_131 {
    use super::*;
    use std::cmp::Ordering;
    use std::convert::{TryFrom, TryInto};
    use crate::{Duration, Instant};
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_131_rrrruuuugggg_test_rug = 0;
        let p0 = Instant::now();
        let p1 = Instant::now();
        p0.sub(p1);
        let _rug_ed_tests_rug_131_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_132 {
    use super::*;
    use std::ops::Sub;
    use crate::Instant;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_132_rrrruuuugggg_test_rug = 0;
        let mut p0 = Instant::now();
        let mut p1 = std::time::Instant::now();
        p0.sub(p1);
        let _rug_ed_tests_rug_132_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_133 {
    use super::*;
    use std::ops::Sub;
    use std::time::Instant;
    use crate::Instant as OtherInstant;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_133_rrrruuuugggg_test_rug = 0;
        let mut p0 = Instant::now();
        let mut p1 = OtherInstant::now();
        <Instant as Sub<OtherInstant>>::sub(p0, p1);
        let _rug_ed_tests_rug_133_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_136 {
    use super::*;
    use std::time::Instant;
    use crate::{Duration, ext::NumericalDuration};
    #[test]
    fn test_sub() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Instant::now();
        let mut p1 = Duration::new(rug_fuzz_0, rug_fuzz_1);
        <std::time::Instant>::sub(p0, p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_137 {
    use super::*;
    use crate::Instant;
    use std::time::Duration;
    use std::ops::Sub;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Instant::now();
        let p1 = Duration::new(rug_fuzz_0, rug_fuzz_1);
        p0.sub(p1);
             }
});    }
}
