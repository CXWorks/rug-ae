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

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7500() {
    rusty_monitor::set_test_id(7500);
    let mut option_0: std::option::Option<usize> = std::option::Option::None;
    let mut option_1: std::option::Option<usize> = std::option::Option::None;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut str_0: &str = "mtOhvb8";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "24VzMtiXUyN";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut option_2: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_0: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_0};
    let mut str_2: &str = "T9vI1O0";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut option_3: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut option_4: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_5: std::option::Option<bool> = std::option::Option::None;
    let mut option_6: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_7: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_8: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_9: std::option::Option<bool> = std::option::Option::None;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    panic!("From RustyUnit with love");
}
}