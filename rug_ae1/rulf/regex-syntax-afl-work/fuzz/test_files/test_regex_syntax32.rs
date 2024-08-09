#[macro_use]
extern crate afl;
extern crate regex_syntax;
fn _to_u8(data:&[u8], index:usize)->u8 {
    data[index]
}


fn test_function32(_param0 :u8 ,_param1 :u8) {
    let mut _local0 = regex_syntax::hir::ClassBytes::empty();
    let _local1 = regex_syntax::hir::ClassBytesRange::new(_param0 ,_param1);
    let _ = regex_syntax::hir::ClassBytes::push(&mut (_local0) ,_local1);
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 2 {return;}
        let _param0 = _to_u8(data, 0);
        let _param1 = _to_u8(data, 1);
        test_function32(_param0 ,_param1);
    });
}
