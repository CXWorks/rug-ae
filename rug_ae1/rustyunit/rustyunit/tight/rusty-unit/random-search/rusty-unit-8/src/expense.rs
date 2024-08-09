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
fn rusty_test_613() {
    rusty_monitor::set_test_id(613);
    let mut u64_0: u64 = 524u64;
    let mut u64_1: u64 = 3764u64;
    let mut u64_2: u64 = 2927u64;
    let mut u64_3: u64 = 5705u64;
    let mut u64_4: u64 = 2996u64;
    let mut u64_5: u64 = 2724u64;
    let mut u64_6: u64 = 9574u64;
    let mut i64_0: i64 = -10915i64;
    let mut u64_7: u64 = 635u64;
    let mut u64_8: u64 = 7904u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_8);
    let mut u64_9: u64 = 8989u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_9};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_10: u64 = 6729u64;
    let mut u64_11: u64 = 9544u64;
    let mut u64_12: u64 = 6071u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_12, u64_11, u64_10);
    let mut option_1: std::option::Option<crate::date::SimpleDate> = std::option::Option::Some(simpledate_0);
    let mut u64_13: u64 = 8596u64;
    let mut u64_14: u64 = 5723u64;
    let mut u64_15: u64 = 4638u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_15, u64_14, u64_13);
    let mut i64_1: i64 = 9058i64;
    let mut u64_16: u64 = 2817u64;
    let mut u64_17: u64 = 8885u64;
    let mut u64_18: u64 = 6311u64;
    let mut u64_19: u64 = 3160u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_19, month: u64_18, day: u64_17};
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Date(simpledate_2);
    let mut u64_20: u64 = 6169u64;
    let mut daydelta_1: crate::date::DayDelta = crate::date::DayDelta {nth: u64_20};
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Day(daydelta_1);
    let mut repetition_1: crate::date::Repetition = crate::date::Repetition {delta: repdelta_1, end: repend_1};
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_1);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_21: u64 = 3740u64;
    let mut u64_22: u64 = 3071u64;
    let mut u64_23: u64 = 843u64;
    let mut simpledate_3: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_23, u64_22, u64_21);
    let mut i64_2: i64 = 7755i64;
    let mut u64_24: u64 = 9439u64;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_25: u64 = 8181u64;
    let mut u64_26: u64 = 4228u64;
    let mut u64_27: u64 = 660u64;
    let mut simpledate_4: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_27, month: u64_26, day: u64_25};
    let mut repend_2: date::RepEnd = crate::date::RepEnd::Date(simpledate_4);
    let mut u64_28: u64 = 8289u64;
    let mut daydelta_2: crate::date::DayDelta = crate::date::DayDelta {nth: u64_28};
    let mut repdelta_2: date::RepDelta = crate::date::RepDelta::Day(daydelta_2);
    let mut repetition_2: crate::date::Repetition = crate::date::Repetition {delta: repdelta_2, end: repend_2};
    let mut option_4: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_2);
    let mut u64_29: u64 = 8290u64;
    let mut duration_0: date::Duration = crate::date::Duration::Year(u64_29);
    let mut option_5: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_30: u64 = 2452u64;
    let mut duration_1: date::Duration = crate::date::Duration::Month(u64_30);
    let mut option_6: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
    let mut option_6_ref_0: &std::option::Option<date::Duration> = &mut option_6;
    let mut u64_31: u64 = 5619u64;
    let mut repend_3: date::RepEnd = crate::date::RepEnd::Count(u64_31);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Sunday;
    let mut u64_32: u64 = 8238u64;
    let mut u64_33: u64 = 3424u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_33, weekid: u64_32, day: weekday_0};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    let mut repdelta_3: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut repetition_3: crate::date::Repetition = crate::date::Repetition {delta: repdelta_3, end: repend_3};
    let mut option_7: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_3);
    let mut option_7_ref_0: &std::option::Option<crate::date::Repetition> = &mut option_7;
    let mut u64_34: u64 = 4841u64;
    let mut u64_35: u64 = 1650u64;
    let mut u64_36: u64 = 1u64;
    let mut u64_37: u64 = 7319u64;
    let mut simpledate_5: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_37, month: u64_36, day: u64_35};
    let mut simpledate_5_ref_0: &crate::date::SimpleDate = &mut simpledate_5;
    let mut option_8: std::option::Option<crate::date::SimpleDate> = crate::expense::Expense::end_date(simpledate_5_ref_0, option_7_ref_0, option_6_ref_0);
    let mut u64_38: u64 = 543u64;
    let mut u64_39: u64 = 344u64;
    let mut u64_40: u64 = 2360u64;
    let mut simpledate_6: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_40, month: u64_39, day: u64_38};
    let mut i64_3: i64 = -6290i64;
    let mut str_0: &str = "kYorHCeSnry5";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_41: u64 = 4408u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense {id: u64_41, description: string_0, amount: i64_3, start: simpledate_6, end: option_8, spread: option_5, repetition: option_4, tags: vec_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4363() {
    rusty_monitor::set_test_id(4363);
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_0: u64 = 4205u64;
    let mut u64_1: u64 = 6418u64;
    let mut u64_2: u64 = 4078u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_2, month: u64_1, day: u64_0};
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut vec_1: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_3: u64 = 6802u64;
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_3, days: vec_1};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnDate(monthdeltadate_0);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_4: u64 = 4632u64;
    let mut u64_5: u64 = 9066u64;
    let mut u64_6: u64 = 3741u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_6, u64_5, u64_4);
    let mut i64_0: i64 = -10994i64;
    let mut str_0: &str = "7sFe4cbEEHXuSOOCtJ";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_7: u64 = 2500u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_7, string_0, i64_0, simpledate_1, option_1, option_0, vec_0);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut vec_2: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_8: u64 = 1639u64;
    let mut vec_3: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_9: u64 = 3393u64;
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_9);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_10: u64 = 4895u64;
    let mut u64_11: u64 = 1111u64;
    let mut u64_12: u64 = 1468u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_12, u64_11, u64_10);
    let mut i64_1: i64 = -13903i64;
    let mut str_1: &str = "goztLZd5Z1jkPIEBp";
    let mut string_1: std::string::String = std::string::String::from(str_1);
    let mut u64_13: u64 = 3254u64;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Wednesday;
    let mut expense_1: crate::expense::Expense = crate::expense::Expense::new(u64_13, string_1, i64_1, simpledate_2, option_3, option_2, vec_3);
    let mut monthdeltadate_1: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_8, days: vec_2};
    let mut vec_4: &std::vec::Vec<std::string::String> = crate::expense::Expense::tags(expense_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_283() {
    rusty_monitor::set_test_id(283);
    let mut u64_0: u64 = 8917u64;
    let mut duration_0: date::Duration = crate::date::Duration::Year(u64_0);
    let mut duration_0_ref_0: &date::Duration = &mut duration_0;
    let mut u64_1: u64 = 8278u64;
    let mut u64_2: u64 = 450u64;
    let mut u64_3: u64 = 2041u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_3, month: u64_2, day: u64_1};
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut u64_4: u64 = 2339u64;
    let mut u64_5: u64 = 8649u64;
    let mut u64_6: u64 = 1671u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_6, u64_5, u64_4);
    let mut str_0: &str = "zcVlYA";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_7: u64 = 6995u64;
    let mut duration_1: date::Duration = crate::date::Duration::Week(u64_7);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
    let mut u64_8: u64 = 7318u64;
    let mut u64_9: u64 = 3889u64;
    let mut u64_10: u64 = 230u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_10, u64_9, u64_8);
    let mut i64_0: i64 = -978i64;
    let mut str_1: &str = "7wWJWMvCHF8F2Dx";
    let mut string_0: std::string::String = std::string::String::from(str_1);
    let mut u64_11: u64 = 5140u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_11, string_0, i64_0, simpledate_2, option_1, option_0, vec_0);
    let mut expense_0_ref_0: &mut crate::expense::Expense = &mut expense_0;
    let mut u64_12: u64 = 9287u64;
    let mut u64_13: u64 = 3936u64;
    let mut duration_2: date::Duration = crate::date::Duration::Week(u64_13);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_12);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Friday;
    crate::expense::Expense::remove_tags(expense_0_ref_0, str_0_ref_0);
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Date(simpledate_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3195() {
    rusty_monitor::set_test_id(3195);
    let mut u64_0: u64 = 7454u64;
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_1: u64 = 6609u64;
    let mut u64_2: u64 = 9402u64;
    let mut u64_3: u64 = 8003u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_3, u64_2, u64_1);
    let mut i64_0: i64 = -15780i64;
    let mut u64_4: u64 = 3688u64;
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_5: u64 = 5382u64;
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_5);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_6: u64 = 4110u64;
    let mut duration_1: date::Duration = crate::date::Duration::Day(u64_6);
    let mut option_4: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
    let mut option_4_ref_0: &std::option::Option<date::Duration> = &mut option_4;
    let mut option_5: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_5_ref_0: &std::option::Option<crate::date::Repetition> = &mut option_5;
    let mut u64_7: u64 = 229u64;
    let mut u64_8: u64 = 8553u64;
    let mut u64_9: u64 = 2178u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_9, u64_8, u64_7);
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut option_6: std::option::Option<crate::date::SimpleDate> = crate::expense::Expense::end_date(simpledate_1_ref_0, option_5_ref_0, option_4_ref_0);
    let mut u64_10: u64 = 5856u64;
    let mut u64_11: u64 = 9139u64;
    let mut u64_12: u64 = 5415u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_12, month: u64_11, day: u64_10};
    let mut i64_1: i64 = 24044i64;
    let mut u64_13: u64 = 8376u64;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1611() {
    rusty_monitor::set_test_id(1611);
    let mut bool_0: bool = false;
    let mut u64_0: u64 = 5124u64;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Monday;
    let mut u64_1: u64 = 4799u64;
    let mut u64_2: u64 = 8307u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_2, weekid: u64_1, day: weekday_0};
    let mut u64_3: u64 = 8554u64;
    let mut u64_4: u64 = 2906u64;
    let mut u64_5: u64 = 147u64;
    let mut u64_6: u64 = 1269u64;
    let mut u64_7: u64 = 6909u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_7, u64_6, u64_5);
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut u64_8: u64 = 1619u64;
    let mut u64_9: u64 = 7776u64;
    let mut u64_10: u64 = 9883u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_10, month: u64_9, day: u64_8};
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut u64_11: u64 = 6165u64;
    let mut u64_12: u64 = 4231u64;
    let mut u64_13: u64 = 7492u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_13, month: u64_12, day: u64_11};
    let mut simpledate_2_ref_0: &crate::date::SimpleDate = &mut simpledate_2;
    let mut u64_14: u64 = 1685u64;
    let mut u64_15: u64 = 5529u64;
    let mut u64_16: u64 = 5250u64;
    let mut simpledate_3: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_16, month: u64_15, day: u64_14};
    let mut simpledate_3_ref_0: &crate::date::SimpleDate = &mut simpledate_3;
    let mut u64_17: u64 = crate::expense::count_overlap_days(simpledate_3_ref_0, simpledate_2_ref_0, simpledate_1_ref_0, simpledate_0_ref_0);
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_4);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_3);
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4272() {
    rusty_monitor::set_test_id(4272);
    let mut u64_0: u64 = 2690u64;
    let mut u64_1: u64 = 4823u64;
    let mut u64_2: u64 = 5134u64;
    let mut u64_3: u64 = 7997u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_3, month: u64_2, day: u64_1};
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut u64_4: u64 = 8823u64;
    let mut option_0: std::option::Option<date::Duration> = std::option::Option::None;
    let mut option_0_ref_0: &std::option::Option<date::Duration> = &mut option_0;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut u64_5: u64 = 6215u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_5};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_1: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_1_ref_0: &std::option::Option<crate::date::Repetition> = &mut option_1;
    let mut u64_6: u64 = 3521u64;
    let mut u64_7: u64 = 9547u64;
    let mut u64_8: u64 = 7593u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_8, month: u64_7, day: u64_6};
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut vec_0: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_9: u64 = 6639u64;
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_9, days: vec_0};
    let mut u64_10: u64 = 7506u64;
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_10);
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnDate(monthdeltadate_0);
    let mut option_2: std::option::Option<crate::date::SimpleDate> = crate::expense::Expense::end_date(simpledate_1_ref_0, option_1_ref_0, option_0_ref_0);
    let mut duration_1: date::Duration = crate::date::Duration::Week(u64_4);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Saturday;
    let mut weekday_1: date::Weekday = crate::date::Weekday::Friday;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2079() {
    rusty_monitor::set_test_id(2079);
    let mut u64_0: u64 = 8641u64;
    let mut u64_1: u64 = 4416u64;
    let mut u64_2: u64 = 2784u64;
    let mut u64_3: u64 = 7642u64;
    let mut u64_4: u64 = 5408u64;
    let mut u64_5: u64 = 7475u64;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Wednesday;
    let mut u64_6: u64 = 6429u64;
    let mut u64_7: u64 = 4064u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_7, weekid: u64_6, day: weekday_0};
    let mut option_0: std::option::Option<date::Duration> = std::option::Option::None;
    let mut option_0_ref_0: &std::option::Option<date::Duration> = &mut option_0;
    let mut u64_8: u64 = 9793u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_8);
    let mut u64_9: u64 = 8739u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_9};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_1: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_1_ref_0: &std::option::Option<crate::date::Repetition> = &mut option_1;
    let mut u64_10: u64 = 6568u64;
    let mut u64_11: u64 = 2033u64;
    let mut u64_12: u64 = 6919u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_12, u64_11, u64_10);
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut u64_13: u64 = 9307u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_13};
    let mut option_2: std::option::Option<crate::date::SimpleDate> = crate::expense::Expense::end_date(simpledate_0_ref_0, option_1_ref_0, option_0_ref_0);
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_5, u64_4, u64_3);
    let mut simpledate_2: crate::date::SimpleDate = std::option::Option::unwrap(option_2);
    let mut simpledate_3: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_2, month: u64_1, day: u64_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4234() {
    rusty_monitor::set_test_id(4234);
    let mut u64_0: u64 = 6360u64;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_0);
    let mut duration_0_ref_0: &date::Duration = &mut duration_0;
    let mut u64_1: u64 = 7264u64;
    let mut u64_2: u64 = 3786u64;
    let mut u64_3: u64 = 866u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_3, month: u64_2, day: u64_1};
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut u64_4: u64 = 8919u64;
    let mut u64_5: u64 = 1619u64;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_6: u64 = 674u64;
    let mut duration_1: date::Duration = crate::date::Duration::Day(u64_6);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
    let mut u64_7: u64 = 3138u64;
    let mut u64_8: u64 = 265u64;
    let mut u64_9: u64 = 6536u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_9, u64_8, u64_7);
    let mut i64_0: i64 = 5406i64;
    let mut str_0: &str = "e8hOa";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_10: u64 = 8807u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_10, string_0, i64_0, simpledate_1, option_1, option_0, vec_0);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut vec_1: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut u64_11: u64 = 6482u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_11};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_12: u64 = 2529u64;
    let mut u64_13: u64 = 1588u64;
    let mut u64_14: u64 = 6994u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_14, u64_13, u64_12);
    let mut i64_1: i64 = -11576i64;
    let mut str_1: &str = "9vGMRT3u5lV";
    let mut string_1: std::string::String = std::string::String::from(str_1);
    let mut u64_15: u64 = 7764u64;
    let mut expense_1: crate::expense::Expense = crate::expense::Expense::new(u64_15, string_1, i64_1, simpledate_2, option_3, option_2, vec_1);
    let mut expense_1_ref_0: &crate::expense::Expense = &mut expense_1;
    let mut ordering_0: std::cmp::Ordering = crate::expense::Expense::compare_dates(expense_1_ref_0, expense_0_ref_0);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Saturday;
    let mut weekday_1: date::Weekday = crate::date::Weekday::Wednesday;
    let mut duration_2: date::Duration = crate::date::Duration::Day(u64_5);
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Count(u64_4);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_602() {
    rusty_monitor::set_test_id(602);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Wednesday;
    let mut u64_0: u64 = 3806u64;
    let mut u64_1: u64 = 622u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_1, weekid: u64_0, day: weekday_0};
    let mut u64_2: u64 = 732u64;
    let mut u64_3: u64 = 7251u64;
    let mut u64_4: u64 = 4283u64;
    let mut u64_5: u64 = 5532u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_5, month: u64_4, day: u64_3};
    let mut u64_6: u64 = 7383u64;
    let mut u64_7: u64 = 3932u64;
    let mut u64_8: u64 = 9292u64;
    let mut u64_9: u64 = 7771u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_9, month: u64_8, day: u64_7};
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut u64_10: u64 = 4324u64;
    let mut u64_11: u64 = 2981u64;
    let mut u64_12: u64 = 8400u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_12, u64_11, u64_10);
    let mut simpledate_2_ref_0: &crate::date::SimpleDate = &mut simpledate_2;
    let mut u64_13: u64 = 1285u64;
    let mut u64_14: u64 = 8037u64;
    let mut u64_15: u64 = 1963u64;
    let mut simpledate_3: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_15, u64_14, u64_13);
    let mut simpledate_3_ref_0: &crate::date::SimpleDate = &mut simpledate_3;
    let mut u64_16: u64 = 1213u64;
    let mut u64_17: u64 = 7409u64;
    let mut u64_18: u64 = 2229u64;
    let mut simpledate_4: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_18, month: u64_17, day: u64_16};
    let mut simpledate_4_ref_0: &crate::date::SimpleDate = &mut simpledate_4;
    let mut u64_19: u64 = 2010u64;
    let mut u64_20: u64 = 1071u64;
    let mut u64_21: u64 = 9498u64;
    let mut simpledate_5: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_21, month: u64_20, day: u64_19};
    let mut simpledate_5_ref_0: &crate::date::SimpleDate = &mut simpledate_5;
    let mut u64_22: u64 = crate::expense::count_overlap_days(simpledate_5_ref_0, simpledate_4_ref_0, simpledate_3_ref_0, simpledate_2_ref_0);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3650() {
    rusty_monitor::set_test_id(3650);
    let mut vec_0: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_0: u64 = 2526u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_0, on: vec_0};
    let mut vec_1: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_1: u64 = 6526u64;
    let mut duration_0: date::Duration = crate::date::Duration::Year(u64_1);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_2: u64 = 5996u64;
    let mut u64_3: u64 = 9940u64;
    let mut u64_4: u64 = 9423u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_4, u64_3, u64_2);
    let mut i64_0: i64 = -15765i64;
    let mut str_0: &str = "81qMt37LmuVUxmYvF";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_5: u64 = 5491u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_5, string_0, i64_0, simpledate_0, option_1, option_0, vec_1);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut vec_2: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_6: u64 = 6852u64;
    let mut duration_1: date::Duration = crate::date::Duration::Month(u64_6);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
    let mut u64_7: u64 = 6755u64;
    let mut u64_8: u64 = 4448u64;
    let mut u64_9: u64 = 6110u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_9, month: u64_8, day: u64_7};
    let mut i64_1: i64 = 3956i64;
    let mut str_1: &str = "";
    let mut string_1: std::string::String = std::string::String::from(str_1);
    let mut u64_10: u64 = 9484u64;
    let mut expense_1: crate::expense::Expense = crate::expense::Expense::new(u64_10, string_1, i64_1, simpledate_1, option_3, option_2, vec_2);
    let mut expense_1_ref_0: &crate::expense::Expense = &mut expense_1;
    let mut ordering_0: std::cmp::Ordering = crate::expense::Expense::compare_dates(expense_1_ref_0, expense_0_ref_0);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Week(weekdelta_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1899() {
    rusty_monitor::set_test_id(1899);
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_0: u64 = 8744u64;
    let mut u64_1: u64 = 6804u64;
    let mut u64_2: u64 = 5355u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_2, u64_1, u64_0);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut u64_3: u64 = 4058u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_3};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_4: u64 = 7288u64;
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_4);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_5: u64 = 6085u64;
    let mut u64_6: u64 = 902u64;
    let mut u64_7: u64 = 3757u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_7, u64_6, u64_5);
    let mut i64_0: i64 = -7941i64;
    let mut str_0: &str = "dU7";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_8: u64 = 7123u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_8, string_0, i64_0, simpledate_1, option_1, option_0, vec_0);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Never;
    let mut u64_9: u64 = 7576u64;
    let mut yeardelta_1: crate::date::YearDelta = crate::date::YearDelta {nth: u64_9};
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Year(yeardelta_1);
    let mut u64_10: u64 = 9556u64;
    let mut duration_1: date::Duration = crate::date::Duration::Month(u64_10);
    let mut repetition_1: crate::date::Repetition = crate::date::Repetition {delta: repdelta_1, end: repend_1};
    let mut option_2: &std::option::Option<crate::date::SimpleDate> = crate::expense::Expense::get_end_date(expense_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2447() {
    rusty_monitor::set_test_id(2447);
    let mut u64_0: u64 = 2578u64;
    let mut u64_1: u64 = 7665u64;
    let mut u64_2: u64 = 4775u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_2, month: u64_1, day: u64_0};
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut u64_3: u64 = 5031u64;
    let mut u64_4: u64 = 3876u64;
    let mut u64_5: u64 = 3073u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_5, month: u64_4, day: u64_3};
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut u64_6: u64 = 1539u64;
    let mut u64_7: u64 = 2265u64;
    let mut u64_8: u64 = 7848u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_8, month: u64_7, day: u64_6};
    let mut simpledate_2_ref_0: &crate::date::SimpleDate = &mut simpledate_2;
    let mut u64_9: u64 = 2065u64;
    let mut u64_10: u64 = 3965u64;
    let mut u64_11: u64 = 2338u64;
    let mut simpledate_3: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_11, month: u64_10, day: u64_9};
    let mut simpledate_3_ref_0: &crate::date::SimpleDate = &mut simpledate_3;
    let mut vec_0: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_12: u64 = 2020u64;
    let mut u64_13: u64 = 3198u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_13);
    let mut u64_14: u64 = 4388u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_14};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut u64_15: u64 = 1980u64;
    let mut duration_0: date::Duration = crate::date::Duration::Week(u64_15);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_12, days: vec_0};
    let mut u64_16: u64 = crate::expense::count_overlap_days(simpledate_3_ref_0, simpledate_2_ref_0, simpledate_1_ref_0, simpledate_0_ref_0);
    panic!("From RustyUnit with love");
}
}