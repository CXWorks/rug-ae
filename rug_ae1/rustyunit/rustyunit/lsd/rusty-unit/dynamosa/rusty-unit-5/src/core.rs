use crate::color::Colors;
use crate::display;
use crate::flags::{
    ColorOption, Display, Flags, HyperlinkOption, IconOption, IconTheme, Layout, SortOrder,
    ThemeOption,
};
use crate::icon::{self, Icons};
use crate::meta::Meta;
use crate::{print_error, print_output, sort};
use std::path::PathBuf;

#[cfg(not(target_os = "windows"))]
use std::io;
#[cfg(not(target_os = "windows"))]
use std::os::unix::io::AsRawFd;

#[cfg(target_os = "windows")]
use terminal_size::terminal_size;

pub struct Core {
    flags: Flags,
    icons: Icons,
    colors: Colors,
    sorters: Vec<(SortOrder, sort::SortFn)>,
}

impl Core {
    pub fn new(mut flags: Flags) -> Self {
        // Check through libc if stdout is a tty. Unix specific so not on windows.
        // Determine color output availability (and initialize color output (for Windows 10))
        #[cfg(not(target_os = "windows"))]
        let tty_available = unsafe { libc::isatty(io::stdout().as_raw_fd()) == 1 };

        #[cfg(not(target_os = "windows"))]
        let console_color_ok = true;

        #[cfg(target_os = "windows")]
        let tty_available = terminal_size().is_some(); // terminal_size allows us to know if the stdout is a tty or not.

        #[cfg(target_os = "windows")]
        let console_color_ok = crossterm::ansi_support::supports_ansi();

        let mut inner_flags = flags.clone();

        let color_theme = match (tty_available && console_color_ok, flags.color.when) {
            (_, ColorOption::Never) | (false, ColorOption::Auto) => ThemeOption::NoColor,
            _ => flags.color.theme.clone(),
        };

        let icon_theme = match (tty_available, flags.icons.when, flags.icons.theme) {
            (_, IconOption::Never, _) | (false, IconOption::Auto, _) => icon::Theme::NoIcon,
            (_, _, IconTheme::Fancy) => icon::Theme::Fancy,
            (_, _, IconTheme::Unicode) => icon::Theme::Unicode,
        };

        // TODO: Rework this so that flags passed downstream does not
        // have Auto option for any (icon, color, hyperlink).
        if matches!(flags.hyperlink, HyperlinkOption::Auto) {
            flags.hyperlink = if tty_available {
                HyperlinkOption::Always
            } else {
                HyperlinkOption::Never
            }
        }

        let icon_separator = flags.icons.separator.0.clone();

        if !tty_available {
            // The output is not a tty, this means the command is piped. (ex: lsd -l | less)
            //
            // Most of the programs does not handle correctly the ansi colors
            // or require a raw output (like the `wc` command).
            inner_flags.layout = Layout::OneLine;
        };

        let sorters = sort::assemble_sorters(&flags);

        Self {
            flags,
            colors: Colors::new(color_theme),
            icons: Icons::new(icon_theme, icon_separator),
            sorters,
        }
    }

    pub fn run(self, paths: Vec<PathBuf>) {
        let mut meta_list = self.fetch(paths);

        self.sort(&mut meta_list);
        self.display(&meta_list)
    }

    fn fetch(&self, paths: Vec<PathBuf>) -> Vec<Meta> {
        let mut meta_list = Vec::with_capacity(paths.len());
        let depth = match self.flags.layout {
            Layout::Tree { .. } => self.flags.recursion.depth,
            _ if self.flags.recursion.enabled => self.flags.recursion.depth,
            _ => 1,
        };

        for path in paths {
            let mut meta = match Meta::from_path(&path, self.flags.dereference.0) {
                Ok(meta) => meta,
                Err(err) => {
                    print_error!("{}: {}.", path.display(), err);
                    continue;
                }
            };

            let recurse =
                self.flags.layout == Layout::Tree || self.flags.display != Display::DirectoryOnly;
            if recurse {
                match meta.recurse_into(depth, &self.flags) {
                    Ok(content) => {
                        meta.content = content;
                        meta_list.push(meta);
                    }
                    Err(err) => {
                        print_error!("lsd: {}: {}\n", path.display(), err);
                        continue;
                    }
                };
            } else {
                meta_list.push(meta);
            };
        }
        // Only calculate the total size of a directory if it will be displayed
        if self.flags.total_size.0 && self.flags.blocks.displays_size() {
            for meta in &mut meta_list.iter_mut() {
                meta.calculate_total_size();
            }
        }

        meta_list
    }

    fn sort(&self, metas: &mut Vec<Meta>) {
        metas.sort_unstable_by(|a, b| sort::by_meta(&self.sorters, a, b));

        for meta in metas {
            if let Some(ref mut content) = meta.content {
                self.sort(content);
            }
        }
    }

    fn display(&self, metas: &[Meta]) {
        let output = if self.flags.layout == Layout::Tree {
            display::tree(metas, &self.flags, &self.colors, &self.icons)
        } else {
            display::grid(metas, &self.flags, &self.colors, &self.icons)
        };

        print_output!("{}", output);
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7670() {
    rusty_monitor::set_test_id(7670);
    let mut str_0: &str = "RGt";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut option_0: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_1: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_2: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_0: bool = true;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_0: usize = 62usize;
    let mut bool_1: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_1, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut option_4: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_5: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_2: bool = true;
    let mut option_6: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut bool_3: bool = true;
    let mut option_7: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut option_8: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut option_9: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_1);
    let mut option_10: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_11: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_12: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_4: bool = false;
    let mut option_13: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut option_14: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_15: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut option_16: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_1);
    let mut option_17: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_17, theme: option_16, separator: option_15};
    let mut option_18: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut option_19: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut bool_5: bool = true;
    let mut option_20: std::option::Option<bool> = std::option::Option::Some(bool_5);
    let mut option_21: std::option::Option<std::string::String> = std::option::Option::None;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut option_22: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_0);
    let mut option_23: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_23, theme: option_22};
    let mut option_24: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_25: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_26: std::option::Option<bool> = std::option::Option::None;
    let mut config_1: crate::config_file::Config = crate::config_file::Config {classic: option_26, blocks: option_25, color: option_24, date: option_21, dereference: option_20, display: option_19, icons: option_18, ignore_globs: option_14, indicators: option_13, layout: option_12, recursion: option_11, size: option_10, permission: option_9, sorting: option_8, no_symlink: option_7, total_size: option_6, symlink_arrow: option_5, hyperlink: option_4};
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut display_1: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_1};
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut bool_6: bool = false;
    let mut bool_7: bool = false;
    let mut bool_8: bool = false;
    let mut bool_9: bool = true;
    let mut bool_10: bool = true;
    let mut bool_11: bool = true;
    let mut bool_12: bool = true;
    let mut bool_13: bool = false;
    let mut bool_14: bool = false;
    let mut bool_15: bool = true;
    let mut bool_16: bool = true;
    let mut bool_17: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_17, user_write: bool_16, user_execute: bool_15, group_read: bool_14, group_write: bool_13, group_execute: bool_12, other_read: bool_11, other_write: bool_10, other_execute: bool_9, sticky: bool_8, setgid: bool_7, setuid: bool_6};
    let mut result_0: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_0_ref_0);
    panic!("From RustyUnit with love");
}
}