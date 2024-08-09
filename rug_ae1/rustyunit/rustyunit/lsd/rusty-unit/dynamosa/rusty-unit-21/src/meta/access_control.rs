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
fn rusty_test_2910() {
    rusty_monitor::set_test_id(2910);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_0: usize = 8453usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_1};
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut u64_0: u64 = 4007u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut themeoption_3_ref_0: &flags::color::ThemeOption = &mut themeoption_3;
    panic!("From RustyUnit with love");
}
}