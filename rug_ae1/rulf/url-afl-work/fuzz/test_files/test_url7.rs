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


fn test_function7(_param0 :&str) {
    let _local0 = url::Url::parse(_param0);
    let _local1_param0_helper1 = _unwrap_result(_local0);
    let _ = url::Url::cannot_be_a_base(&(_local1_param0_helper1));
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() < 1 {return;}
        let dynamic_length = (data.len() - 0) / 1;
        let _param0 = _to_str(data, 0 + 0 * dynamic_length, data.len());
        test_function7(_param0);
    });
}
