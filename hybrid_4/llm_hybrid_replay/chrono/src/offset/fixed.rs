//! The time zone which has a fixed offset from UTC.
use core::fmt;
use core::ops::{Add, Sub};
#[cfg(feature = "rkyv")]
use rkyv::{Archive, Deserialize, Serialize};
use super::{LocalResult, Offset, TimeZone};
use crate::naive::{NaiveDate, NaiveDateTime, NaiveTime};
use crate::time_delta::TimeDelta;
use crate::DateTime;
use crate::Timelike;
/// The time zone with fixed offset, from UTC-23:59:59 to UTC+23:59:59.
///
/// Using the [`TimeZone`](./trait.TimeZone.html) methods
/// on a `FixedOffset` struct is the preferred way to construct
/// `DateTime<FixedOffset>` instances. See the [`east_opt`](#method.east_opt) and
/// [`west_opt`](#method.west_opt) methods for examples.
#[derive(PartialEq, Eq, Hash, Copy, Clone)]
#[cfg_attr(feature = "rkyv", derive(Archive, Deserialize, Serialize))]
pub struct FixedOffset {
    local_minus_utc: i32,
}
impl FixedOffset {
    /// Makes a new `FixedOffset` for the Eastern Hemisphere with given timezone difference.
    /// The negative `secs` means the Western Hemisphere.
    ///
    /// Panics on the out-of-bound `secs`.
    #[deprecated(since = "0.4.23", note = "use `east_opt()` instead")]
    #[must_use]
    pub fn east(secs: i32) -> FixedOffset {
        FixedOffset::east_opt(secs).expect("FixedOffset::east out of bounds")
    }
    /// Makes a new `FixedOffset` for the Eastern Hemisphere with given timezone difference.
    /// The negative `secs` means the Western Hemisphere.
    ///
    /// Returns `None` on the out-of-bound `secs`.
    ///
    /// # Example
    ///
    #[cfg_attr(not(feature = "std"), doc = "```ignore")]
    #[cfg_attr(feature = "std", doc = "```")]
    /// use chrono::{FixedOffset, TimeZone};
    /// let hour = 3600;
    /// let datetime = FixedOffset::east_opt(5 * hour).unwrap().ymd_opt(2016, 11, 08).unwrap()
    ///                                           .and_hms_opt(0, 0, 0).unwrap();
    /// assert_eq!(&datetime.to_rfc3339(), "2016-11-08T00:00:00+05:00")
    /// ```
    #[must_use]
    pub const fn east_opt(secs: i32) -> Option<FixedOffset> {
        if -86_400 < secs && secs < 86_400 {
            Some(FixedOffset {
                local_minus_utc: secs,
            })
        } else {
            None
        }
    }
    /// Makes a new `FixedOffset` for the Western Hemisphere with given timezone difference.
    /// The negative `secs` means the Eastern Hemisphere.
    ///
    /// Panics on the out-of-bound `secs`.
    #[deprecated(since = "0.4.23", note = "use `west_opt()` instead")]
    #[must_use]
    pub fn west(secs: i32) -> FixedOffset {
        FixedOffset::west_opt(secs).expect("FixedOffset::west out of bounds")
    }
    /// Makes a new `FixedOffset` for the Western Hemisphere with given timezone difference.
    /// The negative `secs` means the Eastern Hemisphere.
    ///
    /// Returns `None` on the out-of-bound `secs`.
    ///
    /// # Example
    ///
    #[cfg_attr(not(feature = "std"), doc = "```ignore")]
    #[cfg_attr(feature = "std", doc = "```")]
    /// use chrono::{FixedOffset, TimeZone};
    /// let hour = 3600;
    /// let datetime = FixedOffset::west_opt(5 * hour).unwrap().ymd_opt(2016, 11, 08).unwrap()
    ///                                           .and_hms_opt(0, 0, 0).unwrap();
    /// assert_eq!(&datetime.to_rfc3339(), "2016-11-08T00:00:00-05:00")
    /// ```
    #[must_use]
    pub const fn west_opt(secs: i32) -> Option<FixedOffset> {
        if -86_400 < secs && secs < 86_400 {
            Some(FixedOffset {
                local_minus_utc: -secs,
            })
        } else {
            None
        }
    }
    /// Returns the number of seconds to add to convert from UTC to the local time.
    #[inline]
    pub const fn local_minus_utc(&self) -> i32 {
        self.local_minus_utc
    }
    /// Returns the number of seconds to add to convert from the local time to UTC.
    #[inline]
    pub const fn utc_minus_local(&self) -> i32 {
        -self.local_minus_utc
    }
}
impl TimeZone for FixedOffset {
    type Offset = FixedOffset;
    fn from_offset(offset: &FixedOffset) -> FixedOffset {
        *offset
    }
    fn offset_from_local_date(&self, _local: &NaiveDate) -> LocalResult<FixedOffset> {
        LocalResult::Single(*self)
    }
    fn offset_from_local_datetime(
        &self,
        _local: &NaiveDateTime,
    ) -> LocalResult<FixedOffset> {
        LocalResult::Single(*self)
    }
    fn offset_from_utc_date(&self, _utc: &NaiveDate) -> FixedOffset {
        *self
    }
    fn offset_from_utc_datetime(&self, _utc: &NaiveDateTime) -> FixedOffset {
        *self
    }
}
impl Offset for FixedOffset {
    fn fix(&self) -> FixedOffset {
        *self
    }
}
impl fmt::Debug for FixedOffset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let offset = self.local_minus_utc;
        let (sign, offset) = if offset < 0 { ('-', -offset) } else { ('+', offset) };
        let sec = offset.rem_euclid(60);
        let mins = offset.div_euclid(60);
        let min = mins.rem_euclid(60);
        let hour = mins.div_euclid(60);
        if sec == 0 {
            write!(f, "{}{:02}:{:02}", sign, hour, min)
        } else {
            write!(f, "{}{:02}:{:02}:{:02}", sign, hour, min, sec)
        }
    }
}
impl fmt::Display for FixedOffset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
#[cfg(feature = "arbitrary")]
impl arbitrary::Arbitrary<'_> for FixedOffset {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<FixedOffset> {
        let secs = u.int_in_range(-86_399..=86_399)?;
        let fixed_offset = FixedOffset::east_opt(secs)
            .expect(
                "Could not generate a valid chrono::FixedOffset. It looks like implementation of Arbitrary for FixedOffset is erroneous.",
            );
        Ok(fixed_offset)
    }
}
fn add_with_leapsecond<T>(lhs: &T, rhs: i32) -> T
where
    T: Timelike + Add<TimeDelta, Output = T>,
{
    let nanos = lhs.nanosecond();
    let lhs = lhs.with_nanosecond(0).unwrap();
    (lhs + TimeDelta::seconds(i64::from(rhs))).with_nanosecond(nanos).unwrap()
}
impl Add<FixedOffset> for NaiveTime {
    type Output = NaiveTime;
    #[inline]
    fn add(self, rhs: FixedOffset) -> NaiveTime {
        add_with_leapsecond(&self, rhs.local_minus_utc)
    }
}
impl Sub<FixedOffset> for NaiveTime {
    type Output = NaiveTime;
    #[inline]
    fn sub(self, rhs: FixedOffset) -> NaiveTime {
        add_with_leapsecond(&self, -rhs.local_minus_utc)
    }
}
impl Add<FixedOffset> for NaiveDateTime {
    type Output = NaiveDateTime;
    #[inline]
    fn add(self, rhs: FixedOffset) -> NaiveDateTime {
        add_with_leapsecond(&self, rhs.local_minus_utc)
    }
}
impl Sub<FixedOffset> for NaiveDateTime {
    type Output = NaiveDateTime;
    #[inline]
    fn sub(self, rhs: FixedOffset) -> NaiveDateTime {
        add_with_leapsecond(&self, -rhs.local_minus_utc)
    }
}
impl<Tz: TimeZone> Add<FixedOffset> for DateTime<Tz> {
    type Output = DateTime<Tz>;
    #[inline]
    fn add(self, rhs: FixedOffset) -> DateTime<Tz> {
        add_with_leapsecond(&self, rhs.local_minus_utc)
    }
}
impl<Tz: TimeZone> Sub<FixedOffset> for DateTime<Tz> {
    type Output = DateTime<Tz>;
    #[inline]
    fn sub(self, rhs: FixedOffset) -> DateTime<Tz> {
        add_with_leapsecond(&self, -rhs.local_minus_utc)
    }
}
#[cfg(test)]
mod tests {
    use super::FixedOffset;
    use crate::offset::TimeZone;
    #[test]
    fn test_date_extreme_offset() {
        assert_eq!(
            format!("{:?}", FixedOffset::east_opt(86399).unwrap().with_ymd_and_hms(2012,
            2, 29, 5, 6, 7).unwrap()), "2012-02-29T05:06:07+23:59:59".to_string()
        );
        assert_eq!(
            format!("{:?}", FixedOffset::east_opt(86399).unwrap().with_ymd_and_hms(2012,
            2, 29, 5, 6, 7).unwrap()), "2012-02-29T05:06:07+23:59:59".to_string()
        );
        assert_eq!(
            format!("{:?}", FixedOffset::west_opt(86399).unwrap().with_ymd_and_hms(2012,
            3, 4, 5, 6, 7).unwrap()), "2012-03-04T05:06:07-23:59:59".to_string()
        );
        assert_eq!(
            format!("{:?}", FixedOffset::west_opt(86399).unwrap().with_ymd_and_hms(2012,
            3, 4, 5, 6, 7).unwrap()), "2012-03-04T05:06:07-23:59:59".to_string()
        );
    }
}
#[cfg(test)]
mod tests_llm_16_170_llm_16_170 {
    use super::*;
    use crate::*;
    use crate::offset::TimeZone;
    #[test]
    fn test_fixed_offset_fix() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let offset_seconds: i32 = rug_fuzz_0;
        let fixed_offset = FixedOffset::east(offset_seconds).fix();
        debug_assert_eq!(fixed_offset, FixedOffset::east(offset_seconds));
        let offset_seconds: i32 = -rug_fuzz_1;
        let fixed_offset = FixedOffset::west(offset_seconds.abs()).fix();
        debug_assert_eq!(fixed_offset, FixedOffset::west(offset_seconds.abs()));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_171 {
    use crate::{offset::fixed::FixedOffset, TimeZone};
    #[test]
    fn from_offset_returns_same_offset() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let offset_secs = rug_fuzz_0 * rug_fuzz_1;
        if let Some(original_offset) = FixedOffset::east_opt(offset_secs) {
            let offset = FixedOffset::from_offset(&original_offset);
            debug_assert_eq!(offset, original_offset);
            debug_assert_eq!(
                offset.local_minus_utc(), original_offset.local_minus_utc()
            );
        } else {
            panic!("Invalid FixedOffset created");
        }
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_173 {
    use super::*;
    use crate::*;
    use crate::NaiveDateTime;
    use crate::offset::{TimeZone, LocalResult};
    #[test]
    fn test_offset_from_local_datetime() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, i64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let fixed_offset = FixedOffset::east(rug_fuzz_0);
        let naive_dt = NaiveDateTime::from_timestamp(rug_fuzz_1, rug_fuzz_2);
        let expected = LocalResult::Single(fixed_offset);
        debug_assert_eq!(fixed_offset.offset_from_local_datetime(& naive_dt), expected);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_174 {
    use super::*;
    use crate::*;
    use crate::{NaiveDate, TimeZone};
    #[test]
    fn test_offset_from_utc_date() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, i32, &str, i32, u32, u32, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let fixed_offset = FixedOffset::east_opt(rug_fuzz_0 * rug_fuzz_1)
            .expect(rug_fuzz_2);
        let naive_utc_date = NaiveDate::from_ymd_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .expect(rug_fuzz_6);
        let resulting_offset = TimeZone::offset_from_utc_date(
            &fixed_offset,
            &naive_utc_date,
        );
        debug_assert_eq!(resulting_offset, fixed_offset);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_175 {
    use crate::offset::fixed::FixedOffset;
    use crate::{NaiveDate, NaiveTime, TimeZone};
    #[test]
    fn test_offset_from_utc_datetime() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8)) = <(i32, i32, &str, i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let offset = FixedOffset::east_opt(rug_fuzz_0 * rug_fuzz_1).expect(rug_fuzz_2);
        let utc_datetime = NaiveDate::from_ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .and_hms(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8);
        let offset_from_utc = offset.offset_from_utc_datetime(&utc_datetime);
        debug_assert_eq!(offset_from_utc, offset);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_504 {
    use super::*;
    use crate::*;
    use crate::offset::TimeZone;
    use crate::DateTime;
    use crate::Utc;
    #[test]
    fn test_add_fixed_offset_to_date_time() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, u32, u32, u32, u32, u32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let utc: DateTime<Utc> = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let offset = FixedOffset::east(rug_fuzz_6);
        let result = utc.add(offset);
        debug_assert_eq!(result, Utc.ymd(2023, 4, 10).and_hms(11, 0, 0));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_505_llm_16_505 {
    use super::*;
    use crate::*;
    use crate::naive::NaiveDate;
    use crate::naive::NaiveDateTime;
    use crate::offset::fixed::FixedOffset;
    #[test]
    fn test_add() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12)) = <(i32, u32, u32, u32, u32, u32, i32, i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let datetime = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .unwrap()
            .and_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .unwrap();
        let offset = FixedOffset::east_opt(rug_fuzz_6).unwrap();
        let result = datetime.add(offset);
        let expected = NaiveDate::from_ymd_opt(rug_fuzz_7, rug_fuzz_8, rug_fuzz_9)
            .unwrap()
            .and_hms_opt(rug_fuzz_10, rug_fuzz_11, rug_fuzz_12)
            .unwrap();
        debug_assert_eq!(result, expected);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_506 {
    use super::*;
    use crate::*;
    use crate::{NaiveTime, offset::FixedOffset};
    #[test]
    fn test_add_fixed_offset() {
        let _rug_st_tests_llm_16_506_rrrruuuugggg_test_add_fixed_offset = 0;
        let rug_fuzz_0 = 10;
        let rug_fuzz_1 = 20;
        let rug_fuzz_2 = 30;
        let rug_fuzz_3 = 1800;
        let rug_fuzz_4 = 10;
        let rug_fuzz_5 = 50;
        let rug_fuzz_6 = 30;
        let rug_fuzz_7 = 23;
        let rug_fuzz_8 = 45;
        let rug_fuzz_9 = 0;
        let rug_fuzz_10 = 3600;
        let rug_fuzz_11 = 0;
        let rug_fuzz_12 = 45;
        let rug_fuzz_13 = 0;
        let rug_fuzz_14 = 0;
        let rug_fuzz_15 = 30;
        let rug_fuzz_16 = 0;
        let rug_fuzz_17 = 3600;
        let rug_fuzz_18 = 23;
        let rug_fuzz_19 = 30;
        let rug_fuzz_20 = 0;
        let rug_fuzz_21 = 23;
        let rug_fuzz_22 = 59;
        let rug_fuzz_23 = 59;
        let rug_fuzz_24 = 1_000;
        let rug_fuzz_25 = 0;
        let rug_fuzz_26 = 23;
        let rug_fuzz_27 = 59;
        let rug_fuzz_28 = 59;
        let rug_fuzz_29 = 1_000;
        let rug_fuzz_30 = 0;
        let rug_fuzz_31 = 0;
        let rug_fuzz_32 = 0;
        let rug_fuzz_33 = 86_400;
        let rug_fuzz_34 = 86_400;
        let time = NaiveTime::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let offset = FixedOffset::east(rug_fuzz_3);
        let result = time.add(offset);
        let expected = NaiveTime::from_hms(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        debug_assert_eq!(result, expected);
        let time = NaiveTime::from_hms(rug_fuzz_7, rug_fuzz_8, rug_fuzz_9);
        let offset = FixedOffset::east(rug_fuzz_10);
        let result = time.add(offset);
        let expected = NaiveTime::from_hms(rug_fuzz_11, rug_fuzz_12, rug_fuzz_13);
        debug_assert_eq!(result, expected);
        let time = NaiveTime::from_hms(rug_fuzz_14, rug_fuzz_15, rug_fuzz_16);
        let offset = FixedOffset::east(-rug_fuzz_17);
        let result = time.add(offset);
        let expected = NaiveTime::from_hms(rug_fuzz_18, rug_fuzz_19, rug_fuzz_20);
        debug_assert_eq!(result, expected);
        let time = NaiveTime::from_hms_milli(
            rug_fuzz_21,
            rug_fuzz_22,
            rug_fuzz_23,
            rug_fuzz_24,
        );
        let offset = FixedOffset::east(rug_fuzz_25);
        let result = time.add(offset);
        let expected = NaiveTime::from_hms_milli(
            rug_fuzz_26,
            rug_fuzz_27,
            rug_fuzz_28,
            rug_fuzz_29,
        );
        debug_assert_eq!(result, expected);
        let time = NaiveTime::from_hms(rug_fuzz_30, rug_fuzz_31, rug_fuzz_32);
        let offset = FixedOffset::east(rug_fuzz_33);
        let result = time.add(offset);
        debug_assert!(result.num_seconds_from_midnight() < rug_fuzz_34);
        let _rug_ed_tests_llm_16_506_rrrruuuugggg_test_add_fixed_offset = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_508 {
    use super::*;
    use crate::*;
    use crate::{NaiveDate, NaiveTime, NaiveDateTime};
    #[test]
    fn test_sub_fixed_offset_subtraction() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12)) = <(i32, u32, u32, u32, u32, u32, i32, i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date_time = NaiveDateTime::new(
            NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2),
            NaiveTime::from_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5),
        );
        let fixed_offset = FixedOffset::east(rug_fuzz_6);
        let result = date_time.sub(fixed_offset);
        let expected = NaiveDateTime::new(
            NaiveDate::from_ymd(rug_fuzz_7, rug_fuzz_8, rug_fuzz_9),
            NaiveTime::from_hms(rug_fuzz_10, rug_fuzz_11, rug_fuzz_12),
        );
        debug_assert_eq!(result, expected);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_509 {
    use super::*;
    use crate::*;
    use crate::{TimeZone, FixedOffset, NaiveTime};
    #[test]
    fn test_subtract_fixed_offset() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u32, u32, u32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let time = NaiveTime::from_hms_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        let offset = FixedOffset::east_opt(rug_fuzz_3).unwrap();
        let result = time.sub(offset);
        debug_assert_eq!(result, NaiveTime::from_hms_opt(9, 0, 0).unwrap());
             }
}
}
}    }
    #[test]
    fn test_subtract_negative_fixed_offset() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u32, u32, u32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let time = NaiveTime::from_hms_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        let offset = FixedOffset::west_opt(rug_fuzz_3).unwrap();
        let result = time.sub(offset);
        debug_assert_eq!(result, NaiveTime::from_hms_opt(11, 0, 0).unwrap());
             }
}
}
}    }
    #[test]
    fn test_subtract_zero_offset() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u32, u32, u32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let time = NaiveTime::from_hms_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        let offset = FixedOffset::east_opt(rug_fuzz_3).unwrap();
        let result = time.sub(offset);
        debug_assert_eq!(result, NaiveTime::from_hms_opt(10, 0, 0).unwrap());
             }
}
}
}    }
    #[test]
    fn test_subtract_offset_resulting_in_previous_day() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u32, u32, u32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let time = NaiveTime::from_hms_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        let offset = FixedOffset::east_opt(rug_fuzz_3 * rug_fuzz_4).unwrap();
        let result = time.sub(offset);
        debug_assert_eq!(result, NaiveTime::from_hms_opt(22, 30, 0).unwrap());
             }
}
}
}    }
    #[test]
    fn test_subtract_offset_with_leap_second() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u32, u32, u32, u32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let time = NaiveTime::from_hms_milli_opt(
                rug_fuzz_0,
                rug_fuzz_1,
                rug_fuzz_2,
                rug_fuzz_3,
            )
            .unwrap();
        let offset = FixedOffset::west_opt(rug_fuzz_4 * rug_fuzz_5).unwrap();
        let result = time.sub(offset);
        debug_assert_eq!(
            result, NaiveTime::from_hms_milli_opt(4, 59, 59, 1_000).unwrap()
        );
             }
}
}
}    }
    #[test]
    #[should_panic(expected = "invalid time")]
    fn test_subtract_offset_out_of_bounds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u32, u32, u32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let time = NaiveTime::from_hms_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        let offset = FixedOffset::east_opt(rug_fuzz_3 * rug_fuzz_4).unwrap();
        let _result = time.sub(offset);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_510 {
    use crate::FixedOffset;
    #[test]
    fn test_east_within_bounds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, i32, i32, i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let hour = rug_fuzz_0;
        debug_assert_eq!(FixedOffset::east(rug_fuzz_1).local_minus_utc(), 0);
        debug_assert_eq!(
            FixedOffset::east(rug_fuzz_2 * hour).local_minus_utc(), 5 * hour
        );
        debug_assert_eq!(
            FixedOffset::east(rug_fuzz_3 * hour + rug_fuzz_4 * rug_fuzz_5 + rug_fuzz_6)
            .local_minus_utc(), 23 * hour + 59 * 60 + 59
        );
             }
}
}
}    }
    #[test]
    #[should_panic(expected = "FixedOffset::east out of bounds")]
    fn test_east_out_of_lower_bound() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let hour = rug_fuzz_0;
        FixedOffset::east(-rug_fuzz_1 * hour);
             }
}
}
}    }
    #[test]
    #[should_panic(expected = "FixedOffset::east out of bounds")]
    fn test_east_out_of_upper_bound() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let hour = rug_fuzz_0;
        FixedOffset::east(rug_fuzz_1 * hour);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_511 {
    use super::*;
    use crate::*;
    #[test]
    fn test_east_opt_valid_positive_offset() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let offset_secs = rug_fuzz_0 * rug_fuzz_1;
        debug_assert_eq!(
            FixedOffset::east_opt(offset_secs).unwrap().local_minus_utc(), offset_secs
        );
             }
}
}
}    }
    #[test]
    fn test_east_opt_valid_negative_offset() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let offset_secs = -rug_fuzz_0 * rug_fuzz_1;
        debug_assert_eq!(
            FixedOffset::east_opt(offset_secs).unwrap().local_minus_utc(), offset_secs
        );
             }
}
}
}    }
    #[test]
    fn test_east_opt_at_upper_bound() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let offset_secs = rug_fuzz_0 - rug_fuzz_1;
        debug_assert_eq!(
            FixedOffset::east_opt(offset_secs).unwrap().local_minus_utc(), offset_secs
        );
             }
}
}
}    }
    #[test]
    fn test_east_opt_at_lower_bound() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let offset_secs = -rug_fuzz_0 + rug_fuzz_1;
        debug_assert_eq!(
            FixedOffset::east_opt(offset_secs).unwrap().local_minus_utc(), offset_secs
        );
             }
}
}
}    }
    #[test]
    fn test_east_opt_beyond_upper_bound() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let offset_secs = rug_fuzz_0;
        debug_assert!(FixedOffset::east_opt(offset_secs).is_none());
             }
}
}
}    }
    #[test]
    fn test_east_opt_beyond_lower_bound() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let offset_secs = -rug_fuzz_0;
        debug_assert!(FixedOffset::east_opt(offset_secs).is_none());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_512 {
    use crate::offset::fixed::FixedOffset;
    #[test]
    fn test_local_minus_utc() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, i32, i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let hour = rug_fuzz_0;
        let fixed_offset_east = FixedOffset::east_opt(rug_fuzz_1 * hour).unwrap();
        debug_assert_eq!(fixed_offset_east.local_minus_utc(), 5 * hour);
        let fixed_offset_west = FixedOffset::west_opt(rug_fuzz_2 * hour).unwrap();
        debug_assert_eq!(fixed_offset_west.local_minus_utc(), - 5 * hour);
        let fixed_offset_zero = FixedOffset::east_opt(rug_fuzz_3).unwrap();
        debug_assert_eq!(fixed_offset_zero.local_minus_utc(), 0);
        let fixed_offset_max = FixedOffset::east_opt(rug_fuzz_4).unwrap();
        debug_assert_eq!(fixed_offset_max.local_minus_utc(), 86_399);
        let fixed_offset_min = FixedOffset::west_opt(rug_fuzz_5).unwrap();
        debug_assert_eq!(fixed_offset_min.local_minus_utc(), - 86_399);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_513 {
    use crate::FixedOffset;
    #[test]
    fn utc_minus_local_positive_offset() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let offset = FixedOffset::east_opt(rug_fuzz_0).unwrap();
        debug_assert_eq!(offset.utc_minus_local(), - 3600);
             }
}
}
}    }
    #[test]
    fn utc_minus_local_negative_offset() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let offset = FixedOffset::west_opt(rug_fuzz_0).unwrap();
        debug_assert_eq!(offset.utc_minus_local(), 3600);
             }
}
}
}    }
    #[test]
    fn utc_minus_local_zero_offset() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let offset = FixedOffset::east_opt(rug_fuzz_0).unwrap();
        debug_assert_eq!(offset.utc_minus_local(), 0);
             }
}
}
}    }
    #[test]
    fn utc_minus_local_min_offset() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let offset = FixedOffset::east_opt(-rug_fuzz_0).unwrap();
        debug_assert_eq!(offset.utc_minus_local(), 86399);
             }
}
}
}    }
    #[test]
    fn utc_minus_local_max_offset() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let offset = FixedOffset::west_opt(rug_fuzz_0).unwrap();
        debug_assert_eq!(offset.utc_minus_local(), - 86399);
             }
}
}
}    }
    #[test]
    #[should_panic]
    fn utc_minus_local_out_of_bounds_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        FixedOffset::east_opt(-rug_fuzz_0).unwrap();
             }
}
}
}    }
    #[test]
    #[should_panic]
    fn utc_minus_local_out_of_bounds_positive() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        FixedOffset::west_opt(rug_fuzz_0).unwrap();
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_514 {
    use super::*;
    use crate::*;
    #[test]
    #[should_panic(expected = "FixedOffset::west out of bounds")]
    fn test_west_panic_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        FixedOffset::west(-rug_fuzz_0);
             }
}
}
}    }
    #[test]
    #[should_panic(expected = "FixedOffset::west out of bounds")]
    fn test_west_panic_positive() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        FixedOffset::west(rug_fuzz_0);
             }
}
}
}    }
    #[test]
    fn test_west_valid() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let secs = rug_fuzz_0 * rug_fuzz_1;
        let fo = FixedOffset::west(secs);
        debug_assert_eq!(fo.local_minus_utc(), - secs);
             }
}
}
}    }
    #[test]
    fn test_west_valid_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let secs = -rug_fuzz_0 * rug_fuzz_1;
        let fo = FixedOffset::west(secs);
        debug_assert_eq!(fo.local_minus_utc(), - secs);
             }
}
}
}    }
    #[test]
    fn test_west_valid_edge_cases() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let secs = -rug_fuzz_0;
        let fo = FixedOffset::west(secs);
        debug_assert_eq!(fo.local_minus_utc(), - secs);
        let secs = rug_fuzz_1;
        let fo = FixedOffset::west(secs);
        debug_assert_eq!(fo.local_minus_utc(), - secs);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_515 {
    use super::*;
    use crate::*;
    use crate::offset::{TimeZone, FixedOffset};
    #[test]
    fn test_west_opt_valid() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let hour = rug_fuzz_0;
        debug_assert!(FixedOffset::west_opt(rug_fuzz_1 * hour).is_some());
             }
}
}
}    }
    #[test]
    fn test_west_opt_none_for_out_of_bound() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let hour = rug_fuzz_0;
        debug_assert!(FixedOffset::west_opt(rug_fuzz_1 * hour).is_none());
        debug_assert!(FixedOffset::west_opt(- rug_fuzz_2 * hour).is_none());
             }
}
}
}    }
    #[test]
    fn test_west_opt_none_for_exact_bounds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let day = rug_fuzz_0;
        debug_assert!(FixedOffset::west_opt(day).is_none());
        debug_assert!(FixedOffset::west_opt(- day).is_none());
             }
}
}
}    }
    #[test]
    fn test_west_opt_correct_offset() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(i32, i32, i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let hour = rug_fuzz_0;
        let offset = FixedOffset::west_opt(rug_fuzz_1 * hour).unwrap();
        let datetime = offset
            .ymd(rug_fuzz_2, rug_fuzz_3, rug_fuzz_4)
            .and_hms(rug_fuzz_5, rug_fuzz_6, rug_fuzz_7);
        debug_assert_eq!(& datetime.to_rfc3339(), "2016-11-08T00:00:00-05:00");
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_516 {
    use super::*;
    use crate::*;
    use crate::NaiveTime;
    #[test]
    fn test_adding_leapsecond() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u32, u32, u32, u32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base_time = NaiveTime::from_hms_nano(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
        );
        let added_time = add_with_leapsecond(&base_time, rug_fuzz_4);
        debug_assert_eq!(added_time, NaiveTime::from_hms_nano(0, 0, 2, 100_100_000));
             }
}
}
}    }
    #[test]
    fn test_adding_non_leapsecond() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u32, u32, u32, u32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base_time = NaiveTime::from_hms_nano(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
        );
        let added_time = add_with_leapsecond(&base_time, rug_fuzz_4);
        debug_assert_eq!(added_time, NaiveTime::from_hms_nano(12, 30, 3, 0));
             }
}
}
}    }
    #[test]
    fn test_adding_negative_seconds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u32, u32, u32, u32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base_time = NaiveTime::from_hms_nano(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
        );
        let added_time = add_with_leapsecond(&base_time, -rug_fuzz_4);
        debug_assert_eq!(added_time, NaiveTime::from_hms_nano(12, 29, 55, 0));
             }
}
}
}    }
    #[test]
    #[should_panic]
    fn test_adding_leapsecond_with_invalid_nano() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u32, u32, u32, u32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base_time = NaiveTime::from_hms_nano(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
        );
        let _added_time = add_with_leapsecond(&base_time, rug_fuzz_4);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_117 {
    use crate::{TimeZone, FixedOffset, NaiveDate, LocalResult};
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: FixedOffset = FixedOffset::east(rug_fuzz_0);
        let p1: NaiveDate = NaiveDate::from_ymd(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        debug_assert_eq!(
            < FixedOffset as TimeZone > ::offset_from_local_date(& p0, & p1),
            LocalResult::Single(p0)
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_118 {
    use super::*;
    use crate::{DateTime, FixedOffset, TimeZone, Utc};
    use std::ops::Sub;
    #[test]
    fn test_sub() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i64, u32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: DateTime<Utc> = Utc.timestamp(rug_fuzz_0, rug_fuzz_1);
        let mut p1 = FixedOffset::east(rug_fuzz_2);
        let result = DateTime::<Utc>::sub(p0, p1);
        p1 = FixedOffset::east(rug_fuzz_3);
        debug_assert_eq!(result, Utc.timestamp(1580511600, 0));
             }
}
}
}    }
}
