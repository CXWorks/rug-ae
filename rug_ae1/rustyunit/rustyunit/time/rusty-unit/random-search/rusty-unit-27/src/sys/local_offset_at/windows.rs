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

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3521() {
    rusty_monitor::set_test_id(3521);
    let mut f32_0: f32 = 124.743411f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut f32_1: f32 = 13.016724f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut i64_0: i64 = 271i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::abs(duration_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_4);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i64_1: i64 = 155i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_5);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_2);
    let mut u8_0: u8 = crate::primitive_date_time::PrimitiveDateTime::second(primitivedatetime_1);
    panic!("From RustyUnit with love");
}
}