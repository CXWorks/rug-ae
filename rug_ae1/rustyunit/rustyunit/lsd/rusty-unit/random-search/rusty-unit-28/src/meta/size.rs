use crate::color::{ColoredString, Colors, Elem};
use crate::flags::{Flags, SizeFlag};
use std::fs::Metadata;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Unit {
    None,
    Byte,
    Kilo,
    Mega,
    Giga,
    Tera,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Size {
    bytes: u64,
}

impl<'a> From<&'a Metadata> for Size {
    fn from(meta: &Metadata) -> Self {
        let len = meta.len();
        Self { bytes: len }
    }
}

impl Size {
    pub fn new(bytes: u64) -> Self {
        Self { bytes }
    }

    pub fn get_bytes(&self) -> u64 {
        self.bytes
    }

    fn format_size(&self, number: f64) -> String {
        format!("{0:.1$}", number, if number < 10.0 { 1 } else { 0 })
    }

    pub fn get_unit(&self, flags: &Flags) -> Unit {
        if self.bytes < 1024 || flags.size == SizeFlag::Bytes {
            Unit::Byte
        } else if self.bytes < 1024 * 1024 {
            Unit::Kilo
        } else if self.bytes < 1024 * 1024 * 1024 {
            Unit::Mega
        } else if self.bytes < 1024 * 1024 * 1024 * 1024 {
            Unit::Giga
        } else {
            Unit::Tera
        }
    }

    pub fn render(
        &self,
        colors: &Colors,
        flags: &Flags,
        val_alignment: Option<usize>,
    ) -> ColoredString {
        let val_content = self.render_value(colors, flags);
        let unit_content = self.render_unit(colors, flags);

        let left_pad = if let Some(align) = val_alignment {
            " ".repeat(align - val_content.content().len())
        } else {
            "".to_string()
        };

        let mut strings: Vec<ColoredString> = vec![
            ColoredString::new(Colors::default_style(), left_pad),
            val_content,
        ];
        if flags.size != SizeFlag::Short {
            strings.push(ColoredString::new(Colors::default_style(), " ".into()));
        }
        strings.push(unit_content);

        let res = strings
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join("");
        ColoredString::new(Colors::default_style(), res)
    }

    fn paint(&self, colors: &Colors, flags: &Flags, content: String) -> ColoredString {
        let unit = self.get_unit(flags);

        if unit == Unit::None {
            colors.colorize(content, &Elem::NonFile)
        } else if unit == Unit::Byte || unit == Unit::Kilo {
            colors.colorize(content, &Elem::FileSmall)
        } else if unit == Unit::Mega {
            colors.colorize(content, &Elem::FileMedium)
        } else {
            colors.colorize(content, &Elem::FileLarge)
        }
    }

    pub fn render_value(&self, colors: &Colors, flags: &Flags) -> ColoredString {
        let content = self.value_string(flags);

        self.paint(colors, flags, content)
    }

    pub fn value_string(&self, flags: &Flags) -> String {
        let unit = self.get_unit(flags);

        match unit {
            Unit::None => "".to_string(),
            Unit::Byte => self.bytes.to_string(),
            Unit::Kilo => self.format_size(((self.bytes as f64) / 1024.0 * 10.0).round() / 10.0),
            Unit::Mega => {
                self.format_size(((self.bytes as f64) / (1024.0 * 1024.0) * 10.0).round() / 10.0)
            }
            Unit::Giga => self.format_size(
                ((self.bytes as f64) / (1024.0 * 1024.0 * 1024.0) * 10.0).round() / 10.0,
            ),
            Unit::Tera => self.format_size(
                ((self.bytes as f64) / (1024.0 * 1024.0 * 1024.0 * 1024.0) * 10.0).round() / 10.0,
            ),
        }
    }

    pub fn render_unit(&self, colors: &Colors, flags: &Flags) -> ColoredString {
        let content = self.unit_string(flags);

        self.paint(colors, flags, content)
    }

    pub fn unit_string(&self, flags: &Flags) -> String {
        let unit = self.get_unit(flags);

        match flags.size {
            SizeFlag::Default => match unit {
                Unit::None => String::from("-"),
                Unit::Byte => String::from("B"),
                Unit::Kilo => String::from("KB"),
                Unit::Mega => String::from("MB"),
                Unit::Giga => String::from("GB"),
                Unit::Tera => String::from("TB"),
            },
            SizeFlag::Short => match unit {
                Unit::None => String::from("-"),
                Unit::Byte => String::from("B"),
                Unit::Kilo => String::from("K"),
                Unit::Mega => String::from("M"),
                Unit::Giga => String::from("G"),
                Unit::Tera => String::from("T"),
            },
            SizeFlag::Bytes => String::from(""),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Size;
    use crate::color::{Colors, ThemeOption};
    use crate::flags::{Flags, SizeFlag};

    #[test]
    fn render_byte() {
        let size = Size::new(42); // == 42 bytes
        let mut flags = Flags::default();

        assert_eq!(size.value_string(&flags).as_str(), "42");

        assert_eq!(size.unit_string(&flags).as_str(), "B");
        flags.size = SizeFlag::Short;
        assert_eq!(size.unit_string(&flags).as_str(), "B");
        flags.size = SizeFlag::Bytes;
        assert_eq!(size.unit_string(&flags).as_str(), "");
    }

    #[test]
    fn render_10_minus_kilobyte() {
        let size = Size::new(4 * 1024); // 4 kilobytes
        let mut flags = Flags::default();

        assert_eq!(size.value_string(&flags).as_str(), "4.0");
        assert_eq!(size.unit_string(&flags).as_str(), "KB");
        flags.size = SizeFlag::Short;
        assert_eq!(size.unit_string(&flags).as_str(), "K");
    }

    #[test]
    fn render_kilobyte() {
        let size = Size::new(42 * 1024); // 42 kilobytes
        let mut flags = Flags::default();

        assert_eq!(size.value_string(&flags).as_str(), "42");
        assert_eq!(size.unit_string(&flags).as_str(), "KB");
        flags.size = SizeFlag::Short;
        assert_eq!(size.unit_string(&flags).as_str(), "K");
    }

    #[test]
    fn render_100_plus_kilobyte() {
        let size = Size::new(420 * 1024 + 420); // 420.4 kilobytes
        let mut flags = Flags::default();

        assert_eq!(size.value_string(&flags).as_str(), "420");
        assert_eq!(size.unit_string(&flags).as_str(), "KB");
        flags.size = SizeFlag::Short;
        assert_eq!(size.unit_string(&flags).as_str(), "K");
    }

    #[test]
    fn render_10_minus_megabyte() {
        let size = Size::new(4 * 1024 * 1024); // 4 megabytes
        let mut flags = Flags::default();

        assert_eq!(size.value_string(&flags).as_str(), "4.0");
        assert_eq!(size.unit_string(&flags).as_str(), "MB");
        flags.size = SizeFlag::Short;
        assert_eq!(size.unit_string(&flags).as_str(), "M");
    }

    #[test]
    fn render_megabyte() {
        let size = Size::new(42 * 1024 * 1024); // 42 megabytes
        let mut flags = Flags::default();

        assert_eq!(size.value_string(&flags).as_str(), "42");
        assert_eq!(size.unit_string(&flags).as_str(), "MB");
        flags.size = SizeFlag::Short;
        assert_eq!(size.unit_string(&flags).as_str(), "M");
    }

    #[test]
    fn render_100_plus_megabyte() {
        let size = Size::new(420 * 1024 * 1024 + 420 * 1024); // 420.4 megabytes
        let mut flags = Flags::default();

        assert_eq!(size.value_string(&flags).as_str(), "420");
        assert_eq!(size.unit_string(&flags).as_str(), "MB");
        flags.size = SizeFlag::Short;
        assert_eq!(size.unit_string(&flags).as_str(), "M");
    }

    #[test]
    fn render_10_minus_gigabyte() {
        let size = Size::new(4 * 1024 * 1024 * 1024); // 4 gigabytes
        let mut flags = Flags::default();

        assert_eq!(size.value_string(&flags).as_str(), "4.0");
        assert_eq!(size.unit_string(&flags).as_str(), "GB");
        flags.size = SizeFlag::Short;
        assert_eq!(size.unit_string(&flags).as_str(), "G");
    }

    #[test]
    fn render_gigabyte() {
        let size = Size::new(42 * 1024 * 1024 * 1024); // 42 gigabytes
        let mut flags = Flags::default();

        assert_eq!(size.value_string(&flags).as_str(), "42");
        assert_eq!(size.unit_string(&flags).as_str(), "GB");
        flags.size = SizeFlag::Short;
        assert_eq!(size.unit_string(&flags).as_str(), "G");
    }

    #[test]
    fn render_100_plus_gigabyte() {
        let size = Size::new(420 * 1024 * 1024 * 1024 + 420 * 1024 * 1024); // 420.4 gigabytes
        let mut flags = Flags::default();

        assert_eq!(size.value_string(&flags).as_str(), "420");
        assert_eq!(size.unit_string(&flags).as_str(), "GB");
        flags.size = SizeFlag::Short;
        assert_eq!(size.unit_string(&flags).as_str(), "G");
    }

    #[test]
    fn render_10_minus_terabyte() {
        let size = Size::new(4 * 1024 * 1024 * 1024 * 1024); // 4 terabytes
        let mut flags = Flags::default();

        assert_eq!(size.value_string(&flags).as_str(), "4.0");
        assert_eq!(size.unit_string(&flags).as_str(), "TB");
        flags.size = SizeFlag::Short;
        assert_eq!(size.unit_string(&flags).as_str(), "T");
    }

    #[test]
    fn render_terabyte() {
        let size = Size::new(42 * 1024 * 1024 * 1024 * 1024); // 42 terabytes
        let mut flags = Flags::default();

        assert_eq!(size.value_string(&flags).as_str(), "42");
        assert_eq!(size.unit_string(&flags).as_str(), "TB");
        flags.size = SizeFlag::Short;
        assert_eq!(size.unit_string(&flags).as_str(), "T");
    }

    #[test]
    fn render_100_plus_terabyte() {
        let size = Size::new(420 * 1024 * 1024 * 1024 * 1024 + 420 * 1024 * 1024 * 1024); // 420.4 terabytes
        let mut flags = Flags::default();

        assert_eq!(size.value_string(&flags).as_str(), "420");
        assert_eq!(size.unit_string(&flags).as_str(), "TB");
        flags.size = SizeFlag::Short;
        assert_eq!(size.unit_string(&flags).as_str(), "T");
    }

    #[test]
    fn render_with_a_fraction() {
        let size = Size::new(42 * 1024 + 103); // 42.1 kilobytes
        let flags = Flags::default();

        assert_eq!(size.value_string(&flags).as_str(), "42");
        assert_eq!(size.unit_string(&flags).as_str(), "KB");
    }

    #[test]
    fn render_with_a_truncated_fraction() {
        let size = Size::new(42 * 1024 + 1); // 42.001 kilobytes == 42 kilobytes
        let flags = Flags::default();

        assert_eq!(size.value_string(&flags).as_str(), "42");
        assert_eq!(size.unit_string(&flags).as_str(), "KB");
    }

    #[test]
    fn render_short_nospaces() {
        let size = Size::new(42 * 1024); // 42 kilobytes
        let mut flags = Flags::default();
        flags.size = SizeFlag::Short;
        let colors = Colors::new(ThemeOption::NoColor);

        assert_eq!(size.render(&colors, &flags, Some(2)).to_string(), "42K");
        assert_eq!(size.render(&colors, &flags, Some(3)).to_string(), " 42K");
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
fn rusty_test_5686() {
    rusty_monitor::set_test_id(5686);
    let mut f64_0: f64 = -8.682924f64;
    let mut u64_0: u64 = 71u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut string_0: std::string::String = crate::meta::size::Size::format_size(size_0_ref_0, f64_0);
    let mut option_0: std::option::Option<std::string::String> = std::option::Option::Some(string_0);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_0: usize = 72usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_1};
    let mut config_2: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut option_1: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut option_2: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_1);
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Mega;
    let mut unit_0_ref_0: &meta::size::Unit = &mut unit_0;
    let mut theme_0: icon::Theme = crate::icon::Theme::NoIcon;
    let mut theme_0_ref_0: &icon::Theme = &mut theme_0;
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::clone(unit_0_ref_0);
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_2, theme: option_1, separator: option_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1838() {
    rusty_monitor::set_test_id(1838);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_0: usize = 73usize;
    let mut bool_0: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_2: std::option::Option<bool> = std::option::Option::None;
    let mut bool_1: bool = true;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_4: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::None;
    let mut option_5: std::option::Option<bool> = std::option::Option::None;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut option_6: std::option::Option<flags::sorting::SortColumn> = std::option::Option::Some(sortcolumn_1);
    let mut sorting_1: crate::config_file::Sorting = crate::config_file::Sorting {column: option_6, reverse: option_5, dir_grouping: option_4};
    let mut option_7: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_1);
    let mut option_8: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut option_9: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_10: std::option::Option<usize> = std::option::Option::None;
    let mut bool_2: bool = false;
    let mut option_11: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut recursion_1: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_11, depth: option_10};
    let mut option_12: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_1);
    let mut option_13: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_3: bool = false;
    let mut option_14: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut option_15: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_16: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut option_17: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut bool_4: bool = true;
    let mut option_18: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut option_19: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_20: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_21: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_5: bool = true;
    let mut option_22: std::option::Option<bool> = std::option::Option::Some(bool_5);
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_22, blocks: option_21, color: option_20, date: option_19, dereference: option_18, display: option_17, icons: option_16, ignore_globs: option_15, indicators: option_14, layout: option_13, recursion: option_12, size: option_9, permission: option_8, sorting: option_7, no_symlink: option_3, total_size: option_2, symlink_arrow: option_1, hyperlink: option_0};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut display_1: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut u64_0: u64 = 19u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut u64_1: u64 = 68u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_1};
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size::clone(size_1_ref_0);
    let mut size_2_ref_0: &crate::meta::size::Size = &mut size_2;
    let mut bool_6: bool = crate::meta::size::Size::ne(size_2_ref_0, size_0_ref_0);
    let mut elem_0: color::Elem = crate::color::Elem::Socket;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4687() {
    rusty_monitor::set_test_id(4687);
    let mut usize_0: usize = 78usize;
    let mut bool_0: bool = false;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut option_0: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_1: bool = true;
    let mut option_1: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut bool_2: bool = false;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut option_3: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::Some(dirgrouping_0);
    let mut bool_3: bool = true;
    let mut option_4: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut option_5: std::option::Option<flags::sorting::SortColumn> = std::option::Option::Some(sortcolumn_0);
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_5, reverse: option_4, dir_grouping: option_3};
    let mut option_6: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_0);
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut option_7: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_0);
    let mut option_8: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_9: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_10: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut option_11: std::option::Option<bool> = std::option::Option::None;
    let mut option_12: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_13: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_14: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut bool_4: bool = true;
    let mut option_15: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut option_16: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_17: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_18: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_19: std::option::Option<bool> = std::option::Option::None;
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_1: usize = 7usize;
    let mut bool_5: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_5, depth: usize_1};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Socket;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Pipe;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::NonFile;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Tera;
    let mut unit_0_ref_0: &meta::size::Unit = &mut unit_0;
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::Giga;
    let mut unit_1_ref_0: &meta::size::Unit = &mut unit_1;
    let mut unit_2: meta::size::Unit = crate::meta::size::Unit::clone(unit_1_ref_0);
    let mut unit_2_ref_0: &meta::size::Unit = &mut unit_2;
    let mut bool_6: bool = crate::meta::size::Unit::eq(unit_2_ref_0, unit_0_ref_0);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut bool_7: bool = crate::meta::filetype::FileType::is_dirlike(filetype_0);
    let mut file_0: crate::color::theme::File = crate::color::theme::File {exec_uid: color_4, uid_no_exec: color_3, exec_no_uid: color_2, no_exec_no_uid: color_1};
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1425() {
    rusty_monitor::set_test_id(1425);
    let mut usize_0: usize = 15usize;
    let mut bool_0: bool = true;
    let mut usize_1: usize = 0usize;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_2: usize = 61usize;
    let mut bool_1: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_1, depth: usize_2};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut str_0: &str = "TS2GnLXocw1n";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_1, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_3: usize = 22usize;
    let mut bool_2: bool = false;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_2, depth: usize_3};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_1: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_1};
    let mut u64_0: u64 = 16u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_2: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_2, order: sortorder_1, dir_grouping: dirgrouping_1};
    let mut permissionflag_2: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_2: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_4: usize = 55usize;
    let mut bool_3: bool = true;
    let mut recursion_2: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_3, depth: usize_4};
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut icontheme_2: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_2: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_2: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_2: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_2: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_2: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_2, theme: themeoption_2};
    let mut u64_1: u64 = 35u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut option_0: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut option_1: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut color_3: crate::config_file::Color = crate::config_file::Color {when: option_1, theme: option_0};
    let mut option_2: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_3);
    let mut option_3: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_4: bool = false;
    let mut option_4: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_3);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut sizeflag_3: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut sizeflag_3_ref_0: &flags::size::SizeFlag = &mut sizeflag_3;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Date;
    let mut block_0_ref_0: &flags::blocks::Block = &mut block_0;
    let mut elem_0: color::Elem = crate::color::Elem::Special;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Kilo;
    let mut app_0: clap::App = crate::app::build();
    let mut unit_0_ref_0: &meta::size::Unit = &mut unit_0;
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::clone(unit_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_474() {
    rusty_monitor::set_test_id(474);
    let mut bool_0: bool = false;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut bool_1: bool = true;
    let mut elem_0: color::Elem = crate::color::Elem::INode {valid: bool_1};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Socket;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut bool_2: bool = true;
    let mut bool_3: bool = false;
    let mut elem_2: color::Elem = crate::color::Elem::File {exec: bool_3, uid: bool_2};
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_4: color::Elem = crate::color::Elem::NonFile;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut elem_5: color::Elem = crate::color::Elem::SymLink;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_5_ref_0);
    let mut theme_6: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_6_ref_0: &crate::color::theme::Theme = &mut theme_6;
    let mut elem_6: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut color_6: crossterm::style::Color = crate::color::Elem::get_color(elem_6_ref_0, theme_6_ref_0);
    let mut theme_7: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_7_ref_0: &crate::color::theme::Theme = &mut theme_7;
    let mut bool_4: bool = false;
    let mut elem_7: color::Elem = crate::color::Elem::Links {valid: bool_4};
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut color_7: crossterm::style::Color = crate::color::Elem::get_color(elem_7_ref_0, theme_7_ref_0);
    let mut u64_0: u64 = 65u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_0};
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut tuple_0: () = crate::meta::size::Size::assert_receiver_is_total_eq(size_0_ref_0);
    let mut permission_0: crate::color::theme::Permission = crate::color::theme::Permission {read: color_7, write: color_6, exec: color_5, exec_sticky: color_4, no_access: color_3, octal: color_2, acl: color_1, context: color_0};
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3801() {
    rusty_monitor::set_test_id(3801);
    let mut str_0: &str = "vmpYq";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u64_0: u64 = 92u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut u64_1: u64 = 67u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut bool_3: bool = false;
    let mut bool_4: bool = false;
    let mut bool_5: bool = true;
    let mut bool_6: bool = false;
    let mut bool_7: bool = false;
    let mut bool_8: bool = true;
    let mut bool_9: bool = false;
    let mut bool_10: bool = true;
    let mut bool_11: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut bool_12: bool = false;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut u64_2: u64 = 78u64;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size::new(u64_2);
    let mut size_2_ref_0: &crate::meta::size::Size = &mut size_2;
    let mut u64_3: u64 = 28u64;
    let mut size_3: crate::meta::size::Size = crate::meta::size::Size::new(u64_3);
    let mut size_3_ref_0: &crate::meta::size::Size = &mut size_3;
    let mut bool_13: bool = crate::meta::size::Size::ne(size_3_ref_0, size_2_ref_0);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut displayoption_0: meta::name::DisplayOption = crate::meta::name::DisplayOption::FileName;
    let mut bool_14: bool = crate::meta::permissions::Permissions::is_executable(permissions_0_ref_0);
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut bool_15: bool = crate::meta::size::Size::eq(size_1_ref_0, size_0_ref_0);
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut elem_0: color::Elem = crate::color::Elem::Group;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut option_0: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1718() {
    rusty_monitor::set_test_id(1718);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::None;
    let mut unit_0_ref_0: &meta::size::Unit = &mut unit_0;
    let mut u64_0: u64 = 41u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_0};
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut bool_0: bool = true;
    let mut f64_0: f64 = -5.954399f64;
    let mut u64_1: u64 = 26u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_1};
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut string_0: std::string::String = crate::meta::size::Size::format_size(size_1_ref_0, f64_0);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Formatted(string_0);
    let mut dateflag_0_ref_0: &flags::date::DateFlag = &mut dateflag_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut option_0: std::option::Option<std::vec::Vec<crate::meta::Meta>> = std::option::Option::None;
    let mut u64_2: u64 = 19u64;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size::new(u64_2);
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut bool_1: bool = true;
    let mut bool_2: bool = false;
    let mut bool_3: bool = true;
    let mut bool_4: bool = false;
    let mut bool_5: bool = true;
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut bool_8: bool = false;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut bool_12: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_12, user_write: bool_11, user_execute: bool_10, group_read: bool_9, group_write: bool_8, group_execute: bool_7, other_read: bool_6, other_write: bool_5, other_execute: bool_4, sticky: bool_3, setgid: bool_2, setuid: bool_1};
    let mut bool_13: bool = false;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_13};
    let mut elem_0: color::Elem = crate::color::Elem::Links {valid: bool_0};
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut size_2_ref_0: &crate::meta::size::Size = &mut size_2;
    let mut bool_14: bool = crate::meta::size::Size::eq(size_2_ref_0, size_0_ref_0);
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::Mega;
    let mut tuple_0: () = crate::meta::size::Unit::assert_receiver_is_total_eq(unit_0_ref_0);
    crate::meta::filetype::FileType::render(filetype_0, colors_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3649() {
    rusty_monitor::set_test_id(3649);
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_0: usize = 75usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_0);
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_1: bool = true;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut bool_2: bool = false;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut option_4: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut option_5: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut option_6: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_1);
    let mut option_7: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_8: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_3: bool = false;
    let mut option_9: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut option_10: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_11: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut display_1: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut option_12: std::option::Option<flags::display::Display> = std::option::Option::Some(display_1);
    let mut option_13: std::option::Option<bool> = std::option::Option::None;
    let mut option_14: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_15: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut option_16: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_1);
    let mut color_1: crate::config_file::Color = crate::config_file::Color {when: option_16, theme: option_15};
    let mut option_17: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_1);
    let mut option_18: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_4: bool = false;
    let mut option_19: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut config_2: crate::config_file::Config = crate::config_file::Config {classic: option_19, blocks: option_18, color: option_17, date: option_14, dereference: option_13, display: option_12, icons: option_11, ignore_globs: option_10, indicators: option_9, layout: option_8, recursion: option_7, size: option_6, permission: option_5, sorting: option_4, no_symlink: option_3, total_size: option_2, symlink_arrow: option_1, hyperlink: option_0};
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut str_0: &str = "l3InEZ7gObaB";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_5: bool = false;
    let mut bool_6: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_6, exec: bool_5};
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = true;
    let mut bool_12: bool = false;
    let mut bool_13: bool = true;
    let mut bool_14: bool = true;
    let mut bool_15: bool = false;
    let mut bool_16: bool = false;
    let mut bool_17: bool = true;
    let mut bool_18: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_18, user_write: bool_17, user_execute: bool_16, group_read: bool_15, group_write: bool_14, group_execute: bool_13, other_read: bool_12, other_write: bool_11, other_execute: bool_10, sticky: bool_9, setgid: bool_8, setuid: bool_7};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_20: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Byte;
    let mut unit_0_ref_0: &meta::size::Unit = &mut unit_0;
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::clone(unit_0_ref_0);
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut elem_0: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::INode;
    let mut sizeflag_2: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4178() {
    rusty_monitor::set_test_id(4178);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Write;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::NonFile;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::Acl;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::User;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_4: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut elem_5: color::Elem = crate::color::Elem::HourOld;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_5_ref_0);
    let mut theme_6: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_6_ref_0: &crate::color::theme::Theme = &mut theme_6;
    let mut elem_6: color::Elem = crate::color::Elem::Acl;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut color_6: crossterm::style::Color = crate::color::Elem::get_color(elem_6_ref_0, theme_6_ref_0);
    let mut theme_7: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_7_ref_0: &crate::color::theme::Theme = &mut theme_7;
    let mut elem_7: color::Elem = crate::color::Elem::ExecSticky;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut color_7: crossterm::style::Color = crate::color::Elem::get_color(elem_7_ref_0, theme_7_ref_0);
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Byte;
    let mut unit_0_ref_0: &meta::size::Unit = &mut unit_0;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_1_ref_0: &meta::date::Date = &mut date_1;
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::Tera;
    let mut unit_1_ref_0: &meta::size::Unit = &mut unit_1;
    let mut unit_2: meta::size::Unit = crate::meta::size::Unit::clone(unit_1_ref_0);
    let mut unit_2_ref_0: &meta::size::Unit = &mut unit_2;
    let mut bool_0: bool = crate::meta::size::Unit::eq(unit_2_ref_0, unit_0_ref_0);
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut permission_0: crate::color::theme::Permission = crate::color::theme::Permission {read: color_7, write: color_6, exec: color_5, exec_sticky: color_4, no_access: color_3, octal: color_2, acl: color_1, context: color_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4909() {
    rusty_monitor::set_test_id(4909);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_0: usize = 42usize;
    let mut bool_0: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_1);
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_1: bool = true;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut bool_2: bool = false;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut option_4: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut option_5: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_1);
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut option_6: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_1);
    let mut usize_1: usize = 22usize;
    let mut option_7: std::option::Option<usize> = std::option::Option::Some(usize_1);
    let mut option_8: std::option::Option<bool> = std::option::Option::None;
    let mut recursion_1: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_8, depth: option_7};
    let mut option_9: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_1);
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut option_10: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_1);
    let mut bool_3: bool = false;
    let mut option_11: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut option_12: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_13: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_14: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut option_15: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_15, theme: option_14, separator: option_13};
    let mut option_16: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut option_17: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_18: std::option::Option<bool> = std::option::Option::None;
    let mut option_19: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_20: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut option_21: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_21, theme: option_20};
    let mut option_22: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_23: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_24: std::option::Option<bool> = std::option::Option::None;
    let mut config_1: crate::config_file::Config = crate::config_file::Config {classic: option_24, blocks: option_23, color: option_22, date: option_19, dereference: option_18, display: option_17, icons: option_16, ignore_globs: option_12, indicators: option_11, layout: option_10, recursion: option_9, size: option_6, permission: option_5, sorting: option_4, no_symlink: option_3, total_size: option_2, symlink_arrow: option_1, hyperlink: option_0};
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut u64_0: u64 = 78u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_0};
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut str_0: &str = "A3T";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_4: bool = false;
    let mut bool_5: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_5, exec: bool_4};
    let mut u64_1: u64 = 1u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut u64_2: u64 = 52u64;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_2};
    let mut size_2_ref_0: &crate::meta::size::Size = &mut size_2;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::None;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut unit_0_ref_0: &meta::size::Unit = &mut unit_0;
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::clone(unit_0_ref_0);
    let mut bool_6: bool = crate::meta::size::Size::eq(size_2_ref_0, size_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5630() {
    rusty_monitor::set_test_id(5630);
    let mut bool_0: bool = false;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut elem_0: color::Elem = crate::color::Elem::Write;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut bool_1: bool = true;
    let mut bool_2: bool = false;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut bool_5: bool = true;
    let mut bool_6: bool = false;
    let mut bool_7: bool = false;
    let mut bool_8: bool = true;
    let mut bool_9: bool = false;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut bool_12: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_12, user_write: bool_11, user_execute: bool_10, group_read: bool_9, group_write: bool_8, group_execute: bool_7, other_read: bool_6, other_write: bool_5, other_execute: bool_4, sticky: bool_3, setgid: bool_2, setuid: bool_1};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut f64_0: f64 = 64.437824f64;
    let mut u64_0: u64 = 90u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut string_0: std::string::String = crate::meta::size::Size::format_size(size_0_ref_0, f64_0);
    let mut theme_0: icon::Theme = crate::icon::Theme::NoIcon;
    let mut icons_0: crate::icon::Icons = crate::icon::Icons::new(theme_0, string_0);
    let mut bool_13: bool = crate::color::Elem::has_suid(elem_0_ref_0);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Date;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut elem_1: color::Elem = crate::color::Elem::FileSmall;
    let mut contentstyle_0: crossterm::style::ContentStyle = crate::color::Colors::default_style();
    crate::meta::filetype::FileType::render(filetype_0, colors_0_ref_0);
    let mut elem_2: color::Elem = crate::color::Elem::Exec;
    let mut elem_3: color::Elem = crate::color::Elem::Links {valid: bool_0};
    panic!("From RustyUnit with love");
}
}