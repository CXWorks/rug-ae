use crate::color::{ColoredString, Colors};
use crate::flags::Flags;
use crate::meta::FileType;

#[derive(Clone, Debug)]
pub struct Indicator(&'static str);

impl From<FileType> for Indicator {
    fn from(file_type: FileType) -> Self {
        let res = match file_type {
            FileType::Directory { .. } => "/",
            FileType::File { exec: true, .. } => "*",
            FileType::Pipe => "|",
            FileType::Socket => "=",
            FileType::SymLink { .. } => "@",
            _ => "",
        };

        Indicator(res)
    }
}

impl Indicator {
    pub fn render(&self, flags: &Flags) -> ColoredString {
        if flags.display_indicators.0 {
            ColoredString::new(Colors::default_style(), self.0.to_string())
        } else {
            ColoredString::new(Colors::default_style(), "".into())
        }
    }
}

#[cfg(test)]
mod test {
    use super::Indicator;
    use crate::flags::{Flags, Indicators};
    use crate::meta::FileType;

    #[test]
    fn test_directory_indicator() {
        let mut flags = Flags::default();
        flags.display_indicators = Indicators(true);

        let file_type = Indicator::from(FileType::Directory { uid: false });

        assert_eq!("/", file_type.render(&flags).to_string().as_str());
    }

    #[test]
    fn test_executable_file_indicator() {
        let mut flags = Flags::default();
        flags.display_indicators = Indicators(true);

        let file_type = Indicator::from(FileType::File {
            uid: false,
            exec: true,
        });

        assert_eq!("*", file_type.render(&flags).to_string().as_str());
    }

    #[test]
    fn test_socket_indicator() {
        let mut flags = Flags::default();
        flags.display_indicators = Indicators(true);

        let file_type = Indicator::from(FileType::Socket);

        assert_eq!("=", file_type.render(&flags).to_string().as_str());
    }

    #[test]
    fn test_symlink_indicator() {
        let mut flags = Flags::default();
        flags.display_indicators = Indicators(true);

        let file_type = Indicator::from(FileType::SymLink { is_dir: false });
        assert_eq!("@", file_type.render(&flags).to_string().as_str());

        let file_type = Indicator::from(FileType::SymLink { is_dir: true });
        assert_eq!("@", file_type.render(&flags).to_string().as_str());
    }

    #[test]
    fn test_not_represented_indicator() {
        let mut flags = Flags::default();
        flags.display_indicators = Indicators(true);

        // The File type doesn't have any indicator
        let file_type = Indicator::from(FileType::File {
            exec: false,
            uid: false,
        });

        assert_eq!("", file_type.render(&flags).to_string().as_str());
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7525() {
    rusty_monitor::set_test_id(7525);
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut str_0: &str = "5b4CeamG2UpKV";
    let mut u64_0: u64 = 9329u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut u64_1: u64 = 6730u64;
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
    let mut elem_17_ref_0: &color::Elem = &mut elem_17;
    let mut elem_18: color::Elem = crate::color::Elem::DayOld;
    let mut elem_18_ref_0: &color::Elem = &mut elem_18;
    let mut elem_19: color::Elem = crate::color::Elem::Write;
    let mut elem_19_ref_0: &color::Elem = &mut elem_19;
    let mut elem_20: color::Elem = crate::color::Elem::Group;
    let mut elem_20_ref_0: &color::Elem = &mut elem_20;
    let mut elem_21: color::Elem = crate::color::Elem::SymLink;
    let mut elem_21_ref_0: &color::Elem = &mut elem_21;
    let mut elem_22: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_22_ref_0: &color::Elem = &mut elem_22;
    let mut elem_23: color::Elem = crate::color::Elem::Special;
    let mut elem_23_ref_0: &color::Elem = &mut elem_23;
    let mut elem_24: color::Elem = crate::color::Elem::User;
    let mut elem_24_ref_0: &color::Elem = &mut elem_24;
    let mut elem_25: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_25_ref_0: &color::Elem = &mut elem_25;
    let mut u64_2: u64 = 7953u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut u64_3: u64 = 7000u64;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size::new(u64_2);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut size_3: crate::meta::size::Size = crate::meta::size::Size::new(u64_3);
    let mut size_2_ref_0: &crate::meta::size::Size = &mut size_2;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut filetype_1_ref_0: &meta::filetype::FileType = &mut filetype_1;
    let mut filetype_4: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut filetype_0_ref_0: &meta::filetype::FileType = &mut filetype_0;
    panic!("From RustyUnit with love");
}
}