//! The [`Instant`] struct and its associated `impl`s.
use core::borrow::Borrow;
use core::cmp::{Ord, Ordering, PartialEq, PartialOrd};
use core::ops::{Add, Sub};
use core::time::Duration as StdDuration;
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
            self.0.checked_add(duration.unsigned_abs()).map(Self)
        } else {
            debug_assert!(duration.is_negative());
            self.0.checked_sub(duration.unsigned_abs()).map(Self)
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
            self.0.checked_sub(duration.unsigned_abs()).map(Self)
        } else {
            debug_assert!(duration.is_negative());
            self.0.checked_add(duration.unsigned_abs()).map(Self)
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
    /// # Panics
    ///
    /// This function may panic if the resulting point in time cannot be represented by the
    /// underlying data structure.
    fn add(self, duration: Duration) -> Self::Output {
        if duration.is_positive() {
            Self(self.0 + duration.unsigned_abs())
        } else if duration.is_negative() {
            #[allow(clippy::unchecked_duration_subtraction)]
            Self(self.0 - duration.unsigned_abs())
        } else {
            debug_assert!(duration.is_zero());
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
    /// # Panics
    ///
    /// This function may panic if the resulting point in time cannot be represented by the
    /// underlying data structure.
    fn sub(self, duration: Duration) -> Self::Output {
        if duration.is_positive() {
            #[allow(clippy::unchecked_duration_subtraction)]
            Self(self.0 - duration.unsigned_abs())
        } else if duration.is_negative() {
            Self(self.0 + duration.unsigned_abs())
        } else {
            debug_assert!(duration.is_zero());
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
    /// # Panics
    ///
    /// This function may panic if the resulting point in time cannot be represented by the
    /// underlying data structure.
    fn sub(self, duration: StdDuration) -> Self::Output {
        #[allow(clippy::unchecked_duration_subtraction)] Self(self.0 - duration)
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
mod tests_rug_172 {
    use super::*;
    #[test]
    fn test_instant_now() {
        let _rug_st_tests_rug_172_rrrruuuugggg_test_instant_now = 0;
        let instant = crate::instant::Instant::now();
        debug_assert!(instant.elapsed() >= crate ::Duration::ZERO);
        let _rug_ed_tests_rug_172_rrrruuuugggg_test_instant_now = 0;
    }
}
#[cfg(test)]
mod tests_rug_173 {
    use crate::{Instant, ext::{NumericalStdDuration, NumericalDuration}};
    #[test]
    fn test_elapsed() {
        let _rug_st_tests_rug_173_rrrruuuugggg_test_elapsed = 0;
        let mut p0 = Instant::now();
        p0.elapsed();
        let _rug_ed_tests_rug_173_rrrruuuugggg_test_elapsed = 0;
    }
}
#[cfg(test)]
mod tests_rug_174 {
    use super::*;
    use crate::{Duration, Instant};
    #[test]
    fn test_checked_add() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Instant::now();
        let mut p1 = Duration::minutes(rug_fuzz_0);
        let _result = Instant::checked_add(p0, p1);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_175 {
    use super::*;
    use crate::Duration;
    use crate::Instant;
    #[test]
    fn test_checked_sub() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Instant::now();
        let mut p1 = Duration::minutes(rug_fuzz_0);
        debug_assert_eq!(
            p0.checked_sub(p1), p0.0.checked_sub(p1.unsigned_abs()).map(Instant)
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_176 {
    use super::*;
    use crate::Instant;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_176_rrrruuuugggg_test_rug = 0;
        let mut p0 = Instant::now();
        debug_assert_eq!(Instant::into_inner(p0), p0.0);
        let _rug_ed_tests_rug_176_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_178 {
    use super::*;
    use std::convert::From;
    use crate::Instant;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_178_rrrruuuugggg_test_rug = 0;
        let mut p0 = Instant::now();
        let std_instant = <std::time::Instant>::from(p0);
        let _rug_ed_tests_rug_178_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_179 {
    use super::*;
    use std::cmp::Ordering;
    use std::time::Duration;
    use std::ops::Sub;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_179_rrrruuuugggg_test_rug = 0;
        let mut p0 = Instant::now();
        let mut p1 = Instant::now();
        <Instant as Sub>::sub(p0, p1);
        let _rug_ed_tests_rug_179_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_180 {
    use super::*;
    use std::ops::Sub;
    use std::time::Instant as StdInstant;
    use crate::Instant;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_180_rrrruuuugggg_test_rug = 0;
        let p0 = Instant::now();
        let p1 = StdInstant::now();
        <Instant as Sub<StdInstant>>::sub(p0, p1);
        let _rug_ed_tests_rug_180_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_181 {
    use super::*;
    use std::ops::Sub;
    use std::time::Instant as StdInstant;
    use crate::Instant as TimeInstant;
    #[test]
    fn test_sub() {
        let _rug_st_tests_rug_181_rrrruuuugggg_test_sub = 0;
        let mut p0 = StdInstant::now();
        let mut p1 = TimeInstant::now();
        <StdInstant as Sub<TimeInstant>>::sub(p0, p1);
        let _rug_ed_tests_rug_181_rrrruuuugggg_test_sub = 0;
    }
}
#[cfg(test)]
mod tests_rug_182 {
    use super::*;
    use crate::duration::Duration;
    use crate::instant::Instant;
    use std::ops::Add;
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Instant::now();
        let mut p1 = Duration::minutes(rug_fuzz_0);
        <Instant as Add<Duration>>::add(p0, p1);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_183 {
    use std::ops::Add;
    use std::time::Instant;
    use crate::Duration;
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: Instant = Instant::now();
        let mut p1: Duration = Duration::minutes(rug_fuzz_0);
        <Instant as Add<Duration>>::add(p0, p1);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_184 {
    use super::*;
    use std::ops::Add;
    use std::time::Duration as StdDuration;
    use crate::instant::Instant;
    #[test]
    fn test_add() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Instant::now();
        let mut p1 = StdDuration::new(rug_fuzz_0, rug_fuzz_1);
        <Instant as Add<StdDuration>>::add(p0, p1);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_185 {
    use super::*;
    use std::ops::Sub;
    use crate::{Duration, Instant};
    #[test]
    fn test_sub() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Instant::now();
        let mut p1 = Duration::minutes(rug_fuzz_0);
        <Instant as Sub<Duration>>::sub(p0, p1);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_186 {
    use std::ops::Sub;
    use std::time::Instant;
    use crate::Duration;
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Instant::now();
        let mut p1 = Duration::minutes(rug_fuzz_0);
        Instant::sub(p0, p1);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_187 {
    use super::*;
    use std::ops::Sub;
    use std::time::Duration as StdDuration;
    use crate::Instant;
    #[test]
    fn test_sub() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0 = Instant::now();
        let p1 = StdDuration::new(rug_fuzz_0, rug_fuzz_1);
        <Instant as Sub<StdDuration>>::sub(p0, p1);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_188 {
    use crate::instant::Instant;
    use std::time::Instant as StdInstant;
    use std::cmp::PartialEq;
    #[test]
    fn test_eq() {
        let _rug_st_tests_rug_188_rrrruuuugggg_test_eq = 0;
        let mut p0 = Instant::now();
        let mut p1 = StdInstant::now();
        debug_assert_eq!(
            < Instant as PartialEq < StdInstant > > ::eq(& p0, & p1), p0.0.eq(& p1)
        );
        let _rug_ed_tests_rug_188_rrrruuuugggg_test_eq = 0;
    }
}
#[cfg(test)]
mod tests_rug_189 {
    use super::*;
    use std::cmp::PartialEq;
    use std::time::Instant as StdInstant;
    use crate::Instant as TimeInstant;
    #[test]
    fn test_eq() {
        let _rug_st_tests_rug_189_rrrruuuugggg_test_eq = 0;
        let mut p0: StdInstant = StdInstant::now();
        let mut p1: TimeInstant = TimeInstant::now();
        debug_assert!(< StdInstant as PartialEq < TimeInstant > > ::eq(& p0, & p1));
        let _rug_ed_tests_rug_189_rrrruuuugggg_test_eq = 0;
    }
}
#[cfg(test)]
mod tests_rug_190 {
    use super::*;
    use std::cmp::Ordering;
    use crate::Instant as TimeInstant;
    use std::time::Instant as StdInstant;
    #[test]
    fn test_partial_cmp() {
        let _rug_st_tests_rug_190_rrrruuuugggg_test_partial_cmp = 0;
        let mut p0 = TimeInstant::now();
        let mut p1 = StdInstant::now();
        debug_assert!(p0.partial_cmp(& p1).is_some());
        let _rug_ed_tests_rug_190_rrrruuuugggg_test_partial_cmp = 0;
    }
}
#[cfg(test)]
mod tests_rug_191 {
    use std::cmp::Ordering;
    use std::time::Instant as StdInstant;
    use crate::Instant as TimeInstant;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_191_rrrruuuugggg_test_rug = 0;
        let p0: StdInstant = StdInstant::now();
        let p1: TimeInstant = TimeInstant::now();
        <StdInstant as PartialOrd<TimeInstant>>::partial_cmp(&p0, &p1);
        let _rug_ed_tests_rug_191_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_192 {
    use super::*;
    use std::convert::AsRef;
    #[test]
    fn test_as_ref() {
        let _rug_st_tests_rug_192_rrrruuuugggg_test_as_ref = 0;
        let p0 = Instant::now();
        debug_assert_eq!(
            < Instant as std::convert::AsRef < std::time::Instant > > ::as_ref(& p0), &
            p0.0
        );
        let _rug_ed_tests_rug_192_rrrruuuugggg_test_as_ref = 0;
    }
}
#[cfg(test)]
mod tests_rug_193 {
    use super::*;
    use std::borrow::Borrow;
    use crate::Instant;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_193_rrrruuuugggg_test_rug = 0;
        let mut p0: Instant = Instant::now();
        <Instant as std::borrow::Borrow<std::time::Instant>>::borrow(&p0);
        let _rug_ed_tests_rug_193_rrrruuuugggg_test_rug = 0;
    }
}
