use crate::flags::{DirGrouping, Flags, SortColumn, SortOrder};
use crate::meta::Meta;
use human_sort::compare;
use std::cmp::Ordering;

pub type SortFn = fn(&Meta, &Meta) -> Ordering;

pub fn assemble_sorters(flags: &Flags) -> Vec<(SortOrder, SortFn)> {
    let mut sorters: Vec<(SortOrder, SortFn)> = vec![];
    match flags.sorting.dir_grouping {
        DirGrouping::First => {
            sorters.push((SortOrder::Default, with_dirs_first));
        }
        DirGrouping::Last => {
            sorters.push((SortOrder::Reverse, with_dirs_first));
        }
        DirGrouping::None => {}
    };

    match flags.sorting.column {
        SortColumn::Name => sorters.push((flags.sorting.order, by_name)),
        SortColumn::Size => sorters.push((flags.sorting.order, by_size)),
        SortColumn::Time => sorters.push((flags.sorting.order, by_date)),
        SortColumn::Version => sorters.push((flags.sorting.order, by_version)),
        SortColumn::Extension => sorters.push((flags.sorting.order, by_extension)),
        SortColumn::None => {}
    }
    sorters
}

pub fn by_meta(sorters: &[(SortOrder, SortFn)], a: &Meta, b: &Meta) -> Ordering {
    for (direction, sorter) in sorters.iter() {
        match (sorter)(a, b) {
            Ordering::Equal => continue,
            ordering => {
                return match direction {
                    SortOrder::Reverse => ordering.reverse(),
                    SortOrder::Default => ordering,
                }
            }
        }
    }
    Ordering::Equal
}

fn with_dirs_first(a: &Meta, b: &Meta) -> Ordering {
    b.file_type.is_dirlike().cmp(&a.file_type.is_dirlike())
}

fn by_size(a: &Meta, b: &Meta) -> Ordering {
    b.size.get_bytes().cmp(&a.size.get_bytes())
}

fn by_name(a: &Meta, b: &Meta) -> Ordering {
    a.name.cmp(&b.name)
}

fn by_date(a: &Meta, b: &Meta) -> Ordering {
    b.date.cmp(&a.date).then(a.name.cmp(&b.name))
}

fn by_version(a: &Meta, b: &Meta) -> Ordering {
    compare(&a.name.name, &b.name.name)
}

