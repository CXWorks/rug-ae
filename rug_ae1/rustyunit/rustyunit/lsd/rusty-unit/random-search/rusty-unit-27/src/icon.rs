use crate::meta::{FileType, Name};
use std::collections::HashMap;

pub struct Icons {
    display_icons: bool,
    icons_by_name: HashMap<&'static str, &'static str>,
    icons_by_extension: HashMap<&'static str, &'static str>,
    default_folder_icon: &'static str,
    default_file_icon: &'static str,
    icon_separator: String,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Theme {
    NoIcon,
    Fancy,
    Unicode,
}

// In order to add a new icon, write the unicode value like "\ue5fb" then
// run the command below in vim:
//
// s#\\u[0-9a-f]*#\=eval('"'.submatch(0).'"')#
impl Icons {
    pub fn new(theme: Theme, icon_separator: String) -> Self {
        let display_icons = theme == Theme::Fancy || theme == Theme::Unicode;
        let (icons_by_name, icons_by_extension, default_file_icon, default_folder_icon) =
            if theme == Theme::Fancy {
                (
                    Self::get_default_icons_by_name(),
                    Self::get_default_icons_by_extension(),
                    "\u{f016}", // 
                    "\u{f115}", // 
                )
            } else {
                (
                    HashMap::new(),
                    HashMap::new(),
                    "\u{1f5cb}", // 🗋
                    "\u{1f5c1}", // 🗁
                )
            };

        Self {
            display_icons,
            icons_by_name,
            icons_by_extension,
            default_file_icon,
            default_folder_icon,
            icon_separator,
        }
    }

    pub fn get(&self, name: &Name) -> String {
        if !self.display_icons {
            return String::new();
        }

        // Check file types
        let file_type: FileType = name.file_type();

        let icon = if let FileType::Directory { .. } = file_type {
            self.default_folder_icon
        } else if let FileType::SymLink { is_dir: true } = file_type {
            "\u{f482}" // ""
        } else if let FileType::SymLink { is_dir: false } = file_type {
            "\u{f481}" // ""
        } else if let FileType::Socket = file_type {
            "\u{f6a7}" // ""
        } else if let FileType::Pipe = file_type {
            "\u{f731}" // ""
        } else if let FileType::CharDevice = file_type {
            "\u{e601}" // ""
        } else if let FileType::BlockDevice = file_type {
            "\u{fc29}" // "ﰩ"
        } else if let FileType::Special = file_type {
            "\u{f2dc}" // ""
        } else if let Some(icon) = self
            .icons_by_name
            .get(name.file_name().to_lowercase().as_str())
        {
            // Use the known names.
            icon
        } else if let Some(icon) = name.extension().and_then(|extension| {
            self.icons_by_extension
                .get(extension.to_lowercase().as_str())
        }) {
            // Use the known extensions.
            icon
        } else {
            // Use the default icons.
            self.default_file_icon
        };

        format!("{}{}", icon, self.icon_separator)
    }

