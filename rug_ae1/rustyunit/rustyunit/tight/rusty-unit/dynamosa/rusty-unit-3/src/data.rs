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
fn rusty_test_99() {
    rusty_monitor::set_test_id(99);
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_0: u64 = 7653u64;
    let mut duration_0: date::Duration = crate::date::Duration::Year(u64_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_1: u64 = 8233u64;
    let mut u64_2: u64 = 4396u64;
    let mut u64_3: u64 = 9051u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_3, u64_2, u64_1);
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_4: u64 = 337u64;
    let mut u64_5: u64 = 5378u64;
    let mut u64_6: u64 = 5573u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_6, u64_5, u64_4);
    let mut option_4: std::option::Option<crate::date::SimpleDate> = std::option::Option::Some(simpledate_1);
    let mut u64_7: u64 = 8010u64;
    let mut u64_8: u64 = 9612u64;
    let mut u64_9: u64 = 3575u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_9, month: u64_8, day: u64_7};
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_5: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_6: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_10: u64 = 1406u64;
    let mut u64_11: u64 = 9944u64;
    let mut u64_12: u64 = 8342u64;
    let mut simpledate_3: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_12, month: u64_11, day: u64_10};
    let mut i64_0: i64 = -3661i64;
    let mut str_0: &str = "";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_13: u64 = 3550u64;
    let mut u64_14: u64 = 2700u64;
    let mut vec_1: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Sunday;
    let mut weekday_1: date::Weekday = crate::date::Weekday::Thursday;
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Count(u64_14);
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_13, string_0, i64_0, simpledate_3, option_6, option_5, vec_0);
    let mut weekday_2: date::Weekday = crate::date::Weekday::Saturday;
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut str_1: &str = crate::expense::Expense::description(expense_0_ref_0);
    let mut vec_1_ref_0: &mut std::vec::Vec<crate::expense::Expense> = &mut vec_1;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7036() {
    rusty_monitor::set_test_id(7036);
    let mut u64_0: u64 = 5042u64;
    let mut u64_1: u64 = 9170u64;
    let mut u64_2: u64 = 1146u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_2, u64_1, u64_0);
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut u64_3: u64 = 3950u64;
    let mut u64_4: u64 = 9048u64;
    let mut u64_5: u64 = 6852u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_5, u64_4, u64_3);
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut vec_0: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut u64_6: u64 = 6805u64;
    let mut vec_1: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_7: u64 = 464u64;
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_7, days: vec_1};
    let mut vec_2: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut u64_8: u64 = 332u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_8};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut u64_9: u64 = 7458u64;
    let mut u64_10: u64 = 6389u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_6, month: u64_10, day: u64_9};
    let mut str_0: &str = "Xja8rFY05f0";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Monday;
    let mut expense_slice_0: &[crate::expense::Expense] = crate::data::Datafile::expenses_between(datafile_0_ref_0, simpledate_1_ref_0, simpledate_0_ref_0);
    let mut vec_2_ref_0: &mut std::vec::Vec<std::string::String> = &mut vec_2;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_132() {
    rusty_monitor::set_test_id(132);
    let mut str_0: &str = "tR";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut str_1: &str = "0L";
    let mut u64_0: u64 = 5383u64;
    let mut u64_0_ref_0: &u64 = &mut u64_0;
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut str_1_ref_0: &str = &mut str_1;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3916() {
    rusty_monitor::set_test_id(3916);
    let mut u64_0: u64 = 2337u64;
    let mut str_0: &str = "GrnZ2Z06a1";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_1: u64 = 5833u64;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_2: u64 = 1445u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_2);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Monday;
    let mut u64_3: u64 = 7775u64;
    let mut u64_4: u64 = 5301u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_4, weekid: u64_3, day: weekday_0};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_5: u64 = 8634u64;
    let mut u64_6: u64 = 2807u64;
    let mut u64_7: u64 = 7875u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_7, month: u64_6, day: u64_5};
    let mut i64_0: i64 = 2571i64;
    let mut str_1: &str = "";
    let mut string_1: std::string::String = std::string::String::from(str_1);
    let mut u64_8: u64 = 7960u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_8, string_1, i64_0, simpledate_0, option_1, option_0, vec_0);
    let mut u64_9: u64 = 6886u64;
    let mut duration_0: date::Duration = crate::date::Duration::Year(u64_9);
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Never;
    let mut vec_1: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_10: u64 = 9075u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_10, on: vec_1};
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Week(weekdelta_0);
    let mut repetition_1: crate::date::Repetition = crate::date::Repetition {delta: repdelta_1, end: repend_1};
    let mut repend_2: date::RepEnd = crate::date::RepEnd::Never;
    let mut vec_2: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_11: u64 = 1327u64;
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_11, days: vec_2};
    let mut monthdelta_1: date::MonthDelta = crate::date::MonthDelta::OnDate(monthdeltadate_0);
    let mut repdelta_2: date::RepDelta = crate::date::RepDelta::Month(monthdelta_1);
    let mut u64_12: u64 = 1830u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_12};
    let mut u64_13: u64 = 8753u64;
    let mut u64_14: u64 = 2632u64;
    let mut u64_15: u64 = 268u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_15, month: u64_14, day: u64_13};
    let mut str_2: &str = "k71D35PYjXJ";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "X";
    let mut vec_3: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut u64_16: u64 = 3140u64;
    let mut u64_17: u64 = 4231u64;
    let mut u64_18: u64 = 3292u64;
    let mut u64_19: u64 = 6350u64;
    let mut vec_4: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_20: u64 = 641u64;
    let mut u64_21: u64 = 5292u64;
    let mut u64_22: u64 = 5391u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_19, u64_17, u64_16);
    let mut repend_3: date::RepEnd = crate::date::RepEnd::Date(simpledate_1);
    let mut u64_23: u64 = 8335u64;
    let mut yeardelta_1: crate::date::YearDelta = crate::date::YearDelta {nth: u64_18};
    let mut repdelta_3: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut repetition_2: crate::date::Repetition = crate::date::Repetition {delta: repdelta_2, end: repend_2};
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_1);
    let mut duration_1: date::Duration = crate::date::Duration::Day(u64_20);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_24: u64 = 1616u64;
    let mut u64_25: u64 = 5489u64;
    let mut simpledate_3: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_23, u64_21, u64_22);
    let mut i64_1: i64 = 3614i64;
    let mut string_2: std::string::String = std::string::String::from(str_3);
    let mut expense_1: crate::expense::Expense = crate::expense::Expense::new(u64_25, string_2, i64_1, simpledate_2, option_3, option_2, vec_4);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut simpledate_4: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_0, u64_1, u64_24);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8329() {
    rusty_monitor::set_test_id(8329);
    let mut u64_0: u64 = 5077u64;
    let mut duration_0: date::Duration = crate::date::Duration::Week(u64_0);
    let mut u64_1: u64 = 6648u64;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut u64_2: u64 = 4231u64;
    let mut u64_3: u64 = 3292u64;
    let mut u64_4: u64 = 6350u64;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_5: u64 = 641u64;
    let mut u64_6: u64 = 5292u64;
    let mut u64_7: u64 = 5391u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_7, u64_6, u64_5);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut u64_8: u64 = 8335u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_8};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_9: u64 = 3258u64;
    let mut duration_1: date::Duration = crate::date::Duration::Day(u64_9);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_10: u64 = 1667u64;
    let mut u64_11: u64 = 1616u64;
    let mut u64_12: u64 = 5489u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_12, u64_11, u64_10);
    let mut i64_0: i64 = 3614i64;
    let mut str_0: &str = "4ITKfzok5de";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_13: u64 = 9264u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_13, string_0, i64_0, simpledate_1, option_1, option_0, vec_0);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut option_2: &std::option::Option<crate::date::SimpleDate> = crate::expense::Expense::get_end_date(expense_0_ref_0);
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_4, u64_3, u64_2);
    let mut result_0: std::result::Result<(), std::boxed::Box<dyn std::error::Error>> = crate::data::Datafile::remove(datafile_0_ref_0, u64_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_44() {
    rusty_monitor::set_test_id(44);
    let mut u64_0: u64 = 780u64;
    let mut u64_1: u64 = 2030u64;
    let mut u64_2: u64 = 6313u64;
    let mut u64_3: u64 = 9953u64;
    let mut u64_4: u64 = 2859u64;
    let mut u64_5: u64 = 4816u64;
    let mut u64_6: u64 = 3566u64;
    let mut str_0: &str = "5Uug";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_7: u64 = 1086u64;
    let mut u64_8: u64 = 1468u64;
    let mut u64_9: u64 = 551u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_9, month: u64_8, day: u64_7};
    let mut i64_0: i64 = -19901i64;
    let mut u64_10: u64 = 4057u64;
    let mut str_1: &str = "9SqtooPocnHrXfbff";
    let mut string_0: std::string::String = std::string::String::from(str_1);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut vec_0: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut u64_11: u64 = 2404u64;
    crate::data::Datafile::add_tag(datafile_0_ref_0, string_0);
    let mut duration_0: date::Duration = crate::date::Duration::Year(u64_5);
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_4, month: u64_3, day: u64_2};
    let mut duration_1: date::Duration = crate::date::Duration::Month(u64_1);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Sunday;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6368() {
    rusty_monitor::set_test_id(6368);
    let mut u64_0: u64 = 5400u64;
    let mut u64_1: u64 = 2587u64;
    let mut u64_2: u64 = 400u64;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut vec_0: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut str_0: &str = "X";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u64_3: u64 = 199u64;
    let mut u64_3_ref_0: &u64 = &mut u64_3;
    let mut u64_4: u64 = 6632u64;
    let mut u64_4_ref_0: &u64 = &mut u64_4;
    let mut u64_5: u64 = 6991u64;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_5);
    let mut u64_6: u64 = 1164u64;
    let mut u64_7: u64 = 2396u64;
    let mut u64_8: u64 = 942u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_8, u64_7, u64_6);
    let mut str_1: &str = "RAxx18r5MXprnC";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut u64_9: u64 = 2987u64;
    let mut u64_10: u64 = 1304u64;
    let mut u64_11: u64 = 9858u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_11, month: u64_10, day: u64_9};
    let mut option_0: std::option::Option<&crate::expense::Expense> = crate::data::Datafile::find(datafile_0_ref_0, u64_2);
    let mut duration_1: date::Duration = crate::date::Duration::Month(u64_1);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1104() {
    rusty_monitor::set_test_id(1104);
    let mut u64_0: u64 = 6648u64;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut vec_0: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut u64_1: u64 = 3140u64;
    let mut u64_2: u64 = 4231u64;
    let mut u64_3: u64 = 3292u64;
    let mut u64_4: u64 = 6350u64;
    let mut vec_1: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_5: u64 = 641u64;
    let mut u64_6: u64 = 5292u64;
    let mut u64_7: u64 = 5391u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_7, u64_6, u64_5);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_1);
    let mut u64_8: u64 = 8335u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_8};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_9: u64 = 3258u64;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_9);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_10: u64 = 1667u64;
    let mut u64_11: u64 = 1616u64;
    let mut u64_12: u64 = 5489u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_12, u64_11, u64_10);
    let mut i64_0: i64 = 3614i64;
    let mut str_0: &str = "4ITKfzok5de";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_13: u64 = 9264u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_13, string_0, i64_0, simpledate_1, option_1, option_0, vec_1);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut option_2: &std::option::Option<crate::date::SimpleDate> = crate::expense::Expense::get_end_date(expense_0_ref_0);
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_4, u64_3, u64_2);
    let mut result_0: std::result::Result<(), std::boxed::Box<dyn std::error::Error>> = crate::data::Datafile::remove(datafile_0_ref_0, u64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7759() {
    rusty_monitor::set_test_id(7759);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut vec_0: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut u64_0: u64 = 6805u64;
    let mut vec_1: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_1: u64 = 464u64;
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_1, days: vec_1};
    let mut vec_2: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut u64_2: u64 = 2363u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_2};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_3: u64 = 7544u64;
    let mut duration_0: date::Duration = crate::date::Duration::Year(u64_3);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_4: u64 = 7458u64;
    let mut u64_5: u64 = 6389u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_0, month: u64_5, day: u64_4};
    let mut i64_0: i64 = -12739i64;
    let mut str_0: &str = "Xja8rFY05f0";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_6: u64 = 1705u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_6, string_0, i64_0, simpledate_0, option_1, option_0, vec_2);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Monday;
    let mut simpledate_1: &crate::date::SimpleDate = crate::expense::Expense::get_start_date(expense_0_ref_0);
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnDate(monthdeltadate_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8340() {
    rusty_monitor::set_test_id(8340);
    let mut vec_0: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_0: u64 = 6097u64;
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_0, days: vec_0};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnDate(monthdeltadate_0);
    let mut vec_1: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_1: u64 = 6669u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_1);
    let mut vec_2: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_2: u64 = 6068u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_2, on: vec_2};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_3: u64 = 5668u64;
    let mut u64_4: u64 = 8688u64;
    let mut u64_5: u64 = 3668u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_5, month: u64_4, day: u64_3};
    let mut i64_0: i64 = -1913i64;
    let mut str_0: &str = "42Bl2aWLYq";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_6: u64 = 6365u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_6, string_0, i64_0, simpledate_0, option_1, option_0, vec_1);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut vec_3: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_7: u64 = 8926u64;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_7);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_8: u64 = 4285u64;
    let mut u64_9: u64 = 4383u64;
    let mut u64_10: u64 = 6836u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_10, month: u64_9, day: u64_8};
    let mut i64_1: i64 = 9655i64;
    let mut str_1: &str = "oyKkC0ecb";
    let mut string_1: std::string::String = std::string::String::from(str_1);
    let mut u64_11: u64 = 9313u64;
    let mut expense_1: crate::expense::Expense = crate::expense::Expense::new(u64_11, string_1, i64_1, simpledate_1, option_3, option_2, vec_3);
    let mut expense_1_ref_0: &crate::expense::Expense = &mut expense_1;
    let mut u64_12: u64 = 137u64;
    let mut duration_1: date::Duration = crate::date::Duration::Month(u64_12);
    let mut duration_1_ref_0: &date::Duration = &mut duration_1;
    let mut u64_13: u64 = 404u64;
    let mut u64_14: u64 = 2982u64;
    let mut u64_15: u64 = 2531u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_15, month: u64_14, day: u64_13};
    let mut simpledate_2_ref_0: &crate::date::SimpleDate = &mut simpledate_2;
    let mut ordering_0: std::cmp::Ordering = crate::expense::Expense::compare_dates(expense_1_ref_0, expense_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2028() {
    rusty_monitor::set_test_id(2028);
    let mut u64_0: u64 = 6648u64;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut vec_0: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut u64_1: u64 = 3140u64;
    let mut u64_2: u64 = 4231u64;
    let mut u64_3: u64 = 3292u64;
    let mut u64_4: u64 = 6350u64;
    let mut vec_1: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_5: u64 = 8335u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_5};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_1);
    let mut option_0: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_6: u64 = 1667u64;
    let mut u64_7: u64 = 1616u64;
    let mut u64_8: u64 = 5489u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_8, u64_7, u64_6);
    let mut str_0: &str = "4ITKfzok5de";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_4, u64_3, u64_2);
    let mut result_0: std::result::Result<(), std::boxed::Box<dyn std::error::Error>> = crate::data::Datafile::remove(datafile_0_ref_0, u64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6677() {
    rusty_monitor::set_test_id(6677);
    let mut u64_0: u64 = 7246u64;
    let mut u64_1: u64 = 6648u64;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut vec_0: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut u64_2: u64 = 3140u64;
    let mut u64_3: u64 = 4231u64;
    let mut u64_4: u64 = 3292u64;
    let mut vec_1: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_5: u64 = 641u64;
    let mut u64_6: u64 = 5391u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_0, u64_6, u64_5);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut u64_7: u64 = 8335u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_7};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_8: u64 = 3258u64;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_8);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_9: u64 = 1667u64;
    let mut u64_10: u64 = 1616u64;
    let mut u64_11: u64 = 5489u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_11, u64_10, u64_9);
    let mut i64_0: i64 = 3614i64;
    let mut str_0: &str = "4ITKfzok5de";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_12: u64 = 9264u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_12, string_0, i64_0, simpledate_1, option_1, option_0, vec_1);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut option_2: &std::option::Option<crate::date::SimpleDate> = crate::expense::Expense::get_end_date(expense_0_ref_0);
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_2, u64_4, u64_3);
    let mut result_0: std::result::Result<(), std::boxed::Box<dyn std::error::Error>> = crate::data::Datafile::remove(datafile_0_ref_0, u64_1);
    panic!("From RustyUnit with love");
}
}