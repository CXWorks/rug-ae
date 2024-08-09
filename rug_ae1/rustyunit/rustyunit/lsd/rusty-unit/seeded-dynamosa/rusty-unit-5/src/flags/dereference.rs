//! This module defines the [Dereference] flag. To set it up from [ArgMatches], a [Config] and its
//! [Default] value, use the [configure_from](Configurable::configure_from) method.

use super::Configurable;

use crate::config_file::Config;

use clap::ArgMatches;

/// The flag showing whether to dereference symbolic links.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub struct Dereference(pub bool);

impl Configurable<Self> for Dereference {
    /// Get a potential `Dereference` value from [ArgMatches].
    ///
    /// If the "dereference" argument is passed, this returns a `Dereference` with value `true` in
    /// a [Some]. Otherwise this returns [None].
    fn from_arg_matches(matches: &ArgMatches) -> Option<Self> {
        if matches.is_present("dereference") {
            Some(Self(true))
        } else {
            None
        }
    }

    /// Get a potential `Dereference` value from a [Config].
    ///
    /// If the `Config::dereference` has value, this returns its value
    /// as the value of the `Dereference`, in a [Some], Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        config.dereference.as_ref().map(|deref| Self(*deref))
    }
}

#[cfg(test)]
mod test {
    use super::Dereference;

    use crate::app;
    use crate::config_file::Config;
    use crate::flags::Configurable;

