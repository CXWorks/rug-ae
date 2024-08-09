use crate::color::{ColoredString, Colors, Elem};
use crate::flags::Flags;
use std::fs::read_link;
use std::path::Path;
#[derive(Clone, Debug)]
pub struct SymLink {
    target: Option<String>,
    valid: bool,
}
impl<'a> From<&'a Path> for SymLink {
    fn from(path: &'a Path) -> Self {
        if let Ok(target) = read_link(path) {
            if target.is_absolute() || path.parent() == None {
                return Self {
                    valid: target.exists(),
                    target: Some(
                        target
                            .to_str()
                            .expect("failed to convert symlink to str")
                            .to_string(),
                    ),
                };
            }
            return Self {
                target: Some(
                    target
                        .to_str()
                        .expect("failed to convert symlink to str")
                        .to_string(),
                ),
                valid: path.parent().unwrap().join(target).exists(),
            };
        }
        Self { target: None, valid: false }
    }
}
impl SymLink {
    pub fn symlink_string(&self) -> Option<String> {
        self.target.as_ref().map(|target| target.to_string())
    }
    pub fn render(&self, colors: &Colors, flag: &Flags) -> ColoredString {
        if let Some(target_string) = self.symlink_string() {
            let elem = if self.valid {
                &Elem::SymLink
            } else {
                &Elem::MissingSymLinkTarget
            };
            let strings: &[ColoredString] = &[
                ColoredString::new(
                    Colors::default_style(),
                    format!(" {} ", flag.symlink_arrow),
                ),
                colors.colorize(target_string, elem),
            ];
            let res = strings
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join("");
            ColoredString::new(Colors::default_style(), res)
        } else {
            ColoredString::new(Colors::default_style(), "".into())
        }
    }
}
#[cfg(test)]
mod tests {
    use super::SymLink;
    use crate::app;
    use crate::color::{Colors, ThemeOption};
    use crate::config_file::Config;
    use crate::flags::Flags;
    #[test]
    fn test_symlink_render_default_valid_target_nocolor() {
        let link = SymLink {
            target: Some("/target".to_string()),
            valid: true,
        };
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            format!("{}", " ⇒ /target"), link.render(&
            Colors::new(ThemeOption::NoColor), & Flags::configure_from(& matches, &
            Config::with_none()).unwrap()).to_string()
        );
    }
    #[test]
    fn test_symlink_render_default_invalid_target_nocolor() {
        let link = SymLink {
            target: Some("/target".to_string()),
            valid: false,
        };
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            format!("{}", " ⇒ /target"), link.render(&
            Colors::new(ThemeOption::NoColor), & Flags::configure_from(& matches, &
            Config::with_none()).unwrap()).to_string()
        );
    }
    #[test]
    fn test_symlink_render_default_invalid_target_withcolor() {
        let link = SymLink {
            target: Some("/target".to_string()),
            valid: false,
        };
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            format!("{}", " ⇒ \u{1b}[38;5;124m/target\u{1b}[39m"), link.render(&
            Colors::new(ThemeOption::NoLscolors), & Flags::configure_from(& matches, &
            Config::with_none()).unwrap()).to_string()
        );
    }
}
#[cfg(test)]
mod tests_llm_16_133 {
    use super::*;
    use crate::*;
    use std::path::Path;
    #[test]
    fn test_from() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let path = Path::new(rug_fuzz_0);
        let symlink = SymLink::from(path);
        debug_assert_eq!(symlink.target, None);
        debug_assert_eq!(symlink.valid, false);
        let path2 = Path::new(rug_fuzz_1);
        let symlink2 = SymLink::from(path2);
        debug_assert_eq!(symlink2.target, Some("absolute/path".to_string()));
        debug_assert_eq!(symlink2.valid, true);
        let path3 = Path::new(rug_fuzz_2);
        let symlink3 = SymLink::from(path3);
        debug_assert_eq!(symlink3.target, Some("relative/path".to_string()));
        debug_assert_eq!(symlink3.valid, true);
        let path4 = Path::new(rug_fuzz_3);
        let symlink4 = SymLink::from(path4);
        debug_assert_eq!(symlink4.target, Some("non-existent/path".to_string()));
        debug_assert_eq!(symlink4.valid, false);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_285 {
    use super::*;
    use crate::*;
    #[test]
    fn test_symlink_string_with_target() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let symlink = SymLink {
            target: Some(String::from(rug_fuzz_0)),
            valid: rug_fuzz_1,
        };
        debug_assert_eq!(symlink.symlink_string(), Some(String::from("target")));
             }
});    }
    #[test]
    fn test_symlink_string_without_target() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let symlink = SymLink {
            target: None,
            valid: rug_fuzz_0,
        };
        debug_assert_eq!(symlink.symlink_string(), None);
             }
});    }
}
