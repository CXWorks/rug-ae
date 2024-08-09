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
#[timeout(30000)]fn rusty_test_489() {
//    rusty_monitor::set_test_id(489);
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
#[timeout(30000)]fn rusty_test_117() {
//    rusty_monitor::set_test_id(117);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::NonFile;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Write;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut elem_3: color::Elem = crate::color::Elem::File {exec: bool_1, uid: bool_0};
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_4: color::Elem = crate::color::Elem::NonFile;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut elem_5: color::Elem = crate::color::Elem::Older;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_5_ref_0);
    let mut theme_6: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_6_ref_0: &crate::color::theme::Theme = &mut theme_6;
    let mut elem_6: color::Elem = crate::color::Elem::HourOld;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut color_6: crossterm::style::Color = crate::color::Elem::get_color(elem_6_ref_0, theme_6_ref_0);
    let mut theme_7: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_7_ref_0: &crate::color::theme::Theme = &mut theme_7;
    let mut elem_7: color::Elem = crate::color::Elem::Older;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut color_7: crossterm::style::Color = crate::color::Elem::get_color(elem_7_ref_0, theme_7_ref_0);
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut option_0: std::option::Option<crate::flags::symlink_arrow::SymlinkArrow> = crate::flags::symlink_arrow::SymlinkArrow::from_config(config_0_ref_0);
    let mut permission_0: crate::color::theme::Permission = crate::color::theme::Permission {read: color_7, write: color_6, exec: color_5, exec_sticky: color_4, no_access: color_3, octal: color_2, acl: color_1, context: color_0};
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_610() {
//    rusty_monitor::set_test_id(610);
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
#[timeout(30000)]fn rusty_test_172() {
//    rusty_monitor::set_test_id(172);
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