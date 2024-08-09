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
mod tests_llm_16_266 {
    use super::*;
    use crate::*;
    #[test]
    fn test_new() {
        let _rug_st_tests_llm_16_266_rrrruuuugggg_test_new = 0;
        let rug_fuzz_0 = "test_user";
        let rug_fuzz_1 = "test_group";
        let user = String::from(rug_fuzz_0);
        let group = String::from(rug_fuzz_1);
        let owner = Owner::new(user.clone(), group.clone());
        debug_assert_eq!(owner.user, user);
        debug_assert_eq!(owner.group, group);
        let _rug_ed_tests_llm_16_266_rrrruuuugggg_test_new = 0;
    }
}
#[cfg(test)]
mod tests_rug_103 {
    use super::*;
    use crate::color::{Colors, ThemeOption};
    use crate::meta::owner::Owner;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_103_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = "user";
        let rug_fuzz_1 = "group";
        let mut p0 = Owner::new(rug_fuzz_0.to_string(), rug_fuzz_1.to_string());
        let mut p1 = Colors::new(ThemeOption::Default);
        crate::meta::owner::Owner::render_user(&p0, &p1);
        let _rug_ed_tests_rug_103_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_104 {
    use super::*;
    use crate::meta::owner::Owner;
    use crate::color::{Colors, ThemeOption, Elem, ColoredString};
    #[test]
    fn test_render_group() {
        let _rug_st_tests_rug_104_rrrruuuugggg_test_render_group = 0;
        let rug_fuzz_0 = "user";
        let rug_fuzz_1 = "group";
        let mut p0 = Owner::new(rug_fuzz_0.to_string(), rug_fuzz_1.to_string());
        let mut p1 = Colors::new(ThemeOption::Default);
        Owner::render_group(&p0, &p1);
        let _rug_ed_tests_rug_104_rrrruuuugggg_test_render_group = 0;
    }
}
