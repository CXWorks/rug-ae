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
#[timeout(30000)]fn rusty_test_2895() {
//    rusty_monitor::set_test_id(2895);
    let mut elem_0: color::Elem = crate::color::Elem::Socket;
    let mut elem_1: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_2: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_3: color::Elem = crate::color::Elem::Octal;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut elem_4: color::Elem = crate::color::Elem::Older;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut elem_5: color::Elem = crate::color::Elem::Group;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut elem_6: color::Elem = crate::color::Elem::Special;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut elem_7: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut elem_8: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut elem_9: color::Elem = crate::color::Elem::Group;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut bool_0: bool = false;
    let mut elem_10: color::Elem = crate::color::Elem::Links {valid: bool_0};
    let mut elem_10_ref_0: &color::Elem = &mut elem_10;
    let mut elem_11: color::Elem = crate::color::Elem::Octal;
    let mut elem_11_ref_0: &color::Elem = &mut elem_11;
    let mut elem_12: color::Elem = crate::color::Elem::Socket;
    let mut elem_12_ref_0: &color::Elem = &mut elem_12;
    let mut bool_1: bool = crate::color::Elem::eq(elem_12_ref_0, elem_11_ref_0);
    let mut bool_2: bool = crate::color::Elem::eq(elem_10_ref_0, elem_0_ref_0);
    let mut bool_3: bool = crate::color::Elem::eq(elem_8_ref_0, elem_7_ref_0);
    let mut bool_4: bool = crate::color::Elem::eq(elem_6_ref_0, elem_5_ref_0);
    let mut bool_5: bool = crate::color::Elem::eq(elem_4_ref_0, elem_3_ref_0);
    let mut bool_6: bool = crate::color::Elem::eq(elem_2_ref_0, elem_1_ref_0);
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    let mut elem_13: color::Elem = crate::color::Elem::clone(elem_9_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2820() {
//    rusty_monitor::set_test_id(2820);
    let mut bool_0: bool = false;
    let mut elem_0: color::Elem = crate::color::Elem::Dir {uid: bool_0};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut bool_1: bool = false;
    let mut bool_2: bool = false;
    let mut elem_1: color::Elem = crate::color::Elem::Links {valid: bool_2};
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut elem_2: color::Elem = crate::color::Elem::Octal;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_3: color::Elem = crate::color::Elem::Older;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_4: color::Elem = crate::color::Elem::Group;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut elem_5: color::Elem = crate::color::Elem::Special;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut elem_6: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut elem_7: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut elem_8: color::Elem = crate::color::Elem::Group;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut elem_9: color::Elem = crate::color::Elem::Links {valid: bool_1};
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut elem_10: color::Elem = crate::color::Elem::Octal;
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    let mut elem_11: color::Elem = crate::color::Elem::Socket;
    let mut elem_10_ref_0: &color::Elem = &mut elem_10;
    let mut bool_3: bool = crate::color::Elem::eq(elem_9_ref_0, elem_1_ref_0);
    let mut bool_4: bool = crate::color::Elem::eq(elem_2_ref_0, elem_4_ref_0);
    let mut bool_5: bool = crate::color::Elem::eq(elem_8_ref_0, elem_6_ref_0);
    let mut bool_6: bool = crate::color::Elem::eq(elem_7_ref_0, elem_10_ref_0);
    let mut bool_7: bool = crate::color::Elem::eq(elem_5_ref_0, elem_3_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3600() {
//    rusty_monitor::set_test_id(3600);
    let mut option_0: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_0: &str = "hidden";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_1: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut colors_0: crate::color::Colors = crate::color::Colors {theme: option_1, lscolors: option_0};
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut option_2: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut option_3: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut bool_0: bool = true;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_0};
    crate::meta::filetype::FileType::render(filetype_2, colors_2_ref_0);
    crate::meta::filetype::FileType::render(filetype_1, colors_1_ref_0);
    crate::meta::filetype::FileType::render(filetype_0, colors_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2139() {
//    rusty_monitor::set_test_id(2139);
    let mut bool_0: bool = true;
    let mut str_0: &str = "BlockDevice";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "bin";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bool_1: bool = true;
    let mut str_2: &str = "NoLscolors";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bool_2: bool = true;
    let mut str_3: &str = "L7yuIigRR0R";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut bool_3: bool = true;
    let mut str_4: &str = "";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut bool_4: bool = true;
    let mut str_5: &str = "b";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut bool_5: bool = true;
    let mut str_6: &str = "";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut bool_6: bool = false;
    let mut str_7: &str = "BlockDevice";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut elem_0: color::Elem = crate::color::Elem::Special;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut tuple_0: () = crate::color::Elem::assert_receiver_is_total_eq(elem_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7534() {
//    rusty_monitor::set_test_id(7534);
    let mut elem_0: color::Elem = crate::color::Elem::Write;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut option_0: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut option_1: std::option::Option<crate::color::theme::Theme> = std::option::Option::Some(theme_0);
    let mut colors_0: crate::color::Colors = crate::color::Colors {theme: option_1, lscolors: option_0};
    let mut elem_1: color::Elem = crate::color::Elem::Pipe;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut bool_3: bool = false;
    let mut bool_4: bool = false;
    let mut bool_5: bool = true;
    let mut bool_6: bool = false;
    let mut bool_7: bool = true;
    let mut bool_8: bool = false;
    let mut bool_9: bool = true;
    let mut bool_10: bool = true;
    let mut bool_11: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut elem_2: color::Elem = crate::color::Elem::FileMedium;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut iconoption_1_ref_0: &flags::icons::IconOption = &mut iconoption_1;
    let mut bool_12: bool = crate::meta::permissions::Permissions::is_executable(permissions_0_ref_0);
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut bool_13: bool = crate::color::Elem::ne(elem_2_ref_0, elem_1_ref_0);
    let mut elem_3: color::Elem = crate::color::Elem::clone(elem_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1493() {
//    rusty_monitor::set_test_id(1493);
    let mut bool_0: bool = false;
    let mut str_0: &str = "user";
    let mut option_0: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_1: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut colors_0: crate::color::Colors = crate::color::Colors {theme: option_1, lscolors: option_0};
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut bool_5: bool = true;
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut bool_8: bool = false;
    let mut bool_9: bool = true;
    let mut bool_10: bool = true;
    let mut bool_11: bool = false;
    let mut bool_12: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_12, user_write: bool_11, user_execute: bool_10, group_read: bool_9, group_write: bool_8, group_execute: bool_7, other_read: bool_6, other_write: bool_5, other_execute: bool_4, sticky: bool_3, setgid: bool_2, setuid: bool_1};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_2: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut option_3: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_1: &str = "ohme";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut option_4: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_1_ref_0);
    let mut colors_2: crate::color::Colors = crate::color::Colors {theme: option_4, lscolors: option_3};
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_0, exec: bool_0};
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_3: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_3_ref_0: &crate::color::Colors = &mut colors_3;
    let mut bool_13: bool = true;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_13};
    crate::meta::filetype::FileType::render(filetype_3, colors_3_ref_0);
    crate::meta::filetype::FileType::render(filetype_2, colors_2_ref_0);
    crate::meta::filetype::FileType::render(filetype_1, colors_1_ref_0);
    crate::meta::filetype::FileType::render(filetype_0, colors_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1520() {
//    rusty_monitor::set_test_id(1520);
    let mut elem_0: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Links;
    let mut block_0_ref_0: &flags::blocks::Block = &mut block_0;
    let mut block_1: flags::blocks::Block = crate::flags::blocks::Block::SizeValue;
    let mut block_1_ref_0: &flags::blocks::Block = &mut block_1;
    let mut elem_1: color::Elem = crate::color::Elem::File {exec: bool_1, uid: bool_0};
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut bool_2: bool = crate::color::Elem::ne(elem_1_ref_0, elem_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1765() {
//    rusty_monitor::set_test_id(1765);
    let mut elem_0: color::Elem = crate::color::Elem::Read;
    let mut elem_1: color::Elem = crate::color::Elem::Octal;
    let mut str_0: &str = "py";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "wma";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut usize_0: usize = 73usize;
    let mut tuple_0: (usize, &str) = (usize_0, str_1_ref_0);
    let mut bool_0: bool = true;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_2: color::Elem = crate::color::Elem::Group;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_3: color::Elem = crate::color::Elem::Links {valid: bool_0};
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_4: color::Elem = crate::color::Elem::Octal;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut elem_5: color::Elem = crate::color::Elem::Socket;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut bool_1: bool = crate::color::Elem::eq(elem_0_ref_0, elem_3_ref_0);
    let mut bool_2: bool = crate::color::Elem::eq(elem_2_ref_0, elem_4_ref_0);
    let mut elem_6: color::Elem = crate::color::Elem::FileMedium;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4475() {
//    rusty_monitor::set_test_id(4475);
    let mut bool_0: bool = true;
    let mut elem_0: color::Elem = crate::color::Elem::Dir {uid: bool_0};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut option_0: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_0: &str = "hidden";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_1: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut colors_0: crate::color::Colors = crate::color::Colors {theme: option_1, lscolors: option_0};
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut bool_5: bool = true;
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut bool_8: bool = false;
    let mut bool_9: bool = true;
    let mut bool_10: bool = true;
    let mut bool_11: bool = false;
    let mut bool_12: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_12, user_write: bool_11, user_execute: bool_10, group_read: bool_9, group_write: bool_8, group_execute: bool_7, other_read: bool_6, other_write: bool_5, other_execute: bool_4, sticky: bool_3, setgid: bool_2, setuid: bool_1};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_2: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut option_3: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_1: &str = "ohme";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut option_4: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_1_ref_0);
    let mut colors_2: crate::color::Colors = crate::color::Colors {theme: option_4, lscolors: option_3};
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut bool_13: bool = true;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_13};
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_3: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_3_ref_0: &crate::color::Colors = &mut colors_3;
    let mut bool_14: bool = true;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_14};
    crate::meta::filetype::FileType::render(filetype_3, colors_3_ref_0);
    crate::meta::filetype::FileType::render(filetype_2, colors_2_ref_0);
    crate::meta::filetype::FileType::render(filetype_1, colors_1_ref_0);
    crate::meta::filetype::FileType::render(filetype_0, colors_0_ref_0);
    let mut elem_1: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut bool_15: bool = crate::color::Elem::eq(elem_1_ref_0, elem_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_294() {
//    rusty_monitor::set_test_id(294);
    let mut elem_0: color::Elem = crate::color::Elem::ExecSticky;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_1: color::Elem = crate::color::Elem::DayOld;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_2: color::Elem = crate::color::Elem::Special;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_3: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut elem_4: color::Elem = crate::color::Elem::Context;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut elem_5: color::Elem = crate::color::Elem::Octal;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut elem_6: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut elem_7: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut elem_8: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut elem_9: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    let mut elem_10: color::Elem = crate::color::Elem::Exec;
    let mut elem_10_ref_0: &color::Elem = &mut elem_10;
    let mut elem_11: color::Elem = crate::color::Elem::clone(elem_10_ref_0);
    let mut elem_12: color::Elem = crate::color::Elem::clone(elem_9_ref_0);
    let mut elem_13: color::Elem = crate::color::Elem::clone(elem_8_ref_0);
    let mut elem_14: color::Elem = crate::color::Elem::clone(elem_7_ref_0);
    let mut elem_15: color::Elem = crate::color::Elem::clone(elem_6_ref_0);
    let mut elem_16: color::Elem = crate::color::Elem::clone(elem_5_ref_0);
    let mut elem_17: color::Elem = crate::color::Elem::clone(elem_4_ref_0);
    let mut elem_18: color::Elem = crate::color::Elem::clone(elem_3_ref_0);
    let mut elem_19: color::Elem = crate::color::Elem::clone(elem_2_ref_0);
    let mut elem_20: color::Elem = crate::color::Elem::clone(elem_1_ref_0);
    let mut elem_21: color::Elem = crate::color::Elem::clone(elem_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6442() {
//    rusty_monitor::set_test_id(6442);
    let mut str_0: &str = "failed to convert symlink to str";
    let mut bool_0: bool = true;
    let mut elem_0: color::Elem = crate::color::Elem::Dir {uid: bool_0};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut option_0: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_1: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut colors_0: crate::color::Colors = crate::color::Colors {theme: option_1, lscolors: option_0};
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut bool_5: bool = true;
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut bool_8: bool = false;
    let mut bool_9: bool = true;
    let mut bool_10: bool = true;
    let mut bool_11: bool = false;
    let mut bool_12: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_12, user_write: bool_11, user_execute: bool_10, group_read: bool_9, group_write: bool_8, group_execute: bool_7, other_read: bool_6, other_write: bool_5, other_execute: bool_4, sticky: bool_3, setgid: bool_2, setuid: bool_1};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_2: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut option_3: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_1: &str = "ohme";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut option_4: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_1_ref_0);
    let mut colors_2: crate::color::Colors = crate::color::Colors {theme: option_4, lscolors: option_3};
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut bool_13: bool = true;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_13};
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_3: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_3_ref_0: &crate::color::Colors = &mut colors_3;
    let mut bool_14: bool = true;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_14};
    crate::meta::filetype::FileType::render(filetype_3, colors_3_ref_0);
    crate::meta::filetype::FileType::render(filetype_2, colors_2_ref_0);
    crate::meta::filetype::FileType::render(filetype_1, colors_1_ref_0);
    crate::meta::filetype::FileType::render(filetype_0, colors_0_ref_0);
    let mut elem_1: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut bool_15: bool = crate::color::Elem::eq(elem_1_ref_0, elem_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2726() {
//    rusty_monitor::set_test_id(2726);
    let mut option_0: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_0: &str = "hidden";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_1: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut colors_0: crate::color::Colors = crate::color::Colors {theme: option_1, lscolors: option_0};
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut option_2: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut option_3: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_1: &str = "ohme";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut option_4: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_1_ref_0);
    let mut colors_2: crate::color::Colors = crate::color::Colors {theme: option_4, lscolors: option_3};
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut bool_0: bool = true;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_0};
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_3: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_3_ref_0: &crate::color::Colors = &mut colors_3;
    let mut bool_1: bool = true;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_1};
    crate::meta::filetype::FileType::render(filetype_3, colors_3_ref_0);
    crate::meta::filetype::FileType::render(filetype_2, colors_2_ref_0);
    crate::meta::filetype::FileType::render(filetype_1, colors_1_ref_0);
    crate::meta::filetype::FileType::render(filetype_0, colors_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2747() {
//    rusty_monitor::set_test_id(2747);
    let mut bool_0: bool = false;
    let mut option_0: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_0: &str = "hidden";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_1: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut colors_0: crate::color::Colors = crate::color::Colors {theme: option_1, lscolors: option_0};
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = false;
    let mut bool_5: bool = true;
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut bool_8: bool = false;
    let mut bool_9: bool = true;
    let mut bool_10: bool = true;
    let mut bool_11: bool = false;
    let mut bool_12: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_12, user_write: bool_11, user_execute: bool_10, group_read: bool_9, group_write: bool_8, group_execute: bool_7, other_read: bool_6, other_write: bool_5, other_execute: bool_4, sticky: bool_3, setgid: bool_2, setuid: bool_1};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_2: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_0, exec: bool_0};
    let mut option_3: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_1: &str = "ohme";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut option_4: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_1_ref_0);
    let mut colors_2: crate::color::Colors = crate::color::Colors {theme: option_4, lscolors: option_3};
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut bool_13: bool = true;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_13};
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_3: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_3_ref_0: &crate::color::Colors = &mut colors_3;
    let mut bool_14: bool = true;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_14};
    crate::meta::filetype::FileType::render(filetype_3, colors_3_ref_0);
    crate::meta::filetype::FileType::render(filetype_2, colors_2_ref_0);
    crate::meta::filetype::FileType::render(filetype_1, colors_1_ref_0);
    crate::meta::filetype::FileType::render(filetype_0, colors_0_ref_0);
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1402() {
//    rusty_monitor::set_test_id(1402);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut elem_0: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut bool_3: bool = false;
    let mut bool_4: bool = false;
    let mut bool_5: bool = true;
    let mut bool_6: bool = false;
    let mut bool_7: bool = true;
    let mut bool_8: bool = false;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut sortcolumn_0_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_0;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut sortcolumn_1_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_1;
    let mut elem_1: color::Elem = crate::color::Elem::FileMedium;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut iconoption_1_ref_0: &flags::icons::IconOption = &mut iconoption_1;
    let mut bool_12: bool = crate::meta::permissions::Permissions::is_executable(permissions_0_ref_0);
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut bool_13: bool = crate::color::Elem::ne(elem_1_ref_0, elem_0_ref_0);
    let mut bool_14: bool = crate::meta::filetype::FileType::is_dirlike(filetype_0);
//    panic!("From RustyUnit with love");
}
}