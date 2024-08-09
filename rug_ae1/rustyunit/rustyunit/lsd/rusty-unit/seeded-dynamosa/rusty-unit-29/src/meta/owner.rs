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
#[timeout(30000)]fn rusty_test_9075() {
//    rusty_monitor::set_test_id(9075);
    let mut str_0: &str = "Pipe";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = ".vscode";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "cshtml";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "2kgrDKhL";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "%m-%d %R";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "short";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7: &str = "ipynb";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_8: &str = "Sort the directories then the files";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut str_9: &str = "Permissions";
    let mut str_9_ref_0: &str = &mut str_9;
    let mut str_10: &str = "%F";
    let mut str_10_ref_0: &str = &mut str_10;
    let mut result_0: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_10_ref_0);
    let mut result_1: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_9_ref_0);
    let mut result_2: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_8_ref_0);
    let mut result_3: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_7_ref_0);
    let mut result_4: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_6_ref_0);
    let mut result_5: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_5_ref_0);
    let mut result_6: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_4_ref_0);
    let mut result_7: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_2_ref_0);
    let mut result_8: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_1_ref_0);
    let mut result_9: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_0_ref_0);
//    panic!("From RustyUnit with love");
}
}