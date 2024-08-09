extern crate clap;
fn _to_bool(data:&[u8], index: usize)->bool {
    let bool_value = _to_u8(data, index);
    if bool_value %2 == 0 {
        true
    } else {
        false
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


fn test_function16(_param0 :&str ,_param1 :&str ,_param2 :bool) {
    let _local0 = clap::Arg::with_name(_param0);
    let _local1 = clap::Arg::possible_value(_local0 ,_param1);
    let _ = clap::Arg::case_insensitive(_local1 ,_param2);
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
    if data.len() < 3 {return;}
    let dynamic_length = (data.len() - 1) / 2;
    let _param0 = _to_str(data, 1 + 0 * dynamic_length, 1 + 1 * dynamic_length);
    let _param1 = _to_str(data, 1 + 1 * dynamic_length, data.len());
    let _param2 = _to_bool(data, 0);
    test_function16(_param0 ,_param1 ,_param2);

    println!("No panic!");
}