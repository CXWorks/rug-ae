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
use rustc_middle::mir::{Terminator, Location, TerminatorKind};
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
use rustc_hir::*;
use rustc_hir::QPath::Resolved;
use rustc_hir::{Item, ItemKind};
use rustc_hir::intravisit::{Visitor};

pub(crate) fn transfer_func_name(full_name: String) -> String {
    return full_name.clone().replace("::", "__")
        .replace("<", "_st_")
        .replace(">", "_ed_")
        .replace(",", "_")
        .replace("-", "_")
        .replace("&", "_ad_")
        .replace("'", "_lt_")
        .replace("(", "_lp_")
        .replace(")", "_rp_")
        .replace("[", "_lm_")
        .replace("]", "_rm_")
        .replace("*", "_ptr_")
        .replace("=", "_eq_")
        .replace("+", "_add_")
        .replace(";", "_cut_")
        .replace("^", "_xor_")
        .replace(" ", "");
}

pub(crate) fn resolve_visible_path(tcx: &mut TyCtxt, defId: DefId, parent_mod: DefId) ->String{
    let mut key = tcx.def_key(defId);
    let mut key_def_id = defId;
    let mut visible_path = Vec::new();
    let k2 = tcx.def_key(parent_mod);
    loop {
        if key == k2{
            visible_path.push(String::from("crate"));
            break;
        }
        match key.disambiguated_data.data {
            DefPathData::Impl => {
                let generics = tcx.generics_of(key_def_id);
                let self_ty = tcx.bound_type_of(key_def_id);
                let impl_trait_ref = tcx.bound_impl_trait_ref(key_def_id);
                visible_path.push(format!("<{:?}>", self_ty.0));
                visible_path.reverse();

                return visible_path.join("::");

            }
            DefPathData::TypeNs(sym) => {
                visible_path.push(format!("{}", sym));
            }
            DefPathData::ValueNs(sym) => {
                visible_path.push(format!("{}", sym));
            }
            DefPathData::MacroNs(sym) => {
                visible_path.push(format!("{}", sym));
            }
            DefPathData::LifetimeNs(sym) => {
                visible_path.push(format!("{}", sym));
            }
            DefPathData::CrateRoot => {
                visible_path.push(String::from(tcx.crate_name(key_def_id.krate).as_str()));
            }
            _=>{
                // println!("{:?}", key);
            }
        }
        if let Some(pid) = key.parent{
            key_def_id = DefId { index: pid, ..defId };
            key = tcx.def_key(key_def_id);
        }else {
            break;
        }
    }
    visible_path.reverse();
    visible_path.join("::")
}

pub(crate) fn layer_of_ref(vty: rustc_middle::ty::Ty) -> usize {
    match vty.kind() {
        TyKindS::Ref(_, rty, _) => {
            return 1+layer_of_ref(*rty);
        }
        _=>{
            return 0;
        }
    }
}

pub(crate) fn extract_of_ref(vty: rustc_middle::ty::Ty, layer: usize) -> rustc_middle::ty::Ty {
    match vty.kind() {
        TyKindS::Ref(_, rty, _) => {
            if layer == 1{
                return *rty;
            }else {
                return extract_of_ref(*rty, layer - 1);
            }

        }
        _=>{
            unreachable!()
        }
    }
}

pub(crate) fn donot_deref(vty: rustc_middle::ty::Ty) -> bool {
    match vty.kind() {
        TyKindS::Bool => {
            return true;
        }
        TyKindS::Char => {
            return true;
        }
        TyKindS::Str => {
            return true;
        }
        TyKindS::Int(int_ty) => {
            return true;
        }
        TyKindS::Uint(uint_ty) => {
            return true;
        }
        TyKindS::Float(float_ty) => {
            return true;
        }
        _=>{
            return false;
        }
    }
}
fn handle_int_ty(int_ty: &IntTy) -> String {
    let int_type = match int_ty {
        IntTy::Isize => "isize",
        IntTy::I8 => "i8",
        IntTy::I16 => "i16",
        IntTy::I32 => "i32",
        IntTy::I64 => "i64",
        IntTy::I128 => "i128",
    };
    String::from(int_type)
}

fn handle_uint_ty(uint_ty: &UintTy) -> String {
    let uint_type = match uint_ty {
        UintTy::Usize => "usize",
        UintTy::U8 => "u8",
        UintTy::U16 => "u16",
        UintTy::U32 => "u32",
        UintTy::U64 => "u64",
        UintTy::U128 => "u128",
    };
    String::from(uint_type)
}

fn handle_float_ty(float_ty: &FloatTy) -> String {
    let float_type = match float_ty {
        FloatTy::F32 => "f32",
        FloatTy::F64 => "f64",
    };
    String::from(float_type)
}

