//! Various error types returned by methods in the time crate.

pub mod component_range;
pub mod conversion_range;
pub mod different_variant;
#[cfg(feature = "formatting")]
pub mod format;
#[cfg(feature = "local-offset")]
pub mod indeterminate_offset;
#[cfg(all(any(feature = "formatting", feature = "parsing"), feature = "alloc"))]
pub mod invalid_format_description;
#[cfg(feature = "parsing")]
pub mod parse;
#[cfg(feature = "parsing")]
pub mod parse_from_description;
#[cfg(feature = "parsing")]
pub mod try_from_parsed;

use core::fmt;

pub use component_range::ComponentRange;
pub use conversion_range::ConversionRange;
pub use different_variant::DifferentVariant;
#[cfg(feature = "formatting")]
pub use format::Format;
#[cfg(feature = "local-offset")]
pub use indeterminate_offset::IndeterminateOffset;
#[cfg(all(any(feature = "formatting", feature = "parsing"), feature = "alloc"))]
pub use invalid_format_description::InvalidFormatDescription;
#[cfg(feature = "parsing")]
pub use parse::Parse;
#[cfg(feature = "parsing")]
pub use parse_from_description::ParseFromDescription;
#[cfg(feature = "parsing")]
pub use try_from_parsed::TryFromParsed;

/// A unified error type for anything returned by a method in the time crate.
///
/// This can be used when you either don't know or don't care about the exact error returned.
/// `Result<_, time::Error>` (or its alias `time::Result<_>`) will work in these situations.
#[allow(missing_copy_implementations, variant_size_differences)]
#[allow(clippy::missing_docs_in_private_items)] // variants only
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    ConversionRange(ConversionRange),
    ComponentRange(ComponentRange),
    #[cfg(feature = "local-offset")]
    IndeterminateOffset(IndeterminateOffset),
    #[cfg(feature = "formatting")]
    Format(Format),
    #[cfg(feature = "parsing")]
    ParseFromDescription(ParseFromDescription),
    #[cfg(feature = "parsing")]
    #[non_exhaustive]
    UnexpectedTrailingCharacters,
    #[cfg(feature = "parsing")]
    TryFromParsed(TryFromParsed),
    #[cfg(all(any(feature = "formatting", feature = "parsing"), feature = "alloc"))]
    InvalidFormatDescription(InvalidFormatDescription),
    DifferentVariant(DifferentVariant),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConversionRange(e) => e.fmt(f),
            Self::ComponentRange(e) => e.fmt(f),
            #[cfg(feature = "local-offset")]
            Self::IndeterminateOffset(e) => e.fmt(f),
            #[cfg(feature = "formatting")]
            Self::Format(e) => e.fmt(f),
            #[cfg(feature = "parsing")]
            Self::ParseFromDescription(e) => e.fmt(f),
            #[cfg(feature = "parsing")]
            Self::UnexpectedTrailingCharacters => f.write_str("unexpected trailing characters"),
            #[cfg(feature = "parsing")]
            Self::TryFromParsed(e) => e.fmt(f),
            #[cfg(all(any(feature = "formatting", feature = "parsing"), feature = "alloc"))]
            Self::InvalidFormatDescription(e) => e.fmt(f),
            Self::DifferentVariant(e) => e.fmt(f),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ConversionRange(err) => Some(err),
            Self::ComponentRange(err) => Some(err),
            #[cfg(feature = "local-offset")]
            Self::IndeterminateOffset(err) => Some(err),
            #[cfg(feature = "formatting")]
            Self::Format(err) => Some(err),
            #[cfg(feature = "parsing")]
            Self::ParseFromDescription(err) => Some(err),
            #[cfg(feature = "parsing")]
            Self::UnexpectedTrailingCharacters => None,
            #[cfg(feature = "parsing")]
            Self::TryFromParsed(err) => Some(err),
            #[cfg(all(any(feature = "formatting", feature = "parsing"), feature = "alloc"))]
            Self::InvalidFormatDescription(err) => Some(err),
            Self::DifferentVariant(err) => Some(err),
        }
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5581() {
//    rusty_monitor::set_test_id(5581);
    let mut i8_0: i8 = 0i8;
    let mut i8_1: i8 = 24i8;
    let mut i8_2: i8 = 59i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 999999u32;
    let mut u8_0: u8 = 3u8;
    let mut u8_1: u8 = 11u8;
    let mut u8_2: u8 = 59u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 184i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_0, offset: utcoffset_0};
    let mut u16_0: u16 = 366u16;
    let mut i32_1: i32 = 10i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_1_ref_0: &crate::date::Date = &mut date_1;
    let mut i32_2: i32 = 348i32;
    let mut u16_1: u16 = 365u16;
    let mut i32_3: i32 = 5i32;
    let mut i64_0: i64 = 9223372036854775807i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_3);
    let mut i8_3: i8 = 5i8;
    let mut i8_4: i8 = 6i8;
    let mut i8_5: i8 = 59i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_1, utcoffset_1);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_2);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_1);
    let mut i8_6: i8 = 127i8;
    let mut i8_7: i8 = 2i8;
    let mut i8_8: i8 = 5i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i8_9: i8 = 24i8;
    let mut i8_10: i8 = 59i8;
    let mut i8_11: i8 = 24i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i64_1: i64 = 1000i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_1);
    let mut date_3_ref_0: &crate::date::Date = &mut date_3;
    let mut date_5: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_0);
//    panic!("From RustyUnit with love");
}
}