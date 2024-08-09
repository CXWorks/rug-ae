#[macro_use]
extern crate afl;
extern crate flate2;
fn _to_slice<T>(data:&[u8], start_index: usize, end_index: usize)->&[T] {
    let data_slice = &data[start_index..end_index];
    let (_, shorts, _) = unsafe {data_slice.align_to::<T>()};
    shorts
}


fn test_function9(_param0 :&[u8] ,_param1 :&[u8]) {
    let mut _local0 = flate2::read::ZlibDecoder::new(_param0);
    let _local1 = flate2::read::ZlibDecoder::reset(&mut (_local0) ,_param1);
    let _ = flate2::read::ZlibDecoder::get_ref(&(_local0));
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() < 2 {return;}
        let dynamic_length = (data.len() - 0) / 2;
        let _param0 = _to_slice::<u8>(data, 0 + 0 * dynamic_length, 0 + 1 * dynamic_length);
        let _param1 = _to_slice::<u8>(data, 0 + 1 * dynamic_length, data.len());
        test_function9(_param0 ,_param1);
    });
}
