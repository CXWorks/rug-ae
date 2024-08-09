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


fn test_function32(_param0 :&str ,_param1 :&str ,_param2 :&str) {
    let _local0 = clap::SubCommand::with_name(_param0);
    let _local1 = clap::App::after_help(_local0 ,_param1);
    let _ = clap::App::before_help(_local1 ,_param2);
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() < 3 {return;}
        let dynamic_length = (data.len() - 0) / 3;
        let _param0 = _to_str(data, 0 + 0 * dynamic_length, 0 + 1 * dynamic_length);
        let _param1 = _to_str(data, 0 + 1 * dynamic_length, 0 + 2 * dynamic_length);
        let _param2 = _to_str(data, 0 + 2 * dynamic_length, data.len());
        test_function32(_param0 ,_param1 ,_param2);
    });
}
