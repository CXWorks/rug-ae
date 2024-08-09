use crate::color::{ColoredString, Colors, Elem};
use std::path::Path;

#[derive(Clone, Debug)]
pub struct AccessControl {
    has_acl: bool,
    selinux_context: String,
    smack_context: String,
}

impl AccessControl {
    #[cfg(not(unix))]
    pub fn for_path(_: &Path) -> Self {
        Self::from_data(false, &[], &[])
    }

    #[cfg(unix)]
    pub fn for_path(path: &Path) -> Self {
        let has_acl = !xattr::get(path, Method::Acl.name())
            .unwrap_or_default()
            .unwrap_or_default()
            .is_empty();
        let selinux_context = xattr::get(path, Method::Selinux.name())
            .unwrap_or_default()
            .unwrap_or_default();
        let smack_context = xattr::get(path, Method::Smack.name())
            .unwrap_or_default()
            .unwrap_or_default();

        Self::from_data(has_acl, &selinux_context, &smack_context)
    }

    fn from_data(has_acl: bool, selinux_context: &[u8], smack_context: &[u8]) -> Self {
        let selinux_context = String::from_utf8_lossy(selinux_context).to_string();
        let smack_context = String::from_utf8_lossy(smack_context).to_string();
        Self {
            has_acl,
            selinux_context,
            smack_context,
        }
    }

    pub fn render_method(&self, colors: &Colors) -> ColoredString {
        if self.has_acl {
            colors.colorize(String::from("+"), &Elem::Acl)
        } else if !self.selinux_context.is_empty() || !self.smack_context.is_empty() {
            colors.colorize(String::from("."), &Elem::Context)
        } else {
            colors.colorize(String::from(""), &Elem::Acl)
        }
    }

    pub fn render_context(&self, colors: &Colors) -> ColoredString {
        let mut context = self.selinux_context.clone();
        if !self.smack_context.is_empty() {
            if !context.is_empty() {
                context += "+";
            }
            context += &self.smack_context;
        }
        if context.is_empty() {
            context += "?";
        }
        colors.colorize(context, &Elem::Context)
    }
}

#[cfg(unix)]
enum Method {
    Acl,
    Selinux,
    Smack,
}

#[cfg(unix)]
impl Method {
    fn name(&self) -> &'static str {
        match self {
            Method::Acl => "system.posix_acl_access",
            Method::Selinux => "security.selinux",
            Method::Smack => "security.SMACK64",
        }
    }
}

#[cfg(test)]
mod test {
    use super::AccessControl;
    use crate::color::{Colors, ThemeOption};
    use crossterm::style::{Color, Stylize};

    #[test]
    fn test_acl_only_indicator() {
        // actual file would collide with proper AC data, no permission to scrub those
        let access_control = AccessControl::from_data(true, &[], &[]);

        assert_eq!(
            String::from("+").with(Color::DarkCyan),
            access_control.render_method(&Colors::new(ThemeOption::Default))
        );
    }

    #[test]
    fn test_smack_only_indicator() {
        let access_control = AccessControl::from_data(false, &[], &[b'a']);

        assert_eq!(
            String::from(".").with(Color::Cyan),
            access_control.render_method(&Colors::new(ThemeOption::Default))
        );
    }

    #[test]
    fn test_acl_and_selinux_indicator() {
        let access_control = AccessControl::from_data(true, &[b'a'], &[]);

        assert_eq!(
            String::from("+").with(Color::DarkCyan),
            access_control.render_method(&Colors::new(ThemeOption::Default))
        );
    }

    #[test]
    fn test_selinux_context() {
        let access_control = AccessControl::from_data(false, &[b'a'], &[]);

        assert_eq!(
            String::from("a").with(Color::Cyan),
            access_control.render_context(&Colors::new(ThemeOption::Default))
        );
    }

    #[test]
    fn test_selinux_and_smack_context() {
        let access_control = AccessControl::from_data(false, &[b'a'], &[b'b']);

        assert_eq!(
            String::from("a+b").with(Color::Cyan),
            access_control.render_context(&Colors::new(ThemeOption::Default))
        );
    }

