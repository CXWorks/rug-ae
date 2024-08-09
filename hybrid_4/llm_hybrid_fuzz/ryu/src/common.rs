#[cfg_attr(feature = "no-panic", inline)]
pub fn decimal_length9(v: u32) -> u32 {
    debug_assert!(v < 1000000000);
    if v >= 100000000 {
        9
    } else if v >= 10000000 {
        8
    } else if v >= 1000000 {
        7
    } else if v >= 100000 {
        6
    } else if v >= 10000 {
        5
    } else if v >= 1000 {
        4
    } else if v >= 100 {
        3
    } else if v >= 10 {
        2
    } else {
        1
    }
}
#[cfg_attr(feature = "no-panic", inline)]
#[allow(dead_code)]
pub fn log2_pow5(e: i32) -> i32 {
    debug_assert!(e >= 0);
    debug_assert!(e <= 3528);
    ((e as u32 * 1217359) >> 19) as i32
}
#[cfg_attr(feature = "no-panic", inline)]
pub fn pow5bits(e: i32) -> i32 {
    debug_assert!(e >= 0);
    debug_assert!(e <= 3528);
    (((e as u32 * 1217359) >> 19) + 1) as i32
}
#[cfg_attr(feature = "no-panic", inline)]
#[allow(dead_code)]
pub fn ceil_log2_pow5(e: i32) -> i32 {
    log2_pow5(e) + 1
}
#[cfg_attr(feature = "no-panic", inline)]
pub fn log10_pow2(e: i32) -> u32 {
    debug_assert!(e >= 0);
    debug_assert!(e <= 1650);
    (e as u32 * 78913) >> 18
}
#[cfg_attr(feature = "no-panic", inline)]
pub fn log10_pow5(e: i32) -> u32 {
    debug_assert!(e >= 0);
    debug_assert!(e <= 2620);
    (e as u32 * 732923) >> 20
}
#[cfg(test)]
mod tests_llm_16_12 {
    use super::*;
    use crate::*;
    #[test]
    fn test_ceil_log2_pow5() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(ceil_log2_pow5(rug_fuzz_0), 1);
        debug_assert_eq!(ceil_log2_pow5(rug_fuzz_1), 3);
        debug_assert_eq!(ceil_log2_pow5(rug_fuzz_2), 5);
        debug_assert_eq!(ceil_log2_pow5(rug_fuzz_3), 8);
             }
});    }
}
#[cfg(test)]
mod tests_rug_1 {
    use super::*;
    #[test]
    fn test_decimal_length9() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10)) = <(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: u32 = rug_fuzz_0;
        debug_assert_eq!(crate ::common::decimal_length9(p0), 9);
        p0 = rug_fuzz_1;
        debug_assert_eq!(crate ::common::decimal_length9(p0), 9);
        p0 = rug_fuzz_2;
        debug_assert_eq!(crate ::common::decimal_length9(p0), 8);
        p0 = rug_fuzz_3;
        debug_assert_eq!(crate ::common::decimal_length9(p0), 8);
        p0 = rug_fuzz_4;
        debug_assert_eq!(crate ::common::decimal_length9(p0), 7);
        p0 = rug_fuzz_5;
        debug_assert_eq!(crate ::common::decimal_length9(p0), 7);
        p0 = rug_fuzz_6;
        debug_assert_eq!(crate ::common::decimal_length9(p0), 6);
        p0 = rug_fuzz_7;
        debug_assert_eq!(crate ::common::decimal_length9(p0), 6);
        p0 = rug_fuzz_8;
        debug_assert_eq!(crate ::common::decimal_length9(p0), 5);
        p0 = rug_fuzz_9;
        debug_assert_eq!(crate ::common::decimal_length9(p0), 5);
        p0 = rug_fuzz_10;
        debug_assert_eq!(crate ::common::decimal_length9(p0), 4);
             }
});    }
}
#[cfg(test)]
mod tests_rug_2 {
    use super::*;
    #[test]
    fn test_log2_pow5() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: i32 = rug_fuzz_0;
        debug_assert_eq!(
            crate ::common::log2_pow5(p0), ((p0 as u32 * 1217359) >> 19) as i32
        );
             }
});    }
    #[test]
    #[should_panic]
    fn test_log2_pow5_overflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: i32 = rug_fuzz_0;
        crate::common::log2_pow5(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_3 {
    use super::*;
    #[test]
    fn test_pow5bits() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: i32 = rug_fuzz_0;
        debug_assert_eq!(crate ::common::pow5bits(p0), 5);
             }
});    }
}
#[cfg(test)]
mod tests_rug_4 {
    use super::*;
    #[test]
    fn test_log10_pow2() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: i32 = rug_fuzz_0;
        debug_assert_eq!(crate ::common::log10_pow2(p0), 308);
             }
});    }
}
#[cfg(test)]
mod tests_rug_5 {
    use crate::common::log10_pow5;
    #[test]
    fn test_log10_pow5() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: i32 = rug_fuzz_0;
        let result = log10_pow5(p0);
        debug_assert_eq!(result, (p0 as u32 * 732923) >> 20);
             }
});    }
}
