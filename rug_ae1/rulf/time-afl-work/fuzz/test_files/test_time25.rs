#[macro_use]
extern crate afl;
extern crate time;
fn _to_f32(data:&[u8], index: usize) -> f32 {
    let data_slice = &data[index..index+4];
    use std::convert::TryInto;
    let data_array:[u8;4] = data_slice.try_into().expect("slice with incorrect length");
    f32::from_le_bytes(data_array)
}


fn test_function25(_param0 :f32) {
    let _local0 = time::Duration::day();
    let _local1 = time::Duration::seconds_f32(_param0);
    let _ = time::Duration::checked_add(_local0 ,_local1);
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 4 {return;}
        let _param0 = _to_f32(data, 0);
        test_function25(_param0);
    });
}
