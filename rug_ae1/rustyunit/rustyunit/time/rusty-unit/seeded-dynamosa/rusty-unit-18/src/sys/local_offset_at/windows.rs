//! Get the system's UTC offset on Windows.

use core::convert::TryInto;
use core::mem::MaybeUninit;

use crate::{OffsetDateTime, UtcOffset};

// ffi: WINAPI FILETIME struct
#[repr(C)]
#[allow(non_snake_case)]
struct FileTime {
    dwLowDateTime: u32,
    dwHighDateTime: u32,
}

// ffi: WINAPI SYSTEMTIME struct
#[repr(C)]
#[allow(non_snake_case)]
struct SystemTime {
    wYear: u16,
    wMonth: u16,
    wDayOfWeek: u16,
    wDay: u16,
    wHour: u16,
    wMinute: u16,
    wSecond: u16,
    wMilliseconds: u16,
}

#[link(name = "kernel32")]
extern "system" {
    // https://docs.microsoft.com/en-us/windows/win32/api/timezoneapi/nf-timezoneapi-systemtimetofiletime
    fn SystemTimeToFileTime(lpSystemTime: *const SystemTime, lpFileTime: *mut FileTime) -> i32;

    // https://docs.microsoft.com/en-us/windows/win32/api/timezoneapi/nf-timezoneapi-systemtimetotzspecificlocaltime
    fn SystemTimeToTzSpecificLocalTime(
        lpTimeZoneInformation: *const std::ffi::c_void, // We only pass `nullptr` here
        lpUniversalTime: *const SystemTime,
        lpLocalTime: *mut SystemTime,
    ) -> i32;
}

/// Convert a `SYSTEMTIME` to a `FILETIME`. Returns `None` if any error occurred.
fn systemtime_to_filetime(systime: &SystemTime) -> Option<FileTime> {
    let mut ft = MaybeUninit::uninit();

    // Safety: `SystemTimeToFileTime` is thread-safe.
    if 0 == unsafe { SystemTimeToFileTime(systime, ft.as_mut_ptr()) } {
        // failed
        None
    } else {
        // Safety: The call succeeded.
        Some(unsafe { ft.assume_init() })
    }
}

/// Convert a `FILETIME` to an `i64`, representing a number of seconds.
fn filetime_to_secs(filetime: &FileTime) -> i64 {
    /// FILETIME represents 100-nanosecond intervals
    const FT_TO_SECS: i64 = 10_000_000;
    ((filetime.dwHighDateTime as i64) << 32 | filetime.dwLowDateTime as i64) / FT_TO_SECS
}

/// Convert an [`OffsetDateTime`] to a `SYSTEMTIME`.
fn offset_to_systemtime(datetime: OffsetDateTime) -> SystemTime {
    let (_, month, day_of_month) = datetime.to_offset(UtcOffset::UTC).date().to_calendar_date();
    SystemTime {
        wYear: datetime.year() as _,
        wMonth: month as _,
        wDay: day_of_month as _,
        wDayOfWeek: 0, // ignored
        wHour: datetime.hour() as _,
        wMinute: datetime.minute() as _,
        wSecond: datetime.second() as _,
        wMilliseconds: datetime.millisecond(),
    }
}

/// Obtain the system's UTC offset.
pub(super) fn local_offset_at(datetime: OffsetDateTime) -> Option<UtcOffset> {
    // This function falls back to UTC if any system call fails.
    let systime_utc = offset_to_systemtime(datetime.to_offset(UtcOffset::UTC));

    // Safety: `local_time` is only read if it is properly initialized, and
    // `SystemTimeToTzSpecificLocalTime` is thread-safe.
    let systime_local = unsafe {
        let mut local_time = MaybeUninit::uninit();

        if 0 == SystemTimeToTzSpecificLocalTime(
            core::ptr::null(), // use system's current timezone
            &systime_utc,
            local_time.as_mut_ptr(),
        ) {
            // call failed
            return None;
        } else {
            local_time.assume_init()
        }
    };

    // Convert SYSTEMTIMEs to FILETIMEs so we can perform arithmetic on them.
    let ft_system = systemtime_to_filetime(&systime_utc)?;
    let ft_local = systemtime_to_filetime(&systime_local)?;

    let diff_secs: i32 = (filetime_to_secs(&ft_local) - filetime_to_secs(&ft_system))
        .try_into()
        .ok()?;

    UtcOffset::from_hms(
        (diff_secs / 3_600) as _,
        ((diff_secs / 60) % 60) as _,
        (diff_secs % 60) as _,
    )
    .ok()
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_362() {
//    rusty_monitor::set_test_id(362);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut month_1: month::Month = crate::month::Month::March;
    let mut month_2: month::Month = crate::month::Month::February;
    let mut month_3: month::Month = crate::month::Month::September;
    let mut month_4: month::Month = crate::month::Month::September;
    let mut month_5: month::Month = crate::month::Month::June;
    let mut month_6: month::Month = crate::month::Month::July;
    let mut month_7: month::Month = crate::month::Month::July;
    let mut month_8: month::Month = crate::month::Month::November;
    let mut month_9: month::Month = crate::month::Month::previous(month_8);
    let mut month_10: month::Month = crate::month::Month::June;
    let mut month_11: month::Month = crate::month::Month::previous(month_10);
    let mut month_12: month::Month = crate::month::Month::September;
    let mut month_13: month::Month = crate::month::Month::July;
    let mut month_14: month::Month = crate::month::Month::June;
    let mut month_15: month::Month = crate::month::Month::March;
    let mut month_16: month::Month = crate::month::Month::July;
    let mut month_17: month::Month = crate::month::Month::previous(month_16);
    let mut month_18: month::Month = crate::month::Month::previous(month_15);
    let mut month_19: month::Month = crate::month::Month::previous(month_14);
    let mut month_20: month::Month = crate::month::Month::previous(month_13);
    let mut month_21: month::Month = crate::month::Month::previous(month_12);
    let mut month_22: month::Month = crate::month::Month::previous(month_11);
    let mut month_23: month::Month = crate::month::Month::previous(month_9);
    let mut month_24: month::Month = crate::month::Month::previous(month_7);
    let mut month_25: month::Month = crate::month::Month::previous(month_6);
    let mut month_26: month::Month = crate::month::Month::previous(month_5);
    let mut month_27: month::Month = crate::month::Month::previous(month_4);
    let mut month_28: month::Month = crate::month::Month::previous(month_3);
    let mut month_29: month::Month = crate::month::Month::previous(month_2);
    let mut month_30: month::Month = crate::month::Month::previous(month_1);
    let mut month_31: month::Month = crate::month::Month::previous(month_0);
//    panic!("From RustyUnit with love");
}
}