//! Description of how types should be formatted and parsed.
//!
//! The formatted value will be output to the provided writer. Format descriptions can be
//! [well-known](crate::format_description::well_known) or obtained by using the
//! [`format_description!`](crate::macros::format_description) macro, the
//! [`format_description::parse`](crate::format_description::parse()) function.

pub mod component;
pub mod modifier;
#[cfg(feature = "alloc")]
pub(crate) mod parse;

#[cfg(feature = "alloc")]
use alloc::string::String;
use core::convert::TryFrom;
#[cfg(feature = "alloc")]
use core::fmt;

pub use self::component::Component;
#[cfg(feature = "alloc")]
pub use self::parse::parse;
use crate::error;

/// Helper methods.
#[cfg(feature = "alloc")]
mod helper {
    /// Consume all leading whitespace, advancing `index` as appropriate.
    #[must_use = "This does not modify the original slice."]
    pub(crate) fn consume_whitespace<'a>(bytes: &'a [u8], index: &mut usize) -> &'a [u8] {
        let first_non_whitespace = bytes
            .iter()
            .position(|c| !c.is_ascii_whitespace())
            .unwrap_or(bytes.len());
        *index += first_non_whitespace;
        &bytes[first_non_whitespace..]
    }
}

/// Well-known formats, typically RFCs.
pub mod well_known {
    /// The format described in [RFC 3339](https://tools.ietf.org/html/rfc3339#section-5.6).
    ///
    /// Format example: 1985-04-12T23:20:50.52Z
    ///
    /// ```rust
    /// # use time::{format_description::well_known::Rfc3339, macros::datetime, OffsetDateTime};
    /// assert_eq!(
    ///     OffsetDateTime::parse("1985-04-12T23:20:50.52Z", &Rfc3339)?,
    ///     datetime!(1985-04-12 23:20:50.52 +00:00)
    /// );
    /// # Ok::<_, time::Error>(())
    /// ```
    ///
    /// ```rust
    /// # use time::{format_description::well_known::Rfc3339, macros::datetime};
    /// assert_eq!(
    ///     datetime!(1985-04-12 23:20:50.52 +00:00).format(&Rfc3339)?,
    ///     "1985-04-12T23:20:50.52Z"
    /// );
    /// # Ok::<_, time::Error>(())
    /// ```
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Rfc3339;

    /// The format described in [RFC 2822](https://tools.ietf.org/html/rfc2822#section-3.3).
    ///
    /// Example: Fri, 21 Nov 1997 09:55:06 -0600
    ///
    /// # Examples
    /// ```rust
    /// # use time::{format_description::well_known::Rfc2822, macros::datetime, OffsetDateTime};
    /// assert_eq!(
    ///     OffsetDateTime::parse("Sat, 12 Jun 1993 13:25:19 GMT", &Rfc2822)?,
    ///     datetime!(1993-06-12 13:25:19 +00:00)
    /// );
    /// # Ok::<_, time::Error>(())
    /// ```
    ///
    /// ```rust
    /// # use time::{format_description::well_known::Rfc2822, macros::datetime};
    /// assert_eq!(
    ///     datetime!(1997-11-21 09:55:06 -06:00).format(&Rfc2822)?,
    ///     "Fri, 21 Nov 1997 09:55:06 -0600"
    /// );
    /// # Ok::<_, time::Error>(())
    /// ```
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Rfc2822;
}

/// A complete description of how to format and parse a type.
#[non_exhaustive]
#[cfg_attr(not(feature = "alloc"), derive(Debug))]
#[derive(Clone, PartialEq, Eq)]
pub enum FormatItem<'a> {
    /// Bytes that are formatted as-is.
    ///
    /// **Note**: If you call the `format` method that returns a `String`, these bytes will be
    /// passed through `String::from_utf8_lossy`.
    Literal(&'a [u8]),
    /// A minimal representation of a single non-literal item.
    Component(Component),
    /// A series of literals or components that collectively form a partial or complete
    /// description.
    Compound(&'a [Self]),
    /// A `FormatItem` that may or may not be present when parsing. If parsing fails, there will be
    /// no effect on the resulting `struct`.
    ///
    /// This variant has no effect on formatting, as the value is guaranteed to be present.
    Optional(&'a Self),
    /// A series of `FormatItem`s where, when parsing, the first successful parse is used. When
    /// formatting, the first element of the slice is used.  An empty slice is a no-op when
    /// formatting or parsing.
    First(&'a [Self]),
}

#[cfg(feature = "alloc")]
impl fmt::Debug for FormatItem<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FormatItem::Literal(literal) => f.write_str(&String::from_utf8_lossy(literal)),
            FormatItem::Component(component) => component.fmt(f),
            FormatItem::Compound(compound) => compound.fmt(f),
            FormatItem::Optional(item) => f.debug_tuple("Optional").field(item).finish(),
            FormatItem::First(items) => f.debug_tuple("First").field(items).finish(),
        }
    }
}

