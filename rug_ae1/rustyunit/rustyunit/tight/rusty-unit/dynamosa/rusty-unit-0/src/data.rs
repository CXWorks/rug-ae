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
fn rusty_test_988() {
    rusty_monitor::set_test_id(988);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut u64_0: u64 = 8985u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_0};
    let mut u64_1: u64 = 7873u64;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_1);
    let mut option_0: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_2: u64 = 2611u64;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut u64_3: u64 = 9492u64;
    let mut u64_4: u64 = 4618u64;
    let mut u64_5: u64 = 2556u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_5, u64_4, u64_3);
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Monday;
    let mut option_1: std::option::Option<&crate::expense::Expense> = crate::data::Datafile::find(datafile_0_ref_0, u64_2);
    let mut expense_0: &crate::expense::Expense = std::option::Option::unwrap(option_1);
    let mut weekday_1: date::Weekday = crate::date::Weekday::Saturday;
    let mut weekday_2: date::Weekday = crate::date::Weekday::Friday;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6728() {
    rusty_monitor::set_test_id(6728);
    let mut u64_0: u64 = 6100u64;
    let mut u64_1: u64 = 4962u64;
    let mut u64_2: u64 = 7310u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_2, u64_1, u64_0);
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut u64_3: u64 = 1034u64;
    let mut u64_4: u64 = 4556u64;
    let mut u64_5: u64 = 6127u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_5, month: u64_4, day: u64_3};
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_1_ref_0: &mut crate::data::Datafile = &mut datafile_1;
    let mut option_1: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_6: u64 = 8598u64;
    let mut u64_7: u64 = 6527u64;
    let mut u64_8: u64 = 6712u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_8, u64_7, u64_6);
    let mut str_0: &str = "J2iO6nVwK";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut datafile_2: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_2_ref_0: &mut crate::data::Datafile = &mut datafile_2;
    crate::data::Datafile::add_tag(datafile_2_ref_0, string_0);
    let mut expense_slice_0: &[crate::expense::Expense] = crate::data::Datafile::expenses_between(datafile_0_ref_0, simpledate_1_ref_0, simpledate_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1735() {
    rusty_monitor::set_test_id(1735);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut u64_0: u64 = 8295u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_0};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut option_0: std::option::Option<date::Duration> = std::option::Option::None;
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_1_ref_0: &mut crate::data::Datafile = &mut datafile_1;
    let mut u64_1: u64 = 6975u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_1};
    let mut vec_0: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_2: u64 = 7125u64;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Saturday;
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_2, days: vec_0};
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_0() {
    rusty_monitor::set_test_id(0);
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_0: u64 = 3772u64;
    let mut u64_1: u64 = 4525u64;
    let mut u64_2: u64 = 8713u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_2, u64_1, u64_0);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut vec_1: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_3: u64 = 9292u64;
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_3, days: vec_1};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnDate(monthdeltadate_0);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_4: u64 = 4268u64;
    let mut u64_5: u64 = 7945u64;
    let mut u64_6: u64 = 6374u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_6, month: u64_5, day: u64_4};
    let mut i64_0: i64 = 8636i64;
    let mut str_0: &str = "OPkUwOsy56D";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_7: u64 = 9441u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_7, string_0, i64_0, simpledate_1, option_1, option_0, vec_0);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut u64_8: u64 = 6131u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_8};
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Never;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Wednesday;
    let mut str_1: &str = crate::expense::Expense::description(expense_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_36() {
    rusty_monitor::set_test_id(36);
    let mut u64_0: u64 = 2087u64;
    let mut u64_1: u64 = 7238u64;
    let mut u64_2: u64 = 8391u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_2, u64_1, u64_0);
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut u64_3: u64 = 581u64;
    let mut u64_4: u64 = 7857u64;
    let mut u64_5: u64 = 9314u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_5, u64_4, u64_3);
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut u64_6: u64 = 3849u64;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Tuesday;
    let mut u64_7: u64 = 2211u64;
    let mut u64_8: u64 = 2095u64;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_9: u64 = 8427u64;
    let mut u64_10: u64 = 6954u64;
    let mut u64_11: u64 = 3691u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_11, u64_10, u64_9);
    let mut i64_0: i64 = -11682i64;
    let mut str_0: &str = "";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_12: u64 = 278u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_12, string_0, i64_0, simpledate_2, option_1, option_0, vec_0);
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_8, weekid: u64_7, day: weekday_0};
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_6);
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut expense_slice_0: &[crate::expense::Expense] = crate::data::Datafile::expenses_between(datafile_0_ref_0, simpledate_1_ref_0, simpledate_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_79() {
    rusty_monitor::set_test_id(79);
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_0: u64 = 5379u64;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_1: u64 = 2090u64;
    let mut u64_2: u64 = 4388u64;
    let mut u64_3: u64 = 9200u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_3, month: u64_2, day: u64_1};
    let mut u64_4: u64 = 3691u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_4);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Sunday;
    let mut u64_5: u64 = 9393u64;
    let mut u64_6: u64 = 3290u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_6, weekid: u64_5, day: weekday_0};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::None;
    let mut option_4: std::option::Option<crate::date::SimpleDate> = std::option::Option::None;
    let mut u64_7: u64 = 8466u64;
    let mut u64_8: u64 = 6484u64;
    let mut u64_9: u64 = 704u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_9, month: u64_8, day: u64_7};
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut vec_0: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut vec_1: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Never;
    let mut vec_2: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_10: u64 = 343u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_10, on: vec_2};
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Week(weekdelta_0);
    let mut repetition_1: crate::date::Repetition = crate::date::Repetition {delta: repdelta_1, end: repend_1};
    let mut option_5: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_1);
    let mut option_6: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_11: u64 = 6094u64;
    let mut u64_12: u64 = 4981u64;
    let mut u64_13: u64 = 5669u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_13, u64_12, u64_11);
    let mut i64_0: i64 = -4293i64;
    let mut str_0: &str = "hGhWW7n";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_14: u64 = 9082u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_14, string_0, i64_0, simpledate_2, option_6, option_5, vec_1);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut simpledate_3: &crate::date::SimpleDate = crate::expense::Expense::get_start_date(expense_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8261() {
    rusty_monitor::set_test_id(8261);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut u64_0: u64 = 8295u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_0};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut option_0: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_1: u64 = 6958u64;
    let mut u64_2: u64 = 6811u64;
    let mut u64_3: u64 = 7755u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_3, month: u64_2, day: u64_1};
    let mut u64_4: u64 = 4745u64;
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_1_ref_0: &mut crate::data::Datafile = &mut datafile_1;
    let mut u64_5: u64 = 6975u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_5};
    let mut vec_0: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_6: u64 = 7125u64;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Saturday;
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_6, days: vec_0};
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut result_0: std::result::Result<(), std::boxed::Box<dyn std::error::Error>> = crate::data::Datafile::remove(datafile_1_ref_0, u64_4);
    let mut tuple_0: () = std::result::Result::unwrap(result_0);
    panic!("From RustyUnit with love");
}
}