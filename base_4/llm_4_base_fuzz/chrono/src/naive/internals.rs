//! The internal implementation of the calendar and ordinal date.
//!
//! The current implementation is optimized for determining year, month, day and day of week.
//! 4-bit `YearFlags` map to one of 14 possible classes of year in the Gregorian calendar,
//! which are included in every packed `NaiveDate` instance.
//! The conversion between the packed calendar date (`Mdf`) and the ordinal date (`Of`) is
//! based on the moderately-sized lookup table (~1.5KB)
//! and the packed representation is chosen for the efficient lookup.
//! Every internal data structure does not validate its input,
//! but the conversion keeps the valid value valid and the invalid value invalid
//! so that the user-facing `NaiveDate` can validate the input as late as possible.
#![cfg_attr(feature = "__internal_bench", allow(missing_docs))]
use crate::Weekday;
use core::convert::TryFrom;
use core::{fmt, i32};
/// The internal date representation. This also includes the packed `Mdf` value.
pub(super) type DateImpl = i32;
pub(super) const MAX_YEAR: DateImpl = i32::MAX >> 13;
pub(super) const MIN_YEAR: DateImpl = i32::MIN >> 13;
/// The year flags (aka the dominical letter).
///
/// There are 14 possible classes of year in the Gregorian calendar:
/// common and leap years starting with Monday through Sunday.
/// The `YearFlags` stores this information into 4 bits `abbb`,
/// where `a` is `1` for the common year (simplifies the `Of` validation)
/// and `bbb` is a non-zero `Weekday` (mapping `Mon` to 7) of the last day in the past year
/// (simplifies the day of week calculation from the 1-based ordinal).
#[allow(unreachable_pub)]
#[derive(PartialEq, Eq, Copy, Clone, Hash)]
pub struct YearFlags(pub(super) u8);
pub(super) const A: YearFlags = YearFlags(0o15);
pub(super) const AG: YearFlags = YearFlags(0o05);
pub(super) const B: YearFlags = YearFlags(0o14);
pub(super) const BA: YearFlags = YearFlags(0o04);
pub(super) const C: YearFlags = YearFlags(0o13);
pub(super) const CB: YearFlags = YearFlags(0o03);
pub(super) const D: YearFlags = YearFlags(0o12);
pub(super) const DC: YearFlags = YearFlags(0o02);
pub(super) const E: YearFlags = YearFlags(0o11);
pub(super) const ED: YearFlags = YearFlags(0o01);
pub(super) const F: YearFlags = YearFlags(0o17);
pub(super) const FE: YearFlags = YearFlags(0o07);
pub(super) const G: YearFlags = YearFlags(0o16);
pub(super) const GF: YearFlags = YearFlags(0o06);
static YEAR_TO_FLAGS: [YearFlags; 400] = [
    BA,
    G,
    F,
    E,
    DC,
    B,
    A,
    G,
    FE,
    D,
    C,
    B,
    AG,
    F,
    E,
    D,
    CB,
    A,
    G,
    F,
    ED,
    C,
    B,
    A,
    GF,
    E,
    D,
    C,
    BA,
    G,
    F,
    E,
    DC,
    B,
    A,
    G,
    FE,
    D,
    C,
    B,
    AG,
    F,
    E,
    D,
    CB,
    A,
    G,
    F,
    ED,
    C,
    B,
    A,
    GF,
    E,
    D,
    C,
    BA,
    G,
    F,
    E,
    DC,
    B,
    A,
    G,
    FE,
    D,
    C,
    B,
    AG,
    F,
    E,
    D,
    CB,
    A,
    G,
    F,
    ED,
    C,
    B,
    A,
    GF,
    E,
    D,
    C,
    BA,
    G,
    F,
    E,
    DC,
    B,
    A,
    G,
    FE,
    D,
    C,
    B,
    AG,
    F,
    E,
    D,
    C,
    B,
    A,
    G,
    FE,
    D,
    C,
    B,
    AG,
    F,
    E,
    D,
    CB,
    A,
    G,
    F,
    ED,
    C,
    B,
    A,
    GF,
    E,
    D,
    C,
    BA,
    G,
    F,
    E,
    DC,
    B,
    A,
    G,
    FE,
    D,
    C,
    B,
    AG,
    F,
    E,
    D,
    CB,
    A,
    G,
    F,
    ED,
    C,
    B,
    A,
    GF,
    E,
    D,
    C,
    BA,
    G,
    F,
    E,
    DC,
    B,
    A,
    G,
    FE,
    D,
    C,
    B,
    AG,
    F,
    E,
    D,
    CB,
    A,
    G,
    F,
    ED,
    C,
    B,
    A,
    GF,
    E,
    D,
    C,
    BA,
    G,
    F,
    E,
    DC,
    B,
    A,
    G,
    FE,
    D,
    C,
    B,
    AG,
    F,
    E,
    D,
    CB,
    A,
    G,
    F,
    E,
    D,
    C,
    B,
    AG,
    F,
    E,
    D,
    CB,
    A,
    G,
    F,
    ED,
    C,
    B,
    A,
    GF,
    E,
    D,
    C,
    BA,
    G,
    F,
    E,
    DC,
    B,
    A,
    G,
    FE,
    D,
    C,
    B,
    AG,
    F,
    E,
    D,
    CB,
    A,
    G,
    F,
    ED,
    C,
    B,
    A,
    GF,
    E,
    D,
    C,
    BA,
    G,
    F,
    E,
    DC,
    B,
    A,
    G,
    FE,
    D,
    C,
    B,
    AG,
    F,
    E,
    D,
    CB,
    A,
    G,
    F,
    ED,
    C,
    B,
    A,
    GF,
    E,
    D,
    C,
    BA,
    G,
    F,
    E,
    DC,
    B,
    A,
    G,
    FE,
    D,
    C,
    B,
    AG,
    F,
    E,
    D,
    CB,
    A,
    G,
    F,
    ED,
    C,
    B,
    A,
    G,
    F,
    E,
    D,
    CB,
    A,
    G,
    F,
    ED,
    C,
    B,
    A,
    GF,
    E,
    D,
    C,
    BA,
    G,
    F,
    E,
    DC,
    B,
    A,
    G,
    FE,
    D,
    C,
    B,
    AG,
    F,
    E,
    D,
    CB,
    A,
    G,
    F,
    ED,
    C,
    B,
    A,
    GF,
    E,
    D,
    C,
    BA,
    G,
    F,
    E,
    DC,
    B,
    A,
    G,
    FE,
    D,
    C,
    B,
    AG,
    F,
    E,
    D,
    CB,
    A,
    G,
    F,
    ED,
    C,
    B,
    A,
    GF,
    E,
    D,
    C,
    BA,
    G,
    F,
    E,
    DC,
    B,
    A,
    G,
    FE,
    D,
    C,
    B,
    AG,
    F,
    E,
    D,
    CB,
    A,
    G,
    F,
    ED,
    C,
    B,
    A,
    GF,
    E,
    D,
    C,
];
static YEAR_DELTAS: [u8; 401] = [
    0,
    1,
    1,
    1,
    1,
    2,
    2,
    2,
    2,
    3,
    3,
    3,
    3,
    4,
    4,
    4,
    4,
    5,
    5,
    5,
    5,
    6,
    6,
    6,
    6,
    7,
    7,
    7,
    7,
    8,
    8,
    8,
    8,
    9,
    9,
    9,
    9,
    10,
    10,
    10,
    10,
    11,
    11,
    11,
    11,
    12,
    12,
    12,
    12,
    13,
    13,
    13,
    13,
    14,
    14,
    14,
    14,
    15,
    15,
    15,
    15,
    16,
    16,
    16,
    16,
    17,
    17,
    17,
    17,
    18,
    18,
    18,
    18,
    19,
    19,
    19,
    19,
    20,
    20,
    20,
    20,
    21,
    21,
    21,
    21,
    22,
    22,
    22,
    22,
    23,
    23,
    23,
    23,
    24,
    24,
    24,
    24,
    25,
    25,
    25,
    25,
    25,
    25,
    25,
    25,
    26,
    26,
    26,
    26,
    27,
    27,
    27,
    27,
    28,
    28,
    28,
    28,
    29,
    29,
    29,
    29,
    30,
    30,
    30,
    30,
    31,
    31,
    31,
    31,
    32,
    32,
    32,
    32,
    33,
    33,
    33,
    33,
    34,
    34,
    34,
    34,
    35,
    35,
    35,
    35,
    36,
    36,
    36,
    36,
    37,
    37,
    37,
    37,
    38,
    38,
    38,
    38,
    39,
    39,
    39,
    39,
    40,
    40,
    40,
    40,
    41,
    41,
    41,
    41,
    42,
    42,
    42,
    42,
    43,
    43,
    43,
    43,
    44,
    44,
    44,
    44,
    45,
    45,
    45,
    45,
    46,
    46,
    46,
    46,
    47,
    47,
    47,
    47,
    48,
    48,
    48,
    48,
    49,
    49,
    49,
    49,
    49,
    49,
    49,
    49,
    50,
    50,
    50,
    50,
    51,
    51,
    51,
    51,
    52,
    52,
    52,
    52,
    53,
    53,
    53,
    53,
    54,
    54,
    54,
    54,
    55,
    55,
    55,
    55,
    56,
    56,
    56,
    56,
    57,
    57,
    57,
    57,
    58,
    58,
    58,
    58,
    59,
    59,
    59,
    59,
    60,
    60,
    60,
    60,
    61,
    61,
    61,
    61,
    62,
    62,
    62,
    62,
    63,
    63,
    63,
    63,
    64,
    64,
    64,
    64,
    65,
    65,
    65,
    65,
    66,
    66,
    66,
    66,
    67,
    67,
    67,
    67,
    68,
    68,
    68,
    68,
    69,
    69,
    69,
    69,
    70,
    70,
    70,
    70,
    71,
    71,
    71,
    71,
    72,
    72,
    72,
    72,
    73,
    73,
    73,
    73,
    73,
    73,
    73,
    73,
    74,
    74,
    74,
    74,
    75,
    75,
    75,
    75,
    76,
    76,
    76,
    76,
    77,
    77,
    77,
    77,
    78,
    78,
    78,
    78,
    79,
    79,
    79,
    79,
    80,
    80,
    80,
    80,
    81,
    81,
    81,
    81,
    82,
    82,
    82,
    82,
    83,
    83,
    83,
    83,
    84,
    84,
    84,
    84,
    85,
    85,
    85,
    85,
    86,
    86,
    86,
    86,
    87,
    87,
    87,
    87,
    88,
    88,
    88,
    88,
    89,
    89,
    89,
    89,
    90,
    90,
    90,
    90,
    91,
    91,
    91,
    91,
    92,
    92,
    92,
    92,
    93,
    93,
    93,
    93,
    94,
    94,
    94,
    94,
    95,
    95,
    95,
    95,
    96,
    96,
    96,
    96,
    97,
    97,
    97,
    97,
];
pub(super) fn cycle_to_yo(cycle: u32) -> (u32, u32) {
    let mut year_mod_400 = cycle / 365;
    let mut ordinal0 = cycle % 365;
    let delta = u32::from(YEAR_DELTAS[year_mod_400 as usize]);
    if ordinal0 < delta {
        year_mod_400 -= 1;
        ordinal0 += 365 - u32::from(YEAR_DELTAS[year_mod_400 as usize]);
    } else {
        ordinal0 -= delta;
    }
    (year_mod_400, ordinal0 + 1)
}
pub(super) fn yo_to_cycle(year_mod_400: u32, ordinal: u32) -> u32 {
    year_mod_400 * 365 + u32::from(YEAR_DELTAS[year_mod_400 as usize]) + ordinal - 1
}
impl YearFlags {
    #[allow(unreachable_pub)]
    #[doc(hidden)]
    #[inline]
    #[must_use]
    pub fn from_year(year: i32) -> YearFlags {
        let year = year.rem_euclid(400);
        YearFlags::from_year_mod_400(year)
    }
    #[inline]
    pub(super) fn from_year_mod_400(year: i32) -> YearFlags {
        YEAR_TO_FLAGS[year as usize]
    }
    #[inline]
    pub(super) fn ndays(&self) -> u32 {
        let YearFlags(flags) = *self;
        366 - u32::from(flags >> 3)
    }
    #[inline]
    pub(super) fn isoweek_delta(&self) -> u32 {
        let YearFlags(flags) = *self;
        let mut delta = u32::from(flags) & 0b0111;
        if delta < 3 {
            delta += 7;
        }
        delta
    }
    #[inline]
    pub(super) const fn nisoweeks(&self) -> u32 {
        let YearFlags(flags) = *self;
        52 + ((0b0000_0100_0000_0110 >> flags as usize) & 1)
    }
}
impl fmt::Debug for YearFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let YearFlags(flags) = *self;
        match flags {
            0o15 => "A".fmt(f),
            0o05 => "AG".fmt(f),
            0o14 => "B".fmt(f),
            0o04 => "BA".fmt(f),
            0o13 => "C".fmt(f),
            0o03 => "CB".fmt(f),
            0o12 => "D".fmt(f),
            0o02 => "DC".fmt(f),
            0o11 => "E".fmt(f),
            0o01 => "ED".fmt(f),
            0o10 => "F?".fmt(f),
            0o00 => "FE?".fmt(f),
            0o17 => "F".fmt(f),
            0o07 => "FE".fmt(f),
            0o16 => "G".fmt(f),
            0o06 => "GF".fmt(f),
            _ => write!(f, "YearFlags({})", flags),
        }
    }
}
pub(super) const MIN_OL: u32 = 1 << 1;
pub(super) const MAX_OL: u32 = 366 << 1;
pub(super) const MAX_MDL: u32 = (12 << 6) | (31 << 1) | 1;
const XX: i8 = -128;
static MDL_TO_OL: [i8; MAX_MDL as usize + 1] = [
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    XX,
    XX,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    XX,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    XX,
    XX,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    XX,
    XX,
    XX,
    XX,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    XX,
    XX,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    XX,
    XX,
    XX,
    XX,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    XX,
    XX,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    XX,
    XX,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    XX,
    XX,
    XX,
    XX,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    XX,
    XX,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    XX,
    XX,
    XX,
    XX,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
];
static OL_TO_MDL: [u8; MAX_OL as usize + 1] = [
    0,
    0,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    64,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    66,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    74,
    72,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    76,
    74,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    80,
    78,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    82,
    80,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    86,
    84,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    88,
    86,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    90,
    88,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    94,
    92,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    96,
    94,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
    100,
    98,
];
/// Ordinal (day of year) and year flags: `(ordinal << 4) | flags`.
///
/// The whole bits except for the least 3 bits are referred as `Ol` (ordinal and leap flag),
/// which is an index to the `OL_TO_MDL` lookup table.
#[derive(PartialEq, PartialOrd, Copy, Clone)]
pub(super) struct Of(pub(crate) u32);
impl Of {
    #[inline]
    pub(super) fn new(ordinal: u32, YearFlags(flags): YearFlags) -> Option<Of> {
        match ordinal <= 366 {
            true => Some(Of((ordinal << 4) | u32::from(flags))),
            false => None,
        }
    }
    #[inline]
    pub(super) fn from_mdf(Mdf(mdf): Mdf) -> Of {
        let mdl = mdf >> 3;
        match MDL_TO_OL.get(mdl as usize) {
            Some(&v) => Of(mdf.wrapping_sub((i32::from(v) as u32 & 0x3ff) << 3)),
            None => Of(0),
        }
    }
    #[inline]
    pub(super) fn valid(&self) -> bool {
        let Of(of) = *self;
        let ol = of >> 3;
        (MIN_OL..=MAX_OL).contains(&ol)
    }
    #[inline]
    pub(super) const fn ordinal(&self) -> u32 {
        let Of(of) = *self;
        of >> 4
    }
    #[inline]
    pub(super) const fn with_ordinal(&self, ordinal: u32) -> Option<Of> {
        if ordinal > 366 {
            return None;
        }
        let Of(of) = *self;
        Some(Of((of & 0b1111) | (ordinal << 4)))
    }
    #[inline]
    pub(super) const fn flags(&self) -> YearFlags {
        let Of(of) = *self;
        YearFlags((of & 0b1111) as u8)
    }
    #[inline]
    pub(super) fn weekday(&self) -> Weekday {
        let Of(of) = *self;
        Weekday::try_from((((of >> 4) + (of & 0b111)) % 7) as u8).unwrap()
    }
    #[inline]
    pub(super) fn isoweekdate_raw(&self) -> (u32, Weekday) {
        let Of(of) = *self;
        let weekord = (of >> 4).wrapping_add(self.flags().isoweek_delta());
        (weekord / 7, Weekday::try_from((weekord % 7) as u8).unwrap())
    }
    #[cfg_attr(feature = "cargo-clippy", allow(clippy::wrong_self_convention))]
    #[inline]
    pub(super) fn to_mdf(&self) -> Mdf {
        Mdf::from_of(*self)
    }
    #[inline]
    pub(super) const fn succ(&self) -> Of {
        let Of(of) = *self;
        Of(of + (1 << 4))
    }
    #[inline]
    pub(super) const fn pred(&self) -> Of {
        let Of(of) = *self;
        Of(of - (1 << 4))
    }
}
impl fmt::Debug for Of {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Of(of) = *self;
        write!(
            f, "Of(({} << 4) | {:#04o} /*{:?}*/)", of >> 4, of & 0b1111, YearFlags((of &
            0b1111) as u8)
        )
    }
}
/// Month, day of month and year flags: `(month << 9) | (day << 4) | flags`
///
/// The whole bits except for the least 3 bits are referred as `Mdl`
/// (month, day of month and leap flag),
/// which is an index to the `MDL_TO_OL` lookup table.
#[derive(PartialEq, PartialOrd, Copy, Clone)]
pub(super) struct Mdf(pub(super) u32);
impl Mdf {
    #[inline]
    pub(super) fn new(month: u32, day: u32, YearFlags(flags): YearFlags) -> Option<Mdf> {
        match month <= 12 && day <= 31 {
            true => Some(Mdf((month << 9) | (day << 4) | u32::from(flags))),
            false => None,
        }
    }
    #[inline]
    pub(super) fn from_of(Of(of): Of) -> Mdf {
        let ol = of >> 3;
        match OL_TO_MDL.get(ol as usize) {
            Some(&v) => Mdf(of + (u32::from(v) << 3)),
            None => Mdf(0),
        }
    }
    #[cfg(test)]
    pub(super) fn valid(&self) -> bool {
        let Mdf(mdf) = *self;
        let mdl = mdf >> 3;
        match MDL_TO_OL.get(mdl as usize) {
            Some(&v) => v >= 0,
            None => false,
        }
    }
    #[inline]
    pub(super) const fn month(&self) -> u32 {
        let Mdf(mdf) = *self;
        mdf >> 9
    }
    #[inline]
    pub(super) const fn with_month(&self, month: u32) -> Option<Mdf> {
        if month > 12 {
            return None;
        }
        let Mdf(mdf) = *self;
        Some(Mdf((mdf & 0b1_1111_1111) | (month << 9)))
    }
    #[inline]
    pub(super) const fn day(&self) -> u32 {
        let Mdf(mdf) = *self;
        (mdf >> 4) & 0b1_1111
    }
    #[inline]
    pub(super) const fn with_day(&self, day: u32) -> Option<Mdf> {
        if day > 31 {
            return None;
        }
        let Mdf(mdf) = *self;
        Some(Mdf((mdf & !0b1_1111_0000) | (day << 4)))
    }
    #[inline]
    pub(super) fn with_flags(&self, YearFlags(flags): YearFlags) -> Mdf {
        let Mdf(mdf) = *self;
        Mdf((mdf & !0b1111) | u32::from(flags))
    }
    #[cfg_attr(feature = "cargo-clippy", allow(clippy::wrong_self_convention))]
    #[inline]
    pub(super) fn to_of(&self) -> Of {
        Of::from_mdf(*self)
    }
}
impl fmt::Debug for Mdf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Mdf(mdf) = *self;
        write!(
            f, "Mdf(({} << 9) | ({} << 4) | {:#04o} /*{:?}*/)", mdf >> 9, (mdf >> 4) &
            0b1_1111, mdf & 0b1111, YearFlags((mdf & 0b1111) as u8)
        )
    }
}
#[cfg(test)]
mod tests {
    use num_iter::range_inclusive;
    use std::u32;
    use super::{Mdf, Of};
    use super::{YearFlags, A, AG, B, BA, C, CB, D, DC, E, ED, F, FE, G, GF};
    use crate::Weekday;
    const NONLEAP_FLAGS: [YearFlags; 7] = [A, B, C, D, E, F, G];
    const LEAP_FLAGS: [YearFlags; 7] = [AG, BA, CB, DC, ED, FE, GF];
    const FLAGS: [YearFlags; 14] = [A, B, C, D, E, F, G, AG, BA, CB, DC, ED, FE, GF];
    #[test]
    fn test_year_flags_ndays_from_year() {
        assert_eq!(YearFlags::from_year(2014).ndays(), 365);
        assert_eq!(YearFlags::from_year(2012).ndays(), 366);
        assert_eq!(YearFlags::from_year(2000).ndays(), 366);
        assert_eq!(YearFlags::from_year(1900).ndays(), 365);
        assert_eq!(YearFlags::from_year(1600).ndays(), 366);
        assert_eq!(YearFlags::from_year(1).ndays(), 365);
        assert_eq!(YearFlags::from_year(0).ndays(), 366);
        assert_eq!(YearFlags::from_year(- 1).ndays(), 365);
        assert_eq!(YearFlags::from_year(- 4).ndays(), 366);
        assert_eq!(YearFlags::from_year(- 99).ndays(), 365);
        assert_eq!(YearFlags::from_year(- 100).ndays(), 365);
        assert_eq!(YearFlags::from_year(- 399).ndays(), 365);
        assert_eq!(YearFlags::from_year(- 400).ndays(), 366);
    }
    #[test]
    fn test_year_flags_nisoweeks() {
        assert_eq!(A.nisoweeks(), 52);
        assert_eq!(B.nisoweeks(), 52);
        assert_eq!(C.nisoweeks(), 52);
        assert_eq!(D.nisoweeks(), 53);
        assert_eq!(E.nisoweeks(), 52);
        assert_eq!(F.nisoweeks(), 52);
        assert_eq!(G.nisoweeks(), 52);
        assert_eq!(AG.nisoweeks(), 52);
        assert_eq!(BA.nisoweeks(), 52);
        assert_eq!(CB.nisoweeks(), 52);
        assert_eq!(DC.nisoweeks(), 53);
        assert_eq!(ED.nisoweeks(), 53);
        assert_eq!(FE.nisoweeks(), 52);
        assert_eq!(GF.nisoweeks(), 52);
    }
    #[test]
    fn test_of() {
        fn check(expected: bool, flags: YearFlags, ordinal1: u32, ordinal2: u32) {
            for ordinal in range_inclusive(ordinal1, ordinal2) {
                let of = match Of::new(ordinal, flags) {
                    Some(of) => of,
                    None if !expected => continue,
                    None => panic!("Of::new({}, {:?}) returned None", ordinal, flags),
                };
                assert!(
                    of.valid() == expected,
                    "ordinal {} = {:?} should be {} for dominical year {:?}", ordinal,
                    of, if expected { "valid" } else { "invalid" }, flags
                );
            }
        }
        for &flags in NONLEAP_FLAGS.iter() {
            check(false, flags, 0, 0);
            check(true, flags, 1, 365);
            check(false, flags, 366, 1024);
            check(false, flags, u32::MAX, u32::MAX);
        }
        for &flags in LEAP_FLAGS.iter() {
            check(false, flags, 0, 0);
            check(true, flags, 1, 366);
            check(false, flags, 367, 1024);
            check(false, flags, u32::MAX, u32::MAX);
        }
    }
    #[test]
    fn test_mdf_valid() {
        fn check(
            expected: bool,
            flags: YearFlags,
            month1: u32,
            day1: u32,
            month2: u32,
            day2: u32,
        ) {
            for month in range_inclusive(month1, month2) {
                for day in range_inclusive(day1, day2) {
                    let mdf = match Mdf::new(month, day, flags) {
                        Some(mdf) => mdf,
                        None if !expected => continue,
                        None => {
                            panic!(
                                "Mdf::new({}, {}, {:?}) returned None", month, day, flags
                            )
                        }
                    };
                    assert!(
                        mdf.valid() == expected,
                        "month {} day {} = {:?} should be {} for dominical year {:?}",
                        month, day, mdf, if expected { "valid" } else { "invalid" },
                        flags
                    );
                }
            }
        }
        for &flags in NONLEAP_FLAGS.iter() {
            check(false, flags, 0, 0, 0, 1024);
            check(false, flags, 0, 0, 16, 0);
            check(true, flags, 1, 1, 1, 31);
            check(false, flags, 1, 32, 1, 1024);
            check(true, flags, 2, 1, 2, 28);
            check(false, flags, 2, 29, 2, 1024);
            check(true, flags, 3, 1, 3, 31);
            check(false, flags, 3, 32, 3, 1024);
            check(true, flags, 4, 1, 4, 30);
            check(false, flags, 4, 31, 4, 1024);
            check(true, flags, 5, 1, 5, 31);
            check(false, flags, 5, 32, 5, 1024);
            check(true, flags, 6, 1, 6, 30);
            check(false, flags, 6, 31, 6, 1024);
            check(true, flags, 7, 1, 7, 31);
            check(false, flags, 7, 32, 7, 1024);
            check(true, flags, 8, 1, 8, 31);
            check(false, flags, 8, 32, 8, 1024);
            check(true, flags, 9, 1, 9, 30);
            check(false, flags, 9, 31, 9, 1024);
            check(true, flags, 10, 1, 10, 31);
            check(false, flags, 10, 32, 10, 1024);
            check(true, flags, 11, 1, 11, 30);
            check(false, flags, 11, 31, 11, 1024);
            check(true, flags, 12, 1, 12, 31);
            check(false, flags, 12, 32, 12, 1024);
            check(false, flags, 13, 0, 16, 1024);
            check(false, flags, u32::MAX, 0, u32::MAX, 1024);
            check(false, flags, 0, u32::MAX, 16, u32::MAX);
            check(false, flags, u32::MAX, u32::MAX, u32::MAX, u32::MAX);
        }
        for &flags in LEAP_FLAGS.iter() {
            check(false, flags, 0, 0, 0, 1024);
            check(false, flags, 0, 0, 16, 0);
            check(true, flags, 1, 1, 1, 31);
            check(false, flags, 1, 32, 1, 1024);
            check(true, flags, 2, 1, 2, 29);
            check(false, flags, 2, 30, 2, 1024);
            check(true, flags, 3, 1, 3, 31);
            check(false, flags, 3, 32, 3, 1024);
            check(true, flags, 4, 1, 4, 30);
            check(false, flags, 4, 31, 4, 1024);
            check(true, flags, 5, 1, 5, 31);
            check(false, flags, 5, 32, 5, 1024);
            check(true, flags, 6, 1, 6, 30);
            check(false, flags, 6, 31, 6, 1024);
            check(true, flags, 7, 1, 7, 31);
            check(false, flags, 7, 32, 7, 1024);
            check(true, flags, 8, 1, 8, 31);
            check(false, flags, 8, 32, 8, 1024);
            check(true, flags, 9, 1, 9, 30);
            check(false, flags, 9, 31, 9, 1024);
            check(true, flags, 10, 1, 10, 31);
            check(false, flags, 10, 32, 10, 1024);
            check(true, flags, 11, 1, 11, 30);
            check(false, flags, 11, 31, 11, 1024);
            check(true, flags, 12, 1, 12, 31);
            check(false, flags, 12, 32, 12, 1024);
            check(false, flags, 13, 0, 16, 1024);
            check(false, flags, u32::MAX, 0, u32::MAX, 1024);
            check(false, flags, 0, u32::MAX, 16, u32::MAX);
            check(false, flags, u32::MAX, u32::MAX, u32::MAX, u32::MAX);
        }
    }
    #[test]
    fn test_of_fields() {
        for &flags in FLAGS.iter() {
            for ordinal in range_inclusive(1u32, 366) {
                let of = Of::new(ordinal, flags).unwrap();
                if of.valid() {
                    assert_eq!(of.ordinal(), ordinal);
                }
            }
        }
    }
    #[test]
    fn test_of_with_fields() {
        fn check(flags: YearFlags, ordinal: u32) {
            let of = Of::new(ordinal, flags).unwrap();
            for ordinal in range_inclusive(0u32, 1024) {
                let of = match of.with_ordinal(ordinal) {
                    Some(of) => of,
                    None if ordinal > 366 => continue,
                    None => panic!("failed to create Of with ordinal {}", ordinal),
                };
                assert_eq!(of.valid(), Of::new(ordinal, flags).unwrap().valid());
                if of.valid() {
                    assert_eq!(of.ordinal(), ordinal);
                }
            }
        }
        for &flags in NONLEAP_FLAGS.iter() {
            check(flags, 1);
            check(flags, 365);
        }
        for &flags in LEAP_FLAGS.iter() {
            check(flags, 1);
            check(flags, 366);
        }
    }
    #[test]
    fn test_of_weekday() {
        assert_eq!(Of::new(1, A).unwrap().weekday(), Weekday::Sun);
        assert_eq!(Of::new(1, B).unwrap().weekday(), Weekday::Sat);
        assert_eq!(Of::new(1, C).unwrap().weekday(), Weekday::Fri);
        assert_eq!(Of::new(1, D).unwrap().weekday(), Weekday::Thu);
        assert_eq!(Of::new(1, E).unwrap().weekday(), Weekday::Wed);
        assert_eq!(Of::new(1, F).unwrap().weekday(), Weekday::Tue);
        assert_eq!(Of::new(1, G).unwrap().weekday(), Weekday::Mon);
        assert_eq!(Of::new(1, AG).unwrap().weekday(), Weekday::Sun);
        assert_eq!(Of::new(1, BA).unwrap().weekday(), Weekday::Sat);
        assert_eq!(Of::new(1, CB).unwrap().weekday(), Weekday::Fri);
        assert_eq!(Of::new(1, DC).unwrap().weekday(), Weekday::Thu);
        assert_eq!(Of::new(1, ED).unwrap().weekday(), Weekday::Wed);
        assert_eq!(Of::new(1, FE).unwrap().weekday(), Weekday::Tue);
        assert_eq!(Of::new(1, GF).unwrap().weekday(), Weekday::Mon);
        for &flags in FLAGS.iter() {
            let mut prev = Of::new(1, flags).unwrap().weekday();
            for ordinal in range_inclusive(2u32, flags.ndays()) {
                let of = Of::new(ordinal, flags).unwrap();
                let expected = prev.succ();
                assert_eq!(of.weekday(), expected);
                prev = expected;
            }
        }
    }
    #[test]
    fn test_mdf_fields() {
        for &flags in FLAGS.iter() {
            for month in range_inclusive(1u32, 12) {
                for day in range_inclusive(1u32, 31) {
                    let mdf = match Mdf::new(month, day, flags) {
                        Some(mdf) => mdf,
                        None => continue,
                    };
                    if mdf.valid() {
                        assert_eq!(mdf.month(), month);
                        assert_eq!(mdf.day(), day);
                    }
                }
            }
        }
    }
    #[test]
    fn test_mdf_with_fields() {
        fn check(flags: YearFlags, month: u32, day: u32) {
            let mdf = Mdf::new(month, day, flags).unwrap();
            for month in range_inclusive(0u32, 16) {
                let mdf = match mdf.with_month(month) {
                    Some(mdf) => mdf,
                    None if month > 12 => continue,
                    None => panic!("failed to create Mdf with month {}", month),
                };
                if mdf.valid() {
                    assert_eq!(mdf.month(), month);
                    assert_eq!(mdf.day(), day);
                }
            }
            for day in range_inclusive(0u32, 1024) {
                let mdf = match mdf.with_day(day) {
                    Some(mdf) => mdf,
                    None if day > 31 => continue,
                    None => panic!("failed to create Mdf with month {}", month),
                };
                if mdf.valid() {
                    assert_eq!(mdf.month(), month);
                    assert_eq!(mdf.day(), day);
                }
            }
        }
        for &flags in NONLEAP_FLAGS.iter() {
            check(flags, 1, 1);
            check(flags, 1, 31);
            check(flags, 2, 1);
            check(flags, 2, 28);
            check(flags, 2, 29);
            check(flags, 12, 31);
        }
        for &flags in LEAP_FLAGS.iter() {
            check(flags, 1, 1);
            check(flags, 1, 31);
            check(flags, 2, 1);
            check(flags, 2, 29);
            check(flags, 2, 30);
            check(flags, 12, 31);
        }
    }
    #[test]
    fn test_of_isoweekdate_raw() {
        for &flags in FLAGS.iter() {
            let (week, _) = Of::new(4, flags).unwrap().isoweekdate_raw();
            assert_eq!(week, 1);
        }
    }
    #[test]
    fn test_of_to_mdf() {
        for i in range_inclusive(0u32, 8192) {
            let of = Of(i);
            assert_eq!(of.valid(), of.to_mdf().valid());
        }
    }
    #[test]
    fn test_mdf_to_of() {
        for i in range_inclusive(0u32, 8192) {
            let mdf = Mdf(i);
            assert_eq!(mdf.valid(), mdf.to_of().valid());
        }
    }
    #[test]
    fn test_of_to_mdf_to_of() {
        for i in range_inclusive(0u32, 8192) {
            let of = Of(i);
            if of.valid() {
                assert_eq!(of, of.to_mdf().to_of());
            }
        }
    }
    #[test]
    fn test_mdf_to_of_to_mdf() {
        for i in range_inclusive(0u32, 8192) {
            let mdf = Mdf(i);
            if mdf.valid() {
                assert_eq!(mdf, mdf.to_of().to_mdf());
            }
        }
    }
}
#[cfg(test)]
mod tests_llm_16_433 {
    use super::*;
    use crate::*;
    #[test]
    fn test_month_day_within_bounds() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u32, u32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let month = rug_fuzz_0;
        let day = rug_fuzz_1;
        let year = rug_fuzz_2;
        let year_flags = naive::internals::YearFlags::from_year(year);
        debug_assert!(naive::internals::Mdf::new(month, day, year_flags).is_some());
             }
});    }
    #[test]
    fn test_month_day_out_of_bounds() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u32, u32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let month = rug_fuzz_0;
        let day = rug_fuzz_1;
        let year = rug_fuzz_2;
        let year_flags = naive::internals::YearFlags::from_year(year);
        debug_assert!(naive::internals::Mdf::new(month, day, year_flags).is_none());
             }
});    }
    #[test]
    fn test_day_out_of_bounds() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u32, u32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let month = rug_fuzz_0;
        let day = rug_fuzz_1;
        let year = rug_fuzz_2;
        let year_flags = naive::internals::YearFlags::from_year(year);
        debug_assert!(naive::internals::Mdf::new(month, day, year_flags).is_none());
             }
});    }
    #[test]
    fn test_month_and_day_out_of_bounds() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u32, u32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let month = rug_fuzz_0;
        let day = rug_fuzz_1;
        let year = rug_fuzz_2;
        let year_flags = naive::internals::YearFlags::from_year(year);
        debug_assert!(naive::internals::Mdf::new(month, day, year_flags).is_none());
             }
});    }
    #[test]
    fn test_month_day_on_bounds() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u32, u32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let month = rug_fuzz_0;
        let day = rug_fuzz_1;
        let year = rug_fuzz_2;
        let year_flags = naive::internals::YearFlags::from_year(year);
        debug_assert!(naive::internals::Mdf::new(month, day, year_flags).is_some());
             }
});    }
    #[test]
    fn test_month_day_and_year_flags() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u32, u32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let month = rug_fuzz_0;
        let day = rug_fuzz_1;
        let year = rug_fuzz_2;
        let year_flags = naive::internals::YearFlags::from_year(year);
        debug_assert!(naive::internals::Mdf::new(month, day, year_flags).is_some());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_439 {
    use super::*;
    use crate::*;
    #[test]
    fn from_mdf_returns_expected_of() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u32, u32, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mdf_values = vec![
            Mdf::new(rug_fuzz_0, rug_fuzz_1, YearFlags(rug_fuzz_2)).unwrap(),
            Mdf::new(12, 31, YearFlags(0)).unwrap(), Mdf::new(6, 15, YearFlags(0))
            .unwrap()
        ];
        for mdf in mdf_values {
            let of = Of::from_mdf(mdf);
            debug_assert!(of.valid(), "Resulting Of should be valid");
        }
             }
});    }
    #[test]
    fn from_mdf_with_invalid_mdf_returns_of_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mdf = Mdf(rug_fuzz_0);
        let of = Of::from_mdf(mdf);
        debug_assert_eq!(of, Of(0), "Resulting Of should be zero for invalid Mdf");
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_440 {
    use super::*;
    use crate::*;
    use crate::naive::internals::Of;
    use crate::Weekday;
    #[test]
    fn test_isoweekdate_raw() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u32, u8, u32, u8, u32, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        if let Some(of) = Of::new(rug_fuzz_0, YearFlags(rug_fuzz_1)) {
            let (week, weekday) = of.isoweekdate_raw();
            debug_assert_eq!(week, 1);
            debug_assert_eq!(weekday, Weekday::Mon);
        } else {
            panic!("Failed to construct Of with ordinal 1 and flags 0");
        }
        if let Some(of) = Of::new(rug_fuzz_2, YearFlags(rug_fuzz_3)) {
            let (week, weekday) = of.isoweekdate_raw();
            debug_assert_eq!(week, 1);
            debug_assert_eq!(weekday, Weekday::Sun);
        } else {
            panic!("Failed to construct Of with ordinal 7 and flags 0");
        }
        if let Some(of) = Of::new(rug_fuzz_4, YearFlags(rug_fuzz_5)) {
            let (week, weekday) = of.isoweekdate_raw();
            debug_assert_eq!(week, 2);
            debug_assert_eq!(weekday, Weekday::Mon);
        } else {
            panic!("Failed to construct Of with ordinal 8 and flags 0");
        }
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_441_llm_16_441 {
    use crate::naive::internals::{Of, YearFlags};
    #[test]
    fn test_of_new_with_valid_ordinal_and_flags() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let common_year_flag = YearFlags(rug_fuzz_0);
        let leap_year_flag = YearFlags(rug_fuzz_1);
        let valid_common_ordinal = rug_fuzz_2;
        let valid_leap_ordinal = rug_fuzz_3;
        let common_of = Of::new(valid_common_ordinal, common_year_flag);
        let leap_of = Of::new(valid_leap_ordinal, leap_year_flag);
        debug_assert!(common_of.is_some());
        debug_assert!(leap_of.is_some());
             }
});    }
    #[test]
    fn test_of_new_with_invalid_ordinal() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u8, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let common_year_flag = YearFlags(rug_fuzz_0);
        let invalid_ordinal = rug_fuzz_1;
        let result = Of::new(invalid_ordinal, common_year_flag);
        debug_assert!(result.is_none());
             }
});    }
    #[test]
    fn test_of_new_with_extreme_ordinal_values() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u8, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let leap_year_flag = YearFlags(rug_fuzz_0);
        let result_366 = Of::new(rug_fuzz_1, leap_year_flag);
        let result_0 = Of::new(rug_fuzz_2, leap_year_flag);
        debug_assert!(result_366.is_some());
        debug_assert!(result_0.is_none());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_445_llm_16_445 {
    use crate::naive::internals::{Of, Mdf, YearFlags};
    use std::convert::From;
    #[test]
    fn test_to_mdf() {
        let _rug_st_tests_llm_16_445_llm_16_445_rrrruuuugggg_test_to_mdf = 0;
        let _rug_ed_tests_llm_16_445_llm_16_445_rrrruuuugggg_test_to_mdf = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_446_llm_16_446 {
    use crate::naive::internals::{Of, YearFlags, MAX_OL, MIN_OL};
    #[test]
    fn of_valid_within_range() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u32, i32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mid_ol = (MIN_OL + MAX_OL) / rug_fuzz_0;
        let of = Of((mid_ol << rug_fuzz_1) | rug_fuzz_2);
        debug_assert!(of.valid());
             }
});    }
    #[test]
    fn of_valid_at_min() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let of = Of((MIN_OL << rug_fuzz_0) | rug_fuzz_1);
        debug_assert!(of.valid());
             }
});    }
    #[test]
    fn of_valid_at_max() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let of = Of((MAX_OL << rug_fuzz_0) | rug_fuzz_1);
        debug_assert!(of.valid());
             }
});    }
    #[test]
    fn of_invalid_below_min() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u32, i32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let of = Of(((MIN_OL - rug_fuzz_0) << rug_fuzz_1) | rug_fuzz_2);
        debug_assert!(! of.valid());
             }
});    }
    #[test]
    fn of_invalid_above_max() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u32, i32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let of = Of(((MAX_OL + rug_fuzz_0) << rug_fuzz_1) | rug_fuzz_2);
        debug_assert!(! of.valid());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_447 {
    use crate::{naive::internals::Of, Weekday};
    #[test]
    fn test_weekday() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13)) = <(u32, i32, u32, i32, u32, i32, u32, i32, u32, i32, u32, i32, u32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let ordinal_and_flags_for_monday = rug_fuzz_0;
        let day = Of(ordinal_and_flags_for_monday << rug_fuzz_1);
        debug_assert_eq!(day.weekday(), Weekday::Mon);
        let ordinal_and_flags_for_tuesday = rug_fuzz_2;
        let day = Of(ordinal_and_flags_for_tuesday << rug_fuzz_3);
        debug_assert_eq!(day.weekday(), Weekday::Tue);
        let ordinal_and_flags_for_wednesday = rug_fuzz_4;
        let day = Of(ordinal_and_flags_for_wednesday << rug_fuzz_5);
        debug_assert_eq!(day.weekday(), Weekday::Wed);
        let ordinal_and_flags_for_thursday = rug_fuzz_6;
        let day = Of(ordinal_and_flags_for_thursday << rug_fuzz_7);
        debug_assert_eq!(day.weekday(), Weekday::Thu);
        let ordinal_and_flags_for_friday = rug_fuzz_8;
        let day = Of(ordinal_and_flags_for_friday << rug_fuzz_9);
        debug_assert_eq!(day.weekday(), Weekday::Fri);
        let ordinal_and_flags_for_saturday = rug_fuzz_10;
        let day = Of(ordinal_and_flags_for_saturday << rug_fuzz_11);
        debug_assert_eq!(day.weekday(), Weekday::Sat);
        let ordinal_and_flags_for_sunday = rug_fuzz_12;
        let day = Of(ordinal_and_flags_for_sunday << rug_fuzz_13);
        debug_assert_eq!(day.weekday(), Weekday::Sun);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_449_llm_16_449 {
    use crate::naive::internals::YearFlags;
    #[test]
    fn test_from_year() {
        let _rug_st_tests_llm_16_449_llm_16_449_rrrruuuugggg_test_from_year = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 400;
        let rug_fuzz_2 = 800;
        let rug_fuzz_3 = 1200;
        let rug_fuzz_4 = 1600;
        let rug_fuzz_5 = 2000;
        let rug_fuzz_6 = 4;
        let rug_fuzz_7 = 0b1000;
        let rug_fuzz_8 = 0;
        let rug_fuzz_9 = 400;
        let rug_fuzz_10 = 0b1000;
        let rug_fuzz_11 = 0;
        let rug_fuzz_12 = 800;
        let rug_fuzz_13 = 0b1000;
        let rug_fuzz_14 = 0;
        let rug_fuzz_15 = 2000;
        let rug_fuzz_16 = 0b1000;
        let rug_fuzz_17 = 0;
        let rug_fuzz_18 = 1;
        let rug_fuzz_19 = 0b1000;
        let rug_fuzz_20 = 0;
        let rug_fuzz_21 = 100;
        let rug_fuzz_22 = 0b1000;
        let rug_fuzz_23 = 0;
        let rug_fuzz_24 = 200;
        let rug_fuzz_25 = 0b1000;
        let rug_fuzz_26 = 0;
        let rug_fuzz_27 = 300;
        let rug_fuzz_28 = 0b1000;
        let rug_fuzz_29 = 0;
        let rug_fuzz_30 = 500;
        let rug_fuzz_31 = 0b1000;
        let rug_fuzz_32 = 0;
        let rug_fuzz_33 = 1900;
        let rug_fuzz_34 = 0b1000;
        let rug_fuzz_35 = 0;
        debug_assert_eq!(YearFlags::from_year(rug_fuzz_0), YearFlags(0));
        debug_assert_eq!(YearFlags::from_year(rug_fuzz_1), YearFlags(0));
        debug_assert_eq!(YearFlags::from_year(rug_fuzz_2), YearFlags(0));
        debug_assert_eq!(YearFlags::from_year(rug_fuzz_3), YearFlags(0));
        debug_assert_eq!(YearFlags::from_year(rug_fuzz_4), YearFlags(0));
        debug_assert_eq!(YearFlags::from_year(rug_fuzz_5), YearFlags(0));
        debug_assert!(YearFlags::from_year(rug_fuzz_6).0 & rug_fuzz_7 == rug_fuzz_8);
        debug_assert!(YearFlags::from_year(rug_fuzz_9).0 & rug_fuzz_10 == rug_fuzz_11);
        debug_assert!(YearFlags::from_year(rug_fuzz_12).0 & rug_fuzz_13 == rug_fuzz_14);
        debug_assert!(YearFlags::from_year(rug_fuzz_15).0 & rug_fuzz_16 == rug_fuzz_17);
        debug_assert!(YearFlags::from_year(rug_fuzz_18).0 & rug_fuzz_19 != rug_fuzz_20);
        debug_assert!(YearFlags::from_year(rug_fuzz_21).0 & rug_fuzz_22 != rug_fuzz_23);
        debug_assert!(YearFlags::from_year(rug_fuzz_24).0 & rug_fuzz_25 != rug_fuzz_26);
        debug_assert!(YearFlags::from_year(rug_fuzz_27).0 & rug_fuzz_28 != rug_fuzz_29);
        debug_assert!(YearFlags::from_year(rug_fuzz_30).0 & rug_fuzz_31 != rug_fuzz_32);
        debug_assert!(YearFlags::from_year(rug_fuzz_33).0 & rug_fuzz_34 != rug_fuzz_35);
        let _rug_ed_tests_llm_16_449_llm_16_449_rrrruuuugggg_test_from_year = 0;
    }
}
