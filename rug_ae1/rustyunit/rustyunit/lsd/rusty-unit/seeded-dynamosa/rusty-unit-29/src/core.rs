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

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_9064() {
//    rusty_monitor::set_test_id(9064);
    let mut str_0: &str = "Byte";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "Xrc9sbdGog";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "mYnukpyJCauM7";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "fsi";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut usize_0: usize = 0usize;
    let mut tuple_0: (usize, &str) = (usize_0, str_3_ref_0);
    let mut str_4: &str = "mp3";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut usize_1: usize = 2usize;
    let mut tuple_1: (usize, &str) = (usize_1, str_4_ref_0);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut u64_0: u64 = 1048576u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut bool_0: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_0};
    let mut bool_1: bool = false;
    let mut bool_2: bool = false;
    let mut bool_3: bool = true;
    let mut bool_4: bool = false;
    let mut bool_5: bool = false;
    let mut bool_6: bool = false;
    let mut bool_7: bool = true;
    let mut bool_8: bool = false;
    let mut bool_9: bool = true;
    let mut bool_10: bool = true;
    let mut bool_11: bool = true;
    let mut bool_12: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_12, user_write: bool_11, user_execute: bool_10, group_read: bool_9, group_write: bool_8, group_execute: bool_7, other_read: bool_6, other_write: bool_5, other_execute: bool_4, sticky: bool_3, setgid: bool_2, setuid: bool_1};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut u64_1: u64 = 1024u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut str_5: &str = "sizesort";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut option_1: std::option::Option<usize> = std::option::Option::None;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut displayoption_0: meta::name::DisplayOption = crate::meta::name::DisplayOption::FileName;
    let mut displayoption_0_ref_0: &meta::name::DisplayOption = &mut displayoption_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut bool_13: bool = true;
    let mut bool_14: bool = true;
    let mut bool_15: bool = true;
    let mut bool_16: bool = false;
    let mut bool_17: bool = true;
    let mut bool_18: bool = true;
    let mut bool_19: bool = false;
    let mut bool_20: bool = false;
    let mut bool_21: bool = true;
    let mut bool_22: bool = true;
    let mut bool_23: bool = true;
    let mut bool_24: bool = true;
    let mut permissions_1: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_24, user_write: bool_23, user_execute: bool_22, group_read: bool_21, group_write: bool_20, group_execute: bool_19, other_read: bool_18, other_write: bool_17, other_execute: bool_16, sticky: bool_15, setgid: bool_14, setuid: bool_13};
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Special;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Exec;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::DayOld;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut sorting_0_ref_0: &crate::flags::sorting::Sorting = &mut sorting_0;
    let mut elem_4: color::Elem = crate::color::Elem::Group;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
//    panic!("From RustyUnit with love");
}
}