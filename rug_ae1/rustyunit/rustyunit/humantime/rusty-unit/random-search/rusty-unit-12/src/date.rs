use std::error::Error as StdError;
use std::fmt;
use std::str;
use std::time::{SystemTime, Duration, UNIX_EPOCH};

#[cfg(target_os="cloudabi")]
mod max {
    pub const SECONDS: u64 = ::std::u64::MAX / 1_000_000_000;
    #[allow(unused)]
    pub const TIMESTAMP: &'static str = "2554-07-21T23:34:33Z";
}
#[cfg(all(
    target_pointer_width="32",
    not(target_os="cloudabi"),
    not(target_os="windows"),
    not(all(target_arch="wasm32", not(target_os="emscripten")))
))]
mod max {
    pub const SECONDS: u64 = ::std::i32::MAX as u64;
    #[allow(unused)]
    pub const TIMESTAMP: &'static str = "2038-01-19T03:14:07Z";
}

#[cfg(any(
    target_pointer_width="64",
    target_os="windows",
    all(target_arch="wasm32", not(target_os="emscripten")),
))]
mod max {
    pub const SECONDS: u64 = 253_402_300_800-1;  // last second of year 9999
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
fn two_digits_inner(a: char, b: char) -> Option<u64> {
    let a = a.to_digit(10)?;
    let b = b.to_digit(10)?;

    Some((a*10 + b) as u64)
}

/// Converts two digits given in ASCII to its proper decimal representation.
fn two_digits(b1: u8, b2: u8) -> Result<u64, Error> {

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
    let b = s.as_bytes();  // for careless slicing
    if b[4] != b'-' || b[7] != b'-' || (b[10] != b'T' && b[10] != b' ') ||
       b[13] != b':' || b[16] != b':'
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
    // TODO(tailhook) should we check that leaps second is only on midnight ?
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

    let leap_years = ((year - 1) - 1968) / 4 - ((year - 1) - 1900) / 100 + ((year - 1) - 1600) / 400;
    let days = (year - 1970) * 365 + leap_years + ydays;

    let time = second + minute * 60 + hour * 3600;

    let mut nanos = 0;
    let mut mult = 100_000_000;
    if b.get(19) == Some(&b'.') {
        for idx in 20..b.len() {
            if b[idx] == b'Z' {
                if idx == b.len()-1 {
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

        let dur = self.0.duration_since(UNIX_EPOCH)
            .expect("all times should be after the epoch");
        let secs_since_epoch = dur.as_secs();
        let nanos = dur.subsec_nanos();

        if secs_since_epoch >= 253_402_300_800 { // year 9999
            return Err(fmt::Error);
        }

        /* 2000-03-01 (mod 400 year, immediately after feb29 */
        const LEAPOCH: i64 = 11017;
        const DAYS_PER_400Y: i64 = 365*400 + 97;
        const DAYS_PER_100Y: i64 = 365*100 + 24;
        const DAYS_PER_4Y: i64 = 365*4 + 1;

        let days = (secs_since_epoch / 86400) as i64 - LEAPOCH;
        let secs_of_day = secs_since_epoch % 86400;

        let mut qc_cycles = days / DAYS_PER_400Y;
        let mut remdays = days % DAYS_PER_400Y;

        if remdays < 0 {
            remdays += DAYS_PER_400Y;
            qc_cycles -= 1;
        }

        let mut c_cycles = remdays / DAYS_PER_100Y;
        if c_cycles == 4 { c_cycles -= 1; }
        remdays -= c_cycles * DAYS_PER_100Y;

        let mut q_cycles = remdays / DAYS_PER_4Y;
        if q_cycles == 25 { q_cycles -= 1; }
        remdays -= q_cycles * DAYS_PER_4Y;

        let mut remyears = remdays / 365;
        if remyears == 4 { remyears -= 1; }
        remdays -= remyears * 365;

        let mut year = 2000 +
            remyears + 4*q_cycles + 100*c_cycles + 400*qc_cycles;

        let months = [31,30,31,30,31,31,30,31,30,31,31,29];
        let mut mon = 0;
        for mon_len in months.iter() {
            mon += 1;
            if remdays < *mon_len {
                break;
            }
            remdays -= *mon_len;
        }
        let mday = remdays+1;
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
            // 29th is 'Z'
            29
        };

        // we know our chars are all ascii
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
    use super::{format_rfc3339_nanos};
    use super::max;

    fn from_sec(sec: u64) -> (String, SystemTime) {
        let s = time::at_utc(time::Timespec { sec: sec as i64, nsec: 0 })
                  .rfc3339().to_string();
        let time = UNIX_EPOCH + Duration::new(sec, 0);
        (s, time)
    }

    #[test]
    #[cfg(all(target_pointer_width="32", target_os="linux"))]
    fn year_after_2038_fails_gracefully() {
        // next second
        assert_eq!(parse_rfc3339("2038-01-19T03:14:08Z").unwrap_err(),
                   super::Error::OutOfRange);
        assert_eq!(parse_rfc3339("9999-12-31T23:59:59Z").unwrap_err(),
                   super::Error::OutOfRange);
    }

    #[test]
    fn smoke_tests_parse() {
        assert_eq!(parse_rfc3339("1970-01-01T00:00:00Z").unwrap(),
                   UNIX_EPOCH + Duration::new(0, 0));
        assert_eq!(parse_rfc3339("1970-01-01T00:00:01Z").unwrap(),
                   UNIX_EPOCH + Duration::new(1, 0));
        assert_eq!(parse_rfc3339("2018-02-13T23:08:32Z").unwrap(),
                   UNIX_EPOCH + Duration::new(1_518_563_312, 0));
        assert_eq!(parse_rfc3339("2012-01-01T00:00:00Z").unwrap(),
                   UNIX_EPOCH + Duration::new(1_325_376_000, 0));
    }

    #[test]
    fn smoke_tests_format() {
        assert_eq!(
            format_rfc3339(UNIX_EPOCH + Duration::new(0, 0)).to_string(),
            "1970-01-01T00:00:00Z");
        assert_eq!(
            format_rfc3339(UNIX_EPOCH + Duration::new(1, 0)).to_string(),
            "1970-01-01T00:00:01Z");
        assert_eq!(
            format_rfc3339(UNIX_EPOCH + Duration::new(1_518_563_312, 0)).to_string(),
            "2018-02-13T23:08:32Z");
        assert_eq!(
            format_rfc3339(UNIX_EPOCH + Duration::new(1_325_376_000, 0)).to_string(),
            "2012-01-01T00:00:00Z");
    }

    #[test]
    fn smoke_tests_format_millis() {
        assert_eq!(
            format_rfc3339_millis(UNIX_EPOCH +
                Duration::new(0, 0)).to_string(),
            "1970-01-01T00:00:00.000Z");
        assert_eq!(
            format_rfc3339_millis(UNIX_EPOCH +
                Duration::new(1_518_563_312, 123_000_000)).to_string(),
            "2018-02-13T23:08:32.123Z");
    }

    #[test]
    fn smoke_tests_format_micros() {
        assert_eq!(
            format_rfc3339_micros(UNIX_EPOCH +
                Duration::new(0, 0)).to_string(),
            "1970-01-01T00:00:00.000000Z");
        assert_eq!(
            format_rfc3339_micros(UNIX_EPOCH +
                Duration::new(1_518_563_312, 123_000_000)).to_string(),
            "2018-02-13T23:08:32.123000Z");
        assert_eq!(
            format_rfc3339_micros(UNIX_EPOCH +
                Duration::new(1_518_563_312, 456_123_000)).to_string(),
            "2018-02-13T23:08:32.456123Z");
    }

    #[test]
    fn smoke_tests_format_nanos() {
        assert_eq!(
            format_rfc3339_nanos(UNIX_EPOCH +
                Duration::new(0, 0)).to_string(),
            "1970-01-01T00:00:00.000000000Z");
        assert_eq!(
            format_rfc3339_nanos(UNIX_EPOCH +
                Duration::new(1_518_563_312, 123_000_000)).to_string(),
            "2018-02-13T23:08:32.123000000Z");
        assert_eq!(
            format_rfc3339_nanos(UNIX_EPOCH +
                Duration::new(1_518_563_312, 789_456_123)).to_string(),
            "2018-02-13T23:08:32.789456123Z");
    }

    #[test]
    fn upper_bound() {
        let max = UNIX_EPOCH + Duration::new(max::SECONDS, 0);
        assert_eq!(parse_rfc3339(&max::TIMESTAMP).unwrap(), max);
        assert_eq!(format_rfc3339(max).to_string(), max::TIMESTAMP);
    }

    #[test]
    fn leap_second() {
        assert_eq!(parse_rfc3339("2016-12-31T23:59:60Z").unwrap(),
                   UNIX_EPOCH + Duration::new(1_483_228_799, 0));
    }

    #[test]
    fn first_731_days() {
        let year_start = 0;  // 1970
        for day in 0..= 365 * 2 {  // scan leap year and non-leap year
            let (s, time) = from_sec(year_start + day * 86400);
            assert_eq!(parse_rfc3339(&s).unwrap(), time);
            assert_eq!(format_rfc3339(time).to_string(), s);
        }
    }

    #[test]
    fn the_731_consecutive_days() {
        let year_start = 1_325_376_000;  // 2012
        for day in 0..= 365 * 2 {  // scan leap year and non-leap year
            let (s, time) = from_sec(year_start + day * 86400);
            assert_eq!(parse_rfc3339(&s).unwrap(), time);
            assert_eq!(format_rfc3339(time).to_string(), s);
        }
    }

    #[test]
    fn all_86400_seconds() {
        let day_start = 1_325_376_000;
        for second in 0..86400 {  // scan leap year and non-leap year
            let (s, time) = from_sec(day_start + second);
            assert_eq!(parse_rfc3339(&s).unwrap(), time);
            assert_eq!(format_rfc3339(time).to_string(), s);
        }
    }

    #[test]
    fn random_past() {
        let upper = SystemTime::now().duration_since(UNIX_EPOCH).unwrap()
            .as_secs();
        for _ in 0..10000 {
            let sec = rand::thread_rng().gen_range(0, upper);
            let (s, time) = from_sec(sec);
            assert_eq!(parse_rfc3339(&s).unwrap(), time);
            assert_eq!(format_rfc3339(time).to_string(), s);
        }
    }

    #[test]
    fn random_wide_range() {
        for _ in 0..100_000 {
            let sec = rand::thread_rng().gen_range(0, max::SECONDS);
            let (s, time) = from_sec(sec);
            assert_eq!(parse_rfc3339(&s).unwrap(), time);
            assert_eq!(format_rfc3339(time).to_string(), s);
        }
    }

    #[test]
    fn milliseconds() {
        assert_eq!(parse_rfc3339("1970-01-01T00:00:00.123Z").unwrap(),
                   UNIX_EPOCH + Duration::new(0, 123_000_000));
        assert_eq!(format_rfc3339(UNIX_EPOCH + Duration::new(0, 123_000_000))
            .to_string(), "1970-01-01T00:00:00.123000000Z");
    }

    #[test]
    #[should_panic(expected="OutOfRange")]
    fn zero_month() {
        parse_rfc3339("1970-00-01T00:00:00Z").unwrap();
    }

    #[test]
    #[should_panic(expected="OutOfRange")]
    fn big_month() {
        parse_rfc3339("1970-32-01T00:00:00Z").unwrap();
    }

    #[test]
    #[should_panic(expected="OutOfRange")]
    fn zero_day() {
        parse_rfc3339("1970-01-00T00:00:00Z").unwrap();
    }

    #[test]
    #[should_panic(expected="OutOfRange")]
    fn big_day() {
        parse_rfc3339("1970-12-35T00:00:00Z").unwrap();
    }

    #[test]
    #[should_panic(expected="OutOfRange")]
    fn big_day2() {
        parse_rfc3339("1970-02-30T00:00:00Z").unwrap();
    }

    #[test]
    #[should_panic(expected="OutOfRange")]
    fn big_second() {
        parse_rfc3339("1970-12-30T00:00:78Z").unwrap();
    }

    #[test]
    #[should_panic(expected="OutOfRange")]
    fn big_minute() {
        parse_rfc3339("1970-12-30T00:78:00Z").unwrap();
    }

    #[test]
    #[should_panic(expected="OutOfRange")]
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
        assert_eq!(parse_rfc3339_weak("1970-01-01 00:00:00").unwrap(),
                   UNIX_EPOCH + Duration::new(0, 0));
        parse_rfc3339("1970-01-01 00:00:00").unwrap_err();

        assert_eq!(parse_rfc3339_weak("1970-01-01 00:00:00.000123").unwrap(),
                   UNIX_EPOCH + Duration::new(0, 123_000));
        parse_rfc3339("1970-01-01 00:00:00.000123").unwrap_err();

        assert_eq!(parse_rfc3339_weak("1970-01-01T00:00:00.000123").unwrap(),
                   UNIX_EPOCH + Duration::new(0, 123_000));
        parse_rfc3339("1970-01-01T00:00:00.000123").unwrap_err();

        assert_eq!(parse_rfc3339_weak("1970-01-01 00:00:00.000123Z").unwrap(),
                   UNIX_EPOCH + Duration::new(0, 123_000));
        parse_rfc3339("1970-01-01 00:00:00.000123Z").unwrap_err();

        assert_eq!(parse_rfc3339_weak("1970-01-01 00:00:00Z").unwrap(),
                   UNIX_EPOCH + Duration::new(0, 0));
        parse_rfc3339("1970-01-01 00:00:00Z").unwrap_err();
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2932() {
    rusty_monitor::set_test_id(2932);
    let mut u64_0: u64 = 7159u64;
    let mut usize_0: usize = 9542usize;
    let mut usize_1: usize = 8264usize;
    let mut str_0: &str = "G0AVpgqtcu0MUBnRs";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u64_1: u64 = 801u64;
    let mut u8_0: u8 = 0u8;
    let mut u8_1: u8 = 55u8;
    let mut u8_2: u8 = 52u8;
    let mut u8_3: u8 = 57u8;
    let mut u32_0: u32 = 3789u32;
    let mut str_1: &str = "aDWDOu";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bool_0: bool = true;
    let mut bool_0_ref_0: &mut bool = &mut bool_0;
    let mut u64_2: u64 = 6076u64;
    let mut u64_3: u64 = 5511u64;
    let mut tuple_0: (u64, u64) = (u64_3, u64_2);
    let mut str_2: &str = "9T";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut result_0: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_3, u8_2);
    let mut u64_4: u64 = std::result::Result::unwrap(result_0);
    let mut error_0: duration::Error = crate::duration::Error::Empty;
    let mut result_1: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_1, u8_0);
    let mut u64_5: u64 = std::result::Result::unwrap(result_1);
    let mut precision_0: date::Precision = crate::date::Precision::Micros;
    let mut error_1: date::Error = crate::date::Error::InvalidFormat;
    let mut bool_1: bool = crate::date::is_leap_year(u64_1);
    let mut error_2: date::Error = crate::date::Error::OutOfRange;
    let mut error_3: date::Error = crate::date::Error::InvalidDigit;
    let mut error_4: date::Error = crate::date::Error::InvalidDigit;
    let mut result_2: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_681() {
    rusty_monitor::set_test_id(681);
    let mut usize_0: usize = 3560usize;
    let mut u8_0: u8 = 10u8;
    let mut u8_1: u8 = 45u8;
    let mut usize_1: usize = 6261usize;
    let mut str_0: &str = "E7a3KSdicaP";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut char_0: char = 'E';
    let mut char_1: char = 'f';
    let mut str_1: &str = "fZuIX6ZK2jPcpUR";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "8tvyVsLsV";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut u64_0: u64 = 2687u64;
    let mut str_3: &str = "LAB1rFv";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut bool_0: bool = false;
    let mut bool_0_ref_0: &mut bool = &mut bool_0;
    let mut str_4: &str = "";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_4_ref_0);
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_0);
    let mut precision_0: date::Precision = crate::date::Precision::Micros;
    let mut precision_1: date::Precision = crate::date::Precision::Nanos;
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_2_ref_0);
    let mut error_0: date::Error = crate::date::Error::InvalidDigit;
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_1_ref_0);
    let mut option_0: std::option::Option<u64> = crate::date::two_digits_inner(char_1, char_0);
    let mut result_3: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_0_ref_0);
    let mut error_1: duration::Error = crate::duration::Error::InvalidCharacter(usize_1);
    let mut precision_2: date::Precision = crate::date::Precision::Smart;
    let mut duration_1: std::time::Duration = std::result::Result::unwrap(result_1);
    let mut result_4: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_1, u8_0);
    let mut error_2: duration::Error = crate::duration::Error::InvalidCharacter(usize_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4374() {
    rusty_monitor::set_test_id(4374);
    let mut str_0: &str = "u1rvtZdFgJrRHnDZWu";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u64_0: u64 = 1647u64;
    let mut usize_0: usize = 5941usize;
    let mut usize_1: usize = 3657usize;
    let mut usize_2: usize = 2962usize;
    let mut u64_1: u64 = 5248u64;
    let mut usize_3: usize = 7219usize;
    let mut usize_4: usize = 5284usize;
    let mut str_1: &str = "jpMEgJK9az88b7";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut u32_0: u32 = 1003u32;
    let mut str_2: &str = "aUZvc";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bool_0: bool = false;
    let mut bool_0_ref_0: &mut bool = &mut bool_0;
    let mut precision_0: date::Precision = crate::date::Precision::Millis;
    let mut precision_1: date::Precision = crate::date::Precision::Millis;
    let mut error_0: date::Error = crate::date::Error::InvalidFormat;
    let mut precision_2: date::Precision = crate::date::Precision::Seconds;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_1_ref_0);
    let mut precision_3: date::Precision = crate::date::Precision::Smart;
    let mut error_1: date::Error = crate::date::Error::InvalidDigit;
    let mut precision_4: date::Precision = crate::date::Precision::Millis;
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_0);
    let mut precision_5: date::Precision = crate::date::Precision::Smart;
    let mut error_2: duration::Error = crate::duration::Error::NumberExpected(usize_2);
    let mut error_3: duration::Error = crate::duration::Error::Empty;
    let mut precision_6: date::Precision = crate::date::Precision::Smart;
    let mut precision_7: date::Precision = crate::date::Precision::Nanos;
    let mut result_1: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4106() {
    rusty_monitor::set_test_id(4106);
    let mut str_0: &str = "x";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut usize_0: usize = 3790usize;
    let mut char_0: char = '_';
    let mut char_1: char = ',';
    let mut u64_0: u64 = 7252u64;
    let mut u64_1: u64 = 576u64;
    let mut usize_1: usize = 5302usize;
    let mut usize_2: usize = 5834usize;
    let mut str_1: &str = "6pCDnMsZptDs1";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "w2BESqBApOS";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut error_0: duration::Error = crate::duration::Error::Empty;
    let mut precision_0: date::Precision = crate::date::Precision::Millis;
    let mut error_1: date::Error = crate::date::Error::InvalidDigit;
    let mut precision_1: date::Precision = crate::date::Precision::Seconds;
    let mut result_0: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_2_ref_0);
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_1_ref_0);
    let mut error_2: duration::Error = crate::duration::Error::Empty;
    let mut error_3: date::Error = crate::date::Error::InvalidFormat;
    let mut error_4: date::Error = crate::date::Error::InvalidDigit;
    let mut precision_2: date::Precision = crate::date::Precision::Micros;
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_0);
    let mut bool_0: bool = crate::date::is_leap_year(u64_0);
    let mut precision_3: date::Precision = crate::date::Precision::Nanos;
    let mut option_0: std::option::Option<u64> = crate::date::two_digits_inner(char_1, char_0);
    let mut precision_4: date::Precision = crate::date::Precision::Nanos;
    let mut error_5: duration::Error = crate::duration::Error::InvalidCharacter(usize_0);
    let mut error_6: duration::Error = crate::duration::Error::NumberOverflow;
    let mut result_2: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2477() {
    rusty_monitor::set_test_id(2477);
    let mut str_0: &str = "O4bL7T";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut char_0: char = ',';
    let mut char_1: char = '/';
    let mut u8_0: u8 = 19u8;
    let mut u8_1: u8 = 46u8;
    let mut str_1: &str = "qaJfPBy4x6nhmTyE";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut usize_0: usize = 132usize;
    let mut char_2: char = 'I';
    let mut char_3: char = '"';
    let mut str_2: &str = "gEmv";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut usize_1: usize = 1799usize;
    let mut str_3: &str = "AF9Fy";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "06d35O3t1y7";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "RalFopwLTvtEQ7fYoi";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut u8_2: u8 = 9u8;
    let mut u8_3: u8 = 64u8;
    let mut u64_0: u64 = 5103u64;
    let mut u64_1: u64 = 369u64;
    let mut tuple_0: (u64, u64) = (u64_1, u64_0);
    let mut str_6: &str = "kkE6Ovs6JBf";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut precision_0: date::Precision = crate::date::Precision::Smart;
    let mut result_0: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_3, u8_2);
    let mut error_0: date::Error = crate::date::Error::InvalidFormat;
    let mut precision_1: date::Precision = crate::date::Precision::Nanos;
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_5_ref_0);
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_4_ref_0);
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_1);
    let mut result_3: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_3_ref_0);
    let mut error_1: date::Error = crate::date::Error::InvalidFormat;
    let mut error_2: duration::Error = crate::duration::Error::InvalidCharacter(usize_1);
    let mut precision_2: date::Precision = crate::date::Precision::Seconds;
    let mut result_4: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_2_ref_0);
    let mut duration_1: std::time::Duration = std::result::Result::unwrap(result_2);
    let mut option_0: std::option::Option<u64> = crate::date::two_digits_inner(char_3, char_2);
    let mut error_3: date::Error = crate::date::Error::InvalidFormat;
    let mut error_4: duration::Error = crate::duration::Error::NumberExpected(usize_0);
    let mut u64_2: u64 = std::option::Option::unwrap(option_0);
    let mut result_5: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_1_ref_0);
    let mut result_6: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_1, u8_0);
    let mut option_1: std::option::Option<u64> = crate::date::two_digits_inner(char_1, char_0);
    let mut error_5: date::Error = crate::date::Error::InvalidDigit;
    let mut error_6: date::Error = crate::date::Error::InvalidDigit;
    let mut precision_3: date::Precision = crate::date::Precision::Millis;
    let mut result_7: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3889() {
    rusty_monitor::set_test_id(3889);
    let mut usize_0: usize = 857usize;
    let mut u8_0: u8 = 108u8;
    let mut u8_1: u8 = 45u8;
    let mut str_0: &str = "JIjTq6wHq6nFwlI8";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut char_0: char = '[';
    let mut char_1: char = 'C';
    let mut str_1: &str = "mEthzef9FSu";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut u64_0: u64 = 664u64;
    let mut str_2: &str = "07U5McZmeQMPL8q";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bool_0: bool = true;
    let mut bool_0_ref_0: &mut bool = &mut bool_0;
    let mut str_3: &str = "vweU";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "fBinyDOfdLM1TeD2uVC";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "SFLTN";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut error_0: duration::Error = crate::duration::Error::NumberOverflow;
    let mut result_0: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_5_ref_0);
    let mut result_1: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_4_ref_0);
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_3_ref_0);
    let mut error_1: date::Error = crate::date::Error::InvalidDigit;
    let mut result_3: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_1_ref_0);
    let mut option_0: std::option::Option<u64> = crate::date::two_digits_inner(char_1, char_0);
    let mut result_4: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_0_ref_0);
    let mut error_2: date::Error = crate::date::Error::OutOfRange;
    let mut result_5: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_1, u8_0);
    let mut error_3: duration::Error = crate::duration::Error::Empty;
    let mut error_4: duration::Error = crate::duration::Error::NumberExpected(usize_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4491() {
    rusty_monitor::set_test_id(4491);
    let mut u64_0: u64 = 7902u64;
    let mut usize_0: usize = 8933usize;
    let mut usize_1: usize = 1987usize;
    let mut u64_1: u64 = 8724u64;
    let mut usize_2: usize = 4464usize;
    let mut usize_3: usize = 8037usize;
    let mut str_0: &str = "3vo";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u64_2: u64 = 4936u64;
    let mut usize_4: usize = 7853usize;
    let mut usize_5: usize = 2831usize;
    let mut u64_3: u64 = 3900u64;
    let mut usize_6: usize = 4083usize;
    let mut u64_4: u64 = 2538u64;
    let mut usize_7: usize = 5556usize;
    let mut usize_8: usize = 2372usize;
    let mut char_0: char = 'R';
    let mut char_1: char = 's';
    let mut char_2: char = 'q';
    let mut char_3: char = '-';
    let mut option_0: std::option::Option<u64> = crate::date::two_digits_inner(char_3, char_2);
    let mut option_1: std::option::Option<u64> = crate::date::two_digits_inner(char_1, char_0);
    let mut error_0: duration::Error = crate::duration::Error::InvalidCharacter(usize_6);
    let mut error_1: date::Error = crate::date::Error::InvalidFormat;
    let mut bool_0: bool = crate::date::is_leap_year(u64_3);
    let mut error_2: date::Error = crate::date::Error::OutOfRange;
    let mut error_3: duration::Error = crate::duration::Error::NumberOverflow;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_0_ref_0);
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_0);
    let mut precision_0: date::Precision = crate::date::Precision::Nanos;
    let mut precision_1: date::Precision = crate::date::Precision::Millis;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4883() {
    rusty_monitor::set_test_id(4883);
    let mut u64_0: u64 = 8819u64;
    let mut char_0: char = 'I';
    let mut char_1: char = '~';
    let mut u8_0: u8 = 38u8;
    let mut u8_1: u8 = 26u8;
    let mut u64_1: u64 = 1112u64;
    let mut usize_0: usize = 3338usize;
    let mut usize_1: usize = 725usize;
    let mut u8_2: u8 = 63u8;
    let mut u8_3: u8 = 41u8;
    let mut str_0: &str = "x6KjwHrhlz";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u8_4: u8 = 77u8;
    let mut u8_5: u8 = 14u8;
    let mut u64_2: u64 = 8433u64;
    let mut str_1: &str = "Ks5pO5mMcKyPzh1zGS";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut result_0: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_1_ref_0);
    let mut precision_0: date::Precision = crate::date::Precision::Millis;
    let mut bool_0: bool = crate::date::is_leap_year(u64_2);
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_0);
    let mut result_1: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_5, u8_4);
    let mut error_0: date::Error = crate::date::Error::OutOfRange;
    let mut u64_3: u64 = std::result::Result::unwrap(result_1);
    let mut result_2: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_0_ref_0);
    let mut result_3: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_3, u8_2);
    let mut error_1: date::Error = crate::date::Error::InvalidDigit;
    let mut systemtime_1: std::time::SystemTime = std::result::Result::unwrap(result_2);
    let mut result_4: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_1, u8_0);
    let mut option_0: std::option::Option<u64> = crate::date::two_digits_inner(char_1, char_0);
    let mut bool_1: bool = crate::date::is_leap_year(u64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1010() {
    rusty_monitor::set_test_id(1010);
    let mut usize_0: usize = 9572usize;
    let mut str_0: &str = "vV";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut usize_1: usize = 7723usize;
    let mut usize_2: usize = 4621usize;
    let mut u64_0: u64 = 5447u64;
    let mut str_1: &str = "";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bool_0: bool = false;
    let mut bool_0_ref_0: &mut bool = &mut bool_0;
    let mut usize_3: usize = 6654usize;
    let mut usize_4: usize = 7118usize;
    let mut u64_1: u64 = 6561u64;
    let mut u64_2: u64 = 1436u64;
    let mut u64_3: u64 = 5721u64;
    let mut tuple_0: (u64, u64) = (u64_3, u64_2);
    let mut str_2: &str = "8pIlop9npI";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut u64_4: u64 = 4090u64;
    let mut u64_5: u64 = 9517u64;
    let mut tuple_1: (u64, u64) = (u64_5, u64_4);
    let mut str_3: &str = "532xuq7";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "3w8A7zPg5MYLW2";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_4_ref_0);
    let mut error_0: duration::Error = crate::duration::Error::InvalidCharacter(usize_2);
    let mut precision_0: date::Precision = crate::date::Precision::Micros;
    let mut error_1: duration::Error = crate::duration::Error::NumberExpected(usize_1);
    let mut result_1: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_0_ref_0);
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_0);
    let mut error_2: duration::Error = crate::duration::Error::InvalidCharacter(usize_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4792() {
    rusty_monitor::set_test_id(4792);
    let mut u64_0: u64 = 9599u64;
    let mut str_0: &str = "xWca2CrE9lGmC4V";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u8_0: u8 = 93u8;
    let mut u8_1: u8 = 91u8;
    let mut char_0: char = '\r';
    let mut char_1: char = '8';
    let mut str_1: &str = "VrLfy78bup";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut u8_2: u8 = 111u8;
    let mut u8_3: u8 = 6u8;
    let mut u32_0: u32 = 2577u32;
    let mut str_2: &str = "6dCsne0Ej1o";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bool_0: bool = true;
    let mut bool_0_ref_0: &mut bool = &mut bool_0;
    let mut result_0: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_3, u8_2);
    let mut result_1: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_1_ref_0);
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_1);
    let mut precision_0: date::Precision = crate::date::Precision::Seconds;
    let mut precision_1: date::Precision = crate::date::Precision::Millis;
    let mut option_0: std::option::Option<u64> = crate::date::two_digits_inner(char_1, char_0);
    let mut error_0: date::Error = crate::date::Error::InvalidFormat;
    let mut u64_1: u64 = std::option::Option::unwrap(option_0);
    let mut result_2: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_1, u8_0);
    let mut precision_2: date::Precision = crate::date::Precision::Millis;
    let mut error_1: date::Error = crate::date::Error::InvalidDigit;
    let mut result_3: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_0_ref_0);
    let mut u64_2: u64 = std::result::Result::unwrap(result_0);
    let mut bool_1: bool = crate::date::is_leap_year(u64_0);
    let mut error_2: date::Error = crate::date::Error::OutOfRange;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4798() {
    rusty_monitor::set_test_id(4798);
    let mut u32_0: u32 = 1790u32;
    let mut str_0: &str = "pxVZL1crAdqJoYk9dd";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_0: bool = false;
    let mut bool_0_ref_0: &mut bool = &mut bool_0;
    let mut str_1: &str = "3ItnUBkRtSSClIL";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut usize_0: usize = 9528usize;
    let mut usize_1: usize = 1859usize;
    let mut u64_0: u64 = 1561u64;
    let mut u64_1: u64 = 1272u64;
    let mut u64_2: u64 = 4744u64;
    let mut tuple_0: (u64, u64) = (u64_2, u64_1);
    let mut str_2: &str = "tbFGLDQcrwRd";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut usize_2: usize = 5528usize;
    let mut usize_3: usize = 6752usize;
    let mut u64_3: u64 = 3240u64;
    let mut u64_4: u64 = 3847u64;
    let mut u64_5: u64 = 9001u64;
    let mut tuple_1: (u64, u64) = (u64_5, u64_4);
    let mut str_3: &str = "gQhgC3e5FAfv";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut usize_4: usize = 7163usize;
    let mut u64_6: u64 = 8282u64;
    let mut u64_7: u64 = 6142u64;
    let mut tuple_2: (u64, u64) = (u64_7, u64_6);
    let mut str_4: &str = "4FHIs";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut error_0: duration::Error = crate::duration::Error::NumberExpected(usize_4);
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_1_ref_0);
    let mut error_1: date::Error = crate::date::Error::OutOfRange;
    let mut precision_0: date::Precision = crate::date::Precision::Seconds;
    let mut error_2: date::Error = crate::date::Error::OutOfRange;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3231() {
    rusty_monitor::set_test_id(3231);
    let mut str_0: &str = "PKYK9JJp7J7q7La";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut usize_0: usize = 7710usize;
    let mut u64_0: u64 = 1174u64;
    let mut u64_1: u64 = 1414u64;
    let mut tuple_0: (u64, u64) = (u64_1, u64_0);
    let mut str_1: &str = "e31Z9hFeQxk92hGbUP";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "30CcS8kD2yu4O";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut u64_2: u64 = 2855u64;
    let mut str_3: &str = "F49rmFZHO3HWc7P";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut bool_0: bool = true;
    let mut bool_0_ref_0: &mut bool = &mut bool_0;
    let mut u64_3: u64 = 9514u64;
    let mut u64_4: u64 = 8588u64;
    let mut tuple_1: (u64, u64) = (u64_4, u64_3);
    let mut str_4: &str = "Am";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut result_0: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_5_ref_0);
    let mut error_0: duration::Error = crate::duration::Error::Empty;
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_2_ref_0);
    let mut error_1: duration::Error = crate::duration::Error::NumberExpected(usize_0);
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_0_ref_0);
    let mut precision_0: date::Precision = crate::date::Precision::Nanos;
    let mut error_2: date::Error = crate::date::Error::OutOfRange;
    let mut error_3: date::Error = crate::date::Error::OutOfRange;
    let mut precision_1: date::Precision = crate::date::Precision::Millis;
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_0);
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4073() {
    rusty_monitor::set_test_id(4073);
    let mut usize_0: usize = 6342usize;
    let mut usize_1: usize = 5308usize;
    let mut char_0: char = '?';
    let mut char_1: char = 'd';
    let mut str_0: &str = "xXU7ZQyiIZ";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut char_2: char = '\\';
    let mut char_3: char = 'w';
    let mut char_4: char = '\n';
    let mut char_5: char = '2';
    let mut precision_0: date::Precision = crate::date::Precision::Nanos;
    let mut error_0: date::Error = crate::date::Error::InvalidDigit;
    let mut option_0: std::option::Option<u64> = crate::date::two_digits_inner(char_5, char_4);
    let mut option_1: std::option::Option<u64> = crate::date::two_digits_inner(char_3, char_2);
    let mut precision_1: date::Precision = crate::date::Precision::Smart;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_1_ref_0);
    let mut precision_2: date::Precision = crate::date::Precision::Nanos;
    let mut u64_0: u64 = std::option::Option::unwrap(option_0);
    let mut precision_3: date::Precision = crate::date::Precision::Nanos;
    let mut error_1: date::Error = crate::date::Error::InvalidFormat;
    let mut result_1: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_0_ref_0);
    let mut error_2: duration::Error = crate::duration::Error::NumberOverflow;
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_0);
    let mut option_2: std::option::Option<u64> = crate::date::two_digits_inner(char_1, char_0);
    let mut error_3: duration::Error = crate::duration::Error::InvalidCharacter(usize_1);
    let mut u64_1: u64 = std::option::Option::unwrap(option_1);
    let mut precision_4: date::Precision = crate::date::Precision::Micros;
    let mut precision_5: date::Precision = crate::date::Precision::Smart;
    let mut precision_6: date::Precision = crate::date::Precision::Seconds;
    let mut error_4: date::Error = crate::date::Error::InvalidDigit;
    let mut error_5: duration::Error = crate::duration::Error::InvalidCharacter(usize_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4162() {
    rusty_monitor::set_test_id(4162);
    let mut usize_0: usize = 3971usize;
    let mut str_0: &str = "JNUoC";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "7Xo";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "pITPvXu5zu";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "1JDTNZ2vrg8";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut usize_1: usize = 4014usize;
    let mut error_0: date::Error = crate::date::Error::InvalidDigit;
    let mut precision_0: date::Precision = crate::date::Precision::Smart;
    let mut error_1: duration::Error = crate::duration::Error::Empty;
    let mut error_2: duration::Error = crate::duration::Error::InvalidCharacter(usize_1);
    let mut result_0: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_3_ref_0);
    let mut precision_1: date::Precision = crate::date::Precision::Micros;
    let mut error_3: duration::Error = crate::duration::Error::NumberOverflow;
    let mut result_1: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_2_ref_0);
    let mut error_4: duration::Error = crate::duration::Error::NumberOverflow;
    let mut precision_2: date::Precision = crate::date::Precision::Seconds;
    let mut error_5: duration::Error = crate::duration::Error::NumberOverflow;
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_1);
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_1_ref_0);
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_2);
    let mut precision_3: date::Precision = crate::date::Precision::Seconds;
    let mut result_3: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_0_ref_0);
    let mut error_6: duration::Error = crate::duration::Error::Empty;
    let mut precision_4: date::Precision = crate::date::Precision::Seconds;
    let mut error_7: duration::Error = crate::duration::Error::InvalidCharacter(usize_0);
    let mut error_8: date::Error = crate::date::Error::InvalidDigit;
    let mut precision_5: date::Precision = crate::date::Precision::Millis;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1037() {
    rusty_monitor::set_test_id(1037);
    let mut usize_0: usize = 917usize;
    let mut str_0: &str = "74XsTUp7EJ";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut char_0: char = ' ';
    let mut char_1: char = '8';
    let mut str_1: &str = "";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut u64_0: u64 = 21u64;
    let mut str_2: &str = "MPFoG";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "kBKHLSTJtIEc2rxUFa";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut char_2: char = '"';
    let mut char_3: char = '7';
    let mut u64_1: u64 = 7097u64;
    let mut u64_2: u64 = 4065u64;
    let mut tuple_0: (u64, u64) = (u64_2, u64_1);
    let mut str_4: &str = "RnDdw62KaBgTv";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut precision_0: date::Precision = crate::date::Precision::Millis;
    let mut option_0: std::option::Option<u64> = crate::date::two_digits_inner(char_3, char_2);
    let mut result_0: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_3_ref_0);
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_2_ref_0);
    let mut bool_0: bool = crate::date::is_leap_year(u64_0);
    let mut error_0: duration::Error = crate::duration::Error::Empty;
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_1_ref_0);
    let mut precision_1: date::Precision = crate::date::Precision::Seconds;
    let mut option_1: std::option::Option<u64> = crate::date::two_digits_inner(char_1, char_0);
    let mut result_3: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_0_ref_0);
    let mut error_1: duration::Error = crate::duration::Error::NumberExpected(usize_0);
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4171() {
    rusty_monitor::set_test_id(4171);
    let mut u8_0: u8 = 23u8;
    let mut u8_1: u8 = 116u8;
    let mut u8_2: u8 = 82u8;
    let mut u8_3: u8 = 117u8;
    let mut u8_4: u8 = 40u8;
    let mut u8_5: u8 = 35u8;
    let mut usize_0: usize = 4230usize;
    let mut u64_0: u64 = 7544u64;
    let mut str_0: &str = "9JL0WZOz6LBhMTun";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_0: bool = false;
    let mut bool_0_ref_0: &mut bool = &mut bool_0;
    let mut u64_1: u64 = 2388u64;
    let mut u64_2: u64 = 2129u64;
    let mut tuple_0: (u64, u64) = (u64_2, u64_1);
    let mut str_1: &str = "bHRgdBJNI";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut usize_1: usize = 5849usize;
    let mut usize_2: usize = 5783usize;
    let mut u64_3: u64 = 3978u64;
    let mut u64_4: u64 = 2081u64;
    let mut u64_5: u64 = 8856u64;
    let mut tuple_1: (u64, u64) = (u64_5, u64_4);
    let mut str_2: &str = "oRl4CltSbpS7";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut error_0: duration::Error = crate::duration::Error::InvalidCharacter(usize_0);
    let mut result_0: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_5, u8_4);
    let mut result_1: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_3, u8_2);
    let mut result_2: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_1, u8_0);
    let mut u64_6: u64 = std::result::Result::unwrap(result_2);
    let mut precision_0: date::Precision = crate::date::Precision::Seconds;
    panic!("From RustyUnit with love");
}
}