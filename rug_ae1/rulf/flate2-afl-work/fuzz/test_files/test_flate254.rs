#[macro_use]
extern crate afl;
extern crate flate2;
fn _to_slice<T>(data:&[u8], start_index: usize, end_index: usize)->&[T] {
    let data_slice = &data[start_index..end_index];
    let (_, shorts, _) = unsafe {data_slice.align_to::<T>()};
    shorts
}


fn test_function54(_param0 :&[u8] ,_param1 :&[u8] ,_param2 :&[u8]) {
    let _std_type0 = _param1.to_vec();
    let mut _local0 = flate2::read::DeflateDecoder::new_with_buf(_param0 ,_std_type0);
    let _local1 = flate2::read::DeflateDecoder::reset(&mut (_local0) ,_param2);
    let _ = flate2::read::DeflateDecoder::total_out(&(_local0));
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() < 3 {return;}
        let dynamic_length = (data.len() - 0) / 3;
        let _param0 = _to_slice::<u8>(data, 0 + 0 * dynamic_length, 0 + 1 * dynamic_length);
        let _param1 = _to_slice::<u8>(data, 0 + 1 * dynamic_length, 0 + 2 * dynamic_length);
        let _param2 = _to_slice::<u8>(data, 0 + 2 * dynamic_length, data.len());
        test_function54(_param0 ,_param1 ,_param2);
    });
}
