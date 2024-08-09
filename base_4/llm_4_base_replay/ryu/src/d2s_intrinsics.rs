use core::ptr;
#[cfg_attr(feature = "no-panic", inline)]
pub fn div5(x: u64) -> u64 {
    x / 5
}
#[cfg_attr(feature = "no-panic", inline)]
pub fn div10(x: u64) -> u64 {
    x / 10
}
#[cfg_attr(feature = "no-panic", inline)]
pub fn div100(x: u64) -> u64 {
    x / 100
}
#[cfg_attr(feature = "no-panic", inline)]
fn pow5_factor(mut value: u64) -> u32 {
    let mut count = 0u32;
    loop {
        debug_assert!(value != 0);
        let q = div5(value);
        let r = (value as u32).wrapping_sub(5u32.wrapping_mul(q as u32));
        if r != 0 {
            break;
        }
        value = q;
        count += 1;
    }
    count
}
#[cfg_attr(feature = "no-panic", inline)]
pub fn multiple_of_power_of_5(value: u64, p: u32) -> bool {
    pow5_factor(value) >= p
}
#[cfg_attr(feature = "no-panic", inline)]
pub fn multiple_of_power_of_2(value: u64, p: u32) -> bool {
    debug_assert!(value != 0);
    debug_assert!(p < 64);
    (value & ((1u64 << p) - 1)) == 0
}
#[cfg_attr(feature = "no-panic", inline)]
pub fn mul_shift_64(m: u64, mul: &(u64, u64), j: u32) -> u64 {
    let b0 = m as u128 * mul.0 as u128;
    let b2 = m as u128 * mul.1 as u128;
    (((b0 >> 64) + b2) >> (j - 64)) as u64
}
#[cfg_attr(feature = "no-panic", inline)]
pub unsafe fn mul_shift_all_64(
    m: u64,
    mul: &(u64, u64),
    j: u32,
    vp: *mut u64,
    vm: *mut u64,
    mm_shift: u32,
) -> u64 {
    ptr::write(vp, mul_shift_64(4 * m + 2, mul, j));
    ptr::write(vm, mul_shift_64(4 * m - 1 - mm_shift as u64, mul, j));
    mul_shift_64(4 * m, mul, j)
}
#[cfg(test)]
mod tests_llm_16_25_llm_16_25 {
    use crate::d2s_intrinsics::multiple_of_power_of_2;
    #[test]
    fn test_multiple_of_power_of_2() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17, mut rug_fuzz_18)) = <(u64, u32, u64, u32, u64, u32, u64, u32, u64, u32, u64, u32, u64, i32, u32, u64, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(multiple_of_power_of_2(rug_fuzz_0, rug_fuzz_1));
        debug_assert!(multiple_of_power_of_2(rug_fuzz_2, rug_fuzz_3));
        debug_assert!(multiple_of_power_of_2(rug_fuzz_4, rug_fuzz_5));
        debug_assert!(! multiple_of_power_of_2(rug_fuzz_6, rug_fuzz_7));
        debug_assert!(! multiple_of_power_of_2(rug_fuzz_8, rug_fuzz_9));
        debug_assert!(! multiple_of_power_of_2(rug_fuzz_10, rug_fuzz_11));
        debug_assert!(multiple_of_power_of_2(rug_fuzz_12 << rug_fuzz_13, rug_fuzz_14));
        debug_assert!(
            multiple_of_power_of_2(u64::MAX - (u64::MAX % (rug_fuzz_15 << rug_fuzz_16)),
            rug_fuzz_17)
        );
        debug_assert!(! multiple_of_power_of_2(u64::MAX, rug_fuzz_18));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_26_llm_16_26 {
    use super::*;
    use crate::*;
    #[test]
    fn test_multiple_of_power_of_5() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(u64, u32, u64, u32, u64, u32, u64, u32, u64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(multiple_of_power_of_5(rug_fuzz_0, rug_fuzz_1));
        debug_assert!(! multiple_of_power_of_5(rug_fuzz_2, rug_fuzz_3));
        debug_assert!(multiple_of_power_of_5(rug_fuzz_4, rug_fuzz_5));
        debug_assert!(! multiple_of_power_of_5(rug_fuzz_6, rug_fuzz_7));
        debug_assert!(multiple_of_power_of_5(rug_fuzz_8, rug_fuzz_9));
             }
}
}
}    }
}
