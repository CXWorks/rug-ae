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
impl Icons {
    pub fn new(theme: Theme, icon_separator: String) -> Self {
        let display_icons = theme == Theme::Fancy || theme == Theme::Unicode;
        let (
            icons_by_name,
            icons_by_extension,
            default_file_icon,
            default_folder_icon,
        ) = if theme == Theme::Fancy {
            (
                Self::get_default_icons_by_name(),
                Self::get_default_icons_by_extension(),
                "\u{f016}",
                "\u{f115}",
            )
        } else {
            (HashMap::new(), HashMap::new(), "\u{1f5cb}", "\u{1f5c1}")
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
        let file_type: FileType = name.file_type();
        let icon = if let FileType::Directory { .. } = file_type {
            self.default_folder_icon
        } else if let FileType::SymLink { is_dir: true } = file_type {
            "\u{f482}"
        } else if let FileType::SymLink { is_dir: false } = file_type {
            "\u{f481}"
        } else if let FileType::Socket = file_type {
            "\u{f6a7}"
        } else if let FileType::Pipe = file_type {
            "\u{f731}"
        } else if let FileType::CharDevice = file_type {
            "\u{e601}"
        } else if let FileType::BlockDevice = file_type {
            "\u{fc29}"
        } else if let FileType::Special = file_type {
            "\u{f2dc}"
        } else if let Some(icon)
            = self.icons_by_name.get(name.file_name().to_lowercase().as_str())
        {
            icon
        } else if let Some(icon)
            = name
                .extension()
                .and_then(|extension| {
                    self.icons_by_extension.get(extension.to_lowercase().as_str())
                })
        {
            icon
        } else {
            self.default_file_icon
        };
        format!("{}{}", icon, self.icon_separator)
    }
    fn get_default_icons_by_name() -> HashMap<&'static str, &'static str> {
        let mut m = HashMap::new();
        m.insert(".trash", "\u{f1f8}");
        m.insert(".atom", "\u{e764}");
        m.insert(".bashprofile", "\u{e615}");
        m.insert(".bashrc", "\u{f489}");
        m.insert(".clang-format", "\u{e615}");
        m.insert(".git", "\u{f1d3}");
        m.insert(".gitattributes", "\u{f1d3}");
        m.insert(".gitconfig", "\u{f1d3}");
        m.insert(".github", "\u{f408}");
        m.insert(".gitignore", "\u{f1d3}");
        m.insert(".gitmodules", "\u{f1d3}");
        m.insert(".rvm", "\u{e21e}");
        m.insert(".vimrc", "\u{e62b}");
        m.insert(".vscode", "\u{e70c}");
        m.insert(".zshrc", "\u{f489}");
        m.insert("bin", "\u{e5fc}");
        m.insert("config", "\u{e5fc}");
        m.insert("docker-compose.yml", "\u{f308}");
        m.insert("dockerfile", "\u{f308}");
        m.insert("ds_store", "\u{f179}");
        m.insert("gitignore_global", "\u{f1d3}");
        m.insert("gradle", "\u{e70e}");
        m.insert("gruntfile.coffee", "\u{e611}");
        m.insert("gruntfile.js", "\u{e611}");
        m.insert("gruntfile.ls", "\u{e611}");
        m.insert("gulpfile.coffee", "\u{e610}");
        m.insert("gulpfile.js", "\u{e610}");
        m.insert("gulpfile.ls", "\u{e610}");
        m.insert("hidden", "\u{f023}");
        m.insert("include", "\u{e5fc}");
        m.insert("lib", "\u{f121}");
        m.insert("localized", "\u{f179}");
        m.insert("node_modules", "\u{e718}");
        m.insert("npmignore", "\u{e71e}");
        m.insert("rubydoc", "\u{e73b}");
        m
    }
    fn get_default_icons_by_extension() -> HashMap<&'static str, &'static str> {
        let mut m = HashMap::new();
        m.insert("7z", "\u{f410}");
        m.insert("ai", "\u{e7b4}");
        m.insert("apk", "\u{e70e}");
        m.insert("avi", "\u{f03d}");
        m.insert("avro", "\u{e60b}");
        m.insert("awk", "\u{f489}");
        m.insert("bash", "\u{f489}");
        m.insert("bash_history", "\u{f489}");
        m.insert("bash_profile", "\u{f489}");
        m.insert("bashrc", "\u{f489}");
        m.insert("bat", "\u{f17a}");
        m.insert("bio", "\u{f910}");
        m.insert("bmp", "\u{f1c5}");
        m.insert("bz2", "\u{f410}");
        m.insert("c", "\u{e61e}");
        m.insert("c++", "\u{e61d}");
        m.insert("cc", "\u{e61d}");
        m.insert("cfg", "\u{e615}");
        m.insert("clj", "\u{e768}");
        m.insert("cljs", "\u{e76a}");
        m.insert("cls", "\u{e600}");
        m.insert("coffee", "\u{f0f4}");
        m.insert("conf", "\u{e615}");
        m.insert("cp", "\u{e61d}");
        m.insert("cpp", "\u{e61d}");
        m.insert("cs", "\u{f81a}");
        m.insert("cshtml", "\u{f1fa}");
        m.insert("csproj", "\u{f81a}");
        m.insert("csx", "\u{f81a}");
        m.insert("csh", "\u{f489}");
        m.insert("css", "\u{e749}");
        m.insert("csv", "\u{f1c3}");
        m.insert("cxx", "\u{e61d}");
        m.insert("d", "\u{e7af}");
        m.insert("dart", "\u{e798}");
        m.insert("db", "\u{f1c0}");
        m.insert("diff", "\u{f440}");
        m.insert("doc", "\u{f1c2}");
        m.insert("dockerfile", "\u{f308}");
        m.insert("docx", "\u{f1c2}");
        m.insert("ds_store", "\u{f179}");
        m.insert("dump", "\u{f1c0}");
        m.insert("ebook", "\u{e28b}");
        m.insert("editorconfig", "\u{e615}");
        m.insert("ejs", "\u{e618}");
        m.insert("elm", "\u{e62c}");
        m.insert("env", "\u{f462}");
        m.insert("eot", "\u{f031}");
        m.insert("epub", "\u{e28a}");
        m.insert("erb", "\u{e73b}");
        m.insert("erl", "\u{e7b1}");
        m.insert("exe", "\u{f17a}");
        m.insert("ex", "\u{e62d}");
        m.insert("exs", "\u{e62d}");
        m.insert("fish", "\u{f489}");
        m.insert("flac", "\u{f001}");
        m.insert("flv", "\u{f03d}");
        m.insert("font", "\u{f031}");
        m.insert("fpl", "\u{f910}");
        m.insert("fs", "\u{e7a7}");
        m.insert("fsx", "\u{e7a7}");
        m.insert("fsi", "\u{e7a7}");
        m.insert("gdoc", "\u{f1c2}");
        m.insert("gemfile", "\u{e21e}");
        m.insert("gemspec", "\u{e21e}");
        m.insert("gform", "\u{f298}");
        m.insert("gif", "\u{f1c5}");
        m.insert("git", "\u{f1d3}");
        m.insert("go", "\u{e626}");
        m.insert("gradle", "\u{e70e}");
        m.insert("gsheet", "\u{f1c3}");
        m.insert("gslides", "\u{f1c4}");
        m.insert("guardfile", "\u{e21e}");
        m.insert("gz", "\u{f410}");
        m.insert("h", "\u{f0fd}");
        m.insert("hbs", "\u{e60f}");
        m.insert("heic", "\u{f1c5}");
        m.insert("heif", "\u{f1c5}");
        m.insert("heix", "\u{f1c5}");
        m.insert("hpp", "\u{f0fd}");
        m.insert("hs", "\u{e777}");
        m.insert("htm", "\u{f13b}");
        m.insert("html", "\u{f13b}");
        m.insert("hxx", "\u{f0fd}");
        m.insert("ico", "\u{f1c5}");
        m.insert("image", "\u{f1c5}");
        m.insert("iml", "\u{e7b5}");
        m.insert("ini", "\u{e615}");
        m.insert("ipynb", "\u{e606}");
        m.insert("jar", "\u{e204}");
        m.insert("java", "\u{e204}");
        m.insert("jpeg", "\u{f1c5}");
        m.insert("jpg", "\u{f1c5}");
        m.insert("js", "\u{e74e}");
        m.insert("json", "\u{e60b}");
        m.insert("jsx", "\u{e7ba}");
        m.insert("jl", "\u{e624}");
        m.insert("ksh", "\u{f489}");
        m.insert("less", "\u{e758}");
        m.insert("lhs", "\u{e777}");
        m.insert("license", "\u{f48a}");
        m.insert("localized", "\u{f179}");
        m.insert("lock", "\u{f023}");
        m.insert("log", "\u{f18d}");
        m.insert("lua", "\u{e620}");
        m.insert("lz", "\u{f410}");
        m.insert("m3u", "\u{f910}");
        m.insert("m3u8", "\u{f910}");
        m.insert("m4a", "\u{f001}");
        m.insert("magnet", "\u{f076}");
        m.insert("markdown", "\u{f48a}");
        m.insert("md", "\u{f48a}");
        m.insert("mjs", "\u{e74e}");
        m.insert("mkd", "\u{f48a}");
        m.insert("mkv", "\u{f03d}");
        m.insert("mobi", "\u{e28b}");
        m.insert("mov", "\u{f03d}");
        m.insert("mp3", "\u{f001}");
        m.insert("mp4", "\u{f03d}");
        m.insert("mustache", "\u{e60f}");
        m.insert("nix", "\u{f313}");
        m.insert("npmignore", "\u{e71e}");
        m.insert("opus", "\u{f001}");
        m.insert("ogg", "\u{f001}");
        m.insert("ogv", "\u{f03d}");
        m.insert("otf", "\u{f031}");
        m.insert("pdf", "\u{f1c1}");
        m.insert("pem", "\u{f805}");
        m.insert("php", "\u{e73d}");
        m.insert("pl", "\u{e769}");
        m.insert("pls", "\u{f910}");
        m.insert("pm", "\u{e769}");
        m.insert("png", "\u{f1c5}");
        m.insert("ppt", "\u{f1c4}");
        m.insert("pptx", "\u{f1c4}");
        m.insert("procfile", "\u{e21e}");
        m.insert("properties", "\u{e60b}");
        m.insert("ps1", "\u{f489}");
        m.insert("psd", "\u{e7b8}");
        m.insert("pxm", "\u{f1c5}");
        m.insert("py", "\u{e606}");
        m.insert("pyc", "\u{e606}");
        m.insert("r", "\u{f25d}");
        m.insert("rakefile", "\u{e21e}");
        m.insert("rar", "\u{f410}");
        m.insert("razor", "\u{f1fa}");
        m.insert("rb", "\u{e21e}");
        m.insert("rdata", "\u{f25d}");
        m.insert("rdb", "\u{e76d}");
        m.insert("rdoc", "\u{f48a}");
        m.insert("rds", "\u{f25d}");
        m.insert("readme", "\u{f48a}");
        m.insert("rlib", "\u{e7a8}");
        m.insert("rmd", "\u{f48a}");
        m.insert("rs", "\u{e7a8}");
        m.insert("rspec", "\u{e21e}");
        m.insert("rspec_parallel", "\u{e21e}");
        m.insert("rspec_status", "\u{e21e}");
        m.insert("rss", "\u{f09e}");
        m.insert("ru", "\u{e21e}");
        m.insert("rubydoc", "\u{e73b}");
        m.insert("sass", "\u{e603}");
        m.insert("scala", "\u{e737}");
        m.insert("scpt", "\u{f302}");
        m.insert("scss", "\u{e749}");
        m.insert("sh", "\u{f489}");
        m.insert("shell", "\u{f489}");
        m.insert("slim", "\u{e73b}");
        m.insert("sln", "\u{e70c}");
        m.insert("sql", "\u{f1c0}");
        m.insert("sqlite3", "\u{e7c4}");
        m.insert("styl", "\u{e600}");
        m.insert("stylus", "\u{e600}");
        m.insert("svg", "\u{f1c5}");
        m.insert("swift", "\u{e755}");
        m.insert("t", "\u{e769}");
        m.insert("tar", "\u{f410}");
        m.insert("tex", "\u{e600}");
        m.insert("tiff", "\u{f1c5}");
        m.insert("toml", "\u{e60b}");
        m.insert("torrent", "\u{f98c}");
        m.insert("ts", "\u{e628}");
        m.insert("tsx", "\u{e7ba}");
        m.insert("ttc", "\u{f031}");
        m.insert("ttf", "\u{f031}");
        m.insert("twig", "\u{e61c}");
        m.insert("txt", "\u{f15c}");
        m.insert("video", "\u{f03d}");
        m.insert("vim", "\u{e62b}");
        m.insert("vlc", "\u{f910}");
        m.insert("vue", "\u{fd42}");
        m.insert("wav", "\u{f001}");
        m.insert("webm", "\u{f03d}");
        m.insert("webp", "\u{f1c5}");
        m.insert("windows", "\u{f17a}");
        m.insert("wma", "\u{f001}");
        m.insert("wmv", "\u{f03d}");
        m.insert("wpl", "\u{f910}");
        m.insert("woff", "\u{f031}");
        m.insert("woff2", "\u{f031}");
        m.insert("xls", "\u{f1c3}");
        m.insert("xlsx", "\u{f1c3}");
        m.insert("xml", "\u{e619}");
        m.insert("xul", "\u{e619}");
        m.insert("xz", "\u{f410}");
        m.insert("yaml", "\u{e60b}");
        m.insert("yml", "\u{e60b}");
        m.insert("zip", "\u{f410}");
        m.insert("zsh", "\u{f489}");
        m.insert("zsh-theme", "\u{f489}");
        m.insert("zshrc", "\u{f489}");
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
        assert_eq!(icon_str, format!("{}{}", "\u{f016}", icon.icon_separator));
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
        assert_eq!(icon_str, format!("{}{}", "\u{f115}", icon.icon_separator));
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
        assert_eq!(icon_str, format!("{}{}", "\u{f115}", icon.icon_separator));
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
mod tests_llm_16_225 {
    use std::collections::HashMap;
    use crate::icon::Icons;
    #[test]
    fn test_get_default_icons_by_extension() {
        let _rug_st_tests_llm_16_225_rrrruuuugggg_test_get_default_icons_by_extension = 0;
        let rug_fuzz_0 = "7z";
        let rug_fuzz_1 = "ai";
        let rug_fuzz_2 = "apk";
        let rug_fuzz_3 = "avi";
        let rug_fuzz_4 = "avro";
        let rug_fuzz_5 = "awk";
        let rug_fuzz_6 = "bash";
        let rug_fuzz_7 = "bash_history";
        let rug_fuzz_8 = "bash_profile";
        let rug_fuzz_9 = "bashrc";
        let rug_fuzz_10 = "bat";
        let rug_fuzz_11 = "bio";
        let rug_fuzz_12 = "bmp";
        let rug_fuzz_13 = "bz2";
        let rug_fuzz_14 = "c";
        let rug_fuzz_15 = "c++";
        let rug_fuzz_16 = "cc";
        let rug_fuzz_17 = "cfg";
        let rug_fuzz_18 = "clj";
        let rug_fuzz_19 = "cljs";
        let rug_fuzz_20 = "cls";
        let rug_fuzz_21 = "coffee";
        let rug_fuzz_22 = "conf";
        let rug_fuzz_23 = "cp";
        let rug_fuzz_24 = "cpp";
        let rug_fuzz_25 = "cs";
        let rug_fuzz_26 = "cshtml";
        let rug_fuzz_27 = "csproj";
        let rug_fuzz_28 = "csx";
        let rug_fuzz_29 = "csh";
        let rug_fuzz_30 = "css";
        let rug_fuzz_31 = "csv";
        let rug_fuzz_32 = "cxx";
        let rug_fuzz_33 = "d";
        let rug_fuzz_34 = "dart";
        let rug_fuzz_35 = "db";
        let rug_fuzz_36 = "diff";
        let rug_fuzz_37 = "doc";
        let rug_fuzz_38 = "dockerfile";
        let rug_fuzz_39 = "docx";
        let rug_fuzz_40 = "ds_store";
        let rug_fuzz_41 = "dump";
        let rug_fuzz_42 = "ebook";
        let rug_fuzz_43 = "editorconfig";
        let rug_fuzz_44 = "ejs";
        let rug_fuzz_45 = "elm";
        let rug_fuzz_46 = "env";
        let rug_fuzz_47 = "eot";
        let rug_fuzz_48 = "epub";
        let rug_fuzz_49 = "erb";
        let rug_fuzz_50 = "erl";
        let rug_fuzz_51 = "exe";
        let rug_fuzz_52 = "ex";
        let rug_fuzz_53 = "exs";
        let rug_fuzz_54 = "fish";
        let rug_fuzz_55 = "flac";
        let rug_fuzz_56 = "flv";
        let rug_fuzz_57 = "font";
        let rug_fuzz_58 = "fpl";
        let rug_fuzz_59 = "fs";
        let rug_fuzz_60 = "fsx";
        let rug_fuzz_61 = "fsi";
        let rug_fuzz_62 = "gdoc";
        let rug_fuzz_63 = "gemfile";
        let rug_fuzz_64 = "gemspec";
        let rug_fuzz_65 = "gform";
        let rug_fuzz_66 = "gif";
        let rug_fuzz_67 = "git";
        let rug_fuzz_68 = "go";
        let rug_fuzz_69 = "gradle";
        let rug_fuzz_70 = "gsheet";
        let rug_fuzz_71 = "gslides";
        let rug_fuzz_72 = "guardfile";
        let rug_fuzz_73 = "gz";
        let rug_fuzz_74 = "h";
        let rug_fuzz_75 = "hbs";
        let rug_fuzz_76 = "heic";
        let rug_fuzz_77 = "heif";
        let rug_fuzz_78 = "heix";
        let rug_fuzz_79 = "hpp";
        let rug_fuzz_80 = "hs";
        let rug_fuzz_81 = "htm";
        let rug_fuzz_82 = "html";
        let rug_fuzz_83 = "hxx";
        let rug_fuzz_84 = "ico";
        let rug_fuzz_85 = "image";
        let rug_fuzz_86 = "iml";
        let rug_fuzz_87 = "ini";
        let rug_fuzz_88 = "ipynb";
        let rug_fuzz_89 = "jar";
        let rug_fuzz_90 = "java";
        let rug_fuzz_91 = "jpeg";
        let rug_fuzz_92 = "jpg";
        let rug_fuzz_93 = "js";
        let rug_fuzz_94 = "json";
        let rug_fuzz_95 = "jsx";
        let rug_fuzz_96 = "jl";
        let rug_fuzz_97 = "ksh";
        let rug_fuzz_98 = "less";
        let rug_fuzz_99 = "lhs";
        let rug_fuzz_100 = "license";
        let rug_fuzz_101 = "localized";
        let rug_fuzz_102 = "lock";
        let rug_fuzz_103 = "log";
        let rug_fuzz_104 = "lua";
        let rug_fuzz_105 = "lz";
        let rug_fuzz_106 = "m3u";
        let rug_fuzz_107 = "m3u8";
        let rug_fuzz_108 = "m4a";
        let rug_fuzz_109 = "magnet";
        let rug_fuzz_110 = "markdown";
        let rug_fuzz_111 = "md";
        let rug_fuzz_112 = "mjs";
        let rug_fuzz_113 = "mkd";
        let rug_fuzz_114 = "mkv";
        let rug_fuzz_115 = "mobi";
        let rug_fuzz_116 = "mov";
        let rug_fuzz_117 = "mp3";
        let rug_fuzz_118 = "mp4";
        let rug_fuzz_119 = "mustache";
        let rug_fuzz_120 = "nix";
        let rug_fuzz_121 = "npmignore";
        let rug_fuzz_122 = "opus";
        let rug_fuzz_123 = "ogg";
        let rug_fuzz_124 = "ogv";
        let rug_fuzz_125 = "otf";
        let rug_fuzz_126 = "pdf";
        let rug_fuzz_127 = "pem";
        let rug_fuzz_128 = "php";
        let rug_fuzz_129 = "pl";
        let rug_fuzz_130 = "pls";
        let rug_fuzz_131 = "pm";
        let rug_fuzz_132 = "png";
        let rug_fuzz_133 = "ppt";
        let rug_fuzz_134 = "pptx";
        let rug_fuzz_135 = "procfile";
        let rug_fuzz_136 = "properties";
        let rug_fuzz_137 = "ps1";
        let rug_fuzz_138 = "psd";
        let rug_fuzz_139 = "pxm";
        let rug_fuzz_140 = "py";
        let rug_fuzz_141 = "pyc";
        let rug_fuzz_142 = "r";
        let rug_fuzz_143 = "rakefile";
        let rug_fuzz_144 = "rar";
        let rug_fuzz_145 = "razor";
        let rug_fuzz_146 = "rb";
        let rug_fuzz_147 = "rdata";
        let rug_fuzz_148 = "rdb";
        let rug_fuzz_149 = "rdoc";
        let rug_fuzz_150 = "rds";
        let rug_fuzz_151 = "readme";
        let rug_fuzz_152 = "rlib";
        let rug_fuzz_153 = "rmd";
        let rug_fuzz_154 = "rs";
        let rug_fuzz_155 = "rspec";
        let rug_fuzz_156 = "rspec_parallel";
        let rug_fuzz_157 = "rspec_status";
        let rug_fuzz_158 = "rss";
        let rug_fuzz_159 = "ru";
        let rug_fuzz_160 = "rubydoc";
        let rug_fuzz_161 = "sass";
        let rug_fuzz_162 = "scala";
        let rug_fuzz_163 = "scpt";
        let rug_fuzz_164 = "scss";
        let rug_fuzz_165 = "sh";
        let rug_fuzz_166 = "shell";
        let rug_fuzz_167 = "slim";
        let rug_fuzz_168 = "sln";
        let rug_fuzz_169 = "sql";
        let rug_fuzz_170 = "sqlite3";
        let rug_fuzz_171 = "styl";
        let rug_fuzz_172 = "stylus";
        let rug_fuzz_173 = "svg";
        let rug_fuzz_174 = "swift";
        let rug_fuzz_175 = "t";
        let rug_fuzz_176 = "tar";
        let rug_fuzz_177 = "tex";
        let rug_fuzz_178 = "tiff";
        let rug_fuzz_179 = "toml";
        let rug_fuzz_180 = "torrent";
        let rug_fuzz_181 = "ts";
        let rug_fuzz_182 = "tsx";
        let rug_fuzz_183 = "ttc";
        let rug_fuzz_184 = "ttf";
        let rug_fuzz_185 = "twig";
        let rug_fuzz_186 = "txt";
        let rug_fuzz_187 = "video";
        let rug_fuzz_188 = "vim";
        let rug_fuzz_189 = "vlc";
        let rug_fuzz_190 = "vue";
        let rug_fuzz_191 = "wav";
        let rug_fuzz_192 = "webm";
        let rug_fuzz_193 = "webp";
        let rug_fuzz_194 = "windows";
        let rug_fuzz_195 = "wma";
        let rug_fuzz_196 = "wmv";
        let rug_fuzz_197 = "wpl";
        let rug_fuzz_198 = "woff";
        let rug_fuzz_199 = "woff2";
        let rug_fuzz_200 = "xls";
        let rug_fuzz_201 = "xlsx";
        let rug_fuzz_202 = "xml";
        let rug_fuzz_203 = "xul";
        let rug_fuzz_204 = "xz";
        let rug_fuzz_205 = "yaml";
        let rug_fuzz_206 = "yml";
        let rug_fuzz_207 = "zip";
        let rug_fuzz_208 = "zsh";
        let rug_fuzz_209 = "zsh-theme";
        let rug_fuzz_210 = "zshrc";
        let icons = Icons::get_default_icons_by_extension();
        debug_assert_eq!(icons.get(rug_fuzz_0), Some(& "\u{f410}"));
        debug_assert_eq!(icons.get(rug_fuzz_1), Some(& "\u{e7b4}"));
        debug_assert_eq!(icons.get(rug_fuzz_2), Some(& "\u{e70e}"));
        debug_assert_eq!(icons.get(rug_fuzz_3), Some(& "\u{f03d}"));
        debug_assert_eq!(icons.get(rug_fuzz_4), Some(& "\u{e60b}"));
        debug_assert_eq!(icons.get(rug_fuzz_5), Some(& "\u{f489}"));
        debug_assert_eq!(icons.get(rug_fuzz_6), Some(& "\u{f489}"));
        debug_assert_eq!(icons.get(rug_fuzz_7), Some(& "\u{f489}"));
        debug_assert_eq!(icons.get(rug_fuzz_8), Some(& "\u{f489}"));
        debug_assert_eq!(icons.get(rug_fuzz_9), Some(& "\u{f489}"));
        debug_assert_eq!(icons.get(rug_fuzz_10), Some(& "\u{f17a}"));
        debug_assert_eq!(icons.get(rug_fuzz_11), Some(& "\u{f910}"));
        debug_assert_eq!(icons.get(rug_fuzz_12), Some(& "\u{f1c5}"));
        debug_assert_eq!(icons.get(rug_fuzz_13), Some(& "\u{f410}"));
        debug_assert_eq!(icons.get(rug_fuzz_14), Some(& "\u{e61e}"));
        debug_assert_eq!(icons.get(rug_fuzz_15), Some(& "\u{e61d}"));
        debug_assert_eq!(icons.get(rug_fuzz_16), Some(& "\u{e61d}"));
        debug_assert_eq!(icons.get(rug_fuzz_17), Some(& "\u{e615}"));
        debug_assert_eq!(icons.get(rug_fuzz_18), Some(& "\u{e768}"));
        debug_assert_eq!(icons.get(rug_fuzz_19), Some(& "\u{e76a}"));
        debug_assert_eq!(icons.get(rug_fuzz_20), Some(& "\u{e600}"));
        debug_assert_eq!(icons.get(rug_fuzz_21), Some(& "\u{f0f4}"));
        debug_assert_eq!(icons.get(rug_fuzz_22), Some(& "\u{e615}"));
        debug_assert_eq!(icons.get(rug_fuzz_23), Some(& "\u{e61d}"));
        debug_assert_eq!(icons.get(rug_fuzz_24), Some(& "\u{e61d}"));
        debug_assert_eq!(icons.get(rug_fuzz_25), Some(& "\u{f81a}"));
        debug_assert_eq!(icons.get(rug_fuzz_26), Some(& "\u{f1fa}"));
        debug_assert_eq!(icons.get(rug_fuzz_27), Some(& "\u{f81a}"));
        debug_assert_eq!(icons.get(rug_fuzz_28), Some(& "\u{f81a}"));
        debug_assert_eq!(icons.get(rug_fuzz_29), Some(& "\u{f489}"));
        debug_assert_eq!(icons.get(rug_fuzz_30), Some(& "\u{e749}"));
        debug_assert_eq!(icons.get(rug_fuzz_31), Some(& "\u{f1c3}"));
        debug_assert_eq!(icons.get(rug_fuzz_32), Some(& "\u{e61d}"));
        debug_assert_eq!(icons.get(rug_fuzz_33), Some(& "\u{e7af}"));
        debug_assert_eq!(icons.get(rug_fuzz_34), Some(& "\u{e798}"));
        debug_assert_eq!(icons.get(rug_fuzz_35), Some(& "\u{f1c0}"));
        debug_assert_eq!(icons.get(rug_fuzz_36), Some(& "\u{f440}"));
        debug_assert_eq!(icons.get(rug_fuzz_37), Some(& "\u{f1c2}"));
        debug_assert_eq!(icons.get(rug_fuzz_38), Some(& "\u{f308}"));
        debug_assert_eq!(icons.get(rug_fuzz_39), Some(& "\u{f1c2}"));
        debug_assert_eq!(icons.get(rug_fuzz_40), Some(& "\u{f179}"));
        debug_assert_eq!(icons.get(rug_fuzz_41), Some(& "\u{f1c0}"));
        debug_assert_eq!(icons.get(rug_fuzz_42), Some(& "\u{e28b}"));
        debug_assert_eq!(icons.get(rug_fuzz_43), Some(& "\u{e615}"));
        debug_assert_eq!(icons.get(rug_fuzz_44), Some(& "\u{e618}"));
        debug_assert_eq!(icons.get(rug_fuzz_45), Some(& "\u{e62c}"));
        debug_assert_eq!(icons.get(rug_fuzz_46), Some(& "\u{f462}"));
        debug_assert_eq!(icons.get(rug_fuzz_47), Some(& "\u{f031}"));
        debug_assert_eq!(icons.get(rug_fuzz_48), Some(& "\u{e28a}"));
        debug_assert_eq!(icons.get(rug_fuzz_49), Some(& "\u{e73b}"));
        debug_assert_eq!(icons.get(rug_fuzz_50), Some(& "\u{e7b1}"));
        debug_assert_eq!(icons.get(rug_fuzz_51), Some(& "\u{f17a}"));
        debug_assert_eq!(icons.get(rug_fuzz_52), Some(& "\u{e62d}"));
        debug_assert_eq!(icons.get(rug_fuzz_53), Some(& "\u{e62d}"));
        debug_assert_eq!(icons.get(rug_fuzz_54), Some(& "\u{f489}"));
        debug_assert_eq!(icons.get(rug_fuzz_55), Some(& "\u{f001}"));
        debug_assert_eq!(icons.get(rug_fuzz_56), Some(& "\u{f03d}"));
        debug_assert_eq!(icons.get(rug_fuzz_57), Some(& "\u{f031}"));
        debug_assert_eq!(icons.get(rug_fuzz_58), Some(& "\u{f910}"));
        debug_assert_eq!(icons.get(rug_fuzz_59), Some(& "\u{e7a7}"));
        debug_assert_eq!(icons.get(rug_fuzz_60), Some(& "\u{e7a7}"));
        debug_assert_eq!(icons.get(rug_fuzz_61), Some(& "\u{e7a7}"));
        debug_assert_eq!(icons.get(rug_fuzz_62), Some(& "\u{f1c2}"));
        debug_assert_eq!(icons.get(rug_fuzz_63), Some(& "\u{e21e}"));
        debug_assert_eq!(icons.get(rug_fuzz_64), Some(& "\u{e21e}"));
        debug_assert_eq!(icons.get(rug_fuzz_65), Some(& "\u{f298}"));
        debug_assert_eq!(icons.get(rug_fuzz_66), Some(& "\u{f1c5}"));
        debug_assert_eq!(icons.get(rug_fuzz_67), Some(& "\u{f1d3}"));
        debug_assert_eq!(icons.get(rug_fuzz_68), Some(& "\u{e626}"));
        debug_assert_eq!(icons.get(rug_fuzz_69), Some(& "\u{e70e}"));
        debug_assert_eq!(icons.get(rug_fuzz_70), Some(& "\u{f1c3}"));
        debug_assert_eq!(icons.get(rug_fuzz_71), Some(& "\u{f1c4}"));
        debug_assert_eq!(icons.get(rug_fuzz_72), Some(& "\u{e21e}"));
        debug_assert_eq!(icons.get(rug_fuzz_73), Some(& "\u{f410}"));
        debug_assert_eq!(icons.get(rug_fuzz_74), Some(& "\u{f0fd}"));
        debug_assert_eq!(icons.get(rug_fuzz_75), Some(& "\u{e60f}"));
        debug_assert_eq!(icons.get(rug_fuzz_76), Some(& "\u{f1c5}"));
        debug_assert_eq!(icons.get(rug_fuzz_77), Some(& "\u{f1c5}"));
        debug_assert_eq!(icons.get(rug_fuzz_78), Some(& "\u{f1c5}"));
        debug_assert_eq!(icons.get(rug_fuzz_79), Some(& "\u{f0fd}"));
        debug_assert_eq!(icons.get(rug_fuzz_80), Some(& "\u{e777}"));
        debug_assert_eq!(icons.get(rug_fuzz_81), Some(& "\u{f13b}"));
        debug_assert_eq!(icons.get(rug_fuzz_82), Some(& "\u{f13b}"));
        debug_assert_eq!(icons.get(rug_fuzz_83), Some(& "\u{f0fd}"));
        debug_assert_eq!(icons.get(rug_fuzz_84), Some(& "\u{f1c5}"));
        debug_assert_eq!(icons.get(rug_fuzz_85), Some(& "\u{f1c5}"));
        debug_assert_eq!(icons.get(rug_fuzz_86), Some(& "\u{e7b5}"));
        debug_assert_eq!(icons.get(rug_fuzz_87), Some(& "\u{e615}"));
        debug_assert_eq!(icons.get(rug_fuzz_88), Some(& "\u{e606}"));
        debug_assert_eq!(icons.get(rug_fuzz_89), Some(& "\u{e204}"));
        debug_assert_eq!(icons.get(rug_fuzz_90), Some(& "\u{e204}"));
        debug_assert_eq!(icons.get(rug_fuzz_91), Some(& "\u{f1c5}"));
        debug_assert_eq!(icons.get(rug_fuzz_92), Some(& "\u{f1c5}"));
        debug_assert_eq!(icons.get(rug_fuzz_93), Some(& "\u{e74e}"));
        debug_assert_eq!(icons.get(rug_fuzz_94), Some(& "\u{e60b}"));
        debug_assert_eq!(icons.get(rug_fuzz_95), Some(& "\u{e7ba}"));
        debug_assert_eq!(icons.get(rug_fuzz_96), Some(& "\u{e624}"));
        debug_assert_eq!(icons.get(rug_fuzz_97), Some(& "\u{f489}"));
        debug_assert_eq!(icons.get(rug_fuzz_98), Some(& "\u{e758}"));
        debug_assert_eq!(icons.get(rug_fuzz_99), Some(& "\u{e777}"));
        debug_assert_eq!(icons.get(rug_fuzz_100), Some(& "\u{f48a}"));
        debug_assert_eq!(icons.get(rug_fuzz_101), Some(& "\u{f179}"));
        debug_assert_eq!(icons.get(rug_fuzz_102), Some(& "\u{f023}"));
        debug_assert_eq!(icons.get(rug_fuzz_103), Some(& "\u{f18d}"));
        debug_assert_eq!(icons.get(rug_fuzz_104), Some(& "\u{e620}"));
        debug_assert_eq!(icons.get(rug_fuzz_105), Some(& "\u{f410}"));
        debug_assert_eq!(icons.get(rug_fuzz_106), Some(& "\u{f910}"));
        debug_assert_eq!(icons.get(rug_fuzz_107), Some(& "\u{f910}"));
        debug_assert_eq!(icons.get(rug_fuzz_108), Some(& "\u{f001}"));
        debug_assert_eq!(icons.get(rug_fuzz_109), Some(& "\u{f076}"));
        debug_assert_eq!(icons.get(rug_fuzz_110), Some(& "\u{f48a}"));
        debug_assert_eq!(icons.get(rug_fuzz_111), Some(& "\u{f48a}"));
        debug_assert_eq!(icons.get(rug_fuzz_112), Some(& "\u{e74e}"));
        debug_assert_eq!(icons.get(rug_fuzz_113), Some(& "\u{f48a}"));
        debug_assert_eq!(icons.get(rug_fuzz_114), Some(& "\u{f03d}"));
        debug_assert_eq!(icons.get(rug_fuzz_115), Some(& "\u{e28b}"));
        debug_assert_eq!(icons.get(rug_fuzz_116), Some(& "\u{f03d}"));
        debug_assert_eq!(icons.get(rug_fuzz_117), Some(& "\u{f001}"));
        debug_assert_eq!(icons.get(rug_fuzz_118), Some(& "\u{f03d}"));
        debug_assert_eq!(icons.get(rug_fuzz_119), Some(& "\u{e60f}"));
        debug_assert_eq!(icons.get(rug_fuzz_120), Some(& "\u{f313}"));
        debug_assert_eq!(icons.get(rug_fuzz_121), Some(& "\u{e71e}"));
        debug_assert_eq!(icons.get(rug_fuzz_122), Some(& "\u{f001}"));
        debug_assert_eq!(icons.get(rug_fuzz_123), Some(& "\u{f001}"));
        debug_assert_eq!(icons.get(rug_fuzz_124), Some(& "\u{f03d}"));
        debug_assert_eq!(icons.get(rug_fuzz_125), Some(& "\u{f031}"));
        debug_assert_eq!(icons.get(rug_fuzz_126), Some(& "\u{f1c1}"));
        debug_assert_eq!(icons.get(rug_fuzz_127), Some(& "\u{f805}"));
        debug_assert_eq!(icons.get(rug_fuzz_128), Some(& "\u{e73d}"));
        debug_assert_eq!(icons.get(rug_fuzz_129), Some(& "\u{e769}"));
        debug_assert_eq!(icons.get(rug_fuzz_130), Some(& "\u{f910}"));
        debug_assert_eq!(icons.get(rug_fuzz_131), Some(& "\u{e769}"));
        debug_assert_eq!(icons.get(rug_fuzz_132), Some(& "\u{f1c5}"));
        debug_assert_eq!(icons.get(rug_fuzz_133), Some(& "\u{f1c4}"));
        debug_assert_eq!(icons.get(rug_fuzz_134), Some(& "\u{f1c4}"));
        debug_assert_eq!(icons.get(rug_fuzz_135), Some(& "\u{e21e}"));
        debug_assert_eq!(icons.get(rug_fuzz_136), Some(& "\u{e60b}"));
        debug_assert_eq!(icons.get(rug_fuzz_137), Some(& "\u{f489}"));
        debug_assert_eq!(icons.get(rug_fuzz_138), Some(& "\u{e7b8}"));
        debug_assert_eq!(icons.get(rug_fuzz_139), Some(& "\u{f1c5}"));
        debug_assert_eq!(icons.get(rug_fuzz_140), Some(& "\u{e606}"));
        debug_assert_eq!(icons.get(rug_fuzz_141), Some(& "\u{e606}"));
        debug_assert_eq!(icons.get(rug_fuzz_142), Some(& "\u{f25d}"));
        debug_assert_eq!(icons.get(rug_fuzz_143), Some(& "\u{e21e}"));
        debug_assert_eq!(icons.get(rug_fuzz_144), Some(& "\u{f410}"));
        debug_assert_eq!(icons.get(rug_fuzz_145), Some(& "\u{f1fa}"));
        debug_assert_eq!(icons.get(rug_fuzz_146), Some(& "\u{e21e}"));
        debug_assert_eq!(icons.get(rug_fuzz_147), Some(& "\u{f25d}"));
        debug_assert_eq!(icons.get(rug_fuzz_148), Some(& "\u{e76d}"));
        debug_assert_eq!(icons.get(rug_fuzz_149), Some(& "\u{f48a}"));
        debug_assert_eq!(icons.get(rug_fuzz_150), Some(& "\u{f25d}"));
        debug_assert_eq!(icons.get(rug_fuzz_151), Some(& "\u{f48a}"));
        debug_assert_eq!(icons.get(rug_fuzz_152), Some(& "\u{e7a8}"));
        debug_assert_eq!(icons.get(rug_fuzz_153), Some(& "\u{f48a}"));
        debug_assert_eq!(icons.get(rug_fuzz_154), Some(& "\u{e7a8}"));
        debug_assert_eq!(icons.get(rug_fuzz_155), Some(& "\u{e21e}"));
        debug_assert_eq!(icons.get(rug_fuzz_156), Some(& "\u{e21e}"));
        debug_assert_eq!(icons.get(rug_fuzz_157), Some(& "\u{e21e}"));
        debug_assert_eq!(icons.get(rug_fuzz_158), Some(& "\u{f09e}"));
        debug_assert_eq!(icons.get(rug_fuzz_159), Some(& "\u{e21e}"));
        debug_assert_eq!(icons.get(rug_fuzz_160), Some(& "\u{e73b}"));
        debug_assert_eq!(icons.get(rug_fuzz_161), Some(& "\u{e603}"));
        debug_assert_eq!(icons.get(rug_fuzz_162), Some(& "\u{e737}"));
        debug_assert_eq!(icons.get(rug_fuzz_163), Some(& "\u{f302}"));
        debug_assert_eq!(icons.get(rug_fuzz_164), Some(& "\u{e749}"));
        debug_assert_eq!(icons.get(rug_fuzz_165), Some(& "\u{f489}"));
        debug_assert_eq!(icons.get(rug_fuzz_166), Some(& "\u{f489}"));
        debug_assert_eq!(icons.get(rug_fuzz_167), Some(& "\u{e73b}"));
        debug_assert_eq!(icons.get(rug_fuzz_168), Some(& "\u{e70c}"));
        debug_assert_eq!(icons.get(rug_fuzz_169), Some(& "\u{f1c0}"));
        debug_assert_eq!(icons.get(rug_fuzz_170), Some(& "\u{e7c4}"));
        debug_assert_eq!(icons.get(rug_fuzz_171), Some(& "\u{e600}"));
        debug_assert_eq!(icons.get(rug_fuzz_172), Some(& "\u{e600}"));
        debug_assert_eq!(icons.get(rug_fuzz_173), Some(& "\u{f1c5}"));
        debug_assert_eq!(icons.get(rug_fuzz_174), Some(& "\u{e755}"));
        debug_assert_eq!(icons.get(rug_fuzz_175), Some(& "\u{e769}"));
        debug_assert_eq!(icons.get(rug_fuzz_176), Some(& "\u{f410}"));
        debug_assert_eq!(icons.get(rug_fuzz_177), Some(& "\u{e600}"));
        debug_assert_eq!(icons.get(rug_fuzz_178), Some(& "\u{f1c5}"));
        debug_assert_eq!(icons.get(rug_fuzz_179), Some(& "\u{e60b}"));
        debug_assert_eq!(icons.get(rug_fuzz_180), Some(& "\u{f98c}"));
        debug_assert_eq!(icons.get(rug_fuzz_181), Some(& "\u{e628}"));
        debug_assert_eq!(icons.get(rug_fuzz_182), Some(& "\u{e7ba}"));
        debug_assert_eq!(icons.get(rug_fuzz_183), Some(& "\u{f031}"));
        debug_assert_eq!(icons.get(rug_fuzz_184), Some(& "\u{f031}"));
        debug_assert_eq!(icons.get(rug_fuzz_185), Some(& "\u{e61c}"));
        debug_assert_eq!(icons.get(rug_fuzz_186), Some(& "\u{f15c}"));
        debug_assert_eq!(icons.get(rug_fuzz_187), Some(& "\u{f03d}"));
        debug_assert_eq!(icons.get(rug_fuzz_188), Some(& "\u{e62b}"));
        debug_assert_eq!(icons.get(rug_fuzz_189), Some(& "\u{f910}"));
        debug_assert_eq!(icons.get(rug_fuzz_190), Some(& "\u{fd42}"));
        debug_assert_eq!(icons.get(rug_fuzz_191), Some(& "\u{f001}"));
        debug_assert_eq!(icons.get(rug_fuzz_192), Some(& "\u{f03d}"));
        debug_assert_eq!(icons.get(rug_fuzz_193), Some(& "\u{f1c5}"));
        debug_assert_eq!(icons.get(rug_fuzz_194), Some(& "\u{f17a}"));
        debug_assert_eq!(icons.get(rug_fuzz_195), Some(& "\u{f001}"));
        debug_assert_eq!(icons.get(rug_fuzz_196), Some(& "\u{f03d}"));
        debug_assert_eq!(icons.get(rug_fuzz_197), Some(& "\u{f910}"));
        debug_assert_eq!(icons.get(rug_fuzz_198), Some(& "\u{f031}"));
        debug_assert_eq!(icons.get(rug_fuzz_199), Some(& "\u{f031}"));
        debug_assert_eq!(icons.get(rug_fuzz_200), Some(& "\u{f1c3}"));
        debug_assert_eq!(icons.get(rug_fuzz_201), Some(& "\u{f1c3}"));
        debug_assert_eq!(icons.get(rug_fuzz_202), Some(& "\u{e619}"));
        debug_assert_eq!(icons.get(rug_fuzz_203), Some(& "\u{e619}"));
        debug_assert_eq!(icons.get(rug_fuzz_204), Some(& "\u{f410}"));
        debug_assert_eq!(icons.get(rug_fuzz_205), Some(& "\u{e60b}"));
        debug_assert_eq!(icons.get(rug_fuzz_206), Some(& "\u{e60b}"));
        debug_assert_eq!(icons.get(rug_fuzz_207), Some(& "\u{f410}"));
        debug_assert_eq!(icons.get(rug_fuzz_208), Some(& "\u{f489}"));
        debug_assert_eq!(icons.get(rug_fuzz_209), Some(& "\u{f489}"));
        debug_assert_eq!(icons.get(rug_fuzz_210), Some(& "\u{f489}"));
        let _rug_ed_tests_llm_16_225_rrrruuuugggg_test_get_default_icons_by_extension = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_228 {
    use super::*;
    use crate::*;
    #[test]
    fn test_new() {
        let _rug_st_tests_llm_16_228_rrrruuuugggg_test_new = 0;
        let rug_fuzz_0 = "_";
        let theme = Theme::Fancy;
        let icon_separator = String::from(rug_fuzz_0);
        let icons = Icons::new(theme, icon_separator);
        debug_assert_eq!(icons.display_icons, true);
        debug_assert_eq!(icons.icon_separator, "_");
        let _rug_ed_tests_llm_16_228_rrrruuuugggg_test_new = 0;
    }
}
#[cfg(test)]
mod tests_rug_86 {
    use super::*;
    use std::collections::HashMap;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_86_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = ".trash";
        let rug_fuzz_1 = "\u{f1f8}";
        let rug_fuzz_2 = ".atom";
        let rug_fuzz_3 = "\u{e764}";
        let rug_fuzz_4 = ".bashprofile";
        let rug_fuzz_5 = "\u{e615}";
        let rug_fuzz_6 = ".bashrc";
        let rug_fuzz_7 = "\u{f489}";
        let rug_fuzz_8 = ".clang-format";
        let rug_fuzz_9 = "\u{e615}";
        let rug_fuzz_10 = ".git";
        let rug_fuzz_11 = "\u{f1d3}";
        let rug_fuzz_12 = ".gitattributes";
        let rug_fuzz_13 = "\u{f1d3}";
        let rug_fuzz_14 = ".gitconfig";
        let rug_fuzz_15 = "\u{f1d3}";
        let rug_fuzz_16 = ".github";
        let rug_fuzz_17 = "\u{f408}";
        let rug_fuzz_18 = ".gitignore";
        let rug_fuzz_19 = "\u{f1d3}";
        let rug_fuzz_20 = ".gitmodules";
        let rug_fuzz_21 = "\u{f1d3}";
        let rug_fuzz_22 = ".rvm";
        let rug_fuzz_23 = "\u{e21e}";
        let rug_fuzz_24 = ".vimrc";
        let rug_fuzz_25 = "\u{e62b}";
        let rug_fuzz_26 = ".vscode";
        let rug_fuzz_27 = "\u{e70c}";
        let rug_fuzz_28 = ".zshrc";
        let rug_fuzz_29 = "\u{f489}";
        let rug_fuzz_30 = "bin";
        let rug_fuzz_31 = "\u{e5fc}";
        let rug_fuzz_32 = "config";
        let rug_fuzz_33 = "\u{e5fc}";
        let rug_fuzz_34 = "docker-compose.yml";
        let rug_fuzz_35 = "\u{f308}";
        let rug_fuzz_36 = "dockerfile";
        let rug_fuzz_37 = "\u{f308}";
        let rug_fuzz_38 = "ds_store";
        let rug_fuzz_39 = "\u{f179}";
        let rug_fuzz_40 = "gitignore_global";
        let rug_fuzz_41 = "\u{f1d3}";
        let rug_fuzz_42 = "gradle";
        let rug_fuzz_43 = "\u{e70e}";
        let rug_fuzz_44 = "gruntfile.coffee";
        let rug_fuzz_45 = "\u{e611}";
        let rug_fuzz_46 = "gruntfile.js";
        let rug_fuzz_47 = "\u{e611}";
        let rug_fuzz_48 = "gruntfile.ls";
        let rug_fuzz_49 = "\u{e611}";
        let rug_fuzz_50 = "gulpfile.coffee";
        let rug_fuzz_51 = "\u{e610}";
        let rug_fuzz_52 = "gulpfile.js";
        let rug_fuzz_53 = "\u{e610}";
        let rug_fuzz_54 = "gulpfile.ls";
        let rug_fuzz_55 = "\u{e610}";
        let rug_fuzz_56 = "hidden";
        let rug_fuzz_57 = "\u{f023}";
        let rug_fuzz_58 = "include";
        let rug_fuzz_59 = "\u{e5fc}";
        let rug_fuzz_60 = "lib";
        let rug_fuzz_61 = "\u{f121}";
        let rug_fuzz_62 = "localized";
        let rug_fuzz_63 = "\u{f179}";
        let rug_fuzz_64 = "node_modules";
        let rug_fuzz_65 = "\u{e718}";
        let rug_fuzz_66 = "npmignore";
        let rug_fuzz_67 = "\u{e71e}";
        let rug_fuzz_68 = "rubydoc";
        let rug_fuzz_69 = "\u{e73b}";
        let expected: HashMap<&'static str, &'static str> = {
            let mut m = HashMap::new();
            m.insert(rug_fuzz_0, rug_fuzz_1);
            m.insert(rug_fuzz_2, rug_fuzz_3);
            m.insert(rug_fuzz_4, rug_fuzz_5);
            m.insert(rug_fuzz_6, rug_fuzz_7);
            m.insert(rug_fuzz_8, rug_fuzz_9);
            m.insert(rug_fuzz_10, rug_fuzz_11);
            m.insert(rug_fuzz_12, rug_fuzz_13);
            m.insert(rug_fuzz_14, rug_fuzz_15);
            m.insert(rug_fuzz_16, rug_fuzz_17);
            m.insert(rug_fuzz_18, rug_fuzz_19);
            m.insert(rug_fuzz_20, rug_fuzz_21);
            m.insert(rug_fuzz_22, rug_fuzz_23);
            m.insert(rug_fuzz_24, rug_fuzz_25);
            m.insert(rug_fuzz_26, rug_fuzz_27);
            m.insert(rug_fuzz_28, rug_fuzz_29);
            m.insert(rug_fuzz_30, rug_fuzz_31);
            m.insert(rug_fuzz_32, rug_fuzz_33);
            m.insert(rug_fuzz_34, rug_fuzz_35);
            m.insert(rug_fuzz_36, rug_fuzz_37);
            m.insert(rug_fuzz_38, rug_fuzz_39);
            m.insert(rug_fuzz_40, rug_fuzz_41);
            m.insert(rug_fuzz_42, rug_fuzz_43);
            m.insert(rug_fuzz_44, rug_fuzz_45);
            m.insert(rug_fuzz_46, rug_fuzz_47);
            m.insert(rug_fuzz_48, rug_fuzz_49);
            m.insert(rug_fuzz_50, rug_fuzz_51);
            m.insert(rug_fuzz_52, rug_fuzz_53);
            m.insert(rug_fuzz_54, rug_fuzz_55);
            m.insert(rug_fuzz_56, rug_fuzz_57);
            m.insert(rug_fuzz_58, rug_fuzz_59);
            m.insert(rug_fuzz_60, rug_fuzz_61);
            m.insert(rug_fuzz_62, rug_fuzz_63);
            m.insert(rug_fuzz_64, rug_fuzz_65);
            m.insert(rug_fuzz_66, rug_fuzz_67);
            m.insert(rug_fuzz_68, rug_fuzz_69);
            m
        };
        let result = Icons::get_default_icons_by_name();
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_rug_86_rrrruuuugggg_test_rug = 0;
    }
}
