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
        let _rug_st_tests_llm_16_25_llm_16_25_rrrruuuugggg_test_multiple_of_power_of_2 = 0;
        let rug_fuzz_0 = 8;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 64;
        let rug_fuzz_3 = 6;
        let rug_fuzz_4 = 1024;
        let rug_fuzz_5 = 10;
        let rug_fuzz_6 = 7;
        let rug_fuzz_7 = 3;
        let rug_fuzz_8 = 63;
        let rug_fuzz_9 = 6;
        let rug_fuzz_10 = 1023;
        let rug_fuzz_11 = 10;
        let rug_fuzz_12 = 1u64;
        let rug_fuzz_13 = 63;
        let rug_fuzz_14 = 63;
        let rug_fuzz_15 = 1u64;
        let rug_fuzz_16 = 12;
        let rug_fuzz_17 = 12;
        let rug_fuzz_18 = 12;
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
        let _rug_ed_tests_llm_16_25_llm_16_25_rrrruuuugggg_test_multiple_of_power_of_2 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_26_llm_16_26 {
    use super::*;
    use crate::*;
    #[test]
    fn test_multiple_of_power_of_5() {
        let _rug_st_tests_llm_16_26_llm_16_26_rrrruuuugggg_test_multiple_of_power_of_5 = 0;
        let rug_fuzz_0 = 25;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 24;
        let rug_fuzz_3 = 2;
        let rug_fuzz_4 = 3125;
        let rug_fuzz_5 = 5;
        let rug_fuzz_6 = 3120;
        let rug_fuzz_7 = 5;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 0;
        debug_assert!(multiple_of_power_of_5(rug_fuzz_0, rug_fuzz_1));
        debug_assert!(! multiple_of_power_of_5(rug_fuzz_2, rug_fuzz_3));
        debug_assert!(multiple_of_power_of_5(rug_fuzz_4, rug_fuzz_5));
        debug_assert!(! multiple_of_power_of_5(rug_fuzz_6, rug_fuzz_7));
        debug_assert!(multiple_of_power_of_5(rug_fuzz_8, rug_fuzz_9));
        let _rug_ed_tests_llm_16_26_llm_16_26_rrrruuuugggg_test_multiple_of_power_of_5 = 0;
    }
}
