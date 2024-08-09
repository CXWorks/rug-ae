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

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8769() {
//    rusty_monitor::set_test_id(8769);
    let mut u16_0: u16 = 98u16;
    let mut u8_0: u8 = 10u8;
    let mut u8_1: u8 = 95u8;
    let mut u8_2: u8 = 0u8;
    let mut i32_0: i32 = 376i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut i64_0: i64 = 86400i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_0);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i64_1: i64 = 86400i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut i64_2: i64 = 60i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut i64_3: i64 = -63i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_3);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut i64_4: i64 = 131i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_4);
    let mut i64_5: i64 = 60i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_5);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_5, duration_4);
    let mut i32_1: i32 = 3i32;
    let mut i64_6: i64 = 1000000000i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_6, i32_1);
    let mut i64_7: i64 = 1000i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::days(i64_7);
    let mut duration_9: std::time::Duration = crate::duration::Duration::abs_std(duration_8);
    let mut i64_8: i64 = 7i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::hours(i64_8);
    let mut i64_9: i64 = 12i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_9);
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_11, duration_10);
    let mut duration_13: std::time::Duration = crate::duration::Duration::abs_std(duration_12);
    let mut i64_10: i64 = -10i64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::hours(i64_10);
    let mut u16_1: u16 = 1u16;
    let mut i32_2: i32 = 280i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_1);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_14);
    let mut i64_11: i64 = 9223372036854775807i64;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_11);
    let mut u16_2: u16 = 367u16;
    let mut i32_3: i32 = 25i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_2);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_add(date_3, duration_15);
    let mut option_0: std::option::Option<crate::date::Date> = crate::date::Date::previous_day(date_4);
    let mut option_1: std::option::Option<crate::date::Date> = crate::date::Date::previous_day(date_2);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_milli(u8_2, u8_1, u8_0, u16_0);
//    panic!("From RustyUnit with love");
}
}