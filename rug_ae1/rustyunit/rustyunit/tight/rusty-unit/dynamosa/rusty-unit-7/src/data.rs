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
fn rusty_test_7561() {
    rusty_monitor::set_test_id(7561);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut u64_0: u64 = 7695u64;
    let mut u64_1: u64 = 6591u64;
    let mut u64_2: u64 = 1659u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_2, month: u64_1, day: u64_0};
    let mut str_0: &str = "RtMOsmGyGc22LdTw0";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "dhoEOIMrSZlje";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut vec_0: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut weekday_0: date::Weekday = crate::date::Weekday::Sunday;
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4364() {
    rusty_monitor::set_test_id(4364);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut vec_0: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut u64_0: u64 = 8085u64;
    let mut str_0: &str = "VWV3ejFfxxov7Ejzs";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut vec_1: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut vec_2: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_1: u64 = 7499u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_1, on: vec_2};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Week(weekdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_2: u64 = 3196u64;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_2);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_3: u64 = 3838u64;
    let mut u64_4: u64 = 8296u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_0, month: u64_4, day: u64_3};
    let mut i64_0: i64 = 5274i64;
    let mut str_1: &str = "fzm";
    let mut string_0: std::string::String = std::string::String::from(str_1);
    let mut u64_5: u64 = 68u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_5, string_0, i64_0, simpledate_0, option_1, option_0, vec_1);
    let mut expense_0_ref_0: &mut crate::expense::Expense = &mut expense_0;
    crate::expense::Expense::remove_tags(expense_0_ref_0, str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8456() {
    rusty_monitor::set_test_id(8456);
    let mut str_0: &str = "E9VMnbEl";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut vec_1: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_0: u64 = 4500u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_0, on: vec_1};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Week(weekdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_1: u64 = 7478u64;
    let mut u64_2: u64 = 4087u64;
    let mut u64_3: u64 = 3977u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_3, u64_2, u64_1);
    let mut i64_0: i64 = -2558i64;
    let mut str_1: &str = "Wb1YNk8u6d2Fs8zB";
    let mut string_0: std::string::String = std::string::String::from(str_1);
    let mut u64_4: u64 = 5532u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_4, string_0, i64_0, simpledate_0, option_1, option_0, vec_0);
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_1_ref_0: &mut crate::data::Datafile = &mut datafile_1;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1505() {
    rusty_monitor::set_test_id(1505);
    let mut u64_0: u64 = 519u64;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_1_ref_0: &crate::data::Datafile = &mut datafile_1;
    let mut u64_1: u64 = 2024u64;
    let mut u64_2: u64 = 5830u64;
    let mut u64_3: u64 = 9401u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_3, month: u64_2, day: u64_1};
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut u64_4: u64 = 5450u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_4};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_5: u64 = 1700u64;
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_5);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_6: u64 = 9721u64;
    let mut u64_7: u64 = 6415u64;
    let mut u64_8: u64 = 1871u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_8, u64_7, u64_6);
    let mut u64_9: u64 = 5762u64;
    let mut datafile_2: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_2_ref_0: &mut crate::data::Datafile = &mut datafile_2;
    let mut datafile_3: crate::data::Datafile = crate::data::Datafile::new();
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_10: u64 = 4743u64;
    let mut duration_1: date::Duration = crate::date::Duration::Day(u64_10);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
    let mut u64_11: u64 = 9421u64;
    let mut u64_12: u64 = 6129u64;
    let mut u64_13: u64 = 1918u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_13, u64_12, u64_11);
    let mut i64_0: i64 = 20364i64;
    let mut str_0: &str = "u";
    let mut u64_14: u64 = 4062u64;
    let mut u64_15: u64 = 5890u64;
    let mut u64_16: u64 = 8002u64;
    let mut simpledate_3: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_16, month: u64_15, day: u64_14};
    let mut simpledate_3_ref_0: &crate::date::SimpleDate = &mut simpledate_3;
    let mut u64_17: u64 = 6108u64;
    let mut u64_18: u64 = 7164u64;
    let mut u64_19: u64 = 4432u64;
    let mut simpledate_4: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_19, month: u64_18, day: u64_17};
    let mut simpledate_4_ref_0: &crate::date::SimpleDate = &mut simpledate_4;
    let mut u64_20: u64 = 6680u64;
    let mut u64_21: u64 = 8840u64;
    let mut u64_22: u64 = 7161u64;
    let mut simpledate_5: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_22, month: u64_21, day: u64_20};
    let mut simpledate_5_ref_0: &crate::date::SimpleDate = &mut simpledate_5;
    let mut u64_23: u64 = 1947u64;
    let mut u64_24: u64 = 2903u64;
    let mut u64_25: u64 = 9914u64;
    let mut simpledate_6: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_25, month: u64_24, day: u64_23};
    let mut u64_26: u64 = 7732u64;
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_26, string_0, i64_0, simpledate_2, option_3, option_2, vec_0);
    let mut datafile_4: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_3_ref_0: &mut crate::data::Datafile = &mut datafile_3;
    let mut option_4: std::option::Option<&crate::expense::Expense> = crate::data::Datafile::find(datafile_1_ref_0, u64_9);
    let mut option_5: std::option::Option<&crate::expense::Expense> = crate::data::Datafile::find(datafile_0_ref_0, u64_0);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut simpledate_7: &crate::date::SimpleDate = crate::expense::Expense::get_start_date(expense_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7434() {
    rusty_monitor::set_test_id(7434);
    let mut u64_0: u64 = 2415u64;
    let mut u64_1: u64 = 7856u64;
    let mut u64_2: u64 = 8320u64;
    let mut u64_3: u64 = 3541u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_3, u64_2, u64_1);
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut u64_4: u64 = 650u64;
    let mut u64_5: u64 = 8479u64;
    let mut u64_6: u64 = 4220u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_6, u64_5, u64_4);
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut option_0: std::option::Option<date::Duration> = std::option::Option::None;
    let mut vec_0: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut weekday_0: date::Weekday = crate::date::Weekday::Sunday;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut weekday_1: date::Weekday = crate::date::Weekday::Friday;
    let mut weekday_2: date::Weekday = crate::date::Weekday::Saturday;
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut expense_slice_0: &[crate::expense::Expense] = crate::data::Datafile::expenses_between(datafile_0_ref_0, simpledate_1_ref_0, simpledate_0_ref_0);
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_150() {
    rusty_monitor::set_test_id(150);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut str_0: &str = "8XlWIUeOzO";
    let mut u64_0: u64 = 1592u64;
    let mut duration_0: date::Duration = crate::date::Duration::Year(u64_0);
    let mut u64_1: u64 = 8466u64;
    let mut u64_2: u64 = 5187u64;
    let mut u64_3: u64 = 5941u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_3, month: u64_2, day: u64_1};
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut vec_0: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_4: u64 = 6338u64;
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_4, days: vec_0};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnDate(monthdeltadate_0);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut u64_5: u64 = 8179u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_5};
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut vec_1: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_6: u64 = 246u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_6, on: vec_1};
    let mut weekday_0: date::Weekday = crate::date::Weekday::Friday;
    let mut u64_7: u64 = 6576u64;
    let mut u64_8: u64 = 6942u64;
    let mut u64_9: u64 = 1002u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_8, weekid: u64_7, day: weekday_0};
    let mut u64_10: u64 = 491u64;
    let mut vec_2: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_11: u64 = 3057u64;
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Count(u64_9);
    let mut vec_3: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_12: u64 = 3400u64;
    let mut weekdelta_1: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_10, on: vec_3};
    let mut repdelta_2: date::RepDelta = crate::date::RepDelta::Week(weekdelta_0);
    let mut repetition_1: crate::date::Repetition = crate::date::Repetition {delta: repdelta_1, end: repend_1};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_13: u64 = 6765u64;
    let mut duration_1: date::Duration = crate::date::Duration::Year(u64_12);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_14: u64 = 9505u64;
    let mut u64_15: u64 = 8624u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_11, u64_15, u64_14);
    let mut i64_0: i64 = -1113i64;
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_13, string_0, i64_0, simpledate_1, option_1, option_0, vec_2);
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut vec_4: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    crate::data::Datafile::insert(datafile_0_ref_0, expense_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4693() {
    rusty_monitor::set_test_id(4693);
    let mut u64_0: u64 = 5072u64;
    let mut u64_1: u64 = 6804u64;
    let mut u64_2: u64 = 464u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_2, u64_1, u64_0);
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut u64_3: u64 = 2051u64;
    let mut u64_4: u64 = 1148u64;
    let mut u64_5: u64 = 4042u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_5, month: u64_4, day: u64_3};
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_6: u64 = 7872u64;
    let mut u64_7: u64 = 7628u64;
    let mut u64_8: u64 = 9271u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_8, month: u64_7, day: u64_6};
    let mut i64_0: i64 = -6270i64;
    let mut str_0: &str = "ZbrWql";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_9: u64 = 8u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_9, string_0, i64_0, simpledate_2, option_1, option_0, vec_0);
    let mut vec_1: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_10: u64 = 4255u64;
    let mut str_1: &str = "TMoiSoWFw0YxuH9lj5";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut vec_2: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_11: u64 = 3398u64;
    let mut u64_12: u64 = 128u64;
    let mut u64_13: u64 = 7667u64;
    let mut simpledate_3: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_13, u64_12, u64_11);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_3);
    let mut u64_14: u64 = 130u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_14};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_15: u64 = 737u64;
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_15);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_16: u64 = 7130u64;
    let mut u64_17: u64 = 9588u64;
    let mut u64_18: u64 = 5312u64;
    let mut simpledate_4: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_18, u64_17, u64_16);
    let mut i64_1: i64 = -9766i64;
    let mut str_2: &str = "2";
    let mut string_1: std::string::String = std::string::String::from(str_2);
    let mut u64_19: u64 = 479u64;
    let mut expense_1: crate::expense::Expense = crate::expense::Expense::new(u64_19, string_1, i64_1, simpledate_4, option_3, option_2, vec_2);
    let mut expense_0_ref_0: &mut crate::expense::Expense = &mut expense_0;
    crate::expense::Expense::remove_tags(expense_0_ref_0, str_1_ref_0);
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_10, days: vec_1};
    let mut expense_slice_0: &[crate::expense::Expense] = crate::data::Datafile::expenses_between(datafile_0_ref_0, simpledate_1_ref_0, simpledate_0_ref_0);
    let mut expense_1_ref_0: &crate::expense::Expense = &mut expense_1;
    let mut str_3: &str = crate::expense::Expense::description(expense_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6675() {
    rusty_monitor::set_test_id(6675);
    let mut u64_0: u64 = 228u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_0);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Friday;
    let mut u64_1: u64 = 7405u64;
    let mut u64_2: u64 = 1698u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_2, weekid: u64_1, day: weekday_0};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut u64_3: u64 = 149u64;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut vec_0: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut str_0: &str = "VWV3ejFfxxov7Ejzs";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut vec_1: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Never;
    let mut vec_2: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_4: u64 = 7499u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_4, on: vec_2};
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Week(weekdelta_0);
    let mut repetition_1: crate::date::Repetition = crate::date::Repetition {delta: repdelta_1, end: repend_1};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_5: u64 = 3196u64;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_5);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_6: u64 = 3838u64;
    let mut u64_7: u64 = 8296u64;
    let mut u64_8: u64 = 2522u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_8, month: u64_7, day: u64_6};
    let mut i64_0: i64 = 5274i64;
    let mut str_1: &str = "fzm";
    let mut string_0: std::string::String = std::string::String::from(str_1);
    let mut u64_9: u64 = 68u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_9, string_0, i64_0, simpledate_0, option_1, option_0, vec_1);
    let mut expense_0_ref_0: &mut crate::expense::Expense = &mut expense_0;
    crate::expense::Expense::remove_tags(expense_0_ref_0, str_0_ref_0);
    let mut option_2: std::option::Option<&crate::expense::Expense> = crate::data::Datafile::find(datafile_0_ref_0, u64_3);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7010() {
    rusty_monitor::set_test_id(7010);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Tuesday;
    let mut u64_0: u64 = 491u64;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_1: u64 = 3057u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_1);
    let mut vec_1: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_2: u64 = 3400u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_2, on: vec_1};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Week(weekdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_3: u64 = 6765u64;
    let mut duration_0: date::Duration = crate::date::Duration::Year(u64_3);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_4: u64 = 9505u64;
    let mut u64_5: u64 = 8624u64;
    let mut u64_6: u64 = 9682u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_6, u64_5, u64_4);
    let mut i64_0: i64 = -1113i64;
    let mut str_0: &str = "I2uQIY3";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_7: u64 = 8488u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_7, string_0, i64_0, simpledate_0, option_1, option_0, vec_0);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut vec_2: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    crate::data::Datafile::insert(datafile_0_ref_0, expense_0);
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5438() {
    rusty_monitor::set_test_id(5438);
    let mut u64_0: u64 = 8706u64;
    let mut u64_1: u64 = 3604u64;
    let mut u64_2: u64 = 270u64;
    let mut u64_3: u64 = 5792u64;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_4: u64 = 7478u64;
    let mut u64_5: u64 = 4087u64;
    let mut u64_6: u64 = 3977u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_6, u64_5, u64_4);
    let mut str_0: &str = "Wb1YNk8u6d2Fs8zB";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_1_ref_0: &mut crate::data::Datafile = &mut datafile_1;
    let mut option_1: std::option::Option<&crate::expense::Expense> = crate::data::Datafile::find(datafile_0_ref_0, u64_3);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Thursday;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_2, u64_1, u64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3672() {
    rusty_monitor::set_test_id(3672);
    let mut str_0: &str = "1tHUW8a1";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut vec_0: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut u64_0: u64 = 6108u64;
    let mut u64_1: u64 = 7164u64;
    let mut u64_2: u64 = 4432u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_2, month: u64_1, day: u64_0};
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut u64_3: u64 = 6680u64;
    let mut u64_4: u64 = 8840u64;
    let mut u64_5: u64 = 7161u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_5, month: u64_4, day: u64_3};
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut u64_6: u64 = 1947u64;
    let mut u64_7: u64 = 2903u64;
    let mut u64_8: u64 = 9914u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_8, month: u64_7, day: u64_6};
    let mut simpledate_2_ref_0: &crate::date::SimpleDate = &mut simpledate_2;
    let mut u64_9: u64 = 7732u64;
    let mut u64_10: u64 = 6993u64;
    let mut u64_11: u64 = 673u64;
    let mut simpledate_3: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_11, u64_10, u64_9);
    let mut u64_12: u64 = 9479u64;
    let mut u64_13: u64 = 8204u64;
    let mut u64_14: u64 = 183u64;
    let mut simpledate_4: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_12, month: u64_14, day: u64_13};
    crate::data::Datafile::add_tag(datafile_0_ref_0, string_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8280() {
    rusty_monitor::set_test_id(8280);
    let mut u64_0: u64 = 5139u64;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut vec_0: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut u64_1: u64 = 419u64;
    let mut u64_2: u64 = 5570u64;
    let mut u64_3: u64 = 6062u64;
    let mut u64_4: u64 = 4564u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_4};
    let mut vec_1: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_5: u64 = 3161u64;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Thursday;
    let mut u64_6: u64 = 2375u64;
    let mut u64_7: u64 = 1805u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_7, weekid: u64_6, day: weekday_0};
    let mut vec_2: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_8: u64 = 6954u64;
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_8, days: vec_2};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    let mut weekday_1: date::Weekday = crate::date::Weekday::Monday;
    let mut monthdeltadate_1: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_5, days: vec_1};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_3, month: u64_2, day: u64_1};
    let mut result_0: std::result::Result<(), std::boxed::Box<dyn std::error::Error>> = crate::data::Datafile::remove(datafile_0_ref_0, u64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3191() {
    rusty_monitor::set_test_id(3191);
    let mut str_0: &str = "C";
    let mut u64_0: u64 = 920u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_0);
    let mut u64_1: u64 = 9428u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_1};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut u64_2: u64 = 8706u64;
    let mut u64_3: u64 = 3604u64;
    let mut u64_4: u64 = 270u64;
    let mut u64_5: u64 = 5792u64;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Never;
    let mut vec_1: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_6: u64 = 4500u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_6, on: vec_1};
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Week(weekdelta_0);
    let mut repetition_1: crate::date::Repetition = crate::date::Repetition {delta: repdelta_1, end: repend_1};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_7: u64 = 7478u64;
    let mut u64_8: u64 = 4087u64;
    let mut u64_9: u64 = 3977u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_9, u64_8, u64_7);
    let mut i64_0: i64 = -2558i64;
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_10: u64 = 5532u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_10, string_0, i64_0, simpledate_0, option_1, option_0, vec_0);
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_1_ref_0: &mut crate::data::Datafile = &mut datafile_1;
    let mut repend_2: date::RepEnd = crate::date::RepEnd::Never;
    crate::data::Datafile::insert(datafile_1_ref_0, expense_0);
    let mut option_2: std::option::Option<&crate::expense::Expense> = crate::data::Datafile::find(datafile_0_ref_0, u64_5);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Thursday;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_4, u64_3, u64_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7737() {
    rusty_monitor::set_test_id(7737);
    let mut u64_0: u64 = 8706u64;
    let mut u64_1: u64 = 3604u64;
    let mut u64_2: u64 = 270u64;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut vec_1: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_3: u64 = 4500u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_3, on: vec_1};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Week(weekdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_4: u64 = 7478u64;
    let mut u64_5: u64 = 4087u64;
    let mut u64_6: u64 = 3977u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_6, u64_5, u64_4);
    let mut i64_0: i64 = -2558i64;
    let mut str_0: &str = "Wb1YNk8u6d2Fs8zB";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_7: u64 = 5532u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_7, string_0, i64_0, simpledate_0, option_1, option_0, vec_0);
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_1_ref_0: &mut crate::data::Datafile = &mut datafile_1;
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Never;
    crate::data::Datafile::insert(datafile_1_ref_0, expense_0);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Thursday;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_2, u64_1, u64_0);
    panic!("From RustyUnit with love");
}
}