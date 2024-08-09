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
                            return Some(
                                Err(
                                    Error::with_description(
                                        &message,
                                        ErrorKind::ValueValidation,
                                    ),
                                ),
                            );
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
            if blocks.is_empty() { None } else { Some(Self(blocks)) }
        } else {
            None
        }
    }
    /// This returns a Blocks struct for the long format.
    ///
    /// It contains the [Block]s [Permission](Block::Permission), [User](Block::User),
    /// [Group](Block::Group), [Size](Block::Size), [Date](Block::Date) and [Name](Block::Name).
    fn long() -> Self {
        Self(
            vec![
                Block::Permission, Block::User, Block::Group, Block::Size, Block::Date,
                Block::Name,
            ],
        )
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
            _ => Err(format!("Not a valid block name: {}", & string)),
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
    macro_rules! assert_eq_ok {
        ($left:expr, $right:expr) => {
            assert!(match &$left { Ok(inner) if inner == $right .as_ref().unwrap() =>
            true, _ => false, },
            "\nComparison failed:\nWas:       {:?}\nShould be: {:?}\n", &$left, &$right)
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
        assert!(match Blocks::from_arg_matches(& matches) { None => true, _ => false, });
    }
    #[test]
    fn test_from_arg_matches_one() {
        let argv = vec!["lsd", "--blocks", "permission"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let test_blocks = Blocks(vec![Block::Permission]);
        assert!(
            match Blocks::from_arg_matches(& matches) { Some(Ok(blocks)) if blocks ==
            test_blocks => true, _ => false, }
        );
    }
    #[test]
    fn test_from_arg_matches_multi_occurences() {
        let argv = vec!["lsd", "--blocks", "permission", "--blocks", "name"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let test_blocks = Blocks(vec![Block::Permission, Block::Name]);
        assert!(
            match Blocks::from_arg_matches(& matches) { Some(Ok(blocks)) if blocks ==
            test_blocks => true, _ => false, }
        );
    }
    #[test]
    fn test_from_arg_matches_multi_values() {
        let argv = vec!["lsd", "--blocks", "permission,name"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let test_blocks = Blocks(vec![Block::Permission, Block::Name]);
        assert!(
            match Blocks::from_arg_matches(& matches) { Some(Ok(blocks)) if blocks ==
            test_blocks => true, _ => false, }
        );
    }
    #[test]
    fn test_from_arg_matches_reversed_default() {
        let argv = vec!["lsd", "--blocks", "name,date,size,group,user,permission"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let test_blocks = Blocks(
            vec![
                Block::Name, Block::Date, Block::Size, Block::Group, Block::User,
                Block::Permission,
            ],
        );
        assert!(
            match Blocks::from_arg_matches(& matches) { Some(Ok(blocks)) if blocks ==
            test_blocks => true, _ => false, }
        );
    }
    #[test]
    fn test_from_arg_matches_every_second_one() {
        let argv = vec!["lsd", "--blocks", "permission,group,date"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let test_blocks = Blocks(vec![Block::Permission, Block::Group, Block::Date]);
        assert!(
            match Blocks::from_arg_matches(& matches) { Some(Ok(blocks)) if blocks ==
            test_blocks => true, _ => false, }
        );
    }
    #[test]
    fn test_from_config_none() {
        assert_eq!(None, Blocks::from_config(& Config::with_none()));
    }
    #[test]
    fn test_from_config_one() {
        let mut c = Config::with_none();
        c.blocks = Some(vec!["permission".into()].into());
        let blocks = Blocks(vec![Block::Permission]);
        assert_eq!(Some(blocks), Blocks::from_config(& c));
    }
    #[test]
    fn test_from_config_reversed_default() {
        let target = Blocks(
            vec![
                Block::Name, Block::Date, Block::Size, Block::Group, Block::User,
                Block::Permission,
            ],
        );
        let mut c = Config::with_none();
        c
            .blocks = Some(
            vec![
                "name".into(), "date".into(), "size".into(), "group".into(), "user"
                .into(), "permission".into(),
            ]
                .into(),
        );
        assert_eq!(Some(target), Blocks::from_config(& c));
    }
    #[test]
    fn test_from_config_every_second_one() {
        let mut c = Config::with_none();
        c.blocks = Some(vec!["permission".into(), "group".into(), "date".into()].into());
        let blocks = Blocks(vec![Block::Permission, Block::Group, Block::Date]);
        assert_eq!(Some(blocks), Blocks::from_config(& c));
    }
    #[test]
    fn test_from_config_invalid_is_ignored() {
        let mut c = Config::with_none();
        c.blocks = Some(vec!["permission".into(), "foo".into(), "date".into()].into());
        let blocks = Blocks(vec![Block::Permission, Block::Date]);
        assert_eq!(Some(blocks), Blocks::from_config(& c));
    }
    #[test]
    fn test_context_not_present_on_cli() {
        let argv = vec!["lsd", "--long"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let parsed_blocks = Blocks::configure_from(&matches, &Config::with_none())
            .unwrap();
        let it = parsed_blocks.0.iter();
        assert_eq!(it.filter(|& x | * x == Block::Context).count(), 0);
    }
    #[test]
    fn test_context_present_if_context_on() {
        let argv = vec!["lsd", "--context"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let parsed_blocks = Blocks::configure_from(&matches, &Config::with_none())
            .unwrap();
        let it = parsed_blocks.0.iter();
        assert_eq!(it.filter(|& x | * x == Block::Context).count(), 1);
    }
    #[test]
    fn test_only_one_context_no_other_blocks_affected() {
        let argv = vec![
            "lsd", "--context", "--blocks",
            "name,date,size,context,group,user,permission",
        ];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let test_blocks = Blocks(
            vec![
                Block::Name, Block::Date, Block::Size, Block::Context, Block::Group,
                Block::User, Block::Permission,
            ],
        );
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
            Err(String::from("Not a valid block name: foo")), Block::try_from("foo")
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
mod tests_llm_16_21 {
    use super::*;
    use crate::*;
    use std::convert::TryInto;
    #[test]
    fn test_try_from() {
        let _rug_st_tests_llm_16_21_rrrruuuugggg_test_try_from = 0;
        let rug_fuzz_0 = "permission";
        let test_cases = vec![
            (rug_fuzz_0, Ok(Block::Permission)), ("user", Ok(Block::User)), ("group",
            Ok(Block::Group)), ("context", Ok(Block::Context)), ("size",
            Ok(Block::Size)), ("size_value", Ok(Block::SizeValue)), ("date",
            Ok(Block::Date)), ("name", Ok(Block::Name)), ("inode", Ok(Block::INode)),
            ("links", Ok(Block::Links)), ("invalid",
            Err("Not a valid block name: invalid".to_string()))
        ];
        for (input, expected) in test_cases {
            let result: Result<Block, String> = <Block as TryFrom<
                &str,
            >>::try_from(input);
            debug_assert_eq!(result, expected);
        }
        let _rug_ed_tests_llm_16_21_rrrruuuugggg_test_try_from = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_23_llm_16_22 {
    use super::*;
    use crate::*;
    use crate::flags::blocks::{Block, Blocks};
    #[test]
    fn test_default() {
        let _rug_st_tests_llm_16_23_llm_16_22_rrrruuuugggg_test_default = 0;
        let result = <flags::blocks::Blocks as std::default::Default>::default();
        let expected = Blocks(vec![Block::Name]);
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_23_llm_16_22_rrrruuuugggg_test_default = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_182 {
    use super::*;
    use crate::*;
    use std::convert::TryFrom;
    #[test]
    fn test_contains_inode() {
        let _rug_st_tests_llm_16_182_rrrruuuugggg_test_contains_inode = 0;
        let mut blocks = Blocks::default();
        debug_assert_eq!(blocks.contains_inode(), false);
        let inode_block = Block::INode;
        blocks.0.push(inode_block);
        debug_assert_eq!(blocks.contains_inode(), true);
        let _rug_ed_tests_llm_16_182_rrrruuuugggg_test_contains_inode = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_183 {
    use super::*;
    use crate::*;
    #[test]
    fn test_displays_size_returns_true_if_size_block_is_present() {
        let _rug_st_tests_llm_16_183_rrrruuuugggg_test_displays_size_returns_true_if_size_block_is_present = 0;
        let blocks = Blocks(vec![Block::Permission, Block::User, Block::Size]);
        let displays_size = blocks.displays_size();
        debug_assert!(displays_size);
        let _rug_ed_tests_llm_16_183_rrrruuuugggg_test_displays_size_returns_true_if_size_block_is_present = 0;
    }
    #[test]
    fn test_displays_size_returns_false_if_size_block_is_not_present() {
        let _rug_st_tests_llm_16_183_rrrruuuugggg_test_displays_size_returns_false_if_size_block_is_not_present = 0;
        let blocks = Blocks(vec![Block::Permission, Block::User]);
        let displays_size = blocks.displays_size();
        debug_assert!(! displays_size);
        let _rug_ed_tests_llm_16_183_rrrruuuugggg_test_displays_size_returns_false_if_size_block_is_not_present = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_186 {
    use super::*;
    use crate::*;
    use crate::config_file::*;
    #[test]
    fn test_from_config_with_blocks() {
        let _rug_st_tests_llm_16_186_rrrruuuugggg_test_from_config_with_blocks = 0;
        let rug_fuzz_0 = "block1";
        let config = Config {
            blocks: Some(vec![rug_fuzz_0.to_string(), "block2".to_string()]),
            ..Default::default()
        };
        let blocks = Blocks::from_config(&config);
        debug_assert_eq!(
            blocks, Some(Blocks(vec![Block::try_from("block1").unwrap(),
            Block::try_from("block2").unwrap(),]))
        );
        let _rug_ed_tests_llm_16_186_rrrruuuugggg_test_from_config_with_blocks = 0;
    }
    #[test]
    fn test_from_config_without_blocks() {
        let _rug_st_tests_llm_16_186_rrrruuuugggg_test_from_config_without_blocks = 0;
        let config = Config {
            blocks: Some(vec![]),
            ..Default::default()
        };
        let blocks = Blocks::from_config(&config);
        debug_assert_eq!(blocks, None);
        let _rug_ed_tests_llm_16_186_rrrruuuugggg_test_from_config_without_blocks = 0;
    }
    #[test]
    fn test_from_config_without_config_blocks() {
        let _rug_st_tests_llm_16_186_rrrruuuugggg_test_from_config_without_config_blocks = 0;
        let config = Config { ..Default::default() };
        let blocks = Blocks::from_config(&config);
        debug_assert_eq!(blocks, None);
        let _rug_ed_tests_llm_16_186_rrrruuuugggg_test_from_config_without_config_blocks = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_187 {
    use super::*;
    use crate::*;
    #[test]
    fn test_long() {
        let _rug_st_tests_llm_16_187_rrrruuuugggg_test_long = 0;
        let expected = Blocks(
            vec![
                Block::Permission, Block::User, Block::Group, Block::Size, Block::Date,
                Block::Name
            ],
        );
        let result = Blocks::long();
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_187_rrrruuuugggg_test_long = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_188 {
    use super::*;
    use crate::*;
    #[test]
    fn test_optional_insert_context() {
        let _rug_st_tests_llm_16_188_rrrruuuugggg_test_optional_insert_context = 0;
        let mut blocks = Blocks::default();
        blocks.optional_insert_context();
        debug_assert_eq!(blocks.0, vec![Block::Context, Block::Name]);
        let mut blocks = Blocks(vec![Block::Name, Block::User]);
        blocks.optional_insert_context();
        debug_assert_eq!(blocks.0, vec![Block::Context, Block::Name, Block::User]);
        let mut blocks = Blocks(vec![Block::Name, Block::Group]);
        blocks.optional_insert_context();
        debug_assert_eq!(blocks.0, vec![Block::Name, Block::Context, Block::Group]);
        let mut blocks = Blocks(vec![Block::Name, Block::Context]);
        blocks.optional_insert_context();
        debug_assert_eq!(blocks.0, vec![Block::Name, Block::Context]);
        let _rug_ed_tests_llm_16_188_rrrruuuugggg_test_optional_insert_context = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_189 {
    use super::*;
    use crate::*;
    #[test]
    fn test_optional_prepend_inode() {
        let _rug_st_tests_llm_16_189_rrrruuuugggg_test_optional_prepend_inode = 0;
        let mut blocks = Blocks(vec![Block::User, Block::Group, Block::Size]);
        blocks.optional_prepend_inode();
        debug_assert_eq!(
            blocks.0, vec![Block::INode, Block::User, Block::Group, Block::Size]
        );
        let mut blocks = Blocks(
            vec![Block::INode, Block::User, Block::Group, Block::Size],
        );
        blocks.optional_prepend_inode();
        debug_assert_eq!(
            blocks.0, vec![Block::INode, Block::User, Block::Group, Block::Size]
        );
        let _rug_ed_tests_llm_16_189_rrrruuuugggg_test_optional_prepend_inode = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_190 {
    use super::*;
    use crate::*;
    #[test]
    fn test_prepend_inode() {
        let _rug_st_tests_llm_16_190_rrrruuuugggg_test_prepend_inode = 0;
        let mut blocks = Blocks(vec![Block::User, Block::Group]);
        blocks.prepend_inode();
        debug_assert_eq!(blocks.0, vec![Block::INode, Block::User, Block::Group]);
        let _rug_ed_tests_llm_16_190_rrrruuuugggg_test_prepend_inode = 0;
    }
}
#[cfg(test)]
mod tests_rug_39 {
    use super::*;
    use clap::ArgMatches;
    use crate::config_file::Config;
    #[test]
    fn test_blocks() {
        let _rug_st_tests_rug_39_rrrruuuugggg_test_blocks = 0;
        let mut p0: ArgMatches<'static> = ArgMatches::default();
        let p1 = Config::default();
        crate::flags::blocks::Blocks::configure_from(&p0, &p1);
        let _rug_ed_tests_rug_39_rrrruuuugggg_test_blocks = 0;
    }
}
#[cfg(test)]
mod tests_rug_40 {
    use super::*;
    use crate::flags::blocks::Blocks;
    use clap::{ArgMatches, App, Arg};
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_40_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = "test";
        let rug_fuzz_1 = "blocks";
        let rug_fuzz_2 = true;
        let mut p0: ArgMatches = App::new(rug_fuzz_0)
            .arg(Arg::with_name(rug_fuzz_1).multiple(rug_fuzz_2))
            .get_matches();
        Blocks::from_arg_matches(&p0);
        let _rug_ed_tests_rug_40_rrrruuuugggg_test_rug = 0;
    }
}
