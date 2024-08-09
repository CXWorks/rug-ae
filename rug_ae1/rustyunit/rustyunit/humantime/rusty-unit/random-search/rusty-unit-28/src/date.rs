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
fn rusty_test_4705() {
    rusty_monitor::set_test_id(4705);
    let mut u64_0: u64 = 1617u64;
    let mut usize_0: usize = 4508usize;
    let mut u8_0: u8 = 110u8;
    let mut u8_1: u8 = 57u8;
    let mut u64_1: u64 = 3437u64;
    let mut str_0: &str = "R";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u8_2: u8 = 67u8;
    let mut u8_3: u8 = 58u8;
    let mut u32_0: u32 = 6304u32;
    let mut str_1: &str = "yjK06JmNuQjAA";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bool_0: bool = false;
    let mut bool_0_ref_0: &mut bool = &mut bool_0;
    let mut usize_1: usize = 1796usize;
    let mut error_0: duration::Error = crate::duration::Error::InvalidCharacter(usize_1);
    let mut error_1: date::Error = crate::date::Error::OutOfRange;
    let mut error_2: date::Error = crate::date::Error::OutOfRange;
    let mut precision_0: date::Precision = crate::date::Precision::Millis;
    let mut result_0: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_3, u8_2);
    let mut result_1: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_0_ref_0);
    let mut error_3: duration::Error = crate::duration::Error::NumberOverflow;
    let mut error_4: duration::Error = crate::duration::Error::Empty;
    let mut bool_1: bool = crate::date::is_leap_year(u64_1);
    let mut precision_1: date::Precision = crate::date::Precision::Smart;
    let mut precision_2: date::Precision = crate::date::Precision::Millis;
    let mut result_2: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_1, u8_0);
    let mut error_5: duration::Error = crate::duration::Error::NumberExpected(usize_0);
    let mut error_6: date::Error = crate::date::Error::OutOfRange;
    let mut error_7: date::Error = crate::date::Error::OutOfRange;
    let mut bool_2: bool = crate::date::is_leap_year(u64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4919() {
    rusty_monitor::set_test_id(4919);
    let mut usize_0: usize = 7155usize;
    let mut str_0: &str = "ySZhW8WJxQc23Ds3ce";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "JY8VpImmv";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut u8_0: u8 = 111u8;
    let mut u8_1: u8 = 67u8;
    let mut usize_1: usize = 194usize;
    let mut str_3: &str = "";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut usize_2: usize = 7908usize;
    let mut u64_0: u64 = 6045u64;
    let mut u64_1: u64 = 1913u64;
    let mut tuple_0: (u64, u64) = (u64_1, u64_0);
    let mut str_4: &str = "S9HWNCnlleZ9hnSvMUb";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut error_0: duration::Error = crate::duration::Error::InvalidCharacter(usize_2);
    let mut result_0: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_3_ref_0);
    let mut error_1: duration::Error = crate::duration::Error::NumberExpected(usize_1);
    let mut precision_0: date::Precision = crate::date::Precision::Nanos;
    let mut precision_1: date::Precision = crate::date::Precision::Micros;
    let mut result_1: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_1, u8_0);
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_2_ref_0);
    let mut precision_2: date::Precision = crate::date::Precision::Micros;
    let mut error_2: duration::Error = crate::duration::Error::Empty;
    let mut error_3: duration::Error = crate::duration::Error::NumberOverflow;
    let mut result_3: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_1_ref_0);
    let mut result_4: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_0_ref_0);
    let mut error_4: duration::Error = crate::duration::Error::NumberExpected(usize_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4745() {
    rusty_monitor::set_test_id(4745);
    let mut str_0: &str = "GBa5XFuNA3IV";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u8_0: u8 = 30u8;
    let mut u8_1: u8 = 124u8;
    let mut u64_0: u64 = 850u64;
    let mut usize_0: usize = 575usize;
    let mut usize_1: usize = 5449usize;
    let mut u64_1: u64 = 7743u64;
    let mut str_1: &str = "ApacRpF9yMr";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "tAv6gmP397iaOu3oUq";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut usize_2: usize = 2598usize;
    let mut u64_2: u64 = 8822u64;
    let mut usize_3: usize = 2055usize;
    let mut usize_4: usize = 1076usize;
    let mut u8_2: u8 = 15u8;
    let mut u8_3: u8 = 125u8;
    let mut str_3: &str = "fRWBuTcnqh0ktS4ebC";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "t1pbdAqe";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut char_0: char = '.';
    let mut char_1: char = '7';
    let mut str_5: &str = "67d52WI9k3IC";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut char_2: char = '`';
    let mut char_3: char = 'p';
    let mut u64_3: u64 = 3015u64;
    let mut str_6: &str = "t5";
    let mut string_0: std::string::String = std::string::String::from(str_6);
    let mut usize_5: usize = 5256usize;
    let mut usize_6: usize = 1866usize;
    let mut error_0: duration::Error = crate::duration::Error::UnknownUnit {start: usize_6, end: usize_5, unit: string_0, value: u64_3};
    let mut precision_0: date::Precision = crate::date::Precision::Millis;
    let mut precision_1: date::Precision = crate::date::Precision::Nanos;
    let mut option_0: std::option::Option<u64> = crate::date::two_digits_inner(char_3, char_2);
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_5_ref_0);
    let mut error_1: date::Error = crate::date::Error::OutOfRange;
    let mut option_1: std::option::Option<u64> = crate::date::two_digits_inner(char_1, char_0);
    let mut result_1: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_4_ref_0);
    let mut error_2: duration::Error = crate::duration::Error::Empty;
    let mut precision_2: date::Precision = crate::date::Precision::Millis;
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_3_ref_0);
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_2);
    let mut duration_1: std::time::Duration = std::result::Result::unwrap(result_0);
    let mut error_3: date::Error = crate::date::Error::InvalidDigit;
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_1);
    let mut result_3: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_3, u8_2);
    let mut error_4: date::Error = crate::date::Error::OutOfRange;
    let mut error_5: duration::Error = crate::duration::Error::NumberOverflow;
    let mut error_6: duration::Error = crate::duration::Error::NumberExpected(usize_2);
    let mut error_7: date::Error = crate::date::Error::InvalidFormat;
    let mut u64_4: u64 = std::result::Result::unwrap(result_3);
    let mut result_4: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_2_ref_0);
    let mut result_5: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_1_ref_0);
    let mut bool_0: bool = crate::date::is_leap_year(u64_1);
    let mut systemtime_1: std::time::SystemTime = std::result::Result::unwrap(result_5);
    let mut precision_3: date::Precision = crate::date::Precision::Micros;
    let mut result_6: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_1, u8_0);
    let mut precision_4: date::Precision = crate::date::Precision::Smart;
    let mut error_8: date::Error = crate::date::Error::InvalidDigit;
    let mut error_9: date::Error = crate::date::Error::InvalidDigit;
    let mut error_10: date::Error = crate::date::Error::InvalidFormat;
    let mut error_11: duration::Error = crate::duration::Error::NumberOverflow;
    let mut u64_5: u64 = std::option::Option::unwrap(option_0);
    let mut systemtime_2: std::time::SystemTime = std::result::Result::unwrap(result_4);
    let mut u64_6: u64 = std::option::Option::unwrap(option_1);
    let mut u64_7: u64 = std::result::Result::unwrap(result_6);
    let mut result_7: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4819() {
    rusty_monitor::set_test_id(4819);
    let mut char_0: char = 'C';
    let mut char_1: char = 'e';
    let mut u8_0: u8 = 121u8;
    let mut u8_1: u8 = 42u8;
    let mut usize_0: usize = 5405usize;
    let mut str_0: &str = "3EQ";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "vmqXpuuh0vi";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut u64_0: u64 = 5290u64;
    let mut usize_1: usize = 3980usize;
    let mut usize_2: usize = 8683usize;
    let mut usize_3: usize = 3529usize;
    let mut usize_4: usize = 9809usize;
    let mut usize_5: usize = 5365usize;
    let mut u64_1: u64 = 2130u64;
    let mut str_2: &str = "0M7MNkUnmk6nu4peRpr";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut usize_6: usize = 2238usize;
    let mut precision_0: date::Precision = crate::date::Precision::Micros;
    let mut precision_1: date::Precision = crate::date::Precision::Millis;
    let mut error_0: date::Error = crate::date::Error::InvalidDigit;
    let mut precision_2: date::Precision = crate::date::Precision::Smart;
    let mut error_1: date::Error = crate::date::Error::InvalidDigit;
    let mut error_2: duration::Error = crate::duration::Error::Empty;
    let mut error_3: date::Error = crate::date::Error::OutOfRange;
    let mut error_4: date::Error = crate::date::Error::OutOfRange;
    let mut precision_3: date::Precision = crate::date::Precision::Smart;
    let mut error_5: duration::Error = crate::duration::Error::InvalidCharacter(usize_6);
    let mut error_6: date::Error = crate::date::Error::InvalidFormat;
    let mut error_7: date::Error = crate::date::Error::InvalidFormat;
    let mut error_8: duration::Error = crate::duration::Error::Empty;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_2_ref_0);
    let mut bool_0: bool = crate::date::is_leap_year(u64_1);
    let mut error_9: duration::Error = crate::duration::Error::NumberExpected(usize_5);
    let mut error_10: duration::Error = crate::duration::Error::InvalidCharacter(usize_4);
    let mut error_11: duration::Error = crate::duration::Error::InvalidCharacter(usize_3);
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_1_ref_0);
    let mut precision_4: date::Precision = crate::date::Precision::Seconds;
    let mut precision_5: date::Precision = crate::date::Precision::Seconds;
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_0_ref_0);
    let mut error_12: duration::Error = crate::duration::Error::InvalidCharacter(usize_0);
    let mut precision_6: date::Precision = crate::date::Precision::Millis;
    let mut result_3: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_1, u8_0);
    let mut precision_7: date::Precision = crate::date::Precision::Micros;
    let mut precision_8: date::Precision = crate::date::Precision::Seconds;
    let mut option_0: std::option::Option<u64> = crate::date::two_digits_inner(char_1, char_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4082() {
    rusty_monitor::set_test_id(4082);
    let mut usize_0: usize = 4827usize;
    let mut usize_1: usize = 6598usize;
    let mut str_0: &str = "5p99wY9Lf";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "V71Sv14rPTD";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "72c95Bktur4r8t3rh6";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut usize_2: usize = 2860usize;
    let mut u64_0: u64 = 7896u64;
    let mut str_3: &str = "vlw6";
    let mut string_0: std::string::String = std::string::String::from(str_3);
    let mut usize_3: usize = 4072usize;
    let mut usize_4: usize = 636usize;
    let mut error_0: duration::Error = crate::duration::Error::UnknownUnit {start: usize_4, end: usize_3, unit: string_0, value: u64_0};
    let mut error_1: duration::Error = crate::duration::Error::InvalidCharacter(usize_2);
    let mut precision_0: date::Precision = crate::date::Precision::Micros;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_2_ref_0);
    let mut precision_1: date::Precision = crate::date::Precision::Micros;
    let mut precision_2: date::Precision = crate::date::Precision::Smart;
    let mut result_1: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_1_ref_0);
    let mut result_2: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_0_ref_0);
    let mut error_2: duration::Error = crate::duration::Error::Empty;
    let mut precision_3: date::Precision = crate::date::Precision::Micros;
    let mut error_3: duration::Error = crate::duration::Error::NumberExpected(usize_1);
    let mut error_4: date::Error = crate::date::Error::InvalidDigit;
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_1);
    let mut precision_4: date::Precision = crate::date::Precision::Smart;
    let mut error_5: duration::Error = crate::duration::Error::InvalidCharacter(usize_0);
    let mut precision_5: date::Precision = crate::date::Precision::Smart;
    let mut systemtime_1: std::time::SystemTime = std::result::Result::unwrap(result_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2051() {
    rusty_monitor::set_test_id(2051);
    let mut str_0: &str = "Yv5";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut usize_0: usize = 6196usize;
    let mut str_1: &str = "SIv3cJJFWfC";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut usize_1: usize = 4570usize;
    let mut str_2: &str = "2MmI8IkO6g7lVG";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut u64_0: u64 = 9359u64;
    let mut usize_2: usize = 8502usize;
    let mut str_3: &str = "9j1y1R1u9";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut u8_0: u8 = 57u8;
    let mut u8_1: u8 = 54u8;
    let mut result_0: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_1, u8_0);
    let mut result_1: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_3_ref_0);
    let mut error_0: duration::Error = crate::duration::Error::NumberExpected(usize_2);
    let mut error_1: date::Error = crate::date::Error::InvalidFormat;
    let mut u64_1: u64 = std::result::Result::unwrap(result_0);
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_1);
    let mut bool_0: bool = crate::date::is_leap_year(u64_0);
    let mut result_2: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_2_ref_0);
    let mut error_2: date::Error = crate::date::Error::InvalidFormat;
    let mut precision_0: date::Precision = crate::date::Precision::Millis;
    let mut error_3: duration::Error = crate::duration::Error::NumberExpected(usize_1);
    let mut error_4: duration::Error = crate::duration::Error::NumberOverflow;
    let mut systemtime_1: std::time::SystemTime = std::result::Result::unwrap(result_2);
    let mut precision_1: date::Precision = crate::date::Precision::Smart;
    let mut result_3: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_1_ref_0);
    let mut error_5: duration::Error = crate::duration::Error::InvalidCharacter(usize_0);
    let mut result_4: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4484() {
    rusty_monitor::set_test_id(4484);
    let mut str_0: &str = "wyMLCfr";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut char_0: char = '|';
    let mut char_1: char = '6';
    let mut u64_0: u64 = 3788u64;
    let mut u64_1: u64 = 1050u64;
    let mut tuple_0: (u64, u64) = (u64_1, u64_0);
    let mut str_1: &str = "9maqBxBC7oyB7vngBd";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut u64_2: u64 = 2714u64;
    let mut u64_3: u64 = 2441u64;
    let mut tuple_1: (u64, u64) = (u64_3, u64_2);
    let mut str_2: &str = "OAbOF5v";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut usize_0: usize = 2287usize;
    let mut usize_1: usize = 1756usize;
    let mut u64_4: u64 = 6165u64;
    let mut u64_5: u64 = 7920u64;
    let mut u64_6: u64 = 1045u64;
    let mut tuple_2: (u64, u64) = (u64_6, u64_5);
    let mut str_3: &str = "ktk0yJkPUQIysv9";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut precision_0: date::Precision = crate::date::Precision::Nanos;
    let mut option_0: std::option::Option<u64> = crate::date::two_digits_inner(char_1, char_0);
    let mut u64_7: u64 = std::option::Option::unwrap(option_0);
    let mut precision_1: date::Precision = crate::date::Precision::Nanos;
    let mut precision_2: date::Precision = crate::date::Precision::Nanos;
    let mut result_0: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_0_ref_0);
    let mut error_0: date::Error = crate::date::Error::InvalidDigit;
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_0);
    let mut error_1: duration::Error = crate::duration::Error::Empty;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4449() {
    rusty_monitor::set_test_id(4449);
    let mut str_0: &str = "13qwh6Lc";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "o3Dv7nFoan";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "gZnwtmkg2Zue6Sg";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "lC5KZL3";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut u64_0: u64 = 3486u64;
    let mut str_4: &str = "K82fJ97gUcW";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut bool_0: bool = true;
    let mut bool_0_ref_0: &mut bool = &mut bool_0;
    let mut str_5: &str = "";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut u64_1: u64 = 6046u64;
    let mut u64_2: u64 = 5143u64;
    let mut tuple_0: (u64, u64) = (u64_2, u64_1);
    let mut str_6: &str = "pXS60UNSDglxxK0YDJV";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut error_0: duration::Error = crate::duration::Error::NumberOverflow;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_5_ref_0);
    let mut error_1: duration::Error = crate::duration::Error::Empty;
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_3_ref_0);
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_2_ref_0);
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_2);
    let mut precision_0: date::Precision = crate::date::Precision::Nanos;
    let mut result_3: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_1_ref_0);
    let mut error_2: duration::Error = crate::duration::Error::NumberOverflow;
    let mut duration_1: std::time::Duration = std::result::Result::unwrap(result_3);
    let mut result_4: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4957() {
    rusty_monitor::set_test_id(4957);
    let mut usize_0: usize = 8352usize;
    let mut usize_1: usize = 3557usize;
    let mut u64_0: u64 = 6314u64;
    let mut u8_0: u8 = 100u8;
    let mut u8_1: u8 = 18u8;
    let mut str_0: &str = "Pxjt";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "7Wbc1l6R3CIDI0";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut usize_2: usize = 9613usize;
    let mut usize_3: usize = 6525usize;
    let mut precision_0: date::Precision = crate::date::Precision::Nanos;
    let mut error_0: duration::Error = crate::duration::Error::NumberExpected(usize_3);
    let mut error_1: duration::Error = crate::duration::Error::NumberOverflow;
    let mut precision_1: date::Precision = crate::date::Precision::Micros;
    let mut precision_2: date::Precision = crate::date::Precision::Seconds;
    let mut precision_3: date::Precision = crate::date::Precision::Micros;
    let mut error_2: duration::Error = crate::duration::Error::Empty;
    let mut error_3: date::Error = crate::date::Error::InvalidFormat;
    let mut error_4: duration::Error = crate::duration::Error::InvalidCharacter(usize_2);
    let mut result_0: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_1_ref_0);
    let mut result_1: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_0_ref_0);
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_1);
    let mut systemtime_1: std::time::SystemTime = std::result::Result::unwrap(result_0);
    let mut error_5: date::Error = crate::date::Error::InvalidDigit;
    let mut result_2: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_1, u8_0);
    let mut bool_0: bool = crate::date::is_leap_year(u64_0);
    let mut u64_1: u64 = std::result::Result::unwrap(result_2);
    let mut precision_4: date::Precision = crate::date::Precision::Micros;
    let mut error_6: duration::Error = crate::duration::Error::InvalidCharacter(usize_1);
    let mut error_7: duration::Error = crate::duration::Error::InvalidCharacter(usize_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_931() {
    rusty_monitor::set_test_id(931);
    let mut str_0: &str = "fhI8cH7YoZKjjmTHahv";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u8_0: u8 = 61u8;
    let mut u8_1: u8 = 90u8;
    let mut str_1: &str = "65m";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "f6hiSppRD";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut usize_0: usize = 8789usize;
    let mut u64_0: u64 = 99u64;
    let mut usize_1: usize = 3806usize;
    let mut usize_2: usize = 389usize;
    let mut usize_3: usize = 1694usize;
    let mut str_3: &str = "o4MVBB3o6uvb9JB";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut u8_2: u8 = 88u8;
    let mut u8_3: u8 = 103u8;
    let mut str_4: &str = "uganr";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut usize_4: usize = 2203usize;
    let mut u8_4: u8 = 121u8;
    let mut u8_5: u8 = 82u8;
    let mut char_0: char = '';
    let mut char_1: char = 'I';
    let mut u8_6: u8 = 63u8;
    let mut u8_7: u8 = 62u8;
    let mut result_0: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_7, u8_6);
    let mut error_0: duration::Error = crate::duration::Error::Empty;
    let mut option_0: std::option::Option<u64> = crate::date::two_digits_inner(char_1, char_0);
    let mut precision_0: date::Precision = crate::date::Precision::Smart;
    let mut result_1: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_5, u8_4);
    let mut error_1: duration::Error = crate::duration::Error::NumberExpected(usize_4);
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_4_ref_0);
    let mut error_2: duration::Error = crate::duration::Error::Empty;
    let mut precision_1: date::Precision = crate::date::Precision::Seconds;
    let mut result_3: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_3, u8_2);
    let mut precision_2: date::Precision = crate::date::Precision::Nanos;
    let mut result_4: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_3_ref_0);
    let mut precision_3: date::Precision = crate::date::Precision::Smart;
    let mut error_3: duration::Error = crate::duration::Error::InvalidCharacter(usize_3);
    let mut precision_4: date::Precision = crate::date::Precision::Smart;
    let mut precision_5: date::Precision = crate::date::Precision::Millis;
    let mut precision_6: date::Precision = crate::date::Precision::Smart;
    let mut precision_7: date::Precision = crate::date::Precision::Smart;
    let mut error_4: duration::Error = crate::duration::Error::InvalidCharacter(usize_0);
    let mut error_5: date::Error = crate::date::Error::InvalidDigit;
    let mut error_6: duration::Error = crate::duration::Error::Empty;
    let mut result_5: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_2_ref_0);
    let mut result_6: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_1_ref_0);
    let mut error_7: date::Error = crate::date::Error::OutOfRange;
    let mut result_7: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_1, u8_0);
    let mut error_8: date::Error = crate::date::Error::InvalidFormat;
    let mut precision_8: date::Precision = crate::date::Precision::Nanos;
    let mut u64_1: u64 = std::result::Result::unwrap(result_3);
    let mut u64_2: u64 = std::result::Result::unwrap(result_7);
    let mut u64_3: u64 = std::result::Result::unwrap(result_0);
    let mut u64_4: u64 = std::result::Result::unwrap(result_1);
    let mut precision_9: date::Precision = crate::date::Precision::Millis;
    let mut precision_10: date::Precision = crate::date::Precision::Micros;
    let mut result_8: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3093() {
    rusty_monitor::set_test_id(3093);
    let mut char_0: char = '|';
    let mut char_1: char = '}';
    let mut u8_0: u8 = 31u8;
    let mut u8_1: u8 = 83u8;
    let mut usize_0: usize = 9817usize;
    let mut char_2: char = '8';
    let mut char_3: char = '\'';
    let mut str_0: &str = "fxhDs79jXyJZ7";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u64_0: u64 = 9773u64;
    let mut usize_1: usize = 4352usize;
    let mut usize_2: usize = 7557usize;
    let mut str_1: &str = "2FlXIZnOaQ3qtkqjYAm";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut u64_1: u64 = 6132u64;
    let mut u64_2: u64 = 9241u64;
    let mut tuple_0: (u64, u64) = (u64_2, u64_1);
    let mut str_2: &str = "mEMQh5byl6hxtjqUPU";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut error_0: duration::Error = crate::duration::Error::NumberOverflow;
    let mut precision_0: date::Precision = crate::date::Precision::Smart;
    let mut result_0: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_1_ref_0);
    let mut result_1: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_0_ref_0);
    let mut error_1: date::Error = crate::date::Error::OutOfRange;
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_1);
    let mut systemtime_1: std::time::SystemTime = std::result::Result::unwrap(result_0);
    let mut option_0: std::option::Option<u64> = crate::date::two_digits_inner(char_3, char_2);
    let mut u64_3: u64 = std::option::Option::unwrap(option_0);
    let mut error_2: duration::Error = crate::duration::Error::NumberOverflow;
    let mut error_3: duration::Error = crate::duration::Error::NumberExpected(usize_0);
    let mut result_2: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_1, u8_0);
    let mut error_4: duration::Error = crate::duration::Error::NumberOverflow;
    let mut precision_1: date::Precision = crate::date::Precision::Seconds;
    let mut option_1: std::option::Option<u64> = crate::date::two_digits_inner(char_1, char_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2617() {
    rusty_monitor::set_test_id(2617);
    let mut char_0: char = 'N';
    let mut char_1: char = 'I';
    let mut usize_0: usize = 3335usize;
    let mut str_0: &str = "uv4csYoYtsTC";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "kqwb7pu22XE3M7GoL";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut u64_0: u64 = 4089u64;
    let mut usize_1: usize = 6112usize;
    let mut usize_2: usize = 2657usize;
    let mut str_3: &str = "p5LvFgQpWIfZ";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut precision_0: date::Precision = crate::date::Precision::Seconds;
    let mut result_0: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_3_ref_0);
    let mut precision_1: date::Precision = crate::date::Precision::Nanos;
    let mut precision_2: date::Precision = crate::date::Precision::Millis;
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_2_ref_0);
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_1);
    let mut result_2: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_1_ref_0);
    let mut result_3: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_0_ref_0);
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_0);
    let mut error_0: duration::Error = crate::duration::Error::NumberExpected(usize_0);
    let mut error_1: date::Error = crate::date::Error::InvalidDigit;
    let mut option_0: std::option::Option<u64> = crate::date::two_digits_inner(char_1, char_0);
    let mut precision_3: date::Precision = crate::date::Precision::Seconds;
    let mut error_2: duration::Error = crate::duration::Error::NumberOverflow;
    let mut error_3: duration::Error = crate::duration::Error::Empty;
    let mut error_4: duration::Error = crate::duration::Error::Empty;
    let mut error_5: date::Error = crate::date::Error::OutOfRange;
    let mut error_6: date::Error = crate::date::Error::OutOfRange;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2320() {
    rusty_monitor::set_test_id(2320);
    let mut str_0: &str = "kIyW";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "cFlo3Jra4RqR";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut usize_0: usize = 4215usize;
    let mut usize_1: usize = 7834usize;
    let mut char_0: char = 'N';
    let mut char_1: char = 'M';
    let mut char_2: char = '-';
    let mut char_3: char = '=';
    let mut u64_0: u64 = 7867u64;
    let mut str_2: &str = "5LBiddma";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "sCeajAfW0Gm2";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut precision_0: date::Precision = crate::date::Precision::Smart;
    let mut result_0: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_3_ref_0);
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_2_ref_0);
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_1);
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_0);
    let mut precision_1: date::Precision = crate::date::Precision::Smart;
    let mut bool_0: bool = crate::date::is_leap_year(u64_0);
    let mut option_0: std::option::Option<u64> = crate::date::two_digits_inner(char_3, char_2);
    let mut option_1: std::option::Option<u64> = crate::date::two_digits_inner(char_1, char_0);
    let mut error_0: duration::Error = crate::duration::Error::InvalidCharacter(usize_1);
    let mut error_1: duration::Error = crate::duration::Error::NumberExpected(usize_0);
    let mut error_2: date::Error = crate::date::Error::InvalidDigit;
    let mut precision_2: date::Precision = crate::date::Precision::Nanos;
    let mut precision_3: date::Precision = crate::date::Precision::Micros;
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_1_ref_0);
    let mut result_3: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1703() {
    rusty_monitor::set_test_id(1703);
    let mut u8_0: u8 = 15u8;
    let mut u8_1: u8 = 70u8;
    let mut usize_0: usize = 3525usize;
    let mut usize_1: usize = 9844usize;
    let mut char_0: char = '!';
    let mut char_1: char = '!';
    let mut str_0: &str = "LvJzJp";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u8_2: u8 = 125u8;
    let mut u8_3: u8 = 124u8;
    let mut usize_2: usize = 3025usize;
    let mut str_1: &str = "7y3w324pq";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut u64_0: u64 = 2995u64;
    let mut u64_1: u64 = 1597u64;
    let mut tuple_0: (u64, u64) = (u64_1, u64_0);
    let mut str_2: &str = "KeyoPss0OnxyQ28O";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut error_0: date::Error = crate::date::Error::OutOfRange;
    let mut error_1: duration::Error = crate::duration::Error::Empty;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_1_ref_0);
    let mut error_2: duration::Error = crate::duration::Error::InvalidCharacter(usize_2);
    let mut error_3: duration::Error = crate::duration::Error::Empty;
    let mut precision_0: date::Precision = crate::date::Precision::Nanos;
    let mut result_1: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_3, u8_2);
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_0_ref_0);
    let mut error_4: duration::Error = crate::duration::Error::NumberOverflow;
    let mut error_5: date::Error = crate::date::Error::InvalidFormat;
    let mut precision_1: date::Precision = crate::date::Precision::Smart;
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_2);
    let mut error_6: duration::Error = crate::duration::Error::Empty;
    let mut option_0: std::option::Option<u64> = crate::date::two_digits_inner(char_1, char_0);
    let mut error_7: date::Error = crate::date::Error::OutOfRange;
    let mut duration_1: std::time::Duration = std::result::Result::unwrap(result_0);
    let mut error_8: date::Error = crate::date::Error::OutOfRange;
    let mut precision_2: date::Precision = crate::date::Precision::Smart;
    let mut error_9: duration::Error = crate::duration::Error::InvalidCharacter(usize_1);
    let mut error_10: duration::Error = crate::duration::Error::InvalidCharacter(usize_0);
    let mut u64_2: u64 = std::option::Option::unwrap(option_0);
    let mut result_3: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_1, u8_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4644() {
    rusty_monitor::set_test_id(4644);
    let mut usize_0: usize = 3855usize;
    let mut str_0: &str = "mVp";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u64_0: u64 = 4799u64;
    let mut u64_1: u64 = 7428u64;
    let mut u64_2: u64 = 2770u64;
    let mut str_1: &str = "VTE34";
    let mut string_0: std::string::String = std::string::String::from(str_1);
    let mut usize_1: usize = 1641usize;
    let mut usize_2: usize = 2422usize;
    let mut str_2: &str = "UIK0ibNAz6gNiHUZ";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "58nRrdyuP22WZ";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut u64_3: u64 = 8618u64;
    let mut u64_4: u64 = 6215u64;
    let mut tuple_0: (u64, u64) = (u64_4, u64_3);
    let mut str_4: &str = "SSAYRKxr2bCg0ncZE";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut result_0: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_3_ref_0);
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_2_ref_0);
    let mut error_0: date::Error = crate::date::Error::OutOfRange;
    let mut error_1: duration::Error = crate::duration::Error::UnknownUnit {start: usize_2, end: usize_1, unit: string_0, value: u64_2};
    let mut bool_0: bool = crate::date::is_leap_year(u64_1);
    let mut bool_1: bool = crate::date::is_leap_year(u64_0);
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_0_ref_0);
    let mut error_2: date::Error = crate::date::Error::InvalidDigit;
    let mut precision_0: date::Precision = crate::date::Precision::Millis;
    let mut error_3: duration::Error = crate::duration::Error::NumberExpected(usize_0);
    let mut error_4: date::Error = crate::date::Error::OutOfRange;
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4269() {
    rusty_monitor::set_test_id(4269);
    let mut u64_0: u64 = 7249u64;
    let mut usize_0: usize = 4093usize;
    let mut usize_1: usize = 316usize;
    let mut usize_2: usize = 5385usize;
    let mut u8_0: u8 = 101u8;
    let mut u8_1: u8 = 85u8;
    let mut usize_3: usize = 3711usize;
    let mut u64_1: u64 = 600u64;
    let mut usize_4: usize = 8910usize;
    let mut usize_5: usize = 3039usize;
    let mut u64_2: u64 = 4802u64;
    let mut usize_6: usize = 4124usize;
    let mut usize_7: usize = 9481usize;
    let mut error_0: duration::Error = crate::duration::Error::Empty;
    let mut error_1: date::Error = crate::date::Error::OutOfRange;
    let mut error_2: date::Error = crate::date::Error::InvalidDigit;
    let mut precision_0: date::Precision = crate::date::Precision::Nanos;
    let mut error_3: duration::Error = crate::duration::Error::InvalidCharacter(usize_3);
    let mut precision_1: date::Precision = crate::date::Precision::Smart;
    let mut error_4: date::Error = crate::date::Error::InvalidDigit;
    let mut result_0: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_1, u8_0);
    let mut error_5: duration::Error = crate::duration::Error::InvalidCharacter(usize_2);
    let mut u64_3: u64 = std::result::Result::unwrap(result_0);
    let mut error_6: date::Error = crate::date::Error::OutOfRange;
    let mut error_7: duration::Error = crate::duration::Error::NumberOverflow;
    let mut precision_2: date::Precision = crate::date::Precision::Nanos;
    let mut precision_3: date::Precision = crate::date::Precision::Nanos;
    let mut error_8: date::Error = crate::date::Error::InvalidDigit;
    let mut precision_4: date::Precision = crate::date::Precision::Nanos;
    let mut error_9: date::Error = crate::date::Error::OutOfRange;
    let mut precision_5: date::Precision = crate::date::Precision::Seconds;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2029() {
    rusty_monitor::set_test_id(2029);
    let mut char_0: char = ']';
    let mut char_1: char = 'Y';
    let mut str_0: &str = "sP3CNBtDZsYYuFyi";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "J4ob0cIyudr2CYBl0Fk";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "NfxUkWK4n2XwRkuw9xx";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "fO8YWDf5u6yXciQRi95";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut u64_0: u64 = 8551u64;
    let mut u64_1: u64 = 3508u64;
    let mut u64_2: u64 = 1223u64;
    let mut usize_0: usize = 1963usize;
    let mut usize_1: usize = 5748usize;
    let mut str_4: &str = "cDgwP44Du0Jxb9M";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "47m4N9DxREXEeB";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "aeB4ZBZZlmvq8IHbVD";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut usize_2: usize = 5837usize;
    let mut str_7: &str = "Zj3KE";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut u32_0: u32 = 7453u32;
    let mut str_8: &str = "K";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut bool_0: bool = false;
    let mut bool_0_ref_0: &mut bool = &mut bool_0;
    let mut precision_0: date::Precision = crate::date::Precision::Smart;
    let mut error_0: date::Error = crate::date::Error::InvalidFormat;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_7_ref_0);
    let mut error_1: date::Error = crate::date::Error::OutOfRange;
    let mut error_2: duration::Error = crate::duration::Error::NumberExpected(usize_2);
    let mut result_1: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_6_ref_0);
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_5_ref_0);
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_0);
    let mut error_3: date::Error = crate::date::Error::InvalidDigit;
    let mut duration_1: std::time::Duration = std::result::Result::unwrap(result_2);
    let mut result_3: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_4_ref_0);
    let mut bool_1: bool = crate::date::is_leap_year(u64_1);
    let mut error_4: date::Error = crate::date::Error::InvalidDigit;
    let mut bool_2: bool = crate::date::is_leap_year(u64_0);
    let mut result_4: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_3_ref_0);
    let mut precision_1: date::Precision = crate::date::Precision::Millis;
    let mut result_5: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_2_ref_0);
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_4);
    let mut result_6: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_1_ref_0);
    let mut result_7: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_0_ref_0);
    let mut option_0: std::option::Option<u64> = crate::date::two_digits_inner(char_1, char_0);
    panic!("From RustyUnit with love");
}
}