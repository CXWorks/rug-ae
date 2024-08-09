use crate::color::{Colors, Elem};
use crate::flags::{Block, Display, Flags, HyperlinkOption, Layout};
use crate::icon::Icons;
use crate::meta::name::DisplayOption;
use crate::meta::{FileType, Meta};
use std::collections::HashMap;
use term_grid::{Cell, Direction, Filling, Grid, GridOptions};
use terminal_size::terminal_size;
use unicode_width::UnicodeWidthStr;

const EDGE: &str = "\u{251c}\u{2500}\u{2500}"; // "├──"
const LINE: &str = "\u{2502}  "; // "│  "
const CORNER: &str = "\u{2514}\u{2500}\u{2500}"; // "└──"
const BLANK: &str = "   ";

pub fn grid(metas: &[Meta], flags: &Flags, colors: &Colors, icons: &Icons) -> String {
    let term_width = terminal_size().map(|(w, _)| w.0 as usize);

    inner_display_grid(
        &DisplayOption::None,
        metas,
        flags,
        colors,
        icons,
        0,
        term_width,
    )
}

pub fn tree(metas: &[Meta], flags: &Flags, colors: &Colors, icons: &Icons) -> String {
    let mut grid = Grid::new(GridOptions {
        filling: Filling::Spaces(1),
        direction: Direction::LeftToRight,
    });

    let padding_rules = get_padding_rules(metas, flags);
    let mut index = 0;
    for (i, block) in flags.blocks.0.iter().enumerate() {
        if let Block::Name = block {
            index = i;
            break;
        }
    }

    for cell in inner_display_tree(metas, flags, colors, icons, (0, ""), &padding_rules, index) {
        grid.add(cell);
    }

    grid.fit_into_columns(flags.blocks.0.len()).to_string()
}

fn inner_display_grid(
    display_option: &DisplayOption,
    metas: &[Meta],
    flags: &Flags,
    colors: &Colors,
    icons: &Icons,
    depth: usize,
    term_width: Option<usize>,
) -> String {
    let mut output = String::new();

    let padding_rules = get_padding_rules(metas, flags);
    let mut grid = match flags.layout {
        Layout::OneLine => Grid::new(GridOptions {
            filling: Filling::Spaces(1),
            direction: Direction::LeftToRight,
        }),
        _ => Grid::new(GridOptions {
            filling: Filling::Spaces(2),
            direction: Direction::TopToBottom,
        }),
    };

    // The first iteration (depth == 0) corresponds to the inputs given by the
    // user. We defer displaying directories given by the user unless we've been
    // asked to display the directory itself (rather than its contents).
    let skip_dirs = (depth == 0) && (flags.display != Display::DirectoryOnly);

    // print the files first.
    for meta in metas {
        // Maybe skip showing the directory meta now; show its contents later.
        if skip_dirs
            && (matches!(meta.file_type, FileType::Directory { .. })
                || (matches!(meta.file_type, FileType::SymLink { is_dir: true })
                    && flags.layout != Layout::OneLine))
        {
            continue;
        }

        let blocks = get_output(
            meta,
            colors,
            icons,
            flags,
            display_option,
            &padding_rules,
            (0, ""),
        );

        for block in blocks {
            let block_str = block.to_string();

            grid.add(Cell {
                width: get_visible_width(
                    &block_str,
                    matches!(flags.hyperlink, HyperlinkOption::Always),
                ),
                contents: block_str,
            });
        }
    }

    if flags.layout == Layout::Grid {
        if let Some(tw) = term_width {
            if let Some(gridded_output) = grid.fit_into_width(tw) {
                output += &gridded_output.to_string();
            } else {
                //does not fit into grid, usually because (some) filename(s)
                //are longer or almost as long as term_width
                //print line by line instead!
                output += &grid.fit_into_columns(1).to_string();
            }
        } else {
            output += &grid.fit_into_columns(1).to_string();
        }
    } else {
        output += &grid.fit_into_columns(flags.blocks.0.len()).to_string();
    }

    let should_display_folder_path = should_display_folder_path(depth, metas, flags);

    // print the folder content
    for meta in metas {
        if meta.content.is_some() {
            if should_display_folder_path {
                output += &display_folder_path(meta);
            }

            let display_option = DisplayOption::Relative {
                base_path: &meta.path,
            };

            output += &inner_display_grid(
                &display_option,
                meta.content.as_ref().unwrap(),
                flags,
                colors,
                icons,
                depth + 1,
                term_width,
            );
        }
    }

    output
}

