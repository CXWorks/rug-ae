#[macro_use]
extern crate afl;
extern crate time;
fn _to_u8(data:&[u8], index:usize)->u8 {
    data[index]
}


fn test_function7(_param0 :u8) {
    let _local0 = time::UtcOffset::west_hours(_param0);
    let _local1 = time::PrimitiveDateTime::now();
    let _ = time::PrimitiveDateTime::using_offset(_local1 ,_local0);
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 1 {return;}
        let _param0 = _to_u8(data, 0);
        test_function7(_param0);
    });
}
