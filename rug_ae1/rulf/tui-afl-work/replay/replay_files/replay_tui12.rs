extern crate tui;
fn _to_u8(data:&[u8], index:usize)->u8 {
    data[index]
}

fn _to_u16(data:&[u8], index:usize)->u16 {
    let data0 = _to_u8(data, index) as u16;
    let data1 = _to_u8(data, index+1) as u16;
    data0 << 8 | data1
}

use tui::backend::Backend;

fn test_function12(_param0 :u16 ,_param1 :u16 ,_param2 :u16 ,_param3 :u16) {
    let mut _local0 = tui::backend::TestBackend::new(_param0 ,_param1);
    let _local1 = tui::backend::Backend::get_cursor(&mut (_local0));
    let _ = tui::backend::Backend::set_cursor(&mut (_local0) ,_param2 ,_param3);
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
    if data.len() != 8 {return;}
    let _param0 = _to_u16(data, 0);
    let _param1 = _to_u16(data, 2);
    let _param2 = _to_u16(data, 4);
    let _param3 = _to_u16(data, 6);
    test_function12(_param0 ,_param1 ,_param2 ,_param3);

    println!("No panic!");
}