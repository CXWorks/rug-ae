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
#[timeout(30000)]fn rusty_test_7919() {
//    rusty_monitor::set_test_id(7919);
    let mut u64_0: u64 = 6u64;
    let mut u64_1: u64 = 1700u64;
    let mut u64_2: u64 = 273u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_2, u64_1, u64_0);
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut u64_3: u64 = 273u64;
    let mut u64_4: u64 = 1700u64;
    let mut u64_5: u64 = 5u64;
    let mut u64_6: u64 = 1700u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_6, u64_5, u64_4);
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut u64_7: u64 = 2004u64;
    let mut u64_8: u64 = 12u64;
    let mut u64_9: u64 = 5300u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_9, month: u64_8, day: u64_7};
    let mut simpledate_2_ref_0: &crate::date::SimpleDate = &mut simpledate_2;
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_1_ref_0: &crate::data::Datafile = &mut datafile_1;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Wednesday;
    let mut u64_10: u64 = 8u64;
    let mut datafile_2: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_2_ref_0: &mut crate::data::Datafile = &mut datafile_2;
    let mut weekday_1: date::Weekday = crate::date::Weekday::Tuesday;
    let mut result_0: std::result::Result<(), std::boxed::Box<dyn std::error::Error>> = crate::data::Datafile::remove(datafile_2_ref_0, u64_10);
    let mut tuple_0: () = std::result::Result::unwrap(result_0);
    let mut expense_slice_0: &[crate::expense::Expense] = crate::data::Datafile::expenses_between(datafile_1_ref_0, simpledate_2_ref_0, simpledate_1_ref_0);
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_3);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_124() {
//    rusty_monitor::set_test_id(124);
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_0: u64 = 181u64;
    let mut u64_1: u64 = 7402u64;
    let mut u64_2: u64 = 151u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_2, month: u64_1, day: u64_0};
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut u64_3: u64 = 4182u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_3};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_4: u64 = 273u64;
    let mut u64_5: u64 = 8477u64;
    let mut u64_6: u64 = 2u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_6, month: u64_5, day: u64_4};
    let mut i64_0: i64 = 3445i64;
    let mut str_0: &str = "Sunday";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_7: u64 = 3u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_7, string_0, i64_0, simpledate_1, option_1, option_0, vec_0);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut u64_8: u64 = 9563u64;
    let mut u64_9: u64 = 12u64;
    let mut u64_10: u64 = 3473u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_10, month: u64_9, day: u64_8};
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Date(simpledate_2);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Friday;
    let mut u64_11: u64 = 30u64;
    let mut u64_12: u64 = 100u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_12, weekid: u64_11, day: weekday_0};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut u64_13: u64 = 23u64;
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_13);
    let mut repetition_1: crate::date::Repetition = crate::date::Repetition {delta: repdelta_1, end: repend_1};
    let mut str_1: &str = crate::expense::Expense::description(expense_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4281() {
//    rusty_monitor::set_test_id(4281);
    let mut u64_0: u64 = 31u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_0);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Thursday;
    let mut u64_1: u64 = 2090u64;
    let mut u64_2: u64 = 4620u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_2, weekid: u64_1, day: weekday_0};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut u64_3: u64 = 29u64;
    let mut u64_4: u64 = 3279u64;
    let mut u64_5: u64 = 59u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_5, u64_4, u64_3);
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut u64_6: u64 = 212u64;
    let mut u64_7: u64 = 4544u64;
    let mut u64_8: u64 = 4754u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_8, u64_7, u64_6);
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_9: u64 = 2u64;
    let mut u64_10: u64 = 2763u64;
    let mut u64_11: u64 = 3866u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_11, u64_10, u64_9);
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Date(simpledate_2);
    let mut u64_12: u64 = 1700u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_12};
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_13: u64 = 7687u64;
    let mut u64_14: u64 = 9825u64;
    let mut u64_15: u64 = 9272u64;
    let mut simpledate_3: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_15, u64_14, u64_13);
    let mut i64_0: i64 = 9223372036854775807i64;
    let mut str_0: &str = "repetition";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_16: u64 = 2293u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_16, string_0, i64_0, simpledate_3, option_1, option_0, vec_0);
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_1_ref_0: &mut crate::data::Datafile = &mut datafile_1;
    let mut vec_1: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    crate::data::Datafile::insert(datafile_1_ref_0, expense_0);
    let mut expense_slice_0: &[crate::expense::Expense] = crate::data::Datafile::expenses_between(datafile_0_ref_0, simpledate_1_ref_0, simpledate_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_78() {
//    rusty_monitor::set_test_id(78);
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_0: u64 = 30u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_0);
    let mut vec_1: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_1: u64 = 6145u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_1, on: vec_1};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Week(weekdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_2: u64 = 2512u64;
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_2);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_3: u64 = 3949u64;
    let mut u64_4: u64 = 21u64;
    let mut u64_5: u64 = 29u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_5, month: u64_4, day: u64_3};
    let mut i64_0: i64 = 100i64;
    let mut str_0: &str = "the tag to add";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_6: u64 = 30u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_6, string_0, i64_0, simpledate_0, option_1, option_0, vec_0);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut vec_2: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_7: u64 = 273u64;
    let mut duration_1: date::Duration = crate::date::Duration::Week(u64_7);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
    let mut u64_8: u64 = 6u64;
    let mut u64_9: u64 = 90u64;
    let mut u64_10: u64 = 971u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_10, month: u64_9, day: u64_8};
    let mut i64_1: i64 = 0i64;
    let mut str_1: &str = "every {} month";
    let mut string_1: std::string::String = std::string::String::from(str_1);
    let mut u64_11: u64 = 4u64;
    let mut expense_1: crate::expense::Expense = crate::expense::Expense::new(u64_11, string_1, i64_1, simpledate_1, option_3, option_2, vec_2);
    let mut expense_1_ref_0: &crate::expense::Expense = &mut expense_1;
    let mut ordering_0: std::cmp::Ordering = crate::expense::Expense::compare_dates(expense_1_ref_0, expense_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8774() {
//    rusty_monitor::set_test_id(8774);
    let mut u64_0: u64 = 29u64;
    let mut u64_1: u64 = 3279u64;
    let mut u64_2: u64 = 59u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_2, u64_1, u64_0);
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut u64_3: u64 = 212u64;
    let mut u64_4: u64 = 4544u64;
    let mut u64_5: u64 = 4754u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_5, u64_4, u64_3);
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_6: u64 = 2763u64;
    let mut u64_7: u64 = 1700u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_7};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut option_0: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_8: u64 = 7687u64;
    let mut u64_9: u64 = 9825u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_6, u64_9, u64_8);
    let mut str_0: &str = "repetition";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_1_ref_0: &mut crate::data::Datafile = &mut datafile_1;
    let mut vec_1: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut expense_slice_0: &[crate::expense::Expense] = crate::data::Datafile::expenses_between(datafile_0_ref_0, simpledate_1_ref_0, simpledate_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5998() {
//    rusty_monitor::set_test_id(5998);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_0: u64 = 1700u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_0};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut option_0: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_1: u64 = 7687u64;
    let mut u64_2: u64 = 9825u64;
    let mut u64_3: u64 = 9272u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_3, u64_2, u64_1);
    let mut str_0: &str = "repetition";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_1_ref_0: &mut crate::data::Datafile = &mut datafile_1;
    let mut vec_1: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7819() {
//    rusty_monitor::set_test_id(7819);
    let mut u64_0: u64 = 242u64;
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_0);
    let mut duration_0_ref_0: &date::Duration = &mut duration_0;
    let mut u64_1: u64 = 365u64;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_2: u64 = 396u64;
    let mut duration_1: date::Duration = crate::date::Duration::Year(u64_2);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
    let mut u64_3: u64 = 28u64;
    let mut u64_4: u64 = 12u64;
    let mut u64_5: u64 = 23u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_5, u64_4, u64_3);
    let mut i64_0: i64 = 100i64;
    let mut str_0: &str = "{} months";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_6: u64 = 28u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_6, string_0, i64_0, simpledate_0, option_1, option_0, vec_0);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut simpledate_1: &crate::date::SimpleDate = crate::expense::Expense::get_start_date(expense_0_ref_0);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Tuesday;
    let mut option_2: std::option::Option<&crate::expense::Expense> = crate::data::Datafile::find(datafile_0_ref_0, u64_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_643() {
//    rusty_monitor::set_test_id(643);
    let mut u64_0: u64 = 3169u64;
    let mut u64_1: u64 = 5u64;
    let mut u64_2: u64 = 585u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_2, month: u64_1, day: u64_0};
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut u64_3: u64 = 8439u64;
    let mut str_0: &str = "second";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut vec_1: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_4: u64 = 12u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_4, on: vec_1};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Week(weekdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_5: u64 = 1474u64;
    let mut u64_6: u64 = 151u64;
    let mut u64_7: u64 = 3612u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_7, month: u64_6, day: u64_5};
    let mut i64_0: i64 = 0i64;
    let mut str_1: &str = "the ID of the expense to remove";
    let mut string_0: std::string::String = std::string::String::from(str_1);
    let mut u64_8: u64 = 22u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_8, string_0, i64_0, simpledate_1, option_1, option_0, vec_0);
    let mut expense_0_ref_0: &mut crate::expense::Expense = &mut expense_0;
    crate::expense::Expense::remove_tags(expense_0_ref_0, str_0_ref_0);
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_3};
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5376() {
//    rusty_monitor::set_test_id(5376);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_0: u64 = 2u64;
    let mut u64_1: u64 = 2763u64;
    let mut u64_2: u64 = 3866u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_2, u64_1, u64_0);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut u64_3: u64 = 1700u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_3};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_4: u64 = 7687u64;
    let mut u64_5: u64 = 9825u64;
    let mut u64_6: u64 = 9272u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_6, u64_5, u64_4);
    let mut i64_0: i64 = 9223372036854775807i64;
    let mut str_0: &str = "repetition";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_7: u64 = 2293u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_7, string_0, i64_0, simpledate_1, option_1, option_0, vec_0);
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_1_ref_0: &mut crate::data::Datafile = &mut datafile_1;
    let mut vec_1: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    crate::data::Datafile::insert(datafile_1_ref_0, expense_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_155() {
//    rusty_monitor::set_test_id(155);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut u64_0: u64 = 59u64;
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_1_ref_0: &mut crate::data::Datafile = &mut datafile_1;
    let mut datafile_2: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_2_ref_0: &mut crate::data::Datafile = &mut datafile_2;
    let mut u64_1: u64 = 8778u64;
    let mut datafile_3: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_3_ref_0: &mut crate::data::Datafile = &mut datafile_3;
    let mut u64_2: u64 = 181u64;
    let mut datafile_4: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_4_ref_0: &mut crate::data::Datafile = &mut datafile_4;
    let mut datafile_5: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_5_ref_0: &mut crate::data::Datafile = &mut datafile_5;
    let mut u64_3: u64 = 774u64;
    let mut datafile_6: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_6_ref_0: &mut crate::data::Datafile = &mut datafile_6;
    let mut datafile_7: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_7_ref_0: &mut crate::data::Datafile = &mut datafile_7;
    let mut datafile_8: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_8_ref_0: &mut crate::data::Datafile = &mut datafile_8;
    let mut u64_4: u64 = 9699u64;
    let mut datafile_9: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_9_ref_0: &mut crate::data::Datafile = &mut datafile_9;
    let mut u64_5: u64 = 243u64;
    let mut str_0: &str = "";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut datafile_10: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_10_ref_0: &mut crate::data::Datafile = &mut datafile_10;
    crate::data::Datafile::add_tag(datafile_10_ref_0, string_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7238() {
//    rusty_monitor::set_test_id(7238);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_0: u64 = 23u64;
    let mut duration_0: date::Duration = crate::date::Duration::Week(u64_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_1: u64 = 7809u64;
    let mut u64_2: u64 = 8594u64;
    let mut u64_3: u64 = 90u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_3, u64_2, u64_1);
    let mut str_0: &str = "5yzNStoDOs";
    let mut str_1: &str = "KgdtoipDmpPv";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_2: &str = "invalid end date";
    let mut str_3: &str = "3NeTqG6";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut u64_4: u64 = 12u64;
    let mut u64_5: u64 = 212u64;
    let mut u64_6: u64 = 90u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_6, month: u64_5, day: u64_4};
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_4: &str = "rd";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut u64_7: u64 = 8u64;
    let mut u64_7_ref_0: &u64 = &mut u64_7;
    let mut u64_8: u64 = 1u64;
    let mut str_5: &str = "name";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut u64_9: u64 = 400u64;
    let mut u64_10: u64 = 277u64;
    let mut u64_11: u64 = 334u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_11, u64_10, u64_9);
    let mut simpledate_2_ref_0: &crate::date::SimpleDate = &mut simpledate_2;
    let mut i64_0: i64 = 9223372036854775807i64;
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_8, string_0, i64_0, simpledate_0, option_1, option_0, vec_0);
    let mut datafile_2: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_2_ref_0: &mut crate::data::Datafile = &mut datafile_2;
    let mut vec_1: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    crate::data::Datafile::insert(datafile_2_ref_0, expense_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2795() {
//    rusty_monitor::set_test_id(2795);
    let mut u64_0: u64 = 151u64;
    let mut u64_1: u64 = 100u64;
    let mut u64_2: u64 = 7332u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_2, u64_1, u64_0);
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut u64_3: u64 = 365u64;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_4: u64 = 23u64;
    let mut duration_0: date::Duration = crate::date::Duration::Year(u64_4);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_5: u64 = 28u64;
    let mut u64_6: u64 = 12u64;
    let mut u64_7: u64 = 5539u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_7, u64_6, u64_5);
    let mut vec_1: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_8: u64 = 3408u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_8, on: vec_1};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Week(weekdelta_0);
    let mut option_2: std::option::Option<&crate::expense::Expense> = crate::data::Datafile::find(datafile_0_ref_0, u64_3);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2662() {
//    rusty_monitor::set_test_id(2662);
    let mut u64_0: u64 = 9031u64;
    let mut u64_1: u64 = 5305u64;
    let mut u64_2: u64 = 9183u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_2, month: u64_1, day: u64_0};
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Wednesday;
    let mut u64_3: u64 = 400u64;
    let mut u64_4: u64 = 6799u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_4, weekid: u64_3, day: weekday_0};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_5: u64 = 4742u64;
    let mut u64_6: u64 = 365u64;
    let mut u64_7: u64 = 4504u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_7, month: u64_6, day: u64_5};
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_8: u64 = 8222u64;
    let mut u64_9: u64 = 4032u64;
    let mut u64_10: u64 = 8842u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_10, u64_9, u64_8);
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Date(simpledate_2);
    let mut vec_1: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_11: u64 = 7u64;
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_11, days: vec_1};
    let mut monthdelta_1: date::MonthDelta = crate::date::MonthDelta::OnDate(monthdeltadate_0);
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Month(monthdelta_1);
    let mut repetition_1: crate::date::Repetition = crate::date::Repetition {delta: repdelta_1, end: repend_1};
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_1);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_12: u64 = 400u64;
    let mut u64_13: u64 = 243u64;
    let mut u64_14: u64 = 2001u64;
    let mut simpledate_3: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_14, month: u64_13, day: u64_12};
    let mut i64_0: i64 = 1760i64;
    let mut str_0: &str = "nd";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_15: u64 = 12u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_15, string_0, i64_0, simpledate_3, option_3, option_2, vec_0);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut vec_2: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_16: u64 = 9u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_16, on: vec_2};
    let mut vec_3: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_17: u64 = 7239u64;
    let mut str_1: &str = "look for expenses and incomes with descriptions or tags matching this regular expression";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut repend_2: date::RepEnd = crate::date::RepEnd::Never;
    let mut u64_18: u64 = 120u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_18};
    let mut repdelta_2: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut repetition_2: crate::date::Repetition = crate::date::Repetition {delta: repdelta_2, end: repend_2};
    let mut option_4: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_2);
    let mut option_5: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_19: u64 = 172u64;
    let mut u64_20: u64 = 181u64;
    let mut u64_21: u64 = 9789u64;
    let mut simpledate_4: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_21, month: u64_20, day: u64_19};
    let mut str_2: &str = "parse::u64";
    let mut string_1: std::string::String = std::string::String::from(str_2);
    let mut vec_4: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_22: u64 = 5u64;
    let mut weekdelta_1: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_22, on: vec_4};
    let mut repdelta_3: date::RepDelta = crate::date::RepDelta::Week(weekdelta_1);
    let mut u64_23: u64 = 3408u64;
    let mut weekdelta_2: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_17, on: vec_3};
    let mut repdelta_4: date::RepDelta = crate::date::RepDelta::Week(weekdelta_0);
    let mut simpledate_5: &crate::date::SimpleDate = crate::expense::Expense::get_start_date(expense_0_ref_0);
    let mut weekday_1: date::Weekday = crate::date::Weekday::Tuesday;
    let mut option_6: std::option::Option<&crate::expense::Expense> = crate::data::Datafile::find(datafile_0_ref_0, u64_23);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_9251() {
//    rusty_monitor::set_test_id(9251);
    let mut vec_0: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut vec_1: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut vec_2: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_0: u64 = 22u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_0, on: vec_2};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Week(weekdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_1: u64 = 4u64;
    let mut u64_2: u64 = 21u64;
    let mut u64_3: u64 = 0u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_3, month: u64_2, day: u64_1};
    let mut i64_0: i64 = 0i64;
    let mut str_0: &str = "{} days";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_4: u64 = 8320u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_4, string_0, i64_0, simpledate_0, option_1, option_0, vec_1);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut vec_3: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_5: u64 = 8119u64;
    let mut u64_6: u64 = 3756u64;
    let mut u64_7: u64 = 7u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_7, u64_6, u64_5);
    let mut i64_1: i64 = 100i64;
    let mut str_1: &str = "nth";
    let mut string_1: std::string::String = std::string::String::from(str_1);
    let mut u64_8: u64 = 181u64;
    let mut expense_1: crate::expense::Expense = crate::expense::Expense::new(u64_8, string_1, i64_1, simpledate_1, option_3, option_2, vec_3);
    let mut expense_1_ref_0: &crate::expense::Expense = &mut expense_1;
    let mut vec_4: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_9: u64 = 243u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_9};
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut option_4: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_10: u64 = 23u64;
    let mut u64_11: u64 = 7u64;
    let mut u64_12: u64 = 120u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_12, u64_11, u64_10);
    let mut str_2: &str = "QUnD2LH";
    let mut string_2: std::string::String = std::string::String::from(str_2);
    let mut ordering_0: std::cmp::Ordering = crate::expense::Expense::compare_dates(expense_1_ref_0, expense_0_ref_0);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2944() {
//    rusty_monitor::set_test_id(2944);
    let mut u64_0: u64 = 6099u64;
    let mut u64_1: u64 = 11u64;
    let mut u64_2: u64 = 1700u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_2, month: u64_1, day: u64_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_3: u64 = 9703u64;
    let mut u64_4: u64 = 21u64;
    let mut u64_5: u64 = 1576u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_5, month: u64_4, day: u64_3};
    let mut u64_6: u64 = 8u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut u64_7: u64 = 3u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_7};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_8: u64 = 0u64;
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_8);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut option_4: std::option::Option<date::Duration> = std::option::Option::None;
    let mut option_4_ref_0: &std::option::Option<date::Duration> = &mut option_4;
    let mut u64_9: u64 = 59u64;
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Count(u64_9);
    let mut u64_10: u64 = 4u64;
    let mut daydelta_1: crate::date::DayDelta = crate::date::DayDelta {nth: u64_10};
    let mut u64_11: u64 = 90u64;
    let mut u64_12: u64 = 1700u64;
    let mut u64_13: u64 = 3973u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_13, u64_12, u64_11);
    let mut simpledate_2_ref_0: &crate::date::SimpleDate = &mut simpledate_2;
    let mut u64_14: u64 = 10u64;
    let mut u64_15: u64 = 59u64;
    let mut simpledate_3: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_6, u64_15, u64_14);
    let mut u64_16: u64 = 365u64;
    let mut u64_17: u64 = 5269u64;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_18: u64 = 31u64;
    let mut repend_2: date::RepEnd = crate::date::RepEnd::Count(u64_18);
    let mut vec_1: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_16, on: vec_1};
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Week(weekdelta_0);
    let mut repetition_1: crate::date::Repetition = crate::date::Repetition {delta: repdelta_1, end: repend_2};
    let mut u64_19: u64 = 120u64;
    let mut u64_20: u64 = 529u64;
    let mut u64_21: u64 = 120u64;
    let mut simpledate_4: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_21, month: u64_20, day: u64_19};
    let mut i64_0: i64 = 0i64;
    let mut str_0: &str = "wuU5XrxwKt";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_22: u64 = 6883u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_22, string_0, i64_0, simpledate_4, option_3, option_2, vec_0);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut u64_23: u64 = 4432u64;
    let mut u64_24: u64 = 334u64;
    let mut u64_25: u64 = 719u64;
    let mut simpledate_5: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_25, month: u64_24, day: u64_23};
    let mut simpledate_5_ref_0: &crate::date::SimpleDate = &mut simpledate_5;
    let mut i64_1: i64 = crate::expense::Expense::amount(expense_0_ref_0);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Friday;
    let mut duration_1: date::Duration = crate::date::Duration::Week(u64_17);
    let mut repend_3: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut duration_2: date::Duration = std::option::Option::unwrap(option_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_381() {
//    rusty_monitor::set_test_id(381);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Tuesday;
    let mut u64_0: u64 = 9100u64;
    let mut u64_1: u64 = 3054u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_1, weekid: u64_0, day: weekday_0};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    let mut u64_2: u64 = 29u64;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_3: u64 = 100u64;
    let mut duration_0: date::Duration = crate::date::Duration::Year(u64_3);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_4: u64 = 7923u64;
    let mut u64_5: u64 = 3u64;
    let mut u64_6: u64 = 6267u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_6, u64_5, u64_4);
    let mut i64_0: i64 = 9223372036854775807i64;
    let mut str_0: &str = "";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_7: u64 = 6614u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_7, string_0, i64_0, simpledate_0, option_1, option_0, vec_0);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut u64_8: u64 = 100u64;
    let mut duration_1: date::Duration = crate::date::Duration::Day(u64_8);
    let mut weekday_1: date::Weekday = crate::date::Weekday::Sunday;
    let mut str_1: &str = crate::expense::Expense::description(expense_0_ref_0);
    let mut option_2: std::option::Option<&crate::expense::Expense> = crate::data::Datafile::find(datafile_0_ref_0, u64_2);
    let mut expense_1: &crate::expense::Expense = std::option::Option::unwrap(option_2);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
//    panic!("From RustyUnit with love");
}
}