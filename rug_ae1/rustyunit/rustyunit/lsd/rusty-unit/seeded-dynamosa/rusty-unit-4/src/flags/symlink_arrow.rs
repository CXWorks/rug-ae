use super::Configurable;

use crate::config_file::Config;

use clap::ArgMatches;

/// The flag showing how to display symbolic arrow.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SymlinkArrow(String);

impl Configurable<Self> for SymlinkArrow {
    /// `SymlinkArrow` can not be configured by [ArgMatches]
    ///
    /// Return `None`
    fn from_arg_matches(_: &ArgMatches) -> Option<Self> {
        None
    }
    /// Get a potential `SymlinkArrow` value from a [Config].
    ///
    /// If the `Config::symlink-arrow` has value,
    /// returns its value as the value of the `SymlinkArrow`, in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        config
            .symlink_arrow
            .as_ref()
            .map(|arrow| SymlinkArrow(arrow.to_string()))
    }
}

/// The default value for the `SymlinkArrow` is `\u{21d2}(⇒)`
impl Default for SymlinkArrow {
    fn default() -> Self {
        Self(String::from("\u{21d2}")) // ⇒
    }
}

use std::fmt;
impl fmt::Display for SymlinkArrow {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod test {
    use crate::config_file::Config;
    use crate::flags::Configurable;

    use super::SymlinkArrow;
    #[test]
    fn test_symlink_arrow_from_config_utf8() {
        let mut c = Config::with_none();
        c.symlink_arrow = Some("↹".into());
        assert_eq!(
            Some(SymlinkArrow(String::from("\u{21B9}"))),
            SymlinkArrow::from_config(&c)
        );
    }

    #[test]
    fn test_symlink_arrow_from_args_none() {
        use clap::App;
        assert_eq!(
            None,
            SymlinkArrow::from_arg_matches(&App::new("lsd").get_matches())
        );
    }

    #[test]
    fn test_symlink_arrow_default() {
        assert_eq!(
            SymlinkArrow(String::from("\u{21d2}")),
            SymlinkArrow::default()
        );
    }

