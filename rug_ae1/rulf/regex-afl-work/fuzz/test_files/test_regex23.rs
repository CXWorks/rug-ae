#[macro_use]
extern crate afl;
extern crate regex;
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


fn test_function23(_param0 :&str ,_param1 :&str ,_param2 :&str) {
    let _local0 = regex::Regex::new(_param0);
    let _local1 = regex::Regex::new(_param1);
    let _local2_param0_helper1 = _unwrap_result(_local1);
    let mut _local2 = regex::Regex::capture_locations(&(_local2_param0_helper1));
    let _local3_param0_helper1 = _unwrap_result(_local0);
    let _ = regex::Regex::captures_read(&(_local3_param0_helper1) ,&mut (_local2) ,_param2);
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() < 3 {return;}
        let dynamic_length = (data.len() - 0) / 3;
        let _param0 = _to_str(data, 0 + 0 * dynamic_length, 0 + 1 * dynamic_length);
        let _param1 = _to_str(data, 0 + 1 * dynamic_length, 0 + 2 * dynamic_length);
        let _param2 = _to_str(data, 0 + 2 * dynamic_length, data.len());
        test_function23(_param0 ,_param1 ,_param2);
    });
}
