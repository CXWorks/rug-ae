use core::{convert::TryFrom, fmt};
#[cfg(feature = "rkyv")]
use rkyv::{Archive, Deserialize, Serialize};
use crate::OutOfRange;
/// The month of the year.
///
/// This enum is just a convenience implementation.
/// The month in dates created by DateLike objects does not return this enum.
///
/// It is possible to convert from a date to a month independently
/// ```
/// # use std::convert::TryFrom;
/// use chrono::prelude::*;
/// let date = Utc.with_ymd_and_hms(2019, 10, 28, 9, 10, 11).unwrap();
/// // `2019-10-28T09:10:11Z`
/// let month = Month::try_from(u8::try_from(date.month()).unwrap()).ok();
/// assert_eq!(month, Some(Month::October))
/// ```
/// Or from a Month to an integer usable by dates
/// ```
/// # use chrono::prelude::*;
/// let month = Month::January;
/// let dt = Utc.with_ymd_and_hms(2019, month.number_from_month(), 28, 9, 10, 11).unwrap();
/// assert_eq!((dt.year(), dt.month(), dt.day()), (2019, 1, 28));
/// ```
/// Allows mapping from and to month, from 1-January to 12-December.
/// Can be Serialized/Deserialized with serde
#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash, PartialOrd)]
#[cfg_attr(feature = "rkyv", derive(Archive, Deserialize, Serialize))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub enum Month {
    /// January
    January = 0,
    /// February
    February = 1,
    /// March
    March = 2,
    /// April
    April = 3,
    /// May
    May = 4,
    /// June
    June = 5,
    /// July
    July = 6,
    /// August
    August = 7,
    /// September
    September = 8,
    /// October
    October = 9,
    /// November
    November = 10,
    /// December
    December = 11,
}
impl Month {
    /// The next month.
    ///
    /// `m`:        | `January`  | `February` | `...` | `December`
    /// ----------- | ---------  | ---------- | --- | ---------
    /// `m.succ()`: | `February` | `March`    | `...` | `January`
    #[inline]
    #[must_use]
    pub const fn succ(&self) -> Month {
        match *self {
            Month::January => Month::February,
            Month::February => Month::March,
            Month::March => Month::April,
            Month::April => Month::May,
            Month::May => Month::June,
            Month::June => Month::July,
            Month::July => Month::August,
            Month::August => Month::September,
            Month::September => Month::October,
            Month::October => Month::November,
            Month::November => Month::December,
            Month::December => Month::January,
        }
    }
    /// The previous month.
    ///
    /// `m`:        | `January`  | `February` | `...` | `December`
    /// ----------- | ---------  | ---------- | --- | ---------
    /// `m.pred()`: | `December` | `January`  | `...` | `November`
    #[inline]
    #[must_use]
    pub const fn pred(&self) -> Month {
        match *self {
            Month::January => Month::December,
            Month::February => Month::January,
            Month::March => Month::February,
            Month::April => Month::March,
            Month::May => Month::April,
            Month::June => Month::May,
            Month::July => Month::June,
            Month::August => Month::July,
            Month::September => Month::August,
            Month::October => Month::September,
            Month::November => Month::October,
            Month::December => Month::November,
        }
    }
    /// Returns a month-of-year number starting from January = 1.
    ///
    /// `m`:                     | `January` | `February` | `...` | `December`
    /// -------------------------| --------- | ---------- | --- | -----
    /// `m.number_from_month()`: | 1         | 2          | `...` | 12
    #[inline]
    #[must_use]
    pub const fn number_from_month(&self) -> u32 {
        match *self {
            Month::January => 1,
            Month::February => 2,
            Month::March => 3,
            Month::April => 4,
            Month::May => 5,
            Month::June => 6,
            Month::July => 7,
            Month::August => 8,
            Month::September => 9,
            Month::October => 10,
            Month::November => 11,
            Month::December => 12,
        }
    }
    /// Get the name of the month
    ///
    /// ```
    /// use chrono::Month;
    ///
    /// assert_eq!(Month::January.name(), "January")
    /// ```
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match *self {
            Month::January => "January",
            Month::February => "February",
            Month::March => "March",
            Month::April => "April",
            Month::May => "May",
            Month::June => "June",
            Month::July => "July",
            Month::August => "August",
            Month::September => "September",
            Month::October => "October",
            Month::November => "November",
            Month::December => "December",
        }
    }
}
impl TryFrom<u8> for Month {
    type Error = OutOfRange;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Month::January),
            2 => Ok(Month::February),
            3 => Ok(Month::March),
            4 => Ok(Month::April),
            5 => Ok(Month::May),
            6 => Ok(Month::June),
            7 => Ok(Month::July),
            8 => Ok(Month::August),
            9 => Ok(Month::September),
            10 => Ok(Month::October),
            11 => Ok(Month::November),
            12 => Ok(Month::December),
            _ => Err(OutOfRange::new()),
        }
    }
}
/// A duration in calendar months
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct Months(pub(crate) u32);
impl Months {
    /// Construct a new `Months` from a number of months
    pub const fn new(num: u32) -> Self {
        Self(num)
    }
}
/// An error resulting from reading `<Month>` value with `FromStr`.
#[derive(Clone, PartialEq, Eq)]
pub struct ParseMonthError {
    pub(crate) _dummy: (),
}
impl fmt::Debug for ParseMonthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParseMonthError {{ .. }}")
    }
}
#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
mod month_serde {
    use super::Month;
    use serde::{de, ser};
    use core::fmt;
    impl ser::Serialize for Month {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
        {
            serializer.collect_str(self.name())
        }
    }
    struct MonthVisitor;
    impl<'de> de::Visitor<'de> for MonthVisitor {
        type Value = Month;
        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("Month")
        }
        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            value
                .parse()
                .map_err(|_| E::custom("short (3-letter) or full month names expected"))
        }
    }
    impl<'de> de::Deserialize<'de> for Month {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            deserializer.deserialize_str(MonthVisitor)
        }
    }
    #[test]
    fn test_serde_serialize() {
        use serde_json::to_string;
        use Month::*;
        let cases: Vec<(Month, &str)> = vec![
            (January, "\"January\""), (February, "\"February\""), (March, "\"March\""),
            (April, "\"April\""), (May, "\"May\""), (June, "\"June\""), (July,
            "\"July\""), (August, "\"August\""), (September, "\"September\""), (October,
            "\"October\""), (November, "\"November\""), (December, "\"December\""),
        ];
        for (month, expected_str) in cases {
            let string = to_string(&month).unwrap();
            assert_eq!(string, expected_str);
        }
    }
    #[test]
    fn test_serde_deserialize() {
        use serde_json::from_str;
        use Month::*;
        let cases: Vec<(&str, Month)> = vec![
            ("\"january\"", January), ("\"jan\"", January), ("\"FeB\"", February),
            ("\"MAR\"", March), ("\"mar\"", March), ("\"april\"", April), ("\"may\"",
            May), ("\"june\"", June), ("\"JULY\"", July), ("\"august\"", August),
            ("\"september\"", September), ("\"October\"", October), ("\"November\"",
            November), ("\"DECEmbEr\"", December),
        ];
        for (string, expected_month) in cases {
            let month = from_str::<Month>(string).unwrap();
            assert_eq!(month, expected_month);
        }
        let errors: Vec<&str> = vec![
            "\"not a month\"", "\"ja\"", "\"Dece\"", "Dec", "\"Augustin\""
        ];
        for string in errors {
            from_str::<Month>(string).unwrap_err();
        }
    }
}
#[cfg(test)]
mod tests {
    use core::convert::TryFrom;
    use super::Month;
    use crate::{Datelike, OutOfRange, TimeZone, Utc};
    #[test]
    fn test_month_enum_try_from() {
        assert_eq!(Month::try_from(1), Ok(Month::January));
        assert_eq!(Month::try_from(2), Ok(Month::February));
        assert_eq!(Month::try_from(12), Ok(Month::December));
        assert_eq!(Month::try_from(13), Err(OutOfRange::new()));
        let date = Utc.with_ymd_and_hms(2019, 10, 28, 9, 10, 11).unwrap();
        assert_eq!(Month::try_from(date.month() as u8).ok(), Some(Month::October));
        let month = Month::January;
        let dt = Utc
            .with_ymd_and_hms(2019, month.number_from_month(), 28, 9, 10, 11)
            .unwrap();
        assert_eq!((dt.year(), dt.month(), dt.day()), (2019, 1, 28));
    }
    #[test]
    fn test_month_enum_succ_pred() {
        assert_eq!(Month::January.succ(), Month::February);
        assert_eq!(Month::December.succ(), Month::January);
        assert_eq!(Month::January.pred(), Month::December);
        assert_eq!(Month::February.pred(), Month::January);
    }
    #[test]
    fn test_month_partial_ord() {
        assert!(Month::January <= Month::January);
        assert!(Month::January < Month::February);
        assert!(Month::January < Month::December);
        assert!(Month::July >= Month::May);
        assert!(Month::September > Month::March);
    }
}
#[cfg(test)]
mod tests_llm_16_82 {
    use super::*;
    use crate::*;
    use std::convert::TryFrom;
    #[test]
    fn test_try_from_valid_values() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Month::try_from(rug_fuzz_0).unwrap(), Month::January);
        debug_assert_eq!(Month::try_from(rug_fuzz_1).unwrap(), Month::February);
        debug_assert_eq!(Month::try_from(rug_fuzz_2).unwrap(), Month::March);
        debug_assert_eq!(Month::try_from(rug_fuzz_3).unwrap(), Month::April);
        debug_assert_eq!(Month::try_from(rug_fuzz_4).unwrap(), Month::May);
        debug_assert_eq!(Month::try_from(rug_fuzz_5).unwrap(), Month::June);
        debug_assert_eq!(Month::try_from(rug_fuzz_6).unwrap(), Month::July);
        debug_assert_eq!(Month::try_from(rug_fuzz_7).unwrap(), Month::August);
        debug_assert_eq!(Month::try_from(rug_fuzz_8).unwrap(), Month::September);
        debug_assert_eq!(Month::try_from(rug_fuzz_9).unwrap(), Month::October);
        debug_assert_eq!(Month::try_from(rug_fuzz_10).unwrap(), Month::November);
        debug_assert_eq!(Month::try_from(rug_fuzz_11).unwrap(), Month::December);
             }
}
}
}    }
    #[test]
    fn test_try_from_invalid_values() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(Month::try_from(rug_fuzz_0).is_err());
        debug_assert!(Month::try_from(rug_fuzz_1).is_err());
        debug_assert!(Month::try_from(rug_fuzz_2).is_err());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_349 {
    use crate::Month;
    #[test]
    fn test_month_name() {
        let _rug_st_tests_llm_16_349_rrrruuuugggg_test_month_name = 0;
        debug_assert_eq!(Month::January.name(), "January");
        debug_assert_eq!(Month::February.name(), "February");
        debug_assert_eq!(Month::March.name(), "March");
        debug_assert_eq!(Month::April.name(), "April");
        debug_assert_eq!(Month::May.name(), "May");
        debug_assert_eq!(Month::June.name(), "June");
        debug_assert_eq!(Month::July.name(), "July");
        debug_assert_eq!(Month::August.name(), "August");
        debug_assert_eq!(Month::September.name(), "September");
        debug_assert_eq!(Month::October.name(), "October");
        debug_assert_eq!(Month::November.name(), "November");
        debug_assert_eq!(Month::December.name(), "December");
        let _rug_ed_tests_llm_16_349_rrrruuuugggg_test_month_name = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_350 {
    use crate::Month;
    #[test]
    fn test_number_from_month() {
        let _rug_st_tests_llm_16_350_rrrruuuugggg_test_number_from_month = 0;
        debug_assert_eq!(Month::January.number_from_month(), 1);
        debug_assert_eq!(Month::February.number_from_month(), 2);
        debug_assert_eq!(Month::March.number_from_month(), 3);
        debug_assert_eq!(Month::April.number_from_month(), 4);
        debug_assert_eq!(Month::May.number_from_month(), 5);
        debug_assert_eq!(Month::June.number_from_month(), 6);
        debug_assert_eq!(Month::July.number_from_month(), 7);
        debug_assert_eq!(Month::August.number_from_month(), 8);
        debug_assert_eq!(Month::September.number_from_month(), 9);
        debug_assert_eq!(Month::October.number_from_month(), 10);
        debug_assert_eq!(Month::November.number_from_month(), 11);
        debug_assert_eq!(Month::December.number_from_month(), 12);
        let _rug_ed_tests_llm_16_350_rrrruuuugggg_test_number_from_month = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_351 {
    use super::*;
    use crate::*;
    #[test]
    fn test_pred() {
        let _rug_st_tests_llm_16_351_rrrruuuugggg_test_pred = 0;
        debug_assert_eq!(Month::February.pred(), Month::January);
        debug_assert_eq!(Month::March.pred(), Month::February);
        debug_assert_eq!(Month::April.pred(), Month::March);
        debug_assert_eq!(Month::May.pred(), Month::April);
        debug_assert_eq!(Month::June.pred(), Month::May);
        debug_assert_eq!(Month::July.pred(), Month::June);
        debug_assert_eq!(Month::August.pred(), Month::July);
        debug_assert_eq!(Month::September.pred(), Month::August);
        debug_assert_eq!(Month::October.pred(), Month::September);
        debug_assert_eq!(Month::November.pred(), Month::October);
        debug_assert_eq!(Month::December.pred(), Month::November);
        debug_assert_eq!(Month::January.pred(), Month::December);
        let _rug_ed_tests_llm_16_351_rrrruuuugggg_test_pred = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_352 {
    use crate::Month;
    #[test]
    fn test_succ() {
        let _rug_st_tests_llm_16_352_rrrruuuugggg_test_succ = 0;
        debug_assert_eq!(Month::January.succ(), Month::February);
        debug_assert_eq!(Month::February.succ(), Month::March);
        debug_assert_eq!(Month::March.succ(), Month::April);
        debug_assert_eq!(Month::April.succ(), Month::May);
        debug_assert_eq!(Month::May.succ(), Month::June);
        debug_assert_eq!(Month::June.succ(), Month::July);
        debug_assert_eq!(Month::July.succ(), Month::August);
        debug_assert_eq!(Month::August.succ(), Month::September);
        debug_assert_eq!(Month::September.succ(), Month::October);
        debug_assert_eq!(Month::October.succ(), Month::November);
        debug_assert_eq!(Month::November.succ(), Month::December);
        debug_assert_eq!(Month::December.succ(), Month::January);
        let _rug_ed_tests_llm_16_352_rrrruuuugggg_test_succ = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_353 {
    use crate::Months;
    #[test]
    fn test_new() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let num_months = rug_fuzz_0;
        let months = Months::new(num_months);
        debug_assert_eq!(months.0, num_months);
             }
}
}
}    }
    #[test]
    fn test_clone() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let months = Months::new(rug_fuzz_0);
        let cloned_months = months.clone();
        debug_assert_eq!(months, cloned_months);
             }
}
}
}    }
    #[test]
    fn test_eq() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let months_a = Months::new(rug_fuzz_0);
        let months_b = Months::new(rug_fuzz_1);
        debug_assert_eq!(months_a, months_b);
             }
}
}
}    }
    #[test]
    fn test_partial_eq() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let months_a = Months::new(rug_fuzz_0);
        let months_b = Months::new(rug_fuzz_1);
        let months_c = Months::new(rug_fuzz_2);
        debug_assert!(months_a == months_b);
        debug_assert!(months_a != months_c);
             }
}
}
}    }
    #[test]
    fn test_partial_ord() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let months_a = Months::new(rug_fuzz_0);
        let months_b = Months::new(rug_fuzz_1);
        debug_assert!(months_a < months_b);
             }
}
}
}    }
    #[test]
    fn test_debug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let months = Months::new(rug_fuzz_0);
        debug_assert_eq!(format!("{:?}", months), "Months(10)");
             }
}
}
}    }
    #[test]
    fn test_hash() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let months_a = Months::new(rug_fuzz_0);
        let months_b = Months::new(rug_fuzz_1);
        let months_c = Months::new(rug_fuzz_2);
        let mut hasher_a = DefaultHasher::new();
        months_a.hash(&mut hasher_a);
        let hash_a = hasher_a.finish();
        let mut hasher_b = DefaultHasher::new();
        months_b.hash(&mut hasher_b);
        let hash_b = hasher_b.finish();
        let mut hasher_c = DefaultHasher::new();
        months_c.hash(&mut hasher_c);
        let hash_c = hasher_c.finish();
        debug_assert_eq!(hash_a, hash_b);
        debug_assert_ne!(hash_a, hash_c);
             }
}
}
}    }
    #[test]
    fn test_copy() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let months_a = Months::new(rug_fuzz_0);
        let months_b = months_a;
        let months_c = months_a;
        debug_assert_eq!(months_a, months_b);
        debug_assert_eq!(months_a, months_c);
             }
}
}
}    }
}
