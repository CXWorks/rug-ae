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

fn _to_u8(data:&[u8], index:usize)->u8 {
    data[index]
}

fn _to_u16(data:&[u8], index:usize)->u16 {
    let data0 = _to_u8(data, index) as u16;
    let data1 = _to_u8(data, index+1) as u16;
    data0 << 8 | data1
}


fn test_function25(_param0 :&str ,_param1 :u16) {
    let mut _local0 = url::Url::parse(_param0);
    let mut _local1_param0_helper1 = _unwrap_result(_local0);
    let _ = url::Url::set_port(&mut (_local1_param0_helper1) ,Some(_param1));
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() < 3 {return;}
        let dynamic_length = (data.len() - 2) / 1;
        let _param0 = _to_str(data, 2 + 0 * dynamic_length, data.len());
        let _param1 = _to_u16(data, 0);
        test_function25(_param0 ,_param1);
    });
}
