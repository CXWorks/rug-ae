#![allow(dead_code, non_camel_case_types)]
use crate::as_bytes::Safe;
#[cfg(feature = "simd")]
macro_rules! decl_simd {
    ($($decl:item)*) => {
        $(#[derive(Clone, Copy, Debug)] #[repr(simd)] $decl)*
    };
}
#[cfg(not(feature = "simd"))]
macro_rules! decl_simd {
    ($($decl:item)*) => {
        $(#[derive(Clone, Copy, Debug)] #[repr(C)] $decl)*
    };
}
decl_simd! {
    pub struct Simd2 < T > (pub T, pub T); pub struct Simd4 < T > (pub T, pub T, pub T,
    pub T); pub struct Simd8 < T > (pub T, pub T, pub T, pub T, pub T, pub T, pub T, pub
    T); pub struct Simd16 < T > (pub T, pub T, pub T, pub T, pub T, pub T, pub T, pub T,
    pub T, pub T, pub T, pub T, pub T, pub T, pub T, pub T); pub struct Simd32 < T > (pub
    T, pub T, pub T, pub T, pub T, pub T, pub T, pub T, pub T, pub T, pub T, pub T, pub
    T, pub T, pub T, pub T, pub T, pub T, pub T, pub T, pub T, pub T, pub T, pub T, pub
    T, pub T, pub T, pub T, pub T, pub T, pub T, pub T);
}
pub type u64x2 = Simd2<u64>;
pub type u32x4 = Simd4<u32>;
pub type u64x4 = Simd4<u64>;
pub type u16x8 = Simd8<u16>;
pub type u32x8 = Simd8<u32>;
pub type u8x16 = Simd16<u8>;
pub type u16x16 = Simd16<u16>;
pub type u8x32 = Simd32<u8>;
impl<T> Simd4<T> {
    #[inline(always)]
    pub fn new(e0: T, e1: T, e2: T, e3: T) -> Simd4<T> {
        Simd4(e0, e1, e2, e3)
    }
}
unsafe impl<T: Safe> Safe for Simd2<T> {}
unsafe impl<T: Safe> Safe for Simd4<T> {}
unsafe impl<T: Safe> Safe for Simd8<T> {}
unsafe impl<T: Safe> Safe for Simd16<T> {}
unsafe impl<T: Safe> Safe for Simd32<T> {}
#[cfg(test)]
mod tests_llm_16_68_llm_16_68 {
    use crate::simd::simdty::Simd4;
    #[test]
    fn test_simd4_new() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let e0 = rug_fuzz_0;
        let e1 = rug_fuzz_1;
        let e2 = rug_fuzz_2;
        let e3 = rug_fuzz_3;
        let simd = Simd4::new(e0, e1, e2, e3);
        debug_assert_eq!(simd.0, e0);
        debug_assert_eq!(simd.1, e1);
        debug_assert_eq!(simd.2, e2);
        debug_assert_eq!(simd.3, e3);
             }
}
}
}    }
}
