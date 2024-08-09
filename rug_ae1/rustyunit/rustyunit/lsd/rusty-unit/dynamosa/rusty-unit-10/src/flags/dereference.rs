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

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6578() {
    rusty_monitor::set_test_id(6578);
    let mut str_0: &str = "H7LwT";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "ojW7y64rK8wB";
    let mut bool_0: bool = false;
    let mut elem_0: color::Elem = crate::color::Elem::FileSmall;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut u64_0: u64 = 68u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut sorting_0_ref_0: &crate::flags::sorting::Sorting = &mut sorting_0;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut bool_1: bool = true;
    let mut bool_2: bool = false;
    let mut elem_1: color::Elem = crate::color::Elem::File {exec: bool_2, uid: bool_1};
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::Context;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut symlink_0: crate::color::theme::Symlink = crate::color::theme::Symlink {default: color_2, broken: color_1, missing_target: color_0};
    let mut symlink_0_ref_0: &crate::color::theme::Symlink = &mut symlink_0;
    let mut elem_4: color::Elem = crate::color::Elem::Acl;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_1, order: sortorder_1, dir_grouping: dirgrouping_1};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_5: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut theme_6: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut elem_6: color::Elem = crate::color::Elem::User;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut elem_7: color::Elem = crate::color::Elem::Links {valid: bool_0};
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut theme_7: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_6_ref_0: &crate::color::theme::Theme = &mut theme_6;
    let mut bool_3: bool = false;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut elem_8: color::Elem = crate::color::Elem::Octal;
    let mut bool_4: bool = false;
    let mut elem_9: color::Elem = crate::color::Elem::File {exec: bool_4, uid: bool_3};
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut option_0: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_1_ref_0: &str = &mut str_1;
    let mut option_1: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut theme_7_ref_0: &crate::color::theme::Theme = &mut theme_7;
    let mut bool_5: bool = crate::color::Elem::has_suid(elem_5_ref_0);
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut elem_10: color::Elem = crate::color::Elem::ExecSticky;
    let mut bool_6: bool = crate::meta::filetype::FileType::is_dirlike(filetype_0);
    let mut elem_11: color::Elem = crate::color::Elem::Exec;
    let mut elem_10_ref_0: &color::Elem = &mut elem_10;
    let mut bool_7: bool = crate::color::Elem::has_suid(elem_7_ref_0);
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_5, exec: bool_6};
    panic!("From RustyUnit with love");
}
}