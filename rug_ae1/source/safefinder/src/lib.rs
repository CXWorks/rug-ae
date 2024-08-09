#![feature(rustc_private)]

mod chat_unit_gen;
mod fuzz_harness_builder;
mod fuzz_corpus_builder;
mod fuzz_decompose_builder;
mod fuzz_unit_test_builder;
mod util;
mod rug_verify;
mod fuzz_transform;
mod base_detector;

extern crate rustc_data_structures;
extern crate rustc_driver;
extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_hir_pretty;
extern crate rustc_index;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_span;
extern crate rustc_ast;
extern crate rustc_target;
extern crate libc;

use std::process;
use rustc_middle::ty::TyCtxt;
use crate::base_detector::BaseDetector;
use crate::fuzz_harness_builder::DecomposeChecker;
use crate::fuzz_corpus_builder::FuzzCorpusChecker;
use crate::fuzz_decompose_builder::CorpusDecomposeChecker;
use crate::chat_unit_gen::ChatUnitGen;
use crate::fuzz_transform::FuzzTransformer;
use crate::rug_verify::RugVerify;

pub fn compile_time_sysroot() -> Option<String> {
    let out = process::Command::new("rustc")
        .arg("--print=sysroot")
        .current_dir(".")
        .output()
        .unwrap();
    let sysroot = std::str::from_utf8(&out.stdout).unwrap().trim();
    Some(sysroot.parse().unwrap())
}

fn run_analysis<F, R>(name: &str, f: F) -> R
    where
        F: FnOnce() -> R,
{
    let result = f();
    result
}

pub fn analyze<'tcx>(tcx: TyCtxt<'tcx>) {

    // workaround to mimic arena lifetime
    // let rcx_owner = RudraCtxtOwner::new(tcx, config.report_level);
    // let rcx = &*Box::leak(Box::new(rcx_owner));
    //
    // // shadow the variable tcx
    // #[allow(unused_variables)]
    //     let tcx = ();
    //
    // // Unsafe destructor analysis
    // if config.unsafe_destructor_enabled {
    // run_analysis("Lifetime", || {
    //     let mut checker = LifetimeChecker::new(tcx);
    //     checker.analyze();
    // });
    if let Ok(step) = std::env::var("UNIT_GEN") {
        if step.eq("s1") {
            run_analysis("temp", || {
                let mut checker = DecomposeChecker::new(tcx);
                checker.analyze();
            });
        } else if step.eq("s2") {
            run_analysis("temp", || {
                let mut checker = FuzzCorpusChecker::new(tcx);
                checker.analyze();
            });
        } else if step.eq("s3") {
            run_analysis("temp", || {
                let mut checker = CorpusDecomposeChecker::new(tcx);
                checker.analyze();
            });
        }
    } else if let Ok(st) =  std::env::var("CHAT_UNIT") {
        run_analysis("temp", || {
            let mut checker = ChatUnitGen::new(tcx);
            checker.analyze();
        });
    } else if let Ok(st) =  std::env::var("FUZZ_TEST") {
        run_analysis("temp", || {
            let mut checker = FuzzTransformer::new(tcx);
            checker.analyze();
        });
    }else if let Ok(st) =  std::env::var("RUG_VERIFY") {
        run_analysis("temp", || {
            let mut checker = RugVerify::new(tcx);
            checker.analyze();
        });
    }
    else if let Ok(st) =  std::env::var("RUG_BASE") {
        run_analysis("temp", || {
            let mut checker = BaseDetector::new(tcx);
            checker.analyze();
        });
    }
    else {
        run_analysis("temp", || {
            let mut checker = DecomposeChecker::new(tcx);
            checker.analyze();
        });
    }
}

// run_analysis("temp", || {
//             let mut checker = FFIChecker::new(tcx);
//             checker.analyze();
//         });
//
// // Send/Sync variance analysis
// if config.send_sync_variance_enabled {
//     run_analysis("SendSyncVariance", || {
//         let checker = SendSyncVarianceChecker::new(rcx);
//         checker.analyze();
//     })
// }
//
// // Unsafe dataflow analysis
// if config.unsafe_dataflow_enabled {
//     run_analysis("UnsafeDataflow", || {
//         let checker = UnsafeDataflowChecker::new(rcx);
//         checker.analyze();
//     })
// }
