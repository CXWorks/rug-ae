use chrono::Datelike;

use serde::{Serialize, Deserialize};

use std::convert::TryInto;
use std::error::Error;
use std::fmt;
use std::io::BufRead;
use std::io::Write;

#[derive(Debug)]
struct DateError(String);

#[derive(Serialize, Deserialize, Debug)]
pub enum Duration {
    Day(u64),
    Week(u64),
    Month(u64),
    Year(u64),
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct SimpleDate {
    pub year: u64,
    pub month: u64,
    pub day: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DayDelta {
    pub nth: u64,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeekDelta {
    pub nth: u64,
    pub on: Vec<Weekday>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MonthDeltaDate {
    pub nth:  u64,
    pub days: Vec<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MonthDeltaWeek {
    pub nth: u64,
    pub weekid: u64,
    pub day: Weekday,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MonthDelta {
    OnDate(MonthDeltaDate),
    OnWeek(MonthDeltaWeek),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct YearDelta {
    pub nth: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RepDelta {
    Day(DayDelta),
    Week(WeekDelta),
    Month(MonthDelta),
    Year(YearDelta),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RepEnd {
    Never,
    Date(SimpleDate),
    Count(u64),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Repetition {
    pub delta: RepDelta,
    pub end: RepEnd,
}

impl SimpleDate {
    pub fn from_ymd(year: u64, month: u64, day: u64) -> SimpleDate {
        SimpleDate {
            year,
            month,
            day,
        }
    }

    pub fn from_stdin(handle: &mut std::io::StdinLock) -> Result<SimpleDate, Box<dyn Error>> {
        print!("start date (yyyy-mm-dd, blank for today): ");
        std::io::stdout().flush()?;
        let mut date = String::new();
        handle.read_line(&mut date)?;

        let year:  u64;
        let month: u64;
        let day  : u64;
        if date.trim().is_empty() {
            let now = chrono::Local::now();
            year = now.year().try_into()?;
            month = now.month().into();
            day = now.day().into();
        } else {
            let result = scan_fmt::scan_fmt!(&date, "{}-{}-{}", u64, u64, u64)?;
            year = result.0;
            month = result.1;
            day = result.2;

            if month > 12 {
                return Err(Box::new(DateError("invalid month".into())));
            }

            if day > days_in_month(year, month) {
                return Err(Box::new(DateError("invalid date".into())));
            }
        }

        Ok(SimpleDate {
            year,
            month,
            day,
        })
    }
}

fn days_in_month(year: u64, month: u64) -> u64 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if year % 400 == 0 {
                29
            } else if year % 100 == 0 {
                28
            } else if year % 4 == 0 {
                29
            } else {
                28
            }
        },
        _ => unreachable!(),
    }
}

fn get_weekday_of_date(date: &SimpleDate) -> Weekday {
    let offset = vec!(0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334);
    let days = vec!(Weekday::Monday,
                    Weekday::Tuesday,
                    Weekday::Wednesday,
                    Weekday::Thursday,
                    Weekday::Friday,
                    Weekday::Saturday,
                    Weekday::Sunday);

    let after_feb = if date.month > 2 { 0 } else { 1 };
    let aux = date.year - 1700 - after_feb;
    let day = ((4)  // day of week for 1700-01-01 = friday = 4
        + ((aux + after_feb) * 365) // partial sum of dats between current date and 1700-01-01
        + (aux / 4 - aux / 100 + (aux + 100) / 400) // leap year correction
        + (offset[(date.month as usize) - 1] + (date.day - 1))) // sum month and day offsets
        % 7;

    days[day as usize]
}

impl Ord for SimpleDate {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        if self.year != rhs.year {
            self.year.cmp(&rhs.year)
        } else if self.month != rhs.month {
            self.month.cmp(&rhs.month)
        } else if self.day != rhs.day {
            self.day.cmp(&rhs.day)
        } else {
            std::cmp::Ordering::Equal
        }
    }
}

impl PartialOrd for SimpleDate {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(rhs))
    }
}

impl std::ops::Add<&Duration> for &SimpleDate {
    type Output = SimpleDate;

    fn add(self, rhs: &Duration) -> Self::Output {
        let mut year = self.year;
        let mut month = self.month;
        let mut day = self.day;

        match rhs {
            Duration::Day(d) => day += d,
            Duration::Week(w) => day += w * 7,
            Duration::Month(m) => month += m,
            Duration::Year(y) => year += y,
        }

        loop {
            let mut extra_years = month / 12;
            let mut relative_month = month % 12;

            if relative_month == 0 {
                extra_years -= 1;
                relative_month += 12;
            }

            year += extra_years;
            month = relative_month;

            if day == self.day || day <= days_in_month(year, month) {
                break;
            } else {
                day -= days_in_month(year, month);
                month += 1;
            }
        }

        let clamped_day = days_in_month(year, month).min(day);

        SimpleDate::from_ymd(year, month, clamped_day)
    }
}

impl std::ops::Sub<&Duration> for &SimpleDate {
    type Output = SimpleDate;

    fn sub(self, rhs: &Duration) -> Self::Output {
        let mut year = self.year;
        let mut month = self.month;
        let mut day = self.day;

        let mut months_to_sub = 0;
        let mut days_to_sub = 0;
        match rhs {
            Duration::Day(d) => days_to_sub = *d,
            Duration::Week(w) => days_to_sub = w * 7,
            Duration::Month(m) => months_to_sub = *m,
            Duration::Year(y) => year -= y,
        }

        for _ in 0..days_to_sub {
            day -= 1;
            if day == 0 {
                month -= 1;
                if month == 0 {
                    year -= 1;
                    month = 12;
                }
                day = days_in_month(year, month);
            }
        }

        for _ in 0..months_to_sub {
            month -= 1;
            if month == 0 {
                 year -= 1;
                 month = 12;
             }
        }

        let clamped_day = days_in_month(year, month).min(day);

        SimpleDate::from_ymd(year, month, clamped_day)
    }
}

impl std::ops::Add<&RepDelta> for &SimpleDate {
    type Output = SimpleDate;

    fn add(self, rhs: &RepDelta) -> Self::Output {
        let mut end = SimpleDate::from_ymd(self.year, self.month, self.day);

        match rhs {
            RepDelta::Day(d) => {
                end = &end + &Duration::Day(d.nth);
            },

            RepDelta::Week(w) => {
                loop {
                    if &get_weekday_of_date(&end) == w.on.last().unwrap() {
                        break;
                    }

                    end = &end + &Duration::Day(1);
                }

                end = &end + &Duration::Week(w.nth);
            }

            RepDelta::Month(m) => {
                match m {
                    MonthDelta::OnDate(d) => {
                        let min_day = *d.days.iter().min().unwrap();
                        if end.day >= min_day {
                            end = &end + &Duration::Month(d.nth);
                        } else {
                            end = &end + &Duration::Month(d.nth - 1);
                        }

                        let max_day = *d.days.iter().max().unwrap();
                        end.day = max_day.min(days_in_month(end.year, end.month));
                    },

                    MonthDelta::OnWeek(w) => {
                        let mut current_iter = SimpleDate::from_ymd(end.year, end.month, 1);
                        loop {
                            if get_weekday_of_date(&current_iter) == w.day {
                                break;
                            }

                            current_iter = &current_iter + &Duration::Day(1);
                        }

                        current_iter = &current_iter + &Duration::Week(w.weekid - 1);

                        if end.day >= current_iter.day {
                            end = &end + &Duration::Month(w.nth);
                        } else {
                            end = &end + &Duration::Month(w.nth - 1);
                        }

                        end.day = 1;
                        loop {
                            if get_weekday_of_date(&end) == w.day {
                                break;
                            }

                            end = &end + &Duration::Day(1);
                        };
                        end = &end + &Duration::Week(w.weekid - 1);
                    },
                }
            }

            RepDelta::Year(y) => {
                end = &end + &Duration::Year(y.nth);
            }
        }

        end
    }
}

impl std::ops::Add<&Repetition> for &SimpleDate {
    type Output = SimpleDate;

    fn add(self, rhs: &Repetition) -> Self::Output {
        let mut end = SimpleDate::from_ymd(self.year, self.month, self.day);

        match rhs.end {
            RepEnd::Never => end = SimpleDate::from_ymd(9999, 12, 31),

            RepEnd::Count(c) => {
                for _ in 0..c {
                    end = &end + &rhs.delta;
                }
            },

            RepEnd::Date(d) => {
                let mut new;

                while end < d {
                    new = &end + &rhs.delta;
                    if new > d {
                        return end;
                    }

                    end = new;
                }
            },
        }

        end
    }
}

impl DayDelta {
    fn parse(s: &str) -> Result<DayDelta, Box<dyn Error>> {
        if let Ok(result) = scan_fmt::scan_fmt!(&s, "every {} days", u64) {
            Ok(DayDelta{ nth: result })
        } else if let Ok(result) = scan_fmt::scan_fmt!(&s, "every {} day", u64) {
            Ok(DayDelta{ nth: result })
        } else if let Ok(result) = scan_fmt::scan_fmt!(&s, "{} days", u64) {
            Ok(DayDelta{ nth: result })
        } else if let Ok(result) = scan_fmt::scan_fmt!(&s, "{} day", u64) {
            Ok(DayDelta{ nth: result })
        } else if s == "daily" || s == "every day" {
            Ok(DayDelta{ nth: 1 })
        } else {
            Err(Box::new(DateError("couldn't parse schedule".into())))
        }
    }
}

impl WeekDelta {
    fn parse(s: &str, start: &SimpleDate) -> Result<WeekDelta, Box<dyn Error>> {
        // extract optional "on [day list]"
        let (beginning, end) = if let Some(idx) = s.find(" on ") {
            (&s[..idx], Some(&s[idx..]))
        } else {
            (&s[..], None)
        };

        let day_list = if let Some(s) = end {
            WeekDelta::parse_days(s)?
        } else {
            vec!(get_weekday_of_date(start))
        };

        if let Ok(result) = scan_fmt::scan_fmt!(&beginning, "every {} weeks", u64) {
            Ok(WeekDelta{ nth: result, on: day_list, })
        } else if let Ok(result) = scan_fmt::scan_fmt!(&beginning, "every {} week", u64) {
            Ok(WeekDelta{ nth: result, on: day_list, })
        } else if let Ok(result) = scan_fmt::scan_fmt!(&beginning, "{} weeks", u64) {
            Ok(WeekDelta{ nth: result, on: day_list, })
        } else if let Ok(result) = scan_fmt::scan_fmt!(&beginning, "{} week", u64) {
            Ok(WeekDelta{ nth: result, on: day_list, })
        } else if beginning == "weekly" {
            Ok(WeekDelta{ nth: 1, on: day_list, })
        } else if beginning == "fortnightly" {
            Ok(WeekDelta{ nth: 2, on: day_list, })
        } else {
            Err(Box::new(DateError("couldn't parse schedule".into())))
        }
    }

    fn parse_days(s: &str) -> Result<Vec<Weekday>, Box<dyn Error>> {
        let mut days = vec!();
        if s.contains("mon") {
            days.push(Weekday::Monday);
        }
        if s.contains("tue") {
            days.push(Weekday::Tuesday);
        }
        if s.contains("wed") {
            days.push(Weekday::Wednesday);
        }
        if s.contains("thu") {
            days.push(Weekday::Thursday);
        }
        if s.contains("fri") {
            days.push(Weekday::Friday);
        }
        if s.contains("sat") {
            days.push(Weekday::Saturday);
        }
        if s.contains("sun") {
            days.push(Weekday::Sunday);
        }

        if days.is_empty() {
            return Err(Box::new(DateError("couldn't parse schedule".into())));
        }

        Ok(days)
    }
}

fn suffix_for_day(day: &u64) -> &'static str {
    match day {
        1 | 21 | 31 => "st",
        2 | 22      => "nd",
        3 | 23      => "rd",
        _           => "th",
    }
}

impl MonthDeltaWeek {
    fn weekid_to_str(&self) -> &str {
        match self.weekid {
            0 => "first",
            1 => "second",
            2 => "third",
            3 => "fourth",
            4 => "fifth",
            _ => unreachable!(),
        }
    }
}

impl MonthDelta {
    fn parse(s: &str, start: &SimpleDate) -> Result<MonthDelta, Box<dyn Error>> {
        // extract optional "on [day list]"
        let (beginning, end) = if let Some(idx) = s.find(" on ") {
            (&s[..idx], Some(&s[idx..]))
        } else {
            (&s[..], None)
        };

        let nth = if let Ok(result) = scan_fmt::scan_fmt!(&beginning, "every {} months", u64) {
            result
        } else if let Ok(result) = scan_fmt::scan_fmt!(&beginning, "every {} month", u64) {
            result
        } else if let Ok(result) = scan_fmt::scan_fmt!(&beginning, "{} months", u64) {
            result
        } else if let Ok(result) = scan_fmt::scan_fmt!(&beginning, "month", u64) {
            result
        } else if beginning == "monthly" {
            1
        } else if beginning == "quarterly" {
            3
        } else {
            return Err(Box::new(DateError("couldn't parse schedule".into())));
        };

        if end == None {
            return Ok(MonthDelta::OnDate(MonthDeltaDate{ nth, days: vec!(start.day) }));
        }

        if let Some(day) = MonthDelta::parse_weekday(end.unwrap()) {
            let weekid = if let Some(id) = MonthDelta::parse_nth(end.unwrap()) {
                id
            } else {
                return Err(Box::new(DateError("couldn't parse schedule".into())));
            };

            Ok(MonthDelta::OnWeek(MonthDeltaWeek{ nth, weekid, day }))
        } else {
            let re = regex::Regex::new(r"\d+").unwrap();
            let mut days: Vec<u64> = vec!();
            for m in re.find_iter(end.unwrap()) {
                if let Ok(day) = m.as_str().parse() {
                    if day >= 1 && day <= 31 {
                        days.push(day);
                    } else {
                        return Err(Box::new(DateError("couldn't parse schedule".into())));
                    }
                } else {
                    return Err(Box::new(DateError("couldn't parse schedule".into())));
                }
            }

            if days.is_empty() {
                Err(Box::new(DateError("couldn't parse schedule".into())))
            } else {
                Ok(MonthDelta::OnDate(MonthDeltaDate{ nth, days }))
            }
        }
    }

    fn parse_weekday(s: &str) -> Option<Weekday> {
        if s.contains("mon") {
            Some(Weekday::Monday)
        } else if s.contains("tue") {
            Some(Weekday::Tuesday)
        } else if s.contains("wed") {
            Some(Weekday::Wednesday)
        } else if s.contains("thu") {
            Some(Weekday::Thursday)
        } else if s.contains("fri") {
            Some(Weekday::Friday)
        } else if s.contains("sat") {
            Some(Weekday::Saturday)
        } else if s.contains("sun") {
            Some(Weekday::Sunday)
        } else {
            None
        }
    }

    fn parse_nth(s: &str) -> Option<u64> {
        if s.contains("first") || s.contains("1st") {
            Some(0)
        } else if s.contains("second") || s.contains("2nd") {
            Some(1)
        } else if s.contains("third") || s.contains("3rd") {
            Some(2)
        } else if s.contains("fourth") || s.contains("4th") {
            Some(3)
        } else {
            None
        }
    }
}

impl YearDelta {
    fn parse(s: &str) -> Result<YearDelta, Box<dyn Error>> {
        if let Ok(result) = scan_fmt::scan_fmt!(&s, "every {} years", u64) {
            Ok(YearDelta{ nth: result })
        } else if let Ok(result) = scan_fmt::scan_fmt!(&s, "every {} year", u64) {
            Ok(YearDelta{ nth: result })
        } else if let Ok(result) = scan_fmt::scan_fmt!(&s, "{} years", u64) {
            Ok(YearDelta{ nth: result })
        } else if let Ok(result) = scan_fmt::scan_fmt!(&s, "{} year", u64) {
            Ok(YearDelta{ nth: result })
        } else if s == "annually" || s == "yearly" || s == "every year" {
            Ok(YearDelta{ nth: 1 })
        } else {
            Err(Box::new(DateError("couldn't parse schedule".into())))
        }
    }
}

impl RepEnd {
    fn parse(s: &str) -> Result<RepEnd, Box<dyn Error>> {
        if s.trim().is_empty() || s.contains("never") {
            Ok(RepEnd::Never)
        } else if s.contains("after") || s.contains("times") || s.contains("occurrences") || s.contains("reps") {
            RepEnd::parse_count(s.trim())
        } else {
            RepEnd::parse_date(s.trim())
        }
    }

    fn parse_date(s: &str) -> Result<RepEnd, Box<dyn Error>> {
        let re = regex::Regex::new(r"(\d+)-(\d+)-(\d+)")?;

        let year: u64;
        let month: u64;
        let day: u64;
        if let Some(captures) = re.captures(s) {
            if captures.len() != 4 {
                return Err(Box::new(DateError("invalid date".into())));
            }

            year = captures.get(1).unwrap().as_str().parse()?;
            month = captures.get(2).unwrap().as_str().parse()?;
            day = captures.get(3).unwrap().as_str().parse()?;

            if month > 12 {
                return Err(Box::new(DateError("invalid date".into())));
            }

            if day > days_in_month(year, month) {
                return Err(Box::new(DateError("invalid date".into())));
            }
        } else {
            return Err(Box::new(DateError("invalid end date".into())));
        }

        Ok(RepEnd::Date(SimpleDate::from_ymd(year, month, day)))
    }

    fn parse_count(s: &str) -> Result<RepEnd, Box<dyn Error>> {
        let re = regex::Regex::new(r"(\d+)")?;

        let count: u64;
        if let Some(captures) = re.captures(s) {
            if captures.len() != 2 {
                return Err(Box::new(DateError("couldn't parse ending schedule".into())));
            }

            count = captures.get(1).unwrap().as_str().parse()?;
        } else {
            return Err(Box::new(DateError("couldn't parse ending schedule".into())));
        }

        Ok(RepEnd::Count(count))
    }
}

impl Repetition {
    pub fn from_stdin(handle: &mut std::io::StdinLock, start: &SimpleDate) -> Result<Option<Repetition>, Box<dyn Error>> {
        // parse schedule
        print!("repetition schedule (blank for none): ");
        std::io::stdout().flush()?;
        let mut schedule = String::new();
        handle.read_line(&mut schedule)?;
        schedule.make_ascii_lowercase();

        if schedule.trim().is_empty() {
            return Ok(None);
        }

        let delta = if schedule.contains("year") || schedule.contains("annual") {
            RepDelta::Year(YearDelta::parse(&schedule.trim())?)
        } else if schedule.contains("month") || schedule.contains("quarter") {
            RepDelta::Month(MonthDelta::parse(&schedule.trim(), start)?)
        } else if schedule.contains("week") {
            RepDelta::Week(WeekDelta::parse(&schedule.trim(), start)?)
        } else {
            RepDelta::Day(DayDelta::parse(&schedule.trim())?)
        };

        // parse end
        print!("repetition end (blank for none): ");
        std::io::stdout().flush()?;
        let mut end_s = String::new();
        handle.read_line(&mut end_s)?;
        end_s.make_ascii_lowercase();

        let end = RepEnd::parse(&end_s)?;

        Ok(Some(Repetition{ delta, end }))
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Duration::Day(n)   => write!(f, "{} {}", n, if n == 1 { "day" } else { "days" }),
            Duration::Week(n)  => write!(f, "{} {}", n, if n == 1 { "week" } else { "weeks" }),
            Duration::Month(n) => write!(f, "{} {}", n, if n == 1 { "month" } else { "months" }),
            Duration::Year(n)  => write!(f, "{} {}", n, if n == 1 { "year" } else { "years" }),
        }
    }
}

impl fmt::Display for SimpleDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

impl fmt::Display for DayDelta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.nth == 1 {
            write!(f, "day")
        } else {
            write!(f, "{} days", self.nth)
        }
    }
}

