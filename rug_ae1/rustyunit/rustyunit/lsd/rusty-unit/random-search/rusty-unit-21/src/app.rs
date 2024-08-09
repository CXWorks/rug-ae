use clap::{App, Arg};

pub fn build() -> App<'static, 'static> {
    App::new("lsd")
        .version(crate_version!())
        .about(crate_description!())
        .arg(Arg::with_name("FILE").multiple(true).default_value("."))
        .arg(
            Arg::with_name("all")
                .short("a")
                .overrides_with("almost-all")
                .long("all")
                .multiple(true)
                .help("Do not ignore entries starting with ."),
        )
        .arg(
            Arg::with_name("almost-all")
                .short("A")
                .overrides_with("all")
                .long("almost-all")
                .multiple(true)
                .help("Do not list implied . and .."),
        )
        .arg(
            Arg::with_name("color")
                .long("color")
                .possible_value("always")
                .possible_value("auto")
                .possible_value("never")
                .default_value("auto")
                .multiple(true)
                .number_of_values(1)
                .help("When to use terminal colours"),
        )
        .arg(
            Arg::with_name("icon")
                .long("icon")
                .possible_value("always")
                .possible_value("auto")
                .possible_value("never")
                .default_value("auto")
                .multiple(true)
                .number_of_values(1)
                .help("When to print the icons"),
        )
        .arg(
            Arg::with_name("icon-theme")
                .long("icon-theme")
                .possible_value("fancy")
                .possible_value("unicode")
                .default_value("fancy")
                .multiple(true)
                .number_of_values(1)
                .help("Whether to use fancy or unicode icons"),
        )
        .arg(
            Arg::with_name("indicators")
                .short("F")
                .long("classify")
                .multiple(true)
                .help("Append indicator (one of */=>@|) at the end of the file names"),
        )
        .arg(
            Arg::with_name("long")
                .short("l")
                .long("long")
                .multiple(true)
                .help("Display extended file metadata as a table"),
        )
        .arg(
            Arg::with_name("ignore-config")
                .long("ignore-config")
                .help("Ignore the configuration file"),
        )
        .arg(
            Arg::with_name("config-file")
                .long("config-file")
                .help("Provide a custom lsd configuration file")
                .value_name("config-file")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("oneline")
                .short("1")
                .long("oneline")
                .multiple(true)
                .help("Display one entry per line"),
        )
        .arg(
            Arg::with_name("recursive")
                .short("R")
                .long("recursive")
                .multiple(true)
                .conflicts_with("tree")
                .help("Recurse into directories"),
        )
        .arg(
            Arg::with_name("human_readable")
                .short("h")
                .long("human-readable")
                .multiple(true)
                .help("For ls compatibility purposes ONLY, currently set by default"),
        )
        .arg(
            Arg::with_name("tree")
                .long("tree")
                .multiple(true)
                .conflicts_with("recursive")
                .help("Recurse into directories and present the result as a tree"),
        )
        .arg(
            Arg::with_name("depth")
                .long("depth")
                .multiple(true)
                .takes_value(true)
                .value_name("num")
                .help("Stop recursing into directories after reaching specified depth"),
        )
        .arg(
            Arg::with_name("directory-only")
                .short("d")
                .long("directory-only")
                .conflicts_with("depth")
                .conflicts_with("recursive")
                .help("Display directories themselves, and not their contents (recursively when used with --tree)"),
        )
        .arg(
            Arg::with_name("permission")
                .long("permission")
                .default_value("rwx")
                .possible_value("rwx")
                .possible_value("octal")
                .multiple(true)
                .number_of_values(1)
                .help("How to display permissions"),
        )
        .arg(
            Arg::with_name("size")
                .long("size")
                .possible_value("default")
                .possible_value("short")
                .possible_value("bytes")
                .default_value("default")
                .multiple(true)
                .number_of_values(1)
                .help("How to display size"),
        )
        .arg(
            Arg::with_name("total-size")
                .long("total-size")
                .multiple(true)
                .help("Display the total size of directories"),
        )
        .arg(
            Arg::with_name("date")
                .long("date")
                .validator(validate_date_argument)
                .default_value("date")
                .multiple(true)
                .number_of_values(1)
                .help("How to display date [possible values: date, relative, +date-time-format]"),
        )
        .arg(
            Arg::with_name("timesort")
                .short("t")
                .long("timesort")
                .overrides_with("sizesort")
                .overrides_with("extensionsort")
                .overrides_with("versionsort")
                .overrides_with("sort")
                .overrides_with("no-sort")
                .multiple(true)
                .help("Sort by time modified"),
        )
        .arg(
            Arg::with_name("sizesort")
                .short("S")
                .long("sizesort")
                .overrides_with("timesort")
                .overrides_with("extensionsort")
                .overrides_with("versionsort")
                .overrides_with("sort")
                .overrides_with("no-sort")
                .multiple(true)
                .help("Sort by size"),
        )
        .arg(
            Arg::with_name("extensionsort")
                .short("X")
                .long("extensionsort")
                .overrides_with("sizesort")
                .overrides_with("timesort")
                .overrides_with("versionsort")
                .overrides_with("sort")
                .overrides_with("no-sort")
                .multiple(true)
                .help("Sort by file extension"),
        )
        .arg(
            Arg::with_name("versionsort")
                .short("v")
                .long("versionsort")
                .multiple(true)
                .overrides_with("timesort")
                .overrides_with("sizesort")
                .overrides_with("extensionsort")
                .overrides_with("sort")
                .overrides_with("no-sort")
                .help("Natural sort of (version) numbers within text"),
        )
        .arg(
            Arg::with_name("sort")
                .long("sort")
                .multiple(true)
                .possible_values(&["size", "time", "version", "extension", "none"])
                .takes_value(true)
                .value_name("WORD")
                .overrides_with("timesort")
                .overrides_with("sizesort")
                .overrides_with("extensionsort")
                .overrides_with("versionsort")
                .overrides_with("no-sort")
                .help("sort by WORD instead of name")
        )
        .arg(
            Arg::with_name("no-sort")
            .short("U")
            .long("no-sort")
            .multiple(true)
            .overrides_with("timesort")
            .overrides_with("sizesort")
            .overrides_with("extensionsort")
            .overrides_with("sort")
            .overrides_with("versionsort")
            .help("Do not sort. List entries in directory order")
        )
        .arg(
            Arg::with_name("reverse")
                .short("r")
                .long("reverse")
                .multiple(true)
                .help("Reverse the order of the sort"),
        )
        .arg(
            Arg::with_name("group-dirs")
                .long("group-dirs")
                .possible_value("none")
                .possible_value("first")
                .possible_value("last")
                .multiple(true)
                .number_of_values(1)
                .help("Sort the directories then the files"),
        )
        .arg(
            Arg::with_name("group-directories-first")
                .long("group-directories-first")
                .help("Groups the directories at the top before the files. Same as --group-dirs=first")
        )
        .arg(
            Arg::with_name("blocks")
                .long("blocks")
                .multiple(true)
                .number_of_values(1)
                .require_delimiter(true)
                .possible_values(&[
                    "permission",
                    "user",
                    "group",
                    "context",
                    "size",
                    "date",
                    "name",
                    "inode",
                    "links",
                ])
                .help("Specify the blocks that will be displayed and in what order"),
        )
        .arg(
            Arg::with_name("classic")
                .long("classic")
                .help("Enable classic mode (display output similar to ls)"),
        )
        .arg(
            Arg::with_name("no-symlink")
                .long("no-symlink")
                .multiple(true)
                .help("Do not display symlink target"),
        )
        .arg(
            Arg::with_name("ignore-glob")
                .short("I")
                .long("ignore-glob")
                .multiple(true)
                .number_of_values(1)
                .value_name("pattern")
                .default_value("")
                .help("Do not display files/directories with names matching the glob pattern(s). More than one can be specified by repeating the argument"),
        )
        .arg(
            Arg::with_name("inode")
                .short("i")
                .long("inode")
                .multiple(true)
                .help("Display the index number of each file"),
        )
        .arg(
            Arg::with_name("dereference")
                .short("L")
                .long("dereference")
                .multiple(true)
                .help("When showing file information for a symbolic link, show information for the file the link references rather than for the link itself"),
        )
        .arg(
            Arg::with_name("context")
                .short("Z")
                .long("context")
                .required(false)
                .takes_value(false)
                .help("Print security context (label) of each file"),
        )
        .arg(
            Arg::with_name("hyperlink")
                .long("hyperlink")
                .possible_value("always")
                .possible_value("auto")
                .possible_value("never")
                .default_value("never")
                .multiple(true)
                .number_of_values(1)
                .help("Attach hyperlink to filenames"),
        )
}

