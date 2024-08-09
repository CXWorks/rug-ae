use std::error::Error as StdError;
use std::fmt;
use std::str::Chars;
use std::time::Duration;

/// Error parsing human-friendly duration
#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    /// Invalid character during parsing
    ///
    /// More specifically anything that is not alphanumeric is prohibited
    ///
    /// The field is an byte offset of the character in the string.
    InvalidCharacter(usize),
    /// Non-numeric value where number is expected
    ///
    /// This usually means that either time unit is broken into words,
    /// e.g. `m sec` instead of `msec`, or just number is omitted,
    /// for example `2 hours min` instead of `2 hours 1 min`
    ///
    /// The field is an byte offset of the errorneous character
    /// in the string.
    NumberExpected(usize),
    /// Unit in the number is not one of allowed units
    ///
    /// See documentation of `parse_duration` for the list of supported
    /// time units.
    ///
    /// The two fields are start and end (exclusive) of the slice from
    /// the original string, containing errorneous value
    UnknownUnit {
        /// Start of the invalid unit inside the original string
        start: usize,
        /// End of the invalid unit inside the original string
        end: usize,
        /// The unit verbatim
        unit: String,
        /// A number associated with the unit
        value: u64,
    },
    /// The numeric value is too large
    ///
    /// Usually this means value is too large to be useful. If user writes
    /// data in subsecond units, then the maximum is about 3k years. When
    /// using seconds, or larger units, the limit is even larger.
    NumberOverflow,
    /// The value was an empty string (or consists only whitespace)
    Empty,
}

impl StdError for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidCharacter(offset) => write!(f, "invalid character at {}", offset),
            Error::NumberExpected(offset) => write!(f, "expected number at {}", offset),
            Error::UnknownUnit { unit, value, .. } if unit.is_empty() => {
                write!(f,
                    "time unit needed, for example {0}sec or {0}ms",
                    value,
                )
            }
            Error::UnknownUnit { unit, .. } => {
                write!(
                    f,
                    "unknown time unit {:?}, \
                    supported units: ns, us, ms, sec, min, hours, days, \
                    weeks, months, years (and few variations)",
                    unit
                )
            }
            Error::NumberOverflow => write!(f, "number is too large"),
            Error::Empty => write!(f, "value was empty"),
        }
    }
}

/// A wrapper type that allows you to Display a Duration
#[derive(Debug, Clone)]
pub struct FormattedDuration(Duration);

trait OverflowOp: Sized {
    fn mul(self, other: Self) -> Result<Self, Error>;
    fn add(self, other: Self) -> Result<Self, Error>;
}

impl OverflowOp for u64 {
    fn mul(self, other: Self) -> Result<Self, Error> {
        self.checked_mul(other).ok_or(Error::NumberOverflow)
    }
    fn add(self, other: Self) -> Result<Self, Error> {
        self.checked_add(other).ok_or(Error::NumberOverflow)
    }
}

struct Parser<'a> {
    iter: Chars<'a>,
    src: &'a str,
    current: (u64, u64),
}

impl<'a> Parser<'a> {
    fn off(&self) -> usize {
        self.src.len() - self.iter.as_str().len()
    }

    fn parse_first_char(&mut self) -> Result<Option<u64>, Error> {
        let off = self.off();
        for c in self.iter.by_ref() {
            match c {
                '0'..='9' => {
                    return Ok(Some(c as u64 - '0' as u64));
                }
                c if c.is_whitespace() => continue,
                _ => {
                    return Err(Error::NumberExpected(off));
                }
            }
        }
        Ok(None)
    }
    fn parse_unit(&mut self, n: u64, start: usize, end: usize)
        -> Result<(), Error>
    {
        let (mut sec, nsec) = match &self.src[start..end] {
            "nanos" | "nsec" | "ns" => (0u64, n),
            "usec" | "us" => (0u64, n.mul(1000)?),
            "millis" | "msec" | "ms" => (0u64, n.mul(1_000_000)?),
            "seconds" | "second" | "secs" | "sec" | "s" => (n, 0),
            "minutes" | "minute" | "min" | "mins" | "m"
            => (n.mul(60)?, 0),
            "hours" | "hour" | "hr" | "hrs" | "h" => (n.mul(3600)?, 0),
            "days" | "day" | "d" => (n.mul(86400)?, 0),
            "weeks" | "week" | "w" => (n.mul(86400*7)?, 0),
            "months" | "month" | "M" => (n.mul(2_630_016)?, 0), // 30.44d
            "years" | "year" | "y" => (n.mul(31_557_600)?, 0), // 365.25d
            _ => {
                return Err(Error::UnknownUnit {
                    start, end,
                    unit: self.src[start..end].to_string(),
                    value: n,
                });
            }
        };
        let mut nsec = self.current.1.add(nsec)?;
        if nsec > 1_000_000_000 {
            sec = sec.add(nsec / 1_000_000_000)?;
            nsec %= 1_000_000_000;
        }
        sec = self.current.0.add(sec)?;
        self.current = (sec, nsec);
        Ok(())
    }

    fn parse(mut self) -> Result<Duration, Error> {
        let mut n = self.parse_first_char()?.ok_or(Error::Empty)?;
        'outer: loop {
            let mut off = self.off();
            while let Some(c) = self.iter.next() {
                match c {
                    '0'..='9' => {
                        n = n.checked_mul(10)
                            .and_then(|x| x.checked_add(c as u64 - '0' as u64))
                            .ok_or(Error::NumberOverflow)?;
                    }
                    c if c.is_whitespace() => {}
                    'a'..='z' | 'A'..='Z' => {
                        break;
                    }
                    _ => {
                        return Err(Error::InvalidCharacter(off));
                    }
                }
                off = self.off();
            }
            let start = off;
            let mut off = self.off();
            while let Some(c) = self.iter.next() {
                match c {
                    '0'..='9' => {
                        self.parse_unit(n, start, off)?;
                        n = c as u64 - '0' as u64;
                        continue 'outer;
                    }
                    c if c.is_whitespace() => break,
                    'a'..='z' | 'A'..='Z' => {}
                    _ => {
                        return Err(Error::InvalidCharacter(off));
                    }
                }
                off = self.off();
            }
            self.parse_unit(n, start, off)?;
            n = match self.parse_first_char()? {
                Some(n) => n,
                None => return Ok(
                    Duration::new(self.current.0, self.current.1 as u32)),
            };
        }
    }

}

/// Parse duration object `1hour 12min 5s`
///
/// The duration object is a concatenation of time spans. Where each time
/// span is an integer number and a suffix. Supported suffixes:
///
/// * `nsec`, `ns` -- nanoseconds
/// * `usec`, `us` -- microseconds
/// * `msec`, `ms` -- milliseconds
/// * `seconds`, `second`, `sec`, `s`
/// * `minutes`, `minute`, `min`, `m`
/// * `hours`, `hour`, `hr`, `h`
/// * `days`, `day`, `d`
/// * `weeks`, `week`, `w`
/// * `months`, `month`, `M` -- defined as 30.44 days
/// * `years`, `year`, `y` -- defined as 365.25 days
///
/// # Examples
///
/// ```
/// use std::time::Duration;
/// use humantime::parse_duration;
///
/// assert_eq!(parse_duration("2h 37min"), Ok(Duration::new(9420, 0)));
/// assert_eq!(parse_duration("32ms"), Ok(Duration::new(0, 32_000_000)));
/// ```
pub fn parse_duration(s: &str) -> Result<Duration, Error> {
    Parser {
        iter: s.chars(),
        src: s,
        current: (0, 0),
    }.parse()
}

