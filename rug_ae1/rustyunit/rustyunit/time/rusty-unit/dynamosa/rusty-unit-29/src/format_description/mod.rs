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
fn rusty_test_6072() {
    rusty_monitor::set_test_id(6072);
    let mut f32_0: f32 = 100.636257f32;
    let mut i64_0: i64 = -76i64;
    let mut i32_0: i32 = 22i32;
    let mut i64_1: i64 = 151i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_0);
    let mut u32_0: u32 = 65u32;
    let mut u8_0: u8 = 73u8;
    let mut u8_1: u8 = 28u8;
    let mut u8_2: u8 = 98u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 64u16;
    let mut i32_1: i32 = 44i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_1);
    let mut f64_0: f64 = 20.036033f64;
    let mut i32_2: i32 = -50i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut i64_2: i64 = -79i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_2);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut f32_1: f32 = 117.012122f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut i64_3: i64 = -32i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut i128_0: i128 = 184i128;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut f32_2: f32 = -82.126878f32;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_2);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_7, duration_6);
    let mut f64_1: f64 = 108.679129f64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut duration_10: std::time::Duration = crate::duration::Duration::abs_std(duration_9);
    let mut i32_3: i32 = -31i32;
    let mut i64_4: i64 = 129i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_4, i32_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_11);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut i64_5: i64 = -94i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::seconds(i64_5);
    let mut i64_6: i64 = -38i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::seconds(i64_6);
    let mut u32_1: u32 = 94u32;
    let mut u8_3: u8 = 2u8;
    let mut u8_4: u8 = 32u8;
    let mut u8_5: u8 = 25u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_4, time_2);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_5);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    let mut i32_4: i32 = 47i32;
    let mut u32_2: u32 = 16u32;
    let mut u8_6: u8 = 21u8;
    let mut u8_7: u8 = 30u8;
    let mut u8_8: u8 = 99u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i64_7: i64 = 125i64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_7);
    let mut i64_8: i64 = 6i64;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::microseconds(i64_8);
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_15, duration_14);
    let mut i32_5: i32 = -128i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_5);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_16);
    let mut u8_9: u8 = 71u8;
    let mut u16_1: u16 = 4u16;
    let mut i32_6: i32 = -56i32;
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_6, u16_1);
    let mut i8_0: i8 = 6i8;
    let mut i8_1: i8 = 107i8;
    let mut i8_2: i8 = -44i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_6, utcoffset_2);
    let mut time_4: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_7);
    let mut u16_2: u16 = 38u16;
    let mut i32_7: i32 = -71i32;
    let mut date_5: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_7, u16_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_5, time_4);
    let mut primitivedatetime_2_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_2;
    let mut i64_9: i64 = -53i64;
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut f64_2: f64 = 31.576453f64;
    let mut duration_18: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_19: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_2);
    let mut duration_20: std::time::Duration = crate::duration::Duration::abs_std(duration_13);
    let mut duration_21: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i64_10: i64 = 34i64;
    let mut duration_22: crate::duration::Duration = crate::duration::Duration::microseconds(i64_9);
    let mut duration_23: crate::duration::Duration = crate::duration::Duration::weeks(i64_10);
    let mut tuple_0: (i32, month::Month, u8) = crate::date::Date::to_calendar_date(date_1);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_4, u8_9, weekday_1);
    panic!("From RustyUnit with love");
}
}