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
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1260() {
    rusty_monitor::set_test_id(1260);
    let mut u64_0: u64 = 6247u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_0};
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut u64_1: u64 = 4133u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_1};
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut u64_2: u64 = 2547u64;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_2};
    let mut size_2_ref_0: &crate::meta::size::Size = &mut size_2;
    let mut usize_0: usize = 3772usize;
    let mut option_0: std::option::Option<usize> = std::option::Option::Some(usize_0);
    let mut option_1: std::option::Option<bool> = std::option::Option::None;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Tera;
    let mut unit_0_ref_0: &meta::size::Unit = &mut unit_0;
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::Giga;
    let mut unit_1_ref_0: &meta::size::Unit = &mut unit_1;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut bool_5: bool = false;
    let mut bool_6: bool = false;
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut bool_12: bool = crate::meta::permissions::Permissions::is_executable(permissions_0_ref_0);
    let mut bool_13: bool = crate::meta::size::Unit::eq(unit_1_ref_0, unit_0_ref_0);
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_1, depth: option_0};
    let mut unit_2: meta::size::Unit = crate::meta::size::Unit::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut tuple_0: () = crate::meta::size::Size::assert_receiver_is_total_eq(size_2_ref_0);
    let mut bool_14: bool = crate::meta::size::Size::eq(size_1_ref_0, size_0_ref_0);
    let mut elem_0: color::Elem = crate::color::Elem::SymLink;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Context;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_550() {
    rusty_monitor::set_test_id(550);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Special;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Pipe;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut u64_0: u64 = 8834u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_0};
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut u64_1: u64 = 2547u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_1};
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut usize_0: usize = 3772usize;
    let mut option_0: std::option::Option<usize> = std::option::Option::Some(usize_0);
    let mut option_1: std::option::Option<bool> = std::option::Option::None;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Tera;
    let mut unit_0_ref_0: &meta::size::Unit = &mut unit_0;
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::Giga;
    let mut unit_1_ref_0: &meta::size::Unit = &mut unit_1;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut bool_5: bool = false;
    let mut bool_6: bool = false;
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut bool_12: bool = crate::meta::permissions::Permissions::is_executable(permissions_0_ref_0);
    let mut bool_13: bool = crate::meta::size::Unit::eq(unit_1_ref_0, unit_0_ref_0);
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_1, depth: option_0};
    let mut unit_2: meta::size::Unit = crate::meta::size::Unit::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut tuple_0: () = crate::meta::size::Size::assert_receiver_is_total_eq(size_0_ref_0);
    let mut inode_0: crate::color::theme::INode = crate::color::theme::INode {valid: color_1, invalid: color_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3872() {
    rusty_monitor::set_test_id(3872);
    let mut u64_0: u64 = 2547u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_0};
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut usize_0: usize = 3772usize;
    let mut option_0: std::option::Option<usize> = std::option::Option::Some(usize_0);
    let mut option_1: std::option::Option<bool> = std::option::Option::None;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Tera;
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::Giga;
    let mut unit_1_ref_0: &meta::size::Unit = &mut unit_1;
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_1, depth: option_0};
    let mut unit_2: meta::size::Unit = crate::meta::size::Unit::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut tuple_0: () = crate::meta::size::Size::assert_receiver_is_total_eq(size_0_ref_0);
    let mut app_0: clap::App = crate::app::build();
    let mut unit_0_ref_0: &meta::size::Unit = &mut unit_0;
    let mut unit_3: meta::size::Unit = crate::meta::size::Unit::clone(unit_0_ref_0);
    let mut elem_0: color::Elem = crate::color::Elem::Group;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_386() {
    rusty_monitor::set_test_id(386);
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Byte;
    let mut unit_0_ref_0: &meta::size::Unit = &mut unit_0;
    let mut bool_0: bool = false;
    let mut u64_0: u64 = 2547u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_0};
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut usize_0: usize = 3772usize;
    let mut option_0: std::option::Option<usize> = std::option::Option::Some(usize_0);
    let mut option_1: std::option::Option<bool> = std::option::Option::None;
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::Tera;
    let mut unit_1_ref_0: &meta::size::Unit = &mut unit_1;
    let mut unit_2: meta::size::Unit = crate::meta::size::Unit::Giga;
    let mut unit_2_ref_0: &meta::size::Unit = &mut unit_2;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut bool_5: bool = true;
    let mut bool_6: bool = false;
    let mut bool_7: bool = false;
    let mut bool_8: bool = true;
    let mut bool_9: bool = true;
    let mut bool_10: bool = true;
    let mut bool_11: bool = false;
    let mut bool_12: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_12, user_write: bool_11, user_execute: bool_10, group_read: bool_9, group_write: bool_8, group_execute: bool_7, other_read: bool_6, other_write: bool_5, other_execute: bool_4, sticky: bool_3, setgid: bool_2, setuid: bool_1};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut bool_13: bool = crate::meta::permissions::Permissions::is_executable(permissions_0_ref_0);
    let mut bool_14: bool = crate::meta::size::Unit::eq(unit_2_ref_0, unit_1_ref_0);
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_1, depth: option_0};
    let mut unit_3: meta::size::Unit = crate::meta::size::Unit::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut tuple_0: () = crate::meta::size::Size::assert_receiver_is_total_eq(size_0_ref_0);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_0};
    let mut unit_4: meta::size::Unit = crate::meta::size::Unit::clone(unit_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_11683() {
    rusty_monitor::set_test_id(11683);
    let mut usize_0: usize = 3772usize;
    let mut option_0: std::option::Option<usize> = std::option::Option::Some(usize_0);
    let mut option_1: std::option::Option<bool> = std::option::Option::None;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Tera;
    let mut unit_0_ref_0: &meta::size::Unit = &mut unit_0;
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::Giga;
    let mut unit_1_ref_0: &meta::size::Unit = &mut unit_1;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut bool_5: bool = false;
    let mut bool_6: bool = false;
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut bool_12: bool = crate::meta::permissions::Permissions::is_executable(permissions_0_ref_0);
    let mut bool_13: bool = crate::meta::size::Unit::eq(unit_1_ref_0, unit_0_ref_0);
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_1, depth: option_0};
    let mut unit_2: meta::size::Unit = crate::meta::size::Unit::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3371() {
    rusty_monitor::set_test_id(3371);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut u64_0: u64 = 2547u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_0};
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut usize_0: usize = 3772usize;
    let mut option_0: std::option::Option<usize> = std::option::Option::Some(usize_0);
    let mut option_1: std::option::Option<bool> = std::option::Option::None;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Tera;
    let mut unit_0_ref_0: &meta::size::Unit = &mut unit_0;
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::Giga;
    let mut unit_1_ref_0: &meta::size::Unit = &mut unit_1;
    let mut bool_0: bool = crate::meta::size::Unit::eq(unit_1_ref_0, unit_0_ref_0);
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_1, depth: option_0};
    let mut unit_2: meta::size::Unit = crate::meta::size::Unit::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut tuple_0: () = crate::meta::size::Size::assert_receiver_is_total_eq(size_0_ref_0);
    let mut option_2: std::option::Option<std::path::PathBuf> = crate::config_file::Config::config_file_path();
    let mut unit_3: meta::size::Unit = crate::meta::size::Unit::Byte;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3139() {
    rusty_monitor::set_test_id(3139);
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Giga;
    let mut u64_0: u64 = 2547u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_0};
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut usize_0: usize = 3772usize;
    let mut option_0: std::option::Option<usize> = std::option::Option::Some(usize_0);
    let mut option_1: std::option::Option<bool> = std::option::Option::None;
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::Tera;
    let mut unit_0_ref_0: &meta::size::Unit = &mut unit_0;
    let mut unit_2: meta::size::Unit = crate::meta::size::Unit::Giga;
    let mut unit_2_ref_0: &meta::size::Unit = &mut unit_2;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut bool_5: bool = false;
    let mut bool_6: bool = false;
    let mut bool_7: bool = false;
    let mut bool_8: bool = true;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut bool_12: bool = crate::meta::permissions::Permissions::is_executable(permissions_0_ref_0);
    let mut bool_13: bool = crate::meta::size::Unit::eq(unit_2_ref_0, unit_0_ref_0);
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_1, depth: option_0};
    let mut unit_3: meta::size::Unit = crate::meta::size::Unit::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut tuple_0: () = crate::meta::size::Size::assert_receiver_is_total_eq(size_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3284() {
    rusty_monitor::set_test_id(3284);
    let mut u64_0: u64 = 2547u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_0};
    let mut usize_0: usize = 3772usize;
    let mut option_0: std::option::Option<usize> = std::option::Option::Some(usize_0);
    let mut option_1: std::option::Option<bool> = std::option::Option::None;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Tera;
    let mut unit_0_ref_0: &meta::size::Unit = &mut unit_0;
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::Giga;
    let mut unit_1_ref_0: &meta::size::Unit = &mut unit_1;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut bool_5: bool = false;
    let mut bool_6: bool = false;
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut bool_12: bool = crate::meta::permissions::Permissions::is_executable(permissions_0_ref_0);
    let mut bool_13: bool = crate::meta::size::Unit::eq(unit_1_ref_0, unit_0_ref_0);
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_1, depth: option_0};
    let mut unit_2: meta::size::Unit = crate::meta::size::Unit::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::clone(size_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_10607() {
    rusty_monitor::set_test_id(10607);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut u64_0: u64 = 1303u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_0: color::Elem = crate::color::Elem::DayOld;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut bool_0: bool = true;
    let mut elem_1: color::Elem = crate::color::Elem::Dir {uid: bool_0};
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_2: color::Elem = crate::color::Elem::Older;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_0_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_3: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut bool_1: bool = false;
    let mut elem_4: color::Elem = crate::color::Elem::Links {valid: bool_1};
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_5_ref_0);
    let mut tuple_0: () = crate::meta::size::Size::assert_receiver_is_total_eq(size_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_10490() {
    rusty_monitor::set_test_id(10490);
    let mut u64_0: u64 = 2547u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_0};
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut usize_0: usize = 3772usize;
    let mut option_0: std::option::Option<usize> = std::option::Option::Some(usize_0);
    let mut option_1: std::option::Option<bool> = std::option::Option::None;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Tera;
    let mut unit_0_ref_0: &meta::size::Unit = &mut unit_0;
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::Giga;
    let mut unit_1_ref_0: &meta::size::Unit = &mut unit_1;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut bool_5: bool = false;
    let mut bool_6: bool = false;
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut bool_12: bool = crate::meta::permissions::Permissions::is_executable(permissions_0_ref_0);
    let mut bool_13: bool = crate::meta::size::Unit::eq(unit_1_ref_0, unit_0_ref_0);
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_1, depth: option_0};
    let mut unit_2: meta::size::Unit = crate::meta::size::Unit::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut tuple_0: () = crate::meta::size::Size::assert_receiver_is_total_eq(size_0_ref_0);
    let mut unit_2_ref_0: &meta::size::Unit = &mut unit_2;
    let mut tuple_1: () = crate::meta::size::Unit::assert_receiver_is_total_eq(unit_2_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5383() {
    rusty_monitor::set_test_id(5383);
    let mut u64_0: u64 = 2547u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_0};
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut option_0: std::option::Option<bool> = std::option::Option::None;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Tera;
    let mut unit_0_ref_0: &meta::size::Unit = &mut unit_0;
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::Giga;
    let mut unit_1_ref_0: &meta::size::Unit = &mut unit_1;
    let mut bool_0: bool = crate::meta::size::Unit::eq(unit_1_ref_0, unit_0_ref_0);
    let mut unit_2: meta::size::Unit = crate::meta::size::Unit::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut tuple_0: () = crate::meta::size::Size::assert_receiver_is_total_eq(size_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3176() {
    rusty_monitor::set_test_id(3176);
    let mut u64_0: u64 = 2547u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_0};
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut usize_0: usize = 3772usize;
    let mut option_0: std::option::Option<usize> = std::option::Option::Some(usize_0);
    let mut option_1: std::option::Option<bool> = std::option::Option::None;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Tera;
    let mut unit_0_ref_0: &meta::size::Unit = &mut unit_0;
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::Giga;
    let mut unit_1_ref_0: &meta::size::Unit = &mut unit_1;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut bool_5: bool = false;
    let mut bool_6: bool = false;
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = false;
    let mut bool_10: bool = true;
    let mut bool_11: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut bool_12: bool = crate::meta::permissions::Permissions::is_executable(permissions_0_ref_0);
    let mut bool_13: bool = crate::meta::size::Unit::eq(unit_1_ref_0, unit_0_ref_0);
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_1, depth: option_0};
    let mut unit_2: meta::size::Unit = crate::meta::size::Unit::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut tuple_0: () = crate::meta::size::Size::assert_receiver_is_total_eq(size_0_ref_0);
    panic!("From RustyUnit with love");
}
}