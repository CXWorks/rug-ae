use crate::color::{ColoredString, Colors, Elem};
use crate::flags::{DateFlag, Flags};
use chrono::{DateTime, Duration, Local};
use chrono_humanize::HumanTime;
use std::fs::Metadata;
use std::panic;
use std::time::SystemTime;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Date {
    Date(DateTime<Local>),
    Invalid,
}

// Note that this is split from the From for Metadata so we can test this one (as we can't mock Metadata)
impl<'a> From<SystemTime> for Date {
    fn from(systime: SystemTime) -> Self {
        // FIXME: This should really involve a result, but there's upstream issues in chrono. See https://github.com/chronotope/chrono/issues/110
        let res = panic::catch_unwind(|| systime.into());

        if let Ok(time) = res {
            Date::Date(time)
        } else {
            Date::Invalid
        }
    }
}

impl<'a> From<&'a Metadata> for Date {
    fn from(meta: &'a Metadata) -> Self {
        meta.modified()
            .expect("failed to retrieve modified date")
            .into()
    }
}

impl Date {
    pub fn render(&self, colors: &Colors, flags: &Flags) -> ColoredString {
        let now = Local::now();
        let elem = if let Date::Date(val) = self {
            if *val > now - Duration::hours(1) {
                Elem::HourOld
            } else if *val > now - Duration::days(1) {
                Elem::DayOld
            } else {
                Elem::Older
            }
        } else {
            Elem::Older
        };
        colors.colorize(self.date_string(flags), &elem)
    }

    pub fn date_string(&self, flags: &Flags) -> String {
        if let Date::Date(val) = self {
            match &flags.date {
                DateFlag::Date => val.format("%c").to_string(),
                DateFlag::Relative => format!("{}", HumanTime::from(*val - Local::now())),
                DateFlag::Iso => {
                    // 365.2425 * 24 * 60 * 60 = 31556952 seconds per year
                    // 15778476 seconds are 6 months
                    if *val > Local::now() - Duration::seconds(15_778_476) {
                        val.format("%m-%d %R").to_string()
                    } else {
                        val.format("%F").to_string()
                    }
                }
                DateFlag::Formatted(format) => val.format(format).to_string(),
            }
        } else {
            String::from("-")
        }
    }
}

#[cfg(test)]
mod test {
    use super::Date;
    use crate::color::{Colors, ThemeOption};
    use crate::flags::{DateFlag, Flags};
    use chrono::{DateTime, Duration, Local};
    use crossterm::style::{Color, Stylize};
    use std::io;
    use std::path::Path;
    use std::process::{Command, ExitStatus};
    use std::{env, fs};

    #[cfg(unix)]
    fn cross_platform_touch(path: &Path, date: &DateTime<Local>) -> io::Result<ExitStatus> {
        Command::new("touch")
            .arg("-t")
            .arg(date.format("%Y%m%d%H%M.%S").to_string())
            .arg(&path)
            .status()
    }

    #[cfg(windows)]
    fn cross_platform_touch(path: &Path, date: &DateTime<Local>) -> io::Result<ExitStatus> {
        use std::process::Stdio;

        let copy_success = Command::new("cmd")
            .arg("/C")
            .arg("copy")
            .arg("NUL")
            .arg(path)
            .stdout(Stdio::null()) // Windows doesn't have a quiet flag
            .status()?
            .success();

        assert!(copy_success, "failed to create empty file");

        Command::new("powershell")
            .arg("-Command")
            .arg(format!(
                r#"$(Get-Item {}).lastwritetime=$(Get-Date "{}")"#,
                path.display(),
                date.to_rfc3339()
            ))
            .status()
    }

