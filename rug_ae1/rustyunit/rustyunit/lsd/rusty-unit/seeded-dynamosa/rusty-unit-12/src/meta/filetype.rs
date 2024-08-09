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
	use std::cmp::PartialEq;
	use std::clone::Clone;
	use std::cmp::Eq;
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1606() {
//    rusty_monitor::set_test_id(1606);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut filetype_0_ref_0: &meta::filetype::FileType = &mut filetype_0;
    let mut str_0: &str = "I9VRcx9HMz7n";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "TotalSize";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut option_0: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_1: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_2: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_3: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_4: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut str_2: &str = "selinux_context";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut bool_3: bool = false;
    let mut bool_4: bool = true;
    let mut bool_5: bool = true;
    let mut bool_6: bool = false;
    let mut bool_7: bool = false;
    let mut bool_8: bool = true;
    let mut bool_9: bool = true;
    let mut bool_10: bool = true;
    let mut bool_11: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_5: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut option_6: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_3: &str = ".clang-format";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut option_7: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_3_ref_0);
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut option_8: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut str_4: &str = "icons";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "jsx";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut option_9: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_10: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_11: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_12: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_13: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_0: usize = 1usize;
    let mut bool_12: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_12, depth: usize_0};
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut display_1: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut elem_0: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_1: color::Elem = crate::color::Elem::NoAccess;
    let mut permissionflag_0_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_0;
    let mut filetype_1_ref_0: &meta::filetype::FileType = &mut filetype_1;
    let mut bool_13: bool = crate::meta::filetype::FileType::eq(filetype_1_ref_0, filetype_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_670() {
//    rusty_monitor::set_test_id(670);
    let mut elem_0: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_1: color::Elem = crate::color::Elem::SymLink;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut elem_2: color::Elem = crate::color::Elem::File {exec: bool_1, uid: bool_0};
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_3: color::Elem = crate::color::Elem::Pipe;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut elem_4: color::Elem = crate::color::Elem::Read;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut elem_5: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut elem_6: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut elem_7: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut elem_8: color::Elem = crate::color::Elem::ExecSticky;
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut elem_9: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    let mut bool_2: bool = crate::color::Elem::has_suid(elem_9_ref_0);
    let mut bool_3: bool = crate::color::Elem::has_suid(elem_8_ref_0);
    let mut bool_4: bool = crate::color::Elem::has_suid(elem_7_ref_0);
    let mut bool_5: bool = crate::color::Elem::has_suid(elem_6_ref_0);
    let mut bool_6: bool = crate::color::Elem::has_suid(elem_5_ref_0);
    let mut bool_7: bool = crate::color::Elem::has_suid(elem_4_ref_0);
    let mut bool_8: bool = crate::color::Elem::has_suid(elem_3_ref_0);
    let mut bool_9: bool = crate::color::Elem::has_suid(elem_2_ref_0);
    let mut bool_10: bool = crate::color::Elem::has_suid(elem_1_ref_0);
    let mut bool_11: bool = crate::color::Elem::has_suid(elem_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_381() {
//    rusty_monitor::set_test_id(381);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut filetype_0_ref_0: &meta::filetype::FileType = &mut filetype_0;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut bool_2: bool = false;
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
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut bool_12: bool = true;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_12};
    let mut filetype_1_ref_0: &meta::filetype::FileType = &mut filetype_1;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut filetype_2_ref_0: &meta::filetype::FileType = &mut filetype_2;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut filetype_3_ref_0: &meta::filetype::FileType = &mut filetype_3;
    let mut filetype_4: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut filetype_4_ref_0: &meta::filetype::FileType = &mut filetype_4;
    let mut filetype_5: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut filetype_5_ref_0: &meta::filetype::FileType = &mut filetype_5;
    let mut bool_13: bool = true;
    let mut bool_14: bool = false;
    let mut filetype_6: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_14, exec: bool_13};
    let mut filetype_6_ref_0: &meta::filetype::FileType = &mut filetype_6;
    let mut filetype_7: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut filetype_7_ref_0: &meta::filetype::FileType = &mut filetype_7;
    let mut bool_15: bool = crate::meta::filetype::FileType::ne(filetype_7_ref_0, filetype_6_ref_0);
    let mut bool_16: bool = crate::meta::filetype::FileType::ne(filetype_5_ref_0, filetype_4_ref_0);
    let mut bool_17: bool = crate::meta::filetype::FileType::ne(filetype_3_ref_0, filetype_2_ref_0);
    let mut bool_18: bool = crate::meta::filetype::FileType::ne(filetype_1_ref_0, filetype_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4545() {
//    rusty_monitor::set_test_id(4545);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut dateflag_0_ref_0: &flags::date::DateFlag = &mut dateflag_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut str_0: &str = "X9L";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut u64_0: u64 = 1073741824u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut u64_1: u64 = 1073741824u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut filetype_1_ref_0: &meta::filetype::FileType = &mut filetype_1;
    let mut tuple_0: () = crate::meta::filetype::FileType::assert_receiver_is_total_eq(filetype_1_ref_0);
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8110() {
//    rusty_monitor::set_test_id(8110);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut dateflag_0_ref_0: &flags::date::DateFlag = &mut dateflag_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut str_0: &str = "X9L";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut u64_0: u64 = 1073741824u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut u64_1: u64 = 1073741824u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    crate::meta::filetype::FileType::render(filetype_1, colors_0_ref_0);
    let mut filetype_0_ref_0: &meta::filetype::FileType = &mut filetype_0;
    let mut tuple_0: () = crate::meta::filetype::FileType::assert_receiver_is_total_eq(filetype_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2266() {
//    rusty_monitor::set_test_id(2266);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut dateflag_0_ref_0: &flags::date::DateFlag = &mut dateflag_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut str_0: &str = "X9L";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut u64_0: u64 = 1073741824u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut u64_1: u64 = 1073741824u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut filetype_1_ref_0: &meta::filetype::FileType = &mut filetype_1;
    let mut filetype_4: meta::filetype::FileType = crate::meta::filetype::FileType::clone(filetype_1_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_224() {
//    rusty_monitor::set_test_id(224);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut filetype_0_ref_0: &meta::filetype::FileType = &mut filetype_0;
    let mut bool_0: bool = true;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_0};
    let mut filetype_1_ref_0: &meta::filetype::FileType = &mut filetype_1;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut filetype_2_ref_0: &meta::filetype::FileType = &mut filetype_2;
    let mut bool_1: bool = true;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_1};
    let mut filetype_3_ref_0: &meta::filetype::FileType = &mut filetype_3;
    let mut filetype_4: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut filetype_4_ref_0: &meta::filetype::FileType = &mut filetype_4;
    let mut bool_2: bool = true;
    let mut filetype_5: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_2};
    let mut filetype_5_ref_0: &meta::filetype::FileType = &mut filetype_5;
    let mut filetype_6: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut filetype_6_ref_0: &meta::filetype::FileType = &mut filetype_6;
    let mut bool_3: bool = true;
    let mut bool_4: bool = false;
    let mut filetype_7: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_4, exec: bool_3};
    let mut filetype_7_ref_0: &meta::filetype::FileType = &mut filetype_7;
    let mut filetype_8: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut filetype_8_ref_0: &meta::filetype::FileType = &mut filetype_8;
    let mut bool_5: bool = false;
    let mut filetype_9: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_5};
    let mut filetype_9_ref_0: &meta::filetype::FileType = &mut filetype_9;
    let mut bool_6: bool = crate::meta::filetype::FileType::eq(filetype_9_ref_0, filetype_8_ref_0);
    let mut bool_7: bool = crate::meta::filetype::FileType::eq(filetype_7_ref_0, filetype_6_ref_0);
    let mut bool_8: bool = crate::meta::filetype::FileType::eq(filetype_5_ref_0, filetype_4_ref_0);
    let mut bool_9: bool = crate::meta::filetype::FileType::eq(filetype_3_ref_0, filetype_2_ref_0);
    let mut bool_10: bool = crate::meta::filetype::FileType::eq(filetype_1_ref_0, filetype_0_ref_0);
//    panic!("From RustyUnit with love");
}
}