#[macro_use]
extern crate afl;
extern crate time;
fn _to_str(data:&[u8], start_index: usize, end_index: usize)->&str {
    let data_slice = &data[start_index..end_index];
    use std::str;
    match str::from_utf8(data_slice) {
        Ok(s)=>s,
        Err(_)=>{
            use std::process;
            process::exit(0);
        }
    }
}


fn test_function14(_param0 :&str) {
    let _local0 = time::OffsetDateTime::unix_epoch();
    let _local1 = time::OffsetDateTime::time(_local0);
    let _ = time::Time::format(_local1 ,_param0);
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() < 1 {return;}
        let dynamic_length = (data.len() - 0) / 1;
        let _param0 = _to_str(data, 0 + 0 * dynamic_length, data.len());
        test_function14(_param0);
    });
}
