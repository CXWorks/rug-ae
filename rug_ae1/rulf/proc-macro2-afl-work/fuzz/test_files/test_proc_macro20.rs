#[macro_use]
extern crate afl;
extern crate proc_macro2;
fn _to_str(data:&[u8], start_index: usize, end_index: usize)->&str {
    let data_slice = &data[start_index..end_index];
    use std::str;
    match str::from_utf8(data_slice) {
        Ok(s)=>s,
        Err(_)=>{
            use std::process;
            process::exit(0);
        }
    }
}

fn _to_u8(data:&[u8], index:usize)->u8 {
    data[index]
}


fn test_function0(_param0 :&str ,_param1 :u8) {
    let _local0 = proc_macro2::Span::call_site();
    let mut _local1 = proc_macro2::Ident::new(_param0 ,_local0);
    let _local2 = proc_macro2::Literal::u8_suffixed(_param1);
    let _local3 = proc_macro2::Literal::span(&(_local2));
    let _ = proc_macro2::Ident::set_span(&mut (_local1) ,_local3);
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() < 2 {return;}
        let dynamic_length = (data.len() - 1) / 1;
        let _param0 = _to_str(data, 1 + 0 * dynamic_length, data.len());
        let _param1 = _to_u8(data, 0);
        test_function0(_param0 ,_param1);
    });
}