    #[test]
    fn test_no_context() {
        let access_control = AccessControl::from_data(false, &[], &[]);

        assert_eq!(
            String::from("?").with(Color::Cyan),
            access_control.render_context(&Colors::new(ThemeOption::Default))
        );
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_494() {
//    rusty_monitor::set_test_id(494);
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_1, exec: bool_0};
    let mut bool_2: bool = false;
    let mut bool_3: bool = true;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_3, exec: bool_2};
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut filetype_4: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut bool_4: bool = true;
    let mut filetype_5: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_4};
    let mut filetype_6: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut filetype_7: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut bool_5: bool = true;
    let mut filetype_8: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_5};
    let mut filetype_9: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut bool_6: bool = true;
    let mut filetype_10: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_6};
    let mut bool_7: bool = true;
    let mut filetype_11: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_7};
    let mut bool_8: bool = crate::meta::filetype::FileType::is_dirlike(filetype_11);
    let mut bool_9: bool = crate::meta::filetype::FileType::is_dirlike(filetype_10);
    let mut bool_10: bool = crate::meta::filetype::FileType::is_dirlike(filetype_9);
    let mut bool_11: bool = crate::meta::filetype::FileType::is_dirlike(filetype_8);
    let mut bool_12: bool = crate::meta::filetype::FileType::is_dirlike(filetype_7);
    let mut bool_13: bool = crate::meta::filetype::FileType::is_dirlike(filetype_6);
    let mut bool_14: bool = crate::meta::filetype::FileType::is_dirlike(filetype_5);
    let mut bool_15: bool = crate::meta::filetype::FileType::is_dirlike(filetype_4);
    let mut bool_16: bool = crate::meta::filetype::FileType::is_dirlike(filetype_3);
    let mut bool_17: bool = crate::meta::filetype::FileType::is_dirlike(filetype_2);
    let mut bool_18: bool = crate::meta::filetype::FileType::is_dirlike(filetype_1);
    let mut bool_19: bool = crate::meta::filetype::FileType::is_dirlike(filetype_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_385() {
//    rusty_monitor::set_test_id(385);
    let mut method_0: meta::access_control::Method = crate::meta::access_control::Method::Smack;
    let mut method_0_ref_0: &meta::access_control::Method = &mut method_0;
    let mut method_1: meta::access_control::Method = crate::meta::access_control::Method::Acl;
    let mut method_1_ref_0: &meta::access_control::Method = &mut method_1;
    let mut method_2: meta::access_control::Method = crate::meta::access_control::Method::Acl;
    let mut method_2_ref_0: &meta::access_control::Method = &mut method_2;
    let mut method_3: meta::access_control::Method = crate::meta::access_control::Method::Acl;
    let mut method_3_ref_0: &meta::access_control::Method = &mut method_3;
    let mut method_4: meta::access_control::Method = crate::meta::access_control::Method::Acl;
    let mut method_4_ref_0: &meta::access_control::Method = &mut method_4;
    let mut method_5: meta::access_control::Method = crate::meta::access_control::Method::Smack;
    let mut method_5_ref_0: &meta::access_control::Method = &mut method_5;
    let mut method_6: meta::access_control::Method = crate::meta::access_control::Method::Acl;
    let mut method_6_ref_0: &meta::access_control::Method = &mut method_6;
    let mut method_7: meta::access_control::Method = crate::meta::access_control::Method::Acl;
    let mut method_7_ref_0: &meta::access_control::Method = &mut method_7;
    let mut method_8: meta::access_control::Method = crate::meta::access_control::Method::Acl;
    let mut method_8_ref_0: &meta::access_control::Method = &mut method_8;
    let mut method_9: meta::access_control::Method = crate::meta::access_control::Method::Selinux;
    let mut method_9_ref_0: &meta::access_control::Method = &mut method_9;
    let mut method_10: meta::access_control::Method = crate::meta::access_control::Method::Smack;
    let mut method_10_ref_0: &meta::access_control::Method = &mut method_10;
    let mut str_0: &str = crate::meta::access_control::Method::name(method_10_ref_0);
    let mut str_1: &str = crate::meta::access_control::Method::name(method_9_ref_0);
    let mut str_2: &str = crate::meta::access_control::Method::name(method_8_ref_0);
    let mut str_3: &str = crate::meta::access_control::Method::name(method_7_ref_0);
    let mut str_4: &str = crate::meta::access_control::Method::name(method_6_ref_0);
    let mut str_5: &str = crate::meta::access_control::Method::name(method_5_ref_0);
    let mut str_6: &str = crate::meta::access_control::Method::name(method_4_ref_0);
    let mut str_7: &str = crate::meta::access_control::Method::name(method_3_ref_0);
    let mut str_8: &str = crate::meta::access_control::Method::name(method_2_ref_0);
    let mut str_9: &str = crate::meta::access_control::Method::name(method_1_ref_0);
    let mut str_10: &str = crate::meta::access_control::Method::name(method_0_ref_0);
//    panic!("From RustyUnit with love");
}
}