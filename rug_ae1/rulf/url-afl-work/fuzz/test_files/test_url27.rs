#[macro_use]
extern crate afl;
extern crate url;
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


fn test_function27(_param0 :&str ,_param1 :&str) {
    let _std_type0 = if let Ok(ip_addr) = _param1.parse::<std::net::IpAddr>() {ip_addr} else {std::process::exit(-1);};
    let mut _local0 = url::Url::parse(_param0);
    let mut _local1_param0_helper1 = _unwrap_result(_local0);
    let _ = url::Url::set_ip_host(&mut (_local1_param0_helper1) ,_std_type0);
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() < 2 {return;}
        let dynamic_length = (data.len() - 0) / 2;
        let _param0 = _to_str(data, 0 + 0 * dynamic_length, 0 + 1 * dynamic_length);
        let _param1 = _to_str(data, 0 + 1 * dynamic_length, data.len());
        test_function27(_param0 ,_param1);
    });
}