fn validate_date_argument(arg: String) -> Result<(), String> {
    if arg.starts_with('+') {
        validate_time_format(&arg)
    } else if &arg == "date" || &arg == "relative" {
        Result::Ok(())
    } else {
        Result::Err("possible values: date, relative, +date-time-format".to_owned())
    }
}

pub fn validate_time_format(formatter: &str) -> Result<(), String> {
    let mut chars = formatter.chars();
    loop {
        match chars.next() {
            Some('%') => match chars.next() {
                Some('.') => match chars.next() {
                    Some('f') => (),
                    Some(n @ '3') | Some(n @ '6') | Some(n @ '9') => match chars.next() {
                        Some('f') => (),
                        Some(c) => return Err(format!("invalid format specifier: %.{}{}", n, c)),
                        None => return Err("missing format specifier".to_owned()),
                    },
                    Some(c) => return Err(format!("invalid format specifier: %.{}", c)),
                    None => return Err("missing format specifier".to_owned()),
                },
                Some(n @ ':') | Some(n @ '#') => match chars.next() {
                    Some('z') => (),
                    Some(c) => return Err(format!("invalid format specifier: %{}{}", n, c)),
                    None => return Err("missing format specifier".to_owned()),
                },
                Some(n @ '-') | Some(n @ '_') | Some(n @ '0') => match chars.next() {
                    Some('C') | Some('d') | Some('e') | Some('f') | Some('G') | Some('g')
                    | Some('H') | Some('I') | Some('j') | Some('k') | Some('l') | Some('M')
                    | Some('m') | Some('S') | Some('s') | Some('U') | Some('u') | Some('V')
                    | Some('W') | Some('w') | Some('Y') | Some('y') => (),
                    Some(c) => return Err(format!("invalid format specifier: %{}{}", n, c)),
                    None => return Err("missing format specifier".to_owned()),
                },
                Some('A') | Some('a') | Some('B') | Some('b') | Some('C') | Some('c')
                | Some('D') | Some('d') | Some('e') | Some('F') | Some('f') | Some('G')
                | Some('g') | Some('H') | Some('h') | Some('I') | Some('j') | Some('k')
                | Some('l') | Some('M') | Some('m') | Some('n') | Some('P') | Some('p')
                | Some('R') | Some('r') | Some('S') | Some('s') | Some('T') | Some('t')
                | Some('U') | Some('u') | Some('V') | Some('v') | Some('W') | Some('w')
                | Some('X') | Some('x') | Some('Y') | Some('y') | Some('Z') | Some('z')
                | Some('+') | Some('%') => (),
                Some(n @ '3') | Some(n @ '6') | Some(n @ '9') => match chars.next() {
                    Some('f') => (),
                    Some(c) => return Err(format!("invalid format specifier: %{}{}", n, c)),
                    None => return Err("missing format specifier".to_owned()),
                },
                Some(c) => return Err(format!("invalid format specifier: %{}", c)),
                None => return Err("missing format specifier".to_owned()),
            },
            None => break,
            _ => continue,
        }
    }
    Ok(())
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5104() {
    rusty_monitor::set_test_id(5104);
    let mut bool_0: bool = true;
    let mut usize_0: usize = 62usize;
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut bool_5: bool = false;
    let mut option_0: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut option_1: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_6: bool = false;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_6);
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_1: usize = 57usize;
    let mut bool_7: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_7, depth: usize_1};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut option_3: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_4: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_8: bool = true;
    let mut option_5: std::option::Option<bool> = std::option::Option::Some(bool_8);
    let mut bool_9: bool = false;
    let mut option_6: std::option::Option<bool> = std::option::Option::Some(bool_9);
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut option_7: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::Some(dirgrouping_0);
    let mut bool_10: bool = true;
    let mut option_8: std::option::Option<bool> = std::option::Option::Some(bool_10);
    let mut option_9: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_9, reverse: option_8, dir_grouping: option_7};
    let mut option_10: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_0);
    let mut option_11: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut option_12: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_1);
    let mut usize_2: usize = 33usize;
    let mut option_13: std::option::Option<usize> = std::option::Option::Some(usize_2);
    let mut bool_11: bool = false;
    let mut option_14: std::option::Option<bool> = std::option::Option::Some(bool_11);
    let mut recursion_1: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_14, depth: option_13};
    let mut option_15: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_1);
    let mut option_16: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut option_17: std::option::Option<bool> = std::option::Option::None;
    let mut option_18: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_19: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut display_0: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut option_20: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut option_21: std::option::Option<bool> = std::option::Option::None;
    let mut option_22: std::option::Option<std::string::String> = std::option::Option::None;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut option_23: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_0);
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut option_24: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_24, theme: option_23};
    let mut option_25: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_26: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_12: bool = false;
    let mut option_27: std::option::Option<bool> = std::option::Option::Some(bool_12);
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_27, blocks: option_26, color: option_25, date: option_22, dereference: option_21, display: option_20, icons: option_19, ignore_globs: option_18, indicators: option_17, layout: option_16, recursion: option_15, size: option_12, permission: option_11, sorting: option_10, no_symlink: option_6, total_size: option_5, symlink_arrow: option_4, hyperlink: option_3};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut display_1: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_1};
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut u64_0: u64 = 5u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut bool_13: bool = false;
    let mut bool_14: bool = true;
    let mut bool_15: bool = true;
    let mut bool_16: bool = true;
    let mut bool_17: bool = false;
    let mut bool_18: bool = false;
    let mut bool_19: bool = false;
    let mut bool_20: bool = false;
    let mut bool_21: bool = false;
    let mut bool_22: bool = false;
    let mut bool_23: bool = false;
    let mut bool_24: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_24, user_write: bool_23, user_execute: bool_22, group_read: bool_21, group_write: bool_20, group_execute: bool_19, other_read: bool_18, other_write: bool_17, other_execute: bool_16, sticky: bool_15, setgid: bool_14, setuid: bool_13};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_28: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    panic!("From RustyUnit with love");
}
}