fn inner_display_tree(
    metas: &[Meta],
    flags: &Flags,
    colors: &Colors,
    icons: &Icons,
    tree_depth_prefix: (usize, &str),
    padding_rules: &HashMap<Block, usize>,
    tree_index: usize,
) -> Vec<Cell> {
    let mut cells = Vec::new();
    let last_idx = metas.len();

    for (idx, meta) in metas.iter().enumerate() {
        let current_prefix = if tree_depth_prefix.0 > 0 {
            if idx + 1 != last_idx {
                // is last folder elem
                format!("{}{} ", tree_depth_prefix.1, EDGE)
            } else {
                format!("{}{} ", tree_depth_prefix.1, CORNER)
            }
        } else {
            tree_depth_prefix.1.to_string()
        };

        for block in get_output(
            meta,
            colors,
            icons,
            flags,
            &DisplayOption::FileName,
            padding_rules,
            (tree_index, &current_prefix),
        ) {
            let block_str = block.to_string();

            cells.push(Cell {
                width: get_visible_width(
                    &block_str,
                    matches!(flags.hyperlink, HyperlinkOption::Always),
                ),
                contents: block_str,
            });
        }

        if meta.content.is_some() {
            let new_prefix = if tree_depth_prefix.0 > 0 {
                if idx + 1 != last_idx {
                    // is last folder elem
                    format!("{}{} ", tree_depth_prefix.1, LINE)
                } else {
                    format!("{}{} ", tree_depth_prefix.1, BLANK)
                }
            } else {
                tree_depth_prefix.1.to_string()
            };

            cells.extend(inner_display_tree(
                meta.content.as_ref().unwrap(),
                flags,
                colors,
                icons,
                (tree_depth_prefix.0 + 1, &new_prefix),
                padding_rules,
                tree_index,
            ));
        }
    }

    cells
}

fn should_display_folder_path(depth: usize, metas: &[Meta], flags: &Flags) -> bool {
    if depth > 0 {
        true
    } else {
        let folder_number = metas
            .iter()
            .filter(|x| {
                matches!(x.file_type, FileType::Directory { .. })
                    || (matches!(x.file_type, FileType::SymLink { is_dir: true })
                        && flags.layout != Layout::OneLine)
            })
            .count();

        folder_number > 1 || folder_number < metas.len()
    }
}

fn display_folder_path(meta: &Meta) -> String {
    let mut output = String::new();
    output.push('\n');
    output += &meta.path.to_string_lossy();
    output += ":\n";

    output
}

fn get_output<'a>(
    meta: &'a Meta,
    colors: &'a Colors,
    icons: &'a Icons,
    flags: &'a Flags,
    display_option: &DisplayOption,
    padding_rules: &HashMap<Block, usize>,
    tree: (usize, &'a str),
) -> Vec<String> {
    let mut strings: Vec<String> = Vec::new();
    for (i, block) in flags.blocks.0.iter().enumerate() {
        let mut block_vec = if Layout::Tree == flags.layout && tree.0 == i {
            vec![colors.colorize(tree.1.to_string(), &Elem::TreeEdge)]
        } else {
            Vec::new()
        };

        match block {
            Block::INode => block_vec.push(meta.inode.render(colors)),
            Block::Links => block_vec.push(meta.links.render(colors)),
            Block::Permission => {
                block_vec.extend(vec![
                    meta.file_type.render(colors),
                    meta.permissions.render(colors, flags),
                    meta.access_control.render_method(colors),
                ]);
            }
            Block::User => block_vec.push(meta.owner.render_user(colors)),
            Block::Group => block_vec.push(meta.owner.render_group(colors)),
            Block::Context => block_vec.push(meta.access_control.render_context(colors)),
            Block::Size => {
                let pad = if Layout::Tree == flags.layout && 0 == tree.0 && 0 == i {
                    None
                } else {
                    Some(padding_rules[&Block::SizeValue])
                };
                block_vec.push(meta.size.render(colors, flags, pad))
            }
            Block::SizeValue => block_vec.push(meta.size.render_value(colors, flags)),
            Block::Date => block_vec.push(meta.date.render(colors, flags)),
            Block::Name => {
                block_vec.extend(vec![
                    meta.name
                        .render(colors, icons, display_option, flags.hyperlink),
                    meta.indicator.render(flags),
                ]);
                if !(flags.no_symlink.0 || flags.dereference.0 || flags.layout == Layout::Grid) {
                    block_vec.push(meta.symlink.render(colors, flags))
                }
            }
        };
        strings.push(
            block_vec
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join(""),
        );
    }
    strings
}

