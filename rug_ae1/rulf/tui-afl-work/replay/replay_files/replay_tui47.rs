extern crate tui;
fn _to_u8(data:&[u8], index:usize)->u8 {
    data[index]
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

fn _to_u16(data:&[u8], index:usize)->u16 {
    let data0 = _to_u8(data, index) as u16;
    let data1 = _to_u8(data, index+1) as u16;
    data0 << 8 | data1
}


fn test_function47(_param0 :u16 ,_param1 :u16 ,_param2 :u16 ,_param3 :u16 ,_param4 :u16 ,_param5 :u16 ,_param6 :u16 ,_param7 :u16 ,_param8 :u16 ,_param9 :u16 ,_param10 :u16 ,_param11 :u16 ,_param12 :&str) {
    let _local0 = tui::layout::Rect::new(_param0 ,_param1 ,_param2 ,_param3);
    let mut _local1 = tui::buffer::Buffer::empty(_local0);
    let _local2 = tui::layout::Rect::new(_param4 ,_param5 ,_param6 ,_param7);
    let _local3 = tui::buffer::Buffer::empty(_local2);
    let _local4 = tui::buffer::Buffer::get(&(_local3) ,_param8 ,_param9);
    let _local5 = tui::buffer::Cell::style(_local4);
    let _ = tui::buffer::Buffer::set_string(&mut (_local1) ,_param10 ,_param11 ,_param12 ,_local5);
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
    if data.len() < 25 {return;}
    let dynamic_length = (data.len() - 24) / 1;
    let _param0 = _to_u16(data, 0);
    let _param1 = _to_u16(data, 2);
    let _param2 = _to_u16(data, 4);
    let _param3 = _to_u16(data, 6);
    let _param4 = _to_u16(data, 8);
    let _param5 = _to_u16(data, 10);
    let _param6 = _to_u16(data, 12);
    let _param7 = _to_u16(data, 14);
    let _param8 = _to_u16(data, 16);
    let _param9 = _to_u16(data, 18);
    let _param10 = _to_u16(data, 20);
    let _param11 = _to_u16(data, 22);
    let _param12 = _to_str(data, 24 + 0 * dynamic_length, data.len());
    test_function47(_param0 ,_param1 ,_param2 ,_param3 ,_param4 ,_param5 ,_param6 ,_param7 ,_param8 ,_param9 ,_param10 ,_param11 ,_param12);

    println!("No panic!");
}