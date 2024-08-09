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
fn rusty_test_952() {
    rusty_monitor::set_test_id(952);
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_0: u64 = 7763u64;
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_1: u64 = 4358u64;
    let mut u64_2: u64 = 3356u64;
    let mut u64_3: u64 = 7631u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_3, month: u64_2, day: u64_1};
    let mut u64_4: u64 = 2069u64;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_5: u64 = 4873u64;
    let mut u64_6: u64 = 8164u64;
    let mut u64_7: u64 = 7893u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_7, month: u64_6, day: u64_5};
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_1);
    let mut u64_8: u64 = 2131u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_8};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_9: u64 = 2240u64;
    let mut duration_1: date::Duration = crate::date::Duration::Week(u64_9);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
    let mut u64_10: u64 = 3379u64;
    let mut u64_11: u64 = 5721u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_4, month: u64_11, day: u64_10};
    let mut i64_0: i64 = 10119i64;
    let mut str_0: &str = "";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_12: u64 = 8260u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_12, string_0, i64_0, simpledate_2, option_3, option_2, vec_0);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut vec_1: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Never;
    crate::data::Datafile::insert(datafile_0_ref_0, expense_0);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Wednesday;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6617() {
    rusty_monitor::set_test_id(6617);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut u64_0: u64 = 5634u64;
    let mut u64_1: u64 = 5131u64;
    let mut u64_2: u64 = 4688u64;
    let mut u64_3: u64 = 8531u64;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut vec_1: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_4: u64 = 2467u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_4, on: vec_1};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Week(weekdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_5: u64 = 4205u64;
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_5);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_6: u64 = 2618u64;
    let mut u64_7: u64 = 9308u64;
    let mut u64_8: u64 = 2928u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_8, u64_7, u64_6);
    let mut i64_0: i64 = -1774i64;
    let mut str_0: &str = "XEK";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_9: u64 = 5557u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_9, string_0, i64_0, simpledate_0, option_1, option_0, vec_0);
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_1_ref_0: &mut crate::data::Datafile = &mut datafile_1;
    crate::data::Datafile::insert(datafile_1_ref_0, expense_0);
    let mut duration_1: date::Duration = crate::date::Duration::Month(u64_3);
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_2, month: u64_1, day: u64_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_16() {
    rusty_monitor::set_test_id(16);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut u64_0: u64 = 2408u64;
    let mut u64_1: u64 = 9985u64;
    let mut u64_2: u64 = 2550u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_2);
    let mut u64_3: u64 = 1167u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_3};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_4: u64 = 5524u64;
    let mut u64_5: u64 = 2352u64;
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_5);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_6: u64 = 5919u64;
    let mut u64_7: u64 = 7404u64;
    let mut u64_8: u64 = 7549u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_8, month: u64_7, day: u64_6};
    let mut i64_0: i64 = -15461i64;
    let mut u64_9: u64 = 5140u64;
    let mut u64_10: u64 = 7046u64;
    let mut u64_11: u64 = 4464u64;
    let mut str_0: &str = "uZMLlzgaLowy2wVq";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_1_ref_0: &mut crate::data::Datafile = &mut datafile_1;
    crate::data::Datafile::add_tag(datafile_1_ref_0, string_0);
    let mut duration_1: date::Duration = crate::date::Duration::Month(u64_11);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Sunday;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8() {
    rusty_monitor::set_test_id(8);
    let mut str_0: &str = "FRFmjmky9FzL";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u64_0: u64 = 3139u64;
    let mut u64_1: u64 = 8590u64;
    let mut u64_2: u64 = 2042u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_2, month: u64_1, day: u64_0};
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut u64_3: u64 = 4754u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_3};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_4: u64 = 9815u64;
    let mut u64_5: u64 = 8130u64;
    let mut u64_6: u64 = 4990u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_6, month: u64_5, day: u64_4};
    let mut i64_0: i64 = 15288i64;
    let mut u64_7: u64 = 6100u64;
    let mut u64_8: u64 = 2307u64;
    let mut u64_9: u64 = 4560u64;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_10: u64 = 2704u64;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_10);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_11: u64 = 2643u64;
    let mut u64_12: u64 = 8550u64;
    let mut u64_13: u64 = 9694u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_13, u64_12, u64_11);
    let mut i64_1: i64 = 8557i64;
    let mut str_1: &str = "";
    let mut string_0: std::string::String = std::string::String::from(str_1);
    let mut u64_14: u64 = 7470u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_14, string_0, i64_1, simpledate_2, option_3, option_2, vec_0);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut u64_15: u64 = 6113u64;
    let mut duration_1: date::Duration = crate::date::Duration::Week(u64_15);
    let mut str_2: &str = crate::expense::Expense::description(expense_0_ref_0);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_9};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5344() {
    rusty_monitor::set_test_id(5344);
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_0: u64 = 4464u64;
    let mut duration_0: date::Duration = crate::date::Duration::Year(u64_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_1: u64 = 8585u64;
    let mut u64_2: u64 = 7064u64;
    let mut u64_3: u64 = 4933u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_3, u64_2, u64_1);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut u64_4: u64 = 9476u64;
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut str_0: &str = "k";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u64_5: u64 = 2557u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_5);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Sunday;
    let mut u64_6: u64 = 7886u64;
    let mut u64_7: u64 = 63u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_7, weekid: u64_6, day: weekday_0};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_8: u64 = 5592u64;
    let mut duration_1: date::Duration = crate::date::Duration::Day(u64_8);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
    let mut u64_9: u64 = 3178u64;
    let mut u64_10: u64 = 6494u64;
    let mut u64_11: u64 = 8180u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_11, u64_10, u64_9);
    let mut i64_0: i64 = 19841i64;
    let mut u64_12: u64 = 1132u64;
    let mut str_1: &str = "q1Y8QiZBa";
    let mut string_0: std::string::String = std::string::String::from(str_1);
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_12, string_0, i64_0, simpledate_1, option_3, option_2, vec_0);
    let mut datafile_2: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_1_ref_0: &mut crate::data::Datafile = &mut datafile_1;
    let mut result_0: std::result::Result<(), std::boxed::Box<dyn std::error::Error>> = crate::data::Datafile::remove(datafile_0_ref_0, u64_4);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut vec_1: &std::vec::Vec<std::string::String> = crate::expense::Expense::tags(expense_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_9() {
    rusty_monitor::set_test_id(9);
    let mut u64_0: u64 = 3236u64;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_1: u64 = 2394u64;
    let mut u64_2: u64 = 6291u64;
    let mut u64_3: u64 = 1462u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_3, month: u64_2, day: u64_1};
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut u64_4: u64 = 2920u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_4};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_5: u64 = 5536u64;
    let mut u64_6: u64 = 8633u64;
    let mut u64_7: u64 = 6126u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_7, month: u64_6, day: u64_5};
    let mut i64_0: i64 = -4898i64;
    let mut str_0: &str = "wkOCsE";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_8: u64 = 6006u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_8, string_0, i64_0, simpledate_1, option_1, option_0, vec_0);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut u64_9: u64 = 329u64;
    let mut u64_10: u64 = 6884u64;
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Count(u64_10);
    let mut vec_1: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_11: u64 = 6089u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_11, on: vec_1};
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Week(weekdelta_0);
    let mut repetition_1: crate::date::Repetition = crate::date::Repetition {delta: repdelta_1, end: repend_1};
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_9);
    let mut bool_0: bool = crate::expense::Expense::compare_id(expense_0_ref_0, u64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4046() {
    rusty_monitor::set_test_id(4046);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Thursday;
    let mut u64_0: u64 = 8926u64;
    let mut u64_1: u64 = 7819u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_1, weekid: u64_0, day: weekday_0};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    let mut u64_2: u64 = 7433u64;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut u64_3: u64 = 4161u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_3};
    let mut vec_0: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_4: u64 = 9904u64;
    let mut u64_5: u64 = 8937u64;
    let mut u64_6: u64 = 1366u64;
    let mut vec_1: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut vec_2: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_7: u64 = 3163u64;
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_7, days: vec_2};
    let mut monthdelta_1: date::MonthDelta = crate::date::MonthDelta::OnDate(monthdeltadate_0);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_1);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_8: u64 = 4180u64;
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_8);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_9: u64 = 6739u64;
    let mut u64_10: u64 = 7051u64;
    let mut u64_11: u64 = 7901u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_11, month: u64_10, day: u64_9};
    let mut i64_0: i64 = 2572i64;
    let mut str_0: &str = "k4Wn11UwtAk6niie";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_12: u64 = 7024u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_12, string_0, i64_0, simpledate_0, option_1, option_0, vec_1);
    let mut str_1: &str = "LXqh6ndmuoT";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "X";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "Yj0RQgLCOgPYZjmB";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut duration_1: date::Duration = crate::date::Duration::Month(u64_6);
    let mut duration_2: date::Duration = crate::date::Duration::Day(u64_5);
    let mut monthdeltadate_1: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_4, days: vec_0};
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut option_2: std::option::Option<&crate::expense::Expense> = crate::data::Datafile::find(datafile_0_ref_0, u64_2);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut simpledate_1: &crate::date::SimpleDate = crate::expense::Expense::get_start_date(expense_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3770() {
    rusty_monitor::set_test_id(3770);
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_0: u64 = 4689u64;
    let mut u64_1: u64 = 7953u64;
    let mut u64_2: u64 = 9217u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_2, u64_1, u64_0);
    let mut u64_3: u64 = 8224u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_3);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut u64_4: u64 = 7317u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_4};
    let mut u64_5: u64 = 9899u64;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut vec_1: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_6: u64 = 3163u64;
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_6, days: vec_1};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnDate(monthdeltadate_0);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_7: u64 = 4180u64;
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_7);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_8: u64 = 6739u64;
    let mut u64_9: u64 = 7051u64;
    let mut u64_10: u64 = 7901u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_10, month: u64_9, day: u64_8};
    let mut i64_0: i64 = 2572i64;
    let mut str_0: &str = "k4Wn11UwtAk6niie";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_11: u64 = 7024u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_11, string_0, i64_0, simpledate_1, option_3, option_2, vec_0);
    let mut daydelta_1: crate::date::DayDelta = crate::date::DayDelta {nth: u64_5};
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut u64_12: u64 = 7307u64;
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut vec_2: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut result_0: std::result::Result<(), std::boxed::Box<dyn std::error::Error>> = crate::data::Datafile::remove(datafile_0_ref_0, u64_12);
    let mut datafile_2: crate::data::Datafile = crate::data::Datafile::new();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4790() {
    rusty_monitor::set_test_id(4790);
    let mut u64_0: u64 = 3878u64;
    let mut u64_1: u64 = 6941u64;
    let mut u64_2: u64 = 1114u64;
    let mut u64_3: u64 = 1742u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_3, u64_2, u64_1);
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut u64_4: u64 = 2592u64;
    let mut u64_5: u64 = 5552u64;
    let mut u64_6: u64 = 9771u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_6, u64_5, u64_4);
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut u64_7: u64 = 8531u64;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut vec_1: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_8: u64 = 2467u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_8, on: vec_1};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Week(weekdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_9: u64 = 4205u64;
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_9);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_10: u64 = 2618u64;
    let mut u64_11: u64 = 9308u64;
    let mut u64_12: u64 = 2928u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_12, u64_11, u64_10);
    let mut i64_0: i64 = -698i64;
    let mut str_0: &str = "XEK";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_13: u64 = 5557u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_13, string_0, i64_0, simpledate_2, option_1, option_0, vec_0);
    let mut datafile_2: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_2_ref_0: &mut crate::data::Datafile = &mut datafile_2;
    crate::data::Datafile::insert(datafile_2_ref_0, expense_0);
    let mut duration_1: date::Duration = crate::date::Duration::Month(u64_7);
    let mut expense_slice_0: &[crate::expense::Expense] = crate::data::Datafile::expenses_between(datafile_0_ref_0, simpledate_1_ref_0, simpledate_0_ref_0);
    let mut datafile_1_ref_0: &mut crate::data::Datafile = &mut datafile_1;
    let mut result_0: std::result::Result<(), std::boxed::Box<dyn std::error::Error>> = crate::data::Datafile::remove(datafile_1_ref_0, u64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6414() {
    rusty_monitor::set_test_id(6414);
    let mut u64_0: u64 = 4150u64;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut u64_1: u64 = 5634u64;
    let mut u64_2: u64 = 5131u64;
    let mut u64_3: u64 = 4688u64;
    let mut u64_4: u64 = 8531u64;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut vec_1: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_5: u64 = 2467u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_5, on: vec_1};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Week(weekdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_6: u64 = 4205u64;
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_6);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_7: u64 = 2618u64;
    let mut u64_8: u64 = 9308u64;
    let mut u64_9: u64 = 2928u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_9, u64_8, u64_7);
    let mut i64_0: i64 = -698i64;
    let mut str_0: &str = "XEK";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_10: u64 = 5557u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_10, string_0, i64_0, simpledate_0, option_1, option_0, vec_0);
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_1_ref_0: &mut crate::data::Datafile = &mut datafile_1;
    crate::data::Datafile::insert(datafile_1_ref_0, expense_0);
    let mut duration_1: date::Duration = crate::date::Duration::Month(u64_4);
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_3, month: u64_2, day: u64_1};
    let mut result_0: std::result::Result<(), std::boxed::Box<dyn std::error::Error>> = crate::data::Datafile::remove(datafile_0_ref_0, u64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_562() {
    rusty_monitor::set_test_id(562);
    let mut str_0: &str = "Ug7sBzJcwaixBraeOJ2";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_0: u64 = 3625u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_0);
    let mut vec_1: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_1: u64 = 7705u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_1, on: vec_1};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Week(weekdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_2: u64 = 9281u64;
    let mut duration_0: date::Duration = crate::date::Duration::Week(u64_2);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_3: u64 = 8994u64;
    let mut u64_4: u64 = 8952u64;
    let mut u64_5: u64 = 4370u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_5, u64_4, u64_3);
    let mut i64_0: i64 = -5370i64;
    let mut str_1: &str = "nCgCiGTMbT3kb3w";
    let mut string_0: std::string::String = std::string::String::from(str_1);
    let mut u64_6: u64 = 7544u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_6, string_0, i64_0, simpledate_0, option_1, option_0, vec_0);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut vec_2: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut vec_3: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut vec_4: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6513() {
    rusty_monitor::set_test_id(6513);
    let mut u64_0: u64 = 8094u64;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Tuesday;
    let mut u64_1: u64 = 1387u64;
    let mut u64_2: u64 = 8937u64;
    let mut u64_3: u64 = 2033u64;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut vec_0: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut vec_1: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_4: u64 = 5580u64;
    let mut u64_5: u64 = 967u64;
    let mut u64_6: u64 = 9149u64;
    let mut u64_7: u64 = 866u64;
    let mut duration_0: date::Duration = crate::date::Duration::Week(u64_7);
    let mut u64_8: u64 = 6141u64;
    let mut u64_9: u64 = 1413u64;
    let mut u64_10: u64 = 6867u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_10, month: u64_9, day: u64_8};
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut duration_1: date::Duration = crate::date::Duration::Month(u64_6);
    let mut duration_2: date::Duration = crate::date::Duration::Day(u64_5);
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_4, days: vec_1};
    let mut option_0: std::option::Option<&crate::expense::Expense> = crate::data::Datafile::find(datafile_0_ref_0, u64_3);
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_2};
    let mut duration_3: date::Duration = crate::date::Duration::Day(u64_1);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5625() {
    rusty_monitor::set_test_id(5625);
    let mut u64_0: u64 = 279u64;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut vec_0: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_1: u64 = 4205u64;
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_1);
    let mut option_0: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_2: u64 = 2618u64;
    let mut u64_3: u64 = 9308u64;
    let mut u64_4: u64 = 2928u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_4, u64_3, u64_2);
    let mut str_0: &str = "z1";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_1_ref_0: &mut crate::data::Datafile = &mut datafile_1;
    let mut result_0: std::result::Result<(), std::boxed::Box<dyn std::error::Error>> = crate::data::Datafile::remove(datafile_0_ref_0, u64_0);
    panic!("From RustyUnit with love");
}
}