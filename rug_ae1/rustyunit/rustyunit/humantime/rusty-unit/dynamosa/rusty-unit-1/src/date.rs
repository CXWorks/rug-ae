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
fn rusty_test_172() {
    rusty_monitor::set_test_id(172);
    let mut str_0: &str = "M7VgbcR";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "n7hcLPQBoQ4oGRPD7VN";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut result_0: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_0_ref_0);
    let mut error_0: date::Error = crate::date::Error::OutOfRange;
    let mut error_1: duration::Error = crate::duration::Error::Empty;
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_1_ref_0);
    let mut error_2: date::Error = crate::date::Error::OutOfRange;
    let mut precision_0: date::Precision = crate::date::Precision::Smart;
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_1);
    let mut precision_1: date::Precision = crate::date::Precision::Smart;
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_0);
    let mut precision_2: date::Precision = crate::date::Precision::Micros;
    let mut precision_3: date::Precision = crate::date::Precision::Smart;
    let mut error_3: date::Error = crate::date::Error::InvalidDigit;
    let mut precision_4: date::Precision = crate::date::Precision::Micros;
    let mut error_4: date::Error = crate::date::Error::OutOfRange;
    let mut precision_5: date::Precision = crate::date::Precision::Smart;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2344() {
    rusty_monitor::set_test_id(2344);
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "5r";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut char_0: char = '=';
    let mut char_1: char = '7';
    let mut u64_0: u64 = 3490u64;
    let mut u64_1: u64 = 6048u64;
    let mut tuple_0: (u64, u64) = (u64_1, u64_0);
    let mut str_2: &str = "FoENsg";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut option_0: std::option::Option<u64> = crate::date::two_digits_inner(char_1, char_0);
    let mut error_0: date::Error = crate::date::Error::OutOfRange;
    let mut precision_0: date::Precision = crate::date::Precision::Smart;
    let mut result_0: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_2_ref_0);
    let mut precision_1: date::Precision = crate::date::Precision::Seconds;
    let mut precision_2: date::Precision = crate::date::Precision::Millis;
    let mut precision_3: date::Precision = crate::date::Precision::Millis;
    let mut result_1: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_0_ref_0);
    let mut precision_4: date::Precision = crate::date::Precision::Seconds;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_425() {
    rusty_monitor::set_test_id(425);
    let mut str_0: &str = "xFJV0fPG0K";
    let mut str_1: &str = "hKTJdmrYpSJ";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "5ak2zPXcT";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "Rp9ZZA";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_3_ref_0);
    let mut precision_0: date::Precision = crate::date::Precision::Nanos;
    let mut error_0: date::Error = crate::date::Error::OutOfRange;
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_2_ref_0);
    let mut str_0_ref_0: &str = &mut str_0;
    let mut result_2: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_1_ref_0);
    let mut precision_1: date::Precision = crate::date::Precision::Nanos;
    let mut result_3: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1471() {
    rusty_monitor::set_test_id(1471);
    let mut usize_0: usize = 9376usize;
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "5r";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut u64_0: u64 = 3490u64;
    let mut u64_1: u64 = 6048u64;
    let mut tuple_0: (u64, u64) = (u64_1, u64_0);
    let mut str_2: &str = "FoENsg";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut error_0: date::Error = crate::date::Error::OutOfRange;
    let mut precision_0: date::Precision = crate::date::Precision::Smart;
    let mut precision_1: date::Precision = crate::date::Precision::Micros;
    let mut precision_2: date::Precision = crate::date::Precision::Millis;
    let mut error_1: date::Error = crate::date::Error::InvalidFormat;
    let mut precision_3: date::Precision = crate::date::Precision::Seconds;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_1_ref_0);
    let mut error_2: duration::Error = crate::duration::Error::NumberOverflow;
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_0_ref_0);
    let mut error_3: duration::Error = crate::duration::Error::Empty;
    let mut error_4: date::Error = crate::date::Error::InvalidDigit;
    let mut error_5: duration::Error = crate::duration::Error::InvalidCharacter(usize_0);
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_636() {
    rusty_monitor::set_test_id(636);
    let mut u64_0: u64 = 8802u64;
    let mut usize_0: usize = 3235usize;
    let mut str_0: &str = "";
    let mut str_1: &str = "WMHSg9yQxJaNgcQrj8";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "hyx6g8Gr6";
    let mut str_3: &str = "UuuibIRInX";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut result_0: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_1_ref_0);
    let mut str_4: &str = "2M9oXfQ6sGLDVCvrs";
    let mut str_5: &str = "96mrOcC";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut u64_1: u64 = 4816u64;
    let mut str_6: &str = "IureqQ9xj4p";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut u64_2: u64 = 9057u64;
    let mut tuple_0: (u64, u64) = (u64_2, u64_1);
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7: &str = "SEn1LRecN";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_8: &str = "8fXMt7Do6GiNwePNul";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_8_ref_0: &str = &mut str_8;
    let mut error_0: date::Error = crate::date::Error::OutOfRange;
    let mut str_0_ref_0: &str = &mut str_0;
    let mut error_1: duration::Error = crate::duration::Error::InvalidCharacter(usize_0);
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_4_ref_0);
    let mut bool_0: bool = crate::date::is_leap_year(u64_0);
    let mut result_2: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_0_ref_0);
    let mut precision_0: date::Precision = crate::date::Precision::Seconds;
    let mut precision_1: date::Precision = crate::date::Precision::Millis;
    let mut precision_2: date::Precision = crate::date::Precision::Millis;
    let mut result_3: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_8_ref_0);
    let mut precision_3: date::Precision = crate::date::Precision::Seconds;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4510() {
    rusty_monitor::set_test_id(4510);
    let mut str_0: &str = "9fopNRkzhtaqWp";
    let mut u64_0: u64 = 1505u64;
    let mut option_0: std::option::Option<u64> = std::option::Option::Some(u64_0);
    let mut char_0: char = ')';
    let mut u8_0: u8 = 103u8;
    let mut u8_1: u8 = 40u8;
    let mut char_1: char = '=';
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_1: std::option::Option<u64> = crate::date::two_digits_inner(char_0, char_1);
    let mut u64_1: u64 = std::option::Option::unwrap(option_0);
    let mut error_0: date::Error = crate::date::Error::OutOfRange;
    let mut precision_0: date::Precision = crate::date::Precision::Smart;
    let mut result_0: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_1, u8_0);
    let mut precision_1: date::Precision = crate::date::Precision::Micros;
    let mut u64_2: u64 = std::result::Result::unwrap(result_0);
    let mut precision_2: date::Precision = crate::date::Precision::Millis;
    let mut error_1: date::Error = crate::date::Error::InvalidFormat;
    let mut precision_3: date::Precision = crate::date::Precision::Seconds;
    let mut error_2: date::Error = crate::date::Error::InvalidDigit;
    let mut precision_4: date::Precision = crate::date::Precision::Seconds;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3201() {
    rusty_monitor::set_test_id(3201);
    let mut error_0: duration::Error = crate::duration::Error::NumberOverflow;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = std::result::Result::Err(error_0);
    let mut str_0: &str = "wNc";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "d9p1C30c7xEz";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "RA";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut u64_0: u64 = 2974u64;
    let mut char_0: char = ':';
    let mut char_1: char = ')';
    let mut option_0: std::option::Option<u64> = crate::date::two_digits_inner(char_1, char_0);
    let mut str_3: &str = "EFaA9qPAdCDpk8o";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut result_1: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_3_ref_0);
    let mut str_4: &str = "XQIvKeiwnjHsmy";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut u64_1: u64 = 8716u64;
    let mut u64_2: u64 = 310u64;
    let mut tuple_0: (u64, u64) = (u64_2, u64_1);
    let mut str_5: &str = "7XJOLxd0";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "5";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7: &str = "v0bu37fBbXcySQ3QZsI";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut usize_0: usize = 9220usize;
    let mut u64_3: u64 = 7505u64;
    let mut u64_4: u64 = 5143u64;
    let mut tuple_1: (u64, u64) = (u64_4, u64_3);
    let mut str_8: &str = "VMp";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut error_1: duration::Error = crate::duration::Error::NumberExpected(usize_0);
    let mut result_2: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_7_ref_0);
    let mut result_3: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_6_ref_0);
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_0);
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_2);
    let mut precision_0: date::Precision = crate::date::Precision::Millis;
    let mut bool_0: bool = crate::date::is_leap_year(u64_0);
    let mut precision_1: date::Precision = crate::date::Precision::Smart;
    let mut error_2: date::Error = crate::date::Error::OutOfRange;
    let mut result_4: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_1_ref_0);
    let mut error_3: duration::Error = crate::duration::Error::NumberOverflow;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4017() {
    rusty_monitor::set_test_id(4017);
    let mut usize_0: usize = 5589usize;
    let mut error_0: duration::Error = crate::duration::Error::InvalidCharacter(usize_0);
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = std::result::Result::Err(error_0);
    let mut str_0: &str = "RA";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "0POmiQe7";
    let mut char_0: char = ':';
    let mut char_1: char = ')';
    let mut option_0: std::option::Option<u64> = crate::date::two_digits_inner(char_1, char_0);
    let mut str_2: &str = "EFaA9qPAdCDpk8o";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut result_1: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_2_ref_0);
    let mut str_3: &str = "XQIvKeiwnjHsmy";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut u64_0: u64 = 8716u64;
    let mut u64_1: u64 = 310u64;
    let mut tuple_0: (u64, u64) = (u64_1, u64_0);
    let mut str_4: &str = "7XJOLxd0";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "5";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "v0bu37fBbXcySQ3QZsI";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut usize_1: usize = 9220usize;
    let mut u64_2: u64 = 7505u64;
    let mut u64_3: u64 = 5143u64;
    let mut tuple_1: (u64, u64) = (u64_3, u64_2);
    let mut str_7: &str = "VMp";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut error_1: duration::Error = crate::duration::Error::NumberExpected(usize_1);
    let mut result_2: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_6_ref_0);
    let mut error_2: date::Error = crate::date::Error::InvalidFormat;
    let mut str_1_ref_0: &str = &mut str_1;
    let mut error_3: date::Error = crate::date::Error::OutOfRange;
    let mut result_3: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_3_ref_0);
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_0);
    let mut precision_0: date::Precision = crate::date::Precision::Seconds;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4001() {
    rusty_monitor::set_test_id(4001);
    let mut str_0: &str = "9h";
    let mut str_1: &str = "czD";
    let mut u64_0: u64 = 1505u64;
    let mut option_0: std::option::Option<u64> = std::option::Option::Some(u64_0);
    let mut char_0: char = '%';
    let mut char_1: char = ')';
    let mut option_1: std::option::Option<u64> = crate::date::two_digits_inner(char_1, char_0);
    let mut str_2: &str = "EFaA9qPAdCDpk8o";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut result_0: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_2_ref_0);
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_3: &str = "zcCB591wj2";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_4: &str = "v0bu37fBbXcySQ3QZsI";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut usize_0: usize = 9220usize;
    let mut str_5: &str = "jl";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut error_0: duration::Error = crate::duration::Error::NumberExpected(usize_0);
    let mut result_1: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_4_ref_0);
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_0_ref_0);
    let mut error_1: date::Error = crate::date::Error::InvalidFormat;
    let mut u64_1: u64 = std::option::Option::unwrap(option_0);
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_0);
    let mut error_2: date::Error = crate::date::Error::InvalidFormat;
    let mut result_3: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_1_ref_0);
    let mut precision_0: date::Precision = crate::date::Precision::Seconds;
    let mut precision_1: date::Precision = crate::date::Precision::Seconds;
    let mut u64_2: u64 = std::option::Option::unwrap(option_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_128() {
    rusty_monitor::set_test_id(128);
    let mut error_0: date::Error = crate::date::Error::OutOfRange;
    let mut result_0: std::result::Result<std::time::SystemTime, date::Error> = std::result::Result::Err(error_0);
    let mut str_0: &str = "hG7mZ0fzgk";
    let mut str_1: &str = "ka7";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut precision_0: date::Precision = crate::date::Precision::Nanos;
    let mut str_0_ref_0: &str = &mut str_0;
    let mut error_1: date::Error = crate::date::Error::InvalidFormat;
    let mut error_2: date::Error = crate::date::Error::InvalidFormat;
    let mut error_3: date::Error = crate::date::Error::InvalidDigit;
    let mut precision_1: date::Precision = crate::date::Precision::Millis;
    let mut result_1: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_1_ref_0);
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_0);
    let mut precision_2: date::Precision = crate::date::Precision::Millis;
    let mut error_4: date::Error = crate::date::Error::InvalidDigit;
    let mut error_5: date::Error = crate::date::Error::InvalidFormat;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7726() {
    rusty_monitor::set_test_id(7726);
    let mut str_0: &str = "9d";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_0_ref_0);
    let mut u64_0: u64 = 1505u64;
    let mut option_0: std::option::Option<u64> = std::option::Option::Some(u64_0);
    let mut char_0: char = ':';
    let mut char_1: char = ')';
    let mut option_1: std::option::Option<u64> = crate::date::two_digits_inner(char_1, char_0);
    let mut str_1: &str = "EFaA9qPAdCDpk8o";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut result_1: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_1_ref_0);
    let mut str_2: &str = "XQIvKeiwnjHsmy";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "zcCB591wj2";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "5";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "v0bu37fBbXcySQ3QZsI";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut usize_0: usize = 9220usize;
    let mut u64_1: u64 = 7505u64;
    let mut u64_2: u64 = 5143u64;
    let mut tuple_0: (u64, u64) = (u64_2, u64_1);
    let mut str_6: &str = "VMp";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut error_0: duration::Error = crate::duration::Error::NumberExpected(usize_0);
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_0);
    let mut error_1: date::Error = crate::date::Error::InvalidFormat;
    let mut result_2: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_6_ref_0);
    let mut u64_3: u64 = std::option::Option::unwrap(option_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_227() {
    rusty_monitor::set_test_id(227);
    let mut str_0: &str = "M7VgbcR";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u64_0: u64 = 124u64;
    let mut str_1: &str = "n7hcLPQBoQ4oGRPD7VN";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "LhMlZhRlDxpDSfjVhZ";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut result_0: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_2_ref_0);
    let mut error_0: date::Error = crate::date::Error::OutOfRange;
    let mut precision_0: date::Precision = crate::date::Precision::Smart;
    let mut error_1: date::Error = crate::date::Error::InvalidFormat;
    let mut precision_1: date::Precision = crate::date::Precision::Seconds;
    let mut bool_0: bool = crate::date::is_leap_year(u64_0);
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2170() {
    rusty_monitor::set_test_id(2170);
    let mut str_0: &str = "ey";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut char_0: char = 'f';
    let mut char_1: char = '`';
    let mut option_0: std::option::Option<u64> = crate::date::two_digits_inner(char_1, char_0);
    let mut u64_0: u64 = 9420u64;
    let mut option_1: std::option::Option<u64> = std::option::Option::Some(u64_0);
    let mut u64_1: u64 = 8150u64;
    let mut bool_0: bool = true;
    let mut str_1: &str = "M8aGZh";
    let mut str_2: &str = "frbihqSot2McHQjdqOE";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "hKTJdmrYpSJ";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut u8_0: u8 = 51u8;
    let mut u8_1: u8 = 57u8;
    let mut u64_2: u64 = 7154u64;
    let mut u64_3: u64 = 8562u64;
    let mut tuple_0: (u64, u64) = (u64_1, u64_2);
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_4: &str = "Rp9ZZA";
    let mut precision_0: date::Precision = crate::date::Precision::Nanos;
    let mut error_0: date::Error = crate::date::Error::OutOfRange;
    let mut result_0: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_0, u8_1);
    let mut str_5: &str = "3HuscPiWR";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5_ref_0: &str = &mut str_5;
    let mut bool_0_ref_0: &mut bool = &mut bool_0;
    let mut error_1: date::Error = crate::date::Error::OutOfRange;
    let mut result_1: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_3_ref_0);
    let mut precision_1: date::Precision = crate::date::Precision::Millis;
    let mut bool_1: bool = crate::date::is_leap_year(u64_3);
    let mut precision_2: date::Precision = crate::date::Precision::Smart;
    let mut u64_4: u64 = std::option::Option::unwrap(option_1);
    let mut error_2: date::Error = crate::date::Error::OutOfRange;
    let mut result_2: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_318() {
    rusty_monitor::set_test_id(318);
    let mut str_0: &str = "PZxm1M3h8z";
    let mut str_1: &str = "";
    let mut str_2: &str = "";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_3: &str = "8uoF78TWxPRwK";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_3_ref_0: &str = &mut str_3;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_0_ref_0);
    let mut precision_0: date::Precision = crate::date::Precision::Nanos;
    let mut error_0: date::Error = crate::date::Error::OutOfRange;
    let mut result_1: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_2_ref_0);
    let mut precision_1: date::Precision = crate::date::Precision::Micros;
    let mut precision_2: date::Precision = crate::date::Precision::Nanos;
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_1);
    let mut precision_3: date::Precision = crate::date::Precision::Nanos;
    let mut error_1: duration::Error = crate::duration::Error::Empty;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1959() {
    rusty_monitor::set_test_id(1959);
    let mut char_0: char = 'f';
    let mut char_1: char = '`';
    let mut option_0: std::option::Option<u64> = crate::date::two_digits_inner(char_1, char_0);
    let mut u64_0: u64 = 9420u64;
    let mut option_1: std::option::Option<u64> = std::option::Option::Some(u64_0);
    let mut u64_1: u64 = 8150u64;
    let mut str_0: &str = "M8aGZh";
    let mut str_1: &str = "frbihqSot2McHQjdqOE";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut result_0: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_1_ref_0);
    let mut str_2: &str = "hKTJdmrYpSJ";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_0: bool = crate::date::is_leap_year(u64_1);
    let mut result_1: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_2_ref_0);
    let mut precision_0: date::Precision = crate::date::Precision::Seconds;
    let mut precision_1: date::Precision = crate::date::Precision::Millis;
    let mut precision_2: date::Precision = crate::date::Precision::Millis;
    let mut result_2: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_0_ref_0);
    let mut precision_3: date::Precision = crate::date::Precision::Seconds;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_375() {
    rusty_monitor::set_test_id(375);
    let mut u64_0: u64 = 2340u64;
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut precision_0: date::Precision = crate::date::Precision::Micros;
    let mut result_0: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_0_ref_0);
    let mut bool_0: bool = crate::date::is_leap_year(u64_0);
    let mut precision_1: date::Precision = crate::date::Precision::Nanos;
    let mut error_0: date::Error = crate::date::Error::OutOfRange;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_546() {
    rusty_monitor::set_test_id(546);
    let mut str_0: &str = "UuuibIRInX";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut result_0: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_0_ref_0);
    let mut str_1: &str = "r3jZCPLvDl46h";
    let mut str_2: &str = "96mrOcC";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut u64_0: u64 = 4816u64;
    let mut str_3: &str = "IureqQ9xj4p";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut u64_1: u64 = 9057u64;
    let mut tuple_0: (u64, u64) = (u64_1, u64_0);
    let mut str_4: &str = "AAUCBgAN08b69gjNzZq";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_5: &str = "SEn1LRecN";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5_ref_0: &str = &mut str_5;
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_2_ref_0);
    let mut result_2: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_3_ref_0);
    let mut precision_0: date::Precision = crate::date::Precision::Micros;
    let mut precision_1: date::Precision = crate::date::Precision::Nanos;
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_0);
    let mut precision_2: date::Precision = crate::date::Precision::Nanos;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_345() {
    rusty_monitor::set_test_id(345);
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u8_0: u8 = 51u8;
    let mut u8_1: u8 = 26u8;
    let mut str_1: &str = "qsVQFrzFAgmNdIsNs3";
    let mut str_2: &str = "8uoF78TWxPRwK";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2_ref_0: &str = &mut str_2;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_0_ref_0);
    let mut precision_0: date::Precision = crate::date::Precision::Nanos;
    let mut error_0: date::Error = crate::date::Error::OutOfRange;
    let mut precision_1: date::Precision = crate::date::Precision::Micros;
    let mut result_1: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_1, u8_0);
    let mut precision_2: date::Precision = crate::date::Precision::Micros;
    let mut precision_3: date::Precision = crate::date::Precision::Nanos;
    let mut result_2: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_286() {
    rusty_monitor::set_test_id(286);
    let mut str_0: &str = "K8XsglqJliOO";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_0_ref_0);
    let mut error_0: duration::Error = crate::duration::Error::Empty;
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = std::result::Result::Err(error_0);
    let mut u8_0: u8 = 112u8;
    let mut u8_1: u8 = 71u8;
    let mut char_0: char = 'z';
    let mut char_1: char = '8';
    let mut str_1: &str = "aypYtM1PuP";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "RQfHjeyMQuQd1";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut u8_2: u8 = 117u8;
    let mut u8_3: u8 = 18u8;
    let mut result_2: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_3, u8_2);
    let mut option_0: std::option::Option<u64> = crate::date::two_digits_inner(char_0, char_1);
    let mut result_3: std::result::Result<u64, date::Error> = crate::date::two_digits(u8_1, u8_0);
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_1);
    let mut precision_0: date::Precision = crate::date::Precision::Nanos;
    let mut result_4: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_2_ref_0);
    let mut error_1: date::Error = crate::date::Error::OutOfRange;
    let mut error_2: date::Error = crate::date::Error::InvalidFormat;
    let mut u64_0: u64 = std::result::Result::unwrap(result_3);
    let mut duration_1: std::time::Duration = std::result::Result::unwrap(result_0);
    let mut u64_1: u64 = std::option::Option::unwrap(option_0);
    panic!("From RustyUnit with love");
}
}