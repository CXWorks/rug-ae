use crate::color::{ColoredString, Colors, Elem};
#[cfg(unix)]
use std::fs::Metadata;

#[derive(Clone, Debug)]
pub struct Owner {
    user: String,
    group: String,
}

impl Owner {
    #[cfg_attr(unix, allow(dead_code))]
    pub fn new(user: String, group: String) -> Self {
        Self { user, group }
    }
}

#[cfg(unix)]
impl<'a> From<&'a Metadata> for Owner {
    fn from(meta: &Metadata) -> Self {
        use std::os::unix::fs::MetadataExt;
        use users::{get_group_by_gid, get_user_by_uid};

        let user = match get_user_by_uid(meta.uid()) {
            Some(res) => res.name().to_string_lossy().to_string(),
            None => meta.uid().to_string(),
        };

        let group = match get_group_by_gid(meta.gid()) {
            Some(res) => res.name().to_string_lossy().to_string(),
            None => meta.gid().to_string(),
        };

        Self { user, group }
    }
}

impl Owner {
    pub fn render_user(&self, colors: &Colors) -> ColoredString {
        colors.colorize(self.user.clone(), &Elem::User)
    }

    pub fn render_group(&self, colors: &Colors) -> ColoredString {
        colors.colorize(self.group.clone(), &Elem::Group)
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8838() {
//    rusty_monitor::set_test_id(8838);
    let mut elem_0: color::Elem = crate::color::Elem::NonFile;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_1: color::Elem = crate::color::Elem::Read;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut bool_0: bool = false;
    let mut elem_2: color::Elem = crate::color::Elem::Links {valid: bool_0};
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_3: color::Elem = crate::color::Elem::Read;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut elem_4: color::Elem = crate::color::Elem::DayOld;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut bool_1: bool = false;
    let mut elem_5: color::Elem = crate::color::Elem::INode {valid: bool_1};
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut elem_6: color::Elem = crate::color::Elem::Older;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut elem_7: color::Elem = crate::color::Elem::Read;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut elem_8: color::Elem = crate::color::Elem::Octal;
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut elem_9: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    let mut elem_10: color::Elem = crate::color::Elem::NonFile;
    let mut elem_10_ref_0: &color::Elem = &mut elem_10;
    let mut elem_11: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_11_ref_0: &color::Elem = &mut elem_11;
    let mut elem_12: color::Elem = crate::color::Elem::Socket;
    let mut elem_12_ref_0: &color::Elem = &mut elem_12;
    let mut elem_13: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_13_ref_0: &color::Elem = &mut elem_13;
    let mut bool_2: bool = false;
    let mut elem_14: color::Elem = crate::color::Elem::Links {valid: bool_2};
    let mut elem_14_ref_0: &color::Elem = &mut elem_14;
    let mut elem_15: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_15_ref_0: &color::Elem = &mut elem_15;
    let mut elem_16: color::Elem = crate::color::Elem::Read;
    let mut elem_16_ref_0: &color::Elem = &mut elem_16;
    let mut elem_17: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_17_ref_0: &color::Elem = &mut elem_17;
    let mut elem_18: color::Elem = crate::color::Elem::Older;
    let mut elem_18_ref_0: &color::Elem = &mut elem_18;
    let mut elem_19: color::Elem = crate::color::Elem::SymLink;
    let mut elem_19_ref_0: &color::Elem = &mut elem_19;
    let mut elem_20: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_20_ref_0: &color::Elem = &mut elem_20;
    let mut elem_21: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_21_ref_0: &color::Elem = &mut elem_21;
    let mut elem_22: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_22_ref_0: &color::Elem = &mut elem_22;
    let mut elem_23: color::Elem = crate::color::Elem::User;
    let mut elem_23_ref_0: &color::Elem = &mut elem_23;
    let mut bool_3: bool = false;
    let mut elem_24: color::Elem = crate::color::Elem::Dir {uid: bool_3};
    let mut elem_24_ref_0: &color::Elem = &mut elem_24;
    let mut elem_25: color::Elem = crate::color::Elem::Write;
    let mut elem_25_ref_0: &color::Elem = &mut elem_25;
    let mut elem_26: color::Elem = crate::color::Elem::Context;
    let mut elem_26_ref_0: &color::Elem = &mut elem_26;
    let mut elem_27: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_27_ref_0: &color::Elem = &mut elem_27;
    let mut elem_28: color::Elem = crate::color::Elem::Context;
    let mut elem_28_ref_0: &color::Elem = &mut elem_28;
    let mut elem_29: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_29_ref_0: &color::Elem = &mut elem_29;
    let mut elem_30: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_30_ref_0: &color::Elem = &mut elem_30;
    let mut elem_31: color::Elem = crate::color::Elem::Special;
    let mut elem_31_ref_0: &color::Elem = &mut elem_31;
    let mut elem_32: color::Elem = crate::color::Elem::ExecSticky;
    let mut elem_32_ref_0: &color::Elem = &mut elem_32;
    let mut bool_4: bool = true;
    let mut bool_5: bool = true;
    let mut elem_33: color::Elem = crate::color::Elem::File {exec: bool_5, uid: bool_4};
    let mut elem_33_ref_0: &color::Elem = &mut elem_33;
    let mut bool_6: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_6};
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    crate::meta::filetype::FileType::render(filetype_1, colors_0_ref_0);
//    panic!("From RustyUnit with love");
}
}