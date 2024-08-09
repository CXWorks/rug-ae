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
#[timeout(30000)]fn rusty_test_8804() {
//    rusty_monitor::set_test_id(8804);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Byte;
    let mut elem_0: color::Elem = crate::color::Elem::Octal;
    let mut u64_0: u64 = 0u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut u64_1: u64 = 2u64;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Name;
    let mut bool_0: bool = true;
    let mut block_0_ref_0: &flags::blocks::Block = &mut block_0;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut elem_1: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::Mega;
    let mut unit_0_ref_0: &meta::size::Unit = &mut unit_0;
    let mut unit_2: meta::size::Unit = crate::meta::size::Unit::Mega;
    let mut unit_1_ref_0: &meta::size::Unit = &mut unit_1;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut unit_3: meta::size::Unit = crate::meta::size::Unit::Kilo;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut elem_2: color::Elem = crate::color::Elem::Links {valid: bool_0};
    let mut block_1: flags::blocks::Block = crate::flags::blocks::Block::User;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut elem_3: color::Elem = crate::color::Elem::FileSmall;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut elem_4: color::Elem = crate::color::Elem::User;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut displayoption_0: meta::name::DisplayOption = crate::meta::name::DisplayOption::None;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut displayoption_1: meta::name::DisplayOption = crate::meta::name::DisplayOption::FileName;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut block_1_ref_0: &flags::blocks::Block = &mut block_1;
//    panic!("From RustyUnit with love");
}
}