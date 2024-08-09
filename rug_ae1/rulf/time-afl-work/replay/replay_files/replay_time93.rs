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

fn _to_u32(data:&[u8], index:usize)->u32 {
    let data0 = _to_u16(data, index) as u32;
    let data1 = _to_u16(data, index+2) as u32;
    data0 << 16 | data1
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

fn _to_u16(data:&[u8], index:usize)->u16 {
    let data0 = _to_u8(data, index) as u16;
    let data1 = _to_u8(data, index+1) as u16;
    data0 << 8 | data1
}


fn test_function93(_param0 :i32 ,_param1 :u16 ,_param2 :u8 ,_param3 :u8 ,_param4 :u8 ,_param5 :u32) {
    let _local0 = time::Date::try_from_yo(_param0 ,_param1);
    let _local1_param0_helper1 = _unwrap_result(_local0);
    let _local1 = time::Date::try_with_hms_nano(_local1_param0_helper1 ,_param2 ,_param3 ,_param4 ,_param5);
    let _local2_param0_helper1 = _unwrap_result(_local1);
    let _ = time::PrimitiveDateTime::monday_based_week(_local2_param0_helper1);
}

fn _read_data()-> Vec<u8> {
    use std::env;
    use std::process::exit;
    let args:Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No crash filename provided");
        exit(-1);
    }
    use std::path::PathBuf;
    let crash_file_name = &args[1];
    let crash_path = PathBuf::from(crash_file_name);
    if !crash_path.is_file() {
        println!("Not a valid crash file");
        exit(-1);
    }
    use std::fs;
    let data =  fs::read(crash_path).unwrap();
    data
}

fn main() {
    let _content = _read_data();
    let data = &_content;
    println!("data = {:?}", data);
    println!("data len = {:?}", data.len());
    //actual body emit
    if data.len() != 13 {return;}
    let _param0 = _to_i32(data, 0);
    let _param1 = _to_u16(data, 4);
    let _param2 = _to_u8(data, 6);
    let _param3 = _to_u8(data, 7);
    let _param4 = _to_u8(data, 8);
    let _param5 = _to_u32(data, 9);
    test_function93(_param0 ,_param1 ,_param2 ,_param3 ,_param4 ,_param5);

    println!("No panic!");
}