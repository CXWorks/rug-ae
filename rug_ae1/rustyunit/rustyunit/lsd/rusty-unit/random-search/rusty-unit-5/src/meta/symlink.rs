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
	use std::clone::Clone;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3537() {
    rusty_monitor::set_test_id(3537);
    let mut bool_0: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_0};
    let mut usize_0: usize = 95usize;
    let mut bool_1: bool = true;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::HourOld;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::Special;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_4: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut option_0: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_0);
    let mut option_1: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut bool_5: bool = false;
    let mut bool_6: bool = false;
    let mut bool_7: bool = false;
    let mut bool_8: bool = false;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = true;
    let mut bool_12: bool = true;
    let mut bool_13: bool = false;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut bool_14: bool = false;
    let mut option_2: std::option::Option<std::string::String> = std::option::Option::None;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut bool_15: bool = false;
    let mut option_3: std::option::Option<std::string::String> = std::option::Option::None;
    let mut symlink_0: crate::meta::symlink::SymLink = crate::meta::symlink::SymLink {target: option_3, valid: bool_15};
    let mut symlink_0_ref_0: &crate::meta::symlink::SymLink = &mut symlink_0;
    let mut option_4: std::option::Option<std::string::String> = crate::meta::symlink::SymLink::symlink_string(symlink_0_ref_0);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut bool_16: bool = crate::meta::filetype::FileType::is_dirlike(filetype_1);
    let mut symlink_1: crate::meta::symlink::SymLink = crate::meta::symlink::SymLink {target: option_2, valid: bool_14};
    let mut theme_5: icon::Theme = crate::icon::Theme::Unicode;
    let mut elem_5: color::Elem = crate::color::Elem::SymLink;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_13, user_write: bool_12, user_execute: bool_11, group_read: bool_10, group_write: bool_9, group_execute: bool_8, other_read: bool_7, other_write: bool_6, other_execute: bool_5, sticky: bool_4, setgid: bool_3, setuid: bool_2};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut symlink_1_ref_0: &crate::meta::symlink::SymLink = &mut symlink_1;
    let mut symlink_2: crate::meta::symlink::SymLink = crate::meta::symlink::SymLink::clone(symlink_1_ref_0);
    let mut elem_6: color::Elem = crate::color::Elem::HourOld;
    let mut elem_7: color::Elem = crate::color::Elem::CharDevice;
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut bool_17: bool = crate::meta::permissions::Permissions::is_executable(permissions_0_ref_0);
    let mut elem_8: color::Elem = crate::color::Elem::Group;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut color_4: crate::config_file::Color = crate::config_file::Color {when: option_1, theme: option_0};
    let mut file_0: crate::color::theme::File = crate::color::theme::File {exec_uid: color_3, uid_no_exec: color_2, exec_no_uid: color_1, no_exec_no_uid: color_0};
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_1, depth: usize_0};
    let mut elem_9: color::Elem = crate::color::Elem::Socket;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut elem_10: color::Elem = crate::color::Elem::FileSmall;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    panic!("From RustyUnit with love");
}
}