impl From<Component> for FormatItem<'_> {
    fn from(component: Component) -> Self {
        Self::Component(component)
    }
}

impl TryFrom<FormatItem<'_>> for Component {
    type Error = error::DifferentVariant;

    fn try_from(value: FormatItem<'_>) -> Result<Self, Self::Error> {
        match value {
            FormatItem::Component(component) => Ok(component),
            _ => Err(error::DifferentVariant),
        }
    }
}

impl<'a> From<&'a [FormatItem<'_>]> for FormatItem<'a> {
    fn from(items: &'a [FormatItem<'_>]) -> FormatItem<'a> {
        FormatItem::Compound(items)
    }
}

impl<'a> TryFrom<FormatItem<'a>> for &[FormatItem<'a>] {
    type Error = error::DifferentVariant;

    fn try_from(value: FormatItem<'a>) -> Result<Self, Self::Error> {
        match value {
            FormatItem::Compound(items) => Ok(items),
            _ => Err(error::DifferentVariant),
        }
    }
}

impl PartialEq<Component> for FormatItem<'_> {
    fn eq(&self, rhs: &Component) -> bool {
        matches!(self, FormatItem::Component(component) if component == rhs)
    }
}

impl PartialEq<FormatItem<'_>> for Component {
    fn eq(&self, rhs: &FormatItem<'_>) -> bool {
        rhs == self
    }
}

