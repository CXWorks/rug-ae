//! Get the system's UTC offset on Unix.

use core::convert::TryInto;
use core::mem::MaybeUninit;

use crate::{OffsetDateTime, UtcOffset};

/// Convert the given Unix timestamp to a `libc::tm`. Returns `None` on any error.
///
/// # Safety
///
/// This method must only be called when the process is single-threaded.
///
/// This method will remain `unsafe` until `std::env::set_var` is deprecated or has its behavior
/// altered. This method is, on its own, safe. It is the presence of a safe, unsound way to set
/// environment variables that makes it unsafe.
unsafe fn timestamp_to_tm(timestamp: i64) -> Option<libc::tm> {
    extern "C" {
        #[cfg_attr(target_os = "netbsd", link_name = "__tzset50")]
        fn tzset();
    }

    // The exact type of `timestamp` beforehand can vary, so this conversion is necessary.
    #[allow(clippy::useless_conversion)]
    let timestamp = timestamp.try_into().ok()?;

    let mut tm = MaybeUninit::uninit();

    // Update timezone information from system. `localtime_r` does not do this for us.
    //
    // Safety: tzset is thread-safe.
    unsafe { tzset() };

    // Safety: We are calling a system API, which mutates the `tm` variable. If a null
    // pointer is returned, an error occurred.
    let tm_ptr = unsafe { libc::localtime_r(&timestamp, tm.as_mut_ptr()) };

    if tm_ptr.is_null() {
        None
    } else {
        // Safety: The value was initialized, as we no longer have a null pointer.
        Some(unsafe { tm.assume_init() })
    }
}

/// Convert a `libc::tm` to a `UtcOffset`. Returns `None` on any error.
// `tm_gmtoff` extension
#[cfg(not(any(target_os = "solaris", target_os = "illumos")))]
fn tm_to_offset(tm: libc::tm) -> Option<UtcOffset> {
    let seconds: i32 = tm.tm_gmtoff.try_into().ok()?;
    UtcOffset::from_hms(
        (seconds / 3_600) as _,
        ((seconds / 60) % 60) as _,
        (seconds % 60) as _,
    )
    .ok()
}

/// Convert a `libc::tm` to a `UtcOffset`. Returns `None` on any error.
// Solaris/Illumos is unsound and requires opting into.
#[cfg(all(
    not(unsound_local_offset),
    any(target_os = "solaris", target_os = "illumos")
))]
#[allow(unused_variables, clippy::missing_const_for_fn)]
fn tm_to_offset(tm: libc::tm) -> Option<UtcOffset> {
    None
}

/// Convert a `libc::tm` to a `UtcOffset`. Returns `None` on any error.
#[cfg(all(
    unsound_local_offset,
    any(target_os = "solaris", target_os = "illumos")
))]
fn tm_to_offset(tm: libc::tm) -> Option<UtcOffset> {
    use core::convert::TryFrom;

    use crate::Date;

    let mut tm = tm;
    if tm.tm_sec == 60 {
        // Leap seconds are not currently supported.
        tm.tm_sec = 59;
    }

    let local_timestamp =
        Date::from_ordinal_date(1900 + tm.tm_year, u16::try_from(tm.tm_yday).ok()? + 1)
            .ok()?
            .with_hms(
                tm.tm_hour.try_into().ok()?,
                tm.tm_min.try_into().ok()?,
                tm.tm_sec.try_into().ok()?,
            )
            .ok()?
            .assume_utc()
            .unix_timestamp();

    let diff_secs: i32 = (local_timestamp - datetime.unix_timestamp())
        .try_into()
        .ok()?;

    UtcOffset::from_hms(
        (diff_secs / 3_600) as _,
        ((diff_secs / 60) % 60) as _,
        (diff_secs % 60) as _,
    )
    .ok()
}

/// Obtain the system's UTC offset.
pub(super) fn local_offset_at(datetime: OffsetDateTime) -> Option<UtcOffset> {
    // Ensure that the process is single-threaded unless the user has explicitly opted out of this
    // check. This is to prevent issues with the environment being mutated by a different thread in
    // the process while execution of this function is taking place, which can cause a segmentation
    // fault by dereferencing a dangling pointer.
    // If the `num_threads` crate is incapable of determining the number of running threads, then
    // we conservatively return `None` to avoid a soundness bug.
    if !cfg!(unsound_local_offset) && num_threads::is_single_threaded() != Some(true) {
        return None;
    }

    // Safety: We have just confirmed that the process is single-threaded or the user has explicitly
    // opted out of soundness.
    let tm = unsafe { timestamp_to_tm(datetime.unix_timestamp()) }?;
    tm_to_offset(tm)
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4240() {
    rusty_monitor::set_test_id(4240);
    let mut i64_0: i64 = 148i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut i8_0: i8 = 16i8;
    let mut i8_1: i8 = -17i8;
    let mut i8_2: i8 = -128i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut u8_0: u8 = 60u8;
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_0: i32 = -46i32;
    let mut i64_1: i64 = 16i64;
    let mut i32_1: i32 = 62i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    panic!("From RustyUnit with love");
}
}