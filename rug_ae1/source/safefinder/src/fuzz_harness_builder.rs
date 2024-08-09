use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
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
use rustc_middle::mir::{Terminator, Location, TerminatorKind};
use rustc_middle::ty::Binder;
use rustc_target::abi::{VariantIdx, FieldsShape, Variants, Primitive, TagEncoding, Scalar};
use rustc_target::abi::Scalar::Union;
use rustc_target::abi::Scalar::Initialized;
use rustc_hir::definitions::{DefPathData, DisambiguatedDefPathData};
use crate::rustc_index::vec::Idx;
use std::ops::Deref;
use std::env;
use serde_json::json;
use rustc_middle::ty::ParamEnvAnd;
use rustc_middle::ty::ParamEnv;
use crate::util::{check_is_constructor, get_primitive, impl_to_self, is_template, layer_of_ref, resolve_visible_path};
use crate::util::is_primitive;
use crate::util::has_template;
use rustc_hir::*;
use rustc_hir::QPath::Resolved;
use rustc_hir::{Item, ItemKind};
use rustc_hir::intravisit::{Visitor};
pub struct DecomposeChecker<'tcx> {
    tcx: TyCtxt<'tcx>,
    ctxt: Vec<String>,
    visiting: bool,
    map_: BTreeMap<String, String>,
    impl_: BTreeMap<String, Vec<DefId>>,
}

impl<'tcx> DecomposeChecker<'tcx> {
    pub fn new(tx: TyCtxt<'tcx>) -> Self {
        DecomposeChecker {
            tcx: tx,
            ctxt: Vec::new(),
            visiting: false,
            map_: BTreeMap::new(),
            impl_: BTreeMap::new(),
        }
    }
}



