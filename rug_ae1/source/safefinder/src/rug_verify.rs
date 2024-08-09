use std::cmp::min;
use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};
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
use rustc_middle::middle::exported_symbols::*;
use rustc_middle::ty::Ty as TyS;
use rustc_middle::ty::subst::EarlyBinder;
use serde_json::json;
use rustc_middle::ty::Uint;
use rustc_middle::ty::{IntTy, UintTy, FloatTy, VariantDef, VariantDiscr, Visibility, PredicateKind, Clause};
use rustc_middle::ty::subst::{GenericArg, GenericArgKind};
use rustc_middle::ty::RegionKind;
use std::string::String;
use rustc_hir::def::DefKind;
use rustc_middle::ty::Ty;
use rustc_middle::mir::{Terminator, Location, TerminatorKind};
use rustc_middle::ty::Binder;
use rustc_target::abi::{VariantIdx, FieldsShape, Variants, Primitive, TagEncoding, Scalar};
use rustc_target::abi::Scalar::Union;
use rustc_target::abi::Scalar::Initialized;
use rustc_target::abi::Size;
use rustc_hir::definitions::{DefPathData, DisambiguatedDefPathData};
use crate::rustc_index::vec::Idx;
use std::ops::Deref;
use std::env;
use rustc_middle::ty::ParamEnvAnd;
use rustc_middle::ty::ParamEnv;
use rustc_middle::ty::PolyFnSig;
use crate::util::{is_template, transfer_func_name};
use crate::util::avoid_late_bound;
use rustc_hir::*;
use rustc_hir::QPath::Resolved;
use rustc_middle::ty::TyKind as TyKindS;
use std::io::Write;
use rustc_span::symbol::sym;
use crate::util::check_is_constructor;
use rustc_middle::mir::Place;
use rustc_middle::mir::Rvalue;
use rustc_middle::mir::*;



pub struct RugVerify<'tcx> {
    tcx: TyCtxt<'tcx>,
    srcs: BTreeMap<String, (String, String)>,
    trait_to_struct: BTreeMap<String, BTreeSet<String>>,
    struct_to_trait: BTreeMap<String, BTreeSet<String>>,
    dependencies: BTreeMap<String, BTreeSet<String>>,
    targets:BTreeMap<String, (String, String)>,
    self_to_fn: BTreeMap<String, BTreeSet<String>>,
    name_to_def_id: BTreeMap<String, DefId>,
    struct_to_def_id: BTreeMap<String, DefId>,
    type_to_def_path: BTreeMap<String, String>,
    struct_constructor: BTreeMap<String, BTreeSet<String>>,
    b: Option<&'tcx rustc_middle::mir::Body<'tcx>>,
    var_name: String,
    correct: bool
}

impl<'tcx> RugVerify<'tcx> {
    pub fn new(tx: TyCtxt<'tcx>) -> Self {
        RugVerify {
            tcx: tx,
            srcs: BTreeMap::new(),
            trait_to_struct: BTreeMap::new(),
            struct_to_trait: BTreeMap::new(),
            dependencies: BTreeMap::new(),
            targets: BTreeMap::new(),
            self_to_fn: BTreeMap::new(),
            name_to_def_id: BTreeMap::new(),
            struct_to_def_id: BTreeMap::new(),
            type_to_def_path: BTreeMap::new(),
            struct_constructor: BTreeMap::new(),
            b: None,
            var_name: String::new(),
            correct: false
        }
    }
}


impl<'tcx> RugVerify<'tcx> {


    pub fn analyze(&mut self) {
        if let Ok(mod_name) = std::env::var("MOD"){
            if let Ok(var_name) = std::env::var("VAR"){
                self.var_name = var_name;

                    for def_id in self.tcx.hir_crate_items(()).definitions() {
                        let def_kind = self.tcx.def_kind(def_id);
                        let def_path = self.tcx.def_path_str(def_id.to_def_id());
                        // Find the DefId for the entry point, note that the entry point must be a function
                        if def_kind == DefKind::Fn || def_kind == DefKind::AssocFn {

                            let modid = self.tcx.parent_module_from_def_id(def_id);
                            let item_name = self.tcx.item_name(def_id.to_def_id());
                            let item_name = self.tcx.item_name(def_id.to_def_id());
                            let fnsig = self.tcx.fn_sig(def_id.to_def_id());
                            let cur_hirid = self.tcx.hir().local_def_id_to_hir_id(def_id);
                            let fn_decl = self.tcx.hir().fn_decl_by_hir_id(cur_hirid);
                            if def_path.contains(&format!("{}::sample", mod_name)){

                                let b = self.tcx.optimized_mir(def_id);
                                self.b = Some(b);
                                let file_path: String = self.tcx
                                    .sess
                                    .source_map()
                                    .span_to_filename(b.span)
                                    .display(FileNameDisplayPreference::Remapped)
                                    .to_string();
                                let vis = self.tcx.visibility(def_id.to_def_id());
                                self.visit_body(b);
                            }
                        }
                    }

            }
        }


    }

}

impl<'tcx> Visitor<'tcx> for RugVerify<'tcx> {

    fn visit_place(
        &mut self,
        place: &  Place<'tcx>,
        context: PlaceContext,
        location: Location,
    ) {
        let local_decls =  &self.b.unwrap().local_decls;
        let local_decl = &local_decls[place.local];
        let ty = local_decl.ty;
        let span = local_decl.source_info.span;

// You can use this span to look up the variable name in the source map
        if let Ok(snippet) = self.tcx.sess.source_map().span_to_snippet(span) {
            if ("mut ".to_owned() + &self.var_name.clone()).eq(&snippet){
                println!("{:?}", ty)
            }
        } else {
        }
        self.super_place(place, context, location);
    }
}
