#[macro_use]
extern crate afl;
extern crate clap;
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


fn test_function8(_param0 :&str ,_param1 :bool ,_param2 :&str) {
    let _local0 = clap::Arg::with_name(_param0);
    let _local1 = clap::Arg::allow_hyphen_values(_local0 ,_param1);
    let _ = clap::Arg::required_unless(_local1 ,_param2);
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() < 3 {return;}
        let dynamic_length = (data.len() - 1) / 2;
        let _param0 = _to_str(data, 1 + 0 * dynamic_length, 1 + 1 * dynamic_length);
        let _param1 = _to_bool(data, 0);
        let _param2 = _to_str(data, 1 + 1 * dynamic_length, data.len());
        test_function8(_param0 ,_param1 ,_param2);
    });
}
