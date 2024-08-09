#[macro_use]
extern crate afl;
extern crate serde_json;
fn _to_bool(data:&[u8], index: usize)->bool {
    let bool_value = _to_u8(data, index);
    if bool_value %2 == 0 {
        true
    } else {
        false
    }
}

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

fn _to_u8(data:&[u8], index:usize)->u8 {
    data[index]
}

use serde_json::de::Read;

fn test_function8(_param0 :&str ,mut _param1 :bool) {
    let mut _local0 = serde_json::de::StrRead::new(_param0);
    let _ = serde_json::de::Read::set_failed(&mut (_local0) ,&mut (_param1));
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() < 2 {return;}
        let dynamic_length = (data.len() - 1) / 1;
        let _param0 = _to_str(data, 1 + 0 * dynamic_length, data.len());
        let _param1 = _to_bool(data, 0);
        test_function8(_param0 ,_param1);
    });
}
