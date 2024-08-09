#[macro_use]
extern crate afl;
extern crate regex_syntax;
fn _unwrap_result<T, E>(_res: Result<T, E>) -> T {
    match _res {
        Ok(_t) => _t,
        Err(_) => {
            use std::process;
            process::exit(0);
        },
    }
}

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


fn test_function0(_param0 :bool ,_param1 :&str ,_param2 :&str) {
    let mut _local0 = regex_syntax::hir::translate::TranslatorBuilder::new();
    let _local1 = regex_syntax::hir::translate::TranslatorBuilder::allow_invalid_utf8(&mut (_local0) ,_param0);
    let mut _local2 = regex_syntax::hir::translate::TranslatorBuilder::build(&(_local0));
    let mut _local3 = regex_syntax::ast::parse::Parser::new();
    let _local4 = regex_syntax::ast::parse::Parser::parse(&mut (_local3) ,_param1);
    let _local5_param2_helper1 = _unwrap_result(_local4);
    let _ = regex_syntax::hir::translate::Translator::translate(&mut (_local2) ,_param2 ,&(_local5_param2_helper1));
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() < 3 {return;}
        let dynamic_length = (data.len() - 1) / 2;
        let _param0 = _to_bool(data, 0);
        let _param1 = _to_str(data, 1 + 0 * dynamic_length, 1 + 1 * dynamic_length);
        let _param2 = _to_str(data, 1 + 1 * dynamic_length, data.len());
        test_function0(_param0 ,_param1 ,_param2);
    });
}
