//! This module defines the [NoSymlink] flag. To set it up from [ArgMatches], a [Config] and its
//! [Default] value, use the [configure_from](Configurable::configure_from) method.

use super::Configurable;

use crate::config_file::Config;

use clap::ArgMatches;

/// The flag showing whether to follow symbolic links.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub struct NoSymlink(pub bool);

impl Configurable<Self> for NoSymlink {
    /// Get a potential `NoSymlink` value from [ArgMatches].
    ///
    /// If the "no-symlink" argument is passed, this returns a `NoSymlink` with value `true` in a
    /// [Some]. Otherwise this returns [None].
    fn from_arg_matches(matches: &ArgMatches) -> Option<Self> {
        if matches.is_present("no-symlink") {
            Some(Self(true))
        } else {
            None
        }
    }

    /// Get a potential `NoSymlink` value from a [Config].
    ///
    /// If the `Config::no-symlink` has value,
    /// this returns it as the value of the `NoSymlink`, in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        config.no_symlink.map(Self)
    }
}

#[cfg(test)]
mod test {
    use super::NoSymlink;

    use crate::app;
    use crate::config_file::Config;
    use crate::flags::Configurable;

    #[test]
    fn test_from_arg_matches_none() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(None, NoSymlink::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_true() {
        let argv = vec!["lsd", "--no-symlink"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(Some(NoSymlink(true)), NoSymlink::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, NoSymlink::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_true() {
        let mut c = Config::with_none();
        c.no_symlink = Some(true);
        assert_eq!(Some(NoSymlink(true)), NoSymlink::from_config(&c));
    }

    #[test]
    fn test_from_config_false() {
        let mut c = Config::with_none();
        c.no_symlink = Some(false);
        assert_eq!(Some(NoSymlink(false)), NoSymlink::from_config(&c));
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
#[timeout(30000)]fn rusty_test_689() {
//    rusty_monitor::set_test_id(689);
    let mut nosymlink_0: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_0_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_0;
    let mut nosymlink_1: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_1_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_1;
    let mut nosymlink_2: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_2_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_2;
    let mut nosymlink_3: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_3_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_3;
    let mut nosymlink_4: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_4_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_4;
    let mut nosymlink_5: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_5_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_5;
    let mut nosymlink_6: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_6_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_6;
    let mut nosymlink_7: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_7_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_7;
    let mut nosymlink_8: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_8_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_8;
    let mut nosymlink_9: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_9_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_9;
    let mut nosymlink_10: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_10_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_10;
    let mut tuple_0: () = crate::flags::symlinks::NoSymlink::assert_receiver_is_total_eq(nosymlink_10_ref_0);
    let mut tuple_1: () = crate::flags::symlinks::NoSymlink::assert_receiver_is_total_eq(nosymlink_9_ref_0);
    let mut tuple_2: () = crate::flags::symlinks::NoSymlink::assert_receiver_is_total_eq(nosymlink_8_ref_0);
    let mut tuple_3: () = crate::flags::symlinks::NoSymlink::assert_receiver_is_total_eq(nosymlink_7_ref_0);
    let mut tuple_4: () = crate::flags::symlinks::NoSymlink::assert_receiver_is_total_eq(nosymlink_6_ref_0);
    let mut tuple_5: () = crate::flags::symlinks::NoSymlink::assert_receiver_is_total_eq(nosymlink_5_ref_0);
    let mut tuple_6: () = crate::flags::symlinks::NoSymlink::assert_receiver_is_total_eq(nosymlink_4_ref_0);
    let mut tuple_7: () = crate::flags::symlinks::NoSymlink::assert_receiver_is_total_eq(nosymlink_3_ref_0);
    let mut tuple_8: () = crate::flags::symlinks::NoSymlink::assert_receiver_is_total_eq(nosymlink_2_ref_0);
    let mut tuple_9: () = crate::flags::symlinks::NoSymlink::assert_receiver_is_total_eq(nosymlink_1_ref_0);
    let mut tuple_10: () = crate::flags::symlinks::NoSymlink::assert_receiver_is_total_eq(nosymlink_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_615() {
//    rusty_monitor::set_test_id(615);
    let mut nosymlink_0: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_0_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_0;
    let mut nosymlink_1: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_1_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_1;
    let mut nosymlink_2: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_2_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_2;
    let mut nosymlink_3: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_3_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_3;
    let mut nosymlink_4: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_4_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_4;
    let mut nosymlink_5: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_5_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_5;
    let mut nosymlink_6: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_6_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_6;
    let mut nosymlink_7: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_7_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_7;
    let mut nosymlink_8: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_8_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_8;
    let mut nosymlink_9: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_9_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_9;
    let mut nosymlink_10: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_10_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_10;
    let mut nosymlink_11: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_11_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_11;
    let mut nosymlink_12: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_12_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_12;
    let mut nosymlink_13: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_13_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_13;
    let mut bool_0: bool = crate::flags::symlinks::NoSymlink::eq(nosymlink_13_ref_0, nosymlink_12_ref_0);
    let mut bool_1: bool = crate::flags::symlinks::NoSymlink::eq(nosymlink_11_ref_0, nosymlink_10_ref_0);
    let mut bool_2: bool = crate::flags::symlinks::NoSymlink::eq(nosymlink_9_ref_0, nosymlink_8_ref_0);
    let mut bool_3: bool = crate::flags::symlinks::NoSymlink::eq(nosymlink_7_ref_0, nosymlink_6_ref_0);
    let mut bool_4: bool = crate::flags::symlinks::NoSymlink::eq(nosymlink_5_ref_0, nosymlink_4_ref_0);
    let mut bool_5: bool = crate::flags::symlinks::NoSymlink::eq(nosymlink_3_ref_0, nosymlink_2_ref_0);
    let mut bool_6: bool = crate::flags::symlinks::NoSymlink::eq(nosymlink_1_ref_0, nosymlink_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_89() {
//    rusty_monitor::set_test_id(89);
    let mut u64_0: u64 = 0u64;
    let mut bool_0: bool = false;
    let mut usize_0: usize = 63usize;
    let mut bool_1: bool = false;
    let mut u64_1: u64 = 1024u64;
    let mut usize_1: usize = 0usize;
    let mut bool_2: bool = true;
    let mut bool_3: bool = false;
    let mut bool_4: bool = false;
    let mut bool_5: bool = false;
    let mut bool_6: bool = false;
    let mut bool_7: bool = true;
    let mut u64_2: u64 = 1073741824u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_2);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_8: bool = true;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_8);
    let mut bool_9: bool = true;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_9);
    let mut option_4: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut option_5: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut option_6: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_7: std::option::Option<usize> = std::option::Option::None;
    let mut option_8: std::option::Option<bool> = std::option::Option::None;
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_8, depth: option_7};
    let mut option_9: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_0);
    let mut option_10: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_10: bool = true;
    let mut option_11: std::option::Option<bool> = std::option::Option::Some(bool_10);
    let mut option_12: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_13: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut option_14: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut option_15: std::option::Option<bool> = std::option::Option::None;
    let mut option_16: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_17: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut option_18: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_18, theme: option_17};
    let mut option_19: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_20: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_11: bool = true;
    let mut option_21: std::option::Option<bool> = std::option::Option::Some(bool_11);
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_21, blocks: option_20, color: option_19, date: option_16, dereference: option_15, display: option_14, icons: option_13, ignore_globs: option_12, indicators: option_11, layout: option_10, recursion: option_9, size: option_6, permission: option_5, sorting: option_4, no_symlink: option_3, total_size: option_2, symlink_arrow: option_1, hyperlink: option_0};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_2: usize = 2usize;
    let mut bool_12: bool = false;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_12, depth: usize_2};
    let mut nosymlink_0: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_1: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_1};
    let mut config_2: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut bool_13: bool = false;
    let mut bool_14: bool = true;
    let mut bool_15: bool = false;
    let mut bool_16: bool = false;
    let mut bool_17: bool = true;
    let mut bool_18: bool = true;
    let mut bool_19: bool = false;
    let mut bool_20: bool = true;
    let mut bool_21: bool = true;
    let mut bool_22: bool = false;
    let mut bool_23: bool = true;
    let mut bool_24: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_24, user_write: bool_23, user_execute: bool_22, group_read: bool_21, group_write: bool_20, group_execute: bool_19, other_read: bool_18, other_write: bool_17, other_execute: bool_16, sticky: bool_15, setgid: bool_14, setuid: bool_13};
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut config_3: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_3_ref_0: &crate::config_file::Config = &mut config_3;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut elem_0: color::Elem = crate::color::Elem::Acl;
    let mut option_22: std::option::Option<crate::flags::symlinks::NoSymlink> = crate::flags::symlinks::NoSymlink::from_config(config_3_ref_0);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_709() {
//    rusty_monitor::set_test_id(709);
    let mut nosymlink_0: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_0_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_0;
    let mut nosymlink_1: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_1_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_1;
    let mut nosymlink_2: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_2_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_2;
    let mut nosymlink_3: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_3_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_3;
    let mut nosymlink_4: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_4_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_4;
    let mut nosymlink_5: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_5_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_5;
    let mut nosymlink_6: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_6_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_6;
    let mut nosymlink_7: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_7_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_7;
    let mut nosymlink_8: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_8_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_8;
    let mut nosymlink_9: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_9_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_9;
    let mut nosymlink_10: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_10_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_10;
    let mut nosymlink_11: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_11_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_11;
    let mut nosymlink_12: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_12_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_12;
    let mut nosymlink_13: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_13_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_13;
    let mut bool_0: bool = crate::flags::symlinks::NoSymlink::ne(nosymlink_13_ref_0, nosymlink_12_ref_0);
    let mut bool_1: bool = crate::flags::symlinks::NoSymlink::ne(nosymlink_11_ref_0, nosymlink_10_ref_0);
    let mut bool_2: bool = crate::flags::symlinks::NoSymlink::ne(nosymlink_9_ref_0, nosymlink_8_ref_0);
    let mut bool_3: bool = crate::flags::symlinks::NoSymlink::ne(nosymlink_7_ref_0, nosymlink_6_ref_0);
    let mut bool_4: bool = crate::flags::symlinks::NoSymlink::ne(nosymlink_5_ref_0, nosymlink_4_ref_0);
    let mut bool_5: bool = crate::flags::symlinks::NoSymlink::ne(nosymlink_3_ref_0, nosymlink_2_ref_0);
    let mut bool_6: bool = crate::flags::symlinks::NoSymlink::ne(nosymlink_1_ref_0, nosymlink_0_ref_0);
//    panic!("From RustyUnit with love");
}
}