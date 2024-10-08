#[macro_use]
extern crate afl;
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


fn test_function74(_param0 :u16 ,_param1 :u16 ,_param2 :u16 ,_param3 :u16 ,_param4 :u16 ,_param5 :u16 ,_param6 :&str) {
    let _local0 = tui::layout::Rect::new(_param0 ,_param1 ,_param2 ,_param3);
    let mut _local1 = tui::buffer::Buffer::empty(_local0);
    let _local2 = tui::buffer::Buffer::get_mut(&mut (_local1) ,_param4 ,_param5);
    let _ = tui::buffer::Cell::set_symbol(_local2 ,_param6);
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() < 13 {return;}
        let dynamic_length = (data.len() - 12) / 1;
        let _param0 = _to_u16(data, 0);
        let _param1 = _to_u16(data, 2);
        let _param2 = _to_u16(data, 4);
        let _param3 = _to_u16(data, 6);
        let _param4 = _to_u16(data, 8);
        let _param5 = _to_u16(data, 10);
        let _param6 = _to_str(data, 12 + 0 * dynamic_length, data.len());
        test_function74(_param0 ,_param1 ,_param2 ,_param3 ,_param4 ,_param5 ,_param6);
    });
}
