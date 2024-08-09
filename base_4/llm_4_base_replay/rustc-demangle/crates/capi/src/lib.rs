extern crate rustc_demangle;
use std::io::Write;
use std::os::raw::{c_char, c_int};
/// C-style interface for demangling.
/// Demangles symbol given in `mangled` argument into `out` buffer
///
/// Unsafe as it handles buffers by raw pointers.
///
/// Returns 0 if `mangled` is not Rust symbol or if `out` buffer is too small
/// Returns 1 otherwise
#[no_mangle]
pub unsafe extern "C" fn rustc_demangle(
    mangled: *const c_char,
    out: *mut c_char,
    out_size: usize,
) -> c_int {
    let mangled_str = match std::ffi::CStr::from_ptr(mangled).to_str() {
        Ok(s) => s,
        Err(_) => return 0,
    };
    match rustc_demangle::try_demangle(mangled_str) {
        Ok(demangle) => {
            let mut out_slice = std::slice::from_raw_parts_mut(out as *mut u8, out_size);
            match write!(out_slice, "{:#}\0", demangle) {
                Ok(_) => return 1,
                Err(_) => return 0,
            }
        }
        Err(_) => return 0,
    }
}
#[cfg(test)]
mod tests {
    use std;
    use std::os::raw::c_char;
    #[test]
    fn demangle_c_str_large() {
        let mangled = "_ZN4testE\0";
        let mut out_buf: Vec<u8> = vec![42; 8];
        let res = unsafe {
            super::rustc_demangle(
                mangled.as_ptr() as *const c_char,
                out_buf.as_mut_ptr() as *mut c_char,
                8,
            )
        };
        assert_eq!(res, 1);
        let out_str = std::str::from_utf8(&out_buf[..5]).unwrap();
        assert_eq!(out_str, "test\0");
    }
    #[test]
    fn demangle_c_str_exact() {
        let mangled = "_ZN4testE\0";
        let mut out_buf: Vec<u8> = vec![42; 8];
        let res = unsafe {
            super::rustc_demangle(
                mangled.as_ptr() as *const c_char,
                out_buf.as_mut_ptr() as *mut c_char,
                5,
            )
        };
        assert_eq!(res, 1);
        let out_str = std::str::from_utf8(&out_buf).unwrap();
        assert_eq!(out_str, "test\0***");
    }
    #[test]
    fn demangle_c_str_small() {
        let mangled = "_ZN4testE\0";
        let mut out_buf: Vec<u8> = vec![42; 8];
        let res = unsafe {
            super::rustc_demangle(
                mangled.as_ptr() as *const c_char,
                out_buf.as_mut_ptr() as *mut c_char,
                4,
            )
        };
        assert_eq!(res, 0);
        let out_str = std::str::from_utf8(&out_buf[4..]).unwrap();
        assert_eq!(out_str, "****");
    }
    #[test]
    fn demangle_c_str_smaller() {
        let mangled = "_ZN4testE\0";
        let mut out_buf: Vec<u8> = vec![42; 8];
        let res = unsafe {
            super::rustc_demangle(
                mangled.as_ptr() as *const c_char,
                out_buf.as_mut_ptr() as *mut c_char,
                3,
            )
        };
        assert_eq!(res, 0);
        let out_str = std::str::from_utf8(&out_buf[3..]).unwrap();
        assert_eq!(out_str, "*****");
    }
    #[test]
    fn demangle_c_str_zero() {
        let mangled = "_ZN4testE\0";
        let mut out_buf: Vec<u8> = vec![42; 8];
        let res = unsafe {
            super::rustc_demangle(
                mangled.as_ptr() as *const c_char,
                out_buf.as_mut_ptr() as *mut c_char,
                0,
            )
        };
        assert_eq!(res, 0);
        let out_str = std::str::from_utf8(&out_buf).unwrap();
        assert_eq!(out_str, "********");
    }
    #[test]
    fn demangle_c_str_not_rust_symbol() {
        let mangled = "la la la\0";
        let mut out_buf: Vec<u8> = vec![42; 8];
        let res = unsafe {
            super::rustc_demangle(
                mangled.as_ptr() as *const c_char,
                out_buf.as_mut_ptr() as *mut c_char,
                8,
            )
        };
        assert_eq!(res, 0);
    }
    #[test]
    fn demangle_c_str_null() {
        let mangled = "\0";
        let mut out_buf: Vec<u8> = vec![42; 8];
        let res = unsafe {
            super::rustc_demangle(
                mangled.as_ptr() as *const c_char,
                out_buf.as_mut_ptr() as *mut c_char,
                8,
            )
        };
        assert_eq!(res, 0);
    }
    #[test]
    fn demangle_c_str_invalid_utf8() {
        let mangled = [116, 101, 115, 116, 165, 0];
        let mut out_buf: Vec<u8> = vec![42; 8];
        let res = unsafe {
            super::rustc_demangle(
                mangled.as_ptr() as *const c_char,
                out_buf.as_mut_ptr() as *mut c_char,
                8,
            )
        };
        assert_eq!(res, 0);
    }
}
#[cfg(test)]
mod tests_llm_16_1 {
    use crate::rustc_demangle;
    use std::ffi::CString;
    use std::os::raw::{c_char, c_int};
    #[test]
    fn demangle_rust_symbol() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mangled = CString::new(rug_fuzz_0).expect(rug_fuzz_1);
        let mangled_ptr: *const c_char = mangled.as_ptr();
        let mut out = vec![0u8; 128];
        let out_ptr = out.as_mut_ptr() as *mut c_char;
        unsafe {
            let result = rustc_demangle(mangled_ptr, out_ptr, out.len());
            debug_assert_eq!(result, 1);
            let demangled_str = std::ffi::CStr::from_ptr(out_ptr).to_str().unwrap();
            debug_assert_eq!(demangled_str, "foo::bar\0");
        }
             }
}
}
}    }
    #[test]
    fn non_rust_symbol() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let non_rust = CString::new(rug_fuzz_0).expect(rug_fuzz_1);
        let non_rust_ptr: *const c_char = non_rust.as_ptr();
        let mut out = vec![0u8; 128];
        let out_ptr = out.as_mut_ptr() as *mut c_char;
        unsafe {
            let result = rustc_demangle(non_rust_ptr, out_ptr, out.len());
            debug_assert_eq!(result, 0);
        }
             }
}
}
}    }
    #[test]
    fn buffer_too_small() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mangled = CString::new(rug_fuzz_0).expect(rug_fuzz_1);
        let mangled_ptr: *const c_char = mangled.as_ptr();
        let mut out = vec![0u8; 4];
        let out_ptr = out.as_mut_ptr() as *mut c_char;
        unsafe {
            let result = rustc_demangle(mangled_ptr, out_ptr, out.len());
            debug_assert_eq!(result, 0);
        }
             }
}
}
}    }
}
