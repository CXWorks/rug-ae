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


fn test_function13(_param0 :u8 ,_param1 :u8 ,_param2 :u8) {
    let _local0 = time::OffsetDateTime::try_now_local();
    let _local1_param0_helper1 = _unwrap_result(_local0);
    let _local1 = time::OffsetDateTime::date(_local1_param0_helper1);
    let _ = time::Date::try_with_hms(_local1 ,_param0 ,_param1 ,_param2);
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
    if data.len() != 3 {return;}
    let _param0 = _to_u8(data, 0);
    let _param1 = _to_u8(data, 1);
    let _param2 = _to_u8(data, 2);
    test_function13(_param0 ,_param1 ,_param2);

    println!("No panic!");
}