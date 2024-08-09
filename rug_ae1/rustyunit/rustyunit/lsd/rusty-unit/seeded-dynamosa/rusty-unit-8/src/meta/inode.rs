use crate::color::{ColoredString, Colors, Elem};
use std::fs::Metadata;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct INode {
    index: Option<u64>,
}

impl<'a> From<&'a Metadata> for INode {
    #[cfg(unix)]
    fn from(meta: &Metadata) -> Self {
        use std::os::unix::fs::MetadataExt;

        let index = meta.ino();

        Self { index: Some(index) }
    }

    #[cfg(windows)]
    fn from(_: &Metadata) -> Self {
        Self { index: None }
    }
}

impl INode {
    pub fn render(&self, colors: &Colors) -> ColoredString {
        match self.index {
            Some(i) => colors.colorize(i.to_string(), &Elem::INode { valid: true }),
            None => colors.colorize(String::from("-"), &Elem::INode { valid: false }),
        }
    }
}

#[cfg(test)]
#[cfg(unix)]
mod tests {
    use super::INode;
    use std::env;
    use std::io;
    use std::path::Path;
    use std::process::{Command, ExitStatus};

    fn cross_platform_touch(path: &Path) -> io::Result<ExitStatus> {
        Command::new("touch").arg(&path).status()
    }

