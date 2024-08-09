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
#[timeout(30000)]fn rusty_test_1261() {
//    rusty_monitor::set_test_id(1261);
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Giga;
    let mut unit_0_ref_0: &meta::size::Unit = &mut unit_0;
    let mut bool_0: bool = false;
    let mut u64_0: u64 = 1024u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut elem_0: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::Mega;
    let mut unit_1_ref_0: &meta::size::Unit = &mut unit_1;
    let mut unit_2: meta::size::Unit = crate::meta::size::Unit::Mega;
    let mut unit_2_ref_0: &meta::size::Unit = &mut unit_2;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut bool_1: bool = false;
    let mut unit_3: meta::size::Unit = crate::meta::size::Unit::Kilo;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut elem_1: color::Elem = crate::color::Elem::Links {valid: bool_1};
    let mut unit_3_ref_0: &meta::size::Unit = &mut unit_3;
    let mut unit_4: meta::size::Unit = crate::meta::size::Unit::clone(unit_3_ref_0);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::User;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_0_ref_0);
    let mut elem_2: color::Elem = crate::color::Elem::FileSmall;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut elem_3: color::Elem = crate::color::Elem::User;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut bool_2: bool = crate::meta::size::Unit::eq(unit_2_ref_0, unit_1_ref_0);
    let mut displayoption_0: meta::name::DisplayOption = crate::meta::name::DisplayOption::None;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut displayoption_1: meta::name::DisplayOption = crate::meta::name::DisplayOption::FileName;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut tuple_0: () = crate::meta::size::Size::assert_receiver_is_total_eq(size_0_ref_0);
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_0};
    let mut unit_5: meta::size::Unit = crate::meta::size::Unit::clone(unit_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5010() {
//    rusty_monitor::set_test_id(5010);
    let mut u64_0: u64 = 1024u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut elem_0: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Mega;
    let mut unit_0_ref_0: &meta::size::Unit = &mut unit_0;
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::Mega;
    let mut unit_1_ref_0: &meta::size::Unit = &mut unit_1;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut bool_0: bool = false;
    let mut unit_2: meta::size::Unit = crate::meta::size::Unit::Kilo;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut elem_1: color::Elem = crate::color::Elem::Links {valid: bool_0};
    let mut unit_2_ref_0: &meta::size::Unit = &mut unit_2;
    let mut unit_3: meta::size::Unit = crate::meta::size::Unit::clone(unit_2_ref_0);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::User;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_0_ref_0);
    let mut elem_2: color::Elem = crate::color::Elem::FileSmall;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut elem_3: color::Elem = crate::color::Elem::User;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut tuple_0: () = crate::meta::size::Size::assert_receiver_is_total_eq(size_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7690() {
//    rusty_monitor::set_test_id(7690);
    let mut u64_0: u64 = 1024u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut elem_0: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Mega;
    let mut unit_0_ref_0: &meta::size::Unit = &mut unit_0;
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::Mega;
    let mut unit_1_ref_0: &meta::size::Unit = &mut unit_1;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut unit_2: meta::size::Unit = crate::meta::size::Unit::Kilo;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut unit_2_ref_0: &meta::size::Unit = &mut unit_2;
    let mut unit_3: meta::size::Unit = crate::meta::size::Unit::clone(unit_2_ref_0);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::User;
    let mut elem_1: color::Elem = crate::color::Elem::FileSmall;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut elem_2: color::Elem = crate::color::Elem::User;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut bool_0: bool = crate::meta::size::Unit::eq(unit_1_ref_0, unit_0_ref_0);
    let mut displayoption_0: meta::name::DisplayOption = crate::meta::name::DisplayOption::None;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut displayoption_1: meta::name::DisplayOption = crate::meta::name::DisplayOption::FileName;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8070() {
//    rusty_monitor::set_test_id(8070);
    let mut elem_0: color::Elem = crate::color::Elem::Socket;
    let mut u64_0: u64 = 6u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut elem_1: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Mega;
    let mut unit_0_ref_0: &meta::size::Unit = &mut unit_0;
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::Mega;
    let mut unit_1_ref_0: &meta::size::Unit = &mut unit_1;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut bool_0: bool = false;
    let mut unit_2: meta::size::Unit = crate::meta::size::Unit::Kilo;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut elem_2: color::Elem = crate::color::Elem::Links {valid: bool_0};
    let mut unit_2_ref_0: &meta::size::Unit = &mut unit_2;
    let mut unit_3: meta::size::Unit = crate::meta::size::Unit::clone(unit_2_ref_0);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::User;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut elem_3: color::Elem = crate::color::Elem::FileSmall;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut elem_4: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut bool_1: bool = crate::meta::size::Unit::eq(unit_1_ref_0, unit_0_ref_0);
    let mut displayoption_0: meta::name::DisplayOption = crate::meta::name::DisplayOption::None;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut displayoption_1: meta::name::DisplayOption = crate::meta::name::DisplayOption::FileName;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut tuple_0: () = crate::meta::size::Size::assert_receiver_is_total_eq(size_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8312() {
//    rusty_monitor::set_test_id(8312);
    let mut u64_0: u64 = 1024u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut elem_0: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Mega;
    let mut unit_0_ref_0: &meta::size::Unit = &mut unit_0;
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::Giga;
    let mut unit_1_ref_0: &meta::size::Unit = &mut unit_1;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut bool_0: bool = false;
    let mut unit_2: meta::size::Unit = crate::meta::size::Unit::Mega;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut elem_1: color::Elem = crate::color::Elem::Links {valid: bool_0};
    let mut unit_2_ref_0: &meta::size::Unit = &mut unit_2;
    let mut unit_3: meta::size::Unit = crate::meta::size::Unit::clone(unit_2_ref_0);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::User;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_0_ref_0);
    let mut elem_2: color::Elem = crate::color::Elem::FileSmall;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut elem_3: color::Elem = crate::color::Elem::ExecSticky;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut bool_1: bool = crate::meta::size::Unit::eq(unit_1_ref_0, unit_0_ref_0);
    let mut displayoption_0: meta::name::DisplayOption = crate::meta::name::DisplayOption::None;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut displayoption_1: meta::name::DisplayOption = crate::meta::name::DisplayOption::FileName;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut tuple_0: () = crate::meta::size::Size::assert_receiver_is_total_eq(size_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1456() {
//    rusty_monitor::set_test_id(1456);
    let mut u64_0: u64 = 0u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_0};
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut u64_1: u64 = 0u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut filetype_0_ref_0: &meta::filetype::FileType = &mut filetype_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut bool_0: bool = true;
    let mut elem_0: color::Elem = crate::color::Elem::INode {valid: bool_0};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::Acl;
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::Octal;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_4: color::Elem = crate::color::Elem::Context;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut theme_5: icon::Theme = crate::icon::Theme::Fancy;
    let mut bool_1: bool = crate::meta::size::Size::ne(size_1_ref_0, size_0_ref_0);
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3731() {
//    rusty_monitor::set_test_id(3731);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut u64_0: u64 = 1024u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut elem_0: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Mega;
    let mut unit_0_ref_0: &meta::size::Unit = &mut unit_0;
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::Tera;
    let mut unit_1_ref_0: &meta::size::Unit = &mut unit_1;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut unit_2: meta::size::Unit = crate::meta::size::Unit::Kilo;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::User;
    let mut elem_1: color::Elem = crate::color::Elem::FileSmall;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut elem_2: color::Elem = crate::color::Elem::User;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut bool_0: bool = crate::meta::size::Unit::eq(unit_1_ref_0, unit_0_ref_0);
    let mut displayoption_0: meta::name::DisplayOption = crate::meta::name::DisplayOption::None;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut displayoption_1: meta::name::DisplayOption = crate::meta::name::DisplayOption::FileName;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut tuple_0: () = crate::meta::size::Size::assert_receiver_is_total_eq(size_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_819() {
//    rusty_monitor::set_test_id(819);
    let mut u64_0: u64 = 1024u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut dirgrouping_0_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_0;
    let mut tuple_0: () = crate::meta::size::Size::assert_receiver_is_total_eq(size_0_ref_0);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_9058() {
//    rusty_monitor::set_test_id(9058);
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Mega;
    let mut unit_0_ref_0: &meta::size::Unit = &mut unit_0;
    let mut u64_0: u64 = 1024u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut elem_0: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::Mega;
    let mut unit_1_ref_0: &meta::size::Unit = &mut unit_1;
    let mut unit_2: meta::size::Unit = crate::meta::size::Unit::Mega;
    let mut unit_2_ref_0: &meta::size::Unit = &mut unit_2;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut bool_0: bool = false;
    let mut unit_3: meta::size::Unit = crate::meta::size::Unit::Kilo;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut elem_1: color::Elem = crate::color::Elem::Links {valid: bool_0};
    let mut unit_3_ref_0: &meta::size::Unit = &mut unit_3;
    let mut unit_4: meta::size::Unit = crate::meta::size::Unit::clone(unit_3_ref_0);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::User;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_0_ref_0);
    let mut elem_2: color::Elem = crate::color::Elem::FileSmall;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut elem_3: color::Elem = crate::color::Elem::User;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut bool_1: bool = crate::meta::size::Unit::eq(unit_2_ref_0, unit_1_ref_0);
    let mut displayoption_0: meta::name::DisplayOption = crate::meta::name::DisplayOption::None;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut displayoption_1: meta::name::DisplayOption = crate::meta::name::DisplayOption::FileName;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut tuple_0: () = crate::meta::size::Size::assert_receiver_is_total_eq(size_0_ref_0);
    let mut tuple_1: () = crate::meta::size::Unit::assert_receiver_is_total_eq(unit_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_225() {
//    rusty_monitor::set_test_id(225);
    let mut u64_0: u64 = 1024u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_0};
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut u64_1: u64 = 1099511627776u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_1};
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut u64_2: u64 = 1048576u64;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_2};
    let mut size_2_ref_0: &crate::meta::size::Size = &mut size_2;
    let mut u64_3: u64 = 1073741824u64;
    let mut size_3: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_3};
    let mut size_3_ref_0: &crate::meta::size::Size = &mut size_3;
    let mut u64_4: u64 = 1073741824u64;
    let mut size_4: crate::meta::size::Size = crate::meta::size::Size::new(u64_4);
    let mut size_4_ref_0: &crate::meta::size::Size = &mut size_4;
    let mut u64_5: u64 = 1099511627776u64;
    let mut size_5: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_5};
    let mut size_5_ref_0: &crate::meta::size::Size = &mut size_5;
    let mut u64_6: u64 = 1099511627776u64;
    let mut size_6: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_6};
    let mut size_6_ref_0: &crate::meta::size::Size = &mut size_6;
    let mut u64_7: u64 = 0u64;
    let mut size_7: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_7};
    let mut size_7_ref_0: &crate::meta::size::Size = &mut size_7;
    let mut u64_8: u64 = 1073741824u64;
    let mut size_8: crate::meta::size::Size = crate::meta::size::Size::new(u64_8);
    let mut size_8_ref_0: &crate::meta::size::Size = &mut size_8;
    let mut u64_9: u64 = 1073741824u64;
    let mut size_9: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_9};
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
#[timeout(30000)]fn rusty_test_434() {
//    rusty_monitor::set_test_id(434);
    let mut f64_0: f64 = 54.600384f64;
    let mut u64_0: u64 = 1024u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut string_0: std::string::String = crate::meta::size::Size::format_size(size_0_ref_0, f64_0);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Formatted(string_0);
    let mut dateflag_0_ref_0: &flags::date::DateFlag = &mut dateflag_0;
    let mut option_0: std::option::Option<std::vec::Vec<crate::meta::Meta>> = std::option::Option::None;
    let mut u64_1: u64 = 1099511627776u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut bool_0: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_0};
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = false;
    let mut bool_5: bool = false;
    let mut bool_6: bool = false;
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut bool_12: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_12, user_write: bool_11, user_execute: bool_10, group_read: bool_9, group_write: bool_8, group_execute: bool_7, other_read: bool_6, other_write: bool_5, other_execute: bool_4, sticky: bool_3, setgid: bool_2, setuid: bool_1};
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_489() {
//    rusty_monitor::set_test_id(489);
    let mut u64_0: u64 = 1048576u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_0};
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut u64_1: u64 = 1073741824u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_1};
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut u64_2: u64 = 4u64;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_2};
    let mut size_2_ref_0: &crate::meta::size::Size = &mut size_2;
    let mut u64_3: u64 = 1099511627776u64;
    let mut size_3: crate::meta::size::Size = crate::meta::size::Size::new(u64_3);
    let mut size_3_ref_0: &crate::meta::size::Size = &mut size_3;
    let mut u64_4: u64 = 1048576u64;
    let mut size_4: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_4};
    let mut size_4_ref_0: &crate::meta::size::Size = &mut size_4;
    let mut u64_5: u64 = 1048576u64;
    let mut size_5: crate::meta::size::Size = crate::meta::size::Size::new(u64_5);
    let mut size_5_ref_0: &crate::meta::size::Size = &mut size_5;
    let mut u64_6: u64 = 1048576u64;
    let mut size_6: crate::meta::size::Size = crate::meta::size::Size::new(u64_6);
    let mut size_6_ref_0: &crate::meta::size::Size = &mut size_6;
    let mut u64_7: u64 = 1048576u64;
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
#[timeout(30000)]fn rusty_test_7850() {
//    rusty_monitor::set_test_id(7850);
    let mut f64_0: f64 = 4621819117588971520.000000f64;
    let mut u64_0: u64 = 1048576u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_0};
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut string_0: std::string::String = crate::meta::size::Size::format_size(size_0_ref_0, f64_0);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut bool_2: bool = false;
    let mut bool_3: bool = true;
    let mut bool_4: bool = false;
    let mut bool_5: bool = false;
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut bool_8: bool = false;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_0: usize = 2usize;
    let mut bool_12: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_12, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut f64_1: f64 = 4697254411347427328.000000f64;
    let mut u64_1: u64 = 58u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut string_1: std::string::String = crate::meta::size::Size::format_size(size_1_ref_0, f64_1);
    let mut owner_0: crate::meta::owner::Owner = crate::meta::owner::Owner::new(string_1, string_0);
    let mut owner_0_ref_0: &crate::meta::owner::Owner = &mut owner_0;
    let mut u64_2: u64 = 1024u64;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size::new(u64_2);
    let mut size_2_ref_0: &crate::meta::size::Size = &mut size_2;
    let mut elem_0: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Mega;
    let mut unit_0_ref_0: &meta::size::Unit = &mut unit_0;
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::Mega;
    let mut unit_1_ref_0: &meta::size::Unit = &mut unit_1;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut bool_13: bool = false;
    let mut unit_2: meta::size::Unit = crate::meta::size::Unit::Kilo;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut elem_1: color::Elem = crate::color::Elem::Links {valid: bool_13};
    let mut unit_2_ref_0: &meta::size::Unit = &mut unit_2;
    let mut unit_3: meta::size::Unit = crate::meta::size::Unit::clone(unit_2_ref_0);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::User;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_0_ref_0);
    let mut elem_2: color::Elem = crate::color::Elem::FileSmall;
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut elem_3: color::Elem = crate::color::Elem::User;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut bool_14: bool = crate::meta::size::Unit::eq(unit_1_ref_0, unit_0_ref_0);
    let mut displayoption_0: meta::name::DisplayOption = crate::meta::name::DisplayOption::None;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut displayoption_1: meta::name::DisplayOption = crate::meta::name::DisplayOption::FileName;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut tuple_0: () = crate::meta::size::Size::assert_receiver_is_total_eq(size_2_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_388() {
//    rusty_monitor::set_test_id(388);
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut usize_0: usize = 0usize;
    let mut bool_2: bool = true;
    let mut bool_3: bool = false;
    let mut option_0: std::option::Option<bool> = std::option::Option::None;
    let mut option_1: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut option_2: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut option_3: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_4: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_5: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut option_6: std::option::Option<bool> = std::option::Option::None;
    let mut option_7: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_8: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut option_9: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_0);
    let mut option_10: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_10, theme: option_9, separator: option_8};
    let mut option_11: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut option_12: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut bool_4: bool = true;
    let mut option_13: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut option_14: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_15: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_16: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_17: std::option::Option<bool> = std::option::Option::None;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_1: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut option_18: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_19: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_20: std::option::Option<bool> = std::option::Option::None;
    let mut option_21: std::option::Option<bool> = std::option::Option::None;
    let mut option_22: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut option_23: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_0);
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut option_24: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_0);
    let mut option_25: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut option_26: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_0);
    let mut option_27: std::option::Option<bool> = std::option::Option::None;
    let mut option_28: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_29: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut display_2: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut option_30: std::option::Option<flags::display::Display> = std::option::Option::Some(display_2);
    let mut option_31: std::option::Option<bool> = std::option::Option::None;
    let mut option_32: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_33: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_34: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_5: bool = true;
    let mut option_35: std::option::Option<bool> = std::option::Option::Some(bool_5);
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_35, blocks: option_34, color: option_33, date: option_32, dereference: option_31, display: option_30, icons: option_29, ignore_globs: option_28, indicators: option_27, layout: option_26, recursion: option_25, size: option_24, permission: option_23, sorting: option_22, no_symlink: option_21, total_size: option_20, symlink_arrow: option_19, hyperlink: option_18};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut u64_0: u64 = 1024u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut theme_0: icon::Theme = crate::icon::Theme::Fancy;
    let mut f64_0: f64 = 4742290407621132288.000000f64;
    let mut u64_1: u64 = 1073741824u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut string_0: std::string::String = crate::meta::size::Size::format_size(size_1_ref_0, f64_0);
    let mut theme_1: icon::Theme = crate::icon::Theme::Unicode;
    let mut f64_1: f64 = 4652218415073722368.000000f64;
    let mut u64_2: u64 = 0u64;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size::new(u64_2);
    let mut size_2_ref_0: &crate::meta::size::Size = &mut size_2;
    let mut string_1: std::string::String = crate::meta::size::Size::format_size(size_2_ref_0, f64_1);
    let mut theme_2: icon::Theme = crate::icon::Theme::NoIcon;
    let mut f64_2: f64 = 4787326403894837248.000000f64;
    let mut u64_3: u64 = 1073741824u64;
    let mut size_3: crate::meta::size::Size = crate::meta::size::Size {bytes: u64_3};
    let mut size_3_ref_0: &crate::meta::size::Size = &mut size_3;
    let mut string_2: std::string::String = crate::meta::size::Size::format_size(size_3_ref_0, f64_2);
    let mut theme_3: icon::Theme = crate::icon::Theme::Fancy;
    let mut icons_1: crate::icon::Icons = crate::icon::Icons::new(theme_3, string_2);
    let mut icons_2: crate::icon::Icons = crate::icon::Icons::new(theme_2, string_1);
    let mut icons_3: crate::icon::Icons = crate::icon::Icons::new(theme_1, string_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3919() {
//    rusty_monitor::set_test_id(3919);
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Byte;
    let mut unit_0_ref_0: &meta::size::Unit = &mut unit_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut option_0: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut option_1: std::option::Option<crate::color::theme::Theme> = std::option::Option::Some(theme_0);
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_1, exec: bool_0};
    let mut bool_2: bool = true;
    let mut bool_3: bool = false;
    let mut bool_4: bool = true;
    let mut elem_0: color::Elem = crate::color::Elem::Pipe;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_1: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut elem_2: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut elem_3: color::Elem = crate::color::Elem::SymLink;
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::Kilo;
    let mut bool_5: bool = crate::meta::filetype::FileType::is_dirlike(filetype_2);
    let mut elem_4: color::Elem = crate::color::Elem::File {exec: bool_4, uid: bool_3};
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_2};
    let mut filetype_3_ref_0: &meta::filetype::FileType = &mut filetype_3;
    let mut theme_1: icon::Theme = crate::icon::Theme::Fancy;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut filetype_1_ref_0: &meta::filetype::FileType = &mut filetype_1;
    let mut unit_2: meta::size::Unit = crate::meta::size::Unit::clone(unit_0_ref_0);
//    panic!("From RustyUnit with love");
}
}