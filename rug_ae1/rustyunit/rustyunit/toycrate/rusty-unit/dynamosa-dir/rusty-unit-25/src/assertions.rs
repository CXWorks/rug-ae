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
fn rusty_test_5529() {
    rusty_monitor::set_test_id(5529);
    let mut i32_0: i32 = 14319i32;
    let mut i32_1: i32 = -648i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_1, b: i32_0};
    let mut i32_2: i32 = 1701i32;
    let mut i32_3: i32 = 16983i32;
    let mut i32_4: i32 = 2896i32;
    let mut i32_5: i32 = -823i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_5, i32_4);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_6: i32 = -6512i32;
    let mut i32_7: i32 = -8170i32;
    let mut i32_8: i32 = 11268i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_7, i32_6);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_9: i32 = 10386i32;
    let mut i32_10: i32 = -16537i32;
    let mut i32_11: i32 = 10471i32;
    let mut i32_12: i32 = -3394i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_9, b: i32_3};
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i32_13: i32 = -16664i32;
    let mut i32_14: i32 = -22916i32;
    let mut i32_15: i32 = -15946i32;
    let mut i32_16: i32 = -3606i32;
    let mut i32_17: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_8, i32_10);
    let mut i32_18: i32 = crate::hp::ParryHotter::accio(parryhotter_2_ref_0, i32_12, i32_14);
    crate::hp::ParryHotter::alohomora(i32_15, i32_17, i32_13, i32_2);
    crate::hp::ParryHotter::foo2(parryhotter_1_ref_0, i32_16, i32_11);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6483() {
    rusty_monitor::set_test_id(6483);
    let mut i32_0: i32 = 778i32;
    let mut i32_1: i32 = 9832i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut u16_0: u16 = 5501u16;
    let mut i32_2: i32 = 2895i32;
    let mut u64_0: u64 = 5321u64;
    let mut usize_0: usize = 1244usize;
    let mut i32_3: i32 = 2896i32;
    let mut i32_4: i32 = -823i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_4, i32_3);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i64_0: i64 = 3264i64;
    let mut i64_1: i64 = -1879i64;
    let mut u16_1: u16 = 7747u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut u16_2: u16 = 1398u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut i32_5: i32 = -15434i32;
    let mut i32_6: i32 = 3225i32;
    let mut i32_7: i32 = 6930i32;
    let mut i32_8: i32 = 10385i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_8, i32_7);
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i64_2: i64 = -1726i64;
    let mut i64_3: i64 = -13489i64;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_3};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut i32_9: i32 = 74i32;
    let mut i32_10: i32 = -1920i32;
    let mut i32_11: i32 = 20454i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_11, i32_10);
    let mut parryhotter_3_ref_0: &crate::hp::ParryHotter = &mut parryhotter_3;
    let mut u16_3: u16 = 9373u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut u16_4: u16 = 2235u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_3_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_3;
    let mut i32_12: i32 = 13386i32;
    let mut i32_13: i32 = -6455i32;
    let mut i32_14: i32 = 2367i32;
    let mut parryhotter_4: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_14, b: i32_13};
    let mut parryhotter_4_ref_0: &crate::hp::ParryHotter = &mut parryhotter_4;
    let mut u16_5: u16 = 6689u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    let mut u16_6: u16 = 385u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    let mut u64_1: u64 = 3014u64;
    let mut u64_2: u64 = 3472u64;
    let mut u64_3: u64 = 9290u64;
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_5_ref_0: &crate::hp::RomTiddle = &mut romtiddle_5;
    let mut i32_15: i32 = -3726i32;
    let mut i32_16: i32 = -1834i32;
    let mut i32_17: i32 = 1925i32;
    let mut parryhotter_5: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_15, b: i32_6};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_18: i32 = crate::hp::ParryHotter::accio(parryhotter_3_ref_0, i32_2, i32_17);
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    let mut i32_19: i32 = crate::hp::ParryHotter::accio(parryhotter_2_ref_0, i32_18, i32_5);
    crate::hp::RomTiddle::avada_kedavra(romtiddle_3_ref_0, string_1);
    crate::hp::ParryHotter::foo2(parryhotter_1_ref_0, i32_12, i32_16);
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_2);
    crate::hp::ParryHotter::foo2(parryhotter_0_ref_0, i32_19, i32_9);
    crate::hp::RomTiddle::foo3(romtiddle_0_ref_0, i64_0, i64_1);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_4_ref_0, usize_0, u64_2, u64_3);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7006() {
    rusty_monitor::set_test_id(7006);
    let mut i32_0: i32 = 778i32;
    let mut i32_1: i32 = 9832i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut i32_2: i32 = 2757i32;
    let mut i32_3: i32 = -823i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut u16_0: u16 = 7747u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut u16_1: u16 = 1398u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut i32_4: i32 = 6930i32;
    let mut i32_5: i32 = 10385i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_5, i32_4);
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_689() {
    rusty_monitor::set_test_id(689);
    let mut u16_0: u16 = 7532u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut u64_0: u64 = 6837u64;
    let mut u64_1: u64 = 260u64;
    let mut u16_1: u16 = 4472u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut u16_2: u16 = 1856u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_2;
    let mut i32_0: i32 = 16784i32;
    let mut i32_1: i32 = -5440i32;
    let mut i32_2: i32 = -453i32;
    let mut i32_3: i32 = -13767i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_4: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_1, i32_0);
    crate::hp::RomTiddle::avada_kedavra(romtiddle_2_ref_0, string_1);
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1757() {
    rusty_monitor::set_test_id(1757);
    let mut i32_0: i32 = -4585i32;
    let mut i32_1: i32 = -3505i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_1, b: i32_0};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u64_0: u64 = 5064u64;
    let mut u64_1: u64 = 4796u64;
    let mut usize_0: usize = 1330usize;
    let mut i32_2: i32 = -2045i32;
    let mut i32_3: i32 = -20474i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_4: i32 = -21114i32;
    let mut i32_5: i32 = -5243i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_5, i32_4);
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut u64_2: u64 = 9418u64;
    let mut u64_3: u64 = 9658u64;
    let mut i32_6: i32 = 18435i32;
    let mut i32_7: i32 = -2036i32;
    let mut i32_8: i32 = 18576i32;
    let mut i32_9: i32 = 6554i32;
    crate::hp::ParryHotter::alohomora(i32_9, i32_8, i32_7, i32_6);
    crate::hp::ParryHotter::another_number_fn(u64_3, u64_2);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_1_ref_0, usize_0, u64_1, u64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1212() {
    rusty_monitor::set_test_id(1212);
    let mut i32_0: i32 = -11160i32;
    let mut i32_1: i32 = -5601i32;
    let mut i32_2: i32 = 5232i32;
    let mut i32_3: i32 = 10175i32;
    let mut i32_4: i32 = -2035i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_4, b: i32_3};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u16_0: u16 = 3108u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut u16_1: u16 = 89u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut i32_5: i32 = -16537i32;
    let mut i32_6: i32 = 10471i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_2, b: i32_0};
    let mut i32_7: i32 = -3606i32;
    crate::hp::ParryHotter::alohomora(i32_7, i32_6, i32_5, i32_1);
    panic!("From RustyUnit with love");
}
}