pub(crate) fn get_primitive(vty: rustc_middle::ty::Ty) -> Option<String> {
    if vty.is_unit(){
        return Some(String::from("()"));
    }
    match vty.kind() {
        TyKindS::Ref(_, rty, _) => {
            return get_primitive(*rty);
        }
        TyKindS::RawPtr(ty_mut) =>{
            return get_primitive(ty_mut.ty);
        }
        TyKindS::Array(ety, _) => {
            if let Some(ele) = get_primitive(*ety){
                return Some(format!("[{};1]", ele));
            }else {
                return None;
            }
        }
        TyKindS::Slice(ety) => {
            if let Some(ele) = get_primitive(*ety){
                return Some(format!("[{}].as_mut_slice()", ele));
            }else {
                return None;
            }
        }
        TyKindS::Bool => {
            return Some(String::from("true"));
        }
        TyKindS::Char => {
            return Some(String::from("'a'"));
        }
        TyKindS::Str => {
            return Some(String::from("\"a\""));
        }
        TyKindS::Int(int_ty) => {
            return Some(format!("0{}", handle_int_ty(int_ty)));
        }
        TyKindS::Uint(uint_ty) => {
            return Some(format!("0{}", handle_uint_ty(uint_ty)));
        }
        TyKindS::Float(float_ty) => {
            return Some(format!("0{}", handle_float_ty(float_ty)));
        }
        TyKindS::Adt(adt_def, sub_ref) => {
            let special_type_handler = format!("{:?}", vty);
            if special_type_handler.contains("std::string::String") {
                return Some(String::from("std::string::String::new()"));
            }else {
                return None;
            }
        }
        _=>{
            return None;
        }
    }
}
pub(crate) fn is_primitive(vty: rustc_middle::ty::Ty) -> bool {
    if vty.is_unit(){
        return true;
    }
    match vty.kind() {
        TyKindS::Ref(_, rty, _) => {
            return is_primitive(*rty);
        }
        TyKindS::RawPtr(ty_mut) =>{
            return is_primitive(ty_mut.ty);
        }
        TyKindS::Array(ety, _) => {
            return is_primitive(*ety);
        }
        TyKindS::Slice(ety) => {
            return is_primitive(*ety);
        }
        TyKindS::Bool => {
            return true;
        }
        TyKindS::Char => {
            return true;
        }
        TyKindS::Str => {
            return true;
        }
        TyKindS::Int(int_ty) => {
            return true;
        }
        TyKindS::Uint(uint_ty) => {
            return true;
        }
        TyKindS::Float(float_ty) => {
            return true;
        }
        TyKindS::Adt(adt_def, sub_ref) => {
            let special_type_handler = format!("{:?}", vty);
            if special_type_handler.contains("std::string::String") {
                return true;
            }else {
                return false;
            }
        }
        _=>{
            return false;
        }
    }
}


pub(crate) fn is_template(vty: TyS) -> bool {
    match vty.kind() {
        TyKindS::Ref(_, rty, _) => {
            return has_template(*rty);
        }
        TyKindS::RawPtr(ty_mut) =>{
            return has_template(ty_mut.ty);
        }
        TyKindS::Param(_)=>{
            return true;
        }
        _=>{
            return false;
        }
    }
}

pub(crate) fn has_template(vty: TyS) -> bool {
    match vty.kind() {
        TyKindS::Adt(_, sub_ref) =>  {
            if sub_ref.len() > 0 {
                return true;
            }
            for template_ty in *sub_ref {
                if let GenericArgKind::Type(tty) = template_ty.unpack() {
                    if let TyKindS::Param(_) = tty.kind() {
                        return true;
                    }
                }
            }
        }
        TyKindS::Ref(_, rty, _) => {
            return has_template(*rty);
        }
        TyKindS::RawPtr(ty_mut) =>{
            return has_template(ty_mut.ty);
        }
        TyKindS::Param(_)=>{
            return true;
        }
        _=>{
            return false;
        }
    }
    false
}

fn span2filepath(tcx: &mut TyCtxt, span: Span) -> String {
    return tcx
        .sess
        .source_map()
        .span_to_filename(span)
        .display(FileNameDisplayPreference::Remapped)
        .to_string();
}

pub(crate) fn avoid_late_bound<'tcx>(tcx: &mut TyCtxt<'tcx>, vec: &mut Vec<rustc_middle::ty::GenericArg<'tcx>>, substs: &[rustc_middle::ty::GenericArg<'tcx>]){
    for gene in substs{
        match gene.unpack()  {
            GenericArgKind::Lifetime(region) => {
                if let RegionKind::ReLateBound(_, _) = region.deref(){
                    // vec.push(GenericArg::from(tcx.mk_region(RegionKind::ReErased)));
                }else {
                    vec.push(*gene);
                }
            }
            _ =>{
                vec.push(*gene);
            }
        }
    }
}

pub(crate) fn impl_to_self<'tcx>(tcx: &mut TyCtxt<'tcx>, d: &DefId) -> Option<DefId>{
    if let Some(local) = d.as_local(){
        let item = tcx.hir().expect_item(local);
        if let ItemKind::Impl(iimpl) = item.kind{
            if let TyKind::Path(path_resolve) = &iimpl.self_ty.kind {
                if let Resolved(_, res) = &path_resolve {
                    if let Res::Def(kind, id) = res.res {
                        return Some(id);
                    }
                }
            }
        }
    }
    None
}

pub(crate) fn get_inner_ty(vty: rustc_middle::ty::Ty) -> Option<String> {
    if vty.is_unit() || is_template(vty){
        return None;
    }
    match vty.kind() {
        TyKindS::Adt(adt_def, sub_ref) => {
            let tys = format!("{:?}", adt_def);
            if tys.starts_with("std::result::Result"){
                return get_inner_ty(sub_ref[0].expect_ty());
            }else if tys.starts_with("std::option::Option") {
                return get_inner_ty(sub_ref[0].expect_ty());
            }else {
                return Some(tys);
            }
        }
        _=>{
            return Some(format!("{:?}", vty));
        }
    }
    None
}

pub(crate) fn check_is_constructor<'tcx>(tcx: &mut TyCtxt<'tcx>, f: PolyFnSig) -> Option<String>{
    let out = f.output();
    if let Some(out_ty) = get_inner_ty(out.skip_binder()){
        for input in f.inputs().skip_binder(){
            if let Some(ity) = get_inner_ty(*input){
                if ity.eq(&out_ty){
                    return None
                }
            }
        }
        return Some(out_ty)
    }
    None
}