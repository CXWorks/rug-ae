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
use crate::util::{donot_deref, transfer_func_name};
use crate::util::has_template;
use rustc_target::abi::Size;
use crate::util::avoid_late_bound;
use crate::rustc_middle::ty::TypeVisitable;

pub struct CorpusDecomposeChecker<'tcx> {
    tcx: TyCtxt<'tcx>,
    ctxt: Vec<String>,
    visiting: bool,
    build_: BTreeMap<String, usize>,
    impl_: BTreeMap<String, String>,
    visit_funcs: Vec<(PolyFnSig<'tcx>, DefId)>,
    visit_func_args: Vec<Ty<'tcx>>
}

impl<'tcx> CorpusDecomposeChecker<'tcx> {
    pub fn new(tx: TyCtxt<'tcx>) -> Self {
        CorpusDecomposeChecker {
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

fn handle_int_ty(int_ty: &IntTy, ref_name: String, name_offset: String) -> String {
    let int_type = match int_ty {
        IntTy::Isize => "isize",
        IntTy::I8 => "i8",
        IntTy::I16 => "i16",
        IntTy::I32 => "i32",
        IntTy::I64 => "i64",
        IntTy::I128 => "i128",
    };
    format!("print_int(\"{}\", {}, {});\n", int_type, ref_name, name_offset)
}

fn handle_uint_ty(uint_ty: &UintTy, ref_name: String, name_offset: String) -> String {
    let uint_type = match uint_ty {
        UintTy::Usize => "usize",
        UintTy::U8 => "u8",
        UintTy::U16 => "u16",
        UintTy::U32 => "u32",
        UintTy::U64 => "u64",
        UintTy::U128 => "u128",
    };
    format!("print_uint(\"{}\", {}, {});\n", uint_type, ref_name, name_offset)
}

fn handle_float_ty(float_ty: &FloatTy, ref_name: String, name_offset: String) -> String {
    let float_type = match float_ty {
        FloatTy::F32 => "f32",
        FloatTy::F64 => "f64",
    };
    format!("print_float(\"{}\", {}, {});\n", float_type, ref_name, name_offset)
}

fn handle_bool_ty(ref_name: String, name_offset: String) -> String {
    format!("print_bool({}, {});\n", ref_name, name_offset)
}

fn handle_str_ty(ref_name: String, name_offset: String) -> String {
    format!("print_str({}, {});\n", ref_name, name_offset)
}

fn handle_char_ty(ref_name: String, name_offset: String) -> String {
    format!("print_char({}, {});\n", ref_name, name_offset)
}

impl<'tcx> CorpusDecomposeChecker<'tcx> {
    fn resolve_visible_ty(&mut self, ty: TyS) -> String{
        //todo
        //perfect sol is to resolve this by my self
        //for demo/research purpose, just skip this process
        String::new()
    }

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
                if def_path.contains("fuzzdriver_") && def_path.contains("fuzz_unit_test") && def_path.contains("test_ss") {
                    let b = self.tcx.optimized_mir(def_id);

                    self.visit_body(b);
                    if let Some((fnsig, def_id)) = self.visit_funcs.last() {
                        println!("---------------------------------------------");
                        println!("{}", def_path);
                        let out_ty = fnsig.output();
                        let caller = self.resolve_ret_tys(out_ty.skip_binder(), String::from("ans"), String::from("name"));
                        println!("{}", caller);
                    }
                }


            }
        }

    }




