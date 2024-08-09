use std::{cell::RefCell, collections::hash_map, env, fs, hash::Hasher, time::SystemTime};
use super::tz_info::TimeZone;
use super::{DateTime, FixedOffset, Local, NaiveDateTime};
use crate::{Datelike, LocalResult, Utc};
pub(super) fn now() -> DateTime<Local> {
    let now = Utc::now().naive_utc();
    naive_to_local(&now, false).unwrap()
}
pub(super) fn naive_to_local(
    d: &NaiveDateTime,
    local: bool,
) -> LocalResult<DateTime<Local>> {
    TZ_INFO
        .with(|maybe_cache| {
            maybe_cache.borrow_mut().get_or_insert_with(Cache::default).offset(*d, local)
        })
}
thread_local! {
    static TZ_INFO : RefCell < Option < Cache >> = Default::default();
}
enum Source {
    LocalTime { mtime: SystemTime },
    Environment { hash: u64 },
}
impl Source {
    fn new(env_tz: Option<&str>) -> Source {
        match env_tz {
            Some(tz) => {
                let mut hasher = hash_map::DefaultHasher::new();
                hasher.write(tz.as_bytes());
                let hash = hasher.finish();
                Source::Environment { hash }
            }
            None => {
                match fs::symlink_metadata("/etc/localtime") {
                    Ok(data) => {
                        Source::LocalTime {
                            mtime: data.modified().unwrap_or_else(|_| SystemTime::now()),
                        }
                    }
                    Err(_) => {
                        Source::LocalTime {
                            mtime: SystemTime::now(),
                        }
                    }
                }
            }
        }
    }
}
struct Cache {
    zone: TimeZone,
    source: Source,
    last_checked: SystemTime,
}
#[cfg(target_os = "android")]
const TZDB_LOCATION: &str = " /system/usr/share/zoneinfo";
#[cfg(target_os = "aix")]
const TZDB_LOCATION: &str = "/usr/share/lib/zoneinfo";
#[allow(dead_code)]
#[cfg(not(any(target_os = "android", target_os = "aix")))]
const TZDB_LOCATION: &str = "/usr/share/zoneinfo";
fn fallback_timezone() -> Option<TimeZone> {
    let tz_name = iana_time_zone::get_timezone().ok()?;
    let bytes = fs::read(format!("{}/{}", TZDB_LOCATION, tz_name)).ok()?;
    TimeZone::from_tz_data(&bytes).ok()
}
impl Default for Cache {
    fn default() -> Cache {
        let env_tz = env::var("TZ").ok();
        let env_ref = env_tz.as_deref();
        Cache {
            last_checked: SystemTime::now(),
            source: Source::new(env_ref),
            zone: current_zone(env_ref),
        }
    }
}
fn current_zone(var: Option<&str>) -> TimeZone {
    TimeZone::local(var).ok().or_else(fallback_timezone).unwrap_or_else(TimeZone::utc)
}
impl Cache {
    fn offset(&mut self, d: NaiveDateTime, local: bool) -> LocalResult<DateTime<Local>> {
        let now = SystemTime::now();
        match now.duration_since(self.last_checked) {
            Ok(d) if d.as_secs() < 1 => {}
            Ok(_) | Err(_) => {
                let env_tz = env::var("TZ").ok();
                let env_ref = env_tz.as_deref();
                let new_source = Source::new(env_ref);
                let out_of_date = match (&self.source, &new_source) {
                    (Source::Environment { .. }, Source::LocalTime { .. })
                    | (Source::LocalTime { .. }, Source::Environment { .. }) => true,
                    (
                        Source::LocalTime { mtime: old_mtime },
                        Source::LocalTime { mtime },
                    ) if old_mtime != mtime => true,
                    (
                        Source::Environment { hash: old_hash },
                        Source::Environment { hash },
                    ) if old_hash != hash => true,
                    _ => false,
                };
                if out_of_date {
                    self.zone = current_zone(env_ref);
                }
                self.last_checked = now;
                self.source = new_source;
            }
        }
        if !local {
            let offset = self
                .zone
                .find_local_time_type(d.timestamp())
                .expect("unable to select local time type")
                .offset();
            return match FixedOffset::east_opt(offset) {
                Some(offset) => LocalResult::Single(DateTime::from_utc(d, offset)),
                None => LocalResult::None,
            };
        }
        match self
            .zone
            .find_local_time_type_from_local(d.timestamp(), d.year())
            .expect("unable to select local time type")
        {
            LocalResult::None => LocalResult::None,
            LocalResult::Ambiguous(early, late) => {
                let early_offset = FixedOffset::east_opt(early.offset()).unwrap();
                let late_offset = FixedOffset::east_opt(late.offset()).unwrap();
                LocalResult::Ambiguous(
                    DateTime::from_utc(d - early_offset, early_offset),
                    DateTime::from_utc(d - late_offset, late_offset),
                )
            }
            LocalResult::Single(tt) => {
                let offset = FixedOffset::east_opt(tt.offset()).unwrap();
                LocalResult::Single(DateTime::from_utc(d - offset, offset))
            }
        }
    }
}
#[cfg(test)]
mod tests_llm_16_519 {
    use super::*;
    use crate::*;
    use crate::NaiveDateTime;
    use crate::offset::LocalResult;
    fn naive_date_time(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        min: u32,
        sec: u32,
    ) -> NaiveDateTime {
        NaiveDateTime::new(
            NaiveDate::from_ymd(year, month, day),
            NaiveTime::from_hms(hour, min, sec),
        )
    }
    #[test]
    fn test_offset() {
        let mut cache: Cache = Cache::default();
        let d = naive_date_time(2023, 4, 10, 10, 0, 0);
        match cache.offset(d, true) {
            LocalResult::None => {}
            LocalResult::Single(dt) => {
                assert_eq!(dt.date().year(), 2023);
                assert_eq!(dt.date().month(), 4);
                assert_eq!(dt.date().day(), 10);
                assert_eq!(dt.time().hour(), 10);
                assert_eq!(dt.time().minute(), 0);
                assert_eq!(dt.time().second(), 0);
            }
            LocalResult::Ambiguous(_, _) => {}
        }
        match cache.offset(d, false) {
            LocalResult::None => {}
            LocalResult::Single(dt) => {
                assert_eq!(dt.date().year(), 2023);
                assert_eq!(dt.date().month(), 4);
                assert_eq!(dt.date().day(), 10);
                assert_eq!(dt.time().hour(), 10);
                assert_eq!(dt.time().minute(), 0);
                assert_eq!(dt.time().second(), 0);
            }
            LocalResult::Ambiguous(_, _) => {}
        }
    }
}
#[cfg(test)]
mod tests_llm_16_520 {
    use super::*;
    use crate::*;
    use std::collections::hash_map;
    use std::fs;
    use std::time::SystemTime;
    #[test]
    fn test_source_new_with_environment() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let env_tz = Some(rug_fuzz_0);
        let source = Source::new(env_tz);
        if let Source::Environment { hash } = source {
            let mut hasher = hash_map::DefaultHasher::new();
            hasher.write(env_tz.unwrap().as_bytes());
            let expected_hash = hasher.finish();
            debug_assert_eq!(hash, expected_hash);
        } else {
            panic!("Expected Source::Environment");
        }
             }
}
}
}    }
    #[test]
    fn test_source_new_with_no_environment_and_localtime_exists() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(&str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let _ = fs::remove_file(rug_fuzz_0);
        let _ = fs::File::create(rug_fuzz_1);
        let source = Source::new(None);
        if let Source::LocalTime { mtime } = source {
            let metadata = fs::symlink_metadata(rug_fuzz_2).expect(rug_fuzz_3);
            let expected_mtime = metadata.modified().unwrap();
            debug_assert_eq!(mtime, expected_mtime);
        } else {
            panic!("Expected Source::LocalTime");
        }
        let _ = fs::remove_file(rug_fuzz_4);
             }
}
}
}    }
    #[test]
    fn test_source_new_with_no_environment_and_localtime_missing() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let _ = fs::remove_file(rug_fuzz_0);
        let source = Source::new(None);
        if let Source::LocalTime { mtime } = source {
            let now = SystemTime::now();
            debug_assert!(mtime <= now);
        } else {
            panic!("Expected Source::LocalTime");
        }
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_521_llm_16_521 {
    use crate::offset::local::inner::current_zone;
    use crate::offset::local::tz_info::TimeZone;
    use std::env;
    #[test]
    fn test_current_zone_with_none() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        env::remove_var(rug_fuzz_0);
        let result = current_zone(None);
        debug_assert_eq!(result, TimeZone::utc());
             }
}
}
}    }
    #[test]
    fn test_current_zone_with_empty_string() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let result = current_zone(Some(rug_fuzz_0));
        debug_assert_eq!(result, TimeZone::utc());
             }
}
}
}    }
    #[test]
    fn test_current_zone_with_invalid_string() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let result = current_zone(Some(rug_fuzz_0));
        debug_assert_eq!(result, TimeZone::utc());
             }
}
}
}    }
    #[test]
    fn test_current_zone_with_utc_string() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let result = current_zone(Some(rug_fuzz_0));
        debug_assert_eq!(result, TimeZone::utc());
             }
}
}
}    }
    #[test]
    fn test_current_zone_with_valid_tz_string() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let result = current_zone(Some(rug_fuzz_0));
        debug_assert!(
            result != TimeZone::utc(),
            "Expected a timezone different from UTC for a valid TZ string."
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_524 {
    use super::*;
    use crate::*;
    use crate::offset::Local;
    use crate::DateTime;
    #[test]
    fn test_now() {
        let _rug_st_tests_llm_16_524_rrrruuuugggg_test_now = 0;
        let time1: DateTime<Local> = Local::now();
        let time2: DateTime<Local> = Local::now();
        debug_assert!(time1 <= time2);
        let _rug_ed_tests_llm_16_524_rrrruuuugggg_test_now = 0;
    }
}
#[cfg(test)]
mod tests_rug_119 {
    use crate::{NaiveDate, NaiveDateTime, DateTime, Local, LocalResult};
    use crate::offset::local::inner::naive_to_local;
    #[test]
    fn test_naive_to_local() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, u32, u32, u32, u32, u32, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let mut p1 = rug_fuzz_6;
        naive_to_local(&p0, p1);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_120 {
    use super::*;
    use crate::TimeZone;
    #[test]
    fn test_fallback_timezone() {
        let _rug_st_tests_rug_120_rrrruuuugggg_test_fallback_timezone = 0;
        let result = fallback_timezone();
        debug_assert!(result.is_some());
        let _rug_ed_tests_rug_120_rrrruuuugggg_test_fallback_timezone = 0;
    }
}
#[cfg(test)]
mod tests_rug_121 {
    use super::*;
    use std::default::Default;
    use std::env;
    use crate::offset::local::inner::Cache;
    use std::time::SystemTime;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_121_rrrruuuugggg_test_rug = 0;
        let cache = <Cache as Default>::default();
        debug_assert!(cache.last_checked <= SystemTime::now());
        let _rug_ed_tests_rug_121_rrrruuuugggg_test_rug = 0;
    }
}
