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
            Error::InvalidCharacter(offset) => {
                write!(f, "invalid character at {}", offset)
            }
            Error::NumberExpected(offset) => write!(f, "expected number at {}", offset),
            Error::UnknownUnit { unit, value, .. } if unit.is_empty() => {
                write!(f, "time unit needed, for example {0}sec or {0}ms", value,)
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
    fn parse_unit(&mut self, n: u64, start: usize, end: usize) -> Result<(), Error> {
        let (mut sec, nsec) = match &self.src[start..end] {
            "nanos" | "nsec" | "ns" => (0u64, n),
            "usec" | "us" => (0u64, n.mul(1000)?),
            "millis" | "msec" | "ms" => (0u64, n.mul(1_000_000)?),
            "seconds" | "second" | "secs" | "sec" | "s" => (n, 0),
            "minutes" | "minute" | "min" | "mins" | "m" => (n.mul(60)?, 0),
            "hours" | "hour" | "hr" | "hrs" | "h" => (n.mul(3600)?, 0),
            "days" | "day" | "d" => (n.mul(86400)?, 0),
            "weeks" | "week" | "w" => (n.mul(86400 * 7)?, 0),
            "months" | "month" | "M" => (n.mul(2_630_016)?, 0),
            "years" | "year" | "y" => (n.mul(31_557_600)?, 0),
            _ => {
                return Err(Error::UnknownUnit {
                    start,
                    end,
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
                        n = n
                            .checked_mul(10)
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
                None => return Ok(Duration::new(self.current.0, self.current.1 as u32)),
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
    }
        .parse()
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
fn item_plural(
    f: &mut fmt::Formatter,
    started: &mut bool,
    name: &str,
    value: u64,
) -> fmt::Result {
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
fn item(
    f: &mut fmt::Formatter,
    started: &mut bool,
    name: &str,
    value: u32,
) -> fmt::Result {
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
        let years = secs / 31_557_600;
        let ydays = secs % 31_557_600;
        let months = ydays / 2_630_016;
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
        assert_eq!(parse_duration("3months"), Ok(Duration::new(3 * 2_630_016, 0)));
        assert_eq!(parse_duration("12M"), Ok(Duration::new(31_560_192, 0)));
        assert_eq!(parse_duration("1year"), Ok(Duration::new(31_557_600, 0)));
        assert_eq!(parse_duration("7years"), Ok(Duration::new(7 * 31_557_600, 0)));
        assert_eq!(parse_duration("17y"), Ok(Duration::new(536_479_200, 0)));
    }
    #[test]
    fn test_combo() {
        assert_eq!(parse_duration("20 min 17 nsec "), Ok(Duration::new(1200, 17)));
        assert_eq!(parse_duration("2h 15m"), Ok(Duration::new(8100, 0)));
    }
    #[test]
    fn all_86400_seconds() {
        for second in 0..86400 {
            let d = Duration::new(second, 0);
            assert_eq!(d, parse_duration(& format_duration(d).to_string()).unwrap());
        }
    }
    #[test]
    fn random_second() {
        for _ in 0..10000 {
            let sec = rand::thread_rng().gen_range(0, 253_370_764_800);
            let d = Duration::new(sec, 0);
            assert_eq!(d, parse_duration(& format_duration(d).to_string()).unwrap());
        }
    }
    #[test]
    fn random_any() {
        for _ in 0..10000 {
            let sec = rand::thread_rng().gen_range(0, 253_370_764_800);
            let nanos = rand::thread_rng().gen_range(0, 1_000_000_000);
            let d = Duration::new(sec, nanos);
            assert_eq!(d, parse_duration(& format_duration(d).to_string()).unwrap());
        }
    }
    #[test]
    fn test_overlow() {
        assert_eq!(
            parse_duration("100000000000000000000ns"), Err(Error::NumberOverflow)
        );
        assert_eq!(parse_duration("100000000000000000us"), Err(Error::NumberOverflow));
        assert_eq!(parse_duration("100000000000000ms"), Err(Error::NumberOverflow));
        assert_eq!(parse_duration("100000000000000000000s"), Err(Error::NumberOverflow));
        assert_eq!(parse_duration("10000000000000000000m"), Err(Error::NumberOverflow));
        assert_eq!(parse_duration("1000000000000000000h"), Err(Error::NumberOverflow));
        assert_eq!(parse_duration("100000000000000000d"), Err(Error::NumberOverflow));
        assert_eq!(parse_duration("10000000000000000w"), Err(Error::NumberOverflow));
        assert_eq!(parse_duration("1000000000000000M"), Err(Error::NumberOverflow));
        assert_eq!(parse_duration("10000000000000y"), Err(Error::NumberOverflow));
    }
    #[test]
    fn test_nice_error_message() {
        assert_eq!(
            parse_duration("123").unwrap_err().to_string(),
            "time unit needed, for example 123sec or 123ms"
        );
        assert_eq!(
            parse_duration("10 months 1").unwrap_err().to_string(),
            "time unit needed, for example 1sec or 1ms"
        );
        assert_eq!(
            parse_duration("10nights").unwrap_err().to_string(),
            "unknown time unit \"nights\", supported units: \
            ns, us, ms, sec, min, hours, days, weeks, months, \
            years (and few variations)"
        );
    }
}
#[cfg(test)]
mod tests_llm_16_1_llm_16_1 {
    use super::*;
    use crate::*;
    use crate::duration::OverflowOp;
    use std::time::Duration;
    #[test]
    fn test_add_durations_without_overflow() {
        let _rug_st_tests_llm_16_1_llm_16_1_rrrruuuugggg_test_add_durations_without_overflow = 0;
        let rug_fuzz_0 = 1_000;
        let rug_fuzz_1 = 2_000;
        let duration1 = Duration::from_secs(rug_fuzz_0).as_secs();
        let duration2 = Duration::from_secs(rug_fuzz_1).as_secs();
        debug_assert_eq!(
            < u64 as OverflowOp > ::add(duration1, duration2).unwrap(),
            Duration::from_secs(3_000).as_secs()
        );
        let _rug_ed_tests_llm_16_1_llm_16_1_rrrruuuugggg_test_add_durations_without_overflow = 0;
    }
    #[test]
    fn test_add_durations_with_overflow() {
        let _rug_st_tests_llm_16_1_llm_16_1_rrrruuuugggg_test_add_durations_with_overflow = 0;
        let rug_fuzz_0 = 1;
        let duration1 = u64::MAX;
        let duration2 = rug_fuzz_0;
        debug_assert!(< u64 as OverflowOp > ::add(duration1, duration2).is_err());
        let _rug_ed_tests_llm_16_1_llm_16_1_rrrruuuugggg_test_add_durations_with_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2 {
    use super::*;
    use crate::*;
    use std::u64;
    #[test]
    fn test_mul_no_overflow() {
        let _rug_st_tests_llm_16_2_rrrruuuugggg_test_mul_no_overflow = 0;
        let rug_fuzz_0 = 2u64;
        let rug_fuzz_1 = 3;
        debug_assert_eq!(rug_fuzz_0.mul(rug_fuzz_1), Ok(6));
        let _rug_ed_tests_llm_16_2_rrrruuuugggg_test_mul_no_overflow = 0;
    }
    #[test]
    fn test_mul_with_overflow() {
        let _rug_st_tests_llm_16_2_rrrruuuugggg_test_mul_with_overflow = 0;
        let rug_fuzz_0 = 2;
        debug_assert_eq!(u64::MAX.mul(rug_fuzz_0), Err(Error::NumberOverflow));
        let _rug_ed_tests_llm_16_2_rrrruuuugggg_test_mul_with_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_24 {
    use super::*;
    use crate::*;
    use std::time::Duration;
    #[test]
    fn test_get_ref() {
        let _rug_st_tests_llm_16_24_rrrruuuugggg_test_get_ref = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 0;
        let original_duration = Duration::new(rug_fuzz_0, rug_fuzz_1);
        let formatted_duration = FormattedDuration(original_duration);
        let duration_ref = formatted_duration.get_ref();
        debug_assert_eq!(duration_ref, & original_duration);
        let _rug_ed_tests_llm_16_24_rrrruuuugggg_test_get_ref = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_25 {
    use super::*;
    use crate::*;
    #[test]
    fn test_off_initial() {
        let _rug_st_tests_llm_16_25_rrrruuuugggg_test_off_initial = 0;
        let rug_fuzz_0 = "10s";
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let input = rug_fuzz_0;
        let parser = Parser {
            iter: input.chars(),
            src: input,
            current: (rug_fuzz_1, rug_fuzz_2),
        };
        debug_assert_eq!(parser.off(), 0);
        let _rug_ed_tests_llm_16_25_rrrruuuugggg_test_off_initial = 0;
    }
    #[test]
    fn test_off_partial() {
        let _rug_st_tests_llm_16_25_rrrruuuugggg_test_off_partial = 0;
        let rug_fuzz_0 = "10s";
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let input = rug_fuzz_0;
        let mut parser = Parser {
            iter: input.chars(),
            src: input,
            current: (rug_fuzz_1, rug_fuzz_2),
        };
        parser.iter.next();
        debug_assert_eq!(parser.off(), 1);
        let _rug_ed_tests_llm_16_25_rrrruuuugggg_test_off_partial = 0;
    }
    #[test]
    fn test_off_all_consumed() {
        let _rug_st_tests_llm_16_25_rrrruuuugggg_test_off_all_consumed = 0;
        let rug_fuzz_0 = "10s";
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let input = rug_fuzz_0;
        let mut parser = Parser {
            iter: input.chars(),
            src: input,
            current: (rug_fuzz_1, rug_fuzz_2),
        };
        parser.iter.by_ref().for_each(drop);
        debug_assert_eq!(parser.off(), input.len());
        let _rug_ed_tests_llm_16_25_rrrruuuugggg_test_off_all_consumed = 0;
    }
    #[test]
    fn test_off_partial_consumed() {
        let _rug_st_tests_llm_16_25_rrrruuuugggg_test_off_partial_consumed = 0;
        let rug_fuzz_0 = "10s 20m";
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 4;
        let input = rug_fuzz_0;
        let mut parser = Parser {
            iter: input.chars(),
            src: input,
            current: (rug_fuzz_1, rug_fuzz_2),
        };
        parser.iter.by_ref().take(rug_fuzz_3).for_each(drop);
        debug_assert_eq!(parser.off(), 4);
        let _rug_ed_tests_llm_16_25_rrrruuuugggg_test_off_partial_consumed = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_26 {
    use super::*;
    use crate::*;
    use std::time::Duration;
    #[test]
    fn test_parse_empty_string() {
        let _rug_st_tests_llm_16_26_rrrruuuugggg_test_parse_empty_string = 0;
        let rug_fuzz_0 = "";
        let rug_fuzz_1 = "";
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let parser = Parser {
            iter: rug_fuzz_0.chars(),
            src: rug_fuzz_1,
            current: (rug_fuzz_2, rug_fuzz_3),
        };
        debug_assert_eq!(parser.parse(), Err(Error::Empty));
        let _rug_ed_tests_llm_16_26_rrrruuuugggg_test_parse_empty_string = 0;
    }
    #[test]
    fn test_parse_valid_string() {
        let _rug_st_tests_llm_16_26_rrrruuuugggg_test_parse_valid_string = 0;
        let rug_fuzz_0 = "2h 30m";
        let rug_fuzz_1 = "2h 30m";
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let parser = Parser {
            iter: rug_fuzz_0.chars(),
            src: rug_fuzz_1,
            current: (rug_fuzz_2, rug_fuzz_3),
        };
        debug_assert_eq!(parser.parse(), Ok(Duration::new(2 * 3600 + 30 * 60, 0)));
        let _rug_ed_tests_llm_16_26_rrrruuuugggg_test_parse_valid_string = 0;
    }
    #[test]
    fn test_parse_string_with_invalid_character() {
        let _rug_st_tests_llm_16_26_rrrruuuugggg_test_parse_string_with_invalid_character = 0;
        let rug_fuzz_0 = "2h 30m#";
        let rug_fuzz_1 = "2h 30m#";
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let parser = Parser {
            iter: rug_fuzz_0.chars(),
            src: rug_fuzz_1,
            current: (rug_fuzz_2, rug_fuzz_3),
        };
        debug_assert!(matches!(parser.parse(), Err(Error::InvalidCharacter(_))));
        let _rug_ed_tests_llm_16_26_rrrruuuugggg_test_parse_string_with_invalid_character = 0;
    }
    #[test]
    fn test_parse_string_with_unknown_unit() {
        let _rug_st_tests_llm_16_26_rrrruuuugggg_test_parse_string_with_unknown_unit = 0;
        let rug_fuzz_0 = "2h 30x";
        let rug_fuzz_1 = "2h 30x";
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let parser = Parser {
            iter: rug_fuzz_0.chars(),
            src: rug_fuzz_1,
            current: (rug_fuzz_2, rug_fuzz_3),
        };
        debug_assert!(matches!(parser.parse(), Err(Error::UnknownUnit { .. })));
        let _rug_ed_tests_llm_16_26_rrrruuuugggg_test_parse_string_with_unknown_unit = 0;
    }
    #[test]
    fn test_parse_string_with_overflow() {
        let _rug_st_tests_llm_16_26_rrrruuuugggg_test_parse_string_with_overflow = 0;
        let rug_fuzz_0 = "18446744073709551616ns";
        let rug_fuzz_1 = "18446744073709551616ns";
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let parser = Parser {
            iter: rug_fuzz_0.chars(),
            src: rug_fuzz_1,
            current: (rug_fuzz_2, rug_fuzz_3),
        };
        debug_assert_eq!(parser.parse(), Err(Error::NumberOverflow));
        let _rug_ed_tests_llm_16_26_rrrruuuugggg_test_parse_string_with_overflow = 0;
    }
    #[test]
    fn test_parse_string_with_whitespace() {
        let _rug_st_tests_llm_16_26_rrrruuuugggg_test_parse_string_with_whitespace = 0;
        let rug_fuzz_0 = " \t\n2h\n\t 30m \t\n";
        let rug_fuzz_1 = " \t\n2h\n\t 30m \t\n";
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let parser = Parser {
            iter: rug_fuzz_0.chars(),
            src: rug_fuzz_1,
            current: (rug_fuzz_2, rug_fuzz_3),
        };
        debug_assert_eq!(parser.parse(), Ok(Duration::new(2 * 3600 + 30 * 60, 0)));
        let _rug_ed_tests_llm_16_26_rrrruuuugggg_test_parse_string_with_whitespace = 0;
    }
    #[test]
    fn test_parse_string_with_multiple_units() {
        let _rug_st_tests_llm_16_26_rrrruuuugggg_test_parse_string_with_multiple_units = 0;
        let rug_fuzz_0 = "1h30m45s";
        let rug_fuzz_1 = "1h30m45s";
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let parser = Parser {
            iter: rug_fuzz_0.chars(),
            src: rug_fuzz_1,
            current: (rug_fuzz_2, rug_fuzz_3),
        };
        debug_assert_eq!(parser.parse(), Ok(Duration::new(1 * 3600 + 30 * 60 + 45, 0)));
        let _rug_ed_tests_llm_16_26_rrrruuuugggg_test_parse_string_with_multiple_units = 0;
    }
    #[test]
    fn test_parse_string_number_expected() {
        let _rug_st_tests_llm_16_26_rrrruuuugggg_test_parse_string_number_expected = 0;
        let rug_fuzz_0 = "h";
        let rug_fuzz_1 = "h";
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let parser = Parser {
            iter: rug_fuzz_0.chars(),
            src: rug_fuzz_1,
            current: (rug_fuzz_2, rug_fuzz_3),
        };
        debug_assert!(matches!(parser.parse(), Err(Error::NumberExpected(_))));
        let _rug_ed_tests_llm_16_26_rrrruuuugggg_test_parse_string_number_expected = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_27_llm_16_27 {
    use super::*;
    use crate::*;
    use crate::duration::Error;
    #[test]
    fn test_parse_first_char_with_numbers() {
        let _rug_st_tests_llm_16_27_llm_16_27_rrrruuuugggg_test_parse_first_char_with_numbers = 0;
        let rug_fuzz_0 = "12345";
        let rug_fuzz_1 = "12345";
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let mut parser = Parser {
            iter: rug_fuzz_0.chars(),
            src: rug_fuzz_1,
            current: (rug_fuzz_2, rug_fuzz_3),
        };
        debug_assert_eq!(parser.parse_first_char(), Ok(Some(1)));
        let _rug_ed_tests_llm_16_27_llm_16_27_rrrruuuugggg_test_parse_first_char_with_numbers = 0;
    }
    #[test]
    fn test_parse_first_char_with_leading_whitespace() {
        let _rug_st_tests_llm_16_27_llm_16_27_rrrruuuugggg_test_parse_first_char_with_leading_whitespace = 0;
        let rug_fuzz_0 = "  1";
        let rug_fuzz_1 = "  1";
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let mut parser = Parser {
            iter: rug_fuzz_0.chars(),
            src: rug_fuzz_1,
            current: (rug_fuzz_2, rug_fuzz_3),
        };
        debug_assert_eq!(parser.parse_first_char(), Ok(Some(1)));
        let _rug_ed_tests_llm_16_27_llm_16_27_rrrruuuugggg_test_parse_first_char_with_leading_whitespace = 0;
    }
    #[test]
    fn test_parse_first_char_with_only_whitespace() {
        let _rug_st_tests_llm_16_27_llm_16_27_rrrruuuugggg_test_parse_first_char_with_only_whitespace = 0;
        let rug_fuzz_0 = "    ";
        let rug_fuzz_1 = "    ";
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let mut parser = Parser {
            iter: rug_fuzz_0.chars(),
            src: rug_fuzz_1,
            current: (rug_fuzz_2, rug_fuzz_3),
        };
        debug_assert_eq!(parser.parse_first_char(), Ok(None));
        let _rug_ed_tests_llm_16_27_llm_16_27_rrrruuuugggg_test_parse_first_char_with_only_whitespace = 0;
    }
    #[test]
    fn test_parse_first_char_with_no_numbers() {
        let _rug_st_tests_llm_16_27_llm_16_27_rrrruuuugggg_test_parse_first_char_with_no_numbers = 0;
        let rug_fuzz_0 = "abc";
        let rug_fuzz_1 = "abc";
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let mut parser = Parser {
            iter: rug_fuzz_0.chars(),
            src: rug_fuzz_1,
            current: (rug_fuzz_2, rug_fuzz_3),
        };
        debug_assert_eq!(parser.parse_first_char(), Err(Error::NumberExpected(0)));
        let _rug_ed_tests_llm_16_27_llm_16_27_rrrruuuugggg_test_parse_first_char_with_no_numbers = 0;
    }
    #[test]
    fn test_parse_first_char_with_empty_string() {
        let _rug_st_tests_llm_16_27_llm_16_27_rrrruuuugggg_test_parse_first_char_with_empty_string = 0;
        let rug_fuzz_0 = "";
        let rug_fuzz_1 = "";
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let mut parser = Parser {
            iter: rug_fuzz_0.chars(),
            src: rug_fuzz_1,
            current: (rug_fuzz_2, rug_fuzz_3),
        };
        debug_assert_eq!(parser.parse_first_char(), Ok(None));
        let _rug_ed_tests_llm_16_27_llm_16_27_rrrruuuugggg_test_parse_first_char_with_empty_string = 0;
    }
    #[test]
    fn test_parse_first_char_with_number_after_whitespace() {
        let _rug_st_tests_llm_16_27_llm_16_27_rrrruuuugggg_test_parse_first_char_with_number_after_whitespace = 0;
        let rug_fuzz_0 = "   9abc";
        let rug_fuzz_1 = "   9abc";
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let mut parser = Parser {
            iter: rug_fuzz_0.chars(),
            src: rug_fuzz_1,
            current: (rug_fuzz_2, rug_fuzz_3),
        };
        debug_assert_eq!(parser.parse_first_char(), Ok(Some(9)));
        let _rug_ed_tests_llm_16_27_llm_16_27_rrrruuuugggg_test_parse_first_char_with_number_after_whitespace = 0;
    }
    #[test]
    fn test_parse_first_char_with_non_number_after_whitespace() {
        let _rug_st_tests_llm_16_27_llm_16_27_rrrruuuugggg_test_parse_first_char_with_non_number_after_whitespace = 0;
        let rug_fuzz_0 = "   +abc";
        let rug_fuzz_1 = "   +abc";
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let mut parser = Parser {
            iter: rug_fuzz_0.chars(),
            src: rug_fuzz_1,
            current: (rug_fuzz_2, rug_fuzz_3),
        };
        debug_assert_eq!(parser.parse_first_char(), Err(Error::NumberExpected(3)));
        let _rug_ed_tests_llm_16_27_llm_16_27_rrrruuuugggg_test_parse_first_char_with_non_number_after_whitespace = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_28_llm_16_28 {
    use crate::duration::Error;
    use crate::duration::Parser;
    use std::str::Chars;
    impl<'a> Parser<'a> {
        fn new(src: &'a str) -> Self {
            Self {
                iter: src.chars(),
                src,
                current: (0, 0),
            }
        }
    }
    #[test]
    fn parse_unit_seconds() {
        let _rug_st_tests_llm_16_28_llm_16_28_rrrruuuugggg_parse_unit_seconds = 0;
        let rug_fuzz_0 = "10seconds";
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 7;
        let mut parser = Parser::new(rug_fuzz_0);
        debug_assert_eq!(parser.parse_unit(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3), Ok(()));
        debug_assert_eq!(parser.current, (10, 0));
        let _rug_ed_tests_llm_16_28_llm_16_28_rrrruuuugggg_parse_unit_seconds = 0;
    }
    #[test]
    fn parse_unit_minutes() {
        let _rug_st_tests_llm_16_28_llm_16_28_rrrruuuugggg_parse_unit_minutes = 0;
        let rug_fuzz_0 = "5minutes";
        let rug_fuzz_1 = 5;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 7;
        let mut parser = Parser::new(rug_fuzz_0);
        debug_assert_eq!(parser.parse_unit(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3), Ok(()));
        debug_assert_eq!(parser.current, (300, 0));
        let _rug_ed_tests_llm_16_28_llm_16_28_rrrruuuugggg_parse_unit_minutes = 0;
    }
    #[test]
    fn parse_unit_hours() {
        let _rug_st_tests_llm_16_28_llm_16_28_rrrruuuugggg_parse_unit_hours = 0;
        let rug_fuzz_0 = "1hour";
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 4;
        let mut parser = Parser::new(rug_fuzz_0);
        debug_assert_eq!(parser.parse_unit(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3), Ok(()));
        debug_assert_eq!(parser.current, (3600, 0));
        let _rug_ed_tests_llm_16_28_llm_16_28_rrrruuuugggg_parse_unit_hours = 0;
    }
    #[test]
    fn parse_unit_days() {
        let _rug_st_tests_llm_16_28_llm_16_28_rrrruuuugggg_parse_unit_days = 0;
        let rug_fuzz_0 = "2days";
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 4;
        let mut parser = Parser::new(rug_fuzz_0);
        debug_assert_eq!(parser.parse_unit(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3), Ok(()));
        debug_assert_eq!(parser.current, (2 * 86400, 0));
        let _rug_ed_tests_llm_16_28_llm_16_28_rrrruuuugggg_parse_unit_days = 0;
    }
    #[test]
    fn parse_unit_weeks() {
        let _rug_st_tests_llm_16_28_llm_16_28_rrrruuuugggg_parse_unit_weeks = 0;
        let rug_fuzz_0 = "3weeks";
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 5;
        let mut parser = Parser::new(rug_fuzz_0);
        debug_assert_eq!(parser.parse_unit(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3), Ok(()));
        debug_assert_eq!(parser.current, (3 * 86400 * 7, 0));
        let _rug_ed_tests_llm_16_28_llm_16_28_rrrruuuugggg_parse_unit_weeks = 0;
    }
    #[test]
    fn parse_unit_nanos() {
        let _rug_st_tests_llm_16_28_llm_16_28_rrrruuuugggg_parse_unit_nanos = 0;
        let rug_fuzz_0 = "100nanos";
        let rug_fuzz_1 = 100;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 5;
        let mut parser = Parser::new(rug_fuzz_0);
        debug_assert_eq!(parser.parse_unit(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3), Ok(()));
        debug_assert_eq!(parser.current, (0, 100));
        let _rug_ed_tests_llm_16_28_llm_16_28_rrrruuuugggg_parse_unit_nanos = 0;
    }
    #[test]
    fn parse_unit_unknown() {
        let _rug_st_tests_llm_16_28_llm_16_28_rrrruuuugggg_parse_unit_unknown = 0;
        let rug_fuzz_0 = "10something";
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 10;
        let rug_fuzz_3 = 10;
        let mut parser = Parser::new(rug_fuzz_0);
        let start = rug_fuzz_1;
        let end = rug_fuzz_2;
        debug_assert_eq!(
            parser.parse_unit(rug_fuzz_3, start, end), Err(Error::UnknownUnit { start,
            end, unit : parser.src[start..end].to_string(), value : 10, })
        );
        let _rug_ed_tests_llm_16_28_llm_16_28_rrrruuuugggg_parse_unit_unknown = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_29 {
    use crate::format_duration;
    use std::time::Duration;
    #[test]
    fn test_format_duration_zero() {
        let _rug_st_tests_llm_16_29_rrrruuuugggg_test_format_duration_zero = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let duration = Duration::new(rug_fuzz_0, rug_fuzz_1);
        debug_assert_eq!(format_duration(duration).to_string(), "0s");
        let _rug_ed_tests_llm_16_29_rrrruuuugggg_test_format_duration_zero = 0;
    }
    #[test]
    fn test_format_duration_seconds() {
        let _rug_st_tests_llm_16_29_rrrruuuugggg_test_format_duration_seconds = 0;
        let rug_fuzz_0 = 45;
        let rug_fuzz_1 = 0;
        let duration = Duration::new(rug_fuzz_0, rug_fuzz_1);
        debug_assert_eq!(format_duration(duration).to_string(), "45s");
        let _rug_ed_tests_llm_16_29_rrrruuuugggg_test_format_duration_seconds = 0;
    }
    #[test]
    fn test_format_duration_minutes_seconds() {
        let _rug_st_tests_llm_16_29_rrrruuuugggg_test_format_duration_minutes_seconds = 0;
        let rug_fuzz_0 = 183;
        let rug_fuzz_1 = 0;
        let duration = Duration::new(rug_fuzz_0, rug_fuzz_1);
        debug_assert_eq!(format_duration(duration).to_string(), "3m 3s");
        let _rug_ed_tests_llm_16_29_rrrruuuugggg_test_format_duration_minutes_seconds = 0;
    }
    #[test]
    fn test_format_duration_hours_minutes_seconds() {
        let _rug_st_tests_llm_16_29_rrrruuuugggg_test_format_duration_hours_minutes_seconds = 0;
        let rug_fuzz_0 = 3723;
        let rug_fuzz_1 = 0;
        let duration = Duration::new(rug_fuzz_0, rug_fuzz_1);
        debug_assert_eq!(format_duration(duration).to_string(), "1h 2m 3s");
        let _rug_ed_tests_llm_16_29_rrrruuuugggg_test_format_duration_hours_minutes_seconds = 0;
    }
    #[test]
    fn test_format_duration_days_hours_minutes_seconds() {
        let _rug_st_tests_llm_16_29_rrrruuuugggg_test_format_duration_days_hours_minutes_seconds = 0;
        let rug_fuzz_0 = 90123;
        let rug_fuzz_1 = 0;
        let duration = Duration::new(rug_fuzz_0, rug_fuzz_1);
        debug_assert_eq!(format_duration(duration).to_string(), "1d 1h 2m 3s");
        let _rug_ed_tests_llm_16_29_rrrruuuugggg_test_format_duration_days_hours_minutes_seconds = 0;
    }
    #[test]
    fn test_format_duration_with_milliseconds() {
        let _rug_st_tests_llm_16_29_rrrruuuugggg_test_format_duration_with_milliseconds = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1_234_000_000;
        let duration = Duration::new(rug_fuzz_0, rug_fuzz_1);
        debug_assert_eq!(format_duration(duration).to_string(), "1s 234ms");
        let _rug_ed_tests_llm_16_29_rrrruuuugggg_test_format_duration_with_milliseconds = 0;
    }
    #[test]
    fn test_format_duration_with_microseconds() {
        let _rug_st_tests_llm_16_29_rrrruuuugggg_test_format_duration_with_microseconds = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1_234_567;
        let duration = Duration::new(rug_fuzz_0, rug_fuzz_1);
        debug_assert_eq!(format_duration(duration).to_string(), "1ms 234us");
        let _rug_ed_tests_llm_16_29_rrrruuuugggg_test_format_duration_with_microseconds = 0;
    }
    #[test]
    fn test_format_duration_with_nanoseconds() {
        let _rug_st_tests_llm_16_29_rrrruuuugggg_test_format_duration_with_nanoseconds = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1_234;
        let duration = Duration::new(rug_fuzz_0, rug_fuzz_1);
        debug_assert_eq!(format_duration(duration).to_string(), "1us 234ns");
        let _rug_ed_tests_llm_16_29_rrrruuuugggg_test_format_duration_with_nanoseconds = 0;
    }
    #[test]
    fn test_format_duration_with_all_units() {
        let _rug_st_tests_llm_16_29_rrrruuuugggg_test_format_duration_with_all_units = 0;
        let rug_fuzz_0 = 32_000_000;
        let rug_fuzz_1 = 1_234_567_890;
        let duration = Duration::new(rug_fuzz_0, rug_fuzz_1);
        debug_assert_eq!(
            format_duration(duration).to_string(), "1y 3d 7s 234ms 567us 890ns"
        );
        let _rug_ed_tests_llm_16_29_rrrruuuugggg_test_format_duration_with_all_units = 0;
    }
    #[test]
    fn test_format_duration_with_all_units_less_than_a_year() {
        let _rug_st_tests_llm_16_29_rrrruuuugggg_test_format_duration_with_all_units_less_than_a_year = 0;
        let rug_fuzz_0 = 10_000_000;
        let rug_fuzz_1 = 0;
        let duration = Duration::new(rug_fuzz_0, rug_fuzz_1);
        debug_assert_eq!(format_duration(duration).to_string(), "4m 15d");
        let _rug_ed_tests_llm_16_29_rrrruuuugggg_test_format_duration_with_all_units_less_than_a_year = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_32 {
    use crate::parse_duration;
    use std::time::Duration;
    #[test]
    fn test_parse_seconds() {
        let _rug_st_tests_llm_16_32_rrrruuuugggg_test_parse_seconds = 0;
        let rug_fuzz_0 = "5s";
        debug_assert_eq!(parse_duration(rug_fuzz_0), Ok(Duration::new(5, 0)));
        let _rug_ed_tests_llm_16_32_rrrruuuugggg_test_parse_seconds = 0;
    }
    #[test]
    fn test_parse_minutes() {
        let _rug_st_tests_llm_16_32_rrrruuuugggg_test_parse_minutes = 0;
        let rug_fuzz_0 = "7min";
        debug_assert_eq!(parse_duration(rug_fuzz_0), Ok(Duration::new(7 * 60, 0)));
        let _rug_ed_tests_llm_16_32_rrrruuuugggg_test_parse_minutes = 0;
    }
    #[test]
    fn test_parse_hours() {
        let _rug_st_tests_llm_16_32_rrrruuuugggg_test_parse_hours = 0;
        let rug_fuzz_0 = "3h";
        debug_assert_eq!(parse_duration(rug_fuzz_0), Ok(Duration::new(3 * 3600, 0)));
        let _rug_ed_tests_llm_16_32_rrrruuuugggg_test_parse_hours = 0;
    }
    #[test]
    fn test_parse_days() {
        let _rug_st_tests_llm_16_32_rrrruuuugggg_test_parse_days = 0;
        let rug_fuzz_0 = "2d";
        debug_assert_eq!(parse_duration(rug_fuzz_0), Ok(Duration::new(2 * 86400, 0)));
        let _rug_ed_tests_llm_16_32_rrrruuuugggg_test_parse_days = 0;
    }
    #[test]
    fn test_parse_weeks() {
        let _rug_st_tests_llm_16_32_rrrruuuugggg_test_parse_weeks = 0;
        let rug_fuzz_0 = "1w";
        debug_assert_eq!(parse_duration(rug_fuzz_0), Ok(Duration::new(7 * 86400, 0)));
        let _rug_ed_tests_llm_16_32_rrrruuuugggg_test_parse_weeks = 0;
    }
    #[test]
    fn test_parse_months() {
        let _rug_st_tests_llm_16_32_rrrruuuugggg_test_parse_months = 0;
        let rug_fuzz_0 = "1M";
        debug_assert_eq!(parse_duration(rug_fuzz_0), Ok(Duration::new(30 * 86400, 0)));
        let _rug_ed_tests_llm_16_32_rrrruuuugggg_test_parse_months = 0;
    }
    #[test]
    fn test_parse_years() {
        let _rug_st_tests_llm_16_32_rrrruuuugggg_test_parse_years = 0;
        let rug_fuzz_0 = "1y";
        debug_assert_eq!(parse_duration(rug_fuzz_0), Ok(Duration::new(365 * 86400, 0)));
        let _rug_ed_tests_llm_16_32_rrrruuuugggg_test_parse_years = 0;
    }
    #[test]
    fn test_parse_combined() {
        let _rug_st_tests_llm_16_32_rrrruuuugggg_test_parse_combined = 0;
        let rug_fuzz_0 = "1h 30m 10s";
        debug_assert_eq!(
            parse_duration(rug_fuzz_0), Ok(Duration::new(1 * 3600 + 30 * 60 + 10, 0))
        );
        let _rug_ed_tests_llm_16_32_rrrruuuugggg_test_parse_combined = 0;
    }
    #[test]
    fn test_parse_with_spaces() {
        let _rug_st_tests_llm_16_32_rrrruuuugggg_test_parse_with_spaces = 0;
        let rug_fuzz_0 = "  2 min  ";
        debug_assert_eq!(parse_duration(rug_fuzz_0), Ok(Duration::new(2 * 60, 0)));
        let _rug_ed_tests_llm_16_32_rrrruuuugggg_test_parse_with_spaces = 0;
    }
    #[test]
    fn test_parse_with_invalid_input() {
        let _rug_st_tests_llm_16_32_rrrruuuugggg_test_parse_with_invalid_input = 0;
        let rug_fuzz_0 = "2x";
        debug_assert!(parse_duration(rug_fuzz_0).is_err());
        let _rug_ed_tests_llm_16_32_rrrruuuugggg_test_parse_with_invalid_input = 0;
    }
    #[test]
    fn test_parse_with_empty_input() {
        let _rug_st_tests_llm_16_32_rrrruuuugggg_test_parse_with_empty_input = 0;
        let rug_fuzz_0 = "";
        debug_assert!(parse_duration(rug_fuzz_0).is_err());
        let _rug_ed_tests_llm_16_32_rrrruuuugggg_test_parse_with_empty_input = 0;
    }
    #[test]
    fn test_parse_with_invalid_suffix() {
        let _rug_st_tests_llm_16_32_rrrruuuugggg_test_parse_with_invalid_suffix = 0;
        let rug_fuzz_0 = "10lightyears";
        debug_assert!(parse_duration(rug_fuzz_0).is_err());
        let _rug_ed_tests_llm_16_32_rrrruuuugggg_test_parse_with_invalid_suffix = 0;
    }
}
