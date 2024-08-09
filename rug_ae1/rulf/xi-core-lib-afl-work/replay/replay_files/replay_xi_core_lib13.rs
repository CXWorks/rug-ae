extern crate xi_core_lib;
fn _to_bool(data:&[u8], index: usize)->bool {
    let bool_value = _to_u8(data, index);
    if bool_value %2 == 0 {
        true
    } else {
        false
    }
}

fn _to_u32(data:&[u8], index:usize)->u32 {
    let data0 = _to_u16(data, index) as u32;
    let data1 = _to_u16(data, index+2) as u32;
    data0 << 16 | data1
}

fn _to_u8(data:&[u8], index:usize)->u8 {
    data[index]
}

fn _to_u16(data:&[u8], index:usize)->u16 {
    let data0 = _to_u8(data, index) as u16;
    let data1 = _to_u8(data, index+1) as u16;
    data0 << 8 | data1
}


fn test_function13(_param0 :u16 ,_param1 :u32 ,_param2 :u32 ,_param3 :u16 ,_param4 :bool ,_param5 :bool) {
    let _local0 = xi_core_lib::styles::Style::new(_param0 ,Some(_param1) ,Some(_param2) ,Some(_param3) ,Some(_param4) ,Some(_param5));
    let _ = xi_core_lib::styles::Style::merge(&(_local0) ,&(_local0));
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
    if data.len() != 14 {return;}
    let _param0 = _to_u16(data, 0);
    let _param1 = _to_u32(data, 2);
    let _param2 = _to_u32(data, 6);
    let _param3 = _to_u16(data, 10);
    let _param4 = _to_bool(data, 12);
    let _param5 = _to_bool(data, 13);
    test_function13(_param0 ,_param1 ,_param2 ,_param3 ,_param4 ,_param5);

    println!("No panic!");
}