use crate::datetime::DateTime;
use crate::naive::NaiveDateTime;
use crate::time_delta::TimeDelta;
use crate::TimeZone;
use crate::Timelike;
use core::cmp::Ordering;
use core::fmt;
use core::marker::Sized;
use core::ops::{Add, Sub};
/// Extension trait for subsecond rounding or truncation to a maximum number
/// of digits. Rounding can be used to decrease the error variance when
/// serializing/persisting to lower precision. Truncation is the default
/// behavior in Chrono display formatting.  Either can be used to guarantee
/// equality (e.g. for testing) when round-tripping through a lower precision
/// format.
pub trait SubsecRound {
    /// Return a copy rounded to the specified number of subsecond digits. With
    /// 9 or more digits, self is returned unmodified. Halfway values are
    /// rounded up (away from zero).
    ///
    /// # Example
    /// ``` rust
    /// # use chrono::{DateTime, SubsecRound, Timelike, TimeZone, Utc, NaiveDate};
    /// let dt = NaiveDate::from_ymd_opt(2018, 1, 11).unwrap().and_hms_milli_opt(12, 0, 0, 154).unwrap().and_local_timezone(Utc).unwrap();
    /// assert_eq!(dt.round_subsecs(2).nanosecond(), 150_000_000);
    /// assert_eq!(dt.round_subsecs(1).nanosecond(), 200_000_000);
    /// ```
    fn round_subsecs(self, digits: u16) -> Self;
    /// Return a copy truncated to the specified number of subsecond
    /// digits. With 9 or more digits, self is returned unmodified.
    ///
    /// # Example
    /// ``` rust
    /// # use chrono::{DateTime, SubsecRound, Timelike, TimeZone, Utc, NaiveDate};
    /// let dt = NaiveDate::from_ymd_opt(2018, 1, 11).unwrap().and_hms_milli_opt(12, 0, 0, 154).unwrap().and_local_timezone(Utc).unwrap();
    /// assert_eq!(dt.trunc_subsecs(2).nanosecond(), 150_000_000);
    /// assert_eq!(dt.trunc_subsecs(1).nanosecond(), 100_000_000);
    /// ```
    fn trunc_subsecs(self, digits: u16) -> Self;
}
impl<T> SubsecRound for T
where
    T: Timelike + Add<TimeDelta, Output = T> + Sub<TimeDelta, Output = T>,
{
    fn round_subsecs(self, digits: u16) -> T {
        let span = span_for_digits(digits);
        let delta_down = self.nanosecond() % span;
        if delta_down > 0 {
            let delta_up = span - delta_down;
            if delta_up <= delta_down {
                self + TimeDelta::nanoseconds(delta_up.into())
            } else {
                self - TimeDelta::nanoseconds(delta_down.into())
            }
        } else {
            self
        }
    }
    fn trunc_subsecs(self, digits: u16) -> T {
        let span = span_for_digits(digits);
        let delta_down = self.nanosecond() % span;
        if delta_down > 0 {
            self - TimeDelta::nanoseconds(delta_down.into())
        } else {
            self
        }
    }
}
const fn span_for_digits(digits: u16) -> u32 {
    match digits {
        0 => 1_000_000_000,
        1 => 100_000_000,
        2 => 10_000_000,
        3 => 1_000_000,
        4 => 100_000,
        5 => 10_000,
        6 => 1_000,
        7 => 100,
        8 => 10,
        _ => 1,
    }
}
/// Extension trait for rounding or truncating a DateTime by a TimeDelta.
///
/// # Limitations
/// Both rounding and truncating are done via [`TimeDelta::num_nanoseconds`] and
/// [`DateTime::timestamp_nanos`]. This means that they will fail if either the
/// `TimeDelta` or the `DateTime` are too big to represented as nanoseconds. They
/// will also fail if the `TimeDelta` is bigger than the timestamp.
pub trait DurationRound: Sized {
    /// Error that can occur in rounding or truncating
    #[cfg(any(feature = "std", test))]
    type Err: std::error::Error;
    /// Error that can occur in rounding or truncating
    #[cfg(not(any(feature = "std", test)))]
    type Err: fmt::Debug + fmt::Display;
    /// Return a copy rounded by TimeDelta.
    ///
    /// # Example
    /// ``` rust
    /// # use chrono::{DateTime, DurationRound, TimeDelta, TimeZone, Utc, NaiveDate};
    /// let dt = NaiveDate::from_ymd_opt(2018, 1, 11).unwrap().and_hms_milli_opt(12, 0, 0, 154).unwrap().and_local_timezone(Utc).unwrap();
    /// assert_eq!(
    ///     dt.duration_round(TimeDelta::milliseconds(10)).unwrap().to_string(),
    ///     "2018-01-11 12:00:00.150 UTC"
    /// );
    /// assert_eq!(
    ///     dt.duration_round(TimeDelta::days(1)).unwrap().to_string(),
    ///     "2018-01-12 00:00:00 UTC"
    /// );
    /// ```
    fn duration_round(self, duration: TimeDelta) -> Result<Self, Self::Err>;
    /// Return a copy truncated by TimeDelta.
    ///
    /// # Example
    /// ``` rust
    /// # use chrono::{DateTime, DurationRound, TimeDelta, TimeZone, Utc, NaiveDate};
    /// let dt = NaiveDate::from_ymd_opt(2018, 1, 11).unwrap().and_hms_milli_opt(12, 0, 0, 154).unwrap().and_local_timezone(Utc).unwrap();
    /// assert_eq!(
    ///     dt.duration_trunc(TimeDelta::milliseconds(10)).unwrap().to_string(),
    ///     "2018-01-11 12:00:00.150 UTC"
    /// );
    /// assert_eq!(
    ///     dt.duration_trunc(TimeDelta::days(1)).unwrap().to_string(),
    ///     "2018-01-11 00:00:00 UTC"
    /// );
    /// ```
    fn duration_trunc(self, duration: TimeDelta) -> Result<Self, Self::Err>;
}
/// The maximum number of seconds a DateTime can be to be represented as nanoseconds
const MAX_SECONDS_TIMESTAMP_FOR_NANOS: i64 = 9_223_372_036;
impl<Tz: TimeZone> DurationRound for DateTime<Tz> {
    type Err = RoundingError;
    fn duration_round(self, duration: TimeDelta) -> Result<Self, Self::Err> {
        duration_round(self.naive_local(), self, duration)
    }
    fn duration_trunc(self, duration: TimeDelta) -> Result<Self, Self::Err> {
        duration_trunc(self.naive_local(), self, duration)
    }
}
impl DurationRound for NaiveDateTime {
    type Err = RoundingError;
    fn duration_round(self, duration: TimeDelta) -> Result<Self, Self::Err> {
        duration_round(self, self, duration)
    }
    fn duration_trunc(self, duration: TimeDelta) -> Result<Self, Self::Err> {
        duration_trunc(self, self, duration)
    }
}
fn duration_round<T>(
    naive: NaiveDateTime,
    original: T,
    duration: TimeDelta,
) -> Result<T, RoundingError>
where
    T: Timelike + Add<TimeDelta, Output = T> + Sub<TimeDelta, Output = T>,
{
    if let Some(span) = duration.num_nanoseconds() {
        if naive.timestamp().abs() > MAX_SECONDS_TIMESTAMP_FOR_NANOS {
            return Err(RoundingError::TimestampExceedsLimit);
        }
        let stamp = naive.timestamp_nanos();
        if span > stamp.abs() {
            return Err(RoundingError::DurationExceedsTimestamp);
        }
        if span == 0 {
            return Ok(original);
        }
        let delta_down = stamp % span;
        if delta_down == 0 {
            Ok(original)
        } else {
            let (delta_up, delta_down) = if delta_down < 0 {
                (delta_down.abs(), span - delta_down.abs())
            } else {
                (span - delta_down, delta_down)
            };
            if delta_up <= delta_down {
                Ok(original + TimeDelta::nanoseconds(delta_up))
            } else {
                Ok(original - TimeDelta::nanoseconds(delta_down))
            }
        }
    } else {
        Err(RoundingError::DurationExceedsLimit)
    }
}
fn duration_trunc<T>(
    naive: NaiveDateTime,
    original: T,
    duration: TimeDelta,
) -> Result<T, RoundingError>
where
    T: Timelike + Add<TimeDelta, Output = T> + Sub<TimeDelta, Output = T>,
{
    if let Some(span) = duration.num_nanoseconds() {
        if naive.timestamp().abs() > MAX_SECONDS_TIMESTAMP_FOR_NANOS {
            return Err(RoundingError::TimestampExceedsLimit);
        }
        let stamp = naive.timestamp_nanos();
        if span > stamp.abs() {
            return Err(RoundingError::DurationExceedsTimestamp);
        }
        let delta_down = stamp % span;
        match delta_down.cmp(&0) {
            Ordering::Equal => Ok(original),
            Ordering::Greater => Ok(original - TimeDelta::nanoseconds(delta_down)),
            Ordering::Less => {
                Ok(original - TimeDelta::nanoseconds(span - delta_down.abs()))
            }
        }
    } else {
        Err(RoundingError::DurationExceedsLimit)
    }
}
/// An error from rounding by `TimeDelta`
///
/// See: [`DurationRound`]
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum RoundingError {
    /// Error when the TimeDelta exceeds the TimeDelta from or until the Unix epoch.
    ///
    /// ``` rust
    /// # use chrono::{DateTime, DurationRound, TimeDelta, RoundingError, TimeZone, Utc};
    /// let dt = Utc.with_ymd_and_hms(1970, 12, 12, 0, 0, 0).unwrap();
    ///
    /// assert_eq!(
    ///     dt.duration_round(TimeDelta::days(365)),
    ///     Err(RoundingError::DurationExceedsTimestamp),
    /// );
    /// ```
    DurationExceedsTimestamp,
    /// Error when `TimeDelta.num_nanoseconds` exceeds the limit.
    ///
    /// ``` rust
    /// # use chrono::{DateTime, DurationRound, TimeDelta, RoundingError, TimeZone, Utc, NaiveDate};
    /// let dt = NaiveDate::from_ymd_opt(2260, 12, 31).unwrap().and_hms_nano_opt(23, 59, 59, 1_75_500_000).unwrap().and_local_timezone(Utc).unwrap();
    ///
    /// assert_eq!(
    ///     dt.duration_round(TimeDelta::days(300 * 365)),
    ///     Err(RoundingError::DurationExceedsLimit)
    /// );
    /// ```
    DurationExceedsLimit,
    /// Error when `DateTime.timestamp_nanos` exceeds the limit.
    ///
    /// ``` rust
    /// # use chrono::{DateTime, DurationRound, TimeDelta, RoundingError, TimeZone, Utc};
    /// let dt = Utc.with_ymd_and_hms(2300, 12, 12, 0, 0, 0).unwrap();
    ///
    /// assert_eq!(dt.duration_round(TimeDelta::days(1)), Err(RoundingError::TimestampExceedsLimit),);
    /// ```
    TimestampExceedsLimit,
}
impl fmt::Display for RoundingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RoundingError::DurationExceedsTimestamp => {
                write!(f, "duration in nanoseconds exceeds timestamp")
            }
            RoundingError::DurationExceedsLimit => {
                write!(f, "duration exceeds num_nanoseconds limit")
            }
            RoundingError::TimestampExceedsLimit => {
                write!(f, "timestamp exceeds num_nanoseconds limit")
            }
        }
    }
}
#[cfg(any(feature = "std", test))]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl std::error::Error for RoundingError {
    #[allow(deprecated)]
    fn description(&self) -> &str {
        "error from rounding or truncating with DurationRound"
    }
}
#[cfg(test)]
mod tests {
    use super::{DurationRound, SubsecRound, TimeDelta};
    use crate::offset::{FixedOffset, TimeZone, Utc};
    use crate::NaiveDate;
    use crate::Timelike;
    #[test]
    fn test_round_subsecs() {
        let pst = FixedOffset::east_opt(8 * 60 * 60).unwrap();
        let dt = pst
            .from_local_datetime(
                &NaiveDate::from_ymd_opt(2018, 1, 11)
                    .unwrap()
                    .and_hms_nano_opt(10, 5, 13, 84_660_684)
                    .unwrap(),
            )
            .unwrap();
        assert_eq!(dt.round_subsecs(10), dt);
        assert_eq!(dt.round_subsecs(9), dt);
        assert_eq!(dt.round_subsecs(8).nanosecond(), 84_660_680);
        assert_eq!(dt.round_subsecs(7).nanosecond(), 84_660_700);
        assert_eq!(dt.round_subsecs(6).nanosecond(), 84_661_000);
        assert_eq!(dt.round_subsecs(5).nanosecond(), 84_660_000);
        assert_eq!(dt.round_subsecs(4).nanosecond(), 84_700_000);
        assert_eq!(dt.round_subsecs(3).nanosecond(), 85_000_000);
        assert_eq!(dt.round_subsecs(2).nanosecond(), 80_000_000);
        assert_eq!(dt.round_subsecs(1).nanosecond(), 100_000_000);
        assert_eq!(dt.round_subsecs(0).nanosecond(), 0);
        assert_eq!(dt.round_subsecs(0).second(), 13);
        let dt = Utc
            .from_local_datetime(
                &NaiveDate::from_ymd_opt(2018, 1, 11)
                    .unwrap()
                    .and_hms_nano_opt(10, 5, 27, 750_500_000)
                    .unwrap(),
            )
            .unwrap();
        assert_eq!(dt.round_subsecs(9), dt);
        assert_eq!(dt.round_subsecs(4), dt);
        assert_eq!(dt.round_subsecs(3).nanosecond(), 751_000_000);
        assert_eq!(dt.round_subsecs(2).nanosecond(), 750_000_000);
        assert_eq!(dt.round_subsecs(1).nanosecond(), 800_000_000);
        assert_eq!(dt.round_subsecs(0).nanosecond(), 0);
        assert_eq!(dt.round_subsecs(0).second(), 28);
    }
    #[test]
    fn test_round_leap_nanos() {
        let dt = Utc
            .from_local_datetime(
                &NaiveDate::from_ymd_opt(2016, 12, 31)
                    .unwrap()
                    .and_hms_nano_opt(23, 59, 59, 1_750_500_000)
                    .unwrap(),
            )
            .unwrap();
        assert_eq!(dt.round_subsecs(9), dt);
        assert_eq!(dt.round_subsecs(4), dt);
        assert_eq!(dt.round_subsecs(2).nanosecond(), 1_750_000_000);
        assert_eq!(dt.round_subsecs(1).nanosecond(), 1_800_000_000);
        assert_eq!(dt.round_subsecs(1).second(), 59);
        assert_eq!(dt.round_subsecs(0).nanosecond(), 0);
        assert_eq!(dt.round_subsecs(0).second(), 0);
    }
    #[test]
    fn test_trunc_subsecs() {
        let pst = FixedOffset::east_opt(8 * 60 * 60).unwrap();
        let dt = pst
            .from_local_datetime(
                &NaiveDate::from_ymd_opt(2018, 1, 11)
                    .unwrap()
                    .and_hms_nano_opt(10, 5, 13, 84_660_684)
                    .unwrap(),
            )
            .unwrap();
        assert_eq!(dt.trunc_subsecs(10), dt);
        assert_eq!(dt.trunc_subsecs(9), dt);
        assert_eq!(dt.trunc_subsecs(8).nanosecond(), 84_660_680);
        assert_eq!(dt.trunc_subsecs(7).nanosecond(), 84_660_600);
        assert_eq!(dt.trunc_subsecs(6).nanosecond(), 84_660_000);
        assert_eq!(dt.trunc_subsecs(5).nanosecond(), 84_660_000);
        assert_eq!(dt.trunc_subsecs(4).nanosecond(), 84_600_000);
        assert_eq!(dt.trunc_subsecs(3).nanosecond(), 84_000_000);
        assert_eq!(dt.trunc_subsecs(2).nanosecond(), 80_000_000);
        assert_eq!(dt.trunc_subsecs(1).nanosecond(), 0);
        assert_eq!(dt.trunc_subsecs(0).nanosecond(), 0);
        assert_eq!(dt.trunc_subsecs(0).second(), 13);
        let dt = pst
            .from_local_datetime(
                &NaiveDate::from_ymd_opt(2018, 1, 11)
                    .unwrap()
                    .and_hms_nano_opt(10, 5, 27, 750_500_000)
                    .unwrap(),
            )
            .unwrap();
        assert_eq!(dt.trunc_subsecs(9), dt);
        assert_eq!(dt.trunc_subsecs(4), dt);
        assert_eq!(dt.trunc_subsecs(3).nanosecond(), 750_000_000);
        assert_eq!(dt.trunc_subsecs(2).nanosecond(), 750_000_000);
        assert_eq!(dt.trunc_subsecs(1).nanosecond(), 700_000_000);
        assert_eq!(dt.trunc_subsecs(0).nanosecond(), 0);
        assert_eq!(dt.trunc_subsecs(0).second(), 27);
    }
    #[test]
    fn test_trunc_leap_nanos() {
        let dt = Utc
            .from_local_datetime(
                &NaiveDate::from_ymd_opt(2016, 12, 31)
                    .unwrap()
                    .and_hms_nano_opt(23, 59, 59, 1_750_500_000)
                    .unwrap(),
            )
            .unwrap();
        assert_eq!(dt.trunc_subsecs(9), dt);
        assert_eq!(dt.trunc_subsecs(4), dt);
        assert_eq!(dt.trunc_subsecs(2).nanosecond(), 1_750_000_000);
        assert_eq!(dt.trunc_subsecs(1).nanosecond(), 1_700_000_000);
        assert_eq!(dt.trunc_subsecs(1).second(), 59);
        assert_eq!(dt.trunc_subsecs(0).nanosecond(), 1_000_000_000);
        assert_eq!(dt.trunc_subsecs(0).second(), 59);
    }
    #[test]
    fn test_duration_round() {
        let dt = Utc
            .from_local_datetime(
                &NaiveDate::from_ymd_opt(2016, 12, 31)
                    .unwrap()
                    .and_hms_nano_opt(23, 59, 59, 175_500_000)
                    .unwrap(),
            )
            .unwrap();
        assert_eq!(
            dt.duration_round(TimeDelta::zero()).unwrap().to_string(),
            "2016-12-31 23:59:59.175500 UTC"
        );
        assert_eq!(
            dt.duration_round(TimeDelta::milliseconds(10)).unwrap().to_string(),
            "2016-12-31 23:59:59.180 UTC"
        );
        let dt = Utc
            .from_local_datetime(
                &NaiveDate::from_ymd_opt(2012, 12, 12)
                    .unwrap()
                    .and_hms_milli_opt(18, 22, 30, 0)
                    .unwrap(),
            )
            .unwrap();
        assert_eq!(
            dt.duration_round(TimeDelta::minutes(5)).unwrap().to_string(),
            "2012-12-12 18:25:00 UTC"
        );
        let dt = Utc
            .from_local_datetime(
                &NaiveDate::from_ymd_opt(2012, 12, 12)
                    .unwrap()
                    .and_hms_milli_opt(18, 22, 29, 999)
                    .unwrap(),
            )
            .unwrap();
        assert_eq!(
            dt.duration_round(TimeDelta::minutes(5)).unwrap().to_string(),
            "2012-12-12 18:20:00 UTC"
        );
        assert_eq!(
            dt.duration_round(TimeDelta::minutes(10)).unwrap().to_string(),
            "2012-12-12 18:20:00 UTC"
        );
        assert_eq!(
            dt.duration_round(TimeDelta::minutes(30)).unwrap().to_string(),
            "2012-12-12 18:30:00 UTC"
        );
        assert_eq!(
            dt.duration_round(TimeDelta::hours(1)).unwrap().to_string(),
            "2012-12-12 18:00:00 UTC"
        );
        assert_eq!(
            dt.duration_round(TimeDelta::days(1)).unwrap().to_string(),
            "2012-12-13 00:00:00 UTC"
        );
        let dt = FixedOffset::east_opt(3600)
            .unwrap()
            .with_ymd_and_hms(2020, 10, 27, 15, 0, 0)
            .unwrap();
        assert_eq!(
            dt.duration_round(TimeDelta::days(1)).unwrap().to_string(),
            "2020-10-28 00:00:00 +01:00"
        );
        assert_eq!(
            dt.duration_round(TimeDelta::weeks(1)).unwrap().to_string(),
            "2020-10-29 00:00:00 +01:00"
        );
        let dt = FixedOffset::west_opt(3600)
            .unwrap()
            .with_ymd_and_hms(2020, 10, 27, 15, 0, 0)
            .unwrap();
        assert_eq!(
            dt.duration_round(TimeDelta::days(1)).unwrap().to_string(),
            "2020-10-28 00:00:00 -01:00"
        );
        assert_eq!(
            dt.duration_round(TimeDelta::weeks(1)).unwrap().to_string(),
            "2020-10-29 00:00:00 -01:00"
        );
    }
    #[test]
    fn test_duration_round_naive() {
        let dt = Utc
            .from_local_datetime(
                &NaiveDate::from_ymd_opt(2016, 12, 31)
                    .unwrap()
                    .and_hms_nano_opt(23, 59, 59, 175_500_000)
                    .unwrap(),
            )
            .unwrap()
            .naive_utc();
        assert_eq!(
            dt.duration_round(TimeDelta::zero()).unwrap().to_string(),
            "2016-12-31 23:59:59.175500"
        );
        assert_eq!(
            dt.duration_round(TimeDelta::milliseconds(10)).unwrap().to_string(),
            "2016-12-31 23:59:59.180"
        );
        let dt = Utc
            .from_local_datetime(
                &NaiveDate::from_ymd_opt(2012, 12, 12)
                    .unwrap()
                    .and_hms_milli_opt(18, 22, 30, 0)
                    .unwrap(),
            )
            .unwrap()
            .naive_utc();
        assert_eq!(
            dt.duration_round(TimeDelta::minutes(5)).unwrap().to_string(),
            "2012-12-12 18:25:00"
        );
        let dt = Utc
            .from_local_datetime(
                &NaiveDate::from_ymd_opt(2012, 12, 12)
                    .unwrap()
                    .and_hms_milli_opt(18, 22, 29, 999)
                    .unwrap(),
            )
            .unwrap()
            .naive_utc();
        assert_eq!(
            dt.duration_round(TimeDelta::minutes(5)).unwrap().to_string(),
            "2012-12-12 18:20:00"
        );
        assert_eq!(
            dt.duration_round(TimeDelta::minutes(10)).unwrap().to_string(),
            "2012-12-12 18:20:00"
        );
        assert_eq!(
            dt.duration_round(TimeDelta::minutes(30)).unwrap().to_string(),
            "2012-12-12 18:30:00"
        );
        assert_eq!(
            dt.duration_round(TimeDelta::hours(1)).unwrap().to_string(),
            "2012-12-12 18:00:00"
        );
        assert_eq!(
            dt.duration_round(TimeDelta::days(1)).unwrap().to_string(),
            "2012-12-13 00:00:00"
        );
    }
    #[test]
    fn test_duration_round_pre_epoch() {
        let dt = Utc.with_ymd_and_hms(1969, 12, 12, 12, 12, 12).unwrap();
        assert_eq!(
            dt.duration_round(TimeDelta::minutes(10)).unwrap().to_string(),
            "1969-12-12 12:10:00 UTC"
        );
    }
    #[test]
    fn test_duration_trunc() {
        let dt = Utc
            .from_local_datetime(
                &NaiveDate::from_ymd_opt(2016, 12, 31)
                    .unwrap()
                    .and_hms_nano_opt(23, 59, 59, 175_500_000)
                    .unwrap(),
            )
            .unwrap();
        assert_eq!(
            dt.duration_trunc(TimeDelta::milliseconds(10)).unwrap().to_string(),
            "2016-12-31 23:59:59.170 UTC"
        );
        let dt = Utc
            .from_local_datetime(
                &NaiveDate::from_ymd_opt(2012, 12, 12)
                    .unwrap()
                    .and_hms_milli_opt(18, 22, 30, 0)
                    .unwrap(),
            )
            .unwrap();
        assert_eq!(
            dt.duration_trunc(TimeDelta::minutes(5)).unwrap().to_string(),
            "2012-12-12 18:20:00 UTC"
        );
        let dt = Utc
            .from_local_datetime(
                &NaiveDate::from_ymd_opt(2012, 12, 12)
                    .unwrap()
                    .and_hms_milli_opt(18, 22, 29, 999)
                    .unwrap(),
            )
            .unwrap();
        assert_eq!(
            dt.duration_trunc(TimeDelta::minutes(5)).unwrap().to_string(),
            "2012-12-12 18:20:00 UTC"
        );
        assert_eq!(
            dt.duration_trunc(TimeDelta::minutes(10)).unwrap().to_string(),
            "2012-12-12 18:20:00 UTC"
        );
        assert_eq!(
            dt.duration_trunc(TimeDelta::minutes(30)).unwrap().to_string(),
            "2012-12-12 18:00:00 UTC"
        );
        assert_eq!(
            dt.duration_trunc(TimeDelta::hours(1)).unwrap().to_string(),
            "2012-12-12 18:00:00 UTC"
        );
        assert_eq!(
            dt.duration_trunc(TimeDelta::days(1)).unwrap().to_string(),
            "2012-12-12 00:00:00 UTC"
        );
        let dt = FixedOffset::east_opt(3600)
            .unwrap()
            .with_ymd_and_hms(2020, 10, 27, 15, 0, 0)
            .unwrap();
        assert_eq!(
            dt.duration_trunc(TimeDelta::days(1)).unwrap().to_string(),
            "2020-10-27 00:00:00 +01:00"
        );
        assert_eq!(
            dt.duration_trunc(TimeDelta::weeks(1)).unwrap().to_string(),
            "2020-10-22 00:00:00 +01:00"
        );
        let dt = FixedOffset::west_opt(3600)
            .unwrap()
            .with_ymd_and_hms(2020, 10, 27, 15, 0, 0)
            .unwrap();
        assert_eq!(
            dt.duration_trunc(TimeDelta::days(1)).unwrap().to_string(),
            "2020-10-27 00:00:00 -01:00"
        );
        assert_eq!(
            dt.duration_trunc(TimeDelta::weeks(1)).unwrap().to_string(),
            "2020-10-22 00:00:00 -01:00"
        );
    }
    #[test]
    fn test_duration_trunc_naive() {
        let dt = Utc
            .from_local_datetime(
                &NaiveDate::from_ymd_opt(2016, 12, 31)
                    .unwrap()
                    .and_hms_nano_opt(23, 59, 59, 175_500_000)
                    .unwrap(),
            )
            .unwrap()
            .naive_utc();
        assert_eq!(
            dt.duration_trunc(TimeDelta::milliseconds(10)).unwrap().to_string(),
            "2016-12-31 23:59:59.170"
        );
        let dt = Utc
            .from_local_datetime(
                &NaiveDate::from_ymd_opt(2012, 12, 12)
                    .unwrap()
                    .and_hms_milli_opt(18, 22, 30, 0)
                    .unwrap(),
            )
            .unwrap()
            .naive_utc();
        assert_eq!(
            dt.duration_trunc(TimeDelta::minutes(5)).unwrap().to_string(),
            "2012-12-12 18:20:00"
        );
        let dt = Utc
            .from_local_datetime(
                &NaiveDate::from_ymd_opt(2012, 12, 12)
                    .unwrap()
                    .and_hms_milli_opt(18, 22, 29, 999)
                    .unwrap(),
            )
            .unwrap()
            .naive_utc();
        assert_eq!(
            dt.duration_trunc(TimeDelta::minutes(5)).unwrap().to_string(),
            "2012-12-12 18:20:00"
        );
        assert_eq!(
            dt.duration_trunc(TimeDelta::minutes(10)).unwrap().to_string(),
            "2012-12-12 18:20:00"
        );
        assert_eq!(
            dt.duration_trunc(TimeDelta::minutes(30)).unwrap().to_string(),
            "2012-12-12 18:00:00"
        );
        assert_eq!(
            dt.duration_trunc(TimeDelta::hours(1)).unwrap().to_string(),
            "2012-12-12 18:00:00"
        );
        assert_eq!(
            dt.duration_trunc(TimeDelta::days(1)).unwrap().to_string(),
            "2012-12-12 00:00:00"
        );
    }
    #[test]
    fn test_duration_trunc_pre_epoch() {
        let dt = Utc.with_ymd_and_hms(1969, 12, 12, 12, 12, 12).unwrap();
        assert_eq!(
            dt.duration_trunc(TimeDelta::minutes(10)).unwrap().to_string(),
            "1969-12-12 12:10:00 UTC"
        );
    }
}
#[cfg(test)]
mod tests_llm_16_1 {
    use crate::{DateTime, NaiveDateTime, Utc, Timelike, SubsecRound};
    #[test]
    fn test_round_subsecs() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17, mut rug_fuzz_18, mut rug_fuzz_19, mut rug_fuzz_20, mut rug_fuzz_21, mut rug_fuzz_22)) = <(&str, &str, u16, &str, bool, u16, &str, bool, u16, &str, bool, u16, bool, &str, &str, u16, &str, bool, &str, &str, u16, &str, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let dt_str = rug_fuzz_0;
        let dt = DateTime::<
            Utc,
        >::from_utc(NaiveDateTime::parse_from_str(dt_str, rug_fuzz_1).unwrap(), Utc);
        let dt_rounded_1 = dt.round_subsecs(rug_fuzz_2);
        let expected_1 = rug_fuzz_3;
        debug_assert_eq!(
            dt_rounded_1.to_rfc3339_opts(crate ::SecondsFormat::Nanos, rug_fuzz_4),
            expected_1
        );
        let dt_rounded_2 = dt.round_subsecs(rug_fuzz_5);
        let expected_2 = rug_fuzz_6;
        debug_assert_eq!(
            dt_rounded_2.to_rfc3339_opts(crate ::SecondsFormat::Nanos, rug_fuzz_7),
            expected_2
        );
        let dt_rounded_3 = dt.round_subsecs(rug_fuzz_8);
        let expected_3 = rug_fuzz_9;
        debug_assert_eq!(
            dt_rounded_3.to_rfc3339_opts(crate ::SecondsFormat::Nanos, rug_fuzz_10),
            expected_3
        );
        let dt_rounded_6 = dt.round_subsecs(rug_fuzz_11);
        let expected_6 = dt_str;
        debug_assert_eq!(
            dt_rounded_6.to_rfc3339_opts(crate ::SecondsFormat::Nanos, rug_fuzz_12),
            expected_6
        );
        let dt_halfway_str = rug_fuzz_13;
        let dt_halfway = DateTime::<
            Utc,
        >::from_utc(
            NaiveDateTime::parse_from_str(dt_halfway_str, rug_fuzz_14).unwrap(),
            Utc,
        );
        let dt_halfway_rounded = dt_halfway.round_subsecs(rug_fuzz_15);
        let expected_halfway = rug_fuzz_16;
        debug_assert_eq!(
            dt_halfway_rounded.to_rfc3339_opts(crate ::SecondsFormat::Nanos,
            rug_fuzz_17), expected_halfway
        );
        let dt_more_halfway_str = rug_fuzz_18;
        let dt_more_halfway = DateTime::<
            Utc,
        >::from_utc(
            NaiveDateTime::parse_from_str(dt_more_halfway_str, rug_fuzz_19).unwrap(),
            Utc,
        );
        let dt_more_halfway_rounded = dt_more_halfway.round_subsecs(rug_fuzz_20);
        let expected_more_halfway = rug_fuzz_21;
        debug_assert_eq!(
            dt_more_halfway_rounded.to_rfc3339_opts(crate ::SecondsFormat::Nanos,
            rug_fuzz_22), expected_more_halfway
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_2_llm_16_2 {
    use super::*;
    use crate::*;
    use crate::{DateTime, Utc, TimeZone, Timelike, SubsecRound};
    #[test]
    fn test_trunc_subsecs() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(&str, &str, u16, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let original_time_str = rug_fuzz_0;
        let original_time: DateTime<Utc> = Utc
            .datetime_from_str(original_time_str, rug_fuzz_1)
            .unwrap();
        let test_cases = vec![
            (rug_fuzz_2, rug_fuzz_3), (3, "2023-04-01T12:34:56.789Z"), (6,
            "2023-04-01T12:34:56.789101Z"), (9, "2023-04-01T12:34:56.789101112Z")
        ];
        for (digits, expected) in test_cases {
            let expected_time: DateTime<Utc> = Utc
                .datetime_from_str(expected, rug_fuzz_4)
                .unwrap();
            let truncated_time = original_time.trunc_subsecs(digits);
            debug_assert_eq!(truncated_time, expected_time);
        }
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_29 {
    use super::*;
    use crate::*;
    use crate::{DateTime, FixedOffset, TimeZone, Utc};
    #[test]
    fn test_duration_trunc() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14)) = <(i32, i32, i32, u32, u32, u32, u32, u32, i64, i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let offset = FixedOffset::east(rug_fuzz_0 * rug_fuzz_1);
        let dt: DateTime<FixedOffset> = offset
            .ymd(rug_fuzz_2, rug_fuzz_3, rug_fuzz_4)
            .and_hms(rug_fuzz_5, rug_fuzz_6, rug_fuzz_7);
        let duration = TimeDelta::minutes(rug_fuzz_8);
        let truncated = dt.duration_trunc(duration).unwrap();
        let expected_dt = offset
            .ymd(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11)
            .and_hms(rug_fuzz_12, rug_fuzz_13, rug_fuzz_14);
        debug_assert_eq!(truncated, expected_dt);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_118 {
    use super::*;
    use crate::*;
    use crate::{NaiveDate, NaiveDateTime, NaiveTime, TimeDelta};
    fn naive_date_time(date: NaiveDate, time: NaiveTime) -> NaiveDateTime {
        NaiveDateTime::new(date, time)
    }
    #[test]
    fn test_duration_trunc() {
        let date = NaiveDate::from_ymd(2023, 4, 5);
        let time = NaiveTime::from_hms(13, 46, 32);
        let datetime = naive_date_time(date, time);
        let hour_duration = TimeDelta::hours(1);
        let truncated = datetime.duration_trunc(hour_duration);
        assert_eq!(truncated, Ok(naive_date_time(date, NaiveTime::from_hms(13, 0, 0))));
        let day_duration = TimeDelta::days(1);
        let truncated = datetime.duration_trunc(day_duration);
        assert_eq!(truncated, Ok(naive_date_time(date, NaiveTime::from_hms(0, 0, 0))));
        let zero_duration = TimeDelta::seconds(0);
        let truncated = datetime.duration_trunc(zero_duration);
        assert!(truncated.is_err());
    }
}
#[cfg(test)]
mod tests_llm_16_199 {
    use crate::RoundingError;
    use std::error::Error;
    #[test]
    fn test_rounding_error_description() {
        let _rug_st_tests_llm_16_199_rrrruuuugggg_test_rounding_error_description = 0;
        let err = RoundingError::DurationExceedsTimestamp;
        debug_assert_eq!(
            err.description(), "error from rounding or truncating with DurationRound"
        );
        let err = RoundingError::DurationExceedsLimit;
        debug_assert_eq!(
            err.description(), "error from rounding or truncating with DurationRound"
        );
        let err = RoundingError::TimestampExceedsLimit;
        debug_assert_eq!(
            err.description(), "error from rounding or truncating with DurationRound"
        );
        let _rug_ed_tests_llm_16_199_rrrruuuugggg_test_rounding_error_description = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_594 {
    use super::*;
    use crate::*;
    use crate::{DurationRound, NaiveDate, NaiveTime, TimeZone, Utc};
    #[test]
    fn test_duration_trunc() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15)) = <(i32, u32, u32, u32, u32, u32, i64, i64, i64, i64, i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let time = NaiveTime::from_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let naive_datetime = date.and_time(time);
        let one_hour = TimeDelta::hours(rug_fuzz_6);
        let before_an_hour = naive_datetime + TimeDelta::minutes(rug_fuzz_7);
        debug_assert_eq!(
            round::duration_trunc(before_an_hour, naive_datetime, one_hour).unwrap(),
            naive_datetime
        );
        let an_hour_later = naive_datetime + one_hour;
        debug_assert_eq!(
            round::duration_trunc(an_hour_later, naive_datetime, one_hour).unwrap(),
            naive_datetime
        );
        let over_an_hour = naive_datetime + TimeDelta::minutes(rug_fuzz_8);
        debug_assert_eq!(
            round::duration_trunc(over_an_hour, naive_datetime, one_hour).unwrap(),
            an_hour_later
        );
        let longer_than_timestamp = one_hour + TimeDelta::hours(rug_fuzz_9);
        debug_assert!(
            round::duration_trunc(naive_datetime, naive_datetime, longer_than_timestamp)
            .is_err()
        );
        let far_future_date = NaiveDate::from_ymd(rug_fuzz_10, rug_fuzz_11, rug_fuzz_12)
            .and_hms(rug_fuzz_13, rug_fuzz_14, rug_fuzz_15);
        debug_assert!(
            round::duration_trunc(far_future_date, naive_datetime, one_hour).is_err()
        );
        let far_future_duration = TimeDelta::max_value();
        debug_assert!(
            round::duration_trunc(naive_datetime, naive_datetime, far_future_duration)
            .is_err()
        );
             }
});    }
}
#[cfg(test)]
mod tests_rug_165 {
    use super::*;
    #[test]
    fn test_span_for_digits() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: u16 = rug_fuzz_0;
        debug_assert_eq!(crate ::round::span_for_digits(p0), 10_000);
             }
});    }
}
#[cfg(test)]
mod tests_rug_166 {
    use super::*;
    use crate::{NaiveDate, NaiveDateTime, Local, Date, TimeZone, TimeDelta};
    #[test]
    fn test_duration_round() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12)) = <(i32, u32, u32, u32, u32, u32, i32, u32, u32, u32, u32, u32, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let mut p1 = Local
            .ymd(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8)
            .and_hms(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11);
        let mut p2 = TimeDelta::seconds(rug_fuzz_12);
        crate::round::duration_round(p0, p1, p2);
             }
});    }
}
#[cfg(test)]
mod tests_rug_167 {
    use super::*;
    use crate::{DateTime, TimeZone};
    use crate::offset::Local;
    #[test]
    fn test_duration_round() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: DateTime<Local> = Local::now();
        let mut p1 = TimeDelta::seconds(rug_fuzz_0);
        p0.duration_round(p1).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_168 {
    use crate::{NaiveDate, TimeDelta, DurationRound, naive};
    #[test]
    fn test_duration_round() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, u32, u32, u32, u32, u32, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let mut p1 = TimeDelta::seconds(rug_fuzz_6);
        let _result = naive::NaiveDateTime::duration_round(p0, p1).unwrap();
             }
});    }
}