fn get_visible_width(input: &str, hyperlink: bool) -> usize {
    let mut nb_invisible_char = 0;

    // If the input has color, do not compute the length contributed by the color to the actual length
    for (idx, _) in input.match_indices("\u{1b}[") {
        let (_, s) = input.split_at(idx);

        let m_pos = s.find('m');
        if let Some(len) = m_pos {
            nb_invisible_char += len
        }
    }

    if hyperlink {
        for (idx, _) in input.match_indices("\x1B]8;;") {
            let (_, s) = input.split_at(idx);

            let m_pos = s.find("\x1B\x5C");
            if let Some(len) = m_pos {
                nb_invisible_char += len
            }
        }
    }

    UnicodeWidthStr::width(input) - nb_invisible_char
}

fn detect_size_lengths(metas: &[Meta], flags: &Flags) -> usize {
    let mut max_value_length: usize = 0;

    for meta in metas {
        let value_len = meta.size.value_string(flags).len();

        if value_len > max_value_length {
            max_value_length = value_len;
        }

        if Layout::Tree == flags.layout {
            if let Some(subs) = &meta.content {
                let sub_length = detect_size_lengths(subs, flags);
                if sub_length > max_value_length {
                    max_value_length = sub_length;
                }
            }
        }
    }

    max_value_length
}

