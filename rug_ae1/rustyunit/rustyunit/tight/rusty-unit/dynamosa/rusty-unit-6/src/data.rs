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
fn rusty_test_6196() {
    rusty_monitor::set_test_id(6196);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut str_0: &str = "YuA";
    let mut u64_0: u64 = 230u64;
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_0);
    let mut u64_1: u64 = 4576u64;
    let mut u64_2: u64 = 9569u64;
    let mut u64_3: u64 = 693u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_3, month: u64_2, day: u64_1};
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut u64_4: u64 = 3740u64;
    let mut u64_5: u64 = 399u64;
    let mut duration_1: date::Duration = crate::date::Duration::Day(u64_4);
    let mut option_0: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_6: u64 = 6426u64;
    let mut u64_7: u64 = 4789u64;
    let mut u64_8: u64 = 326u64;
    let mut option_1: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_2: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_9: u64 = 8799u64;
    let mut u64_10: u64 = 4302u64;
    let mut u64_11: u64 = 4229u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_7, u64_6, u64_8);
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_3: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_4: std::option::Option<date::Duration> = std::option::Option::None;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_10, month: u64_9, day: u64_11};
    let mut i64_0: i64 = 22039i64;
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_5, string_0, i64_0, simpledate_2, option_0, option_1, vec_0);
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
fn rusty_test_392() {
    rusty_monitor::set_test_id(392);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut str_0: &str = "ewXSo28AOSdQGrK";
    let mut u64_0: u64 = 8005u64;
    let mut u64_1: u64 = 2982u64;
    let mut u64_2: u64 = 8801u64;
    let mut u64_3: u64 = 8882u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_3, month: u64_2, day: u64_1};
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Saturday;
    let mut u64_4: u64 = 7950u64;
    let mut u64_5: u64 = 2384u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_5, weekid: u64_4, day: weekday_0};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_6: u64 = 9926u64;
    let mut duration_0: date::Duration = crate::date::Duration::Week(u64_6);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_7: u64 = 4901u64;
    let mut u64_8: u64 = 522u64;
    let mut u64_9: u64 = 4150u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_9, month: u64_8, day: u64_7};
    let mut u64_10: u64 = 4229u64;
    let mut i64_0: i64 = 12416i64;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_11: u64 = 4931u64;
    let mut u64_12: u64 = 8729u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_0, month: u64_10, day: u64_11};
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_12, string_0, i64_0, simpledate_1, option_1, option_0, vec_0);
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
fn rusty_test_5903() {
    rusty_monitor::set_test_id(5903);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut str_0: &str = "o";
    let mut str_1: &str = "ZX";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "QDuFLZE2Mkea7";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut u64_0: u64 = 7406u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_0};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_1: u64 = 8799u64;
    let mut u64_2: u64 = 4229u64;
    let mut u64_3: u64 = 5022u64;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_4: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_5: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_4: u64 = 8729u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_2, month: u64_1, day: u64_3};
    let mut i64_0: i64 = 22039i64;
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_4, string_0, i64_0, simpledate_0, option_5, option_4, vec_0);
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
fn rusty_test_3612() {
    rusty_monitor::set_test_id(3612);
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_0: u64 = 2997u64;
    let mut u64_1: u64 = 3778u64;
    let mut u64_2: u64 = 1344u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_2, month: u64_1, day: u64_0};
    let mut weekday_0: date::Weekday = crate::date::Weekday::Monday;
    let mut u64_3: u64 = 4988u64;
    let mut u64_4: u64 = 6601u64;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut vec_0: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut u64_5: u64 = 7114u64;
    let mut u64_6: u64 = 5759u64;
    let mut u64_7: u64 = 3858u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_7, u64_6, u64_5);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_1);
    let mut weekday_1: date::Weekday = crate::date::Weekday::Thursday;
    let mut weekday_2: date::Weekday = crate::date::Weekday::Tuesday;
    let mut result_0: std::result::Result<(), std::boxed::Box<dyn std::error::Error>> = crate::data::Datafile::remove(datafile_0_ref_0, u64_4);
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_3};
    let mut tuple_0: () = std::result::Result::unwrap(result_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4543() {
    rusty_monitor::set_test_id(4543);
    let mut u64_0: u64 = 1433u64;
    let mut str_0: &str = "2kYQKKA9FqJVTetTt";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_1: u64 = 191u64;
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_1);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_2: u64 = 1502u64;
    let mut u64_3: u64 = 7316u64;
    let mut u64_4: u64 = 4536u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_4, u64_3, u64_2);
    let mut i64_0: i64 = 10320i64;
    let mut str_1: &str = "XdanRQqt9wD";
    let mut string_0: std::string::String = std::string::String::from(str_1);
    let mut u64_5: u64 = 239u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_5, string_0, i64_0, simpledate_0, option_1, option_0, vec_0);
    let mut expense_0_ref_0: &mut crate::expense::Expense = &mut expense_0;
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_6: u64 = 2009u64;
    let mut duration_1: date::Duration = crate::date::Duration::Year(u64_6);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
    let mut u64_7: u64 = 2997u64;
    let mut u64_8: u64 = 3778u64;
    let mut u64_9: u64 = 1344u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_9, month: u64_8, day: u64_7};
    let mut weekday_0: date::Weekday = crate::date::Weekday::Monday;
    let mut u64_10: u64 = 2930u64;
    let mut u64_11: u64 = 6937u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_11, weekid: u64_10, day: weekday_0};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    let mut u64_12: u64 = 4988u64;
    let mut u64_13: u64 = 7114u64;
    let mut u64_14: u64 = 5759u64;
    let mut u64_15: u64 = 3858u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_15, u64_14, u64_13);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_2);
    let mut weekday_1: date::Weekday = crate::date::Weekday::Thursday;
    let mut weekday_2: date::Weekday = crate::date::Weekday::Tuesday;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_12};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    crate::expense::Expense::remove_tags(expense_0_ref_0, str_0_ref_0);
    let mut duration_2: date::Duration = crate::date::Duration::Day(u64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4764() {
    rusty_monitor::set_test_id(4764);
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut str_0: &str = "sxz6hh";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut vec_1: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_0: u64 = 1063u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_0);
    let mut u64_1: u64 = 6337u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_1};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_2: u64 = 2447u64;
    let mut u64_3: u64 = 2059u64;
    let mut u64_4: u64 = 7203u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_4, u64_3, u64_2);
    let mut i64_0: i64 = -2701i64;
    let mut str_1: &str = "m6IDIrWXIiitNSHs3c";
    let mut string_1: std::string::String = std::string::String::from(str_1);
    let mut u64_5: u64 = 2764u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_5, string_1, i64_0, simpledate_0, option_3, option_2, vec_1);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Sunday;
    let mut str_2: &str = crate::expense::Expense::description(expense_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7712() {
    rusty_monitor::set_test_id(7712);
    let mut u64_0: u64 = 3879u64;
    let mut u64_1: u64 = 4420u64;
    let mut u64_2: u64 = 4381u64;
    let mut u64_3: u64 = 5617u64;
    let mut u64_4: u64 = 3882u64;
    let mut u64_5: u64 = 9852u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_5, u64_4, u64_3);
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut u64_6: u64 = 8818u64;
    let mut u64_7: u64 = 7838u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_7, month: u64_0, day: u64_6};
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut expense_slice_0: &[crate::expense::Expense] = crate::data::Datafile::expenses_between(datafile_0_ref_0, simpledate_1_ref_0, simpledate_0_ref_0);
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_2};
    let mut datafile_1_ref_0: &crate::data::Datafile = &mut datafile_1;
    let mut option_0: std::option::Option<&crate::expense::Expense> = crate::data::Datafile::find(datafile_1_ref_0, u64_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_897() {
    rusty_monitor::set_test_id(897);
    let mut u64_0: u64 = 4607u64;
    let mut u64_1: u64 = 8342u64;
    let mut u64_2: u64 = 7365u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_2, u64_1, u64_0);
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut u64_3: u64 = 1753u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_3);
    let mut u64_4: u64 = 8664u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_4};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut str_0: &str = "1CQhMk5Gssn";
    let mut u64_5: u64 = 1013u64;
    let mut duration_0: date::Duration = crate::date::Duration::Year(u64_5);
    let mut u64_6: u64 = 5886u64;
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Count(u64_6);
    let mut vec_0: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_7: u64 = 8130u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_7, on: vec_0};
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Week(weekdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_1, end: repend_1};
    let mut repend_2: date::RepEnd = crate::date::RepEnd::Never;
    let mut u64_8: u64 = 7699u64;
    let mut daydelta_1: crate::date::DayDelta = crate::date::DayDelta {nth: u64_8};
    let mut repdelta_2: date::RepDelta = crate::date::RepDelta::Day(daydelta_1);
    let mut u64_9: u64 = 9203u64;
    let mut daydelta_2: crate::date::DayDelta = crate::date::DayDelta {nth: u64_9};
    let mut u64_10: u64 = 6024u64;
    let mut vec_1: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_11: u64 = 7403u64;
    let mut daydelta_3: crate::date::DayDelta = crate::date::DayDelta {nth: u64_10};
    let mut repdelta_3: date::RepDelta = crate::date::RepDelta::Day(daydelta_2);
    let mut repetition_1: crate::date::Repetition = crate::date::Repetition {delta: repdelta_2, end: repend_2};
    let mut option_1: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut duration_1: date::Duration = crate::date::Duration::Day(u64_11);
    let mut option_2: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_12: u64 = 4789u64;
    let mut u64_13: u64 = 326u64;
    let mut option_3: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_4: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_14: u64 = 4302u64;
    let mut vec_2: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_5: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_6: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_15: u64 = 4085u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_15, month: u64_12, day: u64_14};
    let mut i64_0: i64 = 22039i64;
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_13, string_0, i64_0, simpledate_1, option_4, option_1, vec_2);
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut vec_3: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    crate::data::Datafile::insert(datafile_0_ref_0, expense_0);
    let mut repetition_2: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5848() {
    rusty_monitor::set_test_id(5848);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut str_0: &str = "GmGtp0kvXU";
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_0: u64 = 5632u64;
    let mut duration_0: date::Duration = crate::date::Duration::Year(u64_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_1: u64 = 1707u64;
    let mut u64_2: u64 = 1060u64;
    let mut u64_3: u64 = 9369u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_3, month: u64_2, day: u64_1};
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_4: u64 = 6935u64;
    let mut duration_1: date::Duration = crate::date::Duration::Day(u64_4);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
    let mut u64_5: u64 = 7727u64;
    let mut u64_6: u64 = 6116u64;
    let mut u64_7: u64 = 4750u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_7, month: u64_6, day: u64_5};
    let mut u64_8: u64 = 2948u64;
    let mut u64_8_ref_0: &u64 = &mut u64_8;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Tuesday;
    let mut u64_9: u64 = 8557u64;
    let mut u64_10: u64 = 9953u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_10, weekid: u64_9, day: weekday_0};
    let mut monthdeltaweek_0_ref_0: &crate::date::MonthDeltaWeek = &mut monthdeltaweek_0;
    let mut u64_11: u64 = 9700u64;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_4: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut i64_0: i64 = 22039i64;
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_11, string_0, i64_0, simpledate_1, option_3, option_4, vec_0);
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
fn rusty_test_2() {
    rusty_monitor::set_test_id(2);
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_0: u64 = 5378u64;
    let mut u64_1: u64 = 1610u64;
    let mut u64_2: u64 = 8386u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_2, month: u64_1, day: u64_0};
    let mut i64_0: i64 = -5681i64;
    let mut u64_3: u64 = 1857u64;
    let mut u64_4: u64 = 1748u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_4);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Monday;
    let mut u64_5: u64 = 5514u64;
    let mut u64_6: u64 = 8668u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_6, weekid: u64_5, day: weekday_0};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_7: u64 = 8450u64;
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_8: u64 = 3199u64;
    let mut u64_9: u64 = 9114u64;
    let mut u64_10: u64 = 94u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_10, u64_9, u64_8);
    let mut i64_1: i64 = -7205i64;
    let mut u64_11: u64 = 7943u64;
    let mut str_0: &str = "FRd5Myut829ej01MLM";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut weekday_1: date::Weekday = crate::date::Weekday::Tuesday;
    crate::data::Datafile::add_tag(datafile_0_ref_0, string_0);
    let mut repetition_1: crate::date::Repetition = std::option::Option::unwrap(option_2);
    panic!("From RustyUnit with love");
}
}