    #[test]
    fn test_inode_no_zero() {
        let mut file_path = env::temp_dir();
        file_path.push("inode.tmp");

        let success = cross_platform_touch(&file_path).unwrap().success();
        assert!(success, "failed to exec touch");

        let inode = INode::from(&file_path.metadata().unwrap());

        #[cfg(unix)]
        assert!(inode.index.is_some());
        #[cfg(windows)]
        assert!(inode.index.is_none());
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::cmp::PartialEq;
	use std::clone::Clone;
	use std::cmp::Eq;
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_207() {
//    rusty_monitor::set_test_id(207);
    let mut option_0: std::option::Option<u64> = std::option::Option::None;
    let mut inode_0: crate::meta::inode::INode = crate::meta::inode::INode {index: option_0};
    let mut inode_0_ref_0: &crate::meta::inode::INode = &mut inode_0;
    let mut option_1: std::option::Option<u64> = std::option::Option::None;
    let mut inode_1: crate::meta::inode::INode = crate::meta::inode::INode {index: option_1};
    let mut inode_1_ref_0: &crate::meta::inode::INode = &mut inode_1;
    let mut option_2: std::option::Option<u64> = std::option::Option::None;
    let mut inode_2: crate::meta::inode::INode = crate::meta::inode::INode {index: option_2};
    let mut inode_2_ref_0: &crate::meta::inode::INode = &mut inode_2;
    let mut u64_0: u64 = 0u64;
    let mut option_3: std::option::Option<u64> = std::option::Option::Some(u64_0);
    let mut inode_3: crate::meta::inode::INode = crate::meta::inode::INode {index: option_3};
    let mut inode_3_ref_0: &crate::meta::inode::INode = &mut inode_3;
    let mut u64_1: u64 = 1024u64;
    let mut option_4: std::option::Option<u64> = std::option::Option::Some(u64_1);
    let mut inode_4: crate::meta::inode::INode = crate::meta::inode::INode {index: option_4};
    let mut inode_4_ref_0: &crate::meta::inode::INode = &mut inode_4;
    let mut u64_2: u64 = 1099511627776u64;
    let mut option_5: std::option::Option<u64> = std::option::Option::Some(u64_2);
    let mut inode_5: crate::meta::inode::INode = crate::meta::inode::INode {index: option_5};
    let mut inode_5_ref_0: &crate::meta::inode::INode = &mut inode_5;
    let mut option_6: std::option::Option<u64> = std::option::Option::None;
    let mut inode_6: crate::meta::inode::INode = crate::meta::inode::INode {index: option_6};
    let mut inode_6_ref_0: &crate::meta::inode::INode = &mut inode_6;
    let mut tuple_0: () = crate::meta::inode::INode::assert_receiver_is_total_eq(inode_6_ref_0);
    let mut tuple_1: () = crate::meta::inode::INode::assert_receiver_is_total_eq(inode_5_ref_0);
    let mut tuple_2: () = crate::meta::inode::INode::assert_receiver_is_total_eq(inode_4_ref_0);
    let mut tuple_3: () = crate::meta::inode::INode::assert_receiver_is_total_eq(inode_3_ref_0);
    let mut tuple_4: () = crate::meta::inode::INode::assert_receiver_is_total_eq(inode_2_ref_0);
    let mut tuple_5: () = crate::meta::inode::INode::assert_receiver_is_total_eq(inode_1_ref_0);
    let mut tuple_6: () = crate::meta::inode::INode::assert_receiver_is_total_eq(inode_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_257() {
//    rusty_monitor::set_test_id(257);
    let mut option_0: std::option::Option<u64> = std::option::Option::None;
    let mut inode_0: crate::meta::inode::INode = crate::meta::inode::INode {index: option_0};
    let mut inode_0_ref_0: &crate::meta::inode::INode = &mut inode_0;
    let mut u64_0: u64 = 1024u64;
    let mut option_1: std::option::Option<u64> = std::option::Option::Some(u64_0);
    let mut inode_1: crate::meta::inode::INode = crate::meta::inode::INode {index: option_1};
    let mut inode_1_ref_0: &crate::meta::inode::INode = &mut inode_1;
    let mut option_2: std::option::Option<u64> = std::option::Option::None;
    let mut inode_2: crate::meta::inode::INode = crate::meta::inode::INode {index: option_2};
    let mut inode_2_ref_0: &crate::meta::inode::INode = &mut inode_2;
    let mut option_3: std::option::Option<u64> = std::option::Option::None;
    let mut inode_3: crate::meta::inode::INode = crate::meta::inode::INode {index: option_3};
    let mut inode_3_ref_0: &crate::meta::inode::INode = &mut inode_3;
    let mut u64_1: u64 = 1048576u64;
    let mut option_4: std::option::Option<u64> = std::option::Option::Some(u64_1);
    let mut inode_4: crate::meta::inode::INode = crate::meta::inode::INode {index: option_4};
    let mut inode_4_ref_0: &crate::meta::inode::INode = &mut inode_4;
    let mut u64_2: u64 = 1073741824u64;
    let mut option_5: std::option::Option<u64> = std::option::Option::Some(u64_2);
    let mut inode_5: crate::meta::inode::INode = crate::meta::inode::INode {index: option_5};
    let mut inode_5_ref_0: &crate::meta::inode::INode = &mut inode_5;
    let mut u64_3: u64 = 1024u64;
    let mut option_6: std::option::Option<u64> = std::option::Option::Some(u64_3);
    let mut inode_6: crate::meta::inode::INode = crate::meta::inode::INode {index: option_6};
    let mut inode_6_ref_0: &crate::meta::inode::INode = &mut inode_6;
    let mut u64_4: u64 = 1073741824u64;
    let mut option_7: std::option::Option<u64> = std::option::Option::Some(u64_4);
    let mut inode_7: crate::meta::inode::INode = crate::meta::inode::INode {index: option_7};
    let mut inode_7_ref_0: &crate::meta::inode::INode = &mut inode_7;
    let mut bool_0: bool = crate::meta::inode::INode::eq(inode_7_ref_0, inode_6_ref_0);
    let mut bool_1: bool = crate::meta::inode::INode::eq(inode_5_ref_0, inode_4_ref_0);
    let mut bool_2: bool = crate::meta::inode::INode::eq(inode_3_ref_0, inode_2_ref_0);
    let mut bool_3: bool = crate::meta::inode::INode::eq(inode_1_ref_0, inode_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_549() {
//    rusty_monitor::set_test_id(549);
    let mut u64_0: u64 = 1024u64;
    let mut option_0: std::option::Option<u64> = std::option::Option::Some(u64_0);
    let mut inode_0: crate::meta::inode::INode = crate::meta::inode::INode {index: option_0};
    let mut inode_0_ref_0: &crate::meta::inode::INode = &mut inode_0;
    let mut u64_1: u64 = 1099511627776u64;
    let mut option_1: std::option::Option<u64> = std::option::Option::Some(u64_1);
    let mut inode_1: crate::meta::inode::INode = crate::meta::inode::INode {index: option_1};
    let mut inode_1_ref_0: &crate::meta::inode::INode = &mut inode_1;
    let mut option_2: std::option::Option<u64> = std::option::Option::None;
    let mut inode_2: crate::meta::inode::INode = crate::meta::inode::INode {index: option_2};
    let mut inode_2_ref_0: &crate::meta::inode::INode = &mut inode_2;
    let mut u64_2: u64 = 1048576u64;
    let mut option_3: std::option::Option<u64> = std::option::Option::Some(u64_2);
    let mut inode_3: crate::meta::inode::INode = crate::meta::inode::INode {index: option_3};
    let mut inode_3_ref_0: &crate::meta::inode::INode = &mut inode_3;
    let mut option_4: std::option::Option<u64> = std::option::Option::None;
    let mut inode_4: crate::meta::inode::INode = crate::meta::inode::INode {index: option_4};
    let mut inode_4_ref_0: &crate::meta::inode::INode = &mut inode_4;
    let mut u64_3: u64 = 1048576u64;
    let mut option_5: std::option::Option<u64> = std::option::Option::Some(u64_3);
    let mut inode_5: crate::meta::inode::INode = crate::meta::inode::INode {index: option_5};
    let mut inode_5_ref_0: &crate::meta::inode::INode = &mut inode_5;
    let mut u64_4: u64 = 1048576u64;
    let mut option_6: std::option::Option<u64> = std::option::Option::Some(u64_4);
    let mut inode_6: crate::meta::inode::INode = crate::meta::inode::INode {index: option_6};
    let mut inode_6_ref_0: &crate::meta::inode::INode = &mut inode_6;
    let mut inode_7: crate::meta::inode::INode = crate::meta::inode::INode::clone(inode_6_ref_0);
    let mut inode_8: crate::meta::inode::INode = crate::meta::inode::INode::clone(inode_5_ref_0);
    let mut inode_9: crate::meta::inode::INode = crate::meta::inode::INode::clone(inode_4_ref_0);
    let mut inode_10: crate::meta::inode::INode = crate::meta::inode::INode::clone(inode_3_ref_0);
    let mut inode_11: crate::meta::inode::INode = crate::meta::inode::INode::clone(inode_2_ref_0);
    let mut inode_12: crate::meta::inode::INode = crate::meta::inode::INode::clone(inode_1_ref_0);
    let mut inode_13: crate::meta::inode::INode = crate::meta::inode::INode::clone(inode_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_444() {
//    rusty_monitor::set_test_id(444);
    let mut u64_0: u64 = 1073741824u64;
    let mut option_0: std::option::Option<u64> = std::option::Option::Some(u64_0);
    let mut inode_0: crate::meta::inode::INode = crate::meta::inode::INode {index: option_0};
    let mut inode_0_ref_0: &crate::meta::inode::INode = &mut inode_0;
    let mut option_1: std::option::Option<u64> = std::option::Option::None;
    let mut inode_1: crate::meta::inode::INode = crate::meta::inode::INode {index: option_1};
    let mut inode_1_ref_0: &crate::meta::inode::INode = &mut inode_1;
    let mut u64_1: u64 = 1048576u64;
    let mut option_2: std::option::Option<u64> = std::option::Option::Some(u64_1);
    let mut inode_2: crate::meta::inode::INode = crate::meta::inode::INode {index: option_2};
    let mut inode_2_ref_0: &crate::meta::inode::INode = &mut inode_2;
    let mut option_3: std::option::Option<u64> = std::option::Option::None;
    let mut inode_3: crate::meta::inode::INode = crate::meta::inode::INode {index: option_3};
    let mut inode_3_ref_0: &crate::meta::inode::INode = &mut inode_3;
    let mut option_4: std::option::Option<u64> = std::option::Option::None;
    let mut inode_4: crate::meta::inode::INode = crate::meta::inode::INode {index: option_4};
    let mut inode_4_ref_0: &crate::meta::inode::INode = &mut inode_4;
    let mut u64_2: u64 = 1073741824u64;
    let mut option_5: std::option::Option<u64> = std::option::Option::Some(u64_2);
    let mut inode_5: crate::meta::inode::INode = crate::meta::inode::INode {index: option_5};
    let mut inode_5_ref_0: &crate::meta::inode::INode = &mut inode_5;
    let mut option_6: std::option::Option<u64> = std::option::Option::None;
    let mut inode_6: crate::meta::inode::INode = crate::meta::inode::INode {index: option_6};
    let mut inode_6_ref_0: &crate::meta::inode::INode = &mut inode_6;
    let mut option_7: std::option::Option<u64> = std::option::Option::None;
    let mut inode_7: crate::meta::inode::INode = crate::meta::inode::INode {index: option_7};
    let mut inode_7_ref_0: &crate::meta::inode::INode = &mut inode_7;
    let mut option_8: std::option::Option<u64> = std::option::Option::None;
    let mut inode_8: crate::meta::inode::INode = crate::meta::inode::INode {index: option_8};
    let mut inode_8_ref_0: &crate::meta::inode::INode = &mut inode_8;
    let mut option_9: std::option::Option<u64> = std::option::Option::None;
    let mut inode_9: crate::meta::inode::INode = crate::meta::inode::INode {index: option_9};
    let mut inode_9_ref_0: &crate::meta::inode::INode = &mut inode_9;
    let mut bool_0: bool = crate::meta::inode::INode::ne(inode_9_ref_0, inode_8_ref_0);
    let mut bool_1: bool = crate::meta::inode::INode::ne(inode_7_ref_0, inode_6_ref_0);
    let mut bool_2: bool = crate::meta::inode::INode::ne(inode_5_ref_0, inode_4_ref_0);
    let mut bool_3: bool = crate::meta::inode::INode::ne(inode_3_ref_0, inode_2_ref_0);
    let mut bool_4: bool = crate::meta::inode::INode::ne(inode_1_ref_0, inode_0_ref_0);
//    panic!("From RustyUnit with love");
}
}