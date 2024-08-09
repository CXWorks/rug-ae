#[macro_use]
extern crate afl;
extern crate flate2;
fn _to_u32(data:&[u8], index:usize)->u32 {
    let data0 = _to_u16(data, index) as u32;
    let data1 = _to_u16(data, index+2) as u32;
    data0 << 16 | data1
}

fn _to_u8(data:&[u8], index:usize)->u8 {
    data[index]
}

fn _to_slice<T>(data:&[u8], start_index: usize, end_index: usize)->&[T] {
    let data_slice = &data[start_index..end_index];
    let (_, shorts, _) = unsafe {data_slice.align_to::<T>()};
    shorts
}

fn _to_u16(data:&[u8], index:usize)->u16 {
    let data0 = _to_u8(data, index) as u16;
    let data1 = _to_u8(data, index+1) as u16;
    data0 << 8 | data1
}


fn test_function34(_param0 :u32 ,_param1 :&[u8]) {
    let _local0 = flate2::Compression::new(_param0);
    let _local1 = flate2::bufread::DeflateEncoder::new(_param1 ,_local0);
    let _ = flate2::bufread::DeflateEncoder::get_ref(&(_local1));
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() < 5 {return;}
        let dynamic_length = (data.len() - 4) / 1;
        let _param0 = _to_u32(data, 0);
        let _param1 = _to_slice::<u8>(data, 4 + 0 * dynamic_length, data.len());
        test_function34(_param0 ,_param1);
    });
}
