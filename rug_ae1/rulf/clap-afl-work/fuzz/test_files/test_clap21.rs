#[macro_use]
extern crate afl;
extern crate clap;
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


fn test_function21(_param0 :&str ,_param1 :&str ,_param2 :&str ,_param3 :&str ,_param4 :&str) {
    let _local0 = clap::Arg::with_name(_param0);
    let _local1 = clap::Arg::default_value(_local0 ,_param1);
    let _ = clap::Arg::default_value_if(_local1 ,_param2 ,Some(_param3) ,_param4);
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() < 5 {return;}
        let dynamic_length = (data.len() - 0) / 5;
        let _param0 = _to_str(data, 0 + 0 * dynamic_length, 0 + 1 * dynamic_length);
        let _param1 = _to_str(data, 0 + 1 * dynamic_length, 0 + 2 * dynamic_length);
        let _param2 = _to_str(data, 0 + 2 * dynamic_length, 0 + 3 * dynamic_length);
        let _param3 = _to_str(data, 0 + 3 * dynamic_length, 0 + 4 * dynamic_length);
        let _param4 = _to_str(data, 0 + 4 * dynamic_length, data.len());
        test_function21(_param0 ,_param1 ,_param2 ,_param3 ,_param4);
    });
}
