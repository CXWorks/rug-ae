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
#[timeout(30000)]fn rusty_test_8637() {
//    rusty_monitor::set_test_id(8637);
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_0: u64 = 7505u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_0};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut u64_1: u64 = 7899u64;
    let mut u64_2: u64 = 6091u64;
    let mut u64_3: u64 = 6950u64;
    let mut u64_4: u64 = 304u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_4, u64_3, u64_2);
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut result_0: std::result::Result<(), std::boxed::Box<dyn std::error::Error>> = crate::data::Datafile::remove(datafile_0_ref_0, u64_1);
    let mut tuple_0: () = std::result::Result::unwrap(result_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3061() {
//    rusty_monitor::set_test_id(3061);
    let mut u64_0: u64 = 4u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_0};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut option_0: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_1: u64 = 1420u64;
    let mut u64_2: u64 = 100u64;
    let mut u64_3: u64 = 7u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_3, u64_2, u64_1);
    let mut u64_4: u64 = 9u64;
    let mut u64_5: u64 = 2168u64;
    let mut u64_6: u64 = 304u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_6, u64_5, u64_4);
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut u64_7: u64 = 59u64;
    let mut u64_8: u64 = 1582u64;
    let mut u64_9: u64 = 9240u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_9, month: u64_8, day: u64_7};
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Sunday;
    let mut expense_slice_0: &[crate::expense::Expense] = crate::data::Datafile::expenses_between(datafile_0_ref_0, simpledate_0_ref_0, simpledate_1_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_170() {
//    rusty_monitor::set_test_id(170);
    let mut u64_0: u64 = 151u64;
    let mut u64_1: u64 = 7610u64;
    let mut u64_2: u64 = 212u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_2, u64_1, u64_0);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Wednesday;
    let mut u64_3: u64 = 5720u64;
    let mut u64_4: u64 = 334u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_4, weekid: u64_3, day: weekday_0};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_5: u64 = 23u64;
    let mut u64_6: u64 = 243u64;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_6);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_7: u64 = 1441u64;
    let mut u64_8: u64 = 0u64;
    let mut u64_9: u64 = 1u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_9, u64_8, u64_7);
    let mut i64_0: i64 = 0i64;
    let mut u64_10: u64 = 181u64;
    let mut u64_11: u64 = 28u64;
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Count(u64_11);
    let mut u64_12: u64 = 1126u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_12};
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut repetition_1: crate::date::Repetition = crate::date::Repetition {delta: repdelta_1, end: repend_1};
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_1);
    let mut u64_13: u64 = 59u64;
    let mut duration_1: date::Duration = crate::date::Duration::Month(u64_13);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
    let mut u64_14: u64 = 120u64;
    let mut u64_15: u64 = 10u64;
    let mut u64_16: u64 = 1u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_16, month: u64_15, day: u64_14};
    let mut option_4: std::option::Option<crate::date::SimpleDate> = std::option::Option::Some(simpledate_2);
    let mut u64_17: u64 = 8u64;
    let mut u64_18: u64 = 273u64;
    let mut u64_19: u64 = 151u64;
    let mut simpledate_3: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_19, month: u64_18, day: u64_17};
    let mut i64_1: i64 = 0i64;
    let mut u64_20: u64 = 9726u64;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_5: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_21: u64 = 1264u64;
    let mut duration_2: date::Duration = crate::date::Duration::Day(u64_21);
    let mut option_6: std::option::Option<date::Duration> = std::option::Option::Some(duration_2);
    let mut u64_22: u64 = 243u64;
    let mut u64_23: u64 = 273u64;
    let mut u64_24: u64 = 29u64;
    let mut simpledate_4: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_24, month: u64_23, day: u64_22};
    let mut i64_2: i64 = 9223372036854775807i64;
    let mut str_0: &str = "hcfk5Q";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_25: u64 = 151u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_25, string_0, i64_2, simpledate_4, option_6, option_5, vec_0);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut i64_3: i64 = crate::expense::Expense::amount(expense_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_357() {
//    rusty_monitor::set_test_id(357);
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_0: u64 = 6u64;
    let mut u64_1: u64 = 1156u64;
    let mut u64_2: u64 = 334u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_2, u64_1, u64_0);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Wednesday;
    let mut u64_3: u64 = 400u64;
    let mut u64_4: u64 = 400u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_4, weekid: u64_3, day: weekday_0};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_5: u64 = 2577u64;
    let mut duration_0: date::Duration = crate::date::Duration::Week(u64_5);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_6: u64 = 3u64;
    let mut u64_7: u64 = 21u64;
    let mut u64_8: u64 = 1824u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_8, month: u64_7, day: u64_6};
    let mut i64_0: i64 = 0i64;
    let mut str_0: &str = "{} months";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_9: u64 = 9904u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_9, string_0, i64_0, simpledate_1, option_1, option_0, vec_0);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut vec_1: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut weekday_1: date::Weekday = crate::date::Weekday::Tuesday;
    crate::data::Datafile::insert(datafile_0_ref_0, expense_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8470() {
//    rusty_monitor::set_test_id(8470);
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_0: u64 = 4838u64;
    let mut u64_1: u64 = 4256u64;
    let mut u64_2: u64 = 8u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_2, u64_1, u64_0);
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_3: u64 = 59u64;
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_3);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut option_4: std::option::Option<crate::date::SimpleDate> = std::option::Option::None;
    let mut u64_4: u64 = 10u64;
    let mut u64_5: u64 = 7u64;
    let mut u64_6: u64 = 4824u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_6, month: u64_5, day: u64_4};
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_5: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_6: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_7: u64 = 273u64;
    let mut u64_8: u64 = 7u64;
    let mut u64_9: u64 = 59u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_9, month: u64_8, day: u64_7};
    let mut i64_0: i64 = 100i64;
    let mut str_0: &str = "KLhk6IM9ebg7hqrj";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_10: u64 = 2578u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_10, string_0, i64_0, simpledate_2, option_6, option_5, vec_0);
    let mut vec_1: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_11: u64 = 100u64;
    let mut u64_12: u64 = 7u64;
    let mut u64_13: u64 = 7906u64;
    let mut simpledate_3: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_13, month: u64_12, day: u64_11};
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_3);
    let mut vec_2: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_14: u64 = 212u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_14, on: vec_2};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Week(weekdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_7: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_15: u64 = 6u64;
    let mut duration_1: date::Duration = crate::date::Duration::Day(u64_15);
    let mut str_1: &str = "id";
    let mut u64_16: u64 = 9u64;
    let mut u64_17: u64 = 1836u64;
    let mut u64_18: u64 = 12u64;
    let mut simpledate_4: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_18, u64_17, u64_16);
    let mut u64_19: u64 = 875u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_19};
    let mut u64_20: u64 = 10u64;
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Count(u64_20);
    let mut u64_21: u64 = 1717u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_21};
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut u64_22: u64 = 2u64;
    let mut yeardelta_1: crate::date::YearDelta = crate::date::YearDelta {nth: u64_22};
    let mut u64_23: u64 = 151u64;
    let mut u64_24: u64 = 6u64;
    let mut u64_25: u64 = 2u64;
    let mut u64_26: u64 = 6409u64;
    let mut simpledate_5: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_26, u64_25, u64_24);
    let mut option_8: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
    let mut u64_27: u64 = 243u64;
    let mut u64_28: u64 = 12u64;
    let mut u64_29: u64 = 22u64;
    let mut simpledate_6: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_28, u64_27, u64_23);
    let mut i64_1: i64 = 0i64;
    let mut string_1: std::string::String = std::string::String::from(str_1);
    let mut expense_1: crate::expense::Expense = crate::expense::Expense::new(u64_29, string_1, i64_1, simpledate_5, option_8, option_7, vec_1);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_191() {
//    rusty_monitor::set_test_id(191);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut u64_0: u64 = 9739u64;
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_1_ref_0: &mut crate::data::Datafile = &mut datafile_1;
    let mut datafile_2: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_2_ref_0: &mut crate::data::Datafile = &mut datafile_2;
    let mut datafile_3: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_3_ref_0: &mut crate::data::Datafile = &mut datafile_3;
    let mut datafile_4: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_4_ref_0: &mut crate::data::Datafile = &mut datafile_4;
    let mut datafile_5: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_5_ref_0: &mut crate::data::Datafile = &mut datafile_5;
    let mut datafile_6: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_6_ref_0: &mut crate::data::Datafile = &mut datafile_6;
    let mut u64_1: u64 = 2660u64;
    let mut datafile_7: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_7_ref_0: &mut crate::data::Datafile = &mut datafile_7;
    let mut u64_2: u64 = 304u64;
    let mut datafile_8: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_8_ref_0: &mut crate::data::Datafile = &mut datafile_8;
    let mut u64_3: u64 = 5241u64;
    let mut datafile_9: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_9_ref_0: &mut crate::data::Datafile = &mut datafile_9;
    let mut u64_4: u64 = 0u64;
    let mut str_0: &str = "MonthDeltaWeek";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut datafile_10: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_10_ref_0: &mut crate::data::Datafile = &mut datafile_10;
    let mut vec_0: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut u64_5: u64 = 365u64;
    crate::data::Datafile::add_tag(datafile_10_ref_0, string_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_243() {
//    rusty_monitor::set_test_id(243);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8630() {
//    rusty_monitor::set_test_id(8630);
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_0: u64 = 1270u64;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_1: u64 = 120u64;
    let mut u64_2: u64 = 9948u64;
    let mut u64_3: u64 = 304u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_3, u64_2, u64_1);
    let mut i64_0: i64 = 0i64;
    let mut str_0: &str = "{} day";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_4: u64 = 334u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_4, string_0, i64_0, simpledate_0, option_1, option_0, vec_0);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut vec_1: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_5: u64 = 22u64;
    let mut duration_1: date::Duration = crate::date::Duration::Year(u64_5);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
    let mut u64_6: u64 = 243u64;
    let mut u64_7: u64 = 12u64;
    let mut u64_8: u64 = 22u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_8, u64_7, u64_6);
    let mut i64_1: i64 = 0i64;
    let mut str_1: &str = "show";
    let mut string_1: std::string::String = std::string::String::from(str_1);
    let mut u64_9: u64 = 11u64;
    let mut expense_1: crate::expense::Expense = crate::expense::Expense::new(u64_9, string_1, i64_1, simpledate_1, option_3, option_2, vec_1);
    let mut expense_1_ref_0: &crate::expense::Expense = &mut expense_1;
    let mut ordering_0: std::cmp::Ordering = crate::expense::Expense::compare_dates(expense_1_ref_0, expense_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_360() {
//    rusty_monitor::set_test_id(360);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut u64_0: u64 = 12u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_0};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_1: u64 = 4383u64;
    let mut duration_0: date::Duration = crate::date::Duration::Year(u64_1);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_2: u64 = 11u64;
    let mut u64_3: u64 = 212u64;
    let mut u64_4: u64 = 5910u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_4, month: u64_3, day: u64_2};
    let mut u64_5: u64 = 9696u64;
    let mut u64_6: u64 = 151u64;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut str_0: &str = "et";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_7: u64 = 273u64;
    let mut duration_1: date::Duration = crate::date::Duration::Month(u64_7);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
    let mut u64_8: u64 = 28u64;
    let mut u64_9: u64 = 9793u64;
    let mut u64_10: u64 = 29u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_10, month: u64_9, day: u64_8};
    let mut i64_0: i64 = 100i64;
    let mut str_1: &str = "kKW8xQmrMH90G";
    let mut string_0: std::string::String = std::string::String::from(str_1);
    let mut u64_11: u64 = 365u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_11, string_0, i64_0, simpledate_1, option_3, option_2, vec_0);
    let mut expense_0_ref_0: &mut crate::expense::Expense = &mut expense_0;
    crate::expense::Expense::remove_tags(expense_0_ref_0, str_0_ref_0);
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_1_ref_0: &crate::data::Datafile = &mut datafile_1;
    let mut option_4: std::option::Option<&crate::expense::Expense> = crate::data::Datafile::find(datafile_1_ref_0, u64_6);
    let mut duration_2: date::Duration = crate::date::Duration::Year(u64_5);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3533() {
//    rusty_monitor::set_test_id(3533);
    let mut u64_0: u64 = 5328u64;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut vec_0: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut vec_1: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_1: u64 = 1270u64;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_1);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut str_0: &str = "{} day";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut vec_2: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_2: u64 = 22u64;
    let mut duration_1: date::Duration = crate::date::Duration::Year(u64_2);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
    let mut str_1: &str = "show";
    let mut string_1: std::string::String = std::string::String::from(str_1);
    let mut option_4: std::option::Option<&crate::expense::Expense> = crate::data::Datafile::find(datafile_0_ref_0, u64_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2707() {
//    rusty_monitor::set_test_id(2707);
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_0: u64 = 1270u64;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_1: u64 = 120u64;
    let mut u64_2: u64 = 9948u64;
    let mut u64_3: u64 = 304u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_3, u64_2, u64_1);
    let mut i64_0: i64 = 0i64;
    let mut str_0: &str = "{} day";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_4: u64 = 334u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_4, string_0, i64_0, simpledate_0, option_1, option_0, vec_0);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut vec_1: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_5: u64 = 22u64;
    let mut duration_1: date::Duration = crate::date::Duration::Year(u64_5);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
//    panic!("From RustyUnit with love");
}
}