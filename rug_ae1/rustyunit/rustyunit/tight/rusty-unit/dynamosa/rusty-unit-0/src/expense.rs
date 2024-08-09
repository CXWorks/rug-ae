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
fn rusty_test_7556() {
    rusty_monitor::set_test_id(7556);
    let mut u64_0: u64 = 9672u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_0};
    let mut option_0: std::option::Option<date::Duration> = std::option::Option::None;
    let mut option_0_ref_0: &std::option::Option<date::Duration> = &mut option_0;
    let mut u64_1: u64 = 5650u64;
    let mut u64_2: u64 = 8528u64;
    let mut u64_3: u64 = 6951u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_3, u64_2, u64_1);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut u64_4: u64 = 5278u64;
    let mut daydelta_1: crate::date::DayDelta = crate::date::DayDelta {nth: u64_4};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_1);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_1: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_1_ref_0: &std::option::Option<crate::date::Repetition> = &mut option_1;
    let mut u64_5: u64 = 2882u64;
    let mut u64_6: u64 = 1476u64;
    let mut u64_7: u64 = 172u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_7, month: u64_6, day: u64_5};
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut option_2: std::option::Option<crate::date::SimpleDate> = crate::expense::Expense::end_date(simpledate_1_ref_0, option_1_ref_0, option_0_ref_0);
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Thursday;
    let mut simpledate_2: crate::date::SimpleDate = std::option::Option::unwrap(option_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4276() {
    rusty_monitor::set_test_id(4276);
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut u64_0: u64 = 8352u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_0};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_1: u64 = 1769u64;
    let mut duration_0: date::Duration = crate::date::Duration::Year(u64_1);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_2: u64 = 9682u64;
    let mut duration_1: date::Duration = crate::date::Duration::Month(u64_2);
    let mut option_2: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
    let mut option_2_ref_0: &std::option::Option<date::Duration> = &mut option_2;
    let mut u64_3: u64 = 2808u64;
    let mut u64_4: u64 = 4540u64;
    let mut u64_5: u64 = 7642u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_5, month: u64_4, day: u64_3};
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut u64_6: u64 = 5268u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_6};
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut repetition_1: crate::date::Repetition = crate::date::Repetition {delta: repdelta_1, end: repend_1};
    let mut option_3: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_1);
    let mut option_3_ref_0: &std::option::Option<crate::date::Repetition> = &mut option_3;
    let mut u64_7: u64 = 7820u64;
    let mut u64_8: u64 = 5473u64;
    let mut u64_9: u64 = 8971u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_9, month: u64_8, day: u64_7};
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut option_4: std::option::Option<crate::date::SimpleDate> = crate::expense::Expense::end_date(simpledate_1_ref_0, option_3_ref_0, option_2_ref_0);
    let mut u64_10: u64 = 5193u64;
    let mut u64_11: u64 = 8404u64;
    let mut u64_12: u64 = 2467u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_12, month: u64_11, day: u64_10};
    let mut i64_0: i64 = -10701i64;
    let mut str_0: &str = "a3yhcY6MWhsHUCWi0";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_13: u64 = 7900u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense {id: u64_13, description: string_0, amount: i64_0, start: simpledate_2, end: option_4, spread: option_1, repetition: option_0, tags: vec_0};
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut i64_1: i64 = crate::expense::Expense::amount(expense_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7653() {
    rusty_monitor::set_test_id(7653);
    let mut u64_0: u64 = 8826u64;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_0);
    let mut option_0: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut option_1: std::option::Option<crate::date::SimpleDate> = std::option::Option::None;
    let mut u64_1: u64 = 8991u64;
    let mut u64_2: u64 = 4393u64;
    let mut u64_3: u64 = 8230u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_3, month: u64_2, day: u64_1};
    let mut u64_4: u64 = 7969u64;
    let mut u64_5: u64 = 4296u64;
    let mut u64_6: u64 = 9404u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_6, u64_5, u64_4);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_1);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Thursday;
    let mut u64_7: u64 = 7204u64;
    let mut u64_8: u64 = 1209u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_8, weekid: u64_7, day: weekday_0};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_9: u64 = 8865u64;
    let mut duration_1: date::Duration = crate::date::Duration::Month(u64_9);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
    let mut u64_10: u64 = 9375u64;
    let mut u64_11: u64 = 4273u64;
    let mut u64_12: u64 = 7091u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_12, month: u64_11, day: u64_10};
    let mut option_4: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_13: u64 = 7754u64;
    let mut duration_2: date::Duration = crate::date::Duration::Month(u64_13);
    let mut option_5: std::option::Option<date::Duration> = std::option::Option::Some(duration_2);
    let mut option_6: std::option::Option<crate::date::SimpleDate> = std::option::Option::None;
    let mut u64_14: u64 = 3382u64;
    let mut u64_15: u64 = 3699u64;
    let mut u64_16: u64 = 5334u64;
    let mut simpledate_3: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_16, month: u64_15, day: u64_14};
    let mut option_7: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_8: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_17: u64 = 1875u64;
    let mut u64_18: u64 = 6298u64;
    let mut u64_19: u64 = 9535u64;
    let mut simpledate_4: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_19, month: u64_18, day: u64_17};
    let mut u64_20: u64 = 6016u64;
    let mut u64_21: u64 = 8104u64;
    let mut u64_22: u64 = 2779u64;
    let mut u64_23: u64 = 9704u64;
    let mut u64_24: u64 = 2843u64;
    let mut simpledate_5: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_24, month: u64_23, day: u64_22};
    let mut simpledate_5_ref_0: &crate::date::SimpleDate = &mut simpledate_5;
    let mut u64_25: u64 = 5879u64;
    let mut u64_26: u64 = 2405u64;
    let mut u64_27: u64 = 6395u64;
    let mut simpledate_6: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_27, u64_26, u64_25);
    let mut simpledate_6_ref_0: &crate::date::SimpleDate = &mut simpledate_6;
    let mut simpledate_2_ref_0: &crate::date::SimpleDate = &mut simpledate_2;
    let mut u64_28: u64 = 6914u64;
    let mut u64_29: u64 = 1557u64;
    let mut u64_30: u64 = 1578u64;
    let mut simpledate_7: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_30, u64_29, u64_28);
    let mut simpledate_7_ref_0: &crate::date::SimpleDate = &mut simpledate_7;
    let mut u64_31: u64 = crate::expense::count_overlap_days(simpledate_7_ref_0, simpledate_2_ref_0, simpledate_6_ref_0, simpledate_5_ref_0);
    let mut weekday_1: date::Weekday = crate::date::Weekday::Sunday;
    let mut duration_3: date::Duration = crate::date::Duration::Week(u64_21);
    let mut weekday_2: date::Weekday = crate::date::Weekday::Thursday;
    let mut duration_4: date::Duration = crate::date::Duration::Week(u64_20);
    let mut weekday_3: date::Weekday = crate::date::Weekday::Tuesday;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7150() {
    rusty_monitor::set_test_id(7150);
    let mut u64_0: u64 = 3929u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_0};
    let mut u64_1: u64 = 7038u64;
    let mut u64_2: u64 = 5794u64;
    let mut u64_3: u64 = 8056u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_3, month: u64_2, day: u64_1};
    let mut u64_4: u64 = 7756u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_4);
    let mut vec_0: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_5: u64 = 493u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_5, on: vec_0};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Week(weekdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut u64_6: u64 = 297u64;
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Count(u64_6);
    let mut u64_7: u64 = 7200u64;
    let mut yeardelta_1: crate::date::YearDelta = crate::date::YearDelta {nth: u64_7};
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Year(yeardelta_1);
    let mut u64_8: u64 = 771u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_8};
    let mut str_0: &str = "XKERY5LN5gVdVOePz";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u64_9: u64 = 1493u64;
    let mut u64_10: u64 = 5715u64;
    let mut u64_11: u64 = 2561u64;
    let mut u64_12: u64 = 6376u64;
    let mut u64_13: u64 = 7252u64;
    let mut repend_2: date::RepEnd = crate::date::RepEnd::Count(u64_12);
    let mut daydelta_1: crate::date::DayDelta = crate::date::DayDelta {nth: u64_13};
    let mut repdelta_2: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut repetition_1: crate::date::Repetition = crate::date::Repetition {delta: repdelta_1, end: repend_1};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_14: u64 = 1894u64;
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_15: u64 = 4330u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_15, u64_11, u64_14);
    let mut option_2: std::option::Option<crate::date::SimpleDate> = std::option::Option::Some(simpledate_0);
    let mut u64_16: u64 = 931u64;
    let mut u64_17: u64 = 7836u64;
    let mut u64_18: u64 = 6011u64;
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::None;
    let mut option_1_ref_0: &std::option::Option<date::Duration> = &mut option_1;
    let mut repend_3: date::RepEnd = crate::date::RepEnd::Never;
    let mut u64_19: u64 = 525u64;
    let mut yeardelta_2: crate::date::YearDelta = crate::date::YearDelta {nth: u64_17};
    let mut repdelta_3: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut repetition_2: crate::date::Repetition = crate::date::Repetition {delta: repdelta_2, end: repend_3};
    let mut option_4: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_1);
    let mut option_0_ref_0: &std::option::Option<crate::date::Repetition> = &mut option_0;
    let mut u64_20: u64 = 4258u64;
    let mut u64_21: u64 = 5588u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_20, u64_9, u64_18);
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut u64_22: u64 = 5495u64;
    let mut simpledate_3: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_19, u64_16, u64_10);
    let mut vec_1: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_21, days: vec_1};
    let mut repend_4: date::RepEnd = crate::date::RepEnd::Date(simpledate_2);
    let mut duration_0: date::Duration = crate::date::Duration::Year(u64_22);
    let mut option_5: std::option::Option<crate::date::SimpleDate> = crate::expense::Expense::end_date(simpledate_1_ref_0, option_0_ref_0, option_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2519() {
    rusty_monitor::set_test_id(2519);
    let mut u64_0: u64 = 2800u64;
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_1: u64 = 1638u64;
    let mut u64_2: u64 = 1012u64;
    let mut u64_3: u64 = 5947u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_3, month: u64_2, day: u64_1};
    let mut u64_4: u64 = 7252u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_4);
    let mut u64_5: u64 = 7317u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_5};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_6: u64 = 2995u64;
    let mut u64_7: u64 = 4330u64;
    let mut u64_8: u64 = 3234u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_8, u64_7, u64_6);
    let mut option_4: std::option::Option<crate::date::SimpleDate> = std::option::Option::Some(simpledate_1);
    let mut u64_9: u64 = 931u64;
    let mut u64_10: u64 = 7836u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_10, month: u64_9, day: u64_0};
    let mut option_5: std::option::Option<date::Duration> = std::option::Option::None;
    let mut option_5_ref_0: &std::option::Option<date::Duration> = &mut option_5;
    let mut u64_11: u64 = 911u64;
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Count(u64_11);
    let mut u64_12: u64 = 525u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_12};
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut repetition_1: crate::date::Repetition = crate::date::Repetition {delta: repdelta_1, end: repend_1};
    let mut option_6: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_1);
    let mut option_6_ref_0: &std::option::Option<crate::date::Repetition> = &mut option_6;
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut u64_13: u64 = 590u64;
    let mut vec_0: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_14: u64 = 1653u64;
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_14, days: vec_0};
    let mut repend_2: date::RepEnd = crate::date::RepEnd::Date(simpledate_2);
    let mut duration_0: date::Duration = crate::date::Duration::Year(u64_13);
    let mut option_7: std::option::Option<crate::date::SimpleDate> = crate::expense::Expense::end_date(simpledate_0_ref_0, option_6_ref_0, option_5_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_146() {
    rusty_monitor::set_test_id(146);
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut option_2: std::option::Option<date::Duration> = std::option::Option::None;
    let mut option_2_ref_0: &std::option::Option<date::Duration> = &mut option_2;
    let mut u64_0: u64 = 6708u64;
    let mut u64_1: u64 = 4924u64;
    let mut u64_2: u64 = 8248u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_2, month: u64_1, day: u64_0};
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut u64_3: u64 = 5923u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_3};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_3: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_3_ref_0: &std::option::Option<crate::date::Repetition> = &mut option_3;
    let mut u64_4: u64 = 9507u64;
    let mut u64_5: u64 = 1958u64;
    let mut u64_6: u64 = 938u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_6, u64_5, u64_4);
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut option_4: std::option::Option<crate::date::SimpleDate> = crate::expense::Expense::end_date(simpledate_1_ref_0, option_3_ref_0, option_2_ref_0);
    let mut u64_7: u64 = 4669u64;
    let mut u64_8: u64 = 4395u64;
    let mut u64_9: u64 = 3287u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_9, u64_8, u64_7);
    let mut i64_0: i64 = 15112i64;
    let mut str_0: &str = "BoERtwdxBwZZ";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_10: u64 = 1764u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense {id: u64_10, description: string_0, amount: i64_0, start: simpledate_2, end: option_4, spread: option_1, repetition: option_0, tags: vec_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8365() {
    rusty_monitor::set_test_id(8365);
    let mut u64_0: u64 = 7217u64;
    let mut duration_0: date::Duration = crate::date::Duration::Week(u64_0);
    let mut option_0: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut option_0_ref_0: &std::option::Option<date::Duration> = &mut option_0;
    let mut u64_1: u64 = 2838u64;
    let mut u64_2: u64 = 2066u64;
    let mut u64_3: u64 = 392u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_3, month: u64_2, day: u64_1};
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut u64_4: u64 = 5597u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_4};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_1: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_1_ref_0: &std::option::Option<crate::date::Repetition> = &mut option_1;
    let mut u64_5: u64 = 9509u64;
    let mut u64_6: u64 = 7631u64;
    let mut u64_7: u64 = 6380u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_7, u64_6, u64_5);
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut option_2: std::option::Option<crate::date::SimpleDate> = crate::expense::Expense::end_date(simpledate_1_ref_0, option_1_ref_0, option_0_ref_0);
    let mut u64_8: u64 = 7246u64;
    let mut u64_9: u64 = 1493u64;
    let mut u64_10: u64 = 5715u64;
    let mut u64_11: u64 = 2561u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_11, month: u64_10, day: u64_9};
    let mut u64_12: u64 = 6160u64;
    let mut u64_13: u64 = 932u64;
    let mut u64_14: u64 = 5158u64;
    let mut simpledate_3: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_14, u64_13, u64_12);
    let mut simpledate_3_ref_0: &crate::date::SimpleDate = &mut simpledate_3;
    let mut str_0: &str = "yJOJNbwRjblaoQ";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Monday;
    let mut u64_15: u64 = 5527u64;
    let mut u64_16: u64 = 8584u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_16, weekid: u64_15, day: weekday_0};
    let mut daydelta_1: crate::date::DayDelta = crate::date::DayDelta {nth: u64_8};
    let mut str_1: &str = "3gG4WPAEHlmQg0iKYm";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut vec_0: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_17: u64 = 5259u64;
    let mut duration_1: date::Duration = crate::date::Duration::Month(u64_17);
    let mut weekday_1: date::Weekday = crate::date::Weekday::Thursday;
    let mut simpledate_4: crate::date::SimpleDate = std::option::Option::unwrap(option_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5730() {
    rusty_monitor::set_test_id(5730);
    let mut u64_0: u64 = 8826u64;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_0);
    let mut option_0: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_1: u64 = 8991u64;
    let mut u64_2: u64 = 4393u64;
    let mut u64_3: u64 = 8230u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_3, month: u64_2, day: u64_1};
    let mut u64_4: u64 = 7969u64;
    let mut u64_5: u64 = 4296u64;
    let mut u64_6: u64 = 9404u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_6, u64_5, u64_4);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_1);
    let mut u64_7: u64 = 8865u64;
    let mut duration_1: date::Duration = crate::date::Duration::Month(u64_7);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
    let mut u64_8: u64 = 9375u64;
    let mut u64_9: u64 = 4273u64;
    let mut u64_10: u64 = 7091u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_10, month: u64_9, day: u64_8};
    let mut u64_11: u64 = 9920u64;
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_12: u64 = 7754u64;
    let mut duration_2: date::Duration = crate::date::Duration::Month(u64_12);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::Some(duration_2);
    let mut option_4: std::option::Option<crate::date::SimpleDate> = std::option::Option::None;
    let mut u64_13: u64 = 3382u64;
    let mut u64_14: u64 = 3699u64;
    let mut u64_15: u64 = 5334u64;
    let mut simpledate_3: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_15, month: u64_14, day: u64_13};
    let mut option_5: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_6: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_16: u64 = 1875u64;
    let mut u64_17: u64 = 6298u64;
    let mut u64_18: u64 = 9535u64;
    let mut simpledate_4: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_18, month: u64_17, day: u64_16};
    let mut u64_19: u64 = 6016u64;
    let mut u64_20: u64 = 8104u64;
    let mut u64_21: u64 = 2779u64;
    let mut u64_22: u64 = 9704u64;
    let mut u64_23: u64 = 2843u64;
    let mut simpledate_5: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_23, month: u64_22, day: u64_21};
    let mut simpledate_5_ref_0: &crate::date::SimpleDate = &mut simpledate_5;
    let mut u64_24: u64 = 5879u64;
    let mut u64_25: u64 = 2405u64;
    let mut u64_26: u64 = 6395u64;
    let mut simpledate_6: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_26, u64_25, u64_24);
    let mut simpledate_6_ref_0: &crate::date::SimpleDate = &mut simpledate_6;
    let mut u64_27: u64 = 1171u64;
    let mut u64_28: u64 = 6892u64;
    let mut u64_29: u64 = 4013u64;
    let mut simpledate_7: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_29, month: u64_28, day: u64_27};
    let mut simpledate_7_ref_0: &crate::date::SimpleDate = &mut simpledate_7;
    let mut u64_30: u64 = 1557u64;
    let mut u64_31: u64 = 1578u64;
    let mut simpledate_8: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_31, u64_30, u64_11);
    let mut simpledate_8_ref_0: &crate::date::SimpleDate = &mut simpledate_8;
    let mut u64_32: u64 = crate::expense::count_overlap_days(simpledate_8_ref_0, simpledate_7_ref_0, simpledate_6_ref_0, simpledate_5_ref_0);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Sunday;
    let mut duration_3: date::Duration = crate::date::Duration::Week(u64_20);
    let mut weekday_1: date::Weekday = crate::date::Weekday::Thursday;
    let mut duration_4: date::Duration = crate::date::Duration::Week(u64_19);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6110() {
    rusty_monitor::set_test_id(6110);
    let mut u64_0: u64 = 7409u64;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_0);
    let mut u64_1: u64 = 8851u64;
    let mut duration_1: date::Duration = crate::date::Duration::Day(u64_1);
    let mut u64_2: u64 = 9446u64;
    let mut duration_2: date::Duration = crate::date::Duration::Year(u64_2);
    let mut duration_2_ref_0: &date::Duration = &mut duration_2;
    let mut u64_3: u64 = 3727u64;
    let mut u64_4: u64 = 5228u64;
    let mut u64_5: u64 = 2889u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_5, u64_4, u64_3);
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut u64_6: u64 = 545u64;
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_7: u64 = 9604u64;
    let mut duration_3: date::Duration = crate::date::Duration::Day(u64_7);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_3);
    let mut u64_8: u64 = 6878u64;
    let mut u64_9: u64 = 6034u64;
    let mut u64_10: u64 = 2032u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_10, u64_9, u64_8);
    let mut u64_11: u64 = 1131u64;
    let mut u64_12: u64 = 3295u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_12};
    let mut weekday_0: date::Weekday = crate::date::Weekday::Thursday;
    let mut u64_13: u64 = 4415u64;
    let mut u64_14: u64 = 8450u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_14, weekid: u64_13, day: weekday_0};
    let mut u64_15: u64 = 5185u64;
    let mut u64_16: u64 = 9749u64;
    let mut option_2: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
    let mut u64_17: u64 = 9375u64;
    let mut u64_18: u64 = 7091u64;
    let mut u64_19: u64 = 9920u64;
    let mut option_3: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut duration_4: date::Duration = crate::date::Duration::Month(u64_11);
    let mut option_4: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut option_5: std::option::Option<crate::date::SimpleDate> = std::option::Option::None;
    let mut u64_20: u64 = 5334u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_19, month: u64_18, day: u64_16};
    let mut u64_21: u64 = 407u64;
    let mut option_6: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_7: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_22: u64 = 1875u64;
    let mut u64_23: u64 = 6298u64;
    let mut u64_24: u64 = 9535u64;
    let mut simpledate_3: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_21, month: u64_6, day: u64_15};
    let mut u64_25: u64 = 3006u64;
    let mut u64_26: u64 = 6016u64;
    let mut u64_27: u64 = 2779u64;
    let mut u64_28: u64 = 9704u64;
    let mut u64_29: u64 = 2843u64;
    let mut simpledate_4: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_24, month: u64_22, day: u64_25};
    let mut simpledate_2_ref_0: &crate::date::SimpleDate = &mut simpledate_2;
    let mut u64_30: u64 = 2405u64;
    let mut u64_31: u64 = 6395u64;
    let mut simpledate_5: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_20, u64_26, u64_23);
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut simpledate_6: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_31, month: u64_29, day: u64_30};
    let mut simpledate_3_ref_0: &crate::date::SimpleDate = &mut simpledate_3;
    let mut simpledate_7: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_28, u64_27, u64_17);
    let mut simpledate_4_ref_0: &crate::date::SimpleDate = &mut simpledate_4;
    let mut u64_32: u64 = crate::expense::count_overlap_days(simpledate_1_ref_0, simpledate_2_ref_0, simpledate_0_ref_0, simpledate_3_ref_0);
    let mut repetition_0: crate::date::Repetition = std::option::Option::unwrap(option_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_121() {
    rusty_monitor::set_test_id(121);
    let mut u64_0: u64 = 7134u64;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut option_2: std::option::Option<date::Duration> = std::option::Option::None;
    let mut option_2_ref_0: &std::option::Option<date::Duration> = &mut option_2;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut u64_1: u64 = 5219u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_1};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_3: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_3_ref_0: &std::option::Option<crate::date::Repetition> = &mut option_3;
    let mut u64_2: u64 = 1980u64;
    let mut u64_3: u64 = 5522u64;
    let mut u64_4: u64 = 6753u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_4, u64_3, u64_2);
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut option_4: std::option::Option<crate::date::SimpleDate> = crate::expense::Expense::end_date(simpledate_0_ref_0, option_3_ref_0, option_2_ref_0);
    let mut u64_5: u64 = 2155u64;
    let mut u64_6: u64 = 1473u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_0, month: u64_6, day: u64_5};
    let mut i64_0: i64 = -3301i64;
    let mut str_0: &str = "Syf0TZdoDawi";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_7: u64 = 3817u64;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Thursday;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense {id: u64_7, description: string_0, amount: i64_0, start: simpledate_1, end: option_4, spread: option_1, repetition: option_0, tags: vec_0};
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut option_5: &std::option::Option<crate::date::SimpleDate> = crate::expense::Expense::get_end_date(expense_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2314() {
    rusty_monitor::set_test_id(2314);
    let mut u64_0: u64 = 8616u64;
    let mut u64_1: u64 = 2162u64;
    let mut u64_2: u64 = 5422u64;
    let mut u64_3: u64 = 9844u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_3);
    let mut u64_4: u64 = 7726u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_4};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut option_0: std::option::Option<date::Duration> = std::option::Option::None;
    let mut option_0_ref_0: &std::option::Option<date::Duration> = &mut option_0;
    let mut u64_5: u64 = 3110u64;
    let mut u64_6: u64 = 8743u64;
    let mut u64_7: u64 = 800u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_7, u64_6, u64_5);
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut u64_8: u64 = 1017u64;
    let mut u64_9: u64 = 8853u64;
    let mut u64_10: u64 = 6163u64;
    let mut u64_11: u64 = 1891u64;
    let mut u64_12: u64 = 4112u64;
    let mut u64_13: u64 = 9672u64;
    let mut daydelta_1: crate::date::DayDelta = crate::date::DayDelta {nth: u64_13};
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut option_1_ref_0: &std::option::Option<date::Duration> = &mut option_1;
    let mut u64_14: u64 = 5650u64;
    let mut u64_15: u64 = 8528u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_11, u64_15, u64_14);
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Date(simpledate_1);
    let mut u64_16: u64 = 469u64;
    let mut daydelta_2: crate::date::DayDelta = crate::date::DayDelta {nth: u64_16};
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Day(daydelta_2);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_1, end: repend_1};
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_2_ref_0: &std::option::Option<crate::date::Repetition> = &mut option_2;
    let mut u64_17: u64 = 2882u64;
    let mut u64_18: u64 = 1476u64;
    let mut u64_19: u64 = 172u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_19, month: u64_18, day: u64_17};
    let mut simpledate_2_ref_0: &crate::date::SimpleDate = &mut simpledate_2;
    let mut option_3: std::option::Option<crate::date::SimpleDate> = crate::expense::Expense::end_date(simpledate_0_ref_0, option_2_ref_0, option_0_ref_0);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Thursday;
    let mut weekday_1: date::Weekday = crate::date::Weekday::Thursday;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_12);
    let mut simpledate_3: crate::date::SimpleDate = std::option::Option::unwrap(option_3);
    let mut simpledate_4: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_10, u64_9, u64_8);
    let mut weekday_2: date::Weekday = crate::date::Weekday::Sunday;
    let mut repetition_1: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut simpledate_5: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_2, u64_1, u64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4507() {
    rusty_monitor::set_test_id(4507);
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut u64_0: u64 = 8352u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_0};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_1: u64 = 1769u64;
    let mut duration_0: date::Duration = crate::date::Duration::Year(u64_1);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_2: u64 = 9682u64;
    let mut duration_1: date::Duration = crate::date::Duration::Month(u64_2);
    let mut option_2: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
    let mut option_2_ref_0: &std::option::Option<date::Duration> = &mut option_2;
    let mut u64_3: u64 = 2808u64;
    let mut u64_4: u64 = 4540u64;
    let mut u64_5: u64 = 7642u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_5, month: u64_4, day: u64_3};
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut u64_6: u64 = 5268u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_6};
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut repetition_1: crate::date::Repetition = crate::date::Repetition {delta: repdelta_1, end: repend_1};
    let mut option_3: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_1);
    let mut option_3_ref_0: &std::option::Option<crate::date::Repetition> = &mut option_3;
    let mut u64_7: u64 = 7820u64;
    let mut u64_8: u64 = 5473u64;
    let mut u64_9: u64 = 8971u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_9, month: u64_8, day: u64_7};
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut option_4: std::option::Option<crate::date::SimpleDate> = crate::expense::Expense::end_date(simpledate_1_ref_0, option_3_ref_0, option_2_ref_0);
    let mut u64_10: u64 = 5193u64;
    let mut u64_11: u64 = 8404u64;
    let mut u64_12: u64 = 2467u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_12, month: u64_11, day: u64_10};
    let mut i64_0: i64 = -10701i64;
    let mut str_0: &str = "a3yhcY6MWhsHUCWi0";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_13: u64 = 7900u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense {id: u64_13, description: string_0, amount: i64_0, start: simpledate_2, end: option_4, spread: option_1, repetition: option_0, tags: vec_0};
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut vec_1: &std::vec::Vec<std::string::String> = crate::expense::Expense::tags(expense_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8417() {
    rusty_monitor::set_test_id(8417);
    let mut u64_0: u64 = 7977u64;
    let mut u64_1: u64 = 4512u64;
    let mut u64_2: u64 = 6404u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_2, month: u64_1, day: u64_0};
    let mut u64_3: u64 = 6163u64;
    let mut u64_4: u64 = 1891u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_4};
    let mut option_0: std::option::Option<date::Duration> = std::option::Option::None;
    let mut option_0_ref_0: &std::option::Option<date::Duration> = &mut option_0;
    let mut u64_5: u64 = 5650u64;
    let mut u64_6: u64 = 8528u64;
    let mut u64_7: u64 = 603u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_7, u64_6, u64_5);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_1);
    let mut u64_8: u64 = 5278u64;
    let mut daydelta_1: crate::date::DayDelta = crate::date::DayDelta {nth: u64_8};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_1);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_1: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_1_ref_0: &std::option::Option<crate::date::Repetition> = &mut option_1;
    let mut u64_9: u64 = 2882u64;
    let mut u64_10: u64 = 172u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_3, month: u64_10, day: u64_9};
    let mut simpledate_2_ref_0: &crate::date::SimpleDate = &mut simpledate_2;
    let mut option_2: std::option::Option<crate::date::SimpleDate> = crate::expense::Expense::end_date(simpledate_2_ref_0, option_1_ref_0, option_0_ref_0);
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Thursday;
    let mut weekday_1: date::Weekday = crate::date::Weekday::Thursday;
    let mut simpledate_3: crate::date::SimpleDate = std::option::Option::unwrap(option_2);
    let mut weekday_2: date::Weekday = crate::date::Weekday::Wednesday;
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6954() {
    rusty_monitor::set_test_id(6954);
    let mut u64_0: u64 = 6297u64;
    let mut u64_1: u64 = 4523u64;
    let mut u64_2: u64 = 9594u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_2, month: u64_1, day: u64_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_0_ref_0: &std::option::Option<crate::date::Repetition> = &mut option_0;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut u64_3: u64 = 861u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_3};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_1: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_4: u64 = 1769u64;
    let mut duration_0: date::Duration = crate::date::Duration::Year(u64_4);
    let mut option_2: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_5: u64 = 9682u64;
    let mut duration_1: date::Duration = crate::date::Duration::Month(u64_5);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
    let mut option_3_ref_0: &std::option::Option<date::Duration> = &mut option_3;
    let mut u64_6: u64 = 2808u64;
    let mut u64_7: u64 = 4540u64;
    let mut u64_8: u64 = 7642u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_8, month: u64_7, day: u64_6};
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Date(simpledate_1);
    let mut u64_9: u64 = 5268u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_9};
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut repetition_1: crate::date::Repetition = crate::date::Repetition {delta: repdelta_1, end: repend_1};
    let mut option_4: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_1);
    let mut option_4_ref_0: &std::option::Option<crate::date::Repetition> = &mut option_4;
    let mut u64_10: u64 = 7820u64;
    let mut u64_11: u64 = 5473u64;
    let mut u64_12: u64 = 8971u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_12, month: u64_11, day: u64_10};
    let mut simpledate_2_ref_0: &crate::date::SimpleDate = &mut simpledate_2;
    let mut option_5: std::option::Option<crate::date::SimpleDate> = crate::expense::Expense::end_date(simpledate_2_ref_0, option_0_ref_0, option_3_ref_0);
    let mut u64_13: u64 = 5193u64;
    let mut u64_14: u64 = 8404u64;
    let mut u64_15: u64 = 2467u64;
    let mut simpledate_3: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_15, month: u64_14, day: u64_13};
    let mut i64_0: i64 = -10701i64;
    let mut str_0: &str = "a3yhcY6MWhsHUCWi0";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_16: u64 = 7900u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense {id: u64_16, description: string_0, amount: i64_0, start: simpledate_3, end: option_5, spread: option_2, repetition: option_1, tags: vec_0};
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut str_1: &str = crate::expense::Expense::description(expense_0_ref_0);
    let mut repend_2: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5513() {
    rusty_monitor::set_test_id(5513);
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut u64_0: u64 = 1769u64;
    let mut duration_0: date::Duration = crate::date::Duration::Year(u64_0);
    let mut option_0: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_1: u64 = 9682u64;
    let mut duration_1: date::Duration = crate::date::Duration::Month(u64_1);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
    let mut option_1_ref_0: &std::option::Option<date::Duration> = &mut option_1;
    let mut u64_2: u64 = 2808u64;
    let mut u64_3: u64 = 4540u64;
    let mut u64_4: u64 = 7642u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_4, month: u64_3, day: u64_2};
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut u64_5: u64 = 5268u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_5};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_1};
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_2_ref_0: &std::option::Option<crate::date::Repetition> = &mut option_2;
    let mut u64_6: u64 = 7820u64;
    let mut u64_7: u64 = 5473u64;
    let mut u64_8: u64 = 8971u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_8, month: u64_7, day: u64_6};
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut option_3: std::option::Option<crate::date::SimpleDate> = crate::expense::Expense::end_date(simpledate_1_ref_0, option_2_ref_0, option_1_ref_0);
    let mut u64_9: u64 = 5193u64;
    let mut u64_10: u64 = 8404u64;
    let mut u64_11: u64 = 2467u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_11, month: u64_10, day: u64_9};
    let mut str_0: &str = "a3yhcY6MWhsHUCWi0";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    panic!("From RustyUnit with love");
}
}