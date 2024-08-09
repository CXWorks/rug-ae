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
	use std::clone::Clone;
	use std::cmp::Eq;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1662() {
    rusty_monitor::set_test_id(1662);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_0: usize = 41usize;
    let mut bool_0: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut str_0: &str = "np";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut option_2: std::option::Option<bool> = std::option::Option::None;
    let mut bool_1: bool = true;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_4: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::None;
    let mut bool_2: bool = false;
    let mut option_5: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut option_6: std::option::Option<flags::sorting::SortColumn> = std::option::Option::Some(sortcolumn_1);
    let mut sorting_1: crate::config_file::Sorting = crate::config_file::Sorting {column: option_6, reverse: option_5, dir_grouping: option_4};
    let mut option_7: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_1);
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut option_8: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_1);
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut option_9: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_1);
    let mut option_10: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_11: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut option_12: std::option::Option<bool> = std::option::Option::None;
    let mut option_13: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_14: std::option::Option<std::string::String> = std::option::Option::None;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut str_1: &str = "z1";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut option_15: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut option_16: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_16, theme: option_15, separator: option_14};
    let mut option_17: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut option_18: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut bool_3: bool = false;
    let mut option_19: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut option_20: std::option::Option<std::string::String> = std::option::Option::None;
    let mut str_2: &str = "T";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut filetype_4: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut bool_4: bool = true;
    let mut filetype_5: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_4};
    let mut option_21: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_22: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_23: std::option::Option<bool> = std::option::Option::None;
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_23, blocks: option_22, color: option_21, date: option_20, dereference: option_19, display: option_18, icons: option_17, ignore_globs: option_13, indicators: option_12, layout: option_11, recursion: option_10, size: option_9, permission: option_8, sorting: option_7, no_symlink: option_3, total_size: option_2, symlink_arrow: option_1, hyperlink: option_0};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut str_3: &str = "YNL";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut filetype_6: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut theme_0: icon::Theme = crate::icon::Theme::Fancy;
    let mut theme_0_ref_0: &icon::Theme = &mut theme_0;
    let mut tuple_0: () = crate::icon::Theme::assert_receiver_is_total_eq(theme_0_ref_0);
    let mut elem_0: color::Elem = crate::color::Elem::BlockDevice;
    let mut sortcolumn_2: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2467() {
    rusty_monitor::set_test_id(2467);
    let mut option_0: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut option_1: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut bool_2: bool = false;
    let mut bool_3: bool = true;
    let mut bool_4: bool = false;
    let mut bool_5: bool = true;
    let mut bool_6: bool = false;
    let mut bool_7: bool = false;
    let mut bool_8: bool = true;
    let mut bool_9: bool = false;
    let mut bool_10: bool = false;
    let mut bool_11: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut str_0: &str = "M3vR";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_12: bool = false;
    let mut bool_13: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_13, exec: bool_12};
    let mut str_1: &str = "m59";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut option_2: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_1_ref_0);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::SizeValue;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut bool_14: bool = crate::meta::filetype::FileType::is_dirlike(filetype_1);
    let mut theme_0: icon::Theme = crate::icon::Theme::NoIcon;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut theme_0_ref_0: &icon::Theme = &mut theme_0;
    let mut theme_1: icon::Theme = crate::icon::Theme::clone(theme_0_ref_0);
    let mut theme_1_ref_0: &icon::Theme = &mut theme_1;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut bool_15: bool = crate::meta::permissions::Permissions::is_executable(permissions_0_ref_0);
    let mut dateflag_0_ref_0: &flags::date::DateFlag = &mut dateflag_0;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut sortcolumn_0_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_0;
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_1, theme: option_0};
    panic!("From RustyUnit with love");
}
}