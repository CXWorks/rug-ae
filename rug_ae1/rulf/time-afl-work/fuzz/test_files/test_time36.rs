#[macro_use]
extern crate afl;
extern crate time;
fn _to_u8(data:&[u8], index:usize)->u8 {
    data[index]
}

fn _to_u16(data:&[u8], index:usize)->u16 {
    let data0 = _to_u8(data, index) as u16;
    let data1 = _to_u8(data, index+1) as u16;
    data0 << 8 | data1
}


fn test_function36(_param0 :u16) {
    let _local0 = time::UtcOffset::east_minutes(_param0);
    let _ = time::UtcOffset::as_minutes(_local0);
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 2 {return;}
        let _param0 = _to_u16(data, 0);
        test_function36(_param0);
    });
}
