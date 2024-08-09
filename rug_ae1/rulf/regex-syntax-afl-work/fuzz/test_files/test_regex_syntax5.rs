#[macro_use]
extern crate afl;
extern crate regex_syntax;
fn _to_bool(data:&[u8], index: usize)->bool {
    let bool_value = _to_u8(data, index);
    if bool_value %2 == 0 {
        true
    } else {
        false
    }
}

fn _to_u8(data:&[u8], index:usize)->u8 {
    data[index]
}

fn _to_slice<T>(data:&[u8], start_index: usize, end_index: usize)->&[T] {
    let data_slice = &data[start_index..end_index];
    let (_, shorts, _) = unsafe {data_slice.align_to::<T>()};
    shorts
}


fn test_function5(_param0 :bool ,_param1 :&[u8]) {
    let _local0 = regex_syntax::hir::Hir::dot(_param0);
    let mut _local1 = regex_syntax::hir::literal::Literals::suffixes(&(_local0));
    let _ = regex_syntax::hir::literal::Literals::cross_add(&mut (_local1) ,_param1);
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() < 2 {return;}
        let dynamic_length = (data.len() - 1) / 1;
        let _param0 = _to_bool(data, 0);
        let _param1 = _to_slice::<u8>(data, 1 + 0 * dynamic_length, data.len());
        test_function5(_param0 ,_param1);
    });
}
