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
fn rusty_test_2315() {
    rusty_monitor::set_test_id(2315);
    let mut u64_0: u64 = 4394u64;
    let mut u64_1: u64 = 81u64;
    let mut u64_2: u64 = 457u64;
    let mut u64_3: u64 = 8134u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_3, u64_2, u64_1);
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut u64_4: u64 = 4889u64;
    let mut u64_5: u64 = 2818u64;
    let mut u64_6: u64 = 1732u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_6, u64_5, u64_4);
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut u64_7: u64 = 9380u64;
    let mut u64_8: u64 = 3611u64;
    let mut u64_9: u64 = 3597u64;
    let mut bool_0: bool = true;
    let mut u64_10: u64 = 3274u64;
    let mut u64_11: u64 = 2767u64;
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_1_ref_0: &mut crate::data::Datafile = &mut datafile_1;
    let mut u64_12: u64 = 433u64;
    let mut u64_13: u64 = 3134u64;
    let mut datafile_2: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_2_ref_0: &crate::data::Datafile = &mut datafile_2;
    let mut option_0: std::option::Option<&crate::expense::Expense> = crate::data::Datafile::find(datafile_2_ref_0, u64_13);
    let mut expense_0: &crate::expense::Expense = std::option::Option::unwrap(option_0);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Friday;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_9};
    let mut expense_slice_0: &[crate::expense::Expense] = crate::data::Datafile::expenses_between(datafile_0_ref_0, simpledate_1_ref_0, simpledate_0_ref_0);
    let mut daydelta_1: crate::date::DayDelta = crate::date::DayDelta {nth: u64_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2723() {
    rusty_monitor::set_test_id(2723);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_0: u64 = 2618u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_0);
    let mut u64_1: u64 = 4121u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_1};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_2: u64 = 6277u64;
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_2);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_3: u64 = 1810u64;
    let mut u64_4: u64 = 4056u64;
    let mut u64_5: u64 = 3973u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_5, u64_4, u64_3);
    let mut i64_0: i64 = -12795i64;
    let mut str_0: &str = "n9i15iXaTTkE7eOB";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_6: u64 = 7185u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_6, string_0, i64_0, simpledate_0, option_1, option_0, vec_0);
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_1_ref_0: &mut crate::data::Datafile = &mut datafile_1;
    let mut bool_0: bool = true;
    let mut u64_7: u64 = 377u64;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Monday;
    let mut u64_8: u64 = 2185u64;
    let mut u64_9: u64 = 3210u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_9, weekid: u64_8, day: weekday_0};
    crate::data::Datafile::insert(datafile_1_ref_0, expense_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3846() {
    rusty_monitor::set_test_id(3846);
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_0: u64 = 3548u64;
    let mut u64_1: u64 = 9682u64;
    let mut u64_2: u64 = 7556u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_2, month: u64_1, day: u64_0};
    let mut i64_0: i64 = 2577i64;
    let mut u64_3: u64 = 3396u64;
    let mut str_0: &str = "P";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut vec_0: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut u64_4: u64 = 8702u64;
    let mut u64_5: u64 = 7145u64;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Monday;
    let mut u64_6: u64 = 2518u64;
    let mut u64_7: u64 = 5092u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_7, weekid: u64_6, day: weekday_0};
    let mut u64_8: u64 = 1039u64;
    let mut duration_0: date::Duration = crate::date::Duration::Year(u64_8);
    let mut duration_0_ref_0: &date::Duration = &mut duration_0;
    let mut u64_9: u64 = 4036u64;
    let mut u64_10: u64 = 4155u64;
    let mut u64_11: u64 = 1807u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_11, month: u64_10, day: u64_9};
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_5};
    crate::data::Datafile::add_tag(datafile_0_ref_0, string_0);
    let mut weekday_1: date::Weekday = crate::date::Weekday::Wednesday;
    let mut vec_0_ref_0: &mut std::vec::Vec<crate::expense::Expense> = &mut vec_0;
    let mut weekday_2: date::Weekday = crate::date::Weekday::Friday;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2795() {
    rusty_monitor::set_test_id(2795);
    let mut u64_0: u64 = 118u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_0};
    let mut vec_0: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_1: u64 = 8756u64;
    let mut u64_2: u64 = 6406u64;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Saturday;
    let mut u64_3: u64 = 6737u64;
    let mut u64_4: u64 = 8804u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_4, weekid: u64_3, day: weekday_0};
    let mut u64_5: u64 = 9983u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_5};
    let mut vec_1: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_6: u64 = 5332u64;
    let mut u64_7: u64 = 8701u64;
    let mut u64_8: u64 = 9232u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_8, u64_7, u64_6);
    let mut i64_0: i64 = -9197i64;
    let mut str_0: &str = "2eLUfYZc";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_9: u64 = 9553u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_9, string_0, i64_0, simpledate_0, option_1, option_0, vec_1);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut option_2: &std::option::Option<crate::date::SimpleDate> = crate::expense::Expense::get_end_date(expense_0_ref_0);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_2);
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_1, on: vec_0};
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut weekday_1: date::Weekday = crate::date::Weekday::Monday;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4510() {
    rusty_monitor::set_test_id(4510);
    let mut u64_0: u64 = 6600u64;
    let mut u64_1: u64 = 4036u64;
    let mut u64_2: u64 = 790u64;
    let mut u64_3: u64 = 253u64;
    let mut duration_0: date::Duration = crate::date::Duration::Year(u64_3);
    let mut duration_0_ref_0: &date::Duration = &mut duration_0;
    let mut u64_4: u64 = 488u64;
    let mut u64_5: u64 = 1770u64;
    let mut u64_6: u64 = 3385u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_6, u64_5, u64_4);
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut u64_7: u64 = 9995u64;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut u64_8: u64 = 1905u64;
    let mut u64_9: u64 = 5968u64;
    let mut u64_10: u64 = 2779u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_10, month: u64_9, day: u64_8};
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut u64_11: u64 = 3362u64;
    let mut u64_12: u64 = 2380u64;
    let mut u64_13: u64 = 2529u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_13, month: u64_12, day: u64_11};
    let mut simpledate_2_ref_0: &crate::date::SimpleDate = &mut simpledate_2;
    let mut u64_14: u64 = 3706u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_14};
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_1_ref_0: &crate::data::Datafile = &mut datafile_1;
    let mut expense_slice_0: &[crate::expense::Expense] = crate::data::Datafile::expenses_between(datafile_1_ref_0, simpledate_2_ref_0, simpledate_1_ref_0);
    let mut result_0: std::result::Result<(), std::boxed::Box<dyn std::error::Error>> = crate::data::Datafile::remove(datafile_0_ref_0, u64_7);
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1100() {
    rusty_monitor::set_test_id(1100);
    let mut u64_0: u64 = 353u64;
    let mut u64_1: u64 = 7266u64;
    let mut u64_2: u64 = 6848u64;
    let mut u64_3: u64 = 4204u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_3, u64_2, u64_1);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut u64_4: u64 = 7485u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_4};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut u64_5: u64 = 8846u64;
    let mut u64_6: u64 = 5687u64;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_7: u64 = 2076u64;
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_7);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_8: u64 = 1995u64;
    let mut u64_9: u64 = 1988u64;
    let mut u64_10: u64 = 5549u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_10, month: u64_9, day: u64_8};
    let mut i64_0: i64 = -2418i64;
    let mut str_0: &str = "behsM0vUGZgPdVRC";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_11: u64 = 1095u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_11, string_0, i64_0, simpledate_1, option_1, option_0, vec_0);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut vec_1: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut u64_12: u64 = 1203u64;
    crate::data::Datafile::insert(datafile_0_ref_0, expense_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    panic!("From RustyUnit with love");
}
}