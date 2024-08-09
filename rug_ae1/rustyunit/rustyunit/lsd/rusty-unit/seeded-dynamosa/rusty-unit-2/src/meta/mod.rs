pub mod access_control;
pub mod date;
pub mod filetype;
pub mod indicator;
pub mod inode;
pub mod links;
pub mod name;
pub mod owner;
pub mod permissions;
pub mod size;
pub mod symlink;

#[cfg(windows)]
mod windows_utils;

pub use self::access_control::AccessControl;
pub use self::date::Date;
pub use self::filetype::FileType;
pub use self::indicator::Indicator;
pub use self::inode::INode;
pub use self::links::Links;
pub use self::name::Name;
pub use self::owner::Owner;
pub use self::permissions::Permissions;
pub use self::size::Size;
pub use self::symlink::SymLink;
pub use crate::icon::Icons;

use crate::flags::{Display, Flags, Layout};
use crate::print_error;

use std::io::{Error, ErrorKind};
use std::path::{Component, Path, PathBuf};

#[derive(Clone, Debug)]
pub struct Meta {
    pub name: Name,
    pub path: PathBuf,
    pub permissions: Permissions,
    pub date: Date,
    pub owner: Owner,
    pub file_type: FileType,
    pub size: Size,
    pub symlink: SymLink,
    pub indicator: Indicator,
    pub inode: INode,
    pub links: Links,
    pub content: Option<Vec<Meta>>,
    pub access_control: AccessControl,
}

impl Meta {
    pub fn recurse_into(
        &self,
        depth: usize,
        flags: &Flags,
    ) -> Result<Option<Vec<Meta>>, std::io::Error> {
        if depth == 0 {
            return Ok(None);
        }

        if flags.display == Display::DirectoryOnly && flags.layout != Layout::Tree {
            return Ok(None);
        }

        match self.file_type {
            FileType::Directory { .. } => (),
            FileType::SymLink { is_dir: true } => {
                if flags.layout == Layout::OneLine {
                    return Ok(None);
                }
            }
            _ => return Ok(None),
        }

        let entries = match self.path.read_dir() {
            Ok(entries) => entries,
            Err(err) => {
                print_error!("{}: {}.", self.path.display(), err);
                return Ok(None);
            }
        };

        let mut content: Vec<Meta> = Vec::new();

        if Display::All == flags.display && flags.layout != Layout::Tree {
            let mut current_meta = self.clone();
            current_meta.name.name = ".".to_owned();

            let mut parent_meta =
                Self::from_path(&self.path.join(Component::ParentDir), flags.dereference.0)?;
            parent_meta.name.name = "..".to_owned();

            content.push(current_meta);
            content.push(parent_meta);
        }

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            let name = path
                .file_name()
                .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "invalid file name"))?;

            if flags.ignore_globs.0.is_match(&name) {
                continue;
            }

            if let Display::VisibleOnly = flags.display {
                if name.to_string_lossy().starts_with('.') {
                    continue;
                }
            }

            let mut entry_meta = match Self::from_path(&path, flags.dereference.0) {
                Ok(res) => res,
                Err(err) => {
                    print_error!("{}: {}.", path.display(), err);
                    continue;
                }
            };

            // skip files for --tree -d
            if flags.layout == Layout::Tree {
                if let Display::DirectoryOnly = flags.display {
                    if !entry.file_type()?.is_dir() {
                        continue;
                    }
                }
            }

            let dereference =
                !matches!(entry_meta.file_type, FileType::SymLink { .. }) || flags.dereference.0;
            if dereference {
                match entry_meta.recurse_into(depth - 1, flags) {
                    Ok(content) => entry_meta.content = content,
                    Err(err) => {
                        print_error!("{}: {}.", path.display(), err);
                        continue;
                    }
                };
            }

            content.push(entry_meta);
        }

        Ok(Some(content))
    }

    pub fn calculate_total_size(&mut self) {
        if let FileType::Directory { .. } = self.file_type {
            if let Some(metas) = &mut self.content {
                let mut size_accumulated = self.size.get_bytes();
                for x in &mut metas.iter_mut() {
                    x.calculate_total_size();
                    size_accumulated += x.size.get_bytes();
                }
                self.size = Size::new(size_accumulated);
            } else {
                // possibility that 'depth' limited the recursion in 'recurse_into'
                self.size = Size::new(Meta::calculate_total_file_size(&self.path));
            }
        }
    }

    fn calculate_total_file_size(path: &Path) -> u64 {
        let metadata = path.symlink_metadata();
        let metadata = match metadata {
            Ok(meta) => meta,
            Err(err) => {
                print_error!("{}: {}.", path.display(), err);
                return 0;
            }
        };
        let file_type = metadata.file_type();
        if file_type.is_file() {
            metadata.len()
        } else if file_type.is_dir() {
            let mut size = metadata.len();

            let entries = match path.read_dir() {
                Ok(entries) => entries,
                Err(err) => {
                    print_error!("{}: {}.", path.display(), err);
                    return size;
                }
            };
            for entry in entries {
                let path = match entry {
                    Ok(entry) => entry.path(),
                    Err(err) => {
                        print_error!("{}: {}.", path.display(), err);
                        continue;
                    }
                };
                size += Meta::calculate_total_file_size(&path);
            }
            size
        } else {
            0
        }
    }

    pub fn from_path(path: &Path, dereference: bool) -> Result<Self, std::io::Error> {
        let mut metadata = path.symlink_metadata()?;
        let mut symlink_meta = None;
        if metadata.file_type().is_symlink() {
            match path.metadata() {
                Ok(m) => {
                    if dereference {
                        metadata = m;
                    } else {
                        symlink_meta = Some(m);
                    }
                }
                Err(e) => {
                    // This case, it is definitely a symlink or
                    // path.symlink_metadata would have errored out
                    if dereference {
                        return Err(e);
                    }
                }
            }
        }

        #[cfg(unix)]
        let owner = Owner::from(&metadata);
        #[cfg(unix)]
        let permissions = Permissions::from(&metadata);

        #[cfg(windows)]
        let (owner, permissions) = windows_utils::get_file_data(path)?;

        let access_control = AccessControl::for_path(path);
        let file_type = FileType::new(&metadata, symlink_meta.as_ref(), &permissions);
        let name = Name::new(path, file_type);
        let inode = INode::from(&metadata);
        let links = Links::from(&metadata);

        Ok(Self {
            inode,
            links,
            path: path.to_path_buf(),
            symlink: SymLink::from(path),
            size: Size::from(&metadata),
            date: Date::from(&metadata),
            indicator: Indicator::from(file_type),
            owner,
            permissions,
            name,
            file_type,
            content: None,
            access_control,
        })
    }
}