impl PartialEq<&[FormatItem<'_>]> for FormatItem<'_> {
    fn eq(&self, rhs: &&[FormatItem<'_>]) -> bool {
        matches!(self, FormatItem::Compound(compound) if compound == rhs)
    }
}

impl PartialEq<FormatItem<'_>> for &[FormatItem<'_>] {
    fn eq(&self, rhs: &FormatItem<'_>) -> bool {
        rhs == self
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4835() {
    rusty_monitor::set_test_id(4835);
    let mut i32_0: i32 = 118i32;
    let mut u32_0: u32 = 91u32;
    let mut u8_0: u8 = 59u8;
    let mut u8_1: u8 = 23u8;
    let mut u8_2: u8 = 1u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u32_1: u32 = 23u32;
    let mut u8_3: u8 = 13u8;
    let mut u8_4: u8 = 50u8;
    let mut u8_5: u8 = 50u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut u32_2: u32 = 40u32;
    let mut u8_6: u8 = 38u8;
    let mut u8_7: u8 = 88u8;
    let mut u8_8: u8 = 25u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_3: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut u32_3: u32 = 53u32;
    let mut u8_9: u8 = 18u8;
    let mut u8_10: u8 = 84u8;
    let mut u8_11: u8 = 90u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_3);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_4: u32 = 91u32;
    let mut u8_12: u8 = 67u8;
    let mut u8_13: u8 = 67u8;
    let mut u8_14: u8 = 94u8;
    let mut u32_5: u32 = 40u32;
    let mut u8_15: u8 = 48u8;
    let mut u8_16: u8 = 64u8;
    let mut u8_17: u8 = 58u8;
    let mut time_5: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_17, u8_16, u8_15, u32_5);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_6: u32 = 16u32;
    let mut u8_18: u8 = 99u8;
    let mut u8_19: u8 = 42u8;
    let mut u8_20: u8 = 77u8;
    let mut i8_0: i8 = -53i8;
    let mut i8_1: i8 = 17i8;
    let mut i8_2: i8 = 45i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = -50i8;
    let mut i8_4: i8 = -2i8;
    let mut i8_5: i8 = 32i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u32_7: u32 = 19u32;
    let mut u8_21: u8 = 11u8;
    let mut u8_22: u8 = 77u8;
    let mut u8_23: u8 = 2u8;
    let mut time_6: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_23, u8_22, u8_21, u32_7);
    let mut i8_6: i8 = -45i8;
    let mut i8_7: i8 = -65i8;
    let mut i8_8: i8 = 38i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i8_9: i8 = 37i8;
    let mut i8_10: i8 = 0i8;
    let mut i8_11: i8 = 22i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i8_12: i8 = 19i8;
    let mut i8_13: i8 = 92i8;
    let mut i8_14: i8 = -78i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut i8_15: i8 = -71i8;
    let mut i8_16: i8 = -111i8;
    let mut i8_17: i8 = -110i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut u32_8: u32 = 47u32;
    let mut u8_24: u8 = 85u8;
    let mut u8_25: u8 = 47u8;
    let mut u8_26: u8 = 3u8;
    let mut time_7: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_26, u8_25, u8_24, u32_8);
    let mut i8_18: i8 = 125i8;
    let mut i8_19: i8 = 110i8;
    let mut i8_20: i8 = -48i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut i8_21: i8 = 76i8;
    let mut i8_22: i8 = -113i8;
    let mut i8_23: i8 = -61i8;
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_23, i8_22, i8_21);
    let mut u32_9: u32 = 50u32;
    let mut u8_27: u8 = 31u8;
    let mut u8_28: u8 = 97u8;
    let mut u8_29: u8 = 69u8;
    let mut time_8: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_29, u8_28, u8_27, u32_9);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i8_24: i8 = -7i8;
    let mut i8_25: i8 = -54i8;
    let mut i8_26: i8 = -68i8;
    let mut utcoffset_8: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_26, i8_25, i8_24);
    let mut i32_1: i32 = -153i32;
    let mut i32_2: i32 = 4i32;
    let mut i64_0: i64 = -2i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_2);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_1);
    let mut i8_27: i8 = -52i8;
    let mut i8_28: i8 = -2i8;
    let mut i8_29: i8 = -28i8;
    let mut utcoffset_9: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_29, i8_28, i8_27);
    let mut i32_3: i32 = -45i32;
    let mut i64_1: i64 = 201i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_3);
    let mut i8_30: i8 = 56i8;
    let mut i8_31: i8 = -50i8;
    let mut i8_32: i8 = 115i8;
    let mut utcoffset_10: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_32, i8_31, i8_30);
    let mut u32_10: u32 = 27u32;
    let mut u8_30: u8 = 2u8;
    let mut u8_31: u8 = 40u8;
    let mut u8_32: u8 = 31u8;
    let mut time_9: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_32, u8_31, u8_30, u32_10);
    let mut u32_11: u32 = 89u32;
    let mut u8_33: u8 = 10u8;
    let mut u8_34: u8 = 49u8;
    let mut u8_35: u8 = 80u8;
    let mut time_10: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_35, u8_34, u8_33, u32_11);
    let mut f32_0: f32 = 129.330520f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i64_2: i64 = -57i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut i8_33: i8 = 41i8;
    let mut i8_34: i8 = -83i8;
    let mut i8_35: i8 = -91i8;
    let mut utcoffset_11: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_35, i8_34, i8_33);
    let mut i8_36: i8 = 9i8;
    let mut i8_37: i8 = -66i8;
    let mut i8_38: i8 = 9i8;
    let mut utcoffset_12: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_38, i8_37, i8_36);
    let mut i8_39: i8 = -18i8;
    let mut i8_40: i8 = -75i8;
    let mut i8_41: i8 = -15i8;
    let mut utcoffset_13: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_41, i8_40, i8_39);
    let mut i64_3: i64 = 145i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::minutes(i64_3);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::abs(duration_5);
    let mut f64_0: f64 = -100.121003f64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i64_4: i64 = -102i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut duration_9: std::time::Duration = crate::duration::Duration::abs_std(duration_8);
    let mut i64_5: i64 = -109i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::weeks(i64_5);
    let mut i8_42: i8 = 81i8;
    let mut i8_43: i8 = -24i8;
    let mut i8_44: i8 = 78i8;
    let mut utcoffset_14: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_44, i8_43, i8_42);
    let mut i64_6: i64 = -183i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::minutes(i64_6);
    let mut duration_12: std::time::Duration = crate::duration::Duration::abs_std(duration_11);
    let mut i64_7: i64 = -94i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_7);
    let mut i64_8: i64 = -16i64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::seconds(i64_8);
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_14, duration_13);
    let mut i8_45: i8 = -56i8;
    let mut i8_46: i8 = -107i8;
    let mut i8_47: i8 = -77i8;
    let mut utcoffset_15: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_47, i8_46, i8_45);
    let mut f32_1: f32 = -87.762006f32;
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_17: std::time::Duration = crate::duration::Duration::abs_std(duration_16);
    let mut padding_2: time::Padding = crate::time::Padding::Optimize;
    let mut u32_12: u32 = 48u32;
    let mut u8_36: u8 = 28u8;
    let mut u8_37: u8 = 84u8;
    let mut u8_38: u8 = 26u8;
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_9: i64 = crate::offset_date_time::OffsetDateTime::unix_timestamp(offsetdatetime_2);
    panic!("From RustyUnit with love");
}
}