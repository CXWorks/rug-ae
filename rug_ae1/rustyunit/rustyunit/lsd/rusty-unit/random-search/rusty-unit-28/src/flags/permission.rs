//! This module defines the [PermissionFlag]. To set it up from [ArgMatches], a [Config] and its
//! [Default] value, use its [configure_from](Configurable::configure_from) method.

use super::Configurable;

use crate::config_file::Config;

use clap::ArgMatches;
use serde::Deserialize;

/// The flag showing which file permissions units to use.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PermissionFlag {
    /// The variant to show file permissions in rwx format
    Rwx,
    /// The variant to show file permissions in octal format
    Octal,
}

impl PermissionFlag {
    fn from_str(value: &str) -> Option<Self> {
        match value {
            "rwx" => Some(Self::Rwx),
            "octal" => Some(Self::Octal),
            _ => {
                panic!(
                    "Permissions can only be one of rwx or octal, but got {}.",
                    value
                );
            }
        }
    }
}

impl Configurable<Self> for PermissionFlag {
    /// Get a potential `PermissionFlag` variant from [ArgMatches].
    ///
    /// If any of the "rwx" or "octal" arguments is passed, the corresponding
    /// `PermissionFlag` variant is returned in a [Some]. If neither of them is passed,
    /// this returns [None].
    /// Sets permissions to rwx if classic flag is enabled.
    fn from_arg_matches(matches: &ArgMatches) -> Option<Self> {
        if matches.is_present("classic") {
            return Some(Self::Rwx);
        } else if matches.occurrences_of("permission") > 0 {
            if let Some(permissions) = matches.values_of("permission")?.last() {
                return Self::from_str(permissions);
            }
        }
        None
    }

    /// Get a potential `PermissionFlag` variant from a [Config].
    ///
    /// If the `Config::permissions` has value and is one of "rwx" or "octal",
    /// this returns the corresponding `PermissionFlag` variant in a [Some].
    /// Otherwise this returns [None].
    /// Sets permissions to rwx if classic flag is enabled.
    fn from_config(config: &Config) -> Option<Self> {
        if let Some(true) = config.classic {
            Some(Self::Rwx)
        } else {
            config.permission
        }
    }
}

/// The default value for `PermissionFlag` is [PermissionFlag::Default].
impl Default for PermissionFlag {
    fn default() -> Self {
        Self::Rwx
    }
}

#[cfg(test)]
mod test {
    use super::PermissionFlag;

    use crate::app;
    use crate::config_file::Config;
    use crate::flags::Configurable;

    #[test]
    fn test_default() {
        assert_eq!(PermissionFlag::Rwx, PermissionFlag::default());
    }

