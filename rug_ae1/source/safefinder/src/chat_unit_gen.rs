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
use std::env::var;
use rustc_middle::ty::ParamEnvAnd;
use rustc_middle::ty::ParamEnv;
use rustc_middle::ty::PolyFnSig;
use crate::util::{is_template, transfer_func_name};
use crate::util::avoid_late_bound;
use rustc_hir::*;
use rustc_hir::QPath::Resolved;
use rustc_middle::ty::TyKind as TyKindS;
use rustc_hir::{Item, ItemKind, UseKind};
use rustc_hir::def::Namespace;
use rustc_hir::intravisit::{Visitor};
use std::io::Write;
use rustc_span::symbol::sym;
use crate::util::check_is_constructor;
use rustc_span::def_id::LOCAL_CRATE;
use rustc_middle::ty::print::FmtPrinter;
use rustc_middle::ty::print::PrettyPrinter;
struct FnVisitor{
}

impl<'hir> Visitor<'hir> for FnVisitor {

    fn visit_item(&mut self, i: &'hir Item<'hir>) {
        match i.kind {
            ItemKind::Fn(..) => {
            },
            _ => (),
        }
        rustc_hir::intravisit::walk_item(self, i);
    }
}
pub struct ChatUnitGen<'tcx> {
    tcx: TyCtxt<'tcx>,
    srcs: BTreeMap<String, (String, String)>,
    trait_to_struct: BTreeMap<String, BTreeSet<String>>,
    struct_to_trait: BTreeMap<String, BTreeSet<String>>,
    dependencies: BTreeMap<String, BTreeSet<String>>,
    targets:BTreeMap<String, (String, String, String)>,
    self_to_fn: BTreeMap<String, BTreeSet<String>>,
    name_to_def_id: BTreeMap<String, DefId>,
    struct_to_def_id: BTreeMap<String, DefId>,
    type_to_def_path: BTreeMap<String, String>,
    struct_constructor: BTreeMap<String, BTreeSet<String>>,
    single_path_import: BTreeMap<String, String>,
    glob_path_import: BTreeMap<String, String>
}

impl<'tcx> ChatUnitGen<'tcx> {
    pub fn new(tx: TyCtxt<'tcx>) -> Self {
        ChatUnitGen {
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
            single_path_import: BTreeMap::new(),
            glob_path_import: BTreeMap::new()
        }
    }
}


impl<'tcx> ChatUnitGen<'tcx> {


