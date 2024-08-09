//! This module defines the [Layout] flag. To set it up from [ArgMatches], a [Config] and its
//! [Default] value, use its [configure_from](Configurable::configure_from) method.

use crate::config_file::Config;

use super::Configurable;

use clap::ArgMatches;
use serde::Deserialize;

/// The flag showing which output layout to print.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Layout {
    Grid,
    Tree,
    OneLine,
}

impl Configurable<Layout> for Layout {
    /// Get a potential `Layout` variant from [ArgMatches].
    ///
    /// If any of the "tree", "long" or "oneline" arguments is passed, this returns the
    /// corresponding `Layout` variant in a [Some]. Otherwise if the number of passed "blocks"
    /// arguments is greater than 1, this also returns the [OneLine](Layout::OneLine) variant.
    /// Finally if neither of them is passed, this returns [None].
    fn from_arg_matches(matches: &ArgMatches) -> Option<Self> {
        if matches.is_present("tree") {
            Some(Self::Tree)
        } else if matches.is_present("long")
            || matches.is_present("oneline")
            || matches.is_present("inode")
            || matches.is_present("context")
            || matches!(matches.values_of("blocks"), Some(values) if values.len() > 1)
        // TODO: handle this differently
        {
            Some(Self::OneLine)
        } else {
            None
        }
    }

    /// Get a potential Layout variant from a [Config].
    ///
    /// If the `Config::layout` has value and is one of "tree", "oneline" or "grid",
    /// this returns the corresponding `Layout` variant in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        config.layout
    }
}

/// The default value for `Layout` is [Layout::Grid].
impl Default for Layout {
    fn default() -> Self {
        Self::Grid
    }
}

#[cfg(test)]
mod test {
    use super::Layout;

    use crate::app;
    use crate::config_file::Config;
    use crate::flags::Configurable;

