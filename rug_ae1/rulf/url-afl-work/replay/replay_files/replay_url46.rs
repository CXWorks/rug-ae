extern crate url;
fn _unwrap_result<T, E>(_res: Result<T, E>) -> T {
    match _res {
        Ok(_t) => _t,
        Err(_) => {
            use std::process;
            process::exit(0);
        },
    }
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


fn test_function46(_param0 :&str ,_param1 :&str) {
    let mut _local0 = url::Url::parse(_param0);
    let mut _local1_param0_helper1 = _unwrap_result(_local0);
    let _ = url::quirks::set_username(&mut (_local1_param0_helper1) ,_param1);
    decompose_url__Url(&_local1_param0_helper1 as * const _ as usize, String::from("a"))
}

fn decompose_String(a:usize, b: String){
unsafe{
    let astr = &*(a as * const String);
    println!("{}:{}", b, astr);
}
}

fn read_unsigned_int(ptr: usize, size: usize) -> u64{
        let mut ans = 0u64;
        match size {
            1 => { ans = unsafe{*(ptr as * const u8)} as u64;}
            2 => { ans = unsafe{*(ptr as * const u16)}  as u64;}
            4 => { ans = unsafe{*(ptr as * const u32)}  as u64;}
            8 => { ans = unsafe{*(ptr as * const u64)};}
            _ => {}
        }
        return ans;
    }

    fn encode_niche(flag: u64, a : u64,b: u64,c: u64,d: u64) -> u64{
        if let Some(sc) = flag.checked_add(b){
            if(sc <= c){
                return sc;
            }
        }
        return d;
    }

fn decompose_u32(a_ele: usize, mut name_offset: String){
let a_ele_p = (a_ele as * const usize as usize + 0) as * const u32;
println!("{}~u32:{}", name_offset, unsafe{ *a_ele_p });

}

fn decompose_u8(a_ele: usize, mut name_offset: String){
let a_ele_p = (a_ele as * const usize as usize + 0) as * const u8;
println!("{}~u8:{}", name_offset, unsafe{ *a_ele_p });

}

fn decompose__lm_u8_then_4_rm_(a_ele: usize, mut name_offset: String){
for i in 0..4{
decompose_u8(a_ele + i * 1, name_offset.clone() + &i.to_string());
}

}

fn decompose_std__net__Ipv4Addr(a_ele: usize, mut name_offset: String){
decompose__lm_u8_then_4_rm_(a_ele + 0, name_offset.clone()  + "~octets");

}

fn decompose__lm_u8_then_16_rm_(a_ele: usize, mut name_offset: String){
for i in 0..16{
decompose_u8(a_ele + i * 1, name_offset.clone() + &i.to_string());
}

}

fn decompose_std__net__Ipv6Addr(a_ele: usize, mut name_offset: String){
decompose__lm_u8_then_16_rm_(a_ele + 0, name_offset.clone()  + "~octets");

}

fn decompose_url__host__HostInternal(a_ele: usize, mut name_offset: String){

let mut enum_flag = read_unsigned_int(a_ele + 0, 1);
match enum_flag {
0 => {
name_offset += "~None";
println!("{}", name_offset);
}
1 => {
name_offset += "~Domain";
println!("{}", name_offset);
}
2 => {
name_offset += "~Ipv4";
decompose_std__net__Ipv4Addr(a_ele + 1, name_offset.clone());
}
3 => {
name_offset += "~Ipv6";
decompose_std__net__Ipv6Addr(a_ele + 1, name_offset.clone());
}
_ =>{
unreachable!()
}

}

}

fn decompose_u16(a_ele: usize, mut name_offset: String){
let a_ele_p = (a_ele as * const usize as usize + 0) as * const u16;
println!("{}~u16:{}", name_offset, unsafe{ *a_ele_p });

}

fn decompose_std__option__Option_st_u16_ed_(a_ele: usize, mut name_offset: String){

let mut enum_flag = read_unsigned_int(a_ele + 0, 2);
match enum_flag {
0 => {
name_offset += "~None";
println!("{}", name_offset);
}
1 => {
name_offset += "~Some";
decompose_u16(a_ele + 2, name_offset.clone());
}
_ =>{
unreachable!()
}

}

}

fn decompose_std__option__Option_st_u32_ed_(a_ele: usize, mut name_offset: String){

let mut enum_flag = read_unsigned_int(a_ele + 0, 4);
match enum_flag {
0 => {
name_offset += "~None";
println!("{}", name_offset);
}
1 => {
name_offset += "~Some";
decompose_u32(a_ele + 4, name_offset.clone());
}
_ =>{
unreachable!()
}

}

}

fn decompose_url__Url(a_ele: usize, mut name_offset: String){
decompose_String(a_ele + 0, name_offset.clone()  + "~serialization");
decompose_u32(a_ele + 24, name_offset.clone()  + "~scheme_end");
decompose_u32(a_ele + 28, name_offset.clone()  + "~username_end");
decompose_u32(a_ele + 32, name_offset.clone()  + "~host_start");
decompose_u32(a_ele + 36, name_offset.clone()  + "~host_end");
decompose_url__host__HostInternal(a_ele + 64, name_offset.clone()  + "~host");
decompose_std__option__Option_st_u16_ed_(a_ele + 60, name_offset.clone()  + "~port");
decompose_u32(a_ele + 40, name_offset.clone()  + "~path_start");
decompose_std__option__Option_st_u32_ed_(a_ele + 44, name_offset.clone()  + "~query_start");
decompose_std__option__Option_st_u32_ed_(a_ele + 52, name_offset.clone()  + "~fragment_start");

}

fn decompose_url__ParseError(a_ele: usize, mut name_offset: String){

let mut enum_flag = read_unsigned_int(a_ele + 0, 1);
match enum_flag {
0 => {
name_offset += "~EmptyHost";
println!("{}", name_offset);
}
1 => {
name_offset += "~IdnaError";
println!("{}", name_offset);
}
2 => {
name_offset += "~InvalidPort";
println!("{}", name_offset);
}
3 => {
name_offset += "~InvalidIpv4Address";
println!("{}", name_offset);
}
4 => {
name_offset += "~InvalidIpv6Address";
println!("{}", name_offset);
}
5 => {
name_offset += "~InvalidDomainCharacter";
println!("{}", name_offset);
}
6 => {
name_offset += "~RelativeUrlWithoutBase";
println!("{}", name_offset);
}
7 => {
name_offset += "~RelativeUrlWithCannotBeABaseBase";
println!("{}", name_offset);
}
8 => {
name_offset += "~SetHostOnCannotBeABaseUrl";
println!("{}", name_offset);
}
9 => {
name_offset += "~Overflow";
println!("{}", name_offset);
}
10 => {
name_offset += "~__FutureProof";
println!("{}", name_offset);
}
_ =>{
unreachable!()
}

}

}

fn decompose_std__result__Result_st_url__Url_url__ParseError_ed_(a_ele: usize, mut name_offset: String){

let mut enum_flag = read_unsigned_int(a_ele + 44, 4);
enum_flag = encode_niche(enum_flag, 2, 1, 1, 0);
match enum_flag {
0 => {
name_offset += "~Ok";
decompose_url__Url(a_ele + 0, name_offset.clone());
}
1 => {
name_offset += "~Err";
decompose_url__ParseError(a_ele + 0, name_offset.clone());
}
_ =>{
unreachable!()
}

}

}


fn _read_data()-> Vec<u8> {
    use std::env;
    use std::process::exit;
    let args:Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No crash filename provided");
        exit(-1);
    }
    use std::path::PathBuf;
    let crash_file_name = &args[1];
    let crash_path = PathBuf::from(crash_file_name);
    if !crash_path.is_file() {
        println!("Not a valid crash file");
        exit(-1);
    }
    use std::fs;
    let data =  fs::read(crash_path).unwrap();
    data
}

fn main() {
    let _content = _read_data();
    let data = &_content;
    println!("data = {:?}", data);
    println!("data len = {:?}", data.len());
    //actual body emit
    if data.len() < 2 {return;}
    let dynamic_length = (data.len() - 0) / 2;
    let _param0 = _to_str(data, 0 + 0 * dynamic_length, 0 + 1 * dynamic_length);
    let _param1 = _to_str(data, 0 + 1 * dynamic_length, data.len());
    test_function46(_param0 ,_param1);

    println!("No panic!");
}
