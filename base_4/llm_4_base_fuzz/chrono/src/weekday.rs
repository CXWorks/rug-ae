use core::{convert::TryFrom, fmt};
#[cfg(feature = "rkyv")]
use rkyv::{Archive, Deserialize, Serialize};
use crate::OutOfRange;
/// The day of week.
///
/// The order of the days of week depends on the context.
/// (This is why this type does *not* implement `PartialOrd` or `Ord` traits.)
/// One should prefer `*_from_monday` or `*_from_sunday` methods to get the correct result.
///
/// # Example
/// ```
/// use chrono::Weekday;
/// use std::convert::TryFrom;
///
/// let monday = "Monday".parse::<Weekday>().unwrap();
/// assert_eq!(monday, Weekday::Mon);
///
/// let sunday = Weekday::try_from(6).unwrap();
/// assert_eq!(sunday, Weekday::Sun);
///
/// assert_eq!(sunday.num_days_from_monday(), 6); // starts counting with Monday = 0
/// assert_eq!(sunday.number_from_monday(), 7); // starts counting with Monday = 1
/// assert_eq!(sunday.num_days_from_sunday(), 0); // starts counting with Sunday = 0
/// assert_eq!(sunday.number_from_sunday(), 1); // starts counting with Sunday = 1
///
/// assert_eq!(sunday.succ(), monday);
/// assert_eq!(sunday.pred(), Weekday::Sat);
/// ```
#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
#[cfg_attr(feature = "rkyv", derive(Archive, Deserialize, Serialize))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub enum Weekday {
    /// Monday.
    Mon = 0,
    /// Tuesday.
    Tue = 1,
    /// Wednesday.
    Wed = 2,
    /// Thursday.
    Thu = 3,
    /// Friday.
    Fri = 4,
    /// Saturday.
    Sat = 5,
    /// Sunday.
    Sun = 6,
}
impl Weekday {
    /// The next day in the week.
    ///
    /// `w`:        | `Mon` | `Tue` | `Wed` | `Thu` | `Fri` | `Sat` | `Sun`
    /// ----------- | ----- | ----- | ----- | ----- | ----- | ----- | -----
    /// `w.succ()`: | `Tue` | `Wed` | `Thu` | `Fri` | `Sat` | `Sun` | `Mon`
    #[inline]
    #[must_use]
    pub const fn succ(&self) -> Weekday {
        match *self {
            Weekday::Mon => Weekday::Tue,
            Weekday::Tue => Weekday::Wed,
            Weekday::Wed => Weekday::Thu,
            Weekday::Thu => Weekday::Fri,
            Weekday::Fri => Weekday::Sat,
            Weekday::Sat => Weekday::Sun,
            Weekday::Sun => Weekday::Mon,
        }
    }
    /// The previous day in the week.
    ///
    /// `w`:        | `Mon` | `Tue` | `Wed` | `Thu` | `Fri` | `Sat` | `Sun`
    /// ----------- | ----- | ----- | ----- | ----- | ----- | ----- | -----
    /// `w.pred()`: | `Sun` | `Mon` | `Tue` | `Wed` | `Thu` | `Fri` | `Sat`
    #[inline]
    #[must_use]
    pub const fn pred(&self) -> Weekday {
        match *self {
            Weekday::Mon => Weekday::Sun,
            Weekday::Tue => Weekday::Mon,
            Weekday::Wed => Weekday::Tue,
            Weekday::Thu => Weekday::Wed,
            Weekday::Fri => Weekday::Thu,
            Weekday::Sat => Weekday::Fri,
            Weekday::Sun => Weekday::Sat,
        }
    }
    /// Returns a day-of-week number starting from Monday = 1. (ISO 8601 weekday number)
    ///
    /// `w`:                      | `Mon` | `Tue` | `Wed` | `Thu` | `Fri` | `Sat` | `Sun`
    /// ------------------------- | ----- | ----- | ----- | ----- | ----- | ----- | -----
    /// `w.number_from_monday()`: | 1     | 2     | 3     | 4     | 5     | 6     | 7
    #[inline]
    pub const fn number_from_monday(&self) -> u32 {
        self.num_days_from(Weekday::Mon) + 1
    }
    /// Returns a day-of-week number starting from Sunday = 1.
    ///
    /// `w`:                      | `Mon` | `Tue` | `Wed` | `Thu` | `Fri` | `Sat` | `Sun`
    /// ------------------------- | ----- | ----- | ----- | ----- | ----- | ----- | -----
    /// `w.number_from_sunday()`: | 2     | 3     | 4     | 5     | 6     | 7     | 1
    #[inline]
    pub const fn number_from_sunday(&self) -> u32 {
        self.num_days_from(Weekday::Sun) + 1
    }
    /// Returns a day-of-week number starting from Monday = 0.
    ///
    /// `w`:                        | `Mon` | `Tue` | `Wed` | `Thu` | `Fri` | `Sat` | `Sun`
    /// --------------------------- | ----- | ----- | ----- | ----- | ----- | ----- | -----
    /// `w.num_days_from_monday()`: | 0     | 1     | 2     | 3     | 4     | 5     | 6
    #[inline]
    pub const fn num_days_from_monday(&self) -> u32 {
        self.num_days_from(Weekday::Mon)
    }
    /// Returns a day-of-week number starting from Sunday = 0.
    ///
    /// `w`:                        | `Mon` | `Tue` | `Wed` | `Thu` | `Fri` | `Sat` | `Sun`
    /// --------------------------- | ----- | ----- | ----- | ----- | ----- | ----- | -----
    /// `w.num_days_from_sunday()`: | 1     | 2     | 3     | 4     | 5     | 6     | 0
    #[inline]
    pub const fn num_days_from_sunday(&self) -> u32 {
        self.num_days_from(Weekday::Sun)
    }
    /// Returns a day-of-week number starting from the parameter `day` (D) = 0.
    ///
    /// `w`:                        | `D`   | `D+1` | `D+2` | `D+3` | `D+4` | `D+5` | `D+6`
    /// --------------------------- | ----- | ----- | ----- | ----- | ----- | ----- | -----
    /// `w.num_days_from(wd)`:      | 0     | 1     | 2     | 3     | 4     | 5     | 6
    #[inline]
    pub(crate) const fn num_days_from(&self, day: Weekday) -> u32 {
        (*self as u32 + 7 - day as u32) % 7
    }
}
impl fmt::Display for Weekday {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(
            match *self {
                Weekday::Mon => "Mon",
                Weekday::Tue => "Tue",
                Weekday::Wed => "Wed",
                Weekday::Thu => "Thu",
                Weekday::Fri => "Fri",
                Weekday::Sat => "Sat",
                Weekday::Sun => "Sun",
            },
        )
    }
}
/// Any weekday can be represented as an integer from 0 to 6, which equals to
/// [`Weekday::num_days_from_monday`](#method.num_days_from_monday) in this implementation.
/// Do not heavily depend on this though; use explicit methods whenever possible.
impl TryFrom<u8> for Weekday {
    type Error = OutOfRange;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Weekday::Mon),
            1 => Ok(Weekday::Tue),
            2 => Ok(Weekday::Wed),
            3 => Ok(Weekday::Thu),
            4 => Ok(Weekday::Fri),
            5 => Ok(Weekday::Sat),
            6 => Ok(Weekday::Sun),
            _ => Err(OutOfRange::new()),
        }
    }
}
/// An error resulting from reading `Weekday` value with `FromStr`.
#[derive(Clone, PartialEq, Eq)]
pub struct ParseWeekdayError {
    pub(crate) _dummy: (),
}
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl std::error::Error for ParseWeekdayError {}
impl fmt::Display for ParseWeekdayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}
impl fmt::Debug for ParseWeekdayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParseWeekdayError {{ .. }}")
    }
}
#[cfg(test)]
mod tests {
    use super::Weekday;
    use std::convert::TryFrom;
    #[test]
    fn test_num_days_from() {
        for i in 0..7 {
            let base_day = Weekday::try_from(i).unwrap();
            assert_eq!(
                base_day.num_days_from_monday(), base_day.num_days_from(Weekday::Mon)
            );
            assert_eq!(
                base_day.num_days_from_sunday(), base_day.num_days_from(Weekday::Sun)
            );
            assert_eq!(base_day.num_days_from(base_day), 0);
            assert_eq!(base_day.num_days_from(base_day.pred()), 1);
            assert_eq!(base_day.num_days_from(base_day.pred().pred()), 2);
            assert_eq!(base_day.num_days_from(base_day.pred().pred().pred()), 3);
            assert_eq!(base_day.num_days_from(base_day.pred().pred().pred().pred()), 4);
            assert_eq!(
                base_day.num_days_from(base_day.pred().pred().pred().pred().pred()), 5
            );
            assert_eq!(
                base_day.num_days_from(base_day.pred().pred().pred().pred().pred()
                .pred()), 6
            );
            assert_eq!(base_day.num_days_from(base_day.succ()), 6);
            assert_eq!(base_day.num_days_from(base_day.succ().succ()), 5);
            assert_eq!(base_day.num_days_from(base_day.succ().succ().succ()), 4);
            assert_eq!(base_day.num_days_from(base_day.succ().succ().succ().succ()), 3);
            assert_eq!(
                base_day.num_days_from(base_day.succ().succ().succ().succ().succ()), 2
            );
            assert_eq!(
                base_day.num_days_from(base_day.succ().succ().succ().succ().succ()
                .succ()), 1
            );
        }
    }
}
#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
mod weekday_serde {
    use super::Weekday;
    use core::fmt;
    use serde::{de, ser};
    impl ser::Serialize for Weekday {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
        {
            serializer.collect_str(&self)
        }
    }
    struct WeekdayVisitor;
    impl<'de> de::Visitor<'de> for WeekdayVisitor {
        type Value = Weekday;
        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("Weekday")
        }
        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            value.parse().map_err(|_| E::custom("short or long weekday names expected"))
        }
    }
    impl<'de> de::Deserialize<'de> for Weekday {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            deserializer.deserialize_str(WeekdayVisitor)
        }
    }
    #[test]
    fn test_serde_serialize() {
        use serde_json::to_string;
        use Weekday::*;
        let cases: Vec<(Weekday, &str)> = vec![
            (Mon, "\"Mon\""), (Tue, "\"Tue\""), (Wed, "\"Wed\""), (Thu, "\"Thu\""), (Fri,
            "\"Fri\""), (Sat, "\"Sat\""), (Sun, "\"Sun\""),
        ];
        for (weekday, expected_str) in cases {
            let string = to_string(&weekday).unwrap();
            assert_eq!(string, expected_str);
        }
    }
    #[test]
    fn test_serde_deserialize() {
        use serde_json::from_str;
        use Weekday::*;
        let cases: Vec<(&str, Weekday)> = vec![
            ("\"mon\"", Mon), ("\"MONDAY\"", Mon), ("\"MonDay\"", Mon), ("\"mOn\"", Mon),
            ("\"tue\"", Tue), ("\"tuesday\"", Tue), ("\"wed\"", Wed), ("\"wednesday\"",
            Wed), ("\"thu\"", Thu), ("\"thursday\"", Thu), ("\"fri\"", Fri),
            ("\"friday\"", Fri), ("\"sat\"", Sat), ("\"saturday\"", Sat), ("\"sun\"",
            Sun), ("\"sunday\"", Sun),
        ];
        for (str, expected_weekday) in cases {
            let weekday = from_str::<Weekday>(str).unwrap();
            assert_eq!(weekday, expected_weekday);
        }
        let errors: Vec<&str> = vec![
            "\"not a weekday\"", "\"monDAYs\"", "\"mond\"", "mon", "\"thur\"",
            "\"thurs\""
        ];
        for str in errors {
            from_str::<Weekday>(str).unwrap_err();
        }
    }
}
#[cfg(test)]
mod tests_llm_16_627 {
    use crate::Weekday;
    #[test]
    fn num_days_from_monday_to_monday() {
        let _rug_st_tests_llm_16_627_rrrruuuugggg_num_days_from_monday_to_monday = 0;
        debug_assert_eq!(Weekday::Mon.num_days_from(Weekday::Mon), 0);
        let _rug_ed_tests_llm_16_627_rrrruuuugggg_num_days_from_monday_to_monday = 0;
    }
    #[test]
    fn num_days_from_monday_to_sunday() {
        let _rug_st_tests_llm_16_627_rrrruuuugggg_num_days_from_monday_to_sunday = 0;
        debug_assert_eq!(Weekday::Sun.num_days_from(Weekday::Mon), 6);
        let _rug_ed_tests_llm_16_627_rrrruuuugggg_num_days_from_monday_to_sunday = 0;
    }
    #[test]
    fn num_days_from_tuesday_to_monday() {
        let _rug_st_tests_llm_16_627_rrrruuuugggg_num_days_from_tuesday_to_monday = 0;
        debug_assert_eq!(Weekday::Mon.num_days_from(Weekday::Tue), 6);
        let _rug_ed_tests_llm_16_627_rrrruuuugggg_num_days_from_tuesday_to_monday = 0;
    }
    #[test]
    fn num_days_from_sunday_to_sunday() {
        let _rug_st_tests_llm_16_627_rrrruuuugggg_num_days_from_sunday_to_sunday = 0;
        debug_assert_eq!(Weekday::Sun.num_days_from(Weekday::Sun), 0);
        let _rug_ed_tests_llm_16_627_rrrruuuugggg_num_days_from_sunday_to_sunday = 0;
    }
    #[test]
    fn num_days_from_sunday_to_monday() {
        let _rug_st_tests_llm_16_627_rrrruuuugggg_num_days_from_sunday_to_monday = 0;
        debug_assert_eq!(Weekday::Mon.num_days_from(Weekday::Sun), 1);
        let _rug_ed_tests_llm_16_627_rrrruuuugggg_num_days_from_sunday_to_monday = 0;
    }
    #[test]
    fn num_days_from_saturday_to_wednesday() {
        let _rug_st_tests_llm_16_627_rrrruuuugggg_num_days_from_saturday_to_wednesday = 0;
        debug_assert_eq!(Weekday::Sat.num_days_from(Weekday::Wed), 3);
        let _rug_ed_tests_llm_16_627_rrrruuuugggg_num_days_from_saturday_to_wednesday = 0;
    }
    #[test]
    fn num_days_from_friday_to_thursday() {
        let _rug_st_tests_llm_16_627_rrrruuuugggg_num_days_from_friday_to_thursday = 0;
        debug_assert_eq!(Weekday::Fri.num_days_from(Weekday::Thu), 1);
        let _rug_ed_tests_llm_16_627_rrrruuuugggg_num_days_from_friday_to_thursday = 0;
    }
    #[test]
    fn num_days_from_wednesday_to_tuesday() {
        let _rug_st_tests_llm_16_627_rrrruuuugggg_num_days_from_wednesday_to_tuesday = 0;
        debug_assert_eq!(Weekday::Wed.num_days_from(Weekday::Tue), 1);
        let _rug_ed_tests_llm_16_627_rrrruuuugggg_num_days_from_wednesday_to_tuesday = 0;
    }
    #[test]
    fn num_days_from_thursday_to_saturday() {
        let _rug_st_tests_llm_16_627_rrrruuuugggg_num_days_from_thursday_to_saturday = 0;
        debug_assert_eq!(Weekday::Thu.num_days_from(Weekday::Sat), 5);
        let _rug_ed_tests_llm_16_627_rrrruuuugggg_num_days_from_thursday_to_saturday = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_628 {
    use crate::Weekday;
    use std::convert::TryFrom;
    #[test]
    fn test_num_days_from_monday() {
        let _rug_st_tests_llm_16_628_rrrruuuugggg_test_num_days_from_monday = 0;
        debug_assert_eq!(Weekday::Mon.num_days_from_monday(), 0);
        debug_assert_eq!(Weekday::Tue.num_days_from_monday(), 1);
        debug_assert_eq!(Weekday::Wed.num_days_from_monday(), 2);
        debug_assert_eq!(Weekday::Thu.num_days_from_monday(), 3);
        debug_assert_eq!(Weekday::Fri.num_days_from_monday(), 4);
        debug_assert_eq!(Weekday::Sat.num_days_from_monday(), 5);
        debug_assert_eq!(Weekday::Sun.num_days_from_monday(), 6);
        let _rug_ed_tests_llm_16_628_rrrruuuugggg_test_num_days_from_monday = 0;
    }
    #[test]
    fn test_weekday_from_str() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(&str, &str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(rug_fuzz_0.parse(), Ok(Weekday::Mon));
        debug_assert_eq!(rug_fuzz_1.parse(), Ok(Weekday::Tue));
        debug_assert_eq!(rug_fuzz_2.parse(), Ok(Weekday::Wed));
        debug_assert_eq!(rug_fuzz_3.parse(), Ok(Weekday::Thu));
        debug_assert_eq!(rug_fuzz_4.parse(), Ok(Weekday::Fri));
        debug_assert_eq!(rug_fuzz_5.parse(), Ok(Weekday::Sat));
        debug_assert_eq!(rug_fuzz_6.parse(), Ok(Weekday::Sun));
             }
});    }
    #[test]
    fn test_weekday_try_from_u8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(u8, u8, u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Weekday::try_from(rug_fuzz_0), Ok(Weekday::Mon));
        debug_assert_eq!(Weekday::try_from(rug_fuzz_1), Ok(Weekday::Tue));
        debug_assert_eq!(Weekday::try_from(rug_fuzz_2), Ok(Weekday::Wed));
        debug_assert_eq!(Weekday::try_from(rug_fuzz_3), Ok(Weekday::Thu));
        debug_assert_eq!(Weekday::try_from(rug_fuzz_4), Ok(Weekday::Fri));
        debug_assert_eq!(Weekday::try_from(rug_fuzz_5), Ok(Weekday::Sat));
        debug_assert_eq!(Weekday::try_from(rug_fuzz_6), Ok(Weekday::Sun));
        debug_assert!(Weekday::try_from(rug_fuzz_7).is_err());
             }
});    }
    #[test]
    fn test_weekday_succ() {
        let _rug_st_tests_llm_16_628_rrrruuuugggg_test_weekday_succ = 0;
        debug_assert_eq!(Weekday::Mon.succ(), Weekday::Tue);
        debug_assert_eq!(Weekday::Tue.succ(), Weekday::Wed);
        debug_assert_eq!(Weekday::Wed.succ(), Weekday::Thu);
        debug_assert_eq!(Weekday::Thu.succ(), Weekday::Fri);
        debug_assert_eq!(Weekday::Fri.succ(), Weekday::Sat);
        debug_assert_eq!(Weekday::Sat.succ(), Weekday::Sun);
        debug_assert_eq!(Weekday::Sun.succ(), Weekday::Mon);
        let _rug_ed_tests_llm_16_628_rrrruuuugggg_test_weekday_succ = 0;
    }
    #[test]
    fn test_weekday_pred() {
        let _rug_st_tests_llm_16_628_rrrruuuugggg_test_weekday_pred = 0;
        debug_assert_eq!(Weekday::Mon.pred(), Weekday::Sun);
        debug_assert_eq!(Weekday::Tue.pred(), Weekday::Mon);
        debug_assert_eq!(Weekday::Wed.pred(), Weekday::Tue);
        debug_assert_eq!(Weekday::Thu.pred(), Weekday::Wed);
        debug_assert_eq!(Weekday::Fri.pred(), Weekday::Thu);
        debug_assert_eq!(Weekday::Sat.pred(), Weekday::Fri);
        debug_assert_eq!(Weekday::Sun.pred(), Weekday::Sat);
        let _rug_ed_tests_llm_16_628_rrrruuuugggg_test_weekday_pred = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_629 {
    use crate::Weekday;
    #[test]
    fn test_num_days_from_sunday() {
        let _rug_st_tests_llm_16_629_rrrruuuugggg_test_num_days_from_sunday = 0;
        debug_assert_eq!(Weekday::Sun.num_days_from_sunday(), 0);
        debug_assert_eq!(Weekday::Mon.num_days_from_sunday(), 1);
        debug_assert_eq!(Weekday::Tue.num_days_from_sunday(), 2);
        debug_assert_eq!(Weekday::Wed.num_days_from_sunday(), 3);
        debug_assert_eq!(Weekday::Thu.num_days_from_sunday(), 4);
        debug_assert_eq!(Weekday::Fri.num_days_from_sunday(), 5);
        debug_assert_eq!(Weekday::Sat.num_days_from_sunday(), 6);
        let _rug_ed_tests_llm_16_629_rrrruuuugggg_test_num_days_from_sunday = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_630 {
    use super::*;
    use crate::*;
    #[test]
    fn monday_number_from_monday() {
        let _rug_st_tests_llm_16_630_rrrruuuugggg_monday_number_from_monday = 0;
        debug_assert_eq!(Weekday::Mon.number_from_monday(), 1);
        let _rug_ed_tests_llm_16_630_rrrruuuugggg_monday_number_from_monday = 0;
    }
    #[test]
    fn tuesday_number_from_monday() {
        let _rug_st_tests_llm_16_630_rrrruuuugggg_tuesday_number_from_monday = 0;
        debug_assert_eq!(Weekday::Tue.number_from_monday(), 2);
        let _rug_ed_tests_llm_16_630_rrrruuuugggg_tuesday_number_from_monday = 0;
    }
    #[test]
    fn wednesday_number_from_monday() {
        let _rug_st_tests_llm_16_630_rrrruuuugggg_wednesday_number_from_monday = 0;
        debug_assert_eq!(Weekday::Wed.number_from_monday(), 3);
        let _rug_ed_tests_llm_16_630_rrrruuuugggg_wednesday_number_from_monday = 0;
    }
    #[test]
    fn thursday_number_from_monday() {
        let _rug_st_tests_llm_16_630_rrrruuuugggg_thursday_number_from_monday = 0;
        debug_assert_eq!(Weekday::Thu.number_from_monday(), 4);
        let _rug_ed_tests_llm_16_630_rrrruuuugggg_thursday_number_from_monday = 0;
    }
    #[test]
    fn friday_number_from_monday() {
        let _rug_st_tests_llm_16_630_rrrruuuugggg_friday_number_from_monday = 0;
        debug_assert_eq!(Weekday::Fri.number_from_monday(), 5);
        let _rug_ed_tests_llm_16_630_rrrruuuugggg_friday_number_from_monday = 0;
    }
    #[test]
    fn saturday_number_from_monday() {
        let _rug_st_tests_llm_16_630_rrrruuuugggg_saturday_number_from_monday = 0;
        debug_assert_eq!(Weekday::Sat.number_from_monday(), 6);
        let _rug_ed_tests_llm_16_630_rrrruuuugggg_saturday_number_from_monday = 0;
    }
    #[test]
    fn sunday_number_from_monday() {
        let _rug_st_tests_llm_16_630_rrrruuuugggg_sunday_number_from_monday = 0;
        debug_assert_eq!(Weekday::Sun.number_from_monday(), 7);
        let _rug_ed_tests_llm_16_630_rrrruuuugggg_sunday_number_from_monday = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_631 {
    use super::*;
    use crate::*;
    #[test]
    fn test_number_from_sunday() {
        let _rug_st_tests_llm_16_631_rrrruuuugggg_test_number_from_sunday = 0;
        debug_assert_eq!(Weekday::Sun.number_from_sunday(), 1);
        debug_assert_eq!(Weekday::Mon.number_from_sunday(), 2);
        debug_assert_eq!(Weekday::Tue.number_from_sunday(), 3);
        debug_assert_eq!(Weekday::Wed.number_from_sunday(), 4);
        debug_assert_eq!(Weekday::Thu.number_from_sunday(), 5);
        debug_assert_eq!(Weekday::Fri.number_from_sunday(), 6);
        debug_assert_eq!(Weekday::Sat.number_from_sunday(), 7);
        let _rug_ed_tests_llm_16_631_rrrruuuugggg_test_number_from_sunday = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_632 {
    use crate::Weekday;
    #[test]
    fn test_pred() {
        let _rug_st_tests_llm_16_632_rrrruuuugggg_test_pred = 0;
        debug_assert_eq!(Weekday::Mon.pred(), Weekday::Sun);
        debug_assert_eq!(Weekday::Tue.pred(), Weekday::Mon);
        debug_assert_eq!(Weekday::Wed.pred(), Weekday::Tue);
        debug_assert_eq!(Weekday::Thu.pred(), Weekday::Wed);
        debug_assert_eq!(Weekday::Fri.pred(), Weekday::Thu);
        debug_assert_eq!(Weekday::Sat.pred(), Weekday::Fri);
        debug_assert_eq!(Weekday::Sun.pred(), Weekday::Sat);
        let _rug_ed_tests_llm_16_632_rrrruuuugggg_test_pred = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_633 {
    use super::*;
    use crate::*;
    #[test]
    fn test_succ() {
        let _rug_st_tests_llm_16_633_rrrruuuugggg_test_succ = 0;
        debug_assert_eq!(Weekday::Mon.succ(), Weekday::Tue);
        debug_assert_eq!(Weekday::Tue.succ(), Weekday::Wed);
        debug_assert_eq!(Weekday::Wed.succ(), Weekday::Thu);
        debug_assert_eq!(Weekday::Thu.succ(), Weekday::Fri);
        debug_assert_eq!(Weekday::Fri.succ(), Weekday::Sat);
        debug_assert_eq!(Weekday::Sat.succ(), Weekday::Sun);
        debug_assert_eq!(Weekday::Sun.succ(), Weekday::Mon);
        let _rug_ed_tests_llm_16_633_rrrruuuugggg_test_succ = 0;
    }
}
