//! This module defines the [Display] flag. To set it up from [ArgMatches], a [Config] and its
//! [Default] value, use its [configure_from](Configurable::configure_from) method.

use super::Configurable;

use crate::config_file::Config;

use clap::ArgMatches;
use serde::Deserialize;

/// The flag showing which file system nodes to display.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Display {
    All,
    AlmostAll,
    DirectoryOnly,
    VisibleOnly,
}

impl Configurable<Self> for Display {
    /// Get a potential `Display` variant from [ArgMatches].
    ///
    /// If any of the "all", "almost-all" or "directory-only" arguments is passed, this returns the
    /// corresponding `Display` variant in a [Some]. If neither of them is passed, this returns
    /// [None].
    fn from_arg_matches(matches: &ArgMatches) -> Option<Self> {
        if matches.is_present("directory-only") {
            Some(Self::DirectoryOnly)
        } else if matches.is_present("almost-all") {
            Some(Self::AlmostAll)
        } else if matches.is_present("all") {
            Some(Self::All)
        } else {
            None
        }
    }

    /// Get a potential `Display` variant from a [Config].
    ///
    /// If the `Config::display` has value and is one of
    /// "all", "almost-all", "directory-only" or `visible-only`,
    /// this returns the corresponding `Display` variant in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        config.display
    }
}

/// The default value for `Display` is [Display::VisibleOnly].
impl Default for Display {
    fn default() -> Self {
        Display::VisibleOnly
    }
}

#[cfg(test)]
mod test {
    use super::Display;

    use crate::app;
    use crate::config_file::Config;
    use crate::flags::Configurable;

