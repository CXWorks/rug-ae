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
fn rusty_test_1109() {
    rusty_monitor::set_test_id(1109);
    let mut u64_0: u64 = 3405u64;
    let mut u64_1: u64 = 6398u64;
    let mut u64_2: u64 = 8367u64;
    let mut u64_3: u64 = 6818u64;
    let mut u64_4: u64 = 9376u64;
    let mut u64_5: u64 = 3716u64;
    let mut u64_6: u64 = 2022u64;
    let mut u64_7: u64 = 2285u64;
    let mut u64_8: u64 = 3190u64;
    let mut u64_9: u64 = 6471u64;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut u64_10: u64 = 4253u64;
    let mut u64_11: u64 = 2126u64;
    let mut u64_12: u64 = 3216u64;
    let mut u64_13: u64 = 5693u64;
    let mut vec_0: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_14: u64 = 5300u64;
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_14, days: vec_0};
    let mut duration_0: date::Duration = crate::date::Duration::Week(u64_13);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Wednesday;
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_12);
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Count(u64_11);
    let mut weekday_1: date::Weekday = crate::date::Weekday::Friday;
    let mut result_0: std::result::Result<(), std::boxed::Box<dyn std::error::Error>> = crate::data::Datafile::remove(datafile_0_ref_0, u64_9);
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_8};
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_7, month: u64_6, day: u64_5};
    let mut datafile_1_ref_0: &crate::data::Datafile = &mut datafile_1;
    let mut option_0: std::option::Option<&crate::expense::Expense> = crate::data::Datafile::find(datafile_1_ref_0, u64_4);
    let mut duration_1: date::Duration = crate::date::Duration::Week(u64_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3612() {
    rusty_monitor::set_test_id(3612);
    let mut vec_0: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_0: u64 = 1721u64;
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_0, days: vec_0};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnDate(monthdeltadate_0);
    let mut vec_1: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_1: u64 = 9345u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_1);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Monday;
    let mut u64_2: u64 = 3451u64;
    let mut u64_3: u64 = 8566u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_3, weekid: u64_2, day: weekday_0};
    let mut monthdelta_1: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_1);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_4: u64 = 8636u64;
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_4);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_5: u64 = 3160u64;
    let mut u64_6: u64 = 4227u64;
    let mut u64_7: u64 = 7956u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_7, u64_6, u64_5);
    let mut i64_0: i64 = -7695i64;
    let mut str_0: &str = "T9Y56SI";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_8: u64 = 9936u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_8, string_0, i64_0, simpledate_0, option_1, option_0, vec_1);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Never;
    let mut weekday_1: date::Weekday = crate::date::Weekday::Wednesday;
    let mut i64_1: i64 = crate::expense::Expense::amount(expense_0_ref_0);
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2496() {
    rusty_monitor::set_test_id(2496);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Sunday;
    let mut u64_0: u64 = 3812u64;
    let mut u64_1: u64 = 9747u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_1, weekid: u64_0, day: weekday_0};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    let mut u64_2: u64 = 1132u64;
    let mut u64_3: u64 = 7633u64;
    let mut u64_4: u64 = 8451u64;
    let mut u64_5: u64 = 1855u64;
    let mut u64_6: u64 = 1356u64;
    let mut u64_7: u64 = 336u64;
    let mut u64_8: u64 = 3418u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_8, month: u64_7, day: u64_6};
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut u64_9: u64 = 2270u64;
    let mut u64_10: u64 = 7231u64;
    let mut u64_11: u64 = 2979u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_11, month: u64_10, day: u64_9};
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut u64_12: u64 = 8447u64;
    let mut u64_13: u64 = 9926u64;
    let mut str_0: &str = "HDTt";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_1_ref_0: &mut crate::data::Datafile = &mut datafile_1;
    crate::data::Datafile::add_tag(datafile_1_ref_0, string_0);
    let mut expense_slice_0: &[crate::expense::Expense] = crate::data::Datafile::expenses_between(datafile_0_ref_0, simpledate_1_ref_0, simpledate_0_ref_0);
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_4, u64_3, u64_2);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3165() {
    rusty_monitor::set_test_id(3165);
    let mut u64_0: u64 = 7927u64;
    let mut u64_1: u64 = 7758u64;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_2: u64 = 7917u64;
    let mut u64_3: u64 = 7470u64;
    let mut u64_4: u64 = 7540u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_4, u64_3, u64_2);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut vec_1: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_5: u64 = 8065u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_5, on: vec_1};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Week(weekdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_6: u64 = 845u64;
    let mut duration_0: date::Duration = crate::date::Duration::Week(u64_6);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_7: u64 = 4495u64;
    let mut u64_8: u64 = 446u64;
    let mut u64_9: u64 = 4140u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_9, u64_8, u64_7);
    let mut i64_0: i64 = -8152i64;
    let mut str_0: &str = "C7QpsWcSqjizt";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_10: u64 = 6667u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_10, string_0, i64_0, simpledate_1, option_1, option_0, vec_0);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut str_1: &str = crate::expense::Expense::description(expense_0_ref_0);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Wednesday;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_1};
    let mut weekday_1: date::Weekday = crate::date::Weekday::Thursday;
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Count(u64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4893() {
    rusty_monitor::set_test_id(4893);
    let mut u64_0: u64 = 1096u64;
    let mut u64_1: u64 = 1084u64;
    let mut duration_0: date::Duration = crate::date::Duration::Week(u64_1);
    let mut duration_0_ref_0: &date::Duration = &mut duration_0;
    let mut u64_2: u64 = 1997u64;
    let mut u64_3: u64 = 5125u64;
    let mut u64_4: u64 = 3308u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_4, u64_3, u64_2);
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut u64_5: u64 = 6933u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_5};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut u64_6: u64 = 1323u64;
    let mut u64_7: u64 = 8594u64;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Saturday;
    let mut u64_8: u64 = 6731u64;
    let mut u64_9: u64 = 8016u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_9, weekid: u64_8, day: weekday_0};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    let mut u64_10: u64 = 774u64;
    let mut u64_11: u64 = 5475u64;
    let mut duration_1: date::Duration = crate::date::Duration::Month(u64_11);
    let mut weekday_1: date::Weekday = crate::date::Weekday::Sunday;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut option_0: std::option::Option<&crate::expense::Expense> = crate::data::Datafile::find(datafile_0_ref_0, u64_6);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_1_ref_0: &mut crate::data::Datafile = &mut datafile_1;
    let mut result_0: std::result::Result<(), std::boxed::Box<dyn std::error::Error>> = crate::data::Datafile::remove(datafile_1_ref_0, u64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4801() {
    rusty_monitor::set_test_id(4801);
    let mut u64_0: u64 = 7836u64;
    let mut u64_1: u64 = 6736u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_1};
    let mut u64_2: u64 = 3283u64;
    let mut u64_3: u64 = 46u64;
    let mut u64_4: u64 = 452u64;
    let mut u64_5: u64 = 2950u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_5, month: u64_4, day: u64_3};
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_6: u64 = 3855u64;
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_6);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_7: u64 = 8594u64;
    let mut u64_8: u64 = 623u64;
    let mut u64_9: u64 = 4433u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_9, u64_8, u64_7);
    let mut i64_0: i64 = 14478i64;
    let mut str_0: &str = "jOU9";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_10: u64 = 9000u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_10, string_0, i64_0, simpledate_1, option_1, option_0, vec_0);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    crate::data::Datafile::insert(datafile_0_ref_0, expense_0);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut duration_1: date::Duration = crate::date::Duration::Week(u64_2);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Thursday;
    let mut weekday_1: date::Weekday = crate::date::Weekday::Sunday;
    let mut weekday_2: date::Weekday = crate::date::Weekday::Tuesday;
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Never;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2033() {
    rusty_monitor::set_test_id(2033);
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_0: u64 = 7939u64;
    let mut u64_1: u64 = 1109u64;
    let mut u64_2: u64 = 5128u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_2, u64_1, u64_0);
    let mut i64_0: i64 = -14709i64;
    let mut u64_3: u64 = 5888u64;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut str_0: &str = "rWOo8w2TRTIVbC";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_1_ref_0: &mut crate::data::Datafile = &mut datafile_1;
    let mut str_1: &str = "pu9gVJrqTCEcxh6Ev";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_4: u64 = 1747u64;
    let mut duration_0: date::Duration = crate::date::Duration::Week(u64_4);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_5: u64 = 9449u64;
    let mut u64_6: u64 = 3183u64;
    let mut u64_7: u64 = 8759u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_7, u64_6, u64_5);
    let mut i64_1: i64 = -8653i64;
    let mut str_2: &str = "LXzSkCcUp1uc0Xx";
    let mut string_1: std::string::String = std::string::String::from(str_2);
    let mut u64_8: u64 = 9320u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_8, string_1, i64_1, simpledate_1, option_3, option_2, vec_0);
    let mut expense_0_ref_0: &mut crate::expense::Expense = &mut expense_0;
    crate::expense::Expense::remove_tags(expense_0_ref_0, str_1_ref_0);
    crate::data::Datafile::add_tag(datafile_1_ref_0, string_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2999() {
    rusty_monitor::set_test_id(2999);
    let mut u64_0: u64 = 1687u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_0);
    let mut u64_1: u64 = 6768u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_1};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut u64_2: u64 = 4557u64;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut vec_0: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut u64_3: u64 = 65u64;
    let mut vec_1: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut u64_4: u64 = 1781u64;
    let mut str_0: &str = "bsBP";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut vec_2: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_5: u64 = 7714u64;
    let mut duration_0: date::Duration = crate::date::Duration::Week(u64_5);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_6: u64 = 5494u64;
    let mut u64_7: u64 = 5741u64;
    let mut u64_8: u64 = 1870u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_8, month: u64_7, day: u64_6};
    let mut i64_0: i64 = 1652i64;
    let mut str_1: &str = "3lilENXWhXrbQKZH1My";
    let mut string_0: std::string::String = std::string::String::from(str_1);
    let mut u64_9: u64 = 6620u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_9, string_0, i64_0, simpledate_0, option_1, option_0, vec_2);
    let mut expense_0_ref_0: &mut crate::expense::Expense = &mut expense_0;
    crate::expense::Expense::remove_tags(expense_0_ref_0, str_0_ref_0);
    let mut result_0: std::result::Result<(), std::boxed::Box<dyn std::error::Error>> = crate::data::Datafile::remove(datafile_0_ref_0, u64_2);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1984() {
    rusty_monitor::set_test_id(1984);
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_0: u64 = 1504u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_0);
    let mut vec_1: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_1: u64 = 1535u64;
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_1, days: vec_1};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnDate(monthdeltadate_0);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_2: u64 = 7987u64;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_2);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_3: u64 = 290u64;
    let mut u64_4: u64 = 7986u64;
    let mut u64_5: u64 = 7380u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_5, month: u64_4, day: u64_3};
    let mut i64_0: i64 = -14234i64;
    let mut str_0: &str = "";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_6: u64 = 7585u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_6, string_0, i64_0, simpledate_0, option_1, option_0, vec_0);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut vec_2: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_7: u64 = 5768u64;
    let mut monthdeltadate_1: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_7, days: vec_2};
    let mut u64_8: u64 = 7341u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_8};
    let mut weekday_0: date::Weekday = crate::date::Weekday::Monday;
    let mut monthdelta_1: date::MonthDelta = crate::date::MonthDelta::OnDate(monthdeltadate_1);
    let mut simpledate_1: &crate::date::SimpleDate = crate::expense::Expense::get_start_date(expense_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4709() {
    rusty_monitor::set_test_id(4709);
    let mut str_0: &str = "UKwhAChrkRxpgG77";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut u64_0: u64 = 1632u64;
    let mut vec_0: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_1: u64 = 7253u64;
    let mut u64_2: u64 = 883u64;
    let mut u64_3: u64 = 5398u64;
    let mut u64_4: u64 = 2499u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_4, month: u64_3, day: u64_2};
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut u64_5: u64 = 2687u64;
    let mut vec_1: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_6: u64 = 3288u64;
    let mut u64_7: u64 = 4458u64;
    let mut u64_8: u64 = 2251u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_8, month: u64_7, day: u64_6};
    let mut i64_0: i64 = -12018i64;
    let mut str_1: &str = "tAVohcyB3xZE";
    let mut string_1: std::string::String = std::string::String::from(str_1);
    let mut u64_9: u64 = 3582u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_9, string_1, i64_0, simpledate_1, option_1, option_0, vec_1);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut simpledate_2: &crate::date::SimpleDate = crate::expense::Expense::get_start_date(expense_0_ref_0);
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_5);
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_1, on: vec_0};
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_0};
    crate::data::Datafile::add_tag(datafile_0_ref_0, string_0);
    panic!("From RustyUnit with love");
}
}