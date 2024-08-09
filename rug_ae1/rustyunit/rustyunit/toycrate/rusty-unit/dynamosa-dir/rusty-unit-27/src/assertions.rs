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
fn rusty_test_44() {
    rusty_monitor::set_test_id(44);
    let mut i32_0: i32 = -15755i32;
    let mut i32_1: i32 = 6239i32;
    let mut i32_2: i32 = 334i32;
    let mut i32_3: i32 = -18494i32;
    let mut u16_0: u16 = 1577u16;
    let mut i64_0: i64 = -3210i64;
    let mut u16_1: u16 = 539u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut i32_4: i32 = 2608i32;
    let mut i32_5: i32 = -1470i32;
    let mut i32_6: i32 = 12705i32;
    let mut i32_7: i32 = 9190i32;
    let mut i64_1: i64 = -9957i64;
    let mut i64_2: i64 = 5065i64;
    let mut u16_2: u16 = 3013u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_2};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut u16_3: u16 = 1964u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_1);
    crate::hp::ParryHotter::alohomora(i32_7, i32_6, i32_5, i32_4);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    crate::hp::ParryHotter::alohomora(i32_3, i32_2, i32_1, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_668() {
    rusty_monitor::set_test_id(668);
    let mut u64_0: u64 = 9578u64;
    let mut u64_1: u64 = 102u64;
    let mut u16_0: u16 = 1338u16;
    let mut u64_2: u64 = 5056u64;
    let mut u64_3: u64 = 7999u64;
    let mut usize_0: usize = 3789usize;
    let mut i32_0: i32 = 869i32;
    let mut i32_1: i32 = 9161i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_1, b: i32_0};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u16_1: u16 = 7020u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut usize_1: usize = 7068usize;
    let mut i32_2: i32 = -14057i32;
    let mut i32_3: i32 = -8449i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut u64_4: u64 = 9718u64;
    let mut u64_5: u64 = 6990u64;
    let mut i64_0: i64 = -513i64;
    let mut i64_1: i64 = -3172i64;
    let mut u16_2: u16 = 1557u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    crate::hp::RomTiddle::foo3(romtiddle_1_ref_0, i64_1, i64_0);
    crate::hp::ParryHotter::another_number_fn(u64_5, u64_4);
    let mut i32_4: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_1_ref_0, usize_1, string_0);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_0_ref_0, usize_0, u64_3, u64_2);
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    panic!("From RustyUnit with love");
}
}