    #[test]
    fn test_from_arg_matches_none() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(None, Display::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_all() {
        let argv = vec!["lsd", "--all"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(Some(Display::All), Display::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_almost_all() {
        let argv = vec!["lsd", "--almost-all"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(Display::AlmostAll),
            Display::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_directory_only() {
        let argv = vec!["lsd", "--directory-only"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(Display::DirectoryOnly),
            Display::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, Display::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_all() {
        let mut c = Config::with_none();
        c.display = Some(Display::All);
        assert_eq!(Some(Display::All), Display::from_config(&c));
    }

    #[test]
    fn test_from_config_almost_all() {
        let mut c = Config::with_none();
        c.display = Some(Display::AlmostAll);
        assert_eq!(Some(Display::AlmostAll), Display::from_config(&c));
    }

    #[test]
    fn test_from_config_directory_only() {
        let mut c = Config::with_none();
        c.display = Some(Display::DirectoryOnly);
        assert_eq!(Some(Display::DirectoryOnly), Display::from_config(&c));
    }

    #[test]
    fn test_from_config_visible_only() {
        let mut c = Config::with_none();
        c.display = Some(Display::VisibleOnly);
        assert_eq!(Some(Display::VisibleOnly), Display::from_config(&c));
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::default::Default;
	use std::cmp::PartialEq;
	use std::cmp::Eq;
	use flags::Configurable;
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_705() {
//    rusty_monitor::set_test_id(705);
    let mut str_0: &str = "4vQNZM6lqhpF";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut usize_0: usize = 2usize;
    let mut tuple_0: (usize, &str) = (usize_0, str_0_ref_0);
    let mut usize_1: usize = 6usize;
    let mut bool_0: bool = true;
    let mut str_1: &str = "recursion";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut u64_0: u64 = 0u64;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = false;
    let mut bool_5: bool = false;
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = false;
    let mut bool_10: bool = true;
    let mut bool_11: bool = true;
    let mut bool_12: bool = false;
    let mut bool_13: bool = false;
    let mut bool_14: bool = false;
    let mut bool_15: bool = true;
    let mut bool_16: bool = true;
    let mut bool_17: bool = false;
    let mut bool_18: bool = false;
    let mut bool_19: bool = false;
    let mut bool_20: bool = false;
    let mut bool_21: bool = true;
    let mut bool_22: bool = true;
    let mut bool_23: bool = true;
    let mut bool_24: bool = false;
    let mut usize_2: usize = 0usize;
    let mut bool_25: bool = false;
    let mut u64_1: u64 = 1048576u64;
    let mut bool_26: bool = true;
    let mut bool_27: bool = true;
    let mut bool_28: bool = false;
    let mut bool_29: bool = true;
    let mut bool_30: bool = true;
    let mut bool_31: bool = false;
    let mut bool_32: bool = true;
    let mut bool_33: bool = false;
    let mut bool_34: bool = false;
    let mut bool_35: bool = true;
    let mut bool_36: bool = true;
    let mut bool_37: bool = false;
    let mut u64_2: u64 = 1024u64;
    let mut bool_38: bool = true;
    let mut bool_39: bool = true;
    let mut bool_40: bool = false;
    let mut bool_41: bool = true;
    let mut bool_42: bool = false;
    let mut bool_43: bool = false;
    let mut bool_44: bool = false;
    let mut bool_45: bool = false;
    let mut bool_46: bool = true;
    let mut bool_47: bool = false;
    let mut bool_48: bool = true;
    let mut bool_49: bool = false;
    let mut usize_3: usize = 360usize;
    let mut bool_50: bool = false;
    let mut u64_3: u64 = 0u64;
    let mut usize_4: usize = 1usize;
    let mut bool_51: bool = true;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut u64_4: u64 = 59u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_4);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_5: usize = 40usize;
    let mut bool_52: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_52, depth: usize_5};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_0: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_2};
    let mut str_2: &str = "config";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bool_53: bool = true;
    let mut bool_54: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_54, exec: bool_53};
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut bool_55: bool = true;
    let mut bool_56: bool = true;
    let mut bool_57: bool = false;
    let mut bool_58: bool = false;
    let mut bool_59: bool = false;
    let mut bool_60: bool = false;
    let mut bool_61: bool = false;
    let mut bool_62: bool = true;
    let mut bool_63: bool = false;
    let mut bool_64: bool = false;
    let mut bool_65: bool = true;
    let mut bool_66: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_66, user_write: bool_65, user_execute: bool_64, group_read: bool_63, group_write: bool_62, group_execute: bool_61, other_read: bool_60, other_write: bool_59, other_execute: bool_58, sticky: bool_57, setgid: bool_56, setuid: bool_55};
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut bool_67: bool = false;
    let mut bool_68: bool = false;
    let mut bool_69: bool = true;
    let mut bool_70: bool = false;
    let mut bool_71: bool = true;
    let mut bool_72: bool = false;
    let mut bool_73: bool = false;
    let mut bool_74: bool = true;
    let mut bool_75: bool = true;
    let mut bool_76: bool = false;
    let mut bool_77: bool = false;
    let mut bool_78: bool = true;
    let mut permissions_1: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_78, user_write: bool_77, user_execute: bool_76, group_read: bool_75, group_write: bool_74, group_execute: bool_73, other_read: bool_72, other_write: bool_71, other_execute: bool_70, sticky: bool_69, setgid: bool_68, setuid: bool_67};
    let mut permissions_1_ref_0: &crate::meta::permissions::Permissions = &mut permissions_1;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut option_1: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut config_2: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut config_3: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_3_ref_0: &crate::config_file::Config = &mut config_3;
    let mut config_4: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_4_ref_0: &crate::config_file::Config = &mut config_4;
    let mut option_2: std::option::Option<flags::display::Display> = crate::flags::display::Display::from_config(config_4_ref_0);
    let mut option_3: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut option_4: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1424() {
//    rusty_monitor::set_test_id(1424);
    let mut display_0: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut display_0_ref_0: &flags::display::Display = &mut display_0;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_0: usize = 58usize;
    let mut bool_0: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_1: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut bool_1: bool = false;
    let mut bool_2: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_2, exec: bool_1};
    let mut str_0: &str = "Read";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut display_1_ref_0: &flags::display::Display = &mut display_1;
    let mut bool_3: bool = crate::flags::display::Display::eq(display_1_ref_0, display_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8372() {
//    rusty_monitor::set_test_id(8372);
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut elem_0: color::Elem = crate::color::Elem::File {exec: bool_1, uid: bool_0};
    let mut bool_2: bool = true;
    let mut elem_1: color::Elem = crate::color::Elem::Dir {uid: bool_2};
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_2: color::Elem = crate::color::Elem::NonFile;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_3: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_4: color::Elem = crate::color::Elem::User;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_0_ref_0);
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut display_0_ref_0: &flags::display::Display = &mut display_0;
    let mut tuple_0: () = crate::flags::display::Display::assert_receiver_is_total_eq(display_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_647() {
//    rusty_monitor::set_test_id(647);
    let mut display_0: flags::display::Display = crate::flags::display::Display::default();
    let mut display_0_ref_0: &flags::display::Display = &mut display_0;
    let mut display_1: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut display_1_ref_0: &flags::display::Display = &mut display_1;
    let mut display_2: flags::display::Display = crate::flags::display::Display::All;
    let mut display_2_ref_0: &flags::display::Display = &mut display_2;
    let mut display_3: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut display_3_ref_0: &flags::display::Display = &mut display_3;
    let mut display_4: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut display_4_ref_0: &flags::display::Display = &mut display_4;
    let mut display_5: flags::display::Display = crate::flags::display::Display::All;
    let mut display_5_ref_0: &flags::display::Display = &mut display_5;
    let mut display_6: flags::display::Display = crate::flags::display::Display::default();
    let mut display_6_ref_0: &flags::display::Display = &mut display_6;
    let mut display_7: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut display_7_ref_0: &flags::display::Display = &mut display_7;
    let mut display_8: flags::display::Display = crate::flags::display::Display::All;
    let mut display_8_ref_0: &flags::display::Display = &mut display_8;
    let mut display_9: flags::display::Display = crate::flags::display::Display::All;
    let mut display_9_ref_0: &flags::display::Display = &mut display_9;
    let mut display_10: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut display_10_ref_0: &flags::display::Display = &mut display_10;
    let mut display_11: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut display_11_ref_0: &flags::display::Display = &mut display_11;
    let mut display_12: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut display_12_ref_0: &flags::display::Display = &mut display_12;
    let mut display_13: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut display_13_ref_0: &flags::display::Display = &mut display_13;
    let mut bool_0: bool = crate::flags::display::Display::eq(display_13_ref_0, display_12_ref_0);
    let mut bool_1: bool = crate::flags::display::Display::eq(display_11_ref_0, display_10_ref_0);
    let mut bool_2: bool = crate::flags::display::Display::eq(display_9_ref_0, display_8_ref_0);
    let mut bool_3: bool = crate::flags::display::Display::eq(display_7_ref_0, display_6_ref_0);
    let mut bool_4: bool = crate::flags::display::Display::eq(display_5_ref_0, display_4_ref_0);
    let mut bool_5: bool = crate::flags::display::Display::eq(display_3_ref_0, display_2_ref_0);
    let mut bool_6: bool = crate::flags::display::Display::eq(display_1_ref_0, display_0_ref_0);
//    panic!("From RustyUnit with love");
}
}