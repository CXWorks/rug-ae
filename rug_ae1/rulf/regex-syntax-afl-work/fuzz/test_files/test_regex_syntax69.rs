#[macro_use]
extern crate afl;
extern crate regex_syntax;
fn _to_u8(data:&[u8], index:usize)->u8 {
    data[index]
}


fn test_function69(_param0 :u8) {
    let _ = regex_syntax::is_word_byte(_param0);
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 1 {return;}
        let _param0 = _to_u8(data, 0);
        test_function69(_param0);
    });
}
