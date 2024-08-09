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
                is_dir: symlink_meta.map(|m| m.is_dir()).unwrap_or_default(),
            }
        } else {
            FileType::Special
        }
    }
    pub fn is_dirlike(self) -> bool {
        matches!(self, FileType::Directory { .. } | FileType::SymLink { is_dir : true })
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
            FileType::SymLink { .. } => {
                colors.colorize(String::from("l"), &Elem::SymLink)
            }
            FileType::BlockDevice => {
                colors.colorize(String::from("b"), &Elem::BlockDevice)
            }
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
    #[cfg(unix)]
    fn test_file_type() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let file_path = tmp_dir.path().join("file.txt");
        File::create(&file_path).expect("failed to create file");
        let meta = file_path.metadata().expect("failed to get metas");
        let colors = Colors::new(ThemeOption::NoLscolors);
        let file_type = FileType::new(&meta, None, &Permissions::from(&meta));
        assert_eq!(
            ".".to_string().with(Color::AnsiValue(184)), file_type.render(& colors)
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
            "d".to_string().with(Color::AnsiValue(33)), file_type.render(& colors)
        );
    }
    #[test]
    #[cfg(unix)]
    fn test_symlink_type_file() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let file_path = tmp_dir.path().join("file.tmp");
        File::create(&file_path).expect("failed to create file");
        let symlink_path = tmp_dir.path().join("target.tmp");
        symlink(&file_path, &symlink_path).expect("failed to create symlink");
        let meta = symlink_path.symlink_metadata().expect("failed to get metas");
        let colors = Colors::new(ThemeOption::NoLscolors);
        let file_type = FileType::new(&meta, Some(&meta), &Permissions::from(&meta));
        assert_eq!(
            "l".to_string().with(Color::AnsiValue(44)), file_type.render(& colors)
        );
    }
    #[test]
    #[cfg(unix)]
    fn test_symlink_type_dir() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let dir_path = tmp_dir.path().join("dir.d");
        std::fs::create_dir(&dir_path).expect("failed to create dir");
        let symlink_path = tmp_dir.path().join("target.d");
        symlink(&dir_path, &symlink_path).expect("failed to create symlink");
        let meta = symlink_path.symlink_metadata().expect("failed to get metas");
        let colors = Colors::new(ThemeOption::NoLscolors);
        let file_type = FileType::new(&meta, Some(&meta), &Permissions::from(&meta));
        assert_eq!(
            "l".to_string().with(Color::AnsiValue(44)), file_type.render(& colors)
        );
    }
    #[test]
    #[cfg(unix)]
    fn test_pipe_type() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
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
            "|".to_string().with(Color::AnsiValue(44)), file_type.render(& colors)
        );
    }
    #[test]
    #[cfg(feature = "sudo")]
    fn test_char_device_type() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
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
            "c".to_string().with(Color::AnsiValue(44)), file_type.render(& colors)
        );
    }
    #[test]
    #[cfg(unix)]
    fn test_socket_type() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let socket_path = tmp_dir.path().join("socket.tmp");
        UnixListener::bind(&socket_path).expect("failed to create the socket");
        let meta = socket_path.metadata().expect("failed to get metas");
        let colors = Colors::new(ThemeOption::NoLscolors);
        let file_type = FileType::new(&meta, None, &Permissions::from(&meta));
        assert_eq!(
            "s".to_string().with(Color::AnsiValue(44)), file_type.render(& colors)
        );
    }
}
#[cfg(test)]
mod tests_rug_93 {
    use super::*;
    use std::fs::Metadata;
    use std::option::Option;
    use crate::meta::Permissions;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_93_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = "path/to/file";
        let rug_fuzz_1 = true;
        let rug_fuzz_2 = true;
        let rug_fuzz_3 = false;
        let rug_fuzz_4 = true;
        let rug_fuzz_5 = false;
        let rug_fuzz_6 = true;
        let rug_fuzz_7 = false;
        let rug_fuzz_8 = true;
        let rug_fuzz_9 = true;
        let rug_fuzz_10 = false;
        let rug_fuzz_11 = false;
        let rug_fuzz_12 = true;
        let mut p0: Metadata = Metadata::from(std::fs::metadata(rug_fuzz_0).unwrap());
        let mut p1: Option<&Metadata> = None;
        let mut p2 = Permissions {
            user_read: rug_fuzz_1,
            user_write: rug_fuzz_2,
            user_execute: rug_fuzz_3,
            group_read: rug_fuzz_4,
            group_write: rug_fuzz_5,
            group_execute: rug_fuzz_6,
            other_read: rug_fuzz_7,
            other_write: rug_fuzz_8,
            other_execute: rug_fuzz_9,
            sticky: rug_fuzz_10,
            setgid: rug_fuzz_11,
            setuid: rug_fuzz_12,
        };
        crate::meta::filetype::FileType::new(&p0, p1, &p2);
        let _rug_ed_tests_rug_93_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_94 {
    use super::*;
    use crate::meta::FileType;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_94_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = false;
        let rug_fuzz_1 = true;
        let mut p0 = FileType::File {
            uid: rug_fuzz_0,
            exec: rug_fuzz_1,
        };
        FileType::is_dirlike(p0);
        let _rug_ed_tests_rug_94_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_95 {
    use super::*;
    use crate::color::Colors;
    use crate::color::ThemeOption;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_95_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = false;
        let rug_fuzz_1 = true;
        let p0 = FileType::File {
            uid: rug_fuzz_0,
            exec: rug_fuzz_1,
        };
        let p1 = Colors::new(ThemeOption::Default);
        FileType::render(p0, &p1);
        let _rug_ed_tests_rug_95_rrrruuuugggg_test_rug = 0;
    }
}
