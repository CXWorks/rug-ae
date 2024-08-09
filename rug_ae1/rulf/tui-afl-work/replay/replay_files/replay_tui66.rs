extern crate tui;
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


fn test_function66(_param0 :&str ,_param1 :(u16 ,u16)) {
    let _std_type0 = std::borrow::Cow::Owned(String::from(_param0));
    let _local0 = tui::text::Text::raw(_std_type0);
    let _local1 = tui::widgets::Paragraph::new(_local0);
    let _ = tui::widgets::Paragraph::scroll(_local1 ,_param1);
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
    if data.len() < 5 {return;}
    let dynamic_length = (data.len() - 4) / 1;
    let _param0 = _to_str(data, 4 + 0 * dynamic_length, data.len());
    let _param1 = (_to_u16(data, 0), _to_u16(data, 2));
    test_function66(_param0 ,_param1);

    println!("No panic!");
}