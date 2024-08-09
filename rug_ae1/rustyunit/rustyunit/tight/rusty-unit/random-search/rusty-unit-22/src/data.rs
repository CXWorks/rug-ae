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
fn rusty_test_3937() {
    rusty_monitor::set_test_id(3937);
    let mut u64_0: u64 = 5225u64;
    let mut u64_1: u64 = 3548u64;
    let mut u64_2: u64 = 794u64;
    let mut u64_3: u64 = 7458u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_3, u64_2, u64_1);
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut u64_4: u64 = 8482u64;
    let mut u64_5: u64 = 75u64;
    let mut u64_6: u64 = 703u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_6, month: u64_5, day: u64_4};
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut u64_7: u64 = 1327u64;
    let mut u64_8: u64 = 3214u64;
    let mut u64_9: u64 = 574u64;
    let mut u64_10: u64 = 9075u64;
    let mut u64_11: u64 = 9356u64;
    let mut u64_12: u64 = 1809u64;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut vec_0: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut u64_13: u64 = 1529u64;
    let mut u64_14: u64 = 8245u64;
    let mut duration_0: date::Duration = crate::date::Duration::Year(u64_14);
    let mut result_0: std::result::Result<(), std::boxed::Box<dyn std::error::Error>> = crate::data::Datafile::remove(datafile_0_ref_0, u64_12);
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_9};
    let mut duration_1: date::Duration = crate::date::Duration::Year(u64_8);
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_7};
    let mut datafile_1_ref_0: &crate::data::Datafile = &mut datafile_1;
    let mut expense_slice_0: &[crate::expense::Expense] = crate::data::Datafile::expenses_between(datafile_1_ref_0, simpledate_1_ref_0, simpledate_0_ref_0);
    let mut yeardelta_1: crate::date::YearDelta = crate::date::YearDelta {nth: u64_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4308() {
    rusty_monitor::set_test_id(4308);
    let mut u64_0: u64 = 1138u64;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_1: u64 = 1986u64;
    let mut u64_2: u64 = 6769u64;
    let mut u64_3: u64 = 3239u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_3, u64_2, u64_1);
    let mut i64_0: i64 = 2506i64;
    let mut str_0: &str = "";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_4: u64 = 8058u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_4, string_0, i64_0, simpledate_0, option_1, option_0, vec_0);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut vec_1: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_5: u64 = 3316u64;
    let mut u64_6: u64 = 9356u64;
    let mut u64_7: u64 = 239u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_7, u64_6, u64_5);
    let mut i64_1: i64 = -18521i64;
    let mut str_1: &str = "FGUiBkKvTyqWlY";
    let mut string_1: std::string::String = std::string::String::from(str_1);
    let mut u64_8: u64 = 2594u64;
    let mut u64_9: u64 = 5094u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_9};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut expense_1: crate::expense::Expense = crate::expense::Expense::new(u64_8, string_1, i64_1, simpledate_1, option_3, option_2, vec_1);
    let mut expense_1_ref_0: &crate::expense::Expense = &mut expense_1;
    let mut vec_2: &std::vec::Vec<std::string::String> = crate::expense::Expense::tags(expense_1_ref_0);
    let mut bool_0: bool = crate::expense::Expense::compare_id(expense_0_ref_0, u64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2716() {
    rusty_monitor::set_test_id(2716);
    let mut u64_0: u64 = 6268u64;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut vec_0: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut u64_1: u64 = 6145u64;
    let mut u64_2: u64 = 3444u64;
    let mut u64_3: u64 = 127u64;
    let mut vec_1: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_4: u64 = 6992u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_4);
    let mut u64_5: u64 = 7461u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_5};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_6: u64 = 4878u64;
    let mut duration_0: date::Duration = crate::date::Duration::Week(u64_6);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_7: u64 = 8448u64;
    let mut u64_8: u64 = 4986u64;
    let mut u64_9: u64 = 1167u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_9, month: u64_8, day: u64_7};
    let mut i64_0: i64 = -8002i64;
    let mut str_0: &str = "fhlN2kmS8wrSaJ5pbb";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_10: u64 = 5285u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_10, string_0, i64_0, simpledate_0, option_1, option_0, vec_1);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut bool_0: bool = crate::expense::Expense::compare_id(expense_0_ref_0, u64_3);
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_2};
    let mut weekday_0: date::Weekday = crate::date::Weekday::Saturday;
    let mut result_0: std::result::Result<(), std::boxed::Box<dyn std::error::Error>> = crate::data::Datafile::remove(datafile_0_ref_0, u64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2781() {
    rusty_monitor::set_test_id(2781);
    let mut u64_0: u64 = 9987u64;
    let mut u64_1: u64 = 2123u64;
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_2: u64 = 6273u64;
    let mut u64_3: u64 = 4093u64;
    let mut u64_4: u64 = 4508u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_4, u64_3, u64_2);
    let mut i64_0: i64 = 6783i64;
    let mut u64_5: u64 = 7620u64;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut u64_6: u64 = 3660u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_6};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_7: u64 = 6333u64;
    let mut u64_8: u64 = 6366u64;
    let mut u64_9: u64 = 4621u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_9, u64_8, u64_7);
    let mut i64_1: i64 = 4703i64;
    let mut str_0: &str = "OivKq8MiNhHBv5";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_10: u64 = 2724u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_10, string_0, i64_1, simpledate_1, option_3, option_2, vec_0);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut vec_1: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut u64_11: u64 = 8731u64;
    let mut vec_2: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_12: u64 = 4041u64;
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_12, days: vec_2};
    crate::data::Datafile::insert(datafile_0_ref_0, expense_0);
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Count(u64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_330() {
    rusty_monitor::set_test_id(330);
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_0: u64 = 6561u64;
    let mut duration_0: date::Duration = crate::date::Duration::Week(u64_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_1: u64 = 6372u64;
    let mut u64_2: u64 = 2594u64;
    let mut u64_3: u64 = 8729u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_3, month: u64_2, day: u64_1};
    let mut i64_0: i64 = 7779i64;
    let mut u64_4: u64 = 7937u64;
    let mut u64_5: u64 = 7516u64;
    let mut u64_6: u64 = 4624u64;
    let mut u64_7: u64 = 8127u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_7, month: u64_6, day: u64_5};
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_1);
    let mut u64_8: u64 = 8351u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_8};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut u64_9: u64 = 6618u64;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut u64_10: u64 = 915u64;
    let mut vec_0: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_11: u64 = 4879u64;
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_11, days: vec_0};
    let mut duration_1: date::Duration = crate::date::Duration::Year(u64_10);
    let mut option_2: std::option::Option<&crate::expense::Expense> = crate::data::Datafile::find(datafile_0_ref_0, u64_9);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut weekday_0: date::Weekday = crate::date::Weekday::Friday;
    let mut datafile_1_ref_0: &mut crate::data::Datafile = &mut datafile_1;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1267() {
    rusty_monitor::set_test_id(1267);
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_0: u64 = 7907u64;
    let mut u64_1: u64 = 7358u64;
    let mut u64_2: u64 = 2816u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_2, month: u64_1, day: u64_0};
    let mut i64_0: i64 = 3199i64;
    let mut u64_3: u64 = 4707u64;
    let mut u64_4: u64 = 4042u64;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut u64_5: u64 = 5358u64;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Thursday;
    let mut u64_6: u64 = 6622u64;
    let mut u64_7: u64 = 3913u64;
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_8: u64 = 7988u64;
    let mut u64_9: u64 = 1251u64;
    let mut u64_10: u64 = 8252u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_10, month: u64_9, day: u64_8};
    let mut i64_1: i64 = -2458i64;
    let mut u64_11: u64 = 7435u64;
    let mut str_0: &str = "NCG2GbFF5";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_1_ref_0: &mut crate::data::Datafile = &mut datafile_1;
    crate::data::Datafile::add_tag(datafile_1_ref_0, string_0);
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_7, weekid: u64_6, day: weekday_0};
    let mut repetition_0: crate::date::Repetition = std::option::Option::unwrap(option_2);
    let mut result_0: std::result::Result<(), std::boxed::Box<dyn std::error::Error>> = crate::data::Datafile::remove(datafile_0_ref_0, u64_4);
    let mut datafile_2: crate::data::Datafile = crate::data::Datafile::new();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2409() {
    rusty_monitor::set_test_id(2409);
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut vec_1: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_0: u64 = 4664u64;
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_0, days: vec_1};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnDate(monthdeltadate_0);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_1: u64 = 7916u64;
    let mut u64_2: u64 = 602u64;
    let mut u64_3: u64 = 2018u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_3, month: u64_2, day: u64_1};
    let mut i64_0: i64 = 768i64;
    let mut str_0: &str = "gwASVEUGFpcc";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_4: u64 = 7529u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_4, string_0, i64_0, simpledate_0, option_1, option_0, vec_0);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut vec_2: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Never;
    let mut u64_5: u64 = 1939u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_5};
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut repetition_1: crate::date::Repetition = crate::date::Repetition {delta: repdelta_1, end: repend_1};
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_1);
    let mut u64_6: u64 = 7450u64;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_6);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_7: u64 = 5870u64;
    let mut u64_8: u64 = 9674u64;
    let mut u64_9: u64 = 3089u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_9, u64_8, u64_7);
    let mut i64_1: i64 = -3923i64;
    let mut str_1: &str = "3WpIr04qnWU";
    let mut string_1: std::string::String = std::string::String::from(str_1);
    let mut u64_10: u64 = 3820u64;
    let mut expense_1: crate::expense::Expense = crate::expense::Expense::new(u64_10, string_1, i64_1, simpledate_1, option_3, option_2, vec_2);
    let mut expense_1_ref_0: &crate::expense::Expense = &mut expense_1;
    let mut ordering_0: std::cmp::Ordering = crate::expense::Expense::compare_dates(expense_1_ref_0, expense_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1770() {
    rusty_monitor::set_test_id(1770);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Friday;
    let mut u64_0: u64 = 3172u64;
    let mut u64_1: u64 = 709u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_1, weekid: u64_0, day: weekday_0};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    let mut u64_2: u64 = 2174u64;
    let mut u64_3: u64 = 4623u64;
    let mut weekday_1: date::Weekday = crate::date::Weekday::Sunday;
    let mut u64_4: u64 = 1581u64;
    let mut u64_5: u64 = 2211u64;
    let mut u64_6: u64 = 6825u64;
    let mut u64_7: u64 = 2962u64;
    let mut u64_8: u64 = 9272u64;
    let mut u64_9: u64 = 4981u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_9, u64_8, u64_7);
    let mut u64_10: u64 = 5595u64;
    let mut u64_11: u64 = 5797u64;
    let mut u64_12: u64 = 8735u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_12, month: u64_11, day: u64_10};
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut u64_13: u64 = 766u64;
    let mut u64_14: u64 = 5445u64;
    let mut u64_15: u64 = 497u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_15, u64_14, u64_13);
    let mut simpledate_2_ref_0: &crate::date::SimpleDate = &mut simpledate_2;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut expense_slice_0: &[crate::expense::Expense] = crate::data::Datafile::expenses_between(datafile_0_ref_0, simpledate_2_ref_0, simpledate_1_ref_0);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut duration_0: date::Duration = crate::date::Duration::Week(u64_6);
    let mut monthdeltaweek_1: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_5, weekid: u64_4, day: weekday_1};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    panic!("From RustyUnit with love");
}
}