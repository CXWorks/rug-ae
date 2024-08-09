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

use tui::backend::Backend;

fn test_function69(_param0 :u16 ,_param1 :u16) {
    let mut _default0:Vec<u8> = Vec::new();
    let mut _local0 = tui::backend::TermionBackend::new(&mut (_default0));
    let _local1 = tui::backend::Backend::show_cursor(&mut (_local0));
    let _ = tui::backend::Backend::set_cursor(&mut (_local0) ,_param0 ,_param1);
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 4 {return;}
        let _param0 = _to_u16(data, 0);
        let _param1 = _to_u16(data, 2);
        test_function69(_param0 ,_param1);
    });
}
