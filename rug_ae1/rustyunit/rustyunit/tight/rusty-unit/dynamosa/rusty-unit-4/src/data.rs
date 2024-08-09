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
fn rusty_test_1514() {
    rusty_monitor::set_test_id(1514);
    let mut u64_0: u64 = 7357u64;
    let mut u64_1: u64 = 2161u64;
    let mut u64_2: u64 = 9833u64;
    let mut u64_3: u64 = 2802u64;
    let mut u64_4: u64 = 6101u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_4, month: u64_3, day: u64_2};
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut u64_5: u64 = 3763u64;
    let mut u64_6: u64 = 5374u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_6, month: u64_1, day: u64_5};
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut vec_0: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut vec_1: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_7: u64 = 2176u64;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Sunday;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_7, days: vec_1};
    let mut expense_slice_0: &[crate::expense::Expense] = crate::data::Datafile::expenses_between(datafile_0_ref_0, simpledate_1_ref_0, simpledate_0_ref_0);
    let mut weekday_1: date::Weekday = crate::date::Weekday::Friday;
    let mut weekday_2: date::Weekday = crate::date::Weekday::Tuesday;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4610() {
    rusty_monitor::set_test_id(4610);
    let mut u64_0: u64 = 1659u64;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut u64_1: u64 = 3763u64;
    let mut u64_2: u64 = 1244u64;
    let mut u64_3: u64 = 5374u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_3, month: u64_2, day: u64_1};
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_1_ref_0: &crate::data::Datafile = &mut datafile_1;
    let mut vec_0: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut vec_1: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_4: u64 = 2176u64;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Sunday;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_4, days: vec_1};
    let mut option_0: std::option::Option<&crate::expense::Expense> = crate::data::Datafile::find(datafile_0_ref_0, u64_0);
    let mut vec_0_ref_0: &mut std::vec::Vec<crate::expense::Expense> = &mut vec_0;
    let mut option_1: std::option::Option<crate::expense::Expense> = std::vec::Vec::pop(vec_0_ref_0);
    let mut weekday_1: date::Weekday = crate::date::Weekday::Friday;
    let mut weekday_2: date::Weekday = crate::date::Weekday::Tuesday;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4334() {
    rusty_monitor::set_test_id(4334);
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_0: u64 = 9180u64;
    let mut u64_1: u64 = 6589u64;
    let mut u64_2: u64 = 3905u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_2, month: u64_1, day: u64_0};
    let mut i64_0: i64 = 4714i64;
    let mut str_0: &str = "BEu9xIeXEFtqeOaCXC7";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_3: u64 = 9550u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_3, string_0, i64_0, simpledate_0, option_1, option_0, vec_0);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut u64_4: u64 = 6594u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_4};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut u64_5: u64 = 8373u64;
    let mut u64_6: u64 = 6037u64;
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_7: u64 = 6717u64;
    let mut u64_8: u64 = 8531u64;
    let mut u64_9: u64 = 5960u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_9, month: u64_8, day: u64_7};
    let mut u64_10: u64 = 3632u64;
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut u64_11: u64 = 998u64;
    let mut vec_1: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut u64_12: u64 = 7281u64;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Thursday;
    let mut duration_0: date::Duration = crate::date::Duration::Year(u64_12);
    let mut yeardelta_1: crate::date::YearDelta = crate::date::YearDelta {nth: u64_5};
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_6, month: u64_11, day: u64_10};
    let mut simpledate_3: &crate::date::SimpleDate = crate::expense::Expense::get_start_date(expense_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_37() {
    rusty_monitor::set_test_id(37);
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_0: u64 = 1392u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_0);
    let mut vec_1: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_1: u64 = 6319u64;
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_1, days: vec_1};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnDate(monthdeltadate_0);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_2: u64 = 5500u64;
    let mut duration_0: date::Duration = crate::date::Duration::Week(u64_2);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_3: u64 = 4742u64;
    let mut u64_4: u64 = 161u64;
    let mut u64_5: u64 = 4504u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_5, u64_4, u64_3);
    let mut i64_0: i64 = 1949i64;
    let mut str_0: &str = "0yR0ej79";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_6: u64 = 2686u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_6, string_0, i64_0, simpledate_0, option_1, option_0, vec_0);
    let mut vec_2: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut u64_7: u64 = 7890u64;
    let mut u64_8: u64 = 2810u64;
    let mut u64_9: u64 = 6041u64;
    let mut duration_1: date::Duration = crate::date::Duration::Day(u64_9);
    let mut duration_2: date::Duration = crate::date::Duration::Month(u64_8);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Saturday;
    let mut weekday_1: date::Weekday = crate::date::Weekday::Thursday;
    let mut vec_2_ref_0: &mut std::vec::Vec<crate::expense::Expense> = &mut vec_2;
    std::vec::Vec::push(vec_2_ref_0, expense_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5873() {
    rusty_monitor::set_test_id(5873);
    let mut u64_0: u64 = 3763u64;
    let mut u64_1: u64 = 1244u64;
    let mut u64_2: u64 = 5374u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_2, month: u64_1, day: u64_0};
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut vec_0: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_3: u64 = 2176u64;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Sunday;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_3, days: vec_0};
    let mut weekday_1: date::Weekday = crate::date::Weekday::Tuesday;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_39() {
    rusty_monitor::set_test_id(39);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Thursday;
    let mut u64_0: u64 = 796u64;
    let mut u64_1: u64 = 2995u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_1, weekid: u64_0, day: weekday_0};
    let mut u64_2: u64 = 7156u64;
    let mut u64_3: u64 = 8541u64;
    let mut u64_4: u64 = 1622u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_4, month: u64_3, day: u64_2};
    let mut u64_5: u64 = 5388u64;
    let mut u64_6: u64 = 9437u64;
    let mut u64_7: u64 = 5647u64;
    let mut u64_8: u64 = 3746u64;
    let mut str_0: &str = "wYFo";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut u64_9: u64 = 6073u64;
    let mut u64_10: u64 = 4857u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_10};
    let mut weekday_1: date::Weekday = crate::date::Weekday::Friday;
    let mut u64_11: u64 = 1958u64;
    let mut u64_12: u64 = 7292u64;
    let mut monthdeltaweek_1: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_12, weekid: u64_11, day: weekday_1};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_1);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_9);
    crate::data::Datafile::add_tag(datafile_0_ref_0, string_0);
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_8, month: u64_7, day: u64_6};
    let mut weekday_2: date::Weekday = crate::date::Weekday::Friday;
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut monthdelta_1: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6057() {
    rusty_monitor::set_test_id(6057);
    let mut u64_0: u64 = 5751u64;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_1: u64 = 1742u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_1);
    let mut vec_1: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_2: u64 = 1700u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_2, on: vec_1};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Week(weekdelta_0);
    let mut option_0: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_3: u64 = 724u64;
    let mut u64_4: u64 = 717u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_0, u64_4, u64_3);
    let mut str_0: &str = "0TpR";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Friday;
    let mut weekday_1: date::Weekday = crate::date::Weekday::Tuesday;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5425() {
    rusty_monitor::set_test_id(5425);
    let mut tuple_0: () = ();
    let mut result_0: std::result::Result<(), std::boxed::Box<dyn std::error::Error>> = std::result::Result::Ok(tuple_0);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut u64_0: u64 = 3677u64;
    let mut duration_0: date::Duration = crate::date::Duration::Year(u64_0);
    let mut u64_1: u64 = 9786u64;
    let mut u64_2: u64 = 5512u64;
    let mut u64_3: u64 = 5378u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_3, u64_2, u64_1);
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_0: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_4: u64 = 4749u64;
    let mut u64_5: u64 = 7976u64;
    let mut option_1: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut duration_1: date::Duration = crate::date::Duration::Year(u64_4);
    let mut option_2: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut datafile_2: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_1_ref_0: &mut crate::data::Datafile = &mut datafile_1;
    let mut result_1: std::result::Result<(), std::boxed::Box<dyn std::error::Error>> = crate::data::Datafile::remove(datafile_0_ref_0, u64_5);
    let mut tuple_1: () = std::result::Result::unwrap(result_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4122() {
    rusty_monitor::set_test_id(4122);
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_0: u64 = 7880u64;
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_1: u64 = 8816u64;
    let mut u64_2: u64 = 2255u64;
    let mut u64_3: u64 = 1662u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_3, month: u64_2, day: u64_1};
    let mut i64_0: i64 = -3627i64;
    let mut str_0: &str = "";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_4: u64 = 1387u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_4, string_0, i64_0, simpledate_0, option_1, option_0, vec_0);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut u64_5: u64 = 3145u64;
    let mut u64_6: u64 = 9012u64;
    let mut u64_7: u64 = 4239u64;
    let mut vec_1: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_8: u64 = 3669u64;
    let mut duration_1: date::Duration = crate::date::Duration::Week(u64_8);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
    let mut u64_9: u64 = 3055u64;
    let mut u64_10: u64 = 8146u64;
    let mut u64_11: u64 = 8832u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_11, u64_10, u64_9);
    let mut str_1: &str = "tQnMtgI6lIoBrlBhnX";
    let mut string_1: std::string::String = std::string::String::from(str_1);
    let mut u64_12: u64 = 4299u64;
    let mut u64_13: u64 = 6823u64;
    let mut u64_14: u64 = 8220u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_14, month: u64_13, day: u64_12};
    let mut simpledate_2_ref_0: &crate::date::SimpleDate = &mut simpledate_2;
    let mut str_2: &str = "Qvox00zK5Q";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut u64_15: u64 = 6991u64;
    let mut u64_15_ref_0: &u64 = &mut u64_15;
    let mut str_3: &str = "IgrzrTzMemim";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut u64_16: u64 = 2321u64;
    let mut u64_17: u64 = 237u64;
    let mut u64_18: u64 = 2548u64;
    let mut simpledate_3: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_18, u64_17, u64_16);
    let mut simpledate_3_ref_0: &crate::date::SimpleDate = &mut simpledate_3;
    let mut str_4: &str = "5KewTESdHI4OnmSXM";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut u64_19: u64 = 5986u64;
    let mut vec_2: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_19, days: vec_2};
    let mut u64_20: u64 = 5774u64;
    let mut vec_3: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut u64_21: u64 = 7281u64;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Thursday;
    let mut duration_2: date::Duration = crate::date::Duration::Year(u64_21);
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_20};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnDate(monthdeltadate_0);
    let mut simpledate_4: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_7, u64_6, u64_5);
    crate::data::Datafile::insert(datafile_0_ref_0, expense_0);
    panic!("From RustyUnit with love");
}
}