    #[test]
    fn test_from_arg_matches_none() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(None, Layout::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_tree() {
        let argv = vec!["lsd", "--tree"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(Some(Layout::Tree), Layout::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_oneline() {
        let argv = vec!["lsd", "--oneline"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(Some(Layout::OneLine), Layout::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_oneline_through_long() {
        let argv = vec!["lsd", "--long"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(Some(Layout::OneLine), Layout::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_oneline_through_blocks() {
        let argv = vec!["lsd", "--blocks", "permission,name"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(Some(Layout::OneLine), Layout::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, Layout::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_tree() {
        let mut c = Config::with_none();
        c.layout = Some(Layout::Tree);
        assert_eq!(Some(Layout::Tree), Layout::from_config(&c));
    }

    #[test]
    fn test_from_config_oneline() {
        let mut c = Config::with_none();
        c.layout = Some(Layout::OneLine);
        assert_eq!(Some(Layout::OneLine), Layout::from_config(&c));
    }

    #[test]
    fn test_from_config_grid() {
        let mut c = Config::with_none();
        c.layout = Some(Layout::Grid);
        assert_eq!(Some(Layout::Grid), Layout::from_config(&c));
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::default::Default;
	use std::clone::Clone;
	use std::cmp::PartialEq;
	use std::cmp::Eq;
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_520() {
//    rusty_monitor::set_test_id(520);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut layout_0_ref_0: &flags::layout::Layout = &mut layout_0;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut layout_1_ref_0: &flags::layout::Layout = &mut layout_1;
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut layout_2_ref_0: &flags::layout::Layout = &mut layout_2;
    let mut layout_3: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut layout_3_ref_0: &flags::layout::Layout = &mut layout_3;
    let mut layout_4: flags::layout::Layout = crate::flags::layout::Layout::default();
    let mut layout_4_ref_0: &flags::layout::Layout = &mut layout_4;
    let mut layout_5: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut layout_5_ref_0: &flags::layout::Layout = &mut layout_5;
    let mut layout_6: flags::layout::Layout = crate::flags::layout::Layout::default();
    let mut layout_6_ref_0: &flags::layout::Layout = &mut layout_6;
    let mut layout_7: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut layout_7_ref_0: &flags::layout::Layout = &mut layout_7;
    let mut layout_8: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut layout_8_ref_0: &flags::layout::Layout = &mut layout_8;
    let mut layout_9: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut layout_9_ref_0: &flags::layout::Layout = &mut layout_9;
    let mut layout_10: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut layout_10_ref_0: &flags::layout::Layout = &mut layout_10;
    let mut layout_11: flags::layout::Layout = crate::flags::layout::Layout::clone(layout_10_ref_0);
    let mut layout_12: flags::layout::Layout = crate::flags::layout::Layout::clone(layout_9_ref_0);
    let mut layout_13: flags::layout::Layout = crate::flags::layout::Layout::clone(layout_8_ref_0);
    let mut layout_14: flags::layout::Layout = crate::flags::layout::Layout::clone(layout_7_ref_0);
    let mut layout_15: flags::layout::Layout = crate::flags::layout::Layout::clone(layout_6_ref_0);
    let mut layout_16: flags::layout::Layout = crate::flags::layout::Layout::clone(layout_5_ref_0);
    let mut layout_17: flags::layout::Layout = crate::flags::layout::Layout::clone(layout_4_ref_0);
    let mut layout_18: flags::layout::Layout = crate::flags::layout::Layout::clone(layout_3_ref_0);
    let mut layout_19: flags::layout::Layout = crate::flags::layout::Layout::clone(layout_2_ref_0);
    let mut layout_20: flags::layout::Layout = crate::flags::layout::Layout::clone(layout_1_ref_0);
    let mut layout_21: flags::layout::Layout = crate::flags::layout::Layout::clone(layout_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_271() {
//    rusty_monitor::set_test_id(271);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::default();
    let mut layout_0_ref_0: &flags::layout::Layout = &mut layout_0;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut layout_1_ref_0: &flags::layout::Layout = &mut layout_1;
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut layout_2_ref_0: &flags::layout::Layout = &mut layout_2;
    let mut layout_3: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut layout_3_ref_0: &flags::layout::Layout = &mut layout_3;
    let mut layout_4: flags::layout::Layout = crate::flags::layout::Layout::default();
    let mut layout_4_ref_0: &flags::layout::Layout = &mut layout_4;
    let mut layout_5: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut layout_5_ref_0: &flags::layout::Layout = &mut layout_5;
    let mut layout_6: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut layout_6_ref_0: &flags::layout::Layout = &mut layout_6;
    let mut layout_7: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut layout_7_ref_0: &flags::layout::Layout = &mut layout_7;
    let mut layout_8: flags::layout::Layout = crate::flags::layout::Layout::default();
    let mut layout_8_ref_0: &flags::layout::Layout = &mut layout_8;
    let mut layout_9: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut layout_9_ref_0: &flags::layout::Layout = &mut layout_9;
    let mut layout_10: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut layout_10_ref_0: &flags::layout::Layout = &mut layout_10;
    let mut layout_11: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut layout_11_ref_0: &flags::layout::Layout = &mut layout_11;
    let mut layout_12: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut layout_12_ref_0: &flags::layout::Layout = &mut layout_12;
    let mut layout_13: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut layout_13_ref_0: &flags::layout::Layout = &mut layout_13;
    let mut bool_0: bool = crate::flags::layout::Layout::eq(layout_13_ref_0, layout_12_ref_0);
    let mut bool_1: bool = crate::flags::layout::Layout::eq(layout_11_ref_0, layout_10_ref_0);
    let mut bool_2: bool = crate::flags::layout::Layout::eq(layout_9_ref_0, layout_8_ref_0);
    let mut bool_3: bool = crate::flags::layout::Layout::eq(layout_7_ref_0, layout_6_ref_0);
    let mut bool_4: bool = crate::flags::layout::Layout::eq(layout_5_ref_0, layout_4_ref_0);
    let mut bool_5: bool = crate::flags::layout::Layout::eq(layout_3_ref_0, layout_2_ref_0);
    let mut bool_6: bool = crate::flags::layout::Layout::eq(layout_1_ref_0, layout_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3006() {
//    rusty_monitor::set_test_id(3006);
    let mut usize_0: usize = 8usize;
    let mut u64_0: u64 = 96u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut option_0: std::option::Option<usize> = std::option::Option::Some(usize_0);
    let mut option_1: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_2: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_3: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_4: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7679() {
//    rusty_monitor::set_test_id(7679);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::default();
    let mut layout_0_ref_0: &flags::layout::Layout = &mut layout_0;
    let mut elem_0: color::Elem = crate::color::Elem::Older;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_0: std::option::Option<bool> = std::option::Option::None;
    let mut option_1: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_2: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut display_1: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut option_3: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut option_4: std::option::Option<bool> = std::option::Option::None;
    let mut option_5: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_6: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut option_7: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut option_8: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut elem_1: color::Elem = crate::color::Elem::Octal;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_2: color::Elem = crate::color::Elem::Pipe;
    let mut tuple_0: () = crate::flags::layout::Layout::assert_receiver_is_total_eq(layout_0_ref_0);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
//    panic!("From RustyUnit with love");
}
}