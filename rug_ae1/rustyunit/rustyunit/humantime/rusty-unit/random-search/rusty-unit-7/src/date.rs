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
fn rusty_test_515() {
    rusty_monitor::set_test_id(515);
    let mut usize_0: usize = 780usize;
    let mut char_0: char = 'g';
    let mut char_1: char = '"';
    let mut usize_1: usize = 1961usize;
    let mut u64_0: u64 = 3309u64;
    let mut usize_2: usize = 5157usize;
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut char_2: char = 'F';
    let mut char_3: char = 'W';
    let mut char_4: char = 's';
    let mut char_5: char = '~';
    let mut str_1: &str = "9FOuSRv2YB";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut u8_0: u8 = 122u8;
    let mut u8_1: u8 = 66u8;
    let mut char_6: char = '_';
    let mut char_7: char = 'c';
    let mut usize_3: usize = 1302usize;
    let mut u64_1: u64 = 1377u64;
    let mut usize_4: usize = 5221usize;
    let mut usize_5: usize = 8416usize;
    let mut str_2: &str = "8y";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut precision_0: date::Precision = crate::date::Precision::Millis;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_2_ref_0);
    let mut error_0: date::Error = crate::date::Error::InvalidDigit;
    let mut error_1: duration::Error = crate::duration::Error::InvalidCharacter(usize_3);
    let mut precision_1: date::Precision = crate::date::Precision::Micros;
    let mut option_0: std::option::Option<u64> = crate::date::two_digits_inner(char_7, char_6);
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_0);
    let mut result_1: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_1, u8_0);
    let mut result_2: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_1_ref_0);
    let mut option_1: std::option::Option<u64> = crate::date::two_digits_inner(char_5, char_4);
    let mut option_2: std::option::Option<u64> = crate::date::two_digits_inner(char_3, char_2);
    let mut result_3: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_0_ref_0);
    let mut precision_2: date::Precision = crate::date::Precision::Seconds;
    let mut error_2: duration::Error = crate::duration::Error::NumberOverflow;
    let mut precision_3: date::Precision = crate::date::Precision::Seconds;
    let mut error_3: duration::Error = crate::duration::Error::InvalidCharacter(usize_2);
    let mut bool_0: bool = crate::date::is_leap_year(u64_0);
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_3);
    let mut error_4: duration::Error = crate::duration::Error::NumberExpected(usize_1);
    let mut precision_4: date::Precision = crate::date::Precision::Smart;
    let mut precision_5: date::Precision = crate::date::Precision::Seconds;
    let mut u64_2: u64 = std::option::Option::unwrap(option_2);
    let mut u64_3: u64 = std::option::Option::unwrap(option_0);
    let mut option_3: std::option::Option<u64> = crate::date::two_digits_inner(char_1, char_0);
    let mut precision_6: date::Precision = crate::date::Precision::Seconds;
    let mut error_5: date::Error = crate::date::Error::InvalidFormat;
    let mut error_6: date::Error = crate::date::Error::InvalidDigit;
    let mut precision_7: date::Precision = crate::date::Precision::Seconds;
    let mut error_7: duration::Error = crate::duration::Error::NumberExpected(usize_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4456() {
    rusty_monitor::set_test_id(4456);
    let mut usize_0: usize = 4254usize;
    let mut usize_1: usize = 6167usize;
    let mut char_0: char = '\\';
    let mut char_1: char = 'v';
    let mut u8_0: u8 = 19u8;
    let mut u8_1: u8 = 119u8;
    let mut char_2: char = '0';
    let mut char_3: char = 'p';
    let mut str_0: &str = "5wP2h1EjQVfvmmG4igK";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "tUCMhm8s0saI5En";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut usize_2: usize = 5592usize;
    let mut error_0: duration::Error = crate::duration::Error::InvalidCharacter(usize_2);
    let mut precision_0: date::Precision = crate::date::Precision::Seconds;
    let mut error_1: duration::Error = crate::duration::Error::Empty;
    let mut precision_1: date::Precision = crate::date::Precision::Nanos;
    let mut error_2: duration::Error = crate::duration::Error::NumberOverflow;
    let mut error_3: duration::Error = crate::duration::Error::Empty;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_1_ref_0);
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_0);
    let mut result_1: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_0_ref_0);
    let mut error_4: duration::Error = crate::duration::Error::Empty;
    let mut option_0: std::option::Option<u64> = crate::date::two_digits_inner(char_3, char_2);
    let mut precision_2: date::Precision = crate::date::Precision::Smart;
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_1);
    let mut result_2: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_1, u8_0);
    let mut option_1: std::option::Option<u64> = crate::date::two_digits_inner(char_1, char_0);
    let mut precision_3: date::Precision = crate::date::Precision::Micros;
    let mut error_5: duration::Error = crate::duration::Error::InvalidCharacter(usize_1);
    let mut error_6: duration::Error = crate::duration::Error::NumberExpected(usize_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2615() {
    rusty_monitor::set_test_id(2615);
    let mut u8_0: u8 = 92u8;
    let mut u8_1: u8 = 2u8;
    let mut usize_0: usize = 1212usize;
    let mut usize_1: usize = 3864usize;
    let mut str_0: &str = "LV0QO8c7qi";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "eRst7TyEvc6r";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut usize_2: usize = 9416usize;
    let mut char_0: char = 'B';
    let mut char_1: char = ',';
    let mut u64_0: u64 = 2753u64;
    let mut usize_3: usize = 2881usize;
    let mut usize_4: usize = 4151usize;
    let mut usize_5: usize = 1903usize;
    let mut str_2: &str = "Q0iPeFXzwYPhyWXvN9";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut u64_1: u64 = 8768u64;
    let mut usize_6: usize = 1771usize;
    let mut usize_7: usize = 3788usize;
    let mut u64_2: u64 = 6719u64;
    let mut usize_8: usize = 2466usize;
    let mut usize_9: usize = 6010usize;
    let mut u8_2: u8 = 112u8;
    let mut u8_3: u8 = 46u8;
    let mut str_3: &str = "f0hN25wubFjJ";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut u32_0: u32 = 127u32;
    let mut str_4: &str = "CKmf9DDhwp1kyKp5pPk";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut bool_0: bool = false;
    let mut bool_0_ref_0: &mut bool = &mut bool_0;
    let mut str_5: &str = "NsPqdKVBBEIZ";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut u64_3: u64 = 6175u64;
    let mut u64_4: u64 = 7517u64;
    let mut tuple_0: (u64, u64) = (u64_4, u64_3);
    let mut str_6: &str = "f63RX";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7: &str = "8w1rE";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_8: &str = "aoUq5INgp5Pa";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut error_0: date::Error = crate::date::Error::OutOfRange;
    let mut error_1: date::Error = crate::date::Error::InvalidFormat;
    let mut result_0: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_8_ref_0);
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_7_ref_0);
    let mut error_2: duration::Error = crate::duration::Error::Empty;
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_5_ref_0);
    let mut result_3: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_3_ref_0);
    let mut result_4: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_3, u8_2);
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_1);
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_0);
    let mut result_5: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_2_ref_0);
    let mut precision_0: date::Precision = crate::date::Precision::Smart;
    let mut error_3: duration::Error = crate::duration::Error::NumberExpected(usize_5);
    let mut option_0: std::option::Option<u64> = crate::date::two_digits_inner(char_1, char_0);
    let mut error_4: duration::Error = crate::duration::Error::NumberExpected(usize_2);
    let mut result_6: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_1_ref_0);
    let mut result_7: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_0_ref_0);
    let mut precision_1: date::Precision = crate::date::Precision::Millis;
    let mut error_5: duration::Error = crate::duration::Error::NumberExpected(usize_1);
    let mut error_6: duration::Error = crate::duration::Error::NumberExpected(usize_0);
    let mut precision_2: date::Precision = crate::date::Precision::Seconds;
    let mut result_8: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_1, u8_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4720() {
    rusty_monitor::set_test_id(4720);
    let mut u64_0: u64 = 742u64;
    let mut u64_1: u64 = 6224u64;
    let mut usize_0: usize = 145usize;
    let mut usize_1: usize = 3188usize;
    let mut char_0: char = 'K';
    let mut char_1: char = '\r';
    let mut str_0: &str = "8m3";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "W2Q6LQEPFp8iQwc0Ia";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut usize_2: usize = 5829usize;
    let mut str_2: &str = "GDY9Pxkuim";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut u64_2: u64 = 4090u64;
    let mut str_3: &str = "";
    let mut string_0: std::string::String = std::string::String::from(str_3);
    let mut usize_3: usize = 8320usize;
    let mut usize_4: usize = 5605usize;
    let mut error_0: duration::Error = crate::duration::Error::UnknownUnit {start: usize_4, end: usize_3, unit: string_0, value: u64_2};
    let mut error_1: date::Error = crate::date::Error::InvalidFormat;
    let mut result_0: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_2_ref_0);
    let mut precision_0: date::Precision = crate::date::Precision::Nanos;
    let mut error_2: duration::Error = crate::duration::Error::NumberExpected(usize_2);
    let mut error_3: date::Error = crate::date::Error::InvalidDigit;
    let mut result_1: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_1_ref_0);
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_0_ref_0);
    let mut error_4: date::Error = crate::date::Error::InvalidFormat;
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_1);
    let mut error_5: duration::Error = crate::duration::Error::Empty;
    let mut option_0: std::option::Option<u64> = crate::date::two_digits_inner(char_1, char_0);
    let mut error_6: duration::Error = crate::duration::Error::Empty;
    let mut precision_1: date::Precision = crate::date::Precision::Millis;
    let mut error_7: date::Error = crate::date::Error::InvalidFormat;
    let mut bool_0: bool = crate::date::is_leap_year(u64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4567() {
    rusty_monitor::set_test_id(4567);
    let mut usize_0: usize = 3770usize;
    let mut str_0: &str = "8DDwlNGtWh9DNkB8HIm";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u64_0: u64 = 9822u64;
    let mut usize_1: usize = 1149usize;
    let mut usize_2: usize = 6550usize;
    let mut usize_3: usize = 8506usize;
    let mut str_1: &str = "cRw3hwQ9gKsUnoyi";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "WGJvtjovtv";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut u64_1: u64 = 1722u64;
    let mut str_3: &str = "dk";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut bool_0: bool = false;
    let mut bool_0_ref_0: &mut bool = &mut bool_0;
    let mut result_0: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_2_ref_0);
    let mut precision_0: date::Precision = crate::date::Precision::Seconds;
    let mut error_0: duration::Error = crate::duration::Error::NumberOverflow;
    let mut result_1: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_1_ref_0);
    let mut error_1: date::Error = crate::date::Error::InvalidDigit;
    let mut error_2: duration::Error = crate::duration::Error::InvalidCharacter(usize_3);
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_1);
    let mut error_3: duration::Error = crate::duration::Error::NumberOverflow;
    let mut error_4: duration::Error = crate::duration::Error::Empty;
    let mut systemtime_1: std::time::SystemTime = std::result::Result::unwrap(result_0);
    let mut result_2: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_0_ref_0);
    let mut error_5: duration::Error = crate::duration::Error::InvalidCharacter(usize_0);
    let mut error_6: duration::Error = crate::duration::Error::NumberOverflow;
    let mut error_7: date::Error = crate::date::Error::InvalidDigit;
    let mut precision_1: date::Precision = crate::date::Precision::Smart;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_338() {
    rusty_monitor::set_test_id(338);
    let mut u64_0: u64 = 8582u64;
    let mut u64_1: u64 = 2508u64;
    let mut usize_0: usize = 7285usize;
    let mut usize_1: usize = 243usize;
    let mut char_0: char = '<';
    let mut char_1: char = '%';
    let mut u64_2: u64 = 7649u64;
    let mut usize_2: usize = 776usize;
    let mut usize_3: usize = 8781usize;
    let mut u64_3: u64 = 9050u64;
    let mut usize_4: usize = 3560usize;
    let mut usize_5: usize = 1559usize;
    let mut usize_6: usize = 5867usize;
    let mut str_0: &str = "3o3xl2Rbws8JmIKJTdr";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u64_4: u64 = 354u64;
    let mut usize_7: usize = 4126usize;
    let mut u64_5: u64 = 4857u64;
    let mut char_2: char = 'h';
    let mut char_3: char = '9';
    let mut precision_0: date::Precision = crate::date::Precision::Nanos;
    let mut option_0: std::option::Option<u64> = crate::date::two_digits_inner(char_3, char_2);
    let mut u64_6: u64 = std::option::Option::unwrap(option_0);
    let mut bool_0: bool = crate::date::is_leap_year(u64_5);
    let mut error_0: duration::Error = crate::duration::Error::NumberExpected(usize_7);
    let mut bool_1: bool = crate::date::is_leap_year(u64_4);
    let mut result_0: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_0_ref_0);
    let mut error_1: duration::Error = crate::duration::Error::InvalidCharacter(usize_6);
    let mut option_1: std::option::Option<u64> = crate::date::two_digits_inner(char_1, char_0);
    let mut precision_1: date::Precision = crate::date::Precision::Nanos;
    let mut bool_2: bool = crate::date::is_leap_year(u64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4675() {
    rusty_monitor::set_test_id(4675);
    let mut str_0: &str = "aU32eBDKvxTw5k5";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u64_0: u64 = 1671u64;
    let mut usize_0: usize = 6167usize;
    let mut usize_1: usize = 6977usize;
    let mut str_1: &str = "pYf25IWcIGsGnKDq4F";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut u8_0: u8 = 99u8;
    let mut u8_1: u8 = 12u8;
    let mut str_2: &str = "E6iniHQl25";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut char_0: char = '?';
    let mut char_1: char = '\t';
    let mut u64_1: u64 = 9240u64;
    let mut usize_2: usize = 6064usize;
    let mut usize_3: usize = 721usize;
    let mut usize_4: usize = 1815usize;
    let mut str_3: &str = "vBn2kd7ujWVyIzSBcbc";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut error_0: date::Error = crate::date::Error::OutOfRange;
    let mut result_0: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_3_ref_0);
    let mut precision_0: date::Precision = crate::date::Precision::Millis;
    let mut error_1: duration::Error = crate::duration::Error::NumberExpected(usize_4);
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_0);
    let mut option_0: std::option::Option<u64> = crate::date::two_digits_inner(char_1, char_0);
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_2_ref_0);
    let mut error_2: date::Error = crate::date::Error::OutOfRange;
    let mut result_2: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_1, u8_0);
    let mut result_3: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_1_ref_0);
    let mut error_3: duration::Error = crate::duration::Error::NumberOverflow;
    let mut result_4: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4998() {
    rusty_monitor::set_test_id(4998);
    let mut str_0: &str = "iUdjipH0";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u64_0: u64 = 707u64;
    let mut usize_0: usize = 1645usize;
    let mut usize_1: usize = 8718usize;
    let mut usize_2: usize = 3160usize;
    let mut char_0: char = 'U';
    let mut char_1: char = 'b';
    let mut char_2: char = ';';
    let mut char_3: char = '<';
    let mut usize_3: usize = 8120usize;
    let mut u64_1: u64 = 5923u64;
    let mut str_1: &str = "m5stDnIuKvQ";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bool_0: bool = true;
    let mut bool_0_ref_0: &mut bool = &mut bool_0;
    let mut u64_2: u64 = 2406u64;
    let mut u64_3: u64 = 1147u64;
    let mut tuple_0: (u64, u64) = (u64_3, u64_2);
    let mut str_2: &str = "6CtvH";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut error_0: duration::Error = crate::duration::Error::InvalidCharacter(usize_3);
    let mut option_0: std::option::Option<u64> = crate::date::two_digits_inner(char_3, char_2);
    let mut precision_0: date::Precision = crate::date::Precision::Micros;
    let mut u64_4: u64 = std::option::Option::unwrap(option_0);
    let mut option_1: std::option::Option<u64> = crate::date::two_digits_inner(char_1, char_0);
    let mut precision_1: date::Precision = crate::date::Precision::Smart;
    let mut error_1: duration::Error = crate::duration::Error::NumberExpected(usize_2);
    let mut result_0: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_0_ref_0);
    let mut precision_2: date::Precision = crate::date::Precision::Millis;
    let mut error_2: duration::Error = crate::duration::Error::Empty;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2500() {
    rusty_monitor::set_test_id(2500);
    let mut u8_0: u8 = 36u8;
    let mut u8_1: u8 = 82u8;
    let mut str_0: &str = "JWTb5cRFOb2AINV";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u64_0: u64 = 3462u64;
    let mut usize_0: usize = 4078usize;
    let mut str_1: &str = "g0H5f6";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "4IaomhwA";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut error_0: date::Error = crate::date::Error::OutOfRange;
    let mut error_1: duration::Error = crate::duration::Error::Empty;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_2_ref_0);
    let mut result_1: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_1_ref_0);
    let mut error_2: duration::Error = crate::duration::Error::NumberOverflow;
    let mut precision_0: date::Precision = crate::date::Precision::Seconds;
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_0);
    let mut error_3: duration::Error = crate::duration::Error::InvalidCharacter(usize_0);
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_1);
    let mut precision_1: date::Precision = crate::date::Precision::Seconds;
    let mut error_4: duration::Error = crate::duration::Error::Empty;
    let mut precision_2: date::Precision = crate::date::Precision::Smart;
    let mut bool_0: bool = crate::date::is_leap_year(u64_0);
    let mut result_2: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_0_ref_0);
    let mut systemtime_1: std::time::SystemTime = std::result::Result::unwrap(result_2);
    let mut error_5: duration::Error = crate::duration::Error::NumberOverflow;
    let mut result_3: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_1, u8_0);
    let mut error_6: date::Error = crate::date::Error::OutOfRange;
    let mut precision_3: date::Precision = crate::date::Precision::Seconds;
    let mut u64_1: u64 = std::result::Result::unwrap(result_3);
    let mut precision_4: date::Precision = crate::date::Precision::Nanos;
    let mut precision_5: date::Precision = crate::date::Precision::Millis;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4() {
    rusty_monitor::set_test_id(4);
    let mut str_0: &str = "eBuCxV2Dva3jOU";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut char_0: char = '{';
    let mut char_1: char = '=';
    let mut usize_0: usize = 4174usize;
    let mut str_1: &str = "08wNS3UPbJxTjj";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut u32_0: u32 = 8456u32;
    let mut str_2: &str = "LBONi4yBnQDTbiFT";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bool_0: bool = true;
    let mut bool_0_ref_0: &mut bool = &mut bool_0;
    let mut str_3: &str = "Rkh";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut u64_0: u64 = 2162u64;
    let mut u64_1: u64 = 8809u64;
    let mut tuple_0: (u64, u64) = (u64_1, u64_0);
    let mut str_4: &str = "Wu9okr11SPXRlHFR6i8";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_3_ref_0);
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_1_ref_0);
    let mut precision_0: date::Precision = crate::date::Precision::Smart;
    let mut error_0: duration::Error = crate::duration::Error::Empty;
    let mut precision_1: date::Precision = crate::date::Precision::Millis;
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_0);
    let mut duration_1: std::time::Duration = std::result::Result::unwrap(result_1);
    let mut error_1: duration::Error = crate::duration::Error::NumberExpected(usize_0);
    let mut precision_2: date::Precision = crate::date::Precision::Seconds;
    let mut option_0: std::option::Option<u64> = crate::date::two_digits_inner(char_1, char_0);
    let mut precision_3: date::Precision = crate::date::Precision::Nanos;
    let mut u64_2: u64 = std::option::Option::unwrap(option_0);
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4918() {
    rusty_monitor::set_test_id(4918);
    let mut str_0: &str = "W7CULTYgZGOryyzP30";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "u9";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut usize_0: usize = 540usize;
    let mut u8_0: u8 = 15u8;
    let mut u8_1: u8 = 126u8;
    let mut char_0: char = 'h';
    let mut char_1: char = 'Y';
    let mut u8_2: u8 = 24u8;
    let mut u8_3: u8 = 119u8;
    let mut u64_0: u64 = 1129u64;
    let mut u64_1: u64 = 5912u64;
    let mut tuple_0: (u64, u64) = (u64_1, u64_0);
    let mut str_2: &str = "Px";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut precision_0: date::Precision = crate::date::Precision::Millis;
    let mut result_0: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_3, u8_2);
    let mut option_0: std::option::Option<u64> = crate::date::two_digits_inner(char_1, char_0);
    let mut result_1: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_1, u8_0);
    let mut precision_1: date::Precision = crate::date::Precision::Millis;
    let mut error_0: date::Error = crate::date::Error::InvalidDigit;
    let mut error_1: duration::Error = crate::duration::Error::NumberExpected(usize_0);
    let mut u64_2: u64 = std::result::Result::unwrap(result_0);
    let mut result_2: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_1_ref_0);
    let mut u64_3: u64 = std::result::Result::unwrap(result_1);
    let mut result_3: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_0_ref_0);
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_3);
    let mut error_2: date::Error = crate::date::Error::InvalidFormat;
    let mut systemtime_1: std::time::SystemTime = std::result::Result::unwrap(result_2);
    let mut error_3: date::Error = crate::date::Error::InvalidDigit;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2282() {
    rusty_monitor::set_test_id(2282);
    let mut usize_0: usize = 5293usize;
    let mut u64_0: u64 = 6470u64;
    let mut usize_1: usize = 6022usize;
    let mut usize_2: usize = 6935usize;
    let mut str_0: &str = "qTLB";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "mk3XywaieaMAqICb1d";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut u8_0: u8 = 97u8;
    let mut u8_1: u8 = 0u8;
    let mut str_2: &str = "CwFsswbsmVf9egIxgtv";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut u8_2: u8 = 2u8;
    let mut u8_3: u8 = 90u8;
    let mut u8_4: u8 = 65u8;
    let mut u8_5: u8 = 29u8;
    let mut usize_3: usize = 9887usize;
    let mut precision_0: date::Precision = crate::date::Precision::Millis;
    let mut error_0: duration::Error = crate::duration::Error::InvalidCharacter(usize_3);
    let mut precision_1: date::Precision = crate::date::Precision::Millis;
    let mut result_0: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_5, u8_4);
    let mut result_1: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_3, u8_2);
    let mut error_1: date::Error = crate::date::Error::OutOfRange;
    let mut precision_2: date::Precision = crate::date::Precision::Seconds;
    let mut result_2: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_2_ref_0);
    let mut result_3: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_1, u8_0);
    let mut result_4: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_1_ref_0);
    let mut precision_3: date::Precision = crate::date::Precision::Micros;
    let mut u64_1: u64 = std::result::Result::unwrap(result_3);
    let mut result_5: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_0_ref_0);
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_5);
    let mut error_2: duration::Error = crate::duration::Error::NumberExpected(usize_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4970() {
    rusty_monitor::set_test_id(4970);
    let mut str_0: &str = "TrPN9Zp";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut usize_0: usize = 7767usize;
    let mut u64_0: u64 = 6486u64;
    let mut usize_1: usize = 342usize;
    let mut usize_2: usize = 7497usize;
    let mut usize_3: usize = 1886usize;
    let mut usize_4: usize = 7595usize;
    let mut usize_5: usize = 2083usize;
    let mut usize_6: usize = 1782usize;
    let mut usize_7: usize = 123usize;
    let mut str_1: &str = "7m";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut u64_1: u64 = 6660u64;
    let mut u64_2: u64 = 3488u64;
    let mut tuple_0: (u64, u64) = (u64_2, u64_1);
    let mut str_2: &str = "I36LjWy";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut u64_3: u64 = 5623u64;
    let mut u64_4: u64 = 7927u64;
    let mut tuple_1: (u64, u64) = (u64_4, u64_3);
    let mut str_3: &str = "egAHOjdli";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut precision_0: date::Precision = crate::date::Precision::Nanos;
    let mut precision_1: date::Precision = crate::date::Precision::Nanos;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_1_ref_0);
    let mut error_0: duration::Error = crate::duration::Error::InvalidCharacter(usize_7);
    let mut error_1: duration::Error = crate::duration::Error::Empty;
    let mut error_2: duration::Error = crate::duration::Error::NumberExpected(usize_6);
    let mut precision_2: date::Precision = crate::date::Precision::Micros;
    let mut error_3: duration::Error = crate::duration::Error::NumberExpected(usize_5);
    let mut error_4: duration::Error = crate::duration::Error::NumberOverflow;
    let mut precision_3: date::Precision = crate::date::Precision::Millis;
    let mut error_5: duration::Error = crate::duration::Error::NumberExpected(usize_4);
    let mut error_6: duration::Error = crate::duration::Error::NumberExpected(usize_3);
    let mut error_7: date::Error = crate::date::Error::InvalidDigit;
    let mut error_8: date::Error = crate::date::Error::InvalidFormat;
    let mut precision_4: date::Precision = crate::date::Precision::Millis;
    let mut error_9: date::Error = crate::date::Error::InvalidFormat;
    let mut error_10: date::Error = crate::date::Error::InvalidDigit;
    let mut error_11: duration::Error = crate::duration::Error::InvalidCharacter(usize_0);
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_0);
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4002() {
    rusty_monitor::set_test_id(4002);
    let mut u64_0: u64 = 3147u64;
    let mut usize_0: usize = 2779usize;
    let mut usize_1: usize = 6146usize;
    let mut u64_1: u64 = 1058u64;
    let mut usize_2: usize = 6074usize;
    let mut usize_3: usize = 3640usize;
    let mut u8_0: u8 = 95u8;
    let mut u8_1: u8 = 6u8;
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "5WHd7E";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut usize_4: usize = 3928usize;
    let mut precision_0: date::Precision = crate::date::Precision::Micros;
    let mut error_0: duration::Error = crate::duration::Error::Empty;
    let mut precision_1: date::Precision = crate::date::Precision::Millis;
    let mut error_1: duration::Error = crate::duration::Error::InvalidCharacter(usize_4);
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_1_ref_0);
    let mut precision_2: date::Precision = crate::date::Precision::Seconds;
    let mut precision_3: date::Precision = crate::date::Precision::Micros;
    let mut error_2: date::Error = crate::date::Error::OutOfRange;
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_0);
    let mut result_1: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_0_ref_0);
    let mut precision_4: date::Precision = crate::date::Precision::Nanos;
    let mut result_2: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_1, u8_0);
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_1);
    let mut error_3: date::Error = crate::date::Error::InvalidFormat;
    let mut precision_5: date::Precision = crate::date::Precision::Millis;
    let mut u64_2: u64 = std::result::Result::unwrap(result_2);
    let mut precision_6: date::Precision = crate::date::Precision::Smart;
    let mut error_4: duration::Error = crate::duration::Error::NumberOverflow;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3627() {
    rusty_monitor::set_test_id(3627);
    let mut usize_0: usize = 3682usize;
    let mut usize_1: usize = 1132usize;
    let mut str_0: &str = "Ojh";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u64_0: u64 = 4282u64;
    let mut str_1: &str = "z";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut u8_0: u8 = 3u8;
    let mut u8_1: u8 = 16u8;
    let mut usize_2: usize = 767usize;
    let mut u64_1: u64 = 2076u64;
    let mut bool_0: bool = crate::date::is_leap_year(u64_1);
    let mut error_0: duration::Error = crate::duration::Error::InvalidCharacter(usize_2);
    let mut result_0: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_1, u8_0);
    let mut u64_2: u64 = std::result::Result::unwrap(result_0);
    let mut precision_0: date::Precision = crate::date::Precision::Millis;
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_1_ref_0);
    let mut error_1: duration::Error = crate::duration::Error::NumberOverflow;
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_1);
    let mut error_2: date::Error = crate::date::Error::InvalidFormat;
    let mut bool_1: bool = crate::date::is_leap_year(u64_0);
    let mut error_3: duration::Error = crate::duration::Error::Empty;
    let mut result_2: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_0_ref_0);
    let mut precision_1: date::Precision = crate::date::Precision::Nanos;
    let mut error_4: date::Error = crate::date::Error::OutOfRange;
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_2);
    let mut error_5: duration::Error = crate::duration::Error::InvalidCharacter(usize_1);
    let mut error_6: date::Error = crate::date::Error::InvalidFormat;
    let mut error_7: duration::Error = crate::duration::Error::InvalidCharacter(usize_0);
    let mut precision_2: date::Precision = crate::date::Precision::Seconds;
    let mut error_8: date::Error = crate::date::Error::InvalidFormat;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_229() {
    rusty_monitor::set_test_id(229);
    let mut str_0: &str = "N";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut char_0: char = 'Q';
    let mut char_1: char = ',';
    let mut u64_0: u64 = 8864u64;
    let mut u64_1: u64 = 927u64;
    let mut tuple_0: (u64, u64) = (u64_1, u64_0);
    let mut str_1: &str = "2aufQOgmzgCwgEof";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut u64_2: u64 = 1214u64;
    let mut u64_3: u64 = 239u64;
    let mut tuple_1: (u64, u64) = (u64_3, u64_2);
    let mut str_3: &str = "Loc4leRR6zvOOY";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut u64_4: u64 = 6552u64;
    let mut u64_5: u64 = 7414u64;
    let mut tuple_2: (u64, u64) = (u64_5, u64_4);
    let mut str_4: &str = "o7wvVFuMKes5S";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_2_ref_0);
    let mut error_0: date::Error = crate::date::Error::OutOfRange;
    let mut precision_0: date::Precision = crate::date::Precision::Seconds;
    let mut option_0: std::option::Option<u64> = crate::date::two_digits_inner(char_1, char_0);
    let mut result_1: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_0_ref_0);
    let mut precision_1: date::Precision = crate::date::Precision::Millis;
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_1);
    let mut error_1: date::Error = crate::date::Error::InvalidDigit;
    let mut error_2: date::Error = crate::date::Error::InvalidDigit;
    let mut error_3: duration::Error = crate::duration::Error::NumberOverflow;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4939() {
    rusty_monitor::set_test_id(4939);
    let mut usize_0: usize = 3424usize;
    let mut u64_0: u64 = 3039u64;
    let mut str_0: &str = "8uBRq";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "Ggj";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "igDFpk";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "Rh";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut u64_1: u64 = 4449u64;
    let mut str_4: &str = "Kuhcn1UPXV2j5R4";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut precision_0: date::Precision = crate::date::Precision::Seconds;
    let mut precision_1: date::Precision = crate::date::Precision::Millis;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_4_ref_0);
    let mut bool_0: bool = crate::date::is_leap_year(u64_1);
    let mut result_1: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_3_ref_0);
    let mut result_2: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_2_ref_0);
    let mut result_3: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_1_ref_0);
    let mut result_4: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_0_ref_0);
    let mut precision_2: date::Precision = crate::date::Precision::Seconds;
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_4);
    let mut error_0: duration::Error = crate::duration::Error::NumberOverflow;
    let mut bool_1: bool = crate::date::is_leap_year(u64_0);
    let mut precision_3: date::Precision = crate::date::Precision::Seconds;
    let mut systemtime_1: std::time::SystemTime = std::result::Result::unwrap(result_2);
    let mut error_1: duration::Error = crate::duration::Error::InvalidCharacter(usize_0);
    let mut precision_4: date::Precision = crate::date::Precision::Seconds;
    let mut precision_5: date::Precision = crate::date::Precision::Smart;
    let mut error_2: date::Error = crate::date::Error::OutOfRange;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3521() {
    rusty_monitor::set_test_id(3521);
    let mut char_0: char = '1';
    let mut char_1: char = 'c';
    let mut char_2: char = '.';
    let mut char_3: char = 'N';
    let mut str_0: &str = "WgdcjzsR3zxdg9";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "Y9AsZOUK";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut u64_0: u64 = 5263u64;
    let mut usize_0: usize = 7670usize;
    let mut usize_1: usize = 2916usize;
    let mut u64_1: u64 = 3370u64;
    let mut u64_2: u64 = 1763u64;
    let mut tuple_0: (u64, u64) = (u64_2, u64_1);
    let mut str_2: &str = "F5yumU7l";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut u64_3: u64 = 4777u64;
    let mut u64_4: u64 = 2700u64;
    let mut tuple_1: (u64, u64) = (u64_4, u64_3);
    let mut str_3: &str = "XnzM6KTw";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_4_ref_0);
    let mut error_0: duration::Error = crate::duration::Error::NumberOverflow;
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_0);
    let mut precision_0: date::Precision = crate::date::Precision::Millis;
    let mut precision_1: date::Precision = crate::date::Precision::Smart;
    let mut result_1: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_1_ref_0);
    let mut precision_2: date::Precision = crate::date::Precision::Nanos;
    let mut result_2: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_0_ref_0);
    let mut error_1: date::Error = crate::date::Error::InvalidFormat;
    let mut precision_3: date::Precision = crate::date::Precision::Nanos;
    let mut error_2: date::Error = crate::date::Error::InvalidDigit;
    let mut precision_4: date::Precision = crate::date::Precision::Smart;
    let mut option_0: std::option::Option<u64> = crate::date::two_digits_inner(char_3, char_2);
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_1);
    let mut u64_5: u64 = std::option::Option::unwrap(option_0);
    let mut error_3: date::Error = crate::date::Error::InvalidFormat;
    let mut error_4: date::Error = crate::date::Error::InvalidDigit;
    let mut error_5: date::Error = crate::date::Error::OutOfRange;
    let mut option_1: std::option::Option<u64> = crate::date::two_digits_inner(char_1, char_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4845() {
    rusty_monitor::set_test_id(4845);
    let mut u64_0: u64 = 8358u64;
    let mut u64_1: u64 = 9845u64;
    let mut str_0: &str = "XShGP7i5Ys3w";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "YoBO4yi";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "blXkQhygknAPTnT7F7";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut usize_0: usize = 1462usize;
    let mut u8_0: u8 = 36u8;
    let mut u8_1: u8 = 47u8;
    let mut precision_0: date::Precision = crate::date::Precision::Millis;
    let mut result_0: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_1, u8_0);
    let mut error_0: duration::Error = crate::duration::Error::NumberExpected(usize_0);
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_2_ref_0);
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_1_ref_0);
    let mut precision_1: date::Precision = crate::date::Precision::Nanos;
    let mut result_3: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_0_ref_0);
    let mut error_1: date::Error = crate::date::Error::InvalidFormat;
    let mut precision_2: date::Precision = crate::date::Precision::Nanos;
    let mut u64_2: u64 = std::result::Result::unwrap(result_0);
    let mut error_2: duration::Error = crate::duration::Error::NumberOverflow;
    let mut bool_0: bool = crate::date::is_leap_year(u64_1);
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_1);
    let mut precision_3: date::Precision = crate::date::Precision::Nanos;
    let mut bool_1: bool = crate::date::is_leap_year(u64_0);
    let mut precision_4: date::Precision = crate::date::Precision::Micros;
    let mut error_3: duration::Error = crate::duration::Error::NumberOverflow;
    let mut error_4: duration::Error = crate::duration::Error::Empty;
    let mut error_5: duration::Error = crate::duration::Error::NumberOverflow;
    let mut error_6: duration::Error = crate::duration::Error::NumberOverflow;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1959() {
    rusty_monitor::set_test_id(1959);
    let mut u64_0: u64 = 8459u64;
    let mut usize_0: usize = 4978usize;
    let mut usize_1: usize = 3750usize;
    let mut usize_2: usize = 2366usize;
    let mut char_0: char = 'c';
    let mut char_1: char = ' ';
    let mut str_0: &str = "R6";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut usize_3: usize = 2952usize;
    let mut u64_1: u64 = 3080u64;
    let mut str_1: &str = "ix6klXF23nlsnTUK";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut char_2: char = '2';
    let mut char_3: char = '7';
    let mut u64_2: u64 = 8172u64;
    let mut u64_3: u64 = 8363u64;
    let mut tuple_0: (u64, u64) = (u64_3, u64_2);
    let mut str_2: &str = "XmiFAAmBP";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut option_0: std::option::Option<u64> = crate::date::two_digits_inner(char_3, char_2);
    let mut u64_4: u64 = std::option::Option::unwrap(option_0);
    let mut error_0: duration::Error = crate::duration::Error::Empty;
    let mut result_0: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_1_ref_0);
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_0);
    let mut error_1: date::Error = crate::date::Error::OutOfRange;
    let mut error_2: duration::Error = crate::duration::Error::Empty;
    let mut precision_0: date::Precision = crate::date::Precision::Millis;
    let mut error_3: date::Error = crate::date::Error::InvalidFormat;
    let mut bool_0: bool = crate::date::is_leap_year(u64_1);
    let mut precision_1: date::Precision = crate::date::Precision::Smart;
    let mut error_4: duration::Error = crate::duration::Error::NumberExpected(usize_3);
    let mut result_1: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_0_ref_0);
    let mut option_1: std::option::Option<u64> = crate::date::two_digits_inner(char_1, char_0);
    let mut u64_5: u64 = std::option::Option::unwrap(option_1);
    let mut error_5: duration::Error = crate::duration::Error::NumberExpected(usize_2);
    let mut precision_2: date::Precision = crate::date::Precision::Seconds;
    let mut error_6: duration::Error = crate::duration::Error::Empty;
    let mut error_7: date::Error = crate::date::Error::OutOfRange;
    panic!("From RustyUnit with love");
}
}