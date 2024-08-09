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


fn test_function32(_param0 :u16 ,_param1 :u16 ,_param2 :u16 ,_param3 :u16 ,_param4 :u16 ,_param5 :u16 ,_param6 :u16 ,_param7 :u16) {
    let _local0 = tui::layout::Rect::new(_param0 ,_param1 ,_param2 ,_param3);
    let _local1 = tui::layout::Rect::new(_param4 ,_param5 ,_param6 ,_param7);
    let _ = tui::layout::Rect::union(_local0 ,_local1);
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 16 {return;}
        let _param0 = _to_u16(data, 0);
        let _param1 = _to_u16(data, 2);
        let _param2 = _to_u16(data, 4);
        let _param3 = _to_u16(data, 6);
        let _param4 = _to_u16(data, 8);
        let _param5 = _to_u16(data, 10);
        let _param6 = _to_u16(data, 12);
        let _param7 = _to_u16(data, 14);
        test_function32(_param0 ,_param1 ,_param2 ,_param3 ,_param4 ,_param5 ,_param6 ,_param7);
    });
}
