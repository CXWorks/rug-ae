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
fn rusty_test_4798() {
    rusty_monitor::set_test_id(4798);
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_0: u64 = 1970u64;
    let mut u64_1: u64 = 512u64;
    let mut u64_2: u64 = 9264u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_2, u64_1, u64_0);
    let mut i64_0: i64 = 2377i64;
    let mut u64_3: u64 = 1905u64;
    let mut u64_4: u64 = 6665u64;
    let mut u64_5: u64 = 8233u64;
    let mut u64_6: u64 = 5536u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_6, month: u64_5, day: u64_4};
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_1);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Monday;
    let mut u64_7: u64 = 7755u64;
    let mut u64_8: u64 = 9812u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_8, weekid: u64_7, day: weekday_0};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_9: u64 = 4190u64;
    let mut u64_10: u64 = 3929u64;
    let mut u64_11: u64 = 2848u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_11, u64_10, u64_9);
    let mut i64_1: i64 = -6455i64;
    let mut u64_12: u64 = 9236u64;
    let mut u64_13: u64 = 4057u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_13};
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1604() {
    rusty_monitor::set_test_id(1604);
    let mut u64_0: u64 = 2972u64;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_0);
    let mut duration_0_ref_0: &date::Duration = &mut duration_0;
    let mut u64_1: u64 = 3101u64;
    let mut u64_2: u64 = 4746u64;
    let mut u64_3: u64 = 2996u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_3, month: u64_2, day: u64_1};
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_4: u64 = 6506u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_4);
    let mut u64_5: u64 = 9096u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_5};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_6: u64 = 6901u64;
    let mut u64_7: u64 = 8172u64;
    let mut u64_8: u64 = 3849u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_8, u64_7, u64_6);
    let mut i64_0: i64 = 6285i64;
    let mut str_0: &str = "2iWI3zc";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_9: u64 = 8635u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_9, string_0, i64_0, simpledate_1, option_1, option_0, vec_0);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut u64_10: u64 = 3368u64;
    let mut u64_11: u64 = 2043u64;
    let mut u64_12: u64 = 6929u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_12, u64_11, u64_10);
    let mut vec_1: &std::vec::Vec<std::string::String> = crate::expense::Expense::tags(expense_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4390() {
    rusty_monitor::set_test_id(4390);
    let mut u64_0: u64 = 7074u64;
    let mut u64_1: u64 = 6995u64;
    let mut u64_2: u64 = 9069u64;
    let mut u64_3: u64 = 3454u64;
    let mut u64_4: u64 = 8437u64;
    let mut u64_5: u64 = 221u64;
    let mut u64_6: u64 = 7031u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_6, month: u64_5, day: u64_4};
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut u64_7: u64 = 2266u64;
    let mut u64_8: u64 = 7104u64;
    let mut u64_9: u64 = 1710u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_9, month: u64_8, day: u64_7};
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut u64_10: u64 = 2424u64;
    let mut u64_11: u64 = 1458u64;
    let mut u64_12: u64 = 8175u64;
    let mut u64_13: u64 = 3423u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_13};
    let mut str_0: &str = "luVnSWlgpeckR2";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_1_ref_0: &mut crate::data::Datafile = &mut datafile_1;
    crate::data::Datafile::add_tag(datafile_1_ref_0, string_0);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_12, month: u64_11, day: u64_10};
    let mut datafile_2: crate::data::Datafile = crate::data::Datafile::new();
    let mut expense_slice_0: &[crate::expense::Expense] = crate::data::Datafile::expenses_between(datafile_0_ref_0, simpledate_1_ref_0, simpledate_0_ref_0);
    let mut simpledate_3: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_3, month: u64_2, day: u64_1};
    let mut weekday_0: date::Weekday = crate::date::Weekday::Tuesday;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_403() {
    rusty_monitor::set_test_id(403);
    let mut u64_0: u64 = 8499u64;
    let mut u64_1: u64 = 3783u64;
    let mut u64_2: u64 = 9694u64;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_3: u64 = 809u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_3);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Saturday;
    let mut u64_4: u64 = 9728u64;
    let mut u64_5: u64 = 1308u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_5, weekid: u64_4, day: weekday_0};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_6: u64 = 4405u64;
    let mut u64_7: u64 = 3338u64;
    let mut u64_8: u64 = 8293u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_8, month: u64_7, day: u64_6};
    let mut i64_0: i64 = 10083i64;
    let mut str_0: &str = "NRaoPdjmks47zRY0NY";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_9: u64 = 8556u64;
    let mut u64_10: u64 = 1832u64;
    let mut u64_11: u64 = 4120u64;
    let mut u64_12: u64 = 4004u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_12, u64_11, u64_10);
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut u64_13: u64 = 7614u64;
    let mut weekday_1: date::Weekday = crate::date::Weekday::Wednesday;
    let mut duration_0: date::Duration = crate::date::Duration::Week(u64_13);
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_9, string_0, i64_0, simpledate_0, option_1, option_0, vec_0);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut duration_1: date::Duration = crate::date::Duration::Year(u64_2);
    let mut duration_2: date::Duration = crate::date::Duration::Day(u64_1);
    let mut duration_3: date::Duration = crate::date::Duration::Day(u64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2975() {
    rusty_monitor::set_test_id(2975);
    let mut u64_0: u64 = 9526u64;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_1: u64 = 9012u64;
    let mut u64_2: u64 = 4956u64;
    let mut u64_3: u64 = 8801u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_3, month: u64_2, day: u64_1};
    let mut i64_0: i64 = -4421i64;
    let mut str_0: &str = "d9LE6oR";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_4: u64 = 8957u64;
    let mut vec_1: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_5: u64 = 2867u64;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_5);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_6: u64 = 9066u64;
    let mut u64_7: u64 = 8414u64;
    let mut u64_8: u64 = 3629u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_8, month: u64_7, day: u64_6};
    let mut i64_1: i64 = 1024i64;
    let mut str_1: &str = "jyum83iZWXFJdWu";
    let mut string_1: std::string::String = std::string::String::from(str_1);
    let mut u64_9: u64 = 5823u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_9, string_1, i64_1, simpledate_1, option_3, option_2, vec_1);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Friday;
    let mut simpledate_2: &crate::date::SimpleDate = crate::expense::Expense::get_start_date(expense_0_ref_0);
    let mut expense_1: crate::expense::Expense = crate::expense::Expense::new(u64_4, string_0, i64_0, simpledate_0, option_1, option_0, vec_0);
    let mut option_4: std::option::Option<&crate::expense::Expense> = crate::data::Datafile::find(datafile_0_ref_0, u64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_90() {
    rusty_monitor::set_test_id(90);
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_0: u64 = 8278u64;
    let mut u64_1: u64 = 2998u64;
    let mut u64_2: u64 = 4538u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_2, month: u64_1, day: u64_0};
    let mut i64_0: i64 = -4306i64;
    let mut u64_3: u64 = 4600u64;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut u64_4: u64 = 2557u64;
    let mut u64_5: u64 = 998u64;
    let mut u64_6: u64 = 9759u64;
    let mut u64_7: u64 = 4625u64;
    let mut u64_8: u64 = 1785u64;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Friday;
    let mut u64_9: u64 = 887u64;
    let mut u64_10: u64 = 6249u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_10, weekid: u64_9, day: weekday_0};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    let mut u64_11: u64 = 4297u64;
    let mut u64_12: u64 = 5891u64;
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_1_ref_0: &crate::data::Datafile = &mut datafile_1;
    let mut option_2: std::option::Option<&crate::expense::Expense> = crate::data::Datafile::find(datafile_1_ref_0, u64_12);
    let mut weekday_1: date::Weekday = crate::date::Weekday::Sunday;
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut weekday_2: date::Weekday = crate::date::Weekday::Wednesday;
    let mut expense_0: &crate::expense::Expense = std::option::Option::unwrap(option_2);
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_7, month: u64_6, day: u64_5};
    let mut weekday_3: date::Weekday = crate::date::Weekday::Wednesday;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4123() {
    rusty_monitor::set_test_id(4123);
    let mut u64_0: u64 = 1232u64;
    let mut u64_1: u64 = 1232u64;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut u64_2: u64 = 1602u64;
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_1_ref_0: &mut crate::data::Datafile = &mut datafile_1;
    let mut vec_0: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut u64_3: u64 = 8403u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut u64_4: u64 = 3625u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_4};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut vec_1: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_5: u64 = 5028u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_5, on: vec_1};
    let mut u64_6: u64 = 7074u64;
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Never;
    let mut repend_2: date::RepEnd = crate::date::RepEnd::Never;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_6);
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Week(weekdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut result_0: std::result::Result<(), std::boxed::Box<dyn std::error::Error>> = crate::data::Datafile::remove(datafile_1_ref_0, u64_2);
    let mut vec_0_ref_0: &mut std::vec::Vec<crate::expense::Expense> = &mut vec_0;
    let mut option_0: std::option::Option<crate::expense::Expense> = std::vec::Vec::pop(vec_0_ref_0);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Monday;
    let mut weekday_1: date::Weekday = crate::date::Weekday::Tuesday;
    let mut weekday_2: date::Weekday = crate::date::Weekday::Tuesday;
    let mut weekday_3: date::Weekday = crate::date::Weekday::Friday;
    let mut duration_1: date::Duration = crate::date::Duration::Month(u64_1);
    let mut datafile_2: crate::data::Datafile = crate::data::Datafile::new();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1870() {
    rusty_monitor::set_test_id(1870);
    let mut u64_0: u64 = 9238u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_0};
    let mut u64_1: u64 = 9101u64;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut vec_0: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut u64_2: u64 = 1931u64;
    let mut u64_3: u64 = 4319u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_3};
    let mut u64_4: u64 = 5895u64;
    let mut vec_1: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_5: u64 = 6601u64;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_5);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_6: u64 = 3499u64;
    let mut u64_7: u64 = 4252u64;
    let mut u64_8: u64 = 9312u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_8, month: u64_7, day: u64_6};
    let mut i64_0: i64 = 12357i64;
    let mut str_0: &str = "";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_9: u64 = 5068u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_9, string_0, i64_0, simpledate_0, option_1, option_0, vec_1);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut u64_10: u64 = 4991u64;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Thursday;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut duration_1: date::Duration = crate::date::Duration::Month(u64_10);
    let mut bool_0: bool = crate::expense::Expense::compare_id(expense_0_ref_0, u64_4);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut result_0: std::result::Result<(), std::boxed::Box<dyn std::error::Error>> = crate::data::Datafile::remove(datafile_0_ref_0, u64_1);
    let mut vec_0_ref_0: &mut std::vec::Vec<crate::expense::Expense> = &mut vec_0;
    let mut option_2: std::option::Option<crate::expense::Expense> = std::vec::Vec::pop(vec_0_ref_0);
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4260() {
    rusty_monitor::set_test_id(4260);
    let mut u64_0: u64 = 9402u64;
    let mut u64_1: u64 = 204u64;
    let mut u64_2: u64 = 8875u64;
    let mut u64_3: u64 = 4044u64;
    let mut u64_4: u64 = 3228u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_4, month: u64_3, day: u64_2};
    let mut u64_5: u64 = 203u64;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut u64_6: u64 = 3186u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_6};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_7: u64 = 4489u64;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_7);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_8: u64 = 8852u64;
    let mut u64_9: u64 = 8434u64;
    let mut u64_10: u64 = 8848u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_10, u64_9, u64_8);
    let mut i64_0: i64 = 10643i64;
    let mut str_0: &str = "SOgNfMg";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_11: u64 = 4191u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_11, string_0, i64_0, simpledate_1, option_1, option_0, vec_0);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut vec_1: &std::vec::Vec<std::string::String> = crate::expense::Expense::tags(expense_0_ref_0);
    let mut duration_1: date::Duration = crate::date::Duration::Day(u64_5);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut duration_2: date::Duration = crate::date::Duration::Year(u64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3030() {
    rusty_monitor::set_test_id(3030);
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_0: u64 = 6673u64;
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_1: u64 = 2477u64;
    let mut u64_2: u64 = 9994u64;
    let mut u64_3: u64 = 9411u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_3, month: u64_2, day: u64_1};
    let mut i64_0: i64 = -2758i64;
    let mut u64_4: u64 = 6522u64;
    let mut u64_5: u64 = 5020u64;
    let mut u64_6: u64 = 4922u64;
    let mut u64_7: u64 = 5544u64;
    let mut u64_8: u64 = 8190u64;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_9: u64 = 8146u64;
    let mut u64_10: u64 = 372u64;
    let mut u64_11: u64 = 9107u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_11, month: u64_10, day: u64_9};
    let mut i64_1: i64 = 1152i64;
    let mut str_0: &str = "pCtabdZPRCh";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_12: u64 = 592u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_12, string_0, i64_1, simpledate_1, option_3, option_2, vec_0);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut u64_13: u64 = 2183u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_13};
    let mut weekday_0: date::Weekday = crate::date::Weekday::Tuesday;
    crate::data::Datafile::insert(datafile_0_ref_0, expense_0);
    let mut weekday_1: date::Weekday = crate::date::Weekday::Wednesday;
    let mut duration_1: date::Duration = crate::date::Duration::Month(u64_7);
    let mut daydelta_1: crate::date::DayDelta = crate::date::DayDelta {nth: u64_6};
    panic!("From RustyUnit with love");
}
}