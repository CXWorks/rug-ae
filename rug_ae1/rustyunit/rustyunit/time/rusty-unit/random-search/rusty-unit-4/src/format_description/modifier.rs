//! Various modifiers for components.

#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use core::mem;

#[cfg(feature = "alloc")]
use crate::{error::InvalidFormatDescription, format_description::helper};

// region: date modifiers
/// Day of the month.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Day {
    /// The padding to obtain the minimum width.
    pub padding: Padding,
}

/// The representation of a month.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonthRepr {
    /// The number of the month (January is 1, December is 12).
    Numerical,
    /// The long form of the month name (e.g. "January").
    Long,
    /// The short form of the month name (e.g. "Jan").
    Short,
}

/// Month of the year.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Month {
    /// The padding to obtain the minimum width.
    pub padding: Padding,
    /// What form of representation should be used?
    pub repr: MonthRepr,
    /// Is the value case sensitive when parsing?
    pub case_sensitive: bool,
}

/// Ordinal day of the year.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ordinal {
    /// The padding to obtain the minimum width.
    pub padding: Padding,
}

/// The representation used for the day of the week.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeekdayRepr {
    /// The short form of the weekday (e.g. "Mon").
    Short,
    /// The long form of the weekday (e.g. "Monday").
    Long,
    /// A numerical representation using Sunday as the first day of the week.
    ///
    /// Sunday is either 0 or 1, depending on the other modifier's value.
    Sunday,
    /// A numerical representation using Monday as the first day of the week.
    ///
    /// Monday is either 0 or 1, depending on the other modifier's value.
    Monday,
}

/// Day of the week.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Weekday {
    /// What form of representation should be used?
    pub repr: WeekdayRepr,
    /// When using a numerical representation, should it be zero or one-indexed?
    pub one_indexed: bool,
    /// Is the value case sensitive when parsing?
    pub case_sensitive: bool,
}

/// The representation used for the week number.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeekNumberRepr {
    /// Week 1 is the week that contains January 4.
    Iso,
    /// Week 1 begins on the first Sunday of the calendar year.
    Sunday,
    /// Week 1 begins on the first Monday of the calendar year.
    Monday,
}

/// Week within the year.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WeekNumber {
    /// The padding to obtain the minimum width.
    pub padding: Padding,
    /// What kind of representation should be used?
    pub repr: WeekNumberRepr,
}

/// The representation used for a year value.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YearRepr {
    /// The full value of the year.
    Full,
    /// Only the last two digits of the year.
    LastTwo,
}

/// Year of the date.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Year {
    /// The padding to obtain the minimum width.
    pub padding: Padding,
    /// What kind of representation should be used?
    pub repr: YearRepr,
    /// Whether the value is based on the ISO week number or the Gregorian calendar.
    pub iso_week_based: bool,
    /// Whether the `+` sign is present when a positive year contains fewer than five digits.
    pub sign_is_mandatory: bool,
}
// endregion date modifiers

// region: time modifiers
/// Hour of the day.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hour {
    /// The padding to obtain the minimum width.
    pub padding: Padding,
    /// Is the hour displayed using a 12 or 24-hour clock?
    pub is_12_hour_clock: bool,
}

/// Minute within the hour.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Minute {
    /// The padding to obtain the minimum width.
    pub padding: Padding,
}

/// AM/PM part of the time.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Period {
    /// Is the period uppercase or lowercase?
    pub is_uppercase: bool,
    /// Is the value case sensitive when parsing?
    ///
    /// Note that when `false`, the `is_uppercase` field has no effect on parsing behavior.
    pub case_sensitive: bool,
}

/// Second within the minute.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Second {
    /// The padding to obtain the minimum width.
    pub padding: Padding,
}

/// The number of digits present in a subsecond representation.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubsecondDigits {
    /// Exactly one digit.
    One,
    /// Exactly two digits.
    Two,
    /// Exactly three digits.
    Three,
    /// Exactly four digits.
    Four,
    /// Exactly five digits.
    Five,
    /// Exactly six digits.
    Six,
    /// Exactly seven digits.
    Seven,
    /// Exactly eight digits.
    Eight,
    /// Exactly nine digits.
    Nine,
    /// Any number of digits (up to nine) that is at least one. When formatting, the minimum digits
    /// necessary will be used.
    OneOrMore,
}

/// Subsecond within the second.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Subsecond {
    /// How many digits are present in the component?
    pub digits: SubsecondDigits,
}
// endregion time modifiers

// region: offset modifiers
/// Hour of the UTC offset.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OffsetHour {
    /// Whether the `+` sign is present on positive values.
    pub sign_is_mandatory: bool,
    /// The padding to obtain the minimum width.
    pub padding: Padding,
}

