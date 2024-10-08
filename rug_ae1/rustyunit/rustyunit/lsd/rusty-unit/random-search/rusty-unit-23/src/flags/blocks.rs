//! This module defines the [Blocks] struct. To set it up from [ArgMatches], a [Config] and its
//! [Default] value, use its [configure_from](Blocks::configure_from) method.

use crate::config_file::Config;
use crate::print_error;

use std::convert::TryFrom;

use clap::{ArgMatches, Error, ErrorKind};

/// A struct to hold a [Vec] of [Block]s and to provide methods to create it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Blocks(pub Vec<Block>);

impl Blocks {
    /// Returns a value from either [ArgMatches], a [Config] or a default value.
    /// Unless the "long" argument is passed, this returns [Default::default]. Otherwise the first
    /// value, that is not [None], is used. The order of precedence for the value used is:
    /// - [from_arg_matches](Blocks::from_arg_matches)
    /// - [from_config](Blocks::from_config)
    /// - [long](Blocks::long)
    ///
    /// No matter if the "long" argument was passed, if the "inode" argument is passed and the
    /// `Blocks` does not contain a [Block] of variant [INode](Block::INode) yet, one is prepended
    /// to the returned value.
    ///
    /// # Errors
    ///
    /// This errors if any of the [ArgMatches] parameter arguments causes [Block]'s implementation
    /// of [TryFrom::try_from] to return an [Err].
    pub fn configure_from(matches: &ArgMatches, config: &Config) -> Result<Self, Error> {
        let mut result: Result<Self, Error> = if matches.is_present("long") {
            Ok(Self::long())
        } else {
            Ok(Default::default())
        };

        if matches.is_present("long") && !matches.is_present("ignore-config") {
            if let Some(value) = Self::from_config(config) {
                result = Ok(value);
            }
        }

        if let Some(value) = Self::from_arg_matches(matches) {
            result = value;
        }

        if matches.is_present("context") {
            if let Ok(blocks) = result.as_mut() {
                blocks.optional_insert_context();
            }
        }

        if matches.is_present("inode") {
            if let Ok(blocks) = result.as_mut() {
                blocks.optional_prepend_inode();
            }
        }

        result
    }

    /// Get a potential `Blocks` struct from [ArgMatches].
    ///
    /// If the "blocks" argument is passed, then this returns a `Blocks` containing the parameter
    /// values in a [Some]. Otherwise if the "long" argument is passed, this returns
    /// [Blocks::long]. Finally if none of the previous happened, this returns [None].
    ///
    /// # Errors
    ///
    /// This errors if any of the parameter arguments causes [Block]'s implementation of
    /// [TryFrom::try_from] to return an [Err].
    fn from_arg_matches(matches: &ArgMatches) -> Option<Result<Self, Error>> {
        if matches.occurrences_of("blocks") > 0 {
            if let Some(values) = matches.values_of("blocks") {
                let mut blocks: Vec<Block> = vec![];
                for value in values {
                    match Block::try_from(value) {
                        Ok(block) => blocks.push(block),
                        Err(message) => {
                            return Some(Err(Error::with_description(
                                &message,
                                ErrorKind::ValueValidation,
                            )))
                        }
                    }
                }
                Some(Ok(Self(blocks)))
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Get a potential `Blocks` struct from a [Config].
    ///
    /// If the [Config] contains an array of blocks values,
    /// its [String] values is returned as `Blocks` in a [Some].
    /// Otherwise it returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        if let Some(c) = &config.blocks {
            let mut blocks: Vec<Block> = vec![];
            for b in c.iter() {
                match Block::try_from(b.as_str()) {
                    Ok(block) => blocks.push(block),
                    Err(err) => print_error!("{}.", err),
                }
            }
            if blocks.is_empty() {
                None
            } else {
                Some(Self(blocks))
            }
        } else {
            None
        }
    }

    /// This returns a Blocks struct for the long format.
    ///
    /// It contains the [Block]s [Permission](Block::Permission), [User](Block::User),
    /// [Group](Block::Group), [Size](Block::Size), [Date](Block::Date) and [Name](Block::Name).
    fn long() -> Self {
        Self(vec![
            Block::Permission,
            Block::User,
            Block::Group,
            Block::Size,
            Block::Date,
            Block::Name,
        ])
    }

    /// Checks whether `self` already contains a [Block] of variant [INode](Block::INode).
    fn contains_inode(&self) -> bool {
        self.0.contains(&Block::INode)
    }

    /// Prepends a [Block] of variant [INode](Block::INode) to `self`.
    fn prepend_inode(&mut self) {
        self.0.insert(0, Block::INode);
    }

    /// Prepends a [Block] of variant [INode](Block::INode), if `self` does not already contain a
    /// Block of that variant.
    fn optional_prepend_inode(&mut self) {
        if !self.contains_inode() {
            self.prepend_inode()
        }
    }

    pub fn displays_size(&self) -> bool {
        self.0.contains(&Block::Size)
    }

    /// Tnserts a [Block] of variant [INode](Block::Context), if `self` does not already contain a
    /// [Block] of that variant. The positioning will be best-effort approximation of coreutils
    /// ls position for a security context
    fn optional_insert_context(&mut self) {
        if self.0.contains(&Block::Context) {
            return;
        }
        let mut pos = self.0.iter().position(|elem| *elem == Block::Group);
        if pos.is_none() {
            pos = self.0.iter().position(|elem| *elem == Block::User);
        }
        match pos {
            Some(pos) => self.0.insert(pos + 1, Block::Context),
            None => self.0.insert(0, Block::Context),
        }
    }
}

/// The default value for `Blocks` contains a [Vec] of [Name](Block::Name).
impl Default for Blocks {
    fn default() -> Self {
        Self(vec![Block::Name])
    }
}

/// A block of data to show.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Block {
    Permission,
    User,
    Group,
    Context,
    Size,
    SizeValue,
    Date,
    Name,
    INode,
    Links,
}

