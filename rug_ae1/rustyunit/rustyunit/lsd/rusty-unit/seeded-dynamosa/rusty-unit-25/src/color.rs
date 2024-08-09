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
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3610() {
//    rusty_monitor::set_test_id(3610);
    let mut option_0: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_0: &str = "less";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_1: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut colors_0: crate::color::Colors = crate::color::Colors {theme: option_1, lscolors: option_0};
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut option_2: std::option::Option<crate::color::theme::Theme> = std::option::Option::Some(theme_0);
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut option_3: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    crate::meta::filetype::FileType::render(filetype_3, colors_2_ref_0);
    crate::meta::filetype::FileType::render(filetype_0, colors_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8565() {
//    rusty_monitor::set_test_id(8565);
    let mut elem_0: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut option_0: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_0: &str = "less";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_1: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut colors_1: crate::color::Colors = crate::color::Colors {theme: option_1, lscolors: option_0};
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_1, exec: bool_0};
    let mut option_2: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut option_3: std::option::Option<crate::color::theme::Theme> = std::option::Option::Some(theme_0);
    let mut colors_3: crate::color::Colors = crate::color::Colors {theme: option_3, lscolors: option_2};
    let mut colors_3_ref_0: &crate::color::Colors = &mut colors_3;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut option_4: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut option_5: std::option::Option<crate::color::theme::Theme> = std::option::Option::None;
    let mut colors_4: crate::color::Colors = crate::color::Colors {theme: option_5, lscolors: option_4};
    let mut colors_4_ref_0: &crate::color::Colors = &mut colors_4;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_5: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_5_ref_0: &crate::color::Colors = &mut colors_5;
    let mut filetype_4: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    crate::meta::filetype::FileType::render(filetype_4, colors_5_ref_0);
    crate::meta::filetype::FileType::render(filetype_3, colors_4_ref_0);
    crate::meta::filetype::FileType::render(filetype_2, colors_3_ref_0);
    crate::meta::filetype::FileType::render(filetype_1, colors_2_ref_0);
    crate::meta::filetype::FileType::render(filetype_0, colors_1_ref_0);
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut sizeflag_0_ref_0: &flags::size::SizeFlag = &mut sizeflag_0;
    let mut option_6: std::option::Option<lscolors::Indicator> = crate::color::Colors::get_indicator_from_elem(colors_0_ref_0, elem_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8394() {
//    rusty_monitor::set_test_id(8394);
    let mut elem_0: color::Elem = crate::color::Elem::Octal;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut bool_0: bool = true;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Byte;
    let mut elem_1: color::Elem = crate::color::Elem::Dir {uid: bool_0};
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut bool_1: bool = crate::color::Elem::ne(elem_1_ref_0, elem_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4508() {
//    rusty_monitor::set_test_id(4508);
    let mut option_0: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_0: &str = "less";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_1, exec: bool_0};
    let mut option_1: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut option_2: std::option::Option<crate::color::theme::Theme> = std::option::Option::Some(theme_0);
    let mut colors_1: crate::color::Colors = crate::color::Colors {theme: option_2, lscolors: option_1};
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut option_3: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut option_4: std::option::Option<crate::color::theme::Theme> = std::option::Option::None;
    let mut colors_2: crate::color::Colors = crate::color::Colors {theme: option_4, lscolors: option_3};
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_3: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_3_ref_0: &crate::color::Colors = &mut colors_3;
    let mut filetype_4: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    crate::meta::filetype::FileType::render(filetype_4, colors_3_ref_0);
    crate::meta::filetype::FileType::render(filetype_2, colors_1_ref_0);
    crate::meta::filetype::FileType::render(filetype_1, colors_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_948() {
//    rusty_monitor::set_test_id(948);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut bool_0: bool = false;
    let mut elem_0: color::Elem = crate::color::Elem::Links {valid: bool_0};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut bool_1: bool = true;
    let mut elem_1: color::Elem = crate::color::Elem::INode {valid: bool_1};
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_2: color::Elem = crate::color::Elem::Special;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_3: color::Elem = crate::color::Elem::Read;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut elem_4: color::Elem = crate::color::Elem::Exec;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut elem_5: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut elem_6: color::Elem = crate::color::Elem::Special;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut bool_2: bool = true;
    let mut elem_7: color::Elem = crate::color::Elem::Dir {uid: bool_2};
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut elem_8: color::Elem = crate::color::Elem::Octal;
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut elem_9: color::Elem = crate::color::Elem::Socket;
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    let mut elem_10: color::Elem = crate::color::Elem::Exec;
    let mut elem_10_ref_0: &color::Elem = &mut elem_10;
    let mut elem_11: color::Elem = crate::color::Elem::Octal;
    let mut elem_11_ref_0: &color::Elem = &mut elem_11;
    let mut bool_3: bool = crate::color::Elem::eq(elem_11_ref_0, elem_10_ref_0);
    let mut bool_4: bool = crate::color::Elem::eq(elem_9_ref_0, elem_8_ref_0);
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2434() {
//    rusty_monitor::set_test_id(2434);
    let mut elem_0: color::Elem = crate::color::Elem::Read;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_1: color::Elem = crate::color::Elem::Write;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_2: color::Elem = crate::color::Elem::Write;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_1_ref_0);
    let mut elem_3: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut str_0: &str = "Natural sort of (version) numbers within text";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_1_ref_0: &meta::date::Date = &mut date_1;
    let mut date_2: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut option_0: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut elem_4: color::Elem = crate::color::Elem::ExecSticky;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut tuple_0: () = crate::color::Elem::assert_receiver_is_total_eq(elem_4_ref_0);
    let mut elem_5: color::Elem = crate::color::Elem::clone(elem_0_ref_0);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut bool_0: bool = crate::meta::filetype::FileType::is_dirlike(filetype_0);
    let mut inode_0: crate::color::theme::INode = crate::color::theme::INode {valid: color_1, invalid: color_0};
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::SizeValue;
    let mut bool_1: bool = crate::meta::filetype::FileType::is_dirlike(filetype_1);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::None;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_9000() {
//    rusty_monitor::set_test_id(9000);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut filetype_0_ref_0: &meta::filetype::FileType = &mut filetype_0;
    let mut u64_0: u64 = 93u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut str_0: &str = "ckoC";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut str_1: &str = "cshtml";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut u64_1: u64 = 1073741824u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_1, exec: bool_0};
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::INode;
    let mut filetype_3_ref_0: &meta::filetype::FileType = &mut filetype_3;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut elem_0: color::Elem = crate::color::Elem::Group;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_1: color::Elem = crate::color::Elem::clone(elem_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2890() {
//    rusty_monitor::set_test_id(2890);
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_1, exec: bool_0};
    let mut option_0: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_0: &str = "less";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_1: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut colors_0: crate::color::Colors = crate::color::Colors {theme: option_1, lscolors: option_0};
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_3, exec: bool_2};
    let mut option_2: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut option_3: std::option::Option<crate::color::theme::Theme> = std::option::Option::Some(theme_0);
    let mut colors_2: crate::color::Colors = crate::color::Colors {theme: option_3, lscolors: option_2};
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut option_4: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut option_5: std::option::Option<crate::color::theme::Theme> = std::option::Option::None;
    let mut colors_3: crate::color::Colors = crate::color::Colors {theme: option_5, lscolors: option_4};
    let mut colors_3_ref_0: &crate::color::Colors = &mut colors_3;
    let mut filetype_4: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_4: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_4_ref_0: &crate::color::Colors = &mut colors_4;
    let mut filetype_5: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    crate::meta::filetype::FileType::render(filetype_0, colors_4_ref_0);
    crate::meta::filetype::FileType::render(filetype_4, colors_3_ref_0);
    crate::meta::filetype::FileType::render(filetype_3, colors_2_ref_0);
    crate::meta::filetype::FileType::render(filetype_2, colors_1_ref_0);
    crate::meta::filetype::FileType::render(filetype_1, colors_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3466() {
//    rusty_monitor::set_test_id(3466);
    let mut elem_0: color::Elem = crate::color::Elem::HourOld;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_1: color::Elem = crate::color::Elem::Write;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_2: color::Elem = crate::color::Elem::Write;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_1_ref_0);
    let mut elem_3: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut str_0: &str = "Natural sort of (version) numbers within text";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_1_ref_0: &meta::date::Date = &mut date_1;
    let mut option_0: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut elem_4: color::Elem = crate::color::Elem::ExecSticky;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut tuple_0: () = crate::color::Elem::assert_receiver_is_total_eq(elem_4_ref_0);
    let mut elem_5: color::Elem = crate::color::Elem::clone(elem_0_ref_0);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut bool_0: bool = crate::meta::filetype::FileType::is_dirlike(filetype_0);
    let mut inode_0: crate::color::theme::INode = crate::color::theme::INode {valid: color_1, invalid: color_0};
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::SizeValue;
    let mut bool_1: bool = crate::meta::filetype::FileType::is_dirlike(filetype_1);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1034() {
//    rusty_monitor::set_test_id(1034);
    let mut elem_0: color::Elem = crate::color::Elem::NoAccess;
    let mut bool_0: bool = true;
    let mut elem_1: color::Elem = crate::color::Elem::FileMedium;
    let mut str_0: &str = "gitignore_global";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut bool_1: bool = true;
    let mut elem_2: color::Elem = crate::color::Elem::INode {valid: bool_0};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_3: color::Elem = crate::color::Elem::Special;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_4: color::Elem = crate::color::Elem::HourOld;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut elem_5: color::Elem = crate::color::Elem::Exec;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut elem_6: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut elem_7: color::Elem = crate::color::Elem::Special;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut elem_8: color::Elem = crate::color::Elem::Dir {uid: bool_1};
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut elem_9: color::Elem = crate::color::Elem::Octal;
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut elem_10: color::Elem = crate::color::Elem::Socket;
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    let mut elem_11: color::Elem = crate::color::Elem::Exec;
    let mut elem_10_ref_0: &color::Elem = &mut elem_10;
    let mut elem_12: color::Elem = crate::color::Elem::Octal;
    let mut elem_11_ref_0: &color::Elem = &mut elem_11;
    let mut bool_2: bool = crate::color::Elem::eq(elem_9_ref_0, elem_7_ref_0);
    let mut bool_3: bool = crate::color::Elem::eq(elem_1_ref_0, elem_10_ref_0);
    let mut bool_4: bool = crate::color::Elem::eq(elem_4_ref_0, elem_6_ref_0);
    let mut bool_5: bool = crate::color::Elem::eq(elem_11_ref_0, elem_5_ref_0);
    let mut bool_6: bool = crate::color::Elem::eq(elem_0_ref_0, elem_2_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_217() {
//    rusty_monitor::set_test_id(217);
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut elem_0: color::Elem = crate::color::Elem::File {exec: bool_1, uid: bool_0};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_1: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut bool_2: bool = true;
    let mut elem_2: color::Elem = crate::color::Elem::Dir {uid: bool_2};
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_3: color::Elem = crate::color::Elem::NonFile;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut elem_4: color::Elem = crate::color::Elem::NonFile;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut elem_5: color::Elem = crate::color::Elem::SymLink;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut elem_6: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut elem_7: color::Elem = crate::color::Elem::HourOld;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut elem_8: color::Elem = crate::color::Elem::Context;
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut elem_9: color::Elem = crate::color::Elem::TreeEdge;
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
#[timeout(30000)]fn rusty_test_4740() {
//    rusty_monitor::set_test_id(4740);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut elem_0: color::Elem = crate::color::Elem::Write;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Write;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut elem_2: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_1_ref_0: &meta::date::Date = &mut date_1;
    let mut elem_3: color::Elem = crate::color::Elem::clone(elem_2_ref_0);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut bool_0: bool = crate::meta::filetype::FileType::is_dirlike(filetype_0);
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::SizeValue;
    let mut bool_1: bool = crate::meta::filetype::FileType::is_dirlike(filetype_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3990() {
//    rusty_monitor::set_test_id(3990);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut option_0: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_0: &str = "less";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_1: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut colors_0: crate::color::Colors = crate::color::Colors {theme: option_1, lscolors: option_0};
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_1, exec: bool_0};
    let mut option_2: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut option_3: std::option::Option<crate::color::theme::Theme> = std::option::Option::Some(theme_0);
    let mut colors_2: crate::color::Colors = crate::color::Colors {theme: option_3, lscolors: option_2};
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut option_4: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut option_5: std::option::Option<crate::color::theme::Theme> = std::option::Option::None;
    let mut colors_3: crate::color::Colors = crate::color::Colors {theme: option_5, lscolors: option_4};
    let mut colors_3_ref_0: &crate::color::Colors = &mut colors_3;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_4: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_4_ref_0: &crate::color::Colors = &mut colors_4;
    let mut filetype_4: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    crate::meta::filetype::FileType::render(filetype_4, colors_4_ref_0);
    crate::meta::filetype::FileType::render(filetype_3, colors_3_ref_0);
    crate::meta::filetype::FileType::render(filetype_2, colors_2_ref_0);
    crate::meta::filetype::FileType::render(filetype_1, colors_1_ref_0);
    crate::meta::filetype::FileType::render(filetype_0, colors_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8612() {
//    rusty_monitor::set_test_id(8612);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut bool_0: bool = true;
    let mut str_0: &str = "Recursion";
    let mut option_0: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_1: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut colors_0: crate::color::Colors = crate::color::Colors {theme: option_1, lscolors: option_0};
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_2, exec: bool_1};
    let mut option_2: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut option_3: std::option::Option<crate::color::theme::Theme> = std::option::Option::Some(theme_0);
    let mut colors_2: crate::color::Colors = crate::color::Colors {theme: option_3, lscolors: option_2};
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut option_4: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut option_5: std::option::Option<crate::color::theme::Theme> = std::option::Option::None;
    let mut colors_3: crate::color::Colors = crate::color::Colors {theme: option_5, lscolors: option_4};
    let mut colors_3_ref_0: &crate::color::Colors = &mut colors_3;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_0};
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_4: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_4_ref_0: &crate::color::Colors = &mut colors_4;
    let mut filetype_4: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    crate::meta::filetype::FileType::render(filetype_4, colors_4_ref_0);
    crate::meta::filetype::FileType::render(filetype_3, colors_3_ref_0);
    crate::meta::filetype::FileType::render(filetype_2, colors_2_ref_0);
    crate::meta::filetype::FileType::render(filetype_1, colors_1_ref_0);
    crate::meta::filetype::FileType::render(filetype_0, colors_0_ref_0);
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Tera;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Name;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6994() {
//    rusty_monitor::set_test_id(6994);
    let mut elem_0: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_1: color::Elem = crate::color::Elem::Socket;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_2: color::Elem = crate::color::Elem::Read;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_3: color::Elem = crate::color::Elem::Exec;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut elem_4: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut elem_5: color::Elem = crate::color::Elem::Older;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut elem_6: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut elem_7: color::Elem = crate::color::Elem::Exec;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut elem_8: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut elem_9: color::Elem = crate::color::Elem::Older;
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    let mut elem_10: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_10_ref_0: &color::Elem = &mut elem_10;
    let mut elem_11: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_11_ref_0: &color::Elem = &mut elem_11;
    let mut elem_12: color::Elem = crate::color::Elem::DayOld;
    let mut elem_12_ref_0: &color::Elem = &mut elem_12;
    let mut bool_0: bool = crate::color::Elem::ne(elem_11_ref_0, elem_10_ref_0);
    let mut bool_1: bool = crate::color::Elem::ne(elem_9_ref_0, elem_8_ref_0);
    let mut bool_2: bool = crate::color::Elem::ne(elem_7_ref_0, elem_6_ref_0);
    let mut bool_3: bool = crate::color::Elem::ne(elem_5_ref_0, elem_4_ref_0);
    let mut bool_4: bool = crate::color::Elem::ne(elem_3_ref_0, elem_2_ref_0);
    let mut bool_5: bool = crate::color::Elem::ne(elem_1_ref_0, elem_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5073() {
//    rusty_monitor::set_test_id(5073);
    let mut elem_0: color::Elem = crate::color::Elem::DayOld;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_1: color::Elem = crate::color::Elem::Write;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_2: color::Elem = crate::color::Elem::Write;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_1_ref_0);
    let mut elem_3: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut str_0: &str = "Natural sort of (version) numbers within text";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_1_ref_0: &meta::date::Date = &mut date_1;
    let mut date_2: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut option_0: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut elem_4: color::Elem = crate::color::Elem::ExecSticky;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut tuple_0: () = crate::color::Elem::assert_receiver_is_total_eq(elem_4_ref_0);
    let mut elem_5: color::Elem = crate::color::Elem::clone(elem_0_ref_0);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut bool_0: bool = crate::meta::filetype::FileType::is_dirlike(filetype_0);
    let mut inode_0: crate::color::theme::INode = crate::color::theme::INode {valid: color_1, invalid: color_0};
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::SizeValue;
    let mut bool_1: bool = crate::meta::filetype::FileType::is_dirlike(filetype_1);
    let mut elem_6: color::Elem = crate::color::Elem::MissingSymLinkTarget;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5291() {
//    rusty_monitor::set_test_id(5291);
    let mut str_0: &str = "ExecSticky";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut elem_0: color::Elem = crate::color::Elem::Write;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut tuple_0: () = crate::color::Elem::assert_receiver_is_total_eq(elem_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5999() {
//    rusty_monitor::set_test_id(5999);
    let mut option_0: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_0: &str = "less";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_1: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut colors_0: crate::color::Colors = crate::color::Colors {theme: option_1, lscolors: option_0};
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_1, exec: bool_0};
    let mut option_2: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut option_3: std::option::Option<crate::color::theme::Theme> = std::option::Option::Some(theme_0);
    let mut colors_2: crate::color::Colors = crate::color::Colors {theme: option_3, lscolors: option_2};
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut option_4: std::option::Option<crate::color::theme::Theme> = std::option::Option::None;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    crate::meta::filetype::FileType::render(filetype_1, colors_2_ref_0);
    crate::meta::filetype::FileType::render(filetype_0, colors_1_ref_0);
    let mut elem_0: color::Elem = crate::color::Elem::Context;
    let mut bool_2: bool = crate::meta::filetype::FileType::is_dirlike(filetype_3);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8882() {
//    rusty_monitor::set_test_id(8882);
    let mut elem_0: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut option_0: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut option_1: std::option::Option<bool> = std::option::Option::None;
    let mut option_2: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_3: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut option_4: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut option_5: std::option::Option<bool> = std::option::Option::None;
    let mut option_6: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_7: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut option_8: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_8, theme: option_7};
    let mut option_9: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_10: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_0: bool = false;
    let mut option_11: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_0: usize = 0usize;
    let mut bool_1: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_1, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut display_1: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut str_0: &str = "Name";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_0};
    let mut u64_0: u64 = 1024u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut elem_1: color::Elem = crate::color::Elem::Read;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut u64_1: u64 = 1099511627776u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut str_1: &str = "cshtml";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut u64_2: u64 = 1073741824u64;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size::new(u64_2);
    let mut size_2_ref_0: &crate::meta::size::Size = &mut size_2;
    let mut bool_2: bool = true;
    let mut bool_3: bool = false;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_3, exec: bool_2};
    let mut option_12: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut bool_4: bool = crate::meta::filetype::FileType::is_dirlike(filetype_2);
    crate::meta::filetype::FileType::render(filetype_1, colors_0_ref_0);
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut elem_2: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut bool_5: bool = crate::color::Elem::eq(elem_2_ref_0, elem_1_ref_0);
    let mut display_2: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut elem_3: color::Elem = crate::color::Elem::Read;
    let mut elem_4: color::Elem = crate::color::Elem::clone(elem_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_9094() {
//    rusty_monitor::set_test_id(9094);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Write;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Write;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut elem_2: color::Elem = crate::color::Elem::Older;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut str_0: &str = "Natural sort of (version) numbers within text";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_1_ref_0: &meta::date::Date = &mut date_1;
    let mut option_0: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut elem_3: color::Elem = crate::color::Elem::ExecSticky;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut tuple_0: () = crate::color::Elem::assert_receiver_is_total_eq(elem_3_ref_0);
    let mut elem_4: color::Elem = crate::color::Elem::clone(elem_2_ref_0);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut bool_0: bool = crate::meta::filetype::FileType::is_dirlike(filetype_0);
    let mut inode_0: crate::color::theme::INode = crate::color::theme::INode {valid: color_1, invalid: color_0};
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Context;
    let mut bool_1: bool = crate::meta::filetype::FileType::is_dirlike(filetype_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5791() {
//    rusty_monitor::set_test_id(5791);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Write;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Write;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut elem_2: color::Elem = crate::color::Elem::Pipe;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut str_0: &str = "Natural sort of (version) numbers within text";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_1_ref_0: &meta::date::Date = &mut date_1;
    let mut option_0: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut elem_3: color::Elem = crate::color::Elem::Acl;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut tuple_0: () = crate::color::Elem::assert_receiver_is_total_eq(elem_3_ref_0);
    let mut elem_4: color::Elem = crate::color::Elem::clone(elem_2_ref_0);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut bool_0: bool = crate::meta::filetype::FileType::is_dirlike(filetype_0);
    let mut inode_0: crate::color::theme::INode = crate::color::theme::INode {valid: color_1, invalid: color_0};
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::SizeValue;
    let mut bool_1: bool = crate::meta::filetype::FileType::is_dirlike(filetype_1);
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6355() {
//    rusty_monitor::set_test_id(6355);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut option_0: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_0: &str = "less";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_1: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut colors_0: crate::color::Colors = crate::color::Colors {theme: option_1, lscolors: option_0};
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_1, exec: bool_0};
    let mut option_2: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut option_3: std::option::Option<crate::color::theme::Theme> = std::option::Option::Some(theme_0);
    let mut colors_2: crate::color::Colors = crate::color::Colors {theme: option_3, lscolors: option_2};
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut option_4: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut option_5: std::option::Option<crate::color::theme::Theme> = std::option::Option::None;
    let mut colors_3: crate::color::Colors = crate::color::Colors {theme: option_5, lscolors: option_4};
    let mut colors_3_ref_0: &crate::color::Colors = &mut colors_3;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_4: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_4_ref_0: &crate::color::Colors = &mut colors_4;
    let mut filetype_4: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    crate::meta::filetype::FileType::render(filetype_4, colors_4_ref_0);
    crate::meta::filetype::FileType::render(filetype_3, colors_3_ref_0);
    crate::meta::filetype::FileType::render(filetype_2, colors_2_ref_0);
    crate::meta::filetype::FileType::render(filetype_1, colors_1_ref_0);
    crate::meta::filetype::FileType::render(filetype_0, colors_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3846() {
//    rusty_monitor::set_test_id(3846);
    let mut elem_0: color::Elem = crate::color::Elem::Acl;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_1: color::Elem = crate::color::Elem::Group;
    let mut bool_0: bool = true;
    let mut elem_2: color::Elem = crate::color::Elem::Dir {uid: bool_0};
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_3: color::Elem = crate::color::Elem::Write;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_4: color::Elem = crate::color::Elem::Write;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_1_ref_0);
    let mut elem_5: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut str_0: &str = "Natural sort of (version) numbers within text";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_1_ref_0: &meta::date::Date = &mut date_1;
    let mut option_0: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut elem_6: color::Elem = crate::color::Elem::ExecSticky;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut tuple_0: () = crate::color::Elem::assert_receiver_is_total_eq(elem_1_ref_0);
    let mut elem_7: color::Elem = crate::color::Elem::clone(elem_5_ref_0);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut bool_1: bool = crate::meta::filetype::FileType::is_dirlike(filetype_0);
    let mut inode_0: crate::color::theme::INode = crate::color::theme::INode {valid: color_1, invalid: color_0};
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::SizeValue;
    let mut bool_2: bool = crate::meta::filetype::FileType::is_dirlike(filetype_1);
    let mut tuple_1: () = crate::color::Elem::assert_receiver_is_total_eq(elem_0_ref_0);
//    panic!("From RustyUnit with love");
}
}