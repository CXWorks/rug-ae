/*
 * TODO
 * review all unwrap() calls, some are dodgy
 */
use crate::date::*;

use serde::{Serialize, Deserialize};

use std::collections::HashSet;
use std::convert::TryInto;
use std::error::Error;
use std::fmt;
use std::io::BufRead;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
pub struct Expense {
    id: u64,
    description: String,
    amount: i64, // cents
    start: SimpleDate,
    end: Option<SimpleDate>,
    spread: Option<Duration>,
    repetition: Option<Repetition>,
    tags: Vec<String>,
}

#[derive(Debug)]
struct ExpenseError(String);

impl Expense {
    pub fn new(id: u64, description: String, amount: i64, start: SimpleDate,
               spread: Option<Duration>, repetition: Option<Repetition>,
               tags: Vec<String>) -> Expense {
        Expense {
            id,
            description,
            amount,
            start,
            end: Expense::end_date(&start, &repetition, &spread),
            spread,
            repetition,
            tags,
        }
    }

    pub fn from_stdin(mut handle: &mut std::io::StdinLock, id: u64,
                      is_income: bool, allowed_tags: &HashSet<String>)
                      -> Result<Expense, Box<dyn Error>> {
        print!("description: ");
        std::io::stdout().flush()?;
        let mut description = String::new();
        handle.read_line(&mut description)?;

        print!("amount: ");
        std::io::stdout().flush()?;
        let mut amount_s = String::new();
        handle.read_line(&mut amount_s)?;
        let amount_f: f64 = if let Some(stripped) = amount_s.trim().strip_prefix("$") {
            stripped.parse()?
        } else {
            amount_s.trim().parse()?
        };
        let amount: i64 = (amount_f * 100.0).trunc() as i64;

        let start = SimpleDate::from_stdin(&mut handle)?;

        print!("spread (blank for none): ");
        std::io::stdout().flush()?;
        let mut spread_s = String::new();
        handle.read_line(&mut spread_s)?;
        spread_s.make_ascii_lowercase();
        let spread = if spread_s.trim().is_empty() {
            None
        } else {
            let result = scan_fmt::scan_fmt!(&spread_s, "{} {}", u64, String)?;
            if result.0 == 0 {
                None
            } else {
                match &result.1[..] {
                    "day" | "days"     => Some(Duration::Day(result.0)),
                    "week" | "weeks"   => Some(Duration::Week(result.0)),
                    "month" | "months" => Some(Duration::Month(result.0)),
                    "year" | "years"   => Some(Duration::Year(result.0)),
                    _     => { return Err(Box::new(ExpenseError("invalid spread: only day/week/month/year(s) accepted".into()))); },
                }
            }
        };

        let repetition = Repetition::from_stdin(&mut handle, &start)?;

        print!("tags (comma- or space-separated): ");
        std::io::stdout().flush()?;
        let mut tags_s = String::new();
        handle.read_line(&mut tags_s)?;
        let tags = tags_s.split(|c| c == ' ' || c == ',')
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.trim().to_string())
            .collect();
        for t in &tags {
            if !allowed_tags.contains(t) {
                return Err(Box::new(ExpenseError("tag not found!".into())));
            }
        }

        Ok(Expense::new(id,
                        description.trim().to_string(),
                        if is_income { amount } else { -amount },
                        start,
                        spread,
                        repetition,
                        tags))
    }

    // Greater if this ends after other, otherwise Less
    // start date used as tie-breaker, can return Equal
    pub fn compare_dates(&self, other: &Expense) -> std::cmp::Ordering {
        if self.end.is_none() && other.end.is_none() {
            return std::cmp::Ordering::Equal;
        } else if self.end.is_none() {
            return std::cmp::Ordering::Greater;
        } else if other.end.is_none() {
            return std::cmp::Ordering::Less;
        }

        let self_end = &self.end.unwrap();
        let other_end = &other.end.unwrap();
        match self_end.cmp(&other_end) {
            std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
            std::cmp::Ordering::Less    => return std::cmp::Ordering::Less,
            _ => (),
        }

        // end dates match, use start
        match self.start.cmp(&other.start) {
            std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
            std::cmp::Ordering::Less    => return std::cmp::Ordering::Less,
            _ => (),
        }

        // start dates also match
        std::cmp::Ordering::Equal
    }

    pub fn compare_id(&self, other_id: u64) -> bool {
        self.id == other_id
    }

    pub fn amount(&self) -> i64 {
        self.amount
    }

    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    pub fn tags(&self) -> &Vec<String> {
        &self.tags
    }

    pub fn get_end_date(&self) -> &Option<SimpleDate> {
        &self.end
    }

    pub fn get_start_date(&self) -> &SimpleDate {
        &self.start
    }

    pub fn remove_tags(&mut self, tag: &str) {
        self.tags.retain(|t| t != tag);
    }

    fn end_date(start: &SimpleDate, repetition: &Option<Repetition>, spread: &Option<Duration>) -> Option<SimpleDate> {
        let mut end = *start;

        if let Some(r) = repetition {
            match r.end {
                RepEnd::Never => return None,
                _ => end = &end + r,
            }
        }

        if let Some(s) = spread {
            end = &end + s;
        }

        Some(end)
    }
}

