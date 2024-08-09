#[macro_use]
extern crate afl;
extern crate proc_macro2;
fn _to_slice<T>(data:&[u8], start_index: usize, end_index: usize)->&[T] {
    let data_slice = &data[start_index..end_index];
    let (_, shorts, _) = unsafe {data_slice.align_to::<T>()};
    shorts
}


fn test_function30(_param0 :&[u8]) {
    let mut _local0 = proc_macro2::Literal::byte_string(_param0);
    let _local1 = proc_macro2::Literal::span(&(_local0));
    let _ = proc_macro2::Literal::set_span(&mut (_local0) ,_local1);
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() < 1 {return;}
        let dynamic_length = (data.len() - 0) / 1;
        let _param0 = _to_slice::<u8>(data, 0 + 0 * dynamic_length, data.len());
        test_function30(_param0);
    });
}
