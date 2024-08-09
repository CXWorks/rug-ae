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
fn rusty_test_43() {
    rusty_monitor::set_test_id(43);
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_0: u64 = 6650u64;
    let mut u64_1: u64 = 211u64;
    let mut u64_2: u64 = 8202u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_2, month: u64_1, day: u64_0};
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut vec_1: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_3: u64 = 9371u64;
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_3, days: vec_1};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnDate(monthdeltadate_0);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_4: u64 = 9440u64;
    let mut duration_0: date::Duration = crate::date::Duration::Week(u64_4);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_5: u64 = 995u64;
    let mut u64_6: u64 = 1902u64;
    let mut u64_7: u64 = 7699u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_7, u64_6, u64_5);
    let mut i64_0: i64 = 7489i64;
    let mut str_0: &str = "5bg140mL0U9NVF";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_8: u64 = 3317u64;
    let mut vec_2: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_9: u64 = 5807u64;
    let mut monthdeltadate_1: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_9, days: vec_2};
    let mut monthdelta_1: date::MonthDelta = crate::date::MonthDelta::OnDate(monthdeltadate_1);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Wednesday;
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Month(monthdelta_1);
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_8, string_0, i64_0, simpledate_1, option_1, option_0, vec_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3732() {
    rusty_monitor::set_test_id(3732);
    let mut u64_0: u64 = 9570u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_0);
    let mut u64_1: u64 = 1307u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_1};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_2: u64 = 6835u64;
    let mut u64_3: u64 = 5971u64;
    let mut u64_4: u64 = 4616u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_4, u64_3, u64_2);
    let mut u64_5: u64 = 6297u64;
    let mut u64_6: u64 = 8290u64;
    let mut u64_7: u64 = 4662u64;
    let mut u64_8: u64 = 9319u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_8, month: u64_7, day: u64_6};
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut u64_9: u64 = 4916u64;
    let mut u64_10: u64 = 2589u64;
    let mut u64_11: u64 = 8782u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_11, u64_10, u64_9);
    let mut simpledate_2_ref_0: &crate::date::SimpleDate = &mut simpledate_2;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut expense_slice_0: &[crate::expense::Expense] = crate::data::Datafile::expenses_between(datafile_0_ref_0, simpledate_2_ref_0, simpledate_1_ref_0);
    let mut duration_0: date::Duration = crate::date::Duration::Week(u64_5);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Tuesday;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2040() {
    rusty_monitor::set_test_id(2040);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Friday;
    let mut weekday_1: date::Weekday = crate::date::Weekday::Tuesday;
    let mut u64_0: u64 = 6960u64;
    let mut u64_1: u64 = 4804u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_1, weekid: u64_0, day: weekday_1};
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_2: u64 = 1660u64;
    let mut u64_3: u64 = 13u64;
    let mut u64_4: u64 = 7496u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_4, u64_3, u64_2);
    let mut i64_0: i64 = -6358i64;
    let mut str_0: &str = "IFng61NtaWT4wMTUckH";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_5: u64 = 1526u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_5);
    let mut weekday_2: date::Weekday = crate::date::Weekday::Wednesday;
    let mut u64_6: u64 = 8956u64;
    let mut u64_7: u64 = 6307u64;
    let mut monthdeltaweek_1: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_7, weekid: u64_6, day: weekday_2};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_1);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_1: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_8: u64 = 3859u64;
    let mut u64_9: u64 = 1511u64;
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_9);
    let mut option_2: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_10: u64 = 8294u64;
    let mut u64_11: u64 = 3344u64;
    let mut u64_12: u64 = 5308u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_12, month: u64_11, day: u64_10};
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_8, string_0, i64_0, simpledate_0, option_0, option_1, vec_0);
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut vec_1: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    crate::data::Datafile::insert(datafile_0_ref_0, expense_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6606() {
    rusty_monitor::set_test_id(6606);
    let mut u64_0: u64 = 4128u64;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut str_0: &str = "nLc0E39UmqOsxdGi9rz";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_1_ref_0: &mut crate::data::Datafile = &mut datafile_1;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_1: u64 = 563u64;
    let mut u64_2: u64 = 1486u64;
    let mut u64_3: u64 = 5273u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_3, u64_2, u64_1);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut u64_4: u64 = 352u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_4};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut u64_5: u64 = 8832u64;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_5);
    let mut option_0: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut str_1: &str = "e";
    let mut string_1: std::string::String = std::string::String::from(str_1);
    crate::data::Datafile::add_tag(datafile_1_ref_0, string_0);
    let mut option_1: std::option::Option<&crate::expense::Expense> = crate::data::Datafile::find(datafile_0_ref_0, u64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3498() {
    rusty_monitor::set_test_id(3498);
    let mut str_0: &str = "VHLNqErXniqUU4PxizT";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_0: u64 = 263u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_0};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut u64_1: u64 = 8832u64;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_1);
    let mut option_0: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut str_1: &str = "e";
    let mut string_1: std::string::String = std::string::String::from(str_1);
    crate::data::Datafile::add_tag(datafile_0_ref_0, string_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5166() {
    rusty_monitor::set_test_id(5166);
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_0: u64 = 4108u64;
    let mut duration_0: date::Duration = crate::date::Duration::Week(u64_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_1: u64 = 7527u64;
    let mut u64_2: u64 = 6722u64;
    let mut u64_3: u64 = 3674u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_3, u64_2, u64_1);
    let mut str_0: &str = "mNYSIc30";
    let mut u64_4: u64 = 6782u64;
    let mut str_1: &str = "nLc0E39UmqOsxdGi9rz";
    let mut string_0: std::string::String = std::string::String::from(str_1);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_5: u64 = 563u64;
    let mut u64_6: u64 = 1486u64;
    let mut u64_7: u64 = 5273u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_7, u64_6, u64_5);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_1);
    let mut u64_8: u64 = 352u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_8};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_9: u64 = 8832u64;
    let mut duration_1: date::Duration = crate::date::Duration::Day(u64_9);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
    let mut u64_10: u64 = 5125u64;
    let mut u64_11: u64 = 2824u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_4, month: u64_11, day: u64_10};
    let mut i64_0: i64 = 3179i64;
    let mut string_1: std::string::String = std::string::String::from(str_0);
    let mut u64_12: u64 = 3304u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_12, string_1, i64_0, simpledate_2, option_3, option_2, vec_0);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut simpledate_3: &crate::date::SimpleDate = crate::expense::Expense::get_start_date(expense_0_ref_0);
    crate::data::Datafile::add_tag(datafile_0_ref_0, string_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7133() {
    rusty_monitor::set_test_id(7133);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Monday;
    let mut weekday_1: date::Weekday = crate::date::Weekday::Wednesday;
    let mut u64_0: u64 = 127u64;
    let mut u64_1: u64 = 7170u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_1, weekid: u64_0, day: weekday_1};
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut str_0: &str = "fTgvCRlUTybaO";
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_2: u64 = 1834u64;
    let mut duration_0: date::Duration = crate::date::Duration::Year(u64_2);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_3: u64 = 7096u64;
    let mut u64_4: u64 = 3264u64;
    let mut u64_5: u64 = 4770u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_5, u64_4, u64_3);
    let mut i64_0: i64 = 13538i64;
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_6: u64 = 5086u64;
    let mut duration_1: date::Duration = crate::date::Duration::Day(u64_6);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
    let mut u64_7: u64 = 623u64;
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_7, string_0, i64_0, simpledate_0, option_1, option_0, vec_0);
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut vec_1: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    crate::data::Datafile::insert(datafile_0_ref_0, expense_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7051() {
    rusty_monitor::set_test_id(7051);
    let mut str_0: &str = "nLc0E39UmqOsxdGi9rz";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_0: u64 = 563u64;
    let mut u64_1: u64 = 1486u64;
    let mut u64_2: u64 = 5273u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_2, u64_1, u64_0);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut u64_3: u64 = 352u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_3};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_4: u64 = 8832u64;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_4);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_5: u64 = 5125u64;
    let mut u64_6: u64 = 2824u64;
    let mut u64_7: u64 = 560u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_7, month: u64_6, day: u64_5};
    let mut i64_0: i64 = 3179i64;
    let mut str_1: &str = "e";
    let mut string_1: std::string::String = std::string::String::from(str_1);
    let mut u64_8: u64 = 3304u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_8, string_1, i64_0, simpledate_1, option_1, option_0, vec_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6428() {
    rusty_monitor::set_test_id(6428);
    let mut u64_0: u64 = 4078u64;
    let mut u64_1: u64 = 4906u64;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Tuesday;
    let mut u64_2: u64 = 9410u64;
    let mut u64_3: u64 = 3214u64;
    let mut weekday_1: date::Weekday = crate::date::Weekday::Friday;
    let mut u64_4: u64 = 7371u64;
    let mut u64_5: u64 = 9863u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_5, weekid: u64_4, day: weekday_1};
    let mut u64_6: u64 = 4097u64;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut duration_0: date::Duration = crate::date::Duration::Week(u64_1);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_7: u64 = 971u64;
    let mut u64_8: u64 = 7201u64;
    let mut u64_9: u64 = 7319u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_9, month: u64_8, day: u64_7};
    let mut i64_0: i64 = -14562i64;
    let mut str_0: &str = "mG";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_10: u64 = 8761u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_10, string_0, i64_0, simpledate_0, option_1, option_0, vec_0);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut vec_1: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    crate::data::Datafile::insert(datafile_0_ref_0, expense_0);
    let mut weekday_2: date::Weekday = crate::date::Weekday::Wednesday;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_6);
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    let mut weekday_3: date::Weekday = crate::date::Weekday::Thursday;
    let mut weekday_4: date::Weekday = crate::date::Weekday::Sunday;
    let mut monthdeltaweek_1: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_3, weekid: u64_2, day: weekday_0};
    let mut duration_1: date::Duration = crate::date::Duration::Month(u64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3502() {
    rusty_monitor::set_test_id(3502);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Saturday;
    let mut u64_0: u64 = 4585u64;
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_0);
    let mut option_0: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_1: u64 = 6676u64;
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_1_ref_0: &mut crate::data::Datafile = &mut datafile_1;
    let mut u64_2: u64 = 8714u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_2};
    let mut datafile_2: crate::data::Datafile = crate::data::Datafile::new();
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut result_0: std::result::Result<(), std::boxed::Box<dyn std::error::Error>> = crate::data::Datafile::remove(datafile_1_ref_0, u64_1);
    let mut weekday_1: date::Weekday = crate::date::Weekday::Tuesday;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_57() {
    rusty_monitor::set_test_id(57);
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_0: u64 = 5466u64;
    let mut duration_0: date::Duration = crate::date::Duration::Year(u64_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_1: u64 = 1066u64;
    let mut u64_2: u64 = 7207u64;
    let mut u64_3: u64 = 8737u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_3, month: u64_2, day: u64_1};
    let mut i64_0: i64 = 3279i64;
    let mut str_0: &str = "G";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_4: u64 = 8169u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_4, string_0, i64_0, simpledate_0, option_1, option_0, vec_0);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut vec_1: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_5: u64 = 8789u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_5);
    let mut vec_2: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_6: u64 = 9306u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_6, on: vec_2};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Week(weekdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_7: u64 = 3837u64;
    let mut duration_1: date::Duration = crate::date::Duration::Day(u64_7);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
    let mut str_1: &str = "0bl1";
    let mut string_1: std::string::String = std::string::String::from(str_1);
    let mut u64_8: u64 = 4928u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_8};
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut str_2: &str = crate::expense::Expense::description(expense_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_785() {
    rusty_monitor::set_test_id(785);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Tuesday;
    let mut weekday_1: date::Weekday = crate::date::Weekday::Friday;
    let mut u64_0: u64 = 7371u64;
    let mut u64_1: u64 = 9863u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_1, weekid: u64_0, day: weekday_1};
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_2: u64 = 5781u64;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_2);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_3: u64 = 971u64;
    let mut u64_4: u64 = 7201u64;
    let mut u64_5: u64 = 7319u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_5, month: u64_4, day: u64_3};
    let mut i64_0: i64 = -14562i64;
    let mut str_0: &str = "mG";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_6: u64 = 8761u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_6, string_0, i64_0, simpledate_0, option_1, option_0, vec_0);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut vec_1: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    crate::data::Datafile::insert(datafile_0_ref_0, expense_0);
    let mut weekday_2: date::Weekday = crate::date::Weekday::Wednesday;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1189() {
    rusty_monitor::set_test_id(1189);
    let mut u64_0: u64 = 1824u64;
    let mut u64_0_ref_0: &u64 = &mut u64_0;
    let mut str_0: &str = "PaTxsEWA";
    let mut str_1: &str = "nLc0E39UmqOsxdGi9rz";
    let mut string_0: std::string::String = std::string::String::from(str_1);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_1: u64 = 7936u64;
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_1_ref_0: &u64 = &mut u64_1;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_16() {
    rusty_monitor::set_test_id(16);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Thursday;
    let mut u64_0: u64 = 3552u64;
    let mut u64_1: u64 = 3754u64;
    let mut u64_2: u64 = 4794u64;
    let mut u64_3: u64 = 7686u64;
    let mut u64_4: u64 = 6191u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_4, u64_3, u64_2);
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut weekday_1: date::Weekday = crate::date::Weekday::Friday;
    let mut u64_5: u64 = 4977u64;
    let mut u64_6: u64 = 6014u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_6, weekid: u64_5, day: weekday_1};
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_7: u64 = 4952u64;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_7);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_8: u64 = 7582u64;
    let mut u64_9: u64 = 5561u64;
    let mut u64_10: u64 = 6137u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_10, month: u64_9, day: u64_8};
    let mut i64_0: i64 = 8968i64;
    let mut str_0: &str = "0QlaLZhYV7clU";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_11: u64 = 8430u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_11, string_0, i64_0, simpledate_1, option_1, option_0, vec_0);
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut option_2: &std::option::Option<crate::date::SimpleDate> = crate::expense::Expense::get_end_date(expense_0_ref_0);
    let mut monthdeltaweek_1: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_1, weekid: u64_0, day: weekday_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6455() {
    rusty_monitor::set_test_id(6455);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Saturday;
    let mut u64_0: u64 = 179u64;
    let mut u64_1: u64 = 939u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_1, weekid: u64_0, day: weekday_0};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    let mut u64_2: u64 = 6093u64;
    let mut u64_3: u64 = 8062u64;
    let mut u64_4: u64 = 7347u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_4, u64_3, u64_2);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut u64_5: u64 = 6388u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_5};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut str_0: &str = "nLc0E39UmqOsxdGi9rz";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_6: u64 = 563u64;
    let mut u64_7: u64 = 1016u64;
    let mut u64_8: u64 = 5273u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_8, u64_7, u64_6);
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Date(simpledate_1);
    let mut u64_9: u64 = 352u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_9};
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_10: u64 = 8832u64;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_10);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_11: u64 = 5125u64;
    let mut u64_12: u64 = 2824u64;
    let mut u64_13: u64 = 560u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_13, month: u64_12, day: u64_11};
    let mut i64_0: i64 = 3179i64;
    let mut str_1: &str = "e";
    let mut string_1: std::string::String = std::string::String::from(str_1);
    let mut u64_14: u64 = 3304u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_14, string_1, i64_0, simpledate_2, option_1, option_0, vec_0);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut simpledate_3: &crate::date::SimpleDate = crate::expense::Expense::get_start_date(expense_0_ref_0);
    crate::data::Datafile::add_tag(datafile_0_ref_0, string_0);
    let mut repdelta_2: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    panic!("From RustyUnit with love");
}
}