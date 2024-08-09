
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

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_235() {
//    rusty_monitor::set_test_id(235);
    let mut str_0: &str = "weeks";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_0: u64 = 2718u64;
    let mut u64_1: u64 = 23u64;
    let mut u64_2: u64 = 28u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_2, month: u64_1, day: u64_0};
    let mut i64_0: i64 = 9223372036854775807i64;
    let mut str_1: &str = "OnWeek";
    let mut string_0: std::string::String = std::string::String::from(str_1);
    let mut u64_3: u64 = 5638u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_3, string_0, i64_0, simpledate_0, option_1, option_0, vec_0);
    let mut expense_0_ref_0: &mut crate::expense::Expense = &mut expense_0;
    let mut u64_4: u64 = 21u64;
    let mut vec_1: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_5: u64 = 273u64;
    let mut u64_6: u64 = 6232u64;
    let mut u64_7: u64 = 2738u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_7, month: u64_6, day: u64_5};
    let mut i64_1: i64 = -8642i64;
    let mut str_2: &str = "couldn't find item";
    let mut string_1: std::string::String = std::string::String::from(str_2);
    let mut u64_8: u64 = 6u64;
    let mut expense_1: crate::expense::Expense = crate::expense::Expense::new(u64_8, string_1, i64_1, simpledate_1, option_3, option_2, vec_1);
    let mut expense_1_ref_0: &crate::expense::Expense = &mut expense_1;
    let mut u64_9: u64 = 11u64;
    let mut duration_0: date::Duration = crate::date::Duration::Year(u64_9);
    let mut str_3: &str = crate::expense::Expense::description(expense_1_ref_0);
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_4};
    let mut weekday_0: date::Weekday = crate::date::Weekday::Monday;
    crate::expense::Expense::remove_tags(expense_0_ref_0, str_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_734() {
//    rusty_monitor::set_test_id(734);
    let mut u64_0: u64 = 8u64;
    let mut u64_1: u64 = 1700u64;
    let mut u64_2: u64 = 5u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_2, u64_1, u64_0);
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_3: u64 = 151u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_3);
    let mut vec_1: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_4: u64 = 365u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_4, on: vec_1};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Week(weekdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_5: u64 = 4u64;
    let mut u64_6: u64 = 9u64;
    let mut u64_7: u64 = 2569u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_7, u64_6, u64_5);
    let mut i64_0: i64 = 3745i64;
    let mut str_0: &str = "Count";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_8: u64 = 23u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_8, string_0, i64_0, simpledate_1, option_1, option_0, vec_0);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Never;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Tuesday;
    let mut u64_9: u64 = 3873u64;
    let mut u64_10: u64 = 5226u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_10, weekid: u64_9, day: weekday_0};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_0);
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut repetition_1: crate::date::Repetition = crate::date::Repetition {delta: repdelta_1, end: repend_1};
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_1);
    let mut u64_11: u64 = 1u64;
    let mut duration_0: date::Duration = crate::date::Duration::Year(u64_11);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_12: u64 = 243u64;
    let mut u64_13: u64 = 0u64;
    let mut u64_14: u64 = 1700u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_14, month: u64_13, day: u64_12};
    let mut u64_15: u64 = 725u64;
    let mut duration_1: date::Duration = crate::date::Duration::Week(u64_15);
    let mut weekday_1: date::Weekday = crate::date::Weekday::Wednesday;
    let mut u64_16: u64 = 8098u64;
    let mut u64_17: u64 = 4u64;
    let mut monthdeltaweek_1: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_17, weekid: u64_16, day: weekday_1};
    let mut monthdelta_1: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_1);
    let mut u64_18: u64 = 1684u64;
    let mut u64_19: u64 = 5u64;
    let mut u64_20: u64 = 212u64;
    let mut simpledate_3: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_20, month: u64_19, day: u64_18};
    let mut simpledate_3_ref_0: &crate::date::SimpleDate = &mut simpledate_3;
    let mut u64_21: u64 = 9996u64;
    let mut u64_22: u64 = 30u64;
    let mut u64_23: u64 = 28u64;
    let mut simpledate_4: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_23, month: u64_22, day: u64_21};
    let mut simpledate_4_ref_0: &crate::date::SimpleDate = &mut simpledate_4;
    let mut u64_24: u64 = 100u64;
    let mut repend_2: date::RepEnd = crate::date::RepEnd::Count(u64_24);
    let mut str_1: &str = crate::expense::Expense::description(expense_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6486() {
//    rusty_monitor::set_test_id(6486);
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_0: u64 = 11u64;
    let mut u64_1: u64 = 151u64;
    let mut u64_2: u64 = 12u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_2, u64_1, u64_0);
    let mut i64_0: i64 = 100i64;
    let mut str_0: &str = "this is an income, not an expense";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_3: u64 = 3u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_3, string_0, i64_0, simpledate_0, option_1, option_0, vec_0);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut u64_4: u64 = 9522u64;
    let mut vec_1: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_5: u64 = 4945u64;
    let mut vec_2: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_6: u64 = 6761u64;
    let mut u64_7: u64 = 5311u64;
    let mut u64_8: u64 = 59u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_8, u64_7, u64_6);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_1);
    let mut u64_9: u64 = 304u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_9};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_10: u64 = 8u64;
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_10);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_11: u64 = 59u64;
    let mut u64_12: u64 = 400u64;
    let mut u64_13: u64 = 6u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_13, month: u64_12, day: u64_11};
    let mut i64_1: i64 = 9223372036854775807i64;
    let mut str_1: &str = r"\d+";
    let mut string_1: std::string::String = std::string::String::from(str_1);
    let mut u64_14: u64 = 6294u64;
    let mut u64_15: u64 = 12u64;
    let mut duration_1: date::Duration = crate::date::Duration::Day(u64_15);
    let mut expense_1: crate::expense::Expense = crate::expense::Expense::new(u64_14, string_1, i64_1, simpledate_2, option_3, option_2, vec_2);
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_5, days: vec_1};
    let mut weekday_0: date::Weekday = crate::date::Weekday::Saturday;
    let mut duration_2: date::Duration = crate::date::Duration::Week(u64_4);
    let mut i64_2: i64 = crate::expense::Expense::amount(expense_0_ref_0);
//    panic!("From RustyUnit with love");
}
}