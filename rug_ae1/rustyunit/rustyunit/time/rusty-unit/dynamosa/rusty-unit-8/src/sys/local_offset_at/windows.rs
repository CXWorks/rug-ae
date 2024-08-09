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
fn rusty_test_7133() {
    rusty_monitor::set_test_id(7133);
    let mut i32_0: i32 = -55i32;
    let mut i64_0: i64 = -104i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut u32_0: u32 = 48u32;
    let mut u8_0: u8 = 22u8;
    let mut u8_1: u8 = 21u8;
    let mut u8_2: u8 = 98u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_1: i64 = 55i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut i64_2: i64 = 72i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_2, duration_1);
    let mut i32_1: i32 = -123i32;
    let mut i64_3: i64 = -152i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_1);
    let mut i8_0: i8 = 51i8;
    let mut i8_1: i8 = 76i8;
    let mut i8_2: i8 = -79i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut f32_0: f32 = -38.884742f32;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i64_4: i64 = 75i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_4);
    let mut i32_2: i32 = 11i32;
    let mut i64_5: i64 = -15i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::hours(i64_5);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_7, i32_2);
    let mut duration_9: std::time::Duration = crate::duration::Duration::abs_std(duration_8);
    let mut i32_3: i32 = -41i32;
    let mut i64_6: i64 = 130i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_6, i32_3);
    let mut duration_11: std::time::Duration = crate::duration::Duration::abs_std(duration_10);
    let mut i8_3: i8 = -76i8;
    let mut i8_4: i8 = 2i8;
    let mut i8_5: i8 = -38i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut i32_4: i32 = -16i32;
    let mut i32_5: i32 = -5i32;
    let mut i64_7: i64 = 74i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::new(i64_7, i32_5);
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_12, i32_4);
    let mut u32_1: u32 = 39u32;
    let mut u8_3: u8 = 97u8;
    let mut u8_4: u8 = 50u8;
    let mut u8_5: u8 = 36u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i8_6: i8 = -89i8;
    let mut i8_7: i8 = 97i8;
    let mut i8_8: i8 = -94i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i128_0: i128 = 64i128;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i8_9: i8 = -24i8;
    let mut i8_10: i8 = -109i8;
    let mut i8_11: i8 = -51i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i64_8: i64 = 1i64;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::weeks(i64_8);
    let mut i64_9: i64 = -56i64;
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::hours(i64_9);
    let mut i64_10: i64 = 18i64;
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_10);
    let mut duration_18: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_17, duration_16);
    let mut duration_19: std::time::Duration = crate::duration::Duration::abs_std(duration_18);
    let mut i64_11: i64 = -10i64;
    let mut duration_20: crate::duration::Duration = crate::duration::Duration::days(i64_11);
    let mut i32_6: i32 = 33i32;
    let mut i64_12: i64 = 90i64;
    let mut duration_21: crate::duration::Duration = crate::duration::Duration::seconds(i64_12);
    let mut duration_22: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_21, i32_6);
    let mut duration_23: std::time::Duration = crate::duration::Duration::abs_std(duration_22);
    let mut u16_0: u16 = 1u16;
    let mut i32_7: i32 = 47i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_7, u16_0);
    let mut i8_12: i8 = 33i8;
    let mut i8_13: i8 = 3i8;
    let mut i8_14: i8 = -44i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_2, utcoffset_4);
    let mut time_2: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut i32_8: i32 = 57i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_8);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_3, time_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_2);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i32_9: i32 = -158i32;
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_9};
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut i8_15: i8 = 26i8;
    let mut i8_16: i8 = -43i8;
    let mut i8_17: i8 = -3i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut i8_18: i8 = -41i8;
    let mut i8_19: i8 = 26i8;
    let mut i8_20: i8 = -74i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut i32_10: i32 = -40i32;
    let mut date_5: crate::date::Date = crate::date::Date {value: i32_10};
    let mut f32_1: f32 = -13.974023f32;
    let mut duration_24: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_25: std::time::Duration = crate::duration::Duration::abs_std(duration_24);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_26: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut i32_11: i32 = -19i32;
    let mut date_6: crate::date::Date = crate::date::Date {value: i32_11};
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_6);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_3, duration_26);
    let mut i64_13: i64 = 156i64;
    let mut u16_1: u16 = 69u16;
    let mut i32_12: i32 = -171i32;
    let mut date_7: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_12, u16_1);
    let mut i32_13: i32 = 122i32;
    let mut i64_14: i64 = 25i64;
    let mut duration_27: crate::duration::Duration = crate::duration::Duration::weeks(i64_14);
    let mut duration_28: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_27, i32_13);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_5, duration_28);
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_6);
    let mut i8_21: i8 = 78i8;
    let mut i8_22: i8 = -91i8;
    let mut i8_23: i8 = -71i8;
    let mut utcoffset_8: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_23, i8_22, i8_21);
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_8: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_7, utcoffset_8);
    let mut date_8: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_8);
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_8);
    let mut primitivedatetime_6: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_5, utcoffset_0);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    let mut u8_6: u8 = 58u8;
    let mut i32_14: i32 = 75i32;
    let mut f64_0: f64 = -32.307070f64;
    let mut duration_29: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut u32_2: u32 = 19u32;
    let mut u8_7: u8 = 37u8;
    let mut u8_8: u8 = 79u8;
    let mut u8_9: u8 = 98u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_9, u8_8, u8_7, u32_2);
    let mut i64_15: i64 = -56i64;
    let mut duration_30: crate::duration::Duration = crate::duration::Duration::seconds(i64_15);
    let mut u16_2: u16 = 49u16;
    let mut i32_15: i32 = 21i32;
    let mut date_9: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_15, u16_2);
    let mut date_10: crate::date::Date = crate::date::Date::saturating_add(date_9, duration_30);
    let mut primitivedatetime_7: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_10, time_3);
    let mut primitivedatetime_8: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_7, duration_29);
    let mut i64_16: i64 = 123i64;
    let mut duration_31: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_16);
    let mut i32_16: i32 = -194i32;
    let mut date_11: crate::date::Date = crate::date::Date {value: i32_16};
    let mut i64_17: i64 = 2i64;
    let mut duration_32: crate::duration::Duration = crate::duration::Duration::weeks(i64_17);
    let mut i64_18: i64 = 26i64;
    let mut duration_33: crate::duration::Duration = crate::duration::Duration::weeks(i64_18);
    let mut duration_34: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_33, duration_32);
    let mut i32_17: i32 = -179i32;
    let mut date_12: crate::date::Date = crate::date::Date {value: i32_17};
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_14, u8_6, weekday_0);
    let mut tuple_0: (i32, month::Month, u8) = crate::primitive_date_time::PrimitiveDateTime::to_calendar_date(primitivedatetime_6);
    let mut u8_10: u8 = crate::date::Date::sunday_based_week(date_7);
    let mut result_1: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp(i64_13);
    let mut tuple_1: (i32, u8, weekday::Weekday) = crate::primitive_date_time::PrimitiveDateTime::to_iso_week_date(primitivedatetime_4);
    let mut i128_1: i128 = crate::duration::Duration::whole_nanoseconds(duration_34);
    panic!("From RustyUnit with love");
}
}