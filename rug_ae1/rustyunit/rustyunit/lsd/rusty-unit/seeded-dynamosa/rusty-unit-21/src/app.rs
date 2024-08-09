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
#[timeout(30000)]fn rusty_test_8780() {
//    rusty_monitor::set_test_id(8780);
    let mut elem_0: color::Elem = crate::color::Elem::NonFile;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_1: color::Elem = crate::color::Elem::Read;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut bool_0: bool = false;
    let mut elem_2: color::Elem = crate::color::Elem::Links {valid: bool_0};
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_3: color::Elem = crate::color::Elem::Read;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut elem_4: color::Elem = crate::color::Elem::DayOld;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut bool_1: bool = false;
    let mut elem_5: color::Elem = crate::color::Elem::INode {valid: bool_1};
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut elem_6: color::Elem = crate::color::Elem::Read;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut elem_7: color::Elem = crate::color::Elem::Octal;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut elem_8: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut elem_9: color::Elem = crate::color::Elem::NonFile;
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    let mut elem_10: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_10_ref_0: &color::Elem = &mut elem_10;
    let mut elem_11: color::Elem = crate::color::Elem::Socket;
    let mut elem_11_ref_0: &color::Elem = &mut elem_11;
    let mut elem_12: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_12_ref_0: &color::Elem = &mut elem_12;
    let mut bool_2: bool = false;
    let mut elem_13: color::Elem = crate::color::Elem::Links {valid: bool_2};
    let mut elem_13_ref_0: &color::Elem = &mut elem_13;
    let mut elem_14: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_14_ref_0: &color::Elem = &mut elem_14;
    let mut elem_15: color::Elem = crate::color::Elem::Read;
    let mut elem_15_ref_0: &color::Elem = &mut elem_15;
    let mut elem_16: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_16_ref_0: &color::Elem = &mut elem_16;
    let mut elem_17: color::Elem = crate::color::Elem::Older;
    let mut elem_17_ref_0: &color::Elem = &mut elem_17;
    let mut elem_18: color::Elem = crate::color::Elem::SymLink;
    let mut elem_18_ref_0: &color::Elem = &mut elem_18;
    let mut elem_19: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_19_ref_0: &color::Elem = &mut elem_19;
    let mut elem_20: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_20_ref_0: &color::Elem = &mut elem_20;
    let mut elem_21: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_21_ref_0: &color::Elem = &mut elem_21;
    let mut elem_22: color::Elem = crate::color::Elem::User;
    let mut elem_22_ref_0: &color::Elem = &mut elem_22;
    let mut bool_3: bool = false;
    let mut elem_23: color::Elem = crate::color::Elem::Dir {uid: bool_3};
    let mut elem_23_ref_0: &color::Elem = &mut elem_23;
    let mut elem_24: color::Elem = crate::color::Elem::Write;
    let mut elem_24_ref_0: &color::Elem = &mut elem_24;
    let mut elem_25: color::Elem = crate::color::Elem::Context;
    let mut elem_25_ref_0: &color::Elem = &mut elem_25;
    let mut elem_26: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_26_ref_0: &color::Elem = &mut elem_26;
    let mut elem_27: color::Elem = crate::color::Elem::Context;
    let mut elem_27_ref_0: &color::Elem = &mut elem_27;
    let mut elem_28: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_28_ref_0: &color::Elem = &mut elem_28;
    let mut elem_29: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_29_ref_0: &color::Elem = &mut elem_29;
    let mut elem_30: color::Elem = crate::color::Elem::Special;
    let mut elem_30_ref_0: &color::Elem = &mut elem_30;
    let mut elem_31: color::Elem = crate::color::Elem::ExecSticky;
    let mut elem_31_ref_0: &color::Elem = &mut elem_31;
    let mut elem_32: color::Elem = crate::color::Elem::SymLink;
    let mut elem_32_ref_0: &color::Elem = &mut elem_32;
    let mut bool_4: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_4};
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    crate::meta::filetype::FileType::render(filetype_1, colors_0_ref_0);
//    panic!("From RustyUnit with love");
}
}