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
#[timeout(30000)]fn rusty_test_656() {
//    rusty_monitor::set_test_id(656);
    let mut method_0: meta::access_control::Method = crate::meta::access_control::Method::Acl;
    let mut method_0_ref_0: &meta::access_control::Method = &mut method_0;
    let mut method_1: meta::access_control::Method = crate::meta::access_control::Method::Acl;
    let mut method_1_ref_0: &meta::access_control::Method = &mut method_1;
    let mut method_2: meta::access_control::Method = crate::meta::access_control::Method::Selinux;
    let mut method_2_ref_0: &meta::access_control::Method = &mut method_2;
    let mut method_3: meta::access_control::Method = crate::meta::access_control::Method::Selinux;
    let mut method_3_ref_0: &meta::access_control::Method = &mut method_3;
    let mut method_4: meta::access_control::Method = crate::meta::access_control::Method::Smack;
    let mut method_4_ref_0: &meta::access_control::Method = &mut method_4;
    let mut method_5: meta::access_control::Method = crate::meta::access_control::Method::Acl;
    let mut method_5_ref_0: &meta::access_control::Method = &mut method_5;
    let mut method_6: meta::access_control::Method = crate::meta::access_control::Method::Selinux;
    let mut method_6_ref_0: &meta::access_control::Method = &mut method_6;
    let mut method_7: meta::access_control::Method = crate::meta::access_control::Method::Selinux;
    let mut method_7_ref_0: &meta::access_control::Method = &mut method_7;
    let mut method_8: meta::access_control::Method = crate::meta::access_control::Method::Smack;
    let mut method_8_ref_0: &meta::access_control::Method = &mut method_8;
    let mut method_9: meta::access_control::Method = crate::meta::access_control::Method::Acl;
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

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7104() {
//    rusty_monitor::set_test_id(7104);
    let mut option_0: std::option::Option<usize> = std::option::Option::None;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_0: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut u64_0: u64 = 71u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut str_0: &str = "char_device";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut app_0: clap::App = crate::app::build();
    let mut option_1: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
//    panic!("From RustyUnit with love");
}
}