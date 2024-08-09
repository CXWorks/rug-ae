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
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_83() {
    rusty_monitor::set_test_id(83);
    let mut str_0: &str = "Mn3MnlIm5";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "82L71jlnR";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut option_0: std::option::Option<bool> = std::option::Option::None;
    let mut option_1: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut option_2: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut option_3: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_4: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_5: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut option_6: std::option::Option<bool> = std::option::Option::None;
    let mut option_7: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_8: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut option_9: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut bool_0: bool = true;
    let mut option_10: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut option_11: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_12: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_13: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_1: bool = false;
    let mut option_14: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::default();
    let mut usize_0: usize = 8868usize;
    let mut bool_2: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_2, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut display_1: flags::display::Display = crate::flags::display::Display::All;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut option_15: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_0);
    let mut option_16: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_17: std::option::Option<bool> = std::option::Option::None;
    let mut option_18: std::option::Option<bool> = std::option::Option::None;
    let mut option_19: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut option_20: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut option_21: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_0);
    let mut option_22: std::option::Option<usize> = std::option::Option::None;
    let mut option_23: std::option::Option<bool> = std::option::Option::None;
    let mut recursion_1: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_23, depth: option_22};
    let mut option_24: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_1);
    let mut option_25: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_3: bool = false;
    let mut option_26: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut option_27: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_28: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_29: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_30: std::option::Option<bool> = std::option::Option::None;
    let mut option_31: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_32: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_33: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_34: std::option::Option<bool> = std::option::Option::None;
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_34, blocks: option_33, color: option_32, date: option_31, dereference: option_30, display: option_29, icons: option_28, ignore_globs: option_27, indicators: option_26, layout: option_25, recursion: option_24, size: option_21, permission: option_20, sorting: option_19, no_symlink: option_18, total_size: option_17, symlink_arrow: option_16, hyperlink: option_15};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut bool_4: bool = true;
    let mut bool_5: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_5, exec: bool_4};
    let mut bool_6: bool = false;
    let mut bool_7: bool = true;
    let mut bool_8: bool = false;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = true;
    let mut bool_12: bool = true;
    let mut bool_13: bool = true;
    let mut bool_14: bool = false;
    let mut bool_15: bool = true;
    let mut bool_16: bool = false;
    let mut bool_17: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_17, user_write: bool_16, user_execute: bool_15, group_read: bool_14, group_write: bool_13, group_execute: bool_12, other_read: bool_11, other_write: bool_10, other_execute: bool_9, sticky: bool_8, setgid: bool_7, setuid: bool_6};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut option_35: std::option::Option<flags::permission::PermissionFlag> = crate::flags::permission::PermissionFlag::from_str(str_0_ref_0);
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    panic!("From RustyUnit with love");
}
}