impl<'tcx> DecomposeChecker<'tcx> {
    pub fn analyze(&mut self) {
        let mut ty_set = BTreeSet::<String>::new();

        for def_id in self.tcx.hir_crate_items(()).definitions() {
            let def_kind = self.tcx.def_kind(def_id);
            let def_path = self.tcx.def_path_str(def_id.to_def_id());
            if def_kind == DefKind::Impl  {
                let item = self.tcx.hir().expect_item(def_id);

                if let ItemKind::Impl(iimpl) = item.kind{

                    if let TyKind::Path(path_resolve) = &iimpl.self_ty.kind {
                        if let Resolved(_, res) = &path_resolve {
                            if let Res::Def(kind, id) = res.res{
                                let tys = self.tcx.def_path_str(id);
                                // println!("{}", tys);
                                // let tys = format!("{:?}", ty);
                                if let Some(vv) = self.impl_.get_mut(&*tys){
                                    vv.push(def_id.to_def_id());
                                }else {
                                    let mut v = Vec::new();
                                    v.push(def_id.to_def_id());
                                    self.impl_.insert(tys, v);
                                }
                            }
                        }
                    }

                }


            }
        }


        for def_id in self.tcx.hir_crate_items(()).definitions() {
            let def_kind = self.tcx.def_kind(def_id);
            // Find the DefId for the entry point, note that the entry point must be a function
            if def_kind == DefKind::Fn || def_kind == DefKind::AssocFn {
                let mut modid = self.tcx.parent_module_from_def_id(def_id);
                loop {
                    let hir_id = self.tcx.hir().local_def_id_to_hir_id(modid);
                    let parent_def_id = self.tcx.parent_module_from_def_id(modid);

                    if parent_def_id == modid {
                        // We've reached the root
                        break;
                    }

                    modid = parent_def_id;
                }
                // let modid = self.tcx.parent_module_from_def_id(def_id);
                let item_name = self.tcx.item_name(def_id.to_def_id());

                let fnsig = self.tcx.fn_sig(def_id.to_def_id());

                // if item_name.to_string().eq("rust_fuzzer_test_input") || item_name.to_string().eq("run"){
                let file_path = self.tcx
                    .sess
                    .source_map().span_to_filename(self.tcx.def_span(def_id.to_def_id()))
                    .display(FileNameDisplayPreference::Local)
                    .to_string();
                let vis = self.tcx.visibility(def_id.to_def_id());
                let mut lifetimes = Vec::new();

                // if !file_path.starts_with("/") {
                    let fty = self.tcx.type_of(def_id.to_def_id());
                    if let TyKindS::FnDef(d, subst) = fty.kind() {
                        //identify lifetimes
                        for binder in *subst {
                            if let GenericArgKind::Lifetime(lt) = binder.unpack() {
                                if let RegionKind::ReEarlyBound(early) = lt.kind() {
                                    if early.has_name() {
                                        lifetimes.push(String::from(early.name.as_str()));
                                    }
                                }
                            }
                        }


                        //
                        let inner = fnsig.inputs().skip_binder();
                        let mut is_str = Vec::new();

                        let mut idx = 0;
                        let mut deps = HashMap::new();
                        let mut candidates = HashMap::new();
                        let mut topo_sort = Vec::new();
                        self.resolve_template(d, &mut deps, &mut candidates, &mut topo_sort);
                        println!("-----------------");
                        println!("{} {}", file_path, self.tcx.def_path_str(def_id.to_def_id()));
                        if !lifetimes.is_empty() {
                            println!("{}", lifetimes.join(","));
                        }
                        println!("deps:{}", json!(deps));
                        println!("candidates:{}", json!(candidates));
                        idx = -1;
                        for vv in inner {
                            let mut v = vv;
                            let depth = layer_of_ref(*v);
                            idx += 1;
                            if let TyKindS::Ref(_, rty, mt) = v.kind() {
                                let mut tys = format!("{}", rty);
                                if *mt != Mutability::Mut {
                                    if "str".eq_ignore_ascii_case(&format!("{}", rty)) {
                                        println!("let mut p{} = \"sample\"; // None+&str", idx);
                                        is_str.push(true);
                                        continue;
                                    } else if "[u8]".eq_ignore_ascii_case(&format!("{}", rty)) {
                                        println!("let mut p{} = [0u8,0,0].as_mut_slice(); // None+&[u8]", idx);
                                        is_str.push(false);
                                        continue;
                                    }else {
                                        is_str.push(false);
                                        if is_primitive(*rty) {
                                            if get_primitive(*rty).is_none(){
                                                println!("err {}", tys);
                                            }
                                            println!("let mut p{} = & {}; // None+{:?}", idx, get_primitive(*rty).unwrap(), rty);
                                        } else {
                                            let mut template = format!("let mut p{} = &", idx);
                                            v = rty;
                                            while let TyKindS::Ref(_, rty, mt) = v.kind() {
                                                template += " &";
                                                v = rty;
                                            }
                                            template += &*format!(" MaybeUninit::uninit().assume_init(); // {}", format!("{}", v));
                                            println!("{}", template);
                                        }
                                    }
                                }else {
                                    is_str.push(false);
                                    if is_primitive(*rty) {
                                        println!("let mut p{} = &mut {}; // None+{:?}", idx, get_primitive(*rty).unwrap(), rty);
                                    } else {
                                        let mut template = format!("let mut p{} = &mut ", idx);
                                        v = rty;
                                        while let TyKindS::Ref(_, rty, mt) = v.kind() {
                                            template += " &mut";
                                            v = rty;
                                        }
                                        template += &*format!(" MaybeUninit::uninit().assume_init(); // {}", format!("{}", v));
                                        println!("{}", template);
                                    }
                                }
                            }else {
                                is_str.push(false);
                                let tys = format!("{}", v);
                                if is_primitive(*v) {
                                    println!("let mut p{} = {}; // None+{}", idx, get_primitive(*v).unwrap(), tys);
                                } else {
                                    println!("let mut p{} = MaybeUninit::uninit().assume_init(); // {}", idx, tys);
                                }
                            }

                        }
                        // check
                        let func_def = format!("{}", self.tcx.def_path_str(def_id.to_def_id()));
                        if item_name.as_str().eq("fmt") && func_def.contains("std::fmt::Debug"){
                            println!("+format!(\"{{:?}}\", p0);");
                        }
                        if item_name.as_str().eq("fmt") && func_def.contains("std::fmt::Display"){
                            println!("+format!(\"{{}}\", p0);");
                        }

                        let arg_names = self.tcx.fn_arg_names(def_id.to_def_id());
                        if arg_names.len() > 0 && "self".eq_ignore_ascii_case(arg_names[0].name.as_str()) {
                            //the first arg is self, case 1
                            let mut call_stmt = format!("p0.{}(", item_name);
                            for i in 1..arg_names.len() {
                                if *is_str.get(i).unwrap() {
                                    call_stmt += "&";
                                }
                                call_stmt += &*("p".to_owned() + &i.to_string());
                                if i != arg_names.len() - 1 {
                                    call_stmt += ", "
                                }
                            }
                            call_stmt += ");";
                            println!("+{}", call_stmt);
                        }
                        let mut call_stmt = format!("{}", self.tcx.def_path_str(def_id.to_def_id()));
                        call_stmt += "(";
                        for i in 0..arg_names.len() {
                            if *is_str.get(i).unwrap() {
                                call_stmt += "&";
                            }
                            call_stmt += &*("p".to_owned() + &i.to_string());
                            if i != arg_names.len() - 1 {
                                call_stmt += ", "
                            }
                        }
                        call_stmt += ");";
                        println!("+{}", call_stmt);

                        //
                        let mut call_stmt = format!("{}", self.tcx.def_path_str(def_id.to_def_id()));
                        call_stmt += "(";
                        for i in 0..arg_names.len() {
                            if *is_str.get(i).unwrap() {
                                call_stmt += "&";
                            }
                            call_stmt += &*("p".to_owned() + &i.to_string());
                            if i != arg_names.len() - 1 {
                                call_stmt += ", "
                            }
                        }
                        call_stmt += ");";
                        println!("+crate::{}", call_stmt);
                        //case 3
                        let mut call_stmt_template = format!("{}", resolve_visible_path(&mut self.tcx, def_id.to_def_id(), modid.to_def_id()));
                        let mut call_stmt = format!("{}(", resolve_visible_path(&mut self.tcx, def_id.to_def_id(), modid.to_def_id()));
                        call_stmt_template += "(";
                        for i in 0..arg_names.len() {
                            if *is_str.get(i).unwrap() {
                                call_stmt += "&";
                                call_stmt_template += "&"
                            }
                            call_stmt += &*("p".to_owned() + &i.to_string());
                            call_stmt_template += &*("p".to_owned() + &i.to_string());
                            if i != arg_names.len() - 1 {
                                call_stmt += ", ";
                                call_stmt_template += ", ";
                            }
                        }
                        call_stmt += ");";
                        call_stmt_template += ");";
                        println!("+{}", call_stmt);
                        if call_stmt_template.len() > call_stmt.len() {
                            println!("+{}", call_stmt_template);
                        }
                    }
                // }
            }

            // break;
        }
    }



