use crate::date::*;
use crate::expense::*;

use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

use serde::{Serialize, Deserialize};

#[derive(Debug)]
struct DataError(String);

impl fmt::Display for DataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for DataError {}

#[derive(Serialize, Deserialize)]
pub struct Datafile {
    pub version: u64,
    pub tags: HashSet<String>,
    pub entries: Vec<Expense>,
}

impl Datafile {
    fn new() -> Datafile {
        Datafile {
            version: 1,
            tags: HashSet::new(),
            entries: vec!(),
        }
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Datafile, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = std::io::BufReader::new(file);

        let d: Datafile = serde_json::from_reader(reader)?;

        if d.version != 1 {
            return Err(Box::new(DataError("unknown version in datafile!".into())));
        }

        Ok(d)
    }

    pub fn add_tag(&mut self, tag: String) {
        self.tags.insert(tag);
    }

    pub fn insert(&mut self, expense: Expense) {
        let mut insert_idx = 0;
        for (idx, saved) in self.entries.iter().enumerate() {
            match saved.compare_dates(&expense) {
                std::cmp::Ordering::Greater => { insert_idx = idx; break; },
                std::cmp::Ordering::Less    => { insert_idx = idx + 1; },
                std::cmp::Ordering::Equal   => { insert_idx = idx + 1; },
            }
        }

        if insert_idx > self.entries.len() {
            self.entries.push(expense);
        } else {
            self.entries.insert(insert_idx, expense);
        }
    }

    pub fn remove(&mut self, id: u64) -> Result<(), Box<dyn Error>> {
        let mut rm_idx = 0;
        for (idx, saved) in self.entries.iter().enumerate() {
            if saved.compare_id(id) {
                rm_idx = idx;
                break;
            }
        }

        if rm_idx > self.entries.len() {
            return Err(Box::new(DataError("couldn't find item".into())));
        }

        self.entries.remove(rm_idx);
        Ok(())
    }

    pub fn find(&self, id: u64) -> Option<&Expense> {
        for expense in &self.entries {
            if expense.compare_id(id) {
                return Some(expense);
            }
        }

        None
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn Error>> {
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(path)?;
        let writer = std::io::BufWriter::new(file);

        serde_json::to_writer(writer, &self)?;

        Ok(())
    }

    // TODO make this faster
    pub fn expenses_between(&self, start: &SimpleDate, end: &SimpleDate) -> &[Expense] {
        let mut start_idx = 0;
        for (idx, expense) in self.entries.iter().enumerate() {
            if let Some(end_date) = expense.get_end_date() {
                if end_date > start {
                    start_idx = idx;
                    break;
                }
            } else {
                start_idx = idx;
                break;
            }
        }

        let mut end_idx = self.entries.len();
        for (idx, expense) in self.entries[start_idx..].iter().enumerate() {
            if expense.get_start_date() > end {
                end_idx = idx + start_idx;
                break;
            }
        }

        &self.entries[start_idx..end_idx]
    }
}

pub fn initialise<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    let mut file = OpenOptions::new().write(true)
        .create_new(true)
        .open(path)?;
    let contents = serde_json::to_string(&Datafile::new())?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_single() {
        let mut datafile = Datafile::new();
        let expense = Expense::new(0, "test".into(), 100, SimpleDate::from_ymd(2020, 10, 14), None, None, vec!());

        datafile.insert(expense);

        assert_eq!(datafile.entries.len(), 1);
    }

    #[test]
    fn insert_sorted() {
        let mut datafile = Datafile::new();
        let expense1 = Expense::new(0, "test".into(), 100, SimpleDate::from_ymd(2020, 10, 14), None, None, vec!());
        let expense2 = Expense::new(1, "test".into(), 101, SimpleDate::from_ymd(2020, 10, 15), None, None, vec!());

        datafile.insert(expense2);
        datafile.insert(expense1);

        assert_eq!(datafile.entries.len(), 2);
        assert_eq!(datafile.entries[0].amount(), 100);
        assert_eq!(datafile.entries[1].amount(), 101);
    }

