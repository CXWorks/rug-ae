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
fn rusty_test_792() {
    rusty_monitor::set_test_id(792);
    let mut u64_0: u64 = 6142u64;
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut option_2: std::option::Option<crate::date::SimpleDate> = std::option::Option::None;
    let mut u64_1: u64 = 6025u64;
    let mut u64_2: u64 = 4744u64;
    let mut u64_3: u64 = 7550u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_3, u64_2, u64_1);
    let mut option_3: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_4: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_4: u64 = 5970u64;
    let mut u64_5: u64 = 8310u64;
    let mut u64_6: u64 = 7254u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_6, month: u64_5, day: u64_4};
    let mut u64_7: u64 = 7269u64;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_8: u64 = 2559u64;
    let mut u64_9: u64 = 7580u64;
    let mut u64_10: u64 = 3792u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_10, u64_9, u64_8);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_2);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Friday;
    let mut u64_11: u64 = 6134u64;
    let mut u64_12: u64 = 267u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_12, weekid: u64_11, day: weekday_0};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_5: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_13: u64 = 5119u64;
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_13);
    let mut option_6: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_14: u64 = 4985u64;
    let mut duration_1: date::Duration = crate::date::Duration::Year(u64_14);
    let mut option_7: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
    let mut option_7_ref_0: &std::option::Option<date::Duration> = &mut option_7;
    let mut u64_15: u64 = 2702u64;
    let mut u64_16: u64 = 6691u64;
    let mut u64_17: u64 = 7369u64;
    let mut simpledate_3: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_17, u64_16, u64_15);
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Date(simpledate_3);
    let mut u64_18: u64 = 7897u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_18};
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut repetition_1: crate::date::Repetition = crate::date::Repetition {delta: repdelta_1, end: repend_1};
    let mut option_8: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_1);
    let mut option_8_ref_0: &std::option::Option<crate::date::Repetition> = &mut option_8;
    let mut u64_19: u64 = 4060u64;
    let mut u64_20: u64 = 1386u64;
    let mut simpledate_4: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_7, u64_19, u64_20);
    let mut simpledate_4_ref_0: &crate::date::SimpleDate = &mut simpledate_4;
    let mut option_9: std::option::Option<crate::date::SimpleDate> = crate::expense::Expense::end_date(simpledate_4_ref_0, option_8_ref_0, option_7_ref_0);
    let mut u64_21: u64 = 8968u64;
    let mut u64_22: u64 = 1193u64;
    let mut u64_23: u64 = 2682u64;
    let mut simpledate_5: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_23, u64_22, u64_21);
    let mut i64_0: i64 = -2250i64;
    let mut str_0: &str = "AmudUjf7rK";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_24: u64 = 519u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense {id: u64_24, description: string_0, amount: i64_0, start: simpledate_5, end: option_9, spread: option_6, repetition: option_5, tags: vec_0};
    let mut duration_2: date::Duration = crate::date::Duration::Year(u64_0);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut i64_1: i64 = crate::expense::Expense::amount(expense_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_125() {
    rusty_monitor::set_test_id(125);
    let mut u64_0: u64 = 1915u64;
    let mut u64_1: u64 = 4295u64;
    let mut u64_2: u64 = 4725u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_2, u64_1, u64_0);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut u64_3: u64 = 1210u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_3};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_4: u64 = 6096u64;
    let mut u64_5: u64 = 1788u64;
    let mut u64_6: u64 = 8570u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_6, month: u64_5, day: u64_4};
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_7: u64 = 7743u64;
    let mut duration_0: date::Duration = crate::date::Duration::Week(u64_7);
    let mut option_4: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut option_4_ref_0: &std::option::Option<date::Duration> = &mut option_4;
    let mut option_5: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_5_ref_0: &std::option::Option<crate::date::Repetition> = &mut option_5;
    let mut u64_8: u64 = 3623u64;
    let mut u64_9: u64 = 4407u64;
    let mut u64_10: u64 = 8164u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_10, u64_9, u64_8);
    let mut simpledate_2_ref_0: &crate::date::SimpleDate = &mut simpledate_2;
    let mut option_6: std::option::Option<crate::date::SimpleDate> = crate::expense::Expense::end_date(simpledate_2_ref_0, option_5_ref_0, option_4_ref_0);
    let mut u64_11: u64 = 9771u64;
    let mut u64_12: u64 = 3989u64;
    let mut u64_13: u64 = 1999u64;
    let mut simpledate_3: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_13, u64_12, u64_11);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Sunday;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1072() {
    rusty_monitor::set_test_id(1072);
    let mut str_0: &str = "J7bNgO";
    let mut u64_0: u64 = 5211u64;
    let mut u64_1: u64 = 7408u64;
    let mut u64_2: u64 = 6384u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_2, u64_1, u64_0);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut vec_0: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_3: u64 = 9178u64;
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_3, days: vec_0};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnDate(monthdeltadate_0);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut u64_4: u64 = 4152u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_4};
    let mut u64_5: u64 = 7336u64;
    let mut u64_6: u64 = 504u64;
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Count(u64_6);
    let mut u64_7: u64 = 6934u64;
    let mut yeardelta_1: crate::date::YearDelta = crate::date::YearDelta {nth: u64_7};
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Year(yeardelta_1);
    let mut u64_8: u64 = 5190u64;
    let mut u64_9: u64 = 987u64;
    let mut u64_10: u64 = 2002u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_10, month: u64_9, day: u64_8};
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut str_1: &str = "QsSKwO";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u64_11: u64 = 4476u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_11};
    let mut weekday_0: date::Weekday = crate::date::Weekday::Wednesday;
    let mut u64_12: u64 = 6150u64;
    let mut daydelta_1: crate::date::DayDelta = crate::date::DayDelta {nth: u64_12};
    let mut str_2: &str = "";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut u64_13: u64 = 9062u64;
    let mut repend_2: date::RepEnd = crate::date::RepEnd::Count(u64_13);
    let mut u64_14: u64 = 2766u64;
    let mut daydelta_2: crate::date::DayDelta = crate::date::DayDelta {nth: u64_14};
    let mut repdelta_2: date::RepDelta = crate::date::RepDelta::Day(daydelta_2);
    let mut yeardelta_2: crate::date::YearDelta = crate::date::YearDelta {nth: u64_5};
    let mut repdelta_3: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut repetition_1: crate::date::Repetition = crate::date::Repetition {delta: repdelta_1, end: repend_1};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut option_2: std::option::Option<crate::date::SimpleDate> = std::option::Option::None;
    let mut u64_15: u64 = 3458u64;
    let mut u64_16: u64 = 6477u64;
    let mut u64_17: u64 = 1405u64;
    let mut u64_18: u64 = 9840u64;
    let mut u64_19: u64 = 2075u64;
    let mut repend_3: date::RepEnd = crate::date::RepEnd::Count(u64_18);
    let mut u64_20: u64 = 7555u64;
    let mut daydelta_3: crate::date::DayDelta = crate::date::DayDelta {nth: u64_17};
    let mut repdelta_4: date::RepDelta = crate::date::RepDelta::Day(daydelta_1);
    let mut repetition_2: crate::date::Repetition = crate::date::Repetition {delta: repdelta_3, end: repend_3};
    let mut option_3: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_1);
    let mut option_4: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_21: u64 = 885u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_21, u64_15, u64_16);
    let mut u64_22: u64 = 1786u64;
    let mut u64_23: u64 = 9035u64;
    let mut u64_24: u64 = 46u64;
    let mut u64_25: u64 = 2153u64;
    let mut u64_26: u64 = 1999u64;
    let mut simpledate_3: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_26, month: u64_23, day: u64_20};
    let mut simpledate_2_ref_0: &crate::date::SimpleDate = &mut simpledate_2;
    let mut u64_27: u64 = 2960u64;
    let mut u64_28: u64 = 9002u64;
    let mut simpledate_4: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_28, u64_19, u64_27);
    let mut simpledate_3_ref_0: &crate::date::SimpleDate = &mut simpledate_3;
    let mut u64_29: u64 = 5154u64;
    let mut u64_30: u64 = 9669u64;
    let mut simpledate_5: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_24, u64_30, u64_22);
    let mut simpledate_4_ref_0: &crate::date::SimpleDate = &mut simpledate_4;
    let mut u64_31: u64 = 7384u64;
    let mut simpledate_6: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_29, month: u64_31, day: u64_25};
    let mut simpledate_5_ref_0: &crate::date::SimpleDate = &mut simpledate_5;
    let mut u64_32: u64 = crate::expense::count_overlap_days(simpledate_4_ref_0, simpledate_3_ref_0, simpledate_1_ref_0, simpledate_2_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_272() {
    rusty_monitor::set_test_id(272);
    let mut u64_0: u64 = 2603u64;
    let mut u64_1: u64 = 3337u64;
    let mut u64_2: u64 = 9468u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_2, u64_1, u64_0);
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut u64_3: u64 = 2452u64;
    let mut u64_4: u64 = 7706u64;
    let mut u64_5: u64 = 243u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_5, month: u64_4, day: u64_3};
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut u64_6: u64 = 5548u64;
    let mut u64_7: u64 = 232u64;
    let mut u64_8: u64 = 2755u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_8, month: u64_7, day: u64_6};
    let mut simpledate_2_ref_0: &crate::date::SimpleDate = &mut simpledate_2;
    let mut u64_9: u64 = 9578u64;
    let mut u64_10: u64 = 72u64;
    let mut u64_11: u64 = 2275u64;
    let mut simpledate_3: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_11, month: u64_10, day: u64_9};
    let mut simpledate_3_ref_0: &crate::date::SimpleDate = &mut simpledate_3;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_12: u64 = 687u64;
    let mut u64_13: u64 = 91u64;
    let mut u64_14: u64 = 938u64;
    let mut simpledate_4: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_14, month: u64_13, day: u64_12};
    let mut i64_0: i64 = -6127i64;
    let mut str_0: &str = "H0KXSe723f";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_15: u64 = 6339u64;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Saturday;
    let mut u64_16: u64 = 4320u64;
    let mut u64_17: u64 = 4632u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_17, weekid: u64_16, day: weekday_0};
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_15, string_0, i64_0, simpledate_4, option_1, option_0, vec_0);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut str_1: &str = crate::expense::Expense::description(expense_0_ref_0);
    let mut u64_18: u64 = crate::expense::count_overlap_days(simpledate_3_ref_0, simpledate_2_ref_0, simpledate_1_ref_0, simpledate_0_ref_0);
    let mut monthdeltaweek_0_ref_0: &crate::date::MonthDeltaWeek = &mut monthdeltaweek_0;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1584() {
    rusty_monitor::set_test_id(1584);
    let mut u64_0: u64 = 2878u64;
    let mut duration_0: date::Duration = crate::date::Duration::Year(u64_0);
    let mut u64_1: u64 = 8978u64;
    let mut u64_2: u64 = 6149u64;
    let mut u64_3: u64 = 7654u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_3, u64_2, u64_1);
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_4: u64 = 4155u64;
    let mut duration_1: date::Duration = crate::date::Duration::Month(u64_4);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
    let mut u64_5: u64 = 2070u64;
    let mut duration_2: date::Duration = crate::date::Duration::Year(u64_5);
    let mut option_2: std::option::Option<date::Duration> = std::option::Option::Some(duration_2);
    let mut option_2_ref_0: &std::option::Option<date::Duration> = &mut option_2;
    let mut u64_6: u64 = 100u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_6);
    let mut u64_7: u64 = 6366u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_7};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_3: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_3_ref_0: &std::option::Option<crate::date::Repetition> = &mut option_3;
    let mut u64_8: u64 = 2331u64;
    let mut u64_9: u64 = 2545u64;
    let mut u64_10: u64 = 7394u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_10, u64_9, u64_8);
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut option_4: std::option::Option<crate::date::SimpleDate> = crate::expense::Expense::end_date(simpledate_1_ref_0, option_3_ref_0, option_2_ref_0);
    let mut duration_3: date::Duration = std::option::Option::unwrap(option_1);
    let mut repetition_1: crate::date::Repetition = std::option::Option::unwrap(option_0);
    let mut simpledate_2: crate::date::SimpleDate = std::option::Option::unwrap(option_4);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8156() {
    rusty_monitor::set_test_id(8156);
    let mut u64_0: u64 = 1833u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_0);
    let mut u64_1: u64 = 6142u64;
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut option_2: std::option::Option<crate::date::SimpleDate> = std::option::Option::None;
    let mut u64_2: u64 = 6025u64;
    let mut u64_3: u64 = 4744u64;
    let mut u64_4: u64 = 7550u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_4, u64_3, u64_2);
    let mut option_3: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_4: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_5: u64 = 5970u64;
    let mut u64_6: u64 = 8310u64;
    let mut u64_7: u64 = 7254u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_7, month: u64_6, day: u64_5};
    let mut u64_8: u64 = 7269u64;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut weekday_0: date::Weekday = crate::date::Weekday::Friday;
    let mut u64_9: u64 = 6134u64;
    let mut u64_10: u64 = 267u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_10, weekid: u64_9, day: weekday_0};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut u64_11: u64 = 5119u64;
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_11);
    let mut option_5: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_12: u64 = 4985u64;
    let mut duration_1: date::Duration = crate::date::Duration::Year(u64_12);
    let mut option_6: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
    let mut option_6_ref_0: &std::option::Option<date::Duration> = &mut option_6;
    let mut u64_13: u64 = 2702u64;
    let mut u64_14: u64 = 6691u64;
    let mut u64_15: u64 = 7369u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_15, u64_14, u64_13);
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Date(simpledate_2);
    let mut u64_16: u64 = 7897u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_16};
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_7: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_7_ref_0: &std::option::Option<crate::date::Repetition> = &mut option_7;
    let mut u64_17: u64 = 4060u64;
    let mut u64_18: u64 = 550u64;
    let mut simpledate_3: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_8, u64_17, u64_18);
    let mut simpledate_3_ref_0: &crate::date::SimpleDate = &mut simpledate_3;
    let mut option_8: std::option::Option<crate::date::SimpleDate> = crate::expense::Expense::end_date(simpledate_3_ref_0, option_7_ref_0, option_6_ref_0);
    let mut u64_19: u64 = 8968u64;
    let mut u64_20: u64 = 2752u64;
    let mut u64_21: u64 = 2682u64;
    let mut simpledate_4: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_21, u64_20, u64_19);
    let mut str_0: &str = "AmudUjf7rK";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut duration_2: date::Duration = crate::date::Duration::Year(u64_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_42() {
    rusty_monitor::set_test_id(42);
    let mut u64_0: u64 = 2603u64;
    let mut u64_1: u64 = 3337u64;
    let mut u64_2: u64 = 9468u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_2, u64_1, u64_0);
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut u64_3: u64 = 2452u64;
    let mut u64_4: u64 = 7706u64;
    let mut u64_5: u64 = 3405u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_5, month: u64_4, day: u64_3};
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut u64_6: u64 = 5548u64;
    let mut u64_7: u64 = 1015u64;
    let mut u64_8: u64 = 2755u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_8, month: u64_7, day: u64_6};
    let mut simpledate_2_ref_0: &crate::date::SimpleDate = &mut simpledate_2;
    let mut u64_9: u64 = 9578u64;
    let mut u64_10: u64 = 7861u64;
    let mut u64_11: u64 = 2275u64;
    let mut simpledate_3: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_11, month: u64_10, day: u64_9};
    let mut simpledate_3_ref_0: &crate::date::SimpleDate = &mut simpledate_3;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_12: u64 = 5227u64;
    let mut u64_13: u64 = 4007u64;
    let mut u64_14: u64 = 938u64;
    let mut simpledate_4: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_14, month: u64_13, day: u64_12};
    let mut i64_0: i64 = -6127i64;
    let mut str_0: &str = "H0KXSe723f";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_15: u64 = 6339u64;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Saturday;
    let mut u64_16: u64 = 4320u64;
    let mut u64_17: u64 = 4632u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_17, weekid: u64_16, day: weekday_0};
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_15, string_0, i64_0, simpledate_4, option_1, option_0, vec_0);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut str_1: &str = crate::expense::Expense::description(expense_0_ref_0);
    let mut u64_18: u64 = crate::expense::count_overlap_days(simpledate_3_ref_0, simpledate_2_ref_0, simpledate_1_ref_0, simpledate_0_ref_0);
    panic!("From RustyUnit with love");
}
}