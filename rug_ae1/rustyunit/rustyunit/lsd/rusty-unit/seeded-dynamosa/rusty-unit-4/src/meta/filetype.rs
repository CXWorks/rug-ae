use crate::color::{ColoredString, Colors, Elem};
use crate::meta::Permissions;
use std::fs::Metadata;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(windows, allow(dead_code))]
pub enum FileType {
    BlockDevice,
    CharDevice,
    Directory { uid: bool },
    File { uid: bool, exec: bool },
    SymLink { is_dir: bool },
    Pipe,
    Socket,
    Special,
}

impl FileType {
    #[cfg(unix)]
    pub fn new(
        meta: &Metadata,
        symlink_meta: Option<&Metadata>,
        permissions: &Permissions,
    ) -> Self {
        use std::os::unix::fs::FileTypeExt;

        let file_type = meta.file_type();

        if file_type.is_file() {
            FileType::File {
                exec: permissions.is_executable(),
                uid: permissions.setuid,
            }
        } else if file_type.is_dir() {
            FileType::Directory {
                uid: permissions.setuid,
            }
        } else if file_type.is_fifo() {
            FileType::Pipe
        } else if file_type.is_symlink() {
            FileType::SymLink {
                // if broken, defaults to false
                is_dir: symlink_meta.map(|m| m.is_dir()).unwrap_or_default(),
            }
        } else if file_type.is_char_device() {
            FileType::CharDevice
        } else if file_type.is_block_device() {
            FileType::BlockDevice
        } else if file_type.is_socket() {
            FileType::Socket
        } else {
            FileType::Special
        }
    }

    #[cfg(windows)]
    pub fn new(
        meta: &Metadata,
        symlink_meta: Option<&Metadata>,
        permissions: &Permissions,
    ) -> Self {
        let file_type = meta.file_type();

        if file_type.is_file() {
            FileType::File {
                exec: permissions.is_executable(),
                uid: permissions.setuid,
            }
        } else if file_type.is_dir() {
            FileType::Directory {
                uid: permissions.setuid,
            }
        } else if file_type.is_symlink() {
            FileType::SymLink {
                // if broken, defaults to false
                is_dir: symlink_meta.map(|m| m.is_dir()).unwrap_or_default(),
            }
        } else {
            FileType::Special
        }
    }

    pub fn is_dirlike(self) -> bool {
        matches!(
            self,
            FileType::Directory { .. } | FileType::SymLink { is_dir: true }
        )
    }
}

impl FileType {
    pub fn render(self, colors: &Colors) -> ColoredString {
        match self {
            FileType::File { exec, .. } => {
                colors.colorize(String::from("."), &Elem::File { exec, uid: false })
            }
            FileType::Directory { .. } => {
                colors.colorize(String::from("d"), &Elem::Dir { uid: false })
            }
            FileType::Pipe => colors.colorize(String::from("|"), &Elem::Pipe),
            FileType::SymLink { .. } => colors.colorize(String::from("l"), &Elem::SymLink),
            FileType::BlockDevice => colors.colorize(String::from("b"), &Elem::BlockDevice),
            FileType::CharDevice => colors.colorize(String::from("c"), &Elem::CharDevice),
            FileType::Socket => colors.colorize(String::from("s"), &Elem::Socket),
            FileType::Special => colors.colorize(String::from("?"), &Elem::Special),
        }
    }
}

#[cfg(test)]
mod test {
    use super::FileType;
    use crate::color::{Colors, ThemeOption};
    use crate::meta::Meta;
    #[cfg(unix)]
    use crate::meta::Permissions;
    use crossterm::style::{Color, Stylize};
    #[cfg(unix)]
    use std::fs::File;
    #[cfg(unix)]
    use std::os::unix::fs::symlink;
    #[cfg(unix)]
    use std::os::unix::net::UnixListener;
    #[cfg(unix)]
    use std::process::Command;
    use tempfile::tempdir;

