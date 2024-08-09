#[macro_use]
extern crate afl;
extern crate serde_json;
fn _to_slice<T>(data:&[u8], start_index: usize, end_index: usize)->&[T] {
    let data_slice = &data[start_index..end_index];
    let (_, shorts, _) = unsafe {data_slice.align_to::<T>()};
    shorts
}

use serde_json::de::Read;

fn test_function1(_param0 :&[u8]) {
    let mut _local0 = serde_json::de::SliceRead::new(_param0);
    let _local1 = serde_json::de::Read::next(&mut (_local0));
    let _ = serde_json::de::Read::peek(&mut (_local0));
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() < 1 {return;}
        let dynamic_length = (data.len() - 0) / 1;
        let _param0 = _to_slice::<u8>(data, 0 + 0 * dynamic_length, data.len());
        test_function1(_param0);
    });
}
