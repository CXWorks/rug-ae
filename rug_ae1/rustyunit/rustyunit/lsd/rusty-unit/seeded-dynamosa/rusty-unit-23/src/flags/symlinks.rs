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
#[timeout(30000)]fn rusty_test_3909() {
//    rusty_monitor::set_test_id(3909);
    let mut elem_0: color::Elem = crate::color::Elem::SymLink;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut option_0: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_0);
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut option_1: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_0);
    let mut option_2: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut option_3: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_0);
    let mut bool_0: bool = true;
    let mut option_4: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut option_5: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_6: std::option::Option<std::string::String> = std::option::Option::None;
    let mut u64_0: u64 = 0u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut u64_1: u64 = 0u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut str_0: &str = "Theme";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_1: bool = true;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_1};
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut option_7: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut option_8: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_8, theme: option_7, separator: option_6};
    let mut option_9: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut option_10: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut bool_2: bool = false;
    let mut option_11: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut option_12: std::option::Option<std::string::String> = std::option::Option::None;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut str_1: &str = "Do not ignore entries starting with .";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_1_ref_0: &meta::date::Date = &mut date_1;
    let mut u64_2: u64 = 1099511627776u64;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size::new(u64_2);
    let mut elem_1: color::Elem = crate::color::Elem::Group;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    crate::meta::filetype::FileType::render(filetype_1, colors_1_ref_0);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Links;
    let mut elem_2: color::Elem = crate::color::Elem::Octal;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_573() {
//    rusty_monitor::set_test_id(573);
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

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_867() {
//    rusty_monitor::set_test_id(867);
    let mut nosymlink_0: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_1: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_1_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_1;
    let mut nosymlink_2: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_2_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_2;
    let mut nosymlink_3: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_4: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_4_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_4;
    let mut nosymlink_5: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_5_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_5;
    let mut nosymlink_6: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
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
    let mut tuple_4: () = crate::flags::symlinks::NoSymlink::assert_receiver_is_total_eq(nosymlink_5_ref_0);
    let mut tuple_5: () = crate::flags::symlinks::NoSymlink::assert_receiver_is_total_eq(nosymlink_4_ref_0);
    let mut tuple_6: () = crate::flags::symlinks::NoSymlink::assert_receiver_is_total_eq(nosymlink_2_ref_0);
    let mut tuple_7: () = crate::flags::symlinks::NoSymlink::assert_receiver_is_total_eq(nosymlink_1_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_472() {
//    rusty_monitor::set_test_id(472);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_0);
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_2: std::option::Option<bool> = std::option::Option::None;
    let mut bool_0: bool = true;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut option_4: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::None;
    let mut option_5: std::option::Option<bool> = std::option::Option::None;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut option_6: std::option::Option<flags::sorting::SortColumn> = std::option::Option::Some(sortcolumn_0);
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_6, reverse: option_5, dir_grouping: option_4};
    let mut option_7: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_0);
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut option_8: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_0);
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut option_9: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_0);
    let mut option_10: std::option::Option<usize> = std::option::Option::None;
    let mut option_11: std::option::Option<bool> = std::option::Option::None;
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_11, depth: option_10};
    let mut option_12: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_0);
    let mut option_13: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_1: bool = true;
    let mut option_14: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_15: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_16: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut display_0: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut option_17: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut option_18: std::option::Option<bool> = std::option::Option::None;
    let mut option_19: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_20: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut option_21: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_21, theme: option_20};
    let mut option_22: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_23: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_2: bool = true;
    let mut option_24: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_24, blocks: option_23, color: option_22, date: option_19, dereference: option_18, display: option_17, icons: option_16, ignore_globs: option_15, indicators: option_14, layout: option_13, recursion: option_12, size: option_9, permission: option_8, sorting: option_7, no_symlink: option_3, total_size: option_2, symlink_arrow: option_1, hyperlink: option_0};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut config_2: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut option_25: std::option::Option<crate::flags::symlinks::NoSymlink> = crate::flags::symlinks::NoSymlink::from_config(config_2_ref_0);
    let mut option_26: std::option::Option<crate::flags::symlinks::NoSymlink> = crate::flags::symlinks::NoSymlink::from_config(config_1_ref_0);
    let mut option_27: std::option::Option<crate::flags::symlinks::NoSymlink> = crate::flags::symlinks::NoSymlink::from_config(config_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_249() {
//    rusty_monitor::set_test_id(249);
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
}