/// Formats duration into a human-readable string
///
/// Note: this format is guaranteed to have same value when using
/// parse_duration, but we can change some details of the exact composition
/// of the value.
///
/// # Examples
///
/// ```
/// use std::time::Duration;
/// use humantime::format_duration;
///
/// let val1 = Duration::new(9420, 0);
/// assert_eq!(format_duration(val1).to_string(), "2h 37m");
/// let val2 = Duration::new(0, 32_000_000);
/// assert_eq!(format_duration(val2).to_string(), "32ms");
/// ```
pub fn format_duration(val: Duration) -> FormattedDuration {
    FormattedDuration(val)
}

fn item_plural(f: &mut fmt::Formatter, started: &mut bool,
    name: &str, value: u64)
    -> fmt::Result
{
    if value > 0 {
        if *started {
            f.write_str(" ")?;
        }
        write!(f, "{}{}", value, name)?;
        if value > 1 {
            f.write_str("s")?;
        }
        *started = true;
    }
    Ok(())
}
fn item(f: &mut fmt::Formatter, started: &mut bool, name: &str, value: u32)
    -> fmt::Result
{
    if value > 0 {
        if *started {
            f.write_str(" ")?;
        }
        write!(f, "{}{}", value, name)?;
        *started = true;
    }
    Ok(())
}

impl FormattedDuration {
    /// Returns a reference to the [`Duration`][] that is being formatted.
    pub fn get_ref(&self) -> &Duration {
        &self.0
    }
}

