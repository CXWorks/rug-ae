use std::error::Error as StdError;
use std::fmt;
use std::str;
use std::time::{SystemTime, Duration, UNIX_EPOCH};
#[cfg(target_os = "cloudabi")]
mod max {
    pub const SECONDS: u64 = ::std::u64::MAX / 1_000_000_000;
    #[allow(unused)]
    pub const TIMESTAMP: &'static str = "2554-07-21T23:34:33Z";
}
#[cfg(
    all(
        target_pointer_width = "32",
        not(target_os = "cloudabi"),
        not(target_os = "windows"),
        not(all(target_arch = "wasm32", not(target_os = "emscripten")))
    )
)]
mod max {
    pub const SECONDS: u64 = ::std::i32::MAX as u64;
    #[allow(unused)]
    pub const TIMESTAMP: &'static str = "2038-01-19T03:14:07Z";
}
#[cfg(
    any(
        target_pointer_width = "64",
        target_os = "windows",
        all(target_arch = "wasm32", not(target_os = "emscripten")),
    )
)]
mod max {
    pub const SECONDS: u64 = 253_402_300_800 - 1;
    #[allow(unused)]
    pub const TIMESTAMP: &str = "9999-12-31T23:59:59Z";
}
/// Error parsing datetime (timestamp)
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Error {
    /// Numeric component is out of range
    OutOfRange,
    /// Bad character where digit is expected
    InvalidDigit,
    /// Other formatting errors
    InvalidFormat,
}
impl StdError for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::OutOfRange => write!(f, "numeric component is out of range"),
            Error::InvalidDigit => write!(f, "bad character where digit is expected"),
            Error::InvalidFormat => write!(f, "timestamp format is invalid"),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