    #[test]
    fn remove() {
        let mut datafile = Datafile::new();
        let expense1 = Expense::new(0, "test".into(), 100, SimpleDate::from_ymd(2020, 10, 14), None, None, vec!());
        let expense2 = Expense::new(1, "test".into(), 101, SimpleDate::from_ymd(2020, 10, 15), None, None, vec!());

        datafile.insert(expense2);
        datafile.insert(expense1);

        assert_eq!(datafile.entries.len(), 2);

        assert!(datafile.remove(1).is_ok());

        assert_eq!(datafile.entries.len(), 1);
        assert_eq!(datafile.entries[0].amount(), 100);
    }

    #[test]
    fn find() {
        let mut datafile = Datafile::new();
        let expense1 = Expense::new(0, "test".into(), 100, SimpleDate::from_ymd(2020, 10, 14), None, None, vec!());
        let expense2 = Expense::new(1, "test".into(), 101, SimpleDate::from_ymd(2020, 10, 15), None, None, vec!());

        datafile.insert(expense2);
        datafile.insert(expense1);

        assert!(datafile.find(9999).is_none());

        assert!(datafile.find(1).is_some());
        assert_eq!(datafile.find(1).unwrap().amount(), 101);
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2908() {
    rusty_monitor::set_test_id(2908);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Wednesday;
    let mut u64_0: u64 = 3292u64;
    let mut u64_1: u64 = 6896u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_1, weekid: u64_0, day: weekday_0};
    let mut u64_2: u64 = 9077u64;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut vec_0: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut u64_3: u64 = 2724u64;
    let mut vec_1: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut u64_4: u64 = 7765u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_4};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_5: u64 = 2577u64;
    let mut u64_6: u64 = 854u64;
    let mut u64_7: u64 = 8219u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_7, u64_6, u64_5);
    let mut i64_0: i64 = 2173i64;
    let mut str_0: &str = "4wm6W";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_8: u64 = 8568u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_8, string_0, i64_0, simpledate_0, option_1, option_0, vec_1);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut option_2: &std::option::Option<crate::date::SimpleDate> = crate::expense::Expense::get_end_date(expense_0_ref_0);
    let mut weekday_1: date::Weekday = crate::date::Weekday::Friday;
    let mut option_3: std::option::Option<&crate::expense::Expense> = crate::data::Datafile::find(datafile_0_ref_0, u64_2);
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4035() {
    rusty_monitor::set_test_id(4035);
    let mut str_0: &str = "egScT5A";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut u64_0: u64 = 5292u64;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut vec_1: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_1: u64 = 2952u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_1, on: vec_1};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Week(weekdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_2: u64 = 6081u64;
    let mut u64_3: u64 = 2698u64;
    let mut u64_4: u64 = 2406u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_4, month: u64_3, day: u64_2};
    let mut i64_0: i64 = 2166i64;
    let mut str_1: &str = "VoZqWhm50WsZ";
    let mut string_1: std::string::String = std::string::String::from(str_1);
    let mut u64_5: u64 = 9046u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_5, string_1, i64_0, simpledate_0, option_1, option_0, vec_0);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut vec_2: &std::vec::Vec<std::string::String> = crate::expense::Expense::tags(expense_0_ref_0);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Thursday;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_0);
    let mut weekday_1: date::Weekday = crate::date::Weekday::Monday;
    let mut weekday_2: date::Weekday = crate::date::Weekday::Saturday;
    crate::data::Datafile::add_tag(datafile_0_ref_0, string_0);
    let mut weekday_3: date::Weekday = crate::date::Weekday::Thursday;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2553() {
    rusty_monitor::set_test_id(2553);
    let mut str_0: &str = "o";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_0: u64 = 757u64;
    let mut u64_1: u64 = 4658u64;
    let mut u64_2: u64 = 2019u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_2, u64_1, u64_0);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut vec_1: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_3: u64 = 9015u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_3, on: vec_1};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Week(weekdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_4: u64 = 4171u64;
    let mut duration_0: date::Duration = crate::date::Duration::Year(u64_4);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_5: u64 = 7092u64;
    let mut u64_6: u64 = 5384u64;
    let mut u64_7: u64 = 3510u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_7, u64_6, u64_5);
    let mut i64_0: i64 = 1413i64;
    let mut str_1: &str = "";
    let mut string_1: std::string::String = std::string::String::from(str_1);
    let mut u64_8: u64 = 6883u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_8, string_1, i64_0, simpledate_1, option_1, option_0, vec_0);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut u64_9: u64 = 7836u64;
    let mut u64_10: u64 = 8279u64;
    let mut u64_11: u64 = 7786u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_11, month: u64_10, day: u64_9};
    let mut str_2: &str = crate::expense::Expense::description(expense_0_ref_0);
    crate::data::Datafile::add_tag(datafile_0_ref_0, string_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2481() {
    rusty_monitor::set_test_id(2481);
    let mut u64_0: u64 = 5384u64;
    let mut u64_1: u64 = 3742u64;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut vec_1: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_2: u64 = 175u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_2, on: vec_1};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Week(weekdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_3: u64 = 9451u64;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_3);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_4: u64 = 7873u64;
    let mut u64_5: u64 = 5507u64;
    let mut u64_6: u64 = 1735u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_6, u64_5, u64_4);
    let mut i64_0: i64 = -7416i64;
    let mut str_0: &str = "i71fDL2jgvv8Db2";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_7: u64 = 1870u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_7, string_0, i64_0, simpledate_0, option_1, option_0, vec_0);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut vec_2: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut u64_8: u64 = 3557u64;
    let mut u64_9: u64 = 4629u64;
    let mut duration_1: date::Duration = crate::date::Duration::Year(u64_9);
    crate::data::Datafile::insert(datafile_0_ref_0, expense_0);
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Count(u64_1);
    let mut repend_2: date::RepEnd = crate::date::RepEnd::Count(u64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4481() {
    rusty_monitor::set_test_id(4481);
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_0: u64 = 29u64;
    let mut u64_1: u64 = 9342u64;
    let mut u64_2: u64 = 7746u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_2, month: u64_1, day: u64_0};
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut u64_3: u64 = 1632u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_3};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_4: u64 = 8345u64;
    let mut duration_0: date::Duration = crate::date::Duration::Week(u64_4);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_5: u64 = 7546u64;
    let mut u64_6: u64 = 8848u64;
    let mut u64_7: u64 = 656u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_7, u64_6, u64_5);
    let mut i64_0: i64 = -2712i64;
    let mut str_0: &str = "lbQKbPbTPnnLx6I";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_8: u64 = 6534u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_8, string_0, i64_0, simpledate_1, option_1, option_0, vec_0);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut u64_9: u64 = 4788u64;
    let mut u64_10: u64 = 9638u64;
    let mut duration_1: date::Duration = crate::date::Duration::Day(u64_10);
    let mut duration_1_ref_0: &date::Duration = &mut duration_1;
    let mut u64_11: u64 = 3573u64;
    let mut u64_12: u64 = 7064u64;
    let mut u64_13: u64 = 6971u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_13, month: u64_12, day: u64_11};
    let mut simpledate_2_ref_0: &crate::date::SimpleDate = &mut simpledate_2;
    let mut u64_14: u64 = 9933u64;
    let mut yeardelta_1: crate::date::YearDelta = crate::date::YearDelta {nth: u64_14};
    let mut weekday_0: date::Weekday = crate::date::Weekday::Sunday;
    let mut yeardelta_2: crate::date::YearDelta = crate::date::YearDelta {nth: u64_9};
    let mut simpledate_3: &crate::date::SimpleDate = crate::expense::Expense::get_start_date(expense_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2009() {
    rusty_monitor::set_test_id(2009);
    let mut u64_0: u64 = 8638u64;
    let mut u64_1: u64 = 1334u64;
    let mut u64_2: u64 = 9856u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_2, month: u64_1, day: u64_0};
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut u64_3: u64 = 5726u64;
    let mut u64_4: u64 = 7441u64;
    let mut u64_5: u64 = 2626u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_5, month: u64_4, day: u64_3};
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut u64_6: u64 = 2128u64;
    let mut u64_7: u64 = 5508u64;
    let mut u64_8: u64 = 5204u64;
    let mut str_0: &str = "nsiswD0W9L6alGw2";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_9: u64 = 3998u64;
    let mut u64_10: u64 = 2145u64;
    let mut u64_11: u64 = 9160u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_11, month: u64_10, day: u64_9};
    let mut i64_0: i64 = -6943i64;
    let mut str_1: &str = "nzn8sb5";
    let mut string_0: std::string::String = std::string::String::from(str_1);
    let mut u64_12: u64 = 6201u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_12, string_0, i64_0, simpledate_2, option_1, option_0, vec_0);
    let mut expense_0_ref_0: &mut crate::expense::Expense = &mut expense_0;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Monday;
    crate::expense::Expense::remove_tags(expense_0_ref_0, str_0_ref_0);
    let mut simpledate_3: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_8, month: u64_7, day: u64_6};
    let mut expense_slice_0: &[crate::expense::Expense] = crate::data::Datafile::expenses_between(datafile_0_ref_0, simpledate_1_ref_0, simpledate_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1034() {
    rusty_monitor::set_test_id(1034);
    let mut u64_0: u64 = 830u64;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut u64_1: u64 = 3059u64;
    let mut u64_2: u64 = 5337u64;
    let mut u64_3: u64 = 2215u64;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Thursday;
    let mut u64_4: u64 = 5934u64;
    let mut u64_5: u64 = 31u64;
    let mut u64_6: u64 = 8261u64;
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_1_ref_0: &mut crate::data::Datafile = &mut datafile_1;
    let mut u64_7: u64 = 8505u64;
    let mut u64_8: u64 = 6271u64;
    let mut u64_9: u64 = 5383u64;
    let mut u64_10: u64 = 3911u64;
    let mut u64_11: u64 = 9273u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_11, month: u64_10, day: u64_9};
    let mut weekday_1: date::Weekday = crate::date::Weekday::Thursday;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_8);
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_7};
    let mut result_0: std::result::Result<(), std::boxed::Box<dyn std::error::Error>> = crate::data::Datafile::remove(datafile_1_ref_0, u64_6);
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_5, weekid: u64_4, day: weekday_0};
    let mut tuple_0: () = std::result::Result::unwrap(result_0);
    let mut yeardelta_1: crate::date::YearDelta = crate::date::YearDelta {nth: u64_3};
    let mut weekday_2: date::Weekday = crate::date::Weekday::Sunday;
    let mut monthdeltaweek_0_ref_0: &crate::date::MonthDeltaWeek = &mut monthdeltaweek_0;
    let mut weekday_3: date::Weekday = crate::date::Weekday::Friday;
    let mut weekday_4: date::Weekday = crate::date::Weekday::Monday;
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Count(u64_1);
    let mut option_0: std::option::Option<&crate::expense::Expense> = crate::data::Datafile::find(datafile_0_ref_0, u64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1846() {
    rusty_monitor::set_test_id(1846);
    let mut u64_0: u64 = 5774u64;
    let mut u64_1: u64 = 6653u64;
    let mut u64_2: u64 = 3554u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_2, u64_1, u64_0);
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut u64_3: u64 = 6483u64;
    let mut u64_4: u64 = 6549u64;
    let mut u64_5: u64 = 961u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_5, month: u64_4, day: u64_3};
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut u64_6: u64 = 7159u64;
    let mut u64_7: u64 = 2559u64;
    let mut u64_8: u64 = 5088u64;
    let mut u64_9: u64 = 9486u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_9, month: u64_8, day: u64_7};
    let mut simpledate_2_ref_0: &crate::date::SimpleDate = &mut simpledate_2;
    let mut u64_10: u64 = 7111u64;
    let mut u64_11: u64 = 3213u64;
    let mut u64_12: u64 = 167u64;
    let mut simpledate_3: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_12, month: u64_11, day: u64_10};
    let mut simpledate_3_ref_0: &crate::date::SimpleDate = &mut simpledate_3;
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_1_ref_0: &crate::data::Datafile = &mut datafile_1;
    let mut expense_slice_0: &[crate::expense::Expense] = crate::data::Datafile::expenses_between(datafile_1_ref_0, simpledate_3_ref_0, simpledate_2_ref_0);
    let mut datafile_2: crate::data::Datafile = crate::data::Datafile::new();
    let mut duration_0: date::Duration = crate::date::Duration::Week(u64_6);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Thursday;
    let mut expense_slice_1: &[crate::expense::Expense] = crate::data::Datafile::expenses_between(datafile_0_ref_0, simpledate_1_ref_0, simpledate_0_ref_0);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    panic!("From RustyUnit with love");
}
}