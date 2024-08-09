#[macro_use]
extern crate afl;
extern crate proc_macro2;
fn _to_i8(data:&[u8], index:usize)->i8 {    
    data[index] as i8
}


fn test_function6(_param0 :i8) {
    let mut _local0 = proc_macro2::Literal::i8_suffixed(_param0);
    let _local1 = proc_macro2::Literal::span(&(_local0));
    let _ = proc_macro2::Literal::set_span(&mut (_local0) ,_local1);
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 1 {return;}
        let _param0 = _to_i8(data, 0);
        test_function6(_param0);
    });
}
