use crate::d2s;
pub const FLOAT_POW5_INV_BITCOUNT: i32 = d2s::DOUBLE_POW5_INV_BITCOUNT - 64;
pub const FLOAT_POW5_BITCOUNT: i32 = d2s::DOUBLE_POW5_BITCOUNT - 64;
#[cfg_attr(feature = "no-panic", inline)]
fn pow5factor_32(mut value: u32) -> u32 {
    let mut count = 0u32;
    loop {
        debug_assert!(value != 0);
        let q = value / 5;
        let r = value % 5;
        if r != 0 {
            break;
        }
        value = q;
        count += 1;
    }
    count
}
#[cfg_attr(feature = "no-panic", inline)]
pub fn multiple_of_power_of_5_32(value: u32, p: u32) -> bool {
    pow5factor_32(value) >= p
}
#[cfg_attr(feature = "no-panic", inline)]
pub fn multiple_of_power_of_2_32(value: u32, p: u32) -> bool {
    (value & ((1u32 << p) - 1)) == 0
}
#[cfg_attr(feature = "no-panic", inline)]
fn mul_shift_32(m: u32, factor: u64, shift: i32) -> u32 {
    debug_assert!(shift > 32);
    let factor_lo = factor as u32;
    let factor_hi = (factor >> 32) as u32;
    let bits0 = m as u64 * factor_lo as u64;
    let bits1 = m as u64 * factor_hi as u64;
    let sum = (bits0 >> 32) + bits1;
    let shifted_sum = sum >> (shift - 32);
    debug_assert!(shifted_sum <= u32::max_value() as u64);
    shifted_sum as u32
}
#[cfg_attr(feature = "no-panic", inline)]
pub fn mul_pow5_inv_div_pow2(m: u32, q: u32, j: i32) -> u32 {
    #[cfg(feature = "small")]
    {
        let pow5 = unsafe { d2s::compute_inv_pow5(q) };
        mul_shift_32(m, pow5.1 + 1, j)
    }
    #[cfg(not(feature = "small"))]
    {
        debug_assert!(q < d2s::DOUBLE_POW5_INV_SPLIT.len() as u32);
        unsafe {
            mul_shift_32(
                m,
                d2s::DOUBLE_POW5_INV_SPLIT.get_unchecked(q as usize).1 + 1,
                j,
            )
        }
    }
}
#[cfg_attr(feature = "no-panic", inline)]
pub fn mul_pow5_div_pow2(m: u32, i: u32, j: i32) -> u32 {
    #[cfg(feature = "small")]
    {
        let pow5 = unsafe { d2s::compute_pow5(i) };
        mul_shift_32(m, pow5.1, j)
    }
    #[cfg(not(feature = "small"))]
    {
        debug_assert!(i < d2s::DOUBLE_POW5_SPLIT.len() as u32);
        unsafe { mul_shift_32(m, d2s::DOUBLE_POW5_SPLIT.get_unchecked(i as usize).1, j) }
    }
}
#[cfg(test)]
mod tests_llm_16_29_llm_16_29 {
    use super::*;
    use crate::*;
    #[test]
    fn test_mul_pow5_div_pow2_small() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u32, u32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let m: u32 = rug_fuzz_0;
        let i: u32 = rug_fuzz_1;
        let j: i32 = rug_fuzz_2;
        let result = mul_pow5_div_pow2(m, i, j);
        #[cfg(feature = "small")]
        {
            let pow5 = unsafe { super::d2s::compute_pow5(i) };
            let expected = super::mul_shift_32(m, pow5.1, j);
            debug_assert_eq!(result, expected);
        }
        #[cfg(not(feature = "small"))]
        {
            let pow5_split = super::d2s::DOUBLE_POW5_SPLIT;
            let expected = unsafe {
                super::mul_shift_32(m, pow5_split.get_unchecked(i as usize).1, j)
            };
            debug_assert_eq!(result, expected);
        }
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_30 {
    use crate::f2s_intrinsics::mul_pow5_inv_div_pow2;
    use crate::d2s;
    #[test]
    fn test_mul_pow5_inv_div_pow2() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u32, u32, i32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let test_cases = [(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3)];
        for &(m, q, j, expected) in &test_cases {
            let result = mul_pow5_inv_div_pow2(m, q, j);
            debug_assert_eq!(
                result, expected, "Failed for mul_pow5_inv_div_pow2({}, {}, {})", m, q, j
            );
        }
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_32 {
    use super::*;
    use crate::*;
    #[test]
    fn test_multiple_of_power_of_2_32() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            f2s_intrinsics::multiple_of_power_of_2_32(rug_fuzz_0, rug_fuzz_1), true
        );
        debug_assert_eq!(
            f2s_intrinsics::multiple_of_power_of_2_32(rug_fuzz_2, rug_fuzz_3), false
        );
        debug_assert_eq!(
            f2s_intrinsics::multiple_of_power_of_2_32(rug_fuzz_4, rug_fuzz_5), true
        );
        debug_assert_eq!(
            f2s_intrinsics::multiple_of_power_of_2_32(rug_fuzz_6, rug_fuzz_7), true
        );
        debug_assert_eq!(
            f2s_intrinsics::multiple_of_power_of_2_32(rug_fuzz_8, rug_fuzz_9), false
        );
        debug_assert_eq!(
            f2s_intrinsics::multiple_of_power_of_2_32(rug_fuzz_10, rug_fuzz_11), true
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_14 {
    use super::*;
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: u32 = rug_fuzz_0;
        let result = crate::f2s_intrinsics::pow5factor_32(p0);
        debug_assert_eq!(result, 3);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_15 {
    use super::*;
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: u32 = rug_fuzz_0;
        let mut p1: u32 = rug_fuzz_1;
        debug_assert!(crate ::f2s_intrinsics::multiple_of_power_of_5_32(p0, p1));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_16 {
    use super::*;
    #[test]
    fn test_mul_shift_32() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u32, u64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: u32 = rug_fuzz_0;
        let p1: u64 = rug_fuzz_1;
        let p2: i32 = rug_fuzz_2;
        let result = crate::f2s_intrinsics::mul_shift_32(p0, p1, p2);
        debug_assert_eq!(result, (p0 as u64 * p1 >> p2) as u32);
             }
}
}
}    }
}
