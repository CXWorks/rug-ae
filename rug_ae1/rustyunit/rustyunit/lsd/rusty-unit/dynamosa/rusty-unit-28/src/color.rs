pub mod theme;

use crossterm::style::{Attribute, ContentStyle, StyledContent, Stylize};
use theme::Theme;

pub use crate::flags::color::ThemeOption;

use crossterm::style::Color;
use lscolors::{Indicator, LsColors};
use std::path::Path;

#[allow(dead_code)]
#[derive(Hash, Debug, Eq, PartialEq, Clone)]
pub enum Elem {
    /// Node type
    File {
        exec: bool,
        uid: bool,
    },
    SymLink,
    BrokenSymLink,
    MissingSymLinkTarget,
    Dir {
        uid: bool,
    },
    Pipe,
    BlockDevice,
    CharDevice,
    Socket,
    Special,

    /// Permission
    Read,
    Write,
    Exec,
    ExecSticky,
    NoAccess,
    Octal,
    Acl,
    Context,

    /// Last Time Modified
    DayOld,
    HourOld,
    Older,

    /// User / Group Name
    User,
    Group,

    /// File Size
    NonFile,
    FileLarge,
    FileMedium,
    FileSmall,

    /// INode
    INode {
        valid: bool,
    },

    Links {
        valid: bool,
    },

    TreeEdge,
}

impl Elem {
    pub fn has_suid(&self) -> bool {
        matches!(self, Elem::Dir { uid: true } | Elem::File { uid: true, .. })
    }

    pub fn get_color(&self, theme: &theme::Theme) -> Color {
        match self {
            Elem::File {
                exec: true,
                uid: true,
            } => theme.file_type.file.exec_uid,
            Elem::File {
                exec: false,
                uid: true,
            } => theme.file_type.file.uid_no_exec,
            Elem::File {
                exec: true,
                uid: false,
            } => theme.file_type.file.exec_no_uid,
            Elem::File {
                exec: false,
                uid: false,
            } => theme.file_type.file.no_exec_no_uid,
            Elem::SymLink => theme.file_type.symlink.default,
            Elem::BrokenSymLink => theme.file_type.symlink.broken,
            Elem::MissingSymLinkTarget => theme.file_type.symlink.missing_target,
            Elem::Dir { uid: true } => theme.file_type.dir.uid,
            Elem::Dir { uid: false } => theme.file_type.dir.no_uid,
            Elem::Pipe => theme.file_type.pipe,
            Elem::BlockDevice => theme.file_type.block_device,
            Elem::CharDevice => theme.file_type.char_device,
            Elem::Socket => theme.file_type.socket,
            Elem::Special => theme.file_type.special,

            Elem::Read => theme.permission.read,
            Elem::Write => theme.permission.write,
            Elem::Exec => theme.permission.exec,
            Elem::ExecSticky => theme.permission.exec_sticky,
            Elem::NoAccess => theme.permission.no_access,
            Elem::Octal => theme.permission.octal,
            Elem::Acl => theme.permission.acl,
            Elem::Context => theme.permission.context,

            Elem::DayOld => theme.date.day_old,
            Elem::HourOld => theme.date.hour_old,
            Elem::Older => theme.date.older,

            Elem::User => theme.user,
            Elem::Group => theme.group,
            Elem::NonFile => theme.size.none,
            Elem::FileLarge => theme.size.large,
            Elem::FileMedium => theme.size.medium,
            Elem::FileSmall => theme.size.small,
            Elem::INode { valid: false } => theme.inode.valid,
            Elem::INode { valid: true } => theme.inode.invalid,
            Elem::TreeEdge => theme.tree_edge,
            Elem::Links { valid: false } => theme.links.invalid,
            Elem::Links { valid: true } => theme.links.valid,
        }
    }
}

pub type ColoredString = StyledContent<String>;

pub struct Colors {
    theme: Option<Theme>,
    lscolors: Option<LsColors>,
}

impl Colors {
    pub fn new(t: ThemeOption) -> Self {
        let theme = match t {
            ThemeOption::NoColor => None,
            ThemeOption::Default => Some(Theme::default()),
            ThemeOption::NoLscolors => Some(Theme::default()),
            ThemeOption::Custom(ref file) => Some(Theme::from_path(file).unwrap_or_default()),
        };
        let lscolors = match t {
            ThemeOption::Default => Some(LsColors::from_env().unwrap_or_default()),
            ThemeOption::Custom(_) => Some(LsColors::from_env().unwrap_or_default()),
            _ => None,
        };

        Self { theme, lscolors }
    }

    pub fn colorize(&self, input: String, elem: &Elem) -> ColoredString {
        self.style(elem).apply(input)
    }

    pub fn colorize_using_path(&self, input: String, path: &Path, elem: &Elem) -> ColoredString {
        let style_from_path = self.style_from_path(path);
        match style_from_path {
            Some(style_from_path) => style_from_path.apply(input),
            None => self.colorize(input, elem),
        }
    }

    pub fn default_style() -> ContentStyle {
        ContentStyle::default()
    }

    fn style_from_path(&self, path: &Path) -> Option<ContentStyle> {
        match &self.lscolors {
            Some(lscolors) => lscolors.style_for_path(path).map(to_content_style),
            None => None,
        }
    }

    fn style(&self, elem: &Elem) -> ContentStyle {
        match &self.lscolors {
            Some(lscolors) => match self.get_indicator_from_elem(elem) {
                Some(style) => {
                    let style = lscolors.style_for_indicator(style);
                    style.map(to_content_style).unwrap_or_default()
                }
                None => self.style_default(elem),
            },
            None => self.style_default(elem),
        }
    }

    fn style_default(&self, elem: &Elem) -> ContentStyle {
        if let Some(t) = &self.theme {
            let style_fg = ContentStyle::default().with(elem.get_color(t));
            if elem.has_suid() {
                style_fg.on(Color::AnsiValue(124)) // Red3
            } else {
                style_fg
            }
        } else {
            ContentStyle::default()
        }
    }

    fn get_indicator_from_elem(&self, elem: &Elem) -> Option<Indicator> {
        let indicator_string = match elem {
            Elem::File { exec, uid } => match (exec, uid) {
                (_, true) => None,
                (true, false) => Some("ex"),
                (false, false) => Some("fi"),
            },
            Elem::Dir { uid } => {
                if *uid {
                    None
                } else {
                    Some("di")
                }
            }
            Elem::SymLink => Some("ln"),
            Elem::Pipe => Some("pi"),
            Elem::Socket => Some("so"),
            Elem::BlockDevice => Some("bd"),
            Elem::CharDevice => Some("cd"),
            Elem::BrokenSymLink => Some("or"),
            Elem::MissingSymLinkTarget => Some("mi"),
            Elem::INode { valid } => match valid {
                true => Some("so"),
                false => Some("no"),
            },
            Elem::Links { valid } => match valid {
                true => Some("so"),
                false => Some("no"),
            },
            _ => None,
        };

        match indicator_string {
            Some(ids) => Indicator::from(ids),
            None => None,
        }
    }
}

fn to_content_style(ls: &lscolors::Style) -> ContentStyle {
    let to_crossterm_color = |c: &lscolors::Color| match c {
        lscolors::style::Color::RGB(r, g, b) => Color::Rgb {
            r: *r,
            g: *g,
            b: *b,
        },
        lscolors::style::Color::Fixed(n) => Color::AnsiValue(*n),
        lscolors::style::Color::Black => Color::Black,
        lscolors::style::Color::Red => Color::DarkRed,
        lscolors::style::Color::Green => Color::DarkGreen,
        lscolors::style::Color::Yellow => Color::DarkYellow,
        lscolors::style::Color::Blue => Color::DarkBlue,
        lscolors::style::Color::Magenta => Color::DarkMagenta,
        lscolors::style::Color::Cyan => Color::DarkCyan,
        lscolors::style::Color::White => Color::Grey,
        lscolors::style::Color::BrightBlack => Color::DarkGrey,
        lscolors::style::Color::BrightRed => Color::Red,
        lscolors::style::Color::BrightGreen => Color::Green,
        lscolors::style::Color::BrightYellow => Color::Yellow,
        lscolors::style::Color::BrightBlue => Color::Blue,
        lscolors::style::Color::BrightMagenta => Color::Magenta,
        lscolors::style::Color::BrightCyan => Color::Cyan,
        lscolors::style::Color::BrightWhite => Color::White,
    };
    let mut style = ContentStyle {
        foreground_color: ls.foreground.as_ref().map(to_crossterm_color),
        background_color: ls.background.as_ref().map(to_crossterm_color),
        ..ContentStyle::default()
    };

    if ls.font_style.bold {
        style.attributes.set(Attribute::Bold);
    }
    if ls.font_style.dimmed {
        style.attributes.set(Attribute::Dim);
    }
    if ls.font_style.italic {
        style.attributes.set(Attribute::Italic);
    }
    if ls.font_style.underline {
        style.attributes.set(Attribute::Underlined);
    }
    if ls.font_style.rapid_blink {
        style.attributes.set(Attribute::RapidBlink);
    }
    if ls.font_style.slow_blink {
        style.attributes.set(Attribute::SlowBlink);
    }
    if ls.font_style.reverse {
        style.attributes.set(Attribute::Reverse);
    }
    if ls.font_style.hidden {
        style.attributes.set(Attribute::Hidden);
    }
    if ls.font_style.strikethrough {
        style.attributes.set(Attribute::CrossedOut);
    }

    style
}

