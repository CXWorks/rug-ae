//! The [`UtcOffset`] struct and its associated `impl`s.
use core::fmt;
use core::ops::Neg;
#[cfg(feature = "formatting")]
use std::io;
use crate::convert::*;
use crate::error;
#[cfg(feature = "formatting")]
use crate::formatting::Formattable;
#[cfg(feature = "parsing")]
use crate::parsing::Parsable;
#[cfg(feature = "local-offset")]
use crate::sys::local_offset_at;
#[cfg(feature = "local-offset")]
use crate::OffsetDateTime;
/// An offset from UTC.
///
/// This struct can store values up to ±23:59:59. If you need support outside this range, please
/// file an issue with your use case.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UtcOffset {
    #[allow(clippy::missing_docs_in_private_items)]
    hours: i8,
    #[allow(clippy::missing_docs_in_private_items)]
    minutes: i8,
    #[allow(clippy::missing_docs_in_private_items)]
    seconds: i8,
}
impl UtcOffset {
    /// A `UtcOffset` that is UTC.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// # use time_macros::offset;
    /// assert_eq!(UtcOffset::UTC, offset!(UTC));
    /// ```
    pub const UTC: Self = Self::__from_hms_unchecked(0, 0, 0);
    /// Create a `UtcOffset` representing an offset of the hours, minutes, and seconds provided, the
    /// validity of which must be guaranteed by the caller. All three parameters must have the same
    /// sign.
    #[doc(hidden)]
    pub const fn __from_hms_unchecked(hours: i8, minutes: i8, seconds: i8) -> Self {
        if hours < 0 {
            debug_assert!(minutes <= 0);
            debug_assert!(seconds <= 0);
        } else if hours > 0 {
            debug_assert!(minutes >= 0);
            debug_assert!(seconds >= 0);
        }
        if minutes < 0 {
            debug_assert!(seconds <= 0);
        } else if minutes > 0 {
            debug_assert!(seconds >= 0);
        }
        debug_assert!(hours.unsigned_abs() < 24);
        debug_assert!(minutes.unsigned_abs() < Minute.per(Hour));
        debug_assert!(seconds.unsigned_abs() < Second.per(Minute));
        Self { hours, minutes, seconds }
    }
    /// Create a `UtcOffset` representing an offset by the number of hours, minutes, and seconds
    /// provided.
    ///
    /// The sign of all three components should match. If they do not, all smaller components will
    /// have their signs flipped.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::from_hms(1, 2, 3)?.as_hms(), (1, 2, 3));
    /// assert_eq!(UtcOffset::from_hms(1, -2, -3)?.as_hms(), (1, 2, 3));
    /// # Ok::<_, time::Error>(())
    /// ```
    pub const fn from_hms(
        hours: i8,
        mut minutes: i8,
        mut seconds: i8,
    ) -> Result<Self, error::ComponentRange> {
        ensure_value_in_range!(hours in - 23 => 23);
        ensure_value_in_range!(
            minutes in - (Minute.per(Hour) as i8 - 1) => Minute.per(Hour) as i8 - 1
        );
        ensure_value_in_range!(
            seconds in - (Second.per(Minute) as i8 - 1) => Second.per(Minute) as i8 - 1
        );
        if (hours > 0 && minutes < 0) || (hours < 0 && minutes > 0) {
            minutes *= -1;
        }
        if (hours > 0 && seconds < 0) || (hours < 0 && seconds > 0)
            || (minutes > 0 && seconds < 0) || (minutes < 0 && seconds > 0)
        {
            seconds *= -1;
        }
        Ok(Self::__from_hms_unchecked(hours, minutes, seconds))
    }
    /// Create a `UtcOffset` representing an offset by the number of seconds provided.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::from_whole_seconds(3_723)?.as_hms(), (1, 2, 3));
    /// # Ok::<_, time::Error>(())
    /// ```
    pub const fn from_whole_seconds(
        seconds: i32,
    ) -> Result<Self, error::ComponentRange> {
        ensure_value_in_range!(
            seconds in - 24 * Second.per(Hour) as i32 - 1 => 24 * Second.per(Hour) as i32
            - 1
        );
        Ok(
            Self::__from_hms_unchecked(
                (seconds / Second.per(Hour) as i32) as _,
                ((seconds % Second.per(Hour) as i32) / Minute.per(Hour) as i32) as _,
                (seconds % Second.per(Minute) as i32) as _,
            ),
        )
    }
    /// Obtain the UTC offset as its hours, minutes, and seconds. The sign of all three components
    /// will always match. A positive value indicates an offset to the east; a negative to the west.
    ///
    /// ```rust
    /// # use time_macros::offset;
    /// assert_eq!(offset!(+1:02:03).as_hms(), (1, 2, 3));
    /// assert_eq!(offset!(-1:02:03).as_hms(), (-1, -2, -3));
    /// ```
    pub const fn as_hms(self) -> (i8, i8, i8) {
        (self.hours, self.minutes, self.seconds)
    }
    /// Obtain the number of whole hours the offset is from UTC. A positive value indicates an
    /// offset to the east; a negative to the west.
    ///
    /// ```rust
    /// # use time_macros::offset;
    /// assert_eq!(offset!(+1:02:03).whole_hours(), 1);
    /// assert_eq!(offset!(-1:02:03).whole_hours(), -1);
    /// ```
    pub const fn whole_hours(self) -> i8 {
        self.hours
    }
    /// Obtain the number of whole minutes the offset is from UTC. A positive value indicates an
    /// offset to the east; a negative to the west.
    ///
    /// ```rust
    /// # use time_macros::offset;
    /// assert_eq!(offset!(+1:02:03).whole_minutes(), 62);
    /// assert_eq!(offset!(-1:02:03).whole_minutes(), -62);
    /// ```
    pub const fn whole_minutes(self) -> i16 {
        self.hours as i16 * Minute.per(Hour) as i16 + self.minutes as i16
    }
    /// Obtain the number of minutes past the hour the offset is from UTC. A positive value
    /// indicates an offset to the east; a negative to the west.
    ///
    /// ```rust
    /// # use time_macros::offset;
    /// assert_eq!(offset!(+1:02:03).minutes_past_hour(), 2);
    /// assert_eq!(offset!(-1:02:03).minutes_past_hour(), -2);
    /// ```
    pub const fn minutes_past_hour(self) -> i8 {
        self.minutes
    }
    /// Obtain the number of whole seconds the offset is from UTC. A positive value indicates an
    /// offset to the east; a negative to the west.
    ///
    /// ```rust
    /// # use time_macros::offset;
    /// assert_eq!(offset!(+1:02:03).whole_seconds(), 3723);
    /// assert_eq!(offset!(-1:02:03).whole_seconds(), -3723);
    /// ```
    pub const fn whole_seconds(self) -> i32 {
        self.hours as i32 * Second.per(Hour) as i32
            + self.minutes as i32 * Second.per(Minute) as i32 + self.seconds as i32
    }
    /// Obtain the number of seconds past the minute the offset is from UTC. A positive value
    /// indicates an offset to the east; a negative to the west.
    ///
    /// ```rust
    /// # use time_macros::offset;
    /// assert_eq!(offset!(+1:02:03).seconds_past_minute(), 3);
    /// assert_eq!(offset!(-1:02:03).seconds_past_minute(), -3);
    /// ```
    pub const fn seconds_past_minute(self) -> i8 {
        self.seconds
    }
    /// Check if the offset is exactly UTC.
    ///
    ///
    /// ```rust
    /// # use time_macros::offset;
    /// assert!(!offset!(+1:02:03).is_utc());
    /// assert!(!offset!(-1:02:03).is_utc());
    /// assert!(offset!(UTC).is_utc());
    /// ```
    pub const fn is_utc(self) -> bool {
        self.hours == 0 && self.minutes == 0 && self.seconds == 0
    }
    /// Check if the offset is positive, or east of UTC.
    ///
    /// ```rust
    /// # use time_macros::offset;
    /// assert!(offset!(+1:02:03).is_positive());
    /// assert!(!offset!(-1:02:03).is_positive());
    /// assert!(!offset!(UTC).is_positive());
    /// ```
    pub const fn is_positive(self) -> bool {
        self.hours > 0 || self.minutes > 0 || self.seconds > 0
    }
    /// Check if the offset is negative, or west of UTC.
    ///
    /// ```rust
    /// # use time_macros::offset;
    /// assert!(!offset!(+1:02:03).is_negative());
    /// assert!(offset!(-1:02:03).is_negative());
    /// assert!(!offset!(UTC).is_negative());
    /// ```
    pub const fn is_negative(self) -> bool {
        self.hours < 0 || self.minutes < 0 || self.seconds < 0
    }
    /// Attempt to obtain the system's UTC offset at a known moment in time. If the offset cannot be
    /// determined, an error is returned.
    ///
    /// ```rust
    /// # use time::{UtcOffset, OffsetDateTime};
    /// let local_offset = UtcOffset::local_offset_at(OffsetDateTime::UNIX_EPOCH);
    /// # if false {
    /// assert!(local_offset.is_ok());
    /// # }
    /// ```
    #[cfg(feature = "local-offset")]
    pub fn local_offset_at(
        datetime: OffsetDateTime,
    ) -> Result<Self, error::IndeterminateOffset> {
        local_offset_at(datetime).ok_or(error::IndeterminateOffset)
    }
    /// Attempt to obtain the system's current UTC offset. If the offset cannot be determined, an
    /// error is returned.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// let local_offset = UtcOffset::current_local_offset();
    /// # if false {
    /// assert!(local_offset.is_ok());
    /// # }
    /// ```
    #[cfg(feature = "local-offset")]
    pub fn current_local_offset() -> Result<Self, error::IndeterminateOffset> {
        let now = OffsetDateTime::now_utc();
        local_offset_at(now).ok_or(error::IndeterminateOffset)
    }
}
#[cfg(feature = "formatting")]
impl UtcOffset {
    /// Format the `UtcOffset` using the provided [format description](crate::format_description).
    pub fn format_into(
        self,
        output: &mut impl io::Write,
        format: &(impl Formattable + ?Sized),
    ) -> Result<usize, error::Format> {
        format.format_into(output, None, None, Some(self))
    }
    /// Format the `UtcOffset` using the provided [format description](crate::format_description).
    ///
    /// ```rust
    /// # use time::format_description;
    /// # use time_macros::offset;
    /// let format = format_description::parse("[offset_hour sign:mandatory]:[offset_minute]")?;
    /// assert_eq!(offset!(+1).format(&format)?, "+01:00");
    /// # Ok::<_, time::Error>(())
    /// ```
    pub fn format(
        self,
        format: &(impl Formattable + ?Sized),
    ) -> Result<String, error::Format> {
        format.format(None, None, Some(self))
    }
}
#[cfg(feature = "parsing")]
impl UtcOffset {
    /// Parse a `UtcOffset` from the input using the provided [format
    /// description](crate::format_description).
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// # use time_macros::{offset, format_description};
    /// let format = format_description!("[offset_hour]:[offset_minute]");
    /// assert_eq!(UtcOffset::parse("-03:42", &format)?, offset!(-3:42));
    /// # Ok::<_, time::Error>(())
    /// ```
    pub fn parse(
        input: &str,
        description: &(impl Parsable + ?Sized),
    ) -> Result<Self, error::Parse> {
        description.parse_offset(input.as_bytes())
    }
}
impl fmt::Display for UtcOffset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, "{}{:02}:{:02}:{:02}", if self.is_negative() { '-' } else { '+' }, self
            .hours.abs(), self.minutes.abs(), self.seconds.abs()
        )
    }
}
impl fmt::Debug for UtcOffset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
impl Neg for UtcOffset {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::__from_hms_unchecked(-self.hours, -self.minutes, -self.seconds)
    }
}
#[cfg(test)]
mod tests_llm_16_470 {
    use crate::UtcOffset;
    #[test]
    fn from_hms_unchecked_positive() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let offset = UtcOffset::__from_hms_unchecked(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(offset.hours, 1);
        debug_assert_eq!(offset.minutes, 30);
        debug_assert_eq!(offset.seconds, 45);
             }
});    }
    #[test]
    fn from_hms_unchecked_negative() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let offset = UtcOffset::__from_hms_unchecked(
            -rug_fuzz_0,
            -rug_fuzz_1,
            -rug_fuzz_2,
        );
        debug_assert_eq!(offset.hours, - 5);
        debug_assert_eq!(offset.minutes, - 10);
        debug_assert_eq!(offset.seconds, - 20);
             }
});    }
    #[test]
    fn from_hms_unchecked_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let offset = UtcOffset::__from_hms_unchecked(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(offset.hours, 0);
        debug_assert_eq!(offset.minutes, 0);
        debug_assert_eq!(offset.seconds, 0);
             }
});    }
    #[test]
    #[should_panic]
    fn from_hms_unchecked_panic_mismatched_signs() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let _ = UtcOffset::__from_hms_unchecked(-rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
             }
});    }
    #[test]
    #[should_panic]
    fn from_hms_unchecked_panic_invalid_hours() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let _ = UtcOffset::__from_hms_unchecked(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
             }
});    }
    #[test]
    #[should_panic]
    fn from_hms_unchecked_panic_invalid_minutes() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let _ = UtcOffset::__from_hms_unchecked(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
             }
});    }
    #[test]
    #[should_panic]
    fn from_hms_unchecked_panic_invalid_seconds() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let _ = UtcOffset::__from_hms_unchecked(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_471 {
    use crate::UtcOffset;
    use crate::error::ComponentRange;
    #[test]
    fn as_hms_positive_offset() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let offset = UtcOffset::__from_hms_unchecked(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(offset.as_hms(), (1, 2, 3));
             }
});    }
    #[test]
    fn as_hms_negative_offset() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let offset = UtcOffset::__from_hms_unchecked(
            -rug_fuzz_0,
            -rug_fuzz_1,
            -rug_fuzz_2,
        );
        debug_assert_eq!(offset.as_hms(), (- 1, - 2, - 3));
             }
});    }
    #[test]
    fn as_hms_zero_offset() {
        let _rug_st_tests_llm_16_471_rrrruuuugggg_as_hms_zero_offset = 0;
        let offset = UtcOffset::UTC;
        debug_assert_eq!(offset.as_hms(), (0, 0, 0));
        let _rug_ed_tests_llm_16_471_rrrruuuugggg_as_hms_zero_offset = 0;
    }
    #[test]
    fn as_hms_max_offset() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let offset = UtcOffset::__from_hms_unchecked(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(offset.as_hms(), (23, 59, 59));
             }
});    }
    #[test]
    fn as_hms_min_offset() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let offset = UtcOffset::__from_hms_unchecked(
            -rug_fuzz_0,
            -rug_fuzz_1,
            -rug_fuzz_2,
        );
        debug_assert_eq!(offset.as_hms(), (- 23, - 59, - 59));
             }
});    }
    #[test]
    fn as_hms_with_constructor() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let offset = UtcOffset::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        debug_assert_eq!(offset.as_hms(), (5, 10, 15));
             }
});    }
    #[test]
    fn as_hms_with_constructor_negative() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let offset = UtcOffset::from_hms(-rug_fuzz_0, -rug_fuzz_1, -rug_fuzz_2).unwrap();
        debug_assert_eq!(offset.as_hms(), (- 5, - 10, - 15));
             }
});    }
    #[test]
    fn as_hms_with_constructor_invalid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17)) = <(i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(UtcOffset::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).is_err());
        debug_assert!(
            UtcOffset::from_hms(- rug_fuzz_3, rug_fuzz_4, rug_fuzz_5).is_err()
        );
        debug_assert!(UtcOffset::from_hms(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8).is_err());
        debug_assert!(
            UtcOffset::from_hms(rug_fuzz_9, - rug_fuzz_10, rug_fuzz_11).is_err()
        );
        debug_assert!(
            UtcOffset::from_hms(rug_fuzz_12, rug_fuzz_13, rug_fuzz_14).is_err()
        );
        debug_assert!(
            UtcOffset::from_hms(rug_fuzz_15, rug_fuzz_16, - rug_fuzz_17).is_err()
        );
             }
});    }
    #[test]
    fn as_hms_with_constructor_mix_signs() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let offset = UtcOffset::from_hms(-rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        debug_assert_eq!(offset.as_hms(), (- 5, - 10, - 15));
             }
});    }
    #[test]
    fn as_hms_with_constructor_mix_signs2() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let offset = UtcOffset::from_hms(rug_fuzz_0, -rug_fuzz_1, -rug_fuzz_2).unwrap();
        debug_assert_eq!(offset.as_hms(), (5, 10, 15));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_472 {
    use super::*;
    use crate::*;
    use crate::error::ComponentRange;
    #[test]
    fn from_hms_valid_positive_offsets() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            UtcOffset::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap().as_hms(),
            (1, 2, 3)
        );
             }
});    }
    #[test]
    fn from_hms_valid_negative_offsets() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            UtcOffset::from_hms(- rug_fuzz_0, - rug_fuzz_1, - rug_fuzz_2).unwrap()
            .as_hms(), (- 1, - 2, - 3)
        );
             }
});    }
    #[test]
    fn from_hms_valid_mixed_sign_offsets() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i8, i8, i8, i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            UtcOffset::from_hms(rug_fuzz_0, - rug_fuzz_1, - rug_fuzz_2).unwrap()
            .as_hms(), (1, 2, 3)
        );
        debug_assert_eq!(
            UtcOffset::from_hms(- rug_fuzz_3, rug_fuzz_4, rug_fuzz_5).unwrap().as_hms(),
            (- 1, - 2, - 3)
        );
             }
});    }
    #[test]
    fn from_hms_invalid_hours() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i8, i8, i8, i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(
            matches!(UtcOffset::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2),
            Err(ComponentRange))
        );
        debug_assert!(
            matches!(UtcOffset::from_hms(- rug_fuzz_3, rug_fuzz_4, rug_fuzz_5),
            Err(ComponentRange))
        );
             }
});    }
    #[test]
    fn from_hms_invalid_minutes() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i8, i8, i8, i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(
            matches!(UtcOffset::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2),
            Err(ComponentRange))
        );
        debug_assert!(
            matches!(UtcOffset::from_hms(rug_fuzz_3, - rug_fuzz_4, rug_fuzz_5),
            Err(ComponentRange))
        );
             }
});    }
    #[test]
    fn from_hms_invalid_seconds() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i8, i8, i8, i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(
            matches!(UtcOffset::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2),
            Err(ComponentRange))
        );
        debug_assert!(
            matches!(UtcOffset::from_hms(rug_fuzz_3, rug_fuzz_4, - rug_fuzz_5),
            Err(ComponentRange))
        );
             }
});    }
    #[test]
    fn from_hms_boundary_values() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8)) = <(i8, i8, i8, i8, i8, i8, i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            UtcOffset::from_hms(- rug_fuzz_0, - rug_fuzz_1, - rug_fuzz_2).unwrap()
            .as_hms(), (- 23, - 59, - 59)
        );
        debug_assert_eq!(
            UtcOffset::from_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5).unwrap().as_hms(),
            (23, 59, 59)
        );
        debug_assert_eq!(
            UtcOffset::from_hms(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8).unwrap().as_hms(),
            (0, 0, 0)
        );
             }
});    }
    #[test]
    fn from_hms_sign_flipping() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            UtcOffset::from_hms(- rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap().as_hms(),
            (- 1, - 2, - 3)
        );
        debug_assert_eq!(
            UtcOffset::from_hms(rug_fuzz_3, - rug_fuzz_4, rug_fuzz_5).unwrap().as_hms(),
            (1, 2, 3)
        );
        debug_assert_eq!(
            UtcOffset::from_hms(rug_fuzz_6, rug_fuzz_7, - rug_fuzz_8).unwrap().as_hms(),
            (1, 2, 3)
        );
        debug_assert_eq!(
            UtcOffset::from_hms(- rug_fuzz_9, - rug_fuzz_10, rug_fuzz_11).unwrap()
            .as_hms(), (- 1, - 2, - 3)
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_473 {
    use super::*;
    use crate::*;
    use crate::UtcOffset;
    use crate::error::ComponentRange;
    #[test]
    fn from_whole_seconds_valid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let seconds = rug_fuzz_0;
        let offset = UtcOffset::from_whole_seconds(seconds);
        debug_assert!(offset.is_ok());
        let offset = offset.unwrap();
        debug_assert_eq!(offset.as_hms(), (1, 2, 3));
             }
});    }
    #[test]
    fn from_whole_seconds_negative() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let seconds = -rug_fuzz_0;
        let offset = UtcOffset::from_whole_seconds(seconds);
        debug_assert!(offset.is_ok());
        let offset = offset.unwrap();
        debug_assert_eq!(offset.as_hms(), (- 1, - 2, - 3));
             }
});    }
    #[test]
    fn from_whole_seconds_too_large() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let seconds = rug_fuzz_0 * rug_fuzz_1;
        let offset = UtcOffset::from_whole_seconds(seconds);
        debug_assert!(offset.is_err());
        if let Err(e) = offset {
            debug_assert!(matches!(e, ComponentRange));
        }
             }
});    }
    #[test]
    fn from_whole_seconds_too_small() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let seconds = -rug_fuzz_0 * rug_fuzz_1;
        let offset = UtcOffset::from_whole_seconds(seconds);
        debug_assert!(offset.is_err());
        if let Err(e) = offset {
            debug_assert!(matches!(e, ComponentRange));
        }
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_474 {
    use crate::UtcOffset;
    #[test]
    fn test_is_negative() {
        let _rug_st_tests_llm_16_474_rrrruuuugggg_test_is_negative = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 2;
        let rug_fuzz_5 = 3;
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 3;
        let rug_fuzz_9 = 1;
        let rug_fuzz_10 = 2;
        let rug_fuzz_11 = 0;
        let rug_fuzz_12 = 0;
        let rug_fuzz_13 = 0;
        let rug_fuzz_14 = 3;
        let rug_fuzz_15 = 0;
        let rug_fuzz_16 = 2;
        let rug_fuzz_17 = 0;
        let rug_fuzz_18 = 1;
        let rug_fuzz_19 = 0;
        let rug_fuzz_20 = 0;
        let rug_fuzz_21 = 1;
        let rug_fuzz_22 = 2;
        let rug_fuzz_23 = 3;
        let rug_fuzz_24 = 1;
        let rug_fuzz_25 = 0;
        let rug_fuzz_26 = 0;
        let rug_fuzz_27 = 0;
        let rug_fuzz_28 = 2;
        let rug_fuzz_29 = 0;
        let rug_fuzz_30 = 0;
        let rug_fuzz_31 = 0;
        let rug_fuzz_32 = 3;
        let rug_fuzz_33 = 0;
        let rug_fuzz_34 = 2;
        let rug_fuzz_35 = 3;
        let rug_fuzz_36 = 1;
        let rug_fuzz_37 = 0;
        let rug_fuzz_38 = 3;
        let rug_fuzz_39 = 1;
        let rug_fuzz_40 = 2;
        let rug_fuzz_41 = 0;
        debug_assert!(! UtcOffset::UTC.is_negative());
        debug_assert!(
            ! UtcOffset::__from_hms_unchecked(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .is_negative()
        );
        debug_assert!(
            ! UtcOffset::__from_hms_unchecked(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .is_negative()
        );
        debug_assert!(
            ! UtcOffset::__from_hms_unchecked(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8)
            .is_negative()
        );
        debug_assert!(
            ! UtcOffset::__from_hms_unchecked(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11)
            .is_negative()
        );
        debug_assert!(
            ! UtcOffset::__from_hms_unchecked(rug_fuzz_12, rug_fuzz_13, rug_fuzz_14)
            .is_negative()
        );
        debug_assert!(
            ! UtcOffset::__from_hms_unchecked(rug_fuzz_15, rug_fuzz_16, rug_fuzz_17)
            .is_negative()
        );
        debug_assert!(
            ! UtcOffset::__from_hms_unchecked(rug_fuzz_18, rug_fuzz_19, rug_fuzz_20)
            .is_negative()
        );
        debug_assert!(
            UtcOffset::__from_hms_unchecked(- rug_fuzz_21, - rug_fuzz_22, - rug_fuzz_23)
            .is_negative()
        );
        debug_assert!(
            UtcOffset::__from_hms_unchecked(- rug_fuzz_24, rug_fuzz_25, rug_fuzz_26)
            .is_negative()
        );
        debug_assert!(
            UtcOffset::__from_hms_unchecked(rug_fuzz_27, - rug_fuzz_28, rug_fuzz_29)
            .is_negative()
        );
        debug_assert!(
            UtcOffset::__from_hms_unchecked(rug_fuzz_30, rug_fuzz_31, - rug_fuzz_32)
            .is_negative()
        );
        debug_assert!(
            UtcOffset::__from_hms_unchecked(rug_fuzz_33, - rug_fuzz_34, - rug_fuzz_35)
            .is_negative()
        );
        debug_assert!(
            UtcOffset::__from_hms_unchecked(- rug_fuzz_36, rug_fuzz_37, - rug_fuzz_38)
            .is_negative()
        );
        debug_assert!(
            UtcOffset::__from_hms_unchecked(- rug_fuzz_39, - rug_fuzz_40, rug_fuzz_41)
            .is_negative()
        );
        let _rug_ed_tests_llm_16_474_rrrruuuugggg_test_is_negative = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_475 {
    use crate::UtcOffset;
    #[test]
    fn is_positive_returns_true_for_positive_offsets() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let positive_offset = UtcOffset::__from_hms_unchecked(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
        );
        debug_assert!(positive_offset.is_positive());
        let positive_offset = UtcOffset::__from_hms_unchecked(
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
        );
        debug_assert!(positive_offset.is_positive());
        let positive_offset = UtcOffset::__from_hms_unchecked(
            rug_fuzz_6,
            rug_fuzz_7,
            rug_fuzz_8,
        );
        debug_assert!(positive_offset.is_positive());
        let positive_offset = UtcOffset::__from_hms_unchecked(
            rug_fuzz_9,
            rug_fuzz_10,
            rug_fuzz_11,
        );
        debug_assert!(positive_offset.is_positive());
             }
});    }
    #[test]
    fn is_positive_returns_false_for_negative_offsets() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let negative_offset = UtcOffset::__from_hms_unchecked(
            -rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
        );
        debug_assert!(! negative_offset.is_positive());
        let negative_offset = UtcOffset::__from_hms_unchecked(
            rug_fuzz_3,
            -rug_fuzz_4,
            rug_fuzz_5,
        );
        debug_assert!(! negative_offset.is_positive());
        let negative_offset = UtcOffset::__from_hms_unchecked(
            rug_fuzz_6,
            rug_fuzz_7,
            -rug_fuzz_8,
        );
        debug_assert!(! negative_offset.is_positive());
        let negative_offset = UtcOffset::__from_hms_unchecked(
            -rug_fuzz_9,
            -rug_fuzz_10,
            -rug_fuzz_11,
        );
        debug_assert!(! negative_offset.is_positive());
             }
});    }
    #[test]
    fn is_positive_returns_false_for_utc_offset() {
        let _rug_st_tests_llm_16_475_rrrruuuugggg_is_positive_returns_false_for_utc_offset = 0;
        let utc_offset = UtcOffset::UTC;
        debug_assert!(! utc_offset.is_positive());
        let _rug_ed_tests_llm_16_475_rrrruuuugggg_is_positive_returns_false_for_utc_offset = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_476_llm_16_476 {
    use crate::UtcOffset;
    use time_macros::offset;
    #[test]
    fn utc_offset_is_utc() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17)) = <(i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(UtcOffset::UTC.is_utc());
        debug_assert!(
            ! UtcOffset::__from_hms_unchecked(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .is_utc()
        );
        debug_assert!(
            ! UtcOffset::__from_hms_unchecked(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .is_utc()
        );
        debug_assert!(
            ! UtcOffset::__from_hms_unchecked(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8)
            .is_utc()
        );
        debug_assert!(
            ! UtcOffset::__from_hms_unchecked(rug_fuzz_9, rug_fuzz_10, - rug_fuzz_11)
            .is_utc()
        );
        debug_assert!(
            ! UtcOffset::__from_hms_unchecked(rug_fuzz_12, - rug_fuzz_13, rug_fuzz_14)
            .is_utc()
        );
        debug_assert!(
            ! UtcOffset::__from_hms_unchecked(- rug_fuzz_15, rug_fuzz_16, rug_fuzz_17)
            .is_utc()
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_477_llm_16_477 {
    use crate::UtcOffset;
    use time_macros::offset;
    #[test]
    fn minutes_past_hour_positive_offset() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            UtcOffset::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap()
            .minutes_past_hour(), 2
        );
             }
});    }
    #[test]
    fn minutes_past_hour_negative_offset() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            UtcOffset::from_hms(- rug_fuzz_0, - rug_fuzz_1, - rug_fuzz_2).unwrap()
            .minutes_past_hour(), - 2
        );
             }
});    }
    #[test]
    fn minutes_past_hour_zero_offset() {
        let _rug_st_tests_llm_16_477_llm_16_477_rrrruuuugggg_minutes_past_hour_zero_offset = 0;
        debug_assert_eq!(UtcOffset::UTC.minutes_past_hour(), 0);
        let _rug_ed_tests_llm_16_477_llm_16_477_rrrruuuugggg_minutes_past_hour_zero_offset = 0;
    }
    #[test]
    fn minutes_past_hour_no_offset() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            UtcOffset::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap()
            .minutes_past_hour(), 0
        );
             }
});    }
    #[test]
    fn minutes_past_hour_max_positive_offset() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            UtcOffset::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap()
            .minutes_past_hour(), 59
        );
             }
});    }
    #[test]
    fn minutes_past_hour_max_negative_offset() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            UtcOffset::from_hms(- rug_fuzz_0, - rug_fuzz_1, - rug_fuzz_2).unwrap()
            .minutes_past_hour(), - 59
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_478 {
    use crate::UtcOffset;
    #[test]
    fn seconds_past_minute_positive() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let offset = UtcOffset::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        debug_assert_eq!(offset.seconds_past_minute(), 3);
             }
});    }
    #[test]
    fn seconds_past_minute_negative() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let offset = UtcOffset::from_hms(-rug_fuzz_0, -rug_fuzz_1, -rug_fuzz_2).unwrap();
        debug_assert_eq!(offset.seconds_past_minute(), - 3);
             }
});    }
    #[test]
    fn seconds_past_minute_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let offset = UtcOffset::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        debug_assert_eq!(offset.seconds_past_minute(), 0);
             }
});    }
    #[test]
    fn seconds_past_minute_positive_negative() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let offset = UtcOffset::from_hms(rug_fuzz_0, -rug_fuzz_1, -rug_fuzz_2).unwrap();
        debug_assert_eq!(offset.seconds_past_minute(), 3);
             }
});    }
    #[test]
    fn seconds_past_minute_negative_positive() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let offset = UtcOffset::from_hms(-rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        debug_assert_eq!(offset.seconds_past_minute(), - 3);
             }
});    }
    #[test]
    fn seconds_past_minute_max() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let offset = UtcOffset::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        debug_assert_eq!(offset.seconds_past_minute(), 59);
             }
});    }
    #[test]
    fn seconds_past_minute_min() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let offset = UtcOffset::from_hms(-rug_fuzz_0, -rug_fuzz_1, -rug_fuzz_2).unwrap();
        debug_assert_eq!(offset.seconds_past_minute(), - 59);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_480 {
    use crate::UtcOffset;
    #[test]
    fn test_whole_minutes() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14)) = <(i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            UtcOffset::__from_hms_unchecked(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .whole_minutes(), 62
        );
        debug_assert_eq!(
            UtcOffset::__from_hms_unchecked(- rug_fuzz_3, - rug_fuzz_4, - rug_fuzz_5)
            .whole_minutes(), - 62
        );
        debug_assert_eq!(
            UtcOffset::__from_hms_unchecked(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8)
            .whole_minutes(), 0
        );
        debug_assert_eq!(
            UtcOffset::__from_hms_unchecked(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11)
            .whole_minutes(), 1439
        );
        debug_assert_eq!(
            UtcOffset::__from_hms_unchecked(- rug_fuzz_12, - rug_fuzz_13, - rug_fuzz_14)
            .whole_minutes(), - 1439
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_481 {
    use super::*;
    use crate::*;
    #[test]
    fn test_whole_seconds_positive_offset() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let offset = UtcOffset::__from_hms_unchecked(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(offset.whole_seconds(), 9045);
             }
});    }
    #[test]
    fn test_whole_seconds_negative_offset() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let offset = UtcOffset::__from_hms_unchecked(
            -rug_fuzz_0,
            -rug_fuzz_1,
            -rug_fuzz_2,
        );
        debug_assert_eq!(offset.whole_seconds(), - 9045);
             }
});    }
    #[test]
    fn test_whole_seconds_zero_offset() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let offset = UtcOffset::__from_hms_unchecked(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(offset.whole_seconds(), 0);
             }
});    }
    #[test]
    fn test_whole_seconds_max_offset() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let offset = UtcOffset::__from_hms_unchecked(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(offset.whole_seconds(), 23 * 3600 + 59 * 60 + 59);
             }
});    }
    #[test]
    fn test_whole_seconds_min_offset() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let offset = UtcOffset::__from_hms_unchecked(
            -rug_fuzz_0,
            -rug_fuzz_1,
            -rug_fuzz_2,
        );
        debug_assert_eq!(offset.whole_seconds(), - 23 * 3600 - 59 * 60 - 59);
             }
});    }
}
