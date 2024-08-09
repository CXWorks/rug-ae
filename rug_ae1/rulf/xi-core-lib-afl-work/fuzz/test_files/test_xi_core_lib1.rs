#[macro_use]
extern crate afl;
extern crate xi_core_lib;
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


fn test_function1(_param0 :usize ,_param1 :usize) {
    let _local0 = xi_core_lib::tabs::BufferId::new(_param0);
    let _local1 = xi_core_lib::tabs::test_helpers::new_view_id(_param1);
    let _local2 = xi_core_lib::view::View::new(_local1 ,_local0);
    let _ = xi_core_lib::view::View::get_caret_offset(&(_local2));
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 16 {return;}
        let _param0 = _to_usize(data, 0);
        let _param1 = _to_usize(data, 8);
        test_function1(_param0 ,_param1);
    });
}
