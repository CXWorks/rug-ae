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
	use std::clone::Clone;
	use std::cmp::Eq;
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3960() {
//    rusty_monitor::set_test_id(3960);
    let mut symlinkarrow_0: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::default();
    let mut symlinkarrow_0_ref_0: &crate::flags::symlink_arrow::SymlinkArrow = &mut symlinkarrow_0;
    let mut symlinkarrow_1: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::default();
    let mut symlinkarrow_1_ref_0: &crate::flags::symlink_arrow::SymlinkArrow = &mut symlinkarrow_1;
    let mut symlinkarrow_2: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::default();
    let mut symlinkarrow_2_ref_0: &crate::flags::symlink_arrow::SymlinkArrow = &mut symlinkarrow_2;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut bool_2: bool = false;
    let mut bool_3: bool = true;
    let mut bool_4: bool = false;
    let mut bool_5: bool = true;
    let mut bool_6: bool = false;
    let mut bool_7: bool = false;
    let mut bool_8: bool = true;
    let mut bool_9: bool = false;
    let mut bool_10: bool = true;
    let mut bool_11: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut bool_12: bool = true;
    let mut bool_13: bool = true;
    let mut bool_14: bool = true;
    let mut bool_15: bool = false;
    let mut bool_16: bool = true;
    let mut bool_17: bool = false;
    let mut bool_18: bool = true;
    let mut bool_19: bool = false;
    let mut bool_20: bool = true;
    let mut bool_21: bool = false;
    let mut bool_22: bool = false;
    let mut bool_23: bool = false;
    let mut permissions_1: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_23, user_write: bool_22, user_execute: bool_21, group_read: bool_20, group_write: bool_19, group_execute: bool_18, other_read: bool_17, other_write: bool_16, other_execute: bool_15, sticky: bool_14, setgid: bool_13, setuid: bool_12};
    let mut permissions_1_ref_0: &crate::meta::permissions::Permissions = &mut permissions_1;
    let mut bool_24: bool = crate::flags::symlink_arrow::SymlinkArrow::ne(symlinkarrow_2_ref_0, symlinkarrow_1_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_481() {
//    rusty_monitor::set_test_id(481);
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
#[timeout(30000)]fn rusty_test_642() {
//    rusty_monitor::set_test_id(642);
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
    let mut symlinkarrow_11: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::clone(symlinkarrow_10_ref_0);
    let mut symlinkarrow_12: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::clone(symlinkarrow_9_ref_0);
    let mut symlinkarrow_13: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::clone(symlinkarrow_8_ref_0);
    let mut symlinkarrow_14: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::clone(symlinkarrow_7_ref_0);
    let mut symlinkarrow_15: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::clone(symlinkarrow_6_ref_0);
    let mut symlinkarrow_16: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::clone(symlinkarrow_5_ref_0);
    let mut symlinkarrow_17: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::clone(symlinkarrow_4_ref_0);
    let mut symlinkarrow_18: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::clone(symlinkarrow_3_ref_0);
    let mut symlinkarrow_19: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::clone(symlinkarrow_2_ref_0);
    let mut symlinkarrow_20: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::clone(symlinkarrow_1_ref_0);
    let mut symlinkarrow_21: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::clone(symlinkarrow_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_263() {
//    rusty_monitor::set_test_id(263);
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
}