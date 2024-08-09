#[macro_use]
extern crate afl;
extern crate tui;
fn _to_u8(data:&[u8], index:usize)->u8 {
    data[index]
}

fn _to_u16(data:&[u8], index:usize)->u16 {
    let data0 = _to_u8(data, index) as u16;
    let data1 = _to_u8(data, index+1) as u16;
    data0 << 8 | data1
}


fn test_function49(_param0 :u16 ,_param1 :u16 ,_param2 :u16 ,_param3 :u16 ,_param4 :u16 ,_param5 :u16 ,_param6 :u16 ,_param7 :u16 ,_param8 :u16 ,_param9 :u16 ,_param10 :u16 ,_param11 :u16) {
    let _local0 = tui::layout::Rect::new(_param0 ,_param1 ,_param2 ,_param3);
    let _local1 = tui::buffer::Buffer::empty(_local0);
    let _local2 = tui::buffer::Buffer::get(&(_local1) ,_param4 ,_param5);
    let _local3 = tui::buffer::Cell::style(_local2);
    let _local4 = tui::layout::Rect::new(_param6 ,_param7 ,_param8 ,_param9);
    let _local5 = tui::buffer::Buffer::empty(_local4);
    let _local6 = tui::buffer::Buffer::get(&(_local5) ,_param10 ,_param11);
    let _local7 = tui::buffer::Cell::style(_local6);
    let _ = tui::style::Style::patch(_local3 ,_local7);
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 24 {return;}
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
        test_function49(_param0 ,_param1 ,_param2 ,_param3 ,_param4 ,_param5 ,_param6 ,_param7 ,_param8 ,_param9 ,_param10 ,_param11);
    });
}
