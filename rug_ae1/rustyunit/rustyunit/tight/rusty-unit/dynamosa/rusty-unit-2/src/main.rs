
#![feature(no_coverage)]
pub mod rusty_monitor;
pub use ntest::timeout;
pub mod data;
pub mod date;
pub mod expense;

use chrono::Datelike;
use clap::{App, AppSettings, Arg, SubCommand};
use colored::*;
use std::convert::TryInto;

const DATA_LOCATION: &str = ".tight/tight.json";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("A simple expense tracker")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name("init")
                    .about("initialise an empty data file in the current directory"))
        .subcommand(SubCommand::with_name("add")
                    .about("add an income or expense")
                    .setting(AppSettings::SubcommandRequiredElseHelp)
                    .subcommand(SubCommand::with_name("tag")
                                .about("add a new tag")
                                .arg(Arg::with_name("name")
                                     .required(true)
                                     .help("the tag to add")))
                    .subcommand(SubCommand::with_name("expense")
                                .about("add an expense"))
                    .subcommand(SubCommand::with_name("income")
                                .about("add an income")))
        .subcommand(SubCommand::with_name("rm")
                    .about("remove an income or expense")
                    .setting(AppSettings::SubcommandRequiredElseHelp)
                    .subcommand(SubCommand::with_name("tag")
                                .about("remove a tag")
                                .arg(Arg::with_name("name")
                                     .required(true)
                                     .help("the name of the tag to remove")))
                    .subcommand(SubCommand::with_name("expense")
                                .about("remove an expense")
                                .arg(Arg::with_name("id")
                                     .required(true)
                                     .help("the ID of the expense to remove")))
                    .subcommand(SubCommand::with_name("income")
                                .about("remove an income")
                                .arg(Arg::with_name("id")
                                     .required(true)
                                     .help("the ID of the income to remove"))))
        .subcommand(SubCommand::with_name("grep")
                    .about("show expenses/incomes matching a regular expression")
                    .arg(Arg::with_name("regex")
                         .required(true)
                         .help("look for expenses and incomes with descriptions or tags matching this regular expression")))
        .subcommand(SubCommand::with_name("show")
                    .about("show an expense or income with a given ID")
                    .arg(Arg::with_name("id")
                         .required(true)
                         .help("the ID of the expense or income to show")))
        .subcommand(SubCommand::with_name("status")
                    .about("show position for day, week, month and year"))
        .subcommand(SubCommand::with_name("report")
                    .about("run a custom report. reserved for future usage"))
        .subcommand(SubCommand::with_name("verify")
                    .about("verify the integrity of the tight data file"))
        .subcommand(SubCommand::with_name("repair")
                    .about("attempt to repair the tight data file"))
        .subcommand(SubCommand::with_name("migrate")
                    .about("migrate the tight data file to the latest schema"))
        .get_matches();

    match matches.subcommand() {
        ("init", Some(_)) => {
            data::initialise(DATA_LOCATION)?;
        },

        ("add", Some(add_matches)) => {
            let mut data = data::Datafile::from_file(DATA_LOCATION)?;

            match add_matches.subcommand() {
                ("tag", Some(tag_matches)) => {
                    data.add_tag(tag_matches.value_of("name").unwrap().to_string());
                },

                ("expense", Some(_)) => {
                    let stdin = std::io::stdin();
                    let mut handle = stdin.lock();
                    let expense = expense::Expense::from_stdin(&mut handle, data.entries.len() as u64, false, &data.tags)?;
                    println!("added {}", expense);
                    data.insert(expense);
                },

                ("income", Some(_)) => {
                    let stdin = std::io::stdin();
                    let mut handle = stdin.lock();
                    let expense = expense::Expense::from_stdin(&mut handle, data.entries.len() as u64, true, &data.tags)?;
                    println!("added {}", expense);
                    data.insert(expense);
                },

                _ => unreachable!(),
            };

            data.save(DATA_LOCATION)?;
        },

        ("rm", Some(rm_matches)) => {
            let mut data = data::Datafile::from_file(DATA_LOCATION)?;

            match rm_matches.subcommand() {
                ("tag", Some(tag_matches)) => {
                    let name = tag_matches.value_of("name").unwrap();
                    for expense in &mut data.entries {
                        expense.remove_tags(name);
                    }

                    data.tags.retain(|tag| tag != name);
                },

                ("expense", Some(expense_matches)) => {
                    let id = expense_matches.value_of("id").unwrap().parse()?;
                    if let Some(expense) = data.find(id) {
                        if expense.amount() < 0 {
                            println!("removing {}", expense);
                            data.remove(id)?;
                        } else {
                            return Err("this is an income, not an expense".into());
                        }
                    } else {
                        return Err("couldn't find anything with that id".into());
                    }
                },

                ("income", Some(income_matches)) => {
                    let id = income_matches.value_of("id").unwrap().parse()?;
                    if let Some(income) = data.find(id) {
                        if income.amount() > 0 {
                            println!("removing {}", income);
                            data.remove(id)?;
                        } else {
                            return Err("this is an expense, not an income".into());
                        }
                    } else {
                        return Err("couldn't find anything with that id".into());
                    }
                },

                _ => unreachable!(),
            }

            data.save(DATA_LOCATION)?;
        },

        ("grep", Some(grep_matches)) => {
            let data = data::Datafile::from_file(DATA_LOCATION)?;
            let re = regex::Regex::new(grep_matches.value_of("regex").unwrap())?;
            for expense in data.entries {
                let mut print = false;
                for tag in expense.tags() {
                    print |= re.is_match(tag);
                }
                print |= re.is_match(expense.description());

                if print {
                    println!("{}", expense);
                }
            }
        },

        ("show", Some(show_matches)) => {
            let data = data::Datafile::from_file(DATA_LOCATION)?;
            let id = show_matches.value_of("id").unwrap().parse()?;
            if let Some(expense) = data.find(id) {
                println!("{}", expense);
            } else {
                return Err("couldn't find anything with that id".into());
            }
        }

        ("status", Some(_)) => {
            let data = data::Datafile::from_file(DATA_LOCATION)?;

            let now = chrono::Local::now();
            let year = now.year().try_into()?;
            let month = now.month().into();
            let day = now.day().into();
            let today = date::SimpleDate::from_ymd(year, month, day);

            let day_start = &today - &date::Duration::Day(1);
            let week_start = &today - &date::Duration::Week(1);
            let month_start = &today - &date::Duration::Month(1);
            let year_start = &today - &date::Duration::Year(1);

            let day_expenses = data.expenses_between(&day_start, &today);
            let week_expenses = data.expenses_between(&week_start, &today);
            let month_expenses = data.expenses_between(&month_start, &today);
            let year_expenses = data.expenses_between(&year_start, &today);

            let day_total: f64 = expense::calculate_spread(&day_expenses, &day_start, &date::Duration::Day(1));
            let week_total: f64 = expense::calculate_spread(&week_expenses, &week_start, &date::Duration::Week(1));
            let month_total: f64 = expense::calculate_spread(&month_expenses, &month_start, &date::Duration::Month(1));
            let year_total: f64 = expense::calculate_spread(&year_expenses, &year_start, &date::Duration::Year(1));

            let day_formatted = format!("{:.2}", day_total.abs());
            let week_formatted = format!("{:.2}", week_total.abs());
            let month_formatted = format!("{:.2}", month_total.abs());
            let year_formatted = format!("{:.2}", year_total.abs());

            print!("day: ");
            if day_total < 0.0 {
                println!("{}{}", "-$".red(), day_formatted.red());
            } else {
                println!("{}{}", "$".green(), day_formatted.green());
            }

            print!("week: ");
            if week_total < 0.0 {
                println!("{}{}", "-$".red(), week_formatted.red());
            } else {
                println!("{}{}", "$".green(), week_formatted.green());
            }

            print!("month: ");
            if month_total < 0.0 {
                println!("{}{}", "-$".red(), month_formatted.red());
            } else {
                println!("{}{}", "$".green(), month_formatted.green());
            }

            print!("year: ");
            if year_total < 0.0 {
                println!("{}{}", "-$".red(), year_formatted.red());
            } else {
                println!("{}{}", "$".green(), year_formatted.green());
            }
        }

        ("verify", Some(_)) => (),
        ("repair", Some(_)) => (),
        ("report", Some(_)) => (),
        ("migrate", Some(_)) => (),

        _ => unreachable!(),
    }

    Ok(())
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_137() {
    rusty_monitor::set_test_id(137);
    let mut u64_0: u64 = 8195u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_0);
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_1: u64 = 3906u64;
    let mut u64_2: u64 = 8406u64;
    let mut u64_3: u64 = 2817u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_3, u64_2, u64_1);
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Sunday;
    let mut u64_4: u64 = 3641u64;
    let mut u64_5: u64 = 1414u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_5, weekid: u64_4, day: weekday_0};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_1};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_6: u64 = 2701u64;
    let mut u64_7: u64 = 1521u64;
    let mut u64_8: u64 = 8887u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_8, u64_7, u64_6);
    let mut i64_0: i64 = 10338i64;
    let mut str_0: &str = "lXRUo";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_9: u64 = 8798u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_9, string_0, i64_0, simpledate_1, option_1, option_0, vec_0);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut weekday_1: date::Weekday = crate::date::Weekday::Sunday;
    let mut u64_10: u64 = 9564u64;
    let mut u64_11: u64 = 6439u64;
    let mut monthdeltaweek_1: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_11, weekid: u64_10, day: weekday_1};
    let mut monthdelta_1: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_1);
    let mut weekday_2: date::Weekday = crate::date::Weekday::Sunday;
    let mut u64_12: u64 = 8548u64;
    let mut u64_13: u64 = 4197u64;
    let mut monthdeltaweek_2: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_13, weekid: u64_12, day: weekday_2};
    let mut u64_14: u64 = 2485u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_14};
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut option_2: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_15: u64 = 1481u64;
    let mut u64_16: u64 = 4942u64;
    let mut u64_17: u64 = 5506u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_17, month: u64_16, day: u64_15};
    let mut u64_18: u64 = 4554u64;
    let mut u64_19: u64 = 8823u64;
    let mut u64_20: u64 = 4296u64;
    let mut u64_21: u64 = 2777u64;
    let mut simpledate_3: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_21, month: u64_20, day: u64_19};
    let mut repend_2: date::RepEnd = crate::date::RepEnd::Date(simpledate_3);
    let mut weekday_3: date::Weekday = crate::date::Weekday::Sunday;
    let mut u64_22: u64 = 841u64;
    let mut u64_23: u64 = 9783u64;
    let mut monthdeltaweek_3: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_23, weekid: u64_22, day: weekday_3};
    let mut monthdelta_2: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_3);
    let mut repdelta_2: date::RepDelta = crate::date::RepDelta::Month(monthdelta_2);
    let mut repetition_1: crate::date::Repetition = crate::date::Repetition {delta: repdelta_2, end: repend_2};
    let mut option_3: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_1);
    let mut u64_24: u64 = 9068u64;
    let mut duration_0: date::Duration = crate::date::Duration::Week(u64_24);
    let mut option_4: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut weekday_4: date::Weekday = crate::date::Weekday::Thursday;
    let mut u64_25: u64 = 781u64;
    let mut monthdeltaweek_4: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_25, weekid: u64_18, day: weekday_4};
    let mut monthdelta_3: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_2);
    let mut repdelta_3: date::RepDelta = crate::date::RepDelta::Month(monthdelta_1);
    let mut i64_1: i64 = crate::expense::Expense::amount(expense_0_ref_0);
    let mut repetition_2: crate::date::Repetition = crate::date::Repetition {delta: repdelta_1, end: repend_0};
    panic!("From RustyUnit with love");
}
}