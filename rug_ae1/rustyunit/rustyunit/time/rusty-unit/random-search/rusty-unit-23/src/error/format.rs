//! Error formatting a struct

use core::convert::TryFrom;
use core::fmt;
use std::io;

use crate::error;

/// An error occurred when formatting.
#[non_exhaustive]
#[allow(missing_copy_implementations)]
#[cfg_attr(__time_03_docs, doc(cfg(feature = "formatting")))]
#[derive(Debug)]
pub enum Format {
    /// The type being formatted does not contain sufficient information to format a component.
    #[non_exhaustive]
    InsufficientTypeInformation,
    /// The component named has a value that cannot be formatted into the requested format.
    ///
    /// This variant is only returned when using well-known formats.
    InvalidComponent(&'static str),
    /// A value of `std::io::Error` was returned internally.
    StdIo(io::Error),
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InsufficientTypeInformation => f.write_str(
                "The type being formatted does not contain sufficient information to format a \
                 component.",
            ),
            Self::InvalidComponent(component) => write!(
                f,
                "The {} component cannot be formatted into the requested format.",
                component
            ),
            Self::StdIo(err) => err.fmt(f),
        }
    }
}

impl From<io::Error> for Format {
    fn from(err: io::Error) -> Self {
        Self::StdIo(err)
    }
}

impl TryFrom<Format> for io::Error {
    type Error = error::DifferentVariant;

    fn try_from(err: Format) -> Result<Self, Self::Error> {
        match err {
            Format::StdIo(err) => Ok(err),
            _ => Err(error::DifferentVariant),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Format {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::InsufficientTypeInformation | Self::InvalidComponent(_) => None,
            Self::StdIo(ref err) => Some(err),
        }
    }
}

#[cfg_attr(__time_03_docs, doc(cfg(feature = "formatting")))]
impl From<Format> for crate::Error {
    fn from(original: Format) -> Self {
        Self::Format(original)
    }
}

#[cfg_attr(__time_03_docs, doc(cfg(feature = "formatting")))]
impl TryFrom<crate::Error> for Format {
    type Error = error::DifferentVariant;

    fn try_from(err: crate::Error) -> Result<Self, Self::Error> {
        match err {
            crate::Error::Format(err) => Ok(err),
            _ => Err(error::DifferentVariant),
        }
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4797() {
    rusty_monitor::set_test_id(4797);
    let mut bool_0: bool = false;
    let mut i64_0: i64 = 61i64;
    let mut i64_1: i64 = -34i64;
    let mut i64_2: i64 = -56i64;
    let mut str_0: &str = "94pnRbyGmmx8";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut i8_0: i8 = 70i8;
    let mut i8_1: i8 = 4i8;
    let mut i8_2: i8 = -102i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = 56i8;
    let mut i8_4: i8 = -85i8;
    let mut i8_5: i8 = -60i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_3: i64 = -72i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut i32_0: i32 = -79i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_0};
    let mut tuple_0: (i32, u16) = crate::offset_date_time::OffsetDateTime::to_ordinal_date(offsetdatetime_0);
    let mut str_1: &str = crate::error::component_range::ComponentRange::name(componentrange_0);
    panic!("From RustyUnit with love");
}
}