impl TryFrom<&str> for Block {
    type Error = String;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        match string {
            "permission" => Ok(Self::Permission),
            "user" => Ok(Self::User),
            "group" => Ok(Self::Group),
            "context" => Ok(Self::Context),
            "size" => Ok(Self::Size),
            "size_value" => Ok(Self::SizeValue),
            "date" => Ok(Self::Date),
            "name" => Ok(Self::Name),
            "inode" => Ok(Self::INode),
            "links" => Ok(Self::Links),
            _ => Err(format!("Not a valid block name: {}", &string)),
        }
    }
}

#[cfg(test)]
mod test_blocks {
    use super::Block;
    use super::Blocks;

    use crate::app;
    use crate::config_file::Config;

    use clap::Error;

    // The following tests are implemented using match expressions instead of the assert_eq macro,
    // because clap::Error does not implement PartialEq.

    macro_rules! assert_eq_ok {
        ($left:expr, $right:expr) => {
            assert!(
                match &$left {
                    Ok(inner) if inner == $right.as_ref().unwrap() => true,
                    _ => false,
                },
                "\nComparison failed:\nWas:       {:?}\nShould be: {:?}\n",
                &$left,
                &$right
            )
        };
    }

    #[test]
    fn test_configure_from_without_long() {
        let argv = vec!["lsd"];
        let target = Ok::<_, Error>(Blocks::default());

        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let result = Blocks::configure_from(&matches, &Config::with_none());

        assert_eq_ok!(result, target);
    }

    #[test]
    fn test_configure_from_with_long() {
        let argv = vec!["lsd", "--long"];
        let target = Ok::<_, Error>(Blocks::long());

        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let result = Blocks::configure_from(&matches, &Config::with_none());

        assert_eq_ok!(result, target);
    }

    #[test]
    fn test_configure_from_with_blocks_and_without_long() {
        let argv = vec!["lsd", "--blocks", "permission"];
        let target = Ok::<_, Error>(Blocks(vec![Block::Permission]));

        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let result = Blocks::configure_from(&matches, &Config::with_none());

        assert_eq_ok!(result, target);
    }

    #[test]
    fn test_configure_from_with_blocks_and_long() {
        let argv = vec!["lsd", "--long", "--blocks", "permission"];
        let target = Ok::<_, Error>(Blocks(vec![Block::Permission]));

        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let result = Blocks::configure_from(&matches, &Config::with_none());

        assert_eq_ok!(result, target);
    }

    #[test]
    fn test_configure_from_with_inode() {
        let argv = vec!["lsd", "--inode"];

        let mut target_blocks = Blocks::default();
        target_blocks.0.insert(0, Block::INode);
        let target = Ok::<_, Error>(target_blocks);

        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let result = Blocks::configure_from(&matches, &Config::with_none());

        assert_eq_ok!(result, target);
    }

    #[test]
    fn test_configure_from_prepend_inode_without_long() {
        let argv = vec!["lsd", "--blocks", "permission", "--inode"];

        let mut target_blocks = Blocks(vec![Block::Permission]);
        target_blocks.0.insert(0, Block::INode);
        let target = Ok::<_, Error>(target_blocks);

        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let result = Blocks::configure_from(&matches, &Config::with_none());

        assert_eq_ok!(result, target);
    }

    #[test]
    fn test_configure_from_prepend_inode_with_long() {
        let argv = vec!["lsd", "--long", "--blocks", "permission", "--inode"];
        let target = Ok::<_, Error>(Blocks(vec![Block::INode, Block::Permission]));

        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let result = Blocks::configure_from(&matches, &Config::with_none());

        assert_eq_ok!(result, target);
    }

    #[test]
    fn test_configure_from_ignore_prepend_inode_without_long() {
        let argv = vec!["lsd", "--blocks", "permission,inode", "--inode"];

        let target = Ok::<_, Error>(Blocks(vec![Block::Permission, Block::INode]));

        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let result = Blocks::configure_from(&matches, &Config::with_none());

        assert_eq_ok!(result, target);
    }

    #[test]
    fn test_configure_from_ignore_prepend_inode_with_long() {
        let argv = vec!["lsd", "--long", "--blocks", "permission,inode", "--inode"];
        let target = Ok::<_, Error>(Blocks(vec![Block::Permission, Block::INode]));

        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let result = Blocks::configure_from(&matches, &Config::with_none());

        assert_eq_ok!(result, target);
    }

