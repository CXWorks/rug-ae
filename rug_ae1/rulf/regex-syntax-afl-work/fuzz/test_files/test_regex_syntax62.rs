#[macro_use]
extern crate afl;
extern crate regex_syntax;
fn _to_u8(data:&[u8], index:usize)->u8 {
    data[index]
}


fn test_function62(_param0 :u8 ,_param1 :u8) {
    let _local0 = regex_syntax::hir::ClassBytesRange::new(_param0 ,_param1);
    let _ = regex_syntax::hir::ClassBytesRange::start(&(_local0));
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 2 {return;}
        let _param0 = _to_u8(data, 0);
        let _param1 = _to_u8(data, 1);
        test_function62(_param0 ,_param1);
    });
}
