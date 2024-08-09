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
                std::cmp::Ordering::Greater => {
                    insert_idx = idx;
                    break;
                }
                std::cmp::Ordering::Less => {
                    insert_idx = idx + 1;
                }
                std::cmp::Ordering::Equal => {
                    insert_idx = idx + 1;
                }
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
        let file = OpenOptions::new().write(true).truncate(true).open(path)?;
        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer(writer, &self)?;
        Ok(())
    }
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
    let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;
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
        let expense = Expense::new(
            0,
            "test".into(),
            100,
            SimpleDate::from_ymd(2020, 10, 14),
            None,
            None,
            vec!(),
        );
        datafile.insert(expense);
        assert_eq!(datafile.entries.len(), 1);
    }
    #[test]
    fn insert_sorted() {
        let mut datafile = Datafile::new();
        let expense1 = Expense::new(
            0,
            "test".into(),
            100,
            SimpleDate::from_ymd(2020, 10, 14),
            None,
            None,
            vec!(),
        );
        let expense2 = Expense::new(
            1,
            "test".into(),
            101,
            SimpleDate::from_ymd(2020, 10, 15),
            None,
            None,
            vec!(),
        );
        datafile.insert(expense2);
        datafile.insert(expense1);
        assert_eq!(datafile.entries.len(), 2);
        assert_eq!(datafile.entries[0].amount(), 100);
        assert_eq!(datafile.entries[1].amount(), 101);
    }
    #[test]
    fn remove() {
        let mut datafile = Datafile::new();
        let expense1 = Expense::new(
            0,
            "test".into(),
            100,
            SimpleDate::from_ymd(2020, 10, 14),
            None,
            None,
            vec!(),
        );
        let expense2 = Expense::new(
            1,
            "test".into(),
            101,
            SimpleDate::from_ymd(2020, 10, 15),
            None,
            None,
            vec!(),
        );
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
        let expense1 = Expense::new(
            0,
            "test".into(),
            100,
            SimpleDate::from_ymd(2020, 10, 14),
            None,
            None,
            vec!(),
        );
        let expense2 = Expense::new(
            1,
            "test".into(),
            101,
            SimpleDate::from_ymd(2020, 10, 15),
            None,
            None,
            vec!(),
        );
        datafile.insert(expense2);
        datafile.insert(expense1);
        assert!(datafile.find(9999).is_none());
        assert!(datafile.find(1).is_some());
        assert_eq!(datafile.find(1).unwrap().amount(), 101);
    }
}
#[cfg(test)]
mod tests_llm_16_12 {
    use super::*;
    use crate::*;
    use std::path::Path;
    use serde_json;
    #[test]
    fn test_add_tag() {
        let _rug_st_tests_llm_16_12_rrrruuuugggg_test_add_tag = 0;
        let rug_fuzz_0 = "test_tag";
        let mut data = Datafile::new();
        let tag = String::from(rug_fuzz_0);
        data.add_tag(tag.clone());
        debug_assert!(data.tags.contains(& tag));
        let _rug_ed_tests_llm_16_12_rrrruuuugggg_test_add_tag = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_17 {
    use super::*;
    use crate::*;
    use std::path::Path;
    #[test]
    fn test_from_file() {
        let _rug_st_tests_llm_16_17_rrrruuuugggg_test_from_file = 0;
        let rug_fuzz_0 = "example.json";
        let path: &Path = Path::new(rug_fuzz_0);
        let result = Datafile::from_file(path);
        debug_assert!(result.is_ok());
        let datafile = result.unwrap();
        debug_assert_eq!(datafile.version, 1);
        let _rug_ed_tests_llm_16_17_rrrruuuugggg_test_from_file = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_18 {
    use super::*;
    use crate::*;
    use std::path::PathBuf;
    #[test]
    fn test_insert() {
        let _rug_st_tests_llm_16_18_rrrruuuugggg_test_insert = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = "Expense 1";
        let rug_fuzz_2 = 100;
        let rug_fuzz_3 = 2022;
        let rug_fuzz_4 = 1;
        let rug_fuzz_5 = 1;
        let mut datafile = Datafile::new();
        let expense = Expense::new(
            rug_fuzz_0,
            rug_fuzz_1.into(),
            rug_fuzz_2,
            SimpleDate::from_ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5),
            None,
            None,
            vec![],
        );
        datafile.insert(expense);
        debug_assert_eq!(datafile.entries.len(), 1);
        let _rug_ed_tests_llm_16_18_rrrruuuugggg_test_insert = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_19 {
    use super::*;
    use crate::*;
    #[test]
    fn test_new() {
        let _rug_st_tests_llm_16_19_rrrruuuugggg_test_new = 0;
        let datafile = Datafile::new();
        debug_assert_eq!(datafile.version, 1);
        debug_assert_eq!(datafile.tags.len(), 0);
        debug_assert_eq!(datafile.entries.len(), 0);
        let _rug_ed_tests_llm_16_19_rrrruuuugggg_test_new = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_24 {
    use super::*;
    use crate::*;
    use std::fs::File;
    use std::io::{Read, Write};
    #[test]
    fn test_initialise() {
        let _rug_st_tests_llm_16_24_rrrruuuugggg_test_initialise = 0;
        let rug_fuzz_0 = "test_data.json";
        let path = rug_fuzz_0;
        let result = initialise(path);
        debug_assert!(result.is_ok());
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let expected_contents = serde_json::to_string(&Datafile::new()).unwrap();
        debug_assert_eq!(contents, expected_contents);
        std::fs::remove_file(path).unwrap();
        let _rug_ed_tests_llm_16_24_rrrruuuugggg_test_initialise = 0;
    }
}
#[cfg(test)]
mod tests_rug_1 {
    use super::*;
    use crate::data::Datafile;
    #[test]
    fn test_remove() {
        let _rug_st_tests_rug_1_rrrruuuugggg_test_remove = 0;
        let rug_fuzz_0 = 42;
        let mut p0 = Datafile::new();
        let p1: u64 = rug_fuzz_0;
        p0.remove(p1).unwrap();
        let _rug_ed_tests_rug_1_rrrruuuugggg_test_remove = 0;
    }
}
#[cfg(test)]
mod tests_rug_2 {
    use super::*;
    use crate::data::{Datafile, Expense};
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_2_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 12345;
        let mut p0 = Datafile::new();
        let p1: u64 = rug_fuzz_0;
        p0.find(p1);
        let _rug_ed_tests_rug_2_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_3 {
    use super::*;
    use std::path::Path;
    use std::io::Write;
    use std::fs::OpenOptions;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_3_rrrruuuugggg_sample = 0;
        #[cfg(test)]
        mod tests_rug_3_prepare {
            use super::*;
            #[test]
            fn sample() {
                let _rug_st_tests_rug_3_prepare_rrrruuuugggg_sample = 0;
                let rug_fuzz_0 = 0;
                let rug_fuzz_1 = 0;
                let _rug_st_tests_rug_3_rrrruuuugggg_sample = rug_fuzz_0;
                let mut v1 = Datafile::new();
                let _rug_ed_tests_rug_3_rrrruuuugggg_sample = rug_fuzz_1;
                let _rug_ed_tests_rug_3_prepare_rrrruuuugggg_sample = 0;
            }
        }
        let mut p0 = Datafile::new();
        let p1: &Path = Path::new("test.json");
        crate::data::Datafile::save(&p0, &p1);
        let file_exists = Path::new("test.json").exists();
        assert_eq!(file_exists, true);
        std::fs::remove_file("test.json").unwrap();
        let _rug_ed_tests_rug_3_rrrruuuugggg_sample = 0;
    }
}
#[cfg(test)]
mod tests_rug_4 {
    use super::*;
    use crate::data::Datafile;
    use crate::date::SimpleDate;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_4_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 2021;
        let rug_fuzz_1 = 9;
        let rug_fuzz_2 = 14;
        let rug_fuzz_3 = 2021;
        let rug_fuzz_4 = 9;
        let rug_fuzz_5 = 15;
        let mut p0 = Datafile::new();
        let mut p1 = SimpleDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let mut p2 = SimpleDate::from_ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        Datafile::expenses_between(&mut p0, &mut p1, &mut p2);
        let _rug_ed_tests_rug_4_rrrruuuugggg_test_rug = 0;
    }
}
