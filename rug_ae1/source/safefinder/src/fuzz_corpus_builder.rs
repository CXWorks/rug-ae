use std::cmp::min;
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
use rustc_middle::middle::exported_symbols::*;
use rustc_middle::ty::Ty as TyS;
use rustc_middle::ty::subst::EarlyBinder;
use rustc_middle::ty::TyKind as TyKindS;
use rustc_middle::ty::Uint;
use rustc_middle::ty::{IntTy, UintTy, FloatTy, VariantDef, VariantDiscr, Visibility, PredicateKind};
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
use crate::util::transfer_func_name;
use crate::util::avoid_late_bound;

pub struct FuzzCorpusChecker<'tcx> {
    tcx: TyCtxt<'tcx>,
    ctxt: Vec<String>,
    visiting: bool,
    map_: BTreeMap<String, String>,
    impl_: BTreeMap<String, String>,
    build_: BTreeMap<String, (String, i64, i64)>,
    visit_funcs: Vec<(PolyFnSig<'tcx>, DefId)>,
    visit_func_args: Vec<Ty<'tcx>>
}

impl<'tcx> FuzzCorpusChecker<'tcx> {
    pub fn new(tx: TyCtxt<'tcx>) -> Self {
        FuzzCorpusChecker {
            tcx: tx,
            ctxt: Vec::new(),
            visiting: false,
            map_: BTreeMap::new(),
            impl_: BTreeMap::new(),
            build_: BTreeMap::new(),
            visit_funcs: Vec::new(),
            visit_func_args: Vec::new()
        }
    }
}


impl<'tcx> FuzzCorpusChecker<'tcx> {

    fn check_has_unsupported_type(&self, vty: rustc_middle::ty::Ty<'tcx>) -> bool {
        match vty.kind() {
            TyKindS::Ref(_, rty, _) => {
                return self.check_has_unsupported_type(*rty);
            }
            TyKindS::RawPtr(ty_mut) => {
                return self.check_has_unsupported_type(ty_mut.ty);
            }
            TyKindS::Foreign(def_id) => {
                return true;
            }
            TyKindS::Array(ety, _) => {
                return self.check_has_unsupported_type(*ety);
            }
            TyKindS::Slice(ety) => {
                return self.check_has_unsupported_type(*ety);
            }
            TyKindS::FnDef(def_id, _) => {
                return true;
            }
            TyKindS::FnPtr(fsig) => {
                return true;
            }
            TyKindS::Tuple(sub_ref) => {
                return true;
            }
            _ => {
                return false;
            }
        }
    }


    pub fn analyze(&mut self) {
        for def_id in self.tcx.hir_crate_items(()).definitions() {
            let def_kind = self.tcx.def_kind(def_id);
            // Find the DefId for the entry point, note that the entry point must be a function
            if def_kind == DefKind::Fn || def_kind == DefKind::AssocFn {
                let item_name = self.tcx.item_name(def_id.to_def_id());
                let mut ty_set = BTreeSet::<TyS>::new();
                let fnsig = self.tcx.fn_sig(def_id.to_def_id());
                let fn_output = fnsig.output().skip_binder();
                let def_path = self.tcx.def_path_str(def_id.to_def_id());
                if def_path.contains("fuzzdriver_") && def_path.contains("fuzz_unit_test") && def_path.contains("test_ss") {
                    let b = self.tcx.optimized_mir(def_id);
                    self.visit_funcs.clear();
                    self.visit_func_args.clear();
                    self.visit_body(b);
                    //we only care the last function
                    if let Some((fnsig, def_id)) = self.visit_funcs.last(){
                        // assert!(self.visit_func_args.len() <= fnsig.inputs().skip_binder().len());
                        let s = self.tcx.item_name(*def_id);
                        for idx in 0..self.visit_func_args.len(){
                            let arg_ty = *self.visit_func_args.get(idx).expect("err");
                            let innter_ty= self.build_ty(arg_ty, 0);
                            if innter_ty.0.len() > 0{
                                println!("+{:?}+{}+{}+{}", arg_ty, innter_ty.0, innter_ty.1, innter_ty.2);
                            }else {
                                println!("+{:?}+null", arg_ty);
                            }
                            if self.check_adt_no_copy(arg_ty){
                                println!("%{:?}", arg_ty);
                            }

                            if let Some((ety, len)) = self.check_is_slice(arg_ty){
                                println!("${:?}+{:?}+{}", arg_ty, ety, len);
                            }

                        }
                    }
                }
            }
        }
    }


