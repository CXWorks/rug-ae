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
#[timeout(30000)]fn rusty_test_7842() {
//    rusty_monitor::set_test_id(7842);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_0);
    let mut u32_0: u32 = 80u32;
    let mut u8_0: u8 = 3u8;
    let mut u8_1: u8 = 64u8;
    let mut u8_2: u8 = 2u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_0: i8 = 2i8;
    let mut i8_1: i8 = 5i8;
    let mut i8_2: i8 = 59i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_0: i32 = 1000i32;
    let mut i64_0: i64 = 0i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i8_3: i8 = 66i8;
    let mut i8_4: i8 = 5i8;
    let mut i8_5: i8 = 2i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = 59i8;
    let mut i8_7: i8 = 59i8;
    let mut i8_8: i8 = 23i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut f64_0: f64 = 261.574088f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut f32_1: f32 = 1065353216.000000f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_3, duration_2);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_6: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i8_9: i8 = 24i8;
    let mut i8_10: i8 = 2i8;
    let mut i8_11: i8 = 23i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut u32_1: u32 = 1000u32;
    let mut u8_3: u8 = 29u8;
    let mut u8_4: u8 = 42u8;
    let mut u8_5: u8 = 4u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_1: i64 = 2147483647i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut duration_8: std::time::Duration = crate::duration::Duration::abs_std(duration_7);
    let mut i64_2: i64 = 9223372036854775807i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut i64_3: i64 = 38i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_10, duration_9);
    let mut i32_1: i32 = 381i32;
    let mut i64_4: i64 = 253402300799i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_12, i32_1);
    let mut i64_5: i64 = 1000i64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_5);
    let mut i64_6: i64 = 142i64;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::hours(i64_6);
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_15, duration_14);
    let mut i8_12: i8 = 60i8;
    let mut i8_13: i8 = 3i8;
    let mut i8_14: i8 = 23i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut i8_15: i8 = 3i8;
    let mut i8_16: i8 = 127i8;
    let mut i8_17: i8 = 6i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut i64_7: i64 = 1000i64;
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::seconds(i64_7);
    let mut duration_18: std::time::Duration = crate::duration::Duration::abs_std(duration_17);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i64_8: i64 = 1000i64;
    let mut duration_19: crate::duration::Duration = crate::duration::Duration::days(i64_8);
    let mut i8_18: i8 = 127i8;
    let mut i8_19: i8 = 0i8;
    let mut i8_20: i8 = 6i8;
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_2, utcoffset_7);
    let mut time_2: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut i64_9: i64 = 604800i64;
    let mut duration_20: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_9);
    let mut i64_10: i64 = 1000000i64;
    let mut duration_21: crate::duration::Duration = crate::duration::Duration::minutes(i64_10);
    let mut duration_22: std::time::Duration = crate::duration::Duration::abs_std(duration_21);
    let mut i64_11: i64 = 2440588i64;
    let mut duration_23: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_11);
    let mut i64_12: i64 = 1000i64;
    let mut duration_24: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_12);
    let mut i64_13: i64 = 2147483647i64;
    let mut duration_25: crate::duration::Duration = crate::duration::Duration::hours(i64_13);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_4, duration_25);
    let mut time_3: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_5);
    let mut i64_14: i64 = 1000i64;
    let mut duration_26: crate::duration::Duration = crate::duration::Duration::minutes(i64_14);
    let mut i32_2: i32 = 65i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_26);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_2, time_3);
    let mut primitivedatetime_0_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_0;
    let mut i64_15: i64 = 3600i64;
    let mut duration_27: crate::duration::Duration = crate::duration::Duration::minutes(i64_15);
    let mut i64_16: i64 = 24i64;
    let mut duration_28: crate::duration::Duration = crate::duration::Duration::microseconds(i64_16);
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_6, duration_28);
    let mut time_4: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_7);
    let mut i64_17: i64 = 604800i64;
    let mut duration_29: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_17);
    let mut i128_0: i128 = 9223372036854775807i128;
    let mut duration_30: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_31: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_30, duration_29);
    let mut i32_3: i32 = 60i32;
    let mut i64_18: i64 = 60i64;
    let mut duration_32: crate::duration::Duration = crate::duration::Duration::microseconds(i64_18);
    let mut duration_33: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_32, i32_3);
    let mut offsetdatetime_8: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_9: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_8, duration_33);
    let mut date_3: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_9);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_add(date_3, duration_31);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_4, time_4);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_1, duration_27);
    let mut primitivedatetime_2_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_2;
    let mut i32_4: i32 = 268i32;
    let mut i64_19: i64 = 1000i64;
    let mut duration_34: crate::duration::Duration = crate::duration::Duration::weeks(i64_19);
    let mut date_5: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
//    panic!("From RustyUnit with love");
}
}