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
	use std::cmp::Eq;
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_378() {
//    rusty_monitor::set_test_id(378);
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut permissionflag_0_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_0;
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut permissionflag_1_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_1;
    let mut permissionflag_2: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut permissionflag_2_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_2;
    let mut permissionflag_3: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::default();
    let mut permissionflag_3_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_3;
    let mut permissionflag_4: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut permissionflag_4_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_4;
    let mut permissionflag_5: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut permissionflag_5_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_5;
    let mut permissionflag_6: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::default();
    let mut permissionflag_6_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_6;
    let mut permissionflag_7: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut permissionflag_7_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_7;
    let mut permissionflag_8: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut permissionflag_8_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_8;
    let mut permissionflag_9: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut permissionflag_9_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_9;
    let mut permissionflag_10: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::default();
    let mut permissionflag_10_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_10;
    let mut permissionflag_11: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut permissionflag_11_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_11;
    let mut permissionflag_12: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::default();
    let mut permissionflag_12_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_12;
    let mut permissionflag_13: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut permissionflag_13_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_13;
    let mut bool_0: bool = crate::flags::permission::PermissionFlag::eq(permissionflag_13_ref_0, permissionflag_12_ref_0);
    let mut bool_1: bool = crate::flags::permission::PermissionFlag::eq(permissionflag_11_ref_0, permissionflag_10_ref_0);
    let mut bool_2: bool = crate::flags::permission::PermissionFlag::eq(permissionflag_9_ref_0, permissionflag_8_ref_0);
    let mut bool_3: bool = crate::flags::permission::PermissionFlag::eq(permissionflag_7_ref_0, permissionflag_6_ref_0);
    let mut bool_4: bool = crate::flags::permission::PermissionFlag::eq(permissionflag_5_ref_0, permissionflag_4_ref_0);
    let mut bool_5: bool = crate::flags::permission::PermissionFlag::eq(permissionflag_3_ref_0, permissionflag_2_ref_0);
    let mut bool_6: bool = crate::flags::permission::PermissionFlag::eq(permissionflag_1_ref_0, permissionflag_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_407() {
//    rusty_monitor::set_test_id(407);
    let mut str_0: &str = "nZGfS";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "%F %T.%f %z";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "woff2";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "hpp";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "ru";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "ExecSticky";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "selinux_context";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7: &str = "How to display size";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_8: &str = "wBkjboEkzFz66";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut str_9: &str = "ln";
    let mut str_9_ref_0: &str = &mut str_9;
    let mut str_10: &str = "Ev";
    let mut str_10_ref_0: &str = &mut str_10;
    let mut option_0: std::option::Option<flags::permission::PermissionFlag> = crate::flags::permission::PermissionFlag::from_str(str_10_ref_0);
    let mut option_1: std::option::Option<flags::permission::PermissionFlag> = crate::flags::permission::PermissionFlag::from_str(str_9_ref_0);
    let mut option_2: std::option::Option<flags::permission::PermissionFlag> = crate::flags::permission::PermissionFlag::from_str(str_8_ref_0);
    let mut option_3: std::option::Option<flags::permission::PermissionFlag> = crate::flags::permission::PermissionFlag::from_str(str_7_ref_0);
    let mut option_4: std::option::Option<flags::permission::PermissionFlag> = crate::flags::permission::PermissionFlag::from_str(str_6_ref_0);
    let mut option_5: std::option::Option<flags::permission::PermissionFlag> = crate::flags::permission::PermissionFlag::from_str(str_5_ref_0);
    let mut option_6: std::option::Option<flags::permission::PermissionFlag> = crate::flags::permission::PermissionFlag::from_str(str_4_ref_0);
    let mut option_7: std::option::Option<flags::permission::PermissionFlag> = crate::flags::permission::PermissionFlag::from_str(str_3_ref_0);
    let mut option_8: std::option::Option<flags::permission::PermissionFlag> = crate::flags::permission::PermissionFlag::from_str(str_2_ref_0);
    let mut option_9: std::option::Option<flags::permission::PermissionFlag> = crate::flags::permission::PermissionFlag::from_str(str_1_ref_0);
    let mut option_10: std::option::Option<flags::permission::PermissionFlag> = crate::flags::permission::PermissionFlag::from_str(str_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_687() {
//    rusty_monitor::set_test_id(687);
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut permissionflag_0_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_0;
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut permissionflag_1_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_1;
    let mut permissionflag_2: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::default();
    let mut permissionflag_2_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_2;
    let mut permissionflag_3: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut permissionflag_3_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_3;
    let mut permissionflag_4: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut permissionflag_4_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_4;
    let mut permissionflag_5: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut permissionflag_5_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_5;
    let mut permissionflag_6: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut permissionflag_6_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_6;
    let mut permissionflag_7: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut permissionflag_7_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_7;
    let mut permissionflag_8: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::default();
    let mut permissionflag_8_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_8;
    let mut permissionflag_9: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut permissionflag_9_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_9;
    let mut permissionflag_10: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut permissionflag_10_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_10;
    let mut tuple_0: () = crate::flags::permission::PermissionFlag::assert_receiver_is_total_eq(permissionflag_10_ref_0);
    let mut tuple_1: () = crate::flags::permission::PermissionFlag::assert_receiver_is_total_eq(permissionflag_9_ref_0);
    let mut tuple_2: () = crate::flags::permission::PermissionFlag::assert_receiver_is_total_eq(permissionflag_8_ref_0);
    let mut tuple_3: () = crate::flags::permission::PermissionFlag::assert_receiver_is_total_eq(permissionflag_7_ref_0);
    let mut tuple_4: () = crate::flags::permission::PermissionFlag::assert_receiver_is_total_eq(permissionflag_6_ref_0);
    let mut tuple_5: () = crate::flags::permission::PermissionFlag::assert_receiver_is_total_eq(permissionflag_5_ref_0);
    let mut tuple_6: () = crate::flags::permission::PermissionFlag::assert_receiver_is_total_eq(permissionflag_4_ref_0);
    let mut tuple_7: () = crate::flags::permission::PermissionFlag::assert_receiver_is_total_eq(permissionflag_3_ref_0);
    let mut tuple_8: () = crate::flags::permission::PermissionFlag::assert_receiver_is_total_eq(permissionflag_2_ref_0);
    let mut tuple_9: () = crate::flags::permission::PermissionFlag::assert_receiver_is_total_eq(permissionflag_1_ref_0);
    let mut tuple_10: () = crate::flags::permission::PermissionFlag::assert_receiver_is_total_eq(permissionflag_0_ref_0);
//    panic!("From RustyUnit with love");
}
}