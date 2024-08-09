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
	use std::clone::Clone;
	use std::convert::From;
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_674() {
//    rusty_monitor::set_test_id(674);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut filetype_4: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut filetype_5: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut bool_0: bool = false;
    let mut filetype_6: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_0};
    let mut filetype_7: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut filetype_8: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut filetype_9: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut bool_1: bool = true;
    let mut bool_2: bool = false;
    let mut filetype_10: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_2, exec: bool_1};
    let mut bool_3: bool = false;
    let mut filetype_11: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_3};
    let mut bool_4: bool = true;
    let mut bool_5: bool = true;
    let mut filetype_12: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_5, exec: bool_4};
    let mut indicator_0: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_12);
    let mut indicator_1: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_11);
    let mut indicator_2: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_10);
    let mut indicator_3: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_9);
    let mut indicator_4: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_8);
    let mut indicator_5: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_7);
    let mut indicator_6: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_6);
    let mut indicator_7: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_5);
    let mut indicator_8: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_4);
    let mut indicator_9: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_3);
    let mut indicator_10: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_2);
    let mut indicator_11: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_1);
    let mut indicator_12: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_420() {
//    rusty_monitor::set_test_id(420);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut indicator_0: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_0);
    let mut indicator_0_ref_0: &crate::meta::indicator::Indicator = &mut indicator_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut indicator_1: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_1);
    let mut indicator_1_ref_0: &crate::meta::indicator::Indicator = &mut indicator_1;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_1, exec: bool_0};
    let mut indicator_2: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_2);
    let mut indicator_2_ref_0: &crate::meta::indicator::Indicator = &mut indicator_2;
    let mut bool_2: bool = true;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_2};
    let mut indicator_3: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_3);
    let mut indicator_3_ref_0: &crate::meta::indicator::Indicator = &mut indicator_3;
    let mut filetype_4: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut indicator_4: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_4);
    let mut indicator_4_ref_0: &crate::meta::indicator::Indicator = &mut indicator_4;
    let mut filetype_5: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut indicator_5: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_5);
    let mut indicator_5_ref_0: &crate::meta::indicator::Indicator = &mut indicator_5;
    let mut bool_3: bool = true;
    let mut filetype_6: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_3};
    let mut indicator_6: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_6);
    let mut indicator_6_ref_0: &crate::meta::indicator::Indicator = &mut indicator_6;
    let mut filetype_7: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut indicator_7: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_7);
    let mut indicator_7_ref_0: &crate::meta::indicator::Indicator = &mut indicator_7;
    let mut bool_4: bool = true;
    let mut bool_5: bool = true;
    let mut filetype_8: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_5, exec: bool_4};
    let mut indicator_8: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_8);
    let mut indicator_8_ref_0: &crate::meta::indicator::Indicator = &mut indicator_8;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_438() {
//    rusty_monitor::set_test_id(438);
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_1, exec: bool_0};
    let mut indicator_0: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_0);
    let mut indicator_0_ref_0: &crate::meta::indicator::Indicator = &mut indicator_0;
    let mut bool_2: bool = false;
    let mut bool_3: bool = true;
    let mut bool_4: bool = false;
    let mut bool_5: bool = true;
    let mut bool_6: bool = true;
    let mut bool_7: bool = false;
    let mut bool_8: bool = false;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = true;
    let mut bool_12: bool = true;
    let mut bool_13: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_13, user_write: bool_12, user_execute: bool_11, group_read: bool_10, group_write: bool_9, group_execute: bool_8, other_read: bool_7, other_write: bool_6, other_execute: bool_5, sticky: bool_4, setgid: bool_3, setuid: bool_2};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut indicator_1: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_1);
    let mut indicator_1_ref_0: &crate::meta::indicator::Indicator = &mut indicator_1;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut indicator_2: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_2);
    let mut indicator_2_ref_0: &crate::meta::indicator::Indicator = &mut indicator_2;
    let mut bool_14: bool = false;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_14};
    let mut indicator_3: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_3);
    let mut indicator_3_ref_0: &crate::meta::indicator::Indicator = &mut indicator_3;
    let mut filetype_4: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut indicator_4: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_4);
    let mut indicator_4_ref_0: &crate::meta::indicator::Indicator = &mut indicator_4;
    let mut filetype_5: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut indicator_5: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_5);
    let mut indicator_5_ref_0: &crate::meta::indicator::Indicator = &mut indicator_5;
    let mut filetype_6: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut indicator_6: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_6);
    let mut indicator_6_ref_0: &crate::meta::indicator::Indicator = &mut indicator_6;
    let mut indicator_7: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::clone(indicator_6_ref_0);
    let mut indicator_8: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::clone(indicator_5_ref_0);
    let mut indicator_9: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::clone(indicator_4_ref_0);
    let mut indicator_10: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::clone(indicator_3_ref_0);
    let mut indicator_11: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::clone(indicator_2_ref_0);
    let mut indicator_12: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::clone(indicator_1_ref_0);
    let mut indicator_13: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::clone(indicator_0_ref_0);
//    panic!("From RustyUnit with love");
}
}