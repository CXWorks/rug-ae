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
#[timeout(30000)]fn rusty_test_235() {
//    rusty_monitor::set_test_id(235);
    let mut usize_0: usize = 360usize;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut option_0: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_2: bool = false;
    let mut option_1: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut option_2: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_3: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut option_4: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_0);
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut option_5: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_0);
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_5, theme: option_4, separator: option_3};
    let mut option_6: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut option_7: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_8: std::option::Option<bool> = std::option::Option::None;
    let mut option_9: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_10: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_11: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_12: std::option::Option<bool> = std::option::Option::None;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut option_13: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_1);
    let mut option_14: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_3: bool = false;
    let mut option_15: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut option_16: std::option::Option<bool> = std::option::Option::None;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut option_17: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::Some(dirgrouping_1);
    let mut bool_4: bool = false;
    let mut option_18: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut option_19: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut sorting_1: crate::config_file::Sorting = crate::config_file::Sorting {column: option_19, reverse: option_18, dir_grouping: option_17};
    let mut option_20: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_1);
    let mut option_21: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut option_22: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_0);
    let mut option_23: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_24: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut option_25: std::option::Option<bool> = std::option::Option::None;
    let mut option_26: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_27: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_28: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut option_29: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_1);
    let mut icons_1: crate::config_file::Icons = crate::config_file::Icons {when: option_29, theme: option_28, separator: option_27};
    let mut option_30: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_1);
    let mut option_31: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut bool_5: bool = false;
    let mut option_32: std::option::Option<bool> = std::option::Option::Some(bool_5);
    let mut option_33: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_34: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_35: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_36: std::option::Option<bool> = std::option::Option::None;
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_36, blocks: option_35, color: option_34, date: option_33, dereference: option_32, display: option_31, icons: option_30, ignore_globs: option_26, indicators: option_25, layout: option_24, recursion: option_23, size: option_22, permission: option_21, sorting: option_20, no_symlink: option_16, total_size: option_15, symlink_arrow: option_14, hyperlink: option_13};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_1: usize = 80usize;
    let mut bool_6: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_6, depth: usize_1};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_2: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_1};
    let mut hyperlinkoption_2: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_569() {
//    rusty_monitor::set_test_id(569);
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut bool_2: bool = false;
    let mut bool_3: bool = true;
    let mut bool_4: bool = false;
    let mut bool_5: bool = false;
    let mut bool_6: bool = false;
    let mut bool_7: bool = true;
    let mut bool_8: bool = false;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut bool_12: bool = true;
    let mut bool_13: bool = false;
    let mut bool_14: bool = false;
    let mut bool_15: bool = true;
    let mut elem_0: color::Elem = crate::color::Elem::INode {valid: bool_15};
    let mut elem_1: color::Elem = crate::color::Elem::INode {valid: bool_14};
    let mut elem_2: color::Elem = crate::color::Elem::INode {valid: bool_13};
    let mut elem_3: color::Elem = crate::color::Elem::INode {valid: bool_12};
    let mut elem_4: color::Elem = crate::color::Elem::INode {valid: bool_11};
    let mut elem_5: color::Elem = crate::color::Elem::INode {valid: bool_10};
    let mut elem_6: color::Elem = crate::color::Elem::INode {valid: bool_9};
    let mut elem_7: color::Elem = crate::color::Elem::INode {valid: bool_8};
    let mut elem_8: color::Elem = crate::color::Elem::INode {valid: bool_7};
    let mut elem_9: color::Elem = crate::color::Elem::INode {valid: bool_6};
    let mut elem_10: color::Elem = crate::color::Elem::INode {valid: bool_5};
    let mut elem_11: color::Elem = crate::color::Elem::INode {valid: bool_4};
    let mut elem_12: color::Elem = crate::color::Elem::INode {valid: bool_3};
    let mut elem_13: color::Elem = crate::color::Elem::INode {valid: bool_2};
    let mut elem_14: color::Elem = crate::color::Elem::INode {valid: bool_1};
    let mut elem_15: color::Elem = crate::color::Elem::INode {valid: bool_0};
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_414() {
//    rusty_monitor::set_test_id(414);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut hyperlinkoption_2: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut hyperlinkoption_3: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut hyperlinkoption_4: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut hyperlinkoption_5: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut hyperlinkoption_6: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut hyperlinkoption_7: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut hyperlinkoption_8: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut hyperlinkoption_9: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut hyperlinkoption_10: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut hyperlinkoption_11: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut hyperlinkoption_12: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut hyperlinkoption_13: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut hyperlinkoption_14: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut hyperlinkoption_15: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut hyperlinkoption_16: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut hyperlinkoption_17: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut hyperlinkoption_18: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut hyperlinkoption_19: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut hyperlinkoption_20: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut hyperlinkoption_21: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut hyperlinkoption_22: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut hyperlinkoption_23: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut hyperlinkoption_24: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut hyperlinkoption_25: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut hyperlinkoption_26: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut hyperlinkoption_27: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut hyperlinkoption_28: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut hyperlinkoption_29: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_268() {
//    rusty_monitor::set_test_id(268);
    let mut usize_0: usize = 1usize;
    let mut option_0: std::option::Option<usize> = std::option::Option::Some(usize_0);
    let mut option_1: std::option::Option<usize> = std::option::Option::None;
    let mut usize_1: usize = 0usize;
    let mut option_2: std::option::Option<usize> = std::option::Option::Some(usize_1);
    let mut option_3: std::option::Option<usize> = std::option::Option::None;
    let mut usize_2: usize = 1usize;
    let mut option_4: std::option::Option<usize> = std::option::Option::Some(usize_2);
    let mut usize_3: usize = 39usize;
    let mut option_5: std::option::Option<usize> = std::option::Option::Some(usize_3);
    let mut option_6: std::option::Option<usize> = std::option::Option::None;
    let mut option_7: std::option::Option<usize> = std::option::Option::None;
    let mut usize_4: usize = 2usize;
    let mut option_8: std::option::Option<usize> = std::option::Option::Some(usize_4);
    let mut usize_5: usize = 40usize;
    let mut option_9: std::option::Option<usize> = std::option::Option::Some(usize_5);
    let mut option_10: std::option::Option<usize> = std::option::Option::None;
    let mut usize_6: usize = 27usize;
    let mut option_11: std::option::Option<usize> = std::option::Option::Some(usize_6);
    let mut option_12: std::option::Option<usize> = std::option::Option::None;
    let mut usize_7: usize = 6usize;
    let mut option_13: std::option::Option<usize> = std::option::Option::Some(usize_7);
    let mut option_14: std::option::Option<usize> = std::option::Option::None;
    let mut usize_8: usize = 67usize;
    let mut option_15: std::option::Option<usize> = std::option::Option::Some(usize_8);
    let mut usize_9: usize = 360usize;
    let mut option_16: std::option::Option<usize> = std::option::Option::Some(usize_9);
    let mut usize_10: usize = 78usize;
    let mut option_17: std::option::Option<usize> = std::option::Option::Some(usize_10);
    let mut option_18: std::option::Option<usize> = std::option::Option::None;
    let mut option_19: std::option::Option<usize> = std::option::Option::None;
    let mut option_20: std::option::Option<usize> = std::option::Option::None;
//    panic!("From RustyUnit with love");
}
}