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

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4915() {
//    rusty_monitor::set_test_id(4915);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut vec_0: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut str_0: &str = "rd";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_0: u64 = 2702u64;
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut result_0: std::result::Result<(), std::boxed::Box<dyn std::error::Error>> = crate::data::Datafile::remove(datafile_0_ref_0, u64_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_109() {
//    rusty_monitor::set_test_id(109);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Sunday;
    let mut u64_0: u64 = 7325u64;
    let mut u64_1: u64 = 843u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_1, weekid: u64_0, day: weekday_0};
    let mut weekday_1: date::Weekday = crate::date::Weekday::Sunday;
    let mut u64_2: u64 = 400u64;
    let mut u64_3: u64 = 23u64;
    let mut u64_4: u64 = 7276u64;
    let mut str_0: &str = "migrate";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_5: u64 = 4290u64;
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_5);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_6: u64 = 120u64;
    let mut u64_7: u64 = 4u64;
    let mut u64_8: u64 = 334u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_8, month: u64_7, day: u64_6};
    let mut i64_0: i64 = 100i64;
    let mut str_1: &str = "rd";
    let mut string_1: std::string::String = std::string::String::from(str_1);
    let mut u64_9: u64 = 100u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_9, string_1, i64_0, simpledate_0, option_1, option_0, vec_0);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut i64_1: i64 = crate::expense::Expense::amount(expense_0_ref_0);
    crate::data::Datafile::add_tag(datafile_0_ref_0, string_0);
    let mut duration_1: date::Duration = crate::date::Duration::Month(u64_4);
    let mut weekday_2: date::Weekday = crate::date::Weekday::Wednesday;
    let mut monthdeltaweek_1: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_3, weekid: u64_2, day: weekday_1};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_307() {
//    rusty_monitor::set_test_id(307);
    let mut u64_0: u64 = 10u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_0};
    let mut u64_1: u64 = 8758u64;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_2: u64 = 5u64;
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_2);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_3: u64 = 9878u64;
    let mut u64_4: u64 = 161u64;
    let mut u64_5: u64 = 273u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_5, u64_4, u64_3);
    let mut i64_0: i64 = 100i64;
    let mut str_0: &str = "Qdd9uMSC5ItzNzwy6m";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_6: u64 = 6u64;
    let mut u64_7: u64 = 90u64;
    let mut duration_1: date::Duration = crate::date::Duration::Year(u64_7);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_6, string_0, i64_0, simpledate_0, option_1, option_0, vec_0);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Thursday;
    let mut weekday_1: date::Weekday = crate::date::Weekday::Friday;
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut str_1: &str = crate::expense::Expense::description(expense_0_ref_0);
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_1};
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut weekday_2: date::Weekday = crate::date::Weekday::Saturday;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2283() {
//    rusty_monitor::set_test_id(2283);
    let mut u64_0: u64 = 1u64;
    let mut u64_1: u64 = 596u64;
    let mut u64_2: u64 = 100u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_2, u64_1, u64_0);
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut u64_3: u64 = 1478u64;
    let mut u64_4: u64 = 24u64;
    let mut u64_5: u64 = 6059u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_5, u64_4, u64_3);
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut vec_0: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut u64_6: u64 = 2u64;
    let mut vec_1: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_7: u64 = 59u64;
    let mut u64_8: u64 = 59u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_6, month: u64_8, day: u64_7};
    let mut str_0: &str = "every {} year";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut vec_2: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_9: u64 = 5742u64;
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_9, days: vec_2};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnDate(monthdeltadate_0);
    let mut expense_slice_0: &[crate::expense::Expense] = crate::data::Datafile::expenses_between(datafile_0_ref_0, simpledate_1_ref_0, simpledate_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3276() {
//    rusty_monitor::set_test_id(3276);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut u64_0: u64 = 9197u64;
    let mut u64_1: u64 = 23u64;
    let mut u64_2: u64 = 66u64;
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_3: u64 = 30u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_1, u64_0, u64_2);
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_2: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_1_ref_0: &mut crate::data::Datafile = &mut datafile_1;
    let mut result_0: std::result::Result<(), std::boxed::Box<dyn std::error::Error>> = crate::data::Datafile::remove(datafile_0_ref_0, u64_3);
    let mut repetition_0: crate::date::Repetition = std::option::Option::unwrap(option_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8473() {
//    rusty_monitor::set_test_id(8473);
    let mut u64_0: u64 = 11u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_0};
    let mut result_0: std::result::Result<crate::date::DayDelta, std::boxed::Box<dyn std::error::Error>> = std::result::Result::Ok(daydelta_0);
    let mut u64_1: u64 = 8098u64;
    let mut u64_2: u64 = 273u64;
    let mut u64_3: u64 = 334u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_3, month: u64_2, day: u64_1};
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut u64_4: u64 = 59u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_4};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_5: u64 = 2u64;
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Count(u64_5);
    let mut u64_6: u64 = 8042u64;
    let mut daydelta_1: crate::date::DayDelta = crate::date::DayDelta {nth: u64_6};
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Day(daydelta_1);
    let mut repetition_1: crate::date::Repetition = crate::date::Repetition {delta: repdelta_1, end: repend_1};
    let mut option_1: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_1);
    let mut u64_7: u64 = 212u64;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_7);
    let mut option_2: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_8: u64 = 90u64;
    let mut u64_9: u64 = 80u64;
    let mut u64_10: u64 = 334u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_10, u64_9, u64_8);
    let mut i64_0: i64 = 9223372036854775807i64;
    let mut str_0: &str = "icHFKAs";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_11: u64 = 243u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_11, string_0, i64_0, simpledate_1, option_2, option_0, vec_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_618() {
//    rusty_monitor::set_test_id(618);
    let mut str_0: &str = "ToYSiorhhQ";
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut u64_0: u64 = 304u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_0);
    let mut u64_1: u64 = 8370u64;
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_1);
    let mut option_0: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_2: u64 = 9927u64;
    let mut u64_3: u64 = 151u64;
    let mut u64_4: u64 = 238u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_4, u64_3, u64_2);
    let mut u64_5: u64 = 6u64;
    let mut u64_6: u64 = 400u64;
    let mut u64_7: u64 = 4u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_5, u64_7, u64_6);
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut datafile_2: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_1_ref_0: &mut crate::data::Datafile = &mut datafile_1;
    let mut vec_0: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut u64_8: u64 = 28u64;
    crate::data::Datafile::add_tag(datafile_1_ref_0, string_0);
    let mut option_1: std::option::Option<&crate::expense::Expense> = crate::data::Datafile::find(datafile_0_ref_0, u64_8);
    let mut vec_0_ref_0: &mut std::vec::Vec<crate::expense::Expense> = &mut vec_0;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_451() {
//    rusty_monitor::set_test_id(451);
    let mut u64_0: u64 = 212u64;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_1: u64 = 5003u64;
    let mut u64_2: u64 = 5475u64;
    let mut u64_3: u64 = 93u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_3, u64_2, u64_1);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut vec_1: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_4: u64 = 2800u64;
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_4, days: vec_1};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnDate(monthdeltadate_0);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_5: u64 = 9424u64;
    let mut u64_6: u64 = 0u64;
    let mut u64_7: u64 = 29u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_7, month: u64_6, day: u64_5};
    let mut i64_0: i64 = 100i64;
    let mut str_0: &str = r"(\d+)";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_8: u64 = 5u64;
    let mut u64_9: u64 = 9504u64;
    let mut u64_10: u64 = 212u64;
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Count(u64_10);
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_9);
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_8, string_0, i64_0, simpledate_1, option_1, option_0, vec_0);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut bool_0: bool = crate::expense::Expense::compare_id(expense_0_ref_0, u64_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_128() {
//    rusty_monitor::set_test_id(128);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Tuesday;
    let mut u64_0: u64 = 9373u64;
    let mut u64_1: u64 = 365u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_1, weekid: u64_0, day: weekday_0};
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut vec_1: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_2: u64 = 1u64;
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_2, days: vec_1};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnDate(monthdeltadate_0);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_3: u64 = 11u64;
    let mut u64_4: u64 = 8u64;
    let mut u64_5: u64 = 4415u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_5, month: u64_4, day: u64_3};
    let mut i64_0: i64 = 12524i64;
    let mut str_0: &str = "the ID of the expense or income to show";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_6: u64 = 7999u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_6, string_0, i64_0, simpledate_0, option_1, option_0, vec_0);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Never;
    let mut u64_7: u64 = 2u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_7};
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut repetition_1: crate::date::Repetition = crate::date::Repetition {delta: repdelta_1, end: repend_1};
    let mut option_2: &std::option::Option<crate::date::SimpleDate> = crate::expense::Expense::get_end_date(expense_0_ref_0);
    let mut monthdelta_1: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4551() {
//    rusty_monitor::set_test_id(4551);
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_0: u64 = 2u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_0);
    let mut u64_1: u64 = 8042u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_1};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_2: u64 = 212u64;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_2);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_3: u64 = 90u64;
    let mut u64_4: u64 = 80u64;
    let mut u64_5: u64 = 334u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_5, u64_4, u64_3);
    let mut i64_0: i64 = 9223372036854775807i64;
    let mut str_0: &str = "{} weeks";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_6: u64 = 243u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_6, string_0, i64_0, simpledate_0, option_1, option_0, vec_0);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut vec_1: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Never;
    let mut u64_7: u64 = 23u64;
    let mut u64_8: u64 = 4u64;
    let mut u64_9: u64 = 12u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_9, month: u64_8, day: u64_7};
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    crate::data::Datafile::insert(datafile_0_ref_0, expense_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2464() {
//    rusty_monitor::set_test_id(2464);
    let mut u64_0: u64 = 1u64;
    let mut u64_1: u64 = 596u64;
    let mut u64_2: u64 = 100u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_2, u64_1, u64_0);
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut u64_3: u64 = 1478u64;
    let mut u64_4: u64 = 24u64;
    let mut u64_5: u64 = 6059u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_5, u64_4, u64_3);
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut vec_0: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut vec_1: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut vec_2: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_6: u64 = 59u64;
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_6, days: vec_2};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnDate(monthdeltadate_0);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_7: u64 = 59u64;
    let mut u64_8: u64 = 59u64;
    let mut u64_9: u64 = 7066u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_9, month: u64_8, day: u64_7};
    let mut i64_0: i64 = -6822i64;
    let mut str_0: &str = "every {} year";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_10: u64 = 90u64;
    let mut vec_3: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_11: u64 = 5742u64;
    let mut monthdeltadate_1: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_11, days: vec_3};
    let mut monthdelta_1: date::MonthDelta = crate::date::MonthDelta::OnDate(monthdeltadate_1);
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_10, string_0, i64_0, simpledate_2, option_1, option_0, vec_1);
    let mut expense_slice_0: &[crate::expense::Expense] = crate::data::Datafile::expenses_between(datafile_0_ref_0, simpledate_1_ref_0, simpledate_0_ref_0);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut simpledate_3: &crate::date::SimpleDate = crate::expense::Expense::get_start_date(expense_0_ref_0);
//    panic!("From RustyUnit with love");
}
}