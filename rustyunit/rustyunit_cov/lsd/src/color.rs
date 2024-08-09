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
    File { exec: bool, uid: bool },
    SymLink,
    BrokenSymLink,
    MissingSymLinkTarget,
    Dir { uid: bool },
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
    INode { valid: bool },
    Links { valid: bool },
    TreeEdge,
}
impl Elem {
    pub fn has_suid(&self) -> bool {
        matches!(self, Elem::Dir { uid : true } | Elem::File { uid : true, .. })
    }
    pub fn get_color(&self, theme: &theme::Theme) -> Color {
        match self {
            Elem::File { exec: true, uid: true } => theme.file_type.file.exec_uid,
            Elem::File { exec: false, uid: true } => theme.file_type.file.uid_no_exec,
            Elem::File { exec: true, uid: false } => theme.file_type.file.exec_no_uid,
            Elem::File { exec: false, uid: false } => theme.file_type.file.no_exec_no_uid,
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
            ThemeOption::Custom(ref file) => {
                Some(Theme::from_path(file).unwrap_or_default())
            }
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
    pub fn colorize_using_path(
        &self,
        input: String,
        path: &Path,
        elem: &Elem,
    ) -> ColoredString {
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
            Some(lscolors) => {
                match self.get_indicator_from_elem(elem) {
                    Some(style) => {
                        let style = lscolors.style_for_indicator(style);
                        style.map(to_content_style).unwrap_or_default()
                    }
                    None => self.style_default(elem),
                }
            }
            None => self.style_default(elem),
        }
    }
    fn style_default(&self, elem: &Elem) -> ContentStyle {
        if let Some(t) = &self.theme {
            let style_fg = ContentStyle::default().with(elem.get_color(t));
            if elem.has_suid() { style_fg.on(Color::AnsiValue(124)) } else { style_fg }
        } else {
            ContentStyle::default()
        }
    }
    fn get_indicator_from_elem(&self, elem: &Elem) -> Option<Indicator> {
        let indicator_string = match elem {
            Elem::File { exec, uid } => {
                match (exec, uid) {
                    (_, true) => None,
                    (true, false) => Some("ex"),
                    (false, false) => Some("fi"),
                }
            }
            Elem::Dir { uid } => if *uid { None } else { Some("di") }
            Elem::SymLink => Some("ln"),
            Elem::Pipe => Some("pi"),
            Elem::Socket => Some("so"),
            Elem::BlockDevice => Some("bd"),
            Elem::CharDevice => Some("cd"),
            Elem::BrokenSymLink => Some("or"),
            Elem::MissingSymLinkTarget => Some("mi"),
            Elem::INode { valid } => {
                match valid {
                    true => Some("so"),
                    false => Some("no"),
                }
            }
            Elem::Links { valid } => {
                match valid {
                    true => Some("so"),
                    false => Some("no"),
                }
            }
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
        lscolors::style::Color::RGB(r, g, b) => Color::Rgb { r: *r, g: *g, b: *b },
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
            Colors::new(ThemeOption::Default).theme, Some(Theme::default_dark()),
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
            user: Color::AnsiValue(230),
            group: Color::AnsiValue(187),
            permission: theme::Permission {
                read: Color::Green,
                write: Color::Yellow,
                exec: Color::Red,
                exec_sticky: Color::Magenta,
                no_access: Color::AnsiValue(245),
                octal: Color::AnsiValue(6),
                acl: Color::DarkCyan,
                context: Color::Cyan,
            },
            file_type: theme::FileType {
                file: theme::File {
                    exec_uid: Color::AnsiValue(40),
                    uid_no_exec: Color::AnsiValue(184),
                    exec_no_uid: Color::AnsiValue(40),
                    no_exec_no_uid: Color::AnsiValue(184),
                },
                dir: theme::Dir {
                    uid: Color::AnsiValue(33),
                    no_uid: Color::AnsiValue(33),
                },
                pipe: Color::AnsiValue(44),
                symlink: theme::Symlink {
                    default: Color::AnsiValue(44),
                    broken: Color::AnsiValue(124),
                    missing_target: Color::AnsiValue(124),
                },
                block_device: Color::AnsiValue(44),
                char_device: Color::AnsiValue(172),
                socket: Color::AnsiValue(44),
                special: Color::AnsiValue(44),
            },
            date: theme::Date {
                hour_old: Color::AnsiValue(40),
                day_old: Color::AnsiValue(42),
                older: Color::AnsiValue(36),
            },
            size: theme::Size {
                none: Color::AnsiValue(245),
                small: Color::AnsiValue(229),
                medium: Color::AnsiValue(216),
                large: Color::AnsiValue(172),
            },
            inode: theme::INode {
                valid: Color::AnsiValue(13),
                invalid: Color::AnsiValue(245),
            },
            links: theme::Links {
                valid: Color::AnsiValue(13),
                invalid: Color::AnsiValue(245),
            },
            tree_edge: Color::AnsiValue(245),
        }
    }
    #[test]
    fn test_default_theme_color() {
        assert_eq!(
            Elem::File { exec : true, uid : true } .get_color(& test_theme()),
            Color::AnsiValue(40),
        );
        assert_eq!(
            Elem::File { exec : false, uid : true } .get_color(& test_theme()),
            Color::AnsiValue(184),
        );
        assert_eq!(
            Elem::File { exec : true, uid : false } .get_color(& test_theme()),
            Color::AnsiValue(40),
        );
        assert_eq!(
            Elem::File { exec : false, uid : false } .get_color(& test_theme()),
            Color::AnsiValue(184),
        );
    }
}
#[cfg(test)]
mod tests_llm_16_144 {
    use super::*;
    use crate::*;
    use crate::color::ContentStyle;
    #[test]
    fn test_default_style() {
        let _rug_st_tests_llm_16_144_rrrruuuugggg_test_default_style = 0;
        let result = Colors::default_style();
        debug_assert_eq!(result, ContentStyle::default());
        let _rug_ed_tests_llm_16_144_rrrruuuugggg_test_default_style = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_149 {
    use super::*;
    use crate::*;
    use std::path::Path;
    #[test]
    fn test_style_with_lscolors_some_and_style_some() {
        let _rug_st_tests_llm_16_149_rrrruuuugggg_test_style_with_lscolors_some_and_style_some = 0;
        let rug_fuzz_0 = true;
        let rug_fuzz_1 = true;
        let theme = Theme::default();
        let lscolors = LsColors::default();
        let colors = Colors {
            theme: Some(theme),
            lscolors: Some(lscolors),
        };
        let elem = Elem::File {
            exec: rug_fuzz_0,
            uid: rug_fuzz_1,
        };
        let result = colors.style(&elem);
        let _rug_ed_tests_llm_16_149_rrrruuuugggg_test_style_with_lscolors_some_and_style_some = 0;
    }
    #[test]
    fn test_style_with_lscolors_some_and_style_none() {
        let _rug_st_tests_llm_16_149_rrrruuuugggg_test_style_with_lscolors_some_and_style_none = 0;
        let rug_fuzz_0 = true;
        let rug_fuzz_1 = false;
        let theme = Theme::default();
        let lscolors = LsColors::default();
        let colors = Colors {
            theme: Some(theme),
            lscolors: Some(lscolors),
        };
        let elem = Elem::File {
            exec: rug_fuzz_0,
            uid: rug_fuzz_1,
        };
        let result = colors.style(&elem);
        let _rug_ed_tests_llm_16_149_rrrruuuugggg_test_style_with_lscolors_some_and_style_none = 0;
    }
    #[test]
    fn test_style_with_lscolors_some_and_no_matching_style() {
        let _rug_st_tests_llm_16_149_rrrruuuugggg_test_style_with_lscolors_some_and_no_matching_style = 0;
        let theme = Theme::default();
        let lscolors = LsColors::default();
        let colors = Colors {
            theme: Some(theme),
            lscolors: Some(lscolors),
        };
        let elem = Elem::SymLink;
        let result = colors.style(&elem);
        let _rug_ed_tests_llm_16_149_rrrruuuugggg_test_style_with_lscolors_some_and_no_matching_style = 0;
    }
    #[test]
    fn test_style_with_lscolors_none() {
        let _rug_st_tests_llm_16_149_rrrruuuugggg_test_style_with_lscolors_none = 0;
        let theme = Theme::default();
        let colors = Colors {
            theme: Some(theme),
            lscolors: None,
        };
        let elem = Elem::SymLink;
        let result = colors.style(&elem);
        let _rug_ed_tests_llm_16_149_rrrruuuugggg_test_style_with_lscolors_none = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_152 {
    use super::*;
    use crate::*;
    use std::path::Path;
    use crate::color::Colors;
    #[test]
    fn test_style_from_path() {
        let _rug_st_tests_llm_16_152_rrrruuuugggg_test_style_from_path = 0;
        let rug_fuzz_0 = "test.txt";
        let colors = Colors::new(ThemeOption::Default);
        let path = Path::new(rug_fuzz_0);
        debug_assert_eq!(colors.style_from_path(& path), None);
        let _rug_ed_tests_llm_16_152_rrrruuuugggg_test_style_from_path = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_155 {
    use super::*;
    use crate::*;
    #[test]
    fn test_has_suid() {
        let _rug_st_tests_llm_16_155_rrrruuuugggg_test_has_suid = 0;
        let rug_fuzz_0 = true;
        let rug_fuzz_1 = true;
        let rug_fuzz_2 = true;
        let rug_fuzz_3 = false;
        let rug_fuzz_4 = true;
        let rug_fuzz_5 = false;
        let file_with_suid = Elem::File {
            exec: rug_fuzz_0,
            uid: rug_fuzz_1,
        };
        let file_without_suid = Elem::File {
            exec: rug_fuzz_2,
            uid: rug_fuzz_3,
        };
        let dir_with_suid = Elem::Dir { uid: rug_fuzz_4 };
        let dir_without_suid = Elem::Dir { uid: rug_fuzz_5 };
        debug_assert_eq!(file_with_suid.has_suid(), true);
        debug_assert_eq!(file_without_suid.has_suid(), false);
        debug_assert_eq!(dir_with_suid.has_suid(), true);
        debug_assert_eq!(dir_without_suid.has_suid(), false);
        let _rug_ed_tests_llm_16_155_rrrruuuugggg_test_has_suid = 0;
    }
}
#[cfg(test)]
mod tests_rug_3 {
    use super::*;
    use lscolors::style::{Color, FontStyle};
    use lscolors::Style;
    use crossterm::style::{Color as ContentColor, Attribute};
    #[test]
    fn test_to_content_style() {
        let _rug_st_tests_rug_3_rrrruuuugggg_test_to_content_style = 0;
        let rug_fuzz_0 = 128;
        let rug_fuzz_1 = 128;
        let rug_fuzz_2 = 128;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = true;
        let rug_fuzz_7 = false;
        let rug_fuzz_8 = true;
        let rug_fuzz_9 = false;
        let rug_fuzz_10 = true;
        let rug_fuzz_11 = false;
        let rug_fuzz_12 = true;
        let rug_fuzz_13 = false;
        let rug_fuzz_14 = true;
        let mut p0 = Style {
            foreground: Some(Color::RGB(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)),
            background: Some(Color::RGB(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)),
            font_style: FontStyle {
                bold: rug_fuzz_6,
                dimmed: rug_fuzz_7,
                italic: rug_fuzz_8,
                underline: rug_fuzz_9,
                rapid_blink: rug_fuzz_10,
                slow_blink: rug_fuzz_11,
                reverse: rug_fuzz_12,
                hidden: rug_fuzz_13,
                strikethrough: rug_fuzz_14,
            },
        };
        to_content_style(&p0);
        let _rug_ed_tests_rug_3_rrrruuuugggg_test_to_content_style = 0;
    }
}
#[cfg(test)]
mod tests_rug_4 {
    use super::*;
    use crate::color::{Elem, theme::Theme};
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_4_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = true;
        let rug_fuzz_1 = true;
        let mut p0 = Elem::File {
            exec: rug_fuzz_0,
            uid: rug_fuzz_1,
        };
        let mut p1 = Theme::default();
        crate::color::Elem::get_color(&p0, &p1);
        let _rug_ed_tests_rug_4_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_5 {
    use super::*;
    use crate::flags::ThemeOption;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_5_rrrruuuugggg_test_rug = 0;
        let mut p0 = ThemeOption::NoColor;
        Colors::new(p0);
        let _rug_ed_tests_rug_5_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_9 {
    use super::*;
    use crate::color::{Colors, Elem, Indicator};
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_9_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = true;
        let rug_fuzz_1 = false;
        let mut v5 = Colors::new(ThemeOption::Default);
        let mut p0 = &v5;
        let p1 = &Elem::File {
            exec: rug_fuzz_0,
            uid: rug_fuzz_1,
        };
        let _ = p0.get_indicator_from_elem(p1);
        let _rug_ed_tests_rug_9_rrrruuuugggg_test_rug = 0;
    }
}
