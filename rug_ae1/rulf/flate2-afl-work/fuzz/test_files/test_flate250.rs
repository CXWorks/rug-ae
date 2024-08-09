#[macro_use]
extern crate afl;
extern crate flate2;
fn _to_u8(data:&[u8], index:usize)->u8 {
    data[index]
}

fn _to_slice<T>(data:&[u8], start_index: usize, end_index: usize)->&[T] {
    let data_slice = &data[start_index..end_index];
    let (_, shorts, _) = unsafe {data_slice.align_to::<T>()};
    shorts
}


fn test_function50(_param0 :u8 ,_param1 :&[u8]) {
    let _std_type0 = _param1.to_vec();
    let _local0 = flate2::GzBuilder::new();
    let _local1 = flate2::GzBuilder::operating_system(_local0 ,_param0);
    let _ = flate2::GzBuilder::comment(_local1 ,_std_type0);
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() < 2 {return;}
        let dynamic_length = (data.len() - 1) / 1;
        let _param0 = _to_u8(data, 0);
        let _param1 = _to_slice::<u8>(data, 1 + 0 * dynamic_length, data.len());
        test_function50(_param0 ,_param1);
    });
}
