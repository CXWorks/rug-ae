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
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3102() {
    rusty_monitor::set_test_id(3102);
    let mut elem_0: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_1: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_2: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut option_0: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_0: &str = "X9HI23ObtwB";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_1: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut u64_0: u64 = 7561u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut option_2: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut option_3: std::option::Option<crate::color::theme::Theme> = std::option::Option::Some(theme_0);
    let mut colors_0: crate::color::Colors = crate::color::Colors {theme: option_3, lscolors: option_2};
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut bool_2: bool = false;
    let mut bool_3: bool = false;
    let mut bool_4: bool = false;
    let mut bool_5: bool = false;
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut bool_8: bool = false;
    let mut bool_9: bool = false;
    let mut bool_10: bool = false;
    let mut bool_11: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut bool_12: bool = true;
    let mut elem_3: color::Elem = crate::color::Elem::Dir {uid: bool_12};
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_4: color::Elem = crate::color::Elem::CharDevice;
    let mut layout_0_ref_0: &flags::layout::Layout = &mut layout_0;
    let mut elem_5: color::Elem = crate::color::Elem::NonFile;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut bool_13: bool = crate::color::Elem::has_suid(elem_0_ref_0);
    let mut contentstyle_0: crossterm::style::ContentStyle = crate::color::Colors::style(colors_0_ref_0, elem_2_ref_0);
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut elem_6: color::Elem = crate::color::Elem::clone(elem_4_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1729() {
    rusty_monitor::set_test_id(1729);
    let mut option_0: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_0: &str = "Spx8RgnzyxSth";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_1: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut colors_0: crate::color::Colors = crate::color::Colors {theme: option_1, lscolors: option_0};
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut elem_0: color::Elem = crate::color::Elem::Write;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut option_2: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_1: &str = "UArmSBj";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut option_3: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_1_ref_0);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_1: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_0_ref_0);
    let mut bool_0: bool = false;
    let mut elem_2: color::Elem = crate::color::Elem::Dir {uid: bool_0};
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_3: color::Elem = crate::color::Elem::FileSmall;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut u64_0: u64 = 7317u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut u64_1: u64 = 5684u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut str_2: &str = "M15PWEBaeH";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut layout_0_ref_0: &flags::layout::Layout = &mut layout_0;
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut layout_1_ref_0: &flags::layout::Layout = &mut layout_1;
    let mut elem_4: color::Elem = crate::color::Elem::NonFile;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut bool_1: bool = crate::color::Elem::has_suid(elem_2_ref_0);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    crate::meta::filetype::FileType::render(filetype_0, colors_0_ref_0);
    let mut elem_5: color::Elem = crate::color::Elem::SymLink;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7726() {
    rusty_monitor::set_test_id(7726);
    let mut str_0: &str = "mtOhvb8";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u64_0: u64 = 3917u64;
    let mut str_1: &str = "24VzMtiXUyN";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut option_0: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_0: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_0};
    let mut str_2: &str = "T9vI1O0";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut option_1: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut option_2: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_3: std::option::Option<bool> = std::option::Option::None;
    let mut option_4: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_5: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_6: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_7: std::option::Option<bool> = std::option::Option::None;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::TreeEdge;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::SymLink;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut elem_2: color::Elem = crate::color::Elem::Write;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut option_8: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_3: &str = "UArmSBj";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut option_9: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_3_ref_0);
    let mut colors_1: crate::color::Colors = crate::color::Colors {theme: option_9, lscolors: option_8};
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut bool_1: bool = false;
    let mut elem_3: color::Elem = crate::color::Elem::Dir {uid: bool_1};
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut elem_4: color::Elem = crate::color::Elem::FileSmall;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_1_ref_0: &meta::date::Date = &mut date_1;
    let mut u64_1: u64 = 5684u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut str_4: &str = "M15PWEBaeH";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_3: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut layout_0_ref_0: &flags::layout::Layout = &mut layout_0;
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut layout_1_ref_0: &flags::layout::Layout = &mut layout_1;
    let mut elem_5: color::Elem = crate::color::Elem::Pipe;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut bool_2: bool = crate::color::Elem::has_suid(elem_3_ref_0);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut bool_3: bool = crate::meta::filetype::FileType::is_dirlike(filetype_2);
    let mut filetype_4: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    crate::meta::filetype::FileType::render(filetype_4, colors_0_ref_0);
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut elem_6: color::Elem = crate::color::Elem::clone(elem_5_ref_0);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::User;
    let mut result_0: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7941() {
    rusty_monitor::set_test_id(7941);
    let mut str_0: &str = "mtOhvb8";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u64_0: u64 = 3917u64;
    let mut str_1: &str = "24VzMtiXUyN";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut option_0: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_0: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_0};
    let mut str_2: &str = "T9vI1O0";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut option_1: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut option_2: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_3: std::option::Option<bool> = std::option::Option::None;
    let mut option_4: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_5: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_6: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_7: std::option::Option<bool> = std::option::Option::None;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::TreeEdge;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::SymLink;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut option_8: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_3: &str = "UArmSBj";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut option_9: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_3_ref_0);
    let mut colors_1: crate::color::Colors = crate::color::Colors {theme: option_9, lscolors: option_8};
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut bool_1: bool = false;
    let mut elem_3: color::Elem = crate::color::Elem::Dir {uid: bool_1};
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut elem_4: color::Elem = crate::color::Elem::FileSmall;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_1_ref_0: &meta::date::Date = &mut date_1;
    let mut u64_1: u64 = 5684u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut str_4: &str = "M15PWEBaeH";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_3: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut layout_0_ref_0: &flags::layout::Layout = &mut layout_0;
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut layout_1_ref_0: &flags::layout::Layout = &mut layout_1;
    let mut elem_5: color::Elem = crate::color::Elem::DayOld;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut bool_2: bool = crate::color::Elem::has_suid(elem_3_ref_0);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut contentstyle_0: crossterm::style::ContentStyle = crate::color::Colors::style(colors_1_ref_0, elem_2_ref_0);
    let mut bool_3: bool = crate::meta::filetype::FileType::is_dirlike(filetype_2);
    let mut filetype_4: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    crate::meta::filetype::FileType::render(filetype_4, colors_0_ref_0);
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut elem_6: color::Elem = crate::color::Elem::clone(elem_5_ref_0);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::User;
    let mut result_0: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4734() {
    rusty_monitor::set_test_id(4734);
    let mut elem_0: color::Elem = crate::color::Elem::ExecSticky;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_1: color::Elem = crate::color::Elem::HourOld;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut option_0: std::option::Option<usize> = std::option::Option::None;
    let mut bool_0: bool = false;
    let mut option_1: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_1, depth: option_0};
    let mut option_2: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_0);
    let mut option_3: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_1: bool = true;
    let mut option_4: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_5: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_6: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut option_7: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_0);
    let mut option_8: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_8, theme: option_7, separator: option_6};
    let mut option_9: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut option_10: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_11: std::option::Option<bool> = std::option::Option::None;
    let mut option_12: std::option::Option<std::string::String> = std::option::Option::None;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut option_13: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_0);
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut option_14: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_14, theme: option_13};
    let mut option_15: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_16: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_17: std::option::Option<bool> = std::option::Option::None;
    let mut u64_0: u64 = 6281u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut bool_2: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_2};
    let mut elem_2: color::Elem = crate::color::Elem::SymLink;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_3: color::Elem = crate::color::Elem::SymLink;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut option_18: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_19: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut option_20: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_1);
    let mut option_21: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut icons_1: crate::config_file::Icons = crate::config_file::Icons {when: option_21, theme: option_20, separator: option_19};
    let mut option_22: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_1);
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut option_23: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut option_24: std::option::Option<bool> = std::option::Option::None;
    let mut option_25: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_26: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_27: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_28: std::option::Option<bool> = std::option::Option::None;
    let mut u64_1: u64 = 62u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut option_29: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_30: std::option::Option<std::string::String> = std::option::Option::None;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut layout_0_ref_0: &flags::layout::Layout = &mut layout_0;
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut layout_1_ref_0: &flags::layout::Layout = &mut layout_1;
    let mut elem_4: color::Elem = crate::color::Elem::NonFile;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut bool_3: bool = crate::color::Elem::has_suid(elem_2_ref_0);
    let mut bool_3: bool = crate::color::Elem::has_suid(elem_3_ref_0);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut contentstyle_0: crossterm::style::ContentStyle = crate::color::Colors::style(colors_0_ref_0, elem_1_ref_0);
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut elem_5: color::Elem = crate::color::Elem::clone(elem_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_759() {
    rusty_monitor::set_test_id(759);
    let mut elem_0: color::Elem = crate::color::Elem::Context;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut bool_0: bool = true;
    let mut elem_1: color::Elem = crate::color::Elem::Dir {uid: bool_0};
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_2: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_1_ref_0);
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut elem_3: color::Elem = crate::color::Elem::Socket;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut bool_3: bool = false;
    let mut elem_4: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_5: color::Elem = crate::color::Elem::Links {valid: bool_3};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_2_ref_0);
    let mut elem_6: color::Elem = crate::color::Elem::Pipe;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut bool_4: bool = crate::color::Elem::ne(elem_6_ref_0, elem_3_ref_0);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_2, exec: bool_1};
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut links_0: crate::color::theme::Links = crate::color::theme::Links {valid: color_1, invalid: color_0};
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Date;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6975() {
    rusty_monitor::set_test_id(6975);
    let mut str_0: &str = "mtOhvb8";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u64_0: u64 = 3917u64;
    let mut str_1: &str = "24VzMtiXUyN";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut option_0: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_0: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_0};
    let mut str_2: &str = "T9vI1O0";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut option_1: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut option_2: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_3: std::option::Option<bool> = std::option::Option::None;
    let mut option_4: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_5: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_6: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_7: std::option::Option<bool> = std::option::Option::None;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::TreeEdge;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::SymLink;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::Write;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut option_8: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_3: &str = "UArmSBj";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut option_9: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_1_ref_0);
    let mut colors_1: crate::color::Colors = crate::color::Colors {theme: option_9, lscolors: option_8};
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut bool_1: bool = false;
    let mut elem_3: color::Elem = crate::color::Elem::Dir {uid: bool_1};
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut elem_4: color::Elem = crate::color::Elem::FileSmall;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_1_ref_0: &meta::date::Date = &mut date_1;
    let mut u64_1: u64 = 5684u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut str_4: &str = "M15PWEBaeH";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_3: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut layout_0_ref_0: &flags::layout::Layout = &mut layout_0;
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut layout_1_ref_0: &flags::layout::Layout = &mut layout_1;
    let mut elem_5: color::Elem = crate::color::Elem::NonFile;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut bool_2: bool = crate::color::Elem::has_suid(elem_3_ref_0);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut contentstyle_0: crossterm::style::ContentStyle = crate::color::Colors::style(colors_1_ref_0, elem_2_ref_0);
    let mut bool_3: bool = crate::meta::filetype::FileType::is_dirlike(filetype_2);
    let mut filetype_4: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    crate::meta::filetype::FileType::render(filetype_4, colors_0_ref_0);
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut elem_6: color::Elem = crate::color::Elem::clone(elem_5_ref_0);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::User;
    let mut result_0: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2743() {
    rusty_monitor::set_test_id(2743);
    let mut elem_0: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_1: color::Elem = crate::color::Elem::Older;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_2: color::Elem = crate::color::Elem::ExecSticky;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_3: color::Elem = crate::color::Elem::Acl;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut elem_4: color::Elem = crate::color::Elem::SymLink;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut elem_5: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut bool_0: bool = true;
    let mut elem_6: color::Elem = crate::color::Elem::Links {valid: bool_0};
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut elem_7: color::Elem = crate::color::Elem::Group;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut elem_8: color::Elem = crate::color::Elem::Exec;
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut elem_9: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    let mut elem_10: color::Elem = crate::color::Elem::SymLink;
    let mut elem_10_ref_0: &color::Elem = &mut elem_10;
    let mut elem_11: color::Elem = crate::color::Elem::Read;
    let mut elem_11_ref_0: &color::Elem = &mut elem_11;
    let mut elem_12: color::Elem = crate::color::Elem::Older;
    let mut elem_12_ref_0: &color::Elem = &mut elem_12;
    let mut elem_13: color::Elem = crate::color::Elem::Write;
    let mut elem_13_ref_0: &color::Elem = &mut elem_13;
    let mut elem_14: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_14_ref_0: &color::Elem = &mut elem_14;
    let mut elem_15: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_15_ref_0: &color::Elem = &mut elem_15;
    let mut elem_16: color::Elem = crate::color::Elem::Acl;
    let mut elem_16_ref_0: &color::Elem = &mut elem_16;
    let mut elem_17: color::Elem = crate::color::Elem::DayOld;
    let mut elem_18: color::Elem = crate::color::Elem::clone(elem_11_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1344() {
    rusty_monitor::set_test_id(1344);
    let mut bool_0: bool = true;
    let mut elem_0: color::Elem = crate::color::Elem::Dir {uid: bool_0};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_1: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_0_ref_0);
    let mut bool_1: bool = false;
    let mut bool_2: bool = false;
    let mut elem_2: color::Elem = crate::color::Elem::Socket;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut bool_3: bool = false;
    let mut elem_3: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_4: color::Elem = crate::color::Elem::Links {valid: bool_3};
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut elem_5: color::Elem = crate::color::Elem::Pipe;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut bool_4: bool = crate::color::Elem::ne(elem_5_ref_0, elem_2_ref_0);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_2, exec: bool_1};
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Date;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6731() {
    rusty_monitor::set_test_id(6731);
    let mut str_0: &str = "mtOhvb8";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u64_0: u64 = 3917u64;
    let mut str_1: &str = "24VzMtiXUyN";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut option_0: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_0: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_0};
    let mut str_2: &str = "T9vI1O0";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut option_1: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut option_2: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_3: std::option::Option<bool> = std::option::Option::None;
    let mut option_4: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_5: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_6: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_7: std::option::Option<bool> = std::option::Option::None;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::TreeEdge;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::SymLink;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::Write;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut option_8: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_3: &str = "UArmSBj";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut option_9: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_3_ref_0);
    let mut colors_1: crate::color::Colors = crate::color::Colors {theme: option_9, lscolors: option_8};
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_3: color::Elem = crate::color::Elem::FileSmall;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_1_ref_0: &meta::date::Date = &mut date_1;
    let mut u64_1: u64 = 5684u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut str_4: &str = "M15PWEBaeH";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_3: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut layout_1_ref_0: &flags::layout::Layout = &mut layout_1;
    let mut elem_4: color::Elem = crate::color::Elem::NonFile;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut bool_1: bool = crate::color::Elem::has_suid(elem_0_ref_0);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut contentstyle_0: crossterm::style::ContentStyle = crate::color::Colors::style(colors_1_ref_0, elem_2_ref_0);
    let mut bool_2: bool = crate::meta::filetype::FileType::is_dirlike(filetype_1);
    crate::meta::filetype::FileType::render(filetype_0, colors_0_ref_0);
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut elem_5: color::Elem = crate::color::Elem::clone(elem_4_ref_0);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::User;
    let mut result_0: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7922() {
    rusty_monitor::set_test_id(7922);
    let mut elem_0: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut option_0: std::option::Option<usize> = std::option::Option::None;
    let mut option_1: std::option::Option<bool> = std::option::Option::None;
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_1, depth: option_0};
    let mut option_2: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_0);
    let mut option_3: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_0: bool = true;
    let mut option_4: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut option_5: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_6: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut option_7: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_0);
    let mut option_8: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_8, theme: option_7, separator: option_6};
    let mut option_9: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut option_10: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_11: std::option::Option<bool> = std::option::Option::None;
    let mut option_12: std::option::Option<std::string::String> = std::option::Option::None;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut option_13: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_0);
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut option_14: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_14, theme: option_13};
    let mut option_15: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_16: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_17: std::option::Option<bool> = std::option::Option::None;
    let mut u64_0: u64 = 6281u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut bool_1: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_1};
    let mut elem_1: color::Elem = crate::color::Elem::SymLink;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_2: color::Elem = crate::color::Elem::SymLink;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut option_18: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_19: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut option_20: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_1);
    let mut option_21: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut icons_1: crate::config_file::Icons = crate::config_file::Icons {when: option_21, theme: option_20, separator: option_19};
    let mut option_22: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_1);
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut option_23: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut option_24: std::option::Option<bool> = std::option::Option::None;
    let mut option_25: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_26: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_27: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_28: std::option::Option<bool> = std::option::Option::None;
    let mut u64_1: u64 = 62u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut option_29: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_30: std::option::Option<std::string::String> = std::option::Option::None;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut layout_0_ref_0: &flags::layout::Layout = &mut layout_0;
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut layout_1_ref_0: &flags::layout::Layout = &mut layout_1;
    let mut elem_3: color::Elem = crate::color::Elem::NonFile;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut bool_2: bool = crate::color::Elem::has_suid(elem_1_ref_0);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut contentstyle_0: crossterm::style::ContentStyle = crate::color::Colors::style(colors_0_ref_0, elem_2_ref_0);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut option_31: std::option::Option<lscolors::Indicator> = crate::color::Colors::get_indicator_from_elem(colors_1_ref_0, elem_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7960() {
    rusty_monitor::set_test_id(7960);
    let mut str_0: &str = "RV7icIZU5";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut elem_0: color::Elem = crate::color::Elem::Write;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut option_0: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut option_1: std::option::Option<bool> = std::option::Option::None;
    let mut option_2: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_3: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_4: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut option_5: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_0);
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_5, theme: option_4, separator: option_3};
    let mut option_6: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut option_7: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut option_8: std::option::Option<bool> = std::option::Option::None;
    let mut option_9: std::option::Option<std::string::String> = std::option::Option::None;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut option_10: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_0);
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut option_11: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_11, theme: option_10};
    let mut option_12: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_13: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_0: bool = true;
    let mut option_14: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_1: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_1};
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut option_15: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_0);
    let mut option_16: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_1: bool = true;
    let mut option_17: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut bool_2: bool = false;
    let mut option_18: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut option_19: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::None;
    let mut bool_3: bool = false;
    let mut option_20: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut option_21: std::option::Option<flags::sorting::SortColumn> = std::option::Option::Some(sortcolumn_0);
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_21, reverse: option_20, dir_grouping: option_19};
    let mut option_22: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_0);
    let mut option_23: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut option_24: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_0);
    let mut usize_0: usize = 9788usize;
    let mut option_25: std::option::Option<usize> = std::option::Option::Some(usize_0);
    let mut bool_4: bool = true;
    let mut option_26: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_26, depth: option_25};
    let mut option_27: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_0);
    let mut option_28: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut option_29: std::option::Option<bool> = std::option::Option::None;
    let mut option_30: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_31: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut option_32: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_1);
    let mut option_33: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut icons_1: crate::config_file::Icons = crate::config_file::Icons {when: option_33, theme: option_32, separator: option_31};
    let mut option_34: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_1);
    let mut option_35: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_36: std::option::Option<bool> = std::option::Option::None;
    let mut option_37: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_38: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_39: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_5: bool = true;
    let mut option_40: std::option::Option<bool> = std::option::Option::Some(bool_5);
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_40, blocks: option_39, color: option_38, date: option_37, dereference: option_36, display: option_35, icons: option_34, ignore_globs: option_30, indicators: option_29, layout: option_28, recursion: option_27, size: option_24, permission: option_23, sorting: option_22, no_symlink: option_18, total_size: option_17, symlink_arrow: option_16, hyperlink: option_15};
    let mut elem_1: color::Elem = crate::color::Elem::clone(elem_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2136() {
    rusty_monitor::set_test_id(2136);
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut option_0: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::Some(dirgrouping_0);
    let mut option_1: std::option::Option<bool> = std::option::Option::None;
    let mut option_2: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut bool_0: bool = false;
    let mut elem_0: color::Elem = crate::color::Elem::Links {valid: bool_0};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_1: color::Elem = crate::color::Elem::Exec;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut option_3: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_4: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut option_5: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_0);
    let mut option_6: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_6, theme: option_5, separator: option_4};
    let mut option_7: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut option_8: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut option_9: std::option::Option<bool> = std::option::Option::None;
    let mut option_10: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_11: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_12: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_13: std::option::Option<bool> = std::option::Option::None;
    let mut u64_0: u64 = 62u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_1};
    let mut option_14: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_15: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_16: std::option::Option<bool> = std::option::Option::None;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut layout_0_ref_0: &flags::layout::Layout = &mut layout_0;
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut layout_1_ref_0: &flags::layout::Layout = &mut layout_1;
    let mut elem_2: color::Elem = crate::color::Elem::NonFile;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut bool_1: bool = crate::color::Elem::has_suid(elem_0_ref_0);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut contentstyle_0: crossterm::style::ContentStyle = crate::color::Colors::style(colors_0_ref_0, elem_1_ref_0);
    let mut sorting_1: crate::config_file::Sorting = crate::config_file::Sorting {column: option_2, reverse: option_1, dir_grouping: option_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3376() {
    rusty_monitor::set_test_id(3376);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Group;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut bool_0: bool = true;
    let mut elem_2: color::Elem = crate::color::Elem::INode {valid: bool_0};
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut bool_1: bool = false;
    let mut elem_3: color::Elem = crate::color::Elem::Dir {uid: bool_1};
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut file_0: crate::color::theme::File = crate::color::theme::File {exec_uid: color_3, uid_no_exec: color_2, exec_no_uid: color_1, no_exec_no_uid: color_0};
    let mut file_0_ref_0: &crate::color::theme::File = &mut file_0;
    let mut elem_4: color::Elem = crate::color::Elem::Write;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut bool_2: bool = true;
    let mut bool_3: bool = false;
    let mut elem_5: color::Elem = crate::color::Elem::File {exec: bool_3, uid: bool_2};
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut elem_6: color::Elem = crate::color::Elem::SymLink;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_6_ref_0, theme_5_ref_0);
    let mut theme_6: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_6_ref_0: &crate::color::theme::Theme = &mut theme_6;
    let mut bool_4: bool = true;
    let mut elem_7: color::Elem = crate::color::Elem::Dir {uid: bool_4};
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut color_6: crossterm::style::Color = crate::color::Elem::get_color(elem_7_ref_0, theme_6_ref_0);
    let mut elem_8: color::Elem = crate::color::Elem::Write;
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut option_0: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_0: &str = "UArmSBj";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_1: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut colors_0: crate::color::Colors = crate::color::Colors {theme: option_1, lscolors: option_0};
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut theme_7: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2144() {
    rusty_monitor::set_test_id(2144);
    let mut elem_0: color::Elem = crate::color::Elem::Write;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut option_0: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_0: &str = "UArmSBj";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_1: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut colors_0: crate::color::Colors = crate::color::Colors {theme: option_1, lscolors: option_0};
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut elem_1: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut bool_0: bool = false;
    let mut elem_2: color::Elem = crate::color::Elem::Dir {uid: bool_0};
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_3: color::Elem = crate::color::Elem::FileSmall;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut u64_0: u64 = 7317u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut u64_1: u64 = 5684u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut str_1: &str = "M15PWEBaeH";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut layout_0_ref_0: &flags::layout::Layout = &mut layout_0;
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut layout_1_ref_0: &flags::layout::Layout = &mut layout_1;
    let mut elem_4: color::Elem = crate::color::Elem::NonFile;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut bool_1: bool = crate::color::Elem::has_suid(elem_2_ref_0);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut contentstyle_0: crossterm::style::ContentStyle = crate::color::Colors::style(colors_0_ref_0, elem_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6841() {
    rusty_monitor::set_test_id(6841);
    let mut elem_0: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut bool_0: bool = true;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut bool_1: bool = false;
    let mut elem_1: color::Elem = crate::color::Elem::Dir {uid: bool_1};
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_2: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut bool_2: bool = true;
    let mut elem_3: color::Elem = crate::color::Elem::Dir {uid: bool_2};
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_2_ref_0);
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut elem_4: color::Elem = crate::color::Elem::Dir {uid: bool_0};
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut bool_3: bool = crate::color::Elem::ne(elem_4_ref_0, elem_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1323() {
    rusty_monitor::set_test_id(1323);
    let mut bool_0: bool = true;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut bool_1: bool = false;
    let mut elem_0: color::Elem = crate::color::Elem::Dir {uid: bool_1};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut elem_2: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut bool_2: bool = true;
    let mut elem_3: color::Elem = crate::color::Elem::Dir {uid: bool_2};
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_2_ref_0);
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut elem_4: color::Elem = crate::color::Elem::Dir {uid: bool_0};
    let mut elem_5: color::Elem = crate::color::Elem::Pipe;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut tuple_0: () = crate::color::Elem::assert_receiver_is_total_eq(elem_4_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2703() {
    rusty_monitor::set_test_id(2703);
    let mut str_0: &str = "1n";
    let mut elem_0: color::Elem = crate::color::Elem::Write;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut option_0: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_1: &str = "UArmSBj";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut option_1: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_1_ref_0);
    let mut colors_0: crate::color::Colors = crate::color::Colors {theme: option_1, lscolors: option_0};
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_1: color::Elem = crate::color::Elem::TreeEdge;
    let mut bool_0: bool = false;
    let mut elem_2: color::Elem = crate::color::Elem::Dir {uid: bool_0};
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_3: color::Elem = crate::color::Elem::FileSmall;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut u64_0: u64 = 7317u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut u64_1: u64 = 5684u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut layout_0_ref_0: &flags::layout::Layout = &mut layout_0;
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut layout_1_ref_0: &flags::layout::Layout = &mut layout_1;
    let mut elem_4: color::Elem = crate::color::Elem::NonFile;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut bool_1: bool = crate::color::Elem::has_suid(elem_2_ref_0);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut elem_5: color::Elem = crate::color::Elem::clone(elem_4_ref_0);
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_6: color::Elem = crate::color::Elem::clone(elem_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5908() {
    rusty_monitor::set_test_id(5908);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Group;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut elem_1: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_2: color::Elem = crate::color::Elem::Octal;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_1_ref_0);
    let mut elem_3: color::Elem = crate::color::Elem::Socket;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut elem_4: color::Elem = crate::color::Elem::Acl;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut elem_5: color::Elem = crate::color::Elem::File {exec: bool_1, uid: bool_0};
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut elem_6: color::Elem = crate::color::Elem::NonFile;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut option_1: std::option::Option<std::path::PathBuf> = crate::config_file::Config::config_file_path();
    let mut elem_7: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut elem_8: color::Elem = crate::color::Elem::clone(elem_7_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2251() {
    rusty_monitor::set_test_id(2251);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::ExecSticky;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_1: color::Elem = crate::color::Elem::Special;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_2: color::Elem = crate::color::Elem::Older;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_3: color::Elem = crate::color::Elem::Older;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut u64_0: u64 = 7317u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut u64_1: u64 = 5684u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut str_0: &str = "M15PWEBaeH";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut option_0: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_2);
    let mut u64_2: u64 = 1777u64;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size::new(u64_2);
    let mut size_2_ref_0: &crate::meta::size::Size = &mut size_2;
    let mut u64_3: u64 = 1873u64;
    let mut size_3: crate::meta::size::Size = crate::meta::size::Size::new(u64_3);
    let mut size_3_ref_0: &crate::meta::size::Size = &mut size_3;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut str_1: &str = "7BERJlG75X";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_1_ref_0: &meta::date::Date = &mut date_1;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut layout_0_ref_0: &flags::layout::Layout = &mut layout_0;
    let mut elem_4: color::Elem = crate::color::Elem::NonFile;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut bool_0: bool = crate::color::Elem::has_suid(elem_2_ref_0);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut contentstyle_0: crossterm::style::ContentStyle = crate::color::Colors::style(colors_1_ref_0, elem_3_ref_0);
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut elem_5: color::Elem = crate::color::Elem::clone(elem_1_ref_0);
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut bool_1: bool = crate::meta::filetype::FileType::is_dirlike(filetype_3);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6068() {
    rusty_monitor::set_test_id(6068);
    let mut str_0: &str = "ZcsqJ3eQY";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut elem_0: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_1: color::Elem = crate::color::Elem::FileSmall;
    let mut option_0: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut option_1: std::option::Option<bool> = std::option::Option::None;
    let mut option_2: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_3: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_4: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut option_5: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_0);
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_5, theme: option_4, separator: option_3};
    let mut option_6: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut option_7: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut option_8: std::option::Option<bool> = std::option::Option::None;
    let mut option_9: std::option::Option<std::string::String> = std::option::Option::None;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut option_10: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_0);
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut option_11: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_11, theme: option_10};
    let mut option_12: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_13: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_0: bool = true;
    let mut option_14: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_1: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_1};
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut option_15: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_0);
    let mut option_16: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_1: bool = true;
    let mut option_17: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut bool_2: bool = false;
    let mut option_18: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut option_19: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::None;
    let mut bool_3: bool = false;
    let mut option_20: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut option_21: std::option::Option<flags::sorting::SortColumn> = std::option::Option::Some(sortcolumn_0);
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_21, reverse: option_20, dir_grouping: option_19};
    let mut option_22: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_0);
    let mut option_23: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut option_24: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_0);
    let mut usize_0: usize = 9788usize;
    let mut option_25: std::option::Option<usize> = std::option::Option::Some(usize_0);
    let mut bool_4: bool = true;
    let mut option_26: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_26, depth: option_25};
    let mut option_27: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_0);
    let mut option_28: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut option_29: std::option::Option<bool> = std::option::Option::None;
    let mut option_30: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_31: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut option_32: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_1);
    let mut option_33: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut icons_1: crate::config_file::Icons = crate::config_file::Icons {when: option_33, theme: option_32, separator: option_31};
    let mut option_34: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_1);
    let mut option_35: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_36: std::option::Option<bool> = std::option::Option::None;
    let mut option_37: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_38: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_39: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_5: bool = true;
    let mut option_40: std::option::Option<bool> = std::option::Option::Some(bool_5);
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_2: color::Elem = crate::color::Elem::clone(elem_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7953() {
    rusty_monitor::set_test_id(7953);
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut option_0: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut str_0: &str = "mtOhvb8";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u64_0: u64 = 3917u64;
    let mut str_1: &str = "24VzMtiXUyN";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_0: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_0};
    let mut str_2: &str = "T9vI1O0";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut option_2: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut option_3: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_4: std::option::Option<bool> = std::option::Option::None;
    let mut option_5: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_6: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_7: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_8: std::option::Option<bool> = std::option::Option::None;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::TreeEdge;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::SymLink;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::Write;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut option_9: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_3: &str = "UArmSBj";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut option_10: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_3_ref_0);
    let mut colors_1: crate::color::Colors = crate::color::Colors {theme: option_10, lscolors: option_9};
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_3: color::Elem = crate::color::Elem::FileSmall;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_1_ref_0: &meta::date::Date = &mut date_1;
    let mut u64_1: u64 = 5684u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut str_4: &str = "M15PWEBaeH";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_3: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut layout_0_ref_0: &flags::layout::Layout = &mut layout_0;
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut layout_1_ref_0: &flags::layout::Layout = &mut layout_1;
    let mut elem_4: color::Elem = crate::color::Elem::NonFile;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut bool_1: bool = crate::color::Elem::has_suid(elem_0_ref_0);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut contentstyle_0: crossterm::style::ContentStyle = crate::color::Colors::style(colors_1_ref_0, elem_2_ref_0);
    let mut bool_2: bool = crate::meta::filetype::FileType::is_dirlike(filetype_2);
    crate::meta::filetype::FileType::render(filetype_1, colors_0_ref_0);
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut elem_5: color::Elem = crate::color::Elem::clone(elem_4_ref_0);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::User;
    let mut result_0: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7166() {
    rusty_monitor::set_test_id(7166);
    let mut bool_0: bool = true;
    let mut elem_0: color::Elem = crate::color::Elem::Dir {uid: bool_0};
    let mut str_0: &str = "mtOhvb8";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u64_0: u64 = 3917u64;
    let mut str_1: &str = "24VzMtiXUyN";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut option_0: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_1: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_1};
    let mut str_2: &str = "T9vI1O0";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut option_1: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut option_2: std::option::Option<bool> = std::option::Option::None;
    let mut option_3: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_4: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_5: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_6: std::option::Option<bool> = std::option::Option::None;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_1: color::Elem = crate::color::Elem::TreeEdge;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_2: color::Elem = crate::color::Elem::SymLink;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_3: color::Elem = crate::color::Elem::Write;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut option_7: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_3: &str = "UArmSBj";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut option_8: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_3_ref_0);
    let mut colors_1: crate::color::Colors = crate::color::Colors {theme: option_8, lscolors: option_7};
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_4: color::Elem = crate::color::Elem::FileSmall;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_1_ref_0: &meta::date::Date = &mut date_1;
    let mut u64_1: u64 = 5684u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut str_4: &str = "M15PWEBaeH";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_3: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut layout_0_ref_0: &flags::layout::Layout = &mut layout_0;
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut layout_1_ref_0: &flags::layout::Layout = &mut layout_1;
    let mut elem_5: color::Elem = crate::color::Elem::NonFile;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut bool_2: bool = crate::color::Elem::has_suid(elem_1_ref_0);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut contentstyle_0: crossterm::style::ContentStyle = crate::color::Colors::style(colors_1_ref_0, elem_3_ref_0);
    let mut bool_3: bool = crate::meta::filetype::FileType::is_dirlike(filetype_2);
    crate::meta::filetype::FileType::render(filetype_1, colors_0_ref_0);
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_6: color::Elem = crate::color::Elem::clone(elem_0_ref_0);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::User;
    let mut result_0: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_0_ref_0);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6816() {
    rusty_monitor::set_test_id(6816);
    let mut elem_0: color::Elem = crate::color::Elem::Write;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut str_0: &str = "mtOhvb8";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u64_0: u64 = 3917u64;
    let mut str_1: &str = "24VzMtiXUyN";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut option_0: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_0: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_0};
    let mut str_2: &str = "T9vI1O0";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut option_1: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut option_2: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_3: std::option::Option<bool> = std::option::Option::None;
    let mut option_4: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_5: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_6: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_7: std::option::Option<bool> = std::option::Option::None;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_1: color::Elem = crate::color::Elem::TreeEdge;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_2: color::Elem = crate::color::Elem::SymLink;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_3: color::Elem = crate::color::Elem::Write;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut option_8: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_3: &str = "UArmSBj";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut option_9: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_3_ref_0);
    let mut colors_1: crate::color::Colors = crate::color::Colors {theme: option_9, lscolors: option_8};
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_4: color::Elem = crate::color::Elem::FileSmall;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_1_ref_0: &meta::date::Date = &mut date_1;
    let mut u64_1: u64 = 5684u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut str_4: &str = "M15PWEBaeH";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_3: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut layout_0_ref_0: &flags::layout::Layout = &mut layout_0;
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut layout_1_ref_0: &flags::layout::Layout = &mut layout_1;
    let mut elem_5: color::Elem = crate::color::Elem::Group;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut bool_1: bool = crate::color::Elem::has_suid(elem_1_ref_0);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut contentstyle_0: crossterm::style::ContentStyle = crate::color::Colors::style(colors_1_ref_0, elem_3_ref_0);
    let mut bool_2: bool = crate::meta::filetype::FileType::is_dirlike(filetype_2);
    crate::meta::filetype::FileType::render(filetype_1, colors_0_ref_0);
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut elem_6: color::Elem = crate::color::Elem::clone(elem_5_ref_0);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::User;
    let mut result_0: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5448() {
    rusty_monitor::set_test_id(5448);
    let mut str_0: &str = "24VzMtiXUyN";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_0: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_0: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_0};
    let mut str_1: &str = "T9vI1O0";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut option_1: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut option_2: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut option_3: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_4: std::option::Option<bool> = std::option::Option::None;
    let mut option_5: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_6: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_7: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::SymLink;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut elem_2: color::Elem = crate::color::Elem::Write;
    let mut option_8: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_2: &str = "UArmSBj";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut option_9: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_2_ref_0);
    let mut colors_1: crate::color::Colors = crate::color::Colors {theme: option_9, lscolors: option_8};
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut elem_3: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut bool_1: bool = false;
    let mut elem_4: color::Elem = crate::color::Elem::Dir {uid: bool_1};
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut elem_5: color::Elem = crate::color::Elem::FileSmall;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut u64_0: u64 = 7317u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_1_ref_0: &meta::date::Date = &mut date_1;
    let mut u64_1: u64 = 5684u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut str_3: &str = "M15PWEBaeH";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_3: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut layout_0_ref_0: &flags::layout::Layout = &mut layout_0;
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut layout_1_ref_0: &flags::layout::Layout = &mut layout_1;
    let mut elem_6: color::Elem = crate::color::Elem::NonFile;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut bool_2: bool = crate::color::Elem::has_suid(elem_4_ref_0);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut bool_3: bool = crate::meta::filetype::FileType::is_dirlike(filetype_0);
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    crate::meta::filetype::FileType::render(filetype_1, colors_0_ref_0);
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut layout_3: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3220() {
    rusty_monitor::set_test_id(3220);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut option_0: std::option::Option<usize> = std::option::Option::None;
    let mut bool_0: bool = false;
    let mut option_1: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_1, depth: option_0};
    let mut option_2: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_0);
    let mut option_3: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_1: bool = true;
    let mut option_4: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_5: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_6: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut option_7: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_0);
    let mut option_8: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_8, theme: option_7, separator: option_6};
    let mut option_9: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut option_10: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_11: std::option::Option<bool> = std::option::Option::None;
    let mut option_12: std::option::Option<std::string::String> = std::option::Option::None;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut option_13: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_1);
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut option_14: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_14, theme: option_13};
    let mut option_15: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_16: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_17: std::option::Option<bool> = std::option::Option::None;
    let mut u64_0: u64 = 6281u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut bool_2: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_2};
    let mut elem_0: color::Elem = crate::color::Elem::SymLink;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_1: color::Elem = crate::color::Elem::SymLink;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut option_18: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_19: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut option_20: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_1);
    let mut option_21: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut icons_1: crate::config_file::Icons = crate::config_file::Icons {when: option_21, theme: option_20, separator: option_19};
    let mut option_22: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_1);
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut option_23: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut option_24: std::option::Option<bool> = std::option::Option::None;
    let mut option_25: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_26: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_27: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_28: std::option::Option<bool> = std::option::Option::None;
    let mut u64_1: u64 = 62u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_3);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut option_29: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_30: std::option::Option<std::string::String> = std::option::Option::None;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut layout_0_ref_0: &flags::layout::Layout = &mut layout_0;
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut layout_1_ref_0: &flags::layout::Layout = &mut layout_1;
    let mut elem_2: color::Elem = crate::color::Elem::NonFile;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut bool_3: bool = crate::color::Elem::has_suid(elem_0_ref_0);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut contentstyle_0: crossterm::style::ContentStyle = crate::color::Colors::style(colors_1_ref_0, elem_1_ref_0);
    crate::meta::filetype::FileType::render(filetype_0, colors_0_ref_0);
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7140() {
    rusty_monitor::set_test_id(7140);
    let mut bool_0: bool = false;
    let mut elem_0: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut bool_1: bool = true;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_1: color::Elem = crate::color::Elem::Links {valid: bool_0};
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_2: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_1_ref_0);
    let mut elem_3: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_4: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_2_ref_0);
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut elem_5: color::Elem = crate::color::Elem::Dir {uid: bool_1};
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut bool_2: bool = crate::color::Elem::ne(elem_5_ref_0, elem_0_ref_0);
    let mut dirgrouping_0_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_0;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2977() {
    rusty_monitor::set_test_id(2977);
    let mut elem_0: color::Elem = crate::color::Elem::Older;
    let mut elem_1: color::Elem = crate::color::Elem::Write;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut elem_2: color::Elem = crate::color::Elem::File {exec: bool_1, uid: bool_0};
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_3: color::Elem = crate::color::Elem::SymLink;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut bool_2: bool = true;
    let mut elem_4: color::Elem = crate::color::Elem::Dir {uid: bool_2};
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_2_ref_0);
    let mut bool_3: bool = true;
    let mut elem_5: color::Elem = crate::color::Elem::Write;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut option_0: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_0: &str = "UArmSBj";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_1: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut colors_0: crate::color::Colors = crate::color::Colors {theme: option_1, lscolors: option_0};
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_6: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_6_ref_0, theme_3_ref_0);
    let mut bool_4: bool = false;
    let mut elem_7: color::Elem = crate::color::Elem::Dir {uid: bool_4};
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut elem_8: color::Elem = crate::color::Elem::FileSmall;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut u64_0: u64 = 7317u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut u64_1: u64 = 5684u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut str_1: &str = "M15PWEBaeH";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut layout_0_ref_0: &flags::layout::Layout = &mut layout_0;
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut layout_1_ref_0: &flags::layout::Layout = &mut layout_1;
    let mut elem_9: color::Elem = crate::color::Elem::NonFile;
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut bool_5: bool = crate::color::Elem::has_suid(elem_7_ref_0);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut contentstyle_0: crossterm::style::ContentStyle = crate::color::Colors::style(colors_0_ref_0, elem_5_ref_0);
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_10: color::Elem = crate::color::Elem::clone(elem_0_ref_0);
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_3};
    let mut symlink_0: crate::color::theme::Symlink = crate::color::theme::Symlink {default: color_2, broken: color_1, missing_target: color_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6257() {
    rusty_monitor::set_test_id(6257);
    let mut str_0: &str = "VXYLppXAt8";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_0: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_0};
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut option_0: std::option::Option<usize> = std::option::Option::None;
    let mut bool_1: bool = false;
    let mut option_1: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_1, depth: option_0};
    let mut option_2: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_0);
    let mut option_3: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_2: bool = true;
    let mut option_4: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut option_5: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_6: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut option_7: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_0);
    let mut option_8: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_8, theme: option_7, separator: option_6};
    let mut option_9: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut option_10: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_11: std::option::Option<bool> = std::option::Option::None;
    let mut option_12: std::option::Option<std::string::String> = std::option::Option::None;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut option_13: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_0);
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut option_14: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_14, theme: option_13};
    let mut option_15: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_16: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_17: std::option::Option<bool> = std::option::Option::None;
    let mut u64_0: u64 = 6281u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut bool_3: bool = false;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_3};
    let mut elem_0: color::Elem = crate::color::Elem::SymLink;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_1: color::Elem = crate::color::Elem::SymLink;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut option_18: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_19: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut option_20: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_1);
    let mut option_21: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut icons_1: crate::config_file::Icons = crate::config_file::Icons {when: option_21, theme: option_20, separator: option_19};
    let mut option_22: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_1);
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut option_23: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut option_24: std::option::Option<bool> = std::option::Option::None;
    let mut option_25: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_26: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_27: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_28: std::option::Option<bool> = std::option::Option::None;
    let mut u64_1: u64 = 62u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut option_29: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_30: std::option::Option<std::string::String> = std::option::Option::None;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut layout_0_ref_0: &flags::layout::Layout = &mut layout_0;
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut layout_1_ref_0: &flags::layout::Layout = &mut layout_1;
    let mut elem_2: color::Elem = crate::color::Elem::NonFile;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut bool_4: bool = crate::color::Elem::has_suid(elem_0_ref_0);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut bool_5: bool = crate::meta::filetype::FileType::is_dirlike(filetype_1);
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    crate::meta::filetype::FileType::render(filetype_0, colors_0_ref_0);
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_3: color::Elem = crate::color::Elem::clone(elem_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_385() {
    rusty_monitor::set_test_id(385);
    let mut elem_0: color::Elem = crate::color::Elem::Special;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut bool_0: bool = true;
    let mut elem_1: color::Elem = crate::color::Elem::Dir {uid: bool_0};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_2: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_1_ref_0);
    let mut bool_1: bool = false;
    let mut bool_2: bool = false;
    let mut elem_3: color::Elem = crate::color::Elem::Socket;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut bool_3: bool = false;
    let mut elem_4: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_5: color::Elem = crate::color::Elem::Links {valid: bool_3};
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_2_ref_0);
    let mut elem_6: color::Elem = crate::color::Elem::Pipe;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut bool_4: bool = crate::color::Elem::ne(elem_6_ref_0, elem_3_ref_0);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_2, exec: bool_1};
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut links_0: crate::color::theme::Links = crate::color::theme::Links {valid: color_1, invalid: color_0};
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Date;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7992() {
    rusty_monitor::set_test_id(7992);
    let mut elem_0: color::Elem = crate::color::Elem::Exec;
    let mut bool_0: bool = false;
    let mut elem_1: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_2: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_3: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut option_0: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_0: &str = "X9HI23ObtwB";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_1: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut u64_0: u64 = 7561u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut elem_4: color::Elem = crate::color::Elem::Dir {uid: bool_0};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut bool_1: bool = crate::color::Elem::ne(elem_1_ref_0, elem_3_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7600() {
    rusty_monitor::set_test_id(7600);
    let mut str_0: &str = "mtOhvb8";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u64_0: u64 = 3917u64;
    let mut str_1: &str = "24VzMtiXUyN";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut option_0: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_0: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_0};
    let mut str_2: &str = "T9vI1O0";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut option_1: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut option_2: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_3: std::option::Option<bool> = std::option::Option::None;
    let mut option_4: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_5: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_6: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_7: std::option::Option<bool> = std::option::Option::None;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::TreeEdge;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::SymLink;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::Write;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut option_8: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_3: &str = "UArmSBj";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut option_9: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_3_ref_0);
    let mut colors_1: crate::color::Colors = crate::color::Colors {theme: option_9, lscolors: option_8};
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut bool_1: bool = false;
    let mut elem_3: color::Elem = crate::color::Elem::Dir {uid: bool_1};
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut elem_4: color::Elem = crate::color::Elem::FileSmall;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_1_ref_0: &meta::date::Date = &mut date_1;
    let mut u64_1: u64 = 5684u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut str_4: &str = "M15PWEBaeH";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_3: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut layout_0_ref_0: &flags::layout::Layout = &mut layout_0;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut bool_2: bool = crate::color::Elem::has_suid(elem_2_ref_0);
    let mut contentstyle_0: crossterm::style::ContentStyle = crate::color::Colors::style(colors_0_ref_0, elem_3_ref_0);
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_5: color::Elem = crate::color::Elem::clone(elem_4_ref_0);
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_111() {
    rusty_monitor::set_test_id(111);
    let mut elem_0: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut bool_0: bool = false;
    let mut elem_1: color::Elem = crate::color::Elem::INode {valid: bool_0};
    let mut bool_1: bool = true;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut bool_2: bool = false;
    let mut elem_2: color::Elem = crate::color::Elem::Dir {uid: bool_2};
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_3: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_1_ref_0);
    let mut elem_4: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut bool_3: bool = true;
    let mut elem_5: color::Elem = crate::color::Elem::Dir {uid: bool_3};
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_2_ref_0);
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut elem_6: color::Elem = crate::color::Elem::Dir {uid: bool_1};
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut bool_4: bool = crate::color::Elem::eq(elem_6_ref_0, elem_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2653() {
    rusty_monitor::set_test_id(2653);
    let mut str_0: &str = "1n";
    let mut elem_0: color::Elem = crate::color::Elem::Write;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut option_0: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_1: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_0_ref_0);
    let mut bool_0: bool = false;
    let mut elem_2: color::Elem = crate::color::Elem::Dir {uid: bool_0};
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_3: color::Elem = crate::color::Elem::FileSmall;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut u64_0: u64 = 7317u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut u64_1: u64 = 5684u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut layout_0_ref_0: &flags::layout::Layout = &mut layout_0;
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut layout_1_ref_0: &flags::layout::Layout = &mut layout_1;
    let mut elem_4: color::Elem = crate::color::Elem::NonFile;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut bool_1: bool = crate::color::Elem::has_suid(elem_2_ref_0);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut elem_5: color::Elem = crate::color::Elem::clone(elem_4_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6183() {
    rusty_monitor::set_test_id(6183);
    let mut str_0: &str = "69IvMfWvt0";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut elem_0: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut option_0: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut option_1: std::option::Option<crate::color::theme::Theme> = std::option::Option::None;
    let mut colors_0: crate::color::Colors = crate::color::Colors {theme: option_1, lscolors: option_0};
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut elem_1: color::Elem = crate::color::Elem::User;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut bool_0: bool = false;
    let mut elem_2: color::Elem = crate::color::Elem::INode {valid: bool_0};
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut option_2: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut option_3: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_1: bool = true;
    let mut option_4: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_0: usize = 4809usize;
    let mut bool_2: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_2, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_0};
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut option_5: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_0);
    let mut option_6: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_3: bool = true;
    let mut option_7: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut option_8: std::option::Option<bool> = std::option::Option::None;
    let mut option_9: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::None;
    let mut option_10: std::option::Option<bool> = std::option::Option::None;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut option_11: std::option::Option<flags::sorting::SortColumn> = std::option::Option::Some(sortcolumn_0);
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_11, reverse: option_10, dir_grouping: option_9};
    let mut option_12: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_0);
    let mut option_13: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut option_14: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_15: std::option::Option<usize> = std::option::Option::None;
    let mut bool_4: bool = false;
    let mut option_16: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut recursion_1: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_16, depth: option_15};
    let mut option_17: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_1);
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut option_18: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_1);
    let mut bool_5: bool = true;
    let mut option_19: std::option::Option<bool> = std::option::Option::Some(bool_5);
    let mut option_20: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_21: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_22: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut bool_6: bool = true;
    let mut option_23: std::option::Option<bool> = std::option::Option::Some(bool_6);
    let mut option_24: std::option::Option<std::string::String> = std::option::Option::None;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut option_25: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_1);
    let mut coloroption_2: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut option_26: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_2);
    let mut color_1: crate::config_file::Color = crate::config_file::Color {when: option_26, theme: option_25};
    let mut option_27: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_1);
    let mut option_28: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_29: std::option::Option<bool> = std::option::Option::None;
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut layout_0_ref_0: &flags::layout::Layout = &mut layout_0;
    let mut elem_3: color::Elem = crate::color::Elem::NonFile;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut bool_7: bool = crate::color::Elem::has_suid(elem_1_ref_0);
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut contentstyle_0: crossterm::style::ContentStyle = crate::color::Colors::style(colors_0_ref_0, elem_0_ref_0);
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut elem_4: color::Elem = crate::color::Elem::clone(elem_2_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1212() {
    rusty_monitor::set_test_id(1212);
    let mut elem_0: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut option_0: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_0: &str = "X9HI23ObtwB";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_1: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut u64_0: u64 = 7561u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_0: usize = 1992usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut option_2: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut option_3: std::option::Option<crate::color::theme::Theme> = std::option::Option::Some(theme_0);
    let mut colors_0: crate::color::Colors = crate::color::Colors {theme: option_3, lscolors: option_2};
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut contentstyle_0: crossterm::style::ContentStyle = crate::color::Colors::style(colors_0_ref_0, elem_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5207() {
    rusty_monitor::set_test_id(5207);
    let mut str_0: &str = "24VzMtiXUyN";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_0: std::option::Option<std::string::String> = std::option::Option::None;
    let mut str_1: &str = "T9vI1O0";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut option_1: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut option_2: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_2, theme: option_1, separator: option_0};
    let mut option_3: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut option_4: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_5: std::option::Option<bool> = std::option::Option::None;
    let mut option_6: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_7: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_8: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_9: std::option::Option<bool> = std::option::Option::None;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::SymLink;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::Write;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut option_10: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_2: &str = "UArmSBj";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut option_11: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_2_ref_0);
    let mut colors_1: crate::color::Colors = crate::color::Colors {theme: option_11, lscolors: option_10};
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut bool_0: bool = false;
    let mut elem_4: color::Elem = crate::color::Elem::Dir {uid: bool_0};
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut elem_5: color::Elem = crate::color::Elem::FileSmall;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut u64_0: u64 = 7317u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_1_ref_0: &meta::date::Date = &mut date_1;
    let mut u64_1: u64 = 5684u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut str_3: &str = "n7wSBbe";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_3: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut layout_0_ref_0: &flags::layout::Layout = &mut layout_0;
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut layout_1_ref_0: &flags::layout::Layout = &mut layout_1;
    let mut elem_6: color::Elem = crate::color::Elem::NonFile;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut bool_1: bool = crate::color::Elem::has_suid(elem_4_ref_0);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut contentstyle_0: crossterm::style::ContentStyle = crate::color::Colors::style(colors_1_ref_0, elem_2_ref_0);
    let mut bool_2: bool = crate::meta::filetype::FileType::is_dirlike(filetype_1);
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    crate::meta::filetype::FileType::render(filetype_0, colors_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3013() {
    rusty_monitor::set_test_id(3013);
    let mut elem_0: color::Elem = crate::color::Elem::Write;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut elem_1: color::Elem = crate::color::Elem::File {exec: bool_1, uid: bool_0};
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_2: color::Elem = crate::color::Elem::SymLink;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut bool_2: bool = true;
    let mut elem_3: color::Elem = crate::color::Elem::Dir {uid: bool_2};
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_2_ref_0);
    let mut bool_3: bool = true;
    let mut elem_4: color::Elem = crate::color::Elem::Write;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut option_0: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_0: &str = "UArmSBj";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_1: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut colors_0: crate::color::Colors = crate::color::Colors {theme: option_1, lscolors: option_0};
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_5: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_3_ref_0);
    let mut bool_4: bool = false;
    let mut elem_6: color::Elem = crate::color::Elem::Dir {uid: bool_4};
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut elem_7: color::Elem = crate::color::Elem::FileSmall;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut u64_0: u64 = 7317u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut u64_1: u64 = 5684u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut str_1: &str = "M15PWEBaeH";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut layout_0_ref_0: &flags::layout::Layout = &mut layout_0;
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut layout_1_ref_0: &flags::layout::Layout = &mut layout_1;
    let mut elem_8: color::Elem = crate::color::Elem::NonFile;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut bool_5: bool = crate::color::Elem::has_suid(elem_6_ref_0);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut contentstyle_0: crossterm::style::ContentStyle = crate::color::Colors::style(colors_0_ref_0, elem_4_ref_0);
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut elem_9: color::Elem = crate::color::Elem::clone(elem_7_ref_0);
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_3};
    let mut symlink_0: crate::color::theme::Symlink = crate::color::theme::Symlink {default: color_2, broken: color_1, missing_target: color_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1262() {
    rusty_monitor::set_test_id(1262);
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut option_0: std::option::Option<usize> = std::option::Option::None;
    let mut option_1: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_0: bool = true;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut option_3: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_4: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut option_5: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_0);
    let mut option_6: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_6, theme: option_5, separator: option_4};
    let mut option_7: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut option_8: std::option::Option<bool> = std::option::Option::None;
    let mut option_9: std::option::Option<std::string::String> = std::option::Option::None;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut option_10: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_0);
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut option_11: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_11, theme: option_10};
    let mut option_12: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_13: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_14: std::option::Option<bool> = std::option::Option::None;
    let mut u64_0: u64 = 6281u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut bool_1: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_1};
    let mut elem_0: color::Elem = crate::color::Elem::SymLink;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_1: color::Elem = crate::color::Elem::SymLink;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut option_15: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_16: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut option_17: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_1);
    let mut option_18: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut icons_1: crate::config_file::Icons = crate::config_file::Icons {when: option_18, theme: option_17, separator: option_16};
    let mut option_19: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_1);
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut option_20: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut option_21: std::option::Option<bool> = std::option::Option::None;
    let mut option_22: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_23: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_24: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_25: std::option::Option<bool> = std::option::Option::None;
    let mut u64_1: u64 = 62u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut option_26: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_27: std::option::Option<std::string::String> = std::option::Option::None;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut layout_0_ref_0: &flags::layout::Layout = &mut layout_0;
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut layout_1_ref_0: &flags::layout::Layout = &mut layout_1;
    let mut elem_2: color::Elem = crate::color::Elem::NonFile;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut bool_2: bool = crate::color::Elem::has_suid(elem_0_ref_0);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut contentstyle_0: crossterm::style::ContentStyle = crate::color::Colors::style(colors_0_ref_0, elem_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7193() {
    rusty_monitor::set_test_id(7193);
    let mut str_0: &str = "hJaaEzh";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut bool_0: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_0};
    let mut elem_0: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_1: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_2: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut option_1: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_1: &str = "X9HI23ObtwB";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut option_2: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_1_ref_0);
    let mut u64_0: u64 = 7561u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut option_3: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut option_4: std::option::Option<crate::color::theme::Theme> = std::option::Option::Some(theme_0);
    let mut colors_0: crate::color::Colors = crate::color::Colors {theme: option_4, lscolors: option_3};
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut bool_1: bool = true;
    let mut bool_2: bool = false;
    let mut bool_3: bool = false;
    let mut bool_4: bool = false;
    let mut bool_5: bool = false;
    let mut bool_6: bool = false;
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = false;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut bool_12: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_12, user_write: bool_11, user_execute: bool_10, group_read: bool_9, group_write: bool_8, group_execute: bool_7, other_read: bool_6, other_write: bool_5, other_execute: bool_4, sticky: bool_3, setgid: bool_2, setuid: bool_1};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut bool_13: bool = true;
    let mut elem_3: color::Elem = crate::color::Elem::Dir {uid: bool_13};
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_4: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_2_ref_0);
    let mut bool_14: bool = false;
    let mut bool_15: bool = false;
    let mut elem_5: color::Elem = crate::color::Elem::Socket;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut elem_6: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_7: color::Elem = crate::color::Elem::Pipe;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut bool_16: bool = crate::color::Elem::ne(elem_7_ref_0, elem_5_ref_0);
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_15, exec: bool_14};
    let mut contentstyle_0: crossterm::style::ContentStyle = crate::color::Colors::style(colors_0_ref_0, elem_0_ref_0);
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut elem_8: color::Elem = crate::color::Elem::clone(elem_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5650() {
    rusty_monitor::set_test_id(5650);
    let mut elem_0: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut option_0: std::option::Option<bool> = std::option::Option::None;
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_2: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_3: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_0: bool = false;
    let mut option_4: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_0: usize = 7151usize;
    let mut bool_1: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_1, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut option_5: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_1);
    let mut option_6: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_2: bool = false;
    let mut option_7: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut option_8: std::option::Option<bool> = std::option::Option::None;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut option_9: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::Some(dirgrouping_1);
    let mut option_10: std::option::Option<bool> = std::option::Option::None;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut option_11: std::option::Option<flags::sorting::SortColumn> = std::option::Option::Some(sortcolumn_1);
    let mut sorting_1: crate::config_file::Sorting = crate::config_file::Sorting {column: option_11, reverse: option_10, dir_grouping: option_9};
    let mut option_12: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_1);
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut option_13: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_1);
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut option_14: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_1);
    let mut usize_1: usize = 3329usize;
    let mut option_15: std::option::Option<usize> = std::option::Option::Some(usize_1);
    let mut option_16: std::option::Option<bool> = std::option::Option::None;
    let mut recursion_1: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_16, depth: option_15};
    let mut option_17: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_1);
    let mut option_18: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_3: bool = true;
    let mut option_19: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut option_20: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_21: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut display_1: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut option_22: std::option::Option<flags::display::Display> = std::option::Option::Some(display_1);
    let mut bool_4: bool = false;
    let mut option_23: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut option_24: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_25: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_26: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_5: bool = true;
    let mut option_27: std::option::Option<bool> = std::option::Option::Some(bool_5);
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_27, blocks: option_26, color: option_25, date: option_24, dereference: option_23, display: option_22, icons: option_21, ignore_globs: option_20, indicators: option_19, layout: option_18, recursion: option_17, size: option_14, permission: option_13, sorting: option_12, no_symlink: option_8, total_size: option_7, symlink_arrow: option_6, hyperlink: option_5};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_2: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut u64_0: u64 = 3400u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut contentstyle_0: crossterm::style::ContentStyle = crate::color::Colors::style(colors_0_ref_0, elem_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7801() {
    rusty_monitor::set_test_id(7801);
    let mut str_0: &str = "oomG535vT6UaE3";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut elem_0: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_1: color::Elem = crate::color::Elem::Socket;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_2: color::Elem = crate::color::Elem::Write;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_3: color::Elem = crate::color::Elem::Group;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut option_0: std::option::Option<bool> = std::option::Option::None;
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_2: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_3: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_0: bool = false;
    let mut option_4: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_0: usize = 7151usize;
    let mut bool_1: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_1, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut option_5: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_1);
    let mut option_6: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_2: bool = false;
    let mut option_7: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut option_8: std::option::Option<bool> = std::option::Option::None;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut option_9: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::Some(dirgrouping_1);
    let mut option_10: std::option::Option<bool> = std::option::Option::None;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut option_11: std::option::Option<flags::sorting::SortColumn> = std::option::Option::Some(sortcolumn_1);
    let mut sorting_1: crate::config_file::Sorting = crate::config_file::Sorting {column: option_11, reverse: option_10, dir_grouping: option_9};
    let mut option_12: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_1);
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut option_13: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_1);
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut option_14: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_1);
    let mut usize_1: usize = 3329usize;
    let mut option_15: std::option::Option<usize> = std::option::Option::Some(usize_1);
    let mut bool_3: bool = true;
    let mut option_16: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut recursion_1: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_16, depth: option_15};
    let mut option_17: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_1);
    let mut option_18: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_4: bool = true;
    let mut option_19: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut option_20: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_21: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut display_1: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut option_22: std::option::Option<flags::display::Display> = std::option::Option::Some(display_1);
    let mut bool_5: bool = false;
    let mut option_23: std::option::Option<bool> = std::option::Option::Some(bool_5);
    let mut option_24: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_25: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_26: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_6: bool = true;
    let mut option_27: std::option::Option<bool> = std::option::Option::Some(bool_6);
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_27, blocks: option_26, color: option_25, date: option_24, dereference: option_23, display: option_22, icons: option_21, ignore_globs: option_20, indicators: option_19, layout: option_18, recursion: option_17, size: option_14, permission: option_13, sorting: option_12, no_symlink: option_8, total_size: option_7, symlink_arrow: option_6, hyperlink: option_5};
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut layout_0_ref_0: &flags::layout::Layout = &mut layout_0;
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut layout_1_ref_0: &flags::layout::Layout = &mut layout_1;
    let mut elem_4: color::Elem = crate::color::Elem::NonFile;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut bool_7: bool = crate::color::Elem::has_suid(elem_2_ref_0);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut elem_5: color::Elem = crate::color::Elem::clone(elem_1_ref_0);
    panic!("From RustyUnit with love");
}
}