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
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1299() {
    rusty_monitor::set_test_id(1299);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut elem_0: color::Elem = crate::color::Elem::File {exec: bool_1, uid: bool_0};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut elem_1: color::Elem = crate::color::Elem::Write;
    let mut elem_2: color::Elem = crate::color::Elem::Older;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut bool_2: bool = false;
    let mut bool_3: bool = false;
    let mut option_0: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut option_2: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_0);
    let mut option_3: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_3, theme: option_2, separator: option_1};
    let mut option_4: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut option_5: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut option_6: std::option::Option<bool> = std::option::Option::None;
    let mut option_7: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_8: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_9: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_10: std::option::Option<bool> = std::option::Option::None;
    let mut u64_0: u64 = 62u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_3, exec: bool_2};
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Date;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1274() {
    rusty_monitor::set_test_id(1274);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::SymLink;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut elem_3: color::Elem = crate::color::Elem::Write;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut option_0: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_0: &str = "UArmSBj";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_1: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_4: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_3_ref_0);
    let mut bool_0: bool = false;
    let mut elem_5: color::Elem = crate::color::Elem::Dir {uid: bool_0};
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut elem_6: color::Elem = crate::color::Elem::FileSmall;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut u64_0: u64 = 7317u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut u64_1: u64 = 5684u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut str_1: &str = "M15PWEBaeH";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut layout_0_ref_0: &flags::layout::Layout = &mut layout_0;
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut layout_1_ref_0: &flags::layout::Layout = &mut layout_1;
    let mut elem_7: color::Elem = crate::color::Elem::NonFile;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut bool_1: bool = crate::color::Elem::has_suid(elem_5_ref_0);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut symlink_0: crate::color::theme::Symlink = crate::color::theme::Symlink {default: color_2, broken: color_1, missing_target: color_0};
    let mut bool_2: bool = crate::meta::filetype::FileType::is_dirlike(filetype_0);
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut filetype_1_ref_0: &meta::filetype::FileType = &mut filetype_1;
    let mut tuple_0: () = crate::meta::filetype::FileType::assert_receiver_is_total_eq(filetype_1_ref_0);
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6384() {
    rusty_monitor::set_test_id(6384);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut filetype_0_ref_0: &meta::filetype::FileType = &mut filetype_0;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut iconoption_0_ref_0: &flags::icons::IconOption = &mut iconoption_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut str_0: &str = "24VzMtiXUyN";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_0: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_0: bool = true;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_0};
    let mut str_1: &str = "T9vI1O0";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut option_1: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut option_2: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_2, theme: option_1, separator: option_0};
    let mut option_3: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut option_4: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_5: std::option::Option<bool> = std::option::Option::None;
    let mut option_6: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_7: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_8: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_9: std::option::Option<bool> = std::option::Option::None;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_0: color::Elem = crate::color::Elem::SymLink;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_1: color::Elem = crate::color::Elem::Write;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut option_10: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_2: &str = "UArmSBj";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut option_11: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_2_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_2: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_4_ref_0);
    let mut bool_1: bool = false;
    let mut elem_3: color::Elem = crate::color::Elem::Dir {uid: bool_1};
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut elem_4: color::Elem = crate::color::Elem::User;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_0_ref_0);
    let mut filetype_3_ref_0: &meta::filetype::FileType = &mut filetype_3;
    let mut filetype_4: meta::filetype::FileType = crate::meta::filetype::FileType::clone(filetype_3_ref_0);
    let mut filetype_4_ref_0: &meta::filetype::FileType = &mut filetype_4;
    let mut bool_2: bool = crate::meta::filetype::FileType::ne(filetype_4_ref_0, filetype_0_ref_0);
    let mut bool_3: bool = std::option::Option::unwrap(option_5);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_272() {
    rusty_monitor::set_test_id(272);
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_1, exec: bool_0};
    let mut filetype_0_ref_0: &meta::filetype::FileType = &mut filetype_0;
    let mut bool_2: bool = false;
    let mut bool_3: bool = true;
    let mut bool_4: bool = false;
    let mut bool_5: bool = false;
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut bool_8: bool = false;
    let mut bool_9: bool = false;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut bool_12: bool = true;
    let mut bool_13: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_13, user_write: bool_12, user_execute: bool_11, group_read: bool_10, group_write: bool_9, group_execute: bool_8, other_read: bool_7, other_write: bool_6, other_execute: bool_5, sticky: bool_4, setgid: bool_3, setuid: bool_2};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut iconoption_0_ref_0: &flags::icons::IconOption = &mut iconoption_0;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut iconoption_1_ref_0: &flags::icons::IconOption = &mut iconoption_1;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_0: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut option_1: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_2: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_3: std::option::Option<bool> = std::option::Option::None;
    let mut bool_14: bool = false;
    let mut option_4: std::option::Option<bool> = std::option::Option::Some(bool_14);
    let mut option_5: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::None;
    let mut option_6: std::option::Option<bool> = std::option::Option::None;
    let mut option_7: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_7, reverse: option_6, dir_grouping: option_5};
    let mut option_8: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_0);
    let mut option_9: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut option_10: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_0);
    let mut option_11: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut option_12: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_0);
    let mut bool_15: bool = true;
    let mut option_13: std::option::Option<bool> = std::option::Option::Some(bool_15);
    let mut option_14: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_15: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut option_16: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut option_17: std::option::Option<bool> = std::option::Option::None;
    let mut option_18: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_19: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_20: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_16: bool = true;
    let mut option_21: std::option::Option<bool> = std::option::Option::Some(bool_16);
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_21, blocks: option_20, color: option_19, date: option_18, dereference: option_17, display: option_16, icons: option_15, ignore_globs: option_14, indicators: option_13, layout: option_12, recursion: option_11, size: option_10, permission: option_9, sorting: option_8, no_symlink: option_4, total_size: option_3, symlink_arrow: option_2, hyperlink: option_1};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut config_2: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut config_3: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_3_ref_0: &crate::config_file::Config = &mut config_3;
    let mut bool_17: bool = true;
    let mut str_0: &str = "1HOSD30y7Nvs";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut result_0: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_0_ref_0);
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_17};
    let mut elem_1: color::Elem = crate::color::Elem::User;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_1_ref_0);
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_0_ref_0);
    let mut filetype_1_ref_0: &meta::filetype::FileType = &mut filetype_1;
    let mut bool_18: bool = crate::meta::filetype::FileType::eq(filetype_1_ref_0, filetype_0_ref_0);
    panic!("From RustyUnit with love");
}
}