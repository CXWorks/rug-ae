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
	use std::clone::Clone;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8171() {
    rusty_monitor::set_test_id(8171);
    let mut symlinkarrow_0: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::default();
    let mut symlinkarrow_0_ref_0: &crate::flags::symlink_arrow::SymlinkArrow = &mut symlinkarrow_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut elem_0: color::Elem = crate::color::Elem::TreeEdge;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::HourOld;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Permission;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::Context;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::Exec;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_4: color::Elem = crate::color::Elem::Special;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_5: color::Elem = crate::color::Elem::Context;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_0_ref_0);
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut block_1: flags::blocks::Block = crate::flags::blocks::Block::User;
    let mut elem_6: color::Elem = crate::color::Elem::Socket;
    let mut app_0: clap::App = crate::app::build();
    let mut block_1_ref_0: &flags::blocks::Block = &mut block_1;
    let mut elem_7: color::Elem = crate::color::Elem::DayOld;
    let mut symlinkarrow_1: crate::flags::symlink_arrow::SymlinkArrow = crate::flags::symlink_arrow::SymlinkArrow::clone(symlinkarrow_0_ref_0);
    panic!("From RustyUnit with love");
}
}