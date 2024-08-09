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
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5435() {
//    rusty_monitor::set_test_id(5435);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_0);
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_2: std::option::Option<bool> = std::option::Option::None;
    let mut bool_0: bool = false;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut option_4: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut option_5: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_0);
    let mut option_6: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut usize_0: usize = 8usize;
    let mut option_7: std::option::Option<usize> = std::option::Option::Some(usize_0);
    let mut option_8: std::option::Option<bool> = std::option::Option::None;
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_8, depth: option_7};
    let mut option_9: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_0);
    let mut option_10: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_1: bool = false;
    let mut option_11: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_12: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_13: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_14: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut option_15: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_15, theme: option_14, separator: option_13};
    let mut option_16: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut option_17: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut bool_2: bool = false;
    let mut option_18: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut option_19: std::option::Option<std::string::String> = std::option::Option::None;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut option_20: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_0);
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut option_21: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_21, theme: option_20};
    let mut option_22: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_23: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_3: bool = true;
    let mut option_24: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_24, blocks: option_23, color: option_22, date: option_19, dereference: option_18, display: option_17, icons: option_16, ignore_globs: option_12, indicators: option_11, layout: option_10, recursion: option_9, size: option_6, permission: option_5, sorting: option_4, no_symlink: option_3, total_size: option_2, symlink_arrow: option_1, hyperlink: option_0};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut option_25: std::option::Option<bool> = std::option::Option::None;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut config_2: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortorder_0_ref_0: &flags::sorting::SortOrder = &mut sortorder_0;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Mega;
    let mut unit_0_ref_0: &meta::size::Unit = &mut unit_0;
    let mut u64_0: u64 = 1024u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut filetype_0_ref_0: &meta::filetype::FileType = &mut filetype_0;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortorder_1_ref_0: &flags::sorting::SortOrder = &mut sortorder_1;
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::None;
    let mut unit_1_ref_0: &meta::size::Unit = &mut unit_1;
    let mut unit_2: meta::size::Unit = crate::meta::size::Unit::clone(unit_1_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4870() {
//    rusty_monitor::set_test_id(4870);
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Tera;
    let mut unit_0_ref_0: &meta::size::Unit = &mut unit_0;
    let mut str_0: &str = "sticky";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "YDfPdn";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "OneLine";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "reverse";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "owner";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "psd";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "or";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7: &str = "";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_8: &str = ".bashprofile";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut str_9: &str = "U4lLFj8laRZ";
    let mut str_9_ref_0: &str = &mut str_9;
    let mut str_10: &str = "NoAccess";
    let mut str_10_ref_0: &str = &mut str_10;
    let mut result_0: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_10_ref_0);
    let mut result_1: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_9_ref_0);
    let mut result_2: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_8_ref_0);
    let mut result_3: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_7_ref_0);
    let mut result_4: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_6_ref_0);
    let mut result_5: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_5_ref_0);
    let mut result_6: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_4_ref_0);
    let mut result_7: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_3_ref_0);
    let mut result_8: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_2_ref_0);
    let mut result_9: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_1_ref_0);
    let mut result_10: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_0_ref_0);
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::clone(unit_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_747() {
//    rusty_monitor::set_test_id(747);
    let mut f64_0: f64 = -44.572817f64;
    let mut u64_0: u64 = 1024u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut string_0: std::string::String = crate::meta::size::Size::format_size(size_0_ref_0, f64_0);
    let mut theme_0: icon::Theme = crate::icon::Theme::Fancy;
    let mut icons_0: crate::icon::Icons = crate::icon::Icons::new(theme_0, string_0);
    let mut icons_0_ref_0: &crate::icon::Icons = &mut icons_0;
    let mut option_0: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_0: &str = " ";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_1: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_0: usize = 8usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_718() {
//    rusty_monitor::set_test_id(718);
    let mut u64_0: u64 = 16u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_0};
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut u64_1: u64 = 1073741824u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut u64_2: u64 = 1048576u64;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_2};
    let mut size_2_ref_0: &crate::meta::size::Size = &mut size_2;
    let mut u64_3: u64 = 1099511627776u64;
    let mut size_3: crate::meta::size::Size = crate::meta::size::Size::new(u64_3);
    let mut size_3_ref_0: &crate::meta::size::Size = &mut size_3;
    let mut u64_4: u64 = 1073741824u64;
    let mut size_4: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_4};
    let mut size_4_ref_0: &crate::meta::size::Size = &mut size_4;
    let mut u64_5: u64 = 1099511627776u64;
    let mut size_5: crate::meta::size::Size = crate::meta::size::Size::new(u64_5);
    let mut size_5_ref_0: &crate::meta::size::Size = &mut size_5;
    let mut u64_6: u64 = 1073741824u64;
    let mut size_6: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_6};
    let mut size_6_ref_0: &crate::meta::size::Size = &mut size_6;
    let mut u64_7: u64 = 0u64;
    let mut size_7: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_7};
    let mut size_7_ref_0: &crate::meta::size::Size = &mut size_7;
    let mut u64_8: u64 = 1024u64;
    let mut size_8: crate::meta::size::Size = crate::meta::size::Size::new(u64_8);
    let mut size_8_ref_0: &crate::meta::size::Size = &mut size_8;
    let mut u64_9: u64 = 1024u64;
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
#[timeout(30000)]fn rusty_test_330() {
//    rusty_monitor::set_test_id(330);
    let mut u64_0: u64 = 61u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_0};
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut u64_1: u64 = 45u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_1};
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut u64_2: u64 = 0u64;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size::new(u64_2);
    let mut size_2_ref_0: &crate::meta::size::Size = &mut size_2;
    let mut u64_3: u64 = 1073741824u64;
    let mut size_3: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_3};
    let mut size_3_ref_0: &crate::meta::size::Size = &mut size_3;
    let mut u64_4: u64 = 1048576u64;
    let mut size_4: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_4};
    let mut size_4_ref_0: &crate::meta::size::Size = &mut size_4;
    let mut u64_5: u64 = 1099511627776u64;
    let mut size_5: crate::meta::size::Size = crate::meta::size::Size::new(u64_5);
    let mut size_5_ref_0: &crate::meta::size::Size = &mut size_5;
    let mut u64_6: u64 = 1048576u64;
    let mut size_6: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_6};
    let mut size_6_ref_0: &crate::meta::size::Size = &mut size_6;
    let mut u64_7: u64 = 62u64;
    let mut size_7: crate::meta::size::Size = crate::meta::size::Size::new(u64_7);
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
#[timeout(30000)]fn rusty_test_7927() {
//    rusty_monitor::set_test_id(7927);
    let mut str_0: &str = "column";
    let mut str_1: &str = "FileLarge";
    let mut str_2: &str = "";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut f64_0: f64 = 4652218415073722368.000000f64;
    let mut u64_0: u64 = 1099511627776u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_0};
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut string_0: std::string::String = crate::meta::size::Size::format_size(size_0_ref_0, f64_0);
    let mut str_3: &str = "Pipe";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_0_ref_0: &str = &mut str_0;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2597() {
//    rusty_monitor::set_test_id(2597);
    let mut bool_0: bool = false;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut usize_0: usize = 6usize;
    let mut bool_1: bool = false;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut iconoption_0_ref_0: &flags::icons::IconOption = &mut iconoption_0;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut dateflag_0_ref_0: &flags::date::DateFlag = &mut dateflag_0;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut iconoption_1_ref_0: &flags::icons::IconOption = &mut iconoption_1;
    let mut elem_0: color::Elem = crate::color::Elem::Pipe;
    let mut sizeflag_0_ref_0: &flags::size::SizeFlag = &mut sizeflag_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut elem_1: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_1, depth: usize_0};
    let mut config_2: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Tera;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut elem_2: color::Elem = crate::color::Elem::HourOld;
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::Byte;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_0};
    let mut unit_1_ref_0: &meta::size::Unit = &mut unit_1;
    let mut tuple_0: () = crate::meta::size::Unit::assert_receiver_is_total_eq(unit_1_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_58() {
//    rusty_monitor::set_test_id(58);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut f64_0: f64 = 4697254411347427328.000000f64;
    let mut u64_0: u64 = 59u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut string_0: std::string::String = crate::meta::size::Size::format_size(size_0_ref_0, f64_0);
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_0: usize = 21usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut display_0: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_2};
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut usize_1: usize = 1usize;
    let mut option_0: std::option::Option<usize> = std::option::Option::Some(usize_1);
    let mut usize_2: usize = 2usize;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_3);
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_1, order: sortorder_1, dir_grouping: dirgrouping_1};
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_3: usize = 89usize;
    let mut bool_1: bool = true;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_1, depth: usize_3};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut config_2: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_1: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_4: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_4};
    let mut displayoption_0: meta::name::DisplayOption = crate::meta::name::DisplayOption::None;
    let mut displayoption_0_ref_0: &meta::name::DisplayOption = &mut displayoption_0;
    let mut f64_1: f64 = 4652218415073722368.000000f64;
    let mut u64_1: u64 = 1048576u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_1};
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut string_1: std::string::String = crate::meta::size::Size::format_size(size_1_ref_0, f64_1);
    let mut owner_0: crate::meta::owner::Owner = crate::meta::owner::Owner::new(string_1, string_0);
    let mut owner_0_ref_0: &crate::meta::owner::Owner = &mut owner_0;
    crate::meta::owner::Owner::render_user(owner_0_ref_0, colors_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5996() {
//    rusty_monitor::set_test_id(5996);
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut f64_0: f64 = 4652218415073722368.000000f64;
    let mut u64_0: u64 = 1099511627776u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_0};
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut string_0: std::string::String = crate::meta::size::Size::format_size(size_0_ref_0, f64_0);
    let mut str_1: &str = "Pipe";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut bool_0: bool = false;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_0};
    let mut bool_1: bool = true;
    let mut bool_2: bool = false;
    let mut bool_3: bool = true;
    let mut bool_4: bool = false;
    let mut bool_5: bool = true;
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = true;
    let mut bool_10: bool = true;
    let mut bool_11: bool = true;
    let mut bool_12: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_12, user_write: bool_11, user_execute: bool_10, group_read: bool_9, group_write: bool_8, group_execute: bool_7, other_read: bool_6, other_write: bool_5, other_execute: bool_4, sticky: bool_3, setgid: bool_2, setuid: bool_1};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut option_1: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut option_2: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_2, theme: option_1};
    let mut option_3: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_4: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_5: std::option::Option<bool> = std::option::Option::None;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut f64_1: f64 = 4621819117588971520.000000f64;
    let mut u64_1: u64 = 1024u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_1};
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut string_1: std::string::String = crate::meta::size::Size::format_size(size_1_ref_0, f64_1);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Custom(string_1);
    let mut bool_13: bool = false;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_13};
    let mut bool_14: bool = true;
    let mut bool_15: bool = false;
    let mut bool_16: bool = false;
    let mut bool_17: bool = true;
    let mut bool_18: bool = true;
    let mut bool_19: bool = false;
    let mut bool_20: bool = false;
    let mut bool_21: bool = true;
    let mut bool_22: bool = true;
    let mut bool_23: bool = true;
    let mut bool_24: bool = false;
    let mut bool_25: bool = false;
    let mut permissions_1: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_25, user_write: bool_24, user_execute: bool_23, group_read: bool_22, group_write: bool_21, group_execute: bool_20, other_read: bool_19, other_write: bool_18, other_execute: bool_17, sticky: bool_16, setgid: bool_15, setuid: bool_14};
    let mut permissions_1_ref_0: &crate::meta::permissions::Permissions = &mut permissions_1;
    let mut option_6: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_0};
    let mut u64_2: u64 = 1024u64;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_2};
    let mut size_2_ref_0: &crate::meta::size::Size = &mut size_2;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Kilo;
    let mut unit_0_ref_0: &meta::size::Unit = &mut unit_0;
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::Mega;
    let mut unit_1_ref_0: &meta::size::Unit = &mut unit_1;
    let mut unit_2: meta::size::Unit = crate::meta::size::Unit::Giga;
    let mut unit_2_ref_0: &meta::size::Unit = &mut unit_2;
    let mut unit_3: meta::size::Unit = crate::meta::size::Unit::Giga;
    let mut unit_3_ref_0: &meta::size::Unit = &mut unit_3;
    let mut unit_4: meta::size::Unit = crate::meta::size::Unit::Giga;
    let mut unit_4_ref_0: &meta::size::Unit = &mut unit_4;
    let mut unit_5: meta::size::Unit = crate::meta::size::Unit::Tera;
    let mut unit_5_ref_0: &meta::size::Unit = &mut unit_5;
    let mut unit_6: meta::size::Unit = crate::meta::size::Unit::clone(unit_5_ref_0);
    let mut unit_7: meta::size::Unit = crate::meta::size::Unit::clone(unit_4_ref_0);
    let mut unit_8: meta::size::Unit = crate::meta::size::Unit::clone(unit_3_ref_0);
    let mut unit_9: meta::size::Unit = crate::meta::size::Unit::clone(unit_2_ref_0);
    let mut unit_10: meta::size::Unit = crate::meta::size::Unit::clone(unit_1_ref_0);
    let mut unit_11: meta::size::Unit = crate::meta::size::Unit::clone(unit_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7830() {
//    rusty_monitor::set_test_id(7830);
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut f64_0: f64 = 4652218415073722368.000000f64;
    let mut u64_0: u64 = 1099511627776u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_0};
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut string_0: std::string::String = crate::meta::size::Size::format_size(size_0_ref_0, f64_0);
    let mut str_1: &str = "Pipe";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut bool_0: bool = true;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_0};
    let mut bool_1: bool = true;
    let mut bool_2: bool = false;
    let mut bool_3: bool = true;
    let mut bool_4: bool = false;
    let mut bool_5: bool = true;
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = true;
    let mut bool_10: bool = true;
    let mut bool_11: bool = false;
    let mut bool_12: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_12, user_write: bool_11, user_execute: bool_10, group_read: bool_9, group_write: bool_8, group_execute: bool_7, other_read: bool_6, other_write: bool_5, other_execute: bool_4, sticky: bool_3, setgid: bool_2, setuid: bool_1};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut option_1: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut option_2: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_2, theme: option_1};
    let mut option_3: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_4: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_5: std::option::Option<bool> = std::option::Option::None;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut f64_1: f64 = 4621819117588971520.000000f64;
    let mut u64_1: u64 = 1024u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_1};
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut string_1: std::string::String = crate::meta::size::Size::format_size(size_1_ref_0, f64_1);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Custom(string_1);
    let mut bool_13: bool = false;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_13};
    let mut option_6: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_0};
    let mut u64_2: u64 = 1024u64;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_2};
    let mut size_2_ref_0: &crate::meta::size::Size = &mut size_2;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Kilo;
    let mut unit_0_ref_0: &meta::size::Unit = &mut unit_0;
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::Tera;
    let mut unit_1_ref_0: &meta::size::Unit = &mut unit_1;
    let mut unit_2: meta::size::Unit = crate::meta::size::Unit::Giga;
    let mut unit_2_ref_0: &meta::size::Unit = &mut unit_2;
    let mut unit_3: meta::size::Unit = crate::meta::size::Unit::Giga;
    let mut unit_3_ref_0: &meta::size::Unit = &mut unit_3;
    let mut unit_4: meta::size::Unit = crate::meta::size::Unit::Giga;
    let mut unit_4_ref_0: &meta::size::Unit = &mut unit_4;
    let mut unit_5: meta::size::Unit = crate::meta::size::Unit::Tera;
    let mut unit_5_ref_0: &meta::size::Unit = &mut unit_5;
    let mut unit_6: meta::size::Unit = crate::meta::size::Unit::clone(unit_5_ref_0);
    let mut unit_7: meta::size::Unit = crate::meta::size::Unit::clone(unit_4_ref_0);
    let mut unit_8: meta::size::Unit = crate::meta::size::Unit::clone(unit_3_ref_0);
    let mut unit_9: meta::size::Unit = crate::meta::size::Unit::clone(unit_2_ref_0);
    let mut unit_10: meta::size::Unit = crate::meta::size::Unit::clone(unit_1_ref_0);
    let mut unit_11: meta::size::Unit = crate::meta::size::Unit::clone(unit_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_430() {
//    rusty_monitor::set_test_id(430);
    let mut u64_0: u64 = 1024u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_0};
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut u64_1: u64 = 1048576u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_1};
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut u64_2: u64 = 1099511627776u64;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size::new(u64_2);
    let mut size_2_ref_0: &crate::meta::size::Size = &mut size_2;
    let mut u64_3: u64 = 1073741824u64;
    let mut size_3: crate::meta::size::Size = crate::meta::size::Size::new(u64_3);
    let mut size_3_ref_0: &crate::meta::size::Size = &mut size_3;
    let mut u64_4: u64 = 0u64;
    let mut size_4: crate::meta::size::Size = crate::meta::size::Size::new(u64_4);
    let mut size_4_ref_0: &crate::meta::size::Size = &mut size_4;
    let mut u64_5: u64 = 75u64;
    let mut size_5: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_5};
    let mut size_5_ref_0: &crate::meta::size::Size = &mut size_5;
    let mut u64_6: u64 = 61u64;
    let mut size_6: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_6};
    let mut size_6_ref_0: &crate::meta::size::Size = &mut size_6;
    let mut u64_7: u64 = 0u64;
    let mut size_7: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_7};
    let mut size_7_ref_0: &crate::meta::size::Size = &mut size_7;
    let mut tuple_0: () = crate::meta::size::Size::assert_receiver_is_total_eq(size_7_ref_0);
    let mut tuple_1: () = crate::meta::size::Size::assert_receiver_is_total_eq(size_6_ref_0);
    let mut tuple_2: () = crate::meta::size::Size::assert_receiver_is_total_eq(size_5_ref_0);
    let mut tuple_3: () = crate::meta::size::Size::assert_receiver_is_total_eq(size_4_ref_0);
    let mut tuple_4: () = crate::meta::size::Size::assert_receiver_is_total_eq(size_3_ref_0);
    let mut tuple_5: () = crate::meta::size::Size::assert_receiver_is_total_eq(size_2_ref_0);
    let mut tuple_6: () = crate::meta::size::Size::assert_receiver_is_total_eq(size_1_ref_0);
    let mut tuple_7: () = crate::meta::size::Size::assert_receiver_is_total_eq(size_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_617() {
//    rusty_monitor::set_test_id(617);
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Giga;
    let mut unit_0_ref_0: &meta::size::Unit = &mut unit_0;
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::Giga;
    let mut unit_1_ref_0: &meta::size::Unit = &mut unit_1;
    let mut unit_2: meta::size::Unit = crate::meta::size::Unit::Tera;
    let mut unit_2_ref_0: &meta::size::Unit = &mut unit_2;
    let mut unit_3: meta::size::Unit = crate::meta::size::Unit::Giga;
    let mut unit_3_ref_0: &meta::size::Unit = &mut unit_3;
    let mut unit_4: meta::size::Unit = crate::meta::size::Unit::Giga;
    let mut unit_4_ref_0: &meta::size::Unit = &mut unit_4;
    let mut unit_5: meta::size::Unit = crate::meta::size::Unit::Byte;
    let mut unit_5_ref_0: &meta::size::Unit = &mut unit_5;
    let mut unit_6: meta::size::Unit = crate::meta::size::Unit::Mega;
    let mut unit_6_ref_0: &meta::size::Unit = &mut unit_6;
    let mut unit_7: meta::size::Unit = crate::meta::size::Unit::Giga;
    let mut unit_7_ref_0: &meta::size::Unit = &mut unit_7;
    let mut unit_8: meta::size::Unit = crate::meta::size::Unit::Tera;
    let mut unit_8_ref_0: &meta::size::Unit = &mut unit_8;
    let mut unit_9: meta::size::Unit = crate::meta::size::Unit::Byte;
    let mut unit_9_ref_0: &meta::size::Unit = &mut unit_9;
    let mut unit_10: meta::size::Unit = crate::meta::size::Unit::Kilo;
    let mut unit_10_ref_0: &meta::size::Unit = &mut unit_10;
    let mut unit_11: meta::size::Unit = crate::meta::size::Unit::Tera;
    let mut unit_11_ref_0: &meta::size::Unit = &mut unit_11;
    let mut unit_12: meta::size::Unit = crate::meta::size::Unit::Kilo;
    let mut unit_12_ref_0: &meta::size::Unit = &mut unit_12;
    let mut unit_13: meta::size::Unit = crate::meta::size::Unit::Byte;
    let mut unit_13_ref_0: &meta::size::Unit = &mut unit_13;
    let mut bool_0: bool = crate::meta::size::Unit::eq(unit_13_ref_0, unit_12_ref_0);
    let mut bool_1: bool = crate::meta::size::Unit::eq(unit_11_ref_0, unit_10_ref_0);
    let mut bool_2: bool = crate::meta::size::Unit::eq(unit_9_ref_0, unit_8_ref_0);
    let mut bool_3: bool = crate::meta::size::Unit::eq(unit_7_ref_0, unit_6_ref_0);
    let mut bool_4: bool = crate::meta::size::Unit::eq(unit_5_ref_0, unit_4_ref_0);
    let mut bool_5: bool = crate::meta::size::Unit::eq(unit_3_ref_0, unit_2_ref_0);
    let mut bool_6: bool = crate::meta::size::Unit::eq(unit_1_ref_0, unit_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_152() {
//    rusty_monitor::set_test_id(152);
    let mut u64_0: u64 = 1099511627776u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut u64_1: u64 = 1024u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_1};
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut u64_2: u64 = 1048576u64;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_2};
    let mut size_2_ref_0: &crate::meta::size::Size = &mut size_2;
    let mut u64_3: u64 = 1099511627776u64;
    let mut size_3: crate::meta::size::Size = crate::meta::size::Size::new(u64_3);
    let mut size_3_ref_0: &crate::meta::size::Size = &mut size_3;
    let mut u64_4: u64 = 1099511627776u64;
    let mut size_4: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_4};
    let mut size_4_ref_0: &crate::meta::size::Size = &mut size_4;
    let mut u64_5: u64 = 1024u64;
    let mut size_5: crate::meta::size::Size = crate::meta::size::Size::new(u64_5);
    let mut size_5_ref_0: &crate::meta::size::Size = &mut size_5;
    let mut u64_6: u64 = 1099511627776u64;
    let mut size_6: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_6};
    let mut size_6_ref_0: &crate::meta::size::Size = &mut size_6;
    let mut u64_7: u64 = 51u64;
    let mut size_7: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_7};
    let mut size_7_ref_0: &crate::meta::size::Size = &mut size_7;
    let mut u64_8: u64 = 66u64;
    let mut size_8: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_8};
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
#[timeout(30000)]fn rusty_test_6319() {
//    rusty_monitor::set_test_id(6319);
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Byte;
    let mut unit_0_ref_0: &meta::size::Unit = &mut unit_0;
    let mut str_0: &str = "setuid";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "";
    let mut str_2: &str = "";
    let mut str_3: &str = "timesort";
    let mut str_4: &str = "fvJ1";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut bool_0: bool = true;
    let mut option_0: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut option_1: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut option_2: std::option::Option<bool> = std::option::Option::None;
    let mut option_3: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_5: &str = "reverse";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_6: &str = "owner";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7: &str = "or";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_8: &str = "";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_9: &str = ".bashprofile";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut str_10: &str = "U4lLFj8laRZ";
    let mut str_9_ref_0: &str = &mut str_9;
    let mut str_10_ref_0: &str = &mut str_10;
    let mut result_0: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_5_ref_0);
    let mut result_1: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_3_ref_0);
    let mut result_2: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_1_ref_0);
    let mut result_3: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_4_ref_0);
    let mut result_4: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_7_ref_0);
    let mut result_5: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_6_ref_0);
    let mut result_6: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_2_ref_0);
    let mut result_7: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_8_ref_0);
    let mut result_8: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_9_ref_0);
    let mut result_9: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_10_ref_0);
    let mut result_10: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_0_ref_0);
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::clone(unit_0_ref_0);
//    panic!("From RustyUnit with love");
}
}