    #[test]
    fn test_from_arg_matches_none() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(None, Dereference::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_true() {
        let argv = vec!["lsd", "--dereference"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(Dereference(true)),
            Dereference::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, Dereference::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_true() {
        let mut c = Config::with_none();
        c.dereference = Some(true);
        assert_eq!(Some(Dereference(true)), Dereference::from_config(&c));
    }

    #[test]
    fn test_from_config_false() {
        let mut c = Config::with_none();
        c.dereference = Some(false);
        assert_eq!(Some(Dereference(false)), Dereference::from_config(&c));
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
#[timeout(30000)]fn rusty_test_802() {
//    rusty_monitor::set_test_id(802);
    let mut dereference_0: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_0_ref_0: &crate::flags::dereference::Dereference = &mut dereference_0;
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut filetype_4: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut filetype_5: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut filetype_6: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut filetype_7: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut filetype_8: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut filetype_9: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_0, exec: bool_1};
    let mut filetype_10: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut filetype_11: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut filetype_12: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut filetype_13: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut filetype_14: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut filetype_15: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut filetype_16: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut filetype_17: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut filetype_18: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut filetype_19: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut filetype_20: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut filetype_21: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut filetype_22: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut filetype_23: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut tuple_0: () = crate::flags::dereference::Dereference::assert_receiver_is_total_eq(dereference_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_508() {
//    rusty_monitor::set_test_id(508);
    let mut dereference_0: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_0_ref_0: &crate::flags::dereference::Dereference = &mut dereference_0;
    let mut dereference_1: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_1_ref_0: &crate::flags::dereference::Dereference = &mut dereference_1;
    let mut dereference_2: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_2_ref_0: &crate::flags::dereference::Dereference = &mut dereference_2;
    let mut dereference_3: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_3_ref_0: &crate::flags::dereference::Dereference = &mut dereference_3;
    let mut dereference_4: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_4_ref_0: &crate::flags::dereference::Dereference = &mut dereference_4;
    let mut dereference_5: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_5_ref_0: &crate::flags::dereference::Dereference = &mut dereference_5;
    let mut dereference_6: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_6_ref_0: &crate::flags::dereference::Dereference = &mut dereference_6;
    let mut dereference_7: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_7_ref_0: &crate::flags::dereference::Dereference = &mut dereference_7;
    let mut dereference_8: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_8_ref_0: &crate::flags::dereference::Dereference = &mut dereference_8;
    let mut dereference_9: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_9_ref_0: &crate::flags::dereference::Dereference = &mut dereference_9;
    let mut dereference_10: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_10_ref_0: &crate::flags::dereference::Dereference = &mut dereference_10;
    let mut dereference_11: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_11_ref_0: &crate::flags::dereference::Dereference = &mut dereference_11;
    let mut dereference_12: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_12_ref_0: &crate::flags::dereference::Dereference = &mut dereference_12;
    let mut dereference_13: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_13_ref_0: &crate::flags::dereference::Dereference = &mut dereference_13;
    let mut bool_0: bool = crate::flags::dereference::Dereference::eq(dereference_13_ref_0, dereference_12_ref_0);
    let mut bool_1: bool = crate::flags::dereference::Dereference::eq(dereference_11_ref_0, dereference_10_ref_0);
    let mut bool_2: bool = crate::flags::dereference::Dereference::eq(dereference_9_ref_0, dereference_8_ref_0);
    let mut bool_3: bool = crate::flags::dereference::Dereference::eq(dereference_7_ref_0, dereference_6_ref_0);
    let mut bool_4: bool = crate::flags::dereference::Dereference::eq(dereference_5_ref_0, dereference_4_ref_0);
    let mut bool_5: bool = crate::flags::dereference::Dereference::eq(dereference_3_ref_0, dereference_2_ref_0);
    let mut bool_6: bool = crate::flags::dereference::Dereference::eq(dereference_1_ref_0, dereference_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_416() {
//    rusty_monitor::set_test_id(416);
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut usize_0: usize = 2usize;
    let mut bool_2: bool = false;
    let mut usize_1: usize = 1usize;
    let mut bool_3: bool = true;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut u64_0: u64 = 1099511627776u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_2: usize = 6usize;
    let mut bool_4: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_4, depth: usize_2};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dereference_0: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_1};
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_1, order: sortorder_1, dir_grouping: dirgrouping_1};
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_3: usize = 360usize;
    let mut bool_5: bool = false;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_5, depth: usize_3};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_1: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dereference_1: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dateflag_2: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_2: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_2: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_2, theme: themeoption_3};
    let mut themeoption_4: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut option_0: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_4);
    let mut coloroption_3: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut option_1: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_3);
    let mut color_3: crate::config_file::Color = crate::config_file::Color {when: option_1, theme: option_0};
    let mut option_2: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_3);
    let mut option_3: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_6: bool = true;
    let mut option_4: std::option::Option<bool> = std::option::Option::Some(bool_6);
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut config_2: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut config_3: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_3_ref_0: &crate::config_file::Config = &mut config_3;
    let mut config_4: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_4_ref_0: &crate::config_file::Config = &mut config_4;
    let mut config_5: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_5_ref_0: &crate::config_file::Config = &mut config_5;
    let mut config_6: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_6_ref_0: &crate::config_file::Config = &mut config_6;
    let mut option_5: std::option::Option<crate::flags::dereference::Dereference> = crate::flags::dereference::Dereference::from_config(config_6_ref_0);
    let mut option_6: std::option::Option<crate::flags::dereference::Dereference> = crate::flags::dereference::Dereference::from_config(config_5_ref_0);
    let mut option_7: std::option::Option<crate::flags::dereference::Dereference> = crate::flags::dereference::Dereference::from_config(config_4_ref_0);
    let mut option_8: std::option::Option<crate::flags::dereference::Dereference> = crate::flags::dereference::Dereference::from_config(config_3_ref_0);
    let mut option_9: std::option::Option<crate::flags::dereference::Dereference> = crate::flags::dereference::Dereference::from_config(config_2_ref_0);
    let mut option_10: std::option::Option<crate::flags::dereference::Dereference> = crate::flags::dereference::Dereference::from_config(config_1_ref_0);
    let mut option_11: std::option::Option<crate::flags::dereference::Dereference> = crate::flags::dereference::Dereference::from_config(config_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_591() {
//    rusty_monitor::set_test_id(591);
    let mut dereference_0: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_0_ref_0: &crate::flags::dereference::Dereference = &mut dereference_0;
    let mut dereference_1: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_1_ref_0: &crate::flags::dereference::Dereference = &mut dereference_1;
    let mut dereference_2: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_2_ref_0: &crate::flags::dereference::Dereference = &mut dereference_2;
    let mut dereference_3: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_3_ref_0: &crate::flags::dereference::Dereference = &mut dereference_3;
    let mut dereference_4: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_4_ref_0: &crate::flags::dereference::Dereference = &mut dereference_4;
    let mut dereference_5: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_5_ref_0: &crate::flags::dereference::Dereference = &mut dereference_5;
    let mut dereference_6: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_6_ref_0: &crate::flags::dereference::Dereference = &mut dereference_6;
    let mut dereference_7: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_7_ref_0: &crate::flags::dereference::Dereference = &mut dereference_7;
    let mut dereference_8: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_8_ref_0: &crate::flags::dereference::Dereference = &mut dereference_8;
    let mut dereference_9: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_9_ref_0: &crate::flags::dereference::Dereference = &mut dereference_9;
    let mut dereference_10: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_10_ref_0: &crate::flags::dereference::Dereference = &mut dereference_10;
    let mut dereference_11: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_11_ref_0: &crate::flags::dereference::Dereference = &mut dereference_11;
    let mut dereference_12: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_12_ref_0: &crate::flags::dereference::Dereference = &mut dereference_12;
    let mut dereference_13: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_13_ref_0: &crate::flags::dereference::Dereference = &mut dereference_13;
    let mut bool_0: bool = crate::flags::dereference::Dereference::ne(dereference_13_ref_0, dereference_12_ref_0);
    let mut bool_1: bool = crate::flags::dereference::Dereference::ne(dereference_11_ref_0, dereference_10_ref_0);
    let mut bool_2: bool = crate::flags::dereference::Dereference::ne(dereference_9_ref_0, dereference_8_ref_0);
    let mut bool_3: bool = crate::flags::dereference::Dereference::ne(dereference_7_ref_0, dereference_6_ref_0);
    let mut bool_4: bool = crate::flags::dereference::Dereference::ne(dereference_5_ref_0, dereference_4_ref_0);
    let mut bool_5: bool = crate::flags::dereference::Dereference::ne(dereference_3_ref_0, dereference_2_ref_0);
    let mut bool_6: bool = crate::flags::dereference::Dereference::ne(dereference_1_ref_0, dereference_0_ref_0);
//    panic!("From RustyUnit with love");
}
}