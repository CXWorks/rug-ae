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

        let extension = path
            .extension()
            .map(|ext| ext.to_string_lossy().to_string());

        Self {
            name,
            path: PathBuf::from(path),
            extension,
            file_type,
        }
    }

    pub fn file_name(&self) -> &str {
        self.path
            .file_name()
            .and_then(OsStr::to_str)
            .unwrap_or(&self.name)
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
            .take_while(|(target_component, base_component)| target_component == base_component)
            .map(|tuple| tuple.0)
            .collect();

        base_path
            .strip_prefix(&shared_components)
            .unwrap()
            .components()
            .map(|_| Component::ParentDir)
            .chain(
                self.path
                    .strip_prefix(&shared_components)
                    .unwrap()
                    .components(),
            )
            .collect()
    }

    pub fn escape(&self, string: &str) -> String {
        if string
            .chars()
            .all(|c| c >= 0x20 as char && c != 0x7f as char)
        {
            string.to_string()
        } else {
            let mut chars = String::new();
            for c in string.chars() {
                // The `escape_default` method on `char` is *almost* what we want here, but
                // it still escapes non-ASCII UTF-8 characters, which are still printable.
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
                // HyperlinkOption::Auto gets converted to None or Always in core.rs based on tty_available
                match std::fs::canonicalize(&self.path) {
                    Ok(rp) => {
                        match Url::from_file_path(&rp) {
                            Ok(url) => {
                                // Crossterm does not support hyperlinks as of now
                                // https://gist.github.com/egmontkob/eb114294efbcd5adb1944c9f3cb5feda
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
                    "{}{}",
                    icons.get(self),
                    self.hyperlink(self.escape(self.file_name()), hyperlink)
                )
            }
            DisplayOption::Relative { base_path } => format!(
                "{}{}",
                icons.get(self),
                self.hyperlink(
                    self.escape(&self.relative_path(base_path).to_string_lossy()),
                    hyperlink
                )
            ),
            DisplayOption::None => format!(
                "{}{}",
                icons.get(self),
                self.hyperlink(self.escape(&self.path.to_string_lossy()), hyperlink)
            ),
        };

        let elem = match self.file_type {
            FileType::CharDevice => Elem::CharDevice,
            FileType::Directory { uid } => Elem::Dir { uid },
            FileType::SymLink { .. } => Elem::SymLink,
            FileType::File { uid, exec } => Elem::File { uid, exec },
            _ => Elem::File {
                exec: false,
                uid: false,
            },
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
        self.name
            .to_lowercase()
            .partial_cmp(&other.name.to_lowercase())
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
    #[cfg(unix)] // Windows uses different default permissions
    fn test_print_file_name() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let icons = Icons::new(icon::Theme::Fancy, " ".to_string());

        // Create the file;
        let file_path = tmp_dir.path().join("file.txt");
        File::create(&file_path).expect("failed to create file");
        let meta = file_path.metadata().expect("failed to get metas");

        let colors = Colors::new(color::ThemeOption::NoLscolors);
        let file_type = FileType::new(&meta, None, &Permissions::from(&meta));
        let name = Name::new(&file_path, file_type);

        assert_eq!(
            " file.txt".to_string().with(Color::AnsiValue(184)),
            name.render(
                &colors,
                &icons,
                &DisplayOption::FileName,
                HyperlinkOption::Never
            )
        );
    }

    #[test]
    fn test_print_dir_name() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let icons = Icons::new(icon::Theme::Fancy, " ".to_string());

        // Create the directory
        let dir_path = tmp_dir.path().join("directory");
        fs::create_dir(&dir_path).expect("failed to create the dir");
        let meta = Meta::from_path(&dir_path, false).unwrap();

        let colors = Colors::new(color::ThemeOption::NoLscolors);

        assert_eq!(
            " directory".to_string().with(Color::AnsiValue(33)),
            meta.name.render(
                &colors,
                &icons,
                &DisplayOption::FileName,
                HyperlinkOption::Never
            )
        );
    }

    #[test]
    #[cfg(unix)] // Symlinks are hard on Windows
    fn test_print_symlink_name_file() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let icons = Icons::new(icon::Theme::Fancy, " ".to_string());

        // Create the file;
        let file_path = tmp_dir.path().join("file.tmp");
        File::create(&file_path).expect("failed to create file");

        // Create the symlink
        let symlink_path = tmp_dir.path().join("target.tmp");
        symlink(&file_path, &symlink_path).expect("failed to create symlink");
        let meta = symlink_path
            .symlink_metadata()
            .expect("failed to get metas");
        let target_meta = symlink_path.metadata().ok();

        let colors = Colors::new(color::ThemeOption::NoLscolors);
        let file_type = FileType::new(&meta, target_meta.as_ref(), &Permissions::from(&meta));
        let name = Name::new(&symlink_path, file_type);

        assert_eq!(
            " target.tmp".to_string().with(Color::AnsiValue(44)),
            name.render(
                &colors,
                &icons,
                &DisplayOption::FileName,
                HyperlinkOption::Never
            )
        );
    }

    #[test]
    #[cfg(unix)] // Symlinks are hard on Windows
    fn test_print_symlink_name_dir() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let icons = Icons::new(icon::Theme::Fancy, " ".to_string());

        // Create the directory;
        let dir_path = tmp_dir.path().join("tmp.d");
        std::fs::create_dir(&dir_path).expect("failed to create dir");

        // Create the symlink
        let symlink_path = tmp_dir.path().join("target.d");
        symlink(&dir_path, &symlink_path).expect("failed to create symlink");
        let meta = symlink_path
            .symlink_metadata()
            .expect("failed to get metas");
        let target_meta = symlink_path.metadata().ok();

        let colors = Colors::new(color::ThemeOption::NoLscolors);
        let file_type = FileType::new(&meta, target_meta.as_ref(), &Permissions::from(&meta));
        let name = Name::new(&symlink_path, file_type);

        assert_eq!(
            " target.d".to_string().with(Color::AnsiValue(44)),
            name.render(
                &colors,
                &icons,
                &DisplayOption::FileName,
                HyperlinkOption::Never
            )
        );
    }

    #[test]
    #[cfg(unix)]
    fn test_print_other_type_name() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let icons = Icons::new(icon::Theme::Fancy, " ".to_string());

        // Create the pipe;
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
            " pipe.tmp".to_string().with(Color::AnsiValue(184)),
            name.render(
                &colors,
                &icons,
                &DisplayOption::FileName,
                HyperlinkOption::Never
            )
        );
    }

    #[test]
    fn test_print_without_icon_or_color() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let icons = Icons::new(icon::Theme::NoIcon, " ".to_string());

        // Create the file;
        let file_path = tmp_dir.path().join("file.txt");
        File::create(&file_path).expect("failed to create file");
        let meta = Meta::from_path(&file_path, false).unwrap();

        let colors = Colors::new(color::ThemeOption::NoColor);

        assert_eq!(
            "file.txt",
            meta.name
                .render(
                    &colors,
                    &icons,
                    &DisplayOption::FileName,
                    HyperlinkOption::Never
                )
                .to_string()
                .as_str()
        );
    }

    #[test]
    fn test_print_hyperlink() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let icons = Icons::new(icon::Theme::NoIcon, " ".to_string());

        // Create the file;
        let file_path = tmp_dir.path().join("file.txt");
        File::create(&file_path).expect("failed to create file");
        let meta = Meta::from_path(&file_path, false).unwrap();

        let colors = Colors::new(color::ThemeOption::NoColor);

        let real_path = std::fs::canonicalize(&file_path).expect("canonicalize");
        let expected_url = Url::from_file_path(&real_path).expect("absolute path");
        let expected_text = format!(
            "\x1B]8;;{}\x1B\x5C{}\x1B]8;;\x1B\x5C",
            expected_url, "file.txt"
        );

        assert_eq!(
            expected_text,
            meta.name
                .render(
                    &colors,
                    &icons,
                    &DisplayOption::FileName,
                    HyperlinkOption::Always
                )
                .to_string()
                .as_str()
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

        assert_eq!(Ordering::Equal, name_1.cmp(&name_2));
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

        assert_eq!(
            PathBuf::from("../parent1/child"),
            name.relative_path(base_path),
        )
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
            PathBuf::from("../../grand-parent1/parent1/child"),
            name.relative_path(base_path),
        )
    }

    #[test]
    #[cfg(unix)]
    fn test_special_chars_in_filename() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let icons = Icons::new(icon::Theme::Fancy, " ".to_string());

        // Create the file;
        let file_path = tmp_dir.path().join("file\ttab.txt");
        File::create(&file_path).expect("failed to create file");
        let meta = file_path.metadata().expect("failed to get metas");

        let colors = Colors::new(color::ThemeOption::NoLscolors);
        let file_type = FileType::new(&meta, None, &Permissions::from(&meta));
        let name = Name::new(&file_path, file_type);

        assert_eq!(
            " file\\ttab.txt".to_string().with(Color::AnsiValue(184)),
            name.render(
                &colors,
                &icons,
                &DisplayOption::FileName,
                HyperlinkOption::Never
            )
        );

        let file_path = tmp_dir.path().join("file\nnewline.txt");
        File::create(&file_path).expect("failed to create file");
        let meta = file_path.metadata().expect("failed to get metas");

        let colors = Colors::new(color::ThemeOption::NoLscolors);
        let file_type = FileType::new(&meta, None, &Permissions::from(&meta));
        let name = Name::new(&file_path, file_type);

        assert_eq!(
            " file\\nnewline.txt"
                .to_string()
                .with(Color::AnsiValue(184)),
            name.render(
                &colors,
                &icons,
                &DisplayOption::FileName,
                HyperlinkOption::Never
            )
        );
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5086() {
    rusty_monitor::set_test_id(5086);
    let mut usize_0: usize = 71usize;
    let mut bool_0: bool = true;
    let mut usize_1: usize = 75usize;
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut bool_3: bool = false;
    let mut usize_2: usize = 88usize;
    let mut bool_4: bool = true;
    let mut u64_0: u64 = 33u64;
    let mut bool_5: bool = false;
    let mut usize_3: usize = 65usize;
    let mut bool_6: bool = true;
    let mut bool_7: bool = false;
    let mut usize_4: usize = 14usize;
    let mut bool_8: bool = false;
    let mut str_0: &str = "67fqq7QuSLXJ";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_9: bool = true;
    let mut usize_5: usize = 72usize;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut bool_12: bool = false;
    let mut bool_13: bool = false;
    let mut bool_14: bool = true;
    let mut bool_15: bool = true;
    let mut bool_16: bool = false;
    let mut bool_17: bool = false;
    let mut bool_18: bool = true;
    let mut bool_19: bool = true;
    let mut bool_20: bool = true;
    let mut bool_21: bool = true;
    let mut usize_6: usize = 43usize;
    let mut bool_22: bool = false;
    let mut bool_23: bool = true;
    let mut bool_24: bool = false;
    let mut u64_1: u64 = 35u64;
    let mut usize_7: usize = 93usize;
    let mut bool_25: bool = true;
    let mut bool_26: bool = true;
    let mut bool_27: bool = false;
    let mut bool_28: bool = true;
    let mut bool_29: bool = false;
    let mut bool_30: bool = false;
    let mut bool_31: bool = false;
    let mut bool_32: bool = false;
    let mut bool_33: bool = true;
    let mut bool_34: bool = false;
    let mut bool_35: bool = true;
    let mut bool_36: bool = true;
    let mut bool_37: bool = false;
    let mut u64_2: u64 = 47u64;
    let mut usize_8: usize = 20usize;
    let mut bool_38: bool = false;
    let mut usize_9: usize = 64usize;
    let mut bool_39: bool = false;
    let mut bool_40: bool = false;
    let mut bool_41: bool = false;
    let mut option_0: std::option::Option<bool> = std::option::Option::Some(bool_41);
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut option_1: std::option::Option<flags::sorting::SortColumn> = std::option::Option::Some(sortcolumn_0);
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut option_2: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_0);
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut option_3: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_0);
    let mut option_4: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut option_5: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_0);
    let mut option_6: std::option::Option<bool> = std::option::Option::None;
    let mut option_7: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_8: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut option_9: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut option_10: std::option::Option<bool> = std::option::Option::None;
    let mut option_11: std::option::Option<std::string::String> = std::option::Option::None;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut option_12: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_0);
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut option_13: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_13, theme: option_12};
    let mut option_14: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_15: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_16: std::option::Option<bool> = std::option::Option::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_1: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_1};
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut option_17: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_0);
    let mut option_18: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_42: bool = false;
    let mut option_19: std::option::Option<bool> = std::option::Option::Some(bool_42);
    let mut bool_43: bool = false;
    let mut option_20: std::option::Option<bool> = std::option::Option::Some(bool_43);
    let mut option_21: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut option_22: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut option_23: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_1);
    let mut usize_10: usize = 83usize;
    let mut option_24: std::option::Option<usize> = std::option::Option::Some(usize_10);
    let mut bool_44: bool = true;
    let mut option_25: std::option::Option<bool> = std::option::Option::Some(bool_44);
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_25, depth: option_24};
    let mut option_26: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_0);
    let mut option_27: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut option_28: std::option::Option<bool> = std::option::Option::None;
    let mut option_29: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_30: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut option_31: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_1);
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut option_32: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_1);
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_32, theme: option_31, separator: option_30};
    let mut option_33: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut display_2: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut option_34: std::option::Option<flags::display::Display> = std::option::Option::Some(display_2);
    let mut bool_45: bool = false;
    let mut option_35: std::option::Option<bool> = std::option::Option::Some(bool_45);
    let mut option_36: std::option::Option<std::string::String> = std::option::Option::None;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut option_37: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_2);
    let mut coloroption_2: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut option_38: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_2);
    let mut color_2: crate::config_file::Color = crate::config_file::Color {when: option_38, theme: option_37};
    let mut option_39: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_2);
    let mut option_40: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_41: std::option::Option<bool> = std::option::Option::None;
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_41, blocks: option_40, color: option_39, date: option_36, dereference: option_35, display: option_34, icons: option_33, ignore_globs: option_29, indicators: option_28, layout: option_27, recursion: option_26, size: option_23, permission: option_22, sorting: option_21, no_symlink: option_20, total_size: option_19, symlink_arrow: option_18, hyperlink: option_17};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut u64_3: u64 = 88u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_3);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut str_1: &str = "nJp9Vp";
    let mut str_1_ref_0: &str = &mut str_1;
    panic!("From RustyUnit with love");
}
}