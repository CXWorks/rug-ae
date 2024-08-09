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
        let access_control = AccessControl::from_data(true, &[], &[]);
        assert_eq!(
            String::from("+").with(Color::DarkCyan), access_control.render_method(&
            Colors::new(ThemeOption::Default))
        );
    }
    #[test]
    fn test_smack_only_indicator() {
        let access_control = AccessControl::from_data(false, &[], &[b'a']);
        assert_eq!(
            String::from(".").with(Color::Cyan), access_control.render_method(&
            Colors::new(ThemeOption::Default))
        );
    }
    #[test]
    fn test_acl_and_selinux_indicator() {
        let access_control = AccessControl::from_data(true, &[b'a'], &[]);
        assert_eq!(
            String::from("+").with(Color::DarkCyan), access_control.render_method(&
            Colors::new(ThemeOption::Default))
        );
    }
    #[test]
    fn test_selinux_context() {
        let access_control = AccessControl::from_data(false, &[b'a'], &[]);
        assert_eq!(
            String::from("a").with(Color::Cyan), access_control.render_context(&
            Colors::new(ThemeOption::Default))
        );
    }
    #[test]
    fn test_selinux_and_smack_context() {
        let access_control = AccessControl::from_data(false, &[b'a'], &[b'b']);
        assert_eq!(
            String::from("a+b").with(Color::Cyan), access_control.render_context(&
            Colors::new(ThemeOption::Default))
        );
    }
    #[test]
    fn test_no_context() {
        let access_control = AccessControl::from_data(false, &[], &[]);
        assert_eq!(
            String::from("?").with(Color::Cyan), access_control.render_context(&
            Colors::new(ThemeOption::Default))
        );
    }
}
#[cfg(test)]
mod tests_llm_16_235 {
    use super::*;
    use crate::*;
    use std::path::Path;
    #[test]
    #[cfg(unix)]
    fn test_for_path_unix() {
        let _rug_st_tests_llm_16_235_rrrruuuugggg_test_for_path_unix = 0;
        let rug_fuzz_0 = "/path/to/file";
        let path = Path::new(rug_fuzz_0);
        let result = AccessControl::for_path(&path);
        let _rug_ed_tests_llm_16_235_rrrruuuugggg_test_for_path_unix = 0;
    }
    #[test]
    #[cfg(not(unix))]
    fn test_for_path_non_unix() {
        let _rug_st_tests_llm_16_235_rrrruuuugggg_test_for_path_non_unix = 0;
        let rug_fuzz_0 = "/path/to/file";
        let path = Path::new(rug_fuzz_0);
        let result = AccessControl::for_path(&path);
        let _rug_ed_tests_llm_16_235_rrrruuuugggg_test_for_path_non_unix = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_236 {
    use super::*;
    use crate::*;
    use std::path::Path;
    #[test]
    #[cfg(not(unix))]
    fn test_from_data() {
        let _rug_st_tests_llm_16_236_rrrruuuugggg_test_from_data = 0;
        let rug_fuzz_0 = false;
        let has_acl = rug_fuzz_0;
        let selinux_context = [];
        let smack_context = [];
        let access_control = AccessControl::from_data(
            has_acl,
            &selinux_context,
            &smack_context,
        );
        debug_assert_eq!(access_control.has_acl, false);
        debug_assert_eq!(access_control.selinux_context, "");
        debug_assert_eq!(access_control.smack_context, "");
        let _rug_ed_tests_llm_16_236_rrrruuuugggg_test_from_data = 0;
    }
    #[test]
    #[cfg(unix)]
    fn test_from_data() {
        let _rug_st_tests_llm_16_236_rrrruuuugggg_test_from_data = 0;
        let rug_fuzz_0 = "test.txt";
        let path = Path::new(rug_fuzz_0);
        let access_control = AccessControl::for_path(&path);
        debug_assert_eq!(access_control.has_acl, false);
        debug_assert_eq!(access_control.selinux_context, "");
        debug_assert_eq!(access_control.smack_context, "");
        let _rug_ed_tests_llm_16_236_rrrruuuugggg_test_from_data = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_242_llm_16_241 {
    #[cfg(unix)]
    #[test]
    fn test_name() {
        let _rug_st_tests_llm_16_242_llm_16_241_rrrruuuugggg_test_name = 0;
        use crate::meta::access_control::Method;
        let acl = Method::Acl;
        let selinux = Method::Selinux;
        let smack = Method::Smack;
        debug_assert_eq!(acl.name(), "system.posix_acl_access");
        debug_assert_eq!(selinux.name(), "security.selinux");
        debug_assert_eq!(smack.name(), "security.SMACK64");
        let _rug_ed_tests_llm_16_242_llm_16_241_rrrruuuugggg_test_name = 0;
    }
}
#[cfg(test)]
mod tests_rug_87 {
    use super::*;
    use std::path::Path;
    use crate::color;
    use crate::color::{Colors, ColoredString, Elem};
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_87_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = "/path/to/file";
        let mut p0 = AccessControl::for_path(Path::new(rug_fuzz_0));
        let mut p1 = Colors::new(color::ThemeOption::Default);
        p0.render_method(&p1);
        let _rug_ed_tests_rug_87_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_88 {
    use super::*;
    use std::path::Path;
    use crate::color;
    #[test]
    fn test_render_context() {
        let _rug_st_tests_rug_88_rrrruuuugggg_test_render_context = 0;
        let rug_fuzz_0 = "/path/to/file";
        let mut p0 = AccessControl::for_path(Path::new(rug_fuzz_0));
        let mut p1 = color::Colors::new(color::ThemeOption::Default);
        p0.render_context(&p1);
        let _rug_ed_tests_rug_88_rrrruuuugggg_test_render_context = 0;
    }
}