/// Minute within the hour of the UTC offset.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OffsetMinute {
    /// The padding to obtain the minimum width.
    pub padding: Padding,
}

/// Second within the minute of the UTC offset.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OffsetSecond {
    /// The padding to obtain the minimum width.
    pub padding: Padding,
}
// endregion offset modifiers

/// Type of padding to ensure a minimum width.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Padding {
    /// A space character (` `) should be used as padding.
    Space,
    /// A zero character (`0`) should be used as padding.
    Zero,
    /// There is no padding. This can result in a width below the otherwise minimum number of
    /// characters.
    None,
}

/// Generate the provided code if and only if `pub` is present.
macro_rules! if_pub {
    (pub $(#[$attr:meta])*; $($x:tt)*) => {
        $(#[$attr])*
        ///
        /// This function exists since [`Default::default()`] cannot be used in a `const` context.
        /// It may be removed once that becomes possible. As the [`Default`] trait is in the
        /// prelude, removing this function in the future will not cause any resolution failures for
        /// the overwhelming majority of users; only users who use `#![no_implicit_prelude]` will be
        /// affected. As such it will not be considered a breaking change.
        $($x)*
    };
    ($($_:tt)*) => {};
}

/// Implement `Default` for the given type. This also generates an inherent implementation of a
/// `default` method that is `const fn`, permitting the default value to be used in const contexts.
// Every modifier should use this macro rather than a derived `Default`.
macro_rules! impl_const_default {
    ($($(#[$doc:meta])* $(@$pub:ident)? $type:ty => $default:expr;)*) => {$(
        impl $type {
            if_pub! {
                $($pub)?
                $(#[$doc])*;
                pub const fn default() -> Self {
                    $default
                }
            }
        }

        $(#[$doc])*
        impl Default for $type {
            fn default() -> Self {
                $default
            }
        }
    )*};
}

impl_const_default! {
    /// Creates a modifier that indicates the value is [padded with zeroes](Padding::Zero).
    @pub Day => Self { padding: Padding::Zero };
    /// Creates a modifier that indicates the value uses the
    /// [`Numerical`](Self::Numerical) representation.
    MonthRepr => Self::Numerical;
    /// Creates an instance of this type that indicates the value uses the
    /// [`Numerical`](MonthRepr::Numerical) representation, is [padded with zeroes](Padding::Zero),
    /// and is case-sensitive when parsing.
    @pub Month => Self {
        padding: Padding::Zero,
        repr: MonthRepr::Numerical,
        case_sensitive: true,
    };
    /// Creates a modifier that indicates the value is [padded with zeroes](Padding::Zero).
    @pub Ordinal => Self { padding: Padding::Zero };
    /// Creates a modifier that indicates the value uses the [`Long`](Self::Long) representation.
    WeekdayRepr => Self::Long;
    /// Creates a modifier that indicates the value uses the [`Long`](WeekdayRepr::Long)
    /// representation and is case-sensitive when parsing. If the representation is changed to a
    /// numerical one, the instance defaults to one-based indexing.
    @pub Weekday => Self {
        repr: WeekdayRepr::Long,
        one_indexed: true,
        case_sensitive: true,
    };
    /// Creates a modifier that indicates that the value uses the [`Iso`](Self::Iso) representation.
    WeekNumberRepr => Self::Iso;
    /// Creates a modifier that indicates that the value is [padded with zeroes](Padding::Zero)
            /// and uses the [`Iso`](WeekNumberRepr::Iso) representation.
    @pub WeekNumber => Self {
        padding: Padding::Zero,
        repr: WeekNumberRepr::Iso,
    };
    /// Creates a modifier that indicates the value uses the [`Full`](Self::Full) representation.
    YearRepr => Self::Full;
    /// Creates a modifier that indicates the value uses the [`Full`](YearRepr::Full)
    /// representation, is [padded with zeroes](Padding::Zero), uses the Gregorian calendar as its
    /// base, and only includes the year's sign if necessary.
    @pub Year => Self {
        padding: Padding::Zero,
        repr: YearRepr::Full,
        iso_week_based: false,
        sign_is_mandatory: false,
    };
    /// Creates a modifier that indicates the value is [padded with zeroes](Padding::Zero) and
    /// has the 24-hour representation.
    @pub Hour => Self {
        padding: Padding::Zero,
        is_12_hour_clock: false,
    };
    /// Creates a modifier that indicates the value is [padded with zeroes](Padding::Zero).
    @pub Minute => Self { padding: Padding::Zero };
    /// Creates a modifier that indicates the value uses the upper-case representation and is
    /// case-sensitive when parsing.
    @pub Period => Self {
        is_uppercase: true,
        case_sensitive: true,
    };
    /// Creates a modifier that indicates the value is [padded with zeroes](Padding::Zero).
    @pub Second => Self { padding: Padding::Zero };
    /// Creates a modifier that indicates the stringified value contains [one or more
    /// digits](Self::OneOrMore).
    SubsecondDigits => Self::OneOrMore;
    /// Creates a modifier that indicates the stringified value contains [one or more
    /// digits](SubsecondDigits::OneOrMore).
    @pub Subsecond => Self { digits: SubsecondDigits::OneOrMore };
    /// Creates a modifier that indicates the value uses the `+` sign for all positive values
    /// and is [padded with zeroes](Padding::Zero).
    @pub OffsetHour => Self {
        sign_is_mandatory: true,
        padding: Padding::Zero,
    };
    /// Creates a modifier that indicates the value is [padded with zeroes](Padding::Zero).
    @pub OffsetMinute => Self { padding: Padding::Zero };
    /// Creates a modifier that indicates the value is [padded with zeroes](Padding::Zero).
    @pub OffsetSecond => Self { padding: Padding::Zero };
    /// Creates a modifier that indicates the value is [padded with zeroes](Self::Zero).
    Padding => Self::Zero;
}

/// The modifiers parsed for any given component. `None` indicates the modifier was not present.
#[cfg(feature = "alloc")]
#[allow(clippy::missing_docs_in_private_items)] // fields
#[derive(Debug, Default)]
pub(crate) struct Modifiers {
    pub(crate) padding: Option<Padding>,
    pub(crate) hour_is_12_hour_clock: Option<bool>,
    pub(crate) period_is_uppercase: Option<bool>,
    pub(crate) month_repr: Option<MonthRepr>,
    pub(crate) subsecond_digits: Option<SubsecondDigits>,
    pub(crate) weekday_repr: Option<WeekdayRepr>,
    pub(crate) weekday_is_one_indexed: Option<bool>,
    pub(crate) week_number_repr: Option<WeekNumberRepr>,
    pub(crate) year_repr: Option<YearRepr>,
    pub(crate) year_is_iso_week_based: Option<bool>,
    pub(crate) sign_is_mandatory: Option<bool>,
    pub(crate) case_sensitive: Option<bool>,
}

#[cfg(feature = "alloc")]
impl Modifiers {
    /// Parse the modifiers of a given component.
    #[allow(clippy::too_many_lines)]
    pub(crate) fn parse(
        component_name: &[u8],
        mut bytes: &[u8],
        index: &mut usize,
    ) -> Result<Self, InvalidFormatDescription> {
        let mut modifiers = Self::default();

        while !bytes.is_empty() {
            // Trim any whitespace between modifiers.
            bytes = helper::consume_whitespace(bytes, index);

            let modifier;
            if let Some(whitespace_loc) = bytes.iter().position(u8::is_ascii_whitespace) {
                *index += whitespace_loc;
                modifier = &bytes[..whitespace_loc];
                bytes = &bytes[whitespace_loc..];
            } else {
                modifier = mem::take(&mut bytes);
            }

            if modifier.is_empty() {
                break;
            }

            match (component_name, modifier) {
                (
                    b"day" | b"hour" | b"minute" | b"month" | b"offset_hour" | b"offset_minute"
                    | b"offset_second" | b"ordinal" | b"second" | b"week_number" | b"year",
                    b"padding:space",
                ) => modifiers.padding = Some(Padding::Space),
                (
                    b"day" | b"hour" | b"minute" | b"month" | b"offset_hour" | b"offset_minute"
                    | b"offset_second" | b"ordinal" | b"second" | b"week_number" | b"year",
                    b"padding:zero",
                ) => modifiers.padding = Some(Padding::Zero),
                (
                    b"day" | b"hour" | b"minute" | b"month" | b"offset_hour" | b"offset_minute"
                    | b"offset_second" | b"ordinal" | b"second" | b"week_number" | b"year",
                    b"padding:none",
                ) => modifiers.padding = Some(Padding::None),
                (b"hour", b"repr:24") => modifiers.hour_is_12_hour_clock = Some(false),
                (b"hour", b"repr:12") => modifiers.hour_is_12_hour_clock = Some(true),
                (b"month" | b"period" | b"weekday", b"case_sensitive:true") => {
                    modifiers.case_sensitive = Some(true)
                }
                (b"month" | b"period" | b"weekday", b"case_sensitive:false") => {
                    modifiers.case_sensitive = Some(false)
                }
                (b"month", b"repr:numerical") => modifiers.month_repr = Some(MonthRepr::Numerical),
                (b"month", b"repr:long") => modifiers.month_repr = Some(MonthRepr::Long),
                (b"month", b"repr:short") => modifiers.month_repr = Some(MonthRepr::Short),
                (b"offset_hour" | b"year", b"sign:automatic") => {
                    modifiers.sign_is_mandatory = Some(false);
                }
                (b"offset_hour" | b"year", b"sign:mandatory") => {
                    modifiers.sign_is_mandatory = Some(true);
                }
                (b"period", b"case:upper") => modifiers.period_is_uppercase = Some(true),
                (b"period", b"case:lower") => modifiers.period_is_uppercase = Some(false),
                (b"subsecond", b"digits:1") => {
                    modifiers.subsecond_digits = Some(SubsecondDigits::One);
                }
                (b"subsecond", b"digits:2") => {
                    modifiers.subsecond_digits = Some(SubsecondDigits::Two);
                }
                (b"subsecond", b"digits:3") => {
                    modifiers.subsecond_digits = Some(SubsecondDigits::Three);
                }
                (b"subsecond", b"digits:4") => {
                    modifiers.subsecond_digits = Some(SubsecondDigits::Four);
                }
                (b"subsecond", b"digits:5") => {
                    modifiers.subsecond_digits = Some(SubsecondDigits::Five);
                }
                (b"subsecond", b"digits:6") => {
                    modifiers.subsecond_digits = Some(SubsecondDigits::Six);
                }
                (b"subsecond", b"digits:7") => {
                    modifiers.subsecond_digits = Some(SubsecondDigits::Seven);
                }
                (b"subsecond", b"digits:8") => {
                    modifiers.subsecond_digits = Some(SubsecondDigits::Eight);
                }
                (b"subsecond", b"digits:9") => {
                    modifiers.subsecond_digits = Some(SubsecondDigits::Nine);
                }
                (b"subsecond", b"digits:1+") => {
                    modifiers.subsecond_digits = Some(SubsecondDigits::OneOrMore);
                }
                (b"weekday", b"repr:short") => modifiers.weekday_repr = Some(WeekdayRepr::Short),
                (b"weekday", b"repr:long") => modifiers.weekday_repr = Some(WeekdayRepr::Long),
                (b"weekday", b"repr:sunday") => modifiers.weekday_repr = Some(WeekdayRepr::Sunday),
                (b"weekday", b"repr:monday") => modifiers.weekday_repr = Some(WeekdayRepr::Monday),
                (b"weekday", b"one_indexed:true") => modifiers.weekday_is_one_indexed = Some(true),
                (b"weekday", b"one_indexed:false") => {
                    modifiers.weekday_is_one_indexed = Some(false);
                }
                (b"week_number", b"repr:iso") => {
                    modifiers.week_number_repr = Some(WeekNumberRepr::Iso);
                }
                (b"week_number", b"repr:sunday") => {
                    modifiers.week_number_repr = Some(WeekNumberRepr::Sunday);
                }
                (b"week_number", b"repr:monday") => {
                    modifiers.week_number_repr = Some(WeekNumberRepr::Monday);
                }
                (b"year", b"repr:full") => modifiers.year_repr = Some(YearRepr::Full),
                (b"year", b"repr:last_two") => modifiers.year_repr = Some(YearRepr::LastTwo),
                (b"year", b"base:calendar") => modifiers.year_is_iso_week_based = Some(false),
                (b"year", b"base:iso_week") => modifiers.year_is_iso_week_based = Some(true),
                _ => {
                    return Err(InvalidFormatDescription::InvalidModifier {
                        value: String::from_utf8_lossy(modifier).into_owned(),
                        index: *index,
                    });
                }
            }
        }

        Ok(modifiers)
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4972() {
    rusty_monitor::set_test_id(4972);
    let mut i32_0: i32 = -131i32;
    let mut i128_0: i128 = -154i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_0);
    let mut i32_1: i32 = 2i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut i64_0: i64 = 20i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut i64_1: i64 = 47i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut u32_0: u32 = 25u32;
    let mut u8_0: u8 = 74u8;
    let mut u8_1: u8 = 94u8;
    let mut u8_2: u8 = 87u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_2: i64 = -141i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut i8_0: i8 = -100i8;
    let mut i8_1: i8 = -92i8;
    let mut i8_2: i8 = 5i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_6: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_7: std::time::Duration = crate::duration::Duration::abs_std(duration_6);
    let mut i64_3: i64 = -51i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::minutes(i64_3);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::abs(duration_8);
    let mut i8_3: i8 = -114i8;
    let mut i8_4: i8 = -112i8;
    let mut i8_5: i8 = 58i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i128_1: i128 = -55i128;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_1);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::previous(weekday_0);
    let mut u8_3: u8 = crate::weekday::Weekday::number_days_from_monday(weekday_1);
    let mut u8_4: u8 = crate::offset_date_time::OffsetDateTime::minute(offsetdatetime_1);
    panic!("From RustyUnit with love");
}
}