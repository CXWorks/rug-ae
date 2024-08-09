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
                };
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
        let path_a = tmp_dir.path().join("zzz");
        File::create(&path_a).expect("failed to create file");
        let meta_a = Meta::from_path(&path_a, false).expect("failed to get meta");
        let path_z = tmp_dir.path().join("aaa");
        create_dir(&path_z).expect("failed to create dir");
        let meta_z = Meta::from_path(&path_z, false).expect("failed to get meta");
        let mut flags = Flags::default();
        flags.sorting.dir_grouping = DirGrouping::First;
        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(& sorter, & meta_a, & meta_z), Ordering::Greater);
        flags.sorting.order = SortOrder::Reverse;
        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(& sorter, & meta_a, & meta_z), Ordering::Greater);
    }
    #[test]
    fn test_sort_assemble_sorters_by_name_with_files_first() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let path_a = tmp_dir.path().join("zzz");
        File::create(&path_a).expect("failed to create file");
        let meta_a = Meta::from_path(&path_a, false).expect("failed to get meta");
        let path_z = tmp_dir.path().join("aaa");
        create_dir(&path_z).expect("failed to create dir");
        let meta_z = Meta::from_path(&path_z, false).expect("failed to get meta");
        let mut flags = Flags::default();
        flags.sorting.dir_grouping = DirGrouping::Last;
        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(& sorter, & meta_a, & meta_z), Ordering::Less);
        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(& sorter, & meta_a, & meta_z), Ordering::Less);
    }
    #[test]
    fn test_sort_assemble_sorters_by_name_unordered() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let path_a = tmp_dir.path().join("aaa");
        File::create(&path_a).expect("failed to create file");
        let meta_a = Meta::from_path(&path_a, false).expect("failed to get meta");
        let path_z = tmp_dir.path().join("zzz");
        create_dir(&path_z).expect("failed to create dir");
        let meta_z = Meta::from_path(&path_z, false).expect("failed to get meta");
        let mut flags = Flags::default();
        flags.sorting.dir_grouping = DirGrouping::None;
        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(& sorter, & meta_a, & meta_z), Ordering::Less);
        flags.sorting.order = SortOrder::Reverse;
        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(& sorter, & meta_a, & meta_z), Ordering::Greater);
    }
    #[test]
    fn test_sort_assemble_sorters_by_name_unordered_2() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let path_a = tmp_dir.path().join("zzz");
        File::create(&path_a).expect("failed to create file");
        let meta_a = Meta::from_path(&path_a, false).expect("failed to get meta");
        let path_z = tmp_dir.path().join("aaa");
        create_dir(&path_z).expect("failed to create dir");
        let meta_z = Meta::from_path(&path_z, false).expect("failed to get meta");
        let mut flags = Flags::default();
        flags.sorting.dir_grouping = DirGrouping::None;
        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(& sorter, & meta_a, & meta_z), Ordering::Greater);
        flags.sorting.order = SortOrder::Reverse;
        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(& sorter, & meta_a, & meta_z), Ordering::Less);
    }
    #[test]
    fn test_sort_assemble_sorters_by_time() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let path_a = tmp_dir.path().join("aaa");
        File::create(&path_a).expect("failed to create file");
        let meta_a = Meta::from_path(&path_a, false).expect("failed to get meta");
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
        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(& sorter, & meta_a, & meta_z), Ordering::Less);
        flags.sorting.order = SortOrder::Reverse;
        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(& sorter, & meta_a, & meta_z), Ordering::Greater);
    }
    #[test]
    fn test_sort_assemble_sorters_by_extension() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let path_a = tmp_dir.path().join("aaa.rs");
        File::create(&path_a).expect("failed to create file");
        let meta_a = Meta::from_path(&path_a, false).expect("failed to get meta");
        let path_z = tmp_dir.path().join("zzz.rs");
        File::create(&path_z).expect("failed to create file");
        let meta_z = Meta::from_path(&path_z, false).expect("failed to get meta");
        let path_j = tmp_dir.path().join("zzz.js");
        File::create(&path_j).expect("failed to create file");
        let meta_j = Meta::from_path(&path_j, false).expect("failed to get meta");
        let path_t = tmp_dir.path().join("zzz.txt");
        File::create(&path_t).expect("failed to create file");
        let meta_t = Meta::from_path(&path_t, false).expect("failed to get meta");
        let mut flags = Flags::default();
        flags.sorting.column = SortColumn::Extension;
        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(& sorter, & meta_a, & meta_z), Ordering::Equal);
        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(& sorter, & meta_a, & meta_j), Ordering::Greater);
        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(& sorter, & meta_a, & meta_t), Ordering::Less);
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
        assert_eq!(by_meta(& sorter, & meta_b, & meta_a), Ordering::Greater);
        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(& sorter, & meta_b, & meta_c), Ordering::Less);
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
        assert_eq!(by_meta(& sorter, & meta_a, & meta_b), Ordering::Equal);
        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(& sorter, & meta_a, & meta_c), Ordering::Equal);
        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(& sorter, & meta_a, & meta_d), Ordering::Equal);
        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(& sorter, & meta_b, & meta_c), Ordering::Equal);
        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(& sorter, & meta_b, & meta_d), Ordering::Equal);
        let sorter = assemble_sorters(&flags);
        assert_eq!(by_meta(& sorter, & meta_c, & meta_d), Ordering::Equal);
    }
}
#[cfg(test)]
mod tests_llm_16_286 {
    use super::*;
    use crate::*;
    #[test]
    fn test_assemble_sorters() {
        let _rug_st_tests_llm_16_286_rrrruuuugggg_test_assemble_sorters = 0;
        let flags = Flags {
            blocks: Default::default(),
            color: Default::default(),
            date: Default::default(),
            dereference: Default::default(),
            display: Default::default(),
            display_indicators: Default::default(),
            icons: Default::default(),
            ignore_globs: Default::default(),
            layout: Default::default(),
            no_symlink: Default::default(),
            recursion: Default::default(),
            size: Default::default(),
            permission: Default::default(),
            sorting: Default::default(),
            total_size: Default::default(),
            symlink_arrow: Default::default(),
            hyperlink: Default::default(),
        };
        let result = assemble_sorters(&flags);
        let _rug_ed_tests_llm_16_286_rrrruuuugggg_test_assemble_sorters = 0;
    }
}
