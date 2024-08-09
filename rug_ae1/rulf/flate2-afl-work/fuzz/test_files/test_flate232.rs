#[macro_use]
extern crate afl;
extern crate flate2;
fn _to_bool(data:&[u8], index: usize)->bool {
    let bool_value = _to_u8(data, index);
    if bool_value %2 == 0 {
        true
    } else {
        false
    }
}

fn _to_u32(data:&[u8], index:usize)->u32 {
    let data0 = _to_u16(data, index) as u32;
    let data1 = _to_u16(data, index+2) as u32;
    data0 << 16 | data1
}

fn _to_u8(data:&[u8], index:usize)->u8 {
    data[index]
}

fn _to_u16(data:&[u8], index:usize)->u16 {
    let data0 = _to_u8(data, index) as u16;
    let data1 = _to_u8(data, index+1) as u16;
    data0 << 8 | data1
}


fn test_function32(_param0 :u32 ,_param1 :bool) {
    let _local0 = flate2::Compression::new(_param0);
    let _local1 = flate2::Compress::new(_local0 ,_param1);
    let _ = flate2::Compress::total_out(&(_local1));
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 5 {return;}
        let _param0 = _to_u32(data, 0);
        let _param1 = _to_bool(data, 4);
        test_function32(_param0 ,_param1);
    });
}