fn get_padding_rules(metas: &[Meta], flags: &Flags) -> HashMap<Block, usize> {
    let mut padding_rules: HashMap<Block, usize> = HashMap::new();

    if flags.blocks.0.contains(&Block::Size) {
        let size_val = detect_size_lengths(metas, flags);

        padding_rules.insert(Block::SizeValue, size_val);
    }

    padding_rules
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color;
    use crate::color::Colors;
    use crate::flags::HyperlinkOption;
    use crate::icon::Icons;
    use crate::meta::{FileType, Name};
    use crate::Config;
    use crate::{app, flags, icon, sort};
    use assert_fs::prelude::*;
    use std::path::Path;

    #[test]
    fn test_display_get_visible_width_without_icons() {
        for (s, l) in &[
            ("Ｈｅｌｌｏ,ｗｏｒｌｄ!", 22),
            ("ASCII1234-_", 11),
            ("制作样本。", 10),
            ("日本語", 6),
            ("샘플은 무료로 드리겠습니다", 26),
            ("👩🐩", 4),
            ("🔬", 2),
        ] {
            let path = Path::new(s);
            let name = Name::new(
                &path,
                FileType::File {
                    exec: false,
                    uid: false,
                },
            );
            let output = name
                .render(
                    &Colors::new(color::ThemeOption::NoColor),
                    &Icons::new(icon::Theme::NoIcon, " ".to_string()),
                    &DisplayOption::FileName,
                    HyperlinkOption::Never,
                )
                .to_string();

            assert_eq!(get_visible_width(&output, false), *l);
        }
    }

    #[test]
    fn test_display_get_visible_width_with_icons() {
        for (s, l) in &[
            // Add 3 characters for the icons.
            ("Ｈｅｌｌｏ,ｗｏｒｌｄ!", 24),
            ("ASCII1234-_", 13),
            ("File with space", 17),
            ("制作样本。", 12),
            ("日本語", 8),
            ("샘플은 무료로 드리겠습니다", 28),
            ("👩🐩", 6),
            ("🔬", 4),
        ] {
            let path = Path::new(s);
            let name = Name::new(
                &path,
                FileType::File {
                    exec: false,
                    uid: false,
                },
            );
            let output = name
                .render(
                    &Colors::new(color::ThemeOption::NoColor),
                    &Icons::new(icon::Theme::Fancy, " ".to_string()),
                    &DisplayOption::FileName,
                    HyperlinkOption::Never,
                )
                .to_string();

            assert_eq!(get_visible_width(&output, false), *l);
        }
    }

    #[test]
    fn test_display_get_visible_width_with_colors() {
        for (s, l) in &[
            ("Ｈｅｌｌｏ,ｗｏｒｌｄ!", 22),
            ("ASCII1234-_", 11),
            ("File with space", 15),
            ("制作样本。", 10),
            ("日本語", 6),
            ("샘플은 무료로 드리겠습니다", 26),
            ("👩🐩", 4),
            ("🔬", 2),
        ] {
            let path = Path::new(s);
            let name = Name::new(
                &path,
                FileType::File {
                    exec: false,
                    uid: false,
                },
            );
            let output = name
                .render(
                    &Colors::new(color::ThemeOption::NoLscolors),
                    &Icons::new(icon::Theme::NoIcon, " ".to_string()),
                    &DisplayOption::FileName,
                    HyperlinkOption::Never,
                )
                .to_string();

            // check if the color is present.
            assert_eq!(
                true,
                output.starts_with("\u{1b}[38;5;"),
                "{:?} should start with color",
                output,
            );
            assert_eq!(true, output.ends_with("[39m"), "reset foreground color");

            assert_eq!(get_visible_width(&output, false), *l, "visible match");
        }
    }

    #[test]
    fn test_display_get_visible_width_without_colors() {
        for (s, l) in &[
            ("Ｈｅｌｌｏ,ｗｏｒｌｄ!", 22),
            ("ASCII1234-_", 11),
            ("File with space", 15),
            ("制作样本。", 10),
            ("日本語", 6),
            ("샘플은 무료로 드리겠습니다", 26),
            ("👩🐩", 4),
            ("🔬", 2),
        ] {
            let path = Path::new(s);
            let name = Name::new(
                &path,
                FileType::File {
                    exec: false,
                    uid: false,
                },
            );
            let output = name
                .render(
                    &Colors::new(color::ThemeOption::NoColor),
                    &Icons::new(icon::Theme::NoIcon, " ".to_string()),
                    &DisplayOption::FileName,
                    HyperlinkOption::Never,
                )
                .to_string();

            // check if the color is present.
            assert_eq!(false, output.starts_with("\u{1b}[38;5;"));
            assert_eq!(false, output.ends_with("[0m"));

            assert_eq!(get_visible_width(&output, false), *l);
        }
    }

    #[test]
    fn test_display_get_visible_width_hypelink_simple() {
        for (s, l) in &[
            ("Ｈｅｌｌｏ,ｗｏｒｌｄ!", 22),
            ("ASCII1234-_", 11),
            ("File with space", 15),
            ("制作样本。", 10),
            ("日本語", 6),
            ("샘플은 무료로 드리겠습니다", 26),
            ("👩🐩", 4),
            ("🔬", 2),
        ] {
            // rending name require actual file, so we are mocking that
            let output = format!("\x1B]8;;{}\x1B\x5C{}\x1B]8;;\x1B\x5C", "url://fake-url", s);
            assert_eq!(get_visible_width(&output, true), *l);
        }
    }

    fn sort(metas: &mut Vec<Meta>, sorters: &Vec<(flags::SortOrder, sort::SortFn)>) {
        metas.sort_unstable_by(|a, b| sort::by_meta(sorters, a, b));

        for meta in metas {
            if let Some(ref mut content) = meta.content {
                sort(content, sorters);
            }
        }
    }

    #[test]
    fn test_display_tree_with_all() {
        let argv = vec!["lsd", "--tree", "--all"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let flags = Flags::configure_from(&matches, &Config::with_none()).unwrap();

        let dir = assert_fs::TempDir::new().unwrap();
        dir.child("one.d").create_dir_all().unwrap();
        dir.child("one.d/two").touch().unwrap();
        dir.child("one.d/.hidden").touch().unwrap();
        let mut metas = Meta::from_path(Path::new(dir.path()), false)
            .unwrap()
            .recurse_into(42, &flags)
            .unwrap()
            .unwrap();
        sort(&mut metas, &sort::assemble_sorters(&flags));
        let output = tree(
            &metas,
            &flags,
            &Colors::new(color::ThemeOption::NoColor),
            &Icons::new(icon::Theme::NoIcon, " ".to_string()),
        );

        assert_eq!("one.d\n├── .hidden\n└── two\n", output);
    }

    /// Different level of folder may form a different width
    /// we must make sure it is aligned in all level
    ///
    /// dir has a bytes size
    /// empty file has an empty size
    /// `---blocks size,name` can help us for this case
    #[test]
    fn test_tree_align_subfolder() {
        let argv = vec!["lsd", "--tree", "--blocks", "size,name"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let flags = Flags::configure_from(&matches, &Config::with_none()).unwrap();

        let dir = assert_fs::TempDir::new().unwrap();
        dir.child("dir").create_dir_all().unwrap();
        dir.child("dir/file").touch().unwrap();
        let metas = Meta::from_path(Path::new(dir.path()), false)
            .unwrap()
            .recurse_into(42, &flags)
            .unwrap()
            .unwrap();
        let output = tree(
            &metas,
            &flags,
            &Colors::new(color::ThemeOption::NoColor),
            &Icons::new(icon::Theme::NoIcon, " ".to_string()),
        );

        let length_before_b = |i| -> usize {
            output
                .lines()
                .nth(i)
                .unwrap()
                .split(|c| c == 'K' || c == 'B')
                .nth(0)
                .unwrap()
                .len()
        };
        assert_eq!(length_before_b(0), length_before_b(1));
        assert_eq!(
            output.lines().nth(0).unwrap().find("d"),
            output.lines().nth(1).unwrap().find("└")
        );
    }

    #[test]
    #[cfg(unix)]
    fn test_tree_size_first_without_name() {
        let argv = vec!["lsd", "--tree", "--blocks", "size,permission"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let flags = Flags::configure_from(&matches, &Config::with_none()).unwrap();

        let dir = assert_fs::TempDir::new().unwrap();
        dir.child("dir").create_dir_all().unwrap();
        dir.child("dir/file").touch().unwrap();
        let metas = Meta::from_path(Path::new(dir.path()), false)
            .unwrap()
            .recurse_into(42, &flags)
            .unwrap()
            .unwrap();
        let output = tree(
            &metas,
            &flags,
            &Colors::new(color::ThemeOption::NoColor),
            &Icons::new(icon::Theme::NoIcon, " ".to_string()),
        );

        assert_eq!(output.lines().nth(1).unwrap().chars().nth(0).unwrap(), '└');
        assert_eq!(
            output
                .lines()
                .nth(0)
                .unwrap()
                .chars()
                .position(|x| x == 'd'),
            output
                .lines()
                .nth(1)
                .unwrap()
                .chars()
                .position(|x| x == '.'),
        );
    }

    #[test]
    fn test_tree_edge_before_name() {
        let argv = vec!["lsd", "--tree", "--long"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let flags = Flags::configure_from(&matches, &Config::with_none()).unwrap();

        let dir = assert_fs::TempDir::new().unwrap();
        dir.child("one.d").create_dir_all().unwrap();
        dir.child("one.d/two").touch().unwrap();
        let metas = Meta::from_path(Path::new(dir.path()), false)
            .unwrap()
            .recurse_into(42, &flags)
            .unwrap()
            .unwrap();
        let output = tree(
            &metas,
            &flags,
            &Colors::new(color::ThemeOption::NoColor),
            &Icons::new(icon::Theme::NoIcon, " ".to_string()),
        );

        assert!(output.ends_with("└── two\n"));
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1158() {
    rusty_monitor::set_test_id(1158);
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_0: usize = 42usize;
    let mut bool_0: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut usize_1: usize = 74usize;
    let mut option_0: std::option::Option<usize> = std::option::Option::Some(usize_1);
    let mut usize_2: usize = 51usize;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut displayoption_0: meta::name::DisplayOption = crate::meta::name::DisplayOption::FileName;
    let mut displayoption_0_ref_0: &meta::name::DisplayOption = &mut displayoption_0;
    let mut option_1: std::option::Option<std::vec::Vec<crate::meta::Meta>> = std::option::Option::None;
    let mut u64_0: u64 = 17u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = false;
    let mut bool_5: bool = false;
    let mut bool_6: bool = false;
    let mut bool_7: bool = false;
    let mut bool_8: bool = false;
    let mut bool_9: bool = false;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut bool_12: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_12, user_write: bool_11, user_execute: bool_10, group_read: bool_9, group_write: bool_8, group_execute: bool_7, other_read: bool_6, other_write: bool_5, other_execute: bool_4, sticky: bool_3, setgid: bool_2, setuid: bool_1};
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut str_0: &str = "94c5htjW6";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut bool_13: bool = true;
    let mut bool_14: bool = false;
    let mut bool_15: bool = false;
    let mut bool_16: bool = false;
    let mut bool_17: bool = true;
    let mut bool_18: bool = false;
    let mut bool_19: bool = false;
    let mut bool_20: bool = false;
    let mut bool_21: bool = false;
    let mut bool_22: bool = true;
    let mut bool_23: bool = true;
    let mut bool_24: bool = false;
    let mut permissions_1: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_24, user_write: bool_23, user_execute: bool_22, group_read: bool_21, group_write: bool_20, group_execute: bool_19, other_read: bool_18, other_write: bool_17, other_execute: bool_16, sticky: bool_15, setgid: bool_14, setuid: bool_13};
    let mut permissions_1_ref_0: &crate::meta::permissions::Permissions = &mut permissions_1;
    let mut option_2: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut u64_1: u64 = 31u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_1_ref_0: &meta::date::Date = &mut date_1;
    let mut bool_25: bool = true;
    let mut bool_26: bool = true;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_26, exec: bool_25};
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut u64_2: u64 = 36u64;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size::new(u64_2);
    let mut size_2_ref_0: &crate::meta::size::Size = &mut size_2;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_3};
    let mut bool_27: bool = false;
    let mut str_1: &str = "syAqx";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut usize_3: usize = crate::display::get_visible_width(str_1_ref_0, bool_27);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2088() {
    rusty_monitor::set_test_id(2088);
    let mut u64_0: u64 = 69u64;
    let mut str_0: &str = "M8";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = false;
    let mut bool_5: bool = false;
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = false;
    let mut bool_10: bool = true;
    let mut bool_11: bool = true;
    let mut usize_0: usize = 38usize;
    let mut usize_1: usize = 29usize;
    let mut usize_2: usize = 28usize;
    let mut bool_12: bool = true;
    let mut bool_13: bool = true;
    let mut bool_14: bool = false;
    let mut usize_3: usize = 75usize;
    let mut bool_15: bool = true;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut option_0: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_0);
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut option_1: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut option_2: std::option::Option<bool> = std::option::Option::None;
    let mut option_3: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_4: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_5: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_6: std::option::Option<bool> = std::option::Option::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_1: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut option_7: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_8: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_9: std::option::Option<bool> = std::option::Option::None;
    let mut bool_16: bool = false;
    let mut option_10: std::option::Option<bool> = std::option::Option::Some(bool_16);
    let mut option_11: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::None;
    let mut bool_17: bool = true;
    let mut option_12: std::option::Option<bool> = std::option::Option::Some(bool_17);
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut option_13: std::option::Option<flags::sorting::SortColumn> = std::option::Option::Some(sortcolumn_0);
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_13, reverse: option_12, dir_grouping: option_11};
    let mut option_14: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_0);
    let mut option_15: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut option_16: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_17: std::option::Option<usize> = std::option::Option::None;
    let mut option_18: std::option::Option<bool> = std::option::Option::None;
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_18, depth: option_17};
    let mut option_19: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_0);
    let mut option_20: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_18: bool = true;
    let mut option_21: std::option::Option<bool> = std::option::Option::Some(bool_18);
    let mut option_22: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_23: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut option_24: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_1);
    let mut iconoption_2: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut option_25: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_2);
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_25, theme: option_24, separator: option_23};
    let mut option_26: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut option_27: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut bool_19: bool = false;
    let mut option_28: std::option::Option<bool> = std::option::Option::Some(bool_19);
    let mut option_29: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_30: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut option_31: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_1);
    let mut color_1: crate::config_file::Color = crate::config_file::Color {when: option_31, theme: option_30};
    let mut option_32: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_1);
    let mut option_33: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_34: std::option::Option<bool> = std::option::Option::None;
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_34, blocks: option_33, color: option_32, date: option_29, dereference: option_28, display: option_27, icons: option_26, ignore_globs: option_22, indicators: option_21, layout: option_20, recursion: option_19, size: option_16, permission: option_15, sorting: option_14, no_symlink: option_10, total_size: option_9, symlink_arrow: option_8, hyperlink: option_7};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut displayoption_0: meta::name::DisplayOption = crate::meta::name::DisplayOption::FileName;
    let mut displayoption_0_ref_0: &meta::name::DisplayOption = &mut displayoption_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut bool_20: bool = false;
    let mut bool_21: bool = false;
    let mut bool_22: bool = false;
    let mut bool_23: bool = false;
    let mut bool_24: bool = false;
    let mut bool_25: bool = true;
    let mut bool_26: bool = false;
    let mut bool_27: bool = true;
    let mut bool_28: bool = false;
    let mut bool_29: bool = true;
    let mut bool_30: bool = true;
    let mut bool_31: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_31, user_write: bool_30, user_execute: bool_29, group_read: bool_28, group_write: bool_27, group_execute: bool_26, other_read: bool_25, other_write: bool_24, other_execute: bool_23, sticky: bool_22, setgid: bool_21, setuid: bool_20};
    let mut bool_32: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_32};
    let mut bool_33: bool = true;
    let mut str_1: &str = "Vrq";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut usize_4: usize = crate::display::get_visible_width(str_1_ref_0, bool_33);
    panic!("From RustyUnit with love");
}
}