impl fmt::Display for Weekday {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Weekday::Monday    => write!(f, "Monday"),
            Weekday::Tuesday   => write!(f, "Tuesday"),
            Weekday::Wednesday => write!(f, "Wednesday"),
            Weekday::Thursday  => write!(f, "Thursday"),
            Weekday::Friday    => write!(f, "Friday"),
            Weekday::Saturday  => write!(f, "Saturday"),
            Weekday::Sunday    => write!(f, "Sunday"),
        }
    }
}

impl fmt::Display for WeekDelta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.nth ==1 {
            write!(f, "week on ")?;
        } else {
            write!(f, "{} weeks on ", self.nth)?;
        }

        write!(f, "{}", self.on[0])?;
        for day in &self.on[1..] {
            write!(f, ", {}", day)?;
        }
        Ok(())
    }
}

impl fmt::Display for MonthDeltaDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.nth == 1 {
            write!(f, "month on the ")?;
        } else {
            write!(f, "{} months on the ", self.nth)?;
        }

        write!(f, "{}{}", self.days[0], suffix_for_day(&self.days[0]))?;
        for day in &self.days[1..] {
            write!(f, ", {}{}", day, suffix_for_day(day))?;
        }
        Ok(())
    }
}

impl fmt::Display for MonthDeltaWeek {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} on the {} {}", self.nth, if self.nth == 1 { "month" } else { "months" }, self.weekid_to_str(), self.day)
    }
}

impl fmt::Display for MonthDelta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            MonthDelta::OnDate(d) => write!(f, "{}", d),
            MonthDelta::OnWeek(d) => write!(f, "{}", d),
        }
    }
}

impl fmt::Display for YearDelta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.nth == 1 {
            write!(f, "year")
        } else {
            write!(f, "{} years", self.nth)
        }
    }
}

impl fmt::Display for RepDelta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            RepDelta::Day(d)   => write!(f, "{}", d),
            RepDelta::Week(d)  => write!(f, "{}", d),
            RepDelta::Month(d) => write!(f, "{}", d),
            RepDelta::Year(d)  => write!(f, "{}", d),
        }
    }
}

impl fmt::Display for RepEnd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            RepEnd::Never => write!(f, "never ending"),
            RepEnd::Date(d) => write!(f, "ending on {}", d),
            RepEnd::Count(c) => write!(f, "ending after {} {}", c, if c == 1 { "occurrence" } else { "occurrences" }),
        }
    }
}

