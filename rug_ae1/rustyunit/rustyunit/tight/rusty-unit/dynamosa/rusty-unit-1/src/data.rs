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
fn rusty_test_33() {
    rusty_monitor::set_test_id(33);
    let mut u64_0: u64 = 3089u64;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_0);
    let mut duration_0_ref_0: &date::Duration = &mut duration_0;
    let mut u64_1: u64 = 1941u64;
    let mut u64_2: u64 = 3821u64;
    let mut u64_3: u64 = 256u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_3, u64_2, u64_1);
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut str_0: &str = "RumxUqtAX1gc";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_4: u64 = 6064u64;
    let mut u64_5: u64 = 5140u64;
    let mut u64_6: u64 = 5595u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_6, u64_5, u64_4);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_1);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Thursday;
    let mut u64_7: u64 = 3412u64;
    let mut u64_8: u64 = 3692u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_8, weekid: u64_7, day: weekday_0};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_9: u64 = 7118u64;
    let mut u64_10: u64 = 6264u64;
    let mut u64_11: u64 = 4638u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_11, month: u64_10, day: u64_9};
    let mut i64_0: i64 = 1640i64;
    let mut str_1: &str = "tOB";
    let mut string_0: std::string::String = std::string::String::from(str_1);
    let mut u64_12: u64 = 1500u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_12, string_0, i64_0, simpledate_2, option_1, option_0, vec_0);
    let mut expense_0_ref_0: &mut crate::expense::Expense = &mut expense_0;
    crate::expense::Expense::remove_tags(expense_0_ref_0, str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7667() {
    rusty_monitor::set_test_id(7667);
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_0: u64 = 984u64;
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_1: u64 = 8498u64;
    let mut u64_2: u64 = 8069u64;
    let mut u64_3: u64 = 4455u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_3, month: u64_2, day: u64_1};
    let mut i64_0: i64 = -3117i64;
    let mut str_0: &str = "UyGOeEmL";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_4: u64 = 4778u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_4, string_0, i64_0, simpledate_0, option_1, option_0, vec_0);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut vec_1: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut weekday_0: date::Weekday = crate::date::Weekday::Monday;
    let mut u64_5: u64 = 1708u64;
    let mut u64_6: u64 = 3115u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_6, weekid: u64_5, day: weekday_0};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut u64_7: u64 = 3313u64;
    let mut duration_1: date::Duration = crate::date::Duration::Month(u64_7);
    let mut option_2: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_388() {
    rusty_monitor::set_test_id(388);
    let mut u64_0: u64 = 7451u64;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut str_0: &str = "1fbk8QJEkZsPqkPO";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_1: u64 = 5460u64;
    let mut u64_2: u64 = 3733u64;
    let mut u64_3: u64 = 9417u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_3, u64_2, u64_1);
    let mut i64_0: i64 = -234i64;
    let mut str_1: &str = "1kLv2DImOqgIXa";
    let mut string_0: std::string::String = std::string::String::from(str_1);
    let mut u64_4: u64 = 6928u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_4, string_0, i64_0, simpledate_0, option_1, option_0, vec_0);
    let mut expense_0_ref_0: &mut crate::expense::Expense = &mut expense_0;
    let mut vec_1: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_5: u64 = 5420u64;
    let mut vec_2: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_6: u64 = 766u64;
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_6, days: vec_2};
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_5, on: vec_1};
    crate::expense::Expense::remove_tags(expense_0_ref_0, str_0_ref_0);
    let mut option_2: std::option::Option<&crate::expense::Expense> = crate::data::Datafile::find(datafile_0_ref_0, u64_0);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Friday;
    let mut expense_1: &crate::expense::Expense = std::option::Option::unwrap(option_2);
    let mut weekday_1: date::Weekday = crate::date::Weekday::Wednesday;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_376() {
    rusty_monitor::set_test_id(376);
    let mut u64_0: u64 = 3608u64;
    let mut u64_1: u64 = 1355u64;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Sunday;
    let mut u64_2: u64 = 4800u64;
    let mut u64_3: u64 = 2763u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_3, weekid: u64_2, day: weekday_0};
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_4: u64 = 3755u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_4);
    let mut vec_1: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_5: u64 = 5571u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_5, on: vec_1};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Week(weekdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_6: u64 = 9280u64;
    let mut u64_7: u64 = 5785u64;
    let mut u64_8: u64 = 8261u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_8, u64_7, u64_6);
    let mut i64_0: i64 = -7823i64;
    let mut str_0: &str = "0";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_9: u64 = 3821u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_9, string_0, i64_0, simpledate_0, option_1, option_0, vec_0);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    crate::data::Datafile::insert(datafile_0_ref_0, expense_0);
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_1);
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6992() {
    rusty_monitor::set_test_id(6992);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Sunday;
    let mut u64_0: u64 = 154u64;
    let mut u64_1: u64 = 4497u64;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_2: u64 = 141u64;
    let mut duration_0: date::Duration = crate::date::Duration::Year(u64_2);
    let mut option_0: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut str_0: &str = "fe4XQJMjeYQ";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut u64_3: u64 = 5024u64;
    let mut weekday_1: date::Weekday = crate::date::Weekday::Sunday;
    let mut duration_1: date::Duration = crate::date::Duration::Year(u64_3);
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_1, weekid: u64_0, day: weekday_0};
    let mut duration_2: date::Duration = std::option::Option::unwrap(option_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8094() {
    rusty_monitor::set_test_id(8094);
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_0: u64 = 7620u64;
    let mut u64_1: u64 = 511u64;
    let mut u64_2: u64 = 5704u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_2, u64_1, u64_0);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Thursday;
    let mut u64_3: u64 = 141u64;
    let mut duration_0: date::Duration = crate::date::Duration::Year(u64_3);
    let mut option_0: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_4: u64 = 6223u64;
    let mut u64_5: u64 = 4146u64;
    let mut u64_6: u64 = 7895u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_6, u64_5, u64_4);
    let mut str_0: &str = "fe4XQJMjeYQ";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut u64_7: u64 = 5024u64;
    let mut duration_1: date::Duration = crate::date::Duration::Year(u64_7);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3373() {
    rusty_monitor::set_test_id(3373);
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_0: u64 = 4162u64;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_1: u64 = 4368u64;
    let mut u64_2: u64 = 5575u64;
    let mut u64_3: u64 = 7403u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_3, u64_2, u64_1);
    let mut i64_0: i64 = -2958i64;
    let mut str_0: &str = "bwwnvtjnN";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_4: u64 = 5195u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_4, string_0, i64_0, simpledate_0, option_1, option_0, vec_0);
    let mut u64_5: u64 = 7588u64;
    let mut duration_1: date::Duration = crate::date::Duration::Day(u64_5);
    let mut str_1: &str = "N7FJs4X61IRybr61NsM";
    let mut u64_6: u64 = 7209u64;
    let mut u64_7: u64 = 9602u64;
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_8: u64 = 3331u64;
    let mut duration_2: date::Duration = crate::date::Duration::Month(u64_8);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::Some(duration_2);
    let mut u64_9: u64 = 9882u64;
    let mut u64_10: u64 = 3498u64;
    let mut u64_11: u64 = 1991u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_11, u64_10, u64_9);
    let mut i64_1: i64 = -11730i64;
    let mut u64_12: u64 = 551u64;
    let mut u64_13: u64 = 3651u64;
    let mut u64_14: u64 = 6651u64;
    let mut u64_15: u64 = 212u64;
    let mut u64_16: u64 = 3512u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_16, month: u64_15, day: u64_14};
    let mut simpledate_2_ref_0: &crate::date::SimpleDate = &mut simpledate_2;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Sunday;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_6};
    let mut str_2: &str = "QKwhOhtPKMG71pmNU9";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut u64_17: u64 = 8183u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_7);
    let mut u64_18: u64 = 51u64;
    let mut daydelta_1: crate::date::DayDelta = crate::date::DayDelta {nth: u64_13};
    let mut vec_1: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_4: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_19: u64 = 984u64;
    let mut duration_3: date::Duration = crate::date::Duration::Month(u64_12);
    let mut option_5: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
    let mut u64_20: u64 = 4455u64;
    let mut simpledate_3: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_18, month: u64_19, day: u64_17};
    let mut string_1: std::string::String = std::string::String::from(str_2);
    let mut u64_21: u64 = 4778u64;
    let mut expense_1: crate::expense::Expense = crate::expense::Expense::new(u64_20, string_1, i64_1, simpledate_1, option_5, option_2, vec_1);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut vec_2: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Never;
    let mut weekday_1: date::Weekday = crate::date::Weekday::Monday;
    let mut u64_22: u64 = 6074u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_21, weekid: u64_22, day: weekday_0};
    let mut vec_2_ref_0: &mut std::vec::Vec<std::string::String> = &mut vec_2;
    let mut expense_1_ref_0: &crate::expense::Expense = &mut expense_1;
    let mut i64_2: i64 = crate::expense::Expense::amount(expense_1_ref_0);
    let mut weekday_2: date::Weekday = crate::date::Weekday::Sunday;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_447() {
    rusty_monitor::set_test_id(447);
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_0: u64 = 7620u64;
    let mut u64_1: u64 = 511u64;
    let mut u64_2: u64 = 5704u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_2, u64_1, u64_0);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Thursday;
    let mut u64_3: u64 = 4552u64;
    let mut u64_4: u64 = 686u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_4, weekid: u64_3, day: weekday_0};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_5: u64 = 141u64;
    let mut duration_0: date::Duration = crate::date::Duration::Year(u64_5);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_6: u64 = 6223u64;
    let mut u64_7: u64 = 4146u64;
    let mut u64_8: u64 = 7895u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_8, u64_7, u64_6);
    let mut i64_0: i64 = 9574i64;
    let mut str_0: &str = "fe4XQJMjeYQ";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_9: u64 = 4658u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_9, string_0, i64_0, simpledate_1, option_1, option_0, vec_0);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4401() {
    rusty_monitor::set_test_id(4401);
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut i64_0: i64 = 9200i64;
    let mut u64_0: u64 = 5658u64;
    let mut duration_0: date::Duration = crate::date::Duration::Year(u64_0);
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_1: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_1: u64 = 984u64;
    let mut duration_1: date::Duration = crate::date::Duration::Month(u64_1);
    let mut option_2: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
    let mut u64_2: u64 = 8498u64;
    let mut u64_3: u64 = 8069u64;
    let mut u64_4: u64 = 4455u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_4, month: u64_3, day: u64_2};
    let mut i64_1: i64 = -3117i64;
    let mut str_0: &str = "UyGOeEmL";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_5: u64 = 4778u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_5, string_0, i64_1, simpledate_0, option_2, option_1, vec_0);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut vec_1: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Monday;
    let mut u64_6: u64 = 6074u64;
    let mut u64_7: u64 = 3115u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_7, weekid: u64_6, day: weekday_0};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_3: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_8: u64 = 3313u64;
    let mut duration_2: date::Duration = crate::date::Duration::Month(u64_8);
    let mut option_4: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_9: u64 = 4724u64;
    let mut u64_10: u64 = 6851u64;
    let mut u64_11: u64 = 5460u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_11, month: u64_10, day: u64_9};
    let mut str_1: &str = "m7";
    let mut string_1: std::string::String = std::string::String::from(str_1);
    let mut u64_12: u64 = 1717u64;
    let mut expense_1: crate::expense::Expense = crate::expense::Expense::new(u64_12, string_1, i64_0, simpledate_1, option_4, option_0, vec_1);
    let mut expense_1_ref_0: &crate::expense::Expense = &mut expense_1;
    let mut ordering_0: std::cmp::Ordering = crate::expense::Expense::compare_dates(expense_1_ref_0, expense_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8298() {
    rusty_monitor::set_test_id(8298);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Monday;
    let mut u64_0: u64 = 1714u64;
    let mut u64_1: u64 = 408u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_1, weekid: u64_0, day: weekday_0};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    let mut u64_2: u64 = 308u64;
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_2);
    let mut duration_0_ref_0: &date::Duration = &mut duration_0;
    let mut u64_3: u64 = 3814u64;
    let mut u64_4: u64 = 8452u64;
    let mut u64_5: u64 = 9630u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_5, u64_4, u64_3);
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut weekday_1: date::Weekday = crate::date::Weekday::Saturday;
    let mut u64_6: u64 = 3529u64;
    let mut u64_7: u64 = 8212u64;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut option_0: std::option::Option<&crate::expense::Expense> = crate::data::Datafile::find(datafile_0_ref_0, u64_7);
    let mut duration_1: date::Duration = crate::date::Duration::Month(u64_6);
    let mut weekday_2: date::Weekday = crate::date::Weekday::Wednesday;
    let mut expense_0: &crate::expense::Expense = std::option::Option::unwrap(option_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6162() {
    rusty_monitor::set_test_id(6162);
    let mut u64_0: u64 = 673u64;
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_1: u64 = 133u64;
    let mut u64_2: u64 = 7665u64;
    let mut u64_3: u64 = 7150u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_3, u64_2, u64_1);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut str_0: &str = "4WUCuCIcI3S";
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_4: u64 = 8097u64;
    let mut u64_5: u64 = 1328u64;
    let mut u64_6: u64 = 8343u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_6, u64_5, u64_4);
    let mut i64_0: i64 = 9020i64;
    let mut u64_7: u64 = 3490u64;
    let mut u64_8: u64 = 7936u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_8};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut u64_9: u64 = 3858u64;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_9);
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_7, string_0, i64_0, simpledate_1, option_3, option_2, vec_0);
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    crate::data::Datafile::insert(datafile_0_ref_0, expense_0);
    let mut datafile_1_ref_0: &mut crate::data::Datafile = &mut datafile_1;
    let mut duration_1: date::Duration = crate::date::Duration::Day(u64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_38() {
    rusty_monitor::set_test_id(38);
    let mut str_0: &str = "UQLu8s";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut u64_0: u64 = 2471u64;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_1: u64 = 1681u64;
    let mut u64_2: u64 = 270u64;
    let mut u64_3: u64 = 4688u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_3, u64_2, u64_1);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut vec_1: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_4: u64 = 8970u64;
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_4, days: vec_1};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnDate(monthdeltadate_0);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_5: u64 = 7780u64;
    let mut duration_0: date::Duration = crate::date::Duration::Week(u64_5);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_6: u64 = 2021u64;
    let mut u64_7: u64 = 4196u64;
    let mut u64_8: u64 = 8038u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_8, u64_7, u64_6);
    let mut i64_0: i64 = -15960i64;
    let mut str_1: &str = "jfmUO";
    let mut string_1: std::string::String = std::string::String::from(str_1);
    let mut u64_9: u64 = 8465u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_9, string_1, i64_0, simpledate_1, option_1, option_0, vec_0);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Monday;
    let mut bool_0: bool = crate::expense::Expense::compare_id(expense_0_ref_0, u64_0);
    crate::data::Datafile::add_tag(datafile_0_ref_0, string_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4502() {
    rusty_monitor::set_test_id(4502);
    let mut u64_0: u64 = 6691u64;
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_0);
    let mut duration_0_ref_0: &date::Duration = &mut duration_0;
    let mut u64_1: u64 = 6108u64;
    let mut u64_2: u64 = 9601u64;
    let mut u64_3: u64 = 4058u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_3, month: u64_2, day: u64_1};
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_4: u64 = 4464u64;
    let mut u64_5: u64 = 1335u64;
    let mut u64_6: u64 = 8898u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_6};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut weekday_0: date::Weekday = crate::date::Weekday::Thursday;
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut result_0: std::result::Result<(), std::boxed::Box<dyn std::error::Error>> = crate::data::Datafile::remove(datafile_0_ref_0, u64_4);
    let mut duration_1: date::Duration = crate::date::Duration::Month(u64_5);
    let mut repetition_0: crate::date::Repetition = std::option::Option::unwrap(option_0);
    let mut duration_2: date::Duration = std::option::Option::unwrap(option_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8367() {
    rusty_monitor::set_test_id(8367);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Monday;
    let mut u64_0: u64 = 2885u64;
    let mut u64_1: u64 = 1485u64;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_2: u64 = 4354u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_2);
    let mut vec_1: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_3: u64 = 5684u64;
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_3, days: vec_1};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnDate(monthdeltadate_0);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_4: u64 = 5626u64;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_4);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_5: u64 = 8244u64;
    let mut u64_6: u64 = 8560u64;
    let mut u64_7: u64 = 9469u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_7, u64_6, u64_5);
    let mut u64_8: u64 = 6921u64;
    let mut str_0: &str = "6iFPiLpHI";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut weekday_1: date::Weekday = crate::date::Weekday::Thursday;
    let mut u64_9: u64 = 7021u64;
    let mut u64_10: u64 = 27u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_10, weekid: u64_9, day: weekday_1};
    let mut monthdeltaweek_0_ref_0: &crate::date::MonthDeltaWeek = &mut monthdeltaweek_0;
    let mut str_1: &str = "zIVzlSLjiByQ";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "MF0nvTHSzCF1fVFUUQ";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut u64_11: u64 = 9558u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_11};
    let mut u64_12: u64 = 7472u64;
    let mut duration_1: date::Duration = crate::date::Duration::Year(u64_12);
    let mut str_3: &str = "YwDIXsXskd7d1vG2";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "zNbb8P1V9g9dQL";
    let mut i64_0: i64 = 9574i64;
    let mut string_0: std::string::String = std::string::String::from(str_4);
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_8, string_0, i64_0, simpledate_0, option_1, option_0, vec_0);
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    crate::data::Datafile::insert(datafile_0_ref_0, expense_0);
    let mut monthdeltaweek_1: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_1, weekid: u64_0, day: weekday_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5304() {
    rusty_monitor::set_test_id(5304);
    let mut u64_0: u64 = 7209u64;
    let mut u64_1: u64 = 9602u64;
    let mut u64_2: u64 = 1933u64;
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_3: u64 = 3331u64;
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_3);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_4: u64 = 9882u64;
    let mut u64_5: u64 = 3498u64;
    let mut u64_6: u64 = 1991u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_6, u64_5, u64_4);
    let mut u64_7: u64 = 3653u64;
    let mut u64_8: u64 = 6651u64;
    let mut u64_9: u64 = 212u64;
    let mut u64_10: u64 = 3512u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_10, month: u64_9, day: u64_8};
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut expense_slice_0: &[crate::expense::Expense] = crate::data::Datafile::expenses_between(datafile_0_ref_0, simpledate_0_ref_0, simpledate_1_ref_0);
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_7};
    let mut weekday_0: date::Weekday = crate::date::Weekday::Thursday;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_2, u64_1, u64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_19() {
    rusty_monitor::set_test_id(19);
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_0: u64 = 1403u64;
    let mut u64_1: u64 = 74u64;
    let mut u64_2: u64 = 454u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_2, u64_1, u64_0);
    let mut i64_0: i64 = 4743i64;
    let mut u64_3: u64 = 4145u64;
    let mut u64_4: u64 = 669u64;
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_5: u64 = 403u64;
    let mut duration_0: date::Duration = crate::date::Duration::Week(u64_5);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_6: u64 = 1636u64;
    let mut u64_7: u64 = 8889u64;
    let mut u64_8: u64 = 4683u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_8, month: u64_7, day: u64_6};
    let mut i64_1: i64 = 15024i64;
    let mut u64_9: u64 = 7618u64;
    let mut str_0: &str = "c4OtwGTEM";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut vec_0: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut u64_10: u64 = 7131u64;
    let mut vec_1: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_11: u64 = 9091u64;
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_11, days: vec_1};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnDate(monthdeltadate_0);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    crate::data::Datafile::add_tag(datafile_0_ref_0, string_0);
    let mut vec_0_ref_0: &mut std::vec::Vec<crate::expense::Expense> = &mut vec_0;
    panic!("From RustyUnit with love");
}
}