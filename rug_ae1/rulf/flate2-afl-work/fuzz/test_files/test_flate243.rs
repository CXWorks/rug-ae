#[macro_use]
extern crate afl;
extern crate flate2;
fn _to_slice<T>(data:&[u8], start_index: usize, end_index: usize)->&[T] {
    let data_slice = &data[start_index..end_index];
    let (_, shorts, _) = unsafe {data_slice.align_to::<T>()};
    shorts
}


fn test_function43(_param0 :&[u8]) {
    let _local0 = flate2::Compression::fast();
    let _local1 = flate2::read::GzEncoder::new(_param0 ,_local0);
    let _ = flate2::read::GzEncoder::into_inner(_local1);
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() < 1 {return;}
        let dynamic_length = (data.len() - 0) / 1;
        let _param0 = _to_slice::<u8>(data, 0 + 0 * dynamic_length, data.len());
        test_function43(_param0);
    });
}