    #[test]
    fn test_from_arg_matches_none() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(None, PermissionFlag::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_default() {
        let argv = vec!["lsd", "--permission", "rwx"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(PermissionFlag::Rwx),
            PermissionFlag::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_short() {
        let args = vec!["lsd", "--permission", "octal"];
        let matches = app::build().get_matches_from_safe(args).unwrap();
        assert_eq!(
            Some(PermissionFlag::Octal),
            PermissionFlag::from_arg_matches(&matches)
        );
    }

    #[test]
    #[should_panic]
    fn test_from_arg_matches_unknown() {
        let args = vec!["lsd", "--permission", "unknown"];
        let _ = app::build().get_matches_from_safe(args).unwrap();
    }
    #[test]
    fn test_from_arg_matches_permissions_multi() {
        let args = vec!["lsd", "--permission", "octal", "--permission", "rwx"];
        let matches = app::build().get_matches_from_safe(args).unwrap();
        assert_eq!(
            Some(PermissionFlag::Rwx),
            PermissionFlag::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_permissions_classic() {
        let args = vec!["lsd", "--permission", "rwx", "--classic"];
        let matches = app::build().get_matches_from_safe(args).unwrap();
        assert_eq!(
            Some(PermissionFlag::Rwx),
            PermissionFlag::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, PermissionFlag::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_rwx() {
        let mut c = Config::with_none();
        c.permission = Some(PermissionFlag::Rwx);
        assert_eq!(Some(PermissionFlag::Rwx), PermissionFlag::from_config(&c));
    }

    #[test]
    fn test_from_config_octal() {
        let mut c = Config::with_none();
        c.permission = Some(PermissionFlag::Octal);
        assert_eq!(Some(PermissionFlag::Octal), PermissionFlag::from_config(&c));
    }

    #[test]
    fn test_from_config_classic_mode() {
        let mut c = Config::with_none();
        c.classic = Some(true);
        assert_eq!(Some(PermissionFlag::Rwx), PermissionFlag::from_config(&c));
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::default::Default;
	use std::cmp::PartialEq;
	use std::clone::Clone;
	use std::cmp::Eq;
	use flags::Configurable;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3087() {
    rusty_monitor::set_test_id(3087);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut option_0: std::option::Option<std::string::String> = std::option::Option::None;
    let mut str_0: &str = "GJNPXD";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut option_1: std::option::Option<bool> = std::option::Option::None;
    let mut option_2: std::option::Option<bool> = std::option::Option::None;
    let mut option_3: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut option_4: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut option_5: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_0);
    let mut option_6: std::option::Option<usize> = std::option::Option::None;
    let mut bool_0: bool = false;
    let mut option_7: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_7, depth: option_6};
    let mut option_8: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_0);
    let mut option_9: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut option_10: std::option::Option<bool> = std::option::Option::None;
    let mut option_11: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_12: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_13: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut bool_1: bool = true;
    let mut option_14: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_15: std::option::Option<std::string::String> = std::option::Option::None;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut str_1: &str = "R6";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bool_2: bool = true;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_2};
    let mut option_16: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_17: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_18: std::option::Option<bool> = std::option::Option::None;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::default();
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_0: usize = 43usize;
    let mut bool_3: bool = false;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_3, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_0: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut bool_4: bool = true;
    let mut filetype_4: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_4};
    let mut str_2: &str = "";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bool_5: bool = true;
    let mut bool_6: bool = true;
    let mut filetype_5: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_6, exec: bool_5};
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut u64_0: u64 = 58u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut permissionflag_1_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_1;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut themeoption_2_ref_0: &flags::color::ThemeOption = &mut themeoption_2;
    let mut str_3: &str = "lGxRjqkVz";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut iconoption_1_ref_0: &flags::icons::IconOption = &mut iconoption_1;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut themeoption_3_ref_0: &flags::color::ThemeOption = &mut themeoption_3;
    let mut permissionflag_2: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut permissionflag_2_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_2;
    let mut bool_7: bool = crate::flags::permission::PermissionFlag::eq(permissionflag_2_ref_0, permissionflag_1_ref_0);
    let mut permissionflag_3: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::default();
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut dirgrouping_0_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_0;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4154() {
    rusty_monitor::set_test_id(4154);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Exec;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Pipe;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut elem_2: color::Elem = crate::color::Elem::File {exec: bool_1, uid: bool_0};
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::ExecSticky;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_4: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut elem_5: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_5_ref_0);
    let mut theme_6: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_6_ref_0: &crate::color::theme::Theme = &mut theme_6;
    let mut elem_6: color::Elem = crate::color::Elem::HourOld;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut color_6: crossterm::style::Color = crate::color::Elem::get_color(elem_6_ref_0, theme_6_ref_0);
    let mut theme_7: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_7_ref_0: &crate::color::theme::Theme = &mut theme_7;
    let mut elem_7: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut color_7: crossterm::style::Color = crate::color::Elem::get_color(elem_7_ref_0, theme_7_ref_0);
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut option_0: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut config_2: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut option_1: std::option::Option<flags::permission::PermissionFlag> = crate::flags::permission::PermissionFlag::from_config(config_2_ref_0);
    let mut str_1: &str = "Hc2VodzAhSYA";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut option_2: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut config_3: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_3_ref_0: &crate::config_file::Config = &mut config_3;
    let mut option_3: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_4: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut config_4: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_4_ref_0: &crate::config_file::Config = &mut config_4;
    let mut option_5: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_6: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut config_5: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_5_ref_0: &crate::config_file::Config = &mut config_5;
    let mut permission_0: crate::color::theme::Permission = crate::color::theme::Permission {read: color_7, write: color_6, exec: color_5, exec_sticky: color_4, no_access: color_3, octal: color_2, acl: color_1, context: color_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4335() {
    rusty_monitor::set_test_id(4335);
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::default();
    let mut permissionflag_0_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_0;
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut permissionflag_1_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_1;
    let mut bool_0: bool = false;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::ExecSticky;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut bool_1: bool = true;
    let mut elem_1: color::Elem = crate::color::Elem::INode {valid: bool_1};
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::Group;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut str_0: &str = "PZIGRTU44SCc";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut permissionflag_2: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut permissionflag_2_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_2;
    let mut option_0: std::option::Option<usize> = std::option::Option::None;
    let mut option_1: std::option::Option<usize> = std::option::Option::None;
    let mut tuple_0: () = crate::flags::permission::PermissionFlag::assert_receiver_is_total_eq(permissionflag_2_ref_0);
    let mut usize_0: usize = std::option::Option::unwrap(option_0);
    let mut date_0: crate::color::theme::Date = crate::color::theme::Date {hour_old: color_2, day_old: color_1, older: color_0};
    let mut bool_2: bool = crate::meta::filetype::FileType::is_dirlike(filetype_0);
    let mut elem_3: color::Elem = crate::color::Elem::Dir {uid: bool_0};
    let mut bool_3: bool = crate::flags::permission::PermissionFlag::eq(permissionflag_1_ref_0, permissionflag_0_ref_0);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Group;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_580() {
    rusty_monitor::set_test_id(580);
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut bool_3: bool = false;
    let mut bool_4: bool = true;
    let mut bool_5: bool = true;
    let mut bool_6: bool = false;
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = false;
    let mut bool_10: bool = true;
    let mut bool_11: bool = true;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut permissionflag_0_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_0;
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut permissionflag_1_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_1;
    let mut option_0: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_1: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut option_2: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_0);
    let mut str_0: &str = "mxA5rd";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_3: std::option::Option<flags::permission::PermissionFlag> = crate::flags::permission::PermissionFlag::from_str(str_0_ref_0);
    let mut theme_0: icon::Theme = crate::icon::Theme::Unicode;
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_2, theme: option_1, separator: option_0};
    let mut theme_0_ref_0: &icon::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::TreeEdge;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut tuple_0: () = crate::flags::permission::PermissionFlag::assert_receiver_is_total_eq(permissionflag_1_ref_0);
    let mut elem_1: color::Elem = crate::color::Elem::Write;
    let mut elem_2: color::Elem = crate::color::Elem::Read;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut displayoption_0: meta::name::DisplayOption = crate::meta::name::DisplayOption::None;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut elem_3: color::Elem = crate::color::Elem::User;
    let mut permissionflag_2: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::clone(permissionflag_0_ref_0);
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Byte;
    let mut elem_4: color::Elem = crate::color::Elem::NoAccess;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4562() {
    rusty_monitor::set_test_id(4562);
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut permissionflag_0_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_0: color::Elem = crate::color::Elem::Context;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_1: color::Elem = crate::color::Elem::NonFile;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut bool_0: bool = false;
    let mut elem_2: color::Elem = crate::color::Elem::Links {valid: bool_0};
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_3_ref_0);
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Tera;
    let mut unit_0_ref_0: &meta::size::Unit = &mut unit_0;
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut elem_3: color::Elem = crate::color::Elem::Special;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut date_0: crate::color::theme::Date = crate::color::theme::Date {hour_old: color_2, day_old: color_1, older: color_0};
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_0_ref_0);
    let mut permissionflag_1_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_1;
    let mut bool_1: bool = crate::flags::permission::PermissionFlag::eq(permissionflag_1_ref_0, permissionflag_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1207() {
    rusty_monitor::set_test_id(1207);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::default();
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_0: usize = 35usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut str_0: &str = "MPRjiIqEi";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "70s";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::default();
    let mut permissionflag_1_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_1;
    let mut permissionflag_2: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::clone(permissionflag_1_ref_0);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut filetype_0_ref_0: &meta::filetype::FileType = &mut filetype_0;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Tera;
    let mut result_0: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_1_ref_0);
    let mut tuple_0: () = std::result::Result::unwrap(result_0);
    let mut unit_0_ref_0: &meta::size::Unit = &mut unit_0;
    let mut option_0: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    panic!("From RustyUnit with love");
}
}