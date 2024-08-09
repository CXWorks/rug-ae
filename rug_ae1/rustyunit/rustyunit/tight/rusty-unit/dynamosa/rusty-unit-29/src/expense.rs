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
fn rusty_test_5776() {
    rusty_monitor::set_test_id(5776);
    let mut str_0: &str = "D";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_0: u64 = 8178u64;
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_1: u64 = 3247u64;
    let mut duration_0: date::Duration = crate::date::Duration::Year(u64_1);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut option_3_ref_0: &std::option::Option<date::Duration> = &mut option_3;
    let mut option_4: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_4_ref_0: &std::option::Option<crate::date::Repetition> = &mut option_4;
    let mut u64_2: u64 = 5486u64;
    let mut u64_3: u64 = 4164u64;
    let mut u64_4: u64 = 2519u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_4, u64_3, u64_2);
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut option_5: std::option::Option<crate::date::SimpleDate> = crate::expense::Expense::end_date(simpledate_0_ref_0, option_4_ref_0, option_3_ref_0);
    let mut u64_5: u64 = 8102u64;
    let mut u64_6: u64 = 5799u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_6, month: u64_0, day: u64_5};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7789() {
    rusty_monitor::set_test_id(7789);
    let mut u64_0: u64 = 4900u64;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_0);
    let mut option_0: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut option_0_ref_0: &std::option::Option<date::Duration> = &mut option_0;
    let mut option_1: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_1_ref_0: &std::option::Option<crate::date::Repetition> = &mut option_1;
    let mut u64_1: u64 = 4612u64;
    let mut u64_2: u64 = 2274u64;
    let mut u64_3: u64 = 8804u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_3, month: u64_2, day: u64_1};
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut u64_4: u64 = 903u64;
    let mut u64_5: u64 = 274u64;
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_6: u64 = 4678u64;
    let mut duration_1: date::Duration = crate::date::Duration::Month(u64_6);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
    let mut u64_7: u64 = 6354u64;
    let mut u64_8: u64 = 6376u64;
    let mut u64_9: u64 = 8770u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_9, u64_8, u64_7);
    let mut u64_10: u64 = 681u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_10};
    let mut u64_11: u64 = 5486u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_5, u64_11, u64_4);
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut option_4: std::option::Option<crate::date::SimpleDate> = crate::expense::Expense::end_date(simpledate_0_ref_0, option_1_ref_0, option_0_ref_0);
    let mut simpledate_3: crate::date::SimpleDate = std::option::Option::unwrap(option_4);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5531() {
    rusty_monitor::set_test_id(5531);
    let mut u64_0: u64 = 3473u64;
    let mut u64_1: u64 = 4622u64;
    let mut u64_2: u64 = 1915u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_2, u64_1, u64_0);
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut u64_3: u64 = 3368u64;
    let mut u64_4: u64 = 3617u64;
    let mut u64_5: u64 = 5959u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_5, u64_4, u64_3);
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut u64_6: u64 = 8651u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_6);
    let mut option_0: std::option::Option<date::Duration> = std::option::Option::None;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Thursday;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_462() {
    rusty_monitor::set_test_id(462);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut u64_0: u64 = 7287u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_0};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut u64_1: u64 = 5846u64;
    let mut duration_0: date::Duration = crate::date::Duration::Year(u64_1);
    let mut option_0: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut option_0_ref_0: &std::option::Option<date::Duration> = &mut option_0;
    let mut u64_2: u64 = 3417u64;
    let mut u64_3: u64 = 4182u64;
    let mut u64_4: u64 = 7841u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_4, u64_3, u64_2);
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut u64_5: u64 = 9509u64;
    let mut yeardelta_1: crate::date::YearDelta = crate::date::YearDelta {nth: u64_5};
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Year(yeardelta_1);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_1, end: repend_1};
    let mut option_1: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_1_ref_0: &std::option::Option<crate::date::Repetition> = &mut option_1;
    let mut u64_6: u64 = 5185u64;
    let mut u64_7: u64 = 1338u64;
    let mut u64_8: u64 = 9024u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_8, month: u64_7, day: u64_6};
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut u64_9: u64 = 4132u64;
    let mut repend_2: date::RepEnd = crate::date::RepEnd::Count(u64_9);
    let mut option_2: std::option::Option<crate::date::SimpleDate> = crate::expense::Expense::end_date(simpledate_1_ref_0, option_1_ref_0, option_0_ref_0);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Wednesday;
    let mut weekday_1: date::Weekday = crate::date::Weekday::Monday;
    let mut repetition_1: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7008() {
    rusty_monitor::set_test_id(7008);
    let mut u64_0: u64 = 1110u64;
    let mut u64_1: u64 = 1455u64;
    let mut u64_2: u64 = 5879u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_2, month: u64_1, day: u64_0};
    let mut u64_3: u64 = 9316u64;
    let mut duration_0: date::Duration = crate::date::Duration::Week(u64_3);
    let mut u64_4: u64 = 3473u64;
    let mut u64_5: u64 = 4622u64;
    let mut u64_6: u64 = 1915u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_6, u64_5, u64_4);
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut u64_7: u64 = 3368u64;
    let mut u64_8: u64 = 3617u64;
    let mut u64_9: u64 = 5959u64;
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_10: u64 = 5068u64;
    let mut duration_1: date::Duration = crate::date::Duration::Month(u64_9);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_11: u64 = 3247u64;
    let mut duration_2: date::Duration = crate::date::Duration::Year(u64_8);
    let mut option_2: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
    let mut option_1_ref_0: &std::option::Option<date::Duration> = &mut option_1;
    let mut option_3: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_0_ref_0: &std::option::Option<crate::date::Repetition> = &mut option_0;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_10, u64_7, u64_11);
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut option_4: std::option::Option<crate::date::SimpleDate> = crate::expense::Expense::end_date(simpledate_1_ref_0, option_0_ref_0, option_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4478() {
    rusty_monitor::set_test_id(4478);
    let mut u64_0: u64 = 2213u64;
    let mut u64_1: u64 = 2645u64;
    let mut u64_2: u64 = 3886u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_2, month: u64_1, day: u64_0};
    let mut u64_3: u64 = 299u64;
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_3);
    let mut u64_4: u64 = 8399u64;
    let mut u64_5: u64 = 7553u64;
    let mut u64_6: u64 = 2506u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_6, month: u64_5, day: u64_4};
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut str_0: &str = "rS7dGDzcJncqO";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u64_7: u64 = 2534u64;
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_8: u64 = 5068u64;
    let mut duration_1: date::Duration = crate::date::Duration::Month(u64_7);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_9: u64 = 3247u64;
    let mut duration_2: date::Duration = crate::date::Duration::Year(u64_8);
    let mut option_2: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
    let mut option_1_ref_0: &std::option::Option<date::Duration> = &mut option_1;
    let mut option_3: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_0_ref_0: &std::option::Option<crate::date::Repetition> = &mut option_0;
    let mut u64_10: u64 = 5486u64;
    let mut u64_11: u64 = 4164u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_9, u64_11, u64_10);
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut option_4: std::option::Option<crate::date::SimpleDate> = crate::expense::Expense::end_date(simpledate_1_ref_0, option_0_ref_0, option_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1150() {
    rusty_monitor::set_test_id(1150);
    let mut u64_0: u64 = 5966u64;
    let mut u64_1: u64 = 3226u64;
    let mut u64_2: u64 = 1719u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_2, u64_1, u64_0);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut u64_3: u64 = 3356u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_3};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_4: u64 = 5700u64;
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_4);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_5: u64 = 6058u64;
    let mut duration_1: date::Duration = crate::date::Duration::Month(u64_5);
    let mut option_2: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
    let mut option_2_ref_0: &std::option::Option<date::Duration> = &mut option_2;
    let mut u64_6: u64 = 3126u64;
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Count(u64_6);
    let mut u64_7: u64 = 959u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_7};
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut repetition_1: crate::date::Repetition = crate::date::Repetition {delta: repdelta_1, end: repend_1};
    let mut option_3: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_1);
    let mut option_3_ref_0: &std::option::Option<crate::date::Repetition> = &mut option_3;
    let mut u64_8: u64 = 588u64;
    let mut u64_9: u64 = 5062u64;
    let mut u64_10: u64 = 4916u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_10, u64_9, u64_8);
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut option_4: std::option::Option<crate::date::SimpleDate> = crate::expense::Expense::end_date(simpledate_1_ref_0, option_3_ref_0, option_2_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6983() {
    rusty_monitor::set_test_id(6983);
    let mut u64_0: u64 = 2523u64;
    let mut u64_1: u64 = 6516u64;
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_2: u64 = 5699u64;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_2);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_3: u64 = 9019u64;
    let mut u64_4: u64 = 6610u64;
    let mut u64_5: u64 = 6876u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_5, month: u64_4, day: u64_3};
    let mut u64_6: u64 = 2077u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_6);
    let mut u64_7: u64 = 7091u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_7};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_8: u64 = 6555u64;
    let mut duration_1: date::Duration = crate::date::Duration::Day(u64_8);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
    let mut u64_9: u64 = 1158u64;
    let mut duration_2: date::Duration = crate::date::Duration::Month(u64_9);
    let mut option_4: std::option::Option<date::Duration> = std::option::Option::Some(duration_2);
    let mut option_4_ref_0: &std::option::Option<date::Duration> = &mut option_4;
    let mut option_5: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_5_ref_0: &std::option::Option<crate::date::Repetition> = &mut option_5;
    let mut u64_10: u64 = 4366u64;
    let mut u64_11: u64 = 951u64;
    let mut u64_12: u64 = 297u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_12, u64_11, u64_10);
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut option_6: std::option::Option<crate::date::SimpleDate> = crate::expense::Expense::end_date(simpledate_1_ref_0, option_5_ref_0, option_4_ref_0);
    let mut u64_13: u64 = 5901u64;
    let mut u64_14: u64 = 8281u64;
    let mut u64_15: u64 = 7394u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_15, month: u64_14, day: u64_13};
    let mut u64_16: u64 = 8176u64;
    let mut daydelta_1: crate::date::DayDelta = crate::date::DayDelta {nth: u64_16};
    let mut u64_17: u64 = 944u64;
    let mut u64_18: u64 = 1095u64;
    let mut simpledate_3: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_1, u64_18, u64_17);
    let mut simpledate_3_ref_0: &crate::date::SimpleDate = &mut simpledate_3;
    let mut u64_19: u64 = 2636u64;
    let mut u64_20: u64 = 2297u64;
    let mut u64_21: u64 = 379u64;
    let mut simpledate_4: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_21, u64_20, u64_19);
    let mut simpledate_4_ref_0: &crate::date::SimpleDate = &mut simpledate_4;
    let mut u64_22: u64 = 2881u64;
    let mut u64_23: u64 = 9315u64;
    let mut u64_24: u64 = 2601u64;
    let mut simpledate_5: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_24, month: u64_23, day: u64_22};
    let mut simpledate_5_ref_0: &crate::date::SimpleDate = &mut simpledate_5;
    let mut u64_25: u64 = 2089u64;
    let mut u64_26: u64 = 8655u64;
    let mut u64_27: u64 = 4218u64;
    let mut simpledate_6: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_27, month: u64_26, day: u64_25};
    let mut simpledate_6_ref_0: &crate::date::SimpleDate = &mut simpledate_6;
    let mut u64_28: u64 = crate::expense::count_overlap_days(simpledate_6_ref_0, simpledate_5_ref_0, simpledate_4_ref_0, simpledate_3_ref_0);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Wednesday;
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Day(daydelta_1);
    let mut duration_3: date::Duration = crate::date::Duration::Day(u64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4981() {
    rusty_monitor::set_test_id(4981);
    let mut u64_0: u64 = 4832u64;
    let mut u64_1: u64 = 8800u64;
    let mut u64_2: u64 = 1495u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_2, u64_1, u64_0);
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut u64_3: u64 = 9987u64;
    let mut u64_4: u64 = 5421u64;
    let mut u64_5: u64 = 9839u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_5, u64_4, u64_3);
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut u64_6: u64 = 1071u64;
    let mut u64_7: u64 = 3304u64;
    let mut u64_8: u64 = 6940u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_8, month: u64_7, day: u64_6};
    let mut simpledate_2_ref_0: &crate::date::SimpleDate = &mut simpledate_2;
    let mut u64_9: u64 = 6577u64;
    let mut u64_10: u64 = 5693u64;
    let mut u64_11: u64 = 1802u64;
    let mut simpledate_3: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_11, u64_10, u64_9);
    let mut simpledate_3_ref_0: &crate::date::SimpleDate = &mut simpledate_3;
    let mut str_0: &str = "D";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_12: u64 = 3920u64;
    let mut u64_13: u64 = 958u64;
    let mut u64_14: u64 = 2054u64;
    let mut simpledate_4: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_14, month: u64_13, day: u64_12};
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_15: u64 = 5068u64;
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_15);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_16: u64 = 3247u64;
    let mut duration_1: date::Duration = crate::date::Duration::Year(u64_16);
    let mut option_4: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
    let mut option_4_ref_0: &std::option::Option<date::Duration> = &mut option_4;
    let mut option_5: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_5_ref_0: &std::option::Option<crate::date::Repetition> = &mut option_5;
    let mut u64_17: u64 = 5486u64;
    let mut u64_18: u64 = 4164u64;
    let mut u64_19: u64 = 2519u64;
    let mut simpledate_5: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_19, u64_18, u64_17);
    let mut simpledate_5_ref_0: &crate::date::SimpleDate = &mut simpledate_5;
    let mut option_6: std::option::Option<crate::date::SimpleDate> = crate::expense::Expense::end_date(simpledate_5_ref_0, option_5_ref_0, option_4_ref_0);
    let mut u64_20: u64 = 8102u64;
    let mut u64_21: u64 = 5433u64;
    let mut u64_22: u64 = 5799u64;
    let mut simpledate_6: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_22, month: u64_21, day: u64_20};
    let mut u64_23: u64 = crate::expense::count_overlap_days(simpledate_3_ref_0, simpledate_2_ref_0, simpledate_1_ref_0, simpledate_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8420() {
    rusty_monitor::set_test_id(8420);
    let mut u64_0: u64 = 5279u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_0};
    let mut u64_1: u64 = 5084u64;
    let mut u64_2: u64 = 9679u64;
    let mut u64_3: u64 = 9751u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_3, u64_2, u64_1);
    let mut u64_4: u64 = 7820u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_4);
    let mut vec_0: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_5: u64 = 5945u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_5, on: vec_0};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Week(weekdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_6: u64 = 5531u64;
    let mut duration_0: date::Duration = crate::date::Duration::Week(u64_6);
    let mut u64_7: u64 = 132u64;
    let mut duration_1: date::Duration = crate::date::Duration::Month(u64_7);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Wednesday;
    let mut u64_8: u64 = 8989u64;
    let mut u64_9: u64 = 6114u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_9, weekid: u64_8, day: weekday_0};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    let mut str_0: &str = "pPWSGWJ9Coisnw0s";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u64_10: u64 = 5424u64;
    let mut u64_11: u64 = 2978u64;
    let mut u64_12: u64 = 5400u64;
    let mut u64_13: u64 = 1823u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_13, month: u64_12, day: u64_11};
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
    let mut u64_14: u64 = 3247u64;
    let mut duration_2: date::Duration = crate::date::Duration::Year(u64_10);
    let mut option_2: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut option_1_ref_0: &std::option::Option<date::Duration> = &mut option_1;
    let mut option_3: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_0_ref_0: &std::option::Option<crate::date::Repetition> = &mut option_0;
    let mut u64_15: u64 = 5486u64;
    let mut u64_16: u64 = 4164u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_16, u64_14, u64_15);
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut option_4: std::option::Option<crate::date::SimpleDate> = crate::expense::Expense::end_date(simpledate_1_ref_0, option_0_ref_0, option_1_ref_0);
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_546() {
    rusty_monitor::set_test_id(546);
    let mut option_0: std::option::Option<date::Duration> = std::option::Option::None;
    let mut option_0_ref_0: &std::option::Option<date::Duration> = &mut option_0;
    let mut u64_0: u64 = 8767u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_0);
    let mut vec_0: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_1: u64 = 5004u64;
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_1, days: vec_0};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnDate(monthdeltadate_0);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_1: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_1_ref_0: &std::option::Option<crate::date::Repetition> = &mut option_1;
    let mut u64_2: u64 = 2576u64;
    let mut u64_3: u64 = 370u64;
    let mut u64_4: u64 = 1239u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_4, month: u64_3, day: u64_2};
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut u64_5: u64 = 792u64;
    let mut u64_6: u64 = 8060u64;
    let mut u64_7: u64 = 5965u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_7, u64_6, u64_5);
    let mut u64_8: u64 = 9952u64;
    let mut duration_0: date::Duration = crate::date::Duration::Year(u64_8);
    let mut str_0: &str = "Ri00";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_2: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_9: u64 = 958u64;
    let mut u64_10: u64 = 2054u64;
    let mut u64_11: u64 = 8178u64;
    let mut option_3: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_12: u64 = 5068u64;
    let mut duration_1: date::Duration = crate::date::Duration::Month(u64_9);
    let mut option_4: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut duration_2: date::Duration = crate::date::Duration::Year(u64_11);
    let mut option_5: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
    let mut option_4_ref_0: &std::option::Option<date::Duration> = &mut option_4;
    let mut option_6: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_3_ref_0: &std::option::Option<crate::date::Repetition> = &mut option_3;
    let mut u64_13: u64 = 5486u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_13, u64_10, u64_12);
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut option_7: std::option::Option<crate::date::SimpleDate> = crate::expense::Expense::end_date(simpledate_0_ref_0, option_1_ref_0, option_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7855() {
    rusty_monitor::set_test_id(7855);
    let mut u64_0: u64 = 6100u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_0);
    let mut u64_1: u64 = 6200u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_1};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_2: u64 = 3625u64;
    let mut u64_3: u64 = 2444u64;
    let mut u64_4: u64 = 9391u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_4, month: u64_3, day: u64_2};
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_5: u64 = 7368u64;
    let mut duration_0: date::Duration = crate::date::Duration::Week(u64_5);
    let mut option_4: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut option_4_ref_0: &std::option::Option<date::Duration> = &mut option_4;
    let mut u64_6: u64 = 9955u64;
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Count(u64_6);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Tuesday;
    let mut u64_7: u64 = 1587u64;
    let mut u64_8: u64 = 9015u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_8, weekid: u64_7, day: weekday_0};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut repetition_1: crate::date::Repetition = crate::date::Repetition {delta: repdelta_1, end: repend_1};
    let mut option_5: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_1);
    let mut option_5_ref_0: &std::option::Option<crate::date::Repetition> = &mut option_5;
    let mut u64_9: u64 = 6620u64;
    let mut u64_10: u64 = 7579u64;
    let mut u64_11: u64 = 1182u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_11, u64_10, u64_9);
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut option_6: std::option::Option<crate::date::SimpleDate> = crate::expense::Expense::end_date(simpledate_1_ref_0, option_5_ref_0, option_4_ref_0);
    let mut u64_12: u64 = 608u64;
    let mut u64_13: u64 = 7218u64;
    let mut u64_14: u64 = 6487u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_14, u64_13, u64_12);
    let mut str_0: &str = "D";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_7: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_8: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_15: u64 = 3920u64;
    let mut u64_16: u64 = 958u64;
    let mut u64_17: u64 = 2054u64;
    let mut simpledate_3: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_17, month: u64_16, day: u64_15};
    let mut option_9: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_18: u64 = 5068u64;
    let mut duration_1: date::Duration = crate::date::Duration::Month(u64_18);
    let mut option_10: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
    let mut u64_19: u64 = 3247u64;
    let mut duration_2: date::Duration = crate::date::Duration::Year(u64_19);
    let mut option_11: std::option::Option<date::Duration> = std::option::Option::Some(duration_2);
    let mut option_11_ref_0: &std::option::Option<date::Duration> = &mut option_11;
    let mut option_12: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_12_ref_0: &std::option::Option<crate::date::Repetition> = &mut option_12;
    let mut u64_20: u64 = 5486u64;
    let mut u64_21: u64 = 4164u64;
    let mut u64_22: u64 = 2519u64;
    let mut simpledate_4: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_22, u64_21, u64_20);
    let mut simpledate_4_ref_0: &crate::date::SimpleDate = &mut simpledate_4;
    let mut option_13: std::option::Option<crate::date::SimpleDate> = crate::expense::Expense::end_date(simpledate_4_ref_0, option_12_ref_0, option_11_ref_0);
    let mut u64_23: u64 = 8102u64;
    let mut u64_24: u64 = 5433u64;
    let mut u64_25: u64 = 5799u64;
    let mut simpledate_5: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_25, month: u64_24, day: u64_23};
    panic!("From RustyUnit with love");
}
}