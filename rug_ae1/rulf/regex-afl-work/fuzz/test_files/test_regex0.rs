#[macro_use]
extern crate afl;
extern crate regex;
fn _unwrap_option<T>(_opt: Option<T>) -> T {
    match _opt {
        Some(_t) => _t,
        None => {
            use std::process;
            process::exit(0);
        }
    }
}

fn _unwrap_result<T, E>(_res: Result<T, E>) -> T {
    match _res {
        Ok(_t) => _t,
        Err(_) => {
            use std::process;
            process::exit(0);
        },
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


fn test_function0(_param0 :&str ,_param1 :&str ,_param2 :&str ,_param3 :&str) {
    let _local0 = regex::Regex::new(_param0);
    let _local1_param0_helper1 = _unwrap_result(_local0);
    let _local1 = regex::Regex::captures(&(_local1_param0_helper1) ,_param1);
    let mut _local2 = regex::escape(_param2);
    let _local3_param0_helper1 = _unwrap_option(_local1);
    let _ = regex::Captures::expand(&(_local3_param0_helper1) ,_param3 ,&mut (_local2));
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() < 4 {return;}
        let dynamic_length = (data.len() - 0) / 4;
        let _param0 = _to_str(data, 0 + 0 * dynamic_length, 0 + 1 * dynamic_length);
        let _param1 = _to_str(data, 0 + 1 * dynamic_length, 0 + 2 * dynamic_length);
        let _param2 = _to_str(data, 0 + 2 * dynamic_length, 0 + 3 * dynamic_length);
        let _param3 = _to_str(data, 0 + 3 * dynamic_length, data.len());
        test_function0(_param0 ,_param1 ,_param2 ,_param3);
    });
}
