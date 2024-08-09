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

fn _to_i8(data:&[u8], index:usize)->i8 {    
    data[index] as i8
}

fn _to_i16(data:&[u8], index:usize)->i16 {
    let data0 = _to_i8(data, index) as i16;
    let data1 = _to_i8(data, index+1) as i16;
    data0 << 8 | data1
}

fn _to_u8(data:&[u8], index:usize)->u8 {
    data[index]
}

fn _to_i32(data:&[u8], index:usize)->i32 {
    let data0 = _to_i16(data, index) as i32;
    let data1 = _to_i16(data, index+2) as i32;
    data0 << 16 | data1
}


fn test_function9(_param0 :i32 ,_param1 :u8 ,_param2 :u8) {
    let _local0 = time::Time::midnight();
    let _local1 = time::Date::try_from_ymd(_param0 ,_param1 ,_param2);
    let _local2_param0_helper1 = _unwrap_result(_local1);
    let _ = time::PrimitiveDateTime::new(_local2_param0_helper1 ,_local0);
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 6 {return;}
        let _param0 = _to_i32(data, 0);
        let _param1 = _to_u8(data, 4);
        let _param2 = _to_u8(data, 5);
        test_function9(_param0 ,_param1 ,_param2);
    });
}
