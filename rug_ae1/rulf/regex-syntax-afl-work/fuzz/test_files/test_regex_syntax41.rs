#[macro_use]
extern crate afl;
extern crate regex_syntax;
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


fn test_function41(_param0 :bool ,_param1 :bool) {
    let mut _local0 = regex_syntax::ParserBuilder::new();
    let _local1 = regex_syntax::ParserBuilder::dot_matches_new_line(&mut (_local0) ,_param0);
    let _ = regex_syntax::ParserBuilder::swap_greed(&mut (_local0) ,_param1);
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 2 {return;}
        let _param0 = _to_bool(data, 0);
        let _param1 = _to_bool(data, 1);
        test_function41(_param0 ,_param1);
    });
}