fn count_overlap_days(period_start: &SimpleDate, period_end: &SimpleDate,
                      expense_start: &SimpleDate, expense_end: &SimpleDate) -> u64 {
    // exclusion
    if expense_end < period_start || expense_start > period_end {
        return 0;
    }

    let chr_period_start = chrono::NaiveDate::from_ymd(period_start.year.try_into().unwrap(),
                                                       period_start.month.try_into().unwrap(),
                                                       period_start.day.try_into().unwrap());
    let chr_period_end = chrono::NaiveDate::from_ymd(period_end.year.try_into().unwrap(),
                                                     period_end.month.try_into().unwrap(),
                                                     period_end.day.try_into().unwrap());
    let chr_ex_start = chrono::NaiveDate::from_ymd(expense_start.year.try_into().unwrap(),
                                                   expense_start.month.try_into().unwrap(),
                                                   expense_start.day.try_into().unwrap());
    let chr_ex_end = chrono::NaiveDate::from_ymd(expense_end.year.try_into().unwrap(),
                                                 expense_end.month.try_into().unwrap(),
                                                 expense_end.day.try_into().unwrap());

    // containment
    if expense_start >= period_start && expense_end < period_end {
        // period contains expense
        return chr_ex_end.signed_duration_since(chr_ex_start).num_days() as u64;
    } else if period_start >= expense_start && period_end < expense_end {
        // expense contains period
        return chr_period_end.signed_duration_since(chr_period_start).num_days() as u64;
    }

    // date ranges must overlap
    if expense_end < period_end {
        // overlap at start
        return chr_ex_end.signed_duration_since(chr_period_start).num_days() as u64;
    } else {
        // overlap at end
        return chr_period_end.signed_duration_since(chr_ex_start).num_days() as u64;
    }
}

pub fn calculate_spread(expenses: &[Expense], start: &SimpleDate, period: &Duration) -> f64 {
    let end = start + period;
    let mut sum = 0.0;

    for expense in expenses {
        // find repetitions that overlap with (start + period)
        // pro-rata those across running total
        let spread = expense.spread.as_ref().unwrap_or(&Duration::Day(1));

        let mut current_date = expense.start;
        if let Some(repetition) = &expense.repetition {
            while current_date < end {
                let spread_end = &current_date + spread;
                let spread_end_chr = chrono::NaiveDate::from_ymd(spread_end.year.try_into().unwrap(),
                                                                 spread_end.month.try_into().unwrap(),
                                                                 spread_end.day.try_into().unwrap());
                let current_date_chr = chrono::NaiveDate::from_ymd(current_date.year.try_into().unwrap(),
                                                                   current_date.month.try_into().unwrap(),
                                                                   current_date.day.try_into().unwrap());
                let n_days = spread_end_chr.signed_duration_since(current_date_chr).num_days() as f64;
                let amount_per_day = (expense.amount as f64) / n_days;
                let overlap_days = count_overlap_days(start, &end, &current_date, &(&current_date + spread));
                sum += amount_per_day * (overlap_days as f64);

                current_date = &current_date + &repetition.delta;
            }
        } else {
            let spread_end = &current_date + spread;
            let spread_end_chr = chrono::NaiveDate::from_ymd(spread_end.year.try_into().unwrap(),
                                                             spread_end.month.try_into().unwrap(),
                                                             spread_end.day.try_into().unwrap());
            let current_date_chr = chrono::NaiveDate::from_ymd(current_date.year.try_into().unwrap(),
                                                               current_date.month.try_into().unwrap(),
                                                               current_date.day.try_into().unwrap());
            let n_days = spread_end_chr.signed_duration_since(current_date_chr).num_days() as f64;
            let amount_per_day = (expense.amount as f64) / n_days;
            let overlap_days = count_overlap_days(start, &end, &current_date, &(&current_date + spread));
            sum += amount_per_day * (overlap_days as f64);
        }
    }

    sum / 100.0
}

