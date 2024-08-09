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
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5038() {
    rusty_monitor::set_test_id(5038);
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut bool_5: bool = false;
    let mut bool_6: bool = false;
    let mut bool_7: bool = false;
    let mut bool_8: bool = false;
    let mut bool_9: bool = false;
    let mut bool_10: bool = true;
    let mut bool_11: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_0: usize = 92usize;
    let mut bool_12: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_12, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut u64_0: u64 = 85u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut usize_1: usize = 81usize;
    let mut option_1: std::option::Option<usize> = std::option::Option::Some(usize_1);
    let mut option_2: std::option::Option<bool> = std::option::Option::None;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_1_ref_0: &meta::date::Date = &mut date_1;
    let mut bool_13: bool = crate::meta::date::Date::ne(date_1_ref_0, date_0_ref_0);
    let mut recursion_1: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_2, depth: option_1};
    let mut recursion_1_ref_0: &crate::config_file::Recursion = &mut recursion_1;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Name;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4217() {
    rusty_monitor::set_test_id(4217);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_0: usize = 87usize;
    let mut bool_0: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_1_ref_0: &meta::date::Date = &mut date_1;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_1, order: sortorder_1, dir_grouping: dirgrouping_1};
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_1: usize = 48usize;
    let mut bool_3: bool = true;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_3, depth: usize_1};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_1: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_2};
    let mut config_2: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut bool_4: bool = true;
    let mut bool_5: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_5, exec: bool_4};
    let mut dirgrouping_2: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_2: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut themeoption_3_ref_0: &flags::color::ThemeOption = &mut themeoption_3;
    let mut theme_0: icon::Theme = crate::icon::Theme::NoIcon;
    let mut theme_0_ref_0: &icon::Theme = &mut theme_0;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Kilo;
    let mut sortcolumn_2: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::INode;
    let mut sortcolumn_3: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut date_2: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut sortcolumn_4: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::None;
    let mut unit_2: meta::size::Unit = crate::meta::size::Unit::Giga;
    let mut bool_6: bool = crate::meta::filetype::FileType::is_dirlike(filetype_0);
    let mut unit_3: meta::size::Unit = crate::meta::size::Unit::Tera;
    let mut elem_0: color::Elem = crate::color::Elem::Links {valid: bool_2};
    let mut hyperlinkoption_2: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_1};
    let mut date_2_ref_0: &meta::date::Date = &mut date_2;
    let mut option_0: std::option::Option<std::cmp::Ordering> = crate::meta::date::Date::partial_cmp(date_2_ref_0, date_1_ref_0);
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut elem_1: color::Elem = crate::color::Elem::Group;
    let mut unit_2_ref_0: &meta::size::Unit = &mut unit_2;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_328() {
    rusty_monitor::set_test_id(328);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_1_ref_0: &meta::date::Date = &mut date_1;
    let mut bool_0: bool = false;
    let mut date_2: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_2_ref_0: &meta::date::Date = &mut date_2;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut filetype_0_ref_0: &meta::filetype::FileType = &mut filetype_0;
    let mut bool_1: bool = true;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::DayOld;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Read;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::Group;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut date_3: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_3_ref_0: &meta::date::Date = &mut date_3;
    let mut date_4: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_4_ref_0: &meta::date::Date = &mut date_4;
    let mut bool_2: bool = false;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_2};
    let mut ordering_0: std::cmp::Ordering = crate::meta::date::Date::cmp(date_4_ref_0, date_3_ref_0);
    let mut date_5: crate::color::theme::Date = crate::color::theme::Date {hour_old: color_2, day_old: color_1, older: color_0};
    let mut elem_3: color::Elem = crate::color::Elem::INode {valid: bool_1};
    let mut displayoption_0: meta::name::DisplayOption = crate::meta::name::DisplayOption::FileName;
    let mut filetype_1_ref_0: &meta::filetype::FileType = &mut filetype_1;
    let mut date_6: meta::date::Date = crate::meta::date::Date::clone(date_2_ref_0);
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_0};
    let mut bool_3: bool = crate::meta::date::Date::eq(date_1_ref_0, date_0_ref_0);
    let mut date_6_ref_0: &meta::date::Date = &mut date_6;
    let mut tuple_0: () = crate::meta::date::Date::assert_receiver_is_total_eq(date_6_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3360() {
    rusty_monitor::set_test_id(3360);
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_1, order: sortorder_1, dir_grouping: dirgrouping_1};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_0: usize = 42usize;
    let mut bool_0: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut str_0: &str = "7X6PJSMPwfwDk";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_1: bool = false;
    let mut bool_2: bool = false;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_2, exec: bool_1};
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut elem_0: color::Elem = crate::color::Elem::Acl;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut date_1: meta::date::Date = crate::meta::date::Date::clone(date_0_ref_0);
    let mut bool_3: bool = crate::color::Elem::has_suid(elem_0_ref_0);
    let mut date_1_ref_0: &meta::date::Date = &mut date_1;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    panic!("From RustyUnit with love");
}
}