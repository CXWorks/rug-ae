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
fn rusty_test_6791() {
    rusty_monitor::set_test_id(6791);
    let mut i32_0: i32 = -4902i32;
    let mut i32_1: i32 = 1442i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut u16_0: u16 = 9189u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut i32_2: i32 = 16547i32;
    let mut i32_3: i32 = -2545i32;
    let mut i32_4: i32 = -4804i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_4, b: i32_3};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_5: i32 = 11026i32;
    let mut i32_6: i32 = 5151i32;
    let mut i32_7: i32 = -2034i32;
    let mut i32_8: i32 = -494i32;
    let mut i32_9: i32 = -14125i32;
    let mut i32_10: i32 = -12525i32;
    let mut str_0: &str = "r1i6DJ3d0CyYmO";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_0: i64 = -18291i64;
    let mut u16_1: u16 = 8431u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    let mut u16_2: u16 = 375u16;
    let mut i32_11: i32 = -1121i32;
    let mut i32_12: i32 = 19167i32;
    let mut i32_13: i32 = -27230i32;
    let mut i32_14: i32 = 9205i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_14, i32_13);
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut u16_3: u16 = 476u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut u16_4: u16 = 5197u16;
    let mut u64_0: u64 = 3174u64;
    let mut usize_0: usize = 2321usize;
    let mut i32_15: i32 = -7163i32;
    let mut i32_16: i32 = 6500i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_16, b: i32_15};
    let mut parryhotter_3_ref_0: &crate::hp::ParryHotter = &mut parryhotter_3;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    let mut i64_1: i64 = 16046i64;
    let mut u16_5: u16 = 6593u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_1};
    let mut wonreasley_1_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_1;
    let mut i32_17: i32 = -11920i32;
    let mut i32_18: i32 = 1603i32;
    let mut i32_19: i32 = 7530i32;
    let mut i32_20: i32 = -5238i32;
    let mut parryhotter_4: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_20, i32_19);
    let mut parryhotter_4_ref_0: &crate::hp::ParryHotter = &mut parryhotter_4;
    let mut i32_21: i32 = crate::hp::ParryHotter::accio(parryhotter_4_ref_0, i32_18, i32_17);
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut i32_22: i32 = 5939i32;
    let mut i32_23: i32 = 12868i32;
    let mut parryhotter_5: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_10, i32_12);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u64_1: u64 = 7474u64;
    let mut parryhotter_6: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_2, i32_23);
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    let mut i32_24: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_0_ref_0, usize_0, string_1);
    let mut parryhotter_5_ref_0: &crate::hp::ParryHotter = &mut parryhotter_5;
    crate::hp::ParryHotter::foo2(parryhotter_1_ref_0, i32_7, i32_6);
    let mut i32_25: i32 = crate::hp::ParryHotter::accio(parryhotter_2_ref_0, i32_9, i32_22);
    let mut parryhotter_6_ref_0: &crate::hp::ParryHotter = &mut parryhotter_6;
    crate::hp::ParryHotter::alohomora(i32_11, i32_21, i32_5, i32_8);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4423() {
    rusty_monitor::set_test_id(4423);
    let mut u16_0: u16 = 5404u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut u16_1: u16 = 9216u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_1;
    let mut u64_0: u64 = 4342u64;
    let mut u64_1: u64 = 3413u64;
    let mut str_0: &str = "WbHZm";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_0: i64 = -5361i64;
    let mut u16_2: u16 = 9743u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_0};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    crate::hp::RomTiddle::avada_kedavra(romtiddle_1_ref_0, string_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_287() {
    rusty_monitor::set_test_id(287);
    let mut i32_0: i32 = -4699i32;
    let mut i32_1: i32 = -5910i32;
    let mut i32_2: i32 = 21160i32;
    let mut i32_3: i32 = -5492i32;
    let mut i32_4: i32 = 2622i32;
    let mut i32_5: i32 = 8469i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_5, b: i32_4};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i64_0: i64 = 6514i64;
    let mut i64_1: i64 = 9417i64;
    let mut u16_0: u16 = 5702u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_1};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_0);
    let mut i32_6: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_3, i32_2);
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_1, b: i32_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8143() {
    rusty_monitor::set_test_id(8143);
    let mut i32_0: i32 = -6554i32;
    let mut i32_1: i32 = -7866i32;
    let mut i32_2: i32 = -14849i32;
    let mut i32_3: i32 = 15252i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u16_0: u16 = 1927u16;
    let mut u16_1: u16 = 1553u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut u16_2: u16 = 1262u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut i64_0: i64 = 1541i64;
    let mut u16_3: u16 = 6228u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut i32_4: i32 = 3326i32;
    let mut i32_5: i32 = -136i32;
    let mut i32_6: i32 = -13195i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_6, i32_5);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_7: i32 = -2571i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_4, i32_7);
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    crate::hp::ParryHotter::foo2(parryhotter_0_ref_0, i32_1, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2190() {
    rusty_monitor::set_test_id(2190);
    let mut i32_0: i32 = 10521i32;
    let mut i32_1: i32 = 11554i32;
    let mut i32_2: i32 = 665i32;
    let mut i32_3: i32 = -10942i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u16_0: u16 = 5015u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut i64_0: i64 = -11204i64;
    let mut u16_1: u16 = 6915u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut u16_2: u16 = 5066u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut u16_3: u16 = 9635u16;
    let mut i32_4: i32 = -68i32;
    let mut i32_5: i32 = 20662i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_5, b: i32_4};
    let mut i32_6: i32 = -5264i32;
    let mut i32_7: i32 = 5746i32;
    let mut i64_1: i64 = 1541i64;
    let mut i64_2: i64 = 10987i64;
    let mut u16_4: u16 = 6228u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_2};
    let mut wonreasley_1_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_1;
    let mut i32_8: i32 = 2986i32;
    let mut i32_9: i32 = -136i32;
    let mut i32_10: i32 = -13195i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_10, i32_9);
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i32_11: i32 = -14055i32;
    let mut i32_12: i32 = -4065i32;
    let mut u16_5: u16 = 1712u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut usize_0: usize = 8742usize;
    let mut i32_13: i32 = 4063i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_11, b: i32_13};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_14: i32 = -9372i32;
    let mut i32_15: i32 = -3786i32;
    let mut i32_16: i32 = 12913i32;
    let mut u16_6: u16 = 1375u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut i32_17: i32 = -19273i32;
    let mut i32_18: i32 = 14635i32;
    let mut parryhotter_4: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_14, i32_17);
    let mut parryhotter_3_ref_0: &crate::hp::ParryHotter = &mut parryhotter_3;
    let mut i64_3: i64 = -11347i64;
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    let mut romtiddle_5_ref_0: &crate::hp::RomTiddle = &mut romtiddle_5;
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    let mut wonreasley_2: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_1};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    crate::hp::WonReasley::ascendio(wonreasley_1_ref_0, i64_3);
    crate::hp::ParryHotter::foo2(parryhotter_2_ref_0, i32_8, i32_6);
    let mut parryhotter_5: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_18, i32_15);
    let mut i32_19: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_3_ref_0, usize_0, string_3);
    let mut parryhotter_5_ref_0: &crate::hp::ParryHotter = &mut parryhotter_5;
    let mut i32_20: i32 = crate::hp::ParryHotter::accio(parryhotter_1_ref_0, i32_16, i32_12);
    let mut romtiddle_6_ref_0: &crate::hp::RomTiddle = &mut romtiddle_6;
    let mut string_4: std::string::String = crate::hp::RomTiddle::name(romtiddle_5_ref_0);
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut i32_21: i32 = crate::hp::ParryHotter::accio(parryhotter_5_ref_0, i32_20, i32_7);
    crate::hp::ParryHotter::foo2(parryhotter_0_ref_0, i32_1, i32_0);
    panic!("From RustyUnit with love");
}
}