    fn check_adt_no_copy(&mut self, vty: rustc_middle::ty::Ty<'tcx>) -> bool{
        if let TyKindS::Adt(adt_def, sub_ref) = vty.kind(){
            return !self.tcx.is_copy_raw(ParamEnvAnd { param_env: ParamEnv::empty(), value: vty });
        }
        return false;
    }

    fn check_is_slice(&mut self, vty: rustc_middle::ty::Ty<'tcx>) -> Option<(rustc_middle::ty::Ty<'tcx>, usize)>{
        match vty.kind() {
            TyKindS::Ref(_, rty, mutbility) => {
                return self.check_is_slice(*rty);
            }
            TyKindS::RawPtr(ty_mut) => {
                return self.check_is_slice(ty_mut.ty);
            }
            TyKindS::Slice(ety) => {
                return Some((*ety, 16))
            }
            TyKindS::Array(ety, len) => {
                let size = len.eval_usize(self.tcx, ParamEnv::empty());
                return Some((*ety, size as usize))
            }

            _ =>{}
        }
        return None;
    }

    fn build_ty(&mut self, vty: rustc_middle::ty::Ty<'tcx>, ref_count: usize) -> (String, i64, i64) {
        let ty_name = format!("{}", vty);
        let fn_name = format!("build_{}", transfer_func_name(ty_name.clone()));
        if let Some(cache) = self.build_.get(&*fn_name) {
            return (cache.0.clone(), cache.1, cache.2);
        }
        match vty.kind() {
            TyKindS::Bool => {
                let ans = format!("build_bool(data, data_off)");
                return (ans, 0, 1);
            }
            TyKindS::Char => {
                let ans = format!("build_char(data, data_off)");
                return (ans, 0, 1);
            }
            TyKindS::Str => {
                let ans = format!("build_str(data, data_off)");
                return (ans, 0, 0);
            }
            TyKindS::Int(int_ty) => {
                let int_size = match int_ty {
                    IntTy::Isize => 64,
                    IntTy::I8 => 8,
                    IntTy::I16 => 16,
                    IntTy::I32 => 32,
                    IntTy::I64 => 64,
                    IntTy::I128 => 128,
                };
                let ans = format!("build_int(data, data_off, {})", int_size);
                return (ans, 0, int_size / 8);
            }
            TyKindS::Uint(uint_ty) => {
                let int_size = match uint_ty {
                    UintTy::Usize => 64,
                    UintTy::U8 => 8,
                    UintTy::U16 => 16,
                    UintTy::U32 => 32,
                    UintTy::U64 => 64,
                    UintTy::U128 => 128,
                };
                let ans = format!("build_uint(data, data_off, {})", int_size);
                return (ans, 0, int_size / 8);
            }
            TyKindS::Float(float_ty) => {
                let float_size = match float_ty {
                    FloatTy::F32 => 32,
                    FloatTy::F64 => 64,
                };
                let ans = format!("build_float(data, data_off, {})", float_size);
                return (ans, 0, float_size / 8);
            }
            TyKindS::Ref(_, rty, mutbility) => {
                let inner_ty = self.build_ty(*rty, ref_count + 1);
                if inner_ty.0.len() > 0 {
                    return inner_ty;
                } else {
                    return (String::new(), 0, 0);
                }
            }
            TyKindS::RawPtr(ty_mut) => {
                let inner_ty = self.build_ty(ty_mut.ty, ref_count);
                if inner_ty.0.len() > 0 {
                    return inner_ty;
                } else {
                    return (String::new(), 0, 0);
                }
            }
            TyKindS::Array(ety, len) => {
                let array_layout = self.tcx.layout_of(ParamEnvAnd { param_env: ParamEnv::empty(), value: vty }).unwrap().layout;
                let inner_ty = self.build_ty(*ety, ref_count);
                if inner_ty.0.len() > 0{
                    let mut unit_size = inner_ty.2;
                    if inner_ty.1 > 0{ //fat pointer
                        unit_size += 8;
                    }
                    let size = len.eval_usize(self.tcx, ParamEnv::empty());
                    assert!( array_layout.size().bytes_usize() >= (inner_ty.2 as usize * (size as usize)));
                    let mut fn_def = format!("fn {}(data: &[u8], data_off: usize) -> (*mut u8, usize){{\nunsafe{{\nif let Some(res) = get_recur(\"{}\"){{\nreturn (res, 0);\n}}\nlet mut buffer_size = 0;\nlet mut ans = bump_alloc({});\nsave_recur(\"{}\", ans);\n", fn_name, fn_name, array_layout.size().bytes_usize(), fn_name);
                    fn_def+=&*format!("let mut iter = ans as *mut u8;\nlet mut src = {};\nfor i in 0..{} {{\n", inner_ty.0, size);
                    fn_def += &*format!("std::ptr::copy(src.0, iter.offset(i * {}), {});\n", unit_size, inner_ty.2);
                    if inner_ty.1 > 0{
                        fn_def += &* format!("std::ptr::write(iter.offset(i * {} + 8) as * mut usize, {});\n", unit_size, inner_ty.1);
                    }
                    fn_def += "}\n";
                    fn_def += &*format!("\nclear_recur(\"{}\");\n    (ans, buffer_size)\n}}\n}}", fn_name);
                    println!("---start---\n{}\n---end---", fn_def);
                    self.build_.insert(fn_name.clone(), (format!("{}(data, data_off)", fn_name),  size as i64, array_layout.size().bytes_usize() as i64));
                    return (format!("{}(data, data_off)", fn_name),  size as i64, array_layout.size().bytes_usize() as i64);
                }else {
                    return (String::new(), 0, 0);
                }
            }
            TyKindS::Slice(ety) => {

                let inner_ty = self.build_ty(*ety, ref_count);
                let slice_size = inner_ty.2 * 16;
                if inner_ty.0.len() > 0{
                    let mut unit_size = inner_ty.2;
                    if inner_ty.1 > 0{
                        unit_size += 8;
                    }
                    let mut fn_def = format!("fn {}(data: &[u8], data_off: usize) -> (*mut u8, usize){{\nunsafe{{\nif let Some(res) = get_recur(\"{}\"){{\nreturn (res, 0);\n}}\nlet mut buffer_size = 0;\nlet mut ans = bump_alloc({});save_recur(\"{}\", ans);\n", fn_name, fn_name, slice_size, fn_name);
                    fn_def+=&*format!("let mut iter = ans as *mut u8;\nlet mut src = {};\nfor i in 0..{} {{\n", inner_ty.0, 16);
                    fn_def += &* format!("std::ptr::copy(src.0, iter.offset(i * {}), {});\n", unit_size, inner_ty.2);
                    if inner_ty.1 > 0 {
                        fn_def += &*format!("std::ptr::write(iter.offset(i * {} + 8) as * mut usize, {});\n", unit_size, inner_ty.1);
                    }
                    fn_def += "}\n";
                    fn_def += &*format!("\nclear_recur(\"{}\");\n    (ans, buffer_size)\n}}\n}}", fn_name);
                    println!("---start---\n{}\n---end---", fn_def);
                    self.build_.insert(fn_name.clone(), (format!("{}(data, data_off)", fn_name), 16, slice_size));
                    return (format!("{}(data, data_off)", fn_name), 16, slice_size);
                }else {
                    return (String::new(), 0, 0);
                }
            }

            TyKindS::Adt(adt_def, sub_ref) => {
                //our main course!!!
                let mut real_sub_ref = vec!();
                avoid_late_bound(&mut self.tcx, &mut real_sub_ref, sub_ref);
                let real_ty = EarlyBinder(*adt_def).subst(self.tcx, &real_sub_ref);
                let field_adt = EarlyBinder(self.tcx.type_of(adt_def.did())).subst(self.tcx, &real_sub_ref);

                let param_env_and_ty = self.tcx.param_env(adt_def.did()).and(field_adt);
                let ty_layout = self.tcx.layout_of(param_env_and_ty).unwrap();
                let layout = ty_layout.layout;
                self.build_.insert(fn_name.clone(), (format!("{}(data, data_off)", fn_name), 0, layout.size().bytes_usize() as i64));
                let mut fn_def = format!("fn {}(data: &[u8], data_off_t: usize) -> (*mut u8, usize){{\nunsafe{{\nif let Some(res) = get_recur(\"{}\"){{\nreturn (res, 0);\n}}\nlet mut buffer_size = 0;\nlet mut data_off=data_off_t;\nlet mut ans = bump_alloc({});\nsave_recur(\"{}\", ans);\n", fn_name, fn_name, layout.size().bytes_usize(), fn_name);
                if let Variants::Single { index } = layout.variants() {
                    match layout.fields() {
                        FieldsShape::Arbitrary { offsets, memory_index } => {
                            for (idx, v) in real_ty.all_fields().enumerate() {
                                let mem_idx = memory_index.get(idx).expect("err");
                                let offset = offsets.get(idx).expect("err").bytes_usize();
                                let field_adt = EarlyBinder(self.tcx.type_of(v.did)).subst(self.tcx, &real_sub_ref);

                                let field_layout = self.tcx.layout_of(ParamEnvAnd { param_env: ParamEnv::empty(), value: field_adt }).unwrap().layout;
                                self.build_inner(&mut fn_def, offset, field_adt, field_layout.size().bytes_usize());
                            }
                        }
                        FieldsShape::Array { stride, count } => {
                            // unimplemented!()
                        }
                        FieldsShape::Union(field_count) => {
                            let mut max_size = 0;
                            let mut candidate = String::new();
                            for (idx, v) in real_ty.all_fields().enumerate() {
                                // we always build the largest here
                                let field_adt = EarlyBinder(self.tcx.type_of(v.did)).subst(self.tcx, &real_sub_ref);
                                let field_layout = self.tcx.layout_of(ParamEnvAnd { param_env: ParamEnv::empty(), value: field_adt }).unwrap().layout;
                                let mut current = String::new();

                                self.build_inner(&mut current, 0, field_adt, field_layout.size().bytes_usize());
                                if field_layout.size().bytes_usize() >= max_size {
                                    max_size = field_layout.size().bytes_usize();
                                    candidate = current;
                                }
                            }


                            fn_def += candidate.as_str();

                        }
                        FieldsShape::Primitive => {
                            unimplemented!()
                        }
                    }
                } else if let Variants::Multiple { tag, tag_encoding, tag_field, variants } = layout.variants() {
                    let mut signed = false;
                    let mut tag_size = 0;
                    let mut tag_offset = 0usize;
                    if let FieldsShape::Arbitrary{offsets, memory_index} = layout.fields(){
                        tag_offset = offsets.get(0).expect("err").bytes_usize();
                    }else {
                        unimplemented!()
                    }
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
                        fn_def += &*format!("\nlet enum_flag = read_signed_int(data, data_off, {}) % {};\n", tag_size, variants.len());
                    } else {
                        fn_def += &*format!("\nlet enum_flag = read_unsigned_int(data, data_off, {}) %{};\n", tag_size, variants.len());
                    }
                    fn_def += &*format!("let mut mem_flag = enum_flag;\n");
                    if let TagEncoding::Niche { untagged_variant, niche_variants, niche_start } = tag_encoding {
                        let dataful_variant = untagged_variant.as_usize();
                        let niche_variants_ed = niche_variants.end().as_usize();
                        let niche_variants_st = niche_variants.start().as_usize();
                        if !signed {
                            fn_def += &*format!("mem_flag = encode_niche(mem_flag, {}, {}, {}, {});\n", niche_start, niche_variants_st, niche_variants_ed, dataful_variant);
                        }else {
                            fn_def += &*format!("mem_flag = encode_niche_signed(mem_flag, {}, {}, {}, {});\n", niche_start, niche_variants_st, niche_variants_ed, dataful_variant);
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
                                    fn_def += &*format!("mem_flag = mem_flag - ({}::MAX - {} + 1) as i64;\n", ty_name, val);
                                }
                            }
                        }
                    }

                    //write the enum flag into memory
                    if !signed {
                        match tag_size {
                            1 => { fn_def += &*format!("\nstd::ptr::write((ans as usize + {}) as * mut u8, mem_flag as u8);", tag_offset) }
                            2 => { fn_def += &*format!("\nstd::ptr::write((ans as usize + {}) as * mut u16, mem_flag as u16);", tag_offset)}
                            4 => { fn_def += &*format!("\nstd::ptr::write((ans as usize + {}) as * mut u32, mem_flag as u32);", tag_offset) }
                            8 => { fn_def += &*format!("\nstd::ptr::write((ans as usize + {}) as * mut u64, mem_flag as u64);", tag_offset) }
                            _ => { unreachable!() }
                        }
                    }else {
                        match tag_size {
                            1 => { fn_def += &*format!("\nstd::ptr::write((ans as usize + {}) as * mut i8, mem_flag as i8);", tag_offset) }
                            2 => { fn_def += &*format!("\nstd::ptr::write((ans as usize + {}) as * mut i16, mem_flag as i16);", tag_offset) }
                            4 => { fn_def += &*format!("\nstd::ptr::write((ans as usize + {}) as * mut i32, mem_flag as i32);", tag_offset) }
                            8 => { fn_def += &*format!("\nstd::ptr::write((ans as usize + {}) as * mut i64, mem_flag as i64);", tag_offset) }
                            _ => { unreachable!() }
                        }
                    }
                    // start field field
                    fn_def += "\nmatch enum_flag {\n";
                    let mut max_size_field = 0;
                    for (idx, field_layout) in variants.iter().enumerate() {
                        fn_def += &*format!("{} => {{\n", idx);
                        let variant_def = real_ty.variants().get(idx.into()).unwrap();
                        let var_ty = EarlyBinder(self.tcx.type_of(variant_def.def_id)).subst(self.tcx, &real_sub_ref);
                        if let FieldsShape::Arbitrary { offsets, memory_index } = &field_layout.fields {
                            for (idx, f) in variant_def.fields.iter().enumerate() {
                                let f_ty = EarlyBinder(self.tcx.type_of(f.did)).subst(self.tcx, &real_sub_ref);
                                let field_layout = self.tcx.layout_of(ParamEnvAnd { param_env: ParamEnv::empty(), value: f_ty }).unwrap().layout;
                                let t: &Size = offsets.get(idx).unwrap();
                                let mut offset : usize = t.bytes_usize();
                                assert!(offset <= layout.size().bytes_usize());
                                if offset == layout.size().bytes_usize(){
                                    offset = 0;
                                }
                                self.build_inner(&mut fn_def, offset, f_ty, field_layout.size().bytes_usize());

                            }
                        } else {
                            unimplemented!()
                        }
                        fn_def += "\n}\n";
                    }
                    fn_def += "_ =>{\nunreachable!()\n}\n";
                    fn_def += "\n}\n";


                }
                fn_def += &*format!("\nclear_recur(\"{}\");\n    (ans, buffer_size)\n}}\n}}", fn_name);
                println!("---start---\n{}\n---end---", fn_def);

