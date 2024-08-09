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
fn rusty_test_2610() {
    rusty_monitor::set_test_id(2610);
    let mut u64_0: u64 = 6772u64;
    let mut usize_0: usize = 762usize;
    let mut u64_1: u64 = 64u64;
    let mut usize_1: usize = 7736usize;
    let mut usize_2: usize = 8159usize;
    let mut u64_2: u64 = 1648u64;
    let mut str_0: &str = "x7pekI9QH3TTT";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut usize_3: usize = 2848usize;
    let mut usize_4: usize = 8657usize;
    let mut u32_0: u32 = 1735u32;
    let mut str_1: &str = "O5X41SL";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bool_0: bool = true;
    let mut bool_0_ref_0: &mut bool = &mut bool_0;
    let mut error_0: duration::Error = crate::duration::Error::UnknownUnit {start: usize_4, end: usize_3, unit: string_0, value: u64_2};
    let mut precision_0: date::Precision = crate::date::Precision::Seconds;
    let mut precision_1: date::Precision = crate::date::Precision::Nanos;
    let mut precision_2: date::Precision = crate::date::Precision::Micros;
    let mut error_1: date::Error = crate::date::Error::InvalidDigit;
    let mut precision_3: date::Precision = crate::date::Precision::Seconds;
    let mut error_2: duration::Error = crate::duration::Error::NumberOverflow;
    let mut error_3: duration::Error = crate::duration::Error::Empty;
    let mut error_4: date::Error = crate::date::Error::OutOfRange;
    let mut error_5: duration::Error = crate::duration::Error::NumberExpected(usize_0);
    let mut error_6: date::Error = crate::date::Error::InvalidDigit;
    let mut precision_4: date::Precision = crate::date::Precision::Nanos;
    let mut precision_5: date::Precision = crate::date::Precision::Nanos;
    let mut precision_6: date::Precision = crate::date::Precision::Smart;
    let mut bool_1: bool = crate::date::is_leap_year(u64_0);
    let mut error_7: date::Error = crate::date::Error::OutOfRange;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3777() {
    rusty_monitor::set_test_id(3777);
    let mut usize_0: usize = 2264usize;
    let mut u8_0: u8 = 116u8;
    let mut u8_1: u8 = 86u8;
    let mut usize_1: usize = 8281usize;
    let mut str_0: &str = "djO69v";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut usize_2: usize = 7736usize;
    let mut u64_0: u64 = 47u64;
    let mut char_0: char = '&';
    let mut char_1: char = '7';
    let mut u8_2: u8 = 56u8;
    let mut u8_3: u8 = 71u8;
    let mut u64_1: u64 = 1721u64;
    let mut error_0: date::Error = crate::date::Error::InvalidFormat;
    let mut bool_0: bool = crate::date::is_leap_year(u64_1);
    let mut error_1: duration::Error = crate::duration::Error::Empty;
    let mut error_2: duration::Error = crate::duration::Error::NumberOverflow;
    let mut error_3: duration::Error = crate::duration::Error::NumberOverflow;
    let mut result_0: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_3, u8_2);
    let mut option_0: std::option::Option<u64> = crate::date::two_digits_inner(char_1, char_0);
    let mut bool_1: bool = crate::date::is_leap_year(u64_0);
    let mut precision_0: date::Precision = crate::date::Precision::Micros;
    let mut error_4: duration::Error = crate::duration::Error::InvalidCharacter(usize_2);
    let mut precision_1: date::Precision = crate::date::Precision::Nanos;
    let mut u64_2: u64 = std::result::Result::unwrap(result_0);
    let mut result_1: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_0_ref_0);
    let mut error_5: date::Error = crate::date::Error::OutOfRange;
    let mut error_6: duration::Error = crate::duration::Error::InvalidCharacter(usize_1);
    let mut result_2: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_1, u8_0);
    let mut error_7: duration::Error = crate::duration::Error::InvalidCharacter(usize_0);
    let mut precision_2: date::Precision = crate::date::Precision::Seconds;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4731() {
    rusty_monitor::set_test_id(4731);
    let mut str_0: &str = "ZYkWaxOhMo";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "w1T98BA3oNVYAM7";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut u64_0: u64 = 2253u64;
    let mut str_2: &str = "lLGYm3f7EWFdpREJ8m";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut u64_1: u64 = 9150u64;
    let mut str_3: &str = "jTnVVGaxW8x75gJhYnD";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut usize_0: usize = 322usize;
    let mut usize_1: usize = 8437usize;
    let mut u8_0: u8 = 21u8;
    let mut u8_1: u8 = 43u8;
    let mut result_0: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_1, u8_0);
    let mut error_0: duration::Error = crate::duration::Error::NumberExpected(usize_1);
    let mut error_1: duration::Error = crate::duration::Error::NumberExpected(usize_0);
    let mut error_2: date::Error = crate::date::Error::OutOfRange;
    let mut u64_2: u64 = std::result::Result::unwrap(result_0);
    let mut error_3: duration::Error = crate::duration::Error::Empty;
    let mut result_1: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_3_ref_0);
    let mut bool_0: bool = crate::date::is_leap_year(u64_1);
    let mut precision_0: date::Precision = crate::date::Precision::Nanos;
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_1);
    let mut precision_1: date::Precision = crate::date::Precision::Smart;
    let mut precision_2: date::Precision = crate::date::Precision::Smart;
    let mut result_2: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_2_ref_0);
    let mut precision_3: date::Precision = crate::date::Precision::Smart;
    let mut bool_1: bool = crate::date::is_leap_year(u64_0);
    let mut result_3: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_1_ref_0);
    let mut result_4: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3350() {
    rusty_monitor::set_test_id(3350);
    let mut str_0: &str = "MMEPUKK6Djd7dJKcvj";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "I3";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut u8_0: u8 = 68u8;
    let mut u8_1: u8 = 35u8;
    let mut usize_0: usize = 6209usize;
    let mut char_0: char = 'Y';
    let mut char_1: char = 'u';
    let mut str_2: &str = "iHSiKlqPAw7Z";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut char_2: char = 'F';
    let mut char_3: char = 'd';
    let mut char_4: char = '4';
    let mut char_5: char = 'A';
    let mut str_3: &str = "ho3";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut usize_1: usize = 9934usize;
    let mut usize_2: usize = 1172usize;
    let mut usize_3: usize = 6464usize;
    let mut u64_0: u64 = 886u64;
    let mut usize_4: usize = 4364usize;
    let mut usize_5: usize = 2922usize;
    let mut str_4: &str = "xWuQoT";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "dsIIMFwWIihRSE4jcF";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut usize_6: usize = 5973usize;
    let mut str_6: &str = "26Uc4J2K8xVkBD6yv";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7: &str = "vL";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_8: &str = "7s";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_8_ref_0);
    let mut error_0: date::Error = crate::date::Error::InvalidDigit;
    let mut precision_0: date::Precision = crate::date::Precision::Micros;
    let mut result_1: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_7_ref_0);
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_1);
    let mut result_2: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_6_ref_0);
    let mut systemtime_1: std::time::SystemTime = std::result::Result::unwrap(result_2);
    let mut error_1: duration::Error = crate::duration::Error::InvalidCharacter(usize_6);
    let mut result_3: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_5_ref_0);
    let mut result_4: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_4_ref_0);
    let mut error_2: duration::Error = crate::duration::Error::NumberExpected(usize_3);
    let mut error_3: duration::Error = crate::duration::Error::NumberExpected(usize_2);
    let mut precision_1: date::Precision = crate::date::Precision::Smart;
    let mut error_4: duration::Error = crate::duration::Error::InvalidCharacter(usize_1);
    let mut result_5: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_3_ref_0);
    let mut option_0: std::option::Option<u64> = crate::date::two_digits_inner(char_5, char_4);
    let mut option_1: std::option::Option<u64> = crate::date::two_digits_inner(char_3, char_2);
    let mut precision_2: date::Precision = crate::date::Precision::Micros;
    let mut result_6: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_2_ref_0);
    let mut error_5: duration::Error = crate::duration::Error::NumberOverflow;
    let mut systemtime_2: std::time::SystemTime = std::result::Result::unwrap(result_6);
    let mut systemtime_3: std::time::SystemTime = std::result::Result::unwrap(result_5);
    let mut systemtime_4: std::time::SystemTime = std::result::Result::unwrap(result_4);
    let mut error_6: date::Error = crate::date::Error::InvalidFormat;
    let mut error_7: date::Error = crate::date::Error::InvalidFormat;
    let mut option_2: std::option::Option<u64> = crate::date::two_digits_inner(char_1, char_0);
    let mut precision_3: date::Precision = crate::date::Precision::Smart;
    let mut precision_4: date::Precision = crate::date::Precision::Seconds;
    let mut error_8: duration::Error = crate::duration::Error::Empty;
    let mut precision_5: date::Precision = crate::date::Precision::Millis;
    let mut error_9: duration::Error = crate::duration::Error::NumberExpected(usize_0);
    let mut precision_6: date::Precision = crate::date::Precision::Smart;
    let mut result_7: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_1, u8_0);
    let mut result_8: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_1_ref_0);
    let mut precision_7: date::Precision = crate::date::Precision::Seconds;
    let mut result_9: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4158() {
    rusty_monitor::set_test_id(4158);
    let mut char_0: char = 'x';
    let mut char_1: char = '"';
    let mut str_0: &str = "PO6LXyJbpkzg";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut usize_0: usize = 7821usize;
    let mut str_1: &str = "HzCBnRnKZsexn";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "oB7XGhSyOBpqZH";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "88mrYPQLSOEbW";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut usize_1: usize = 4889usize;
    let mut u8_0: u8 = 58u8;
    let mut u8_1: u8 = 115u8;
    let mut usize_2: usize = 3401usize;
    let mut char_2: char = '>';
    let mut char_3: char = 'e';
    let mut usize_3: usize = 9118usize;
    let mut char_4: char = '\'';
    let mut char_5: char = 'e';
    let mut str_4: &str = "AEoyjShPkmZzB1oX";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut u64_0: u64 = 2179u64;
    let mut str_5: &str = "SuxfhEC";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut bool_0: bool = true;
    let mut bool_0_ref_0: &mut bool = &mut bool_0;
    let mut str_6: &str = "u7Xykpq36LLeTNy2";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7: &str = "1d1Wa";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut error_0: duration::Error = crate::duration::Error::NumberOverflow;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_7_ref_0);
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_0);
    let mut result_1: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_6_ref_0);
    let mut precision_0: date::Precision = crate::date::Precision::Seconds;
    let mut error_1: duration::Error = crate::duration::Error::NumberOverflow;
    let mut error_2: duration::Error = crate::duration::Error::Empty;
    let mut precision_1: date::Precision = crate::date::Precision::Seconds;
    let mut precision_2: date::Precision = crate::date::Precision::Millis;
    let mut error_3: date::Error = crate::date::Error::InvalidDigit;
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_1);
    let mut result_2: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_4_ref_0);
    let mut systemtime_1: std::time::SystemTime = std::result::Result::unwrap(result_2);
    let mut error_4: duration::Error = crate::duration::Error::NumberOverflow;
    let mut option_0: std::option::Option<u64> = crate::date::two_digits_inner(char_5, char_4);
    let mut error_5: duration::Error = crate::duration::Error::NumberExpected(usize_3);
    let mut u64_1: u64 = std::option::Option::unwrap(option_0);
    let mut option_1: std::option::Option<u64> = crate::date::two_digits_inner(char_3, char_2);
    let mut error_6: duration::Error = crate::duration::Error::InvalidCharacter(usize_2);
    let mut result_3: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_1, u8_0);
    let mut error_7: duration::Error = crate::duration::Error::InvalidCharacter(usize_1);
    let mut result_4: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_3_ref_0);
    let mut error_8: date::Error = crate::date::Error::InvalidFormat;
    let mut u64_2: u64 = std::option::Option::unwrap(option_1);
    let mut precision_3: date::Precision = crate::date::Precision::Millis;
    let mut result_5: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_2_ref_0);
    let mut error_9: duration::Error = crate::duration::Error::NumberOverflow;
    let mut result_6: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_1_ref_0);
    let mut error_10: date::Error = crate::date::Error::InvalidFormat;
    let mut precision_4: date::Precision = crate::date::Precision::Micros;
    let mut error_11: duration::Error = crate::duration::Error::Empty;
    let mut systemtime_2: std::time::SystemTime = std::result::Result::unwrap(result_6);
    let mut systemtime_3: std::time::SystemTime = std::result::Result::unwrap(result_4);
    let mut precision_5: date::Precision = crate::date::Precision::Millis;
    let mut error_12: date::Error = crate::date::Error::InvalidDigit;
    let mut duration_1: std::time::Duration = std::result::Result::unwrap(result_5);
    let mut error_13: duration::Error = crate::duration::Error::InvalidCharacter(usize_0);
    let mut result_7: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_0_ref_0);
    let mut option_2: std::option::Option<u64> = crate::date::two_digits_inner(char_1, char_0);
    let mut precision_6: date::Precision = crate::date::Precision::Seconds;
    let mut error_14: date::Error = crate::date::Error::InvalidFormat;
    let mut duration_2: std::time::Duration = std::result::Result::unwrap(result_7);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3481() {
    rusty_monitor::set_test_id(3481);
    let mut u64_0: u64 = 9380u64;
    let mut char_0: char = 'z';
    let mut char_1: char = 'b';
    let mut char_2: char = ')';
    let mut char_3: char = 'p';
    let mut u64_1: u64 = 5411u64;
    let mut u8_0: u8 = 78u8;
    let mut u8_1: u8 = 118u8;
    let mut str_0: &str = "O5YgvOzUYT1JK";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut char_4: char = ':';
    let mut char_5: char = 'Z';
    let mut str_1: &str = "2";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut u64_2: u64 = 6914u64;
    let mut usize_0: usize = 2083usize;
    let mut u8_2: u8 = 14u8;
    let mut u8_3: u8 = 126u8;
    let mut str_2: &str = "35w";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut u8_4: u8 = 20u8;
    let mut u8_5: u8 = 118u8;
    let mut char_6: char = 'v';
    let mut char_7: char = 'S';
    let mut u64_3: u64 = 2223u64;
    let mut usize_1: usize = 7274usize;
    let mut usize_2: usize = 6983usize;
    let mut u64_4: u64 = 651u64;
    let mut str_3: &str = "MT";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut bool_0: bool = true;
    let mut bool_0_ref_0: &mut bool = &mut bool_0;
    let mut error_0: duration::Error = crate::duration::Error::NumberOverflow;
    let mut error_1: date::Error = crate::date::Error::InvalidFormat;
    let mut precision_0: date::Precision = crate::date::Precision::Micros;
    let mut option_0: std::option::Option<u64> = crate::date::two_digits_inner(char_7, char_6);
    let mut result_0: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_5, u8_4);
    let mut precision_1: date::Precision = crate::date::Precision::Micros;
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_2_ref_0);
    let mut result_2: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_3, u8_2);
    let mut error_2: date::Error = crate::date::Error::InvalidDigit;
    let mut u64_5: u64 = std::option::Option::unwrap(option_0);
    let mut error_3: duration::Error = crate::duration::Error::NumberExpected(usize_0);
    let mut precision_2: date::Precision = crate::date::Precision::Nanos;
    let mut error_4: duration::Error = crate::duration::Error::Empty;
    let mut precision_3: date::Precision = crate::date::Precision::Nanos;
    let mut u64_6: u64 = std::result::Result::unwrap(result_2);
    let mut u64_7: u64 = std::result::Result::unwrap(result_0);
    let mut precision_4: date::Precision = crate::date::Precision::Micros;
    let mut bool_1: bool = crate::date::is_leap_year(u64_2);
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_1);
    let mut precision_5: date::Precision = crate::date::Precision::Seconds;
    let mut precision_6: date::Precision = crate::date::Precision::Millis;
    let mut error_5: date::Error = crate::date::Error::InvalidDigit;
    let mut precision_7: date::Precision = crate::date::Precision::Smart;
    let mut error_6: date::Error = crate::date::Error::InvalidFormat;
    let mut precision_8: date::Precision = crate::date::Precision::Smart;
    let mut error_7: date::Error = crate::date::Error::OutOfRange;
    let mut result_3: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_1_ref_0);
    let mut precision_9: date::Precision = crate::date::Precision::Millis;
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_3);
    let mut error_8: date::Error = crate::date::Error::InvalidDigit;
    let mut option_1: std::option::Option<u64> = crate::date::two_digits_inner(char_5, char_4);
    let mut result_4: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_0_ref_0);
    let mut precision_10: date::Precision = crate::date::Precision::Micros;
    let mut error_9: date::Error = crate::date::Error::InvalidFormat;
    let mut result_5: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_1, u8_0);
    let mut bool_2: bool = crate::date::is_leap_year(u64_1);
    let mut u64_8: u64 = std::result::Result::unwrap(result_5);
    let mut precision_11: date::Precision = crate::date::Precision::Micros;
    let mut error_10: duration::Error = crate::duration::Error::NumberOverflow;
    let mut u64_9: u64 = std::option::Option::unwrap(option_1);
    let mut systemtime_1: std::time::SystemTime = std::result::Result::unwrap(result_4);
    let mut precision_12: date::Precision = crate::date::Precision::Micros;
    let mut precision_13: date::Precision = crate::date::Precision::Micros;
    let mut option_2: std::option::Option<u64> = crate::date::two_digits_inner(char_3, char_2);
    let mut option_3: std::option::Option<u64> = crate::date::two_digits_inner(char_1, char_0);
    let mut error_11: duration::Error = crate::duration::Error::NumberOverflow;
    let mut bool_3: bool = crate::date::is_leap_year(u64_0);
    let mut error_12: date::Error = crate::date::Error::InvalidFormat;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2109() {
    rusty_monitor::set_test_id(2109);
    let mut char_0: char = 'o';
    let mut char_1: char = ',';
    let mut usize_0: usize = 802usize;
    let mut str_0: &str = "vZ9QKtto43q";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u64_0: u64 = 6498u64;
    let mut str_1: &str = "zJ";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut usize_1: usize = 6648usize;
    let mut u64_1: u64 = 8755u64;
    let mut str_2: &str = "IQZcMd";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "5y0S4JjPQx";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut u32_0: u32 = 9908u32;
    let mut str_4: &str = "qo";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut bool_0: bool = true;
    let mut bool_0_ref_0: &mut bool = &mut bool_0;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_3_ref_0);
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_2_ref_0);
    let mut bool_1: bool = crate::date::is_leap_year(u64_1);
    let mut error_0: duration::Error = crate::duration::Error::NumberExpected(usize_1);
    let mut error_1: duration::Error = crate::duration::Error::NumberOverflow;
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_1_ref_0);
    let mut precision_0: date::Precision = crate::date::Precision::Seconds;
    let mut precision_1: date::Precision = crate::date::Precision::Millis;
    let mut bool_2: bool = crate::date::is_leap_year(u64_0);
    let mut error_2: duration::Error = crate::duration::Error::NumberOverflow;
    let mut error_3: date::Error = crate::date::Error::OutOfRange;
    let mut error_4: date::Error = crate::date::Error::InvalidFormat;
    let mut result_3: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_0_ref_0);
    let mut error_5: duration::Error = crate::duration::Error::InvalidCharacter(usize_0);
    let mut option_0: std::option::Option<u64> = crate::date::two_digits_inner(char_1, char_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3747() {
    rusty_monitor::set_test_id(3747);
    let mut usize_0: usize = 3723usize;
    let mut u64_0: u64 = 3082u64;
    let mut u64_1: u64 = 6159u64;
    let mut usize_1: usize = 917usize;
    let mut usize_2: usize = 3852usize;
    let mut usize_3: usize = 6606usize;
    let mut str_0: &str = "rJpgKIiPQD";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut char_0: char = ' ';
    let mut char_1: char = 'T';
    let mut u8_0: u8 = 117u8;
    let mut u8_1: u8 = 47u8;
    let mut error_0: date::Error = crate::date::Error::InvalidFormat;
    let mut result_0: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_1, u8_0);
    let mut option_0: std::option::Option<u64> = crate::date::two_digits_inner(char_1, char_0);
    let mut precision_0: date::Precision = crate::date::Precision::Nanos;
    let mut precision_1: date::Precision = crate::date::Precision::Smart;
    let mut error_1: date::Error = crate::date::Error::InvalidFormat;
    let mut precision_2: date::Precision = crate::date::Precision::Smart;
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_1_ref_0);
    let mut precision_3: date::Precision = crate::date::Precision::Seconds;
    let mut result_2: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_0_ref_0);
    let mut error_2: duration::Error = crate::duration::Error::InvalidCharacter(usize_3);
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_1);
    let mut precision_4: date::Precision = crate::date::Precision::Seconds;
    let mut precision_5: date::Precision = crate::date::Precision::Nanos;
    let mut bool_0: bool = crate::date::is_leap_year(u64_0);
    let mut error_3: duration::Error = crate::duration::Error::Empty;
    let mut error_4: duration::Error = crate::duration::Error::InvalidCharacter(usize_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3855() {
    rusty_monitor::set_test_id(3855);
    let mut usize_0: usize = 6649usize;
    let mut u8_0: u8 = 38u8;
    let mut u8_1: u8 = 109u8;
    let mut str_0: &str = "uctefAzqkum0";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut char_0: char = '6';
    let mut char_1: char = '1';
    let mut usize_1: usize = 1580usize;
    let mut u64_0: u64 = 6695u64;
    let mut u64_1: u64 = 381u64;
    let mut tuple_0: (u64, u64) = (u64_1, u64_0);
    let mut str_1: &str = "aVhy";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut u64_2: u64 = 762u64;
    let mut u64_3: u64 = 7298u64;
    let mut tuple_1: (u64, u64) = (u64_3, u64_2);
    let mut str_2: &str = "JClI7FGPy";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut error_0: duration::Error = crate::duration::Error::InvalidCharacter(usize_1);
    let mut precision_0: date::Precision = crate::date::Precision::Seconds;
    let mut option_0: std::option::Option<u64> = crate::date::two_digits_inner(char_1, char_0);
    let mut result_0: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_0_ref_0);
    let mut u64_4: u64 = std::option::Option::unwrap(option_0);
    let mut error_1: date::Error = crate::date::Error::InvalidFormat;
    let mut precision_1: date::Precision = crate::date::Precision::Smart;
    let mut precision_2: date::Precision = crate::date::Precision::Nanos;
    let mut error_2: duration::Error = crate::duration::Error::Empty;
    let mut result_1: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_1, u8_0);
    let mut error_3: duration::Error = crate::duration::Error::InvalidCharacter(usize_0);
    let mut error_4: date::Error = crate::date::Error::OutOfRange;
    let mut precision_3: date::Precision = crate::date::Precision::Nanos;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4805() {
    rusty_monitor::set_test_id(4805);
    let mut u8_0: u8 = 7u8;
    let mut u8_1: u8 = 62u8;
    let mut char_0: char = 'z';
    let mut char_1: char = '+';
    let mut u64_0: u64 = 5703u64;
    let mut usize_0: usize = 1608usize;
    let mut usize_1: usize = 1527usize;
    let mut char_2: char = 'N';
    let mut char_3: char = '4';
    let mut char_4: char = 'y';
    let mut char_5: char = 'y';
    let mut str_0: &str = "e";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut result_0: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_0_ref_0);
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_0);
    let mut error_0: duration::Error = crate::duration::Error::Empty;
    let mut option_0: std::option::Option<u64> = crate::date::two_digits_inner(char_5, char_4);
    let mut u64_1: u64 = std::option::Option::unwrap(option_0);
    let mut precision_0: date::Precision = crate::date::Precision::Smart;
    let mut error_1: date::Error = crate::date::Error::OutOfRange;
    let mut error_2: date::Error = crate::date::Error::OutOfRange;
    let mut option_1: std::option::Option<u64> = crate::date::two_digits_inner(char_3, char_2);
    let mut precision_1: date::Precision = crate::date::Precision::Millis;
    let mut u64_2: u64 = std::option::Option::unwrap(option_1);
    let mut precision_2: date::Precision = crate::date::Precision::Nanos;
    let mut option_2: std::option::Option<u64> = crate::date::two_digits_inner(char_1, char_0);
    let mut u64_3: u64 = std::option::Option::unwrap(option_2);
    let mut precision_3: date::Precision = crate::date::Precision::Nanos;
    let mut result_1: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_1, u8_0);
    let mut error_3: date::Error = crate::date::Error::InvalidFormat;
    let mut error_4: date::Error = crate::date::Error::InvalidFormat;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2012() {
    rusty_monitor::set_test_id(2012);
    let mut str_0: &str = "1dHnUgQxiy";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "okpfB6SLc0";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "G5Pcn";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut usize_0: usize = 6741usize;
    let mut usize_1: usize = 988usize;
    let mut u64_0: u64 = 6052u64;
    let mut u64_1: u64 = 3026u64;
    let mut u64_2: u64 = 5458u64;
    let mut tuple_0: (u64, u64) = (u64_2, u64_1);
    let mut str_3: &str = "dIxCzl2Yok";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut u64_3: u64 = 9218u64;
    let mut str_4: &str = "pw6hidS1QLa0hcs";
    let mut string_0: std::string::String = std::string::String::from(str_4);
    let mut usize_2: usize = 5902usize;
    let mut usize_3: usize = 6544usize;
    let mut str_5: &str = "JrDvApxE4WOnoCsfo3";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "4ja7tVcVlAJGCyijpcy";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut result_0: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_6_ref_0);
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_5_ref_0);
    let mut error_0: duration::Error = crate::duration::Error::UnknownUnit {start: usize_3, end: usize_2, unit: string_0, value: u64_3};
    let mut result_2: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_2_ref_0);
    let mut result_3: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_1_ref_0);
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_3);
    let mut result_4: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_0_ref_0);
    let mut systemtime_1: std::time::SystemTime = std::result::Result::unwrap(result_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3272() {
    rusty_monitor::set_test_id(3272);
    let mut usize_0: usize = 1486usize;
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut usize_1: usize = 7486usize;
    let mut u64_0: u64 = 5834u64;
    let mut u64_1: u64 = 4253u64;
    let mut str_1: &str = "3q4zDzg7JUKE";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bool_0: bool = true;
    let mut bool_0_ref_0: &mut bool = &mut bool_0;
    let mut str_2: &str = "9K2Axxr4MLe";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut u32_0: u32 = 9138u32;
    let mut str_3: &str = "cylWWZP";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut bool_1: bool = false;
    let mut bool_1_ref_0: &mut bool = &mut bool_1;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_2_ref_0);
    let mut error_0: date::Error = crate::date::Error::InvalidFormat;
    let mut precision_0: date::Precision = crate::date::Precision::Millis;
    let mut precision_1: date::Precision = crate::date::Precision::Smart;
    let mut bool_2: bool = crate::date::is_leap_year(u64_0);
    let mut precision_2: date::Precision = crate::date::Precision::Micros;
    let mut precision_3: date::Precision = crate::date::Precision::Smart;
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_0);
    let mut error_1: duration::Error = crate::duration::Error::InvalidCharacter(usize_1);
    let mut result_1: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_0_ref_0);
    let mut error_2: date::Error = crate::date::Error::InvalidFormat;
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_1);
    let mut error_3: duration::Error = crate::duration::Error::InvalidCharacter(usize_0);
    let mut precision_4: date::Precision = crate::date::Precision::Smart;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4838() {
    rusty_monitor::set_test_id(4838);
    let mut char_0: char = 'G';
    let mut char_1: char = '8';
    let mut u64_0: u64 = 4574u64;
    let mut usize_0: usize = 8152usize;
    let mut usize_1: usize = 1391usize;
    let mut u64_1: u64 = 1168u64;
    let mut char_2: char = '^';
    let mut char_3: char = 'M';
    let mut str_0: &str = "WozomO";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut usize_2: usize = 6350usize;
    let mut precision_0: date::Precision = crate::date::Precision::Millis;
    let mut precision_1: date::Precision = crate::date::Precision::Nanos;
    let mut error_0: duration::Error = crate::duration::Error::NumberExpected(usize_2);
    let mut error_1: date::Error = crate::date::Error::InvalidDigit;
    let mut result_0: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_1_ref_0);
    let mut error_2: date::Error = crate::date::Error::InvalidDigit;
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_0_ref_0);
    let mut precision_2: date::Precision = crate::date::Precision::Micros;
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_1);
    let mut option_0: std::option::Option<u64> = crate::date::two_digits_inner(char_3, char_2);
    let mut precision_3: date::Precision = crate::date::Precision::Nanos;
    let mut bool_0: bool = crate::date::is_leap_year(u64_1);
    let mut precision_4: date::Precision = crate::date::Precision::Micros;
    let mut precision_5: date::Precision = crate::date::Precision::Seconds;
    let mut error_3: date::Error = crate::date::Error::InvalidFormat;
    let mut option_1: std::option::Option<u64> = crate::date::two_digits_inner(char_1, char_0);
    let mut precision_6: date::Precision = crate::date::Precision::Seconds;
    let mut precision_7: date::Precision = crate::date::Precision::Nanos;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3785() {
    rusty_monitor::set_test_id(3785);
    let mut str_0: &str = "cyiyUwu";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "9UW2GwOKZ3Jc4Ozflq";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut usize_0: usize = 9786usize;
    let mut str_2: &str = "3ZZLeKgwQ";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut usize_1: usize = 4652usize;
    let mut u32_0: u32 = 2004u32;
    let mut str_3: &str = "vI2LfOVzbZ3zGA";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut bool_0: bool = false;
    let mut bool_0_ref_0: &mut bool = &mut bool_0;
    let mut u64_0: u64 = 7777u64;
    let mut str_4: &str = "i2ux";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut bool_1: bool = true;
    let mut bool_1_ref_0: &mut bool = &mut bool_1;
    let mut error_0: date::Error = crate::date::Error::InvalidFormat;
    let mut error_1: duration::Error = crate::duration::Error::InvalidCharacter(usize_1);
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_2_ref_0);
    let mut precision_0: date::Precision = crate::date::Precision::Seconds;
    let mut error_2: duration::Error = crate::duration::Error::NumberExpected(usize_0);
    let mut precision_1: date::Precision = crate::date::Precision::Smart;
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_0);
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_1_ref_0);
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_0_ref_0);
    let mut duration_1: std::time::Duration = std::result::Result::unwrap(result_2);
    let mut error_3: duration::Error = crate::duration::Error::Empty;
    let mut precision_2: date::Precision = crate::date::Precision::Millis;
    let mut precision_3: date::Precision = crate::date::Precision::Micros;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2169() {
    rusty_monitor::set_test_id(2169);
    let mut u64_0: u64 = 2930u64;
    let mut usize_0: usize = 3010usize;
    let mut usize_1: usize = 3496usize;
    let mut str_0: &str = "bT2PbGnzrvuSAeMPknX";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u8_0: u8 = 88u8;
    let mut u8_1: u8 = 6u8;
    let mut u8_2: u8 = 48u8;
    let mut u8_3: u8 = 50u8;
    let mut str_1: &str = "99ouQcJ6s";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut usize_2: usize = 8575usize;
    let mut usize_3: usize = 2315usize;
    let mut usize_4: usize = 3163usize;
    let mut usize_5: usize = 6223usize;
    let mut u64_1: u64 = 4163u64;
    let mut u64_2: u64 = 7958u64;
    let mut tuple_0: (u64, u64) = (u64_2, u64_1);
    let mut str_2: &str = "NjqA5lBKFw";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut precision_0: date::Precision = crate::date::Precision::Smart;
    let mut error_0: duration::Error = crate::duration::Error::NumberExpected(usize_5);
    let mut error_1: duration::Error = crate::duration::Error::InvalidCharacter(usize_4);
    let mut error_2: duration::Error = crate::duration::Error::NumberExpected(usize_3);
    let mut error_3: duration::Error = crate::duration::Error::InvalidCharacter(usize_2);
    let mut precision_1: date::Precision = crate::date::Precision::Micros;
    let mut precision_2: date::Precision = crate::date::Precision::Nanos;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_1_ref_0);
    let mut precision_3: date::Precision = crate::date::Precision::Micros;
    let mut result_1: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_3, u8_2);
    let mut error_4: duration::Error = crate::duration::Error::NumberOverflow;
    let mut result_2: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_1, u8_0);
    let mut result_3: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_0_ref_0);
    panic!("From RustyUnit with love");
}
}