impl fmt::Display for Expense {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: ${}.{:02} on {}", self.description,  self.amount.abs() / 100, self.amount.abs() % 100, self.start)?;

        if self.spread.is_some() || self.repetition.is_some() {
            write!(f, " (")?;

            if self.spread.is_some() {
                write!(f, "spread over {}", self.spread.as_ref().unwrap())?;
                if self.repetition.is_some() {
                    write!(f, ", ")?;
                }
            }

            if self.repetition.is_some() {
                write!(f, "repeats every {}", self.repetition.as_ref().unwrap())?;
            }
            write!(f, ")")?;
        }

        if !self.tags.is_empty() {
            write!(f, " tags: {}", self.tags[0])?;
            for tag in &self.tags[1..] {
                write!(f, ", {}", tag)?;
            }
        }

        write!(f, " [id={}]", self.id)
    }
}

impl fmt::Display for ExpenseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_overlap_days_exclusion_left() {
        let period_start = SimpleDate::from_ymd(2020, 11, 1);
        let period_end = SimpleDate::from_ymd(2020, 11, 30);

        let expense_start = SimpleDate::from_ymd(2020, 10, 1);
        let expense_end = SimpleDate::from_ymd(2020, 10, 31);

        let overlap_days = count_overlap_days(&period_start, &period_end, &expense_start, &expense_end);

        assert_eq!(overlap_days, 0);
    }

    #[test]
    fn count_overlap_days_exclusion_right() {
        let period_start = SimpleDate::from_ymd(2020, 11, 1);
        let period_end = SimpleDate::from_ymd(2020, 11, 30);

        let expense_start = SimpleDate::from_ymd(2020, 12, 1);
        let expense_end = SimpleDate::from_ymd(2020, 12, 31);

        let overlap_days = count_overlap_days(&period_start, &period_end, &expense_start, &expense_end);

        assert_eq!(overlap_days, 0);
    }

    #[test]
    fn count_overlap_days_containment_inner() {
        let period_start = SimpleDate::from_ymd(2020, 11, 1);
        let period_end = SimpleDate::from_ymd(2020, 11, 30);

        let expense_start = SimpleDate::from_ymd(2020, 11, 2);
        let expense_end = SimpleDate::from_ymd(2020, 11, 29);

        let overlap_days = count_overlap_days(&period_start, &period_end, &expense_start, &expense_end);

        assert_eq!(overlap_days, 27);
    }

    #[test]
    fn count_overlap_days_containment_outer() {
        let period_start = SimpleDate::from_ymd(2020, 11, 1);
        let period_end = SimpleDate::from_ymd(2020, 11, 30);

        let expense_start = SimpleDate::from_ymd(2020, 10, 31);
        let expense_end = SimpleDate::from_ymd(2020, 12, 1);

        let overlap_days = count_overlap_days(&period_start, &period_end, &expense_start, &expense_end);

        assert_eq!(overlap_days, 29);
    }

    #[test]
    fn count_overlap_days_edge_left() {
        let period_start = SimpleDate::from_ymd(2020, 11, 1);
        let period_end = SimpleDate::from_ymd(2020, 11, 30);

        let expense_start = SimpleDate::from_ymd(2020, 10, 15);
        let expense_end = SimpleDate::from_ymd(2020, 11, 15);

        let overlap_days = count_overlap_days(&period_start, &period_end, &expense_start, &expense_end);

        assert_eq!(overlap_days, 14);
    }

    #[test]
    fn count_overlap_days_edge_right() {
        let period_start = SimpleDate::from_ymd(2020, 11, 1);
        let period_end = SimpleDate::from_ymd(2020, 11, 30);

        let expense_start = SimpleDate::from_ymd(2020, 11, 15);
        let expense_end = SimpleDate::from_ymd(2020, 12, 15);

        let overlap_days = count_overlap_days(&period_start, &period_end, &expense_start, &expense_end);

        assert_eq!(overlap_days, 15);
    }
}