fn by_extension(a: &Meta, b: &Meta) -> Ordering {
    a.name.extension().cmp(&b.name.extension())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flags::Flags;
    use std::fs::{create_dir, File};
    use std::process::Command;
    use tempfile::tempdir;

    #[test]
    fn test_sort_assemble_sorters_by_name_with_dirs_first() {
        let tmp_dir = tempdir().expect("failed to create temp dir");

        // Create the file;
        let path_a = tmp_dir.path().join("zzz");
        File::create(&path_a).expect("failed to create file");
        let meta_a = Meta::from_path(&path_a, false).expect("failed to get meta");

        // Create a dir;
        let path_z = tmp_dir.path().join("aaa");
        create_dir(&path_z).expect("failed to create dir");
        let meta_z = Meta::from_path(&path_z, false).expect("failed to get meta");

        let mut flags = Flags::default();
        flags.sorting.dir_grouping = DirGrouping::First;

        //  Sort with the dirs first
        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(&sorter, &meta_a, &meta_z), Ordering::Greater);

        //  Sort with the dirs first (the dirs stay first)
        flags.sorting.order = SortOrder::Reverse;

        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(&sorter, &meta_a, &meta_z), Ordering::Greater);
    }

    #[test]
    fn test_sort_assemble_sorters_by_name_with_files_first() {
        let tmp_dir = tempdir().expect("failed to create temp dir");

        // Create the file;
        let path_a = tmp_dir.path().join("zzz");
        File::create(&path_a).expect("failed to create file");
        let meta_a = Meta::from_path(&path_a, false).expect("failed to get meta");

        // Create a dir;
        let path_z = tmp_dir.path().join("aaa");
        create_dir(&path_z).expect("failed to create dir");
        let meta_z = Meta::from_path(&path_z, false).expect("failed to get meta");

        let mut flags = Flags::default();
        flags.sorting.dir_grouping = DirGrouping::Last;

        // Sort with file first
        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(&sorter, &meta_a, &meta_z), Ordering::Less);

        // Sort with file first reversed (thie files stay first)
        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(&sorter, &meta_a, &meta_z), Ordering::Less);
    }

    #[test]
    fn test_sort_assemble_sorters_by_name_unordered() {
        let tmp_dir = tempdir().expect("failed to create temp dir");

        // Create the file;
        let path_a = tmp_dir.path().join("aaa");
        File::create(&path_a).expect("failed to create file");
        let meta_a = Meta::from_path(&path_a, false).expect("failed to get meta");

        // Create a dir;
        let path_z = tmp_dir.path().join("zzz");
        create_dir(&path_z).expect("failed to create dir");
        let meta_z = Meta::from_path(&path_z, false).expect("failed to get meta");

        let mut flags = Flags::default();
        flags.sorting.dir_grouping = DirGrouping::None;

        // Sort by name unordered
        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(&sorter, &meta_a, &meta_z), Ordering::Less);

        // Sort by name unordered
        flags.sorting.order = SortOrder::Reverse;

        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(&sorter, &meta_a, &meta_z), Ordering::Greater);
    }

    #[test]
    fn test_sort_assemble_sorters_by_name_unordered_2() {
        let tmp_dir = tempdir().expect("failed to create temp dir");

        // Create the file;
        let path_a = tmp_dir.path().join("zzz");
        File::create(&path_a).expect("failed to create file");
        let meta_a = Meta::from_path(&path_a, false).expect("failed to get meta");

        // Create a dir;
        let path_z = tmp_dir.path().join("aaa");
        create_dir(&path_z).expect("failed to create dir");
        let meta_z = Meta::from_path(&path_z, false).expect("failed to get meta");

        let mut flags = Flags::default();
        flags.sorting.dir_grouping = DirGrouping::None;

        // Sort by name unordered
        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(&sorter, &meta_a, &meta_z), Ordering::Greater);

        // Sort by name unordered reversed
        flags.sorting.order = SortOrder::Reverse;

        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(&sorter, &meta_a, &meta_z), Ordering::Less);
    }

    #[test]
    fn test_sort_assemble_sorters_by_time() {
        let tmp_dir = tempdir().expect("failed to create temp dir");

        // Create the file;
        let path_a = tmp_dir.path().join("aaa");
        File::create(&path_a).expect("failed to create file");
        let meta_a = Meta::from_path(&path_a, false).expect("failed to get meta");

        // Create the file;
        let path_z = tmp_dir.path().join("zzz");
        File::create(&path_z).expect("failed to create file");

        #[cfg(unix)]
        let success = Command::new("touch")
            .arg("-t")
            .arg("198511160000")
            .arg(&path_z)
            .status()
            .unwrap()
            .success();

        #[cfg(windows)]
        let success = Command::new("powershell")
            .arg("-Command")
            .arg("$(Get-Item")
            .arg(&path_z)
            .arg(").lastwritetime=$(Get-Date \"1985-11-16\")")
            .status()
            .unwrap()
            .success();

        assert_eq!(true, success, "failed to change file timestamp");
        let meta_z = Meta::from_path(&path_z, false).expect("failed to get meta");

        let mut flags = Flags::default();
        flags.sorting.column = SortColumn::Time;

        // Sort by time
        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(&sorter, &meta_a, &meta_z), Ordering::Less);

        // Sort by time reversed
        flags.sorting.order = SortOrder::Reverse;
        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(&sorter, &meta_a, &meta_z), Ordering::Greater);
    }

    #[test]
    fn test_sort_assemble_sorters_by_extension() {
        let tmp_dir = tempdir().expect("failed to create temp dir");

        // Create the file with rs extension;
        let path_a = tmp_dir.path().join("aaa.rs");
        File::create(&path_a).expect("failed to create file");
        let meta_a = Meta::from_path(&path_a, false).expect("failed to get meta");

        // Create the file with rs extension;
        let path_z = tmp_dir.path().join("zzz.rs");
        File::create(&path_z).expect("failed to create file");
        let meta_z = Meta::from_path(&path_z, false).expect("failed to get meta");

        // Create the file with js extension;
        let path_j = tmp_dir.path().join("zzz.js");
        File::create(&path_j).expect("failed to create file");
        let meta_j = Meta::from_path(&path_j, false).expect("failed to get meta");

        // Create the file with txt extension;
        let path_t = tmp_dir.path().join("zzz.txt");
        File::create(&path_t).expect("failed to create file");
        let meta_t = Meta::from_path(&path_t, false).expect("failed to get meta");

        let mut flags = Flags::default();
        flags.sorting.column = SortColumn::Extension;

        // Sort by extension
        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(&sorter, &meta_a, &meta_z), Ordering::Equal);

        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(&sorter, &meta_a, &meta_j), Ordering::Greater);

        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(&sorter, &meta_a, &meta_t), Ordering::Less);
    }

    #[test]
    fn test_sort_assemble_sorters_by_version() {
        let tmp_dir = tempdir().expect("failed to create temp dir");

        let path_a = tmp_dir.path().join("2");
        File::create(&path_a).expect("failed to create file");
        let meta_a = Meta::from_path(&path_a, false).expect("failed to get meta");

        let path_b = tmp_dir.path().join("11");
        File::create(&path_b).expect("failed to create file");
        let meta_b = Meta::from_path(&path_b, false).expect("failed to get meta");

        let path_c = tmp_dir.path().join("12");
        File::create(&path_c).expect("failed to create file");
        let meta_c = Meta::from_path(&path_c, false).expect("failed to get meta");

        let mut flags = Flags::default();
        flags.sorting.column = SortColumn::Version;

        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(&sorter, &meta_b, &meta_a), Ordering::Greater);

        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(&sorter, &meta_b, &meta_c), Ordering::Less);
    }

    #[test]
    fn test_sort_assemble_sorters_no_sort() {
        let tmp_dir = tempdir().expect("failed to create temp dir");

        let path_a = tmp_dir.path().join("aaa.aa");
        File::create(&path_a).expect("failed to create file");
        let meta_a = Meta::from_path(&path_a, false).expect("failed to get meta");

        let path_b = tmp_dir.path().join("aaa");
        create_dir(&path_b).expect("failed to create dir");
        let meta_b = Meta::from_path(&path_b, false).expect("failed to get meta");

        let path_c = tmp_dir.path().join("zzz.zz");
        File::create(&path_c).expect("failed to create file");
        let meta_c = Meta::from_path(&path_c, false).expect("failed to get meta");

        let path_d = tmp_dir.path().join("zzz");
        create_dir(&path_d).expect("failed to create dir");
        let meta_d = Meta::from_path(&path_d, false).expect("failed to get meta");

        let mut flags = Flags::default();
        flags.sorting.column = SortColumn::None;

        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(&sorter, &meta_a, &meta_b), Ordering::Equal);

        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(&sorter, &meta_a, &meta_c), Ordering::Equal);

        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(&sorter, &meta_a, &meta_d), Ordering::Equal);

        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(&sorter, &meta_b, &meta_c), Ordering::Equal);

        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(&sorter, &meta_b, &meta_d), Ordering::Equal);

        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(&sorter, &meta_c, &meta_d), Ordering::Equal);
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5050() {
    rusty_monitor::set_test_id(5050);
    let mut bool_0: bool = true;
    let mut usize_0: usize = 51usize;
    let mut bool_1: bool = false;
    let mut option_0: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut option_1: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut usize_1: usize = 19usize;
    let mut option_2: std::option::Option<usize> = std::option::Option::Some(usize_1);
    let mut option_3: std::option::Option<bool> = std::option::Option::None;
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_3, depth: option_2};
    let mut option_4: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_0);
    let mut option_5: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_2: bool = false;
    let mut option_6: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut option_7: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_8: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_9: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut option_10: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_0);
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_10, theme: option_9, separator: option_8};
    let mut option_11: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut option_12: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut bool_3: bool = true;
    let mut option_13: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut option_14: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_15: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut option_16: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_16, theme: option_15};
    let mut option_17: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_18: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_19: std::option::Option<bool> = std::option::Option::None;
    let mut display_0: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_0};
    let mut option_20: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_21: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_22: std::option::Option<bool> = std::option::Option::None;
    let mut option_23: std::option::Option<bool> = std::option::Option::None;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut option_24: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::Some(dirgrouping_0);
    let mut option_25: std::option::Option<bool> = std::option::Option::None;
    let mut option_26: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_26, reverse: option_25, dir_grouping: option_24};
    let mut option_27: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_0);
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut option_28: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_0);
    let mut option_29: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_30: std::option::Option<usize> = std::option::Option::None;
    let mut option_31: std::option::Option<bool> = std::option::Option::None;
    let mut recursion_1: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_31, depth: option_30};
    let mut option_32: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_1);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut option_33: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_0);
    let mut option_34: std::option::Option<bool> = std::option::Option::None;
    let mut option_35: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_36: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_37: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut option_38: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_1);
    let mut icons_1: crate::config_file::Icons = crate::config_file::Icons {when: option_38, theme: option_37, separator: option_36};
    let mut option_39: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_1);
    let mut display_1: flags::display::Display = crate::flags::display::Display::All;
    let mut option_40: std::option::Option<flags::display::Display> = std::option::Option::Some(display_1);
    let mut bool_4: bool = true;
    let mut option_41: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut option_42: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_43: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_44: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_45: std::option::Option<bool> = std::option::Option::None;
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_45, blocks: option_44, color: option_43, date: option_42, dereference: option_41, display: option_40, icons: option_39, ignore_globs: option_35, indicators: option_34, layout: option_33, recursion: option_32, size: option_29, permission: option_28, sorting: option_27, no_symlink: option_23, total_size: option_22, symlink_arrow: option_21, hyperlink: option_20};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut u64_0: u64 = 31u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut bool_5: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_5};
    let mut str_0: &str = "JBB02nUWXf";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut theme_0: icon::Theme = crate::icon::Theme::Fancy;
    panic!("From RustyUnit with love");
}
}