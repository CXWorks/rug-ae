#[macro_use]
extern crate afl;
extern crate xi_core_lib;
fn _to_u64(data:&[u8], index:usize)->u64 {
    let data0 = _to_u32(data, index) as u64;
    let data1 = _to_u32(data, index+4) as u64;
    data0 << 32 | data1
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


fn test_function23(_param0 :(u64 ,u32)) {
    let mut _local0 = xi_core_lib::editor::Editor::new();
    let _ = xi_core_lib::editor::Editor::dec_revs_in_flight(&mut (_local0));
    let _ = xi_core_lib::editor::Editor::set_session_id(&mut (_local0) ,_param0);
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 12 {return;}
        let _param0 = (_to_u64(data, 0), _to_u32(data, 8));
        test_function23(_param0);
    });
}