impl fmt::Display for Repetition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.delta)?;
        match self.end {
            RepEnd::Never => Ok(()),
            _ => write!(f, " {}", self.end),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_date() {
        let date = SimpleDate::from_ymd(2020, 9, 19);

        assert_eq!(date.day, 19);
        assert_eq!(date.month, 9);
        assert_eq!(date.year, 2020);
    }

    #[test]
    fn num_days_in_month() {
        assert_eq!(days_in_month(1999, 2), 28);
        assert_eq!(days_in_month(2000, 2), 29);
        assert_eq!(days_in_month(2004, 2), 29);
        assert_eq!(days_in_month(2100, 2), 28);

        assert_eq!(days_in_month(1999, 1), 31);
        assert_eq!(days_in_month(2000, 1), 31);
        assert_eq!(days_in_month(2004, 1), 31);
        assert_eq!(days_in_month(2100, 1), 31);
    }

    #[test]
    fn add_year_to_date() {
        let start = SimpleDate::from_ymd(2020, 9, 19);
        let duration = Duration::Year(1);
        let end = &start + &duration;

        assert_eq!(end.day, 19);
        assert_eq!(end.month, 9);
        assert_eq!(end.year, 2021);
    }

    #[test]
    fn add_years_to_date() {
        let start = SimpleDate::from_ymd(2020, 9, 19);
        let duration = Duration::Year(5);
        let end = &start + &duration;

        assert_eq!(end.day, 19);
        assert_eq!(end.month, 9);
        assert_eq!(end.year, 2025);
    }

    #[test]
    fn add_year_to_leap_date() {
        let start = SimpleDate::from_ymd(2020, 2, 29);
        let duration = Duration::Year(1);
        let end = &start + &duration;

        assert_eq!(end.day, 28);
        assert_eq!(end.month, 2);
        assert_eq!(end.year, 2021);
    }

    #[test]
    fn add_month_to_date() {
        let start = SimpleDate::from_ymd(2020, 2, 29);
        let duration = Duration::Month(1);
        let end = &start + &duration;

        assert_eq!(end.day, 29);
        assert_eq!(end.month, 3);
        assert_eq!(end.year, 2020);
    }

    #[test]
    fn add_months_to_date() {
        let start = SimpleDate::from_ymd(2020, 2, 28);
        let duration = Duration::Month(12);
        let end = &start + &duration;

        assert_eq!(end.day, 28);
        assert_eq!(end.month, 2);
        assert_eq!(end.year, 2021);
    }

    #[test]
    fn add_month_to_date_short_month() {
        let start = SimpleDate::from_ymd(2019, 1, 31);
        let duration = Duration::Month(1);
        let end = &start + &duration;

        assert_eq!(end.day, 28);
        assert_eq!(end.month, 2);
        assert_eq!(end.year, 2019);
    }

    #[test]
    fn add_month_to_date_short_month_leap_year() {
        let start = SimpleDate::from_ymd(2020, 1, 31);
        let duration = Duration::Month(1);
        let end = &start + &duration;

        assert_eq!(end.day, 29);
        assert_eq!(end.month, 2);
        assert_eq!(end.year, 2020);
    }

    #[test]
    fn add_week_to_date() {
        let start = SimpleDate::from_ymd(2020, 1, 1);
        let duration = Duration::Week(1);
        let end = &start + &duration;

        assert_eq!(end.day, 8);
        assert_eq!(end.month, 1);
        assert_eq!(end.year, 2020);
    }

    #[test]
    fn add_weeks_to_date() {
        let start = SimpleDate::from_ymd(2020, 8, 29);
        let duration = Duration::Week(7);
        let end = &start + &duration;

        assert_eq!(end.day, 17);
        assert_eq!(end.month, 10);
        assert_eq!(end.year, 2020);
    }

    #[test]
    fn add_weeks_to_date_overflow_month() {
        let start = SimpleDate::from_ymd(2020, 12, 1);
        let duration = Duration::Week(5);
        let end = &start + &duration;

        assert_eq!(end.day, 5);
        assert_eq!(end.month, 1);
        assert_eq!(end.year, 2021);
    }

    #[test]
    fn add_day_to_date() {
        let start = SimpleDate::from_ymd(2020, 12, 31);
        let duration = Duration::Day(1);
        let end = &start + &duration;

        assert_eq!(end.day, 1);
        assert_eq!(end.month, 1);
        assert_eq!(end.year, 2021);
    }

    #[test]
    fn add_days_to_date() {
        let start = SimpleDate::from_ymd(2021, 1, 1);
        let duration = Duration::Day(100);
        let end = &start + &duration;

        assert_eq!(end.day, 11);
        assert_eq!(end.month, 4);
        assert_eq!(end.year, 2021);
    }

    #[test]
    fn add_days_to_date_multiple_years() {
        let start = SimpleDate::from_ymd(2021, 1, 1);
        let duration = Duration::Day(730);
        let end = &start + &duration;

        assert_eq!(end.day, 1);
        assert_eq!(end.month, 1);
        assert_eq!(end.year, 2023);
    }

    #[test]
    fn weekday_of_date() {
        assert_eq!(get_weekday_of_date(&SimpleDate::from_ymd(1789, 7, 14)), Weekday::Tuesday);
        assert_eq!(get_weekday_of_date(&SimpleDate::from_ymd(1900, 1, 1)), Weekday::Monday);
        assert_eq!(get_weekday_of_date(&SimpleDate::from_ymd(1945, 4, 30)), Weekday::Monday);
        assert_eq!(get_weekday_of_date(&SimpleDate::from_ymd(1969, 7, 20)), Weekday::Sunday);
        assert_eq!(get_weekday_of_date(&SimpleDate::from_ymd(2013, 6, 15)), Weekday::Saturday);
        assert_eq!(get_weekday_of_date(&SimpleDate::from_ymd(2020, 9, 20)), Weekday::Sunday);
        assert_eq!(get_weekday_of_date(&SimpleDate::from_ymd(2020, 12, 31)), Weekday::Thursday);
    }

    #[test]
    fn add_rep_day_to_date() {
        let date = SimpleDate::from_ymd(2020, 9, 20);
        let last = &date + &RepDelta::Day(DayDelta{ nth: 8 });

        assert_eq!(last.year, 2020);
        assert_eq!(last.month, 9);
        assert_eq!(last.day, 28);
    }

    #[test]
    fn add_rep_week_to_date() {
        let date = SimpleDate::from_ymd(2020, 9, 20);
        let last = &date + &RepDelta::Week(WeekDelta{ nth: 3, on: vec!(Weekday::Monday)});

        assert_eq!(last.year, 2020);
        assert_eq!(last.month, 10);
        assert_eq!(last.day, 12);
    }

    #[test]
    fn add_rep_month_date_to_date_leap() {
        let date = SimpleDate::from_ymd(2019, 11, 30);
        let last = &date + &RepDelta::Month(MonthDelta::OnDate(MonthDeltaDate{ nth: 4, days: vec!(31) }));

        assert_eq!(last.year, 2020);
        assert_eq!(last.month, 2);
        assert_eq!(last.day, 29);
    }

    #[test]
    fn add_rep_month_date_to_date_leap_multiple_days() {
        let date = SimpleDate::from_ymd(2019, 11, 30);
        let last = &date + &RepDelta::Month(MonthDelta::OnDate(MonthDeltaDate{ nth: 4, days: vec!(15, 31) }));

        assert_eq!(last.year, 2020);
        assert_eq!(last.month, 3);
        assert_eq!(last.day, 31);
    }

    #[test]
    fn add_rep_month_date_to_date_higher() {
        let date = SimpleDate::from_ymd(2019, 11, 10);
        let last = &date + &RepDelta::Month(MonthDelta::OnDate(MonthDeltaDate{ nth: 3, days: vec!(15) }));

        assert_eq!(last.year, 2020);
        assert_eq!(last.month, 1);
        assert_eq!(last.day, 15);
    }

    #[test]
    fn add_rep_month_date_to_date_higher_multiple_days() {
        let date = SimpleDate::from_ymd(2019, 11, 10);
        let last = &date + &RepDelta::Month(MonthDelta::OnDate(MonthDeltaDate{ nth: 3, days: vec!(11, 15, 20) }));

        assert_eq!(last.year, 2020);
        assert_eq!(last.month, 1);
        assert_eq!(last.day, 20);
    }

    #[test]
    fn add_rep_month_date_to_date_lower() {
        let date = SimpleDate::from_ymd(2019, 11, 20);
        let last = &date + &RepDelta::Month(MonthDelta::OnDate(MonthDeltaDate{ nth: 3, days: vec!(15) }));

        assert_eq!(last.year, 2020);
        assert_eq!(last.month, 2);
        assert_eq!(last.day, 15);
    }

    #[test]
    fn add_rep_month_date_to_date_lower_multiple_days() {
        let date = SimpleDate::from_ymd(2019, 11, 20);
        let last = &date + &RepDelta::Month(MonthDelta::OnDate(MonthDeltaDate{ nth: 3, days: vec!(10, 15, 25) }));

        assert_eq!(last.year, 2020);
        assert_eq!(last.month, 2);
        assert_eq!(last.day, 25);
    }

    #[test]
    fn add_rep_month_date_to_week_lower() {
        let date = SimpleDate::from_ymd(2020, 9, 1);
        let last = &date + &RepDelta::Month(MonthDelta::OnWeek(MonthDeltaWeek{ nth: 2, weekid: 2, day: Weekday::Monday }));

        assert_eq!(last.year, 2020);
        assert_eq!(last.month, 10);
        assert_eq!(last.day, 12);
    }

    #[test]
    fn add_rep_month_date_to_week_higher() {
        let date = SimpleDate::from_ymd(2020, 9, 21);
        let last = &date + &RepDelta::Month(MonthDelta::OnWeek(MonthDeltaWeek{ nth: 2, weekid: 2, day: Weekday::Monday }));

        assert_eq!(last.year, 2020);
        assert_eq!(last.month, 11);
        assert_eq!(last.day, 9);
    }

    #[test]
    fn cmp_simple_date() {
        let old = SimpleDate::from_ymd(2020, 9, 20);
        let new = SimpleDate::from_ymd(2020, 9, 21);

        assert!(old < new);
        assert!(new > old);

        let old2 = SimpleDate::from_ymd(2020, 9, 20);
        assert!(old == old2);
    }

    #[test]
    fn add_rep_never_to_date() {
        let date = SimpleDate::from_ymd(2020, 9, 20);
        let rep = Repetition{ delta: RepDelta::Day(DayDelta{ nth: 1 }), end: RepEnd::Never };

        let result = &date + &rep;
        assert_eq!(result.year, 9999);
        assert_eq!(result.month, 12);
        assert_eq!(result.day, 31);
    }

    #[test]
    fn add_rep_count_to_date() {
        let date = SimpleDate::from_ymd(2020, 9, 20);
        let rep = Repetition{ delta: RepDelta::Day(DayDelta{ nth: 1 }), end: RepEnd::Count(5) };

        let result = &date + &rep;
        assert_eq!(result.year, 2020);
        assert_eq!(result.month, 9);
        assert_eq!(result.day, 25);
    }

    #[test]
    fn add_rep_date_to_date() {
        let date = SimpleDate::from_ymd(2020, 9, 20);
        let rep = Repetition{ delta: RepDelta::Day(DayDelta{ nth: 1 }), end: RepEnd::Date(SimpleDate::from_ymd(2020, 12, 31)) };

        let result = &date + &rep;
        assert_eq!(result.year, 2020);
        assert_eq!(result.month, 12);
        assert_eq!(result.day, 31);
    }

    #[test]
    fn add_rep_count_and_delta_to_date() {
        let date = SimpleDate::from_ymd(2020, 9, 20);
        let rep = Repetition{ delta: RepDelta::Month(MonthDelta::OnDate(MonthDeltaDate{ nth: 3, days: vec!(15) })), end: RepEnd::Count(5) };

        let result = &date + &rep;
        assert_eq!(result.year, 2021);
        assert_eq!(result.month, 12);
        assert_eq!(result.day, 15);
    }

    #[test]
    fn add_rep_date_and_delta_to_date() {
        let date = SimpleDate::from_ymd(2020, 9, 20);
        let rep = Repetition{ delta: RepDelta::Month(MonthDelta::OnDate(MonthDeltaDate{ nth: 3, days: vec!(15) })), end: RepEnd::Date(SimpleDate::from_ymd(2021, 12, 31)) };

        let result = &date + &rep;
        assert_eq!(result.year, 2021);
        assert_eq!(result.month, 12);
        assert_eq!(result.day, 15);
    }

    #[test]
    fn sub_year_from_date() {
        let start = SimpleDate::from_ymd(2020, 11, 1);
        let duration = Duration::Year(1);
        let end = &start - &duration;

        assert_eq!(end.year, 2019);
        assert_eq!(end.month, 11);
        assert_eq!(end.day, 1);
    }

    #[test]
    fn sub_year_from_date_non_leap() {
        let start = SimpleDate::from_ymd(2020, 2, 29);
        let duration = Duration::Year(1);
        let end = &start - &duration;

        assert_eq!(end.year, 2019);
        assert_eq!(end.month, 2);
        assert_eq!(end.day, 28);
    }

    #[test]
    fn sub_year_from_date_leap() {
        let start = SimpleDate::from_ymd(2020, 2, 29);
        let duration = Duration::Year(4);
        let end = &start - &duration;

        assert_eq!(end.year, 2016);
        assert_eq!(end.month, 2);
        assert_eq!(end.day, 29);
    }

    #[test]
    fn sub_month_from_date() {
        let start = SimpleDate::from_ymd(2020, 11, 1);
        let duration = Duration::Month(1);
        let end = &start - &duration;

        assert_eq!(end.year, 2020);
        assert_eq!(end.month, 10);
        assert_eq!(end.day, 1);
    }

    #[test]
    fn sub_months_from_date_underflow_year() {
        let start = SimpleDate::from_ymd(2020, 2, 1);
        let duration = Duration::Month(2);
        let end = &start - &duration;

        assert_eq!(end.year, 2019);
        assert_eq!(end.month, 12);
        assert_eq!(end.day, 1);
    }

    #[test]
    fn sub_week_from_date() {
        let start = SimpleDate::from_ymd(2020, 10, 31);
        let duration = Duration::Week(1);
        let end = &start - &duration;

        assert_eq!(end.year, 2020);
        assert_eq!(end.month, 10);
        assert_eq!(end.day, 24);
    }

    #[test]
    fn sub_weeks_from_date_underflow_month() {
        let start = SimpleDate::from_ymd(2020, 11, 1);
        let duration = Duration::Week(2);
        let end = &start - &duration;

        assert_eq!(end.year, 2020);
        assert_eq!(end.month, 10);
        assert_eq!(end.day, 18);
    }

    #[test]
    fn sub_weeks_from_date_underflow_year() {
        let start = SimpleDate::from_ymd(2020, 2, 2);
        let duration = Duration::Week(5);
        let end = &start - &duration;

        assert_eq!(end.year, 2019);
        assert_eq!(end.month, 12);
        assert_eq!(end.day, 29);
    }

    #[test]
    fn sub_day_from_date() {
        let start = SimpleDate::from_ymd(2020, 11, 1);
        let duration = Duration::Day(1);
        let end = &start - &duration;

        assert_eq!(end.year, 2020);
        assert_eq!(end.month, 10);
        assert_eq!(end.day, 31);
    }
}

