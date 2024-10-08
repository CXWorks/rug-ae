#[macro_use]
extern crate afl;
extern crate semver;
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


fn test_function0(_param0 :u64 ,_param1 :u64 ,_param2 :u64) {
    let _local0 = semver::Version::new(_param0 ,_param1 ,_param2);
    let _local1 = semver::VersionReq::exact(&(_local0));
    let _ = semver::VersionReq::matches(&(_local1) ,&(_local0));
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 24 {return;}
        let _param0 = _to_u64(data, 0);
        let _param1 = _to_u64(data, 8);
        let _param2 = _to_u64(data, 16);
        test_function0(_param0 ,_param1 ,_param2);
    });
}
