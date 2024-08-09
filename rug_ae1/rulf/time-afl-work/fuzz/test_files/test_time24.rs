#[macro_use]
extern crate afl;
extern crate time;
fn _to_f64(data:&[u8], index: usize) -> f64 {
    let data_slice = &data[index..index+8];
    use std::convert::TryInto;
    let data_array:[u8;8] = data_slice.try_into().expect("slice with incorrect length");
    f64::from_le_bytes(data_array)
}


fn test_function24(_param0 :f64) {
    let _local0 = time::Duration::hour();
    let _local1 = time::Duration::seconds_f64(_param0);
    let _ = time::Duration::checked_add(_local0 ,_local1);
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 8 {return;}
        let _param0 = _to_f64(data, 0);
        test_function24(_param0);
    });
}
