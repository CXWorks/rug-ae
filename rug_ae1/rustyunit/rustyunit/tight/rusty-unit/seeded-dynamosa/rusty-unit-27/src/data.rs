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
#[timeout(30000)]fn rusty_test_140() {
//    rusty_monitor::set_test_id(140);
    let mut u64_0: u64 = 4114u64;
    let mut u64_1: u64 = 181u64;
    let mut u64_2: u64 = 90u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_2, u64_1, u64_0);
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut u64_3: u64 = 0u64;
    let mut u64_4: u64 = 0u64;
    let mut u64_5: u64 = 4u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_5, month: u64_4, day: u64_3};
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut vec_0: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut u64_6: u64 = 31u64;
    let mut u64_7: u64 = 1194u64;
    let mut vec_1: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut u64_8: u64 = 3654u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_8};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_9: u64 = 22u64;
    let mut duration_0: date::Duration = crate::date::Duration::Week(u64_9);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_10: u64 = 31u64;
    let mut u64_11: u64 = 6439u64;
    let mut u64_12: u64 = 7610u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_12, month: u64_11, day: u64_10};
    let mut i64_0: i64 = 100i64;
    let mut str_0: &str = "every {} days";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_13: u64 = 3437u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_13, string_0, i64_0, simpledate_2, option_1, option_0, vec_1);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut simpledate_3: &crate::date::SimpleDate = crate::expense::Expense::get_start_date(expense_0_ref_0);
    let mut duration_1: date::Duration = crate::date::Duration::Day(u64_7);
    let mut expense_slice_0: &[crate::expense::Expense] = crate::data::Datafile::expenses_between(datafile_0_ref_0, simpledate_1_ref_0, simpledate_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_68() {
//    rusty_monitor::set_test_id(68);
    let mut u64_0: u64 = 3u64;
    let mut u64_1: u64 = 212u64;
    let mut u64_2: u64 = 10u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_2, month: u64_1, day: u64_0};
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut u64_3: u64 = 21u64;
    let mut bool_0: bool = true;
    let mut u64_4: u64 = 5211u64;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Thursday;
    let mut u64_5: u64 = 2u64;
    let mut u64_6: u64 = 6u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_6, weekid: u64_5, day: weekday_0};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_7: u64 = 2727u64;
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_7);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_8: u64 = 2683u64;
    let mut u64_9: u64 = 151u64;
    let mut u64_10: u64 = 8725u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_10, month: u64_9, day: u64_8};
    let mut i64_0: i64 = -2048i64;
    let mut str_0: &str = "fourth";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_11: u64 = 9048u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_11, string_0, i64_0, simpledate_1, option_1, option_0, vec_0);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut option_2: &std::option::Option<crate::date::SimpleDate> = crate::expense::Expense::get_end_date(expense_0_ref_0);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_3);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6744() {
//    rusty_monitor::set_test_id(6744);
    let mut str_0: &str = "whUUPrJS0";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut u64_0: u64 = 10u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_0};
    let mut u64_1: u64 = 21u64;
    let mut u64_2: u64 = 7165u64;
    let mut u64_3: u64 = 304u64;
    let mut u64_4: u64 = 4u64;
    let mut u64_5: u64 = 4037u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_5, u64_4, u64_3);
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_2};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut option_0: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_6: u64 = 21u64;
    let mut u64_7: u64 = 5524u64;
    let mut u64_8: u64 = 400u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_8, u64_7, u64_6);
    let mut u64_9: u64 = 1u64;
    let mut u64_10: u64 = 365u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_10, u64_1, u64_9);
    let mut simpledate_2_ref_0: &crate::date::SimpleDate = &mut simpledate_2;
    let mut str_1: &str = "every {} year";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "48z4Oiif";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    crate::data::Datafile::add_tag(datafile_0_ref_0, string_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8501() {
//    rusty_monitor::set_test_id(8501);
    let mut u64_0: u64 = 243u64;
    let mut u64_1: u64 = 2717u64;
    let mut u64_2: u64 = 28u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_2, month: u64_1, day: u64_0};
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut option_0: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_3: u64 = 2560u64;
    let mut u64_4: u64 = 212u64;
    let mut u64_5: u64 = 120u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_5, month: u64_4, day: u64_3};
    let mut u64_6: u64 = 2u64;
    let mut u64_7: u64 = 0u64;
    let mut u64_8: u64 = 30u64;
    let mut u64_9: u64 = 4351u64;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut option_1: std::option::Option<&crate::expense::Expense> = crate::data::Datafile::find(datafile_0_ref_0, u64_9);
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_8, month: u64_7, day: u64_6};
    let mut expense_0: &crate::expense::Expense = std::option::Option::unwrap(option_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4822() {
//    rusty_monitor::set_test_id(4822);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Wednesday;
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_0: u64 = 10u64;
    let mut duration_0: date::Duration = crate::date::Duration::Year(u64_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_1: u64 = 12u64;
    let mut u64_2: u64 = 11u64;
    let mut u64_3: u64 = 151u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_3, month: u64_2, day: u64_1};
    let mut u64_4: u64 = 712u64;
    let mut u64_5: u64 = 4u64;
    let mut u64_6: u64 = 9247u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_6, u64_5, u64_4);
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7977() {
//    rusty_monitor::set_test_id(7977);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut str_0: &str = "";
    let mut u64_0: u64 = 5132u64;
    let mut u64_1: u64 = 6207u64;
    let mut u64_2: u64 = 1614u64;
    let mut u64_3: u64 = 243u64;
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_3);
    let mut option_0: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_4: u64 = 365u64;
    let mut u64_5: u64 = 3790u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_5, month: u64_0, day: u64_2};
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_1: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_2: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_6: u64 = 2405u64;
    let mut u64_7: u64 = 3486u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_1, month: u64_4, day: u64_6};
    let mut i64_0: i64 = 9223372036854775807i64;
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_7, string_0, i64_0, simpledate_1, option_2, option_1, vec_0);
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut vec_1: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    crate::data::Datafile::insert(datafile_0_ref_0, expense_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4883() {
//    rusty_monitor::set_test_id(4883);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut str_0: &str = "{}-{}-{}";
    let mut u64_0: u64 = 3442u64;
    let mut u64_1: u64 = 12u64;
    let mut u64_2: u64 = 3904u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_2, month: u64_1, day: u64_0};
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Sunday;
    let mut u64_3: u64 = 30u64;
    let mut u64_4: u64 = 6156u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_4, weekid: u64_3, day: weekday_0};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_5: u64 = 212u64;
    let mut u64_6: u64 = 100u64;
    let mut u64_7: u64 = 151u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_7, u64_6, u64_5);
    let mut i64_0: i64 = 9223372036854775807i64;
    let mut str_1: &str = "invalid end date";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut u64_8: u64 = 8u64;
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_8, string_0, i64_0, simpledate_1, option_1, option_0, vec_0);
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut vec_1: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    crate::data::Datafile::insert(datafile_0_ref_0, expense_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1437() {
//    rusty_monitor::set_test_id(1437);
    let mut u64_0: u64 = 5u64;
    let mut u64_1: u64 = 334u64;
    let mut u64_2: u64 = 8u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_2, month: u64_1, day: u64_0};
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut u64_3: u64 = 304u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_3};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_4: u64 = 100u64;
    let mut u64_5: u64 = 5u64;
    let mut u64_6: u64 = 7u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_6, month: u64_5, day: u64_4};
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_7: u64 = 30u64;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_7);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_8: u64 = 22u64;
    let mut u64_9: u64 = 4915u64;
    let mut u64_10: u64 = 4u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_10, u64_9, u64_8);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut str_0: &str = "pIq";
    let mut u64_11: u64 = 5302u64;
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Count(u64_11);
    let mut vec_0: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_12: u64 = 119u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_12, on: vec_0};
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Week(weekdelta_0);
    let mut repetition_1: crate::date::Repetition = crate::date::Repetition {delta: repdelta_1, end: repend_1};
    let mut u64_13: u64 = 9u64;
    let mut repend_2: date::RepEnd = crate::date::RepEnd::Count(u64_13);
    let mut u64_14: u64 = 5295u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_14};
    let mut repdelta_2: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut u64_15: u64 = 212u64;
    let mut yeardelta_1: crate::date::YearDelta = crate::date::YearDelta {nth: u64_15};
    let mut u64_16: u64 = 5755u64;
    let mut repend_3: date::RepEnd = crate::date::RepEnd::Count(u64_16);
    let mut u64_17: u64 = 1700u64;
    let mut daydelta_1: crate::date::DayDelta = crate::date::DayDelta {nth: u64_17};
    let mut repdelta_3: date::RepDelta = crate::date::RepDelta::Year(yeardelta_1);
    let mut repetition_2: crate::date::Repetition = crate::date::Repetition {delta: repdelta_2, end: repend_2};
    let mut option_4: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_1);
    let mut option_5: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_18: u64 = 3790u64;
    let mut u64_19: u64 = 243u64;
    let mut vec_1: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_6: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_7: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_20: u64 = 125u64;
    let mut u64_21: u64 = 3486u64;
    let mut simpledate_3: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_20, month: u64_18, day: u64_19};
    let mut i64_0: i64 = 9223372036854775807i64;
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_21, string_0, i64_0, simpledate_3, option_7, option_4, vec_1);
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut vec_2: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    crate::data::Datafile::insert(datafile_0_ref_0, expense_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7563() {
//    rusty_monitor::set_test_id(7563);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut i64_0: i64 = 290i64;
    let mut str_0: &str = "TZsaWzH9SFpTBGzSzO";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_0: u64 = 5512u64;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_0);
    let mut option_0: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_1: u64 = 151u64;
    let mut u64_2: u64 = 1552u64;
    let mut u64_3: u64 = 7457u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_3, month: u64_2, day: u64_1};
    let mut str_1: &str = "";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut u64_4: u64 = 6u64;
    let mut u64_5: u64 = 304u64;
    let mut u64_6: u64 = 4u64;
    let mut u64_7: u64 = 4037u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_7, u64_6, u64_5);
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut u64_8: u64 = 3u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_8);
    let mut u64_9: u64 = 29u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_9};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_1: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_2: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_10: u64 = 21u64;
    let mut u64_11: u64 = 5524u64;
    let mut u64_12: u64 = 400u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_12, u64_11, u64_10);
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_4, string_0, i64_0, simpledate_0, option_2, option_1, vec_0);
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut vec_1: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    crate::data::Datafile::insert(datafile_0_ref_0, expense_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_291() {
//    rusty_monitor::set_test_id(291);
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut u64_0: u64 = 4725u64;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_0);
    let mut u64_1: u64 = 29u64;
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut duration_1: date::Duration = crate::date::Duration::Day(u64_1);
    let mut u64_2: u64 = 5250u64;
    let mut u64_3: u64 = 243u64;
    let mut option_1: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_4: u64 = 6u64;
    let mut duration_2: date::Duration = crate::date::Duration::Week(u64_3);
    let mut option_2: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
    let mut u64_5: u64 = 3362u64;
    let mut u64_6: u64 = 4065u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_4, u64_2, u64_5);
    let mut datafile_1: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &mut crate::data::Datafile = &mut datafile_0;
    let mut result_0: std::result::Result<(), std::boxed::Box<dyn std::error::Error>> = crate::data::Datafile::remove(datafile_0_ref_0, u64_6);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5852() {
//    rusty_monitor::set_test_id(5852);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Wednesday;
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_0: u64 = 10u64;
    let mut duration_0: date::Duration = crate::date::Duration::Year(u64_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_1: u64 = 12u64;
    let mut u64_2: u64 = 11u64;
    let mut u64_3: u64 = 151u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_3, month: u64_2, day: u64_1};
    let mut u64_4: u64 = 31u64;
    let mut u64_5: u64 = 5292u64;
    let mut u64_6: u64 = 1700u64;
    let mut u64_7: u64 = 6954u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_7, u64_6, u64_5);
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut u64_8: u64 = 7677u64;
    let mut u64_9: u64 = 9247u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_9, u64_4, u64_8);
    let mut simpledate_2_ref_0: &crate::date::SimpleDate = &mut simpledate_2;
    let mut datafile_0: crate::data::Datafile = crate::data::Datafile::new();
    let mut datafile_0_ref_0: &crate::data::Datafile = &mut datafile_0;
    let mut expense_slice_0: &[crate::expense::Expense] = crate::data::Datafile::expenses_between(datafile_0_ref_0, simpledate_2_ref_0, simpledate_1_ref_0);
    let mut duration_1: date::Duration = std::option::Option::unwrap(option_1);
//    panic!("From RustyUnit with love");
}
}