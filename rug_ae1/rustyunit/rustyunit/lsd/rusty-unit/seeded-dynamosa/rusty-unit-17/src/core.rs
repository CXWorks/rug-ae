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
#[timeout(30000)]fn rusty_test_8428() {
//    rusty_monitor::set_test_id(8428);
    let mut str_0: &str = "hyperlink";
    let mut str_1: &str = "gsheet";
    let mut str_2: &str = "Indicators";
    let mut str_3: &str = ".zshrc";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "WORD";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "yCqJqEQB8kJana";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7: &str = "less";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_8: &str = "cp";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut str_9: &str = "Always";
    let mut str_9_ref_0: &str = &mut str_9;
    let mut str_10: &str = "%F";
    let mut str_10_ref_0: &str = &mut str_10;
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_11: &str = "or";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_12: &str = "";
    let mut str_11_ref_0: &str = &mut str_11;
    let mut str_13: &str = ".bashprofile";
    let mut str_12_ref_0: &str = &mut str_12;
    let mut str_14: &str = "U4lLFj8laRZ";
    let mut str_13_ref_0: &str = &mut str_13;
    let mut str_14_ref_0: &str = &mut str_14;
    let mut result_0: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_1_ref_0);
    let mut result_1: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_7_ref_0);
    let mut result_2: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_4_ref_0);
    let mut result_3: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_9_ref_0);
    let mut result_4: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_5_ref_0);
    let mut result_5: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_6_ref_0);
    let mut result_6: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_11_ref_0);
    let mut result_7: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_13_ref_0);
    let mut result_8: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_14_ref_0);
    let mut result_9: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_12_ref_0);
    let mut result_10: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_10_ref_0);
    let mut tuple_0: () = std::result::Result::unwrap(result_4);
//    panic!("From RustyUnit with love");
}
}