enum Precision {
    Smart,
    Seconds,
    Millis,
    Micros,
    Nanos,
}
/// A wrapper type that allows you to Display a SystemTime
#[derive(Debug, Clone)]
pub struct Rfc3339Timestamp(SystemTime, Precision);
#[inline]
/// Converts two digits given in ASCII to its proper decimal representation.
fn two_digits(b1: u8, b2: u8) -> Result<u64, Error> {
    fn two_digits_inner(a: char, b: char) -> Option<u64> {
        let a = a.to_digit(10)?;
        let b = b.to_digit(10)?;
        Some((a * 10 + b) as u64)
    }
    two_digits_inner(b1 as char, b2 as char).ok_or(Error::InvalidDigit)
}
/// Parse RFC3339 timestamp `2018-02-14T00:28:07Z`
///
/// Supported feature: any precision of fractional
/// digits `2018-02-14T00:28:07.133Z`.
///
/// Unsupported feature: localized timestamps. Only UTC is supported.
pub fn parse_rfc3339(s: &str) -> Result<SystemTime, Error> {
    if s.len() < "2018-02-14T00:28:07Z".len() {
        return Err(Error::InvalidFormat);
    }
    let b = s.as_bytes();
    if b[10] != b'T' || b.last() != Some(&b'Z') {
        return Err(Error::InvalidFormat);
    }
    parse_rfc3339_weak(s)
}
/// Parse RFC3339-like timestamp `2018-02-14 00:28:07`
///
/// Supported features:
///
/// 1. Any precision of fractional digits `2018-02-14 00:28:07.133`.
/// 2. Supports timestamp with or without either of `T` or `Z`
/// 3. Anything valid for [`parse_rfc3339`](parse_rfc3339) is valid for this function
///
/// Unsupported feature: localized timestamps. Only UTC is supported, even if
/// `Z` is not specified.
///
/// This function is intended to use for parsing human input. Whereas
/// `parse_rfc3339` is for strings generated programmatically.
pub fn parse_rfc3339_weak(s: &str) -> Result<SystemTime, Error> {
    if s.len() < "2018-02-14T00:28:07".len() {
        return Err(Error::InvalidFormat);
    }
    let b = s.as_bytes();
    if b[4] != b'-' || b[7] != b'-' || (b[10] != b'T' && b[10] != b' ') || b[13] != b':'
        || b[16] != b':'
    {
        return Err(Error::InvalidFormat);
    }
    let year = two_digits(b[0], b[1])? * 100 + two_digits(b[2], b[3])?;
    let month = two_digits(b[5], b[6])?;
    let day = two_digits(b[8], b[9])?;
    let hour = two_digits(b[11], b[12])?;
    let minute = two_digits(b[14], b[15])?;
    let mut second = two_digits(b[17], b[18])?;
    if year < 1970 || hour > 23 || minute > 59 || second > 60 {
        return Err(Error::OutOfRange);
    }
    if second == 60 {
        second = 59;
    }
    let leap = is_leap_year(year);
    let (mut ydays, mdays) = match month {
        1 => (0, 31),
        2 if leap => (31, 29),
        2 => (31, 28),
        3 => (59, 31),
        4 => (90, 30),
        5 => (120, 31),
        6 => (151, 30),
        7 => (181, 31),
        8 => (212, 31),
        9 => (243, 30),
        10 => (273, 31),
        11 => (304, 30),
        12 => (334, 31),
        _ => return Err(Error::OutOfRange),
    };
    if day > mdays || day == 0 {
        return Err(Error::OutOfRange);
    }
    ydays += day - 1;
    if leap && month > 2 {
        ydays += 1;
    }
    let leap_years = ((year - 1) - 1968) / 4 - ((year - 1) - 1900) / 100
        + ((year - 1) - 1600) / 400;
    let days = (year - 1970) * 365 + leap_years + ydays;
    let time = second + minute * 60 + hour * 3600;
    let mut nanos = 0;
    let mut mult = 100_000_000;
    if b.get(19) == Some(&b'.') {
        for idx in 20..b.len() {
            if b[idx] == b'Z' {
                if idx == b.len() - 1 {
                    break;
                }
                return Err(Error::InvalidDigit);
            }
            nanos += mult * (b[idx] as char).to_digit(10).ok_or(Error::InvalidDigit)?;
            mult /= 10;
        }
    } else if b.len() != 19 && (b.len() > 20 || b[19] != b'Z') {
        return Err(Error::InvalidFormat);
    }
    let total_seconds = time + days * 86400;
    if total_seconds > max::SECONDS {
        return Err(Error::OutOfRange);
    }
    Ok(UNIX_EPOCH + Duration::new(total_seconds, nanos))
}
fn is_leap_year(y: u64) -> bool {
    y % 4 == 0 && (y % 100 != 0 || y % 400 == 0)
}
/// Format an RFC3339 timestamp `2018-02-14T00:28:07Z`
///
/// This function formats timestamp with smart precision: i.e. if it has no
/// fractional seconds, they aren't written at all. And up to nine digits if
/// they are.
///
/// The value is always UTC and ignores system timezone.
pub fn format_rfc3339(system_time: SystemTime) -> Rfc3339Timestamp {
    Rfc3339Timestamp(system_time, Precision::Smart)
}
/// Format an RFC3339 timestamp `2018-02-14T00:28:07Z`
///
/// This format always shows timestamp without fractional seconds.
///
/// The value is always UTC and ignores system timezone.
pub fn format_rfc3339_seconds(system_time: SystemTime) -> Rfc3339Timestamp {
    Rfc3339Timestamp(system_time, Precision::Seconds)
}
/// Format an RFC3339 timestamp `2018-02-14T00:28:07.000Z`
///
/// This format always shows milliseconds even if millisecond value is zero.
///
/// The value is always UTC and ignores system timezone.
pub fn format_rfc3339_millis(system_time: SystemTime) -> Rfc3339Timestamp {
    Rfc3339Timestamp(system_time, Precision::Millis)
}
/// Format an RFC3339 timestamp `2018-02-14T00:28:07.000000Z`
///
/// This format always shows microseconds even if microsecond value is zero.
///
/// The value is always UTC and ignores system timezone.
pub fn format_rfc3339_micros(system_time: SystemTime) -> Rfc3339Timestamp {
    Rfc3339Timestamp(system_time, Precision::Micros)
}
/// Format an RFC3339 timestamp `2018-02-14T00:28:07.000000000Z`
///
/// This format always shows nanoseconds even if nanosecond value is zero.
///
/// The value is always UTC and ignores system timezone.
pub fn format_rfc3339_nanos(system_time: SystemTime) -> Rfc3339Timestamp {
    Rfc3339Timestamp(system_time, Precision::Nanos)
}
impl Rfc3339Timestamp {
    /// Returns a reference to the [`SystemTime`][] that is being formatted.
    pub fn get_ref(&self) -> &SystemTime {
        &self.0
    }
}
impl fmt::Display for Rfc3339Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Precision::*;
        let dur = self
            .0
            .duration_since(UNIX_EPOCH)
            .expect("all times should be after the epoch");
        let secs_since_epoch = dur.as_secs();
        let nanos = dur.subsec_nanos();
        if secs_since_epoch >= 253_402_300_800 {
            return Err(fmt::Error);
        }
        const LEAPOCH: i64 = 11017;
        const DAYS_PER_400Y: i64 = 365 * 400 + 97;
        const DAYS_PER_100Y: i64 = 365 * 100 + 24;
        const DAYS_PER_4Y: i64 = 365 * 4 + 1;
        let days = (secs_since_epoch / 86400) as i64 - LEAPOCH;
        let secs_of_day = secs_since_epoch % 86400;
        let mut qc_cycles = days / DAYS_PER_400Y;
        let mut remdays = days % DAYS_PER_400Y;
        if remdays < 0 {
            remdays += DAYS_PER_400Y;
            qc_cycles -= 1;
        }
        let mut c_cycles = remdays / DAYS_PER_100Y;
        if c_cycles == 4 {
            c_cycles -= 1;
        }
        remdays -= c_cycles * DAYS_PER_100Y;
        let mut q_cycles = remdays / DAYS_PER_4Y;
        if q_cycles == 25 {
            q_cycles -= 1;
        }
        remdays -= q_cycles * DAYS_PER_4Y;
        let mut remyears = remdays / 365;
        if remyears == 4 {
            remyears -= 1;
        }
        remdays -= remyears * 365;
        let mut year = 2000 + remyears + 4 * q_cycles + 100 * c_cycles + 400 * qc_cycles;
        let months = [31, 30, 31, 30, 31, 31, 30, 31, 30, 31, 31, 29];
        let mut mon = 0;
        for mon_len in months.iter() {
            mon += 1;
            if remdays < *mon_len {
                break;
            }
            remdays -= *mon_len;
        }
        let mday = remdays + 1;
        let mon = if mon + 2 > 12 {
            year += 1;
            mon - 10
        } else {
            mon + 2
        };
        const BUF_INIT: [u8; 30] = *b"0000-00-00T00:00:00.000000000Z";
        let mut buf: [u8; 30] = BUF_INIT;
        buf[0] = b'0' + (year / 1000) as u8;
        buf[1] = b'0' + (year / 100 % 10) as u8;
        buf[2] = b'0' + (year / 10 % 10) as u8;
        buf[3] = b'0' + (year % 10) as u8;
        buf[5] = b'0' + (mon / 10) as u8;
        buf[6] = b'0' + (mon % 10) as u8;
        buf[8] = b'0' + (mday / 10) as u8;
        buf[9] = b'0' + (mday % 10) as u8;
        buf[11] = b'0' + (secs_of_day / 3600 / 10) as u8;
        buf[12] = b'0' + (secs_of_day / 3600 % 10) as u8;
        buf[14] = b'0' + (secs_of_day / 60 / 10 % 6) as u8;
        buf[15] = b'0' + (secs_of_day / 60 % 10) as u8;
        buf[17] = b'0' + (secs_of_day / 10 % 6) as u8;
        buf[18] = b'0' + (secs_of_day % 10) as u8;
        let offset = if self.1 == Seconds || nanos == 0 && self.1 == Smart {
            buf[19] = b'Z';
            19
        } else if self.1 == Millis {
            buf[20] = b'0' + (nanos / 100_000_000) as u8;
            buf[21] = b'0' + (nanos / 10_000_000 % 10) as u8;
            buf[22] = b'0' + (nanos / 1_000_000 % 10) as u8;
            buf[23] = b'Z';
            23
        } else if self.1 == Micros {
            buf[20] = b'0' + (nanos / 100_000_000) as u8;
            buf[21] = b'0' + (nanos / 10_000_000 % 10) as u8;
            buf[22] = b'0' + (nanos / 1_000_000 % 10) as u8;
            buf[23] = b'0' + (nanos / 100_000 % 10) as u8;
            buf[24] = b'0' + (nanos / 10_000 % 10) as u8;
            buf[25] = b'0' + (nanos / 1_000 % 10) as u8;
            buf[26] = b'Z';
            26
        } else {
            buf[20] = b'0' + (nanos / 100_000_000) as u8;
            buf[21] = b'0' + (nanos / 10_000_000 % 10) as u8;
            buf[22] = b'0' + (nanos / 1_000_000 % 10) as u8;
            buf[23] = b'0' + (nanos / 100_000 % 10) as u8;
            buf[24] = b'0' + (nanos / 10_000 % 10) as u8;
            buf[25] = b'0' + (nanos / 1_000 % 10) as u8;
            buf[26] = b'0' + (nanos / 100 % 10) as u8;
            buf[27] = b'0' + (nanos / 10 % 10) as u8;
            buf[28] = b'0' + (nanos % 10) as u8;
            29
        };
        f.write_str(str::from_utf8(&buf[..=offset]).expect("Conversion to utf8 failed"))
    }
}
#[cfg(test)]
mod test {
    use std::str::from_utf8;
    use std::time::{UNIX_EPOCH, SystemTime, Duration};
    use rand::Rng;
    use super::{parse_rfc3339, parse_rfc3339_weak, format_rfc3339};
    use super::{format_rfc3339_millis, format_rfc3339_micros};
    use super::format_rfc3339_nanos;
    use super::max;
    fn from_sec(sec: u64) -> (String, SystemTime) {
        let s = time::at_utc(time::Timespec {
                sec: sec as i64,
                nsec: 0,
            })
            .rfc3339()
            .to_string();
        let time = UNIX_EPOCH + Duration::new(sec, 0);
        (s, time)
    }
    #[test]
    #[cfg(all(target_pointer_width = "32", target_os = "linux"))]
    fn year_after_2038_fails_gracefully() {
        assert_eq!(
            parse_rfc3339("2038-01-19T03:14:08Z").unwrap_err(), super::Error::OutOfRange
        );
        assert_eq!(
            parse_rfc3339("9999-12-31T23:59:59Z").unwrap_err(), super::Error::OutOfRange
        );
    }
    #[test]
    fn smoke_tests_parse() {
        assert_eq!(
            parse_rfc3339("1970-01-01T00:00:00Z").unwrap(), UNIX_EPOCH + Duration::new(0,
            0)
        );
        assert_eq!(
            parse_rfc3339("1970-01-01T00:00:01Z").unwrap(), UNIX_EPOCH + Duration::new(1,
            0)
        );
        assert_eq!(
            parse_rfc3339("2018-02-13T23:08:32Z").unwrap(), UNIX_EPOCH +
            Duration::new(1_518_563_312, 0)
        );
        assert_eq!(
            parse_rfc3339("2012-01-01T00:00:00Z").unwrap(), UNIX_EPOCH +
            Duration::new(1_325_376_000, 0)
        );
    }
    #[test]
    fn smoke_tests_format() {
        assert_eq!(
            format_rfc3339(UNIX_EPOCH + Duration::new(0, 0)).to_string(),
            "1970-01-01T00:00:00Z"
        );
        assert_eq!(
            format_rfc3339(UNIX_EPOCH + Duration::new(1, 0)).to_string(),
            "1970-01-01T00:00:01Z"
        );
        assert_eq!(
            format_rfc3339(UNIX_EPOCH + Duration::new(1_518_563_312, 0)).to_string(),
            "2018-02-13T23:08:32Z"
        );
        assert_eq!(
            format_rfc3339(UNIX_EPOCH + Duration::new(1_325_376_000, 0)).to_string(),
            "2012-01-01T00:00:00Z"
        );
    }
    #[test]
    fn smoke_tests_format_millis() {
        assert_eq!(
            format_rfc3339_millis(UNIX_EPOCH + Duration::new(0, 0)).to_string(),
            "1970-01-01T00:00:00.000Z"
        );
        assert_eq!(
            format_rfc3339_millis(UNIX_EPOCH + Duration::new(1_518_563_312, 123_000_000))
            .to_string(), "2018-02-13T23:08:32.123Z"
        );
    }
    #[test]
    fn smoke_tests_format_micros() {
        assert_eq!(
            format_rfc3339_micros(UNIX_EPOCH + Duration::new(0, 0)).to_string(),
            "1970-01-01T00:00:00.000000Z"
        );
        assert_eq!(
            format_rfc3339_micros(UNIX_EPOCH + Duration::new(1_518_563_312, 123_000_000))
            .to_string(), "2018-02-13T23:08:32.123000Z"
        );
        assert_eq!(
            format_rfc3339_micros(UNIX_EPOCH + Duration::new(1_518_563_312, 456_123_000))
            .to_string(), "2018-02-13T23:08:32.456123Z"
        );
    }
    #[test]
    fn smoke_tests_format_nanos() {
        assert_eq!(
            format_rfc3339_nanos(UNIX_EPOCH + Duration::new(0, 0)).to_string(),
            "1970-01-01T00:00:00.000000000Z"
        );
        assert_eq!(
            format_rfc3339_nanos(UNIX_EPOCH + Duration::new(1_518_563_312, 123_000_000))
            .to_string(), "2018-02-13T23:08:32.123000000Z"
        );
        assert_eq!(
            format_rfc3339_nanos(UNIX_EPOCH + Duration::new(1_518_563_312, 789_456_123))
            .to_string(), "2018-02-13T23:08:32.789456123Z"
        );
    }
    #[test]
    fn upper_bound() {
        let max = UNIX_EPOCH + Duration::new(max::SECONDS, 0);
        assert_eq!(parse_rfc3339(& max::TIMESTAMP).unwrap(), max);
        assert_eq!(format_rfc3339(max).to_string(), max::TIMESTAMP);
    }
    #[test]
    fn leap_second() {
        assert_eq!(
            parse_rfc3339("2016-12-31T23:59:60Z").unwrap(), UNIX_EPOCH +
            Duration::new(1_483_228_799, 0)
        );
    }
    #[test]
    fn first_731_days() {
        let year_start = 0;
        for day in 0..=365 * 2 {
            let (s, time) = from_sec(year_start + day * 86400);
            assert_eq!(parse_rfc3339(& s).unwrap(), time);
            assert_eq!(format_rfc3339(time).to_string(), s);
        }
    }
    #[test]
    fn the_731_consecutive_days() {
        let year_start = 1_325_376_000;
        for day in 0..=365 * 2 {
            let (s, time) = from_sec(year_start + day * 86400);
            assert_eq!(parse_rfc3339(& s).unwrap(), time);
            assert_eq!(format_rfc3339(time).to_string(), s);
        }
    }
    #[test]
    fn all_86400_seconds() {
        let day_start = 1_325_376_000;
        for second in 0..86400 {
            let (s, time) = from_sec(day_start + second);
            assert_eq!(parse_rfc3339(& s).unwrap(), time);
            assert_eq!(format_rfc3339(time).to_string(), s);
        }
    }
    #[test]
    fn random_past() {
        let upper = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        for _ in 0..10000 {
            let sec = rand::thread_rng().gen_range(0, upper);
            let (s, time) = from_sec(sec);
            assert_eq!(parse_rfc3339(& s).unwrap(), time);
            assert_eq!(format_rfc3339(time).to_string(), s);
        }
    }
    #[test]
    fn random_wide_range() {
        for _ in 0..100_000 {
            let sec = rand::thread_rng().gen_range(0, max::SECONDS);
            let (s, time) = from_sec(sec);
            assert_eq!(parse_rfc3339(& s).unwrap(), time);
            assert_eq!(format_rfc3339(time).to_string(), s);
        }
    }
    #[test]
    fn milliseconds() {
        assert_eq!(
            parse_rfc3339("1970-01-01T00:00:00.123Z").unwrap(), UNIX_EPOCH +
            Duration::new(0, 123_000_000)
        );
        assert_eq!(
            format_rfc3339(UNIX_EPOCH + Duration::new(0, 123_000_000)).to_string(),
            "1970-01-01T00:00:00.123000000Z"
        );
    }
    #[test]
    #[should_panic(expected = "OutOfRange")]
    fn zero_month() {
        parse_rfc3339("1970-00-01T00:00:00Z").unwrap();
    }
    #[test]
    #[should_panic(expected = "OutOfRange")]
    fn big_month() {
        parse_rfc3339("1970-32-01T00:00:00Z").unwrap();
    }
    #[test]
    #[should_panic(expected = "OutOfRange")]
    fn zero_day() {
        parse_rfc3339("1970-01-00T00:00:00Z").unwrap();
    }
    #[test]
    #[should_panic(expected = "OutOfRange")]
    fn big_day() {
        parse_rfc3339("1970-12-35T00:00:00Z").unwrap();
    }
    #[test]
    #[should_panic(expected = "OutOfRange")]
    fn big_day2() {
        parse_rfc3339("1970-02-30T00:00:00Z").unwrap();
    }
    #[test]
    #[should_panic(expected = "OutOfRange")]
    fn big_second() {
        parse_rfc3339("1970-12-30T00:00:78Z").unwrap();
    }
    #[test]
    #[should_panic(expected = "OutOfRange")]
    fn big_minute() {
        parse_rfc3339("1970-12-30T00:78:00Z").unwrap();
    }
    #[test]
    #[should_panic(expected = "OutOfRange")]
    fn big_hour() {
        parse_rfc3339("1970-12-30T24:00:00Z").unwrap();
    }
    #[test]
    fn break_data() {
        for pos in 0.."2016-12-31T23:59:60Z".len() {
            let mut s = b"2016-12-31T23:59:60Z".to_vec();
            s[pos] = b'x';
            parse_rfc3339(from_utf8(&s).unwrap()).unwrap_err();
        }
    }
    #[test]
    fn weak_smoke_tests() {
        assert_eq!(
            parse_rfc3339_weak("1970-01-01 00:00:00").unwrap(), UNIX_EPOCH +
            Duration::new(0, 0)
        );
        parse_rfc3339("1970-01-01 00:00:00").unwrap_err();
        assert_eq!(
            parse_rfc3339_weak("1970-01-01 00:00:00.000123").unwrap(), UNIX_EPOCH +
            Duration::new(0, 123_000)
        );
        parse_rfc3339("1970-01-01 00:00:00.000123").unwrap_err();
        assert_eq!(
            parse_rfc3339_weak("1970-01-01T00:00:00.000123").unwrap(), UNIX_EPOCH +
            Duration::new(0, 123_000)
        );
        parse_rfc3339("1970-01-01T00:00:00.000123").unwrap_err();
        assert_eq!(
            parse_rfc3339_weak("1970-01-01 00:00:00.000123Z").unwrap(), UNIX_EPOCH +
            Duration::new(0, 123_000)
        );
        parse_rfc3339("1970-01-01 00:00:00.000123Z").unwrap_err();
        assert_eq!(
            parse_rfc3339_weak("1970-01-01 00:00:00Z").unwrap(), UNIX_EPOCH +
            Duration::new(0, 0)
        );
        parse_rfc3339("1970-01-01 00:00:00Z").unwrap_err();
    }
}
#[cfg(test)]
mod tests_llm_16_13 {
    use super::*;
    use crate::*;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};
    #[test]
    fn test_get_ref() {
        let _rug_st_tests_llm_16_13_rrrruuuugggg_test_get_ref = 0;
        let now = SystemTime::now();
        let timestamp = Rfc3339Timestamp(now, Precision::Seconds);
        let time_ref = timestamp.get_ref();
        debug_assert_eq!(now, * time_ref);
        let _rug_ed_tests_llm_16_13_rrrruuuugggg_test_get_ref = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_14_llm_16_14 {
    use super::*;
    use crate::*;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};
    #[test]
    fn test_format_rfc3339_no_fractional_seconds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let time = UNIX_EPOCH + Duration::new(rug_fuzz_0, rug_fuzz_1);
        let formatted = super::format_rfc3339(time);
        debug_assert_eq!(formatted.to_string(), "2017-12-31T23:54:47Z");
             }
}
}
}    }
    #[test]
    fn test_format_rfc3339_with_fractional_seconds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let time = UNIX_EPOCH + Duration::new(rug_fuzz_0, rug_fuzz_1);
        let formatted = super::format_rfc3339(time);
        debug_assert_eq!(formatted.to_string(), "2017-12-31T23:54:47.123456789Z");
             }
}
}
}    }
    #[test]
    fn test_format_rfc3339_with_zero_seconds() {
        let _rug_st_tests_llm_16_14_llm_16_14_rrrruuuugggg_test_format_rfc3339_with_zero_seconds = 0;
        let time = UNIX_EPOCH;
        let formatted = super::format_rfc3339(time);
        debug_assert_eq!(formatted.to_string(), "1970-01-01T00:00:00Z");
        let _rug_ed_tests_llm_16_14_llm_16_14_rrrruuuugggg_test_format_rfc3339_with_zero_seconds = 0;
    }
    #[test]
    #[should_panic(expected = "all times should be after the epoch")]
    fn test_format_rfc3339_before_epoch() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let time = UNIX_EPOCH - Duration::new(rug_fuzz_0, rug_fuzz_1);
        let _formatted = super::format_rfc3339(time);
             }
}
}
}    }
    #[test]
    fn test_format_rfc3339_year_9999() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u64, u64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let time = UNIX_EPOCH + Duration::new(rug_fuzz_0 - rug_fuzz_1, rug_fuzz_2);
        let formatted = super::format_rfc3339(time);
        debug_assert_eq!(formatted.to_string(), "9999-12-31T23:59:59Z");
             }
}
}
}    }
    #[test]
    #[should_panic(expected = "all times should be after the epoch")]
    fn test_format_rfc3339_invalid_utf8() {
        let _rug_st_tests_llm_16_14_llm_16_14_rrrruuuugggg_test_format_rfc3339_invalid_utf8 = 0;
        let _rug_ed_tests_llm_16_14_llm_16_14_rrrruuuugggg_test_format_rfc3339_invalid_utf8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_15 {
    use super::*;
    use crate::*;
    use std::time::{Duration, UNIX_EPOCH};
    #[test]
    fn test_format_rfc3339_micros() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u64, u32, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let time = UNIX_EPOCH + Duration::new(rug_fuzz_0, rug_fuzz_1);
        let timestamp = date::format_rfc3339_micros(time);
        let expected = rug_fuzz_2;
        debug_assert_eq!(timestamp.to_string(), expected);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_16 {
    use super::*;
    use crate::*;
    use std::time::{Duration, UNIX_EPOCH};
    #[test]
    fn test_format_rfc3339_millis() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u64, u32, u64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let test_time = UNIX_EPOCH + Duration::new(rug_fuzz_0, rug_fuzz_1);
        let formatted_time = format_rfc3339_millis(test_time);
        debug_assert_eq!(formatted_time.to_string(), "2018-02-14T00:28:07.123Z");
        let test_time_zero_millis = UNIX_EPOCH + Duration::new(rug_fuzz_2, rug_fuzz_3);
        let formatted_time_zero_millis = format_rfc3339_millis(test_time_zero_millis);
        debug_assert_eq!(
            formatted_time_zero_millis.to_string(), "2018-02-14T00:28:07.000Z"
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_17 {
    use super::*;
    use crate::*;
    use std::time::{SystemTime, UNIX_EPOCH};
    #[test]
    fn test_format_rfc3339_nanos() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u64, u32, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let test_time = UNIX_EPOCH + std::time::Duration::new(rug_fuzz_0, rug_fuzz_1);
        let formatted_time = format_rfc3339_nanos(test_time);
        let formatted_time_str = formatted_time.to_string();
        let expected_time_str = rug_fuzz_2;
        debug_assert_eq!(formatted_time_str, expected_time_str);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_18 {
    use super::*;
    use crate::*;
    use std::time::{SystemTime, UNIX_EPOCH};
    use date::Rfc3339Timestamp;
    #[test]
    fn test_format_rfc3339_seconds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let time = UNIX_EPOCH + std::time::Duration::new(rug_fuzz_0, rug_fuzz_1);
        let formatted = format_rfc3339_seconds(time);
        debug_assert_eq!(formatted.to_string(), "2018-02-14T00:28:47Z");
             }
}
}
}    }
    #[test]
    fn test_format_rfc3339_seconds_before_epoch() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u64, u32, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let time = UNIX_EPOCH - std::time::Duration::new(rug_fuzz_0, rug_fuzz_1);
        let formatted = format_rfc3339_seconds(time);
        debug_assert!(formatted.to_string().starts_with(rug_fuzz_2));
             }
}
}
}    }
    #[test]
    fn test_format_rfc3339_seconds_edge_case() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let max_time = UNIX_EPOCH + std::time::Duration::new(rug_fuzz_0, rug_fuzz_1);
        let formatted = format_rfc3339_seconds(max_time);
        debug_assert_eq!(formatted.to_string(), "9999-12-31T23:59:59Z");
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_20 {
    use crate::parse_rfc3339;
    use std::time::SystemTime;
    #[test]
    fn test_parse_rfc3339_valid() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let inputs = vec![
            rug_fuzz_0, "2018-02-14T00:28:07.133Z", "2018-02-14T00:28:07.133333Z"
        ];
        for input in inputs {
            debug_assert!(parse_rfc3339(input).is_ok());
        }
             }
}
}
}    }
    #[test]
    fn test_parse_rfc3339_invalid_format() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let inputs = vec![
            rug_fuzz_0, "2018-02-14T00:28:07", "20180214T002807Z",
            "2018-02-14T00:28:07.133+00:00"
        ];
        for input in inputs {
            match parse_rfc3339(input) {
                Err(super::Error::InvalidFormat) => {}
                _ => panic!("expected Error::InvalidFormat for input {}", input),
            }
        }
             }
}
}
}    }
    #[test]
    fn test_parse_rfc3339_invalid_length() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = rug_fuzz_0;
        match parse_rfc3339(input) {
            Err(super::Error::InvalidFormat) => {}
            _ => panic!("expected Error::InvalidFormat for input {}", input),
        }
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_21_llm_16_21 {
    use super::*;
    use crate::*;
    use crate::date::parse_rfc3339_weak;
    use crate::date::Error;
    use std::time::{Duration, UNIX_EPOCH};
    #[test]
    fn test_parse_rfc3339_weak_valid() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let datetime_str = rug_fuzz_0;
        debug_assert_eq!(
            parse_rfc3339_weak(datetime_str), Ok(UNIX_EPOCH + Duration::new(1518562087,
            0))
        );
        let datetime_str_with_fraction = rug_fuzz_1;
        debug_assert_eq!(
            parse_rfc3339_weak(datetime_str_with_fraction), Ok(UNIX_EPOCH +
            Duration::new(1518562087, 133_000_000))
        );
        let datetime_str_with_t = rug_fuzz_2;
        debug_assert_eq!(
            parse_rfc3339_weak(datetime_str_with_t), Ok(UNIX_EPOCH +
            Duration::new(1518562087, 0))
        );
        let datetime_str_with_z = rug_fuzz_3;
        debug_assert_eq!(
            parse_rfc3339_weak(datetime_str_with_z), Ok(UNIX_EPOCH +
            Duration::new(1518562087, 0))
        );
             }
}
}
}    }
    #[test]
    fn test_parse_rfc3339_weak_invalid_format() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let datetime_str_too_short = rug_fuzz_0;
        debug_assert_eq!(
            parse_rfc3339_weak(datetime_str_too_short), Err(Error::InvalidFormat)
        );
        let datetime_str_bad_chars = rug_fuzz_1;
        debug_assert_eq!(
            parse_rfc3339_weak(datetime_str_bad_chars), Err(Error::InvalidFormat)
        );
        let datetime_str_missing_parts = rug_fuzz_2;
        debug_assert_eq!(
            parse_rfc3339_weak(datetime_str_missing_parts), Err(Error::InvalidFormat)
        );
             }
}
}
}    }
    #[test]
    fn test_parse_rfc3339_weak_out_of_range() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(&str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let datetime_str_invalid_month = rug_fuzz_0;
        debug_assert_eq!(
            parse_rfc3339_weak(datetime_str_invalid_month), Err(Error::OutOfRange)
        );
        let datetime_str_invalid_day = rug_fuzz_1;
        debug_assert_eq!(
            parse_rfc3339_weak(datetime_str_invalid_day), Err(Error::OutOfRange)
        );
        let datetime_str_invalid_hour = rug_fuzz_2;
        debug_assert_eq!(
            parse_rfc3339_weak(datetime_str_invalid_hour), Err(Error::OutOfRange)
        );
        let datetime_str_invalid_minute = rug_fuzz_3;
        debug_assert_eq!(
            parse_rfc3339_weak(datetime_str_invalid_minute), Err(Error::OutOfRange)
        );
        let datetime_str_invalid_second = rug_fuzz_4;
        debug_assert_eq!(
            parse_rfc3339_weak(datetime_str_invalid_second), Err(Error::OutOfRange)
        );
             }
}
}
}    }
    #[test]
    fn test_parse_rfc3339_weak_invalid_digit() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let datetime_str_bad_fraction = rug_fuzz_0;
        debug_assert_eq!(
            parse_rfc3339_weak(datetime_str_bad_fraction), Err(Error::InvalidDigit)
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_22 {
    use super::*;
    use crate::*;
    #[test]
    fn test_two_digits_valid_input() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(two_digits(rug_fuzz_0, rug_fuzz_1).unwrap(), 12);
        debug_assert_eq!(two_digits(rug_fuzz_2, rug_fuzz_3).unwrap(), 0);
        debug_assert_eq!(two_digits(rug_fuzz_4, rug_fuzz_5).unwrap(), 99);
             }
}
}
}    }
    #[test]
    fn test_two_digits_invalid_input() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(two_digits(rug_fuzz_0, rug_fuzz_1).is_err());
        debug_assert!(two_digits(rug_fuzz_2, rug_fuzz_3).is_err());
        debug_assert!(two_digits(rug_fuzz_4, rug_fuzz_5).is_err());
             }
}
}
}    }
    #[test]
    fn test_two_digits_boundary_input() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(two_digits(rug_fuzz_0, rug_fuzz_1).unwrap(), 9);
        debug_assert_eq!(two_digits(rug_fuzz_2, rug_fuzz_3).unwrap(), 10);
        debug_assert_eq!(two_digits(rug_fuzz_4, rug_fuzz_5).unwrap(), 90);
             }
}
}
}    }
    #[test]
    fn test_two_digits_non_digit_input() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(two_digits(rug_fuzz_0, rug_fuzz_1).is_err());
        debug_assert!(two_digits(rug_fuzz_2, rug_fuzz_3).is_err());
        debug_assert!(two_digits(rug_fuzz_4, rug_fuzz_5).is_err());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_4 {
    use super::*;
    #[test]
    fn test_is_leap_year() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u64, u64, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: u64 = rug_fuzz_0;
        debug_assert_eq!(crate ::date::is_leap_year(p0), true);
        p0 = rug_fuzz_1;
        debug_assert_eq!(crate ::date::is_leap_year(p0), false);
        p0 = rug_fuzz_2;
        debug_assert_eq!(crate ::date::is_leap_year(p0), true);
        p0 = rug_fuzz_3;
        debug_assert_eq!(crate ::date::is_leap_year(p0), false);
             }
}
}
}    }
}
