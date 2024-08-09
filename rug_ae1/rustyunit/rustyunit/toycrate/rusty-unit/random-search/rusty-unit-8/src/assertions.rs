fn foo(x: &[i32], y: usize) -> i32 {
    x[y]
}


#[cfg(test)]
mod rusty_tests {
	use crate::*;

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2607() {
    rusty_monitor::set_test_id(2607);
    let mut u64_0: u64 = 2308u64;
    let mut u64_1: u64 = 5686u64;
    let mut usize_0: usize = 97usize;
    let mut u64_2: u64 = 3609u64;
    let mut u64_3: u64 = 4499u64;
    let mut usize_1: usize = 5736usize;
    let mut i32_0: i32 = -12539i32;
    let mut i32_1: i32 = 10667i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_1, b: i32_0};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_2: i32 = 6259i32;
    let mut i32_3: i32 = 15064i32;
    let mut i32_4: i32 = 894i32;
    let mut i32_5: i32 = 2444i32;
    let mut i32_6: i32 = 4614i32;
    let mut i32_7: i32 = 4771i32;
    let mut str_0: &str = "n8IOakW";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_0: i64 = -8859i64;
    let mut u16_0: u16 = 2413u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_7, i32_6);
    crate::hp::ParryHotter::alohomora(i32_5, i32_4, i32_3, i32_2);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_0_ref_0, usize_1, u64_3, u64_2);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut bool_1: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_1_ref_0, usize_0, u64_1, u64_0);
    panic!("From RustyUnit with love");
}
}