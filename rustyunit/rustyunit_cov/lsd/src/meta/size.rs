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
            ColoredString::new(Colors::default_style(), left_pad), val_content,
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
            Unit::Kilo => {
                self.format_size(((self.bytes as f64) / 1024.0 * 10.0).round() / 10.0)
            }
            Unit::Mega => {
                self
                    .format_size(
                        ((self.bytes as f64) / (1024.0 * 1024.0) * 10.0).round() / 10.0,
                    )
            }
            Unit::Giga => {
                self
                    .format_size(
                        ((self.bytes as f64) / (1024.0 * 1024.0 * 1024.0) * 10.0).round()
                            / 10.0,
                    )
            }
            Unit::Tera => {
                self
                    .format_size(
                        ((self.bytes as f64) / (1024.0 * 1024.0 * 1024.0 * 1024.0)
                            * 10.0)
                            .round() / 10.0,
                    )
            }
        }
    }
    pub fn render_unit(&self, colors: &Colors, flags: &Flags) -> ColoredString {
        let content = self.unit_string(flags);
        self.paint(colors, flags, content)
    }
    pub fn unit_string(&self, flags: &Flags) -> String {
        let unit = self.get_unit(flags);
        match flags.size {
            SizeFlag::Default => {
                match unit {
                    Unit::None => String::from("-"),
                    Unit::Byte => String::from("B"),
                    Unit::Kilo => String::from("KB"),
                    Unit::Mega => String::from("MB"),
                    Unit::Giga => String::from("GB"),
                    Unit::Tera => String::from("TB"),
                }
            }
            SizeFlag::Short => {
                match unit {
                    Unit::None => String::from("-"),
                    Unit::Byte => String::from("B"),
                    Unit::Kilo => String::from("K"),
                    Unit::Mega => String::from("M"),
                    Unit::Giga => String::from("G"),
                    Unit::Tera => String::from("T"),
                }
            }
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
        let size = Size::new(42);
        let mut flags = Flags::default();
        assert_eq!(size.value_string(& flags).as_str(), "42");
        assert_eq!(size.unit_string(& flags).as_str(), "B");
        flags.size = SizeFlag::Short;
        assert_eq!(size.unit_string(& flags).as_str(), "B");
        flags.size = SizeFlag::Bytes;
        assert_eq!(size.unit_string(& flags).as_str(), "");
    }
    #[test]
    fn render_10_minus_kilobyte() {
        let size = Size::new(4 * 1024);
        let mut flags = Flags::default();
        assert_eq!(size.value_string(& flags).as_str(), "4.0");
        assert_eq!(size.unit_string(& flags).as_str(), "KB");
        flags.size = SizeFlag::Short;
        assert_eq!(size.unit_string(& flags).as_str(), "K");
    }
    #[test]
    fn render_kilobyte() {
        let size = Size::new(42 * 1024);
        let mut flags = Flags::default();
        assert_eq!(size.value_string(& flags).as_str(), "42");
        assert_eq!(size.unit_string(& flags).as_str(), "KB");
        flags.size = SizeFlag::Short;
        assert_eq!(size.unit_string(& flags).as_str(), "K");
    }
    #[test]
    fn render_100_plus_kilobyte() {
        let size = Size::new(420 * 1024 + 420);
        let mut flags = Flags::default();
        assert_eq!(size.value_string(& flags).as_str(), "420");
        assert_eq!(size.unit_string(& flags).as_str(), "KB");
        flags.size = SizeFlag::Short;
        assert_eq!(size.unit_string(& flags).as_str(), "K");
    }
    #[test]
    fn render_10_minus_megabyte() {
        let size = Size::new(4 * 1024 * 1024);
        let mut flags = Flags::default();
        assert_eq!(size.value_string(& flags).as_str(), "4.0");
        assert_eq!(size.unit_string(& flags).as_str(), "MB");
        flags.size = SizeFlag::Short;
        assert_eq!(size.unit_string(& flags).as_str(), "M");
    }
    #[test]
    fn render_megabyte() {
        let size = Size::new(42 * 1024 * 1024);
        let mut flags = Flags::default();
        assert_eq!(size.value_string(& flags).as_str(), "42");
        assert_eq!(size.unit_string(& flags).as_str(), "MB");
        flags.size = SizeFlag::Short;
        assert_eq!(size.unit_string(& flags).as_str(), "M");
    }
    #[test]
    fn render_100_plus_megabyte() {
        let size = Size::new(420 * 1024 * 1024 + 420 * 1024);
        let mut flags = Flags::default();
        assert_eq!(size.value_string(& flags).as_str(), "420");
        assert_eq!(size.unit_string(& flags).as_str(), "MB");
        flags.size = SizeFlag::Short;
        assert_eq!(size.unit_string(& flags).as_str(), "M");
    }
    #[test]
    fn render_10_minus_gigabyte() {
        let size = Size::new(4 * 1024 * 1024 * 1024);
        let mut flags = Flags::default();
        assert_eq!(size.value_string(& flags).as_str(), "4.0");
        assert_eq!(size.unit_string(& flags).as_str(), "GB");
        flags.size = SizeFlag::Short;
        assert_eq!(size.unit_string(& flags).as_str(), "G");
    }
    #[test]
    fn render_gigabyte() {
        let size = Size::new(42 * 1024 * 1024 * 1024);
        let mut flags = Flags::default();
        assert_eq!(size.value_string(& flags).as_str(), "42");
        assert_eq!(size.unit_string(& flags).as_str(), "GB");
        flags.size = SizeFlag::Short;
        assert_eq!(size.unit_string(& flags).as_str(), "G");
    }
    #[test]
    fn render_100_plus_gigabyte() {
        let size = Size::new(420 * 1024 * 1024 * 1024 + 420 * 1024 * 1024);
        let mut flags = Flags::default();
        assert_eq!(size.value_string(& flags).as_str(), "420");
        assert_eq!(size.unit_string(& flags).as_str(), "GB");
        flags.size = SizeFlag::Short;
        assert_eq!(size.unit_string(& flags).as_str(), "G");
    }
    #[test]
    fn render_10_minus_terabyte() {
        let size = Size::new(4 * 1024 * 1024 * 1024 * 1024);
        let mut flags = Flags::default();
        assert_eq!(size.value_string(& flags).as_str(), "4.0");
        assert_eq!(size.unit_string(& flags).as_str(), "TB");
        flags.size = SizeFlag::Short;
        assert_eq!(size.unit_string(& flags).as_str(), "T");
    }
    #[test]
    fn render_terabyte() {
        let size = Size::new(42 * 1024 * 1024 * 1024 * 1024);
        let mut flags = Flags::default();
        assert_eq!(size.value_string(& flags).as_str(), "42");
        assert_eq!(size.unit_string(& flags).as_str(), "TB");
        flags.size = SizeFlag::Short;
        assert_eq!(size.unit_string(& flags).as_str(), "T");
    }
    #[test]
    fn render_100_plus_terabyte() {
        let size = Size::new(420 * 1024 * 1024 * 1024 * 1024 + 420 * 1024 * 1024 * 1024);
        let mut flags = Flags::default();
        assert_eq!(size.value_string(& flags).as_str(), "420");
        assert_eq!(size.unit_string(& flags).as_str(), "TB");
        flags.size = SizeFlag::Short;
        assert_eq!(size.unit_string(& flags).as_str(), "T");
    }
    #[test]
    fn render_with_a_fraction() {
        let size = Size::new(42 * 1024 + 103);
        let flags = Flags::default();
        assert_eq!(size.value_string(& flags).as_str(), "42");
        assert_eq!(size.unit_string(& flags).as_str(), "KB");
    }
    #[test]
    fn render_with_a_truncated_fraction() {
        let size = Size::new(42 * 1024 + 1);
        let flags = Flags::default();
        assert_eq!(size.value_string(& flags).as_str(), "42");
        assert_eq!(size.unit_string(& flags).as_str(), "KB");
    }
    #[test]
    fn render_short_nospaces() {
        let size = Size::new(42 * 1024);
        let mut flags = Flags::default();
        flags.size = SizeFlag::Short;
        let colors = Colors::new(ThemeOption::NoColor);
        assert_eq!(size.render(& colors, & flags, Some(2)).to_string(), "42K");
        assert_eq!(size.render(& colors, & flags, Some(3)).to_string(), " 42K");
    }
}
#[cfg(test)]
mod tests_llm_16_131 {
    use super::*;
    use crate::*;
    use std::fs::Metadata;
    #[test]
    fn test_from() {
        let _rug_st_tests_llm_16_131_rrrruuuugggg_test_from = 0;
        let meta: Metadata = unimplemented!();
        let result: Size = Size::from(&meta);
        let _rug_ed_tests_llm_16_131_rrrruuuugggg_test_from = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_274 {
    use super::*;
    use crate::*;
    #[test]
    fn test_format_size() {
        let _rug_st_tests_llm_16_274_rrrruuuugggg_test_format_size = 0;
        let rug_fuzz_0 = 1024;
        let rug_fuzz_1 = 1024.0;
        let rug_fuzz_2 = 102.5;
        let rug_fuzz_3 = 1024.55;
        let rug_fuzz_4 = 1024;
        let rug_fuzz_5 = 1024;
        let rug_fuzz_6 = 1024.0;
        let rug_fuzz_7 = 1024.5;
        let rug_fuzz_8 = 1024.55;
        let rug_fuzz_9 = 1024;
        let rug_fuzz_10 = 1024;
        let rug_fuzz_11 = 1024;
        let rug_fuzz_12 = 1024.0;
        let rug_fuzz_13 = 1024.5;
        let rug_fuzz_14 = 1024.55;
        let rug_fuzz_15 = 1024;
        let rug_fuzz_16 = 1024;
        let rug_fuzz_17 = 1024;
        let rug_fuzz_18 = 1024;
        let rug_fuzz_19 = 1024.0;
        let rug_fuzz_20 = 1024.5;
        let rug_fuzz_21 = 1024.55;
        let size = Size::new(rug_fuzz_0);
        debug_assert_eq!(size.format_size(rug_fuzz_1), "1024.0");
        debug_assert_eq!(size.format_size(rug_fuzz_2), "102.5");
        debug_assert_eq!(size.format_size(rug_fuzz_3), "1024.6");
        let size = Size::new(rug_fuzz_4 * rug_fuzz_5);
        debug_assert_eq!(size.format_size(rug_fuzz_6), "1024.0");
        debug_assert_eq!(size.format_size(rug_fuzz_7), "1024.5");
        debug_assert_eq!(size.format_size(rug_fuzz_8), "1024.6");
        let size = Size::new(rug_fuzz_9 * rug_fuzz_10 * rug_fuzz_11);
        debug_assert_eq!(size.format_size(rug_fuzz_12), "1024.0");
        debug_assert_eq!(size.format_size(rug_fuzz_13), "1024.5");
        debug_assert_eq!(size.format_size(rug_fuzz_14), "1024.6");
        let size = Size::new(rug_fuzz_15 * rug_fuzz_16 * rug_fuzz_17 * rug_fuzz_18);
        debug_assert_eq!(size.format_size(rug_fuzz_19), "1024.0");
        debug_assert_eq!(size.format_size(rug_fuzz_20), "1024.5");
        debug_assert_eq!(size.format_size(rug_fuzz_21), "1024.6");
        let _rug_ed_tests_llm_16_274_rrrruuuugggg_test_format_size = 0;
    }
}
mod tests_llm_16_275 {
    use super::*;
    use crate::*;
    #[test]
    fn test_get_bytes() {
        let _rug_st_tests_llm_16_275_rrrruuuugggg_test_get_bytes = 0;
        let rug_fuzz_0 = 1024;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 1000000;
        let size = Size::new(rug_fuzz_0);
        debug_assert_eq!(size.get_bytes(), 1024);
        let size = Size::new(rug_fuzz_1);
        debug_assert_eq!(size.get_bytes(), 0);
        let size = Size::new(rug_fuzz_2);
        debug_assert_eq!(size.get_bytes(), 1000000);
        let _rug_ed_tests_llm_16_275_rrrruuuugggg_test_get_bytes = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_276 {
    use super::*;
    use crate::*;
    use crate::flags::*;
    #[test]
    fn test_get_unit_bytes() {
        let _rug_st_tests_llm_16_276_rrrruuuugggg_test_get_unit_bytes = 0;
        let rug_fuzz_0 = 500;
        let size = Size::new(rug_fuzz_0);
        let flags = Flags {
            size: SizeFlag::Bytes,
            ..Default::default()
        };
        let result = size.get_unit(&flags);
        debug_assert_eq!(result, Unit::Byte);
        let _rug_ed_tests_llm_16_276_rrrruuuugggg_test_get_unit_bytes = 0;
    }
    #[test]
    fn test_get_unit_kilo() {
        let _rug_st_tests_llm_16_276_rrrruuuugggg_test_get_unit_kilo = 0;
        let rug_fuzz_0 = 1024;
        let size = Size::new(rug_fuzz_0);
        let flags = Flags {
            size: SizeFlag::Bytes,
            ..Default::default()
        };
        let result = size.get_unit(&flags);
        debug_assert_eq!(result, Unit::Kilo);
        let _rug_ed_tests_llm_16_276_rrrruuuugggg_test_get_unit_kilo = 0;
    }
    #[test]
    fn test_get_unit_mega() {
        let _rug_st_tests_llm_16_276_rrrruuuugggg_test_get_unit_mega = 0;
        let rug_fuzz_0 = 1024;
        let rug_fuzz_1 = 1024;
        let size = Size::new(rug_fuzz_0 * rug_fuzz_1);
        let flags = Flags {
            size: SizeFlag::Bytes,
            ..Default::default()
        };
        let result = size.get_unit(&flags);
        debug_assert_eq!(result, Unit::Mega);
        let _rug_ed_tests_llm_16_276_rrrruuuugggg_test_get_unit_mega = 0;
    }
    #[test]
    fn test_get_unit_giga() {
        let _rug_st_tests_llm_16_276_rrrruuuugggg_test_get_unit_giga = 0;
        let rug_fuzz_0 = 1024;
        let rug_fuzz_1 = 1024;
        let rug_fuzz_2 = 1024;
        let size = Size::new(rug_fuzz_0 * rug_fuzz_1 * rug_fuzz_2);
        let flags = Flags {
            size: SizeFlag::Bytes,
            ..Default::default()
        };
        let result = size.get_unit(&flags);
        debug_assert_eq!(result, Unit::Giga);
        let _rug_ed_tests_llm_16_276_rrrruuuugggg_test_get_unit_giga = 0;
    }
    #[test]
    fn test_get_unit_tera() {
        let _rug_st_tests_llm_16_276_rrrruuuugggg_test_get_unit_tera = 0;
        let rug_fuzz_0 = 1024;
        let rug_fuzz_1 = 1024;
        let rug_fuzz_2 = 1024;
        let rug_fuzz_3 = 1024;
        let size = Size::new(rug_fuzz_0 * rug_fuzz_1 * rug_fuzz_2 * rug_fuzz_3);
        let flags = Flags {
            size: SizeFlag::Bytes,
            ..Default::default()
        };
        let result = size.get_unit(&flags);
        debug_assert_eq!(result, Unit::Tera);
        let _rug_ed_tests_llm_16_276_rrrruuuugggg_test_get_unit_tera = 0;
    }
    #[test]
    fn test_get_unit_default() {
        let _rug_st_tests_llm_16_276_rrrruuuugggg_test_get_unit_default = 0;
        let rug_fuzz_0 = 1024;
        let rug_fuzz_1 = 1024;
        let rug_fuzz_2 = 1024;
        let size = Size::new(rug_fuzz_0 * rug_fuzz_1 * rug_fuzz_2);
        let flags = Flags {
            size: SizeFlag::Default,
            ..Default::default()
        };
        let result = size.get_unit(&flags);
        debug_assert_eq!(result, Unit::Giga);
        let _rug_ed_tests_llm_16_276_rrrruuuugggg_test_get_unit_default = 0;
    }
    #[test]
    fn test_get_unit_short() {
        let _rug_st_tests_llm_16_276_rrrruuuugggg_test_get_unit_short = 0;
        let rug_fuzz_0 = 1024;
        let rug_fuzz_1 = 1024;
        let rug_fuzz_2 = 1024;
        let size = Size::new(rug_fuzz_0 * rug_fuzz_1 * rug_fuzz_2);
        let flags = Flags {
            size: SizeFlag::Short,
            ..Default::default()
        };
        let result = size.get_unit(&flags);
        debug_assert_eq!(result, Unit::Giga);
        let _rug_ed_tests_llm_16_276_rrrruuuugggg_test_get_unit_short = 0;
    }
    #[test]
    fn test_get_unit_bytes_short() {
        let _rug_st_tests_llm_16_276_rrrruuuugggg_test_get_unit_bytes_short = 0;
        let rug_fuzz_0 = 1024;
        let size = Size::new(rug_fuzz_0);
        let flags = Flags {
            size: SizeFlag::Short,
            ..Default::default()
        };
        let result = size.get_unit(&flags);
        debug_assert_eq!(result, Unit::Kilo);
        let _rug_ed_tests_llm_16_276_rrrruuuugggg_test_get_unit_bytes_short = 0;
    }
    #[test]
    fn test_get_unit_bytes_bytes() {
        let _rug_st_tests_llm_16_276_rrrruuuugggg_test_get_unit_bytes_bytes = 0;
        let rug_fuzz_0 = 1024;
        let size = Size::new(rug_fuzz_0);
        let flags = Flags {
            size: SizeFlag::Bytes,
            ..Default::default()
        };
        let result = size.get_unit(&flags);
        debug_assert_eq!(result, Unit::Byte);
        let _rug_ed_tests_llm_16_276_rrrruuuugggg_test_get_unit_bytes_bytes = 0;
    }
    #[test]
    fn test_get_unit_bytes_default() {
        let _rug_st_tests_llm_16_276_rrrruuuugggg_test_get_unit_bytes_default = 0;
        let rug_fuzz_0 = 1024;
        let size = Size::new(rug_fuzz_0);
        let flags = Flags {
            size: SizeFlag::Default,
            ..Default::default()
        };
        let result = size.get_unit(&flags);
        debug_assert_eq!(result, Unit::Kilo);
        let _rug_ed_tests_llm_16_276_rrrruuuugggg_test_get_unit_bytes_default = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_277 {
    use super::*;
    use crate::*;
    #[test]
    fn test_new() {
        let _rug_st_tests_llm_16_277_rrrruuuugggg_test_new = 0;
        let rug_fuzz_0 = 100;
        let bytes = rug_fuzz_0;
        let size = Size::new(bytes);
        debug_assert_eq!(size.get_bytes(), bytes);
        let _rug_ed_tests_llm_16_277_rrrruuuugggg_test_new = 0;
    }
}
#[cfg(test)]
mod tests_rug_113 {
    use super::*;
    use crate::meta::size::{Size, Unit};
    use crate::flags::{Flags, SizeFlag};
    #[test]
    fn test_unit_string() {
        let _rug_st_tests_rug_113_rrrruuuugggg_test_unit_string = 0;
        let rug_fuzz_0 = 1024;
        let size = Size::new(rug_fuzz_0);
        let flags = Flags::default();
        debug_assert_eq!(size.unit_string(& flags), "KB");
        let _rug_ed_tests_rug_113_rrrruuuugggg_test_unit_string = 0;
    }
}
