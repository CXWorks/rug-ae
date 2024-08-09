//! This module defines the [TotalSize] flag. To set it up from [ArgMatches], a [Config] and its
//! [Default] value, use the [configure_from](Configurable::configure_from) method.

use super::Configurable;

use crate::config_file::Config;

use clap::ArgMatches;

/// The flag showing whether to show the total size for directories.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub struct TotalSize(pub bool);

impl Configurable<Self> for TotalSize {
    /// Get a potential `TotalSize` value from [ArgMatches].
    ///
    /// If the "total-size" argument is passed, this returns a `TotalSize` with value `true` in a
    /// [Some]. Otherwise this returns [None].
    fn from_arg_matches(matches: &ArgMatches) -> Option<Self> {
        if matches.is_present("total-size") {
            Some(Self(true))
        } else {
            None
        }
    }

    /// Get a potential `TotalSize` value from a [Config].
    ///
    /// If the `Config::total-size` has value,
    /// this returns it as the value of the `TotalSize`, in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        config.total_size.map(Self)
    }
}

#[cfg(test)]
mod test {
    use super::TotalSize;

    use crate::app;
    use crate::config_file::Config;
    use crate::flags::Configurable;

    #[test]
    fn test_from_arg_matches_none() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(None, TotalSize::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_true() {
        let argv = vec!["lsd", "--total-size"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(Some(TotalSize(true)), TotalSize::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, TotalSize::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_true() {
        let mut c = Config::with_none();
        c.total_size = Some(true);
        assert_eq!(Some(TotalSize(true)), TotalSize::from_config(&c));
    }

    #[test]
    fn test_from_config_false() {
        let mut c = Config::with_none();
        c.total_size = Some(false);
        assert_eq!(Some(TotalSize(false)), TotalSize::from_config(&c));
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::default::Default;
	use std::clone::Clone;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2496() {
    rusty_monitor::set_test_id(2496);
    let mut bool_0: bool = true;
    let mut option_0: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut option_1: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::Some(dirgrouping_0);
    let mut option_2: std::option::Option<bool> = std::option::Option::None;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut option_3: std::option::Option<flags::sorting::SortColumn> = std::option::Option::Some(sortcolumn_0);
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_3, reverse: option_2, dir_grouping: option_1};
    let mut option_4: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_0);
    let mut option_5: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut option_6: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_0);
    let mut option_7: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut option_8: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_0);
    let mut option_9: std::option::Option<bool> = std::option::Option::None;
    let mut option_10: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_11: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_12: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_13: std::option::Option<bool> = std::option::Option::None;
    let mut option_14: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_1: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_1};
    let mut str_0: &str = "LcQ4B";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut bool_2: bool = true;
    let mut bool_3: bool = false;
    let mut bool_4: bool = true;
    let mut bool_5: bool = true;
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = true;
    let mut bool_10: bool = true;
    let mut bool_11: bool = false;
    let mut bool_12: bool = false;
    let mut bool_13: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_13, user_write: bool_12, user_execute: bool_11, group_read: bool_10, group_write: bool_9, group_execute: bool_8, other_read: bool_7, other_write: bool_6, other_execute: bool_5, sticky: bool_4, setgid: bool_3, setuid: bool_2};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_15: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut option_16: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_0);
    let mut option_17: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_17, theme: option_16};
    let mut option_18: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_19: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_20: std::option::Option<bool> = std::option::Option::None;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_1};
    let mut totalsize_0: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_0_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_0;
    let mut elem_0: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_1: color::Elem = crate::color::Elem::Acl;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_2: color::Elem = crate::color::Elem::Write;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut bool_14: bool = false;
    let mut elem_3: color::Elem = crate::color::Elem::Links {valid: bool_14};
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_1_ref_0);
    let mut elem_4: color::Elem = crate::color::Elem::ExecSticky;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut elem_5: color::Elem = crate::color::Elem::HourOld;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut elem_6: color::Elem = crate::color::Elem::Write;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut elem_7: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut elem_8: color::Elem = crate::color::Elem::BlockDevice;
    let mut totalsize_1: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::clone(totalsize_0_ref_0);
    panic!("From RustyUnit with love");
}
}