    #[test]
    fn test_symlink_display() {
        assert_eq!("⇒", format!("{}", SymlinkArrow::default()));
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
#[timeout(30000)]fn rusty_test_199() {
//    rusty_monitor::set_test_id(199);
    let mut symlinkarrow_0: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::default();
    let mut symlinkarrow_0_ref_0: &crate::flags::symlink_arrow::SymlinkArrow = &mut symlinkarrow_0;
    let mut symlinkarrow_1: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::default();
    let mut symlinkarrow_1_ref_0: &crate::flags::symlink_arrow::SymlinkArrow = &mut symlinkarrow_1;
    let mut symlinkarrow_2: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::default();
    let mut symlinkarrow_2_ref_0: &crate::flags::symlink_arrow::SymlinkArrow = &mut symlinkarrow_2;
    let mut symlinkarrow_3: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::default();
    let mut symlinkarrow_3_ref_0: &crate::flags::symlink_arrow::SymlinkArrow = &mut symlinkarrow_3;
    let mut symlinkarrow_4: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::default();
    let mut symlinkarrow_4_ref_0: &crate::flags::symlink_arrow::SymlinkArrow = &mut symlinkarrow_4;
    let mut symlinkarrow_5: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::default();
    let mut symlinkarrow_5_ref_0: &crate::flags::symlink_arrow::SymlinkArrow = &mut symlinkarrow_5;
    let mut symlinkarrow_6: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::default();
    let mut symlinkarrow_6_ref_0: &crate::flags::symlink_arrow::SymlinkArrow = &mut symlinkarrow_6;
    let mut symlinkarrow_7: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::default();
    let mut symlinkarrow_7_ref_0: &crate::flags::symlink_arrow::SymlinkArrow = &mut symlinkarrow_7;
    let mut symlinkarrow_8: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::default();
    let mut symlinkarrow_8_ref_0: &crate::flags::symlink_arrow::SymlinkArrow = &mut symlinkarrow_8;
    let mut symlinkarrow_9: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::default();
    let mut symlinkarrow_9_ref_0: &crate::flags::symlink_arrow::SymlinkArrow = &mut symlinkarrow_9;
    let mut symlinkarrow_10: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::default();
    let mut symlinkarrow_10_ref_0: &crate::flags::symlink_arrow::SymlinkArrow = &mut symlinkarrow_10;
    let mut symlinkarrow_11: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::default();
    let mut symlinkarrow_11_ref_0: &crate::flags::symlink_arrow::SymlinkArrow = &mut symlinkarrow_11;
    let mut symlinkarrow_12: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::default();
    let mut symlinkarrow_12_ref_0: &crate::flags::symlink_arrow::SymlinkArrow = &mut symlinkarrow_12;
    let mut symlinkarrow_13: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::default();
    let mut symlinkarrow_13_ref_0: &crate::flags::symlink_arrow::SymlinkArrow = &mut symlinkarrow_13;
    let mut bool_0: bool = crate::flags::symlink_arrow::SymlinkArrow::eq(symlinkarrow_13_ref_0, symlinkarrow_12_ref_0);
    let mut bool_1: bool = crate::flags::symlink_arrow::SymlinkArrow::eq(symlinkarrow_11_ref_0, symlinkarrow_10_ref_0);
    let mut bool_2: bool = crate::flags::symlink_arrow::SymlinkArrow::eq(symlinkarrow_9_ref_0, symlinkarrow_8_ref_0);
    let mut bool_3: bool = crate::flags::symlink_arrow::SymlinkArrow::eq(symlinkarrow_7_ref_0, symlinkarrow_6_ref_0);
    let mut bool_4: bool = crate::flags::symlink_arrow::SymlinkArrow::eq(symlinkarrow_5_ref_0, symlinkarrow_4_ref_0);
    let mut bool_5: bool = crate::flags::symlink_arrow::SymlinkArrow::eq(symlinkarrow_3_ref_0, symlinkarrow_2_ref_0);
    let mut bool_6: bool = crate::flags::symlink_arrow::SymlinkArrow::eq(symlinkarrow_1_ref_0, symlinkarrow_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_713() {
//    rusty_monitor::set_test_id(713);
    let mut usize_0: usize = 40usize;
    let mut bool_0: bool = false;
    let mut usize_1: usize = 6usize;
    let mut bool_1: bool = true;
    let mut usize_2: usize = 80usize;
    let mut bool_2: bool = false;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut u64_0: u64 = 1048576u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut symlinkarrow_0: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::default();
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_3: usize = 360usize;
    let mut bool_3: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_3, depth: usize_3};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_1: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_1};
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut str_0: &str = "day_old";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut bool_4: bool = false;
    let mut bool_5: bool = false;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_5, exec: bool_4};
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut symlinkarrow_1: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::default();
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_1, order: sortorder_1, dir_grouping: dirgrouping_1};
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_4: usize = 120usize;
    let mut bool_6: bool = true;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_6, depth: usize_4};
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_2: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_2: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_2: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_2: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_2: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_2: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_2, theme: themeoption_2};
    let mut u64_1: u64 = 0u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut option_0: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_1: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_7: bool = true;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_7);
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
    let mut option_3: std::option::Option<crate::flags::symlink_arrow::SymlinkArrow> = crate::flags::symlink_arrow::SymlinkArrow::from_config(config_4_ref_0);
    let mut option_4: std::option::Option<crate::flags::symlink_arrow::SymlinkArrow> = crate::flags::symlink_arrow::SymlinkArrow::from_config(config_3_ref_0);
    let mut option_5: std::option::Option<crate::flags::symlink_arrow::SymlinkArrow> = crate::flags::symlink_arrow::SymlinkArrow::from_config(config_2_ref_0);
    let mut option_6: std::option::Option<crate::flags::symlink_arrow::SymlinkArrow> = crate::flags::symlink_arrow::SymlinkArrow::from_config(config_1_ref_0);
    let mut option_7: std::option::Option<crate::flags::symlink_arrow::SymlinkArrow> = crate::flags::symlink_arrow::SymlinkArrow::from_config(config_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_116() {
//    rusty_monitor::set_test_id(116);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut symlinkarrow_0: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::default();
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_0: usize = 1usize;
    let mut bool_0: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_1};
    let mut symlinkarrow_1: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::default();
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::INode;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    crate::meta::filetype::FileType::render(filetype_0, colors_0_ref_0);
    let mut elem_0: color::Elem = crate::color::Elem::NoAccess;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_771() {
//    rusty_monitor::set_test_id(771);
    let mut symlinkarrow_0: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::default();
    let mut symlinkarrow_0_ref_0: &crate::flags::symlink_arrow::SymlinkArrow = &mut symlinkarrow_0;
    let mut symlinkarrow_1: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::default();
    let mut symlinkarrow_1_ref_0: &crate::flags::symlink_arrow::SymlinkArrow = &mut symlinkarrow_1;
    let mut symlinkarrow_2: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::default();
    let mut symlinkarrow_2_ref_0: &crate::flags::symlink_arrow::SymlinkArrow = &mut symlinkarrow_2;
    let mut symlinkarrow_3: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::default();
    let mut symlinkarrow_3_ref_0: &crate::flags::symlink_arrow::SymlinkArrow = &mut symlinkarrow_3;
    let mut symlinkarrow_4: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::default();
    let mut symlinkarrow_4_ref_0: &crate::flags::symlink_arrow::SymlinkArrow = &mut symlinkarrow_4;
    let mut symlinkarrow_5: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::default();
    let mut symlinkarrow_5_ref_0: &crate::flags::symlink_arrow::SymlinkArrow = &mut symlinkarrow_5;
    let mut symlinkarrow_6: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::default();
    let mut symlinkarrow_6_ref_0: &crate::flags::symlink_arrow::SymlinkArrow = &mut symlinkarrow_6;
    let mut symlinkarrow_7: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::default();
    let mut symlinkarrow_7_ref_0: &crate::flags::symlink_arrow::SymlinkArrow = &mut symlinkarrow_7;
    let mut symlinkarrow_8: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::default();
    let mut symlinkarrow_8_ref_0: &crate::flags::symlink_arrow::SymlinkArrow = &mut symlinkarrow_8;
    let mut symlinkarrow_9: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::default();
    let mut symlinkarrow_9_ref_0: &crate::flags::symlink_arrow::SymlinkArrow = &mut symlinkarrow_9;
    let mut symlinkarrow_10: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::default();
    let mut symlinkarrow_10_ref_0: &crate::flags::symlink_arrow::SymlinkArrow = &mut symlinkarrow_10;
    let mut tuple_0: () = crate::flags::symlink_arrow::SymlinkArrow::assert_receiver_is_total_eq(symlinkarrow_10_ref_0);
    let mut tuple_1: () = crate::flags::symlink_arrow::SymlinkArrow::assert_receiver_is_total_eq(symlinkarrow_9_ref_0);
    let mut tuple_2: () = crate::flags::symlink_arrow::SymlinkArrow::assert_receiver_is_total_eq(symlinkarrow_8_ref_0);
    let mut tuple_3: () = crate::flags::symlink_arrow::SymlinkArrow::assert_receiver_is_total_eq(symlinkarrow_7_ref_0);
    let mut tuple_4: () = crate::flags::symlink_arrow::SymlinkArrow::assert_receiver_is_total_eq(symlinkarrow_6_ref_0);
    let mut tuple_5: () = crate::flags::symlink_arrow::SymlinkArrow::assert_receiver_is_total_eq(symlinkarrow_5_ref_0);
    let mut tuple_6: () = crate::flags::symlink_arrow::SymlinkArrow::assert_receiver_is_total_eq(symlinkarrow_4_ref_0);
    let mut tuple_7: () = crate::flags::symlink_arrow::SymlinkArrow::assert_receiver_is_total_eq(symlinkarrow_3_ref_0);
    let mut tuple_8: () = crate::flags::symlink_arrow::SymlinkArrow::assert_receiver_is_total_eq(symlinkarrow_2_ref_0);
    let mut tuple_9: () = crate::flags::symlink_arrow::SymlinkArrow::assert_receiver_is_total_eq(symlinkarrow_1_ref_0);
    let mut tuple_10: () = crate::flags::symlink_arrow::SymlinkArrow::assert_receiver_is_total_eq(symlinkarrow_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_253() {
//    rusty_monitor::set_test_id(253);
    let mut symlinkarrow_0: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::default();
    let mut symlinkarrow_0_ref_0: &crate::flags::symlink_arrow::SymlinkArrow = &mut symlinkarrow_0;
    let mut symlinkarrow_1: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::default();
    let mut symlinkarrow_1_ref_0: &crate::flags::symlink_arrow::SymlinkArrow = &mut symlinkarrow_1;
    let mut symlinkarrow_2: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::default();
    let mut symlinkarrow_2_ref_0: &crate::flags::symlink_arrow::SymlinkArrow = &mut symlinkarrow_2;
    let mut symlinkarrow_3: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::default();
    let mut symlinkarrow_3_ref_0: &crate::flags::symlink_arrow::SymlinkArrow = &mut symlinkarrow_3;
    let mut symlinkarrow_4: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::default();
    let mut symlinkarrow_4_ref_0: &crate::flags::symlink_arrow::SymlinkArrow = &mut symlinkarrow_4;
    let mut symlinkarrow_5: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::default();
    let mut symlinkarrow_5_ref_0: &crate::flags::symlink_arrow::SymlinkArrow = &mut symlinkarrow_5;
    let mut symlinkarrow_6: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::default();
    let mut symlinkarrow_6_ref_0: &crate::flags::symlink_arrow::SymlinkArrow = &mut symlinkarrow_6;
    let mut symlinkarrow_7: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::default();
    let mut symlinkarrow_7_ref_0: &crate::flags::symlink_arrow::SymlinkArrow = &mut symlinkarrow_7;
    let mut symlinkarrow_8: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::default();
    let mut symlinkarrow_8_ref_0: &crate::flags::symlink_arrow::SymlinkArrow = &mut symlinkarrow_8;
    let mut symlinkarrow_9: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::default();
    let mut symlinkarrow_9_ref_0: &crate::flags::symlink_arrow::SymlinkArrow = &mut symlinkarrow_9;
    let mut symlinkarrow_10: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::default();
    let mut symlinkarrow_10_ref_0: &crate::flags::symlink_arrow::SymlinkArrow = &mut symlinkarrow_10;
    let mut symlinkarrow_11: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::default();
    let mut symlinkarrow_11_ref_0: &crate::flags::symlink_arrow::SymlinkArrow = &mut symlinkarrow_11;
    let mut symlinkarrow_12: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::default();
    let mut symlinkarrow_12_ref_0: &crate::flags::symlink_arrow::SymlinkArrow = &mut symlinkarrow_12;
    let mut symlinkarrow_13: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::default();
    let mut symlinkarrow_13_ref_0: &crate::flags::symlink_arrow::SymlinkArrow = &mut symlinkarrow_13;
    let mut bool_0: bool = crate::flags::symlink_arrow::SymlinkArrow::ne(symlinkarrow_13_ref_0, symlinkarrow_12_ref_0);
    let mut bool_1: bool = crate::flags::symlink_arrow::SymlinkArrow::ne(symlinkarrow_11_ref_0, symlinkarrow_10_ref_0);
    let mut bool_2: bool = crate::flags::symlink_arrow::SymlinkArrow::ne(symlinkarrow_9_ref_0, symlinkarrow_8_ref_0);
    let mut bool_3: bool = crate::flags::symlink_arrow::SymlinkArrow::ne(symlinkarrow_7_ref_0, symlinkarrow_6_ref_0);
    let mut bool_4: bool = crate::flags::symlink_arrow::SymlinkArrow::ne(symlinkarrow_5_ref_0, symlinkarrow_4_ref_0);
    let mut bool_5: bool = crate::flags::symlink_arrow::SymlinkArrow::ne(symlinkarrow_3_ref_0, symlinkarrow_2_ref_0);
    let mut bool_6: bool = crate::flags::symlink_arrow::SymlinkArrow::ne(symlinkarrow_1_ref_0, symlinkarrow_0_ref_0);
//    panic!("From RustyUnit with love");
}
}