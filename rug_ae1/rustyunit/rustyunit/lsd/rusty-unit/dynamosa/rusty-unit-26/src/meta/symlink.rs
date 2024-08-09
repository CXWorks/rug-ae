use crate::color::{ColoredString, Colors, Elem};
use crate::flags::Flags;
use std::fs::read_link;
use std::path::Path;

#[derive(Clone, Debug)]
pub struct SymLink {
    target: Option<String>,
    valid: bool,
}

impl<'a> From<&'a Path> for SymLink {
    fn from(path: &'a Path) -> Self {
        if let Ok(target) = read_link(path) {
            if target.is_absolute() || path.parent() == None {
                return Self {
                    valid: target.exists(),
                    target: Some(
                        target
                            .to_str()
                            .expect("failed to convert symlink to str")
                            .to_string(),
                    ),
                };
            }

            return Self {
                target: Some(
                    target
                        .to_str()
                        .expect("failed to convert symlink to str")
                        .to_string(),
                ),
                valid: path.parent().unwrap().join(target).exists(),
            };
        }

        Self {
            target: None,
            valid: false,
        }
    }
}

impl SymLink {
    pub fn symlink_string(&self) -> Option<String> {
        self.target.as_ref().map(|target| target.to_string())
    }

    pub fn render(&self, colors: &Colors, flag: &Flags) -> ColoredString {
        if let Some(target_string) = self.symlink_string() {
            let elem = if self.valid {
                &Elem::SymLink
            } else {
                &Elem::MissingSymLinkTarget
            };

            let strings: &[ColoredString] = &[
                ColoredString::new(Colors::default_style(), format!(" {} ", flag.symlink_arrow)), // ⇒ \u{21d2}
                colors.colorize(target_string, elem),
            ];

            let res = strings
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join("");
            ColoredString::new(Colors::default_style(), res)
        } else {
            ColoredString::new(Colors::default_style(), "".into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SymLink;
    use crate::app;
    use crate::color::{Colors, ThemeOption};
    use crate::config_file::Config;
    use crate::flags::Flags;

    #[test]
    fn test_symlink_render_default_valid_target_nocolor() {
        let link = SymLink {
            target: Some("/target".to_string()),
            valid: true,
        };
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            format!("{}", " ⇒ /target"),
            link.render(
                &Colors::new(ThemeOption::NoColor),
                &Flags::configure_from(&matches, &Config::with_none()).unwrap()
            )
            .to_string()
        );
    }

    #[test]
    fn test_symlink_render_default_invalid_target_nocolor() {
        let link = SymLink {
            target: Some("/target".to_string()),
            valid: false,
        };
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            format!("{}", " ⇒ /target"),
            link.render(
                &Colors::new(ThemeOption::NoColor),
                &Flags::configure_from(&matches, &Config::with_none()).unwrap()
            )
            .to_string()
        );
    }

    #[test]
    fn test_symlink_render_default_invalid_target_withcolor() {
        let link = SymLink {
            target: Some("/target".to_string()),
            valid: false,
        };
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            format!("{}", " ⇒ \u{1b}[38;5;124m/target\u{1b}[39m"),
            link.render(
                &Colors::new(ThemeOption::NoLscolors),
                &Flags::configure_from(&matches, &Config::with_none()).unwrap()
            )
            .to_string()
        );
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5644() {
    rusty_monitor::set_test_id(5644);
    let mut str_0: &str = "oiKd7RrVdAo";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut str_1: &str = "wSiG";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_1, exec: bool_0};
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut option_0: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_2);
    let mut option_1: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_1, theme: option_0};
    let mut option_2: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_3: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_2: bool = false;
    let mut option_4: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut str_2: &str = "FF1uK";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_4, exec: bool_3};
    let mut bool_5: bool = false;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_5};
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_3};
    let mut u64_0: u64 = 1689u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut bool_6: bool = false;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_6};
    let mut elem_0: color::Elem = crate::color::Elem::User;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_1: color::Elem = crate::color::Elem::SymLink;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut themeoption_4: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut themeoption_5: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_4);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut bool_7: bool = true;
    let mut filetype_4: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_7};
    let mut filetype_5: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_2: color::Elem = crate::color::Elem::Read;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_3: color::Elem = crate::color::Elem::Special;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_4: color::Elem = crate::color::Elem::Octal;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_2_ref_0);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Context;
    let mut block_0_ref_0: &flags::blocks::Block = &mut block_0;
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_5: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut block_1: flags::blocks::Block = crate::flags::blocks::Block::Context;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut block_1_ref_0: &flags::blocks::Block = &mut block_1;
    let mut elem_6: color::Elem = crate::color::Elem::ExecSticky;
    let mut filetype_6: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut layout_0_ref_0: &flags::layout::Layout = &mut layout_0;
    let mut symlink_0: crate::color::theme::Symlink = crate::color::theme::Symlink {default: color_4, broken: color_3, missing_target: color_2};
    let mut filetype_7: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    crate::meta::filetype::FileType::render(filetype_4, colors_1_ref_0);
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_5);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    panic!("From RustyUnit with love");
}
}