    fn resolve_enum_type(&mut self, val: Primitive) -> &'static str {
        assert!(!val.is_float());
        match val {
            Primitive::Int(si, signed) => {
                let size = si.size().bytes_usize();
                if signed {
                    return match size {
                        1 => "i8",
                        2 => "i16",
                        4 => "i32",
                        8 => "i64",
                        16 => "i128",
                        _ => { unreachable!() }
                    };
                } else {
                    return match size {
                        1 => "u8",
                        2 => "u16",
                        4 => "u32",
                        8 => "u64",
                        16 => "u128",
                        _ => { unreachable!() }
                    };
                }
            }
            Primitive::Pointer => {
                return "usize";
            }
            _ => {
                unreachable!()
            }
        }
    }

    fn recursive_resolve_ret_ty(&mut self, ty: TyS<'tcx>, ref_name: String, name_offset: String) -> String {
        // println!("%%%% {:?}", ty);
        //all the ref_name here are ptrs inside the memory
        //we need to get the offset of the memory, then read it as primitive types and print
        match ty.kind() {
            TyKindS::Bool => {
                return handle_bool_ty(ref_name, name_offset);
            }
            TyKindS::Char => {
                return handle_char_ty(ref_name,  name_offset);
            }
            TyKindS::Str => {
                return handle_str_ty(ref_name,  name_offset);
            }
            TyKindS::Int(int_ty) => {
                return handle_int_ty(int_ty, ref_name,  name_offset);
            }
            TyKindS::Uint(uint_ty) => {
                return handle_uint_ty(uint_ty, ref_name,  name_offset);
            }
            TyKindS::Float(float_ty) => {
                return handle_float_ty(float_ty, ref_name,  name_offset);
            }
            TyKindS::Projection(proj) => {
                //todo revisit
            }
            TyKindS::Adt(adt_def, sub_ref) => {
                // todo reimplement using the binded types
                let real_ty = EarlyBinder(*adt_def).subst(self.tcx, &sub_ref);
                let field_adt = EarlyBinder(self.tcx.type_of(adt_def.did())).subst(self.tcx, &sub_ref);

                let param_env_and_ty = self.tcx.param_env(adt_def.did()).and(field_adt);
                if field_adt.has_escaping_bound_vars(){
                    return String::new();
                }
                let ty_layout = self.tcx.layout_of(param_env_and_ty).unwrap();
                let layout = ty_layout.layout;
                let fn_name = transfer_func_name(format!("print_{}", ty));
                if let Some(_) = self.build_.get(fn_name.as_str()){
                    //todo sth
                    return format!("{}({}, {});\n", fn_name, ref_name, name_offset);
                }else {
                    self.build_.insert(fn_name.clone(), 0);
                }
                let mut fn_def = format!("fn {}(ptr: usize, name_offset: std::string::String) {{\n unsafe{{\n let mut ptr_val = 0;\n let mut name_off_new = String::new();\n", fn_name);

                if let Variants::Single { index } = layout.variants() {
                    if 0 != index.index(){
                        match layout.fields() {
                            FieldsShape::Arbitrary { offsets, memory_index } => {
                                let mut has_data = false;
                                if let Some(v) = real_ty.variants().get(*index){
                                    has_data = true;
                                    let offset = offsets.get(0).expect("err").bytes_usize();
                                    let field_adt = EarlyBinder(self.tcx.type_of(v.def_id)).subst(self.tcx, &sub_ref);
                                    fn_def.push_str(format!("ptr_val = ptr + {};\n", offset).as_str());
                                    fn_def.push_str(format!("name_off_new = name_offset.clone() + \"~{}\";\n", v.name).as_str());
                                    let stmt = self.recursive_resolve_ret_ty(field_adt, String::from("ptr_val"), String::from("name_off_new"));
                                    fn_def.push_str(stmt.as_str());
                                }
                                if !has_data {
                                    fn_def.push_str(format!("name_off_new = name_offset.clone() + \"~self\";\n").as_str());
                                    fn_def += format!("println!(\"{{}}: empty\", name_off_new);\n").as_str();
                                }
                            }
                            _ => { unimplemented!() }
                        }
                        fn_def += "\n}\n}\n";
                        println!("---start---\n{}\n---end---", fn_def);

                        return format!("{}({}, {});\n", fn_name, ref_name, name_offset);
                    }
                    match layout.fields() {
                        FieldsShape::Arbitrary { offsets, memory_index } => {
                            let mut has_data = false;
                            for (idx, v) in real_ty.all_fields().enumerate() {
                                has_data = true;
                                let offset = offsets.get(idx).expect("err").bytes_usize();
                                let field_adt = EarlyBinder(self.tcx.type_of(v.did)).subst(self.tcx, &sub_ref);
                                fn_def.push_str(format!("ptr_val = ptr + {};\n", offset).as_str());
                                fn_def.push_str(format!("name_off_new = name_offset.clone() + \"~{}\";\n", v.name).as_str());
                                let stmt = self.recursive_resolve_ret_ty(field_adt, String::from("ptr_val"), String::from("name_off_new"));
                                fn_def.push_str(stmt.as_str());
                            }
                            if !has_data{
                                fn_def.push_str(format!("name_off_new = name_offset.clone() + \"~self\";\n").as_str());
                                fn_def += format!("println!(\"{{}}: empty\", name_off_new);\n").as_str();
                            }

                        }
                        FieldsShape::Array { stride, count } => {
                            // println!("#### {:?} {:?} {:?} {:?} {:?}", ty_layout, adt_def.variants(), sub_ref, stride, count);
                            // unimplemented!()
                            //todo vector goes here, ignore for now
                        }
                        FieldsShape::Union(field_count) => {
                            let mut max_size = 0;
                            let mut candidate = 10000;
                            for (idx, v) in real_ty.all_fields().enumerate() {
                                // we always build the largest here
                                let field_adt = EarlyBinder(self.tcx.type_of(v.did)).subst(self.tcx, &sub_ref);
                                let field_layout = self.tcx.layout_of(ParamEnvAnd { param_env: ParamEnv::empty(), value: field_adt }).unwrap().layout;

                                // self.build_inner(&mut current, 0, field_adt, field_layout.size().bytes_usize());
                                if field_layout.size().bytes_usize() >= max_size {
                                    max_size = field_layout.size().bytes_usize();
                                    candidate = idx;
                                }
                            }
                            //second round to build
                            if candidate != 10000 {
                                for (idx, v) in real_ty.all_fields().enumerate() {
                                    // we always build the largest here
                                    if idx == candidate{
                                        let field_adt = EarlyBinder(self.tcx.type_of(v.did)).subst(self.tcx, &sub_ref);
                                        let stmt = self.recursive_resolve_ret_ty(field_adt, String::from("ptr"), String::from("name_offset"));
                                        fn_def.push_str(stmt.as_str());
                                    }
                                }
                            }

                        }
                        FieldsShape::Primitive => {
                            fn_def.push_str(format!("name_off_new = name_offset.clone() + \"~self\";\n").as_str());
                            fn_def += format!("println!(\"{{}}: empty\", name_off_new);\n").as_str();
                        }
                    }
                } else if let Variants::Multiple { tag, tag_encoding, tag_field, variants } = layout.variants() {
                    let mut tag_offset = 0usize;
                    if let FieldsShape::Arbitrary{offsets, memory_index} = layout.fields(){
                        tag_offset = offsets.get(0).expect("err").bytes_usize();
                    }else {
                        unimplemented!()
                    }

                    let mut signed = false;
                    let mut tag_size = 0;

                    match tag.primitive() {
                        Primitive::Int(integer, _signed) => {
                            tag_size = integer.size().bytes_usize();
                            signed = _signed;
                        }
                        Primitive::Pointer => {
                            tag_size = 8;
                        }
                        _ => {
                            unreachable!()
                        }
                    }

                    if signed {
                        fn_def += &*format!("\nlet mut enum_flag = get_signed_int(ptr + {}, 0, {});\n", tag_offset, tag_size);
                    } else {
                        fn_def += &*format!("\nlet mut enum_flag = get_unsigned_int(ptr + {}, 0, {});\n", tag_offset, tag_size);
                    }
                    if let TagEncoding::Niche { untagged_variant, niche_variants, niche_start } = tag_encoding {
                        let dataful_variant = untagged_variant.as_usize();
                        let niche_variants_ed = niche_variants.end().as_usize();
                        let niche_variants_st = niche_variants.start().as_usize();
                        //todo revisit when needed
                        if !signed {
                            fn_def += &*format!("enum_flag = decode_niche(enum_flag, {}, {}, {}, {});\n", niche_start, niche_variants_st, niche_variants_ed, dataful_variant);
                        }else {
                            fn_def += &*format!("enum_flag = decode_niche_signed(enum_flag, {}, {}, {}, {});\n", niche_start, niche_variants_st, niche_variants_ed, dataful_variant);
                        }
                    }else {
                        if signed {
                            let val = tag.valid_range(&self.tcx).start;
                            let ed = tag.valid_range(&self.tcx).end;
                            if val > ed {
                                if let Primitive::Int(integer, _signed) = tag.primitive() {
                                    let ty_name = match integer.size().bytes_usize() {
                                        1=>"u8",
                                        2=>"u16",
                                        4=>"u32",
                                        8=>"u64",
                                        _=>{unreachable!()}
                                    };
                                    fn_def += &*format!("enum_flag = enum_flag + ({}::MAX - {} + 1) as i64;\n", ty_name, val);
                                }
                            }
                        }

                    }

                    // start field field
                    fn_def += "match enum_flag {\n";
                    let mut max_size_field = 0;
                    for (idx, field_layout) in variants.iter().enumerate() {

                        fn_def += &*format!("{} => {{\n", idx);
                        let variant_def = real_ty.variants().get(idx.into()).unwrap();
                        // println!("%%%%%%%{} {:?} {:?} {:?}", idx, field_layout, variant_def, tag);
                        match &field_layout.fields {
                            FieldsShape::Arbitrary { offsets, memory_index } => {
                                let mut has_data = false;
                                for (iidx, vv) in variant_def.fields.iter().enumerate(){
                                    has_data=true;
                                    let offset = offsets.get(iidx).expect("err").bytes_usize();
                                    let var_ty = EarlyBinder(self.tcx.type_of(vv.did)).subst(self.tcx, &sub_ref);
                                    fn_def.push_str(format!("ptr_val = ptr + {};\n", offset).as_str());
                                    fn_def.push_str(format!("name_off_new = name_offset.clone() + \"~{}~{}\";\n",variant_def.name, vv.name).as_str());
                                    let stmt = self.recursive_resolve_ret_ty(var_ty, String::from("ptr_val"), String::from("name_off_new"));
                                    fn_def.push_str(stmt.as_str());
                                }
                                if !has_data{
                                    fn_def.push_str(format!("name_off_new = name_offset.clone() + \"~{}\";\n", variant_def.name).as_str());
                                    fn_def += format!("println!(\"{{}}: empty\", name_off_new);\n").as_str();
                                }
                            }
                            _ => {
                                unreachable!()
                            }
                        }

                        fn_def += "\n}\n";
                    }
                    fn_def += "_ =>{\nunreachable!()\n}\n";
                    fn_def += "\n}\n";


                }
                fn_def += "\n}\n}\n";
                println!("---start---\n{}\n---end---", fn_def);

                return format!("{}({}, {});\n", fn_name, ref_name, name_offset);

            }
            TyKindS::Array(ety, len) => {
                let fn_name = transfer_func_name(format!("print_{}", ty));
                if let Some(_) = self.build_.get(fn_name.as_str()){
                    return format!("{}({}, {});\n", fn_name, ref_name, name_offset);
                }else {
                    self.build_.insert(fn_name.clone(), 0);
                }
                let mut new_ref = String::from("ptr");
                let mut new_name_offset = String::from("name_offset");
                let mut fn_def = format!("fn {}(ptr: usize, name_offset: std::string::String) {{\n unsafe{{\n", fn_name);
                new_ref.push_str("_ele");
                new_name_offset.push_str("_ele");
                let array_layout = self.tcx.layout_of(ParamEnvAnd { param_env: ParamEnv::empty(), value: ty }).unwrap().layout;
                let stmt = self.recursive_resolve_ret_ty(*ety, new_ref.clone(), new_name_offset.clone());
                if stmt.len() > 0{
                    let mut unit_size = self.tcx.layout_of(ParamEnvAnd { param_env: ParamEnv::empty(), value: *ety }).unwrap().layout.size().bytes_usize();
                    let size = len.eval_usize(self.tcx, ParamEnv::empty());
                    fn_def += &*format!("\nfor i in 0..{}{{\n\t let {} = ptr + i * {};\n let mut {} = name_offset.clone() + &i.to_string();\n {} }}\n",
                                        size, new_ref, unit_size, new_name_offset, stmt);
                    fn_def += "\n}\n}\n";
                    println!("---start---\n{}\n---end---", fn_def);
                    return format!("{}({}, {});\n", fn_name, ref_name, name_offset);
                }else {
                    return String::new();
                }
            }
            TyKindS::Slice(ety) => {
                let fn_name = transfer_func_name(format!("print_{}", ty));
                return self.handle_fat_pointer(*ety, &ref_name, &name_offset, fn_name);
                // println!("----- slice {:?}", ty);
                // let mut unit_size = self.tcx.layout_of(ParamEnvAnd { param_env: ParamEnv::empty(), value: *ety }).unwrap().layout.size().bytes_usize();
                // let fn_name = transfer_func_name(format!("{}", ty));
                // let mut new_ref = String::from("ptr");
                // let mut new_name_offset = String::from("name_offset");
                // let mut fn_def = format!("fn {}(ptr: usize, name_offset: std::string::String) {{\n unsafe{{\n", fn_name);
                // new_ref.push_str("_ele");
                // new_name_offset.push_str("_ele");
                // let array_layout = self.tcx.layout_of(ParamEnvAnd { param_env: ParamEnv::empty(), value: ty }).unwrap().layout;
                // let stmt = self.recursive_resolve_ret_ty(*ety, new_ref.clone(), new_name_offset);
                // if stmt.len() > 0{
                //     fn_def += &*format!("let slice_size = unsafe{{ *((ptr as usize + 8) as * const usize) }};");
                //     let mut unit_size = self.tcx.layout_of(ParamEnvAnd { param_env: ParamEnv::empty(), value: *ety }).unwrap().layout.size().bytes_usize();
                //     fn_def += &*format!("\nfor i in 0..slice_size{{\n\t let {} = {} + i * {};\n let mut {} = {}.clone() + i.to_string();\n {} }}\n",
                //                         new_ref, ref_name, unit_size, new_name_offset, name_offset, stmt);
                //     fn_def += "\n}\n}\n";
                //     println!("---start---\n{}\n---end---", fn_def);
                //     return format!("{}({}, {});\n", fn_name, ref_name, name_offset);
                // }else {
                //     return String::new();
                // }
            }
            TyKindS::RawPtr(ty_mut) => {
                //ignore the context for now
                // first check if it's null
                // then recur call to resolve it
                // check if it's fat pointer
                let fn_name = transfer_func_name(format!("print_{}", ty));
                if let TyKindS::Slice(ety) = ty_mut.ty.kind() {
                    return self.handle_fat_pointer(*ety, &ref_name, &name_offset, fn_name);
                }
                if let Some(_) = self.build_.get(fn_name.as_str()){
                    return format!("{}({}, {});\n", fn_name, ref_name, name_offset);
                }else {
                    self.build_.insert(fn_name.clone(), 0);
                }
                let mut fn_def = format!("fn {}(ptr: usize, name_offset: std::string::String) {{\n unsafe{{\n", fn_name);
                fn_def += "let name_offset_new = name_offset.clone() + \"~ptr\";\n";
                let mut deref = "let mut ptr_val = *(ptr as * const usize);\n";
                if donot_deref(ty_mut.ty) {
                    deref = "let mut ptr_val = ptr as usize;\n";
                }
                fn_def += deref;
                fn_def += "if ptr_val <= 0x1000 {\n println!(\"{}: null\", name_offset_new);\n }\n";
                let stmt = self.recursive_resolve_ret_ty(ty_mut.ty, String::from("ptr_val"), String::from("name_offset_new"));
                fn_def += &*format!("else {{\n {} }}\n}}\n}}\n", stmt);
                println!("---start---\n{}\n---end---", fn_def);
                return format!("{}({}, {});\n", fn_name, ref_name, name_offset);
            }
            TyKindS::Ref(_, rty, _) => {
                let full_name = format!("{:?}", *rty);
                if full_name.eq("str") {
                    return handle_str_ty(ref_name, name_offset);
                } else if let TyKindS::Slice(ety) = rty.kind() {
                    //fat pointer
                    let fn_name = transfer_func_name(format!("print_{}", ty));
                    return self.handle_fat_pointer(*ety, &ref_name, &name_offset, fn_name);
                } else {
                    let fn_name = transfer_func_name(format!("print_{}", ty));
                    if let Some(_) = self.build_.get(fn_name.as_str()){
                        return format!("{}({}, {});\n", fn_name, ref_name, name_offset);
                    }else {
                        self.build_.insert(fn_name.clone(), 0);
                    }
                    let mut fn_def = format!("fn {}(ptr: usize, name_offset: std::string::String) {{\n unsafe{{\n", fn_name);
                    fn_def += "let name_offset_new = name_offset.clone() + \"~ref\";\n let ptr_val = *(ptr as * const usize);\n if ptr_val == 0 {\n println!(\"{}: null\", name_offset);\n }\n";
                    let stmt = self.recursive_resolve_ret_ty(*rty, String::from("ptr_val"), String::from("name_offset"));
                    fn_def += &*format!("else {{\n {} }}\n}}\n}}\n", stmt);
                    println!("---start---\n{}\n---end---", fn_def);
                    return format!("{}({}, {});\n", fn_name, ref_name, name_offset);
                }
            }
            TyKindS::Tuple(sub_ref) => {
                let mut offset = 0usize;
                let fn_name = transfer_func_name(format!("print_{}", ty));
                if let Some(_) = self.build_.get(fn_name.as_str()){
                    return format!("{}({}, {});\n", fn_name, ref_name, name_offset);
                }else {
                    self.build_.insert(fn_name.clone(), 0);
                }
                let tuple_layout = self.tcx.layout_of(ParamEnvAnd { param_env: ParamEnv::empty(), value: ty }).unwrap().layout;

                let mut fn_def = format!("fn {}(ptr: usize, name_offset: std::string::String) {{\n unsafe{{\n let mut ptr_val = 0;\n let mut name_off_new = String::new();\n", fn_name);
                match &tuple_layout.fields() {
                    FieldsShape::Arbitrary { offsets, memory_index } => {
                        for (idx, ety) in sub_ref.iter().enumerate() {
                            let offset = offsets.get(idx).expect("err in tuple").bytes_usize();
                            fn_def.push_str(&*format!("ptr_val = ptr + {};\nname_off_new = name_offset.clone() + &{}.to_string();\n", offset, idx));
                            let stmt = self.recursive_resolve_ret_ty(ety, String::from("ptr_val"), String::from("name_off_new"));
                            if stmt.len() == 0{
                                break;
                            }
                            fn_def.push_str(&* stmt);
                        }
                    }
                    _ => {
                        unreachable!()
                    }
                }


                fn_def += "\n}\n}\n";
                println!("---start---\n{}\n---end---", fn_def);
                return format!("{}({}, {});\n", fn_name, ref_name, name_offset);
            }
            _ => {}
        }
        String::new()
    }

    fn handle_fat_pointer(&mut self, ety: TyS<'tcx>, ref_name: &String, name_offset: &String, fn_name: String) -> String {
        let mut unit_size = self.tcx.layout_of(ParamEnvAnd { param_env: ParamEnv::empty(), value: ety }).unwrap().layout.size().bytes_usize();
        if let Some(_) = self.build_.get(fn_name.as_str()) {
            return format!("{}({}, {});\n", fn_name, ref_name, name_offset);
        } else {
            self.build_.insert(fn_name.clone(), 0);
        }
        let mut new_ref = String::from("ptr_val");
        let mut new_name_offset = String::from("name_offset_new");
        let mut fn_def = format!("fn {}(ptr: usize, name_offset: std::string::String) {{\n unsafe{{\n", fn_name);
        let stmt = self.recursive_resolve_ret_ty(ety, new_ref.clone(), new_name_offset.clone());
        if stmt.len() > 0 {
            fn_def += &*format!("let slice_size = unsafe{{ *((ptr as usize + 8) as * const usize) }};");
            let mut unit_size = self.tcx.layout_of(ParamEnvAnd { param_env: ParamEnv::empty(), value: ety }).unwrap().layout.size().bytes_usize();
            //todo fix bug
            fn_def += &*format!("\nfor i in 0..slice_size{{\n\t let {} = ptr + i * {};\n let mut {} = name_offset.clone() + &i.to_string();\n {} }}\n",
                                new_ref, unit_size, new_name_offset, stmt);
            fn_def += "\n}\n}\n";
            println!("---start---\n{}\n---end---", fn_def);
            return format!("{}({}, {});\n", fn_name, ref_name, name_offset);
        } else {
            return String::new();
        }
    }


    fn resolve_ret_tys(&mut self, ty: TyS<'tcx>, ref_name: String, name_offset: String) -> String {
        // the diff between this and the recur version is the type here are byval returns
        // or some struct implements Copy traits
        // otherwise we need the recur version to retrive the value from heap
        match ty.kind() {
            TyKindS::Bool => {
                return format!("println!(\"{{}}~bool:{{}}\", {}, {});\n"
                               , name_offset, ref_name);
            }
            TyKindS::Char => {
                return format!("println!(\"{{}}~char:{{}}\", {}, {});\n"
                               , name_offset, ref_name);
            }
            TyKindS::Str => {
                return format!("println!(\"{{}}~str:{{}}\", {}, {});\n"
                               , name_offset, ref_name);
            }
            TyKindS::Int(int_ty) => {
                let int_type = match int_ty {
                    IntTy::Isize => "isize",
                    IntTy::I8 => "i8",
                    IntTy::I16 => "i16",
                    IntTy::I32 => "i32",
                    IntTy::I64 => "i64",
                    IntTy::I128 => "i128",
                };
                return format!("println!(\"{{}}~{}:{{}}\", {}, {});\n"
                               , int_type, name_offset, ref_name);
            }
            TyKindS::Uint(uint_ty) => {
                let uint_type = match uint_ty {
                    UintTy::Usize => "usize",
                    UintTy::U8 => "u8",
                    UintTy::U16 => "u16",
                    UintTy::U32 => "u32",
                    UintTy::U64 => "u64",
                    UintTy::U128 => "u128",
                };
                return format!("println!(\"{{}}~{}:{{}}\", {}, {});\n"
                               , uint_type, name_offset, ref_name);
            }
            TyKindS::Float(float_ty) => {
                let float_type = match float_ty {
                    FloatTy::F32 => "f32",
                    FloatTy::F64 => "f64",
                };
                return format!("println!(\"{{}}~{}:{{}}\", {}, {});\n"
                               , float_type, name_offset, ref_name);
            }
            TyKindS::Adt(_, _) => {
                // we will put this effort to recur version, just change it into ref for now
                // because this struct is in the stack now
                let mut new_ref = ref_name.clone();
                new_ref.push_str("_ele");
                let full_name = format!("{:?}", ty);
                let stmt = self.recursive_resolve_ret_ty(ty, new_ref.clone(),  name_offset);
                let final_stmt = format!("\nlet {} = &{};\nlet {} = {} as * const _ as usize;\n {}\n", new_ref, ref_name, new_ref, new_ref, stmt);
                return final_stmt;
            }
            TyKindS::Array(ety, _) => {
                let mut new_ref = ref_name.clone();
                new_ref.push_str("_ele");
                let new_name_offset= name_offset.clone() + "_new";
                let stmt = self.resolve_ret_tys(*ety, new_ref.clone(), new_name_offset.clone());
                let final_stmt = format!("\nfor {} in {}{{\nlet {} = {}.clone();\n {} }}\n", new_ref, ref_name, new_name_offset, name_offset, stmt);
                return final_stmt;
            }
            TyKindS::Slice(ety) => {
                let mut new_ref = ref_name.clone();
                new_ref.push_str("_ele");
                let new_name_offset= name_offset.clone() + "_new";
                let stmt = self.resolve_ret_tys(*ety, new_ref.clone(), new_name_offset.clone());
                let final_stmt = format!("\nfor {} in {}{{\nlet {} = {}.clone();\n {} }}\n", new_ref, ref_name, new_name_offset, name_offset, stmt);
                return final_stmt;
            }
            TyKindS::RawPtr(ty_mut) => {
                let stmt =  self.recursive_resolve_ret_ty(ty_mut.ty, ref_name.clone(),  name_offset.clone());
                let final_stmt = format!("let {} = {} as * const _ as usize;\n {};\n", ref_name.clone(), ref_name.clone(), stmt);
                return final_stmt;
            }
            TyKindS::Ref(_, rty, _) => {
                let full_name = format!("{:?}", *rty);
                if full_name.eq("str") {
                    return format!("println!(\"str:{{}}\", {});\n"
                                   , ref_name);
                }else if let TyKindS::Slice(ety) = rty.kind() {
                    let stmt = self.recursive_resolve_ret_ty(ty, ref_name.clone(), name_offset);
                    let final_stmt = format!("let {} = &{} as * const _ as usize;\n {};\n", ref_name.clone(), ref_name.clone(), stmt);
                    return final_stmt;
                }else {
                    let stmt = self.recursive_resolve_ret_ty(*rty, ref_name.clone(), name_offset);
                    if stmt.len() == 0{
                        return String::new()
                    }
                    let final_stmt = format!("let {} = {} as * const _ as usize;\n {};\n", ref_name.clone(), ref_name.clone(), stmt);
                    return final_stmt;
                }
            }
            TyKindS::FnPtr(_) => {
                // for fn ptr, we only do check if it's null or not
                return format!("println!(\"fnptr:{{:p}}\", {});\n"
                               , ref_name);
            }
            TyKindS::Tuple(sub_ref) => {
                let mut ans = String::new();
                for (idx, ty) in sub_ref.iter().enumerate() {
                    let mut new_ref = ref_name.clone();
                    new_ref.push_str(format!("_{}", idx).as_str());
                    ans.push_str(format!("let {} = {}.{};\n", new_ref, ref_name, idx).as_str());
                    ans.push_str(format!("let {}_t = format!(\"{}_{{}}\", {});\n", name_offset,  name_offset, idx).as_str());
                    let stmt = self.resolve_ret_tys(ty, new_ref.to_string(), format!("{}_t", name_offset));
                    ans.push_str(&*stmt);
                }
                return ans;
            }
            _ => {
                //ignored todo revisit
            }
        }
        String::new()
    }
}

impl<'tcx> Visitor<'tcx> for CorpusDecomposeChecker<'tcx> {

    fn visit_ty(&mut self, ty: Ty<'tcx>, _: TyContext) {
        if let TyKindS::FnDef(def_id, subref) = ty.kind() {
            let fnsig = EarlyBinder(self.tcx.fn_sig(def_id)).subst(self.tcx, subref);
            self.visit_funcs.push((fnsig, *def_id));
        }
        self.super_ty(ty);
    }
}