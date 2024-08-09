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
fn rusty_test_2638() {
    rusty_monitor::set_test_id(2638);
    let mut i32_0: i32 = -9016i32;
    let mut i32_1: i32 = -1801i32;
    let mut i32_2: i32 = -6115i32;
    let mut i32_3: i32 = 14800i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u16_0: u16 = 5699u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut u16_1: u16 = 2583u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut usize_0: usize = 7055usize;
    let mut i32_4: i32 = -3614i32;
    let mut i32_5: i32 = 10953i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_5, i32_4);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut u16_2: u16 = 556u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut usize_1: usize = 6066usize;
    let mut i32_6: i32 = 1637i32;
    let mut i32_7: i32 = 7965i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_7, b: i32_6};
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i32_8: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_2_ref_0, usize_1, string_1);
    let mut i32_9: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_1_ref_0, usize_0, string_0);
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    crate::hp::ParryHotter::foo2(parryhotter_0_ref_0, i32_1, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2395() {
    rusty_monitor::set_test_id(2395);
    let mut i32_0: i32 = 103i32;
    let mut i32_1: i32 = -2325i32;
    let mut u16_0: u16 = 3093u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut u16_1: u16 = 3404u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_1;
    let mut i32_2: i32 = -5519i32;
    let mut i32_3: i32 = 18996i32;
    let mut i32_4: i32 = 1896i32;
    let mut i32_5: i32 = 714i32;
    let mut i32_6: i32 = -883i32;
    let mut i32_7: i32 = -323i32;
    let mut i32_8: i32 = -11145i32;
    let mut i32_9: i32 = 7280i32;
    let mut str_0: &str = "614UN";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_0: i64 = 18500i64;
    let mut u16_2: u16 = 5523u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_0};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
    crate::hp::ParryHotter::alohomora(i32_9, i32_8, i32_7, i32_6);
    crate::hp::ParryHotter::alohomora(i32_5, i32_4, i32_3, i32_2);
    crate::hp::RomTiddle::avada_kedavra(romtiddle_1_ref_0, string_0);
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_1, b: i32_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1422() {
    rusty_monitor::set_test_id(1422);
    let mut u16_0: u16 = 6755u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut u64_0: u64 = 8542u64;
    let mut u64_1: u64 = 7450u64;
    let mut u64_2: u64 = 1694u64;
    let mut u64_3: u64 = 915u64;
    let mut usize_0: usize = 4756usize;
    let mut i32_0: i32 = -5437i32;
    let mut i32_1: i32 = -23106i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_2: i32 = 16968i32;
    let mut i32_3: i32 = 2773i32;
    let mut i32_4: i32 = 7169i32;
    let mut i32_5: i32 = 7842i32;
    let mut i64_0: i64 = 9851i64;
    let mut i64_1: i64 = -2141i64;
    let mut u16_1: u16 = 8472u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut i32_6: i32 = -5879i32;
    let mut i32_7: i32 = 20221i32;
    let mut i32_8: i32 = 22150i32;
    let mut i32_9: i32 = 9075i32;
    crate::hp::ParryHotter::alohomora(i32_9, i32_8, i32_7, i32_6);
    crate::hp::RomTiddle::foo3(romtiddle_1_ref_0, i64_1, i64_0);
    crate::hp::ParryHotter::alohomora(i32_5, i32_4, i32_3, i32_2);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_0_ref_0, usize_0, u64_3, u64_2);
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    panic!("From RustyUnit with love");
}
}