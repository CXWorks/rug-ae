#[macro_use]
extern crate afl;
extern crate tui;
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


fn test_function59(_param0 :&str) {
    let _std_type0 = std::borrow::Cow::Owned(String::from(_param0));
    let _local0 = tui::text::Text::raw(_std_type0);
    let _ = tui::text::Text::width(&(_local0));
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() < 1 {return;}
        let dynamic_length = (data.len() - 0) / 1;
        let _param0 = _to_str(data, 0 + 0 * dynamic_length, data.len());
        test_function59(_param0);
    });
}
