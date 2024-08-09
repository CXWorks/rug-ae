use crate::color::{ColoredString, Colors, Elem};
use crate::flags::HyperlinkOption;
use crate::icon::Icons;
use crate::meta::filetype::FileType;
use crate::print_error;
use crate::url::Url;
use std::cmp::{Ordering, PartialOrd};
use std::ffi::OsStr;
use std::path::{Component, Path, PathBuf};
#[derive(Debug)]
pub enum DisplayOption<'a> {
    FileName,
    Relative { base_path: &'a Path },
    None,
}
#[derive(Clone, Debug, Eq)]
pub struct Name {
    pub name: String,
    path: PathBuf,
    extension: Option<String>,
    file_type: FileType,
}
impl Name {
    pub fn new(path: &Path, file_type: FileType) -> Self {
        let name = match path.file_name() {
            Some(name) => name.to_string_lossy().to_string(),
            None => path.to_string_lossy().to_string(),
        };
        let extension = path.extension().map(|ext| ext.to_string_lossy().to_string());
        Self {
            name,
            path: PathBuf::from(path),
            extension,
            file_type,
        }
    }
    pub fn file_name(&self) -> &str {
        self.path.file_name().and_then(OsStr::to_str).unwrap_or(&self.name)
    }
    fn relative_path<T: AsRef<Path> + Clone>(&self, base_path: T) -> PathBuf {
        let base_path = base_path.as_ref();
        if self.path == base_path {
            return PathBuf::from(AsRef::<Path>::as_ref(&Component::CurDir));
        }
        let shared_components: PathBuf = self
            .path
            .components()
            .zip(base_path.components())
            .take_while(|(target_component, base_component)| {
                target_component == base_component
            })
            .map(|tuple| tuple.0)
            .collect();
        base_path
            .strip_prefix(&shared_components)
            .unwrap()
            .components()
            .map(|_| Component::ParentDir)
            .chain(self.path.strip_prefix(&shared_components).unwrap().components())
            .collect()
    }
    pub fn escape(&self, string: &str) -> String {
        if string.chars().all(|c| c >= 0x20 as char && c != 0x7f as char) {
            string.to_string()
        } else {
            let mut chars = String::new();
            for c in string.chars() {
                if c >= 0x20 as char && c != 0x7f as char {
                    chars.push(c);
                } else {
                    chars += &c.escape_default().collect::<String>();
                }
            }
            chars
        }
    }
    fn hyperlink(&self, name: String, hyperlink: HyperlinkOption) -> String {
        match hyperlink {
            HyperlinkOption::Always => {
                match std::fs::canonicalize(&self.path) {
                    Ok(rp) => {
                        match Url::from_file_path(&rp) {
                            Ok(url) => {
                                format!("\x1B]8;;{}\x1B\x5C{}\x1B]8;;\x1B\x5C", url, name)
                            }
                            Err(_) => {
                                print_error!("{}: unable to form url.", name);
                                name
                            }
                        }
                    }
                    Err(err) => {
                        print_error!("{}: {}.", name, err);
                        name
                    }
                }
            }
            _ => name,
        }
    }
    pub fn render(
        &self,
        colors: &Colors,
        icons: &Icons,
        display_option: &DisplayOption,
        hyperlink: HyperlinkOption,
    ) -> ColoredString {
        let content = match display_option {
            DisplayOption::FileName => {
                format!(
                    "{}{}", icons.get(self), self.hyperlink(self.escape(self
                    .file_name()), hyperlink)
                )
            }
            DisplayOption::Relative { base_path } => {
                format!(
                    "{}{}", icons.get(self), self.hyperlink(self.escape(& self
                    .relative_path(base_path).to_string_lossy()), hyperlink)
                )
            }
            DisplayOption::None => {
                format!(
                    "{}{}", icons.get(self), self.hyperlink(self.escape(& self.path
                    .to_string_lossy()), hyperlink)
                )
            }
        };
        let elem = match self.file_type {
            FileType::CharDevice => Elem::CharDevice,
            FileType::Directory { uid } => Elem::Dir { uid },
            FileType::SymLink { .. } => Elem::SymLink,
            FileType::File { uid, exec } => Elem::File { uid, exec },
            _ => {
                Elem::File {
                    exec: false,
                    uid: false,
                }
            }
        };
        colors.colorize_using_path(content, &self.path, &elem)
    }
    pub fn extension(&self) -> Option<&str> {
        self.extension.as_deref()
    }
    pub fn file_type(&self) -> FileType {
        self.file_type
    }
}
impl Ord for Name {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.to_lowercase().cmp(&other.name.to_lowercase())
    }
}
impl PartialOrd for Name {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.name.to_lowercase().partial_cmp(&other.name.to_lowercase())
    }
}
impl PartialEq for Name {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq_ignore_ascii_case(&other.name.to_lowercase())
    }
}
#[cfg(test)]
mod test {
    use super::DisplayOption;
    use super::Name;
    use crate::color::{self, Colors};
    use crate::flags::HyperlinkOption;
    use crate::icon::{self, Icons};
    use crate::meta::FileType;
    use crate::meta::Meta;
    #[cfg(unix)]
    use crate::meta::Permissions;
    use crate::url::Url;
    use crossterm::style::{Color, Stylize};
    use std::cmp::Ordering;
    use std::fs::{self, File};
    #[cfg(unix)]
    use std::os::unix::fs::symlink;
    use std::path::{Path, PathBuf};
    #[cfg(unix)]
    use std::process::Command;
    use tempfile::tempdir;
    #[test]
    #[cfg(unix)]
    fn test_print_file_name() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let icons = Icons::new(icon::Theme::Fancy, " ".to_string());
        let file_path = tmp_dir.path().join("file.txt");
        File::create(&file_path).expect("failed to create file");
        let meta = file_path.metadata().expect("failed to get metas");
        let colors = Colors::new(color::ThemeOption::NoLscolors);
        let file_type = FileType::new(&meta, None, &Permissions::from(&meta));
        let name = Name::new(&file_path, file_type);
        assert_eq!(
            " file.txt".to_string().with(Color::AnsiValue(184)), name.render(& colors,
            & icons, & DisplayOption::FileName, HyperlinkOption::Never)
        );
    }
    #[test]
    fn test_print_dir_name() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let icons = Icons::new(icon::Theme::Fancy, " ".to_string());
        let dir_path = tmp_dir.path().join("directory");
        fs::create_dir(&dir_path).expect("failed to create the dir");
        let meta = Meta::from_path(&dir_path, false).unwrap();
        let colors = Colors::new(color::ThemeOption::NoLscolors);
        assert_eq!(
            " directory".to_string().with(Color::AnsiValue(33)), meta.name.render(&
            colors, & icons, & DisplayOption::FileName, HyperlinkOption::Never)
        );
    }
    #[test]
    #[cfg(unix)]
    fn test_print_symlink_name_file() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let icons = Icons::new(icon::Theme::Fancy, " ".to_string());
        let file_path = tmp_dir.path().join("file.tmp");
        File::create(&file_path).expect("failed to create file");
        let symlink_path = tmp_dir.path().join("target.tmp");
        symlink(&file_path, &symlink_path).expect("failed to create symlink");
        let meta = symlink_path.symlink_metadata().expect("failed to get metas");
        let target_meta = symlink_path.metadata().ok();
        let colors = Colors::new(color::ThemeOption::NoLscolors);
        let file_type = FileType::new(
            &meta,
            target_meta.as_ref(),
            &Permissions::from(&meta),
        );
        let name = Name::new(&symlink_path, file_type);
        assert_eq!(
            " target.tmp".to_string().with(Color::AnsiValue(44)), name.render(&
            colors, & icons, & DisplayOption::FileName, HyperlinkOption::Never)
        );
    }
    #[test]
    #[cfg(unix)]
    fn test_print_symlink_name_dir() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let icons = Icons::new(icon::Theme::Fancy, " ".to_string());
        let dir_path = tmp_dir.path().join("tmp.d");
        std::fs::create_dir(&dir_path).expect("failed to create dir");
        let symlink_path = tmp_dir.path().join("target.d");
        symlink(&dir_path, &symlink_path).expect("failed to create symlink");
        let meta = symlink_path.symlink_metadata().expect("failed to get metas");
        let target_meta = symlink_path.metadata().ok();
        let colors = Colors::new(color::ThemeOption::NoLscolors);
        let file_type = FileType::new(
            &meta,
            target_meta.as_ref(),
            &Permissions::from(&meta),
        );
        let name = Name::new(&symlink_path, file_type);
        assert_eq!(
            " target.d".to_string().with(Color::AnsiValue(44)), name.render(& colors,
            & icons, & DisplayOption::FileName, HyperlinkOption::Never)
        );
    }
    #[test]
    #[cfg(unix)]
    fn test_print_other_type_name() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let icons = Icons::new(icon::Theme::Fancy, " ".to_string());
        let pipe_path = tmp_dir.path().join("pipe.tmp");
        let success = Command::new("mkfifo")
            .arg(&pipe_path)
            .status()
            .expect("failed to exec mkfifo")
            .success();
        assert_eq!(true, success, "failed to exec mkfifo");
        let meta = pipe_path.metadata().expect("failed to get metas");
        let colors = Colors::new(color::ThemeOption::NoLscolors);
        let file_type = FileType::new(&meta, None, &Permissions::from(&meta));
        let name = Name::new(&pipe_path, file_type);
        assert_eq!(
            " pipe.tmp".to_string().with(Color::AnsiValue(184)), name.render(& colors,
            & icons, & DisplayOption::FileName, HyperlinkOption::Never)
        );
    }
    #[test]
    fn test_print_without_icon_or_color() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let icons = Icons::new(icon::Theme::NoIcon, " ".to_string());
        let file_path = tmp_dir.path().join("file.txt");
        File::create(&file_path).expect("failed to create file");
        let meta = Meta::from_path(&file_path, false).unwrap();
        let colors = Colors::new(color::ThemeOption::NoColor);
        assert_eq!(
            "file.txt", meta.name.render(& colors, & icons, & DisplayOption::FileName,
            HyperlinkOption::Never).to_string().as_str()
        );
    }
    #[test]
    fn test_print_hyperlink() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let icons = Icons::new(icon::Theme::NoIcon, " ".to_string());
        let file_path = tmp_dir.path().join("file.txt");
        File::create(&file_path).expect("failed to create file");
        let meta = Meta::from_path(&file_path, false).unwrap();
        let colors = Colors::new(color::ThemeOption::NoColor);
        let real_path = std::fs::canonicalize(&file_path).expect("canonicalize");
        let expected_url = Url::from_file_path(&real_path).expect("absolute path");
        let expected_text = format!(
            "\x1B]8;;{}\x1B\x5C{}\x1B]8;;\x1B\x5C", expected_url, "file.txt"
        );
        assert_eq!(
            expected_text, meta.name.render(& colors, & icons, & DisplayOption::FileName,
            HyperlinkOption::Always).to_string().as_str()
        );
    }
    #[test]
    fn test_extensions_with_valid_file() {
        let path = Path::new("some-file.txt");
        let name = Name::new(
            &path,
            FileType::File {
                uid: false,
                exec: false,
            },
        );
        assert_eq!(Some("txt"), name.extension());
    }
    #[test]
    fn test_extensions_with_file_without_extension() {
        let path = Path::new(".gitignore");
        let name = Name::new(
            &path,
            FileType::File {
                uid: false,
                exec: false,
            },
        );
        assert_eq!(None, name.extension());
    }
    #[test]
    fn test_order_impl_is_case_insensitive() {
        let path_1 = Path::new("/AAAA");
        let name_1 = Name::new(
            &path_1,
            FileType::File {
                uid: false,
                exec: false,
            },
        );
        let path_2 = Path::new("/aaaa");
        let name_2 = Name::new(
            &path_2,
            FileType::File {
                uid: false,
                exec: false,
            },
        );
        assert_eq!(Ordering::Equal, name_1.cmp(& name_2));
    }
    #[test]
    fn test_partial_order_impl() {
        let path_a = Path::new("/aaaa");
        let name_a = Name::new(
            &path_a,
            FileType::File {
                uid: false,
                exec: false,
            },
        );
        let path_z = Path::new("/zzzz");
        let name_z = Name::new(
            &path_z,
            FileType::File {
                uid: false,
                exec: false,
            },
        );
        assert_eq!(true, name_a < name_z);
    }
    #[test]
    fn test_partial_order_impl_is_case_insensitive() {
        let path_a = Path::new("aaaa");
        let name_a = Name::new(
            &path_a,
            FileType::File {
                uid: false,
                exec: false,
            },
        );
        let path_z = Path::new("ZZZZ");
        let name_z = Name::new(
            &path_z,
            FileType::File {
                uid: false,
                exec: false,
            },
        );
        assert_eq!(true, name_a < name_z);
    }
    #[test]
    fn test_partial_eq_impl() {
        let path_1 = Path::new("aaaa");
        let name_1 = Name::new(
            &path_1,
            FileType::File {
                uid: false,
                exec: false,
            },
        );
        let path_2 = Path::new("aaaa");
        let name_2 = Name::new(
            &path_2,
            FileType::File {
                uid: false,
                exec: false,
            },
        );
        assert_eq!(true, name_1 == name_2);
    }
    #[test]
    fn test_partial_eq_impl_is_case_insensitive() {
        let path_1 = Path::new("AAAA");
        let name_1 = Name::new(
            &path_1,
            FileType::File {
                uid: false,
                exec: false,
            },
        );
        let path_2 = Path::new("aaaa");
        let name_2 = Name::new(
            &path_2,
            FileType::File {
                uid: false,
                exec: false,
            },
        );
        assert_eq!(true, name_1 == name_2);
    }
    #[test]
    fn test_parent_relative_path() {
        let name = Name::new(
            Path::new("/home/parent1/child"),
            FileType::File {
                uid: false,
                exec: false,
            },
        );
        let base_path = Path::new("/home/parent2");
        assert_eq!(PathBuf::from("../parent1/child"), name.relative_path(base_path),)
    }
    #[test]
    fn test_current_relative_path() {
        let name = Name::new(
            Path::new("/home/parent1/child"),
            FileType::File {
                uid: false,
                exec: false,
            },
        );
        let base_path = PathBuf::from("/home/parent1");
        assert_eq!(PathBuf::from("child"), name.relative_path(base_path),)
    }
    #[test]
    fn test_grand_parent_relative_path() {
        let name = Name::new(
            Path::new("/home/grand-parent1/parent1/child"),
            FileType::File {
                uid: false,
                exec: false,
            },
        );
        let base_path = PathBuf::from("/home/grand-parent2/parent1");
        assert_eq!(
            PathBuf::from("../../grand-parent1/parent1/child"), name
            .relative_path(base_path),
        )
    }
    #[test]
    #[cfg(unix)]
    fn test_special_chars_in_filename() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let icons = Icons::new(icon::Theme::Fancy, " ".to_string());
        let file_path = tmp_dir.path().join("file\ttab.txt");
        File::create(&file_path).expect("failed to create file");
        let meta = file_path.metadata().expect("failed to get metas");
        let colors = Colors::new(color::ThemeOption::NoLscolors);
        let file_type = FileType::new(&meta, None, &Permissions::from(&meta));
        let name = Name::new(&file_path, file_type);
        assert_eq!(
            " file\\ttab.txt".to_string().with(Color::AnsiValue(184)), name.render(&
            colors, & icons, & DisplayOption::FileName, HyperlinkOption::Never)
        );
        let file_path = tmp_dir.path().join("file\nnewline.txt");
        File::create(&file_path).expect("failed to create file");
        let meta = file_path.metadata().expect("failed to get metas");
        let colors = Colors::new(color::ThemeOption::NoLscolors);
        let file_type = FileType::new(&meta, None, &Permissions::from(&meta));
        let name = Name::new(&file_path, file_type);
        assert_eq!(
            " file\\nnewline.txt".to_string().with(Color::AnsiValue(184)), name
            .render(& colors, & icons, & DisplayOption::FileName, HyperlinkOption::Never)
        );
    }
}
#[cfg(test)]
mod tests_llm_16_124 {
    use super::*;
    use crate::*;
    use std::cmp::Ordering;
    #[test]
    fn test_cmp() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(&str, &str, bool, bool, &str, &str, bool, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let name1 = Name {
            name: String::from(rug_fuzz_0),
            path: PathBuf::from(rug_fuzz_1),
            extension: None,
            file_type: FileType::File {
                uid: rug_fuzz_2,
                exec: rug_fuzz_3,
            },
        };
        let name2 = Name {
            name: String::from(rug_fuzz_4),
            path: PathBuf::from(rug_fuzz_5),
            extension: None,
            file_type: FileType::File {
                uid: rug_fuzz_6,
                exec: rug_fuzz_7,
            },
        };
        let result = name1.cmp(&name2);
        debug_assert_eq!(result, Ordering::Less);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_125 {
    use super::*;
    use crate::*;
    use std::path::Path;
    use crate::meta::filetype::FileType;
    use crate::meta::filetype::FileType::*;
    #[test]
    fn test_eq() {
        let _rug_st_tests_llm_16_125_rrrruuuugggg_test_eq = 0;
        let rug_fuzz_0 = "test.txt";
        let rug_fuzz_1 = "path/to/test.txt";
        let rug_fuzz_2 = "txt";
        let rug_fuzz_3 = false;
        let rug_fuzz_4 = false;
        let rug_fuzz_5 = "TEST.txt";
        let rug_fuzz_6 = "path/to/TEST.txt";
        let rug_fuzz_7 = "txt";
        let rug_fuzz_8 = false;
        let rug_fuzz_9 = false;
        let rug_fuzz_10 = "test.txt";
        let rug_fuzz_11 = "path/to/other.txt";
        let rug_fuzz_12 = "txt";
        let rug_fuzz_13 = false;
        let rug_fuzz_14 = false;
        let rug_fuzz_15 = "test.txt";
        let rug_fuzz_16 = "path/to/test.txt";
        let rug_fuzz_17 = "pdf";
        let rug_fuzz_18 = false;
        let rug_fuzz_19 = false;
        let rug_fuzz_20 = "test.txt";
        let rug_fuzz_21 = "path/to/test.txt";
        let rug_fuzz_22 = "txt";
        let rug_fuzz_23 = true;
        let rug_fuzz_24 = false;
        let rug_fuzz_25 = "test.txt";
        let rug_fuzz_26 = "path/to/test.txt";
        let rug_fuzz_27 = "txt";
        let rug_fuzz_28 = false;
        let rug_fuzz_29 = true;
        let name1 = Name {
            name: String::from(rug_fuzz_0),
            path: Path::new(rug_fuzz_1).to_path_buf(),
            extension: Some(String::from(rug_fuzz_2)),
            file_type: File {
                uid: rug_fuzz_3,
                exec: rug_fuzz_4,
            },
        };
        let name2 = Name {
            name: String::from(rug_fuzz_5),
            path: Path::new(rug_fuzz_6).to_path_buf(),
            extension: Some(String::from(rug_fuzz_7)),
            file_type: File {
                uid: rug_fuzz_8,
                exec: rug_fuzz_9,
            },
        };
        let name3 = Name {
            name: String::from(rug_fuzz_10),
            path: Path::new(rug_fuzz_11).to_path_buf(),
            extension: Some(String::from(rug_fuzz_12)),
            file_type: File {
                uid: rug_fuzz_13,
                exec: rug_fuzz_14,
            },
        };
        let name4 = Name {
            name: String::from(rug_fuzz_15),
            path: Path::new(rug_fuzz_16).to_path_buf(),
            extension: Some(String::from(rug_fuzz_17)),
            file_type: File {
                uid: rug_fuzz_18,
                exec: rug_fuzz_19,
            },
        };
        let name5 = Name {
            name: String::from(rug_fuzz_20),
            path: Path::new(rug_fuzz_21).to_path_buf(),
            extension: Some(String::from(rug_fuzz_22)),
            file_type: File {
                uid: rug_fuzz_23,
                exec: rug_fuzz_24,
            },
        };
        let name6 = Name {
            name: String::from(rug_fuzz_25),
            path: Path::new(rug_fuzz_26).to_path_buf(),
            extension: Some(String::from(rug_fuzz_27)),
            file_type: File {
                uid: rug_fuzz_28,
                exec: rug_fuzz_29,
            },
        };
        debug_assert_eq!(name1.eq(& name2), true);
        debug_assert_eq!(name1.eq(& name3), false);
        debug_assert_eq!(name1.eq(& name4), false);
        debug_assert_eq!(name1.eq(& name5), true);
        debug_assert_eq!(name1.eq(& name6), true);
        let _rug_ed_tests_llm_16_125_rrrruuuugggg_test_eq = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_126 {
    use super::*;
    use crate::*;
    #[test]
    fn test_partial_cmp() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(&str, &str, bool, bool, &str, &str, bool, bool, &str, &str, bool, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let name1 = Name {
            name: rug_fuzz_0.to_string(),
            path: PathBuf::from(rug_fuzz_1),
            extension: None,
            file_type: FileType::File {
                uid: rug_fuzz_2,
                exec: rug_fuzz_3,
            },
        };
        let name2 = Name {
            name: rug_fuzz_4.to_string(),
            path: PathBuf::from(rug_fuzz_5),
            extension: None,
            file_type: FileType::File {
                uid: rug_fuzz_6,
                exec: rug_fuzz_7,
            },
        };
        let name3 = Name {
            name: rug_fuzz_8.to_string(),
            path: PathBuf::from(rug_fuzz_9),
            extension: None,
            file_type: FileType::File {
                uid: rug_fuzz_10,
                exec: rug_fuzz_11,
            },
        };
        let result1 = name1.partial_cmp(&name2);
        let result2 = name1.partial_cmp(&name3);
        debug_assert_eq!(result1, Some(Ordering::Less));
        debug_assert_eq!(result2, Some(Ordering::Equal));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_256 {
    use super::*;
    use crate::*;
    #[test]
    fn test_escape_all_printable_chars() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(&str, bool, bool, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let name = Name {
            name: String::from(rug_fuzz_0),
            path: PathBuf::new(),
            extension: None,
            file_type: FileType::File {
                exec: rug_fuzz_1,
                uid: rug_fuzz_2,
            },
        };
        let actual = name.escape(rug_fuzz_3);
        let expected = rug_fuzz_4.to_string();
        debug_assert_eq!(actual, expected);
             }
});    }
    #[test]
    fn test_escape_non_printable_chars() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(&str, bool, bool, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let name = Name {
            name: String::from(rug_fuzz_0),
            path: PathBuf::new(),
            extension: None,
            file_type: FileType::File {
                exec: rug_fuzz_1,
                uid: rug_fuzz_2,
            },
        };
        let actual = name.escape(rug_fuzz_3);
        let expected = rug_fuzz_4.to_string();
        debug_assert_eq!(actual, expected);
             }
});    }
    #[test]
    fn test_escape_mixed_chars() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(&str, bool, bool, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let name = Name {
            name: String::from(rug_fuzz_0),
            path: PathBuf::new(),
            extension: None,
            file_type: FileType::File {
                exec: rug_fuzz_1,
                uid: rug_fuzz_2,
            },
        };
        let actual = name.escape(rug_fuzz_3);
        let expected = rug_fuzz_4.to_string();
        debug_assert_eq!(actual, expected);
             }
});    }
    #[test]
    fn test_escape_non_ascii_chars() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(&str, bool, bool, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let name = Name {
            name: String::from(rug_fuzz_0),
            path: PathBuf::new(),
            extension: None,
            file_type: FileType::File {
                exec: rug_fuzz_1,
                uid: rug_fuzz_2,
            },
        };
        let actual = name.escape(rug_fuzz_3);
        let expected = rug_fuzz_4.to_string();
        debug_assert_eq!(actual, expected);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_257 {
    use super::*;
    use crate::*;
    #[test]
    fn test_extension() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(&str, &str, &str, bool, bool, &str, &str, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let name = Name {
            name: String::from(rug_fuzz_0),
            path: PathBuf::from(rug_fuzz_1),
            extension: Some(String::from(rug_fuzz_2)),
            file_type: FileType::File {
                uid: rug_fuzz_3,
                exec: rug_fuzz_4,
            },
        };
        debug_assert_eq!(name.extension(), Some("txt"));
        let name = Name {
            name: String::from(rug_fuzz_5),
            path: PathBuf::from(rug_fuzz_6),
            extension: None,
            file_type: FileType::Directory {
                uid: rug_fuzz_7,
            },
        };
        debug_assert_eq!(name.extension(), None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_258 {
    use super::*;
    use crate::*;
    #[test]
    fn test_file_name() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(&str, &str, &str, bool, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let name = Name {
            name: String::from(rug_fuzz_0),
            path: PathBuf::from(rug_fuzz_1),
            extension: Some(String::from(rug_fuzz_2)),
            file_type: FileType::File {
                uid: rug_fuzz_3,
                exec: rug_fuzz_4,
            },
        };
        debug_assert_eq!(name.file_name(), "example.txt");
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_261 {
    use super::*;
    use crate::*;
    use crate::flags::hyperlink::HyperlinkOption;
    #[test]
    fn test_hyperlink() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, &str, &str, bool, bool, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let name = Name {
            name: rug_fuzz_0.to_string(),
            path: PathBuf::from(rug_fuzz_1),
            extension: Some(rug_fuzz_2.to_string()),
            file_type: FileType::File {
                uid: rug_fuzz_3,
                exec: rug_fuzz_4,
            },
        };
        let result = name.hyperlink(rug_fuzz_5.to_string(), HyperlinkOption::Always);
        debug_assert_eq!(
            result, "\x1B]8;;file:///path/to/test.txt\x1B\x5Ctest.txt\x1B]8;;\x1B\x5C"
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_262 {
    use super::*;
    use crate::*;
    use std::path::Path;
    #[test]
    fn test_new() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, bool, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let path = Path::new(rug_fuzz_0);
        let file_type = FileType::File {
            uid: rug_fuzz_1,
            exec: rug_fuzz_2,
        };
        let name = Name::new(&path, file_type);
        debug_assert_eq!(name.name, "test-file");
        debug_assert_eq!(name.path, PathBuf::from("test-file"));
        debug_assert_eq!(name.extension, None);
        debug_assert_eq!(name.file_type, FileType::File { uid : false, exec : false });
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_263 {
    use super::*;
    use crate::*;
    use std::path::Path;
    #[test]
    fn test_relative_path() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(&str, &str, &str, bool, bool, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let name = Name {
            name: rug_fuzz_0.to_string(),
            path: PathBuf::from(rug_fuzz_1),
            extension: Some(rug_fuzz_2.to_string()),
            file_type: FileType::File {
                uid: rug_fuzz_3,
                exec: rug_fuzz_4,
            },
        };
        let base_path = rug_fuzz_5;
        let result = name.relative_path(base_path);
        let expected = PathBuf::from(rug_fuzz_6);
        debug_assert_eq!(result, expected);
             }
});    }
}
#[cfg(test)]
mod tests_rug_101 {
    use super::*;
    use std::path::Path;
    use crate::meta::Name;
    use crate::meta::FileType;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, bool, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let path = Path::new(rug_fuzz_0);
        let file_type = FileType::File {
            uid: rug_fuzz_1,
            exec: rug_fuzz_2,
        };
        let p0 = Name::new(path, file_type);
        p0.file_type();
             }
});    }
}
