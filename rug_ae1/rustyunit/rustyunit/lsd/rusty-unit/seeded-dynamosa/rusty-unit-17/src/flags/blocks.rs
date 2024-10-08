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
	use std::cmp::PartialEq;
	use std::clone::Clone;
	use std::cmp::PartialOrd;
	use std::convert::TryFrom;
	use std::cmp::Eq;
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_380() {
//    rusty_monitor::set_test_id(380);
    let mut blocks_0: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_0_ref_0: &crate::flags::blocks::Blocks = &mut blocks_0;
    let mut blocks_1: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_1_ref_0: &crate::flags::blocks::Blocks = &mut blocks_1;
    let mut blocks_2: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_2_ref_0: &crate::flags::blocks::Blocks = &mut blocks_2;
    let mut blocks_3: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_3_ref_0: &crate::flags::blocks::Blocks = &mut blocks_3;
    let mut blocks_4: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_4_ref_0: &crate::flags::blocks::Blocks = &mut blocks_4;
    let mut blocks_5: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_5_ref_0: &crate::flags::blocks::Blocks = &mut blocks_5;
    let mut blocks_6: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_6_ref_0: &crate::flags::blocks::Blocks = &mut blocks_6;
    let mut blocks_7: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_7_ref_0: &crate::flags::blocks::Blocks = &mut blocks_7;
    let mut blocks_8: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_8_ref_0: &crate::flags::blocks::Blocks = &mut blocks_8;
    let mut blocks_9: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_9_ref_0: &crate::flags::blocks::Blocks = &mut blocks_9;
    let mut blocks_10: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_10_ref_0: &crate::flags::blocks::Blocks = &mut blocks_10;
    let mut blocks_11: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_11_ref_0: &crate::flags::blocks::Blocks = &mut blocks_11;
    let mut blocks_12: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_12_ref_0: &crate::flags::blocks::Blocks = &mut blocks_12;
    let mut blocks_13: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_13_ref_0: &crate::flags::blocks::Blocks = &mut blocks_13;
    let mut bool_0: bool = crate::flags::blocks::Blocks::ne(blocks_13_ref_0, blocks_12_ref_0);
    let mut bool_1: bool = crate::flags::blocks::Blocks::ne(blocks_11_ref_0, blocks_10_ref_0);
    let mut bool_2: bool = crate::flags::blocks::Blocks::ne(blocks_9_ref_0, blocks_8_ref_0);
    let mut bool_3: bool = crate::flags::blocks::Blocks::ne(blocks_7_ref_0, blocks_6_ref_0);
    let mut bool_4: bool = crate::flags::blocks::Blocks::ne(blocks_5_ref_0, blocks_4_ref_0);
    let mut bool_5: bool = crate::flags::blocks::Blocks::ne(blocks_3_ref_0, blocks_2_ref_0);
    let mut bool_6: bool = crate::flags::blocks::Blocks::ne(blocks_1_ref_0, blocks_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_214() {
//    rusty_monitor::set_test_id(214);
    let mut blocks_0: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_0_ref_0: &crate::flags::blocks::Blocks = &mut blocks_0;
    let mut blocks_1: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_1_ref_0: &crate::flags::blocks::Blocks = &mut blocks_1;
    let mut blocks_2: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_2_ref_0: &crate::flags::blocks::Blocks = &mut blocks_2;
    let mut blocks_3: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_3_ref_0: &crate::flags::blocks::Blocks = &mut blocks_3;
    let mut blocks_4: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_4_ref_0: &crate::flags::blocks::Blocks = &mut blocks_4;
    let mut blocks_5: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_5_ref_0: &crate::flags::blocks::Blocks = &mut blocks_5;
    let mut blocks_6: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_6_ref_0: &crate::flags::blocks::Blocks = &mut blocks_6;
    let mut blocks_7: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_7_ref_0: &crate::flags::blocks::Blocks = &mut blocks_7;
    let mut blocks_8: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_8_ref_0: &crate::flags::blocks::Blocks = &mut blocks_8;
    let mut blocks_9: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_9_ref_0: &crate::flags::blocks::Blocks = &mut blocks_9;
    let mut blocks_10: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_10_ref_0: &crate::flags::blocks::Blocks = &mut blocks_10;
    let mut tuple_0: () = crate::flags::blocks::Blocks::assert_receiver_is_total_eq(blocks_10_ref_0);
    let mut tuple_1: () = crate::flags::blocks::Blocks::assert_receiver_is_total_eq(blocks_9_ref_0);
    let mut tuple_2: () = crate::flags::blocks::Blocks::assert_receiver_is_total_eq(blocks_8_ref_0);
    let mut tuple_3: () = crate::flags::blocks::Blocks::assert_receiver_is_total_eq(blocks_7_ref_0);
    let mut tuple_4: () = crate::flags::blocks::Blocks::assert_receiver_is_total_eq(blocks_6_ref_0);
    let mut tuple_5: () = crate::flags::blocks::Blocks::assert_receiver_is_total_eq(blocks_5_ref_0);
    let mut tuple_6: () = crate::flags::blocks::Blocks::assert_receiver_is_total_eq(blocks_4_ref_0);
    let mut tuple_7: () = crate::flags::blocks::Blocks::assert_receiver_is_total_eq(blocks_3_ref_0);
    let mut tuple_8: () = crate::flags::blocks::Blocks::assert_receiver_is_total_eq(blocks_2_ref_0);
    let mut tuple_9: () = crate::flags::blocks::Blocks::assert_receiver_is_total_eq(blocks_1_ref_0);
    let mut tuple_10: () = crate::flags::blocks::Blocks::assert_receiver_is_total_eq(blocks_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_857() {
//    rusty_monitor::set_test_id(857);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut blocks_0: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_1: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_1_ref_0: &crate::flags::blocks::Blocks = &mut blocks_1;
    let mut blocks_2: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_2_ref_0: &crate::flags::blocks::Blocks = &mut blocks_2;
    let mut blocks_3: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_3_ref_0: &crate::flags::blocks::Blocks = &mut blocks_3;
    let mut blocks_4: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_4_ref_0: &crate::flags::blocks::Blocks = &mut blocks_4;
    let mut blocks_5: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_5_ref_0: &crate::flags::blocks::Blocks = &mut blocks_5;
    let mut blocks_6: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_0_ref_0: &crate::flags::blocks::Blocks = &mut blocks_0;
    let mut blocks_7: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_7_ref_0: &crate::flags::blocks::Blocks = &mut blocks_7;
    let mut blocks_8: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_8_ref_0: &crate::flags::blocks::Blocks = &mut blocks_8;
    let mut blocks_9: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_9_ref_0: &crate::flags::blocks::Blocks = &mut blocks_9;
    let mut blocks_10: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_10_ref_0: &crate::flags::blocks::Blocks = &mut blocks_10;
    let mut blocks_11: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_11_ref_0: &crate::flags::blocks::Blocks = &mut blocks_11;
    let mut blocks_12: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_12_ref_0: &crate::flags::blocks::Blocks = &mut blocks_12;
    let mut blocks_13: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_13_ref_0: &crate::flags::blocks::Blocks = &mut blocks_13;
    let mut blocks_14: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_14_ref_0: &crate::flags::blocks::Blocks = &mut blocks_14;
    let mut bool_0: bool = crate::flags::blocks::Blocks::ne(blocks_14_ref_0, blocks_13_ref_0);
    let mut bool_1: bool = crate::flags::blocks::Blocks::ne(blocks_12_ref_0, blocks_11_ref_0);
    let mut bool_2: bool = crate::flags::blocks::Blocks::ne(blocks_10_ref_0, blocks_9_ref_0);
    let mut bool_3: bool = crate::flags::blocks::Blocks::ne(blocks_8_ref_0, blocks_7_ref_0);
    let mut bool_4: bool = crate::flags::blocks::Blocks::ne(blocks_0_ref_0, blocks_5_ref_0);
    let mut bool_5: bool = crate::flags::blocks::Blocks::ne(blocks_4_ref_0, blocks_3_ref_0);
    let mut bool_6: bool = crate::flags::blocks::Blocks::ne(blocks_2_ref_0, blocks_1_ref_0);
    crate::meta::filetype::FileType::render(filetype_0, colors_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_475() {
//    rusty_monitor::set_test_id(475);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_0: usize = 40usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_0: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut blocks_0: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut bool_1: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_1};
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    crate::meta::filetype::FileType::render(filetype_1, colors_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_453() {
//    rusty_monitor::set_test_id(453);
    let mut blocks_0: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_0_ref_0: &mut crate::flags::blocks::Blocks = &mut blocks_0;
    let mut blocks_1: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_1_ref_0: &mut crate::flags::blocks::Blocks = &mut blocks_1;
    let mut blocks_2: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_2_ref_0: &mut crate::flags::blocks::Blocks = &mut blocks_2;
    let mut blocks_3: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_3_ref_0: &mut crate::flags::blocks::Blocks = &mut blocks_3;
    let mut blocks_4: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_4_ref_0: &mut crate::flags::blocks::Blocks = &mut blocks_4;
    let mut blocks_5: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_5_ref_0: &mut crate::flags::blocks::Blocks = &mut blocks_5;
    let mut blocks_6: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_6_ref_0: &mut crate::flags::blocks::Blocks = &mut blocks_6;
    let mut blocks_7: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_7_ref_0: &mut crate::flags::blocks::Blocks = &mut blocks_7;
    let mut blocks_8: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_8_ref_0: &mut crate::flags::blocks::Blocks = &mut blocks_8;
    let mut blocks_9: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_9_ref_0: &mut crate::flags::blocks::Blocks = &mut blocks_9;
    let mut blocks_10: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_10_ref_0: &mut crate::flags::blocks::Blocks = &mut blocks_10;
    crate::flags::blocks::Blocks::optional_insert_context(blocks_10_ref_0);
    crate::flags::blocks::Blocks::optional_insert_context(blocks_9_ref_0);
    crate::flags::blocks::Blocks::optional_insert_context(blocks_8_ref_0);
    crate::flags::blocks::Blocks::optional_insert_context(blocks_7_ref_0);
    crate::flags::blocks::Blocks::optional_insert_context(blocks_6_ref_0);
    crate::flags::blocks::Blocks::optional_insert_context(blocks_5_ref_0);
    crate::flags::blocks::Blocks::optional_insert_context(blocks_4_ref_0);
    crate::flags::blocks::Blocks::optional_insert_context(blocks_3_ref_0);
    crate::flags::blocks::Blocks::optional_insert_context(blocks_2_ref_0);
    crate::flags::blocks::Blocks::optional_insert_context(blocks_1_ref_0);
    crate::flags::blocks::Blocks::optional_insert_context(blocks_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_447() {
//    rusty_monitor::set_test_id(447);
    let mut blocks_0: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_0_ref_0: &crate::flags::blocks::Blocks = &mut blocks_0;
    let mut blocks_1: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_1_ref_0: &crate::flags::blocks::Blocks = &mut blocks_1;
    let mut blocks_2: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_2_ref_0: &crate::flags::blocks::Blocks = &mut blocks_2;
    let mut blocks_3: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_3_ref_0: &crate::flags::blocks::Blocks = &mut blocks_3;
    let mut blocks_4: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_4_ref_0: &crate::flags::blocks::Blocks = &mut blocks_4;
    let mut blocks_5: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_5_ref_0: &crate::flags::blocks::Blocks = &mut blocks_5;
    let mut blocks_6: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_6_ref_0: &crate::flags::blocks::Blocks = &mut blocks_6;
    let mut blocks_7: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_7_ref_0: &crate::flags::blocks::Blocks = &mut blocks_7;
    let mut blocks_8: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_8_ref_0: &crate::flags::blocks::Blocks = &mut blocks_8;
    let mut blocks_9: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_9_ref_0: &crate::flags::blocks::Blocks = &mut blocks_9;
    let mut blocks_10: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_10_ref_0: &crate::flags::blocks::Blocks = &mut blocks_10;
    let mut bool_0: bool = crate::flags::blocks::Blocks::displays_size(blocks_10_ref_0);
    let mut bool_1: bool = crate::flags::blocks::Blocks::displays_size(blocks_9_ref_0);
    let mut bool_2: bool = crate::flags::blocks::Blocks::displays_size(blocks_8_ref_0);
    let mut bool_3: bool = crate::flags::blocks::Blocks::displays_size(blocks_7_ref_0);
    let mut bool_4: bool = crate::flags::blocks::Blocks::displays_size(blocks_6_ref_0);
    let mut bool_5: bool = crate::flags::blocks::Blocks::displays_size(blocks_5_ref_0);
    let mut bool_6: bool = crate::flags::blocks::Blocks::displays_size(blocks_4_ref_0);
    let mut bool_7: bool = crate::flags::blocks::Blocks::displays_size(blocks_3_ref_0);
    let mut bool_8: bool = crate::flags::blocks::Blocks::displays_size(blocks_2_ref_0);
    let mut bool_9: bool = crate::flags::blocks::Blocks::displays_size(blocks_1_ref_0);
    let mut bool_10: bool = crate::flags::blocks::Blocks::displays_size(blocks_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_262() {
//    rusty_monitor::set_test_id(262);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Name;
    let mut block_0_ref_0: &flags::blocks::Block = &mut block_0;
    let mut block_1: flags::blocks::Block = crate::flags::blocks::Block::Name;
    let mut block_1_ref_0: &flags::blocks::Block = &mut block_1;
    let mut block_2: flags::blocks::Block = crate::flags::blocks::Block::User;
    let mut block_2_ref_0: &flags::blocks::Block = &mut block_2;
    let mut block_3: flags::blocks::Block = crate::flags::blocks::Block::Permission;
    let mut block_3_ref_0: &flags::blocks::Block = &mut block_3;
    let mut block_4: flags::blocks::Block = crate::flags::blocks::Block::Name;
    let mut block_4_ref_0: &flags::blocks::Block = &mut block_4;
    let mut block_5: flags::blocks::Block = crate::flags::blocks::Block::Size;
    let mut block_5_ref_0: &flags::blocks::Block = &mut block_5;
    let mut block_6: flags::blocks::Block = crate::flags::blocks::Block::Size;
    let mut block_6_ref_0: &flags::blocks::Block = &mut block_6;
    let mut block_7: flags::blocks::Block = crate::flags::blocks::Block::SizeValue;
    let mut block_7_ref_0: &flags::blocks::Block = &mut block_7;
    let mut block_8: flags::blocks::Block = crate::flags::blocks::Block::User;
    let mut block_8_ref_0: &flags::blocks::Block = &mut block_8;
    let mut block_9: flags::blocks::Block = crate::flags::blocks::Block::User;
    let mut block_9_ref_0: &flags::blocks::Block = &mut block_9;
    let mut block_10: flags::blocks::Block = crate::flags::blocks::Block::Name;
    let mut block_10_ref_0: &flags::blocks::Block = &mut block_10;
    let mut block_11: flags::blocks::Block = crate::flags::blocks::Block::SizeValue;
    let mut block_11_ref_0: &flags::blocks::Block = &mut block_11;
    let mut block_12: flags::blocks::Block = crate::flags::blocks::Block::Links;
    let mut block_12_ref_0: &flags::blocks::Block = &mut block_12;
    let mut block_13: flags::blocks::Block = crate::flags::blocks::Block::Context;
    let mut block_13_ref_0: &flags::blocks::Block = &mut block_13;
    let mut option_0: std::option::Option<std::cmp::Ordering> = crate::flags::blocks::Block::partial_cmp(block_13_ref_0, block_12_ref_0);
    let mut option_1: std::option::Option<std::cmp::Ordering> = crate::flags::blocks::Block::partial_cmp(block_11_ref_0, block_10_ref_0);
    let mut option_2: std::option::Option<std::cmp::Ordering> = crate::flags::blocks::Block::partial_cmp(block_9_ref_0, block_8_ref_0);
    let mut option_3: std::option::Option<std::cmp::Ordering> = crate::flags::blocks::Block::partial_cmp(block_7_ref_0, block_6_ref_0);
    let mut option_4: std::option::Option<std::cmp::Ordering> = crate::flags::blocks::Block::partial_cmp(block_5_ref_0, block_4_ref_0);
    let mut option_5: std::option::Option<std::cmp::Ordering> = crate::flags::blocks::Block::partial_cmp(block_3_ref_0, block_2_ref_0);
    let mut option_6: std::option::Option<std::cmp::Ordering> = crate::flags::blocks::Block::partial_cmp(block_1_ref_0, block_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6926() {
//    rusty_monitor::set_test_id(6926);
    let mut str_0: &str = "+";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "tree_edge";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "Sort by file extension";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "mi";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "group_write";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7: &str = "js";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_8: &str = "gz";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut str_9: &str = "TreeEdge";
    let mut str_9_ref_0: &str = &mut str_9;
    let mut result_0: std::result::Result<flags::blocks::Block, std::string::String> = crate::flags::blocks::Block::try_from(str_9_ref_0);
    let mut result_1: std::result::Result<flags::blocks::Block, std::string::String> = crate::flags::blocks::Block::try_from(str_8_ref_0);
    let mut result_2: std::result::Result<flags::blocks::Block, std::string::String> = crate::flags::blocks::Block::try_from(str_7_ref_0);
    let mut result_3: std::result::Result<flags::blocks::Block, std::string::String> = crate::flags::blocks::Block::try_from(str_5_ref_0);
    let mut result_4: std::result::Result<flags::blocks::Block, std::string::String> = crate::flags::blocks::Block::try_from(str_4_ref_0);
    let mut result_5: std::result::Result<flags::blocks::Block, std::string::String> = crate::flags::blocks::Block::try_from(str_3_ref_0);
    let mut result_6: std::result::Result<flags::blocks::Block, std::string::String> = crate::flags::blocks::Block::try_from(str_2_ref_0);
    let mut result_7: std::result::Result<flags::blocks::Block, std::string::String> = crate::flags::blocks::Block::try_from(str_1_ref_0);
    let mut result_8: std::result::Result<flags::blocks::Block, std::string::String> = crate::flags::blocks::Block::try_from(str_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_200() {
//    rusty_monitor::set_test_id(200);
    let mut blocks_0: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_0_ref_0: &crate::flags::blocks::Blocks = &mut blocks_0;
    let mut blocks_1: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_1_ref_0: &crate::flags::blocks::Blocks = &mut blocks_1;
    let mut blocks_2: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_2_ref_0: &crate::flags::blocks::Blocks = &mut blocks_2;
    let mut blocks_3: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_3_ref_0: &crate::flags::blocks::Blocks = &mut blocks_3;
    let mut blocks_4: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_4_ref_0: &crate::flags::blocks::Blocks = &mut blocks_4;
    let mut blocks_5: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_5_ref_0: &crate::flags::blocks::Blocks = &mut blocks_5;
    let mut blocks_6: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_6_ref_0: &crate::flags::blocks::Blocks = &mut blocks_6;
    let mut blocks_7: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_7_ref_0: &crate::flags::blocks::Blocks = &mut blocks_7;
    let mut blocks_8: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_8_ref_0: &crate::flags::blocks::Blocks = &mut blocks_8;
    let mut blocks_9: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_9_ref_0: &crate::flags::blocks::Blocks = &mut blocks_9;
    let mut blocks_10: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_10_ref_0: &crate::flags::blocks::Blocks = &mut blocks_10;
    let mut blocks_11: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_11_ref_0: &crate::flags::blocks::Blocks = &mut blocks_11;
    let mut blocks_12: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_12_ref_0: &crate::flags::blocks::Blocks = &mut blocks_12;
    let mut blocks_13: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_13_ref_0: &crate::flags::blocks::Blocks = &mut blocks_13;
    let mut bool_0: bool = crate::flags::blocks::Blocks::eq(blocks_13_ref_0, blocks_12_ref_0);
    let mut bool_1: bool = crate::flags::blocks::Blocks::eq(blocks_11_ref_0, blocks_10_ref_0);
    let mut bool_2: bool = crate::flags::blocks::Blocks::eq(blocks_9_ref_0, blocks_8_ref_0);
    let mut bool_3: bool = crate::flags::blocks::Blocks::eq(blocks_7_ref_0, blocks_6_ref_0);
    let mut bool_4: bool = crate::flags::blocks::Blocks::eq(blocks_5_ref_0, blocks_4_ref_0);
    let mut bool_5: bool = crate::flags::blocks::Blocks::eq(blocks_3_ref_0, blocks_2_ref_0);
    let mut bool_6: bool = crate::flags::blocks::Blocks::eq(blocks_1_ref_0, blocks_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1642() {
//    rusty_monitor::set_test_id(1642);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Group;
    let mut block_0_ref_0: &flags::blocks::Block = &mut block_0;
    let mut elem_0: color::Elem = crate::color::Elem::ExecSticky;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut u64_0: u64 = 0u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut elem_1: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut bool_0: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_0};
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut elem_2: color::Elem = crate::color::Elem::NonFile;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut str_0: &str = "IconSeparator";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut option_0: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_1: &str = "";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut option_1: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_1_ref_0);
    let mut block_1: flags::blocks::Block = crate::flags::blocks::Block::Size;
    let mut block_1_ref_0: &flags::blocks::Block = &mut block_1;
    let mut ordering_0: std::cmp::Ordering = crate::flags::blocks::Block::cmp(block_1_ref_0, block_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_774() {
//    rusty_monitor::set_test_id(774);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::SizeValue;
    let mut block_0_ref_0: &flags::blocks::Block = &mut block_0;
    let mut block_1: flags::blocks::Block = crate::flags::blocks::Block::Name;
    let mut block_1_ref_0: &flags::blocks::Block = &mut block_1;
    let mut block_2: flags::blocks::Block = crate::flags::blocks::Block::Name;
    let mut block_2_ref_0: &flags::blocks::Block = &mut block_2;
    let mut block_3: flags::blocks::Block = crate::flags::blocks::Block::Permission;
    let mut block_3_ref_0: &flags::blocks::Block = &mut block_3;
    let mut block_4: flags::blocks::Block = crate::flags::blocks::Block::Group;
    let mut block_4_ref_0: &flags::blocks::Block = &mut block_4;
    let mut block_5: flags::blocks::Block = crate::flags::blocks::Block::Links;
    let mut block_5_ref_0: &flags::blocks::Block = &mut block_5;
    let mut block_6: flags::blocks::Block = crate::flags::blocks::Block::INode;
    let mut block_6_ref_0: &flags::blocks::Block = &mut block_6;
    let mut block_7: flags::blocks::Block = crate::flags::blocks::Block::Date;
    let mut block_7_ref_0: &flags::blocks::Block = &mut block_7;
    let mut block_8: flags::blocks::Block = crate::flags::blocks::Block::SizeValue;
    let mut block_8_ref_0: &flags::blocks::Block = &mut block_8;
    let mut block_9: flags::blocks::Block = crate::flags::blocks::Block::Date;
    let mut block_9_ref_0: &flags::blocks::Block = &mut block_9;
    let mut block_10: flags::blocks::Block = crate::flags::blocks::Block::Date;
    let mut block_10_ref_0: &flags::blocks::Block = &mut block_10;
    let mut tuple_0: () = crate::flags::blocks::Block::assert_receiver_is_total_eq(block_10_ref_0);
    let mut tuple_1: () = crate::flags::blocks::Block::assert_receiver_is_total_eq(block_9_ref_0);
    let mut tuple_2: () = crate::flags::blocks::Block::assert_receiver_is_total_eq(block_8_ref_0);
    let mut tuple_3: () = crate::flags::blocks::Block::assert_receiver_is_total_eq(block_7_ref_0);
    let mut tuple_4: () = crate::flags::blocks::Block::assert_receiver_is_total_eq(block_6_ref_0);
    let mut tuple_5: () = crate::flags::blocks::Block::assert_receiver_is_total_eq(block_5_ref_0);
    let mut tuple_6: () = crate::flags::blocks::Block::assert_receiver_is_total_eq(block_4_ref_0);
    let mut tuple_7: () = crate::flags::blocks::Block::assert_receiver_is_total_eq(block_3_ref_0);
    let mut tuple_8: () = crate::flags::blocks::Block::assert_receiver_is_total_eq(block_2_ref_0);
    let mut tuple_9: () = crate::flags::blocks::Block::assert_receiver_is_total_eq(block_1_ref_0);
    let mut tuple_10: () = crate::flags::blocks::Block::assert_receiver_is_total_eq(block_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_867() {
//    rusty_monitor::set_test_id(867);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Permission;
    let mut block_0_ref_0: &flags::blocks::Block = &mut block_0;
    let mut block_1: flags::blocks::Block = crate::flags::blocks::Block::Date;
    let mut block_1_ref_0: &flags::blocks::Block = &mut block_1;
    let mut block_2: flags::blocks::Block = crate::flags::blocks::Block::Group;
    let mut block_2_ref_0: &flags::blocks::Block = &mut block_2;
    let mut block_3: flags::blocks::Block = crate::flags::blocks::Block::Size;
    let mut block_3_ref_0: &flags::blocks::Block = &mut block_3;
    let mut block_4: flags::blocks::Block = crate::flags::blocks::Block::Size;
    let mut block_4_ref_0: &flags::blocks::Block = &mut block_4;
    let mut block_5: flags::blocks::Block = crate::flags::blocks::Block::Name;
    let mut block_5_ref_0: &flags::blocks::Block = &mut block_5;
    let mut block_6: flags::blocks::Block = crate::flags::blocks::Block::User;
    let mut block_6_ref_0: &flags::blocks::Block = &mut block_6;
    let mut block_7: flags::blocks::Block = crate::flags::blocks::Block::Permission;
    let mut block_8: flags::blocks::Block = crate::flags::blocks::Block::User;
    let mut block_8_ref_0: &flags::blocks::Block = &mut block_8;
    let mut block_9: flags::blocks::Block = crate::flags::blocks::Block::Name;
    let mut block_9_ref_0: &flags::blocks::Block = &mut block_9;
    let mut block_10: flags::blocks::Block = crate::flags::blocks::Block::Permission;
    let mut block_10_ref_0: &flags::blocks::Block = &mut block_10;
    let mut block_11: flags::blocks::Block = crate::flags::blocks::Block::clone(block_10_ref_0);
    let mut block_12: flags::blocks::Block = crate::flags::blocks::Block::clone(block_9_ref_0);
    let mut block_13: flags::blocks::Block = crate::flags::blocks::Block::clone(block_8_ref_0);
    let mut block_14: flags::blocks::Block = crate::flags::blocks::Block::clone(block_6_ref_0);
    let mut block_15: flags::blocks::Block = crate::flags::blocks::Block::clone(block_5_ref_0);
    let mut block_16: flags::blocks::Block = crate::flags::blocks::Block::clone(block_4_ref_0);
    let mut block_17: flags::blocks::Block = crate::flags::blocks::Block::clone(block_3_ref_0);
    let mut block_18: flags::blocks::Block = crate::flags::blocks::Block::clone(block_2_ref_0);
    let mut block_19: flags::blocks::Block = crate::flags::blocks::Block::clone(block_1_ref_0);
    let mut block_20: flags::blocks::Block = crate::flags::blocks::Block::clone(block_0_ref_0);
    let mut block_13_ref_0: &flags::blocks::Block = &mut block_13;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_363() {
//    rusty_monitor::set_test_id(363);
    let mut blocks_0: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_0_ref_0: &crate::flags::blocks::Blocks = &mut blocks_0;
    let mut blocks_1: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_1_ref_0: &crate::flags::blocks::Blocks = &mut blocks_1;
    let mut blocks_2: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_2_ref_0: &crate::flags::blocks::Blocks = &mut blocks_2;
    let mut blocks_3: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_3_ref_0: &crate::flags::blocks::Blocks = &mut blocks_3;
    let mut blocks_4: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_4_ref_0: &crate::flags::blocks::Blocks = &mut blocks_4;
    let mut blocks_5: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_5_ref_0: &crate::flags::blocks::Blocks = &mut blocks_5;
    let mut blocks_6: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_6_ref_0: &crate::flags::blocks::Blocks = &mut blocks_6;
    let mut blocks_7: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_7_ref_0: &crate::flags::blocks::Blocks = &mut blocks_7;
    let mut blocks_8: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_8_ref_0: &crate::flags::blocks::Blocks = &mut blocks_8;
    let mut blocks_9: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_9_ref_0: &crate::flags::blocks::Blocks = &mut blocks_9;
    let mut blocks_10: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_10_ref_0: &crate::flags::blocks::Blocks = &mut blocks_10;
    let mut blocks_11: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::clone(blocks_10_ref_0);
    let mut blocks_12: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::clone(blocks_9_ref_0);
    let mut blocks_13: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::clone(blocks_8_ref_0);
    let mut blocks_14: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::clone(blocks_7_ref_0);
    let mut blocks_15: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::clone(blocks_6_ref_0);
    let mut blocks_16: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::clone(blocks_5_ref_0);
    let mut blocks_17: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::clone(blocks_4_ref_0);
    let mut blocks_18: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::clone(blocks_3_ref_0);
    let mut blocks_19: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::clone(blocks_2_ref_0);
    let mut blocks_20: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::clone(blocks_1_ref_0);
    let mut blocks_21: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::clone(blocks_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_289() {
//    rusty_monitor::set_test_id(289);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::User;
    let mut block_0_ref_0: &flags::blocks::Block = &mut block_0;
    let mut block_1: flags::blocks::Block = crate::flags::blocks::Block::Context;
    let mut block_1_ref_0: &flags::blocks::Block = &mut block_1;
    let mut block_2: flags::blocks::Block = crate::flags::blocks::Block::Date;
    let mut block_2_ref_0: &flags::blocks::Block = &mut block_2;
    let mut block_3: flags::blocks::Block = crate::flags::blocks::Block::Context;
    let mut block_3_ref_0: &flags::blocks::Block = &mut block_3;
    let mut block_4: flags::blocks::Block = crate::flags::blocks::Block::Size;
    let mut block_4_ref_0: &flags::blocks::Block = &mut block_4;
    let mut block_5: flags::blocks::Block = crate::flags::blocks::Block::SizeValue;
    let mut block_5_ref_0: &flags::blocks::Block = &mut block_5;
    let mut block_6: flags::blocks::Block = crate::flags::blocks::Block::SizeValue;
    let mut block_6_ref_0: &flags::blocks::Block = &mut block_6;
    let mut block_7: flags::blocks::Block = crate::flags::blocks::Block::Links;
    let mut block_7_ref_0: &flags::blocks::Block = &mut block_7;
    let mut block_8: flags::blocks::Block = crate::flags::blocks::Block::Date;
    let mut block_8_ref_0: &flags::blocks::Block = &mut block_8;
    let mut block_9: flags::blocks::Block = crate::flags::blocks::Block::Date;
    let mut block_9_ref_0: &flags::blocks::Block = &mut block_9;
    let mut block_10: flags::blocks::Block = crate::flags::blocks::Block::SizeValue;
    let mut block_10_ref_0: &flags::blocks::Block = &mut block_10;
    let mut block_11: flags::blocks::Block = crate::flags::blocks::Block::SizeValue;
    let mut block_11_ref_0: &flags::blocks::Block = &mut block_11;
    let mut block_12: flags::blocks::Block = crate::flags::blocks::Block::SizeValue;
    let mut block_12_ref_0: &flags::blocks::Block = &mut block_12;
    let mut block_13: flags::blocks::Block = crate::flags::blocks::Block::SizeValue;
    let mut block_13_ref_0: &flags::blocks::Block = &mut block_13;
    let mut ordering_0: std::cmp::Ordering = crate::flags::blocks::Block::cmp(block_13_ref_0, block_12_ref_0);
    let mut ordering_1: std::cmp::Ordering = crate::flags::blocks::Block::cmp(block_11_ref_0, block_10_ref_0);
    let mut ordering_2: std::cmp::Ordering = crate::flags::blocks::Block::cmp(block_9_ref_0, block_8_ref_0);
    let mut ordering_3: std::cmp::Ordering = crate::flags::blocks::Block::cmp(block_7_ref_0, block_6_ref_0);
    let mut ordering_4: std::cmp::Ordering = crate::flags::blocks::Block::cmp(block_5_ref_0, block_4_ref_0);
    let mut ordering_5: std::cmp::Ordering = crate::flags::blocks::Block::cmp(block_3_ref_0, block_2_ref_0);
    let mut ordering_6: std::cmp::Ordering = crate::flags::blocks::Block::cmp(block_1_ref_0, block_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_910() {
//    rusty_monitor::set_test_id(910);
    let mut blocks_0: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_0_ref_0: &mut crate::flags::blocks::Blocks = &mut blocks_0;
    let mut blocks_1: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_1_ref_0: &mut crate::flags::blocks::Blocks = &mut blocks_1;
    let mut blocks_2: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_2_ref_0: &mut crate::flags::blocks::Blocks = &mut blocks_2;
    let mut blocks_3: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_3_ref_0: &mut crate::flags::blocks::Blocks = &mut blocks_3;
    let mut blocks_4: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_4_ref_0: &mut crate::flags::blocks::Blocks = &mut blocks_4;
    let mut blocks_5: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_5_ref_0: &mut crate::flags::blocks::Blocks = &mut blocks_5;
    let mut blocks_6: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_6_ref_0: &mut crate::flags::blocks::Blocks = &mut blocks_6;
    let mut blocks_7: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_7_ref_0: &mut crate::flags::blocks::Blocks = &mut blocks_7;
    let mut blocks_8: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::long();
    let mut blocks_8_ref_0: &mut crate::flags::blocks::Blocks = &mut blocks_8;
    let mut blocks_9: crate::flags::blocks::Blocks = crate::flags::blocks::Blocks::default();
    let mut blocks_9_ref_0: &mut crate::flags::blocks::Blocks = &mut blocks_9;
    crate::flags::blocks::Blocks::optional_prepend_inode(blocks_9_ref_0);
    crate::flags::blocks::Blocks::optional_prepend_inode(blocks_8_ref_0);
    crate::flags::blocks::Blocks::optional_prepend_inode(blocks_7_ref_0);
    crate::flags::blocks::Blocks::optional_prepend_inode(blocks_6_ref_0);
    crate::flags::blocks::Blocks::optional_prepend_inode(blocks_5_ref_0);
    crate::flags::blocks::Blocks::optional_prepend_inode(blocks_4_ref_0);
    crate::flags::blocks::Blocks::optional_prepend_inode(blocks_3_ref_0);
    crate::flags::blocks::Blocks::optional_prepend_inode(blocks_2_ref_0);
    crate::flags::blocks::Blocks::optional_prepend_inode(blocks_1_ref_0);
    crate::flags::blocks::Blocks::optional_prepend_inode(blocks_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2400() {
//    rusty_monitor::set_test_id(2400);
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::INode;
    let mut block_0_ref_0: &flags::blocks::Block = &mut block_0;
    let mut block_1: flags::blocks::Block = crate::flags::blocks::Block::User;
    let mut block_1_ref_0: &flags::blocks::Block = &mut block_1;
    let mut str_0: &str = "+";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "tree_edge";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "Sort by file extension";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "mi";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "group_write";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "Default";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7: &str = "";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_8: &str = "js";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut str_9: &str = "gz";
    let mut str_9_ref_0: &str = &mut str_9;
    let mut str_10: &str = "TreeEdge";
    let mut str_10_ref_0: &str = &mut str_10;
    let mut result_0: std::result::Result<flags::blocks::Block, std::string::String> = crate::flags::blocks::Block::try_from(str_10_ref_0);
    let mut result_1: std::result::Result<flags::blocks::Block, std::string::String> = crate::flags::blocks::Block::try_from(str_9_ref_0);
    let mut result_2: std::result::Result<flags::blocks::Block, std::string::String> = crate::flags::blocks::Block::try_from(str_8_ref_0);
    let mut result_3: std::result::Result<flags::blocks::Block, std::string::String> = crate::flags::blocks::Block::try_from(str_7_ref_0);
    let mut result_4: std::result::Result<flags::blocks::Block, std::string::String> = crate::flags::blocks::Block::try_from(str_6_ref_0);
    let mut result_5: std::result::Result<flags::blocks::Block, std::string::String> = crate::flags::blocks::Block::try_from(str_5_ref_0);
    let mut result_6: std::result::Result<flags::blocks::Block, std::string::String> = crate::flags::blocks::Block::try_from(str_4_ref_0);
    let mut result_7: std::result::Result<flags::blocks::Block, std::string::String> = crate::flags::blocks::Block::try_from(str_3_ref_0);
    let mut result_8: std::result::Result<flags::blocks::Block, std::string::String> = crate::flags::blocks::Block::try_from(str_2_ref_0);
    let mut result_9: std::result::Result<flags::blocks::Block, std::string::String> = crate::flags::blocks::Block::try_from(str_1_ref_0);
    let mut result_10: std::result::Result<flags::blocks::Block, std::string::String> = crate::flags::blocks::Block::try_from(str_0_ref_0);
    let mut bool_0: bool = crate::flags::blocks::Block::eq(block_1_ref_0, block_0_ref_0);
    let mut option_0: std::option::Option<crate::flags::blocks::Blocks> = crate::flags::blocks::Blocks::from_config(config_0_ref_0);
    let mut block_2: flags::blocks::Block = std::result::Result::unwrap(result_5);
//    panic!("From RustyUnit with love");
}
}