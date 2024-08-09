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
#[timeout(30000)]fn rusty_test_5677() {
//    rusty_monitor::set_test_id(5677);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut dateflag_0_ref_0: &flags::date::DateFlag = &mut dateflag_0;
    let mut u64_0: u64 = 1024u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut u64_1: u64 = 1099511627776u64;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_0: usize = 120usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_1);
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_1: bool = false;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_3: std::option::Option<bool> = std::option::Option::None;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut option_4: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::Some(dirgrouping_1);
    let mut option_5: std::option::Option<bool> = std::option::Option::None;
    let mut option_6: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut sorting_1: crate::config_file::Sorting = crate::config_file::Sorting {column: option_6, reverse: option_5, dir_grouping: option_4};
    let mut option_7: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_1);
    let mut option_8: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut option_9: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut usize_1: usize = 40usize;
    let mut option_10: std::option::Option<usize> = std::option::Option::Some(usize_1);
    let mut option_11: std::option::Option<bool> = std::option::Option::None;
    let mut recursion_1: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_11, depth: option_10};
    let mut option_12: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_1);
    let mut option_13: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut option_14: std::option::Option<bool> = std::option::Option::None;
    let mut option_15: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_16: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut option_17: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
//    panic!("From RustyUnit with love");
}
}