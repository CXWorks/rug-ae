
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
fn rusty_test_4981() {
    rusty_monitor::set_test_id(4981);
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut vec_1: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_0: u64 = 2998u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_0, on: vec_1};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Week(weekdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_1: u64 = 179u64;
    let mut u64_2: u64 = 2517u64;
    let mut u64_3: u64 = 6343u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_3, month: u64_2, day: u64_1};
    let mut i64_0: i64 = -12564i64;
    let mut str_0: &str = "NCLzoZ0lROWDyxB";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_4: u64 = 6777u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_4, string_0, i64_0, simpledate_0, option_1, option_0, vec_0);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_5: u64 = 5444u64;
    let mut u64_6: u64 = 1059u64;
    let mut u64_7: u64 = 9062u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_7, month: u64_6, day: u64_5};
    let mut i64_1: i64 = -9365i64;
    let mut u64_8: u64 = 6419u64;
    let mut option_4: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_9: u64 = 6468u64;
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_9);
    let mut option_5: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut option_6: std::option::Option<crate::date::SimpleDate> = std::option::Option::None;
    let mut u64_10: u64 = 6387u64;
    let mut u64_11: u64 = 3767u64;
    let mut u64_12: u64 = 5110u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_12, month: u64_11, day: u64_10};
    let mut i64_2: i64 = -7602i64;
    let mut u64_13: u64 = 6175u64;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4997() {
    rusty_monitor::set_test_id(4997);
    let mut u64_0: u64 = 5578u64;
    let mut u64_1: u64 = 3337u64;
    let mut u64_2: u64 = 9373u64;
    let mut u64_3: u64 = 8613u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_3, u64_2, u64_1);
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut u64_4: u64 = 2103u64;
    let mut u64_5: u64 = 6623u64;
    let mut u64_6: u64 = 3033u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_6, u64_5, u64_4);
    let mut i64_0: i64 = -21829i64;
    let mut u64_7: u64 = 607u64;
    let mut bool_0: bool = true;
    let mut u64_8: u64 = 9667u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut u64_9: u64 = 6149u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_9};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_10: u64 = 3165u64;
    let mut u64_11: u64 = 2979u64;
    let mut u64_12: u64 = 7315u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_12, u64_11, u64_10);
    let mut i64_1: i64 = -1897i64;
    let mut u64_13: u64 = 9239u64;
    let mut u64_14: u64 = 5191u64;
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Count(u64_14);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Saturday;
    let mut u64_15: u64 = 4451u64;
    let mut u64_16: u64 = 8746u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_16, weekid: u64_15, day: weekday_0};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut repetition_1: crate::date::Repetition = crate::date::Repetition {delta: repdelta_1, end: repend_1};
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_1);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_17: u64 = 4970u64;
    let mut u64_18: u64 = 4947u64;
    let mut u64_19: u64 = 279u64;
    let mut simpledate_3: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_19, month: u64_18, day: u64_17};
    let mut option_4: std::option::Option<crate::date::SimpleDate> = std::option::Option::Some(simpledate_3);
    let mut u64_20: u64 = 7413u64;
    let mut u64_21: u64 = 4251u64;
    let mut u64_22: u64 = 3189u64;
    let mut simpledate_4: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_22, month: u64_21, day: u64_20};
    let mut i64_2: i64 = -7442i64;
    let mut u64_23: u64 = 1171u64;
    let mut str_0: &str = "aYtLROTqSrq";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_24: u64 = 4553u64;
    let mut u64_25: u64 = 4311u64;
    let mut u64_26: u64 = 3337u64;
    let mut simpledate_5: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_26, u64_25, u64_24);
    let mut repend_2: date::RepEnd = crate::date::RepEnd::Date(simpledate_5);
    let mut vec_1: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_27: u64 = 6603u64;
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_27, days: vec_1};
    let mut monthdelta_1: date::MonthDelta = crate::date::MonthDelta::OnDate(monthdeltadate_0);
    let mut repdelta_2: date::RepDelta = crate::date::RepDelta::Month(monthdelta_1);
    let mut repetition_2: crate::date::Repetition = crate::date::Repetition {delta: repdelta_2, end: repend_2};
    let mut option_5: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_2);
    let mut option_6: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_28: u64 = 8130u64;
    let mut u64_29: u64 = 6232u64;
    let mut u64_30: u64 = 2303u64;
    let mut simpledate_6: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_30, u64_29, u64_28);
    let mut i64_3: i64 = 6353i64;
    let mut str_1: &str = "d1ChCn5uB7SH5";
    let mut string_0: std::string::String = std::string::String::from(str_1);
    let mut u64_31: u64 = 4516u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_31, string_0, i64_3, simpledate_6, option_6, option_5, vec_0);
    let mut expense_0_ref_0: &mut crate::expense::Expense = &mut expense_0;
    let mut vec_2: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_32: u64 = 9835u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_32, on: vec_2};
    let mut repdelta_3: date::RepDelta = crate::date::RepDelta::Week(weekdelta_0);
    let mut weekday_1: date::Weekday = crate::date::Weekday::Thursday;
    let mut weekday_2: date::Weekday = crate::date::Weekday::Thursday;
    crate::expense::Expense::remove_tags(expense_0_ref_0, str_0_ref_0);
    panic!("From RustyUnit with love");
}
}