impl fmt::Display for DateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for DateError {}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_921() {
//    rusty_monitor::set_test_id(921);
    let mut str_0: &str = "internal u64";
    let mut vec_0: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_0: u64 = 23u64;
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_0, days: vec_0};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnDate(monthdeltadate_0);
    let mut u64_1: u64 = 0u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_1);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Sunday;
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut str_1: &str = "parse::String";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_2: &str = "occurrence";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2_ref_0: &str = &mut str_2;
    let mut option_2: std::option::Option<date::Weekday> = crate::date::MonthDelta::parse_weekday(str_2_ref_0);
    let mut result_0: std::result::Result<date::RepEnd, std::boxed::Box<dyn std::error::Error>> = crate::date::RepEnd::parse_count(str_0_ref_0);
    let mut result_1: std::result::Result<date::RepEnd, std::boxed::Box<dyn std::error::Error>> = crate::date::RepEnd::parse_count(str_1_ref_0);
    let mut weekday_1: date::Weekday = std::option::Option::unwrap(option_2);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6749() {
//    rusty_monitor::set_test_id(6749);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Sunday;
    let mut u64_0: u64 = 3u64;
    let mut u64_1: u64 = 1445u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_1, weekid: u64_0, day: weekday_0};
    let mut u64_2: u64 = 212u64;
    let mut u64_3: u64 = 400u64;
    let mut weekday_1: date::Weekday = crate::date::Weekday::Saturday;
    let mut u64_4: u64 = 5245u64;
    let mut u64_5: u64 = 243u64;
    let mut u64_6: u64 = 4525u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_6);
    let mut u64_7: u64 = 8387u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_7};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_8: u64 = 100u64;
    let mut u64_9: u64 = 4129u64;
    let mut u64_10: u64 = 181u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_10, u64_9, u64_8);
    let mut weekday_2: date::Weekday = crate::date::Weekday::Tuesday;
    let mut u64_11: u64 = 1350u64;
    let mut u64_12: u64 = 12u64;
    let mut weekday_3: date::Weekday = crate::date::Weekday::Monday;
    let mut monthdeltaweek_1: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_12, weekid: u64_11, day: weekday_2};
    let mut weekday_4: date::Weekday = crate::date::Weekday::Friday;
    let mut monthdeltaweek_0_ref_0: &crate::date::MonthDeltaWeek = &mut monthdeltaweek_0;
    let mut str_0: &str = crate::date::MonthDeltaWeek::weekid_to_str(monthdeltaweek_0_ref_0);
    let mut repetition_1: crate::date::Repetition = std::option::Option::unwrap(option_0);
    let mut monthdeltaweek_2: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_5, weekid: u64_4, day: weekday_1};
    let mut u64_13: u64 = crate::date::days_in_month(u64_3, u64_2);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1691() {
//    rusty_monitor::set_test_id(1691);
    let mut u64_0: u64 = 22u64;
    let mut u64_1: u64 = 181u64;
    let mut u64_2: u64 = 3976u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_2, month: u64_1, day: u64_0};
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut u64_3: u64 = 1700u64;
    let mut u64_4: u64 = 151u64;
    let mut u64_5: u64 = 7160u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_5, u64_4, u64_3);
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut u64_6: u64 = 3u64;
    let mut u64_7: u64 = 4166u64;
    let mut u64_8: u64 = 6795u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_8, u64_7, u64_6);
    let mut simpledate_2_ref_0: &crate::date::SimpleDate = &mut simpledate_2;
    let mut u64_9: u64 = 28u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_9};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Saturday;
    let mut str_0: &str = "description";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut weekday_1: date::Weekday = crate::date::get_weekday_of_date(simpledate_2_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3850() {
//    rusty_monitor::set_test_id(3850);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Sunday;
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_0: u64 = 6028u64;
    let mut u64_1: u64 = 5940u64;
    let mut u64_2: u64 = 10u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_2, u64_1, u64_0);
    let mut str_0: &str = "occurrence";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "description";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "ZnGJ";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut option_2: std::option::Option<date::Weekday> = crate::date::MonthDelta::parse_weekday(str_2_ref_0);
    let mut result_0: std::result::Result<date::RepEnd, std::boxed::Box<dyn std::error::Error>> = crate::date::RepEnd::parse_count(str_1_ref_0);
    let mut result_1: std::result::Result<date::RepEnd, std::boxed::Box<dyn std::error::Error>> = crate::date::RepEnd::parse(str_0_ref_0);
    let mut weekday_1: date::Weekday = std::option::Option::unwrap(option_2);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1002() {
//    rusty_monitor::set_test_id(1002);
    let mut str_0: &str = "first";
    let mut str_1: &str = "years";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "ScZAbazI2lPCxbMGc3";
    let mut str_3: &str = "nth";
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut u64_0: u64 = 5494u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_0};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut str_4: &str = "aXEKj6NjwGr5g5";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut u64_1: u64 = 243u64;
    let mut u64_2: u64 = 22u64;
    let mut u64_3: u64 = 120u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_3, month: u64_2, day: u64_1};
    let mut str_5: &str = "start";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut u64_4: u64 = 28u64;
    let mut vec_0: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_5: u64 = 23u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_5, on: vec_0};
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_4);
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_0: std::option::Option<u64> = crate::date::MonthDelta::parse_nth(str_2_ref_0);
    let mut option_1: std::option::Option<u64> = crate::date::MonthDelta::parse_nth(str_0_ref_0);
    let mut option_2: std::option::Option<u64> = crate::date::MonthDelta::parse_nth(str_3_ref_0);
    let mut option_3: std::option::Option<u64> = crate::date::MonthDelta::parse_nth(str_4_ref_0);
    let mut result_0: std::result::Result<date::RepEnd, std::boxed::Box<dyn std::error::Error>> = crate::date::RepEnd::parse(str_1_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2696() {
//    rusty_monitor::set_test_id(2696);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut u64_0: u64 = 21u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_0};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Never;
    let mut u64_1: u64 = 1402u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_1};
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut repetition_1: crate::date::Repetition = crate::date::Repetition {delta: repdelta_1, end: repend_1};
    let mut u64_2: u64 = 5u64;
    let mut u64_2_ref_0: &u64 = &mut u64_2;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_1);
    let mut u64_3: u64 = 1270u64;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_3);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_4: u64 = 120u64;
    let mut u64_5: u64 = 9948u64;
    let mut u64_6: u64 = 304u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_6, u64_5, u64_4);
    let mut i64_0: i64 = 0i64;
    let mut str_0: &str = "{} day";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_7: u64 = 334u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_7, string_0, i64_0, simpledate_0, option_1, option_0, vec_0);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut vec_1: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_8: u64 = 22u64;
    let mut duration_1: date::Duration = crate::date::Duration::Year(u64_8);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
    let mut u64_9: u64 = 243u64;
    let mut u64_10: u64 = 12u64;
    let mut u64_11: u64 = 22u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_11, u64_10, u64_9);
    let mut i64_1: i64 = 0i64;
    let mut str_1: &str = "show";
    let mut string_1: std::string::String = std::string::String::from(str_1);
    let mut u64_12: u64 = 11u64;
    let mut expense_1: crate::expense::Expense = crate::expense::Expense::new(u64_12, string_1, i64_1, simpledate_1, option_3, option_2, vec_1);
    let mut expense_1_ref_0: &crate::expense::Expense = &mut expense_1;
    let mut ordering_0: std::cmp::Ordering = crate::expense::Expense::compare_dates(expense_1_ref_0, expense_0_ref_0);
    let mut str_2: &str = crate::date::suffix_for_day(u64_2_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1902() {
//    rusty_monitor::set_test_id(1902);
    let mut u64_0: u64 = 0u64;
    let mut u64_1: u64 = 243u64;
    let mut u64_2: u64 = 9121u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_2, month: u64_1, day: u64_0};
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut str_0: &str = "tag";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_3: u64 = 30u64;
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_3);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut str_1: &str = "th";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut u64_4: u64 = 5866u64;
    let mut duration_1: date::Duration = crate::date::Duration::Day(u64_4);
    let mut duration_1_ref_0: &date::Duration = &mut duration_1;
    let mut u64_5: u64 = 1u64;
    let mut u64_6: u64 = 6023u64;
    let mut u64_7: u64 = 22u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_7, u64_6, u64_5);
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut u64_8: u64 = 1700u64;
    let mut duration_2: date::Duration = crate::date::Duration::Month(u64_8);
    let mut option_2: std::option::Option<u64> = crate::date::MonthDelta::parse_nth(str_1_ref_0);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Tuesday;
    let mut result_0: std::result::Result<date::MonthDelta, std::boxed::Box<dyn std::error::Error>> = crate::date::MonthDelta::parse(str_0_ref_0, simpledate_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7136() {
//    rusty_monitor::set_test_id(7136);
    let mut u64_0: u64 = 212u64;
    let mut u64_1: u64 = 400u64;
    let mut weekday_0: date::Weekday = crate::date::Weekday::Thursday;
    let mut u64_2: u64 = 4525u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_2);
    let mut u64_3: u64 = 8387u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_3};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_4: u64 = 100u64;
    let mut u64_5: u64 = 4129u64;
    let mut u64_6: u64 = 181u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_6, u64_5, u64_4);
    let mut weekday_1: date::Weekday = crate::date::Weekday::Tuesday;
    let mut weekday_2: date::Weekday = crate::date::Weekday::Monday;
    let mut repetition_1: crate::date::Repetition = std::option::Option::unwrap(option_0);
    let mut u64_7: u64 = crate::date::days_in_month(u64_1, u64_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1991() {
//    rusty_monitor::set_test_id(1991);
    let mut u64_0: u64 = 23u64;
    let mut u64_1: u64 = 304u64;
    let mut u64_2: u64 = 9824u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_2, u64_1, u64_0);
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut str_0: &str = "every {} months";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut u64_3: u64 = 5494u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_3};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut str_1: &str = "aXEKj6NjwGr5g5";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut u64_4: u64 = 9468u64;
    let mut u64_5: u64 = 243u64;
    let mut u64_6: u64 = 22u64;
    let mut u64_7: u64 = 120u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_7, month: u64_6, day: u64_5};
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut str_2: &str = "start";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut u64_8: u64 = 5399u64;
    let mut u64_9: u64 = 12u64;
    let mut u64_10: u64 = 893u64;
    let mut u64_11: u64 = 28u64;
    let mut u64_12: u64 = 2755u64;
    let mut vec_0: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_13: u64 = 23u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_13, on: vec_0};
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_12};
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_11);
    let mut u64_14: u64 = crate::date::days_in_month(u64_10, u64_9);
    let mut duration_1: date::Duration = crate::date::Duration::Day(u64_8);
    let mut result_0: std::result::Result<crate::date::YearDelta, std::boxed::Box<dyn std::error::Error>> = crate::date::YearDelta::parse(str_2_ref_0);
    let mut duration_2: date::Duration = crate::date::Duration::Month(u64_4);
    let mut result_1: std::result::Result<date::RepEnd, std::boxed::Box<dyn std::error::Error>> = crate::date::RepEnd::parse_count(str_1_ref_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut result_2: std::result::Result<date::MonthDelta, std::boxed::Box<dyn std::error::Error>> = crate::date::MonthDelta::parse(str_0_ref_0, simpledate_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3370() {
//    rusty_monitor::set_test_id(3370);
    let mut u64_0: u64 = 365u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_0};
    let mut str_0: &str = "N";
    let mut u64_1: u64 = 4u64;
    let mut u64_2: u64 = 3u64;
    let mut u64_3: u64 = 7362u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_3, u64_2, u64_1);
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut str_1: &str = "OnDate";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "6";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut u64_4: u64 = 2933u64;
    let mut u64_5: u64 = 6807u64;
    let mut u64_6: u64 = 1700u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_6, month: u64_5, day: u64_4};
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut str_3: &str = "the ID of the expense to remove";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut u64_7: u64 = 7620u64;
    let mut u64_8: u64 = 965u64;
    let mut u64_9: u64 = 308u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_9, u64_8, u64_7);
    let mut simpledate_2_ref_0: &crate::date::SimpleDate = &mut simpledate_2;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut u64_10: u64 = 2u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_10};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_11: u64 = 28u64;
    let mut duration_0: date::Duration = crate::date::Duration::Year(u64_11);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_12: u64 = 29u64;
    let mut u64_13: u64 = 3100u64;
    let mut u64_14: u64 = 28u64;
    let mut simpledate_3: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_14, month: u64_13, day: u64_12};
    let mut str_4: &str = "Date";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "ExpenseError";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut u64_15: u64 = 6974u64;
    let mut u64_15_ref_0: &u64 = &mut u64_15;
    let mut str_6: &str = crate::date::suffix_for_day(u64_15_ref_0);
    let mut result_0: std::result::Result<std::vec::Vec<date::Weekday>, std::boxed::Box<dyn std::error::Error>> = crate::date::WeekDelta::parse_days(str_5_ref_0);
    let mut result_1: std::result::Result<std::vec::Vec<date::Weekday>, std::boxed::Box<dyn std::error::Error>> = crate::date::WeekDelta::parse_days(str_4_ref_0);
    let mut str_7: &str = "week";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_8: &str = "sPdzngvsLKmF";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_8_ref_0: &str = &mut str_8;
    let mut result_2: std::result::Result<date::RepEnd, std::boxed::Box<dyn std::error::Error>> = crate::date::RepEnd::parse_count(str_7_ref_0);
    let mut result_3: std::result::Result<date::RepEnd, std::boxed::Box<dyn std::error::Error>> = crate::date::RepEnd::parse_date(str_8_ref_0);
    let mut result_4: std::result::Result<date::MonthDelta, std::boxed::Box<dyn std::error::Error>> = crate::date::MonthDelta::parse(str_2_ref_0, simpledate_1_ref_0);
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Never;
    let mut result_5: std::result::Result<crate::date::WeekDelta, std::boxed::Box<dyn std::error::Error>> = crate::date::WeekDelta::parse(str_0_ref_0, simpledate_0_ref_0);
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut monthdelta_0: date::MonthDelta = std::result::Result::unwrap(result_4);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2219() {
//    rusty_monitor::set_test_id(2219);
    let mut u64_0: u64 = 412u64;
    let mut u64_1: u64 = 6u64;
    let mut u64_2: u64 = 2394u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_2, u64_1, u64_0);
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut str_0: &str = "4OhG2LL";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "the name of the tag to remove";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "every {} year";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "Gp9";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "PGzefjU6pejvEsHE";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "start";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "delta";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7: &str = "ko";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_8: &str = "this is an expense, not an income";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut str_9: &str = "jsO62";
    let mut str_9_ref_0: &str = &mut str_9;
    let mut str_10: &str = "month";
    let mut str_10_ref_0: &str = &mut str_10;
    let mut str_11: &str = "WJx";
    let mut str_11_ref_0: &str = &mut str_11;
    let mut result_0: std::result::Result<crate::date::YearDelta, std::boxed::Box<dyn std::error::Error>> = crate::date::YearDelta::parse(str_11_ref_0);
    let mut result_1: std::result::Result<crate::date::YearDelta, std::boxed::Box<dyn std::error::Error>> = crate::date::YearDelta::parse(str_10_ref_0);
    let mut result_2: std::result::Result<crate::date::YearDelta, std::boxed::Box<dyn std::error::Error>> = crate::date::YearDelta::parse(str_9_ref_0);
    let mut result_3: std::result::Result<crate::date::YearDelta, std::boxed::Box<dyn std::error::Error>> = crate::date::YearDelta::parse(str_8_ref_0);
    let mut result_4: std::result::Result<crate::date::YearDelta, std::boxed::Box<dyn std::error::Error>> = crate::date::YearDelta::parse(str_7_ref_0);
    let mut result_5: std::result::Result<crate::date::YearDelta, std::boxed::Box<dyn std::error::Error>> = crate::date::YearDelta::parse(str_6_ref_0);
    let mut result_6: std::result::Result<crate::date::YearDelta, std::boxed::Box<dyn std::error::Error>> = crate::date::YearDelta::parse(str_5_ref_0);
    let mut result_7: std::result::Result<crate::date::YearDelta, std::boxed::Box<dyn std::error::Error>> = crate::date::YearDelta::parse(str_4_ref_0);
    let mut result_8: std::result::Result<crate::date::YearDelta, std::boxed::Box<dyn std::error::Error>> = crate::date::YearDelta::parse(str_3_ref_0);
    let mut result_9: std::result::Result<crate::date::YearDelta, std::boxed::Box<dyn std::error::Error>> = crate::date::YearDelta::parse(str_2_ref_0);
    let mut result_10: std::result::Result<crate::date::YearDelta, std::boxed::Box<dyn std::error::Error>> = crate::date::YearDelta::parse(str_1_ref_0);
    let mut result_11: std::result::Result<crate::date::WeekDelta, std::boxed::Box<dyn std::error::Error>> = crate::date::WeekDelta::parse(str_0_ref_0, simpledate_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7047() {
//    rusty_monitor::set_test_id(7047);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Wednesday;
    let mut u64_0: u64 = 1u64;
    let mut u64_1: u64 = 510u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_1, weekid: u64_0, day: weekday_0};
    let mut monthdeltaweek_0_ref_0: &crate::date::MonthDeltaWeek = &mut monthdeltaweek_0;
    let mut u64_2: u64 = 90u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_2);
    let mut vec_0: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_3: u64 = 5u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_3, on: vec_0};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Week(weekdelta_0);
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Never;
    let mut weekday_1: date::Weekday = crate::date::Weekday::Tuesday;
    let mut u64_4: u64 = 4782u64;
    let mut u64_5: u64 = 10u64;
    let mut monthdeltaweek_1: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_5, weekid: u64_4, day: weekday_1};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_1);
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut u64_6: u64 = 304u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_6};
    let mut u64_7: u64 = 365u64;
    let mut str_0: &str = "the name of the tag to remove";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "every {} year";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "Gp9";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "PGzefjU6pejvEsHE";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut daydelta_1: crate::date::DayDelta = crate::date::DayDelta {nth: u64_7};
    let mut repdelta_2: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_1, end: repend_1};
    let mut option_0: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_8: u64 = 100u64;
    let mut u64_9: u64 = 4129u64;
    let mut u64_10: u64 = 181u64;
    let mut u64_11: u64 = 8122u64;
    let mut weekday_2: date::Weekday = crate::date::Weekday::Tuesday;
    let mut weekday_3: date::Weekday = crate::date::Weekday::Monday;
    let mut monthdeltaweek_2: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_11, weekid: u64_10, day: weekday_2};
    let mut weekday_4: date::Weekday = crate::date::Weekday::Friday;
    let mut monthdeltaweek_2_ref_0: &crate::date::MonthDeltaWeek = &mut monthdeltaweek_2;
    let mut str_4: &str = crate::date::MonthDeltaWeek::weekid_to_str(monthdeltaweek_0_ref_0);
    let mut monthdeltaweek_3: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_9, weekid: u64_8, day: weekday_4};
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7268() {
//    rusty_monitor::set_test_id(7268);
    let mut str_0: &str = "tag";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_0: u64 = 30u64;
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut str_1: &str = "th";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut u64_1: u64 = 5866u64;
    let mut duration_1: date::Duration = crate::date::Duration::Day(u64_1);
    let mut duration_1_ref_0: &date::Duration = &mut duration_1;
    let mut u64_2: u64 = 1u64;
    let mut u64_3: u64 = 6023u64;
    let mut u64_4: u64 = 22u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_4, u64_3, u64_2);
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut str_2: &str = "every {} days";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut u64_5: u64 = 1700u64;
    let mut duration_2: date::Duration = crate::date::Duration::Month(u64_5);
    let mut result_0: std::result::Result<date::RepEnd, std::boxed::Box<dyn std::error::Error>> = crate::date::RepEnd::parse_date(str_2_ref_0);
    let mut option_2: std::option::Option<u64> = crate::date::MonthDelta::parse_nth(str_1_ref_0);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Tuesday;
    let mut weekday_1: date::Weekday = crate::date::Weekday::Thursday;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_517() {
//    rusty_monitor::set_test_id(517);
    let mut u64_0: u64 = 5900u64;
    let mut u64_0_ref_0: &u64 = &mut u64_0;
    let mut u64_1: u64 = 7224u64;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_1);
    let mut u64_2: u64 = 324u64;
    let mut duration_1: date::Duration = crate::date::Duration::Year(u64_2);
    let mut u64_3: u64 = 29u64;
    let mut str_0: &str = "Date";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u64_3_ref_0: &u64 = &mut u64_3;
    let mut str_1: &str = crate::date::suffix_for_day(u64_0_ref_0);
    let mut result_0: std::result::Result<std::vec::Vec<date::Weekday>, std::boxed::Box<dyn std::error::Error>> = crate::date::WeekDelta::parse_days(str_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_497() {
//    rusty_monitor::set_test_id(497);
    let mut u64_0: u64 = 4u64;
    let mut u64_1: u64 = 2423u64;
    let mut u64_2: u64 = 400u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_2, month: u64_1, day: u64_0};
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut vec_0: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_3: u64 = 8136u64;
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_3, days: vec_0};
    let mut vec_1: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_4: u64 = 3230u64;
    let mut u64_5: u64 = 243u64;
    let mut u64_6: u64 = 1700u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_6, u64_5, u64_4);
    let mut str_0: &str = "YTU6ucUnwBVkcanuFaG";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnDate(monthdeltadate_0);
    let mut weekday_0: date::Weekday = crate::date::get_weekday_of_date(simpledate_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3944() {
//    rusty_monitor::set_test_id(3944);
    let mut u64_0: u64 = 870u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_0};
    let mut u64_1: u64 = 23u64;
    let mut u64_2: u64 = 12u64;
    let mut u64_3: u64 = 1u64;
    let mut u64_4: u64 = 2677u64;
    let mut u64_5: u64 = 400u64;
    let mut u64_6: u64 = 31u64;
    let mut u64_7: u64 = 8u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_7, month: u64_6, day: u64_5};
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_4, month: u64_3, day: u64_2};
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut u64_8: u64 = 21u64;
    let mut u64_8_ref_0: &u64 = &mut u64_8;
    let mut str_0: &str = "couldn't find anything with that id";
    let mut u64_9: u64 = 28u64;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_9);
    let mut str_1: &str = "the name of the tag to remove";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "every {} year";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "Gp9";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "bCAr";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "this is an expense, not an income";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "jsO62";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7: &str = "month";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_8: &str = "WJx";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut result_0: std::result::Result<crate::date::YearDelta, std::boxed::Box<dyn std::error::Error>> = crate::date::YearDelta::parse(str_8_ref_0);
    let mut result_1: std::result::Result<crate::date::YearDelta, std::boxed::Box<dyn std::error::Error>> = crate::date::YearDelta::parse(str_7_ref_0);
    let mut result_2: std::result::Result<crate::date::YearDelta, std::boxed::Box<dyn std::error::Error>> = crate::date::YearDelta::parse(str_6_ref_0);
    let mut result_3: std::result::Result<crate::date::YearDelta, std::boxed::Box<dyn std::error::Error>> = crate::date::YearDelta::parse(str_5_ref_0);
    let mut u64_1_ref_0: &u64 = &mut u64_1;
    let mut str_9: &str = crate::date::suffix_for_day(u64_8_ref_0);
    let mut result_4: std::result::Result<std::vec::Vec<date::Weekday>, std::boxed::Box<dyn std::error::Error>> = crate::date::WeekDelta::parse_days(str_3_ref_0);
    let mut result_5: std::result::Result<std::vec::Vec<date::Weekday>, std::boxed::Box<dyn std::error::Error>> = crate::date::WeekDelta::parse_days(str_1_ref_0);
    let mut str_10: &str = "week";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_11: &str = "sPdzngvsLKmF";
    let mut str_10_ref_0: &str = &mut str_10;
    let mut str_11_ref_0: &str = &mut str_11;
    let mut result_6: std::result::Result<date::RepEnd, std::boxed::Box<dyn std::error::Error>> = crate::date::RepEnd::parse_count(str_0_ref_0);
    let mut result_7: std::result::Result<date::RepEnd, std::boxed::Box<dyn std::error::Error>> = crate::date::RepEnd::parse_date(str_11_ref_0);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut result_8: std::result::Result<crate::date::WeekDelta, std::boxed::Box<dyn std::error::Error>> = crate::date::WeekDelta::parse(str_2_ref_0, simpledate_1_ref_0);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8254() {
//    rusty_monitor::set_test_id(8254);
    let mut u64_0: u64 = 4u64;
    let mut u64_1: u64 = 2423u64;
    let mut u64_2: u64 = 400u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_2, month: u64_1, day: u64_0};
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut vec_0: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_3: u64 = 8136u64;
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_3, days: vec_0};
    let mut vec_1: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_4: u64 = 8u64;
    let mut u64_5: u64 = 2068u64;
    let mut u64_6: u64 = 6386u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_6, month: u64_5, day: u64_4};
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_1);
    let mut u64_7: u64 = 5u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_7};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_8: u64 = 6u64;
    let mut u64_9: u64 = 243u64;
    let mut u64_10: u64 = 1700u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_10, u64_9, u64_8);
    let mut i64_0: i64 = 0i64;
    let mut str_0: &str = "YTU6ucUnwBVkcanuFaG";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_11: u64 = 7209u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_11, string_0, i64_0, simpledate_2, option_1, option_0, vec_1);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut simpledate_3: &crate::date::SimpleDate = crate::expense::Expense::get_start_date(expense_0_ref_0);
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnDate(monthdeltadate_0);
    let mut weekday_0: date::Weekday = crate::date::get_weekday_of_date(simpledate_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_528() {
//    rusty_monitor::set_test_id(528);
    let mut str_0: &str = "k7AqtF";
    let mut u64_0: u64 = 6709u64;
    let mut u64_1: u64 = 304u64;
    let mut u64_2: u64 = 120u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_2, month: u64_1, day: u64_0};
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut str_1: &str = "fYu3";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "TQGYrsjqa";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u64_3: u64 = 2u64;
    let mut u64_4: u64 = 1148u64;
    let mut u64_5: u64 = 143u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_5, u64_4, u64_3);
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut str_3: &str = "P2YCwaAQI";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut u64_6: u64 = 3611u64;
    let mut u64_7: u64 = 5040u64;
    let mut u64_8: u64 = 31u64;
    let mut str_2_ref_0: &str = &mut str_2;
    let mut result_0: std::result::Result<date::RepEnd, std::boxed::Box<dyn std::error::Error>> = crate::date::RepEnd::parse_date(str_2_ref_0);
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_8, month: u64_7, day: u64_6};
    let mut result_1: std::result::Result<date::MonthDelta, std::boxed::Box<dyn std::error::Error>> = crate::date::MonthDelta::parse(str_3_ref_0, simpledate_1_ref_0);
    let mut result_2: std::result::Result<date::RepEnd, std::boxed::Box<dyn std::error::Error>> = crate::date::RepEnd::parse_date(str_0_ref_0);
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut result_3: std::result::Result<crate::date::WeekDelta, std::boxed::Box<dyn std::error::Error>> = crate::date::WeekDelta::parse(str_1_ref_0, simpledate_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2777() {
//    rusty_monitor::set_test_id(2777);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Saturday;
    let mut weekday_1: date::Weekday = crate::date::Weekday::Friday;
    let mut u64_0: u64 = 2u64;
    let mut u64_1: u64 = 8u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_1, weekid: u64_0, day: weekday_1};
    let mut monthdeltaweek_0_ref_0: &crate::date::MonthDeltaWeek = &mut monthdeltaweek_0;
    let mut u64_2: u64 = 9191u64;
    let mut u64_3: u64 = 100u64;
    let mut u64_4: u64 = 6u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_4, month: u64_3, day: u64_2};
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut str_0: &str = "repair";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_5: u64 = 2995u64;
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_5);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_6: u64 = 9u64;
    let mut u64_7: u64 = 4u64;
    let mut u64_8: u64 = 1700u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_8, month: u64_7, day: u64_6};
    let mut str_1: &str = "OnDate";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "amount";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut u64_9: u64 = 2933u64;
    let mut u64_10: u64 = 6807u64;
    let mut u64_11: u64 = 1700u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_11, month: u64_10, day: u64_9};
    let mut simpledate_2_ref_0: &crate::date::SimpleDate = &mut simpledate_2;
    let mut str_3: &str = "the ID of the expense to remove";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = crate::date::MonthDeltaWeek::weekid_to_str(monthdeltaweek_0_ref_0);
    let mut repetition_0: crate::date::Repetition = std::option::Option::unwrap(option_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8634() {
//    rusty_monitor::set_test_id(8634);
    let mut u64_0: u64 = 4u64;
    let mut u64_1: u64 = 2423u64;
    let mut u64_2: u64 = 400u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_2, month: u64_1, day: u64_0};
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut vec_0: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_3: u64 = 8136u64;
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_3, days: vec_0};
    let mut vec_1: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut u64_4: u64 = 8u64;
    let mut u64_5: u64 = 2068u64;
    let mut u64_6: u64 = 6386u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_6, month: u64_5, day: u64_4};
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_1);
    let mut u64_7: u64 = 5u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_7};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_8: u64 = 3230u64;
    let mut u64_9: u64 = 243u64;
    let mut u64_10: u64 = 1700u64;
    let mut simpledate_2: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_10, u64_9, u64_8);
    let mut i64_0: i64 = 0i64;
    let mut str_0: &str = "YTU6ucUnwBVkcanuFaG";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_11: u64 = 7209u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_11, string_0, i64_0, simpledate_2, option_1, option_0, vec_1);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut simpledate_3: &crate::date::SimpleDate = crate::expense::Expense::get_start_date(expense_0_ref_0);
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnDate(monthdeltadate_0);
    let mut weekday_0: date::Weekday = crate::date::get_weekday_of_date(simpledate_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4244() {
//    rusty_monitor::set_test_id(4244);
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_0: u64 = 1270u64;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_1: u64 = 120u64;
    let mut u64_2: u64 = 9948u64;
    let mut u64_3: u64 = 304u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_3, u64_2, u64_1);
    let mut vec_1: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_2: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_4: u64 = 22u64;
    let mut duration_1: date::Duration = crate::date::Duration::Year(u64_4);
    let mut option_3: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
    let mut u64_5: u64 = 243u64;
    let mut u64_6: u64 = 12u64;
    let mut u64_7: u64 = 22u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_7, u64_6, u64_5);
    let mut i64_0: i64 = 0i64;
    let mut str_0: &str = "show";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut u64_8: u64 = 11u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_8, string_0, i64_0, simpledate_1, option_3, option_2, vec_1);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8537() {
//    rusty_monitor::set_test_id(8537);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Tuesday;
    let mut u64_0: u64 = 7248u64;
    let mut u64_1: u64 = 1u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_1, weekid: u64_0, day: weekday_0};
    let mut monthdeltaweek_0_ref_0: &crate::date::MonthDeltaWeek = &mut monthdeltaweek_0;
    let mut weekday_1: date::Weekday = crate::date::Weekday::Wednesday;
    let mut u64_2: u64 = 90u64;
    let mut u64_3: u64 = 7951u64;
    let mut monthdeltaweek_1: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_3, weekid: u64_2, day: weekday_1};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnWeek(monthdeltaweek_1);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut str_0: &str = "Njx1SZIlT7Msem3";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u64_4: u64 = 346u64;
    let mut str_1: &str = "show";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "yI7zjjs";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut result_0: std::result::Result<crate::date::DayDelta, std::boxed::Box<dyn std::error::Error>> = crate::date::DayDelta::parse(str_2_ref_0);
    let mut result_1: std::result::Result<crate::date::YearDelta, std::boxed::Box<dyn std::error::Error>> = crate::date::YearDelta::parse(str_1_ref_0);
    let mut daydelta_0: crate::date::DayDelta = std::result::Result::unwrap(result_0);
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_4};
    let mut result_2: std::result::Result<std::vec::Vec<date::Weekday>, std::boxed::Box<dyn std::error::Error>> = crate::date::WeekDelta::parse_days(str_0_ref_0);
    let mut str_3: &str = crate::date::MonthDeltaWeek::weekid_to_str(monthdeltaweek_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4925() {
//    rusty_monitor::set_test_id(4925);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Tuesday;
    let mut u64_0: u64 = 7248u64;
    let mut u64_1: u64 = 1u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_1, weekid: u64_0, day: weekday_0};
    let mut monthdeltaweek_0_ref_0: &crate::date::MonthDeltaWeek = &mut monthdeltaweek_0;
    let mut u64_2: u64 = 0u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_2);
    let mut weekday_1: date::Weekday = crate::date::Weekday::Wednesday;
    let mut str_0: &str = "occurrence";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "Njx1SZIlT7Msem3";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut u64_3: u64 = 22u64;
    let mut str_2: &str = "show";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut result_0: std::result::Result<crate::date::YearDelta, std::boxed::Box<dyn std::error::Error>> = crate::date::YearDelta::parse(str_2_ref_0);
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_3};
    let mut result_1: std::result::Result<std::vec::Vec<date::Weekday>, std::boxed::Box<dyn std::error::Error>> = crate::date::WeekDelta::parse_days(str_1_ref_0);
    let mut result_2: std::result::Result<crate::date::YearDelta, std::boxed::Box<dyn std::error::Error>> = crate::date::YearDelta::parse(str_0_ref_0);
    let mut str_3: &str = crate::date::MonthDeltaWeek::weekid_to_str(monthdeltaweek_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2458() {
//    rusty_monitor::set_test_id(2458);
    let mut u64_0: u64 = 120u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_0);
    let mut u64_1: u64 = 1700u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_1};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut u64_2: u64 = 243u64;
    let mut u64_2_ref_0: &u64 = &mut u64_2;
    let mut u64_3: u64 = 1532u64;
    let mut u64_4: u64 = 8846u64;
    let mut u64_5: u64 = 31u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_5, month: u64_4, day: u64_3};
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut str_0: &str = "{} months";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u64_6: u64 = 4u64;
    let mut u64_7: u64 = 55u64;
    let mut u64_8: u64 = 1700u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_8, month: u64_7, day: u64_6};
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut str_1: &str = "every {} month";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "init";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "ZnGJ";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut option_0: std::option::Option<date::Weekday> = crate::date::MonthDelta::parse_weekday(str_1_ref_0);
    let mut result_0: std::result::Result<date::RepEnd, std::boxed::Box<dyn std::error::Error>> = crate::date::RepEnd::parse_count(str_0_ref_0);
    let mut result_1: std::result::Result<date::RepEnd, std::boxed::Box<dyn std::error::Error>> = crate::date::RepEnd::parse(str_2_ref_0);
    let mut result_2: std::result::Result<date::RepEnd, std::boxed::Box<dyn std::error::Error>> = crate::date::RepEnd::parse_count(str_3_ref_0);
    let mut weekday_0: date::Weekday = std::option::Option::unwrap(option_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1344() {
//    rusty_monitor::set_test_id(1344);
    let mut u64_0: u64 = 400u64;
    let mut u64_0_ref_0: &u64 = &mut u64_0;
    let mut str_0: &str = "every {} month";
    let mut u64_1: u64 = 9u64;
    let mut duration_0: date::Duration = crate::date::Duration::Year(u64_1);
    let mut u64_2: u64 = 59u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut u64_3: u64 = 5800u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_3};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut u64_4: u64 = 334u64;
    let mut u64_5: u64 = 4310u64;
    let mut u64_6: u64 = 2644u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_6, month: u64_5, day: u64_4};
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Date(simpledate_0);
    let mut vec_0: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_7: u64 = 6u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_7, on: vec_0};
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Week(weekdelta_0);
    let mut repend_2: date::RepEnd = crate::date::RepEnd::Never;
    let mut u64_8: u64 = 5494u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_8};
    let mut repdelta_2: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut repetition_1: crate::date::Repetition = crate::date::Repetition {delta: repdelta_1, end: repend_1};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut u64_9: u64 = 28u64;
    let mut duration_1: date::Duration = crate::date::Duration::Year(u64_2);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_10: u64 = 29u64;
    let mut u64_11: u64 = 3100u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_9, month: u64_11, day: u64_10};
    let mut u64_12: u64 = 28u64;
    let mut str_1: &str = "Date";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1_ref_0: &str = &mut str_1;
    let mut u64_12_ref_0: &u64 = &mut u64_12;
    let mut str_2: &str = crate::date::suffix_for_day(u64_0_ref_0);
    let mut result_0: std::result::Result<std::vec::Vec<date::Weekday>, std::boxed::Box<dyn std::error::Error>> = crate::date::WeekDelta::parse_days(str_0_ref_0);
    let mut result_1: std::result::Result<date::RepEnd, std::boxed::Box<dyn std::error::Error>> = crate::date::RepEnd::parse_date(str_2);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_105() {
//    rusty_monitor::set_test_id(105);
    let mut str_0: &str = "Yvssi";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "{} day";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "fAsM";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut u64_0: u64 = 3235u64;
    let mut str_3: &str = "eds";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut vec_0: std::vec::Vec<std::string::String> = std::vec::Vec::new();
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_1: u64 = 304u64;
    let mut duration_0: date::Duration = crate::date::Duration::Year(u64_1);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut u64_2: u64 = 710u64;
    let mut u64_3: u64 = 30u64;
    let mut u64_4: u64 = 11u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_4, u64_3, u64_2);
    let mut i64_0: i64 = 100i64;
    let mut str_4: &str = "DateError";
    let mut string_0: std::string::String = std::string::String::from(str_4);
    let mut u64_5: u64 = 21u64;
    let mut expense_0: crate::expense::Expense = crate::expense::Expense::new(u64_5, string_0, i64_0, simpledate_0, option_1, option_0, vec_0);
    let mut expense_0_ref_0: &crate::expense::Expense = &mut expense_0;
    let mut option_2: &std::option::Option<crate::date::SimpleDate> = crate::expense::Expense::get_end_date(expense_0_ref_0);
    let mut result_0: std::result::Result<date::RepEnd, std::boxed::Box<dyn std::error::Error>> = crate::date::RepEnd::parse(str_3_ref_0);
    let mut repend_0: date::RepEnd = std::result::Result::unwrap(result_0);
    let mut result_1: std::result::Result<std::vec::Vec<date::Weekday>, std::boxed::Box<dyn std::error::Error>> = crate::date::WeekDelta::parse_days(str_2_ref_0);
    let mut option_3: std::option::Option<date::Weekday> = crate::date::MonthDelta::parse_weekday(str_1_ref_0);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Tuesday;
    let mut result_2: std::result::Result<date::RepEnd, std::boxed::Box<dyn std::error::Error>> = crate::date::RepEnd::parse_count(str_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_380() {
//    rusty_monitor::set_test_id(380);
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_0: u64 = 151u64;
    let mut duration_0: date::Duration = crate::date::Duration::Month(u64_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut str_0: &str = "parse::String";
    let mut str_1: &str = "occurrence";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_2: &str = "description";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2_ref_0: &str = &mut str_2;
    let mut result_0: std::result::Result<date::RepEnd, std::boxed::Box<dyn std::error::Error>> = crate::date::RepEnd::parse_count(str_2_ref_0);
    let mut result_1: std::result::Result<date::RepEnd, std::boxed::Box<dyn std::error::Error>> = crate::date::RepEnd::parse(str_1_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5556() {
//    rusty_monitor::set_test_id(5556);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Monday;
    let mut u64_0: u64 = 4u64;
    let mut u64_1: u64 = 1u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_1, weekid: u64_0, day: weekday_0};
    let mut monthdeltaweek_0_ref_0: &crate::date::MonthDeltaWeek = &mut monthdeltaweek_0;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut vec_0: std::vec::Vec<u64> = std::vec::Vec::new();
    let mut u64_2: u64 = 120u64;
    let mut monthdeltadate_0: crate::date::MonthDeltaDate = crate::date::MonthDeltaDate {nth: u64_2, days: vec_0};
    let mut monthdelta_0: date::MonthDelta = crate::date::MonthDelta::OnDate(monthdeltadate_0);
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Month(monthdelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut str_0: &str = "cc";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "invalid month";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "{} {}";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut u64_3: u64 = 2702u64;
    let mut duration_0: date::Duration = crate::date::Duration::Year(u64_3);
    let mut u64_4: u64 = 12u64;
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut option_1: std::option::Option<date::Duration> = std::option::Option::None;
    let mut u64_5: u64 = 100u64;
    let mut u64_6: u64 = 4129u64;
    let mut u64_7: u64 = 1677u64;
    let mut weekday_1: date::Weekday = crate::date::Weekday::Tuesday;
    let mut u64_8: u64 = 1350u64;
    let mut u64_9: u64 = 12u64;
    let mut weekday_2: date::Weekday = crate::date::Weekday::Monday;
    let mut monthdeltaweek_1: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_9, weekid: u64_4, day: weekday_1};
    let mut weekday_3: date::Weekday = crate::date::Weekday::Friday;
    let mut monthdeltaweek_1_ref_0: &crate::date::MonthDeltaWeek = &mut monthdeltaweek_1;
    let mut str_3: &str = crate::date::MonthDeltaWeek::weekid_to_str(monthdeltaweek_0_ref_0);
    let mut repetition_1: crate::date::Repetition = std::option::Option::unwrap(option_0);
    let mut monthdeltaweek_2: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_7, weekid: u64_5, day: weekday_2};
    let mut u64_10: u64 = crate::date::days_in_month(u64_8, u64_6);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2166() {
//    rusty_monitor::set_test_id(2166);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Sunday;
    let mut u64_0: u64 = 334u64;
    let mut u64_1: u64 = 7822u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_1, weekid: u64_0, day: weekday_0};
    let mut monthdeltaweek_0_ref_0: &crate::date::MonthDeltaWeek = &mut monthdeltaweek_0;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Never;
    let mut u64_2: u64 = 2076u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_2};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut str_0: &str = "QG";
    let mut u64_3: u64 = 4u64;
    let mut u64_4: u64 = 1u64;
    let mut u64_5: u64 = 1700u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_5, month: u64_4, day: u64_3};
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut str_1: &str = "every {} month";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "init";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut u64_6: u64 = 2400u64;
    let mut u64_7: u64 = 31u64;
    let mut u64_8: u64 = 9606u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate::from_ymd(u64_8, u64_7, u64_6);
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut str_3: &str = "weekid";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut u64_9: u64 = 4714u64;
    let mut str_4: &str = "show";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_4_ref_0: &str = &mut str_4;
    let mut result_0: std::result::Result<crate::date::DayDelta, std::boxed::Box<dyn std::error::Error>> = crate::date::DayDelta::parse(str_1_ref_0);
    let mut result_1: std::result::Result<crate::date::YearDelta, std::boxed::Box<dyn std::error::Error>> = crate::date::YearDelta::parse(str_4_ref_0);
    let mut daydelta_0: crate::date::DayDelta = std::result::Result::unwrap(result_0);
    let mut yeardelta_1: crate::date::YearDelta = crate::date::YearDelta {nth: u64_9};
    let mut result_2: std::result::Result<std::vec::Vec<date::Weekday>, std::boxed::Box<dyn std::error::Error>> = crate::date::WeekDelta::parse_days(str_0_ref_0);
    let mut result_3: std::result::Result<crate::date::YearDelta, std::boxed::Box<dyn std::error::Error>> = crate::date::YearDelta::parse(str_3_ref_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut str_5: &str = crate::date::MonthDeltaWeek::weekid_to_str(monthdeltaweek_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6759() {
//    rusty_monitor::set_test_id(6759);
    let mut str_0: &str = "occurrences";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "Never";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "the ID of the expense or income to show";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut u64_0: u64 = 334u64;
    let mut duration_0: date::Duration = crate::date::Duration::Day(u64_0);
    let mut u64_1: u64 = 3540u64;
    let mut duration_1: date::Duration = crate::date::Duration::Week(u64_1);
    let mut str_3: &str = "52s3KPWFtIQUk3fNdre";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "tzsQhT4ur46JQpCJC4";
    let mut str_5: &str = "NvqlswKuu";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "Week";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7: &str = "tag";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_8: &str = r"(\d+)";
    let mut str_9: &str = "YXjQu5";
    let mut str_9_ref_0: &str = &mut str_9;
    let mut str_10: &str = "fourth";
    let mut str_10_ref_0: &str = &mut str_10;
    let mut str_11: &str = "the ID of the expense or income to show";
    let mut str_11_ref_0: &str = &mut str_11;
    let mut u64_2: u64 = 28u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_2};
    let mut u64_3: u64 = 181u64;
    let mut u64_4: u64 = 6u64;
    let mut u64_5: u64 = 3807u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_5, month: u64_4, day: u64_3};
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut u64_6: u64 = 4u64;
    let mut u64_6_ref_0: &u64 = &mut u64_6;
    let mut str_12: &str = "ldDPZ8SLk";
    let mut u64_7: u64 = 243u64;
    let mut u64_8: u64 = 100u64;
    let mut u64_9: u64 = 7690u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_9, month: u64_8, day: u64_7};
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Date(simpledate_1);
    let mut u64_10: u64 = 7986u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_10};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Never;
    let mut u64_11: u64 = 5494u64;
    let mut daydelta_1: crate::date::DayDelta = crate::date::DayDelta {nth: u64_11};
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Day(daydelta_1);
    let mut str_13: &str = "aXEKj6NjwGr5g5";
    let mut str_13_ref_0: &str = &mut str_13;
    let mut str_8_ref_0: &str = &mut str_8;
    let mut result_0: std::result::Result<crate::date::YearDelta, std::boxed::Box<dyn std::error::Error>> = crate::date::YearDelta::parse(str_11_ref_0);
    let mut result_1: std::result::Result<crate::date::YearDelta, std::boxed::Box<dyn std::error::Error>> = crate::date::YearDelta::parse(str_10_ref_0);
    let mut result_2: std::result::Result<crate::date::YearDelta, std::boxed::Box<dyn std::error::Error>> = crate::date::YearDelta::parse(str_9_ref_0);
    let mut result_3: std::result::Result<crate::date::YearDelta, std::boxed::Box<dyn std::error::Error>> = crate::date::YearDelta::parse(str_5_ref_0);
    let mut result_4: std::result::Result<crate::date::YearDelta, std::boxed::Box<dyn std::error::Error>> = crate::date::YearDelta::parse(str_8_ref_0);
    let mut result_5: std::result::Result<crate::date::YearDelta, std::boxed::Box<dyn std::error::Error>> = crate::date::YearDelta::parse(str_3_ref_0);
    let mut result_6: std::result::Result<crate::date::YearDelta, std::boxed::Box<dyn std::error::Error>> = crate::date::YearDelta::parse(str_6_ref_0);
    let mut option_0: std::option::Option<date::Duration> = std::option::Option::Some(duration_1);
    let mut str_14: &str = "Date";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_15: &str = "ExpenseError";
    let mut str_14_ref_0: &str = &mut str_14;
    let mut str_16: &str = crate::date::suffix_for_day(u64_6_ref_0);
    let mut result_7: std::result::Result<std::vec::Vec<date::Weekday>, std::boxed::Box<dyn std::error::Error>> = crate::date::WeekDelta::parse_days(str_13_ref_0);
    let mut result_8: std::result::Result<std::vec::Vec<date::Weekday>, std::boxed::Box<dyn std::error::Error>> = crate::date::WeekDelta::parse_days(str_14_ref_0);
    let mut u64_12: u64 = 334u64;
    let mut str_17: &str = "k7AqtF";
    let mut option_0: std::option::Option<date::Duration> = std::option::Option::Some(duration_0);
    let mut str_14: &str = "Date";
    let mut str_4_ref_0: &str = &mut str_12;
    let mut str_15: &str = "ExpenseError";
    let mut str_14_ref_0: &str = &mut str_17;
    let mut u64_12_ref_0: &u64 = &mut u64_12;
    let mut result_7: std::result::Result<std::vec::Vec<date::Weekday>, std::boxed::Box<dyn std::error::Error>> = crate::date::WeekDelta::parse_days(str_4_ref_0);
    let mut str_18: &str = "week";
    let mut str_15_ref_0: &str = &mut str_15;
    let mut str_19: &str = "sPdzngvsLKmF";
    let mut str_18_ref_0: &str = &mut str_18;
    let mut str_19_ref_0: &str = &mut str_19;
    let mut result_9: std::result::Result<date::RepEnd, std::boxed::Box<dyn std::error::Error>> = crate::date::RepEnd::parse_count(str_18_ref_0);
    let mut result_10: std::result::Result<date::RepEnd, std::boxed::Box<dyn std::error::Error>> = crate::date::RepEnd::parse_date(str_7_ref_0);
    let mut result_11: std::result::Result<date::MonthDelta, std::boxed::Box<dyn std::error::Error>> = crate::date::MonthDelta::parse(str_15_ref_0, simpledate_0_ref_0);
    let mut repend_2: date::RepEnd = crate::date::RepEnd::Never;
    let mut option_1: std::option::Option<date::Weekday> = crate::date::MonthDelta::parse_weekday(str_19_ref_0);
    let mut result_12: std::result::Result<crate::date::DayDelta, std::boxed::Box<dyn std::error::Error>> = crate::date::DayDelta::parse(str_2_ref_0);
    let mut option_2: std::option::Option<u64> = crate::date::MonthDelta::parse_nth(str_1_ref_0);
    let mut result_13: std::result::Result<date::RepEnd, std::boxed::Box<dyn std::error::Error>> = crate::date::RepEnd::parse(str_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3281() {
//    rusty_monitor::set_test_id(3281);
    let mut str_0: &str = "the name of the tag to remove";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "every {} year";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "Gp9";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "start";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "ko";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "jsO62";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "month";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7: &str = "WJx";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut result_0: std::result::Result<crate::date::YearDelta, std::boxed::Box<dyn std::error::Error>> = crate::date::YearDelta::parse(str_7_ref_0);
    let mut result_1: std::result::Result<crate::date::YearDelta, std::boxed::Box<dyn std::error::Error>> = crate::date::YearDelta::parse(str_6_ref_0);
    let mut result_2: std::result::Result<crate::date::YearDelta, std::boxed::Box<dyn std::error::Error>> = crate::date::YearDelta::parse(str_5_ref_0);
    let mut result_3: std::result::Result<crate::date::YearDelta, std::boxed::Box<dyn std::error::Error>> = crate::date::YearDelta::parse(str_4_ref_0);
    let mut result_4: std::result::Result<crate::date::YearDelta, std::boxed::Box<dyn std::error::Error>> = crate::date::YearDelta::parse(str_3_ref_0);
    let mut result_5: std::result::Result<crate::date::YearDelta, std::boxed::Box<dyn std::error::Error>> = crate::date::YearDelta::parse(str_2_ref_0);
    let mut result_6: std::result::Result<crate::date::YearDelta, std::boxed::Box<dyn std::error::Error>> = crate::date::YearDelta::parse(str_1_ref_0);
    let mut result_7: std::result::Result<crate::date::YearDelta, std::boxed::Box<dyn std::error::Error>> = crate::date::YearDelta::parse(str_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6238() {
//    rusty_monitor::set_test_id(6238);
    let mut weekday_0: date::Weekday = crate::date::Weekday::Saturday;
    let mut u64_0: u64 = 7u64;
    let mut repend_0: date::RepEnd = crate::date::RepEnd::Count(u64_0);
    let mut u64_1: u64 = 1050u64;
    let mut yeardelta_0: crate::date::YearDelta = crate::date::YearDelta {nth: u64_1};
    let mut repdelta_0: date::RepDelta = crate::date::RepDelta::Year(yeardelta_0);
    let mut repetition_0: crate::date::Repetition = crate::date::Repetition {delta: repdelta_0, end: repend_0};
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::Some(repetition_0);
    let mut weekday_1: date::Weekday = crate::date::Weekday::Thursday;
    let mut u64_2: u64 = 0u64;
    let mut u64_3: u64 = 120u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_3, weekid: u64_2, day: weekday_1};
    let mut monthdeltaweek_0_ref_0: &crate::date::MonthDeltaWeek = &mut monthdeltaweek_0;
    let mut weekday_2: date::Weekday = crate::date::Weekday::Saturday;
    let mut u64_4: u64 = 9u64;
    let mut u64_5: u64 = 181u64;
    let mut monthdeltaweek_1: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_5, weekid: u64_4, day: weekday_2};
    let mut repend_1: date::RepEnd = crate::date::RepEnd::Never;
    let mut u64_6: u64 = 5494u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_6};
    let mut repdelta_1: date::RepDelta = crate::date::RepDelta::Day(daydelta_0);
    let mut u64_7: u64 = 334u64;
    let mut str_0: &str = "aXEKj6NjwGr5g5";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u64_8: u64 = 243u64;
    let mut u64_9: u64 = 22u64;
    let mut u64_10: u64 = 120u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_10, month: u64_9, day: u64_8};
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut str_1: &str = "start";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut u64_11: u64 = 5399u64;
    let mut u64_12: u64 = 893u64;
    let mut u64_13: u64 = 28u64;
    let mut u64_14: u64 = 2755u64;
    let mut vec_0: std::vec::Vec<date::Weekday> = std::vec::Vec::new();
    let mut u64_15: u64 = 23u64;
    let mut weekdelta_0: crate::date::WeekDelta = crate::date::WeekDelta {nth: u64_15, on: vec_0};
    let mut yeardelta_1: crate::date::YearDelta = crate::date::YearDelta {nth: u64_14};
    let mut monthdeltaweek_1_ref_0: &crate::date::MonthDeltaWeek = &mut monthdeltaweek_1;
    let mut str_2: &str = crate::date::MonthDeltaWeek::weekid_to_str(monthdeltaweek_0_ref_0);
    let mut repetition_1: crate::date::Repetition = std::option::Option::unwrap(option_0);
    let mut monthdeltaweek_2: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_7, weekid: u64_13, day: weekday_0};
    let mut u64_16: u64 = crate::date::days_in_month(u64_12, u64_11);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5376() {
//    rusty_monitor::set_test_id(5376);
    let mut option_0: std::option::Option<crate::date::Repetition> = std::option::Option::None;
    let mut u64_0: u64 = 1902u64;
    let mut u64_1: u64 = 3u64;
    let mut u64_2: u64 = 1700u64;
    let mut simpledate_0: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_2, month: u64_1, day: u64_0};
    let mut simpledate_0_ref_0: &crate::date::SimpleDate = &mut simpledate_0;
    let mut weekday_0: date::Weekday = crate::date::get_weekday_of_date(simpledate_0_ref_0);
    let mut u64_3: u64 = 7649u64;
    let mut u64_4: u64 = 12u64;
    let mut monthdeltaweek_0: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_4, weekid: u64_3, day: weekday_0};
    let mut monthdeltaweek_0_ref_0: &crate::date::MonthDeltaWeek = &mut monthdeltaweek_0;
    let mut u64_5: u64 = 5161u64;
    let mut str_0: &str = "YXjQu5";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "fourth";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "the ID of the expense or income to show";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut u64_6: u64 = 28u64;
    let mut daydelta_0: crate::date::DayDelta = crate::date::DayDelta {nth: u64_6};
    let mut u64_7: u64 = 181u64;
    let mut u64_8: u64 = 6u64;
    let mut u64_9: u64 = 3807u64;
    let mut simpledate_1: crate::date::SimpleDate = crate::date::SimpleDate {year: u64_9, month: u64_8, day: u64_7};
    let mut simpledate_1_ref_0: &crate::date::SimpleDate = &mut simpledate_1;
    let mut u64_10: u64 = 4u64;
    let mut u64_10_ref_0: &u64 = &mut u64_10;
    let mut u64_11: u64 = 1677u64;
    let mut weekday_1: date::Weekday = crate::date::Weekday::Tuesday;
    let mut u64_12: u64 = 1350u64;
    let mut u64_13: u64 = 12u64;
    let mut weekday_2: date::Weekday = crate::date::Weekday::Monday;
    let mut monthdeltaweek_1: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_13, weekid: u64_12, day: weekday_1};
    let mut weekday_3: date::Weekday = crate::date::Weekday::Friday;
    let mut monthdeltaweek_1_ref_0: &crate::date::MonthDeltaWeek = &mut monthdeltaweek_1;
    let mut str_3: &str = crate::date::MonthDeltaWeek::weekid_to_str(monthdeltaweek_0_ref_0);
    let mut repetition_0: crate::date::Repetition = std::option::Option::unwrap(option_0);
    let mut monthdeltaweek_2: crate::date::MonthDeltaWeek = crate::date::MonthDeltaWeek {nth: u64_11, weekid: u64_5, day: weekday_2};
//    panic!("From RustyUnit with love");
}
}