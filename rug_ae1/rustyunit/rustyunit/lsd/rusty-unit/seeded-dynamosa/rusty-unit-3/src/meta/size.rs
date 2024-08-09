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
	use std::cmp::PartialEq;
	use std::clone::Clone;
	use std::cmp::Eq;
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_419() {
//    rusty_monitor::set_test_id(419);
    let mut u64_0: u64 = 1073741824u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut u64_1: u64 = 1024u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_1};
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut u64_2: u64 = 0u64;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size::new(u64_2);
    let mut size_2_ref_0: &crate::meta::size::Size = &mut size_2;
    let mut u64_3: u64 = 24u64;
    let mut size_3: crate::meta::size::Size = crate::meta::size::Size::new(u64_3);
    let mut size_3_ref_0: &crate::meta::size::Size = &mut size_3;
    let mut u64_4: u64 = 1024u64;
    let mut size_4: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_4};
    let mut size_4_ref_0: &crate::meta::size::Size = &mut size_4;
    let mut u64_5: u64 = 1073741824u64;
    let mut size_5: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_5};
    let mut size_5_ref_0: &crate::meta::size::Size = &mut size_5;
    let mut u64_6: u64 = 34u64;
    let mut size_6: crate::meta::size::Size = crate::meta::size::Size::new(u64_6);
    let mut size_6_ref_0: &crate::meta::size::Size = &mut size_6;
    let mut u64_7: u64 = 1099511627776u64;
    let mut size_7: crate::meta::size::Size = crate::meta::size::Size::new(u64_7);
    let mut size_7_ref_0: &crate::meta::size::Size = &mut size_7;
    let mut u64_8: u64 = 1048576u64;
    let mut size_8: crate::meta::size::Size = crate::meta::size::Size::new(u64_8);
    let mut size_8_ref_0: &crate::meta::size::Size = &mut size_8;
    let mut u64_9: u64 = 34u64;
    let mut size_9: crate::meta::size::Size = crate::meta::size::Size::new(u64_9);
    let mut size_9_ref_0: &crate::meta::size::Size = &mut size_9;
    let mut bool_0: bool = crate::meta::size::Size::ne(size_9_ref_0, size_8_ref_0);
    let mut bool_1: bool = crate::meta::size::Size::ne(size_7_ref_0, size_6_ref_0);
    let mut bool_2: bool = crate::meta::size::Size::ne(size_5_ref_0, size_4_ref_0);
    let mut bool_3: bool = crate::meta::size::Size::ne(size_3_ref_0, size_2_ref_0);
    let mut bool_4: bool = crate::meta::size::Size::ne(size_1_ref_0, size_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3991() {
//    rusty_monitor::set_test_id(3991);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut bool_2: bool = false;
    let mut bool_3: bool = false;
    let mut bool_4: bool = false;
    let mut bool_5: bool = false;
    let mut bool_6: bool = false;
    let mut bool_7: bool = true;
    let mut bool_8: bool = false;
    let mut bool_9: bool = true;
    let mut bool_10: bool = true;
    let mut bool_11: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut str_0: &str = ".";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut f64_0: f64 = 4621819117588971520.000000f64;
    let mut u64_0: u64 = 25u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_0};
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut string_0: std::string::String = crate::meta::size::Size::format_size(size_0_ref_0, f64_0);
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut option_1: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_1);
    let mut option_2: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_12: bool = false;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_12);
    let mut option_4: std::option::Option<bool> = std::option::Option::None;
    let mut option_5: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut option_6: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_0);
    let mut option_7: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    crate::meta::filetype::FileType::render(filetype_0, colors_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2202() {
//    rusty_monitor::set_test_id(2202);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_0: usize = 8usize;
    let mut bool_0: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut u64_0: u64 = 86u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_0};
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut display_1: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut display_1_ref_0: &flags::display::Display = &mut display_1;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Kilo;
    let mut unit_0_ref_0: &meta::size::Unit = &mut unit_0;
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::Giga;
    let mut unit_2: meta::size::Unit = crate::meta::size::Unit::None;
    let mut unit_2_ref_0: &meta::size::Unit = &mut unit_2;
    let mut bool_1: bool = crate::meta::size::Unit::eq(unit_2_ref_0, unit_0_ref_0);
    let mut display_2: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut display_2_ref_0: &flags::display::Display = &mut display_2;
    let mut theme_0: icon::Theme = crate::icon::Theme::NoIcon;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3814() {
//    rusty_monitor::set_test_id(3814);
    let mut bool_0: bool = false;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Mega;
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::None;
    let mut unit_0_ref_0: &meta::size::Unit = &mut unit_0;
    let mut unit_2: meta::size::Unit = crate::meta::size::Unit::Tera;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut u64_0: u64 = 1099511627776u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut u64_1: u64 = 60u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_2, exec: bool_1};
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut str_0: &str = "webp";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut display_1: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut display_0_ref_0: &flags::display::Display = &mut display_0;
    let mut unit_2_ref_0: &meta::size::Unit = &mut unit_2;
    let mut unit_3: meta::size::Unit = crate::meta::size::Unit::clone(unit_0_ref_0);
    let mut unit_4: meta::size::Unit = crate::meta::size::Unit::Kilo;
    let mut elem_0: color::Elem = crate::color::Elem::INode {valid: bool_0};
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_396() {
//    rusty_monitor::set_test_id(396);
    let mut f64_0: f64 = 4742290407621132288.000000f64;
    let mut u64_0: u64 = 10u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut string_0: std::string::String = crate::meta::size::Size::format_size(size_0_ref_0, f64_0);
    let mut f64_1: f64 = 26.085616f64;
    let mut u64_1: u64 = 1073741824u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut string_1: std::string::String = crate::meta::size::Size::format_size(size_1_ref_0, f64_1);
    let mut option_0: std::option::Option<std::vec::Vec<crate::meta::Meta>> = std::option::Option::None;
    let mut u64_2: u64 = 1024u64;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size::new(u64_2);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = false;
    let mut bool_5: bool = false;
    let mut bool_6: bool = true;
    let mut bool_7: bool = false;
    let mut bool_8: bool = false;
    let mut bool_9: bool = true;
    let mut bool_10: bool = true;
    let mut bool_11: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut bool_12: bool = true;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_12};
    let mut owner_0: crate::meta::owner::Owner = crate::meta::owner::Owner::new(string_1, string_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8442() {
//    rusty_monitor::set_test_id(8442);
    let mut elem_0: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut bool_0: bool = true;
    let mut elem_1: color::Elem = crate::color::Elem::INode {valid: bool_0};
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_2: color::Elem = crate::color::Elem::HourOld;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_3: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut bool_1: bool = false;
    let mut elem_4: color::Elem = crate::color::Elem::Dir {uid: bool_1};
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_5: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_6: color::Elem = crate::color::Elem::Exec;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_6_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut elem_7: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_7_ref_0, theme_5_ref_0);
    let mut theme_6: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_6_ref_0: &crate::color::theme::Theme = &mut theme_6;
    let mut elem_8: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut color_6: crossterm::style::Color = crate::color::Elem::get_color(elem_8_ref_0, theme_6_ref_0);
    let mut theme_7: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_7_ref_0: &crate::color::theme::Theme = &mut theme_7;
    let mut elem_9: color::Elem = crate::color::Elem::Older;
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    let mut color_7: crossterm::style::Color = crate::color::Elem::get_color(elem_9_ref_0, theme_7_ref_0);
    let mut elem_10: color::Elem = crate::color::Elem::Exec;
    let mut bool_2: bool = false;
    let mut bool_3: bool = true;
    let mut option_0: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_0: &str = "procfile";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_1: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut elem_11: color::Elem = crate::color::Elem::Octal;
    let mut elem_11_ref_0: &color::Elem = &mut elem_11;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::SizeValue;
    let mut block_0_ref_0: &flags::blocks::Block = &mut block_0;
    let mut u64_0: u64 = 4u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut elem_12: color::Elem = crate::color::Elem::FileMedium;
    let mut u64_1: u64 = crate::meta::size::Size::get_bytes(size_0_ref_0);
    let mut block_1: flags::blocks::Block = crate::flags::blocks::Block::SizeValue;
    let mut block_1_ref_0: &flags::blocks::Block = &mut block_1;
    let mut elem_12_ref_0: &color::Elem = &mut elem_12;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Mega;
    let mut unit_0_ref_0: &meta::size::Unit = &mut unit_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut elem_13: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_10_ref_0: &color::Elem = &mut elem_10;
    let mut elem_14: color::Elem = crate::color::Elem::Acl;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_3, exec: bool_2};
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::None;
    let mut permission_0: crate::color::theme::Permission = crate::color::theme::Permission {read: color_7, write: color_6, exec: color_5, exec_sticky: color_4, no_access: color_3, octal: color_2, acl: color_1, context: color_0};
    let mut unit_1_ref_0: &meta::size::Unit = &mut unit_1;
    let mut unit_2: meta::size::Unit = crate::meta::size::Unit::clone(unit_1_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4843() {
//    rusty_monitor::set_test_id(4843);
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Giga;
    let mut unit_0_ref_0: &meta::size::Unit = &mut unit_0;
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::Mega;
    let mut str_0: &str = "歷";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut usize_0: usize = 8usize;
    let mut option_0: std::option::Option<usize> = std::option::Option::Some(usize_0);
    let mut option_1: std::option::Option<bool> = std::option::Option::None;
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_1, depth: option_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut option_2: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_0);
    let mut option_3: std::option::Option<bool> = std::option::Option::None;
    let mut option_4: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_5: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_6: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_7: std::option::Option<bool> = std::option::Option::None;
    let mut option_8: std::option::Option<std::string::String> = std::option::Option::None;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut unit_2: meta::size::Unit = crate::meta::size::Unit::None;
    let mut unit_1_ref_0: &meta::size::Unit = &mut unit_1;
    let mut display_1: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut display_0_ref_0: &flags::display::Display = &mut display_0;
    let mut unit_2_ref_0: &meta::size::Unit = &mut unit_2;
    let mut unit_3: meta::size::Unit = crate::meta::size::Unit::clone(unit_0_ref_0);
    let mut unit_4: meta::size::Unit = crate::meta::size::Unit::Kilo;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4885() {
//    rusty_monitor::set_test_id(4885);
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Kilo;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_0: usize = 8usize;
    let mut bool_0: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut u64_0: u64 = 86u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_0};
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut display_1: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut display_1_ref_0: &flags::display::Display = &mut display_1;
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::Kilo;
    let mut unit_1_ref_0: &meta::size::Unit = &mut unit_1;
    let mut unit_2: meta::size::Unit = crate::meta::size::Unit::Giga;
    let mut unit_3: meta::size::Unit = crate::meta::size::Unit::None;
    let mut unit_0_ref_0: &meta::size::Unit = &mut unit_0;
    let mut bool_1: bool = crate::meta::size::Unit::eq(unit_0_ref_0, unit_1_ref_0);
    let mut display_2: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut display_2_ref_0: &flags::display::Display = &mut display_2;
    let mut unit_2_ref_0: &meta::size::Unit = &mut unit_2;
    let mut unit_4: meta::size::Unit = crate::meta::size::Unit::clone(unit_2_ref_0);
    let mut theme_0: icon::Theme = crate::icon::Theme::NoIcon;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut unit_5: meta::size::Unit = crate::meta::size::Unit::Kilo;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_1};
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8330() {
//    rusty_monitor::set_test_id(8330);
    let mut u64_0: u64 = 1048576u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_0};
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut str_0: &str = "di";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut f64_0: f64 = 4621819117588971520.000000f64;
    let mut u64_1: u64 = 25u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_1};
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut string_0: std::string::String = crate::meta::size::Size::format_size(size_1_ref_0, f64_0);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_1);
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_0: bool = false;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Name;
    let mut tuple_0: () = crate::meta::size::Size::assert_receiver_is_total_eq(size_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3199() {
//    rusty_monitor::set_test_id(3199);
    let mut str_0: &str = "jl";
    let mut str_1: &str = "";
    let mut f64_0: f64 = 4652218415073722368.000000f64;
    let mut u64_0: u64 = 1024u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut string_0: std::string::String = crate::meta::size::Size::format_size(size_0_ref_0, f64_0);
    let mut option_0: std::option::Option<std::string::String> = std::option::Option::Some(string_0);
    let mut option_1: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_0_ref_0: &str = &mut str_0;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_236() {
//    rusty_monitor::set_test_id(236);
    let mut u64_0: u64 = 1073741824u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut u64_1: u64 = 1073741824u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_1};
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut u64_2: u64 = 0u64;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size::new(u64_2);
    let mut size_2_ref_0: &crate::meta::size::Size = &mut size_2;
    let mut u64_3: u64 = 1024u64;
    let mut size_3: crate::meta::size::Size = crate::meta::size::Size::new(u64_3);
    let mut size_3_ref_0: &crate::meta::size::Size = &mut size_3;
    let mut u64_4: u64 = 1073741824u64;
    let mut size_4: crate::meta::size::Size = crate::meta::size::Size::new(u64_4);
    let mut size_4_ref_0: &crate::meta::size::Size = &mut size_4;
    let mut u64_5: u64 = 1048576u64;
    let mut size_5: crate::meta::size::Size = crate::meta::size::Size::new(u64_5);
    let mut size_5_ref_0: &crate::meta::size::Size = &mut size_5;
    let mut u64_6: u64 = 0u64;
    let mut size_6: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_6};
    let mut size_6_ref_0: &crate::meta::size::Size = &mut size_6;
    let mut u64_7: u64 = 1073741824u64;
    let mut size_7: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_7};
    let mut size_7_ref_0: &crate::meta::size::Size = &mut size_7;
    let mut size_8: crate::meta::size::Size = crate::meta::size::Size::clone(size_7_ref_0);
    let mut size_9: crate::meta::size::Size = crate::meta::size::Size::clone(size_6_ref_0);
    let mut size_10: crate::meta::size::Size = crate::meta::size::Size::clone(size_5_ref_0);
    let mut size_11: crate::meta::size::Size = crate::meta::size::Size::clone(size_4_ref_0);
    let mut size_12: crate::meta::size::Size = crate::meta::size::Size::clone(size_3_ref_0);
    let mut size_13: crate::meta::size::Size = crate::meta::size::Size::clone(size_2_ref_0);
    let mut size_14: crate::meta::size::Size = crate::meta::size::Size::clone(size_1_ref_0);
    let mut size_15: crate::meta::size::Size = crate::meta::size::Size::clone(size_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1851() {
//    rusty_monitor::set_test_id(1851);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut str_0: &str = "gdoc";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut f64_0: f64 = 4652218415073722368.000000f64;
    let mut u64_0: u64 = 1024u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut string_0: std::string::String = crate::meta::size::Size::format_size(size_0_ref_0, f64_0);
    let mut option_0: std::option::Option<std::string::String> = std::option::Option::Some(string_0);
    let mut option_1: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_2: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_0: bool = false;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3776() {
//    rusty_monitor::set_test_id(3776);
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut option_0: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_0: &str = "procfile";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_1: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut elem_0: color::Elem = crate::color::Elem::Octal;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::SizeValue;
    let mut block_0_ref_0: &flags::blocks::Block = &mut block_0;
    let mut u64_0: u64 = 4u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut elem_1: color::Elem = crate::color::Elem::FileMedium;
    let mut u64_1: u64 = crate::meta::size::Size::get_bytes(size_0_ref_0);
    let mut block_1: flags::blocks::Block = crate::flags::blocks::Block::SizeValue;
    let mut block_1_ref_0: &flags::blocks::Block = &mut block_1;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Mega;
    let mut unit_0_ref_0: &meta::size::Unit = &mut unit_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut elem_2: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_3: color::Elem = crate::color::Elem::Group;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_1, exec: bool_0};
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::None;
    let mut unit_2: meta::size::Unit = crate::meta::size::Unit::None;
    let mut unit_2_ref_0: &meta::size::Unit = &mut unit_2;
    let mut tuple_0: () = crate::meta::size::Unit::assert_receiver_is_total_eq(unit_2_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_675() {
//    rusty_monitor::set_test_id(675);
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Mega;
    let mut unit_0_ref_0: &meta::size::Unit = &mut unit_0;
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::Tera;
    let mut unit_1_ref_0: &meta::size::Unit = &mut unit_1;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_0: usize = 80usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut u64_0: u64 = 41u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut unit_2: meta::size::Unit = crate::meta::size::Unit::Byte;
    let mut unit_2_ref_0: &meta::size::Unit = &mut unit_2;
    let mut unit_3: meta::size::Unit = crate::meta::size::Unit::clone(unit_2_ref_0);
    let mut unit_4: meta::size::Unit = crate::meta::size::Unit::clone(unit_1_ref_0);
    let mut unit_5: meta::size::Unit = crate::meta::size::Unit::clone(unit_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_111() {
//    rusty_monitor::set_test_id(111);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Octal;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Write;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::NonFile;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut elem_3: color::Elem = crate::color::Elem::File {exec: bool_1, uid: bool_0};
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut bool_2: bool = false;
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Mega;
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::Byte;
    let mut elem_4: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_5: color::Elem = crate::color::Elem::Octal;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut unit_0_ref_0: &meta::size::Unit = &mut unit_0;
    let mut unit_2: meta::size::Unit = crate::meta::size::Unit::clone(unit_0_ref_0);
    let mut size_0: crate::color::theme::Size = crate::color::theme::Size {none: color_3, small: color_2, medium: color_1, large: color_0};
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_213() {
//    rusty_monitor::set_test_id(213);
    let mut u64_0: u64 = 1099511627776u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut u64_1: u64 = 1073741824u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_1};
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut u64_2: u64 = 1099511627776u64;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_2};
    let mut size_2_ref_0: &crate::meta::size::Size = &mut size_2;
    let mut u64_3: u64 = 1024u64;
    let mut size_3: crate::meta::size::Size = crate::meta::size::Size::new(u64_3);
    let mut size_3_ref_0: &crate::meta::size::Size = &mut size_3;
    let mut u64_4: u64 = 0u64;
    let mut size_4: crate::meta::size::Size = crate::meta::size::Size::new(u64_4);
    let mut size_4_ref_0: &crate::meta::size::Size = &mut size_4;
    let mut u64_5: u64 = 1024u64;
    let mut size_5: crate::meta::size::Size = crate::meta::size::Size::new(u64_5);
    let mut size_5_ref_0: &crate::meta::size::Size = &mut size_5;
    let mut u64_6: u64 = 1024u64;
    let mut size_6: crate::meta::size::Size = crate::meta::size::Size::new(u64_6);
    let mut size_6_ref_0: &crate::meta::size::Size = &mut size_6;
    let mut u64_7: u64 = 0u64;
    let mut size_7: crate::meta::size::Size = crate::meta::size::Size::new(u64_7);
    let mut size_7_ref_0: &crate::meta::size::Size = &mut size_7;
    let mut u64_8: u64 = 1048576u64;
    let mut size_8: crate::meta::size::Size = crate::meta::size::Size::new(u64_8);
    let mut size_8_ref_0: &crate::meta::size::Size = &mut size_8;
    let mut u64_9: u64 = 1048576u64;
    let mut size_9: crate::meta::size::Size = crate::meta::size::Size::new(u64_9);
    let mut size_9_ref_0: &crate::meta::size::Size = &mut size_9;
    let mut bool_0: bool = crate::meta::size::Size::eq(size_9_ref_0, size_8_ref_0);
    let mut bool_1: bool = crate::meta::size::Size::eq(size_7_ref_0, size_6_ref_0);
    let mut bool_2: bool = crate::meta::size::Size::eq(size_5_ref_0, size_4_ref_0);
    let mut bool_3: bool = crate::meta::size::Size::eq(size_3_ref_0, size_2_ref_0);
    let mut bool_4: bool = crate::meta::size::Size::eq(size_1_ref_0, size_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4371() {
//    rusty_monitor::set_test_id(4371);
    let mut elem_0: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_1: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_2: color::Elem = crate::color::Elem::Read;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_3: color::Elem = crate::color::Elem::Acl;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut elem_4: color::Elem = crate::color::Elem::File {exec: bool_1, uid: bool_0};
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_5: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
//    panic!("From RustyUnit with love");
}
}