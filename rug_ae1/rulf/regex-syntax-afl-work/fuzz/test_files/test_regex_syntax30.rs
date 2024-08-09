#[macro_use]
extern crate afl;
extern crate regex_syntax;
fn _to_u64(data:&[u8], index:usize)->u64 {
    let data0 = _to_u32(data, index) as u64;
    let data1 = _to_u32(data, index+4) as u64;
    data0 << 32 | data1
}

fn _to_usize(data:&[u8], index:usize)->usize {
    _to_u64(data, index) as usize
}

fn _to_u32(data:&[u8], index:usize)->u32 {
    let data0 = _to_u16(data, index) as u32;
    let data1 = _to_u16(data, index+2) as u32;
    data0 << 16 | data1
}

fn _to_u8(data:&[u8], index:usize)->u8 {
    data[index]
}

fn _to_u16(data:&[u8], index:usize)->u16 {
    let data0 = _to_u8(data, index) as u16;
    let data1 = _to_u8(data, index+1) as u16;
    data0 << 8 | data1
}


fn test_function30(_param0 :usize ,_param1 :usize ,_param2 :usize ,_param3 :usize ,_param4 :usize ,_param5 :usize) {
    let _local0 = regex_syntax::ast::Position::new(_param0 ,_param1 ,_param2);
    let _local1 = regex_syntax::ast::Span::splat(_local0);
    let _local2 = regex_syntax::ast::Position::new(_param3 ,_param4 ,_param5);
    let _ = regex_syntax::ast::Span::with_start(_local1 ,_local2);
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 48 {return;}
        let _param0 = _to_usize(data, 0);
        let _param1 = _to_usize(data, 8);
        let _param2 = _to_usize(data, 16);
        let _param3 = _to_usize(data, 24);
        let _param4 = _to_usize(data, 32);
        let _param5 = _to_usize(data, 40);
        test_function30(_param0 ,_param1 ,_param2 ,_param3 ,_param4 ,_param5);
    });
}
