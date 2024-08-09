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
fn rusty_test_7041() {
    rusty_monitor::set_test_id(7041);
    let mut str_0: &str = "eV5437dNz9QA";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut option_0: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_0);
    let mut bool_0: bool = true;
    let mut option_1: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut option_2: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_3: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut option_4: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut bool_1: bool = true;
    let mut option_5: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_6: std::option::Option<std::string::String> = std::option::Option::None;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut option_7: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_0);
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut option_8: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_8, theme: option_7};
    let mut option_9: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_10: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_2: bool = true;
    let mut option_11: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut str_1: &str = "209Ybj2zwI6";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut usize_0: usize = 63usize;
    let mut tuple_0: (usize, &str) = (usize_0, str_2_ref_0);
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_1: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_1};
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut option_12: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut theme_0: icon::Theme = crate::icon::Theme::Unicode;
    let mut option_13: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_1_ref_0);
    let mut theme_1: icon::Theme = crate::icon::Theme::Fancy;
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut app_0: clap::App = crate::app::build();
    let mut bool_3: bool = std::option::Option::unwrap(option_5);
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut result_0: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_0_ref_0);
    let mut bool_4: bool = std::option::Option::unwrap(option_11);
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut elem_0: color::Elem = crate::color::Elem::Group;
    let mut elem_1: color::Elem = crate::color::Elem::CharDevice;
    let mut sortcolumn_2: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    panic!("From RustyUnit with love");
}
}