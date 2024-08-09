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
fn rusty_test_8119() {
    rusty_monitor::set_test_id(8119);
    let mut str_0: &str = "T0I";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_0: u64 = 3666u64;
    let mut duration_0: date::Duration = crate::date::Duration::Week(u64_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_1: u64 = 4368u64;
    let mut u64_2: u64 = 1546u64;
    let mut u64_3: u64 = 7371u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_3, month: u64_2, day: u64_1};
    let mut i64_0: i64 = -4129i64;
    let mut str_1: &str = "jA0F";
    let mut string_0: std::string::String = std::string::String::from(str_1);
    let mut u64_4: u64 = 9726u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_4, string_0, i64_0, simpledate_0, option_1, option_0, vec_0);
    let mut expense_0_ref_0: &mut crate::expense::Expense = &mut expense_0;
    let mut u64_5: u64 = 6868u64;
    let mut u64_6: u64 = 8449u64;
    let mut u64_7: u64 = 2360u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_7, month: u64_6, day: u64_5};
    let mut vec_1: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_8: u64 = 8870u64;
    let mut u64_9: u64 = 8288u64;
    let mut u64_10: u64 = 7666u64;
    let mut u64_11: u64 = 6972u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_11, month: u64_10, day: u64_9};
    let mut simpledate_2_ref_0: &crate::date::SimpleDate = &mut simpledate_2;
    let mut str_2: &str = "CSogO2mCNAf";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "GHRyeJiTAxseJJrF";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut u64_12: u64 = 1737u64;
    let mut u64_13: u64 = 6097u64;
    let mut u64_14: u64 = 4793u64;
    let mut simpledate_3: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_14, u64_13, u64_12);
    let mut simpledate_3_ref_0: &crate::date::SimpleDate = &mut simpledate_3;
    let mut str_4: &str = "7";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "teABPDxRy3NpK0p";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut u64_15: u64 = 8738u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_8, on: vec_1};
    let mut weekday_0: date::Weekday = crate::date::Weekday::Tuesday;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Never;
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Week(weekdelta_0);
    let mut duration_1: date::Duration = crate::date::Duration::Month(u64_15);
    let mut repend_2: date::RepEnd = crate::date::RepEnd::Date(simpledate_1);
    crate::expense::Expense::remove_tags(expense_0_ref_0, str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1012() {
    rusty_monitor::set_test_id(1012);
    let mut u64_0: u64 = 6850u64;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut str_0: &str = "iLPX3V6Bxu";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_1_ref_0: &mut crate::data::Datafile = &mut datafile_1;
    let mut u64_1: u64 = 8397u64;
    let mut u64_2: u64 = 999u64;
    let mut u64_3: u64 = 3489u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_3, u64_2, u64_1);
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut u64_4: u64 = 3568u64;
    let mut u64_5: u64 = 2147u64;
    let mut u64_6: u64 = 9490u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_6, month: u64_5, day: u64_4};
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut u64_7: u64 = 4506u64;
    let mut vec_0: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_8: u64 = 8738u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_8, on: vec_0};
    let mut weekday_0: date::Weekday = crate::date::Weekday::Tuesday;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Never;
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Week(weekdelta_0);
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_7);
    crate::data::Datafile::add_tag(datafile_1_ref_0, string_0);
    let mut option_0: std::option::Option<&crate::expense::Expense> = crate::data::Datafile::find(datafile_0_ref_0, u64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2840() {
    rusty_monitor::set_test_id(2840);
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut vec_1: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_0: u64 = 1725u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_0, on: vec_1};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Week(weekdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_1: u64 = 7152u64;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_1);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_2: u64 = 4549u64;
    let mut u64_3: u64 = 2392u64;
    let mut u64_4: u64 = 4569u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_4, u64_3, u64_2);
    let mut i64_0: i64 = -283i64;
    let mut str_0: &str = "hwWNet9dHsb";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_5: u64 = 4885u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_5, string_0, i64_0, simpledate_0, option_3, option_2, vec_0);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    crate::data::Datafile::insert(datafile_0_ref_0, expense_0);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Thursday;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_245() {
    rusty_monitor::set_test_id(245);
    let mut str_0: &str = "A8uDk";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_0: u64 = 5072u64;
    let mut u64_1: u64 = 7340u64;
    let mut u64_2: u64 = 4076u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_2, u64_1, u64_0);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut vec_0: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_3: u64 = 1725u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_3, on: vec_0};
    let mut u64_4: u64 = 4549u64;
    let mut u64_5: u64 = 2392u64;
    let mut u64_6: u64 = 4569u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_6, u64_5, u64_4);
    let mut str_1: &str = "hwWNet9dHsb";
    let mut string_0: std::string::String = std::string::String::from(str_1);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_43() {
    rusty_monitor::set_test_id(43);
    let mut u64_0: u64 = 8308u64;
    let mut u64_1: u64 = 217u64;
    let mut u64_2: u64 = 9839u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_2, month: u64_1, day: u64_0};
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut u64_3: u64 = 9714u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_3};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_4: u64 = 3881u64;
    let mut duration_0: date::Duration = crate::date::Duration::Week(u64_4);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_5: u64 = 7265u64;
    let mut u64_6: u64 = 566u64;
    let mut u64_7: u64 = 6355u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_7, u64_6, u64_5);
    let mut i64_0: i64 = -7292i64;
    let mut u64_8: u64 = 5724u64;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_9: u64 = 8009u64;
    let mut u64_10: u64 = 1060u64;
    let mut u64_11: u64 = 4765u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_11, month: u64_10, day: u64_9};
    let mut i64_1: i64 = 11872i64;
    let mut str_0: &str = "9";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_12: u64 = 4948u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_12, string_0, i64_1, simpledate_2, option_3, option_2, vec_0);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut str_1: &str = crate::expense::Expense::description(expense_0_ref_0);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6950() {
    rusty_monitor::set_test_id(6950);
    let mut u64_0: u64 = 3268u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_0);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Friday;
    let mut u64_1: u64 = 8175u64;
    let mut u64_2: u64 = 765u64;
    let mut u64_3: u64 = 9765u64;
    let mut duration_0: date::Duration = crate::date::Duration::Year(u64_3);
    let mut option_0: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_4: u64 = 6492u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_2, month: u64_1, day: u64_4};
    let mut u64_5: u64 = 5117u64;
    let mut u64_6: u64 = 5534u64;
    let mut u64_7: u64 = 5056u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_7, u64_6, u64_5);
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut u64_8: u64 = 144u64;
    let mut u64_9: u64 = 8863u64;
    let mut u64_10: u64 = 7330u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_10, month: u64_9, day: u64_8};
    let mut simpledate_2_ref_0: &crate::date::SimpleDate = &mut simpledate_2;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut u64_11: u64 = 4802u64;
    let mut u64_12: u64 = 3105u64;
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_1_ref_0: &mut crate::data::Datafile = &mut datafile_1;
    let mut u64_13: u64 = 4730u64;
    let mut datafile_2: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_2_ref_0: &crate::data::Datafile = &mut datafile_2;
    let mut option_1: std::option::Option<&crate::expense::Expense> = crate::data::Datafile::find(datafile_2_ref_0, u64_13);
    let mut result_0: std::result::Result<(), std::boxed::Box<dyn std::error::Error>> = crate::data::Datafile::remove(datafile_1_ref_0, u64_12);
    let mut weekday_1: date::Weekday = crate::date::Weekday::Saturday;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_11};
    let mut expense_0: &crate::expense::Expense = std::option::Option::unwrap(option_1);
    let mut expense_slice_0: &[crate::expense::Expense] = crate::data::Datafile::expenses_between(datafile_0_ref_0, simpledate_2_ref_0, simpledate_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7889() {
    rusty_monitor::set_test_id(7889);
    let mut u64_0: u64 = 3360u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_0};
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_1: u64 = 1790u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_1);
    let mut vec_1: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_2: u64 = 1322u64;
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_2, days: vec_1};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnDate(monthdeltadate_0);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_3: u64 = 1869u64;
    let mut duration_0: date::Duration = crate::date::Duration::Week(u64_3);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_4: u64 = 4601u64;
    let mut u64_5: u64 = 1381u64;
    let mut u64_6: u64 = 3566u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_6, u64_5, u64_4);
    let mut i64_0: i64 = -2463i64;
    let mut str_0: &str = "N";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_7: u64 = 2336u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_7, string_0, i64_0, simpledate_0, option_1, option_0, vec_0);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut simpledate_1: &crate::date::SimpleDate = crate::expense::Expense::get_start_date(expense_0_ref_0);
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2601() {
    rusty_monitor::set_test_id(2601);
    let mut str_0: &str = "A8uDk";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut vec_1: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_0: u64 = 1725u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_0, on: vec_1};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Week(weekdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut u64_1: u64 = 7152u64;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_1);
    let mut option_2: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_2: u64 = 4549u64;
    let mut u64_3: u64 = 2392u64;
    let mut u64_4: u64 = 4569u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_4, u64_3, u64_2);
    let mut i64_0: i64 = -283i64;
    let mut str_1: &str = "hwWNet9dHsb";
    let mut string_0: std::string::String = std::string::String::from(str_1);
    let mut u64_5: u64 = 4885u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_5, string_0, i64_0, simpledate_0, option_2, option_0, vec_0);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    crate::data::Datafile::insert(datafile_0_ref_0, expense_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5188() {
    rusty_monitor::set_test_id(5188);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Saturday;
    let mut u64_0: u64 = 258u64;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut vec_1: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_1: u64 = 6909u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_1, on: vec_1};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Week(weekdelta_0);
    let mut option_0: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_2: u64 = 2289u64;
    let mut u64_3: u64 = 4244u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_0, u64_3, u64_2);
    let mut str_0: &str = "CxsQuxKcpET09";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7118() {
    rusty_monitor::set_test_id(7118);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Wednesday;
    let mut u64_0: u64 = 9714u64;
    let mut u64_1: u64 = 2579u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_1, weekid: u64_0, day: weekday_0};
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut weekday_1: date::Weekday = crate::date::Weekday::Tuesday;
    let mut u64_2: u64 = 9186u64;
    let mut u64_3: u64 = 6231u64;
    let mut monthdeltaweek_1: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_3, weekid: u64_2, day: weekday_1};
    let mut u64_4: u64 = 4203u64;
    let mut u64_5: u64 = 1737u64;
    let mut u64_6: u64 = 6097u64;
    let mut u64_7: u64 = 4793u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_7, u64_6, u64_5);
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut str_0: &str = "teABPDxRy3NpK0p";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u64_8: u64 = 1579u64;
    let mut u64_9: u64 = 9108u64;
    let mut u64_10: u64 = 9385u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_10, month: u64_9, day: u64_8};
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_1);
    let mut weekday_2: date::Weekday = crate::date::Weekday::Monday;
    let mut u64_11: u64 = 3564u64;
    let mut monthdeltaweek_2: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_11, weekid: u64_4, day: weekday_2};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_2);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut u64_12: u64 = 829u64;
    let mut u64_13: u64 = 348u64;
    let mut u64_14: u64 = 4799u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_14, month: u64_13, day: u64_12};
    let mut simpledate_2_ref_0: &crate::date::SimpleDate = &mut simpledate_2;
    let mut u64_15: u64 = 697u64;
    let mut u64_15_ref_0: &u64 = &mut u64_15;
    let mut str_1: &str = "gEErlO1DIyt3l";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut expense_slice_0: &[crate::expense::Expense] = crate::data::Datafile::expenses_between(datafile_0_ref_0, simpledate_2_ref_0, simpledate_0_ref_0);
    let mut monthdelta_1: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4063() {
    rusty_monitor::set_test_id(4063);
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_0: u64 = 8330u64;
    let mut u64_1: u64 = 9884u64;
    let mut u64_2: u64 = 4484u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_2, u64_1, u64_0);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut str_0: &str = "zW8T72iiG6VY";
    let mut u64_3: u64 = 2304u64;
    let mut duration_0: date::Duration = crate::date::Duration::Week(u64_3);
    let mut u64_4: u64 = 4075u64;
    let mut u64_5: u64 = 9950u64;
    let mut u64_6: u64 = 9786u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_6, u64_5, u64_4);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_1);
    let mut u64_7: u64 = 3263u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_7};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut u64_8: u64 = 7562u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_8};
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut vec_0: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_9: u64 = 1902u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_9, on: vec_0};
    let mut weekday_0: date::Weekday = crate::date::Weekday::Monday;
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_10: u64 = 5072u64;
    let mut u64_11: u64 = 7340u64;
    let mut u64_12: u64 = 4076u64;
    let mut i64_0: i64 = -1532i64;
    let mut u64_13: u64 = 6931u64;
    let mut vec_1: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Never;
    let mut vec_2: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut weekdelta_1: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_11, on: vec_2};
    let mut repdelta_2: date::RepDelta = crate::date::RepDelta::Week(weekdelta_0);
    let mut repetition_1: crate::date::Repetition = crate::date::Repetition {delta: repdelta_1, end: repend_1};
    let mut option_4: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut duration_1: date::Duration = crate::date::Duration::Day(u64_10);
    let mut option_5: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_14: u64 = 4549u64;
    let mut u64_15: u64 = 2392u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_14, u64_15, u64_12);
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_13, string_0, i64_0, simpledate_2, option_5, option_4, vec_1);
    let mut datafile_2: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_1_ref_0: &mut crate::data::Datafile = &mut datafile_1;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3034() {
    rusty_monitor::set_test_id(3034);
    let mut str_0: &str = "Di8yw";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Saturday;
    let mut u64_0: u64 = 258u64;
    let mut u64_1: u64 = 5178u64;
    let mut u64_2: u64 = 1561u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_2};
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_3: u64 = 1970u64;
    let mut u64_4: u64 = 5442u64;
    let mut u64_5: u64 = 2971u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_5, u64_4, u64_3);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut vec_1: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_6: u64 = 6909u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_6, on: vec_1};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Week(weekdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<date::Duration> = std::option::Option::None;
    let mut str_1: &str = "CxsQuxKcpET09";
    let mut string_1: std::string::String = std::string::String::from(str_1);
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_1, weekid: u64_0, day: weekday_0};
    crate::data::Datafile::add_tag(datafile_0_ref_0, string_0);
    let mut weekday_1: date::Weekday = crate::date::Weekday::Wednesday;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_162() {
    rusty_monitor::set_test_id(162);
    let mut u64_0: u64 = 6108u64;
    let mut u64_1: u64 = 5129u64;
    let mut u64_2: u64 = 4725u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_2, u64_1, u64_0);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut u64_3: u64 = 8384u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_3};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_4: u64 = 7551u64;
    let mut u64_5: u64 = 2332u64;
    let mut u64_6: u64 = 317u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_6, month: u64_5, day: u64_4};
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_7: u64 = 7834u64;
    let mut duration_0: date::Duration = crate::date::Duration::Year(u64_7);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_8: u64 = 4832u64;
    let mut u64_9: u64 = 7897u64;
    let mut u64_10: u64 = 4037u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_10, month: u64_9, day: u64_8};
    let mut i64_0: i64 = 17499i64;
    let mut str_0: &str = "ntfifzen";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_11: u64 = 4289u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_11, string_0, i64_0, simpledate_2, option_3, option_2, vec_0);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    crate::data::Datafile::insert(datafile_0_ref_0, expense_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6202() {
    rusty_monitor::set_test_id(6202);
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_0: u64 = 4555u64;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_1: u64 = 1632u64;
    let mut u64_2: u64 = 2356u64;
    let mut u64_3: u64 = 2168u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_3, month: u64_2, day: u64_1};
    let mut i64_0: i64 = -7369i64;
    let mut str_0: &str = "ou9FKfxGdIdfJQd";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_4: u64 = 5850u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_4, string_0, i64_0, simpledate_0, option_1, option_0, vec_0);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_5: u64 = 4208u64;
    let mut duration_1: date::Duration = crate::date::Duration::Month(u64_5);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
    let mut u64_6: u64 = 1706u64;
    let mut u64_7: u64 = 2013u64;
    let mut u64_8: u64 = 618u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_8, month: u64_7, day: u64_6};
    let mut u64_9: u64 = 6832u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_9);
    let mut u64_10: u64 = 3366u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_10};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_4: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_5: std::option::Option<date::Duration> = std::option::Option::None;
    let mut option_6: std::option::Option<crate::date::SimpleDate> = std::option::Option::None;
    let mut u64_11: u64 = 8957u64;
    let mut u64_12: u64 = 3979u64;
    let mut u64_13: u64 = 4484u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_13, u64_12, u64_11);
    let mut vec_1: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_7: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_14: u64 = 2881u64;
    let mut duration_2: date::Duration = crate::date::Duration::Week(u64_14);
    let mut option_8: std::option::Option<date::Duration> = std::option::Option::Some(duration_2);
    let mut u64_15: u64 = 6878u64;
    let mut u64_16: u64 = 7061u64;
    let mut u64_17: u64 = 9430u64;
    let mut simpledate_3: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_17, u64_16, u64_15);
    let mut i64_1: i64 = 16873i64;
    let mut str_1: &str = "wCneub6NATYegirbmq";
    let mut string_1: std::string::String = std::string::String::from(str_1);
    let mut u64_18: u64 = 2187u64;
    let mut expense_1: crate::expense::Expense = crate::expense::Expense::new(u64_18, string_1, i64_1, simpledate_3, option_8, option_7, vec_1);
    let mut expense_1_ref_0: &crate::expense::Expense = &mut expense_1;
    let mut vec_2: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_19: u64 = 6987u64;
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_19, days: vec_2};
    let mut str_2: &str = crate::expense::Expense::description(expense_1_ref_0);
    let mut i64_2: i64 = crate::expense::Expense::amount(expense_0_ref_0);
    panic!("From RustyUnit with love");
}
}