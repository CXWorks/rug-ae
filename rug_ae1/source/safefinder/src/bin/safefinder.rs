#![feature(backtrace)]
#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_errors;
extern crate rustc_interface;

#[macro_use]
extern crate log;

use std::env;

use rustc_driver::Compilation;
use rustc_interface::{interface::Compiler, Queries};
use safefinder::{analyze, compile_time_sysroot};


pub struct RudraCompilerCalls {

}

impl RudraCompilerCalls {
    fn new() -> RudraCompilerCalls {
        Self{}
    }
}

impl rustc_driver::Callbacks for RudraCompilerCalls {
    fn after_analysis<'tcx>(
        &mut self,
        compiler: &Compiler,
        queries: &'tcx Queries<'tcx>,
    ) -> Compilation {
        compiler.session().abort_if_errors();


        debug!(
            "Input file name: {}",
            compiler.input().source_name().prefer_local()
        );
        debug!("Crate name: {}", queries.crate_name().unwrap().peek_mut());


        queries.global_ctxt().unwrap().peek_mut().enter(|tcx| {
            analyze(tcx);
        });


        compiler.session().abort_if_errors();
        Compilation::Stop
    }
}

/// Execute a compiler with the given CLI arguments and callbacks.
fn run_compiler(
    mut args: Vec<String>,
    callbacks: &mut (dyn rustc_driver::Callbacks + Send),
) -> i32 {
    // Make sure we use the right default sysroot. The default sysroot is wrong,
    // because `get_or_default_sysroot` in `librustc_session` bases that on `current_exe`.
    //
    // Make sure we always call `compile_time_sysroot` as that also does some sanity-checks
    // of the environment we were built in.
    // FIXME: Ideally we'd turn a bad build env into a compile-time error via CTFE or so.
    if let Some(sysroot) = compile_time_sysroot() {
        let sysroot_flag = "--sysroot";
        if !args.iter().any(|e| e == sysroot_flag) {
            // We need to overwrite the default that librustc_session would compute.
            args.push(sysroot_flag.to_owned());
            args.push(sysroot);
        }
    }



    // Invoke compiler, and handle return code.
    let exit_code = rustc_driver::catch_with_exit_code(move || {
        rustc_driver::RunCompiler::new(&args, callbacks).run()
    });

    exit_code
}

fn parse_config() -> Vec<String> {
    // collect arguments

    let mut rustc_args = vec![];
    for arg in std::env::args() {
        match arg.as_str() {
            // "-Zrudra-enable-unsafe-destructor" => {
            //     config.unsafe_destructor_enabled = true;
            // }
            // "-Zrudra-disable-unsafe-destructor" => {
            //     config.unsafe_destructor_enabled = false;
            // }
            // "-Zrudra-enable-send-sync-variance" => config.send_sync_variance_enabled = true,
            // "-Zrudra-disable-send-sync-variance" => config.send_sync_variance_enabled = false,
            // "-Zrudra-enable-unsafe-dataflow" => config.unsafe_dataflow_enabled = true,
            // "-Zrudra-disable-unsafe-dataflow" => config.unsafe_dataflow_enabled = false,
            // "-v" => config.verbosity = Verbosity::Verbose,
            // "-vv" => config.verbosity = Verbosity::Trace,
            // "-Zsensitivity-high" => config.report_level = ReportLevel::Error,
            // "-Zsensitivity-med" => config.report_level = ReportLevel::Warning,
            // "-Zsensitivity-low" => config.report_level = ReportLevel::Info,
            _ => {
                rustc_args.push(arg);
            }
        }
    }

    rustc_args
}

fn main() {
    rustc_driver::install_ice_hook(); // ICE: Internal Compilation Error
    let exit_code = {
        // initialize the report logger
        // `logger_handle` must be nested because it flushes the logs when it goes out of the scope
        let mut rustc_args = parse_config();

        // init rustc logger
        if env::var_os("RUSTC_LOG").is_some() {
            rustc_driver::init_rustc_env_logger();
        }

        if let Some(sysroot) = compile_time_sysroot() {
            let sysroot_flag = "--sysroot";
            if !rustc_args.iter().any(|e| e == sysroot_flag) {
                // We need to overwrite the default that librustc would compute.
                rustc_args.push(sysroot_flag.to_owned());
                rustc_args.push(sysroot);
            }
        }


        debug!("rustc arguments: {:?}", &rustc_args);
        run_compiler(rustc_args, &mut RudraCompilerCalls::new())
    };

    std::process::exit(exit_code)
}
