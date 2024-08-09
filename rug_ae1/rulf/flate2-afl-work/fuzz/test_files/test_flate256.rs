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

fn _to_u8(data:&[u8], index:usize)->u8 {
    data[index]
}


fn test_function56(_param0 :bool) {
    let _local0 = flate2::Decompress::new(_param0);
    let _ = flate2::Decompress::total_out(&(_local0));
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 1 {return;}
        let _param0 = _to_bool(data, 0);
        test_function56(_param0);
    });
}