    #[test]
    fn test_from_arg_matches_none() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert!(match Blocks::from_arg_matches(&matches) {
            None => true,
            _ => false,
        });
    }

    #[test]
    fn test_from_arg_matches_one() {
        let argv = vec!["lsd", "--blocks", "permission"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let test_blocks = Blocks(vec![Block::Permission]);
        assert!(match Blocks::from_arg_matches(&matches) {
            Some(Ok(blocks)) if blocks == test_blocks => true,
            _ => false,
        });
    }

    #[test]
    fn test_from_arg_matches_multi_occurences() {
        let argv = vec!["lsd", "--blocks", "permission", "--blocks", "name"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let test_blocks = Blocks(vec![Block::Permission, Block::Name]);
        assert!(match Blocks::from_arg_matches(&matches) {
            Some(Ok(blocks)) if blocks == test_blocks => true,
            _ => false,
        });
    }

    #[test]
    fn test_from_arg_matches_multi_values() {
        let argv = vec!["lsd", "--blocks", "permission,name"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let test_blocks = Blocks(vec![Block::Permission, Block::Name]);
        assert!(match Blocks::from_arg_matches(&matches) {
            Some(Ok(blocks)) if blocks == test_blocks => true,
            _ => false,
        });
    }

    #[test]
    fn test_from_arg_matches_reversed_default() {
        let argv = vec!["lsd", "--blocks", "name,date,size,group,user,permission"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let test_blocks = Blocks(vec![
            Block::Name,
            Block::Date,
            Block::Size,
            Block::Group,
            Block::User,
            Block::Permission,
        ]);
        assert!(match Blocks::from_arg_matches(&matches) {
            Some(Ok(blocks)) if blocks == test_blocks => true,
            _ => false,
        });
    }

    #[test]
    fn test_from_arg_matches_every_second_one() {
        let argv = vec!["lsd", "--blocks", "permission,group,date"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let test_blocks = Blocks(vec![Block::Permission, Block::Group, Block::Date]);
        assert!(match Blocks::from_arg_matches(&matches) {
            Some(Ok(blocks)) if blocks == test_blocks => true,
            _ => false,
        });
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, Blocks::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_one() {
        let mut c = Config::with_none();
        c.blocks = Some(vec!["permission".into()].into());

        let blocks = Blocks(vec![Block::Permission]);
        assert_eq!(Some(blocks), Blocks::from_config(&c));
    }

    #[test]
    fn test_from_config_reversed_default() {
        let target = Blocks(vec![
            Block::Name,
            Block::Date,
            Block::Size,
            Block::Group,
            Block::User,
            Block::Permission,
        ]);
        let mut c = Config::with_none();
        c.blocks = Some(
            vec![
                "name".into(),
                "date".into(),
                "size".into(),
                "group".into(),
                "user".into(),
                "permission".into(),
            ]
            .into(),
        );

        assert_eq!(Some(target), Blocks::from_config(&c));
    }

    #[test]
    fn test_from_config_every_second_one() {
        let mut c = Config::with_none();
        c.blocks = Some(vec!["permission".into(), "group".into(), "date".into()].into());
        let blocks = Blocks(vec![Block::Permission, Block::Group, Block::Date]);
        assert_eq!(Some(blocks), Blocks::from_config(&c));
    }

    #[test]
    fn test_from_config_invalid_is_ignored() {
        let mut c = Config::with_none();
        c.blocks = Some(vec!["permission".into(), "foo".into(), "date".into()].into());
        let blocks = Blocks(vec![Block::Permission, Block::Date]);
        assert_eq!(Some(blocks), Blocks::from_config(&c));
    }

    #[test]
    fn test_context_not_present_on_cli() {
        let argv = vec!["lsd", "--long"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let parsed_blocks = Blocks::configure_from(&matches, &Config::with_none()).unwrap();
        let it = parsed_blocks.0.iter();
        assert_eq!(it.filter(|&x| *x == Block::Context).count(), 0);
    }

    #[test]
    fn test_context_present_if_context_on() {
        let argv = vec!["lsd", "--context"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let parsed_blocks = Blocks::configure_from(&matches, &Config::with_none()).unwrap();
        let it = parsed_blocks.0.iter();
        assert_eq!(it.filter(|&x| *x == Block::Context).count(), 1);
    }

    #[test]
    fn test_only_one_context_no_other_blocks_affected() {
        let argv = vec![
            "lsd",
            "--context",
            "--blocks",
            "name,date,size,context,group,user,permission",
        ];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let test_blocks = Blocks(vec![
            Block::Name,
            Block::Date,
            Block::Size,
            Block::Context,
            Block::Group,
            Block::User,
            Block::Permission,
        ]);
        let parsed_blocks = Blocks::from_arg_matches(&matches).unwrap().unwrap();
        assert_eq!(test_blocks, parsed_blocks);
    }
}

#[cfg(test)]
mod test_block {
    use super::Block;

    use std::convert::TryFrom;

    #[test]
    fn test_err() {
        assert_eq!(
            Err(String::from("Not a valid block name: foo")),
            Block::try_from("foo")
        );
    }

    #[test]
    fn test_permission() {
        assert_eq!(Ok(Block::Permission), Block::try_from("permission"));
    }

    #[test]
    fn test_user() {
        assert_eq!(Ok(Block::User), Block::try_from("user"));
    }

    #[test]
    fn test_group() {
        assert_eq!(Ok(Block::Group), Block::try_from("group"));
    }

    #[test]
    fn test_size() {
        assert_eq!(Ok(Block::Size), Block::try_from("size"));
    }

    #[test]
    fn test_size_value() {
        assert_eq!(Ok(Block::SizeValue), Block::try_from("size_value"));
    }

    #[test]
    fn test_date() {
        assert_eq!(Ok(Block::Date), Block::try_from("date"));
    }

    #[test]
    fn test_name() {
        assert_eq!(Ok(Block::Name), Block::try_from("name"));
    }

    #[test]
    fn test_inode() {
        assert_eq!(Ok(Block::INode), Block::try_from("inode"));
    }

    #[test]
    fn test_links() {
        assert_eq!(Ok(Block::Links), Block::try_from("links"));
    }

    #[test]
    fn test_context() {
        assert_eq!(Ok(Block::Context), Block::try_from("context"));
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::cmp::Ord;
	use std::default::Default;
	use std::clone::Clone;
	use std::cmp::PartialEq;
	use std::cmp::PartialOrd;
	use std::convert::TryFrom;
	use std::cmp::Eq;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5357() {
    rusty_monitor::set_test_id(5357);
    let mut usize_0: usize = 1usize;
    let mut bool_0: bool = false;
    let mut u64_0: u64 = 23u64;
    let mut usize_1: usize = 81usize;
    let mut bool_1: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_1, depth: usize_1};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_2: bool = true;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut option_3: std::option::Option<bool> = std::option::Option::None;
    let mut option_4: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut option_5: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_0);
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut option_6: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_0);
    let mut option_7: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut option_8: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_1);
    let mut option_9: std::option::Option<bool> = std::option::Option::None;
    let mut option_10: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_11: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_12: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut option_13: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_1);
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_13, theme: option_12, separator: option_11};
    let mut option_14: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut display_0: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut option_15: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut option_16: std::option::Option<bool> = std::option::Option::None;
    let mut option_17: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_18: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_19: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_3: bool = true;
    let mut option_20: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_20, blocks: option_19, color: option_18, date: option_17, dereference: option_16, display: option_15, icons: option_14, ignore_globs: option_10, indicators: option_9, layout: option_8, recursion: option_7, size: option_6, permission: option_5, sorting: option_4, no_symlink: option_3, total_size: option_2, symlink_arrow: option_1, hyperlink: option_0};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut display_1: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut blocks_0: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut u64_1: u64 = 25u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut bool_4: bool = false;
    let mut bool_5: bool = false;
    let mut bool_6: bool = false;
    let mut bool_7: bool = true;
    let mut bool_8: bool = false;
    let mut bool_9: bool = false;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut bool_12: bool = false;
    let mut bool_13: bool = true;
    let mut bool_14: bool = false;
    let mut bool_15: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_15, user_write: bool_14, user_execute: bool_13, group_read: bool_12, group_write: bool_11, group_execute: bool_10, other_read: bool_9, other_write: bool_8, other_execute: bool_7, sticky: bool_6, setgid: bool_5, setuid: bool_4};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_21: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::INode;
    let mut block_0_ref_0: &flags::blocks::Block = &mut block_0;
    let mut option_22: std::option::Option<usize> = std::option::Option::None;
    let mut option_23: std::option::Option<bool> = std::option::Option::None;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Giga;
    let mut unit_0_ref_0: &meta::size::Unit = &mut unit_0;
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::Giga;
    let mut unit_1_ref_0: &meta::size::Unit = &mut unit_1;
    let mut block_1: flags::blocks::Block = crate::flags::blocks::Block::INode;
    let mut option_24: std::option::Option<std::path::PathBuf> = crate::config_file::Config::config_file_path();
    let mut recursion_1: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_23, depth: option_22};
    let mut block_1_ref_0: &flags::blocks::Block = &mut block_1;
    let mut option_25: std::option::Option<std::cmp::Ordering> = crate::flags::blocks::Block::partial_cmp(block_1_ref_0, block_0_ref_0);
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    crate::meta::filetype::FileType::render(filetype_0, colors_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3534() {
    rusty_monitor::set_test_id(3534);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_0: usize = 96usize;
    let mut bool_0: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut blocks_0: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut u64_0: u64 = 91u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut blocks_1: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_1_ref_0: &mut crate::flags::blocks::Blocks = &mut blocks_1;
    let mut elem_0: color::Elem = crate::color::Elem::User;
    crate::flags::blocks::Blocks::prepend_inode(blocks_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2201() {
    rusty_monitor::set_test_id(2201);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Name;
    let mut block_0_ref_0: &flags::blocks::Block = &mut block_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::NonFile;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Write;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut option_0: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_0: &str = "TwuXE";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_1: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut option_2: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut option_3: std::option::Option<crate::color::theme::Theme> = std::option::Option::None;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut block_1: flags::blocks::Block = crate::flags::blocks::Block::Links;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut symlink_0: crate::color::theme::Symlink = crate::color::theme::Symlink {default: color_2, broken: color_1, missing_target: color_0};
    let mut block_1_ref_0: &flags::blocks::Block = &mut block_1;
    let mut option_4: std::option::Option<std::cmp::Ordering> = crate::flags::blocks::Block::partial_cmp(block_1_ref_0, block_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4178() {
    rusty_monitor::set_test_id(4178);
    let mut str_0: &str = "Q7emkguhP0";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::SymLink;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut bool_0: bool = true;
    let mut elem_1: color::Elem = crate::color::Elem::Dir {uid: bool_0};
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_2, exec: bool_1};
    let mut blocks_0: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_0_ref_0: &crate::flags::blocks::Blocks = &mut blocks_0;
    let mut elem_3: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut option_0: std::option::Option<usize> = std::option::Option::None;
    let mut option_1: std::option::Option<usize> = std::option::Option::None;
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_4: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_5: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_4_ref_0);
    let mut str_1: &str = "LJuheCbToP4A";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut option_2: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_1_ref_0);
    let mut dir_0: crate::color::theme::Dir = crate::color::theme::Dir {uid: color_4, no_uid: color_3};
    let mut bool_3: bool = crate::color::Elem::has_suid(elem_3_ref_0);
    let mut blocks_1: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut bool_4: bool = crate::flags::blocks::Blocks::contains_inode(blocks_0_ref_0);
    let mut elem_6: color::Elem = crate::color::Elem::Socket;
    let mut elem_7: color::Elem = crate::color::Elem::DayOld;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut displayoption_0: meta::name::DisplayOption = crate::meta::name::DisplayOption::FileName;
    let mut date_0: crate::color::theme::Date = crate::color::theme::Date {hour_old: color_2, day_old: color_1, older: color_0};
    let mut date_0_ref_0: &crate::color::theme::Date = &mut date_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut result_0: std::result::Result<flags::blocks::Block, std::string::String> = crate::flags::blocks::Block::try_from(str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_113() {
    rusty_monitor::set_test_id(113);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut bool_0: bool = true;
    let mut elem_0: color::Elem = crate::color::Elem::Dir {uid: bool_0};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Group;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::Read;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut elem_4: color::Elem = crate::color::Elem::Special;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Context;
    let mut block_0_ref_0: &flags::blocks::Block = &mut block_0;
    let mut displayoption_0: meta::name::DisplayOption = crate::meta::name::DisplayOption::None;
    let mut block_1: flags::blocks::Block = crate::flags::blocks::Block::Group;
    let mut block_1_ref_0: &flags::blocks::Block = &mut block_1;
    let mut block_2: flags::blocks::Block = crate::flags::blocks::Block::clone(block_1_ref_0);
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut file_0: crate::color::theme::File = crate::color::theme::File {exec_uid: color_3, uid_no_exec: color_2, exec_no_uid: color_1, no_exec_no_uid: color_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2529() {
    rusty_monitor::set_test_id(2529);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_0: usize = 20usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut bool_1: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_1};
    let mut bool_2: bool = false;
    let mut bool_3: bool = true;
    let mut bool_4: bool = false;
    let mut bool_5: bool = false;
    let mut bool_6: bool = false;
    let mut bool_7: bool = true;
    let mut bool_8: bool = false;
    let mut bool_9: bool = false;
    let mut bool_10: bool = true;
    let mut bool_11: bool = true;
    let mut bool_12: bool = true;
    let mut bool_13: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_13, user_write: bool_12, user_execute: bool_11, group_read: bool_10, group_write: bool_9, group_execute: bool_8, other_read: bool_7, other_write: bool_6, other_execute: bool_5, sticky: bool_4, setgid: bool_3, setuid: bool_2};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut str_0: &str = "i3xfNY0";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut u64_0: u64 = 16u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut u64_1: u64 = 33u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_2};
    let mut config_2: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut blocks_0: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_1: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_1_ref_0: &crate::flags::blocks::Blocks = &mut blocks_1;
    let mut tuple_0: () = crate::flags::blocks::Blocks::assert_receiver_is_total_eq(blocks_1_ref_0);
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_954() {
    rusty_monitor::set_test_id(954);
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_0: usize = 52usize;
    let mut bool_0: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut blocks_0: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut u64_0: u64 = 53u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_1, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_1: usize = 63usize;
    let mut bool_1: bool = false;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_1, depth: usize_1};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_1: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_2};
    let mut blocks_1: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut str_0: &str = "Gz";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut option_0: std::option::Option<bool> = std::option::Option::None;
    let mut bool_2: bool = true;
    let mut option_1: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut option_2: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut option_3: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut sizeflag_2: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut option_4: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_2);
    let mut option_5: std::option::Option<usize> = std::option::Option::None;
    let mut option_6: std::option::Option<bool> = std::option::Option::None;
    let mut recursion_2: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_6, depth: option_5};
    let mut option_7: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_2);
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut option_8: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_2);
    let mut option_9: std::option::Option<bool> = std::option::Option::None;
    let mut option_10: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_11: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_2: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut option_12: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_2);
    let mut iconoption_2: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut option_13: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_2);
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_13, theme: option_12, separator: option_11};
    let mut option_14: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut display_2: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut option_15: std::option::Option<flags::display::Display> = std::option::Option::Some(display_2);
    let mut option_16: std::option::Option<bool> = std::option::Option::None;
    let mut option_17: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_18: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut coloroption_2: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut option_19: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_2);
    let mut color_2: crate::config_file::Color = crate::config_file::Color {when: option_19, theme: option_18};
    let mut option_20: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_2);
    let mut option_21: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_3: bool = false;
    let mut option_22: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut option_23: std::option::Option<crate::flags::blocks::Blocks> = crate::flags::blocks::Blocks::from_config(config_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4201() {
    rusty_monitor::set_test_id(4201);
    let mut blocks_0: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_0_ref_0: &mut crate::flags::blocks::Blocks = &mut blocks_0;
    let mut bool_0: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_0};
    let mut bool_1: bool = true;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Links;
    let mut block_0_ref_0: &flags::blocks::Block = &mut block_0;
    let mut str_0: &str = "2ggwX3I1l8";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut option_0: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_0);
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut option_1: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Context;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::ExecSticky;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::ExecSticky;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut filetype_1_ref_0: &meta::filetype::FileType = &mut filetype_1;
    let mut block_1: flags::blocks::Block = crate::flags::blocks::Block::Date;
    let mut block_2: flags::blocks::Block = crate::flags::blocks::Block::Links;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut theme_3: icon::Theme = crate::icon::Theme::Fancy;
    let mut display_1: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut symlink_0: crate::color::theme::Symlink = crate::color::theme::Symlink {default: color_2, broken: color_1, missing_target: color_0};
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut sorting_0_ref_0: &crate::flags::sorting::Sorting = &mut sorting_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut color_3: crate::config_file::Color = crate::config_file::Color {when: option_1, theme: option_0};
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Byte;
    let mut option_2: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut elem_3: color::Elem = crate::color::Elem::Acl;
    let mut display_2: flags::display::Display = crate::flags::display::Display::All;
    let mut contentstyle_0: crossterm::style::ContentStyle = crate::color::Colors::default_style();
    let mut block_2_ref_0: &flags::blocks::Block = &mut block_2;
    let mut bool_2: bool = crate::flags::blocks::Block::eq(block_2_ref_0, block_0_ref_0);
    let mut elem_4: color::Elem = crate::color::Elem::Dir {uid: bool_1};
    crate::flags::blocks::Blocks::optional_prepend_inode(blocks_0_ref_0);
    let mut elem_5: color::Elem = crate::color::Elem::Read;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2758() {
    rusty_monitor::set_test_id(2758);
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut bool_2: bool = true;
    let mut bool_3: bool = false;
    let mut elem_0: color::Elem = crate::color::Elem::File {exec: bool_3, uid: bool_2};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Permission;
    let mut block_0_ref_0: &flags::blocks::Block = &mut block_0;
    let mut elem_1: color::Elem = crate::color::Elem::Exec;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut block_1: flags::blocks::Block = crate::flags::blocks::Block::Permission;
    let mut block_1_ref_0: &flags::blocks::Block = &mut block_1;
    let mut elem_2: color::Elem = crate::color::Elem::User;
    let mut block_2: flags::blocks::Block = crate::flags::blocks::Block::SizeValue;
    let mut block_2_ref_0: &flags::blocks::Block = &mut block_2;
    let mut option_0: std::option::Option<std::cmp::Ordering> = crate::flags::blocks::Block::partial_cmp(block_2_ref_0, block_1_ref_0);
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut app_0: clap::App = crate::app::build();
    let mut elem_3: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    crate::meta::filetype::FileType::render(filetype_0, colors_1_ref_0);
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_1_ref_0);
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_1, exec: bool_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3064() {
    rusty_monitor::set_test_id(3064);
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut bool_3: bool = false;
    let mut bool_4: bool = false;
    let mut bool_5: bool = false;
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = false;
    let mut bool_10: bool = false;
    let mut bool_11: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut usize_0: usize = 45usize;
    let mut option_1: std::option::Option<usize> = std::option::Option::Some(usize_0);
    let mut bool_12: bool = true;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_12);
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_2, depth: option_1};
    let mut recursion_0_ref_0: &crate::config_file::Recursion = &mut recursion_0;
    let mut u64_0: u64 = 90u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut usize_1: usize = 62usize;
    let mut option_3: std::option::Option<usize> = std::option::Option::Some(usize_1);
    let mut bool_13: bool = true;
    let mut option_4: std::option::Option<bool> = std::option::Option::Some(bool_13);
    let mut blocks_0: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_0_ref_0: &crate::flags::blocks::Blocks = &mut blocks_0;
    let mut blocks_1: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_1_ref_0: &crate::flags::blocks::Blocks = &mut blocks_1;
    let mut bool_14: bool = crate::flags::blocks::Blocks::ne(blocks_1_ref_0, blocks_0_ref_0);
    let mut recursion_1: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_4, depth: option_3};
    let mut u64_1: u64 = crate::meta::size::Size::get_bytes(size_0_ref_0);
    let mut displayoption_0: meta::name::DisplayOption = crate::meta::name::DisplayOption::FileName;
    let mut recursion_1_ref_0: &crate::config_file::Recursion = &mut recursion_1;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut blocks_2: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4339() {
    rusty_monitor::set_test_id(4339);
    let mut blocks_0: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_0_ref_0: &crate::flags::blocks::Blocks = &mut blocks_0;
    let mut blocks_1: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_1_ref_0: &crate::flags::blocks::Blocks = &mut blocks_1;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut bool_0: bool = true;
    let mut elem_1: color::Elem = crate::color::Elem::Links {valid: bool_0};
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut str_0: &str = "rQDu8eCYjT";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_0: usize = 22usize;
    let mut bool_1: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_1, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_2: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut blocks_2: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut u64_0: u64 = 36u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut blocks_3: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_3_ref_0: &mut crate::flags::blocks::Blocks = &mut blocks_3;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    crate::flags::blocks::Blocks::prepend_inode(blocks_3_ref_0);
    let mut option_0: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut hyperlinkoption_0_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_0;
    let mut elem_2: color::Elem = crate::color::Elem::SymLink;
    let mut elem_3: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Group;
    let mut dir_0: crate::color::theme::Dir = crate::color::theme::Dir {uid: color_1, no_uid: color_0};
    let mut bool_2: bool = crate::flags::blocks::Blocks::eq(blocks_1_ref_0, blocks_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5471() {
    rusty_monitor::set_test_id(5471);
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_0: usize = 28usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut blocks_0: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_1: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_1_ref_0: &mut crate::flags::blocks::Blocks = &mut blocks_1;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut config_2: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_1: usize = 9usize;
    let mut bool_1: bool = true;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_1, depth: usize_1};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut config_3: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_3_ref_0: &crate::config_file::Config = &mut config_3;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_1: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_1};
    let mut blocks_2: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_3: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_3_ref_0: &crate::flags::blocks::Blocks = &mut blocks_3;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Date;
    let mut block_0_ref_0: &flags::blocks::Block = &mut block_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut bool_2: bool = true;
    let mut elem_0: color::Elem = crate::color::Elem::Links {valid: bool_2};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut elem_1: color::Elem = crate::color::Elem::NonFile;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_2: color::Elem = crate::color::Elem::SymLink;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_3: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut bool_3: bool = false;
    let mut elem_4: color::Elem = crate::color::Elem::Dir {uid: bool_3};
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_5: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_2_ref_0);
    let mut elem_6: color::Elem = crate::color::Elem::Acl;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut elem_7: color::Elem = crate::color::Elem::Pipe;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut block_1: flags::blocks::Block = crate::flags::blocks::Block::Group;
    let mut block_1_ref_0: &flags::blocks::Block = &mut block_1;
    let mut bool_4: bool = crate::flags::blocks::Block::eq(block_1_ref_0, block_0_ref_0);
    let mut block_2: flags::blocks::Block = crate::flags::blocks::Block::Group;
    let mut block_3: flags::blocks::Block = crate::flags::blocks::Block::Group;
    let mut tuple_0: () = crate::flags::blocks::Blocks::assert_receiver_is_total_eq(blocks_3_ref_0);
    crate::flags::blocks::Blocks::optional_insert_context(blocks_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1690() {
    rusty_monitor::set_test_id(1690);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Name;
    let mut block_0_ref_0: &flags::blocks::Block = &mut block_0;
    let mut block_1: flags::blocks::Block = crate::flags::blocks::Block::Name;
    let mut block_1_ref_0: &flags::blocks::Block = &mut block_1;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Pipe;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::User;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::ExecSticky;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut str_0: &str = "7eSHoF";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut block_2: flags::blocks::Block = crate::flags::blocks::Block::Context;
    let mut block_3: flags::blocks::Block = crate::flags::blocks::Block::Context;
    let mut elem_3: color::Elem = crate::color::Elem::Dir {uid: bool_1};
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_0};
    let mut date_0: crate::color::theme::Date = crate::color::theme::Date {hour_old: color_2, day_old: color_1, older: color_0};
    let mut block_2_ref_0: &flags::blocks::Block = &mut block_2;
    let mut bool_2: bool = crate::flags::blocks::Block::eq(block_2_ref_0, block_1_ref_0);
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut block_3_ref_0: &flags::blocks::Block = &mut block_3;
    let mut ordering_0: std::cmp::Ordering = crate::flags::blocks::Block::cmp(block_3_ref_0, block_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2655() {
    rusty_monitor::set_test_id(2655);
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut bool_5: bool = true;
    let mut bool_6: bool = false;
    let mut bool_7: bool = false;
    let mut bool_8: bool = true;
    let mut bool_9: bool = true;
    let mut bool_10: bool = true;
    let mut bool_11: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Acl;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Write;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::DayOld;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Group;
    let mut block_0_ref_0: &flags::blocks::Block = &mut block_0;
    let mut tuple_0: () = crate::flags::blocks::Block::assert_receiver_is_total_eq(block_0_ref_0);
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Kilo;
    let mut elem_3: color::Elem = crate::color::Elem::SymLink;
    let mut symlink_0: crate::color::theme::Symlink = crate::color::theme::Symlink {default: color_2, broken: color_1, missing_target: color_0};
    let mut bool_12: bool = crate::meta::permissions::Permissions::is_executable(permissions_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_528() {
    rusty_monitor::set_test_id(528);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Context;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Special;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut bool_0: bool = false;
    let mut elem_2: color::Elem = crate::color::Elem::INode {valid: bool_0};
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::Older;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut bool_1: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_1};
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::INode;
    let mut block_0_ref_0: &flags::blocks::Block = &mut block_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut option_0: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_0);
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut option_1: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut blocks_0: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_0_ref_0: &crate::flags::blocks::Blocks = &mut blocks_0;
    let mut block_1: flags::blocks::Block = crate::flags::blocks::Block::Context;
    let mut block_1_ref_0: &flags::blocks::Block = &mut block_1;
    let mut usize_0: usize = 56usize;
    let mut option_2: std::option::Option<usize> = std::option::Option::Some(usize_0);
    let mut usize_1: usize = 48usize;
    let mut option_3: std::option::Option<usize> = std::option::Option::Some(usize_1);
    let mut usize_2: usize = std::option::Option::unwrap(option_3);
    let mut usize_3: usize = std::option::Option::unwrap(option_2);
    let mut elem_4: color::Elem = crate::color::Elem::DayOld;
    let mut block_2: flags::blocks::Block = crate::flags::blocks::Block::Links;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut block_2_ref_0: &flags::blocks::Block = &mut block_2;
    let mut option_4: std::option::Option<std::cmp::Ordering> = crate::flags::blocks::Block::partial_cmp(block_2_ref_0, block_1_ref_0);
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut coloroption_2: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Kilo;
    let mut bool_2: bool = crate::flags::blocks::Blocks::displays_size(blocks_0_ref_0);
    let mut color_4: crate::config_file::Color = crate::config_file::Color {when: option_1, theme: option_0};
    let mut color_4_ref_0: &crate::config_file::Color = &mut color_4;
    let mut tuple_0: () = crate::flags::blocks::Block::assert_receiver_is_total_eq(block_0_ref_0);
    let mut block_3: flags::blocks::Block = crate::flags::blocks::Block::User;
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut file_0: crate::color::theme::File = crate::color::theme::File {exec_uid: color_3, uid_no_exec: color_2, exec_no_uid: color_1, no_exec_no_uid: color_0};
    let mut elem_5: color::Elem = crate::color::Elem::Octal;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3601() {
    rusty_monitor::set_test_id(3601);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Name;
    let mut block_0_ref_0: &flags::blocks::Block = &mut block_0;
    let mut elem_0: color::Elem = crate::color::Elem::Group;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_1: color::Elem = crate::color::Elem::Read;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_2: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_0_ref_0);
    let mut elem_3: color::Elem = crate::color::Elem::Older;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut elem_4: color::Elem = crate::color::Elem::SymLink;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut elem_5: color::Elem = crate::color::Elem::Read;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut block_1: flags::blocks::Block = crate::flags::blocks::Block::Size;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut block_1_ref_0: &flags::blocks::Block = &mut block_1;
    let mut bool_0: bool = crate::flags::blocks::Block::eq(block_1_ref_0, block_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1244() {
    rusty_monitor::set_test_id(1244);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_1);
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_0: bool = false;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut bool_1: bool = true;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_4: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut option_5: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut option_6: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_7: std::option::Option<usize> = std::option::Option::None;
    let mut option_8: std::option::Option<bool> = std::option::Option::None;
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_8, depth: option_7};
    let mut option_9: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_0);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut option_10: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_0);
    let mut bool_2: bool = true;
    let mut option_11: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut option_12: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_13: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_14: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut bool_3: bool = false;
    let mut option_15: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut option_16: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_17: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut option_18: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_18, theme: option_17};
    let mut option_19: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_20: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_4: bool = true;
    let mut option_21: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_21, blocks: option_20, color: option_19, date: option_16, dereference: option_15, display: option_14, icons: option_13, ignore_globs: option_12, indicators: option_11, layout: option_10, recursion: option_9, size: option_6, permission: option_5, sorting: option_4, no_symlink: option_3, total_size: option_2, symlink_arrow: option_1, hyperlink: option_0};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_0: usize = 40usize;
    let mut bool_5: bool = true;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_5, depth: usize_0};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut display_0: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut blocks_0: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Links;
    let mut block_0_ref_0: &flags::blocks::Block = &mut block_0;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut icontheme_1_ref_0: &flags::icons::IconTheme = &mut icontheme_1;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::NonFile;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut u64_0: u64 = 30u64;
    let mut links_0: crate::color::theme::Links = crate::color::theme::Links {valid: color_3, invalid: color_2};
    let mut icontheme_2: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut icontheme_2_ref_0: &flags::icons::IconTheme = &mut icontheme_2;
    let mut block_1: flags::blocks::Block = crate::flags::blocks::Block::Links;
    let mut block_1_ref_0: &flags::blocks::Block = &mut block_1;
    let mut ordering_0: std::cmp::Ordering = crate::flags::blocks::Block::cmp(block_1_ref_0, block_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1329() {
    rusty_monitor::set_test_id(1329);
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut blocks_0: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_0_ref_0: &mut crate::flags::blocks::Blocks = &mut blocks_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_0: usize = 34usize;
    let mut bool_0: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut blocks_1: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut option_0: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut str_0: &str = "IX4U4";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "L5unZYxpT";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut option_1: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut usize_1: usize = 39usize;
    let mut option_2: std::option::Option<usize> = std::option::Option::Some(usize_1);
    let mut option_3: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_4: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut option_5: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_2);
    let mut str_2: &str = "JwyJNR";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut option_6: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_2: usize = 80usize;
    let mut bool_1: bool = true;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_1, depth: usize_2};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut iconoption_2: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_1: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut blocks_2: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut sizeflag_2: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    crate::flags::blocks::Blocks::optional_insert_context(blocks_0_ref_0);
    let mut sizeflag_3: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut elem_0: color::Elem = crate::color::Elem::BlockDevice;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut elem_1: color::Elem = crate::color::Elem::Acl;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut displayoption_0: meta::name::DisplayOption = crate::meta::name::DisplayOption::FileName;
    let mut elem_2: color::Elem = crate::color::Elem::FileLarge;
    let mut permissionflag_2: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut permissionflag_3: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut permissionflag_2_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_2;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut theme_1: icon::Theme = crate::icon::Theme::Fancy;
    panic!("From RustyUnit with love");
}
}