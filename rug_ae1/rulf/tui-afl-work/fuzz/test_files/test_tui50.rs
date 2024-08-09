#[macro_use]
extern crate afl;
extern crate tui;
fn _unwrap_option<T>(_opt: Option<T>) -> T {
    match _opt {
        Some(_t) => _t,
        None => {
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


fn test_function50(_param0 :u16) {
    let _local0 = tui::style::Modifier::from_bits(_param0);
    let _local1_param0_helper1 = _unwrap_option(_local0);
    let _ = tui::style::Modifier::is_all(&(_local1_param0_helper1));
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 2 {return;}
        let _param0 = _to_u16(data, 0);
        test_function50(_param0);
    });
}