                return (format!("{}(data, data_off)", fn_name), 0, layout.size().bytes_usize() as i64);
            }
            _ => {
                //we only support primitive types + fat pointers
                return (String::new(), 0, 0);
            }
        }
    }

    fn build_inner(&mut self, fn_def: &mut String, offset: usize, field_adt: Ty<'tcx>, field_fize: usize){

        match field_adt.kind() {
            TyKindS::Ref(_, rty, mutbility) => {
                let checku8 = format!("{}", rty);
                if checku8.eq("[u8]") {
                    *fn_def += &*format!("\nstd::ptr::write(ans.offset({}) as * mut usize, data.as_ptr().offset((data_off % data.len()) as isize) as * const _ as usize);", offset);
                    *fn_def += &*format!("std::ptr::write((ans.offset({}) as * mut usize).offset(1), data.len() - (data_off % data.len()));", offset);
                    return;
                }
                let inner_ty = self.build_ty(*rty, 0);
                if inner_ty.0.len() > 0 {
                    *fn_def += &*format!("\nlet (ptr, size) = {};\nbuffer_size+=  if size < 0 {{ 0 }} else {{ size }};\ndata_off+=  if size < 0 {{ 0 }} else {{ size }};\
                                    \nstd::ptr::write(ans.offset({}) as * mut usize, ptr as usize);", inner_ty.0, offset);
                    if inner_ty.1 > 0{
                        *fn_def += &*format!("std::ptr::write((ans.offset({}) as * mut usize).offset(1), {} as usize);", offset, inner_ty.1);
                    }
                }
            }
            TyKindS::RawPtr(ty_mut) => {
                let inner_ty = self.build_ty(ty_mut.ty, 0);
                if inner_ty.0.len() > 0 {
                    *fn_def += &*format!("\nlet (ptr, size) = {};\nbuffer_size+=  if size < 0 {{ 0 }} else {{ size }};\ndata_off+=  if size < 0 {{ 0 }} else {{ size }};\
                                    \nstd::ptr::write(ans.offset({}) as * mut usize, ptr as usize);", inner_ty.0, offset);
                    if inner_ty.1 > 0{
                        *fn_def += &*format!("std::ptr::write((ans.offset({}) as * mut usize).offset(1), {} as usize);", offset, inner_ty.1);
                    }

                }
            }
            TyKindS::Array(ety, len) => {
                let inner_ty = self.build_ty(*ety, 0);
                if inner_ty.0.len() > 0 {
                    let size = len.eval_usize(self.tcx, ParamEnv::empty());
                    *fn_def += &*format!("\nlet (ptr, size) = {};\nbuffer_size+=  if size < 0 {{ 0 }} else {{ size }};\ndata_off+=  if size < 0 {{ 0 }} else {{ size }};\n", inner_ty.0);
                    *fn_def+=&*format!("let mut iter = ans.offset({}) as *mut u8;\nlet mut src = {};\nfor i in 0..{} {{\n", offset, inner_ty.0, size);
                    let mut unit_size = inner_ty.2;
                    if inner_ty.1 > 0 {
                        unit_size += 8;
                    }
                    *fn_def += &*format!("std::ptr::copy(src.0, iter.offset(i * {}) as * mut u8, {});", unit_size, inner_ty.2);
                    if inner_ty.1 > 0{
                        *fn_def += &*format!("std::ptr::write((iter.offset(i * {}) as * mut usize).offset(1), {} as usize);", offset, inner_ty.1);
                    }
                    *fn_def += "\n}\n";

                }
            }
            TyKindS::Slice(ety) => {
                let inner_ty = self.build_ty(*ety, 0);
                if inner_ty.0.len() > 0 {
                    let size = 16;
                    *fn_def += &*format!("\nlet (ptr, size) = {};\nbuffer_size+=  if size < 0 {{ 0 }} else {{ size }};\ndata_off+=  if size < 0 {{ 0 }} else {{ size }};\n", inner_ty.0);
                    *fn_def+=&*format!("let mut iter = ans.offset({}) as *mut u8;\nlet mut src = {};\nfor i in 0..{} {{\n", offset, inner_ty.0, size);
                    let mut unit_size = inner_ty.2;
                    if inner_ty.1 > 0 {
                        unit_size += 8;
                    }
                    *fn_def += &*format!("std::ptr::copy(src.0, iter.offset(i * {}) as * mut u8, {});", unit_size, inner_ty.2);
                    if inner_ty.1 > 0{
                        *fn_def += &*format!("std::ptr::write((iter.offset(i * {}) as * mut usize).offset(1), {} as usize);", offset, inner_ty.1);
                    }
                    *fn_def += "\n}\n";

                }
            }
            _ => unsafe {
                let inner_ty = self.build_ty(field_adt, 0);
                if inner_ty.0.len() > 0 {
                    *fn_def += &*format!("\nlet (ptr, size) = {};\nbuffer_size+=  if size < 0 {{ 0 }} else {{ size }};\ndata_off+=  if size < 0 {{ 0 }} else {{ size }};\nstd::ptr::copy(ptr as * mut u8, ans.offset({}), {});",
                                         inner_ty.0, offset, field_fize);

                }
            }
        }

    }
}

impl<'tcx> Visitor<'tcx> for FuzzCorpusChecker<'tcx> {

    fn visit_ty(&mut self, ty: Ty<'tcx>, _: TyContext) {

        if let TyKindS::FnDef(def_id, subref) = ty.kind() {
            let s = self.tcx.item_name(*def_id);
            //
            // let mut default = String::from("from_slice");
            // if let Ok(tar) = std::env::var("TARGET_FUNC") {
            //     default = tar;
            // }
            if s.as_str().eq("assume_init") {
                for generic in *subref{
                    let t = generic.expect_ty();
                    if let TyKindS::Tuple(tys) = t.kind(){
                        if let Some(tt) = tys.first(){
                            self.visit_func_args.push(*tt);
                        }
                    }

                }
            }

            //     println!("+++++++++++{}", s);
            let fnsig = EarlyBinder(self.tcx.fn_sig(def_id)).subst(self.tcx, subref);
            //     //we do return value first
            //
            //
            // }
            self.visit_funcs.push((fnsig, *def_id));


            //
        }

        self.super_ty(ty);
    }
}