    fn get_default_icons_by_name() -> HashMap<&'static str, &'static str> {
        let mut m = HashMap::new();

        // Note: filenames must be lower-case

        m.insert(".trash", "\u{f1f8}"); // ""
        m.insert(".atom", "\u{e764}"); // ""
        m.insert(".bashprofile", "\u{e615}"); // ""
        m.insert(".bashrc", "\u{f489}"); // ""
        m.insert(".clang-format", "\u{e615}"); // ""
        m.insert(".git", "\u{f1d3}"); // ""
        m.insert(".gitattributes", "\u{f1d3}"); // ""
        m.insert(".gitconfig", "\u{f1d3}"); // ""
        m.insert(".github", "\u{f408}"); // ""
        m.insert(".gitignore", "\u{f1d3}"); // ""
        m.insert(".gitmodules", "\u{f1d3}"); // ""
        m.insert(".rvm", "\u{e21e}"); // ""
        m.insert(".vimrc", "\u{e62b}"); // ""
        m.insert(".vscode", "\u{e70c}"); // ""
        m.insert(".zshrc", "\u{f489}"); // ""
        m.insert("bin", "\u{e5fc}"); // ""
        m.insert("config", "\u{e5fc}"); // ""
        m.insert("docker-compose.yml", "\u{f308}"); // ""
        m.insert("dockerfile", "\u{f308}"); // ""
        m.insert("ds_store", "\u{f179}"); // ""
        m.insert("gitignore_global", "\u{f1d3}"); // ""
        m.insert("gradle", "\u{e70e}"); // ""
        m.insert("gruntfile.coffee", "\u{e611}"); // ""
        m.insert("gruntfile.js", "\u{e611}"); // ""
        m.insert("gruntfile.ls", "\u{e611}"); // ""
        m.insert("gulpfile.coffee", "\u{e610}"); // ""
        m.insert("gulpfile.js", "\u{e610}"); // ""
        m.insert("gulpfile.ls", "\u{e610}"); // ""
        m.insert("hidden", "\u{f023}"); // ""
        m.insert("include", "\u{e5fc}"); // ""
        m.insert("lib", "\u{f121}"); // ""
        m.insert("localized", "\u{f179}"); // ""
        m.insert("node_modules", "\u{e718}"); // ""
        m.insert("npmignore", "\u{e71e}"); // ""
        m.insert("rubydoc", "\u{e73b}"); // ""

        m
    }