#[cfg(unix)]
#[cfg(test)]
mod tests {
    use super::Meta;

    #[test]
    fn test_from_path_path() {
        let dir = assert_fs::TempDir::new().unwrap();
        let meta = Meta::from_path(dir.path(), false).unwrap();
        assert_eq!(meta.path, dir.path())
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_198() {
//    rusty_monitor::set_test_id(198);
    let mut str_0: &str = "pOCLKJE7Gwx";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut usize_0: usize = 80usize;
    let mut str_1: &str = "ExecSticky";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut usize_1: usize = 8usize;
    let mut tuple_0: (usize, &str) = (usize_1, str_1_ref_0);
    let mut usize_2: usize = 6usize;
    let mut bool_0: bool = false;
    let mut usize_3: usize = 1usize;
    let mut bool_1: bool = true;
    let mut str_2: &str = "clj";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "Icons";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut bool_2: bool = false;
    let mut bool_3: bool = false;
    let mut u64_0: u64 = 1048576u64;
    let mut usize_4: usize = 1usize;
    let mut usize_5: usize = 8usize;
    let mut usize_6: usize = 0usize;
    let mut bool_4: bool = false;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_7: usize = 40usize;
    let mut bool_5: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_5, depth: usize_7};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut u64_1: u64 = 1024u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut bool_6: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_6};
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_8: usize = 2usize;
    let mut bool_7: bool = false;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_7, depth: usize_8};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_1: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_2};
    let mut str_4: &str = "classic";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_3);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_1, order: sortorder_1, dir_grouping: dirgrouping_1};
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_2: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_9: usize = 8usize;
    let mut bool_8: bool = false;
    let mut recursion_2: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_8, depth: usize_9};
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut icontheme_2: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_2: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_2: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_2: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_4: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_2: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_2: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_2, theme: themeoption_4};
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut bool_12: bool = true;
    let mut bool_13: bool = true;
    let mut bool_14: bool = false;
    let mut bool_15: bool = true;
    let mut bool_16: bool = true;
    let mut bool_17: bool = true;
    let mut bool_18: bool = false;
    let mut bool_19: bool = true;
    let mut bool_20: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_20, user_write: bool_19, user_execute: bool_18, group_read: bool_17, group_write: bool_16, group_execute: bool_15, other_read: bool_14, other_write: bool_13, other_execute: bool_12, sticky: bool_11, setgid: bool_10, setuid: bool_9};
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut option_0: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_1: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_2: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_203() {
//    rusty_monitor::set_test_id(203);
    let mut option_0: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut option_1: std::option::Option<crate::color::theme::Theme> = std::option::Option::Some(theme_0);
    let mut elem_0: color::Elem = crate::color::Elem::Group;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_1: color::Elem = crate::color::Elem::Group;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_2: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_3: color::Elem = crate::color::Elem::Write;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut elem_4: color::Elem = crate::color::Elem::Special;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut bool_0: bool = true;
    let mut elem_5: color::Elem = crate::color::Elem::Dir {uid: bool_0};
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut elem_6: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut elem_7: color::Elem = crate::color::Elem::HourOld;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut elem_8: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut elem_9: color::Elem = crate::color::Elem::Acl;
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    let mut elem_10: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_10_ref_0: &color::Elem = &mut elem_10;
    let mut elem_11: color::Elem = crate::color::Elem::NonFile;
    let mut elem_11_ref_0: &color::Elem = &mut elem_11;
    let mut elem_12: color::Elem = crate::color::Elem::Read;
    let mut elem_12_ref_0: &color::Elem = &mut elem_12;
    let mut elem_13: color::Elem = crate::color::Elem::Context;
    let mut elem_13_ref_0: &color::Elem = &mut elem_13;
    let mut elem_14: color::Elem = crate::color::Elem::HourOld;
    let mut elem_14_ref_0: &color::Elem = &mut elem_14;
    let mut elem_15: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_15_ref_0: &color::Elem = &mut elem_15;
    let mut elem_16: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_16_ref_0: &color::Elem = &mut elem_16;
    let mut elem_17: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_17_ref_0: &color::Elem = &mut elem_17;
    let mut elem_18: color::Elem = crate::color::Elem::NonFile;
    let mut elem_18_ref_0: &color::Elem = &mut elem_18;
    let mut elem_19: color::Elem = crate::color::Elem::Pipe;
    let mut elem_19_ref_0: &color::Elem = &mut elem_19;
    let mut elem_20: color::Elem = crate::color::Elem::Acl;
    let mut elem_20_ref_0: &color::Elem = &mut elem_20;
    let mut elem_21: color::Elem = crate::color::Elem::SymLink;
    let mut elem_21_ref_0: &color::Elem = &mut elem_21;
    let mut bool_1: bool = false;
    let mut elem_22: color::Elem = crate::color::Elem::Dir {uid: bool_1};
    let mut elem_22_ref_0: &color::Elem = &mut elem_22;
    let mut elem_23: color::Elem = crate::color::Elem::NonFile;
    let mut elem_23_ref_0: &color::Elem = &mut elem_23;
    let mut bool_2: bool = false;
    let mut elem_24: color::Elem = crate::color::Elem::Links {valid: bool_2};
    let mut elem_24_ref_0: &color::Elem = &mut elem_24;
    let mut bool_3: bool = false;
    let mut elem_25: color::Elem = crate::color::Elem::Dir {uid: bool_3};
    let mut elem_25_ref_0: &color::Elem = &mut elem_25;
    let mut elem_26: color::Elem = crate::color::Elem::Special;
    let mut elem_26_ref_0: &color::Elem = &mut elem_26;
    let mut elem_27: color::Elem = crate::color::Elem::HourOld;
    let mut elem_27_ref_0: &color::Elem = &mut elem_27;
    let mut bool_4: bool = false;
    let mut elem_28: color::Elem = crate::color::Elem::INode {valid: bool_4};
    let mut elem_28_ref_0: &color::Elem = &mut elem_28;
    let mut elem_29: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_29_ref_0: &color::Elem = &mut elem_29;
    let mut elem_30: color::Elem = crate::color::Elem::User;
    let mut elem_30_ref_0: &color::Elem = &mut elem_30;
    let mut elem_31: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_31_ref_0: &color::Elem = &mut elem_31;
    let mut elem_32: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_32_ref_0: &color::Elem = &mut elem_32;
    let mut elem_33: color::Elem = crate::color::Elem::Exec;
    let mut elem_33_ref_0: &color::Elem = &mut elem_33;
    let mut elem_34: color::Elem = crate::color::Elem::Special;
    let mut elem_34_ref_0: &color::Elem = &mut elem_34;
    let mut elem_35: color::Elem = crate::color::Elem::Older;
    let mut elem_35_ref_0: &color::Elem = &mut elem_35;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_356() {
//    rusty_monitor::set_test_id(356);
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::None;
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::None;
    let mut unit_2: meta::size::Unit = crate::meta::size::Unit::None;
    let mut unit_3: meta::size::Unit = crate::meta::size::Unit::None;
    let mut unit_4: meta::size::Unit = crate::meta::size::Unit::None;
    let mut unit_5: meta::size::Unit = crate::meta::size::Unit::None;
    let mut unit_6: meta::size::Unit = crate::meta::size::Unit::None;
    let mut unit_7: meta::size::Unit = crate::meta::size::Unit::None;
    let mut unit_8: meta::size::Unit = crate::meta::size::Unit::None;
    let mut unit_9: meta::size::Unit = crate::meta::size::Unit::None;
    let mut unit_10: meta::size::Unit = crate::meta::size::Unit::None;
    let mut unit_11: meta::size::Unit = crate::meta::size::Unit::None;
    let mut unit_12: meta::size::Unit = crate::meta::size::Unit::None;
    let mut unit_13: meta::size::Unit = crate::meta::size::Unit::None;
    let mut unit_14: meta::size::Unit = crate::meta::size::Unit::None;
    let mut unit_15: meta::size::Unit = crate::meta::size::Unit::None;
    let mut unit_16: meta::size::Unit = crate::meta::size::Unit::None;
    let mut unit_17: meta::size::Unit = crate::meta::size::Unit::None;
    let mut unit_18: meta::size::Unit = crate::meta::size::Unit::None;
    let mut unit_19: meta::size::Unit = crate::meta::size::Unit::None;
    let mut unit_20: meta::size::Unit = crate::meta::size::Unit::None;
    let mut unit_21: meta::size::Unit = crate::meta::size::Unit::None;
    let mut unit_22: meta::size::Unit = crate::meta::size::Unit::None;
    let mut unit_23: meta::size::Unit = crate::meta::size::Unit::None;
    let mut unit_24: meta::size::Unit = crate::meta::size::Unit::None;
    let mut unit_25: meta::size::Unit = crate::meta::size::Unit::None;
    let mut unit_26: meta::size::Unit = crate::meta::size::Unit::None;
    let mut unit_27: meta::size::Unit = crate::meta::size::Unit::None;
    let mut unit_28: meta::size::Unit = crate::meta::size::Unit::None;
    let mut unit_29: meta::size::Unit = crate::meta::size::Unit::None;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_358() {
//    rusty_monitor::set_test_id(358);
    let mut usize_0: usize = 120usize;
    let mut option_0: std::option::Option<usize> = std::option::Option::Some(usize_0);
    let mut option_1: std::option::Option<usize> = std::option::Option::None;
    let mut option_2: std::option::Option<usize> = std::option::Option::None;
    let mut usize_1: usize = 84usize;
    let mut option_3: std::option::Option<usize> = std::option::Option::Some(usize_1);
    let mut usize_2: usize = 120usize;
    let mut option_4: std::option::Option<usize> = std::option::Option::Some(usize_2);
    let mut option_5: std::option::Option<usize> = std::option::Option::None;
    let mut option_6: std::option::Option<usize> = std::option::Option::None;
    let mut usize_3: usize = 1usize;
    let mut option_7: std::option::Option<usize> = std::option::Option::Some(usize_3);
    let mut option_8: std::option::Option<usize> = std::option::Option::None;
    let mut usize_4: usize = 40usize;
    let mut option_9: std::option::Option<usize> = std::option::Option::Some(usize_4);
    let mut option_10: std::option::Option<usize> = std::option::Option::None;
    let mut usize_5: usize = 0usize;
    let mut option_11: std::option::Option<usize> = std::option::Option::Some(usize_5);
    let mut usize_6: usize = 6usize;
    let mut option_12: std::option::Option<usize> = std::option::Option::Some(usize_6);
    let mut usize_7: usize = 27usize;
    let mut option_13: std::option::Option<usize> = std::option::Option::Some(usize_7);
    let mut option_14: std::option::Option<usize> = std::option::Option::None;
    let mut option_15: std::option::Option<usize> = std::option::Option::None;
    let mut option_16: std::option::Option<usize> = std::option::Option::None;
    let mut option_17: std::option::Option<usize> = std::option::Option::None;
    let mut option_18: std::option::Option<usize> = std::option::Option::None;
    let mut option_19: std::option::Option<usize> = std::option::Option::None;
    let mut option_20: std::option::Option<usize> = std::option::Option::None;
    let mut usize_8: usize = 19usize;
    let mut option_21: std::option::Option<usize> = std::option::Option::Some(usize_8);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_570() {
//    rusty_monitor::set_test_id(570);
    let mut str_0: &str = "user_execute";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut usize_0: usize = 80usize;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut bool_5: bool = false;
    let mut bool_6: bool = false;
    let mut bool_7: bool = true;
    let mut usize_1: usize = 360usize;
    let mut bool_8: bool = true;
    let mut u64_0: u64 = 1024u64;
    let mut bool_9: bool = false;
    let mut usize_2: usize = 2usize;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut bool_12: bool = true;
    let mut bool_13: bool = false;
    let mut bool_14: bool = false;
    let mut bool_15: bool = false;
    let mut bool_16: bool = false;
    let mut bool_17: bool = true;
    let mut bool_18: bool = false;
    let mut bool_19: bool = true;
    let mut bool_20: bool = true;
    let mut bool_21: bool = false;
    let mut bool_22: bool = true;
    let mut bool_23: bool = true;
    let mut bool_24: bool = false;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_2: std::option::Option<bool> = std::option::Option::None;
    let mut bool_25: bool = false;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_25);
    let mut option_4: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::None;
    let mut bool_26: bool = false;
    let mut option_5: std::option::Option<bool> = std::option::Option::Some(bool_26);
    let mut option_6: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_6, reverse: option_5, dir_grouping: option_4};
    let mut option_7: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_0);
    let mut option_8: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut option_9: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_10: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_11: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut option_12: std::option::Option<bool> = std::option::Option::None;
    let mut option_13: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_14: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_15: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_16: std::option::Option<bool> = std::option::Option::None;
    let mut option_17: std::option::Option<std::string::String> = std::option::Option::None;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut option_18: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_0);
    let mut option_19: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_19, theme: option_18};
    let mut option_20: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_21: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_27: bool = true;
    let mut option_22: std::option::Option<bool> = std::option::Option::Some(bool_27);
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_22, blocks: option_21, color: option_20, date: option_17, dereference: option_16, display: option_15, icons: option_14, ignore_globs: option_13, indicators: option_12, layout: option_11, recursion: option_10, size: option_9, permission: option_8, sorting: option_7, no_symlink: option_3, total_size: option_2, symlink_arrow: option_1, hyperlink: option_0};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_3: usize = 2usize;
    let mut bool_28: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_28, depth: usize_3};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut option_23: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_24: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_25: std::option::Option<bool> = std::option::Option::None;
    let mut option_26: std::option::Option<bool> = std::option::Option::None;
    let mut option_27: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut option_28: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_1);
    let mut option_29: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_30: std::option::Option<usize> = std::option::Option::None;
    let mut bool_29: bool = false;
    let mut option_31: std::option::Option<bool> = std::option::Option::Some(bool_29);
    let mut recursion_1: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_31, depth: option_30};
    let mut option_32: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_1);
    let mut option_33: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_30: bool = true;
    let mut option_34: std::option::Option<bool> = std::option::Option::Some(bool_30);
    let mut option_35: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_36: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut option_37: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_1);
    let mut option_38: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_38, theme: option_37, separator: option_36};
    let mut option_39: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut option_40: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_41: std::option::Option<bool> = std::option::Option::None;
    let mut option_42: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_43: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_44: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_31: bool = false;
    let mut option_45: std::option::Option<bool> = std::option::Option::Some(bool_31);
    let mut config_1: crate::config_file::Config = crate::config_file::Config {classic: option_45, blocks: option_44, color: option_43, date: option_42, dereference: option_41, display: option_40, icons: option_39, ignore_globs: option_35, indicators: option_34, layout: option_33, recursion: option_32, size: option_29, permission: option_28, sorting: option_27, no_symlink: option_26, total_size: option_25, symlink_arrow: option_24, hyperlink: option_23};
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut display_0: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_1};
    let mut u64_1: u64 = 1048576u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
//    panic!("From RustyUnit with love");
}
}