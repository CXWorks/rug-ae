#[macro_use]
extern crate afl;
extern crate time;
fn _to_u8(data:&[u8], index:usize)->u8 {
    data[index]
}


fn test_function6(_param0 :u8) {
    let _local0 = time::UtcOffset::east_hours(_param0);
    let _local1 = time::OffsetDateTime::now();
    let _ = time::OffsetDateTime::to_offset(_local1 ,_local0);
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 1 {return;}
        let _param0 = _to_u8(data, 0);
        test_function6(_param0);
    });
}
