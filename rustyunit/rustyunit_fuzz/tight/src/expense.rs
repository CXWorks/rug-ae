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
    amount: i64,
    start: SimpleDate,
    end: Option<SimpleDate>,
    spread: Option<Duration>,
    repetition: Option<Repetition>,
    tags: Vec<String>,
}
#[derive(Debug)]
struct ExpenseError(String);
impl Expense {
    pub fn new(
        id: u64,
        description: String,
        amount: i64,
        start: SimpleDate,
        spread: Option<Duration>,
        repetition: Option<Repetition>,
        tags: Vec<String>,
    ) -> Expense {
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
    pub fn from_stdin(
        mut handle: &mut std::io::StdinLock,
        id: u64,
        is_income: bool,
        allowed_tags: &HashSet<String>,
    ) -> Result<Expense, Box<dyn Error>> {
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
            let result = scan_fmt::scan_fmt!(& spread_s, "{} {}", u64, String)?;
            if result.0 == 0 {
                None
            } else {
                match &result.1[..] {
                    "day" | "days" => Some(Duration::Day(result.0)),
                    "week" | "weeks" => Some(Duration::Week(result.0)),
                    "month" | "months" => Some(Duration::Month(result.0)),
                    "year" | "years" => Some(Duration::Year(result.0)),
                    _ => {
                        return Err(
                            Box::new(
                                ExpenseError(
                                    "invalid spread: only day/week/month/year(s) accepted"
                                        .into(),
                                ),
                            ),
                        );
                    }
                }
            }
        };
        let repetition = Repetition::from_stdin(&mut handle, &start)?;
        print!("tags (comma- or space-separated): ");
        std::io::stdout().flush()?;
        let mut tags_s = String::new();
        handle.read_line(&mut tags_s)?;
        let tags = tags_s
            .split(|c| c == ' ' || c == ',')
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.trim().to_string())
            .collect();
        for t in &tags {
            if !allowed_tags.contains(t) {
                return Err(Box::new(ExpenseError("tag not found!".into())));
            }
        }
        Ok(
            Expense::new(
                id,
                description.trim().to_string(),
                if is_income { amount } else { -amount },
                start,
                spread,
                repetition,
                tags,
            ),
        )
    }
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
            std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
            _ => {}
        }
        match self.start.cmp(&other.start) {
            std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
            std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
            _ => {}
        }
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
    fn end_date(
        start: &SimpleDate,
        repetition: &Option<Repetition>,
        spread: &Option<Duration>,
    ) -> Option<SimpleDate> {
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
fn count_overlap_days(
    period_start: &SimpleDate,
    period_end: &SimpleDate,
    expense_start: &SimpleDate,
    expense_end: &SimpleDate,
) -> u64 {
    if expense_end < period_start || expense_start > period_end {
        return 0;
    }
    let chr_period_start = chrono::NaiveDate::from_ymd(
        period_start.year.try_into().unwrap(),
        period_start.month.try_into().unwrap(),
        period_start.day.try_into().unwrap(),
    );
    let chr_period_end = chrono::NaiveDate::from_ymd(
        period_end.year.try_into().unwrap(),
        period_end.month.try_into().unwrap(),
        period_end.day.try_into().unwrap(),
    );
    let chr_ex_start = chrono::NaiveDate::from_ymd(
        expense_start.year.try_into().unwrap(),
        expense_start.month.try_into().unwrap(),
        expense_start.day.try_into().unwrap(),
    );
    let chr_ex_end = chrono::NaiveDate::from_ymd(
        expense_end.year.try_into().unwrap(),
        expense_end.month.try_into().unwrap(),
        expense_end.day.try_into().unwrap(),
    );
    if expense_start >= period_start && expense_end < period_end {
        return chr_ex_end.signed_duration_since(chr_ex_start).num_days() as u64;
    } else if period_start >= expense_start && period_end < expense_end {
        return chr_period_end.signed_duration_since(chr_period_start).num_days() as u64;
    }
    if expense_end < period_end {
        return chr_ex_end.signed_duration_since(chr_period_start).num_days() as u64;
    } else {
        return chr_period_end.signed_duration_since(chr_ex_start).num_days() as u64;
    }
}
pub fn calculate_spread(
    expenses: &[Expense],
    start: &SimpleDate,
    period: &Duration,
) -> f64 {
    let end = start + period;
    let mut sum = 0.0;
    for expense in expenses {
        let spread = expense.spread.as_ref().unwrap_or(&Duration::Day(1));
        let mut current_date = expense.start;
        if let Some(repetition) = &expense.repetition {
            while current_date < end {
                let spread_end = &current_date + spread;
                let spread_end_chr = chrono::NaiveDate::from_ymd(
                    spread_end.year.try_into().unwrap(),
                    spread_end.month.try_into().unwrap(),
                    spread_end.day.try_into().unwrap(),
                );
                let current_date_chr = chrono::NaiveDate::from_ymd(
                    current_date.year.try_into().unwrap(),
                    current_date.month.try_into().unwrap(),
                    current_date.day.try_into().unwrap(),
                );
                let n_days = spread_end_chr
                    .signed_duration_since(current_date_chr)
                    .num_days() as f64;
                let amount_per_day = (expense.amount as f64) / n_days;
                let overlap_days = count_overlap_days(
                    start,
                    &end,
                    &current_date,
                    &(&current_date + spread),
                );
                sum += amount_per_day * (overlap_days as f64);
                current_date = &current_date + &repetition.delta;
            }
        } else {
            let spread_end = &current_date + spread;
            let spread_end_chr = chrono::NaiveDate::from_ymd(
                spread_end.year.try_into().unwrap(),
                spread_end.month.try_into().unwrap(),
                spread_end.day.try_into().unwrap(),
            );
            let current_date_chr = chrono::NaiveDate::from_ymd(
                current_date.year.try_into().unwrap(),
                current_date.month.try_into().unwrap(),
                current_date.day.try_into().unwrap(),
            );
            let n_days = spread_end_chr
                .signed_duration_since(current_date_chr)
                .num_days() as f64;
            let amount_per_day = (expense.amount as f64) / n_days;
            let overlap_days = count_overlap_days(
                start,
                &end,
                &current_date,
                &(&current_date + spread),
            );
            sum += amount_per_day * (overlap_days as f64);
        }
    }
    sum / 100.0
}
impl fmt::Display for Expense {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, "{}: ${}.{:02} on {}", self.description, self.amount.abs() / 100, self
            .amount.abs() % 100, self.start
        )?;
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
        let overlap_days = count_overlap_days(
            &period_start,
            &period_end,
            &expense_start,
            &expense_end,
        );
        assert_eq!(overlap_days, 0);
    }
    #[test]
    fn count_overlap_days_exclusion_right() {
        let period_start = SimpleDate::from_ymd(2020, 11, 1);
        let period_end = SimpleDate::from_ymd(2020, 11, 30);
        let expense_start = SimpleDate::from_ymd(2020, 12, 1);
        let expense_end = SimpleDate::from_ymd(2020, 12, 31);
        let overlap_days = count_overlap_days(
            &period_start,
            &period_end,
            &expense_start,
            &expense_end,
        );
        assert_eq!(overlap_days, 0);
    }
    #[test]
    fn count_overlap_days_containment_inner() {
        let period_start = SimpleDate::from_ymd(2020, 11, 1);
        let period_end = SimpleDate::from_ymd(2020, 11, 30);
        let expense_start = SimpleDate::from_ymd(2020, 11, 2);
        let expense_end = SimpleDate::from_ymd(2020, 11, 29);
        let overlap_days = count_overlap_days(
            &period_start,
            &period_end,
            &expense_start,
            &expense_end,
        );
        assert_eq!(overlap_days, 27);
    }
    #[test]
    fn count_overlap_days_containment_outer() {
        let period_start = SimpleDate::from_ymd(2020, 11, 1);
        let period_end = SimpleDate::from_ymd(2020, 11, 30);
        let expense_start = SimpleDate::from_ymd(2020, 10, 31);
        let expense_end = SimpleDate::from_ymd(2020, 12, 1);
        let overlap_days = count_overlap_days(
            &period_start,
            &period_end,
            &expense_start,
            &expense_end,
        );
        assert_eq!(overlap_days, 29);
    }
    #[test]
    fn count_overlap_days_edge_left() {
        let period_start = SimpleDate::from_ymd(2020, 11, 1);
        let period_end = SimpleDate::from_ymd(2020, 11, 30);
        let expense_start = SimpleDate::from_ymd(2020, 10, 15);
        let expense_end = SimpleDate::from_ymd(2020, 11, 15);
        let overlap_days = count_overlap_days(
            &period_start,
            &period_end,
            &expense_start,
            &expense_end,
        );
        assert_eq!(overlap_days, 14);
    }
    #[test]
    fn count_overlap_days_edge_right() {
        let period_start = SimpleDate::from_ymd(2020, 11, 1);
        let period_end = SimpleDate::from_ymd(2020, 11, 30);
        let expense_start = SimpleDate::from_ymd(2020, 11, 15);
        let expense_end = SimpleDate::from_ymd(2020, 12, 15);
        let overlap_days = count_overlap_days(
            &period_start,
            &period_end,
            &expense_start,
            &expense_end,
        );
        assert_eq!(overlap_days, 15);
    }
}
impl Error for ExpenseError {}
#[cfg(test)]
mod tests_llm_16_56 {
    use super::*;
    use crate::*;
    #[test]
    fn test_compare_dates_self_end_none_other_end_none() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(u64, &str, i64, u64, u64, u64, u64, &str, i64, u64, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let exp1 = Expense {
            id: rug_fuzz_0,
            description: rug_fuzz_1.to_string(),
            amount: rug_fuzz_2,
            start: SimpleDate {
                year: rug_fuzz_3,
                month: rug_fuzz_4,
                day: rug_fuzz_5,
            },
            end: None,
            spread: None,
            repetition: None,
            tags: vec![],
        };
        let exp2 = Expense {
            id: rug_fuzz_6,
            description: rug_fuzz_7.to_string(),
            amount: rug_fuzz_8,
            start: SimpleDate {
                year: rug_fuzz_9,
                month: rug_fuzz_10,
                day: rug_fuzz_11,
            },
            end: None,
            spread: None,
            repetition: None,
            tags: vec![],
        };
        let result = exp1.compare_dates(&exp2);
        debug_assert_eq!(result, std::cmp::Ordering::Equal);
             }
});    }
    #[test]
    fn test_compare_dates_self_end_none() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14)) = <(u64, &str, i64, u64, u64, u64, u64, &str, i64, u64, u64, u64, u64, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let exp1 = Expense {
            id: rug_fuzz_0,
            description: rug_fuzz_1.to_string(),
            amount: rug_fuzz_2,
            start: SimpleDate {
                year: rug_fuzz_3,
                month: rug_fuzz_4,
                day: rug_fuzz_5,
            },
            end: None,
            spread: None,
            repetition: None,
            tags: vec![],
        };
        let exp2 = Expense {
            id: rug_fuzz_6,
            description: rug_fuzz_7.to_string(),
            amount: rug_fuzz_8,
            start: SimpleDate {
                year: rug_fuzz_9,
                month: rug_fuzz_10,
                day: rug_fuzz_11,
            },
            end: Some(SimpleDate {
                year: rug_fuzz_12,
                month: rug_fuzz_13,
                day: rug_fuzz_14,
            }),
            spread: None,
            repetition: None,
            tags: vec![],
        };
        let result = exp1.compare_dates(&exp2);
        debug_assert_eq!(result, std::cmp::Ordering::Less);
             }
});    }
    #[test]
    fn test_compare_dates_other_end_none() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14)) = <(u64, &str, i64, u64, u64, u64, u64, u64, u64, u64, &str, i64, u64, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let exp1 = Expense {
            id: rug_fuzz_0,
            description: rug_fuzz_1.to_string(),
            amount: rug_fuzz_2,
            start: SimpleDate {
                year: rug_fuzz_3,
                month: rug_fuzz_4,
                day: rug_fuzz_5,
            },
            end: Some(SimpleDate {
                year: rug_fuzz_6,
                month: rug_fuzz_7,
                day: rug_fuzz_8,
            }),
            spread: None,
            repetition: None,
            tags: vec![],
        };
        let exp2 = Expense {
            id: rug_fuzz_9,
            description: rug_fuzz_10.to_string(),
            amount: rug_fuzz_11,
            start: SimpleDate {
                year: rug_fuzz_12,
                month: rug_fuzz_13,
                day: rug_fuzz_14,
            },
            end: None,
            spread: None,
            repetition: None,
            tags: vec![],
        };
        let result = exp1.compare_dates(&exp2);
        debug_assert_eq!(result, std::cmp::Ordering::Greater);
             }
});    }
    #[test]
    fn test_compare_dates_self_greater() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17)) = <(u64, &str, i64, u64, u64, u64, u64, u64, u64, u64, &str, i64, u64, u64, u64, u64, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let exp1 = Expense {
            id: rug_fuzz_0,
            description: rug_fuzz_1.to_string(),
            amount: rug_fuzz_2,
            start: SimpleDate {
                year: rug_fuzz_3,
                month: rug_fuzz_4,
                day: rug_fuzz_5,
            },
            end: Some(SimpleDate {
                year: rug_fuzz_6,
                month: rug_fuzz_7,
                day: rug_fuzz_8,
            }),
            spread: None,
            repetition: None,
            tags: vec![],
        };
        let exp2 = Expense {
            id: rug_fuzz_9,
            description: rug_fuzz_10.to_string(),
            amount: rug_fuzz_11,
            start: SimpleDate {
                year: rug_fuzz_12,
                month: rug_fuzz_13,
                day: rug_fuzz_14,
            },
            end: Some(SimpleDate {
                year: rug_fuzz_15,
                month: rug_fuzz_16,
                day: rug_fuzz_17,
            }),
            spread: None,
            repetition: None,
            tags: vec![],
        };
        let result = exp1.compare_dates(&exp2);
        debug_assert_eq!(result, std::cmp::Ordering::Greater);
             }
});    }
    #[test]
    fn test_compare_dates_self_lesser() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17)) = <(u64, &str, i64, u64, u64, u64, u64, u64, u64, u64, &str, i64, u64, u64, u64, u64, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let exp1 = Expense {
            id: rug_fuzz_0,
            description: rug_fuzz_1.to_string(),
            amount: rug_fuzz_2,
            start: SimpleDate {
                year: rug_fuzz_3,
                month: rug_fuzz_4,
                day: rug_fuzz_5,
            },
            end: Some(SimpleDate {
                year: rug_fuzz_6,
                month: rug_fuzz_7,
                day: rug_fuzz_8,
            }),
            spread: None,
            repetition: None,
            tags: vec![],
        };
        let exp2 = Expense {
            id: rug_fuzz_9,
            description: rug_fuzz_10.to_string(),
            amount: rug_fuzz_11,
            start: SimpleDate {
                year: rug_fuzz_12,
                month: rug_fuzz_13,
                day: rug_fuzz_14,
            },
            end: Some(SimpleDate {
                year: rug_fuzz_15,
                month: rug_fuzz_16,
                day: rug_fuzz_17,
            }),
            spread: None,
            repetition: None,
            tags: vec![],
        };
        let result = exp1.compare_dates(&exp2);
        debug_assert_eq!(result, std::cmp::Ordering::Less);
             }
});    }
    #[test]
    fn test_compare_dates_equal() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17)) = <(u64, &str, i64, u64, u64, u64, u64, u64, u64, u64, &str, i64, u64, u64, u64, u64, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let exp1 = Expense {
            id: rug_fuzz_0,
            description: rug_fuzz_1.to_string(),
            amount: rug_fuzz_2,
            start: SimpleDate {
                year: rug_fuzz_3,
                month: rug_fuzz_4,
                day: rug_fuzz_5,
            },
            end: Some(SimpleDate {
                year: rug_fuzz_6,
                month: rug_fuzz_7,
                day: rug_fuzz_8,
            }),
            spread: None,
            repetition: None,
            tags: vec![],
        };
        let exp2 = Expense {
            id: rug_fuzz_9,
            description: rug_fuzz_10.to_string(),
            amount: rug_fuzz_11,
            start: SimpleDate {
                year: rug_fuzz_12,
                month: rug_fuzz_13,
                day: rug_fuzz_14,
            },
            end: Some(SimpleDate {
                year: rug_fuzz_15,
                month: rug_fuzz_16,
                day: rug_fuzz_17,
            }),
            spread: None,
            repetition: None,
            tags: vec![],
        };
        let result = exp1.compare_dates(&exp2);
        debug_assert_eq!(result, std::cmp::Ordering::Equal);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_58 {
    use crate::expense::{Expense, SimpleDate};
    #[test]
    fn test_compare_id() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(u64, &str, i64, u64, u64, u64, bool, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let expense = Expense {
            id: rug_fuzz_0,
            description: String::from(rug_fuzz_1),
            amount: rug_fuzz_2,
            start: SimpleDate::from_ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5),
            end: None,
            spread: None,
            repetition: None,
            tags: Vec::new(),
        };
        debug_assert_eq!(rug_fuzz_6, expense.compare_id(1));
        debug_assert_eq!(rug_fuzz_7, expense.compare_id(2));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_71 {
    use super::*;
    use crate::*;
    #[test]
    fn test_remove_tags() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(u64, &str, i64, u64, u64, u64, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut expense = Expense {
            id: rug_fuzz_0,
            description: rug_fuzz_1.to_string(),
            amount: rug_fuzz_2,
            start: SimpleDate::from_ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5),
            end: None,
            spread: None,
            repetition: None,
            tags: vec![rug_fuzz_6.to_string(), "tag2".to_string()],
        };
        expense.remove_tags(rug_fuzz_7);
        debug_assert_eq!(expense.tags, vec!["tag2".to_string()]);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_72 {
    use super::*;
    use crate::*;
    use std::collections::HashSet;
    #[test]
    fn test_tags() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(u64, &str, i64, u64, u64, u64, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let expense = Expense {
            id: rug_fuzz_0,
            description: rug_fuzz_1.to_string(),
            amount: rug_fuzz_2,
            start: SimpleDate::from_ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5),
            end: None,
            spread: None,
            repetition: None,
            tags: vec![rug_fuzz_6.to_string(), "tag2".to_string()],
        };
        let expected_tags = vec![rug_fuzz_7.to_string(), "tag2".to_string()];
        debug_assert_eq!(expense.tags(), & expected_tags);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_73 {
    use crate::expense::{calculate_spread, Expense, Duration, SimpleDate};
    #[test]
    fn test_calculate_spread() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10)) = <(u64, &str, i64, u64, u64, u64, u64, u64, u64, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let expenses = vec![
            Expense { id : rug_fuzz_0, description : rug_fuzz_1.to_string(), amount :
            rug_fuzz_2, start : SimpleDate { year : rug_fuzz_3, month : rug_fuzz_4, day :
            rug_fuzz_5, }, end : None, spread : Some(Duration::Day(rug_fuzz_6)),
            repetition : None, tags : vec![], }, Expense { id : 2, description :
            "Expense 2".to_string(), amount : 200, start : SimpleDate { year : 2021,
            month : 1, day : 1, }, end : None, spread : None, repetition : None, tags :
            vec![], }
        ];
        let start = SimpleDate {
            year: rug_fuzz_7,
            month: rug_fuzz_8,
            day: rug_fuzz_9,
        };
        let period = Duration::Day(rug_fuzz_10);
        debug_assert_eq!(calculate_spread(& expenses, & start, & period), 0.0);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_74 {
    use crate::expense::count_overlap_days;
    use crate::date::SimpleDate;
    #[test]
    fn test_count_overlap_days() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let period_start = SimpleDate {
            year: rug_fuzz_0,
            month: rug_fuzz_1,
            day: rug_fuzz_2,
        };
        let period_end = SimpleDate {
            year: rug_fuzz_3,
            month: rug_fuzz_4,
            day: rug_fuzz_5,
        };
        let expense_start = SimpleDate {
            year: rug_fuzz_6,
            month: rug_fuzz_7,
            day: rug_fuzz_8,
        };
        let expense_end = SimpleDate {
            year: rug_fuzz_9,
            month: rug_fuzz_10,
            day: rug_fuzz_11,
        };
        debug_assert_eq!(
            count_overlap_days(& period_start, & period_end, & expense_start, &
            expense_end), 11
        );
             }
});    }
}
#[cfg(test)]
mod tests_rug_19 {
    use super::*;
    use crate::date::{SimpleDate, Duration, Repetition};
    use crate::expense::Expense;
    #[test]
    fn test_new() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(u64, &str, i64, u64, u64, u64, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: u64 = rug_fuzz_0;
        let p1: String = rug_fuzz_1.to_string();
        let p2: i64 = rug_fuzz_2;
        let p3: SimpleDate = SimpleDate::from_ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let p4: Option<Duration> = None;
        let p5: Option<Repetition> = None;
        let p6: Vec<String> = vec![rug_fuzz_6.to_string(), "tag2".to_string()];
        Expense::new(p0, p1, p2, p3, p4, p5, p6);
             }
});    }
}
#[cfg(test)]
mod tests_rug_20 {
    use super::*;
    use std::io::StdinLock;
    use std::io::Write;
    use std::collections::HashSet;
    use crate::expense::{Expense, ExpenseError, Repetition, Duration};
    #[test]
    fn test_from_stdin() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u64, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let stdin = std::io::stdin();
        let mut handle = stdin.lock();
        let handle_ref: &mut StdinLock<'_> = &mut handle;
        let id: u64 = rug_fuzz_0;
        let is_income: bool = rug_fuzz_1;
        let allowed_tags: HashSet<String> = HashSet::new();
        let result = Expense::from_stdin(handle_ref, id, is_income, &allowed_tags);
        debug_assert!(result.is_ok());
             }
});    }
}
#[cfg(test)]
mod tests_rug_21 {
    use super::*;
    use crate::expense::Expense;
    use crate::expense::SimpleDate;
    use crate::expense::Duration;
    use crate::expense::Repetition;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u64, u64, u64, u64, &str, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let start = SimpleDate {
            year: rug_fuzz_0,
            month: rug_fuzz_1,
            day: rug_fuzz_2,
        };
        let p0 = Expense {
            id: rug_fuzz_3,
            description: String::from(rug_fuzz_4),
            amount: rug_fuzz_5,
            start,
            end: None,
            spread: None,
            repetition: None,
            tags: Vec::new(),
        };
        debug_assert_eq!(p0.amount(), 1000);
             }
});    }
}
#[cfg(test)]
mod tests_rug_25 {
    use super::*;
    use crate::date::{SimpleDate, Repetition, Duration};
    use crate::expense::{Expense, RepEnd};
    #[test]
    fn test_end_date() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u64, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = SimpleDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let p1: Option<Repetition> = None;
        let p2: Option<Duration> = None;
        Expense::end_date(&p0, &p1, &p2);
             }
});    }
}
