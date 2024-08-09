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
fn rusty_test_4842() {
    rusty_monitor::set_test_id(4842);
    let mut i64_0: i64 = 31i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut f64_0: f64 = -97.895637f64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_1);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut date_1_ref_0: &crate::date::Date = &mut date_1;
    let mut i8_0: i8 = 49i8;
    let mut i8_1: i8 = -69i8;
    let mut i8_2: i8 = -51i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = -75i8;
    let mut i8_4: i8 = 27i8;
    let mut i8_5: i8 = 120i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_1: i64 = -97i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut u32_0: u32 = 93u32;
    let mut u8_0: u8 = 96u8;
    let mut u8_1: u8 = 44u8;
    let mut u8_2: u8 = 63u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_2: i64 = -164i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut i8_6: i8 = 85i8;
    let mut i8_7: i8 = 12i8;
    let mut i8_8: i8 = 115i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i128_0: i128 = -28i128;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i64_3: i64 = 78i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut i32_0: i32 = 182i32;
    let mut i64_4: i64 = -110i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_4, i32_0);
    let mut duration_7: std::time::Duration = crate::duration::Duration::abs_std(duration_6);
    let mut f64_1: f64 = -29.856124f64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut duration_9: std::time::Duration = crate::duration::Duration::abs_std(duration_8);
    let mut i8_9: i8 = -23i8;
    let mut i8_10: i8 = -21i8;
    let mut i8_11: i8 = 20i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i8_12: i8 = -17i8;
    let mut i8_13: i8 = 46i8;
    let mut i8_14: i8 = 21i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut u32_1: u32 = 4u32;
    let mut u8_3: u8 = 45u8;
    let mut u8_4: u8 = 79u8;
    let mut u8_5: u8 = 42u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_5: i64 = 183i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::hours(i64_5);
    let mut f64_2: f64 = -57.885005f64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_6: i64 = -170i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_6);
    let mut i8_15: i8 = -117i8;
    let mut i8_16: i8 = 15i8;
    let mut i8_17: i8 = -115i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut i64_7: i64 = -99i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::days(i64_7);
    let mut i8_18: i8 = -35i8;
    let mut i8_19: i8 = -74i8;
    let mut i8_20: i8 = -14i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut u32_2: u32 = 95u32;
    let mut u8_6: u8 = 74u8;
    let mut u8_7: u8 = 8u8;
    let mut u8_8: u8 = 85u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i8_21: i8 = -106i8;
    let mut i8_22: i8 = 94i8;
    let mut i8_23: i8 = 25i8;
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_23, i8_22, i8_21);
    let mut i8_24: i8 = 104i8;
    let mut i8_25: i8 = 31i8;
    let mut i8_26: i8 = 56i8;
    let mut utcoffset_8: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_26, i8_25, i8_24);
    let mut u32_3: u32 = 84u32;
    let mut u8_9: u8 = 14u8;
    let mut u8_10: u8 = 89u8;
    let mut u8_11: u8 = 5u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_3);
    let mut i64_8: i64 = 62i64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::weeks(i64_8);
    let mut i64_9: i64 = -132i64;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::days(i64_9);
    let mut i8_27: i8 = 8i8;
    let mut i8_28: i8 = 20i8;
    let mut i8_29: i8 = -47i8;
    let mut utcoffset_9: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_29, i8_28, i8_27);
    let mut i8_30: i8 = -35i8;
    let mut i8_31: i8 = 59i8;
    let mut i8_32: i8 = -112i8;
    let mut utcoffset_10: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_32, i8_31, i8_30);
    let mut i8_33: i8 = 103i8;
    let mut i8_34: i8 = 42i8;
    let mut i8_35: i8 = -19i8;
    let mut utcoffset_11: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_35, i8_34, i8_33);
    let mut f64_3: f64 = -149.982034f64;
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_3);
    let mut u32_4: u32 = 16u32;
    let mut u8_12: u8 = 1u8;
    let mut u8_13: u8 = 84u8;
    let mut u8_14: u8 = 20u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_14, u8_13, u8_12, u32_4);
    let mut i8_36: i8 = 59i8;
    let mut i8_37: i8 = 80i8;
    let mut i8_38: i8 = -90i8;
    let mut utcoffset_12: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_38, i8_37, i8_36);
    let mut f64_4: f64 = 0.334274f64;
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_4);
    let mut i32_1: i32 = -163i32;
    let mut i64_10: i64 = 110i64;
    let mut duration_18: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_10, i32_1);
    let mut duration_19: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_18, duration_17);
    let mut u32_5: u32 = 44u32;
    let mut u8_15: u8 = 65u8;
    let mut u8_16: u8 = 97u8;
    let mut u8_17: u8 = 42u8;
    let mut time_5: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_17, u8_16, u8_15, u32_5);
    let mut i64_11: i64 = -261i64;
    let mut duration_20: crate::duration::Duration = crate::duration::Duration::days(i64_11);
    let mut duration_21: std::time::Duration = crate::duration::Duration::abs_std(duration_20);
    let mut i64_12: i64 = 0i64;
    let mut duration_22: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_12);
    let mut f32_0: f32 = -128.623372f32;
    let mut duration_23: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut f32_1: f32 = -121.617107f32;
    let mut duration_24: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_25: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_24, duration_23);
    let mut i8_39: i8 = -3i8;
    let mut i8_40: i8 = -11i8;
    let mut i8_41: i8 = -41i8;
    let mut utcoffset_13: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_41, i8_40, i8_39);
    let mut i8_42: i8 = 61i8;
    let mut i8_43: i8 = -45i8;
    let mut i8_44: i8 = 4i8;
    let mut utcoffset_14: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_44, i8_43, i8_42);
    let mut i64_13: i64 = 69i64;
    let mut duration_26: crate::duration::Duration = crate::duration::Duration::seconds(i64_13);
    let mut i64_14: i64 = -152i64;
    let mut duration_27: crate::duration::Duration = crate::duration::Duration::microseconds(i64_14);
    let mut f32_2: f32 = -19.305392f32;
    let mut duration_28: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_2);
    let mut duration_29: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_28, duration_27);
    let mut i32_2: i32 = -164i32;
    let mut i64_15: i64 = 170i64;
    let mut duration_30: crate::duration::Duration = crate::duration::Duration::new(i64_15, i32_2);
    let mut duration_31: std::time::Duration = crate::duration::Duration::abs_std(duration_30);
    let mut i8_45: i8 = -10i8;
    let mut i8_46: i8 = -49i8;
    let mut i8_47: i8 = 36i8;
    let mut utcoffset_15: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_47, i8_46, i8_45);
    let mut f32_3: f32 = -3.380830f32;
    let mut duration_32: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_3);
    let mut i64_16: i64 = -127i64;
    let mut duration_33: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_16);
    let mut duration_34: std::time::Duration = crate::duration::Duration::abs_std(duration_33);
    let mut f64_5: f64 = 24.838522f64;
    let mut duration_35: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_5);
    let mut duration_36: std::time::Duration = crate::duration::Duration::abs_std(duration_35);
    let mut i8_48: i8 = -77i8;
    let mut i8_49: i8 = 12i8;
    let mut i8_50: i8 = 116i8;
    let mut utcoffset_16: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_50, i8_49, i8_48);
    let mut u32_6: u32 = 42u32;
    let mut u8_18: u8 = 72u8;
    let mut u8_19: u8 = 64u8;
    let mut u8_20: u8 = 12u8;
    let mut time_6: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_20, u8_19, u8_18, u32_6);
    let mut i64_17: i64 = 213i64;
    let mut duration_37: crate::duration::Duration = crate::duration::Duration::microseconds(i64_17);
    let mut i8_51: i8 = 69i8;
    let mut i8_52: i8 = -120i8;
    let mut i8_53: i8 = -18i8;
    let mut utcoffset_17: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_53, i8_52, i8_51);
    let mut f64_6: f64 = -18.561935f64;
    let mut duration_38: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_6);
    let mut f32_4: f32 = -7.504794f32;
    let mut duration_39: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_4);
    let mut i8_54: i8 = -13i8;
    let mut i64_18: i64 = 131i64;
    let mut duration_40: crate::duration::Duration = crate::duration::Duration::seconds(i64_18);
    let mut i64_19: i64 = 98i64;
    let mut duration_41: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_19);
    let mut duration_42: std::time::Duration = crate::duration::Duration::abs_std(duration_41);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut f32_5: f32 = -17.575020f32;
    let mut duration_43: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_5);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    panic!("From RustyUnit with love");
}
}