impl Error for ExpenseError {}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1095() {
    rusty_monitor::set_test_id(1095);
    let mut option_0: std::option::Option<date::Duration> = std::option::Option::None;
    let mut option_0_ref_0: &std::option::Option<date::Duration> = &mut option_0;
    let mut option_1: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_1_ref_0: &std::option::Option<crate::date::Repetition> = &mut option_1;
    let mut u64_0: u64 = 822u64;
    let mut u64_1: u64 = 4796u64;
    let mut u64_2: u64 = 6822u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_2, month: u64_1, day: u64_0};
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_3: u64 = 8311u64;
    let mut duration_0: date::Duration = crate::date::Duration::Year(u64_3);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_4: u64 = 3441u64;
    let mut u64_5: u64 = 4274u64;
    let mut u64_6: u64 = 5704u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_6, month: u64_5, day: u64_4};
    let mut i64_0: i64 = -6040i64;
    let mut str_0: &str = "";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_7: u64 = 2447u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_7, string_0, i64_0, simpledate_1, option_3, option_2, vec_0);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut vec_1: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_4: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_8: u64 = 9997u64;
    let mut duration_1: date::Duration = crate::date::Duration::Day(u64_8);
    let mut option_5: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
    let mut u64_9: u64 = 6342u64;
    let mut u64_10: u64 = 3664u64;
    let mut u64_11: u64 = 3780u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_11, month: u64_10, day: u64_9};
    let mut i64_1: i64 = -4917i64;
    let mut str_1: &str = "y";
    let mut string_1: std::string::String = std::string::String::from(str_1);
    let mut u64_12: u64 = 1873u64;
    let mut expense_1: crate::expense::Expense = crate::expense::Expense::new(u64_12, string_1, i64_1, simpledate_2, option_5, option_4, vec_1);
    let mut expense_1_ref_0: &crate::expense::Expense = &mut expense_1;
    let mut ordering_0: std::cmp::Ordering = crate::expense::Expense::compare_dates(expense_1_ref_0, expense_0_ref_0);
    let mut option_6: std::option::Option<crate::date::SimpleDate> = crate::expense::Expense::end_date(simpledate_0_ref_0, option_1_ref_0, option_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3947() {
    rusty_monitor::set_test_id(3947);
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_0: u64 = 5357u64;
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_1: u64 = 2953u64;
    let mut u64_2: u64 = 8023u64;
    let mut u64_3: u64 = 7028u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_3, u64_2, u64_1);
    let mut i64_0: i64 = 482i64;
    let mut u64_4: u64 = 8575u64;
    let mut u64_5: u64 = 7492u64;
    let mut u64_6: u64 = 9444u64;
    let mut u64_7: u64 = 9789u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_7, month: u64_6, day: u64_5};
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut u64_8: u64 = 7018u64;
    let mut u64_9: u64 = 6820u64;
    let mut u64_10: u64 = 4285u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_10, month: u64_9, day: u64_8};
    let mut simpledate_2_ref_0: &crate::date::SimpleDate = &mut simpledate_2;
    let mut u64_11: u64 = 1629u64;
    let mut u64_12: u64 = 6184u64;
    let mut u64_13: u64 = 7610u64;
    let mut simpledate_3: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_13, month: u64_12, day: u64_11};
    let mut simpledate_3_ref_0: &crate::date::SimpleDate = &mut simpledate_3;
    let mut u64_14: u64 = 7114u64;
    let mut u64_15: u64 = 4295u64;
    let mut u64_16: u64 = 9478u64;
    let mut simpledate_4: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_16, month: u64_15, day: u64_14};
    let mut simpledate_4_ref_0: &crate::date::SimpleDate = &mut simpledate_4;
    let mut u64_17: u64 = crate::expense::count_overlap_days(simpledate_4_ref_0, simpledate_3_ref_0, simpledate_2_ref_0, simpledate_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1807() {
    rusty_monitor::set_test_id(1807);
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_0: u64 = 1051u64;
    let mut u64_1: u64 = 2713u64;
    let mut u64_2: u64 = 4132u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_2, month: u64_1, day: u64_0};
    let mut i64_0: i64 = -11348i64;
    let mut u64_3: u64 = 3996u64;
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_4: u64 = 1376u64;
    let mut duration_0: date::Duration = crate::date::Duration::Year(u64_4);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut option_4: std::option::Option<date::Duration> = std::option::Option::None;
    let mut option_4_ref_0: &std::option::Option<date::Duration> = &mut option_4;
    let mut u64_5: u64 = 1040u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_5);
    let mut u64_6: u64 = 6530u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_6};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_5: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_5_ref_0: &std::option::Option<crate::date::Repetition> = &mut option_5;
    let mut u64_7: u64 = 1195u64;
    let mut u64_8: u64 = 2224u64;
    let mut u64_9: u64 = 6093u64;
    let mut u64_10: u64 = 3042u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_10, month: u64_9, day: u64_8};
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut option_6: std::option::Option<crate::date::SimpleDate> = crate::expense::Expense::end_date(simpledate_1_ref_0, option_5_ref_0, option_4_ref_0);
    let mut u64_11: u64 = 9684u64;
    let mut u64_12: u64 = 3160u64;
    let mut u64_13: u64 = 5098u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_13, u64_12, u64_11);
    let mut i64_1: i64 = 1780i64;
    let mut u64_14: u64 = 6119u64;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_591() {
    rusty_monitor::set_test_id(591);
    let mut u64_0: u64 = 348u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_0};
    let mut weekday_0: date::Weekday = crate::date::Weekday::Friday;
    let mut u64_1: u64 = 2447u64;
    let mut u64_2: u64 = 3878u64;
    let mut u64_3: u64 = 405u64;
    let mut u64_4: u64 = 1916u64;
    let mut u64_5: u64 = 6162u64;
    let mut u64_6: u64 = 9849u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_6, month: u64_5, day: u64_4};
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut u64_7: u64 = 3598u64;
    let mut u64_8: u64 = 803u64;
    let mut u64_9: u64 = 8549u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_9, u64_8, u64_7);
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut u64_10: u64 = 944u64;
    let mut u64_11: u64 = 7772u64;
    let mut u64_12: u64 = 1530u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_12, u64_11, u64_10);
    let mut simpledate_2_ref_0: &crate::date::SimpleDate = &mut simpledate_2;
    let mut u64_13: u64 = 7686u64;
    let mut u64_14: u64 = 4889u64;
    let mut u64_15: u64 = 3563u64;
    let mut simpledate_3: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_15, month: u64_14, day: u64_13};
    let mut simpledate_3_ref_0: &crate::date::SimpleDate = &mut simpledate_3;
    let mut u64_16: u64 = crate::expense::count_overlap_days(simpledate_3_ref_0, simpledate_2_ref_0, simpledate_1_ref_0, simpledate_0_ref_0);
    let mut weekday_1: date::Weekday = crate::date::Weekday::Sunday;
    let mut weekday_2: date::Weekday = crate::date::Weekday::Tuesday;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_3};
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_2, weekid: u64_1, day: weekday_0};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2397() {
    rusty_monitor::set_test_id(2397);
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_0: u64 = 852u64;
    let mut duration_0: date::Duration = crate::date::Duration::Week(u64_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_1: u64 = 1749u64;
    let mut duration_1: date::Duration = crate::date::Duration::Day(u64_1);
    let mut option_2: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
    let mut option_2_ref_0: &std::option::Option<date::Duration> = &mut option_2;
    let mut option_3: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_3_ref_0: &std::option::Option<crate::date::Repetition> = &mut option_3;
    let mut u64_2: u64 = 950u64;
    let mut u64_3: u64 = 6181u64;
    let mut u64_4: u64 = 5778u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_4, month: u64_3, day: u64_2};
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut option_4: std::option::Option<crate::date::SimpleDate> = crate::expense::Expense::end_date(simpledate_0_ref_0, option_3_ref_0, option_2_ref_0);
    let mut u64_5: u64 = 7680u64;
    let mut u64_6: u64 = 7510u64;
    let mut u64_7: u64 = 2488u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_7, month: u64_6, day: u64_5};
    let mut i64_0: i64 = -10601i64;
    let mut str_0: &str = "ue67nQQqyU";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_8: u64 = 3891u64;
    let mut u64_9: u64 = 5428u64;
    let mut bool_0: bool = false;
    let mut u64_10: u64 = 49u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_9);
    let mut expense_0: crate::expense::Expense = crate::expense::Expense {id: u64_8, description: string_0, amount: i64_0, start: simpledate_1, end: option_4, spread: option_1, repetition: option_0, tags: vec_0};
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut i64_1: i64 = crate::expense::Expense::amount(expense_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4502() {
    rusty_monitor::set_test_id(4502);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Sunday;
    let mut u64_0: u64 = 5462u64;
    let mut u64_1: u64 = 4862u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_1, weekid: u64_0, day: weekday_0};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_2: u64 = 5160u64;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_2);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_3: u64 = 2105u64;
    let mut duration_1: date::Duration = crate::date::Duration::Month(u64_3);
    let mut option_2: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
    let mut option_2_ref_0: &std::option::Option<date::Duration> = &mut option_2;
    let mut option_3: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_3_ref_0: &std::option::Option<crate::date::Repetition> = &mut option_3;
    let mut u64_4: u64 = 5155u64;
    let mut u64_5: u64 = 2784u64;
    let mut u64_6: u64 = 5541u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_6, month: u64_5, day: u64_4};
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut option_4: std::option::Option<crate::date::SimpleDate> = crate::expense::Expense::end_date(simpledate_0_ref_0, option_3_ref_0, option_2_ref_0);
    let mut u64_7: u64 = 3341u64;
    let mut u64_8: u64 = 5720u64;
    let mut u64_9: u64 = 6777u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_9, u64_8, u64_7);
    let mut i64_0: i64 = 5878i64;
    let mut str_0: &str = "yRJrdufEVpB7E";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_10: u64 = 577u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense {id: u64_10, description: string_0, amount: i64_0, start: simpledate_1, end: option_4, spread: option_1, repetition: option_0, tags: vec_0};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4027() {
    rusty_monitor::set_test_id(4027);
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_0: u64 = 4879u64;
    let mut duration_0: date::Duration = crate::date::Duration::Week(u64_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_1: u64 = 7816u64;
    let mut u64_2: u64 = 8805u64;
    let mut u64_3: u64 = 7127u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_3, month: u64_2, day: u64_1};
    let mut i64_0: i64 = -18276i64;
    let mut str_0: &str = "wmh3oSnejhotfBP";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_4: u64 = 5208u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_4, string_0, i64_0, simpledate_0, option_1, option_0, vec_0);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut u64_5: u64 = 3958u64;
    let mut vec_1: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_6: u64 = 1714u64;
    let mut duration_1: date::Duration = crate::date::Duration::Month(u64_6);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
    let mut u64_7: u64 = 4586u64;
    let mut u64_8: u64 = 5039u64;
    let mut u64_9: u64 = 9747u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_9, u64_8, u64_7);
    let mut i64_1: i64 = -1244i64;
    let mut str_1: &str = "9CzS47dPKcjHG8oFtw";
    let mut string_1: std::string::String = std::string::String::from(str_1);
    let mut u64_10: u64 = 890u64;
    let mut expense_1: crate::expense::Expense = crate::expense::Expense::new(u64_10, string_1, i64_1, simpledate_1, option_3, option_2, vec_1);
    let mut expense_1_ref_0: &crate::expense::Expense = &mut expense_1;
    let mut u64_11: u64 = 6769u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_11};
    let mut bool_0: bool = crate::expense::Expense::compare_id(expense_1_ref_0, u64_5);
    let mut simpledate_2: &crate::date::SimpleDate = crate::expense::Expense::get_start_date(expense_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_450() {
    rusty_monitor::set_test_id(450);
    let mut u64_0: u64 = 2814u64;
    let mut u64_1: u64 = 8575u64;
    let mut u64_2: u64 = 5610u64;
    let mut u64_3: u64 = 160u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_3, u64_2, u64_1);
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut u64_4: u64 = 1277u64;
    let mut u64_5: u64 = 1453u64;
    let mut u64_6: u64 = 8995u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_6, u64_5, u64_4);
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut u64_7: u64 = 4630u64;
    let mut u64_8: u64 = 1072u64;
    let mut u64_9: u64 = 8468u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_9, month: u64_8, day: u64_7};
    let mut simpledate_2_ref_0: &crate::date::SimpleDate = &mut simpledate_2;
    let mut u64_10: u64 = 9196u64;
    let mut u64_11: u64 = 4993u64;
    let mut u64_12: u64 = 2294u64;
    let mut simpledate_3: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_12, month: u64_11, day: u64_10};
    let mut simpledate_3_ref_0: &crate::date::SimpleDate = &mut simpledate_3;
    let mut u64_13: u64 = 3568u64;
    let mut vec_0: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_14: u64 = 2206u64;
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_14, days: vec_0};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnDate(monthdeltadate_0);
    let mut vec_1: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_15: u64 = 8351u64;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Monday;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_15, on: vec_1};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut weekday_1: date::Weekday = crate::date::Weekday::Friday;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_13);
    let mut u64_16: u64 = crate::expense::count_overlap_days(simpledate_3_ref_0, simpledate_2_ref_0, simpledate_1_ref_0, simpledate_0_ref_0);
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Count(u64_0);
    panic!("From RustyUnit with love");
}
}