#[cfg(test)]
mod tests {
    use super::Colors;
    use crate::color::Theme;
    use crate::color::ThemeOption;
    #[test]
    fn test_color_new_no_color_theme() {
        assert!(Colors::new(ThemeOption::NoColor).theme.is_none());
    }

    #[test]
    fn test_color_new_default_theme() {
        assert_eq!(
            Colors::new(ThemeOption::Default).theme,
            Some(Theme::default_dark()),
        );
    }

    #[test]
    fn test_color_new_bad_custom_theme() {
        assert_eq!(
            Colors::new(ThemeOption::Custom("not-existed".to_string())).theme,
            Some(Theme::default_dark()),
        );
    }
}

#[cfg(test)]
mod elem {
    use super::Elem;
    use crate::color::{theme, Theme};
    use crossterm::style::Color;

    #[cfg(test)]
    fn test_theme() -> Theme {
        Theme {
            user: Color::AnsiValue(230),  // Cornsilk1
            group: Color::AnsiValue(187), // LightYellow3
            permission: theme::Permission {
                read: Color::Green,
                write: Color::Yellow,
                exec: Color::Red,
                exec_sticky: Color::Magenta,
                no_access: Color::AnsiValue(245), // Grey
                octal: Color::AnsiValue(6),
                acl: Color::DarkCyan,
                context: Color::Cyan,
            },
            file_type: theme::FileType {
                file: theme::File {
                    exec_uid: Color::AnsiValue(40),        // Green3
                    uid_no_exec: Color::AnsiValue(184),    // Yellow3
                    exec_no_uid: Color::AnsiValue(40),     // Green3
                    no_exec_no_uid: Color::AnsiValue(184), // Yellow3
                },
                dir: theme::Dir {
                    uid: Color::AnsiValue(33),    // DodgerBlue1
                    no_uid: Color::AnsiValue(33), // DodgerBlue1
                },
                pipe: Color::AnsiValue(44), // DarkTurquoise
                symlink: theme::Symlink {
                    default: Color::AnsiValue(44),         // DarkTurquoise
                    broken: Color::AnsiValue(124),         // Red3
                    missing_target: Color::AnsiValue(124), // Red3
                },
                block_device: Color::AnsiValue(44), // DarkTurquoise
                char_device: Color::AnsiValue(172), // Orange3
                socket: Color::AnsiValue(44),       // DarkTurquoise
                special: Color::AnsiValue(44),      // DarkTurquoise
            },
            date: theme::Date {
                hour_old: Color::AnsiValue(40), // Green3
                day_old: Color::AnsiValue(42),  // SpringGreen2
                older: Color::AnsiValue(36),    // DarkCyan
            },
            size: theme::Size {
                none: Color::AnsiValue(245),   // Grey
                small: Color::AnsiValue(229),  // Wheat1
                medium: Color::AnsiValue(216), // LightSalmon1
                large: Color::AnsiValue(172),  // Orange3
            },
            inode: theme::INode {
                valid: Color::AnsiValue(13),    // Pink
                invalid: Color::AnsiValue(245), // Grey
            },
            links: theme::Links {
                valid: Color::AnsiValue(13),    // Pink
                invalid: Color::AnsiValue(245), // Grey
            },
            tree_edge: Color::AnsiValue(245), // Grey
        }
    }

