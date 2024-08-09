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
fn rusty_test_2043() {
    rusty_monitor::set_test_id(2043);
    let mut u64_0: u64 = 8628u64;
    let mut u64_1: u64 = 8209u64;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_2: u64 = 1082u64;
    let mut u64_3: u64 = 2077u64;
    let mut u64_4: u64 = 4895u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_4, u64_3, u64_2);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut vec_1: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_5: u64 = 33u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_5, on: vec_1};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Week(weekdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_6: u64 = 8735u64;
    let mut duration_0: date::Duration = crate::date::Duration::Week(u64_6);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_7: u64 = 3014u64;
    let mut u64_8: u64 = 5207u64;
    let mut u64_9: u64 = 1407u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_9, u64_8, u64_7);
    let mut i64_0: i64 = -11211i64;
    let mut str_0: &str = "2EnLa6PdifVgcK5";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_10: u64 = 5963u64;
    let mut u64_11: u64 = 4734u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_11};
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_10, string_0, i64_0, simpledate_1, option_1, option_0, vec_0);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut duration_1: date::Duration = crate::date::Duration::Month(u64_1);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Monday;
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut option_2: std::option::Option<&crate::expense::Expense> = crate::data::Datafile::find(datafile_0_ref_0, u64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2103() {
    rusty_monitor::set_test_id(2103);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Saturday;
    let mut u64_0: u64 = 1158u64;
    let mut u64_1: u64 = 7667u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_1, weekid: u64_0, day: weekday_0};
    let mut u64_2: u64 = 6609u64;
    let mut u64_3: u64 = 2324u64;
    let mut u64_4: u64 = 5557u64;
    let mut u64_5: u64 = 6455u64;
    let mut str_0: &str = "Zl";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_6: u64 = 3340u64;
    let mut duration_0: date::Duration = crate::date::Duration::Week(u64_6);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_7: u64 = 4978u64;
    let mut u64_8: u64 = 9104u64;
    let mut u64_9: u64 = 8466u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_9, u64_8, u64_7);
    let mut i64_0: i64 = -7026i64;
    let mut str_1: &str = "eTZbl70wkCQ";
    let mut string_0: std::string::String = std::string::String::from(str_1);
    let mut u64_10: u64 = 1852u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_10, string_0, i64_0, simpledate_0, option_1, option_0, vec_0);
    let mut expense_0_ref_0: &mut crate::expense::Expense = &mut expense_0;
    crate::expense::Expense::remove_tags(expense_0_ref_0, str_0_ref_0);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_5};
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_4, month: u64_3, day: u64_2};
    let mut weekday_1: date::Weekday = crate::date::Weekday::Sunday;
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3322() {
    rusty_monitor::set_test_id(3322);
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut u64_0: u64 = 998u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_0};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_1: u64 = 241u64;
    let mut u64_2: u64 = 3375u64;
    let mut u64_3: u64 = 6154u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_3, month: u64_2, day: u64_1};
    let mut i64_0: i64 = 3630i64;
    let mut str_0: &str = "XZcI1EF";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_4: u64 = 2762u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_4, string_0, i64_0, simpledate_0, option_1, option_0, vec_0);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut u64_5: u64 = 9429u64;
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Never;
    let mut u64_6: u64 = 2047u64;
    let mut yeardelta_1: crate::date::YearDelta = crate::date::YearDelta {nth: u64_6};
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Year(yeardelta_1);
    let mut u64_7: u64 = 2253u64;
    let mut u64_8: u64 = 9328u64;
    let mut u64_9: u64 = 8109u64;
    let mut u64_10: u64 = 8574u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_10};
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_9, month: u64_8, day: u64_7};
    let mut repetition_1: crate::date::Repetition = crate::date::Repetition {delta: repdelta_1, end: repend_1};
    let mut yeardelta_2: crate::date::YearDelta = crate::date::YearDelta {nth: u64_5};
    crate::data::Datafile::insert(datafile_0_ref_0, expense_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1663() {
    rusty_monitor::set_test_id(1663);
    let mut u64_0: u64 = 331u64;
    let mut u64_1: u64 = 568u64;
    let mut u64_2: u64 = 8987u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_2, u64_1, u64_0);
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut u64_3: u64 = 8790u64;
    let mut u64_4: u64 = 5624u64;
    let mut u64_5: u64 = 9444u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_5, u64_4, u64_3);
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut u64_6: u64 = 4260u64;
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_7: u64 = 9894u64;
    let mut u64_8: u64 = 2603u64;
    let mut u64_9: u64 = 8805u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_9, month: u64_8, day: u64_7};
    let mut i64_0: i64 = -12732i64;
    let mut u64_10: u64 = 8870u64;
    let mut u64_11: u64 = 4425u64;
    let mut u64_12: u64 = 5454u64;
    let mut u64_13: u64 = 5432u64;
    let mut u64_14: u64 = 3641u64;
    let mut u64_15: u64 = 524u64;
    let mut u64_16: u64 = 1323u64;
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut simpledate_3: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_16, month: u64_15, day: u64_14};
    let mut simpledate_4: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_13, u64_12, u64_11);
    let mut expense_slice_0: &[crate::expense::Expense] = crate::data::Datafile::expenses_between(datafile_0_ref_0, simpledate_1_ref_0, simpledate_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4815() {
    rusty_monitor::set_test_id(4815);
    let mut u64_0: u64 = 9819u64;
    let mut u64_1: u64 = 4811u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_1};
    let mut u64_2: u64 = 7955u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_2};
    let mut u64_3: u64 = 81u64;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut u64_4: u64 = 7572u64;
    let mut u64_5: u64 = 1455u64;
    let mut u64_6: u64 = 686u64;
    let mut u64_7: u64 = 6435u64;
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_1_ref_0: &mut crate::data::Datafile = &mut datafile_1;
    let mut vec_0: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut u64_8: u64 = 4762u64;
    let mut bool_0: bool = false;
    let mut u64_9: u64 = 8848u64;
    let mut bool_1: bool = false;
    let mut u64_10: u64 = 5958u64;
    let mut vec_1: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_11: u64 = 8674u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_11, on: vec_1};
    let mut result_0: std::result::Result<(), std::boxed::Box<dyn std::error::Error>> = crate::data::Datafile::remove(datafile_1_ref_0, u64_7);
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_6, u64_5, u64_4);
    let mut datafile_2: crate::data::Datafile = crate::data::Datafile::new();
    let mut tuple_0: () = std::result::Result::unwrap(result_0);
    let mut result_1: std::result::Result<(), std::boxed::Box<dyn std::error::Error>> = crate::data::Datafile::remove(datafile_0_ref_0, u64_3);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut yeardelta_1: crate::date::YearDelta = crate::date::YearDelta {nth: u64_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2345() {
    rusty_monitor::set_test_id(2345);
    let mut u64_0: u64 = 2076u64;
    let mut u64_1: u64 = 8903u64;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Thursday;
    let mut u64_2: u64 = 980u64;
    let mut u64_3: u64 = 3416u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_3, weekid: u64_2, day: weekday_0};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    let mut u64_4: u64 = 103u64;
    let mut u64_5: u64 = 8253u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_5};
    let mut weekday_1: date::Weekday = crate::date::Weekday::Tuesday;
    let mut u64_6: u64 = 5326u64;
    let mut u64_7: u64 = 1647u64;
    let mut monthdeltaweek_1: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_7, weekid: u64_6, day: weekday_1};
    let mut monthdelta_1: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_1);
    let mut u64_8: u64 = 2233u64;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut vec_0: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut u64_9: u64 = 1564u64;
    let mut u64_10: u64 = 1752u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_10};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut option_0: std::option::Option<&crate::expense::Expense> = crate::data::Datafile::find(datafile_0_ref_0, u64_8);
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Month(monthdelta_1);
    let mut repdelta_2: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut daydelta_1: crate::date::DayDelta = crate::date::DayDelta {nth: u64_4};
    let mut expense_0: &crate::expense::Expense = std::option::Option::unwrap(option_0);
    let mut repdelta_3: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut weekday_2: date::Weekday = crate::date::Weekday::Saturday;
    let mut daydelta_2: crate::date::DayDelta = crate::date::DayDelta {nth: u64_1};
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1472() {
    rusty_monitor::set_test_id(1472);
    let mut u64_0: u64 = 5803u64;
    let mut u64_1: u64 = 158u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Saturday;
    let mut u64_2: u64 = 8624u64;
    let mut u64_3: u64 = 9758u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_3, weekid: u64_2, day: weekday_0};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_4: u64 = 674u64;
    let mut u64_5: u64 = 8211u64;
    let mut duration_0: date::Duration = crate::date::Duration::Year(u64_5);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_6: u64 = 5045u64;
    let mut u64_7: u64 = 2978u64;
    let mut u64_8: u64 = 8124u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_8, u64_7, u64_6);
    let mut i64_0: i64 = 2955i64;
    let mut u64_9: u64 = 576u64;
    let mut str_0: &str = "B6ETJKGo6";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut vec_0: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut u64_10: u64 = 5280u64;
    crate::data::Datafile::add_tag(datafile_0_ref_0, string_0);
    let mut vec_0_ref_0: &mut std::vec::Vec<crate::expense::Expense> = &mut vec_0;
    let mut option_2: std::option::Option<crate::expense::Expense> = std::vec::Vec::pop(vec_0_ref_0);
    let mut duration_1: date::Duration = crate::date::Duration::Week(u64_1);
    let mut duration_2: date::Duration = crate::date::Duration::Month(u64_0);
    panic!("From RustyUnit with love");
}
}