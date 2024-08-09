#[macro_use]
extern crate afl;
extern crate time;
fn _unwrap_result<T, E>(_res: Result<T, E>) -> T {
    match _res {
        Ok(_t) => _t,
        Err(_) => {
            use std::process;
            process::exit(0);
        },
    }
}

fn _to_u8(data:&[u8], index:usize)->u8 {
    data[index]
}


fn test_function95(_param0 :u8 ,_param1 :u8 ,_param2 :u8) {
    let _local0 = time::Date::today();
    let _local1 = time::Date::try_with_hms(_local0 ,_param0 ,_param1 ,_param2);
    let _local2_param0_helper1 = _unwrap_result(_local1);
    let _ = time::PrimitiveDateTime::hour(_local2_param0_helper1);
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 3 {return;}
        let _param0 = _to_u8(data, 0);
        let _param1 = _to_u8(data, 1);
        let _param2 = _to_u8(data, 2);
        test_function95(_param0 ,_param1 ,_param2);
    });
}