    #[test]
    fn test_default_theme_color() {
        assert_eq!(
            Elem::File {
                exec: true,
                uid: true
            }
            .get_color(&test_theme()),
            Color::AnsiValue(40),
        );
        assert_eq!(
            Elem::File {
                exec: false,
                uid: true
            }
            .get_color(&test_theme()),
            Color::AnsiValue(184),
        );
        assert_eq!(
            Elem::File {
                exec: true,
                uid: false
            }
            .get_color(&test_theme()),
            Color::AnsiValue(40),
        );
        assert_eq!(
            Elem::File {
                exec: false,
                uid: false
            }
            .get_color(&test_theme()),
            Color::AnsiValue(184),
        );
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::cmp::PartialEq;
	use std::clone::Clone;
	use std::cmp::Eq;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1607() {
    rusty_monitor::set_test_id(1607);
    let mut str_0: &str = "7yHm";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut bool_0: bool = true;
    let mut elem_0: color::Elem = crate::color::Elem::Dir {uid: bool_0};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut bool_1: bool = false;
    let mut elem_3: color::Elem = crate::color::Elem::Dir {uid: bool_1};
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_4: color::Elem = crate::color::Elem::SymLink;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut elem_5: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_5_ref_0);
    let mut theme_6: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_6_ref_0: &crate::color::theme::Theme = &mut theme_6;
    let mut elem_6: color::Elem = crate::color::Elem::Write;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut color_6: crossterm::style::Color = crate::color::Elem::get_color(elem_6_ref_0, theme_6_ref_0);
    let mut theme_7: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_7_ref_0: &crate::color::theme::Theme = &mut theme_7;
    let mut elem_7: color::Elem = crate::color::Elem::Exec;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut color_7: crossterm::style::Color = crate::color::Elem::get_color(elem_7_ref_0, theme_7_ref_0);
    let mut bool_2: bool = true;
    let mut str_1: &str = "tck8xXTg04PC";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut elem_8: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut elem_9: color::Elem = crate::color::Elem::Special;
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    let mut bool_3: bool = crate::color::Elem::ne(elem_9_ref_0, elem_8_ref_0);
    let mut elem_10: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_11: color::Elem = crate::color::Elem::INode {valid: bool_2};
    let mut permission_0: crate::color::theme::Permission = crate::color::theme::Permission {read: color_7, write: color_6, exec: color_5, exec_sticky: color_4, no_access: color_3, octal: color_2, acl: color_1, context: color_0};
    crate::meta::filetype::FileType::render(filetype_0, colors_0_ref_0);
    let mut option_0: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut elem_11_ref_0: &color::Elem = &mut elem_11;
    let mut elem_12: color::Elem = crate::color::Elem::clone(elem_11_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6901() {
    rusty_monitor::set_test_id(6901);
    let mut bool_0: bool = false;
    let mut elem_0: color::Elem = crate::color::Elem::Links {valid: bool_0};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut str_0: &str = "WXK5";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut bool_1: bool = true;
    let mut elem_1: color::Elem = crate::color::Elem::Links {valid: bool_1};
    let mut bool_2: bool = false;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut bool_3: bool = true;
    let mut elem_2: color::Elem = crate::color::Elem::Links {valid: bool_3};
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_3: color::Elem = crate::color::Elem::Older;
    let mut option_0: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_4: bool = false;
    let mut option_1: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_4: color::Elem = crate::color::Elem::INode {valid: bool_2};
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_0_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_5: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_6: color::Elem = crate::color::Elem::Octal;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_2_ref_0);
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut elem_7: color::Elem = crate::color::Elem::NoAccess;
    let mut bool_5: bool = crate::meta::filetype::FileType::is_dirlike(filetype_0);
    let mut option_2: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut theme_4: icon::Theme = crate::icon::Theme::Fancy;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut bool_6: bool = crate::color::Elem::eq(elem_6_ref_0, elem_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_518() {
    rusty_monitor::set_test_id(518);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut filetype_0_ref_0: &meta::filetype::FileType = &mut filetype_0;
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut bool_5: bool = false;
    let mut bool_6: bool = true;
    let mut bool_7: bool = false;
    let mut bool_8: bool = true;
    let mut bool_9: bool = false;
    let mut bool_10: bool = true;
    let mut bool_11: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut bool_12: bool = false;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_12};
    let mut filetype_1_ref_0: &meta::filetype::FileType = &mut filetype_1;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut elem_0: color::Elem = crate::color::Elem::TreeEdge;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut elem_1: color::Elem = crate::color::Elem::Special;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::User;
    let mut elem_2: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut elem_3: color::Elem = crate::color::Elem::Context;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut tuple_0: () = crate::color::Elem::assert_receiver_is_total_eq(elem_3_ref_0);
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7886() {
    rusty_monitor::set_test_id(7886);
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut bool_3: bool = false;
    let mut bool_4: bool = false;
    let mut bool_5: bool = false;
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = false;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut elem_0: color::Elem = crate::color::Elem::Group;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Acl;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut bool_12: bool = crate::meta::permissions::Permissions::is_executable(permissions_0_ref_0);
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut elem_2: color::Elem = crate::color::Elem::SymLink;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_3: color::Elem = crate::color::Elem::clone(elem_2_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2554() {
    rusty_monitor::set_test_id(2554);
    let mut bool_0: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_0};
    let mut elem_0: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut elem_1: color::Elem = crate::color::Elem::Read;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut option_0: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_0: &str = "lzeJY";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_1: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut colors_1: crate::color::Colors = crate::color::Colors {theme: option_1, lscolors: option_0};
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut elem_2: color::Elem = crate::color::Elem::Socket;
    let mut str_1: &str = "7yHm";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut bool_1: bool = true;
    let mut elem_3: color::Elem = crate::color::Elem::Dir {uid: bool_1};
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_4: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_5: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut bool_2: bool = false;
    let mut elem_6: color::Elem = crate::color::Elem::Dir {uid: bool_2};
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_6_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_7: color::Elem = crate::color::Elem::SymLink;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_7_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut elem_8: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_5_ref_0);
    let mut theme_6: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_6_ref_0: &crate::color::theme::Theme = &mut theme_6;
    let mut elem_9: color::Elem = crate::color::Elem::Write;
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    let mut color_6: crossterm::style::Color = crate::color::Elem::get_color(elem_9_ref_0, theme_6_ref_0);
    let mut theme_7: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_7_ref_0: &crate::color::theme::Theme = &mut theme_7;
    let mut elem_10: color::Elem = crate::color::Elem::Exec;
    let mut elem_10_ref_0: &color::Elem = &mut elem_10;
    let mut color_7: crossterm::style::Color = crate::color::Elem::get_color(elem_10_ref_0, theme_7_ref_0);
    let mut bool_3: bool = true;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut elem_11: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_12: color::Elem = crate::color::Elem::Special;
    let mut elem_12_ref_0: &color::Elem = &mut elem_12;
    let mut bool_4: bool = crate::color::Elem::ne(elem_12_ref_0, elem_2_ref_0);
    let mut elem_13: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_14: color::Elem = crate::color::Elem::INode {valid: bool_3};
    let mut permission_0: crate::color::theme::Permission = crate::color::theme::Permission {read: color_7, write: color_6, exec: color_5, exec_sticky: color_4, no_access: color_3, octal: color_2, acl: color_1, context: color_0};
    crate::meta::filetype::FileType::render(filetype_1, colors_2_ref_0);
    let mut contentstyle_0: crossterm::style::ContentStyle = crate::color::Colors::style(colors_0_ref_0, elem_1_ref_0);
    let mut elem_11_ref_0: &color::Elem = &mut elem_11;
    let mut elem_15: color::Elem = crate::color::Elem::clone(elem_8_ref_0);
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut bool_5: bool = crate::meta::filetype::FileType::is_dirlike(filetype_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1809() {
    rusty_monitor::set_test_id(1809);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut elem_0: color::Elem = crate::color::Elem::Read;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut option_0: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_0: &str = "lzeJY";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut elem_1: color::Elem = crate::color::Elem::Socket;
    let mut str_1: &str = "7yHm";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut bool_0: bool = true;
    let mut elem_2: color::Elem = crate::color::Elem::Dir {uid: bool_0};
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_3: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_4: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut bool_1: bool = false;
    let mut elem_5: color::Elem = crate::color::Elem::Dir {uid: bool_1};
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_6: color::Elem = crate::color::Elem::SymLink;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_6_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut elem_7: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_7_ref_0, theme_5_ref_0);
    let mut theme_6: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_6_ref_0: &crate::color::theme::Theme = &mut theme_6;
    let mut elem_8: color::Elem = crate::color::Elem::Write;
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut color_6: crossterm::style::Color = crate::color::Elem::get_color(elem_8_ref_0, theme_6_ref_0);
    let mut theme_7: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_7_ref_0: &crate::color::theme::Theme = &mut theme_7;
    let mut elem_9: color::Elem = crate::color::Elem::Exec;
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    let mut color_7: crossterm::style::Color = crate::color::Elem::get_color(elem_9_ref_0, theme_7_ref_0);
    let mut bool_2: bool = true;
    let mut str_2: &str = "tck8xXTg04PC";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut elem_10: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_11: color::Elem = crate::color::Elem::Special;
    let mut elem_11_ref_0: &color::Elem = &mut elem_11;
    let mut elem_12: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_13: color::Elem = crate::color::Elem::INode {valid: bool_2};
    let mut permission_0: crate::color::theme::Permission = crate::color::theme::Permission {read: color_7, write: color_6, exec: color_5, exec_sticky: color_4, no_access: color_3, octal: color_2, acl: color_1, context: color_0};
    crate::meta::filetype::FileType::render(filetype_0, colors_1_ref_0);
    let mut option_1: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_1_ref_0);
    let mut contentstyle_0: crossterm::style::ContentStyle = crate::color::Colors::style(colors_0_ref_0, elem_0_ref_0);
    let mut elem_10_ref_0: &color::Elem = &mut elem_10;
    let mut elem_14: color::Elem = crate::color::Elem::clone(elem_10_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_353() {
    rusty_monitor::set_test_id(353);
    let mut elem_0: color::Elem = crate::color::Elem::Special;
    let mut option_0: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut bool_0: bool = true;
    let mut elem_1: color::Elem = crate::color::Elem::SymLink;
    let mut elem_2: color::Elem = crate::color::Elem::User;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_3: color::Elem = crate::color::Elem::Group;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut bool_1: bool = false;
    let mut elem_4: color::Elem = crate::color::Elem::INode {valid: bool_1};
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_5: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_6: color::Elem = crate::color::Elem::Octal;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_7: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_8: color::Elem = crate::color::Elem::SymLink;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut bool_2: bool = false;
    let mut elem_9: color::Elem = crate::color::Elem::INode {valid: bool_0};
    let mut elem_10: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    let mut elem_11: color::Elem = crate::color::Elem::NonFile;
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut elem_12: color::Elem = crate::color::Elem::HourOld;
    let mut elem_11_ref_0: &color::Elem = &mut elem_11;
    let mut bool_3: bool = false;
    let mut elem_13: color::Elem = crate::color::Elem::Dir {uid: bool_2};
    let mut elem_12_ref_0: &color::Elem = &mut elem_12;
    let mut elem_14: color::Elem = crate::color::Elem::INode {valid: bool_3};
    let mut elem_13_ref_0: &color::Elem = &mut elem_13;
    let mut elem_15: color::Elem = crate::color::Elem::Pipe;
    let mut elem_14_ref_0: &color::Elem = &mut elem_14;
    let mut elem_16: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_15_ref_0: &color::Elem = &mut elem_15;
    let mut elem_17: color::Elem = crate::color::Elem::Exec;
    let mut elem_16_ref_0: &color::Elem = &mut elem_16;
    let mut elem_18: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_17_ref_0: &color::Elem = &mut elem_17;
    let mut elem_19: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_18_ref_0: &color::Elem = &mut elem_18;
    let mut elem_20: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_19_ref_0: &color::Elem = &mut elem_19;
    let mut elem_21: color::Elem = crate::color::Elem::Older;
    let mut elem_20_ref_0: &color::Elem = &mut elem_20;
    let mut elem_22: color::Elem = crate::color::Elem::Read;
    let mut elem_21_ref_0: &color::Elem = &mut elem_21;
    let mut elem_23: color::Elem = crate::color::Elem::Read;
    let mut elem_22_ref_0: &color::Elem = &mut elem_22;
    let mut elem_24: color::Elem = crate::color::Elem::Exec;
    let mut elem_23_ref_0: &color::Elem = &mut elem_23;
    let mut elem_25: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_24_ref_0: &color::Elem = &mut elem_24;
    let mut elem_26: color::Elem = crate::color::Elem::Context;
    let mut elem_25_ref_0: &color::Elem = &mut elem_25;
    let mut elem_27: color::Elem = crate::color::Elem::DayOld;
    let mut elem_26_ref_0: &color::Elem = &mut elem_26;
    let mut elem_28: color::Elem = crate::color::Elem::Context;
    let mut elem_27_ref_0: &color::Elem = &mut elem_27;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut elem_28_ref_0: &color::Elem = &mut elem_28;
    let mut elem_29: color::Elem = crate::color::Elem::clone(elem_13_ref_0);
    let mut elem_30: color::Elem = crate::color::Elem::Older;
    let mut elem_31: color::Elem = crate::color::Elem::Context;
    let mut elem_32: color::Elem = crate::color::Elem::Older;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1948() {
    rusty_monitor::set_test_id(1948);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut elem_0: color::Elem = crate::color::Elem::SymLink;
    let mut str_0: &str = "AExF8WqTdyCBQ";
    let mut bool_0: bool = true;
    let mut option_0: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_1: bool = false;
    let mut option_1: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_0: usize = 6603usize;
    let mut bool_2: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_2, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut option_2: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_0);
    let mut option_3: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_3: bool = false;
    let mut option_4: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut bool_4: bool = false;
    let mut option_5: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut option_6: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut option_7: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut option_8: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_1);
    let mut option_9: std::option::Option<usize> = std::option::Option::None;
    let mut bool_5: bool = true;
    let mut option_10: std::option::Option<bool> = std::option::Option::Some(bool_5);
    let mut recursion_1: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_10, depth: option_9};
    let mut option_11: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_1);
    let mut option_12: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut option_13: std::option::Option<bool> = std::option::Option::None;
    let mut option_14: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_15: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_16: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut option_17: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_0);
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_17, theme: option_16, separator: option_15};
    let mut option_18: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut option_19: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut bool_6: bool = false;
    let mut option_20: std::option::Option<bool> = std::option::Option::Some(bool_6);
    let mut option_21: std::option::Option<std::string::String> = std::option::Option::None;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut str_1: &str = "yL7riF";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut option_22: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut option_23: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_23, theme: option_22};
    let mut option_24: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_25: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_26: std::option::Option<bool> = std::option::Option::None;
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_26, blocks: option_25, color: option_24, date: option_21, dereference: option_20, display: option_19, icons: option_18, ignore_globs: option_14, indicators: option_13, layout: option_12, recursion: option_11, size: option_8, permission: option_7, sorting: option_6, no_symlink: option_5, total_size: option_4, symlink_arrow: option_3, hyperlink: option_2};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_1: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut bool_7: bool = false;
    let mut bool_8: bool = true;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_8, exec: bool_7};
    let mut str_2: &str = "g7tTqQryoN";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut elem_1: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_2: color::Elem = crate::color::Elem::Special;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_3: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_4: color::Elem = crate::color::Elem::INode {valid: bool_0};
    crate::meta::filetype::FileType::render(filetype_2, colors_0_ref_0);
    let mut option_27: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_1_ref_0);
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut bool_9: bool = crate::color::Elem::eq(elem_1_ref_0, elem_0_ref_0);
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3359() {
    rusty_monitor::set_test_id(3359);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut elem_0: color::Elem = crate::color::Elem::Read;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut bool_0: bool = true;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_1: bool = true;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut bool_2: bool = true;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut bool_3: bool = true;
    let mut option_4: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut option_5: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut option_6: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut option_7: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_8: std::option::Option<usize> = std::option::Option::None;
    let mut option_9: std::option::Option<bool> = std::option::Option::None;
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_9, depth: option_8};
    let mut option_10: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_0);
    let mut elem_1: color::Elem = crate::color::Elem::Acl;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_2: color::Elem = crate::color::Elem::DayOld;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_3: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_4: color::Elem = crate::color::Elem::SymLink;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut bool_4: bool = false;
    let mut elem_5: color::Elem = crate::color::Elem::INode {valid: bool_0};
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut elem_6: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut elem_7: color::Elem = crate::color::Elem::NonFile;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut elem_8: color::Elem = crate::color::Elem::HourOld;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut bool_5: bool = false;
    let mut elem_9: color::Elem = crate::color::Elem::Dir {uid: bool_4};
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut elem_10: color::Elem = crate::color::Elem::INode {valid: bool_5};
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    let mut elem_11: color::Elem = crate::color::Elem::Pipe;
    let mut elem_10_ref_0: &color::Elem = &mut elem_10;
    let mut elem_12: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_11_ref_0: &color::Elem = &mut elem_11;
    let mut elem_13: color::Elem = crate::color::Elem::Exec;
    let mut elem_12_ref_0: &color::Elem = &mut elem_12;
    let mut elem_14: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_13_ref_0: &color::Elem = &mut elem_13;
    let mut elem_15: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_14_ref_0: &color::Elem = &mut elem_14;
    let mut elem_16: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_15_ref_0: &color::Elem = &mut elem_15;
    let mut elem_17: color::Elem = crate::color::Elem::Older;
    let mut elem_16_ref_0: &color::Elem = &mut elem_16;
    let mut elem_18: color::Elem = crate::color::Elem::Read;
    let mut elem_17_ref_0: &color::Elem = &mut elem_17;
    let mut elem_19: color::Elem = crate::color::Elem::Read;
    let mut elem_18_ref_0: &color::Elem = &mut elem_18;
    let mut elem_20: color::Elem = crate::color::Elem::Exec;
    let mut elem_19_ref_0: &color::Elem = &mut elem_19;
    let mut elem_21: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_20_ref_0: &color::Elem = &mut elem_20;
    let mut elem_22: color::Elem = crate::color::Elem::Context;
    let mut elem_21_ref_0: &color::Elem = &mut elem_21;
    let mut elem_23: color::Elem = crate::color::Elem::DayOld;
    let mut elem_22_ref_0: &color::Elem = &mut elem_22;
    let mut elem_24: color::Elem = crate::color::Elem::Context;
    let mut elem_23_ref_0: &color::Elem = &mut elem_23;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut elem_25: color::Elem = crate::color::Elem::Read;
    let mut elem_24_ref_0: &color::Elem = &mut elem_24;
    let mut elem_26: color::Elem = crate::color::Elem::clone(elem_7_ref_0);
    let mut elem_27: color::Elem = crate::color::Elem::Older;
    let mut elem_28: color::Elem = crate::color::Elem::Context;
    let mut elem_29: color::Elem = crate::color::Elem::Older;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1288() {
    rusty_monitor::set_test_id(1288);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut elem_0: color::Elem = crate::color::Elem::FileMedium;
    let mut str_0: &str = "7yHm";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut bool_0: bool = true;
    let mut elem_1: color::Elem = crate::color::Elem::Dir {uid: bool_0};
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_2: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_3: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_2_ref_0);
    let mut bool_1: bool = false;
    let mut elem_4: color::Elem = crate::color::Elem::Dir {uid: bool_1};
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_5: color::Elem = crate::color::Elem::SymLink;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_6: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_6_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut elem_7: color::Elem = crate::color::Elem::Write;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_7_ref_0, theme_5_ref_0);
    let mut theme_6: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_6_ref_0: &crate::color::theme::Theme = &mut theme_6;
    let mut elem_8: color::Elem = crate::color::Elem::Exec;
    let mut elem_9: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut elem_10: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    let mut elem_11: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_10_ref_0: &color::Elem = &mut elem_10;
    let mut elem_12: color::Elem = crate::color::Elem::Older;
    let mut elem_11_ref_0: &color::Elem = &mut elem_11;
    let mut elem_13: color::Elem = crate::color::Elem::Read;
    let mut elem_12_ref_0: &color::Elem = &mut elem_12;
    let mut elem_14: color::Elem = crate::color::Elem::Read;
    let mut elem_13_ref_0: &color::Elem = &mut elem_13;
    let mut elem_15: color::Elem = crate::color::Elem::Exec;
    let mut elem_14_ref_0: &color::Elem = &mut elem_14;
    let mut elem_16: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_15_ref_0: &color::Elem = &mut elem_15;
    let mut elem_17: color::Elem = crate::color::Elem::Context;
    let mut elem_16_ref_0: &color::Elem = &mut elem_16;
    let mut elem_18: color::Elem = crate::color::Elem::DayOld;
    let mut elem_17_ref_0: &color::Elem = &mut elem_17;
    let mut elem_19: color::Elem = crate::color::Elem::Context;
    let mut elem_18_ref_0: &color::Elem = &mut elem_18;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut elem_20: color::Elem = crate::color::Elem::Read;
    let mut elem_19_ref_0: &color::Elem = &mut elem_19;
    let mut elem_21: color::Elem = crate::color::Elem::clone(elem_18_ref_0);
    let mut elem_22: color::Elem = crate::color::Elem::Older;
    let mut elem_23: color::Elem = crate::color::Elem::Context;
    let mut elem_24: color::Elem = crate::color::Elem::Older;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_923() {
    rusty_monitor::set_test_id(923);
    let mut option_0: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut option_1: std::option::Option<crate::color::theme::Theme> = std::option::Option::None;
    let mut colors_0: crate::color::Colors = crate::color::Colors {theme: option_1, lscolors: option_0};
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut elem_0: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_1: color::Elem = crate::color::Elem::Older;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut filetype_0_ref_0: &meta::filetype::FileType = &mut filetype_0;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_2: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_3: color::Elem = crate::color::Elem::Group;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_4: color::Elem = crate::color::Elem::INode {valid: bool_1};
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut elem_5: color::Elem = crate::color::Elem::Acl;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut elem_6: color::Elem = crate::color::Elem::DayOld;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut elem_7: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut elem_8: color::Elem = crate::color::Elem::SymLink;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut bool_4: bool = false;
    let mut elem_9: color::Elem = crate::color::Elem::File {exec: bool_2, uid: bool_4};
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut elem_10: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    let mut elem_11: color::Elem = crate::color::Elem::NonFile;
    let mut elem_10_ref_0: &color::Elem = &mut elem_10;
    let mut elem_12: color::Elem = crate::color::Elem::HourOld;
    let mut elem_11_ref_0: &color::Elem = &mut elem_11;
    let mut elem_13: color::Elem = crate::color::Elem::Dir {uid: bool_3};
    let mut elem_12_ref_0: &color::Elem = &mut elem_12;
    let mut elem_14: color::Elem = crate::color::Elem::INode {valid: bool_0};
    let mut elem_13_ref_0: &color::Elem = &mut elem_13;
    let mut elem_15: color::Elem = crate::color::Elem::Pipe;
    let mut elem_14_ref_0: &color::Elem = &mut elem_14;
    let mut elem_16: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_15_ref_0: &color::Elem = &mut elem_15;
    let mut elem_17: color::Elem = crate::color::Elem::Exec;
    let mut elem_16_ref_0: &color::Elem = &mut elem_16;
    let mut elem_18: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_17_ref_0: &color::Elem = &mut elem_17;
    let mut elem_19: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_18_ref_0: &color::Elem = &mut elem_18;
    let mut elem_20: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_19_ref_0: &color::Elem = &mut elem_19;
    let mut elem_21: color::Elem = crate::color::Elem::Older;
    let mut elem_20_ref_0: &color::Elem = &mut elem_20;
    let mut elem_22: color::Elem = crate::color::Elem::Read;
    let mut elem_21_ref_0: &color::Elem = &mut elem_21;
    let mut elem_23: color::Elem = crate::color::Elem::Read;
    let mut elem_22_ref_0: &color::Elem = &mut elem_22;
    let mut elem_24: color::Elem = crate::color::Elem::Exec;
    let mut elem_23_ref_0: &color::Elem = &mut elem_23;
    let mut elem_25: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_24_ref_0: &color::Elem = &mut elem_24;
    let mut elem_26: color::Elem = crate::color::Elem::Context;
    let mut elem_25_ref_0: &color::Elem = &mut elem_25;
    let mut elem_27: color::Elem = crate::color::Elem::DayOld;
    let mut elem_26_ref_0: &color::Elem = &mut elem_26;
    let mut elem_28: color::Elem = crate::color::Elem::Context;
    let mut elem_27_ref_0: &color::Elem = &mut elem_27;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut elem_29: color::Elem = crate::color::Elem::Read;
    let mut elem_28_ref_0: &color::Elem = &mut elem_28;
    let mut elem_30: color::Elem = crate::color::Elem::clone(elem_17_ref_0);
    let mut elem_31: color::Elem = crate::color::Elem::Older;
    let mut elem_32: color::Elem = crate::color::Elem::Context;
    let mut elem_33: color::Elem = crate::color::Elem::Older;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2678() {
    rusty_monitor::set_test_id(2678);
    let mut elem_0: color::Elem = crate::color::Elem::User;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut bool_0: bool = true;
    let mut str_0: &str = "TV46";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u64_0: u64 = 7690u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut bool_1: bool = true;
    let mut elem_1: color::Elem = crate::color::Elem::Links {valid: bool_0};
    let mut elem_2: color::Elem = crate::color::Elem::Older;
    let mut option_0: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_1: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_3: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_4: color::Elem = crate::color::Elem::Octal;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_3_ref_0);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut elem_5: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut bool_2: bool = crate::color::Elem::ne(elem_4_ref_0, elem_0_ref_0);
    let mut elem_6: color::Elem = crate::color::Elem::FileSmall;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5293() {
    rusty_monitor::set_test_id(5293);
    let mut str_0: &str = "WXK5";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut bool_0: bool = true;
    let mut elem_0: color::Elem = crate::color::Elem::Links {valid: bool_0};
    let mut bool_1: bool = false;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut bool_2: bool = true;
    let mut elem_1: color::Elem = crate::color::Elem::Links {valid: bool_2};
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_2: color::Elem = crate::color::Elem::Older;
    let mut option_0: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_3: bool = false;
    let mut option_1: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_3: color::Elem = crate::color::Elem::INode {valid: bool_1};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_0_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_4: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_2_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_5: color::Elem = crate::color::Elem::Octal;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut elem_6: color::Elem = crate::color::Elem::NoAccess;
    let mut bool_4: bool = crate::meta::filetype::FileType::is_dirlike(filetype_0);
    let mut option_2: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut theme_5: icon::Theme = crate::icon::Theme::Fancy;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut elem_7: color::Elem = crate::color::Elem::clone(elem_6_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1057() {
    rusty_monitor::set_test_id(1057);
    let mut theme_0: icon::Theme = crate::icon::Theme::Unicode;
    let mut elem_0: color::Elem = crate::color::Elem::Context;
    let mut elem_1: color::Elem = crate::color::Elem::FileMedium;
    let mut option_0: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_0: &str = "m";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_1: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut colors_0: crate::color::Colors = crate::color::Colors {theme: option_1, lscolors: option_0};
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut bool_0: bool = false;
    let mut elem_2: color::Elem = crate::color::Elem::SymLink;
    let mut elem_3: color::Elem = crate::color::Elem::User;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_4: color::Elem = crate::color::Elem::Group;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_1_ref_0);
    let mut bool_1: bool = false;
    let mut elem_5: color::Elem = crate::color::Elem::INode {valid: bool_1};
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_6: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_6_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_7: color::Elem = crate::color::Elem::Octal;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut elem_8: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_9: color::Elem = crate::color::Elem::SymLink;
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut elem_10: color::Elem = crate::color::Elem::INode {valid: bool_0};
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    let mut elem_11: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_10_ref_0: &color::Elem = &mut elem_10;
    let mut elem_12: color::Elem = crate::color::Elem::NonFile;
    let mut elem_11_ref_0: &color::Elem = &mut elem_11;
    let mut elem_13: color::Elem = crate::color::Elem::HourOld;
    let mut elem_12_ref_0: &color::Elem = &mut elem_12;
    let mut elem_13_ref_0: &color::Elem = &mut elem_13;
    let mut elem_14: color::Elem = crate::color::Elem::Pipe;
    let mut elem_15: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_14_ref_0: &color::Elem = &mut elem_14;
    let mut elem_16: color::Elem = crate::color::Elem::Exec;
    let mut elem_15_ref_0: &color::Elem = &mut elem_15;
    let mut elem_17: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_16_ref_0: &color::Elem = &mut elem_16;
    let mut elem_18: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_17_ref_0: &color::Elem = &mut elem_17;
    let mut elem_16_ref_0: &color::Elem = &mut elem_18;
    let mut elem_18: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_17_ref_0: &color::Elem = &mut elem_1;
    let mut elem_19: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_20: color::Elem = crate::color::Elem::Older;
    let mut elem_19_ref_0: &color::Elem = &mut elem_19;
    let mut elem_21: color::Elem = crate::color::Elem::Read;
    let mut elem_20_ref_0: &color::Elem = &mut elem_20;
    let mut elem_22: color::Elem = crate::color::Elem::Read;
    let mut elem_21_ref_0: &color::Elem = &mut elem_21;
    let mut elem_23: color::Elem = crate::color::Elem::Exec;
    let mut elem_22_ref_0: &color::Elem = &mut elem_22;
    let mut elem_24: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_23_ref_0: &color::Elem = &mut elem_23;
    let mut elem_25: color::Elem = crate::color::Elem::Context;
    let mut elem_24_ref_0: &color::Elem = &mut elem_24;
    let mut elem_26: color::Elem = crate::color::Elem::DayOld;
    let mut elem_25_ref_0: &color::Elem = &mut elem_25;
    let mut elem_27: color::Elem = crate::color::Elem::Context;
    let mut elem_26_ref_0: &color::Elem = &mut elem_26;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut elem_28: color::Elem = crate::color::Elem::Read;
    let mut elem_27_ref_0: &color::Elem = &mut elem_27;
    let mut elem_29: color::Elem = crate::color::Elem::clone(elem_0_ref_0);
    let mut elem_30: color::Elem = crate::color::Elem::Older;
    let mut elem_31: color::Elem = crate::color::Elem::Context;
    let mut elem_32: color::Elem = crate::color::Elem::Older;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3052() {
    rusty_monitor::set_test_id(3052);
    let mut elem_0: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut str_0: &str = "MAxCVNaO";
    let mut str_1: &str = "4iijvXYnmj6m";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut usize_0: usize = 3381usize;
    let mut tuple_0: (usize, &str) = (usize_0, str_1_ref_0);
    let mut str_2: &str = "9QVRUtKQkF4";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "WVos";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut usize_1: usize = 7103usize;
    let mut tuple_1: (usize, &str) = (usize_1, str_3_ref_0);
    let mut usize_2: usize = 4650usize;
    let mut option_0: std::option::Option<usize> = std::option::Option::Some(usize_2);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_3: usize = 9169usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_3};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_0: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_1};
    let mut displayoption_0: meta::name::DisplayOption = crate::meta::name::DisplayOption::None;
    let mut displayoption_0_ref_0: &meta::name::DisplayOption = &mut displayoption_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut bool_1: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_1};
    let mut bool_2: bool = true;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_1: color::Elem = crate::color::Elem::Acl;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut bool_3: bool = true;
    let mut elem_2: color::Elem = crate::color::Elem::Links {valid: bool_2};
    let mut bool_4: bool = false;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut bool_5: bool = true;
    let mut elem_3: color::Elem = crate::color::Elem::Links {valid: bool_3};
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_4: color::Elem = crate::color::Elem::Older;
    let mut option_1: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_0_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_5: color::Elem = crate::color::Elem::INode {valid: bool_5};
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_2_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_1_ref_0);
    let mut bool_6: bool = true;
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut elem_6: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_7: color::Elem = crate::color::Elem::Special;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut bool_7: bool = crate::color::Elem::ne(elem_0_ref_0, elem_2_ref_0);
    let mut elem_8: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_9: color::Elem = crate::color::Elem::INode {valid: bool_6};
    let mut contentstyle_0: crossterm::style::ContentStyle = crate::color::Colors::style(colors_0_ref_0, elem_5_ref_0);
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2450() {
    rusty_monitor::set_test_id(2450);
    let mut elem_0: color::Elem = crate::color::Elem::Group;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_1: color::Elem = crate::color::Elem::ExecSticky;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_2: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_3: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut elem_4: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut elem_5: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut elem_6: color::Elem = crate::color::Elem::Group;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut bool_0: bool = true;
    let mut elem_7: color::Elem = crate::color::Elem::INode {valid: bool_0};
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut elem_8: color::Elem = crate::color::Elem::Acl;
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut elem_9: color::Elem = crate::color::Elem::DayOld;
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    let mut elem_10: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_10_ref_0: &color::Elem = &mut elem_10;
    let mut elem_11: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_11_ref_0: &color::Elem = &mut elem_11;
    let mut bool_1: bool = false;
    let mut elem_12: color::Elem = crate::color::Elem::INode {valid: bool_1};
    let mut elem_12_ref_0: &color::Elem = &mut elem_12;
    let mut elem_13: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_13_ref_0: &color::Elem = &mut elem_13;
    let mut elem_14: color::Elem = crate::color::Elem::NonFile;
    let mut elem_14_ref_0: &color::Elem = &mut elem_14;
    let mut elem_15: color::Elem = crate::color::Elem::HourOld;
    let mut elem_15_ref_0: &color::Elem = &mut elem_15;
    let mut bool_2: bool = false;
    let mut elem_16: color::Elem = crate::color::Elem::Dir {uid: bool_2};
    let mut elem_16_ref_0: &color::Elem = &mut elem_16;
    let mut bool_3: bool = true;
    let mut elem_17: color::Elem = crate::color::Elem::INode {valid: bool_3};
    let mut elem_17_ref_0: &color::Elem = &mut elem_17;
    let mut elem_18: color::Elem = crate::color::Elem::Pipe;
    let mut elem_18_ref_0: &color::Elem = &mut elem_18;
    let mut elem_19: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_19_ref_0: &color::Elem = &mut elem_19;
    let mut elem_20: color::Elem = crate::color::Elem::Exec;
    let mut elem_20_ref_0: &color::Elem = &mut elem_20;
    let mut elem_21: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_21_ref_0: &color::Elem = &mut elem_21;
    let mut elem_22: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_22_ref_0: &color::Elem = &mut elem_22;
    let mut elem_23: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_23_ref_0: &color::Elem = &mut elem_23;
    let mut elem_24: color::Elem = crate::color::Elem::Older;
    let mut elem_24_ref_0: &color::Elem = &mut elem_24;
    let mut elem_25: color::Elem = crate::color::Elem::Read;
    let mut elem_25_ref_0: &color::Elem = &mut elem_25;
    let mut elem_26: color::Elem = crate::color::Elem::Read;
    let mut elem_26_ref_0: &color::Elem = &mut elem_26;
    let mut elem_27: color::Elem = crate::color::Elem::Exec;
    let mut elem_27_ref_0: &color::Elem = &mut elem_27;
    let mut elem_28: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_28_ref_0: &color::Elem = &mut elem_28;
    let mut elem_29: color::Elem = crate::color::Elem::Context;
    let mut elem_29_ref_0: &color::Elem = &mut elem_29;
    let mut elem_30: color::Elem = crate::color::Elem::DayOld;
    let mut elem_30_ref_0: &color::Elem = &mut elem_30;
    let mut elem_31: color::Elem = crate::color::Elem::Context;
    let mut elem_31_ref_0: &color::Elem = &mut elem_31;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut elem_32: color::Elem = crate::color::Elem::HourOld;
    let mut elem_32_ref_0: &color::Elem = &mut elem_32;
    let mut elem_33: color::Elem = crate::color::Elem::clone(elem_32_ref_0);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::SizeValue;
    let mut elem_34: color::Elem = crate::color::Elem::Older;
    let mut elem_35: color::Elem = crate::color::Elem::Context;
    let mut elem_36: color::Elem = crate::color::Elem::Older;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7883() {
    rusty_monitor::set_test_id(7883);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_0: usize = 682usize;
    let mut bool_0: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Octal;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut str_0: &str = "WXK5";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut bool_1: bool = true;
    let mut elem_1: color::Elem = crate::color::Elem::Links {valid: bool_1};
    let mut bool_2: bool = false;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut bool_3: bool = true;
    let mut elem_2: color::Elem = crate::color::Elem::Links {valid: bool_3};
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_3: color::Elem = crate::color::Elem::Older;
    let mut option_0: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_4: bool = false;
    let mut option_1: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_4: color::Elem = crate::color::Elem::INode {valid: bool_2};
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_1_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut elem_5: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_6: color::Elem = crate::color::Elem::Octal;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_3_ref_0);
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut elem_7: color::Elem = crate::color::Elem::NoAccess;
    let mut bool_5: bool = crate::meta::filetype::FileType::is_dirlike(filetype_0);
    let mut option_2: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut theme_5: icon::Theme = crate::icon::Theme::Fancy;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut elem_8: color::Elem = crate::color::Elem::clone(elem_6_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2660() {
    rusty_monitor::set_test_id(2660);
    let mut str_0: &str = "TV46";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut elem_0: color::Elem = crate::color::Elem::Read;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut option_0: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_1: &str = "lzeJY";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut option_1: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_1_ref_0);
    let mut colors_0: crate::color::Colors = crate::color::Colors {theme: option_1, lscolors: option_0};
    let mut option_2: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_861() {
    rusty_monitor::set_test_id(861);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut elem_0: color::Elem = crate::color::Elem::ExecSticky;
    let mut elem_1: color::Elem = crate::color::Elem::NonFile;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_2: color::Elem = crate::color::Elem::Acl;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_3: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_4: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_5: color::Elem = crate::color::Elem::HourOld;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_3_ref_0);
    let mut size_0: crate::color::theme::Size = crate::color::theme::Size {none: color_3, small: color_2, medium: color_1, large: color_0};
    let mut size_0_ref_0: &crate::color::theme::Size = &mut size_0;
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_6: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_6_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut elem_7: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_7_ref_0, theme_5_ref_0);
    let mut theme_6: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_6_ref_0: &crate::color::theme::Theme = &mut theme_6;
    let mut bool_0: bool = true;
    let mut elem_8: color::Elem = crate::color::Elem::INode {valid: bool_0};
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut color_6: crossterm::style::Color = crate::color::Elem::get_color(elem_8_ref_0, theme_6_ref_0);
    let mut theme_7: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_7_ref_0: &crate::color::theme::Theme = &mut theme_7;
    let mut bool_1: bool = true;
    let mut elem_9: color::Elem = crate::color::Elem::Links {valid: bool_1};
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    let mut color_7: crossterm::style::Color = crate::color::Elem::get_color(elem_9_ref_0, theme_7_ref_0);
    let mut theme_8: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_8_ref_0: &crate::color::theme::Theme = &mut theme_8;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_10: color::Elem = crate::color::Elem::Older;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_11: color::Elem = crate::color::Elem::Read;
    let mut elem_10_ref_0: &color::Elem = &mut elem_10;
    let mut elem_12: color::Elem = crate::color::Elem::Read;
    let mut elem_11_ref_0: &color::Elem = &mut elem_11;
    let mut elem_13: color::Elem = crate::color::Elem::Exec;
    let mut elem_12_ref_0: &color::Elem = &mut elem_12;
    let mut elem_14: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_13_ref_0: &color::Elem = &mut elem_13;
    let mut elem_15: color::Elem = crate::color::Elem::Context;
    let mut elem_14_ref_0: &color::Elem = &mut elem_14;
    let mut elem_16: color::Elem = crate::color::Elem::DayOld;
    let mut elem_15_ref_0: &color::Elem = &mut elem_15;
    let mut elem_17: color::Elem = crate::color::Elem::Context;
    let mut elem_16_ref_0: &color::Elem = &mut elem_16;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut elem_18: color::Elem = crate::color::Elem::Read;
    let mut elem_17_ref_0: &color::Elem = &mut elem_17;
    let mut elem_19: color::Elem = crate::color::Elem::clone(elem_10_ref_0);
    let mut elem_20: color::Elem = crate::color::Elem::Older;
    let mut elem_21: color::Elem = crate::color::Elem::Context;
    let mut elem_22: color::Elem = crate::color::Elem::Older;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3401() {
    rusty_monitor::set_test_id(3401);
    let mut str_0: &str = "7yHm";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut bool_0: bool = true;
    let mut elem_0: color::Elem = crate::color::Elem::Dir {uid: bool_0};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut bool_1: bool = false;
    let mut elem_3: color::Elem = crate::color::Elem::Dir {uid: bool_1};
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_4: color::Elem = crate::color::Elem::SymLink;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut elem_5: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut theme_6: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_6_ref_0: &crate::color::theme::Theme = &mut theme_6;
    let mut elem_6: color::Elem = crate::color::Elem::Write;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_6_ref_0, theme_6_ref_0);
    let mut theme_7: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_7_ref_0: &crate::color::theme::Theme = &mut theme_7;
    let mut elem_7: color::Elem = crate::color::Elem::Exec;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_7_ref_0, theme_7_ref_0);
    let mut bool_2: bool = true;
    let mut str_1: &str = "tck8xXTg04PC";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut elem_8: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut elem_9: color::Elem = crate::color::Elem::Special;
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    let mut bool_3: bool = crate::color::Elem::ne(elem_9_ref_0, elem_8_ref_0);
    let mut elem_10: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_11: color::Elem = crate::color::Elem::INode {valid: bool_2};
    crate::meta::filetype::FileType::render(filetype_0, colors_0_ref_0);
    let mut option_0: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut elem_10_ref_0: &color::Elem = &mut elem_10;
    let mut elem_12: color::Elem = crate::color::Elem::clone(elem_10_ref_0);
    let mut app_0: clap::App = crate::app::build();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6478() {
    rusty_monitor::set_test_id(6478);
    let mut elem_0: color::Elem = crate::color::Elem::Group;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_1: color::Elem = crate::color::Elem::ExecSticky;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_2: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_3: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut elem_4: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut elem_5: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut elem_6: color::Elem = crate::color::Elem::Group;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut bool_0: bool = true;
    let mut elem_7: color::Elem = crate::color::Elem::INode {valid: bool_0};
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut elem_8: color::Elem = crate::color::Elem::Acl;
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut elem_9: color::Elem = crate::color::Elem::DayOld;
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    let mut elem_10: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_10_ref_0: &color::Elem = &mut elem_10;
    let mut elem_11: color::Elem = crate::color::Elem::SymLink;
    let mut elem_11_ref_0: &color::Elem = &mut elem_11;
    let mut bool_1: bool = false;
    let mut elem_12: color::Elem = crate::color::Elem::INode {valid: bool_1};
    let mut elem_12_ref_0: &color::Elem = &mut elem_12;
    let mut elem_13: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_13_ref_0: &color::Elem = &mut elem_13;
    let mut elem_14: color::Elem = crate::color::Elem::NonFile;
    let mut elem_14_ref_0: &color::Elem = &mut elem_14;
    let mut elem_15: color::Elem = crate::color::Elem::HourOld;
    let mut elem_15_ref_0: &color::Elem = &mut elem_15;
    let mut bool_2: bool = false;
    let mut elem_16: color::Elem = crate::color::Elem::Dir {uid: bool_2};
    let mut elem_16_ref_0: &color::Elem = &mut elem_16;
    let mut bool_3: bool = true;
    let mut elem_17: color::Elem = crate::color::Elem::INode {valid: bool_3};
    let mut elem_17_ref_0: &color::Elem = &mut elem_17;
    let mut elem_18: color::Elem = crate::color::Elem::Pipe;
    let mut elem_18_ref_0: &color::Elem = &mut elem_18;
    let mut elem_19: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_19_ref_0: &color::Elem = &mut elem_19;
    let mut elem_20: color::Elem = crate::color::Elem::Exec;
    let mut elem_20_ref_0: &color::Elem = &mut elem_20;
    let mut elem_21: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_21_ref_0: &color::Elem = &mut elem_21;
    let mut elem_22: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_22_ref_0: &color::Elem = &mut elem_22;
    let mut elem_23: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_23_ref_0: &color::Elem = &mut elem_23;
    let mut elem_24: color::Elem = crate::color::Elem::Older;
    let mut elem_24_ref_0: &color::Elem = &mut elem_24;
    let mut elem_25: color::Elem = crate::color::Elem::Read;
    let mut elem_25_ref_0: &color::Elem = &mut elem_25;
    let mut elem_26: color::Elem = crate::color::Elem::Read;
    let mut elem_26_ref_0: &color::Elem = &mut elem_26;
    let mut elem_27: color::Elem = crate::color::Elem::Exec;
    let mut elem_27_ref_0: &color::Elem = &mut elem_27;
    let mut elem_28: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_28_ref_0: &color::Elem = &mut elem_28;
    let mut elem_29: color::Elem = crate::color::Elem::DayOld;
    let mut elem_29_ref_0: &color::Elem = &mut elem_29;
    let mut elem_30: color::Elem = crate::color::Elem::Context;
    let mut elem_30_ref_0: &color::Elem = &mut elem_30;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut elem_31: color::Elem = crate::color::Elem::Read;
    let mut elem_31_ref_0: &color::Elem = &mut elem_31;
    let mut elem_32: color::Elem = crate::color::Elem::clone(elem_1_ref_0);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::SizeValue;
    let mut elem_33: color::Elem = crate::color::Elem::Older;
    let mut elem_34: color::Elem = crate::color::Elem::Context;
    let mut elem_35: color::Elem = crate::color::Elem::Older;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1036() {
    rusty_monitor::set_test_id(1036);
    let mut bool_0: bool = true;
    let mut option_0: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut elem_0: color::Elem = crate::color::Elem::FileLarge;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut elem_1: color::Elem = crate::color::Elem::File {exec: bool_2, uid: bool_1};
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_2: color::Elem = crate::color::Elem::Acl;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_3: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_4: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_5: color::Elem = crate::color::Elem::HourOld;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_3_ref_0);
    let mut size_0: crate::color::theme::Size = crate::color::theme::Size {none: color_3, small: color_2, medium: color_1, large: color_0};
    let mut size_0_ref_0: &crate::color::theme::Size = &mut size_0;
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_6: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_6_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut elem_7: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_7_ref_0, theme_5_ref_0);
    let mut theme_6: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_6_ref_0: &crate::color::theme::Theme = &mut theme_6;
    let mut bool_3: bool = true;
    let mut elem_8: color::Elem = crate::color::Elem::INode {valid: bool_3};
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut color_6: crossterm::style::Color = crate::color::Elem::get_color(elem_8_ref_0, theme_6_ref_0);
    let mut theme_7: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_7_ref_0: &crate::color::theme::Theme = &mut theme_7;
    let mut bool_4: bool = true;
    let mut elem_9: color::Elem = crate::color::Elem::Links {valid: bool_4};
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    let mut color_7: crossterm::style::Color = crate::color::Elem::get_color(elem_9_ref_0, theme_7_ref_0);
    let mut theme_8: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_8_ref_0: &crate::color::theme::Theme = &mut theme_8;
    let mut bool_5: bool = false;
    let mut elem_10: color::Elem = crate::color::Elem::Dir {uid: bool_5};
    let mut elem_10_ref_0: &color::Elem = &mut elem_10;
    let mut color_8: crossterm::style::Color = crate::color::Elem::get_color(elem_10_ref_0, theme_8_ref_0);
    let mut theme_9: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_9_ref_0: &crate::color::theme::Theme = &mut theme_9;
    let mut elem_11: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_11_ref_0: &color::Elem = &mut elem_11;
    let mut color_9: crossterm::style::Color = crate::color::Elem::get_color(elem_11_ref_0, theme_9_ref_0);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut links_0: crate::color::theme::Links = crate::color::theme::Links {valid: color_9, invalid: color_8};
    let mut theme_10: icon::Theme = crate::icon::Theme::NoIcon;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_12: color::Elem = crate::color::Elem::Context;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut elem_13: color::Elem = crate::color::Elem::Read;
    let mut elem_12_ref_0: &color::Elem = &mut elem_12;
    let mut elem_14: color::Elem = crate::color::Elem::clone(elem_0_ref_0);
    let mut elem_15: color::Elem = crate::color::Elem::Older;
    let mut elem_16: color::Elem = crate::color::Elem::Context;
    let mut elem_17: color::Elem = crate::color::Elem::Older;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8126() {
    rusty_monitor::set_test_id(8126);
    let mut option_0: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut option_1: std::option::Option<crate::color::theme::Theme> = std::option::Option::None;
    let mut colors_0: crate::color::Colors = crate::color::Colors {theme: option_1, lscolors: option_0};
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut elem_0: color::Elem = crate::color::Elem::Read;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut elem_1: color::Elem = crate::color::Elem::INode {valid: bool_1};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_2: color::Elem = crate::color::Elem::Acl;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_3: color::Elem = crate::color::Elem::DayOld;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_4: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut elem_5: color::Elem = crate::color::Elem::SymLink;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut bool_3: bool = false;
    let mut elem_6: color::Elem = crate::color::Elem::INode {valid: bool_2};
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut elem_7: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut elem_8: color::Elem = crate::color::Elem::NonFile;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut elem_9: color::Elem = crate::color::Elem::HourOld;
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut elem_10: color::Elem = crate::color::Elem::Dir {uid: bool_0};
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    let mut elem_11: color::Elem = crate::color::Elem::INode {valid: bool_3};
    let mut elem_10_ref_0: &color::Elem = &mut elem_10;
    let mut elem_12: color::Elem = crate::color::Elem::Pipe;
    let mut elem_11_ref_0: &color::Elem = &mut elem_11;
    let mut elem_13: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_12_ref_0: &color::Elem = &mut elem_12;
    let mut elem_14: color::Elem = crate::color::Elem::Exec;
    let mut elem_13_ref_0: &color::Elem = &mut elem_13;
    let mut elem_15: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_14_ref_0: &color::Elem = &mut elem_14;
    let mut elem_16: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_15_ref_0: &color::Elem = &mut elem_15;
    let mut elem_17: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_16_ref_0: &color::Elem = &mut elem_16;
    let mut elem_18: color::Elem = crate::color::Elem::Older;
    let mut elem_17_ref_0: &color::Elem = &mut elem_17;
    let mut elem_19: color::Elem = crate::color::Elem::Read;
    let mut elem_18_ref_0: &color::Elem = &mut elem_18;
    let mut elem_20: color::Elem = crate::color::Elem::Read;
    let mut elem_19_ref_0: &color::Elem = &mut elem_19;
    let mut elem_21: color::Elem = crate::color::Elem::Exec;
    let mut elem_20_ref_0: &color::Elem = &mut elem_20;
    let mut elem_22: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_21_ref_0: &color::Elem = &mut elem_21;
    let mut elem_23: color::Elem = crate::color::Elem::Context;
    let mut elem_22_ref_0: &color::Elem = &mut elem_22;
    let mut elem_24: color::Elem = crate::color::Elem::DayOld;
    let mut elem_23_ref_0: &color::Elem = &mut elem_23;
    let mut elem_25: color::Elem = crate::color::Elem::Context;
    let mut elem_24_ref_0: &color::Elem = &mut elem_24;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut elem_26: color::Elem = crate::color::Elem::Read;
    let mut elem_25_ref_0: &color::Elem = &mut elem_25;
    let mut elem_27: color::Elem = crate::color::Elem::clone(elem_0_ref_0);
    let mut elem_28: color::Elem = crate::color::Elem::Older;
    let mut elem_29: color::Elem = crate::color::Elem::Context;
    let mut elem_30: color::Elem = crate::color::Elem::Older;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1541() {
    rusty_monitor::set_test_id(1541);
    let mut str_0: &str = "Z";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_0: bool = true;
    let mut elem_0: color::Elem = crate::color::Elem::Dir {uid: bool_0};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut elem_1: color::Elem = crate::color::Elem::ExecSticky;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_2: color::Elem = crate::color::Elem::Acl;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_3: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut elem_4: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_5: color::Elem = crate::color::Elem::HourOld;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_6: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_6_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut elem_7: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_7_ref_0, theme_5_ref_0);
    let mut theme_6: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_6_ref_0: &crate::color::theme::Theme = &mut theme_6;
    let mut bool_1: bool = true;
    let mut elem_8: color::Elem = crate::color::Elem::INode {valid: bool_1};
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_8_ref_0, theme_6_ref_0);
    let mut theme_7: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_7_ref_0: &crate::color::theme::Theme = &mut theme_7;
    let mut bool_2: bool = true;
    let mut elem_9: color::Elem = crate::color::Elem::Links {valid: bool_2};
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_9_ref_0, theme_7_ref_0);
    let mut theme_8: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_8_ref_0: &crate::color::theme::Theme = &mut theme_8;
    let mut bool_3: bool = false;
    let mut elem_10: color::Elem = crate::color::Elem::Dir {uid: bool_3};
    let mut elem_10_ref_0: &color::Elem = &mut elem_10;
    let mut color_6: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_0_ref_0);
    let mut theme_9: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_9_ref_0: &crate::color::theme::Theme = &mut theme_9;
    let mut elem_11: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_7: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_9_ref_0);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut links_0: crate::color::theme::Links = crate::color::theme::Links {valid: color_7, invalid: color_6};
    let mut theme_10: icon::Theme = crate::icon::Theme::NoIcon;
    let mut links_0_ref_0: &crate::color::theme::Links = &mut links_0;
    let mut size_0: crate::color::theme::Size = crate::color::theme::Size {none: color_5, small: color_4, medium: color_3, large: color_2};
    let mut size_0_ref_0: &crate::color::theme::Size = &mut size_0;
    let mut contentstyle_0: crossterm::style::ContentStyle = crate::color::Colors::style_default(colors_0_ref_0, elem_0_ref_0);
    let mut result_0: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1892() {
    rusty_monitor::set_test_id(1892);
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut bool_3: bool = false;
    let mut bool_4: bool = true;
    let mut bool_5: bool = true;
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut elem_0: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut str_0: &str = "7yHm";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut bool_12: bool = true;
    let mut elem_1: color::Elem = crate::color::Elem::Dir {uid: bool_12};
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_2: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_3: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut bool_13: bool = false;
    let mut elem_4: color::Elem = crate::color::Elem::Dir {uid: bool_13};
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_5: color::Elem = crate::color::Elem::SymLink;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut elem_6: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_6_ref_0, theme_5_ref_0);
    let mut theme_6: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_6_ref_0: &crate::color::theme::Theme = &mut theme_6;
    let mut elem_7: color::Elem = crate::color::Elem::Write;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut color_6: crossterm::style::Color = crate::color::Elem::get_color(elem_7_ref_0, theme_6_ref_0);
    let mut theme_7: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_7_ref_0: &crate::color::theme::Theme = &mut theme_7;
    let mut elem_8: color::Elem = crate::color::Elem::Exec;
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut color_7: crossterm::style::Color = crate::color::Elem::get_color(elem_8_ref_0, theme_7_ref_0);
    let mut bool_14: bool = true;
    let mut str_1: &str = "tck8xXTg04PC";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut elem_9: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    let mut elem_10: color::Elem = crate::color::Elem::Special;
    let mut elem_10_ref_0: &color::Elem = &mut elem_10;
    let mut bool_15: bool = crate::color::Elem::ne(elem_10_ref_0, elem_9_ref_0);
    let mut elem_11: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_12: color::Elem = crate::color::Elem::INode {valid: bool_14};
    let mut permission_0: crate::color::theme::Permission = crate::color::theme::Permission {read: color_7, write: color_6, exec: color_5, exec_sticky: color_4, no_access: color_3, octal: color_2, acl: color_1, context: color_0};
    crate::meta::filetype::FileType::render(filetype_1, colors_0_ref_0);
    let mut option_0: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut elem_13: color::Elem = crate::color::Elem::clone(elem_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1021() {
    rusty_monitor::set_test_id(1021);
    let mut str_0: &str = "XYJ0Yws01PWSk";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_0: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut elem_0: color::Elem = crate::color::Elem::Octal;
    let mut elem_1: color::Elem = crate::color::Elem::User;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_2: color::Elem = crate::color::Elem::Acl;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_3: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_4: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_5: color::Elem = crate::color::Elem::HourOld;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_3_ref_0);
    let mut size_0: crate::color::theme::Size = crate::color::theme::Size {none: color_3, small: color_2, medium: color_1, large: color_0};
    let mut size_0_ref_0: &crate::color::theme::Size = &mut size_0;
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_6: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_6_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut elem_7: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_7_ref_0, theme_5_ref_0);
    let mut theme_6: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_6_ref_0: &crate::color::theme::Theme = &mut theme_6;
    let mut bool_0: bool = true;
    let mut elem_8: color::Elem = crate::color::Elem::INode {valid: bool_0};
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut color_6: crossterm::style::Color = crate::color::Elem::get_color(elem_8_ref_0, theme_6_ref_0);
    let mut theme_7: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_9: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_10: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    let mut elem_11: color::Elem = crate::color::Elem::Older;
    let mut elem_10_ref_0: &color::Elem = &mut elem_10;
    let mut elem_12: color::Elem = crate::color::Elem::Read;
    let mut elem_11_ref_0: &color::Elem = &mut elem_11;
    let mut elem_13: color::Elem = crate::color::Elem::Read;
    let mut elem_12_ref_0: &color::Elem = &mut elem_12;
    let mut elem_14: color::Elem = crate::color::Elem::Exec;
    let mut elem_13_ref_0: &color::Elem = &mut elem_13;
    let mut elem_15: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_14_ref_0: &color::Elem = &mut elem_14;
    let mut elem_16: color::Elem = crate::color::Elem::Context;
    let mut elem_15_ref_0: &color::Elem = &mut elem_15;
    let mut elem_17: color::Elem = crate::color::Elem::DayOld;
    let mut elem_16_ref_0: &color::Elem = &mut elem_16;
    let mut elem_18: color::Elem = crate::color::Elem::Context;
    let mut elem_17_ref_0: &color::Elem = &mut elem_17;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut elem_19: color::Elem = crate::color::Elem::Read;
    let mut elem_20: color::Elem = crate::color::Elem::clone(elem_1_ref_0);
    let mut elem_21: color::Elem = crate::color::Elem::Older;
    let mut elem_22: color::Elem = crate::color::Elem::Context;
    let mut elem_23: color::Elem = crate::color::Elem::Older;
    panic!("From RustyUnit with love");
}
}