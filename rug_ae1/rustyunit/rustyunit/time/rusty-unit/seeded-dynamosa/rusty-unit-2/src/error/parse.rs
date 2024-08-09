//! Error that occurred at some stage of parsing

use core::convert::TryFrom;
use core::fmt;

use crate::error::{self, ParseFromDescription, TryFromParsed};

/// An error that occurred at some stage of parsing.
#[cfg_attr(__time_03_docs, doc(cfg(feature = "parsing")))]
#[allow(variant_size_differences)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Parse {
    #[allow(clippy::missing_docs_in_private_items)]
    TryFromParsed(TryFromParsed),
    #[allow(clippy::missing_docs_in_private_items)]
    ParseFromDescription(ParseFromDescription),
    /// The input should have ended, but there were characters remaining.
    #[non_exhaustive]
    UnexpectedTrailingCharacters,
}

impl fmt::Display for Parse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TryFromParsed(err) => err.fmt(f),
            Self::ParseFromDescription(err) => err.fmt(f),
            Self::UnexpectedTrailingCharacters => f.write_str("unexpected trailing characters"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Parse {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::TryFromParsed(err) => Some(err),
            Self::ParseFromDescription(err) => Some(err),
            Self::UnexpectedTrailingCharacters => None,
        }
    }
}

#[cfg_attr(__time_03_docs, doc(cfg(feature = "parsing")))]
impl From<TryFromParsed> for Parse {
    fn from(err: TryFromParsed) -> Self {
        Self::TryFromParsed(err)
    }
}

#[cfg_attr(__time_03_docs, doc(cfg(feature = "parsing")))]
impl TryFrom<Parse> for TryFromParsed {
    type Error = error::DifferentVariant;

    fn try_from(err: Parse) -> Result<Self, Self::Error> {
        match err {
            Parse::TryFromParsed(err) => Ok(err),
            _ => Err(error::DifferentVariant),
        }
    }
}

#[cfg_attr(__time_03_docs, doc(cfg(feature = "parsing")))]
impl From<ParseFromDescription> for Parse {
    fn from(err: ParseFromDescription) -> Self {
        Self::ParseFromDescription(err)
    }
}

#[cfg_attr(__time_03_docs, doc(cfg(feature = "parsing")))]
impl TryFrom<Parse> for ParseFromDescription {
    type Error = error::DifferentVariant;

    fn try_from(err: Parse) -> Result<Self, Self::Error> {
        match err {
            Parse::ParseFromDescription(err) => Ok(err),
            _ => Err(error::DifferentVariant),
        }
    }
}

#[cfg_attr(__time_03_docs, doc(cfg(feature = "parsing")))]
impl From<Parse> for crate::Error {
    fn from(err: Parse) -> Self {
        match err {
            Parse::TryFromParsed(err) => Self::TryFromParsed(err),
            Parse::ParseFromDescription(err) => Self::ParseFromDescription(err),
            Parse::UnexpectedTrailingCharacters => Self::UnexpectedTrailingCharacters,
        }
    }
}

#[cfg_attr(__time_03_docs, doc(cfg(feature = "parsing")))]
impl TryFrom<crate::Error> for Parse {
    type Error = error::DifferentVariant;

    fn try_from(err: crate::Error) -> Result<Self, Self::Error> {
        match err {
            crate::Error::ParseFromDescription(err) => Ok(Self::ParseFromDescription(err)),
            crate::Error::UnexpectedTrailingCharacters => Ok(Self::UnexpectedTrailingCharacters),
            crate::Error::TryFromParsed(err) => Ok(Self::TryFromParsed(err)),
            _ => Err(error::DifferentVariant),
        }
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_413() {
//    rusty_monitor::set_test_id(413);
    let mut month_0: month::Month = crate::month::Month::November;
    let mut i32_0: i32 = 43i32;
    let mut month_1: month::Month = crate::month::Month::February;
    let mut i32_1: i32 = 3600i32;
    let mut month_2: month::Month = crate::month::Month::February;
    let mut i32_2: i32 = 1721425i32;
    let mut u16_0: u16 = 7u16;
    let mut i32_3: i32 = 308i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_0);
    let mut month_3: month::Month = crate::date::Date::month(date_0);
    let mut i32_4: i32 = 172i32;
    let mut month_4: month::Month = crate::month::Month::July;
    let mut i32_5: i32 = 122i32;
    let mut month_5: month::Month = crate::month::Month::August;
    let mut i32_6: i32 = 364i32;
    let mut month_6: month::Month = crate::month::Month::February;
    let mut i32_7: i32 = 20i32;
    let mut month_7: month::Month = crate::month::Month::September;
    let mut i32_8: i32 = 111i32;
    let mut month_8: month::Month = crate::month::Month::October;
    let mut i32_9: i32 = 268i32;
    let mut month_9: month::Month = crate::month::Month::December;
    let mut i32_10: i32 = 370i32;
    let mut u8_0: u8 = crate::util::days_in_year_month(i32_10, month_9);
    let mut u8_1: u8 = crate::util::days_in_year_month(i32_9, month_8);
    let mut u8_2: u8 = crate::util::days_in_year_month(i32_8, month_7);
    let mut u8_3: u8 = crate::util::days_in_year_month(i32_7, month_6);
    let mut u8_4: u8 = crate::util::days_in_year_month(i32_6, month_5);
    let mut u8_5: u8 = crate::util::days_in_year_month(i32_5, month_4);
    let mut u8_6: u8 = crate::util::days_in_year_month(i32_4, month_3);
    let mut u8_7: u8 = crate::util::days_in_year_month(i32_2, month_2);
    let mut u8_8: u8 = crate::util::days_in_year_month(i32_1, month_1);
    let mut u8_9: u8 = crate::util::days_in_year_month(i32_0, month_0);
//    panic!("From RustyUnit with love");
}
}