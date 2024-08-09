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
fn rusty_test_21() {
    rusty_monitor::set_test_id(21);
    let mut u64_0: u64 = 7016u64;
    let mut u64_1: u64 = 4153u64;
    let mut u64_2: u64 = 7736u64;
    let mut u64_3: u64 = 7349u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_3, month: u64_2, day: u64_1};
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut u64_4: u64 = 9959u64;
    let mut u64_5: u64 = 3648u64;
    let mut u64_6: u64 = 2502u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_6, u64_5, u64_4);
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut u64_7: u64 = 2647u64;
    let mut u64_8: u64 = 5754u64;
    let mut u64_9: u64 = 2611u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_9, month: u64_8, day: u64_7};
    let mut simpledate_2_ref_0: &crate::date::SimpleDate = &mut simpledate_2;
    let mut u64_10: u64 = 7404u64;
    let mut u64_11: u64 = 975u64;
    let mut u64_12: u64 = 7934u64;
    let mut simpledate_3: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_12, month: u64_11, day: u64_10};
    let mut simpledate_3_ref_0: &crate::date::SimpleDate = &mut simpledate_3;
    let mut vec_0: std::vec::Vec<crate::expense::Expense> = std::vec::Vec::new();
    let mut u64_13: u64 = 3288u64;
    let mut u64_14: u64 = 7345u64;
    let mut vec_1: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_15: u64 = 5009u64;
    let mut vec_2: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_16: u64 = 3582u64;
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_16, days: vec_2};
    let mut weekday_0: date::Weekday = crate::date::Weekday::Wednesday;
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnDate(monthdeltadate_0);
    let mut monthdeltadate_1: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_15, days: vec_1};
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_14};
    let mut u64_17: u64 = crate::expense::count_overlap_days(simpledate_3_ref_0, simpledate_2_ref_0, simpledate_1_ref_0, simpledate_0_ref_0);
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_802() {
    rusty_monitor::set_test_id(802);
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_0: u64 = 1548u64;
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut option_2: std::option::Option<date::Duration> = std::option::Option::None;
    let mut option_2_ref_0: &std::option::Option<date::Duration> = &mut option_2;
    let mut u64_1: u64 = 8668u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_1);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Sunday;
    let mut u64_2: u64 = 1988u64;
    let mut u64_3: u64 = 3932u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_3, weekid: u64_2, day: weekday_0};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_3: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_3_ref_0: &std::option::Option<crate::date::Repetition> = &mut option_3;
    let mut u64_4: u64 = 4349u64;
    let mut u64_5: u64 = 3872u64;
    let mut u64_6: u64 = 2837u64;
    let mut u64_7: u64 = 779u64;
    let mut u64_8: u64 = 3271u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_8, u64_7, u64_6);
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut option_4: std::option::Option<crate::date::SimpleDate> = crate::expense::Expense::end_date(simpledate_0_ref_0, option_3_ref_0, option_2_ref_0);
    let mut u64_9: u64 = 9755u64;
    let mut u64_10: u64 = 4298u64;
    let mut u64_11: u64 = 8240u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_11, month: u64_10, day: u64_9};
    let mut i64_0: i64 = -13795i64;
    let mut str_0: &str = "tOmo70MncQe";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_12: u64 = 499u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense {id: u64_12, description: string_0, amount: i64_0, start: simpledate_1, end: option_4, spread: option_1, repetition: option_0, tags: vec_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4901() {
    rusty_monitor::set_test_id(4901);
    let mut u64_0: u64 = 1492u64;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Tuesday;
    let mut u64_1: u64 = 1302u64;
    let mut u64_2: u64 = 4581u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_2, weekid: u64_1, day: weekday_0};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    let mut u64_3: u64 = 5924u64;
    let mut u64_4: u64 = 6842u64;
    let mut u64_5: u64 = 6231u64;
    let mut u64_6: u64 = 6555u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_6, u64_5, u64_4);
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut u64_7: u64 = 2413u64;
    let mut u64_8: u64 = 3567u64;
    let mut u64_9: u64 = 5013u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_9, month: u64_8, day: u64_7};
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut u64_10: u64 = 7709u64;
    let mut u64_11: u64 = 7068u64;
    let mut u64_12: u64 = 8621u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_12, u64_11, u64_10);
    let mut simpledate_2_ref_0: &crate::date::SimpleDate = &mut simpledate_2;
    let mut u64_13: u64 = 5868u64;
    let mut u64_14: u64 = 4011u64;
    let mut u64_15: u64 = 1191u64;
    let mut simpledate_3: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_15, u64_14, u64_13);
    let mut simpledate_3_ref_0: &crate::date::SimpleDate = &mut simpledate_3;
    let mut vec_0: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_16: u64 = 7104u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_16, on: vec_0};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Week(weekdelta_0);
    let mut u64_17: u64 = crate::expense::count_overlap_days(simpledate_3_ref_0, simpledate_2_ref_0, simpledate_1_ref_0, simpledate_0_ref_0);
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_326() {
    rusty_monitor::set_test_id(326);
    let mut u64_0: u64 = 3196u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_0};
    let mut u64_1: u64 = 9320u64;
    let mut u64_2: u64 = 4505u64;
    let mut u64_3: u64 = 5374u64;
    let mut u64_4: u64 = 5962u64;
    let mut u64_5: u64 = 4955u64;
    let mut u64_6: u64 = 2532u64;
    let mut u64_7: u64 = 8732u64;
    let mut u64_8: u64 = 5279u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_8, u64_7, u64_6);
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut u64_9: u64 = 7075u64;
    let mut u64_10: u64 = 3589u64;
    let mut u64_11: u64 = 5743u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_11, u64_10, u64_9);
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut u64_12: u64 = 1675u64;
    let mut u64_13: u64 = 5421u64;
    let mut u64_14: u64 = 110u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_14, u64_13, u64_12);
    let mut simpledate_2_ref_0: &crate::date::SimpleDate = &mut simpledate_2;
    let mut u64_15: u64 = 3015u64;
    let mut u64_16: u64 = 7001u64;
    let mut u64_17: u64 = 2359u64;
    let mut simpledate_3: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_17, u64_16, u64_15);
    let mut simpledate_3_ref_0: &crate::date::SimpleDate = &mut simpledate_3;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Sunday;
    let mut u64_18: u64 = crate::expense::count_overlap_days(simpledate_3_ref_0, simpledate_2_ref_0, simpledate_1_ref_0, simpledate_0_ref_0);
    let mut weekday_1: date::Weekday = crate::date::Weekday::Tuesday;
    let mut weekday_2: date::Weekday = crate::date::Weekday::Tuesday;
    let mut weekday_3: date::Weekday = crate::date::Weekday::Thursday;
    let mut simpledate_4: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_3, month: u64_2, day: u64_1};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3688() {
    rusty_monitor::set_test_id(3688);
    let mut u64_0: u64 = 7126u64;
    let mut u64_1: u64 = 6823u64;
    let mut u64_2: u64 = 1815u64;
    let mut vec_0: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_3: u64 = 1540u64;
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_3, days: vec_0};
    let mut vec_1: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut vec_2: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_4: u64 = 2003u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_4, on: vec_2};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Week(weekdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_5: u64 = 8820u64;
    let mut duration_0: date::Duration = crate::date::Duration::Year(u64_5);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_6: u64 = 8048u64;
    let mut u64_7: u64 = 3220u64;
    let mut u64_8: u64 = 3939u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_8, u64_7, u64_6);
    let mut i64_0: i64 = -1706i64;
    let mut str_0: &str = "kOY5";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_9: u64 = 7895u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_9, string_0, i64_0, simpledate_0, option_1, option_0, vec_1);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Tuesday;
    let mut option_2: &std::option::Option<crate::date::SimpleDate> = crate::expense::Expense::get_end_date(expense_0_ref_0);
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnDate(monthdeltadate_0);
    let mut weekday_1: date::Weekday = crate::date::Weekday::Friday;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_2, month: u64_1, day: u64_0};
    panic!("From RustyUnit with love");
}
}