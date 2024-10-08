#[macro_use]
extern crate afl;
extern crate flate2;
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


fn test_function80(_param0 :u32) {
    let mut _default0:Vec<u8> = Vec::new();
    let mut _default1:Vec<u8> = Vec::new();
    let _local0 = flate2::Compression::new(_param0);
    let mut _local1 = flate2::write::DeflateEncoder::new(&mut (_default0) ,_local0);
    let _ = flate2::write::DeflateEncoder::reset(&mut (_local1) ,&mut (_default1));
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 4 {return;}
        let _param0 = _to_u32(data, 0);
        test_function80(_param0);
    });
}
