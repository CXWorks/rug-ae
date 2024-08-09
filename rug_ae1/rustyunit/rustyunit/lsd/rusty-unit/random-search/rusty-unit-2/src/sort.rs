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
fn rusty_test_5177() {
    rusty_monitor::set_test_id(5177);
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut bool_2: bool = false;
    let mut bool_3: bool = false;
    let mut bool_4: bool = false;
    let mut bool_5: bool = false;
    let mut bool_6: bool = true;
    let mut bool_7: bool = false;
    let mut bool_8: bool = true;
    let mut bool_9: bool = false;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut bool_12: bool = true;
    let mut bool_13: bool = true;
    let mut bool_14: bool = false;
    let mut bool_15: bool = true;
    let mut bool_16: bool = false;
    let mut bool_17: bool = false;
    let mut bool_18: bool = true;
    let mut bool_19: bool = false;
    let mut bool_20: bool = true;
    let mut bool_21: bool = true;
    let mut bool_22: bool = false;
    let mut bool_23: bool = false;
    let mut bool_24: bool = false;
    let mut bool_25: bool = true;
    let mut u64_0: u64 = 78u64;
    let mut bool_26: bool = false;
    let mut bool_27: bool = false;
    let mut bool_28: bool = true;
    let mut bool_29: bool = false;
    let mut bool_30: bool = true;
    let mut bool_31: bool = false;
    let mut bool_32: bool = false;
    let mut bool_33: bool = false;
    let mut bool_34: bool = true;
    let mut bool_35: bool = false;
    let mut bool_36: bool = false;
    let mut bool_37: bool = false;
    let mut bool_38: bool = true;
    let mut usize_0: usize = 88usize;
    let mut bool_39: bool = true;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_0);
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_2: std::option::Option<bool> = std::option::Option::None;
    let mut bool_40: bool = true;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_40);
    let mut option_4: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::None;
    let mut option_5: std::option::Option<bool> = std::option::Option::None;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut option_6: std::option::Option<flags::sorting::SortColumn> = std::option::Option::Some(sortcolumn_0);
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_6, reverse: option_5, dir_grouping: option_4};
    let mut option_7: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_0);
    let mut option_8: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut option_9: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_0);
    let mut option_10: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_11: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut option_12: std::option::Option<bool> = std::option::Option::None;
    let mut option_13: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_14: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_15: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut option_16: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_1);
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_16, theme: option_15, separator: option_14};
    let mut option_17: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut option_18: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut option_19: std::option::Option<bool> = std::option::Option::None;
    let mut option_20: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_21: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut option_22: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_22, theme: option_21};
    let mut option_23: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_24: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_25: std::option::Option<bool> = std::option::Option::None;
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_25, blocks: option_24, color: option_23, date: option_20, dereference: option_19, display: option_18, icons: option_17, ignore_globs: option_13, indicators: option_12, layout: option_11, recursion: option_10, size: option_9, permission: option_8, sorting: option_7, no_symlink: option_3, total_size: option_2, symlink_arrow: option_1, hyperlink: option_0};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut display_1: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_0};
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut option_26: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_1);
    let mut option_27: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_28: std::option::Option<bool> = std::option::Option::None;
    let mut option_29: std::option::Option<bool> = std::option::Option::None;
    let mut option_30: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut option_31: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_0);
    let mut option_32: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_33: std::option::Option<usize> = std::option::Option::None;
    let mut bool_41: bool = true;
    let mut option_34: std::option::Option<bool> = std::option::Option::Some(bool_41);
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_34, depth: option_33};
    let mut option_35: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_0);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut option_36: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_0);
    let mut option_37: std::option::Option<bool> = std::option::Option::None;
    let mut option_38: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_39: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_40: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut bool_42: bool = false;
    let mut option_41: std::option::Option<bool> = std::option::Option::Some(bool_42);
    let mut option_42: std::option::Option<std::string::String> = std::option::Option::None;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut option_43: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_1);
    let mut option_44: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut color_2: crate::config_file::Color = crate::config_file::Color {when: option_44, theme: option_43};
    let mut option_45: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_2);
    let mut option_46: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_47: std::option::Option<bool> = std::option::Option::None;
    let mut config_1: crate::config_file::Config = crate::config_file::Config {classic: option_47, blocks: option_46, color: option_45, date: option_42, dereference: option_41, display: option_40, icons: option_39, ignore_globs: option_38, indicators: option_37, layout: option_36, recursion: option_35, size: option_32, permission: option_31, sorting: option_30, no_symlink: option_29, total_size: option_28, symlink_arrow: option_27, hyperlink: option_26};
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut u64_1: u64 = 85u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    panic!("From RustyUnit with love");
}
}