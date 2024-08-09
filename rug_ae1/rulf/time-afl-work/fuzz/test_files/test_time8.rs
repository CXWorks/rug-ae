#[macro_use]
extern crate afl;
extern crate time;
fn _to_i8(data:&[u8], index:usize)->i8 {    
    data[index] as i8
}


fn test_function8(_param0 :i8) {
    let _local0 = time::UtcOffset::hours(_param0);
    let _local1 = time::PrimitiveDateTime::unix_epoch();
    let _ = time::PrimitiveDateTime::assume_offset(_local1 ,_local0);
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 1 {return;}
        let _param0 = _to_i8(data, 0);
        test_function8(_param0);
    });
}
