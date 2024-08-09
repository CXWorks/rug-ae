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
	use std::cmp::Eq;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_357() {
    rusty_monitor::set_test_id(357);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Acl;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Special;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::Read;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::Context;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_4: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut elem_5: color::Elem = crate::color::Elem::Group;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_5_ref_0);
    let mut theme_6: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_6_ref_0: &crate::color::theme::Theme = &mut theme_6;
    let mut elem_6: color::Elem = crate::color::Elem::Write;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut color_6: crossterm::style::Color = crate::color::Elem::get_color(elem_6_ref_0, theme_6_ref_0);
    let mut symlink_0: crate::color::theme::Symlink = crate::color::theme::Symlink {default: color_6, broken: color_5, missing_target: color_4};
    let mut theme_7: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_7_ref_0: &crate::color::theme::Theme = &mut theme_7;
    let mut elem_7: color::Elem = crate::color::Elem::User;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut theme_8: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_8_ref_0: &crate::color::theme::Theme = &mut theme_8;
    let mut elem_8: color::Elem = crate::color::Elem::Pipe;
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut color_7: crossterm::style::Color = crate::color::Elem::get_color(elem_8_ref_0, theme_8_ref_0);
    let mut theme_9: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_9_ref_0: &crate::color::theme::Theme = &mut theme_9;
    let mut elem_9: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    let mut color_8: crossterm::style::Color = crate::color::Elem::get_color(elem_9_ref_0, theme_9_ref_0);
    let mut dir_0: crate::color::theme::Dir = crate::color::theme::Dir {uid: color_8, no_uid: color_7};
    let mut theme_10: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_10_ref_0: &crate::color::theme::Theme = &mut theme_10;
    let mut elem_10: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_10_ref_0: &color::Elem = &mut elem_10;
    let mut color_9: crossterm::style::Color = crate::color::Elem::get_color(elem_10_ref_0, theme_10_ref_0);
    let mut theme_11: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_11_ref_0: &crate::color::theme::Theme = &mut theme_11;
    let mut elem_11: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_11_ref_0: &color::Elem = &mut elem_11;
    let mut color_10: crossterm::style::Color = crate::color::Elem::get_color(elem_11_ref_0, theme_11_ref_0);
    let mut theme_12: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_12_ref_0: &crate::color::theme::Theme = &mut theme_12;
    let mut bool_0: bool = true;
    let mut elem_12: color::Elem = crate::color::Elem::Dir {uid: bool_0};
    let mut elem_12_ref_0: &color::Elem = &mut elem_12;
    let mut color_11: crossterm::style::Color = crate::color::Elem::get_color(elem_12_ref_0, theme_12_ref_0);
    let mut theme_13: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_13_ref_0: &crate::color::theme::Theme = &mut theme_13;
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut elem_13: color::Elem = crate::color::Elem::File {exec: bool_2, uid: bool_1};
    let mut elem_13_ref_0: &color::Elem = &mut elem_13;
    let mut color_12: crossterm::style::Color = crate::color::Elem::get_color(elem_13_ref_0, theme_13_ref_0);
    let mut file_0: crate::color::theme::File = crate::color::theme::File {exec_uid: color_12, uid_no_exec: color_11, exec_no_uid: color_10, no_exec_no_uid: color_9};
    let mut nosymlink_0: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_0_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_0;
    let mut tuple_0: () = crate::flags::symlinks::NoSymlink::assert_receiver_is_total_eq(nosymlink_0_ref_0);
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    panic!("From RustyUnit with love");
}
}