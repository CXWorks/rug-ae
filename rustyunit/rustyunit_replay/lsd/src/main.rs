#![allow(
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::match_same_arms,
    clippy::cast_possible_wrap
)]
#![feature(no_coverage)]
#[macro_use]
extern crate clap;
extern crate chrono;
extern crate chrono_humanize;
extern crate dirs;
extern crate libc;
extern crate lscolors;
#[cfg(test)]
extern crate tempfile;
extern crate term_grid;
extern crate terminal_size;
extern crate unicode_width;
extern crate url;
extern crate wild;
extern crate xdg;
extern crate yaml_rust;
#[cfg(unix)]
extern crate users;
#[cfg(windows)]
extern crate winapi;
pub use ntest::timeout;
pub mod app;
pub mod color;
pub mod config_file;
pub mod core;
pub mod display;
pub mod flags;
pub mod icon;
pub mod meta;
pub mod sort;
use crate::config_file::Config;
use crate::core::Core;
use crate::flags::Flags;
use std::path::PathBuf;
/// Macro used to avoid panicking when the lsd method is used with a pipe and
/// stderr close before our program.
#[macro_export]
macro_rules! print_error {
    ($($arg:tt)*) => {
        { use std::io::Write; let stderr = std::io::stderr(); { let mut handle = stderr
        .lock(); let res = handle.write_all(std::format!("lsd: {}\n\n",
        std::format!($($arg)*)) .as_bytes()); if res.is_err() { std::process::exit(0); }
        } }
    };
}
/// Macro used to avoid panicking when the lsd method is used with a pipe and
/// stdout close before our program.
#[macro_export]
macro_rules! print_output {
    ($($arg:tt)*) => {
        use std::io::Write; let stderr = std::io::stdout(); { let mut handle = stderr
        .lock(); let res = handle.write_all(std::format!($($arg)*) .as_bytes()); if res
        .is_err() { std::process::exit(0); } }
    };
}
fn main() {
    let matches = app::build().get_matches_from(wild::args_os());
    let inputs = matches
        .values_of("FILE")
        .expect("failed to retrieve cli value")
        .map(PathBuf::from)
        .collect();
    let config = if matches.is_present("ignore-config") {
        Config::with_none()
    } else if matches.is_present("config-file") {
        let path = matches
            .value_of("config-file")
            .expect("Invalid config file path")
            .into();
        Config::from_file(path).expect("Provided file path is invalid")
    } else {
        Config::default()
    };
    let flags = Flags::configure_from(&matches, &config)
        .unwrap_or_else(|err| err.exit());
    let core = Core::new(flags);
    core.run(inputs);
}
#[cfg(test)]
mod tests_rug_27 {
    use super::*;
    use std::path::PathBuf;
    #[test]
    fn test_main() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(&str, &str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let matches = app::build().get_matches_from(wild::args_os());
        let inputs = matches
            .values_of(rug_fuzz_0)
            .expect(rug_fuzz_1)
            .map(PathBuf::from)
            .collect::<Vec<PathBuf>>();
        let config = if matches.is_present(rug_fuzz_2) {
            Config::with_none()
        } else if matches.is_present(rug_fuzz_3) {
            let path = matches.value_of(rug_fuzz_4).expect(rug_fuzz_5).into();
            Config::from_file(path).expect(rug_fuzz_6)
        } else {
            Config::default()
        };
        let flags = Flags::configure_from(&matches, &config)
            .unwrap_or_else(|err| err.exit());
        let core = Core::new(flags);
        core.run(inputs);
             }
}
}
}    }
}
