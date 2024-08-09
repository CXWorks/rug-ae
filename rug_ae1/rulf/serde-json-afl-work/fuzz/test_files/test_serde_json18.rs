#[macro_use]
extern crate afl;
extern crate serde_json;
fn _unwrap_option<T>(_opt: Option<T>) -> T {
    match _opt {
        Some(_t) => _t,
        None => {
            use std::process;
            process::exit(0);
        }
    }
}

fn _to_f64(data:&[u8], index: usize) -> f64 {
    let data_slice = &data[index..index+8];
    use std::convert::TryInto;
    let data_array:[u8;8] = data_slice.try_into().expect("slice with incorrect length");
    f64::from_le_bytes(data_array)
}


fn test_function18(_param0 :f64) {
    let _local0 = serde_json::Number::from_f64(_param0);
    let _local1_param0_helper1 = _unwrap_option(_local0);
    let _ = serde_json::Number::is_u64(&(_local1_param0_helper1));
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 8 {return;}
        let _param0 = _to_f64(data, 0);
        test_function18(_param0);
    });
}
