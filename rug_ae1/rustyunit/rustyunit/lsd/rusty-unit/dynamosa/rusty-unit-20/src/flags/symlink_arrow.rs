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
	use flags::Configurable;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_27() {
    rusty_monitor::set_test_id(27);
    let mut usize_0: usize = 7727usize;
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut bool_2: bool = false;
    let mut bool_3: bool = false;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut option_0: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_0);
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut option_1: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut option_2: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_3: std::option::Option<bool> = std::option::Option::None;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_4: bool = true;
    let mut bool_5: bool = false;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_5, exec: bool_4};
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut symlinkarrow_0: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::default();
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_1: usize = 9065usize;
    let mut bool_6: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_6, depth: usize_1};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut display_0: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut u64_0: u64 = 9323u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut symlinkarrow_1: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::default();
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_1, order: sortorder_1, dir_grouping: dirgrouping_1};
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_2: usize = 803usize;
    let mut bool_7: bool = true;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_7, depth: usize_2};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut config_2: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_1: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut config_3: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_3_ref_0: &crate::config_file::Config = &mut config_3;
    let mut u64_1: u64 = 4757u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut bool_8: bool = true;
    let mut bool_9: bool = true;
    let mut bool_10: bool = true;
    let mut bool_11: bool = false;
    let mut bool_12: bool = false;
    let mut bool_13: bool = false;
    let mut bool_14: bool = false;
    let mut bool_15: bool = true;
    let mut bool_16: bool = true;
    let mut bool_17: bool = false;
    let mut bool_18: bool = false;
    let mut bool_19: bool = false;
    let mut config_4: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_4_ref_0: &crate::config_file::Config = &mut config_4;
    let mut config_5: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_5_ref_0: &crate::config_file::Config = &mut config_5;
    let mut option_4: std::option::Option<crate::flags::symlink_arrow::SymlinkArrow> = crate::flags::symlink_arrow::SymlinkArrow::from_config(config_5_ref_0);
    let mut symlinkarrow_2: crate::flags::symlink_arrow::SymlinkArrow = std::option::Option::unwrap(option_4);
    let mut display_2: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_19, user_write: bool_18, user_execute: bool_17, group_read: bool_16, group_write: bool_15, group_execute: bool_14, other_read: bool_13, other_write: bool_12, other_execute: bool_11, sticky: bool_10, setgid: bool_9, setuid: bool_8};
    panic!("From RustyUnit with love");
}
}