impl fmt::Display for FormattedDuration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let secs = self.0.as_secs();
        let nanos = self.0.subsec_nanos();

        if secs == 0 && nanos == 0 {
            f.write_str("0s")?;
            return Ok(());
        }

        let years = secs / 31_557_600;  // 365.25d
        let ydays = secs % 31_557_600;
        let months = ydays / 2_630_016;  // 30.44d
        let mdays = ydays % 2_630_016;
        let days = mdays / 86400;
        let day_secs = mdays % 86400;
        let hours = day_secs / 3600;
        let minutes = day_secs % 3600 / 60;
        let seconds = day_secs % 60;

        let millis = nanos / 1_000_000;
        let micros = nanos / 1000 % 1000;
        let nanosec = nanos % 1000;

        let started = &mut false;
        item_plural(f, started, "year", years)?;
        item_plural(f, started, "month", months)?;
        item_plural(f, started, "day", days)?;
        item(f, started, "h", hours as u32)?;
        item(f, started, "m", minutes as u32)?;
        item(f, started, "s", seconds as u32)?;
        item(f, started, "ms", millis)?;
        item(f, started, "us", micros)?;
        item(f, started, "ns", nanosec)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use rand::Rng;

    use super::{parse_duration, format_duration};
    use super::Error;

    #[test]
    #[allow(clippy::cognitive_complexity)]
    fn test_units() {
        assert_eq!(parse_duration("17nsec"), Ok(Duration::new(0, 17)));
        assert_eq!(parse_duration("17nanos"), Ok(Duration::new(0, 17)));
        assert_eq!(parse_duration("33ns"), Ok(Duration::new(0, 33)));
        assert_eq!(parse_duration("3usec"), Ok(Duration::new(0, 3000)));
        assert_eq!(parse_duration("78us"), Ok(Duration::new(0, 78000)));
        assert_eq!(parse_duration("31msec"), Ok(Duration::new(0, 31_000_000)));
        assert_eq!(parse_duration("31millis"), Ok(Duration::new(0, 31_000_000)));
        assert_eq!(parse_duration("6ms"), Ok(Duration::new(0, 6_000_000)));
        assert_eq!(parse_duration("3000s"), Ok(Duration::new(3000, 0)));
        assert_eq!(parse_duration("300sec"), Ok(Duration::new(300, 0)));
        assert_eq!(parse_duration("300secs"), Ok(Duration::new(300, 0)));
        assert_eq!(parse_duration("50seconds"), Ok(Duration::new(50, 0)));
        assert_eq!(parse_duration("1second"), Ok(Duration::new(1, 0)));
        assert_eq!(parse_duration("100m"), Ok(Duration::new(6000, 0)));
        assert_eq!(parse_duration("12min"), Ok(Duration::new(720, 0)));
        assert_eq!(parse_duration("12mins"), Ok(Duration::new(720, 0)));
        assert_eq!(parse_duration("1minute"), Ok(Duration::new(60, 0)));
        assert_eq!(parse_duration("7minutes"), Ok(Duration::new(420, 0)));
        assert_eq!(parse_duration("2h"), Ok(Duration::new(7200, 0)));
        assert_eq!(parse_duration("7hr"), Ok(Duration::new(25200, 0)));
        assert_eq!(parse_duration("7hrs"), Ok(Duration::new(25200, 0)));
        assert_eq!(parse_duration("1hour"), Ok(Duration::new(3600, 0)));
        assert_eq!(parse_duration("24hours"), Ok(Duration::new(86400, 0)));
        assert_eq!(parse_duration("1day"), Ok(Duration::new(86400, 0)));
        assert_eq!(parse_duration("2days"), Ok(Duration::new(172_800, 0)));
        assert_eq!(parse_duration("365d"), Ok(Duration::new(31_536_000, 0)));
        assert_eq!(parse_duration("1week"), Ok(Duration::new(604_800, 0)));
        assert_eq!(parse_duration("7weeks"), Ok(Duration::new(4_233_600, 0)));
        assert_eq!(parse_duration("52w"), Ok(Duration::new(31_449_600, 0)));
        assert_eq!(parse_duration("1month"), Ok(Duration::new(2_630_016, 0)));
        assert_eq!(parse_duration("3months"), Ok(Duration::new(3*2_630_016, 0)));
        assert_eq!(parse_duration("12M"), Ok(Duration::new(31_560_192, 0)));
        assert_eq!(parse_duration("1year"), Ok(Duration::new(31_557_600, 0)));
        assert_eq!(parse_duration("7years"), Ok(Duration::new(7*31_557_600, 0)));
        assert_eq!(parse_duration("17y"), Ok(Duration::new(536_479_200, 0)));
    }

    #[test]
    fn test_combo() {
        assert_eq!(parse_duration("20 min 17 nsec "), Ok(Duration::new(1200, 17)));
        assert_eq!(parse_duration("2h 15m"), Ok(Duration::new(8100, 0)));
    }

    #[test]
    fn all_86400_seconds() {
        for second in 0..86400 {  // scan leap year and non-leap year
            let d = Duration::new(second, 0);
            assert_eq!(d,
                parse_duration(&format_duration(d).to_string()).unwrap());
        }
    }

    #[test]
    fn random_second() {
        for _ in 0..10000 {
            let sec = rand::thread_rng().gen_range(0, 253_370_764_800);
            let d = Duration::new(sec, 0);
            assert_eq!(d,
                parse_duration(&format_duration(d).to_string()).unwrap());
        }
    }

    #[test]
    fn random_any() {
        for _ in 0..10000 {
            let sec = rand::thread_rng().gen_range(0, 253_370_764_800);
            let nanos = rand::thread_rng().gen_range(0, 1_000_000_000);
            let d = Duration::new(sec, nanos);
            assert_eq!(d,
                parse_duration(&format_duration(d).to_string()).unwrap());
        }
    }

    #[test]
    fn test_overlow() {
        // Overflow on subseconds is earlier because of how we do conversion
        // we could fix it, but I don't see any good reason for this
        assert_eq!(parse_duration("100000000000000000000ns"),
            Err(Error::NumberOverflow));
        assert_eq!(parse_duration("100000000000000000us"),
            Err(Error::NumberOverflow));
        assert_eq!(parse_duration("100000000000000ms"),
            Err(Error::NumberOverflow));

        assert_eq!(parse_duration("100000000000000000000s"),
            Err(Error::NumberOverflow));
        assert_eq!(parse_duration("10000000000000000000m"),
            Err(Error::NumberOverflow));
        assert_eq!(parse_duration("1000000000000000000h"),
            Err(Error::NumberOverflow));
        assert_eq!(parse_duration("100000000000000000d"),
            Err(Error::NumberOverflow));
        assert_eq!(parse_duration("10000000000000000w"),
            Err(Error::NumberOverflow));
        assert_eq!(parse_duration("1000000000000000M"),
            Err(Error::NumberOverflow));
        assert_eq!(parse_duration("10000000000000y"),
            Err(Error::NumberOverflow));
    }

    #[test]
    fn test_nice_error_message() {
        assert_eq!(parse_duration("123").unwrap_err().to_string(),
            "time unit needed, for example 123sec or 123ms");
        assert_eq!(parse_duration("10 months 1").unwrap_err().to_string(),
            "time unit needed, for example 1sec or 1ms");
        assert_eq!(parse_duration("10nights").unwrap_err().to_string(),
            "unknown time unit \"nights\", supported units: \
            ns, us, ms, sec, min, hours, days, weeks, months, \
            years (and few variations)");
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7427() {
//    rusty_monitor::set_test_id(7427);
    let mut str_0: &str = "rZLojQBzu4E8";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "Smart";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "XTLzfo0v";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = " ";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "day";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "2018-02-14T00:28:07";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "year";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7: &str = "2018-02-14T00:28:07Z";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_8: &str = "0s";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut str_9: &str = "day";
    let mut str_9_ref_0: &str = &mut str_9;
    let mut str_10: &str = "ZISH8Pu1VXsJN";
    let mut str_10_ref_0: &str = &mut str_10;
    let mut str_11: &str = "yzUx";
    let mut str_11_ref_0: &str = &mut str_11;
    let mut str_12: &str = "InvalidFormat";
    let mut str_12_ref_0: &str = &mut str_12;
    let mut str_13: &str = "s";
    let mut str_13_ref_0: &str = &mut str_13;
    let mut str_14: &str = " ";
    let mut str_14_ref_0: &str = &mut str_14;
    let mut str_15: &str = "VgbUdHjccBxOoA7ofma";
    let mut str_15_ref_0: &str = &mut str_15;
    let mut str_16: &str = "pW";
    let mut str_16_ref_0: &str = &mut str_16;
    let mut str_17: &str = "Duration";
    let mut str_18: &str = "NumberExpected";
    let mut str_19: &str = "Conversion to utf8 failed";
    let mut str_20: &str = "nN0IQnRilp6";
    let mut str_21: &str = "start";
    let mut str_22: &str = "Nanos";
    let mut str_23: &str = "cnr";
    let mut str_24: &str = "2018-02-14T00:28:07Z";
    let mut str_25: &str = "28w";
    let mut str_26: &str = "9QV";
    let mut str_27: &str = "IBFPAk4zj";
    let mut str_28: &str = "end";
    let mut str_29: &str = "us";
    let mut str_30: &str = "2018-02-14T00:28:07Z";
    let mut str_31: &str = "end";
    let mut str_32: &str = "rspmuWonm15ZPaOT";
    let mut str_33: &str = "nJo";
    let mut str_34: &str = "MJth39";
    let mut str_34_ref_0: &str = &mut str_34;
    let mut bool_0: bool = true;
    let mut bool_0_ref_0: &mut bool = &mut bool_0;
    let mut str_35: &str = "rJUiDI";
    let mut str_35_ref_0: &str = &mut str_35;
    let mut str_36: &str = "GLpZU3sLvV9o";
    let mut str_36_ref_0: &str = &mut str_36;
    let mut str_37: &str = "InvalidCharacter";
    let mut str_37_ref_0: &str = &mut str_37;
    let mut str_38: &str = "value";
    let mut str_39: &str = "Conversion to utf8 failed";
    let mut str_39_ref_0: &str = &mut str_39;
    let mut str_40: &str = "Timestamp";
    let mut str_40_ref_0: &str = &mut str_40;
    let mut str_41: &str = "tJM8ac1reQk0hGv3eC2";
    let mut str_41_ref_0: &str = &mut str_41;
    let mut str_42: &str = "NumberOverflow";
    let mut str_42_ref_0: &str = &mut str_42;
    let mut str_43: &str = "SI4HWXXC9";
    let mut str_43_ref_0: &str = &mut str_43;
    let mut str_44: &str = "2018-02-14T00:28:07Z";
    let mut str_45: &str = "end";
    let mut str_46: &str = "4cV";
    let mut str_46_ref_0: &str = &mut str_46;
    let mut str_47: &str = "ns";
    let mut str_47_ref_0: &str = &mut str_47;
    let mut str_48: &str = "0s";
    let mut str_48_ref_0: &str = &mut str_48;
    let mut result_0: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_47_ref_0);
    let mut str_45_ref_0: &str = &mut str_45;
    let mut str_49: &str = "b8SH7WFzHoH82t6oikT";
    let mut str_44_ref_0: &str = &mut str_44;
    let mut str_49_ref_0: &str = &mut str_49;
    let mut str_50: &str = "nrYzENBrrdI";
    let mut str_38_ref_0: &str = &mut str_38;
    let mut str_51: &str = "Rfc3339Timestamp";
    let mut str_50_ref_0: &str = &mut str_50;
    let mut str_52: &str = "us";
    let mut str_51_ref_0: &str = &mut str_51;
    let mut str_53: &str = "ns";
    let mut str_52_ref_0: &str = &mut str_52;
    let mut str_54: &str = "rHqK";
    let mut str_53_ref_0: &str = &mut str_53;
    let mut str_55: &str = "year";
    let mut str_54_ref_0: &str = &mut str_54;
    let mut str_55_ref_0: &str = &mut str_55;
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_51_ref_0);
    let mut str_56: &str = "Empty";
    let mut str_33_ref_0: &str = &mut str_33;
    let mut str_57: &str = "X67bxREoCUUFoYUj9";
    let mut str_58: &str = "2018-02-14T00:28:07Z";
    let mut str_59: &str = "Seconds";
    let mut str_56_ref_0: &str = &mut str_56;
    let mut str_60: &str = "Micros";
    let mut str_57_ref_0: &str = &mut str_57;
    let mut str_61: &str = "Wy0cXQH";
    let mut str_59_ref_0: &str = &mut str_59;
    let mut str_35: &str = "rJUiDI";
    let mut str_35_ref_0: &str = &mut str_61;
    let mut str_36: &str = "GLpZU3sLvV9o";
    let mut str_36_ref_0: &str = &mut str_60;
    let mut str_37: &str = "InvalidCharacter";
    let mut str_37_ref_0: &str = &mut str_58;
    let mut str_39_ref_0: &str = &mut str_32;
    let mut str_40: &str = "Timestamp";
    let mut str_40_ref_0: &str = &mut str_31;
    let mut str_41: &str = "tJM8ac1reQk0hGv3eC2";
    let mut str_41_ref_0: &str = &mut str_30;
    let mut str_42_ref_0: &str = &mut str_29;
    let mut str_43: &str = "SI4HWXXC9";
    let mut str_44: &str = "2018-02-14T00:28:07Z";
    let mut str_45: &str = "end";
    let mut str_46: &str = "4cV";
    let mut str_46_ref_0: &str = &mut str_28;
    let mut str_47_ref_0: &str = &mut str_27;
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_35_ref_0);
    let mut result_0: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_40_ref_0);
    let mut str_45_ref_0: &str = &mut str_26;
    let mut str_49: &str = "Conversion to utf8 failed";
    let mut str_44_ref_0: &str = &mut str_25;
    let mut str_49_ref_0: &str = &mut str_24;
    let mut str_50: &str = "nrYzENBrrdI";
    let mut str_38_ref_0: &str = &mut str_23;
    let mut str_51: &str = "Rfc3339Timestamp";
    let mut str_50_ref_0: &str = &mut str_22;
    let mut str_51_ref_0: &str = &mut str_17;
    let mut str_53: &str = "ns";
    let mut str_52_ref_0: &str = &mut str_21;
    let mut str_54: &str = "rHqK";
    let mut str_53_ref_0: &str = &mut str_20;
    let mut str_55: &str = "year";
    let mut str_54_ref_0: &str = &mut str_19;
    let mut str_55_ref_0: &str = &mut str_18;
    let mut result_3: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_49_ref_0);
    let mut result_4: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_42_ref_0);
    let mut result_5: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_37_ref_0);
    let mut result_6: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_53_ref_0);
    let mut result_7: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_54_ref_0);
    let mut result_8: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_44_ref_0);
    let mut result_9: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_45_ref_0);
    let mut result_10: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_46_ref_0);
    let mut result_11: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_39_ref_0);
    let mut result_12: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_52_ref_0);
    let mut result_13: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_34_ref_0);
    let mut result_14: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_38_ref_0);
    let mut result_15: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_56_ref_0);
    let mut result_16: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_36_ref_0);
    let mut result_17: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_55_ref_0);
    let mut result_18: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_41_ref_0);
    let mut result_11: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_33_ref_0);
    let mut result_12: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_57_ref_0);
    let mut result_13: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_7_ref_0);
    let mut result_14: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_9_ref_0);
    let mut result_15: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_0_ref_0);
    let mut result_16: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_15_ref_0);
    let mut result_17: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_6_ref_0);
    let mut result_18: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_13_ref_0);
    let mut result_19: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_11_ref_0);
    let mut result_20: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_43_ref_0);
    let mut error_0: duration::Error = crate::duration::Error::NumberOverflow;
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_10);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1125() {
//    rusty_monitor::set_test_id(1125);
    let mut str_0: &str = "UnknownUnit";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut result_0: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_0_ref_0);
    let mut str_1: &str = "flSopf";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut error_0: date::Error = crate::date::Error::InvalidFormat;
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_357() {
//    rusty_monitor::set_test_id(357);
    let mut str_0: &str = "InvalidFormat";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u64_0: u64 = 8607u64;
    let mut u64_1: u64 = 2875u64;
    let mut tuple_0: (u64, u64) = (u64_1, u64_0);
    let mut str_1: &str = "unit";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut u64_2: u64 = 642u64;
    let mut u64_3: u64 = 30u64;
    let mut tuple_1: (u64, u64) = (u64_3, u64_2);
    let mut str_2: &str = "Millis";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "2018-02-14T00:28:07";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut u64_4: u64 = 9339u64;
    let mut u64_5: u64 = 60u64;
    let mut tuple_2: (u64, u64) = (u64_5, u64_4);
    let mut str_4: &str = "UnknownUnit";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut error_0: duration::Error = crate::duration::Error::Empty;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_3_ref_0);
    let mut error_1: duration::Error = crate::duration::Error::Empty;
    let mut error_2: date::Error = crate::date::Error::InvalidFormat;
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6992() {
//    rusty_monitor::set_test_id(6992);
    let mut str_0: &str = "unit";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "f7HWgUsvew5P4pG6";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "eDfaG9fPAQ";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "NumberExpected";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = " ";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "end";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "RXRkY9";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7: &str = "zyTt7Kv7SfqFtd";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_8: &str = "ms";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut str_9: &str = "";
    let mut str_9_ref_0: &str = &mut str_9;
    let mut str_10: &str = "S6Ft3gmst8lRzH50srA";
    let mut str_10_ref_0: &str = &mut str_10;
    let mut str_11: &str = "ns";
    let mut str_11_ref_0: &str = &mut str_11;
    let mut str_12: &str = "Timestamp";
    let mut str_12_ref_0: &str = &mut str_12;
    let mut str_13: &str = "month";
    let mut str_13_ref_0: &str = &mut str_13;
    let mut str_14: &str = "InvalidDigit";
    let mut str_14_ref_0: &str = &mut str_14;
    let mut str_15: &str = "NzhfHsnlJiz3ikp6O";
    let mut str_15_ref_0: &str = &mut str_15;
    let mut str_16: &str = "1FRx";
    let mut str_16_ref_0: &str = &mut str_16;
    let mut str_17: &str = "7dn41MHbJ42g";
    let mut str_17_ref_0: &str = &mut str_17;
    let mut str_18: &str = "s";
    let mut str_19: &str = "m";
    let mut str_20: &str = "sjKFPzX9VtCS";
    let mut str_20_ref_0: &str = &mut str_20;
    let mut str_21: &str = "Rfc3339Timestamp";
    let mut str_21_ref_0: &str = &mut str_21;
    let mut str_22: &str = "I6WRpCDOw";
    let mut str_22_ref_0: &str = &mut str_22;
    let mut str_23: &str = "InvalidFormat";
    let mut str_23_ref_0: &str = &mut str_23;
    let mut str_24: &str = "start";
    let mut str_24_ref_0: &str = &mut str_24;
    let mut str_25: &str = "a5";
    let mut str_25_ref_0: &str = &mut str_25;
    let mut str_26: &str = "Rfc3339Timestamp";
    let mut str_26_ref_0: &str = &mut str_26;
    let mut str_27: &str = "";
    let mut str_27_ref_0: &str = &mut str_27;
    let mut str_28: &str = "Nanos";
    let mut str_28_ref_0: &str = &mut str_28;
    let mut str_29: &str = "VCAfc";
    let mut str_29_ref_0: &str = &mut str_29;
    let mut str_30: &str = "UnQssUosfCXkm5et";
    let mut str_30_ref_0: &str = &mut str_30;
    let mut str_31: &str = "2018-02-14T00:28:07";
    let mut str_31_ref_0: &str = &mut str_31;
    let mut str_32: &str = "jSlT3sp3iuAiQIwey";
    let mut str_32_ref_0: &str = &mut str_32;
    let mut str_33: &str = "0s";
    let mut str_33_ref_0: &str = &mut str_33;
    let mut str_34: &str = "start";
    let mut str_34_ref_0: &str = &mut str_34;
    let mut str_35: &str = "Seconds";
    let mut str_35_ref_0: &str = &mut str_35;
    let mut str_36: &str = "h";
    let mut str_36_ref_0: &str = &mut str_36;
    let mut str_37: &str = "m";
    let mut str_37_ref_0: &str = &mut str_37;
    let mut str_38: &str = "year";
    let mut str_38_ref_0: &str = &mut str_38;
    let mut str_39: &str = "UnknownUnit";
    let mut str_39_ref_0: &str = &mut str_39;
    let mut str_40: &str = "2018-02-14T00:28:07Z";
    let mut str_41: &str = "month";
    let mut str_42: &str = "";
    let mut str_43: &str = "VmI";
    let mut str_44: &str = "start";
    let mut str_45: &str = "UnknownUnit";
    let mut str_46: &str = "Millis";
    let mut str_47: &str = "07WMKfb74r";
    let mut str_48: &str = "pVzfzAwkSWa2UPRnne";
    let mut str_49: &str = "year";
    let mut str_50: &str = "ABxn9XFU4sjeoLFiJ";
    let mut str_50_ref_0: &str = &mut str_50;
    let mut str_51: &str = "zChTuWi3";
    let mut str_51_ref_0: &str = &mut str_51;
    let mut str_52: &str = "month";
    let mut str_53: &str = "S7gwwXLKPK4VjjYfu";
    let mut str_54: &str = "FormattedDuration";
    let mut str_55: &str = "all times should be after the epoch";
    let mut str_56: &str = "G1wX9GQ7Aymw6";
    let mut str_57: &str = "UnknownUnit";
    let mut str_58: &str = "ns";
    let mut str_59: &str = "s";
    let mut str_60: &str = "wjTJgb6H6uRJGpm";
    let mut str_61: &str = "z";
    let mut str_62: &str = "ns";
    let mut str_63: &str = "elkysDq6QGWtl";
    let mut str_64: &str = "FormattedDuration";
    let mut str_65: &str = "NumberExpected";
    let mut str_66: &str = "oDKYgVRJWbDfk5p9";
    let mut str_67: &str = "LsGvCuN6mr7sWBhc5";
    let mut str_67_ref_0: &str = &mut str_67;
    let mut str_68: &str = "X";
    let mut str_69: &str = "9D1NJQBcPu";
    let mut str_70: &str = "xPhqwA0j";
    let mut str_71: &str = "m";
    let mut str_72: &str = "Duration";
    let mut str_73: &str = "BXQUvz0x";
    let mut str_74: &str = "OutOfRange";
    let mut str_75: &str = "ms";
    let mut str_76: &str = "93J5za";
    let mut str_76_ref_0: &str = &mut str_76;
    let mut str_77: &str = "h";
    let mut str_77_ref_0: &str = &mut str_77;
    let mut str_78: &str = "start";
    let mut str_79: &str = "InvalidDigit";
    let mut str_80: &str = "ns";
    let mut str_81: &str = "Conversion to utf8 failed";
    let mut str_82: &str = "Seconds";
    let mut str_83: &str = "Conversion to utf8 failed";
    let mut str_84: &str = "8WJZsGQmC01dRORsG";
    let mut str_85: &str = "Conversion to utf8 failed";
    let mut str_86: &str = "Empty";
    let mut str_87: &str = "m";
    let mut str_88: &str = "Q9";
    let mut str_89: &str = "R0ANCvn";
    let mut str_90: &str = "Z4u";
    let mut str_91: &str = "5Mb6K0eOQ";
    let mut str_92: &str = "aTgMimfhm9b5yG2F";
    let mut str_93: &str = "Millis";
    let mut str_94: &str = " ";
    let mut str_95: &str = "NumberOverflow";
    let mut str_96: &str = "2018-02-14T00:28:07";
    let mut str_97: &str = "Timestamp";
    let mut str_98: &str = "Millis";
    let mut str_99: &str = "Ls8iJxawlDrtNps";
    let mut str_100: &str = "UnknownUnit";
    let mut str_101: &str = "s";
    let mut str_102: &str = "Conversion to utf8 failed";
    let mut str_103: &str = "3M9JyMxrcP";
    let mut str_104: &str = "NumberExpected";
    let mut str_105: &str = "Duration";
    let mut str_106: &str = "Rfc3339Timestamp";
    let mut str_107: &str = "OutOfRange";
    let mut str_108: &str = " ";
    let mut str_109: &str = "TTXZdKTFr";
    let mut str_110: &str = "2018-02-14T00:28:07Z";
    let mut str_111: &str = "liOqY11f";
    let mut str_112: &str = " ";
    let mut str_113: &str = "6g7Fh";
    let mut str_114: &str = "Micros";
    let mut str_115: &str = "month";
    let mut str_116: &str = "e1eQtAL9WFvOh0S";
    let mut str_117: &str = "0s";
    let mut str_118: &str = "Duration";
    let mut str_119: &str = "day";
    let mut str_120: &str = "FormattedDuration";
    let mut str_121: &str = "Duration";
    let mut bool_0: bool = false;
    let mut str_122: &str = "rZLojQBzu4E8";
    let mut str_122_ref_0: &str = &mut str_122;
    let mut str_123: &str = "Smart";
    let mut str_123_ref_0: &str = &mut str_123;
    let mut str_124: &str = "XTLzfo0v";
    let mut str_124_ref_0: &str = &mut str_124;
    let mut str_125: &str = " ";
    let mut str_125_ref_0: &str = &mut str_125;
    let mut str_126: &str = "day";
    let mut str_126_ref_0: &str = &mut str_126;
    let mut str_127: &str = "Micros";
    let mut str_127_ref_0: &str = &mut str_127;
    let mut str_128: &str = "2018-02-14T00:28:07";
    let mut str_128_ref_0: &str = &mut str_128;
    let mut str_129: &str = "year";
    let mut str_129_ref_0: &str = &mut str_129;
    let mut str_130: &str = "2018-02-14T00:28:07Z";
    let mut str_130_ref_0: &str = &mut str_130;
    let mut str_131: &str = "0s";
    let mut str_131_ref_0: &str = &mut str_131;
    let mut str_132: &str = "day";
    let mut str_132_ref_0: &str = &mut str_132;
    let mut str_133: &str = "day";
    let mut str_133_ref_0: &str = &mut str_133;
    let mut str_134: &str = "ZISH8Pu1VXsJN";
    let mut str_134_ref_0: &str = &mut str_134;
    let mut str_135: &str = "yzUx";
    let mut str_135_ref_0: &str = &mut str_135;
    let mut str_136: &str = "InvalidFormat";
    let mut str_136_ref_0: &str = &mut str_136;
    let mut str_137: &str = "s";
    let mut str_137_ref_0: &str = &mut str_137;
    let mut str_138: &str = " ";
    let mut str_138_ref_0: &str = &mut str_138;
    let mut str_139: &str = "VgbUdHjccBxOoA7ofma";
    let mut str_139_ref_0: &str = &mut str_139;
    let mut str_140: &str = "pW";
    let mut str_140_ref_0: &str = &mut str_140;
    let mut str_141: &str = "oWa0S0JYw0kHW";
    let mut str_142: &str = "Duration";
    let mut str_143: &str = "NumberExpected";
    let mut str_144: &str = "Conversion to utf8 failed";
    let mut str_145: &str = "nN0IQnRilp6";
    let mut str_146: &str = "start";
    let mut str_147: &str = "Millis";
    let mut str_148: &str = "Nanos";
    let mut str_149: &str = "cnr";
    let mut str_150: &str = "2018-02-14T00:28:07Z";
    let mut str_151: &str = "28w";
    let mut str_152: &str = "9QV";
    let mut str_153: &str = "Seconds";
    let mut str_154: &str = "IBFPAk4zj";
    let mut str_155: &str = "end";
    let mut str_156: &str = "2018-02-14T00:28:07";
    let mut str_157: &str = "us";
    let mut str_158: &str = "2018-02-14T00:28:07Z";
    let mut str_159: &str = "end";
    let mut str_160: &str = "rspmuWonm15ZPaOT";
    let mut str_161: &str = "nJo";
    let mut str_162: &str = "MJth39";
    let mut str_162_ref_0: &str = &mut str_162;
    let mut bool_1: bool = true;
    let mut bool_1_ref_0: &mut bool = &mut bool_1;
    let mut str_163: &str = "rJUiDI";
    let mut str_163_ref_0: &str = &mut str_163;
    let mut str_164: &str = "GLpZU3sLvV9o";
    let mut str_164_ref_0: &str = &mut str_164;
    let mut str_165: &str = "InvalidCharacter";
    let mut str_165_ref_0: &str = &mut str_165;
    let mut str_166: &str = "value";
    let mut str_167: &str = "Conversion to utf8 failed";
    let mut str_167_ref_0: &str = &mut str_167;
    let mut str_168: &str = "Timestamp";
    let mut str_168_ref_0: &str = &mut str_168;
    let mut str_169: &str = "tJM8ac1reQk0hGv3eC2";
    let mut str_169_ref_0: &str = &mut str_169;
    let mut str_170: &str = "NumberOverflow";
    let mut str_170_ref_0: &str = &mut str_170;
    let mut str_171: &str = "SI4HWXXC9";
    let mut str_171_ref_0: &str = &mut str_171;
    let mut str_172: &str = "2018-02-14T00:28:07Z";
    let mut str_173: &str = "end";
    let mut str_174: &str = "4cV";
    let mut str_174_ref_0: &str = &mut str_174;
    let mut str_175: &str = "ns";
    let mut str_175_ref_0: &str = &mut str_175;
    let mut str_176: &str = "0s";
    let mut str_176_ref_0: &str = &mut str_176;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_176_ref_0);
    let mut result_1: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_175_ref_0);
    let mut str_173_ref_0: &str = &mut str_173;
    let mut str_177: &str = "b8SH7WFzHoH82t6oikT";
    let mut str_172_ref_0: &str = &mut str_172;
    let mut str_177_ref_0: &str = &mut str_177;
    let mut str_178: &str = "nrYzENBrrdI";
    let mut str_166_ref_0: &str = &mut str_166;
    let mut str_179: &str = "Rfc3339Timestamp";
    let mut str_178_ref_0: &str = &mut str_178;
    let mut str_180: &str = "us";
    let mut str_179_ref_0: &str = &mut str_179;
    let mut str_181: &str = "ns";
    let mut str_180_ref_0: &str = &mut str_180;
    let mut str_182: &str = "rHqK";
    let mut str_181_ref_0: &str = &mut str_181;
    let mut str_183: &str = "year";
    let mut str_182_ref_0: &str = &mut str_182;
    let mut str_183_ref_0: &str = &mut str_183;
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_179_ref_0);
    let mut str_184: &str = "Empty";
    let mut str_161_ref_0: &str = &mut str_161;
    let mut str_185: &str = "X67bxREoCUUFoYUj9";
    let mut str_186: &str = "2018-02-14T00:28:07Z";
    let mut str_187: &str = "Seconds";
    let mut str_184_ref_0: &str = &mut str_184;
    let mut str_188: &str = "Micros";
    let mut str_185_ref_0: &str = &mut str_185;
    let mut str_189: &str = "Wy0cXQH";
    let mut str_187_ref_0: &str = &mut str_187;
    let mut str_163: &str = "rJUiDI";
    let mut str_163_ref_0: &str = &mut str_189;
    let mut str_164: &str = "GLpZU3sLvV9o";
    let mut str_164_ref_0: &str = &mut str_188;
    let mut str_165: &str = "InvalidCharacter";
    let mut str_165_ref_0: &str = &mut str_186;
    let mut str_167_ref_0: &str = &mut str_160;
    let mut str_168: &str = "Timestamp";
    let mut str_168_ref_0: &str = &mut str_159;
    let mut str_169: &str = "tJM8ac1reQk0hGv3eC2";
    let mut str_169_ref_0: &str = &mut str_158;
    let mut str_170_ref_0: &str = &mut str_157;
    let mut str_171: &str = "SI4HWXXC9";
    let mut str_171_ref_0: &str = &mut str_156;
    let mut str_172: &str = "2018-02-14T00:28:07Z";
    let mut str_173: &str = "end";
    let mut str_174: &str = "4cV";
    let mut str_174_ref_0: &str = &mut str_155;
    let mut str_175_ref_0: &str = &mut str_154;
    let mut str_176_ref_0: &str = &mut str_153;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_163_ref_0);
    let mut result_1: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_168_ref_0);
    let mut str_173_ref_0: &str = &mut str_152;
    let mut str_177: &str = "Conversion to utf8 failed";
    let mut str_172_ref_0: &str = &mut str_151;
    let mut str_177_ref_0: &str = &mut str_150;
    let mut str_178: &str = "nrYzENBrrdI";
    let mut str_166_ref_0: &str = &mut str_149;
    let mut str_179: &str = "Rfc3339Timestamp";
    let mut str_178_ref_0: &str = &mut str_148;
    let mut str_179_ref_0: &str = &mut str_147;
    let mut str_181: &str = "ns";
    let mut str_180_ref_0: &str = &mut str_146;
    let mut str_182: &str = "rHqK";
    let mut str_181_ref_0: &str = &mut str_145;
    let mut str_183: &str = "year";
    let mut str_146: &str = "start";
    let mut str_147: &str = "Millis";
    let mut str_148: &str = "Nanos";
    let mut str_149: &str = "cnr";
    let mut str_150: &str = "2018-02-14T00:28:07Z";
    let mut str_151: &str = "28w";
    let mut str_152: &str = "9QV";
    let mut str_153: &str = "laZcFuI";
    let mut str_154: &str = "IBFPAk4zj";
    let mut str_155: &str = "end";
    let mut str_157: &str = "us";
    let mut str_158: &str = "2018-02-14T00:28:07Z";
    let mut str_159: &str = "end";
    let mut str_160: &str = "rspmuWonm15ZPaOT";
    let mut str_161: &str = "nJo";
    let mut str_162: &str = "MJth39";
    let mut str_162_ref_0: &str = &mut str_142;
    let mut bool_1: bool = true;
    let mut bool_1_ref_0: &mut bool = &mut bool_0;
    let mut str_163: &str = "rJUiDI";
    let mut str_163_ref_0: &str = &mut str_144;
    let mut str_164: &str = "GLpZU3sLvV9o";
    let mut str_164_ref_0: &str = &mut str_141;
    let mut str_165: &str = "InvalidCharacter";
    let mut str_165_ref_0: &str = &mut str_143;
    let mut str_166: &str = "value";
    let mut str_167: &str = "Conversion to utf8 failed";
    let mut str_167_ref_0: &str = &mut str_121;
    let mut str_168: &str = "Timestamp";
    let mut str_168_ref_0: &str = &mut str_120;
    let mut str_169: &str = "tJM8ac1reQk0hGv3eC2";
    let mut str_169_ref_0: &str = &mut str_119;
    let mut str_170: &str = "NumberOverflow";
    let mut str_170_ref_0: &str = &mut str_118;
    let mut str_171: &str = "SI4HWXXC9";
    let mut str_171_ref_0: &str = &mut str_117;
    let mut str_172: &str = "2018-02-14T00:28:07Z";
    let mut str_173: &str = "end";
    let mut str_174: &str = "4cV";
    let mut str_174_ref_0: &str = &mut str_116;
    let mut str_175: &str = "ns";
    let mut str_175_ref_0: &str = &mut str_115;
    let mut str_176: &str = "0s";
    let mut str_176_ref_0: &str = &mut str_114;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_169_ref_0);
    let mut result_1: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_123_ref_0);
    let mut str_173_ref_0: &str = &mut str_113;
    let mut str_177: &str = "b8SH7WFzHoH82t6oikT";
    let mut str_172_ref_0: &str = &mut str_112;
    let mut str_177_ref_0: &str = &mut str_111;
    let mut str_178: &str = "nrYzENBrrdI";
    let mut str_166_ref_0: &str = &mut str_110;
    let mut str_179: &str = "Rfc3339Timestamp";
    let mut str_178_ref_0: &str = &mut str_109;
    let mut str_180: &str = "us";
    let mut str_179_ref_0: &str = &mut str_108;
    let mut str_181: &str = "ns";
    let mut str_180_ref_0: &str = &mut str_107;
    let mut str_182: &str = "rHqK";
    let mut str_181_ref_0: &str = &mut str_106;
    let mut str_183: &str = "year";
    let mut str_182_ref_0: &str = &mut str_105;
    let mut str_183_ref_0: &str = &mut str_104;
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_177_ref_0);
    let mut str_184: &str = "Empty";
    let mut str_161_ref_0: &str = &mut str_103;
    let mut str_185: &str = "X67bxREoCUUFoYUj9";
    let mut str_186: &str = "2018-02-14T00:28:07Z";
    let mut str_187: &str = "Seconds";
    let mut str_184_ref_0: &str = &mut str_102;
    let mut str_188: &str = "Micros";
    let mut str_185_ref_0: &str = &mut str_101;
    let mut str_189: &str = "Wy0cXQH";
    let mut str_187_ref_0: &str = &mut str_100;
    let mut str_163: &str = "rJUiDI";
    let mut str_163_ref_0: &str = &mut str_99;
    let mut str_164: &str = "GLpZU3sLvV9o";
    let mut str_164_ref_0: &str = &mut str_98;
    let mut str_165: &str = "InvalidCharacter";
    let mut str_165_ref_0: &str = &mut str_97;
    let mut str_167_ref_0: &str = &mut str_96;
    let mut str_168: &str = "Timestamp";
    let mut str_168_ref_0: &str = &mut str_95;
    let mut str_169: &str = "tJM8ac1reQk0hGv3eC2";
    let mut str_169_ref_0: &str = &mut str_94;
    let mut str_170_ref_0: &str = &mut str_93;
    let mut str_171: &str = "SI4HWXXC9";
    let mut str_172: &str = "2018-02-14T00:28:07Z";
    let mut str_173: &str = "end";
    let mut str_174: &str = "4cV";
    let mut str_174_ref_0: &str = &mut str_92;
    let mut str_175_ref_0: &str = &mut str_91;
    let mut str_176_ref_0: &str = &mut str_90;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_180_ref_0);
    let mut result_1: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_171_ref_0);
    let mut str_173_ref_0: &str = &mut str_89;
    let mut str_177: &str = "Conversion to utf8 failed";
    let mut str_172_ref_0: &str = &mut str_88;
    let mut str_177_ref_0: &str = &mut str_87;
    let mut str_178: &str = "nrYzENBrrdI";
    let mut str_166_ref_0: &str = &mut str_86;
    let mut str_179: &str = "Rfc3339Timestamp";
    let mut str_178_ref_0: &str = &mut str_85;
    let mut str_179_ref_0: &str = &mut str_84;
    let mut str_181: &str = "ns";
    let mut str_180_ref_0: &str = &mut str_83;
    let mut str_182: &str = "rHqK";
    let mut str_181_ref_0: &str = &mut str_82;
    let mut str_183: &str = "year";
    let mut str_182_ref_0: &str = &mut str_81;
    let mut str_183_ref_0: &str = &mut str_80;
    let mut result_3: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_136_ref_0);
    let mut result_4: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_174_ref_0);
    let mut result_5: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_173_ref_0);
    let mut result_6: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_170_ref_0);
    let mut result_7: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_126_ref_0);
    let mut result_8: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_128_ref_0);
    let mut result_9: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_165_ref_0);
    let mut result_10: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_162_ref_0);
    let mut result_11: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_134_ref_0);
    let mut result_12: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_178_ref_0);
    let mut result_13: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_167_ref_0);
    let mut result_14: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_182_ref_0);
    let mut result_15: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_166_ref_0);
    let mut result_16: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_187_ref_0);
    let mut result_17: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_181_ref_0);
    let mut result_18: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_140_ref_0);
    let mut result_19: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_131_ref_0);
    let mut result_5: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_172_ref_0);
    let mut result_6: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_137_ref_0);
    let mut str_183: &str = "year";
    let mut str_182_ref_0: &str = &mut str_79;
    let mut str_183_ref_0: &str = &mut str_78;
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_127_ref_0);
    let mut result_3: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_161_ref_0);
    let mut result_4: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_184_ref_0);
    let mut result_6: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_185_ref_0);
    let mut result_7: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_183_ref_0);
    let mut result_8: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_138_ref_0);
    let mut result_20: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_122_ref_0);
    let mut result_9: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_129_ref_0);
    let mut result_10: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_133_ref_0);
    let mut result_11: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_164_ref_0);
    let mut result_12: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_125_ref_0);
    let mut result_13: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_132_ref_0);
    let mut result_14: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_124_ref_0);
    let mut result_15: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_130_ref_0);
    let mut result_17: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_135_ref_0);
    let mut result_18: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_139_ref_0);
    let mut result_21: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_77_ref_0);
    let mut result_19: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_76_ref_0);
    let mut str_177_ref_0: &str = &mut str_75;
    let mut str_178: &str = "nrYzENBrrdI";
    let mut str_166_ref_0: &str = &mut str_74;
    let mut str_179: &str = "Rfc3339Timestamp";
    let mut str_178_ref_0: &str = &mut str_73;
    let mut str_180: &str = "us";
    let mut str_179_ref_0: &str = &mut str_72;
    let mut str_181: &str = "ns";
    let mut str_180_ref_0: &str = &mut str_71;
    let mut str_182: &str = "rHqK";
    let mut str_181_ref_0: &str = &mut str_70;
    let mut str_183: &str = "year";
    let mut str_182_ref_0: &str = &mut str_69;
    let mut str_183_ref_0: &str = &mut str_68;
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_67_ref_0);
    let mut str_184: &str = "Empty";
    let mut str_161_ref_0: &str = &mut str_66;
    let mut str_185: &str = "X67bxREoCUUFoYUj9";
    let mut str_186: &str = "2018-02-14T00:28:07Z";
    let mut str_187: &str = "Seconds";
    let mut str_184_ref_0: &str = &mut str_65;
    let mut str_188: &str = "Micros";
    let mut str_185_ref_0: &str = &mut str_64;
    let mut str_189: &str = "Wy0cXQH";
    let mut str_187_ref_0: &str = &mut str_63;
    let mut str_163: &str = "rJUiDI";
    let mut str_163_ref_0: &str = &mut str_62;
    let mut str_164: &str = "GLpZU3sLvV9o";
    let mut str_164_ref_0: &str = &mut str_61;
    let mut str_165: &str = "InvalidCharacter";
    let mut str_165_ref_0: &str = &mut str_60;
    let mut str_167_ref_0: &str = &mut str_59;
    let mut str_168: &str = "Timestamp";
    let mut str_168_ref_0: &str = &mut str_58;
    let mut str_169: &str = "tJM8ac1reQk0hGv3eC2";
    let mut str_169_ref_0: &str = &mut str_57;
    let mut str_170_ref_0: &str = &mut str_56;
    let mut str_171: &str = "SI4HWXXC9";
    let mut str_171_ref_0: &str = &mut str_55;
    let mut str_172: &str = "2018-02-14T00:28:07Z";
    let mut str_173: &str = "end";
    let mut str_174: &str = "4cV";
    let mut str_174_ref_0: &str = &mut str_54;
    let mut str_175_ref_0: &str = &mut str_53;
    let mut str_176_ref_0: &str = &mut str_52;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_51_ref_0);
    let mut result_1: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_50_ref_0);
    let mut str_173_ref_0: &str = &mut str_49;
    let mut str_177: &str = "Conversion to utf8 failed";
    let mut str_172_ref_0: &str = &mut str_48;
    let mut str_177_ref_0: &str = &mut str_47;
    let mut str_178: &str = "nrYzENBrrdI";
    let mut str_166_ref_0: &str = &mut str_46;
    let mut str_179: &str = "Rfc3339Timestamp";
    let mut str_178_ref_0: &str = &mut str_45;
    let mut str_179_ref_0: &str = &mut str_44;
    let mut str_181: &str = "ns";
    let mut str_180_ref_0: &str = &mut str_43;
    let mut str_182: &str = "rHqK";
    let mut str_181_ref_0: &str = &mut str_42;
    let mut str_183: &str = "year";
    let mut str_182_ref_0: &str = &mut str_41;
    let mut str_183_ref_0: &str = &mut str_40;
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_39_ref_0);
    let mut result_3: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_38_ref_0);
    let mut result_4: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_37_ref_0);
    let mut result_5: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_36_ref_0);
    let mut result_6: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_35_ref_0);
    let mut result_7: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_34_ref_0);
    let mut result_8: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_33_ref_0);
    let mut result_9: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_32_ref_0);
    let mut result_10: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_31_ref_0);
    let mut result_11: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_30_ref_0);
    let mut result_12: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_29_ref_0);
    let mut result_13: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_28_ref_0);
    let mut result_14: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_27_ref_0);
    let mut result_15: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_26_ref_0);
    let mut result_16: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_25_ref_0);
    let mut result_17: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_24_ref_0);
    let mut result_18: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_23_ref_0);
    let mut result_19: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_22_ref_0);
    let mut result_5: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_21_ref_0);
    let mut result_6: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_20_ref_0);
    let mut str_183: &str = "year";
    let mut str_182_ref_0: &str = &mut str_19;
    let mut str_183_ref_0: &str = &mut str_18;
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_17_ref_0);
    let mut result_3: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_16_ref_0);
    let mut result_4: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_15_ref_0);
    let mut result_6: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_14_ref_0);
    let mut result_7: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_13_ref_0);
    let mut result_8: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_12_ref_0);
    let mut result_9: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_11_ref_0);
    let mut result_10: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_10_ref_0);
    let mut result_11: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_9_ref_0);
    let mut result_12: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_8_ref_0);
    let mut result_13: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_7_ref_0);
    let mut result_14: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_6_ref_0);
    let mut result_15: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_5_ref_0);
    let mut result_16: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_4_ref_0);
    let mut result_17: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_3_ref_0);
    let mut result_18: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_2_ref_0);
    let mut result_21: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_1_ref_0);
    let mut result_19: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_0_ref_0);
    let mut error_0: duration::Error = crate::duration::Error::NumberOverflow;
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_8);
//    panic!("From RustyUnit with love");
}
}