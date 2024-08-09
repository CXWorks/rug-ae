#[macro_use]
extern crate afl;
extern crate time;
fn _to_i16(data:&[u8], index:usize)->i16 {
    let data0 = _to_i8(data, index) as i16;
    let data1 = _to_i8(data, index+1) as i16;
    data0 << 8 | data1
}

fn _to_i8(data:&[u8], index:usize)->i8 {    
    data[index] as i8
}


fn test_function69(_param0 :i16) {
    let _local0 = time::UtcOffset::minutes(_param0);
    let _local1 = time::OffsetDateTime::now_utc();
    let _ = time::OffsetDateTime::to_offset(_local1 ,_local0);
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 2 {return;}
        let _param0 = _to_i16(data, 0);
        test_function69(_param0);
    });
}