    #[test]
    #[cfg(unix)] // Windows uses different default permissions
    fn test_file_type() {
        let tmp_dir = tempdir().expect("failed to create temp dir");

        // Create the file;
        let file_path = tmp_dir.path().join("file.txt");
        File::create(&file_path).expect("failed to create file");
        let meta = file_path.metadata().expect("failed to get metas");

        let colors = Colors::new(ThemeOption::NoLscolors);
        let file_type = FileType::new(&meta, None, &Permissions::from(&meta));

        assert_eq!(
            ".".to_string().with(Color::AnsiValue(184)),
            file_type.render(&colors)
        );
    }

    #[test]
    fn test_dir_type() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let meta = Meta::from_path(&tmp_dir.path().to_path_buf(), false)
            .expect("failed to get tempdir path");
        let metadata = tmp_dir.path().metadata().expect("failed to get metas");

        let colors = Colors::new(ThemeOption::NoLscolors);
        let file_type = FileType::new(&metadata, None, &meta.permissions);

        assert_eq!(
            "d".to_string().with(Color::AnsiValue(33)),
            file_type.render(&colors)
        );
    }

    #[test]
    #[cfg(unix)] // Symlink support is *hard* on Windows
    fn test_symlink_type_file() {
        let tmp_dir = tempdir().expect("failed to create temp dir");

        // Create the file;
        let file_path = tmp_dir.path().join("file.tmp");
        File::create(&file_path).expect("failed to create file");

        // Create the symlink
        let symlink_path = tmp_dir.path().join("target.tmp");
        symlink(&file_path, &symlink_path).expect("failed to create symlink");
        let meta = symlink_path
            .symlink_metadata()
            .expect("failed to get metas");

        let colors = Colors::new(ThemeOption::NoLscolors);
        let file_type = FileType::new(&meta, Some(&meta), &Permissions::from(&meta));

        assert_eq!(
            "l".to_string().with(Color::AnsiValue(44)),
            file_type.render(&colors)
        );
    }

    #[test]
    #[cfg(unix)]
    fn test_symlink_type_dir() {
        let tmp_dir = tempdir().expect("failed to create temp dir");

        // Create directory
        let dir_path = tmp_dir.path().join("dir.d");
        std::fs::create_dir(&dir_path).expect("failed to create dir");

        // Create symlink
        let symlink_path = tmp_dir.path().join("target.d");
        symlink(&dir_path, &symlink_path).expect("failed to create symlink");
        let meta = symlink_path
            .symlink_metadata()
            .expect("failed to get metas");

        let colors = Colors::new(ThemeOption::NoLscolors);
        let file_type = FileType::new(&meta, Some(&meta), &Permissions::from(&meta));

        assert_eq!(
            "l".to_string().with(Color::AnsiValue(44)),
            file_type.render(&colors)
        );
    }

    #[test]
    #[cfg(unix)] // Windows pipes aren't like Unix pipes
    fn test_pipe_type() {
        let tmp_dir = tempdir().expect("failed to create temp dir");

        // Create the pipe;
        let pipe_path = tmp_dir.path().join("pipe.tmp");
        let success = Command::new("mkfifo")
            .arg(&pipe_path)
            .status()
            .expect("failed to exec mkfifo")
            .success();
        assert_eq!(true, success, "failed to exec mkfifo");
        let meta = pipe_path.metadata().expect("failed to get metas");

        let colors = Colors::new(ThemeOption::NoLscolors);
        let file_type = FileType::new(&meta, None, &Permissions::from(&meta));

        assert_eq!(
            "|".to_string().with(Color::AnsiValue(44)),
            file_type.render(&colors)
        );
    }

    #[test]
    #[cfg(feature = "sudo")]
    fn test_char_device_type() {
        let tmp_dir = tempdir().expect("failed to create temp dir");

        // Create the char device;
        let char_device_path = tmp_dir.path().join("char-device.tmp");
        let success = Command::new("sudo")
            .arg("mknod")
            .arg(&char_device_path)
            .arg("c")
            .arg("89")
            .arg("1")
            .status()
            .expect("failed to exec mknod")
            .success();
        assert_eq!(true, success, "failed to exec mknod");
        let meta = char_device_path.metadata().expect("failed to get metas");

        let colors = Colors::new(ThemeOption::NoLscolors);
        let file_type = FileType::new(&meta, None, &Permissions::from(&meta));

        assert_eq!(
            "c".to_string().with(Color::AnsiValue(44)),
            file_type.render(&colors)
        );
    }

    #[test]
    #[cfg(unix)] // Sockets don't work the same way on Windows
    fn test_socket_type() {
        let tmp_dir = tempdir().expect("failed to create temp dir");

        // Create the socket;
        let socket_path = tmp_dir.path().join("socket.tmp");
        UnixListener::bind(&socket_path).expect("failed to create the socket");
        let meta = socket_path.metadata().expect("failed to get metas");

        let colors = Colors::new(ThemeOption::NoLscolors);
        let file_type = FileType::new(&meta, None, &Permissions::from(&meta));

        assert_eq!(
            "s".to_string().with(Color::AnsiValue(44)),
            file_type.render(&colors)
        );
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
#[timeout(30000)]fn rusty_test_6833() {
//    rusty_monitor::set_test_id(6833);
    let mut option_0: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut str_0: &str = "%F %R";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut dateflag_0_ref_0: &flags::date::DateFlag = &mut dateflag_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut u64_0: u64 = 20u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut filetype_0_ref_0: &meta::filetype::FileType = &mut filetype_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::clone(filetype_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3904() {
//    rusty_monitor::set_test_id(3904);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut dateflag_0_ref_0: &flags::date::DateFlag = &mut dateflag_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut u64_0: u64 = 20u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut str_0: &str = "failed to retrieve modified date";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut u64_1: u64 = 0u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut dateflag_1_ref_0: &flags::date::DateFlag = &mut dateflag_1;
    let mut dateflag_2: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut dateflag_2_ref_0: &flags::date::DateFlag = &mut dateflag_2;
    let mut dateflag_3: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut dateflag_3_ref_0: &flags::date::DateFlag = &mut dateflag_3;
    let mut dateflag_4: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut dateflag_4_ref_0: &flags::date::DateFlag = &mut dateflag_4;
    let mut filetype_0_ref_0: &meta::filetype::FileType = &mut filetype_0;
    let mut tuple_0: () = crate::meta::filetype::FileType::assert_receiver_is_total_eq(filetype_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_535() {
//    rusty_monitor::set_test_id(535);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut filetype_0_ref_0: &meta::filetype::FileType = &mut filetype_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut filetype_1_ref_0: &meta::filetype::FileType = &mut filetype_1;
    let mut bool_0: bool = false;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_0};
    let mut filetype_2_ref_0: &meta::filetype::FileType = &mut filetype_2;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut filetype_3_ref_0: &meta::filetype::FileType = &mut filetype_3;
    let mut filetype_4: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut filetype_4_ref_0: &meta::filetype::FileType = &mut filetype_4;
    let mut filetype_5: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut filetype_5_ref_0: &meta::filetype::FileType = &mut filetype_5;
    let mut filetype_6: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut filetype_6_ref_0: &meta::filetype::FileType = &mut filetype_6;
    let mut bool_1: bool = true;
    let mut filetype_7: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_1};
    let mut filetype_7_ref_0: &meta::filetype::FileType = &mut filetype_7;
    let mut filetype_8: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut filetype_8_ref_0: &meta::filetype::FileType = &mut filetype_8;
    let mut bool_2: bool = true;
    let mut filetype_9: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_2};
    let mut filetype_9_ref_0: &meta::filetype::FileType = &mut filetype_9;
    let mut filetype_10: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut filetype_10_ref_0: &meta::filetype::FileType = &mut filetype_10;
    let mut filetype_11: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut filetype_11_ref_0: &meta::filetype::FileType = &mut filetype_11;
    let mut bool_3: bool = crate::meta::filetype::FileType::ne(filetype_11_ref_0, filetype_10_ref_0);
    let mut bool_4: bool = crate::meta::filetype::FileType::ne(filetype_9_ref_0, filetype_8_ref_0);
    let mut bool_5: bool = crate::meta::filetype::FileType::ne(filetype_7_ref_0, filetype_6_ref_0);
    let mut bool_6: bool = crate::meta::filetype::FileType::ne(filetype_5_ref_0, filetype_4_ref_0);
    let mut bool_7: bool = crate::meta::filetype::FileType::ne(filetype_3_ref_0, filetype_2_ref_0);
    let mut bool_8: bool = crate::meta::filetype::FileType::ne(filetype_1_ref_0, filetype_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2591() {
//    rusty_monitor::set_test_id(2591);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut option_0: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut dateflag_0_ref_0: &flags::date::DateFlag = &mut dateflag_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut u64_0: u64 = 20u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_112() {
//    rusty_monitor::set_test_id(112);
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
    let mut usize_0: usize = 80usize;
    let mut option_7: std::option::Option<usize> = std::option::Option::Some(usize_0);
    let mut option_8: std::option::Option<bool> = std::option::Option::None;
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_8, depth: option_7};
    let mut option_9: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_0);
    let mut option_10: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_1: bool = false;
    let mut option_11: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_12: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_13: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_14: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_15: std::option::Option<bool> = std::option::Option::None;
    let mut option_16: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_17: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_18: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_2: bool = true;
    let mut option_19: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_19, blocks: option_18, color: option_17, date: option_16, dereference: option_15, display: option_14, icons: option_13, ignore_globs: option_12, indicators: option_11, layout: option_10, recursion: option_9, size: option_6, permission: option_5, sorting: option_4, no_symlink: option_3, total_size: option_2, symlink_arrow: option_1, hyperlink: option_0};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut filetype_0_ref_0: &meta::filetype::FileType = &mut filetype_0;
    let mut elem_0: color::Elem = crate::color::Elem::Context;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_1: color::Elem = crate::color::Elem::DayOld;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut filetype_1_ref_0: &meta::filetype::FileType = &mut filetype_1;
    let mut bool_3: bool = crate::meta::filetype::FileType::eq(filetype_1_ref_0, filetype_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3685() {
//    rusty_monitor::set_test_id(3685);
    let mut bool_0: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_0};
    let mut filetype_0_ref_0: &meta::filetype::FileType = &mut filetype_0;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut option_1: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut str_0: &str = "%F %R";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut u64_0: u64 = 20u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut filetype_1_ref_0: &meta::filetype::FileType = &mut filetype_1;
    let mut bool_1: bool = crate::meta::filetype::FileType::ne(filetype_1_ref_0, filetype_0_ref_0);
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3010() {
//    rusty_monitor::set_test_id(3010);
    let mut bool_0: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_0};
    let mut filetype_0_ref_0: &meta::filetype::FileType = &mut filetype_0;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut dateflag_0_ref_0: &flags::date::DateFlag = &mut dateflag_0;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut dateflag_1_ref_0: &flags::date::DateFlag = &mut dateflag_1;
    let mut dateflag_2: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut dateflag_2_ref_0: &flags::date::DateFlag = &mut dateflag_2;
    let mut dateflag_3: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut dateflag_3_ref_0: &flags::date::DateFlag = &mut dateflag_3;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut u64_0: u64 = 72u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut str_0: &str = "keNUuqNt";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut dateflag_4: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut dateflag_4_ref_0: &flags::date::DateFlag = &mut dateflag_4;
    let mut filetype_1_ref_0: &meta::filetype::FileType = &mut filetype_1;
    let mut bool_1: bool = crate::meta::filetype::FileType::eq(filetype_1_ref_0, filetype_0_ref_0);
//    panic!("From RustyUnit with love");
}
}