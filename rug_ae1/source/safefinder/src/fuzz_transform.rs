use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt::format;
use std::iter::Map;
use std::process::{exit, id};
use std::ptr::null_mut;
use serde::de::Unexpected::Str;
use rustc_ast::Mutability;
use rustc_hir::def_id::DefId;
use rustc_hir::def::Res;
use rustc_middle::hir::nested_filter;
use rustc_middle::mir::visit::*;
use rustc_span::{Span, Symbol, FileNameDisplayPreference};
use rustc_middle::ty::TyCtxt;
use rustc_hir::Mod;
use rustc_middle::ty::Ty as TyS;
use rustc_middle::ty::subst::EarlyBinder;
use rustc_middle::ty::TyKind as TyKindS;
use rustc_middle::ty::Uint;
use rustc_middle::ty::{IntTy, UintTy, FloatTy, VariantDef, VariantDiscr, Visibility, PredicateKind, Clause};
use rustc_middle::ty::subst::{GenericArg, GenericArgKind};
use rustc_middle::ty::RegionKind;
use std::string::String;
use rustc_hir::def::DefKind;
use rustc_middle::ty::Ty;
use rustc_middle::mir::{Terminator, Location, TerminatorKind, VarDebugInfoContents};
use rustc_middle::ty::Binder;
use rustc_target::abi::{VariantIdx, FieldsShape, Variants, Primitive, TagEncoding, Scalar};
use rustc_target::abi::Scalar::Union;
use rustc_target::abi::Scalar::Initialized;
use rustc_hir::definitions::{DefPathData, DisambiguatedDefPathData};
use crate::rustc_index::vec::Idx;
use std::ops::Deref;
use std::env;
use rustc_middle::ty::ParamEnvAnd;
use rustc_middle::ty::ParamEnv;
use rustc_middle::ty::PolyFnSig;
use crate::util::{donot_deref, transfer_func_name};
use crate::util::has_template;
use rustc_target::abi::Size;
use crate::util::avoid_late_bound;
use crate::rustc_middle::ty::TypeVisitable;

pub struct FuzzTransformer<'tcx> {
    tcx: TyCtxt<'tcx>,
    ctxt: Vec<String>,
    visiting: bool,
    build_: BTreeMap<String, usize>,
    impl_: BTreeMap<String, String>,
    visit_funcs: Vec<(PolyFnSig<'tcx>, DefId)>,
    visit_func_args: Vec<Ty<'tcx>>
}

impl<'tcx> FuzzTransformer<'tcx> {
    pub fn new(tx: TyCtxt<'tcx>) -> Self {
        FuzzTransformer {
            tcx: tx,
            ctxt: Vec::new(),
            visiting: false,
            build_: BTreeMap::new(),
            impl_: BTreeMap::new(),
            visit_funcs: Vec::new(),
            visit_func_args: Vec::new()
        }
    }
}


impl<'tcx> FuzzTransformer<'tcx> {

    pub fn analyze(&mut self) {
        let mut ty_set = BTreeSet::<String>::new();
        for def_id in self.tcx.hir().body_owners() {
            let def_kind = self.tcx.def_kind(def_id);
            // Find the DefId for the entry point, note that the entry point must be a function
            if def_kind == DefKind::Fn || def_kind == DefKind::AssocFn {

                let modid = self.tcx.parent_module_from_def_id(def_id);
                let item_name = self.tcx.item_name(def_id.to_def_id());
                let fnsig = self.tcx.fn_sig(def_id.to_def_id());
                let def_path = self.tcx.def_path_str(def_id.to_def_id());
                if def_path.contains("tests_llm_16") || def_path.contains("tests_rug_"){
                    let b = self.tcx.optimized_mir(def_id);

                    for (idx, local_decl) in b.local_decls.iter_enumerated() {
                        let ty = local_decl.ty;
                        for var_debug_info in &b.var_debug_info {
                            // Match the local using `source_info.scope`
                            if let VarDebugInfoContents::Place(place) = var_debug_info.value {
                                if place.local == idx {
                                    // If the local matches, return the name as a string
                                    let name = var_debug_info.name.to_string();
                                    if name.to_string().starts_with("rug_fuzz_"){
                                        println!("{}~{}~{}~{}", def_path, item_name, name.to_string(), ty);
                                    }
                                }
                            }
                        }
                    }
                }


            }
        }

    }

}

impl<'tcx> Visitor<'tcx> for FuzzTransformer<'tcx> {

    fn visit_ty(&mut self, ty: Ty<'tcx>, _: TyContext) {
        if let TyKindS::FnDef(def_id, subref) = ty.kind() {
            let fnsig = EarlyBinder(self.tcx.fn_sig(def_id)).subst(self.tcx, subref);
            self.visit_funcs.push((fnsig, *def_id));
        }
        self.super_ty(ty);
    }
}