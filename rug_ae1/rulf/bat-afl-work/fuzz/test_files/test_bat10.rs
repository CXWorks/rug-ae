#[macro_use]
extern crate afl;
extern crate bat;
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

fn _to_slice<T>(data:&[u8], start_index: usize, end_index: usize)->&[T] {
    let data_slice = &data[start_index..end_index];
    let (_, shorts, _) = unsafe {data_slice.align_to::<T>()};
    shorts
}


fn test_function10(_param0 :&[u8] ,_param1 :bool) {
    let mut _local0 = bat::PrettyPrinter::new();
    let _local1 = bat::PrettyPrinter::input_from_bytes(&mut (_local0) ,_param0);
    let _ = bat::PrettyPrinter::vcs_modification_markers(&mut (_local0) ,_param1);
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() < 2 {return;}
        let dynamic_length = (data.len() - 1) / 1;
        let _param0 = _to_slice::<u8>(data, 1 + 0 * dynamic_length, data.len());
        let _param1 = _to_bool(data, 0);
        test_function10(_param0 ,_param1);
    });
}
