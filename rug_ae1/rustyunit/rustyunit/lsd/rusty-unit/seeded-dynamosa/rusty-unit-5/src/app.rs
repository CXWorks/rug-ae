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

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1453() {
//    rusty_monitor::set_test_id(1453);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Group;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut bool_0: bool = false;
    let mut u64_0: u64 = 0u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut elem_1: color::Elem = crate::color::Elem::User;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut u64_1: u64 = 0u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_0};
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Links;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut block_0_ref_0: &flags::blocks::Block = &mut block_0;
//    panic!("From RustyUnit with love");
}
}