    fn resolve_template(&mut self, d: &DefId, deps: &mut HashMap<String, HashMap<String, HashSet<String>>>, candidates: &mut HashMap<String, HashMap<String, HashSet<String>>>, topo_sort: &mut Vec<String>){

        let def_path = self.tcx.def_path_str(*d);
        if let Some(_) = deps.get(&def_path){
            return;
        }
        topo_sort.push(def_path.clone());

        let mut generic_preds = self.tcx.predicates_of(*d);

        let mut v = Vec::new();
        loop {
            let predicates = generic_preds.predicates;
            for (pred, _) in predicates {
                if let PredicateKind::Clause(clause) = pred.kind().skip_binder() {
                    if let Clause::Trait(trait_pred) = clause {
                        let template_param = format!("{}", trait_pred.trait_ref.substs[0]);
                        v.push(template_param);
                    }
                }
            }
            if let Some(def_id) = generic_preds.parent {
                generic_preds = self.tcx.predicates_of(def_id);
            } else {
                break;
            }
        }
        for tp in v{
            self.recursive_resolve_template_dependencies(tp, d, deps, candidates, topo_sort);
        }
    }

    fn recursive_resolve_template_dependencies(&mut self, template: String, parent_bounds: &DefId, deps: &mut HashMap<String, HashMap<String, HashSet<String>>>, candidates: &mut HashMap<String, HashMap<String, HashSet<String>>>, topo_sort: &mut Vec<String>){

        let def_path = self.tcx.def_path_str(*parent_bounds);


        let mut template_preds = HashMap::<String, HashSet<String>>::new();
        let mut template_preds_id = HashMap::<String, HashSet<DefId>>::new();
        let mut wl = Vec::new();
        wl.push(*parent_bounds);
        if let Some(v) = self.impl_.get(&def_path){
            wl.extend(v.iter());
        }
        for parent in wl{
            let mut generic_preds = self.tcx.predicates_of(parent);

            loop {
                let predicates = generic_preds.predicates;
                for (pred, _) in predicates {
                    if let PredicateKind::Clause(clause) = pred.kind().skip_binder() {
                        if let Clause::Trait(trait_pred) = clause {
                            let template_param = format!("{}", trait_pred.trait_ref.substs[0]);
                            if template_param.eq(&template){
                                if let Some(v) = template_preds.get_mut(&*template_param) {
                                    v.insert(self.tcx.def_path_str(trait_pred.trait_ref.def_id));
                                } else {
                                    let mut v = HashSet::new();
                                    v.insert(self.tcx.def_path_str(trait_pred.trait_ref.def_id));
                                    template_preds.insert(template_param.clone(), v);
                                }
                                if let Some(v) = template_preds_id.get_mut(&*template_param) {
                                    v.insert(trait_pred.trait_ref.def_id);
                                } else {
                                    let mut v = HashSet::new();
                                    v.insert(trait_pred.trait_ref.def_id);
                                    template_preds_id.insert(template_param, v);
                                }
                            }

                        }
                    }
                }
                if let Some(def_id) = generic_preds.parent {
                    generic_preds = self.tcx.predicates_of(def_id);
                } else {
                    break;
                }
            }
        }

        //now for each template, we already get the preds, we need to select the preds
        let mut template_binder = HashMap::<String, Vec<DefId>>::with_capacity(template_preds.len());
        let mut template_selection = HashMap::<String, HashSet<String>>::with_capacity(template_preds.len());
        if let Some(pds) = deps.get_mut(&def_path.clone()){
            for (k,v) in template_preds{
                if let Some(pds_v) = pds.get_mut(k.as_str()){
                    for vv in v{
                        pds_v.insert(vv);
                    }
                }else {
                    pds.insert(k, v);
                }
            }
        }else {
            deps.insert(def_path.clone(), template_preds);
        }

        for (name, v) in template_preds_id {
            let mut temp_hashtb = HashMap::<DefId, u8>::new();
            let mut expect = v.len();

            for trait_id in v{
                let trait_def = self.tcx.trait_def(trait_id);
                if "std::marker::Sized".eq_ignore_ascii_case(&format!("{:?}", trait_def)) {
                    expect -= 1;
                    continue;
                }
                for (_, impl_ids) in self.tcx.trait_impls_of(trait_id).non_blanket_impls() {

                    for impl_id in impl_ids{

                        if let Some(val) = temp_hashtb.get_mut(&impl_id) {
                            *val += 1;
                        } else {
                            temp_hashtb.insert(*impl_id, 1u8);
                        }
                    }
                }
                for impl_id in self.tcx.trait_impls_of(trait_id).blanket_impls() {
                    if let Some(val) = temp_hashtb.get_mut(&impl_id) {
                        *val += 1;
                    } else {
                        temp_hashtb.insert(*impl_id, 1u8);
                    }
                }
            }
            let mut dedup: HashMap<String, Vec<DefId>> = HashMap::new();
            for (k, val) in temp_hashtb {
                let real_ty = self.tcx.type_of(k);

                if let Some(v) = dedup.get_mut(&(real_ty.to_string())){
                    v.push(k);
                }else {
                    let mut hs = Vec::new();
                    hs.push(k);
                    dedup.insert(real_ty.to_string(), hs);
                }
            }
            for (_, val) in dedup {
                let k = *val.get(0).unwrap();
                if val.len() == expect {
                    let real_ty = self.tcx.type_of(k);
                    if !is_template(real_ty) {
                        if let Some(tys) = template_binder.get_mut(&*name) {
                            tys.push(k);
                        } else {
                            let mut tys = Vec::new();
                            tys.push(k);
                            template_binder.insert(name.clone(), tys);
                        }
                        if has_template(real_ty){
                            if let Some(local) = k.as_local(){
                                let item = self.tcx.hir().expect_item(local);
                                if let ItemKind::Impl(iimpl) = item.kind{
                                    if let TyKind::Path(path_resolve) = &iimpl.self_ty.kind {
                                        if let Resolved(_, res) = &path_resolve {
                                            if let Res::Def(kind, id) = res.res {
                                                self.resolve_template(&id, deps, candidates, topo_sort);
                                            }
                                        }
                                    }
                                }
                            }else {

                            }
                        }
                    }else {
                        let tm = format!("{}", real_ty);
                        self.recursive_resolve_template_dependencies(tm,&k, deps, candidates, topo_sort );
                        let mut hs :HashSet<String> = HashSet::new();
                        hs.insert(String::from(self.tcx.def_path_str(k)));
                        template_selection.insert(name.clone(), hs);
                    }
                }
            }
            if expect == 0 {
                let mut hs :HashSet<String> = HashSet::new();
                hs.insert(String::from("RUG_ANY"));
                template_selection.insert(name.clone(), hs);
            }
        }
        //start selection

        for (k, v) in &template_binder {
            let mut priority = Vec::new();
            for candidate in v {
                if candidate.is_local() {
                    priority.push(candidate);
                }
            }
            //primitive type
            if priority.is_empty() {
                for candidate in v {
                    let real_ty = self.tcx.type_of(candidate);
                    let ty_name = format!("{:?}", real_ty);
                    if "i8 u8 char i16 u16 i32 u32 i64 u64 bool usize".contains(&ty_name) {
                        priority.push(candidate);
                    }
                }
            }
            //std or core
            if priority.is_empty() {
                for candidate in v {
                    let real_ty = self.tcx.type_of(candidate);
                    let ty_name = format!("{:?}", real_ty);
                    if ty_name.contains("std::") || ty_name.contains("core::") {
                        priority.push(candidate);
                    }
                }
            }
            if priority.is_empty() {
                for candidate in v {
                    let real_ty = self.tcx.type_of(candidate);
                    let ty_name = format!("{:?}", real_ty);
                    priority.push(candidate);
                }
            }
            let mut hs = HashSet::with_capacity(priority.len());
            for did in priority{
                // println!("------------ {}", self.tcx.def_path_str(*did));
                if let Some(local) = did.as_local(){
                    let item = self.tcx.hir().expect_item(local);
                    if let ItemKind::Impl(iimpl) = item.kind{
                        if let TyKind::Path(path_resolve) = &iimpl.self_ty.kind {
                            if let Resolved(_, res) = &path_resolve {
                                if let Res::Def(kind, id) = res.res {
                                    // println!("** {}", self.tcx.def_path_str(id));
                                    hs.insert(self.tcx.def_path_str(id));
                                }
                            }
                        }
                    }
                }else {
                    // let parent_def_id = DefId { index: self.tcx.def_key(*did).parent.unwrap(), ..*did };
                    hs.insert(self.tcx.type_of(*did).to_string());
                }


            }
            if let Some(es) = template_selection.get_mut(&*k.clone()){
                for d in hs{
                    es.insert(d);
                }
            }else {
                template_selection.insert((*k.clone()).parse().unwrap(), hs);
            }
        }

        let generics = self.tcx.generics_of(parent_bounds);

        for param in generics.params.iter() {
            match param.kind {
                // If the parameter is a type parameter, it might have a default.
                rustc_middle::ty::GenericParamDefKind::Type { has_default, .. } => {
                    if has_default {
                        // We can get the default, if it exists, by using the `type_of` query.
                        // The `type_of` query returns the "own" type of the item, which for
                        // type parameters with defaults is that default.
                        let default = self.tcx.type_of(param.def_id);
                        if let Some(es) = template_selection.get_mut(&param.name.to_string()){
                            es.insert(default.to_string());
                        }else {
                            let mut hs = HashSet::new();
                            hs.insert(default.to_string());
                            template_selection.insert(param.name.to_string(), hs);
                        }
                        // Here `default` is the default type for the type parameter.
                        // Note: It may be a further generic type that depends on other parameters.
                    }
                }
                // For consts and lifetimes, handle accordingly (lifetimes generally don't have defaults)
                _ => {}
            }
        }
        if let Some(pds) = candidates.get_mut(&def_path.clone()){
            for (k,v) in template_selection{
                if let Some(pds_v) = pds.get_mut(k.as_str()){
                    for vv in v{
                        pds_v.insert(vv);
                    }
                }else {
                    pds.insert(k, v);
                }
            }
        }else {
            candidates.insert(def_path.clone(), template_selection);
        }

    }
}