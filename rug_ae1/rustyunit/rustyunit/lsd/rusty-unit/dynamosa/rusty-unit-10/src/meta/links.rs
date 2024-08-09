use crate::color::{ColoredString, Colors, Elem};
use std::fs::Metadata;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Links {
    nlink: Option<u64>,
}

impl<'a> From<&'a Metadata> for Links {
    #[cfg(unix)]
    fn from(meta: &Metadata) -> Self {
        use std::os::unix::fs::MetadataExt;

        let nlink = meta.nlink();

        Self { nlink: Some(nlink) }
    }

    #[cfg(windows)]
    fn from(_: &Metadata) -> Self {
        Self { nlink: None }
    }
}

impl Links {
    pub fn render(&self, colors: &Colors) -> ColoredString {
        match self.nlink {
            Some(i) => colors.colorize(i.to_string(), &Elem::Links { valid: true }),
            None => colors.colorize(String::from("-"), &Elem::Links { valid: false }),
        }
    }
}

#[cfg(test)]
#[cfg(unix)]
mod tests {
    use super::Links;
    use std::env;
    use std::io;
    use std::path::Path;
    use std::process::{Command, ExitStatus};

    fn cross_platform_touch(path: &Path) -> io::Result<ExitStatus> {
        Command::new("touch").arg(&path).status()
    }

    #[test]
    fn test_hardlinks_no_zero() {
        let mut file_path = env::temp_dir();
        file_path.push("inode.tmp");

        let success = cross_platform_touch(&file_path).unwrap().success();
        assert!(success, "failed to exec touch");

        let links = Links::from(&file_path.metadata().unwrap());

        #[cfg(unix)]
        assert!(links.nlink.is_some());
        #[cfg(windows)]
        assert!(links.nlink.is_none());
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6153() {
    rusty_monitor::set_test_id(6153);
    let mut str_0: &str = "sHZ";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "cDXHa5NIpAthV";
    let mut elem_0: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut bool_0: bool = false;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut elem_1: color::Elem = crate::color::Elem::NonFile;
    let mut bool_1: bool = true;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_2: color::Elem = crate::color::Elem::Older;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut option_0: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_2: bool = false;
    let mut option_1: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_3: color::Elem = crate::color::Elem::DayOld;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_4: color::Elem = crate::color::Elem::User;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_3_ref_0);
    let mut elem_5: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_6: color::Elem = crate::color::Elem::Read;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_6_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut elem_7: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_7_ref_0, theme_5_ref_0);
    let mut theme_6: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_6_ref_0: &crate::color::theme::Theme = &mut theme_6;
    let mut elem_8: color::Elem = crate::color::Elem::SymLink;
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_8_ref_0, theme_6_ref_0);
    let mut theme_7: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_7_ref_0: &crate::color::theme::Theme = &mut theme_7;
    let mut elem_9: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_9_ref_0, theme_7_ref_0);
    let mut dir_0: crate::color::theme::Dir = crate::color::theme::Dir {uid: color_5, no_uid: color_4};
    let mut theme_8: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_8_ref_0: &crate::color::theme::Theme = &mut theme_8;
    let mut elem_10: color::Elem = crate::color::Elem::User;
    let mut elem_10_ref_0: &color::Elem = &mut elem_10;
    let mut color_6: crossterm::style::Color = crate::color::Elem::get_color(elem_10_ref_0, theme_8_ref_0);
    let mut theme_9: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_9_ref_0: &crate::color::theme::Theme = &mut theme_9;
    let mut bool_3: bool = false;
    let mut elem_11: color::Elem = crate::color::Elem::Links {valid: bool_3};
    let mut elem_11_ref_0: &color::Elem = &mut elem_11;
    let mut color_7: crossterm::style::Color = crate::color::Elem::get_color(elem_11_ref_0, theme_9_ref_0);
    let mut elem_10: color::Elem = crate::color::Elem::User;
    let mut elem_10_ref_0: &color::Elem = &mut elem_2;
    let mut color_6: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_1_ref_0);
    let mut bool_3: bool = false;
    let mut elem_11: color::Elem = crate::color::Elem::Links {valid: bool_1};
    let mut elem_11_ref_0: &color::Elem = &mut elem_1;
    let mut theme_10: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut bool_4: bool = false;
    let mut elem_12: color::Elem = crate::color::Elem::INode {valid: bool_0};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_13: color::Elem = crate::color::Elem::Octal;
    let mut elem_12_ref_0: &color::Elem = &mut elem_12;
    let mut bool_5: bool = false;
    let mut elem_14: color::Elem = crate::color::Elem::File {exec: bool_4, uid: bool_5};
    let mut elem_13_ref_0: &color::Elem = &mut elem_13;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut option_2: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_1_ref_0: &str = &mut str_1;
    let mut option_3: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut theme_10_ref_0: &crate::color::theme::Theme = &mut theme_10;
    let mut bool_6: bool = crate::color::Elem::has_suid(elem_13_ref_0);
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut elem_15: color::Elem = crate::color::Elem::ExecSticky;
    let mut bool_7: bool = crate::meta::filetype::FileType::is_dirlike(filetype_0);
    let mut elem_16: color::Elem = crate::color::Elem::Exec;
    panic!("From RustyUnit with love");
}
}