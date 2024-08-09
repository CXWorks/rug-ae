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
fn rusty_test_7449() {
    rusty_monitor::set_test_id(7449);
    let mut elem_0: color::Elem = crate::color::Elem::DayOld;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut display_0: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut elem_1: color::Elem = crate::color::Elem::ExecSticky;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut str_0: &str = "pz0vV3SCJlVzj";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "yLadLiI4rK";
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut elem_2: color::Elem = crate::color::Elem::File {exec: bool_1, uid: bool_0};
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_3: color::Elem = crate::color::Elem::HourOld;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut bool_2: bool = true;
    let mut elem_4: color::Elem = crate::color::Elem::Dir {uid: bool_2};
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_5: color::Elem = crate::color::Elem::Special;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_6: color::Elem = crate::color::Elem::DayOld;
    let mut str_1_ref_0: &str = &mut str_1;
    let mut option_0: std::option::Option<usize> = std::option::Option::None;
    let mut option_1: std::option::Option<bool> = std::option::Option::None;
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_1, depth: option_0};
    let mut recursion_0_ref_0: &crate::config_file::Recursion = &mut recursion_0;
    let mut usize_0: usize = 24usize;
    let mut option_2: std::option::Option<usize> = std::option::Option::Some(usize_0);
    let mut option_3: std::option::Option<bool> = std::option::Option::None;
    let mut recursion_1: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_3, depth: option_2};
    let mut recursion_1_ref_0: &crate::config_file::Recursion = &mut recursion_1;
    let mut str_2: &str = "";
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_4_ref_0);
    let mut str_2_ref_0: &str = &mut str_2;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut elem_7: color::Elem = crate::color::Elem::Pipe;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut dir_0: crate::color::theme::Dir = crate::color::theme::Dir {uid: color_0, no_uid: color_1};
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_6_ref_0, theme_0_ref_0);
    crate::meta::filetype::FileType::render(filetype_0, colors_0_ref_0);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut display_1: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut display_0_ref_0: &flags::display::Display = &mut display_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut elem_8: color::Elem = crate::color::Elem::Octal;
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut coloroption_0_ref_0: &flags::color::ColorOption = &mut coloroption_0;
    let mut bool_3: bool = crate::meta::filetype::FileType::is_dirlike(filetype_1);
    panic!("From RustyUnit with love");
}
}