    fn get_default_icons_by_extension() -> HashMap<&'static str, &'static str> {
        let mut m = HashMap::new();

        // Note: extensions must be lower-case

        m.insert("7z", "\u{f410}"); // ""
        m.insert("ai", "\u{e7b4}"); // ""
        m.insert("apk", "\u{e70e}"); // ""
        m.insert("avi", "\u{f03d}"); // ""
        m.insert("avro", "\u{e60b}"); // ""
        m.insert("awk", "\u{f489}"); // ""
        m.insert("bash", "\u{f489}"); // ""
        m.insert("bash_history", "\u{f489}"); // ""
        m.insert("bash_profile", "\u{f489}"); // ""
        m.insert("bashrc", "\u{f489}"); // ""
        m.insert("bat", "\u{f17a}"); // ""
        m.insert("bio", "\u{f910}"); // "蘿"
        m.insert("bmp", "\u{f1c5}"); // ""
        m.insert("bz2", "\u{f410}"); // ""
        m.insert("c", "\u{e61e}"); // ""
        m.insert("c++", "\u{e61d}"); // ""
        m.insert("cc", "\u{e61d}"); // ""
        m.insert("cfg", "\u{e615}"); // ""
        m.insert("clj", "\u{e768}"); // ""
        m.insert("cljs", "\u{e76a}"); // ""
        m.insert("cls", "\u{e600}"); // ""
        m.insert("coffee", "\u{f0f4}"); // ""
        m.insert("conf", "\u{e615}"); // ""
        m.insert("cp", "\u{e61d}"); // ""
        m.insert("cpp", "\u{e61d}"); // ""
        m.insert("cs", "\u{f81a}"); // ""
        m.insert("cshtml", "\u{f1fa}"); // ""
        m.insert("csproj", "\u{f81a}"); // ""
        m.insert("csx", "\u{f81a}"); // ""
        m.insert("csh", "\u{f489}"); // ""
        m.insert("css", "\u{e749}"); // ""
        m.insert("csv", "\u{f1c3}"); // ""
        m.insert("cxx", "\u{e61d}"); // ""
        m.insert("d", "\u{e7af}"); // ""
        m.insert("dart", "\u{e798}"); // ""
        m.insert("db", "\u{f1c0}"); // ""
        m.insert("diff", "\u{f440}"); // ""
        m.insert("doc", "\u{f1c2}"); // ""
        m.insert("dockerfile", "\u{f308}"); // ""
        m.insert("docx", "\u{f1c2}"); // ""
        m.insert("ds_store", "\u{f179}"); // ""
        m.insert("dump", "\u{f1c0}"); // ""
        m.insert("ebook", "\u{e28b}"); // ""
        m.insert("editorconfig", "\u{e615}"); // ""
        m.insert("ejs", "\u{e618}"); // ""
        m.insert("elm", "\u{e62c}"); // ""
        m.insert("env", "\u{f462}"); // ""
        m.insert("eot", "\u{f031}"); // ""
        m.insert("epub", "\u{e28a}"); // ""
        m.insert("erb", "\u{e73b}"); // ""
        m.insert("erl", "\u{e7b1}"); // ""
        m.insert("exe", "\u{f17a}"); // ""
        m.insert("ex", "\u{e62d}"); // ""
        m.insert("exs", "\u{e62d}"); // ""
        m.insert("fish", "\u{f489}"); // ""
        m.insert("flac", "\u{f001}"); // ""
        m.insert("flv", "\u{f03d}"); // ""
        m.insert("font", "\u{f031}"); // ""
        m.insert("fpl", "\u{f910}"); // "蘿"
        m.insert("fs", "\u{e7a7}"); // ""
        m.insert("fsx", "\u{e7a7}"); // ""
        m.insert("fsi", "\u{e7a7}"); // ""
        m.insert("gdoc", "\u{f1c2}"); // ""
        m.insert("gemfile", "\u{e21e}"); // ""
        m.insert("gemspec", "\u{e21e}"); // ""
        m.insert("gform", "\u{f298}"); // ""
        m.insert("gif", "\u{f1c5}"); // ""
        m.insert("git", "\u{f1d3}"); // ""
        m.insert("go", "\u{e626}"); // ""
        m.insert("gradle", "\u{e70e}"); // ""
        m.insert("gsheet", "\u{f1c3}"); // ""
        m.insert("gslides", "\u{f1c4}"); // ""
        m.insert("guardfile", "\u{e21e}"); // ""
        m.insert("gz", "\u{f410}"); // ""
        m.insert("h", "\u{f0fd}"); // ""
        m.insert("hbs", "\u{e60f}"); // ""
        m.insert("heic", "\u{f1c5}"); // ""
        m.insert("heif", "\u{f1c5}"); // ""
        m.insert("heix", "\u{f1c5}"); // ""
        m.insert("hpp", "\u{f0fd}"); // ""
        m.insert("hs", "\u{e777}"); // ""
        m.insert("htm", "\u{f13b}"); // ""
        m.insert("html", "\u{f13b}"); // ""
        m.insert("hxx", "\u{f0fd}"); // ""
        m.insert("ico", "\u{f1c5}"); // ""
        m.insert("image", "\u{f1c5}"); // ""
        m.insert("iml", "\u{e7b5}"); // ""
        m.insert("ini", "\u{e615}"); // ""
        m.insert("ipynb", "\u{e606}"); // ""
        m.insert("jar", "\u{e204}"); // ""
        m.insert("java", "\u{e204}"); // ""
        m.insert("jpeg", "\u{f1c5}"); // ""
        m.insert("jpg", "\u{f1c5}"); // ""
        m.insert("js", "\u{e74e}"); // ""
        m.insert("json", "\u{e60b}"); // ""
        m.insert("jsx", "\u{e7ba}"); // ""
        m.insert("jl", "\u{e624}"); // ""
        m.insert("ksh", "\u{f489}"); // ""
        m.insert("less", "\u{e758}"); // ""
        m.insert("lhs", "\u{e777}"); // ""
        m.insert("license", "\u{f48a}"); // ""
        m.insert("localized", "\u{f179}"); // ""
        m.insert("lock", "\u{f023}"); // ""
        m.insert("log", "\u{f18d}"); // ""
        m.insert("lua", "\u{e620}"); // ""
        m.insert("lz", "\u{f410}"); // ""
        m.insert("m3u", "\u{f910}"); // "蘿"
        m.insert("m3u8", "\u{f910}"); // "蘿"
        m.insert("m4a", "\u{f001}"); // ""
        m.insert("magnet", "\u{f076}"); // ""
        m.insert("markdown", "\u{f48a}"); // ""
        m.insert("md", "\u{f48a}"); // ""
        m.insert("mjs", "\u{e74e}"); // ""
        m.insert("mkd", "\u{f48a}"); // ""
        m.insert("mkv", "\u{f03d}"); // ""
        m.insert("mobi", "\u{e28b}"); // ""
        m.insert("mov", "\u{f03d}"); // ""
        m.insert("mp3", "\u{f001}"); // ""
        m.insert("mp4", "\u{f03d}"); // ""
        m.insert("mustache", "\u{e60f}"); // ""
        m.insert("nix", "\u{f313}"); // ""
        m.insert("npmignore", "\u{e71e}"); // ""
        m.insert("opus", "\u{f001}"); // ""
        m.insert("ogg", "\u{f001}"); // ""
        m.insert("ogv", "\u{f03d}"); // ""
        m.insert("otf", "\u{f031}"); // ""
        m.insert("pdf", "\u{f1c1}"); // ""
        m.insert("pem", "\u{f805}"); // ""
        m.insert("php", "\u{e73d}"); // ""
        m.insert("pl", "\u{e769}"); // ""
        m.insert("pls", "\u{f910}"); // "蘿"
        m.insert("pm", "\u{e769}"); // ""
        m.insert("png", "\u{f1c5}"); // ""
        m.insert("ppt", "\u{f1c4}"); // ""
        m.insert("pptx", "\u{f1c4}"); // ""
        m.insert("procfile", "\u{e21e}"); // ""
        m.insert("properties", "\u{e60b}"); // ""
        m.insert("ps1", "\u{f489}"); // ""
        m.insert("psd", "\u{e7b8}"); // ""
        m.insert("pxm", "\u{f1c5}"); // ""
        m.insert("py", "\u{e606}"); // ""
        m.insert("pyc", "\u{e606}"); // ""
        m.insert("r", "\u{f25d}"); // ""
        m.insert("rakefile", "\u{e21e}"); // ""
        m.insert("rar", "\u{f410}"); // ""
        m.insert("razor", "\u{f1fa}"); // ""
        m.insert("rb", "\u{e21e}"); // ""
        m.insert("rdata", "\u{f25d}"); // ""
        m.insert("rdb", "\u{e76d}"); // ""
        m.insert("rdoc", "\u{f48a}"); // ""
        m.insert("rds", "\u{f25d}"); // ""
        m.insert("readme", "\u{f48a}"); // ""
        m.insert("rlib", "\u{e7a8}"); // ""
        m.insert("rmd", "\u{f48a}"); // ""
        m.insert("rs", "\u{e7a8}"); // ""
        m.insert("rspec", "\u{e21e}"); // ""
        m.insert("rspec_parallel", "\u{e21e}"); // ""
        m.insert("rspec_status", "\u{e21e}"); // ""
        m.insert("rss", "\u{f09e}"); // ""
        m.insert("ru", "\u{e21e}"); // ""
        m.insert("rubydoc", "\u{e73b}"); // ""
        m.insert("sass", "\u{e603}"); // ""
        m.insert("scala", "\u{e737}"); // ""
        m.insert("scpt", "\u{f302}"); // ""
        m.insert("scss", "\u{e749}"); // ""
        m.insert("sh", "\u{f489}"); // ""
        m.insert("shell", "\u{f489}"); // ""
        m.insert("slim", "\u{e73b}"); // ""
        m.insert("sln", "\u{e70c}"); // ""
        m.insert("sql", "\u{f1c0}"); // ""
        m.insert("sqlite3", "\u{e7c4}"); // ""
        m.insert("styl", "\u{e600}"); // ""
        m.insert("stylus", "\u{e600}"); // ""
        m.insert("svg", "\u{f1c5}"); // ""
        m.insert("swift", "\u{e755}"); // ""
        m.insert("t", "\u{e769}"); // ""
        m.insert("tar", "\u{f410}"); // ""
        m.insert("tex", "\u{e600}"); // ""
        m.insert("tiff", "\u{f1c5}"); // ""
        m.insert("toml", "\u{e60b}"); // ""
        m.insert("torrent", "\u{f98c}"); // "歷"
        m.insert("ts", "\u{e628}"); // ""
        m.insert("tsx", "\u{e7ba}"); // ""
        m.insert("ttc", "\u{f031}"); // ""
        m.insert("ttf", "\u{f031}"); // ""
        m.insert("twig", "\u{e61c}"); // ""
        m.insert("txt", "\u{f15c}"); // ""
        m.insert("video", "\u{f03d}"); // ""
        m.insert("vim", "\u{e62b}"); // ""
        m.insert("vlc", "\u{f910}"); // "蘿"
        m.insert("vue", "\u{fd42}"); // "﵂"
        m.insert("wav", "\u{f001}"); // ""
        m.insert("webm", "\u{f03d}"); // ""
        m.insert("webp", "\u{f1c5}"); // ""
        m.insert("windows", "\u{f17a}"); // ""
        m.insert("wma", "\u{f001}"); // ""
        m.insert("wmv", "\u{f03d}"); // ""
        m.insert("wpl", "\u{f910}"); // "蘿"
        m.insert("woff", "\u{f031}"); // ""
        m.insert("woff2", "\u{f031}"); // ""
        m.insert("xls", "\u{f1c3}"); // ""
        m.insert("xlsx", "\u{f1c3}"); // ""
        m.insert("xml", "\u{e619}"); // ""
        m.insert("xul", "\u{e619}"); // ""
        m.insert("xz", "\u{f410}"); // ""
        m.insert("yaml", "\u{e60b}"); // ""
        m.insert("yml", "\u{e60b}"); // ""
        m.insert("zip", "\u{f410}"); // ""
        m.insert("zsh", "\u{f489}"); // ""
        m.insert("zsh-theme", "\u{f489}"); // ""
        m.insert("zshrc", "\u{f489}"); // ""

        m
    }
}

#[cfg(test)]
mod test {
    use super::{Icons, Theme};
    use crate::meta::Meta;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn get_no_icon() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let file_path = tmp_dir.path().join("file.txt");
        File::create(&file_path).expect("failed to create file");
        let meta = Meta::from_path(&file_path, false).unwrap();

