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
	use std::clone::Clone;
	use std::cmp::PartialEq;
	use std::cmp::Eq;
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5943() {
//    rusty_monitor::set_test_id(5943);
    let mut str_0: &str = "zsh-theme";
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut elem_0: color::Elem = crate::color::Elem::File {exec: bool_1, uid: bool_0};
    let mut elem_1: color::Elem = crate::color::Elem::Acl;
    let mut bool_2: bool = false;
    let mut bool_3: bool = true;
    let mut bool_4: bool = false;
    let mut str_1: &str = "config-file";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut u64_0: u64 = 1073741824u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut u64_1: u64 = 1048576u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut str_2: &str = "Time";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut option_0: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_1);
    let mut u64_2: u64 = 1099511627776u64;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size::new(u64_2);
    let mut str_3: &str = "IzjHVmU8";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut u64_3: u64 = 70u64;
    let mut size_3: crate::meta::size::Size = crate::meta::size::Size::new(u64_3);
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_2: color::Elem = crate::color::Elem::Older;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_3: color::Elem = crate::color::Elem::Socket;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_4: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut elem_5: color::Elem = crate::color::Elem::Exec;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut elem_6: color::Elem = crate::color::Elem::Context;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut elem_7: color::Elem = crate::color::Elem::INode {valid: bool_4};
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut elem_8: color::Elem = crate::color::Elem::Group;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut elem_9: color::Elem = crate::color::Elem::NonFile;
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut elem_10: color::Elem = crate::color::Elem::Socket;
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    let mut elem_11: color::Elem = crate::color::Elem::Special;
    let mut elem_10_ref_0: &color::Elem = &mut elem_10;
    let mut elem_12: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_11_ref_0: &color::Elem = &mut elem_11;
    let mut elem_13: color::Elem = crate::color::Elem::DayOld;
    let mut elem_12_ref_0: &color::Elem = &mut elem_12;
    let mut elem_14: color::Elem = crate::color::Elem::INode {valid: bool_2};
    let mut elem_13_ref_0: &color::Elem = &mut elem_13;
    let mut elem_15: color::Elem = crate::color::Elem::Octal;
    let mut elem_14_ref_0: &color::Elem = &mut elem_14;
    let mut elem_16: color::Elem = crate::color::Elem::Acl;
    let mut elem_15_ref_0: &color::Elem = &mut elem_15;
    let mut elem_17: color::Elem = crate::color::Elem::DayOld;
    let mut elem_16_ref_0: &color::Elem = &mut elem_16;
    let mut elem_17_ref_0: &color::Elem = &mut elem_17;
    let mut elem_18: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut str_0_ref_0: &str = &mut str_0;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut elem_19: color::Elem = crate::color::Elem::INode {valid: bool_3};
    let mut elem_18_ref_0: &color::Elem = &mut elem_18;
    let mut elem_20: color::Elem = crate::color::Elem::clone(elem_4_ref_0);
    let mut elem_21: color::Elem = crate::color::Elem::BlockDevice;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_142() {
//    rusty_monitor::set_test_id(142);
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut bool_2: bool = false;
    let mut bool_3: bool = false;
    let mut option_0: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_0: usize = 6usize;
    let mut bool_4: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_4, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut option_1: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_2: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_5: bool = true;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_5);
    let mut option_4: std::option::Option<bool> = std::option::Option::None;
    let mut option_5: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut option_6: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut option_7: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_8: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut option_9: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_1);
    let mut bool_6: bool = true;
    let mut option_10: std::option::Option<bool> = std::option::Option::Some(bool_6);
    let mut option_11: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_12: std::option::Option<std::string::String> = std::option::Option::None;
    let mut str_0: &str = "markdown";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut bool_7: bool = false;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_7};
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut option_13: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_0);
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut option_14: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_0);
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_14, theme: option_13, separator: option_12};
    let mut option_15: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut option_16: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut bool_8: bool = true;
    let mut option_17: std::option::Option<bool> = std::option::Option::Some(bool_8);
    let mut option_18: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_19: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_20: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_9: bool = true;
    let mut option_21: std::option::Option<bool> = std::option::Option::Some(bool_9);
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_21, blocks: option_20, color: option_19, date: option_18, dereference: option_17, display: option_16, icons: option_15, ignore_globs: option_11, indicators: option_10, layout: option_9, recursion: option_8, size: option_7, permission: option_6, sorting: option_5, no_symlink: option_4, total_size: option_3, symlink_arrow: option_2, hyperlink: option_1};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_11, exec: bool_10};
    let mut str_1: &str = "dart";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut u64_0: u64 = 48u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut bool_12: bool = false;
    let mut bool_13: bool = true;
    let mut elem_0: color::Elem = crate::color::Elem::HourOld;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_1: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_2: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_3: color::Elem = crate::color::Elem::ExecSticky;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut option_22: std::option::Option<lscolors::Indicator> = crate::color::Colors::get_indicator_from_elem(colors_0_ref_0, elem_3_ref_0);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Size;
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut block_1: flags::blocks::Block = crate::flags::blocks::Block::Context;
    let mut elem_4: color::Elem = crate::color::Elem::clone(elem_2_ref_0);
    let mut bool_14: bool = crate::color::Elem::ne(elem_1_ref_0, elem_0_ref_0);
    let mut filetype_4: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_13, exec: bool_12};
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2652() {
//    rusty_monitor::set_test_id(2652);
    let mut str_0: &str = "";
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut elem_0: color::Elem = crate::color::Elem::HourOld;
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut bool_2: bool = false;
    let mut bool_3: bool = false;
    let mut bool_4: bool = true;
    let mut bool_5: bool = true;
    let mut bool_6: bool = false;
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = true;
    let mut bool_10: bool = true;
    let mut bool_11: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut bool_12: bool = true;
    let mut bool_13: bool = true;
    let mut bool_14: bool = false;
    let mut bool_15: bool = true;
    let mut bool_16: bool = true;
    let mut bool_17: bool = true;
    let mut bool_18: bool = false;
    let mut bool_19: bool = false;
    let mut bool_20: bool = true;
    let mut bool_21: bool = false;
    let mut bool_22: bool = false;
    let mut bool_23: bool = true;
    let mut permissions_1: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_23, user_write: bool_22, user_execute: bool_21, group_read: bool_20, group_write: bool_19, group_execute: bool_18, other_read: bool_17, other_write: bool_16, other_execute: bool_15, sticky: bool_14, setgid: bool_13, setuid: bool_12};
    let mut permissions_1_ref_0: &crate::meta::permissions::Permissions = &mut permissions_1;
    let mut bool_24: bool = false;
    let mut bool_25: bool = true;
    let mut bool_26: bool = false;
    let mut bool_27: bool = false;
    let mut bool_28: bool = true;
    let mut bool_29: bool = true;
    let mut bool_30: bool = false;
    let mut bool_31: bool = false;
    let mut bool_32: bool = false;
    let mut bool_33: bool = true;
    let mut bool_34: bool = false;
    let mut bool_35: bool = false;
    let mut permissions_2: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_35, user_write: bool_34, user_execute: bool_33, group_read: bool_32, group_write: bool_31, group_execute: bool_30, other_read: bool_29, other_write: bool_28, other_execute: bool_27, sticky: bool_26, setgid: bool_25, setuid: bool_24};
    let mut permissions_2_ref_0: &crate::meta::permissions::Permissions = &mut permissions_2;
    let mut bool_36: bool = false;
    let mut bool_37: bool = false;
    let mut elem_1: color::Elem = crate::color::Elem::Socket;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_2: color::Elem = crate::color::Elem::Special;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_3: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_4: color::Elem = crate::color::Elem::DayOld;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut elem_5: color::Elem = crate::color::Elem::INode {valid: bool_36};
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut elem_6: color::Elem = crate::color::Elem::Octal;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut elem_7: color::Elem = crate::color::Elem::Acl;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut elem_8: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut elem_9: color::Elem = crate::color::Elem::Older;
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut elem_10: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut str_0_ref_0: &str = &mut str_0;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut elem_11: color::Elem = crate::color::Elem::INode {valid: bool_37};
    let mut elem_10_ref_0: &color::Elem = &mut elem_10;
    let mut elem_12: color::Elem = crate::color::Elem::clone(elem_3_ref_0);
    let mut elem_13: color::Elem = crate::color::Elem::BlockDevice;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7451() {
//    rusty_monitor::set_test_id(7451);
    let mut elem_0: color::Elem = crate::color::Elem::FileMedium;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut option_0: std::option::Option<bool> = std::option::Option::None;
    let mut option_1: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut elem_1: color::Elem = crate::color::Elem::DayOld;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_2: color::Elem = crate::color::Elem::Socket;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_3: color::Elem = crate::color::Elem::Write;
    let mut elem_4: color::Elem = crate::color::Elem::File {exec: bool_1, uid: bool_0};
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut elem_5: color::Elem = crate::color::Elem::Octal;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut elem_6: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_7: color::Elem = crate::color::Elem::Pipe;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut elem_8: color::Elem = crate::color::Elem::NonFile;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut bool_2: bool = crate::color::Elem::eq(elem_4_ref_0, elem_3_ref_0);
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut elem_9: color::Elem = crate::color::Elem::clone(elem_6_ref_0);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut elem_10: color::Elem = crate::color::Elem::User;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Context;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2683() {
//    rusty_monitor::set_test_id(2683);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Socket;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut elem_1: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_2: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_3: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_1_ref_0);
    let mut elem_4: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut elem_5: color::Elem = crate::color::Elem::DayOld;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut elem_6: color::Elem = crate::color::Elem::Pipe;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut elem_7: color::Elem = crate::color::Elem::Octal;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut elem_8: color::Elem = crate::color::Elem::clone(elem_7_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7372() {
//    rusty_monitor::set_test_id(7372);
    let mut elem_0: color::Elem = crate::color::Elem::Special;
    let mut bool_0: bool = false;
    let mut option_0: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut option_1: std::option::Option<bool> = std::option::Option::None;
    let mut option_2: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut elem_1: color::Elem = crate::color::Elem::DayOld;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_2: color::Elem = crate::color::Elem::Socket;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_3: color::Elem = crate::color::Elem::Write;
    let mut bool_1: bool = true;
    let mut elem_4: color::Elem = crate::color::Elem::File {exec: bool_0, uid: bool_1};
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut elem_5: color::Elem = crate::color::Elem::Octal;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut elem_6: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_7: color::Elem = crate::color::Elem::Pipe;
    let mut elem_8: color::Elem = crate::color::Elem::NonFile;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut elem_9: color::Elem = crate::color::Elem::clone(elem_0_ref_0);
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2467() {
//    rusty_monitor::set_test_id(2467);
    let mut str_0: &str = "exs";
    let mut elem_0: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut option_0: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_0: bool = true;
    let mut option_1: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut option_2: std::option::Option<bool> = std::option::Option::None;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut option_3: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::Some(dirgrouping_0);
    let mut bool_1: bool = true;
    let mut option_4: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut option_5: std::option::Option<flags::sorting::SortColumn> = std::option::Option::Some(sortcolumn_0);
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_5, reverse: option_4, dir_grouping: option_3};
    let mut option_6: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_0);
    let mut option_7: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut option_8: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_9: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut option_10: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_0);
    let mut bool_2: bool = false;
    let mut option_11: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut option_12: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_13: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_14: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut bool_3: bool = true;
    let mut option_15: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut option_16: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_17: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_18: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_4: bool = true;
    let mut option_19: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_0: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut u64_0: u64 = 1099511627776u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_1, order: sortorder_0, dir_grouping: dirgrouping_1};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_0: usize = 73usize;
    let mut bool_5: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_5, depth: usize_0};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_1: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut elem_1: color::Elem = crate::color::Elem::Acl;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_2: color::Elem = crate::color::Elem::DayOld;
    let mut elem_3: color::Elem = crate::color::Elem::Older;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_4: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_6: bool = true;
    let mut display_2: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut display_0_ref_0: &flags::display::Display = &mut display_0;
    let mut elem_5: color::Elem = crate::color::Elem::INode {valid: bool_6};
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut elem_6: color::Elem = crate::color::Elem::clone(elem_2_ref_0);
    let mut elem_7: color::Elem = crate::color::Elem::BlockDevice;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8787() {
//    rusty_monitor::set_test_id(8787);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Socket;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::Context;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut elem_4: color::Elem = crate::color::Elem::BlockDevice;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut bool_2: bool = false;
    let mut option_0: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut option_1: std::option::Option<bool> = std::option::Option::None;
    let mut option_2: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut elem_5: color::Elem = crate::color::Elem::DayOld;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut elem_6: color::Elem = crate::color::Elem::Socket;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut elem_7: color::Elem = crate::color::Elem::Write;
    let mut bool_3: bool = true;
    let mut elem_8: color::Elem = crate::color::Elem::File {exec: bool_2, uid: bool_3};
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut elem_9: color::Elem = crate::color::Elem::Octal;
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut elem_10: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_11: color::Elem = crate::color::Elem::File {exec: bool_0, uid: bool_1};
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut elem_12: color::Elem = crate::color::Elem::Pipe;
    let mut elem_11_ref_0: &color::Elem = &mut elem_11;
    let mut elem_13: color::Elem = crate::color::Elem::NonFile;
    let mut elem_10_ref_0: &color::Elem = &mut elem_10;
    let mut bool_4: bool = crate::color::Elem::ne(elem_8_ref_0, elem_11_ref_0);
    let mut bool_5: bool = crate::color::Elem::eq(elem_5_ref_0, elem_7_ref_0);
    let mut elem_12_ref_0: &color::Elem = &mut elem_12;
    let mut elem_14: color::Elem = crate::color::Elem::clone(elem_4_ref_0);
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut links_0: crate::color::theme::Links = crate::color::theme::Links {valid: color_3, invalid: color_2};
    let mut dir_0: crate::color::theme::Dir = crate::color::theme::Dir {uid: color_1, no_uid: color_0};
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Context;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1087() {
//    rusty_monitor::set_test_id(1087);
    let mut bool_0: bool = false;
    let mut elem_0: color::Elem = crate::color::Elem::INode {valid: bool_0};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_1: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut bool_1: bool = false;
    let mut bool_2: bool = false;
    let mut elem_2: color::Elem = crate::color::Elem::File {exec: bool_2, uid: bool_1};
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_3: color::Elem = crate::color::Elem::Context;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut elem_4: color::Elem = crate::color::Elem::User;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut elem_5: color::Elem = crate::color::Elem::User;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut elem_6: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut elem_7: color::Elem = crate::color::Elem::Pipe;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut elem_8: color::Elem = crate::color::Elem::NonFile;
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut elem_9: color::Elem = crate::color::Elem::DayOld;
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    let mut elem_10: color::Elem = crate::color::Elem::Pipe;
    let mut elem_10_ref_0: &color::Elem = &mut elem_10;
    let mut elem_11: color::Elem = crate::color::Elem::ExecSticky;
    let mut elem_11_ref_0: &color::Elem = &mut elem_11;
    let mut bool_3: bool = crate::color::Elem::eq(elem_11_ref_0, elem_10_ref_0);
    let mut bool_4: bool = crate::color::Elem::eq(elem_9_ref_0, elem_8_ref_0);
    let mut bool_5: bool = crate::color::Elem::eq(elem_7_ref_0, elem_6_ref_0);
    let mut bool_6: bool = crate::color::Elem::eq(elem_5_ref_0, elem_4_ref_0);
    let mut bool_7: bool = crate::color::Elem::eq(elem_3_ref_0, elem_2_ref_0);
    let mut bool_8: bool = crate::color::Elem::eq(elem_1_ref_0, elem_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3704() {
//    rusty_monitor::set_test_id(3704);
    let mut str_0: &str = "zsh-theme";
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut elem_0: color::Elem = crate::color::Elem::Special;
    let mut elem_1: color::Elem = crate::color::Elem::CharDevice;
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut bool_2: bool = false;
    let mut bool_3: bool = false;
    let mut bool_4: bool = true;
    let mut bool_5: bool = true;
    let mut bool_6: bool = false;
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = true;
    let mut bool_10: bool = true;
    let mut bool_11: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut bool_12: bool = true;
    let mut bool_13: bool = true;
    let mut bool_14: bool = false;
    let mut bool_15: bool = true;
    let mut bool_16: bool = true;
    let mut bool_17: bool = true;
    let mut bool_18: bool = false;
    let mut bool_19: bool = false;
    let mut bool_20: bool = true;
    let mut bool_21: bool = false;
    let mut bool_22: bool = false;
    let mut bool_23: bool = true;
    let mut permissions_1: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_23, user_write: bool_22, user_execute: bool_21, group_read: bool_20, group_write: bool_19, group_execute: bool_18, other_read: bool_17, other_write: bool_16, other_execute: bool_15, sticky: bool_14, setgid: bool_13, setuid: bool_12};
    let mut permissions_1_ref_0: &crate::meta::permissions::Permissions = &mut permissions_1;
    let mut bool_24: bool = false;
    let mut bool_25: bool = true;
    let mut bool_26: bool = false;
    let mut bool_27: bool = false;
    let mut bool_28: bool = true;
    let mut bool_29: bool = true;
    let mut bool_30: bool = false;
    let mut bool_31: bool = false;
    let mut bool_32: bool = false;
    let mut bool_33: bool = true;
    let mut bool_34: bool = false;
    let mut bool_35: bool = false;
    let mut permissions_2: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_35, user_write: bool_34, user_execute: bool_33, group_read: bool_32, group_write: bool_31, group_execute: bool_30, other_read: bool_29, other_write: bool_28, other_execute: bool_27, sticky: bool_26, setgid: bool_25, setuid: bool_24};
    let mut permissions_2_ref_0: &crate::meta::permissions::Permissions = &mut permissions_2;
    let mut bool_36: bool = true;
    let mut bool_37: bool = true;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_2: color::Elem = crate::color::Elem::Context;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut bool_38: bool = true;
    let mut elem_3: color::Elem = crate::color::Elem::INode {valid: bool_37};
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_4: color::Elem = crate::color::Elem::Group;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut elem_5: color::Elem = crate::color::Elem::NonFile;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut elem_6: color::Elem = crate::color::Elem::Socket;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut elem_7: color::Elem = crate::color::Elem::Special;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut elem_8: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut elem_9: color::Elem = crate::color::Elem::DayOld;
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut elem_10: color::Elem = crate::color::Elem::INode {valid: bool_38};
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    let mut elem_11: color::Elem = crate::color::Elem::Octal;
    let mut elem_10_ref_0: &color::Elem = &mut elem_10;
    let mut elem_12: color::Elem = crate::color::Elem::Acl;
    let mut elem_11_ref_0: &color::Elem = &mut elem_11;
    let mut elem_13: color::Elem = crate::color::Elem::DayOld;
    let mut elem_12_ref_0: &color::Elem = &mut elem_12;
    let mut elem_14: color::Elem = crate::color::Elem::Older;
    let mut elem_13_ref_0: &color::Elem = &mut elem_13;
    let mut elem_15: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_14_ref_0: &color::Elem = &mut elem_14;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut str_0_ref_0: &str = &mut str_0;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut elem_16: color::Elem = crate::color::Elem::INode {valid: bool_36};
    let mut elem_15_ref_0: &color::Elem = &mut elem_15;
    let mut elem_17: color::Elem = crate::color::Elem::clone(elem_4_ref_0);
    let mut elem_18: color::Elem = crate::color::Elem::BlockDevice;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5800() {
//    rusty_monitor::set_test_id(5800);
    let mut elem_0: color::Elem = crate::color::Elem::Older;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut tuple_0: () = crate::color::Elem::assert_receiver_is_total_eq(elem_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8984() {
//    rusty_monitor::set_test_id(8984);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut elem_0: color::Elem = crate::color::Elem::File {exec: bool_1, uid: bool_0};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::HourOld;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::HourOld;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::Older;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_4: color::Elem = crate::color::Elem::Exec;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut elem_5: color::Elem = crate::color::Elem::Group;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_5_ref_0);
    let mut theme_6: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_6_ref_0: &crate::color::theme::Theme = &mut theme_6;
    let mut elem_6: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut color_6: crossterm::style::Color = crate::color::Elem::get_color(elem_6_ref_0, theme_6_ref_0);
    let mut theme_7: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_7_ref_0: &crate::color::theme::Theme = &mut theme_7;
    let mut elem_7: color::Elem = crate::color::Elem::Socket;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut color_7: crossterm::style::Color = crate::color::Elem::get_color(elem_7_ref_0, theme_7_ref_0);
    let mut elem_8: color::Elem = crate::color::Elem::Acl;
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_1_ref_0: &meta::date::Date = &mut date_1;
    let mut date_2: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_2_ref_0: &meta::date::Date = &mut date_2;
    let mut date_3: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_3_ref_0: &meta::date::Date = &mut date_3;
    let mut date_4: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_4_ref_0: &meta::date::Date = &mut date_4;
    let mut date_5: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_5_ref_0: &meta::date::Date = &mut date_5;
    let mut date_6: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_6_ref_0: &meta::date::Date = &mut date_6;
    let mut date_7: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_7_ref_0: &meta::date::Date = &mut date_7;
    let mut date_8: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_8_ref_0: &meta::date::Date = &mut date_8;
    let mut date_9: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut elem_9: color::Elem = crate::color::Elem::clone(elem_8_ref_0);
    let mut elem_10: color::Elem = crate::color::Elem::Acl;
    let mut permission_0: crate::color::theme::Permission = crate::color::theme::Permission {read: color_7, write: color_6, exec: color_5, exec_sticky: color_4, no_access: color_3, octal: color_2, acl: color_1, context: color_0};
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_403() {
//    rusty_monitor::set_test_id(403);
    let mut elem_0: color::Elem = crate::color::Elem::Socket;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut option_0: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_0: &str = "HJ3A4c0";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_1: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut colors_0: crate::color::Colors = crate::color::Colors {theme: option_1, lscolors: option_0};
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut elem_1: color::Elem = crate::color::Elem::Read;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut option_2: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut option_3: std::option::Option<crate::color::theme::Theme> = std::option::Option::Some(theme_0);
    let mut colors_1: crate::color::Colors = crate::color::Colors {theme: option_3, lscolors: option_2};
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut elem_2: color::Elem = crate::color::Elem::Exec;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut option_4: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_1: &str = "auto";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut option_5: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_1_ref_0);
    let mut colors_2: crate::color::Colors = crate::color::Colors {theme: option_5, lscolors: option_4};
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut elem_3: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_3: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_3_ref_0: &crate::color::Colors = &mut colors_3;
    let mut elem_4: color::Elem = crate::color::Elem::Read;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_4: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_4_ref_0: &crate::color::Colors = &mut colors_4;
    let mut option_6: std::option::Option<lscolors::Indicator> = crate::color::Colors::get_indicator_from_elem(colors_4_ref_0, elem_4_ref_0);
    let mut option_7: std::option::Option<lscolors::Indicator> = crate::color::Colors::get_indicator_from_elem(colors_3_ref_0, elem_3_ref_0);
    let mut option_8: std::option::Option<lscolors::Indicator> = crate::color::Colors::get_indicator_from_elem(colors_2_ref_0, elem_2_ref_0);
    let mut option_9: std::option::Option<lscolors::Indicator> = crate::color::Colors::get_indicator_from_elem(colors_1_ref_0, elem_1_ref_0);
    let mut option_10: std::option::Option<lscolors::Indicator> = crate::color::Colors::get_indicator_from_elem(colors_0_ref_0, elem_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5804() {
//    rusty_monitor::set_test_id(5804);
    let mut str_0: &str = "tex";
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut elem_0: color::Elem = crate::color::Elem::Group;
    let mut elem_1: color::Elem = crate::color::Elem::HourOld;
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut option_0: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut option_1: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_1, theme: option_0};
    let mut option_2: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_3: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_4: bool = true;
    let mut option_4: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_2: color::Elem = crate::color::Elem::Acl;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_3: color::Elem = crate::color::Elem::ExecSticky;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_4: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut elem_5: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut bool_5: bool = false;
    let mut elem_6: color::Elem = crate::color::Elem::File {exec: bool_2, uid: bool_0};
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut elem_7: color::Elem = crate::color::Elem::Acl;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut elem_8: color::Elem = crate::color::Elem::Older;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut elem_9: color::Elem = crate::color::Elem::Older;
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut elem_10: color::Elem = crate::color::Elem::Socket;
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    let mut elem_11: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_10_ref_0: &color::Elem = &mut elem_10;
    let mut elem_12: color::Elem = crate::color::Elem::Exec;
    let mut elem_11_ref_0: &color::Elem = &mut elem_11;
    let mut elem_13: color::Elem = crate::color::Elem::Context;
    let mut elem_12_ref_0: &color::Elem = &mut elem_12;
    let mut elem_14: color::Elem = crate::color::Elem::INode {valid: bool_3};
    let mut elem_13_ref_0: &color::Elem = &mut elem_13;
    let mut elem_15: color::Elem = crate::color::Elem::Group;
    let mut elem_14_ref_0: &color::Elem = &mut elem_14;
    let mut elem_16: color::Elem = crate::color::Elem::NonFile;
    let mut elem_15_ref_0: &color::Elem = &mut elem_15;
    let mut elem_17: color::Elem = crate::color::Elem::Socket;
    let mut elem_16_ref_0: &color::Elem = &mut elem_16;
    let mut elem_18: color::Elem = crate::color::Elem::Special;
    let mut elem_17_ref_0: &color::Elem = &mut elem_17;
    let mut elem_19: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_18_ref_0: &color::Elem = &mut elem_18;
    let mut elem_20: color::Elem = crate::color::Elem::DayOld;
    let mut elem_19_ref_0: &color::Elem = &mut elem_19;
    let mut elem_21: color::Elem = crate::color::Elem::INode {valid: bool_1};
    let mut elem_20_ref_0: &color::Elem = &mut elem_20;
    let mut elem_22: color::Elem = crate::color::Elem::Octal;
    let mut elem_21_ref_0: &color::Elem = &mut elem_21;
    let mut elem_23: color::Elem = crate::color::Elem::Acl;
    let mut elem_22_ref_0: &color::Elem = &mut elem_22;
    let mut elem_24: color::Elem = crate::color::Elem::DayOld;
    let mut elem_23_ref_0: &color::Elem = &mut elem_23;
    let mut elem_25: color::Elem = crate::color::Elem::Older;
    let mut elem_24_ref_0: &color::Elem = &mut elem_24;
    let mut elem_26: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_25_ref_0: &color::Elem = &mut elem_25;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut str_0_ref_0: &str = &mut str_0;
    let mut display_1: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut display_0_ref_0: &flags::display::Display = &mut display_0;
    let mut elem_27: color::Elem = crate::color::Elem::INode {valid: bool_5};
    let mut elem_26_ref_0: &color::Elem = &mut elem_26;
    let mut elem_28: color::Elem = crate::color::Elem::clone(elem_8_ref_0);
    let mut elem_29: color::Elem = crate::color::Elem::BlockDevice;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1092() {
//    rusty_monitor::set_test_id(1092);
    let mut elem_0: color::Elem = crate::color::Elem::BlockDevice;
    let mut bool_0: bool = true;
    let mut elem_1: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut bool_1: bool = false;
    let mut elem_2: color::Elem = crate::color::Elem::File {exec: bool_0, uid: bool_1};
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_3: color::Elem = crate::color::Elem::Pipe;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_4: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut elem_5: color::Elem = crate::color::Elem::NonFile;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut bool_2: bool = crate::color::Elem::ne(elem_0_ref_0, elem_2_ref_0);
    let mut bool_3: bool = crate::color::Elem::ne(elem_4_ref_0, elem_3_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5722() {
//    rusty_monitor::set_test_id(5722);
    let mut bool_0: bool = false;
    let mut elem_0: color::Elem = crate::color::Elem::Links {valid: bool_0};
    let mut elem_1: color::Elem = crate::color::Elem::User;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_2: color::Elem = crate::color::Elem::Exec;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::None;
    let mut unit_0_ref_0: &meta::size::Unit = &mut unit_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_0: usize = 40usize;
    let mut bool_1: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_1, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_3: color::Elem = crate::color::Elem::clone(elem_1_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6651() {
//    rusty_monitor::set_test_id(6651);
    let mut elem_0: color::Elem = crate::color::Elem::Special;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut bool_2: bool = false;
    let mut option_0: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut option_1: std::option::Option<bool> = std::option::Option::None;
    let mut option_2: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut elem_1: color::Elem = crate::color::Elem::DayOld;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_2: color::Elem = crate::color::Elem::Socket;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut bool_3: bool = true;
    let mut elem_3: color::Elem = crate::color::Elem::File {exec: bool_2, uid: bool_3};
    let mut elem_4: color::Elem = crate::color::Elem::Octal;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut elem_5: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_6: color::Elem = crate::color::Elem::File {exec: bool_0, uid: bool_1};
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_7: color::Elem = crate::color::Elem::Pipe;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut elem_8: color::Elem = crate::color::Elem::NonFile;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut bool_4: bool = crate::color::Elem::ne(elem_3_ref_0, elem_6_ref_0);
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut elem_9: color::Elem = crate::color::Elem::clone(elem_0_ref_0);
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5499() {
//    rusty_monitor::set_test_id(5499);
    let mut str_0: &str = "color";
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut elem_0: color::Elem = crate::color::Elem::CharDevice;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut option_0: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut option_1: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut bool_4: bool = true;
    let mut elem_1: color::Elem = crate::color::Elem::File {exec: bool_0, uid: bool_2};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_2: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_3: color::Elem = crate::color::Elem::Context;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut bool_5: bool = true;
    let mut elem_4: color::Elem = crate::color::Elem::Dir {uid: bool_3};
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut elem_5: color::Elem = crate::color::Elem::Acl;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut elem_6: color::Elem = crate::color::Elem::ExecSticky;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut elem_7: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut elem_8: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut bool_6: bool = false;
    let mut elem_9: color::Elem = crate::color::Elem::File {exec: bool_1, uid: bool_6};
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut elem_10: color::Elem = crate::color::Elem::Acl;
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    let mut elem_11: color::Elem = crate::color::Elem::Older;
    let mut elem_10_ref_0: &color::Elem = &mut elem_10;
    let mut elem_12: color::Elem = crate::color::Elem::Older;
    let mut elem_11_ref_0: &color::Elem = &mut elem_11;
    let mut elem_13: color::Elem = crate::color::Elem::Socket;
    let mut elem_12_ref_0: &color::Elem = &mut elem_12;
    let mut elem_14: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_13_ref_0: &color::Elem = &mut elem_13;
    let mut elem_15: color::Elem = crate::color::Elem::Exec;
    let mut elem_14_ref_0: &color::Elem = &mut elem_14;
    let mut elem_16: color::Elem = crate::color::Elem::Context;
    let mut elem_15_ref_0: &color::Elem = &mut elem_15;
    let mut bool_7: bool = true;
    let mut elem_17: color::Elem = crate::color::Elem::INode {valid: bool_4};
    let mut elem_16_ref_0: &color::Elem = &mut elem_16;
    let mut elem_18: color::Elem = crate::color::Elem::Group;
    let mut elem_17_ref_0: &color::Elem = &mut elem_17;
    let mut elem_19: color::Elem = crate::color::Elem::NonFile;
    let mut elem_18_ref_0: &color::Elem = &mut elem_18;
    let mut elem_20: color::Elem = crate::color::Elem::Socket;
    let mut elem_21: color::Elem = crate::color::Elem::Special;
    let mut elem_20_ref_0: &color::Elem = &mut elem_20;
    let mut elem_22: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_21_ref_0: &color::Elem = &mut elem_21;
    let mut elem_23: color::Elem = crate::color::Elem::DayOld;
    let mut elem_22_ref_0: &color::Elem = &mut elem_22;
    let mut elem_24: color::Elem = crate::color::Elem::INode {valid: bool_7};
    let mut elem_23_ref_0: &color::Elem = &mut elem_23;
    let mut elem_25: color::Elem = crate::color::Elem::Octal;
    let mut elem_24_ref_0: &color::Elem = &mut elem_24;
    let mut elem_26: color::Elem = crate::color::Elem::Acl;
    let mut elem_25_ref_0: &color::Elem = &mut elem_25;
    let mut elem_27: color::Elem = crate::color::Elem::DayOld;
    let mut elem_26_ref_0: &color::Elem = &mut elem_26;
    let mut elem_28: color::Elem = crate::color::Elem::Older;
    let mut elem_27_ref_0: &color::Elem = &mut elem_27;
    let mut elem_29: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_28_ref_0: &color::Elem = &mut elem_28;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut str_0_ref_0: &str = &mut str_0;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut elem_30: color::Elem = crate::color::Elem::INode {valid: bool_5};
    let mut elem_29_ref_0: &color::Elem = &mut elem_29;
    let mut elem_31: color::Elem = crate::color::Elem::clone(elem_1_ref_0);
    let mut elem_32: color::Elem = crate::color::Elem::BlockDevice;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_461() {
//    rusty_monitor::set_test_id(461);
    let mut elem_0: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_1: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_2: color::Elem = crate::color::Elem::ExecSticky;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_3: color::Elem = crate::color::Elem::HourOld;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut bool_0: bool = false;
    let mut elem_4: color::Elem = crate::color::Elem::Dir {uid: bool_0};
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut bool_1: bool = false;
    let mut elem_5: color::Elem = crate::color::Elem::Links {valid: bool_1};
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut bool_2: bool = false;
    let mut elem_6: color::Elem = crate::color::Elem::Dir {uid: bool_2};
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut elem_7: color::Elem = crate::color::Elem::Read;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut elem_8: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut elem_9: color::Elem = crate::color::Elem::SymLink;
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    let mut elem_10: color::Elem = crate::color::Elem::clone(elem_9_ref_0);
    let mut elem_11: color::Elem = crate::color::Elem::clone(elem_8_ref_0);
    let mut elem_12: color::Elem = crate::color::Elem::clone(elem_7_ref_0);
    let mut elem_13: color::Elem = crate::color::Elem::clone(elem_6_ref_0);
    let mut elem_14: color::Elem = crate::color::Elem::clone(elem_5_ref_0);
    let mut elem_15: color::Elem = crate::color::Elem::clone(elem_4_ref_0);
    let mut elem_16: color::Elem = crate::color::Elem::clone(elem_3_ref_0);
    let mut elem_17: color::Elem = crate::color::Elem::clone(elem_2_ref_0);
    let mut elem_18: color::Elem = crate::color::Elem::clone(elem_1_ref_0);
    let mut elem_19: color::Elem = crate::color::Elem::clone(elem_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6022() {
//    rusty_monitor::set_test_id(6022);
    let mut elem_0: color::Elem = crate::color::Elem::SymLink;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut elem_1: color::Elem = crate::color::Elem::File {exec: bool_1, uid: bool_0};
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_0);
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_2: std::option::Option<bool> = std::option::Option::None;
    let mut bool_2: bool = true;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut option_4: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::None;
    let mut option_5: std::option::Option<bool> = std::option::Option::None;
    let mut option_6: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_6, reverse: option_5, dir_grouping: option_4};
    let mut option_7: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_0);
    let mut option_8: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut option_9: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_0);
    let mut option_10: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut bool_3: bool = false;
    let mut option_11: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut option_12: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_13: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut display_0: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut option_14: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut option_15: std::option::Option<bool> = std::option::Option::None;
    let mut option_16: std::option::Option<std::string::String> = std::option::Option::None;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_2: color::Elem = crate::color::Elem::clone(elem_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4910() {
//    rusty_monitor::set_test_id(4910);
    let mut option_0: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_0: &str = "c++";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_1: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut colors_0: crate::color::Colors = crate::color::Colors {theme: option_1, lscolors: option_0};
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut elem_0: color::Elem = crate::color::Elem::FileSmall;
    let mut bool_0: bool = false;
    let mut elem_1: color::Elem = crate::color::Elem::INode {valid: bool_0};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_2: color::Elem = crate::color::Elem::HourOld;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_3: color::Elem = crate::color::Elem::DayOld;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut elem_4: color::Elem = crate::color::Elem::Socket;
    let mut elem_5: color::Elem = crate::color::Elem::Write;
    let mut bool_1: bool = true;
    let mut bool_2: bool = false;
    let mut elem_6: color::Elem = crate::color::Elem::File {exec: bool_2, uid: bool_1};
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut elem_7: color::Elem = crate::color::Elem::Octal;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut elem_8: color::Elem = crate::color::Elem::NoAccess;
    let mut bool_3: bool = false;
    let mut bool_4: bool = false;
    let mut elem_9: color::Elem = crate::color::Elem::File {exec: bool_4, uid: bool_3};
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    let mut elem_10: color::Elem = crate::color::Elem::Pipe;
    let mut elem_10_ref_0: &color::Elem = &mut elem_10;
    let mut elem_11: color::Elem = crate::color::Elem::NonFile;
    let mut elem_11_ref_0: &color::Elem = &mut elem_11;
    let mut bool_5: bool = crate::color::Elem::ne(elem_10_ref_0, elem_9_ref_0);
    let mut bool_6: bool = crate::color::Elem::ne(elem_3_ref_0, elem_2_ref_0);
    let mut bool_7: bool = crate::color::Elem::eq(elem_7_ref_0, elem_0_ref_0);
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_12: color::Elem = crate::color::Elem::clone(elem_1_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8937() {
//    rusty_monitor::set_test_id(8937);
    let mut elem_0: color::Elem = crate::color::Elem::Context;
    let mut str_0: &str = "gsheet";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut elem_1: color::Elem = crate::color::Elem::File {exec: bool_0, uid: bool_1};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_2: color::Elem = crate::color::Elem::Octal;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_3: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_4: color::Elem = crate::color::Elem::Pipe;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut bool_2: bool = crate::color::Elem::eq(elem_4_ref_0, elem_1_ref_0);
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut elem_5: color::Elem = crate::color::Elem::clone(elem_2_ref_0);
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2067() {
//    rusty_monitor::set_test_id(2067);
    let mut bool_0: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_0};
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut bool_3: bool = false;
    let mut bool_4: bool = true;
    let mut bool_5: bool = true;
    let mut bool_6: bool = false;
    let mut bool_7: bool = false;
    let mut bool_8: bool = false;
    let mut bool_9: bool = false;
    let mut bool_10: bool = false;
    let mut bool_11: bool = true;
    let mut bool_12: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_12, user_write: bool_11, user_execute: bool_10, group_read: bool_9, group_write: bool_8, group_execute: bool_7, other_read: bool_6, other_write: bool_5, other_execute: bool_4, sticky: bool_3, setgid: bool_2, setuid: bool_1};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_0: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut option_1: std::option::Option<crate::color::theme::Theme> = std::option::Option::Some(theme_0);
    let mut colors_0: crate::color::Colors = crate::color::Colors {theme: option_1, lscolors: option_0};
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut bool_13: bool = crate::meta::permissions::Permissions::is_executable(permissions_0_ref_0);
    let mut dateflag_0_ref_0: &flags::date::DateFlag = &mut dateflag_0;
    let mut elem_0: color::Elem = crate::color::Elem::FileSmall;
    let mut bool_14: bool = crate::meta::filetype::FileType::is_dirlike(filetype_0);
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut bool_15: bool = crate::color::Elem::has_suid(elem_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8214() {
//    rusty_monitor::set_test_id(8214);
    let mut elem_0: color::Elem = crate::color::Elem::Pipe;
    let mut str_0: &str = "Acl";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "first";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut usize_0: usize = 1usize;
    let mut tuple_0: (usize, &str) = (usize_0, str_2_ref_0);
    let mut elem_1: color::Elem = crate::color::Elem::Pipe;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_2: color::Elem = crate::color::Elem::NonFile;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_3: color::Elem = crate::color::Elem::clone(elem_1_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_9042() {
//    rusty_monitor::set_test_id(9042);
    let mut str_0: &str = "NoIcon";
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut elem_0: color::Elem = crate::color::Elem::Special;
    let mut bool_0: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_0};
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut bool_3: bool = false;
    let mut bool_4: bool = false;
    let mut bool_5: bool = false;
    let mut bool_6: bool = false;
    let mut elem_1: color::Elem = crate::color::Elem::File {exec: bool_4, uid: bool_1};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_2: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_3: color::Elem = crate::color::Elem::Context;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_4: color::Elem = crate::color::Elem::Dir {uid: bool_2};
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut elem_5: color::Elem = crate::color::Elem::Acl;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut elem_6: color::Elem = crate::color::Elem::ExecSticky;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut elem_7: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut elem_8: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut bool_7: bool = false;
    let mut elem_9: color::Elem = crate::color::Elem::File {exec: bool_6, uid: bool_3};
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut elem_10: color::Elem = crate::color::Elem::Acl;
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    let mut elem_11: color::Elem = crate::color::Elem::Older;
    let mut elem_10_ref_0: &color::Elem = &mut elem_10;
    let mut elem_12: color::Elem = crate::color::Elem::Older;
    let mut elem_11_ref_0: &color::Elem = &mut elem_11;
    let mut elem_13: color::Elem = crate::color::Elem::Socket;
    let mut elem_12_ref_0: &color::Elem = &mut elem_12;
    let mut elem_14: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_13_ref_0: &color::Elem = &mut elem_13;
    let mut elem_15: color::Elem = crate::color::Elem::Exec;
    let mut elem_14_ref_0: &color::Elem = &mut elem_14;
    let mut elem_16: color::Elem = crate::color::Elem::Context;
    let mut elem_15_ref_0: &color::Elem = &mut elem_15;
    let mut elem_17: color::Elem = crate::color::Elem::INode {valid: bool_5};
    let mut elem_16_ref_0: &color::Elem = &mut elem_16;
    let mut elem_18: color::Elem = crate::color::Elem::Group;
    let mut elem_17_ref_0: &color::Elem = &mut elem_17;
    let mut elem_19: color::Elem = crate::color::Elem::NonFile;
    let mut elem_18_ref_0: &color::Elem = &mut elem_18;
    let mut elem_20: color::Elem = crate::color::Elem::Socket;
    let mut elem_19_ref_0: &color::Elem = &mut elem_19;
    let mut elem_21: color::Elem = crate::color::Elem::Special;
    let mut elem_20_ref_0: &color::Elem = &mut elem_20;
    let mut elem_22: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_21_ref_0: &color::Elem = &mut elem_21;
    let mut elem_23: color::Elem = crate::color::Elem::DayOld;
    let mut elem_22_ref_0: &color::Elem = &mut elem_22;
    let mut bool_8: bool = false;
    let mut elem_24: color::Elem = crate::color::Elem::INode {valid: bool_7};
    let mut elem_23_ref_0: &color::Elem = &mut elem_23;
    let mut elem_25: color::Elem = crate::color::Elem::Octal;
    let mut elem_24_ref_0: &color::Elem = &mut elem_24;
    let mut elem_26: color::Elem = crate::color::Elem::Acl;
    let mut elem_25_ref_0: &color::Elem = &mut elem_25;
    let mut elem_27: color::Elem = crate::color::Elem::DayOld;
    let mut elem_26_ref_0: &color::Elem = &mut elem_26;
    let mut elem_28: color::Elem = crate::color::Elem::Older;
    let mut elem_27_ref_0: &color::Elem = &mut elem_27;
    let mut elem_29: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_28_ref_0: &color::Elem = &mut elem_28;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut str_0_ref_0: &str = &mut str_0;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut elem_30: color::Elem = crate::color::Elem::INode {valid: bool_8};
    let mut elem_29_ref_0: &color::Elem = &mut elem_29;
    let mut elem_31: color::Elem = crate::color::Elem::clone(elem_13_ref_0);
    let mut elem_32: color::Elem = crate::color::Elem::BlockDevice;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1720() {
//    rusty_monitor::set_test_id(1720);
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut elem_0: color::Elem = crate::color::Elem::File {exec: bool_1, uid: bool_0};
    let mut str_0: &str = "config-file";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u64_0: u64 = 1073741824u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut u64_1: u64 = 1048576u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut str_1: &str = "Time";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut elem_1: color::Elem = crate::color::Elem::Pipe;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_2: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_3: color::Elem = crate::color::Elem::NonFile;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut bool_2: bool = crate::color::Elem::ne(elem_0_ref_0, elem_1_ref_0);
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut elem_4: color::Elem = crate::color::Elem::clone(elem_3_ref_0);
    let mut elem_5: color::Elem = crate::color::Elem::Socket;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8437() {
//    rusty_monitor::set_test_id(8437);
    let mut str_0: &str = ".clang-format";
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut elem_0: color::Elem = crate::color::Elem::User;
    let mut elem_1: color::Elem = crate::color::Elem::Pipe;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_0);
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_2: std::option::Option<bool> = std::option::Option::None;
    let mut bool_0: bool = true;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut option_4: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::None;
    let mut option_5: std::option::Option<bool> = std::option::Option::None;
    let mut option_6: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_6, reverse: option_5, dir_grouping: option_4};
    let mut option_7: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_0);
    let mut option_8: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut option_9: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_0);
    let mut option_10: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut option_11: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_0);
    let mut bool_1: bool = false;
    let mut option_12: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_13: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_14: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut display_0: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut option_15: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut option_16: std::option::Option<bool> = std::option::Option::None;
    let mut option_17: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_2: bool = true;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_2: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut bool_3: bool = false;
    let mut bool_4: bool = false;
    let mut elem_3: color::Elem = crate::color::Elem::File {exec: bool_3, uid: bool_2};
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_4: color::Elem = crate::color::Elem::Acl;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut elem_5: color::Elem = crate::color::Elem::Older;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut elem_6: color::Elem = crate::color::Elem::Older;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut elem_7: color::Elem = crate::color::Elem::Socket;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut elem_8: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut elem_9: color::Elem = crate::color::Elem::Exec;
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut elem_10: color::Elem = crate::color::Elem::Context;
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    let mut bool_5: bool = true;
    let mut elem_11: color::Elem = crate::color::Elem::INode {valid: bool_4};
    let mut elem_10_ref_0: &color::Elem = &mut elem_10;
    let mut elem_12: color::Elem = crate::color::Elem::Group;
    let mut elem_11_ref_0: &color::Elem = &mut elem_11;
    let mut elem_13: color::Elem = crate::color::Elem::NonFile;
    let mut elem_12_ref_0: &color::Elem = &mut elem_12;
    let mut elem_14: color::Elem = crate::color::Elem::Socket;
    let mut elem_13_ref_0: &color::Elem = &mut elem_13;
    let mut elem_15: color::Elem = crate::color::Elem::Special;
    let mut elem_14_ref_0: &color::Elem = &mut elem_14;
    let mut elem_16: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_15_ref_0: &color::Elem = &mut elem_15;
    let mut elem_17: color::Elem = crate::color::Elem::DayOld;
    let mut elem_16_ref_0: &color::Elem = &mut elem_16;
    let mut bool_6: bool = false;
    let mut elem_18: color::Elem = crate::color::Elem::INode {valid: bool_5};
    let mut elem_17_ref_0: &color::Elem = &mut elem_17;
    let mut elem_19: color::Elem = crate::color::Elem::Octal;
    let mut elem_18_ref_0: &color::Elem = &mut elem_18;
    let mut elem_20: color::Elem = crate::color::Elem::Acl;
    let mut elem_19_ref_0: &color::Elem = &mut elem_19;
    let mut elem_21: color::Elem = crate::color::Elem::DayOld;
    let mut elem_20_ref_0: &color::Elem = &mut elem_20;
    let mut elem_22: color::Elem = crate::color::Elem::Older;
    let mut elem_21_ref_0: &color::Elem = &mut elem_21;
    let mut elem_23: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_22_ref_0: &color::Elem = &mut elem_22;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut str_0_ref_0: &str = &mut str_0;
    let mut display_1: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut elem_24: color::Elem = crate::color::Elem::INode {valid: bool_6};
    let mut elem_23_ref_0: &color::Elem = &mut elem_23;
    let mut elem_25: color::Elem = crate::color::Elem::clone(elem_2_ref_0);
    let mut elem_26: color::Elem = crate::color::Elem::BlockDevice;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_106() {
//    rusty_monitor::set_test_id(106);
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut elem_0: color::Elem = crate::color::Elem::File {exec: bool_1, uid: bool_0};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut bool_2: bool = false;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut bool_5: bool = true;
    let mut bool_6: bool = false;
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = false;
    let mut bool_10: bool = false;
    let mut bool_11: bool = true;
    let mut bool_12: bool = true;
    let mut bool_13: bool = true;
    let mut option_0: std::option::Option<usize> = std::option::Option::None;
    let mut option_1: std::option::Option<bool> = std::option::Option::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut bool_14: bool = true;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut elem_1: color::Elem = crate::color::Elem::Links {valid: bool_14};
    let mut elem_2: color::Elem = crate::color::Elem::Octal;
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_1, depth: option_0};
    let mut elem_3: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_4: color::Elem = crate::color::Elem::clone(elem_1_ref_0);
    let mut elem_5: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_13, user_write: bool_12, user_execute: bool_11, group_read: bool_10, group_write: bool_9, group_execute: bool_8, other_read: bool_7, other_write: bool_6, other_execute: bool_5, sticky: bool_4, setgid: bool_3, setuid: bool_2};
    let mut option_2: std::option::Option<lscolors::Indicator> = crate::color::Colors::get_indicator_from_elem(colors_0_ref_0, elem_0_ref_0);
//    panic!("From RustyUnit with love");
}
}