    #[test]
    fn test_an_hour_old_file_color() {
        let mut file_path = env::temp_dir();
        file_path.push("test_an_hour_old_file_color.tmp");

        let creation_date = Local::now() - chrono::Duration::seconds(4);

        let success = cross_platform_touch(&file_path, &creation_date)
            .unwrap()
            .success();
        assert!(success, "failed to exec touch");

        let colors = Colors::new(ThemeOption::Default);
        let date = Date::from(&file_path.metadata().unwrap());
        let flags = Flags::default();

        assert_eq!(
            creation_date
                .format("%c")
                .to_string()
                .with(Color::AnsiValue(40)),
            date.render(&colors, &flags)
        );

        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_a_day_old_file_color() {
        let mut file_path = env::temp_dir();
        file_path.push("test_a_day_old_file_color.tmp");

        let creation_date = Local::now() - chrono::Duration::hours(4);

        let success = cross_platform_touch(&file_path, &creation_date)
            .unwrap()
            .success();
        assert!(success, "failed to exec touch");

        let colors = Colors::new(ThemeOption::Default);
        let date = Date::from(&file_path.metadata().unwrap());
        let flags = Flags::default();

        assert_eq!(
            creation_date
                .format("%c")
                .to_string()
                .with(Color::AnsiValue(42)),
            date.render(&colors, &flags)
        );

        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_a_several_days_old_file_color() {
        let mut file_path = env::temp_dir();
        file_path.push("test_a_several_days_old_file_color.tmp");

        let creation_date = Local::now() - chrono::Duration::days(2);

        let success = cross_platform_touch(&file_path, &creation_date)
            .unwrap()
            .success();
        assert!(success, "failed to exec touch");

        let colors = Colors::new(ThemeOption::Default);
        let date = Date::from(&file_path.metadata().unwrap());
        let flags = Flags::default();

        assert_eq!(
            creation_date
                .format("%c")
                .to_string()
                .with(Color::AnsiValue(36)),
            date.render(&colors, &flags)
        );

        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_with_relative_date() {
        let mut file_path = env::temp_dir();
        file_path.push("test_with_relative_date.tmp");

        let creation_date = Local::now() - chrono::Duration::days(2);

        let success = cross_platform_touch(&file_path, &creation_date)
            .unwrap()
            .success();
        assert!(success, "failed to exec touch");

        let colors = Colors::new(ThemeOption::Default);
        let date = Date::from(&file_path.metadata().unwrap());

        let mut flags = Flags::default();
        flags.date = DateFlag::Relative;

        assert_eq!(
            "2 days ago".to_string().with(Color::AnsiValue(36)),
            date.render(&colors, &flags)
        );

        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_with_relative_date_now() {
        let mut file_path = env::temp_dir();
        file_path.push("test_with_relative_date_now.tmp");

        let creation_date = Local::now();
        let success = cross_platform_touch(&file_path, &creation_date)
            .unwrap()
            .success();
        assert_eq!(true, success, "failed to exec touch");

        let colors = Colors::new(ThemeOption::Default);
        let date = Date::from(&file_path.metadata().unwrap());

        let mut flags = Flags::default();
        flags.date = DateFlag::Relative;

        assert_eq!(
            "now".to_string().with(Color::AnsiValue(40)),
            date.render(&colors, &flags)
        );

        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_iso_format_now() {
        let mut file_path = env::temp_dir();
        file_path.push("test_iso_format_now.tmp");

        let creation_date = Local::now();
        let success = cross_platform_touch(&file_path, &creation_date)
            .unwrap()
            .success();
        assert_eq!(true, success, "failed to exec touch");

        let colors = Colors::new(ThemeOption::Default);
        let date = Date::from(&file_path.metadata().unwrap());

        let mut flags = Flags::default();
        flags.date = DateFlag::Iso;

        assert_eq!(
            creation_date
                .format("%m-%d %R")
                .to_string()
                .with(Color::AnsiValue(40)),
            date.render(&colors, &flags)
        );

        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_iso_format_year_old() {
        let mut file_path = env::temp_dir();
        file_path.push("test_iso_format_year_old.tmp");

        let creation_date = Local::now() - Duration::days(400);
        let success = cross_platform_touch(&file_path, &creation_date)
            .unwrap()
            .success();
        assert_eq!(true, success, "failed to exec touch");

        let colors = Colors::new(ThemeOption::Default);
        let date = Date::from(&file_path.metadata().unwrap());

        let mut flags = Flags::default();
        flags.date = DateFlag::Iso;

        assert_eq!(
            creation_date
                .format("%F")
                .to_string()
                .with(Color::AnsiValue(36)),
            date.render(&colors, &flags)
        );

        fs::remove_file(file_path).unwrap();
    }

    #[test]
    #[cfg(all(not(windows), target_arch = "x86_64"))]
    fn test_bad_date() {
        // 4437052 is the bad year taken from https://github.com/Peltoche/lsd/issues/529 that we know is both
        // a) high enough to break chrono
        // b) not high enough to break SystemTime (as Duration::MAX would)
        let end_time = std::time::SystemTime::UNIX_EPOCH
            + std::time::Duration::new(4437052 * 365 * 24 * 60 * 60, 0);
        let colors = Colors::new(ThemeOption::Default);
        let date = Date::from(end_time);

        let mut flags = Flags::default();
        flags.date = DateFlag::Date;

        assert_eq!(
            "-".to_string().with(Color::AnsiValue(36)),
            date.render(&colors, &flags)
        );
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::cmp::Ord;
	use std::cmp::PartialEq;
	use std::clone::Clone;
	use std::cmp::PartialOrd;
	use std::cmp::Eq;
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7580() {
//    rusty_monitor::set_test_id(7580);
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_1_ref_0: &meta::date::Date = &mut date_1;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut bool_0: bool = crate::meta::date::Date::eq(date_1_ref_0, date_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_9194() {
//    rusty_monitor::set_test_id(9194);
    let mut elem_0: color::Elem = crate::color::Elem::Group;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut filetype_0_ref_0: &meta::filetype::FileType = &mut filetype_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut filetype_1_ref_0: &meta::filetype::FileType = &mut filetype_1;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut filetype_2_ref_0: &meta::filetype::FileType = &mut filetype_2;
    let mut option_0: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut bool_0: bool = false;
    let mut option_1: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut option_2: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_3: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut elem_1: color::Elem = crate::color::Elem::Older;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut option_4: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_1);
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut date_1_ref_0: &meta::date::Date = &mut date_1;
    let mut date_2: meta::date::Date = crate::meta::date::Date::clone(date_1_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_507() {
//    rusty_monitor::set_test_id(507);
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_1_ref_0: &meta::date::Date = &mut date_1;
    let mut date_2: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_2_ref_0: &meta::date::Date = &mut date_2;
    let mut date_3: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_3_ref_0: &meta::date::Date = &mut date_3;
    let mut date_4: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_4_ref_0: &meta::date::Date = &mut date_4;
    let mut date_5: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_5_ref_0: &meta::date::Date = &mut date_5;
    let mut date_6: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_6_ref_0: &meta::date::Date = &mut date_6;
    let mut date_7: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_7_ref_0: &meta::date::Date = &mut date_7;
    let mut date_8: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_8_ref_0: &meta::date::Date = &mut date_8;
    let mut date_9: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_9_ref_0: &meta::date::Date = &mut date_9;
    let mut date_10: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_10_ref_0: &meta::date::Date = &mut date_10;
    let mut tuple_0: () = crate::meta::date::Date::assert_receiver_is_total_eq(date_10_ref_0);
    let mut tuple_1: () = crate::meta::date::Date::assert_receiver_is_total_eq(date_9_ref_0);
    let mut tuple_2: () = crate::meta::date::Date::assert_receiver_is_total_eq(date_8_ref_0);
    let mut tuple_3: () = crate::meta::date::Date::assert_receiver_is_total_eq(date_7_ref_0);
    let mut tuple_4: () = crate::meta::date::Date::assert_receiver_is_total_eq(date_6_ref_0);
    let mut tuple_5: () = crate::meta::date::Date::assert_receiver_is_total_eq(date_5_ref_0);
    let mut tuple_6: () = crate::meta::date::Date::assert_receiver_is_total_eq(date_4_ref_0);
    let mut tuple_7: () = crate::meta::date::Date::assert_receiver_is_total_eq(date_3_ref_0);
    let mut tuple_8: () = crate::meta::date::Date::assert_receiver_is_total_eq(date_2_ref_0);
    let mut tuple_9: () = crate::meta::date::Date::assert_receiver_is_total_eq(date_1_ref_0);
    let mut tuple_10: () = crate::meta::date::Date::assert_receiver_is_total_eq(date_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1627() {
//    rusty_monitor::set_test_id(1627);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_0: usize = 6usize;
    let mut bool_0: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut u64_0: u64 = 1073741824u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_1_ref_0: &meta::date::Date = &mut date_1;
    let mut date_2: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_2_ref_0: &meta::date::Date = &mut date_2;
    let mut date_3: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_3_ref_0: &meta::date::Date = &mut date_3;
    let mut date_4: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_4_ref_0: &meta::date::Date = &mut date_4;
    let mut date_5: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_5_ref_0: &meta::date::Date = &mut date_5;
    let mut date_6: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_6_ref_0: &meta::date::Date = &mut date_6;
    let mut date_7: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_7_ref_0: &meta::date::Date = &mut date_7;
    let mut date_8: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_8_ref_0: &meta::date::Date = &mut date_8;
    let mut date_9: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_9_ref_0: &meta::date::Date = &mut date_9;
    let mut date_10: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_10_ref_0: &meta::date::Date = &mut date_10;
    let mut date_11: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_11_ref_0: &meta::date::Date = &mut date_11;
    let mut date_12: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_12_ref_0: &meta::date::Date = &mut date_12;
    let mut date_13: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_13_ref_0: &meta::date::Date = &mut date_13;
    let mut ordering_0: std::cmp::Ordering = crate::meta::date::Date::cmp(date_13_ref_0, date_12_ref_0);
    let mut ordering_1: std::cmp::Ordering = crate::meta::date::Date::cmp(date_11_ref_0, date_10_ref_0);
    let mut ordering_2: std::cmp::Ordering = crate::meta::date::Date::cmp(date_9_ref_0, date_8_ref_0);
    let mut ordering_3: std::cmp::Ordering = crate::meta::date::Date::cmp(date_7_ref_0, date_6_ref_0);
    let mut ordering_4: std::cmp::Ordering = crate::meta::date::Date::cmp(date_5_ref_0, date_4_ref_0);
    let mut ordering_5: std::cmp::Ordering = crate::meta::date::Date::cmp(date_3_ref_0, date_2_ref_0);
    let mut ordering_6: std::cmp::Ordering = crate::meta::date::Date::cmp(date_1_ref_0, date_0_ref_0);
    let mut elem_0: color::Elem = crate::color::Elem::Read;
    crate::meta::filetype::FileType::render(filetype_0, colors_1_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8776() {
//    rusty_monitor::set_test_id(8776);
    let mut option_0: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut option_1: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut display_0: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut date_2: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_1_ref_0: &meta::date::Date = &mut date_1;
    let mut date_2_ref_0: &meta::date::Date = &mut date_2;
    let mut date_3: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_4: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_3_ref_0: &meta::date::Date = &mut date_3;
    let mut date_5: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_4_ref_0: &meta::date::Date = &mut date_4;
    let mut date_6: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_5_ref_0: &meta::date::Date = &mut date_5;
    let mut ordering_0: std::cmp::Ordering = crate::meta::date::Date::cmp(date_3_ref_0, date_4_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_414() {
//    rusty_monitor::set_test_id(414);
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_1_ref_0: &meta::date::Date = &mut date_1;
    let mut date_2: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_2_ref_0: &meta::date::Date = &mut date_2;
    let mut date_3: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_3_ref_0: &meta::date::Date = &mut date_3;
    let mut date_4: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_4_ref_0: &meta::date::Date = &mut date_4;
    let mut date_5: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_5_ref_0: &meta::date::Date = &mut date_5;
    let mut date_6: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_6_ref_0: &meta::date::Date = &mut date_6;
    let mut date_7: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_7_ref_0: &meta::date::Date = &mut date_7;
    let mut date_8: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_8_ref_0: &meta::date::Date = &mut date_8;
    let mut date_9: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_9_ref_0: &meta::date::Date = &mut date_9;
    let mut date_10: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_10_ref_0: &meta::date::Date = &mut date_10;
    let mut date_11: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_11_ref_0: &meta::date::Date = &mut date_11;
    let mut date_12: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_12_ref_0: &meta::date::Date = &mut date_12;
    let mut date_13: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_13_ref_0: &meta::date::Date = &mut date_13;
    let mut bool_0: bool = crate::meta::date::Date::ne(date_13_ref_0, date_12_ref_0);
    let mut bool_1: bool = crate::meta::date::Date::ne(date_11_ref_0, date_10_ref_0);
    let mut bool_2: bool = crate::meta::date::Date::ne(date_9_ref_0, date_8_ref_0);
    let mut bool_3: bool = crate::meta::date::Date::ne(date_7_ref_0, date_6_ref_0);
    let mut bool_4: bool = crate::meta::date::Date::ne(date_5_ref_0, date_4_ref_0);
    let mut bool_5: bool = crate::meta::date::Date::ne(date_3_ref_0, date_2_ref_0);
    let mut bool_6: bool = crate::meta::date::Date::ne(date_1_ref_0, date_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_405() {
//    rusty_monitor::set_test_id(405);
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_1_ref_0: &meta::date::Date = &mut date_1;
    let mut date_2: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_2_ref_0: &meta::date::Date = &mut date_2;
    let mut date_3: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_3_ref_0: &meta::date::Date = &mut date_3;
    let mut date_4: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_4_ref_0: &meta::date::Date = &mut date_4;
    let mut date_5: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_5_ref_0: &meta::date::Date = &mut date_5;
    let mut date_6: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_6_ref_0: &meta::date::Date = &mut date_6;
    let mut date_7: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_7_ref_0: &meta::date::Date = &mut date_7;
    let mut date_8: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_8_ref_0: &meta::date::Date = &mut date_8;
    let mut date_9: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_9_ref_0: &meta::date::Date = &mut date_9;
    let mut date_10: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_10_ref_0: &meta::date::Date = &mut date_10;
    let mut date_11: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_11_ref_0: &meta::date::Date = &mut date_11;
    let mut date_12: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_12_ref_0: &meta::date::Date = &mut date_12;
    let mut date_13: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_13_ref_0: &meta::date::Date = &mut date_13;
    let mut option_0: std::option::Option<std::cmp::Ordering> = crate::meta::date::Date::partial_cmp(date_13_ref_0, date_12_ref_0);
    let mut option_1: std::option::Option<std::cmp::Ordering> = crate::meta::date::Date::partial_cmp(date_11_ref_0, date_10_ref_0);
    let mut option_2: std::option::Option<std::cmp::Ordering> = crate::meta::date::Date::partial_cmp(date_9_ref_0, date_8_ref_0);
    let mut option_3: std::option::Option<std::cmp::Ordering> = crate::meta::date::Date::partial_cmp(date_7_ref_0, date_6_ref_0);
    let mut option_4: std::option::Option<std::cmp::Ordering> = crate::meta::date::Date::partial_cmp(date_5_ref_0, date_4_ref_0);
    let mut option_5: std::option::Option<std::cmp::Ordering> = crate::meta::date::Date::partial_cmp(date_3_ref_0, date_2_ref_0);
    let mut option_6: std::option::Option<std::cmp::Ordering> = crate::meta::date::Date::partial_cmp(date_1_ref_0, date_0_ref_0);
//    panic!("From RustyUnit with love");
}
}