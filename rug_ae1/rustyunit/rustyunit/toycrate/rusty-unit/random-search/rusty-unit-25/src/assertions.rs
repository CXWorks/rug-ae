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
fn rusty_test_3568() {
    rusty_monitor::set_test_id(3568);
    let mut u16_0: u16 = 7762u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut usize_0: usize = 8612usize;
    let mut i32_0: i32 = -8896i32;
    let mut i32_1: i32 = -4684i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_2: i32 = 7496i32;
    let mut i32_3: i32 = 10743i32;
    let mut i32_4: i32 = -3235i32;
    let mut i32_5: i32 = 4067i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_5, b: i32_4};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_6: i32 = -2789i32;
    let mut i32_7: i32 = -2481i32;
    let mut i32_8: i32 = -31176i32;
    let mut i32_9: i32 = 8701i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_9, b: i32_8};
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i32_10: i32 = -8675i32;
    let mut i32_11: i32 = 2170i32;
    let mut i32_12: i32 = 6773i32;
    let mut i32_13: i32 = -405i32;
    let mut u16_1: u16 = 5226u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    crate::hp::ParryHotter::alohomora(i32_13, i32_12, i32_11, i32_10);
    crate::hp::ParryHotter::foo2(parryhotter_2_ref_0, i32_7, i32_6);
    crate::hp::ParryHotter::foo2(parryhotter_1_ref_0, i32_3, i32_2);
    let mut i32_14: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_0_ref_0, usize_0, string_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3461() {
    rusty_monitor::set_test_id(3461);
    let mut i32_0: i32 = 7831i32;
    let mut i32_1: i32 = 7050i32;
    let mut i32_2: i32 = 16583i32;
    let mut i32_3: i32 = 3149i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i64_0: i64 = -15004i64;
    let mut i64_1: i64 = -7337i64;
    let mut u16_0: u16 = 8922u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_1};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut i64_2: i64 = -5316i64;
    let mut i64_3: i64 = 12311i64;
    let mut u16_1: u16 = 9303u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut u16_2: u16 = 3054u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut usize_0: usize = 5313usize;
    let mut i32_4: i32 = 17667i32;
    let mut i32_5: i32 = 15468i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_5, i32_4);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_6: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_1_ref_0, usize_0, string_1);
    crate::hp::RomTiddle::foo3(romtiddle_1_ref_0, i64_3, i64_2);
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_0);
    let mut i32_7: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_1, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1653() {
    rusty_monitor::set_test_id(1653);
    let mut i32_0: i32 = -387i32;
    let mut i32_1: i32 = 8320i32;
    let mut u64_0: u64 = 8051u64;
    let mut u64_1: u64 = 3648u64;
    let mut usize_0: usize = 687usize;
    let mut i32_2: i32 = 1570i32;
    let mut i32_3: i32 = 1853i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_4: i32 = 1126i32;
    let mut i32_5: i32 = 11644i32;
    let mut i32_6: i32 = -641i32;
    let mut i32_7: i32 = 4630i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_7, b: i32_6};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_8: i32 = 2489i32;
    let mut i32_9: i32 = -5650i32;
    let mut i32_10: i32 = -6038i32;
    let mut i32_11: i32 = 15187i32;
    let mut i32_12: i32 = -523i32;
    let mut i32_13: i32 = 9700i32;
    let mut i32_14: i32 = -22161i32;
    let mut i32_15: i32 = 7681i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_15, b: i32_14};
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    crate::hp::ParryHotter::foo2(parryhotter_2_ref_0, i32_13, i32_12);
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_11, b: i32_10};
    let mut parryhotter_4: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_9, i32_8);
    let mut i32_16: i32 = crate::hp::ParryHotter::accio(parryhotter_1_ref_0, i32_5, i32_4);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_0_ref_0, usize_0, u64_1, u64_0);
    let mut parryhotter_4_ref_0: &crate::hp::ParryHotter = &mut parryhotter_4;
    crate::hp::ParryHotter::foo2(parryhotter_4_ref_0, i32_1, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1448() {
    rusty_monitor::set_test_id(1448);
    let mut i32_0: i32 = -442i32;
    let mut i32_1: i32 = 5739i32;
    let mut i32_2: i32 = -6939i32;
    let mut i32_3: i32 = -339i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u64_0: u64 = 3879u64;
    let mut u64_1: u64 = 7671u64;
    let mut usize_0: usize = 8578usize;
    let mut i32_4: i32 = 8451i32;
    let mut i32_5: i32 = 10420i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_5, b: i32_4};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut u16_0: u16 = 6916u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut str_0: &str = "VXMy";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_0: i64 = -785i64;
    let mut u16_1: u16 = 2235u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    let mut i32_6: i32 = 18160i32;
    let mut i32_7: i32 = 5095i32;
    let mut i32_8: i32 = -475i32;
    let mut i32_9: i32 = 10713i32;
    let mut u64_2: u64 = 6408u64;
    let mut u64_3: u64 = 3939u64;
    let mut u16_2: u16 = 8701u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut usize_1: usize = 4423usize;
    let mut i32_10: i32 = -19067i32;
    let mut i32_11: i32 = -23053i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_11, b: i32_10};
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut u16_3: u16 = 6701u16;
    let mut i32_12: i32 = 9055i32;
    let mut i32_13: i32 = 5912i32;
    let mut i32_14: i32 = -16430i32;
    let mut i32_15: i32 = 2041i32;
    let mut i32_16: i32 = -3827i32;
    let mut i32_17: i32 = 3260i32;
    let mut i32_18: i32 = -361i32;
    let mut i32_19: i32 = 511i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_19, i32_18);
    let mut parryhotter_3_ref_0: &crate::hp::ParryHotter = &mut parryhotter_3;
    let mut i32_20: i32 = -357i32;
    let mut i32_21: i32 = 19878i32;
    let mut i32_22: i32 = -1533i32;
    let mut i32_23: i32 = -7201i32;
    let mut u16_4: u16 = 7395u16;
    let mut i64_1: i64 = 392i64;
    let mut i64_2: i64 = 14455i64;
    let mut u16_5: u16 = 0u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_2};
    let mut wonreasley_1_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_1;
    crate::hp::WonReasley::ascendio(wonreasley_1_ref_0, i64_1);
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    crate::hp::ParryHotter::alohomora(i32_23, i32_22, i32_21, i32_20);
    let mut i32_24: i32 = crate::hp::ParryHotter::accio(parryhotter_3_ref_0, i32_17, i32_16);
    crate::hp::ParryHotter::alohomora(i32_15, i32_14, i32_13, i32_12);
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut i32_25: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_2_ref_0, usize_1, string_1);
    crate::hp::ParryHotter::another_number_fn(u64_3, u64_2);
    crate::hp::ParryHotter::alohomora(i32_9, i32_8, i32_7, i32_6);
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_1_ref_0, usize_0, u64_1, u64_0);
    crate::hp::ParryHotter::foo2(parryhotter_0_ref_0, i32_1, i32_0);
    panic!("From RustyUnit with love");
}
}