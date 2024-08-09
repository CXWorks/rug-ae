#[macro_use]
extern crate afl;
extern crate serde_json;
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

use serde_json::de::Read;

fn test_function4(_param0 :&str) {
    let mut _local0 = serde_json::de::StrRead::new(_param0);
    let _local1 = serde_json::de::Read::decode_hex_escape(&mut (_local0));
    let _ = serde_json::de::Read::byte_offset(&(_local0));
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() < 1 {return;}
        let dynamic_length = (data.len() - 0) / 1;
        let _param0 = _to_str(data, 0 + 0 * dynamic_length, data.len());
        test_function4(_param0);
    });
}
