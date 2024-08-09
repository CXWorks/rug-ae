#![feature(assoc_char_funcs)]
#[macro_use]
extern crate afl;
extern crate regex_syntax;
fn _to_u32(data:&[u8], index:usize)->u32 {
    let data0 = _to_u16(data, index) as u32;
    let data1 = _to_u16(data, index+2) as u32;
    data0 << 16 | data1
}

fn _to_char(data:&[u8], index: usize)->char {
    let char_value = _to_u32(data,index);
    match char::from_u32(char_value) {
        Some(c)=>c,
        None=>{
            use std::process;
            process::exit(0);
        }
    }
}

fn _to_u8(data:&[u8], index:usize)->u8 {
    data[index]
}

fn _to_u16(data:&[u8], index:usize)->u16 {
    let data0 = _to_u8(data, index) as u16;
    let data1 = _to_u8(data, index+1) as u16;
    data0 << 8 | data1
}


fn test_function48(_param0 :char ,_param1 :char) {
    let _local0 = regex_syntax::hir::ClassUnicodeRange::new(_param0 ,_param1);
    let _ = regex_syntax::hir::ClassUnicodeRange::start(&(_local0));
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 8 {return;}
        let _param0 = _to_char(data, 0);
        let _param1 = _to_char(data, 4);
        test_function48(_param0 ,_param1);
    });
}