    pub fn analyze(&mut self) {
        let hir = self.tcx.hir();
        let krate = hir.krate();

        let mut visitor = UseVisitor { tcx: self.tcx, single: &mut self.single_path_import, glob: &mut self.glob_path_import };
        for item_id in hir.items() {
            let item = hir.item(item_id);
            visitor.visit_item(item);
        }

        for def_id in self.tcx.hir_crate_items(()).definitions() {
            let def_kind = self.tcx.def_kind(def_id);
            let def_path = self.tcx.def_path_str(def_id.to_def_id());

            self.name_to_def_id.insert(def_path.clone(), def_id.to_def_id());
            if def_kind == DefKind::Struct{
                self.struct_to_def_id.insert(def_path.clone(), def_id.to_def_id());
            }
            // Find the DefId for the entry point, note that the entry point must be a function
            if def_kind == DefKind::Fn || def_kind == DefKind::AssocFn {

                let modid = self.tcx.parent_module_from_def_id(def_id);
                let item_name = self.tcx.item_name(def_id.to_def_id());
                let fnsig = self.tcx.fn_sig(def_id.to_def_id());
                let cur_hirid = self.tcx.hir().local_def_id_to_hir_id(def_id);
                let fn_decl = self.tcx.hir().fn_decl_by_hir_id(cur_hirid);

                if let Some(out_ty) = check_is_constructor(&mut self.tcx, fnsig){
                    if let Some(hs) = self.struct_constructor.get_mut(&out_ty){
                        hs.insert(item_name.to_string());
                    }else {
                        let mut hs = BTreeSet::new();
                        hs.insert(item_name.to_string());
                        self.struct_constructor.insert(out_ty, hs);
                    }
                }
                if let Ok(header) = self.tcx
                    .sess
                    .source_map().span_to_snippet(self.tcx.def_span(def_id.to_def_id())){
                    if let Some(fn_data) = self.tcx.hir().maybe_body_owned_by(def_id) {
                        let body = self.tcx.hir().body(fn_data);
                        let mut rustdoc = String::new();
                        // Look at each attribute.
                        let attrs = self.tcx.get_attrs_unchecked(def_id.to_def_id());
                        for attr in attrs{
                            if let Ok(header) = self.tcx
                                .sess
                                .source_map().span_to_snippet(attr.span){
                                rustdoc += &header;
                                rustdoc += "\n";
                            }
                        }
                        if let Ok(bd) = self.tcx
                            .sess
                            .source_map().span_to_snippet(body.value.span){
                            if !header.eq(&bd) {
                                let filename = self.tcx
                                    .sess
                                    .source_map().span_to_filename(self.tcx.def_span(def_id.to_def_id()));
                                let fn_src = format!("{}{}", header, bd);
                                self.srcs.insert(def_path.clone(), (rustdoc + &*fn_src, format!("{:?}", filename)));


                                // Get the HirId of the parent item
                                let parent_owner_id = self.tcx.hir().get_parent_item(cur_hirid);


                                // Get the parent HIR node as an item
                                let parent_node = self.tcx.hir().find(parent_owner_id.into());

                                let parent_item = match parent_node {
                                    Some(rustc_hir::Node::Item(item)) => Some(item), // Found the parent HIR item
                                    _ => None, // Parent node is not an item or doesn't exist
                                };
                                let mut trait_def = String::new();
                                if let Some(pitem) = parent_item{
                                    if let ItemKind::Impl(iimpl) = pitem.kind {
                                        if let Some(trait_ref) = &iimpl.of_trait {
                                            if let Res::Def(kind, id) = trait_ref.path.res {
                                                trait_def = String::from(self.tcx.def_path_str(id));
                                            }
                                        }
                                    }
                                }




                                self.targets.insert(def_path, (item_name.to_string(), format!("{:?}", filename), trait_def));
                            }
                        }
                    }
                }

            }else if def_kind == DefKind::Struct || def_kind == DefKind::Union || def_kind == DefKind::Enum || def_kind == DefKind::Trait {
                let item = self.tcx.hir().expect_item(def_id);
                let mut rustdoc = String::new();
                // Look at each attribute.
                let attrs = self.tcx.get_attrs_unchecked(def_id.to_def_id());
                for attr in attrs{
                    if let Ok(header) = self.tcx
                        .sess
                        .source_map().span_to_snippet(attr.span){
                        rustdoc += &header;
                        rustdoc += "\n";
                    }
                }
                if let Ok(header) = self.tcx
                    .sess
                    .source_map().span_to_snippet(item.span){
                    let filename = self.tcx
                        .sess
                        .source_map().span_to_filename(self.tcx.def_span(def_id.to_def_id()));
                    self.srcs.insert(def_path.clone(), (rustdoc + &header, format!("{:?}", filename)));
                    // println!("{} {}", def_path, header)
                }
                if def_kind != DefKind::Trait{
                    self.type_to_def_path.insert(format!("{}", self.tcx.type_of(def_id.to_def_id())), def_path);
                }

            }else if def_kind == DefKind::Impl  {
                let item = self.tcx.hir().expect_item(def_id);
                if let ItemKind::Impl(iimpl) = item.kind{
                    if let Some(trait_ref) = &iimpl.of_trait{
                        if let Res::Def(kind, id) = trait_ref.path.res{
                            let trait_def = self.tcx.def_path_str(id);
                            if let TyKind::Path(path_resolve) = &iimpl.self_ty.kind {
                                if let Resolved(_, res) = &path_resolve {
                                    if let Res::Def(kind, id) = res.res{
                                        let struct_def = self.tcx.def_path_str(id);
                                        if let Some(sets) = self.trait_to_struct.get_mut(&trait_def){
                                            sets.insert(struct_def.clone());
                                        }else {
                                            let mut sets = BTreeSet::new();
                                            sets.insert(struct_def.clone());
                                            self.trait_to_struct.insert(trait_def.clone(), sets);
                                        }
                                        if let Some(sets) = self.struct_to_trait.get_mut(&struct_def){
                                            sets.insert(trait_def.clone());
                                        }else {
                                            let mut sets = BTreeSet::new();
                                            sets.insert(trait_def.clone());
                                            self.struct_to_trait.insert(struct_def, sets);
                                        }
                                    }
                                }
                            }
                        }
                    }

                }

                //handle the impl for struct
                if let ItemKind::Impl(iimpl) = item.kind{
                    if let TyKind::Path(path_resolve) = &iimpl.self_ty.kind {
                        if let Resolved(_, res) = &path_resolve {
                            if let Res::Def(kind, id) = res.res {
                                let struct_def = self.tcx.def_path_str(id);
                                let item = self.tcx.hir().expect_item(def_id);
                                if let Ok(header) = self.tcx
                                    .sess
                                    .source_map().span_to_snippet(item.span){
                                    if let Some(sets) = self.self_to_fn.get_mut(&struct_def){
                                        sets.insert(header.clone());
                                    }else {
                                        let mut sets = BTreeSet::new();
                                        sets.insert(header.clone());
                                        self.self_to_fn.insert(struct_def, sets);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        for def_id in self.tcx.hir_crate_items(()).definitions() {
            let def_kind = self.tcx.def_kind(def_id);
            let def_path = self.tcx.def_path_str(def_id.to_def_id());
            // Find the DefId for the entry point, note that the entry point must be a function
            if def_kind == DefKind::Fn || def_kind == DefKind::AssocFn || def_kind == DefKind::Struct || def_kind == DefKind::Union || def_kind == DefKind::Enum {
                // if !def_path.eq("enc::impls::<impl enc::Encode for i8>::encode"){
                //     continue;
                // }
                // println!("ffff {}", def_path);
                let item_name = self.tcx.item_name(def_id.to_def_id());
                let mut ty_set = BTreeSet::new();
                let node = self.tcx.hir().find_by_def_id(def_id);
                self.handle_ty_dependencies(self.tcx.type_of(def_id.to_def_id()), "1", &mut ty_set);
                let fty = self.tcx.type_of(def_id.to_def_id());
                self.dependencies.insert(def_path, ty_set);
            }
        }
        // srcs: BTreeMap<String, String>,
        // trait_to_struct: BTreeMap<String, BTreeSet<String>>,
        // struct_to_trait: BTreeMap<String, BTreeSet<String>>,
        // dependencies: BTreeMap<String, BTreeSet<String>>,
        // targets:BTreeMap<String, String>,
        // self_to_fn: BTreeMap<String, BTreeSet<String>>
        let mut info = BTreeMap::new();
        info.insert("srcs",  json!(self.srcs));
        info.insert("trait_to_struct", json!(self.trait_to_struct));
        info.insert("struct_to_trait", json!(self.struct_to_trait));
        info.insert("dependencies", json!(self.dependencies));
        info.insert("targets", json!(self.targets));
        info.insert("self_to_fn", json!(self.self_to_fn));
        info.insert("type_to_def_path", json!(self.type_to_def_path));
        info.insert("struct_constructor", json!(self.struct_constructor));
        info.insert("single_path_import", json!(self.single_path_import));
        info.insert("glob_path_import", json!(self.glob_path_import));
        // println!("{}", json!(self.dependencies));
        let json: String = serde_json::to_string(&info).unwrap();
        if let Ok(mut file) = std::fs::File::create("preprocess.json"){
            file.write_all(json.as_bytes());
        }

    }


    fn handle_ty_dependencies(&mut self, vty: TyS<'tcx>, key: &str, deps: &mut BTreeSet<String>) {
        let ty_name = format!("{}", vty);
        match vty.kind() {
            TyKindS::Bool => {
            }
            TyKindS::Char => {
            }
            TyKindS::Str => {
            }
            TyKindS::Int(int_ty) => {
            }
            TyKindS::Uint(uint_ty) => {
            }
            TyKindS::Float(float_ty) => {
            }
            TyKindS::Ref(_, rty, mutbility) => {
                self.handle_ty_dependencies(*rty, key, deps);
            }
            TyKindS::RawPtr(ty_mut) => {
                self.handle_ty_dependencies(ty_mut.ty, key, deps);
            }
            TyKindS::Array(ety, len) => {
                self.handle_ty_dependencies(*ety, key, deps);
            }
            TyKindS::Slice(ety) => {
                self.handle_ty_dependencies(*ety, key, deps);
            }
            TyKindS::Adt(adt_def, sub_ref) => {
                //our main course!!!
                let adt = EarlyBinder(*adt_def).subst(self.tcx, &sub_ref);
                for variant in adt.variants(){
                    for field in &variant.fields{
                        if field.did.is_local() {
                            self.handle_ty_dependencies(self.tcx.type_of(field.did), key, deps);
                        }
                    }
                }
                let mut q: VecDeque<DefId> = VecDeque::new();
                q.push_back(adt.did());
                let mut shown = BTreeSet::new();
                shown.insert(adt.did());
                while !q.is_empty() {
                    let cid = q.pop_front().unwrap();
                    deps.insert(self.tcx.def_path_str(cid));
                    let df = self.tcx.def_path_str(cid);
                    if let Some(impls) = self.trait_to_struct.get(&df){
                        if let Some(ans) = impls.first(){
                            if let Some(did) = self.struct_to_def_id.get(ans){
                                if !is_template(self.tcx.type_of(did)) {
                                    deps.insert(ans.clone());
                                    if did.is_local() && !shown.contains(did){
                                        shown.insert(*did);
                                        self.handle_ty_dependencies(self.tcx.type_of(did), key, deps);
                                    }

                                }
                            }else {
                                deps.insert(ans.clone());
                            }
                            //
                        }
                    }
                    let mut generic_preds = self.tcx.explicit_predicates_of(cid);
                    for pred in generic_preds.predicates{
                        if let PredicateKind::Clause(cls) =  pred.0.kind().skip_binder(){
                            if let Clause::Trait(trait_pred) = cls{
                                deps.insert(String::from(self.tcx.def_path_str(trait_pred.trait_ref.def_id)));
                                if let Some(impls) = self.trait_to_struct.get(&*String::from(self.tcx.def_path_str(trait_pred.trait_ref.def_id))){
                                    if let Some(ans) = impls.first(){
                                        if let Some(did) = self.name_to_def_id.get(ans){
                                            // println!("err {:?}", self.tcx.type_of(did));
                                            if !is_template(self.tcx.type_of(did)) {
                                                deps.insert(ans.clone());
                                                if did.is_local() && !shown.contains(did){
                                                    shown.insert(*did);
                                                    self.handle_ty_dependencies(self.tcx.type_of(did), key, deps);
                                                }
                                            }
                                        }else {
                                            deps.insert(ans.clone());
                                        }
                                        //
                                    }
                                }
                                if let Some(local) = trait_pred.trait_ref.def_id.as_local() {
                                    let item = self.tcx.hir().expect_item(local);
                                    if let ItemKind::Trait(_, _,_, bounds, _) = item.kind {

                                        for bound in bounds{
                                            if let Some(tf) = bound.trait_ref(){
                                                if let Some(tid) = tf.trait_def_id(){
                                                    q.push_back(tid);
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
            TyKindS::Foreign(fdefid) => {

            }
            TyKindS::FnDef(func_def_id, substs) => {
                let fnsig = self.tcx.fn_sig(func_def_id);
                let subed_fnsig = EarlyBinder(fnsig).subst(self.tcx, substs);
                for (idx, sub_ty) in subed_fnsig.skip_binder().inputs_and_output.iter().enumerate(){
                    self.handle_ty_dependencies(sub_ty, key, deps);
                }
                let mut generic_preds = self.tcx.explicit_predicates_of(func_def_id);
                for pred in generic_preds.predicates{
                    if let PredicateKind::Clause(cls) =  pred.0.kind().skip_binder(){
                        if let Clause::Trait(trait_pred) = cls{
                            deps.insert(String::from(self.tcx.def_path_str(trait_pred.trait_ref.def_id)));
                            if let Some(impls) = self.trait_to_struct.get(&*String::from(self.tcx.def_path_str(trait_pred.trait_ref.def_id))){
                                if let Some(ans) = impls.first(){

                                    if let Some(did) = self.name_to_def_id.get(ans){
                                        if !is_template(self.tcx.type_of(did)) {

                                            deps.insert(ans.clone());
                                            self.handle_ty_dependencies(self.tcx.type_of(did), key, deps);

                                        }
                                    }else {
                                        deps.insert(ans.clone());
                                    }
                                    //
                                }
                            }
                            let trait_def = self.tcx.trait_def(trait_pred.trait_ref.def_id);
                        }
                    }
                }
            }
            TyKindS::FnPtr(fnsig)=> {
                for sub_ty in fnsig.skip_binder().inputs_and_output{
                    self.handle_ty_dependencies(sub_ty, key, deps);
                }
            }
            TyKindS::Dynamic(binders, _, _)=>{

            }
            TyKindS::Tuple(ty_list) => {
                for sub_ty in *ty_list{
                    self.handle_ty_dependencies(sub_ty, key, deps);
                }
            }
            _=>{}

        }
    }


}

struct UseVisitor<'a, 'tcx> {
    tcx: TyCtxt<'tcx>,
    single: &'a mut BTreeMap<String, String>,
    glob: &'a mut BTreeMap<String, String>,
}

impl<'a, 'tcx> Visitor<'tcx> for UseVisitor<'a, 'tcx> {
    type Map = rustc_middle::hir::map::Map<'tcx>;

    fn visit_item(&mut self, item: &'tcx Item<'tcx>) {
        match item.kind {
            // Here, we match against ItemKind::Use where we can further inspect the use path
            ItemKind::Use(path, kind) => {
                let mut prefix = String::from(self.tcx.def_path_str(item.owner_id.to_def_id()));
                let idx = prefix.find("{").unwrap();
                prefix = prefix[0..idx].to_string();
                let UsePath { segments, ref res, span } = *path;
                if kind == UseKind::Single{
                    let pp = self.tcx
                        .sess
                        .source_map().span_to_snippet(item.vis_span).unwrap();
                    if pp.eq("pub"){
                        let last = segments.last().unwrap();
                        for v in res{
                            if let Res::Def(_, did) = v{
                                // println!("{} -> {}", self.tcx.def_path_str(*did), prefix.clone()+last.ident.as_str());
                                self.single.insert(self.tcx.def_path_str(*did), prefix.clone()+last.ident.as_str());
                            }

                        }
                    }
                }else if kind == UseKind::Glob {
                    let pp = self.tcx
                        .sess
                        .source_map().span_to_snippet(item.vis_span).unwrap();
                    if pp.eq("pub"){
                        let header = self.tcx
                            .sess
                            .source_map().span_to_snippet(item.span).unwrap();
                        // let last = segments.last().unwrap();
                        for v in res{
                            if let Res::Def(_, did) = v{
                                self.glob.insert(self.tcx.def_path_str(*did), prefix.clone());
                            }

                        }
                    }
                }
                // Found a public use statement; do something with it
                // Use the TyCtxt to get more information if needed


            }
            _ => {}
        }
        intravisit::walk_item(self, item);
    }
}