        let icon = Icons::new(Theme::NoIcon, " ".to_string());
        let icon = icon.get(&meta.name);

        assert_eq!(icon, "");
    }

    #[test]
    fn get_default_file_icon() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let file_path = tmp_dir.path().join("file");
        File::create(&file_path).expect("failed to create file");
        let meta = Meta::from_path(&file_path, false).unwrap();

        let icon = Icons::new(Theme::Fancy, " ".to_string());
        let icon_str = icon.get(&meta.name);

        assert_eq!(icon_str, format!("{}{}", "\u{f016}", icon.icon_separator)); // 
    }

    #[test]
    fn get_default_file_icon_unicode() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let file_path = tmp_dir.path().join("file");
        File::create(&file_path).expect("failed to create file");
        let meta = Meta::from_path(&file_path, false).unwrap();

        let icon = Icons::new(Theme::Unicode, " ".to_string());
        let icon_str = icon.get(&meta.name);

        assert_eq!(icon_str, format!("{}{}", "\u{1f5cb}", icon.icon_separator));
    }

    #[test]
    fn get_directory_icon() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let file_path = tmp_dir.path();
        let meta = Meta::from_path(&file_path.to_path_buf(), false).unwrap();

        let icon = Icons::new(Theme::Fancy, " ".to_string());
        let icon_str = icon.get(&meta.name);

        assert_eq!(icon_str, format!("{}{}", "\u{f115}", icon.icon_separator)); // 
    }

    #[test]
    fn get_directory_icon_unicode() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let file_path = tmp_dir.path();
        let meta = Meta::from_path(&file_path.to_path_buf(), false).unwrap();

        let icon = Icons::new(Theme::Unicode, " ".to_string());
        let icon_str = icon.get(&meta.name);

        assert_eq!(icon_str, format!("{}{}", "\u{1f5c1}", icon.icon_separator));
    }

    #[test]
    fn get_directory_icon_with_ext() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let file_path = tmp_dir.path();
        let meta = Meta::from_path(&file_path.to_path_buf(), false).unwrap();

        let icon = Icons::new(Theme::Fancy, " ".to_string());
        let icon_str = icon.get(&meta.name);

        assert_eq!(icon_str, format!("{}{}", "\u{f115}", icon.icon_separator)); // 
    }

    #[test]
    fn get_icon_by_name() {
        let tmp_dir = tempdir().expect("failed to create temp dir");

        for (file_name, file_icon) in &Icons::get_default_icons_by_name() {
            let file_path = tmp_dir.path().join(file_name);
            File::create(&file_path).expect("failed to create file");
            let meta = Meta::from_path(&file_path, false).unwrap();

            let icon = Icons::new(Theme::Fancy, " ".to_string());
            let icon_str = icon.get(&meta.name);

            assert_eq!(icon_str, format!("{}{}", file_icon, icon.icon_separator));
        }
    }

    #[test]
    fn get_icon_by_extension() {
        let tmp_dir = tempdir().expect("failed to create temp dir");

        for (ext, file_icon) in &Icons::get_default_icons_by_extension() {
            let file_path = tmp_dir.path().join(format!("file.{}", ext));
            File::create(&file_path).expect("failed to create file");
            let meta = Meta::from_path(&file_path, false).unwrap();

            let icon = Icons::new(Theme::Fancy, " ".to_string());
            let icon_str = icon.get(&meta.name);

            assert_eq!(icon_str, format!("{}{}", file_icon, icon.icon_separator));
        }
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::cmp::PartialEq;
	use std::clone::Clone;
	use std::cmp::Eq;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2987() {
    rusty_monitor::set_test_id(2987);
    let mut bool_0: bool = false;
    let mut usize_0: usize = 26usize;
    let mut bool_1: bool = true;
    let mut theme_0: icon::Theme = crate::icon::Theme::Unicode;
    let mut theme_0_ref_0: &icon::Theme = &mut theme_0;
    let mut option_0: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::None;
    let mut option_1: std::option::Option<bool> = std::option::Option::None;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut option_2: std::option::Option<flags::sorting::SortColumn> = std::option::Option::Some(sortcolumn_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_0: color::Elem = crate::color::Elem::HourOld;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_1: color::Elem = crate::color::Elem::Special;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_2: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_3_ref_0);
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Mega;
    let mut symlink_0: crate::color::theme::Symlink = crate::color::theme::Symlink {default: color_2, broken: color_1, missing_target: color_0};
    let mut theme_4: icon::Theme = crate::icon::Theme::Unicode;
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_2, reverse: option_1, dir_grouping: option_0};
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut theme_4_ref_0: &icon::Theme = &mut theme_4;
    let mut bool_2: bool = crate::icon::Theme::eq(theme_4_ref_0, theme_0_ref_0);
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_1, depth: usize_0};
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortorder_0_ref_0: &flags::sorting::SortOrder = &mut sortorder_0;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1839() {
    rusty_monitor::set_test_id(1839);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut elem_0: color::Elem = crate::color::Elem::File {exec: bool_1, uid: bool_0};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut bool_2: bool = true;
    let mut bool_3: bool = false;
    let mut elem_1: color::Elem = crate::color::Elem::File {exec: bool_3, uid: bool_2};
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut bool_4: bool = false;
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::Socket;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut bool_5: bool = false;
    let mut elem_4: color::Elem = crate::color::Elem::Links {valid: bool_5};
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut elem_5: color::Elem = crate::color::Elem::User;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_5_ref_0);
    let mut theme_6: icon::Theme = crate::icon::Theme::Unicode;
    let mut theme_6_ref_0: &icon::Theme = &mut theme_6;
    let mut tuple_0: () = crate::icon::Theme::assert_receiver_is_total_eq(theme_6_ref_0);
    let mut size_0: crate::color::theme::Size = crate::color::theme::Size {none: color_5, small: color_4, medium: color_3, large: color_2};
    let mut elem_6: color::Elem = crate::color::Elem::Dir {uid: bool_4};
    let mut inode_0: crate::color::theme::INode = crate::color::theme::INode {valid: color_1, invalid: color_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3200() {
    rusty_monitor::set_test_id(3200);
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut bool_2: bool = true;
    let mut elem_0: color::Elem = crate::color::Elem::Links {valid: bool_2};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_0: usize = 94usize;
    let mut bool_3: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_3, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut bool_4: bool = false;
    let mut bool_5: bool = true;
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = true;
    let mut bool_10: bool = true;
    let mut bool_11: bool = true;
    let mut bool_12: bool = false;
    let mut bool_13: bool = false;
    let mut bool_14: bool = true;
    let mut bool_15: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_15, user_write: bool_14, user_execute: bool_13, group_read: bool_12, group_write: bool_11, group_execute: bool_10, other_read: bool_9, other_write: bool_8, other_execute: bool_7, sticky: bool_6, setgid: bool_5, setuid: bool_4};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut u64_0: u64 = 43u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut str_0: &str = "wfJg5qpqk";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_16: bool = false;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_16};
    let mut u64_1: u64 = 52u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_2};
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_3);
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut bool_17: bool = false;
    let mut themeoption_4: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_3: crate::color::Colors = crate::color::Colors::new(themeoption_4);
    let mut colors_3_ref_0: &crate::color::Colors = &mut colors_3;
    let mut themeoption_5: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_4: crate::color::Colors = crate::color::Colors::new(themeoption_5);
    let mut colors_4_ref_0: &crate::color::Colors = &mut colors_4;
    let mut theme_1: icon::Theme = crate::icon::Theme::NoIcon;
    let mut theme_1_ref_0: &icon::Theme = &mut theme_1;
    let mut tuple_0: () = crate::icon::Theme::assert_receiver_is_total_eq(theme_1_ref_0);
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    crate::meta::filetype::FileType::render(filetype_2, colors_3_ref_0);
    let mut display_1: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut elem_1: color::Elem = crate::color::Elem::Links {valid: bool_17};
    let mut display_2: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut option_1: std::option::Option<std::path::PathBuf> = crate::config_file::Config::config_file_path();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2431() {
    rusty_monitor::set_test_id(2431);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Size;
    let mut block_0_ref_0: &flags::blocks::Block = &mut block_0;
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "YXVCJiRHO";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Acl;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Special;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::Write;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::Pipe;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_4: color::Elem = crate::color::Elem::Older;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut elem_5: color::Elem = crate::color::Elem::Older;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_5_ref_0);
    let mut theme_6: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_6_ref_0: &crate::color::theme::Theme = &mut theme_6;
    let mut elem_6: color::Elem = crate::color::Elem::HourOld;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut color_6: crossterm::style::Color = crate::color::Elem::get_color(elem_6_ref_0, theme_6_ref_0);
    let mut theme_7: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_7_ref_0: &crate::color::theme::Theme = &mut theme_7;
    let mut elem_7: color::Elem = crate::color::Elem::DayOld;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut color_7: crossterm::style::Color = crate::color::Elem::get_color(elem_7_ref_0, theme_7_ref_0);
    let mut theme_8: icon::Theme = crate::icon::Theme::Unicode;
    let mut theme_8_ref_0: &icon::Theme = &mut theme_8;
    let mut theme_9: icon::Theme = crate::icon::Theme::clone(theme_8_ref_0);
    let mut block_1: flags::blocks::Block = crate::flags::blocks::Block::INode;
    let mut permission_0: crate::color::theme::Permission = crate::color::theme::Permission {read: color_7, write: color_6, exec: color_5, exec_sticky: color_4, no_access: color_3, octal: color_2, acl: color_1, context: color_0};
    let mut option_0: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_1_ref_0);
    let mut result_0: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_0_ref_0);
    let mut block_1_ref_0: &flags::blocks::Block = &mut block_1;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2863() {
    rusty_monitor::set_test_id(2863);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::Read;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut elem_4: color::Elem = crate::color::Elem::ExecSticky;
    let mut hashmap_0: std::collections::HashMap<&str, &str> = crate::icon::Icons::get_default_icons_by_extension();
    let mut bool_0: bool = crate::meta::filetype::FileType::is_dirlike(filetype_0);
    let mut file_0: crate::color::theme::File = crate::color::theme::File {exec_uid: color_3, uid_no_exec: color_2, exec_no_uid: color_1, no_exec_no_uid: color_0};
    let mut elem_5: color::Elem = crate::color::Elem::FileSmall;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    panic!("From RustyUnit with love");
}
}