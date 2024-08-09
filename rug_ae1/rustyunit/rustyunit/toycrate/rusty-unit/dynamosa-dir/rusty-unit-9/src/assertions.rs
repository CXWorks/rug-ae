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
fn rusty_test_629() {
    rusty_monitor::set_test_id(629);
    let mut i64_0: i64 = 8813i64;
    let mut u16_0: u16 = 6535u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut u16_1: u16 = 6809u16;
    let mut i32_0: i32 = -14457i32;
    let mut i32_1: i32 = 16882i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut u16_2: u16 = 3792u16;
    let mut i32_2: i32 = 12025i32;
    let mut i32_3: i32 = -10454i32;
    let mut i32_4: i32 = -14948i32;
    let mut i32_5: i32 = 1163i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_5, b: i32_4};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut u16_3: u16 = 6629u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_1_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_1;
    let mut u16_4: u16 = 9419u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut i32_6: i32 = -346i32;
    let mut i32_7: i32 = 5930i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_7, i32_6);
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i64_1: i64 = 6943i64;
    let mut u16_5: u16 = 7345u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut i32_8: i32 = 16686i32;
    let mut i32_9: i32 = 7394i32;
    let mut i32_10: i32 = 9875i32;
    let mut i32_11: i32 = -7443i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_11, b: i32_10};
    let mut parryhotter_3_ref_0: &crate::hp::ParryHotter = &mut parryhotter_3;
    let mut i32_12: i32 = 1531i32;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut i32_13: i32 = -2921i32;
    let mut i32_14: i32 = -10451i32;
    let mut i32_15: i32 = -6928i32;
    let mut parryhotter_4: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_12, b: i32_8};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i64_2: i64 = -1881i64;
    let mut u16_6: u16 = 5305u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut i64_3: i64 = 6236i64;
    let mut u16_7: u16 = 5168u16;
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    let mut romtiddle_5_ref_0: &crate::hp::RomTiddle = &mut romtiddle_5;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_3};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut i32_16: i32 = 1559i32;
    let mut i32_17: i32 = -1917i32;
    let mut i32_18: i32 = -5585i32;
    let mut parryhotter_5: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_17, b: i32_15};
    let mut parryhotter_4_ref_0: &crate::hp::ParryHotter = &mut parryhotter_4;
    let mut u16_8: u16 = 4858u16;
    let mut romtiddle_7: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_7};
    let mut romtiddle_6_ref_0: &crate::hp::RomTiddle = &mut romtiddle_6;
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut u16_9: u16 = 9805u16;
    let mut romtiddle_8: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_8};
    let mut romtiddle_7_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_7;
    crate::hp::RomTiddle::avada_kedavra(romtiddle_1_ref_0, string_3);
    let mut parryhotter_6: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_13};
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_1);
    let mut i32_19: i32 = crate::hp::ParryHotter::accio(parryhotter_3_ref_0, i32_16, i32_18);
    let mut romtiddle_9: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_9};
    crate::hp::ParryHotter::alohomora(i32_2, i32_19, i32_9, i32_14);
    let mut wonreasley_2: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_2};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_219() {
    rusty_monitor::set_test_id(219);
    let mut i64_0: i64 = -21806i64;
    let mut i64_1: i64 = -3840i64;
    let mut u16_0: u16 = 6344u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut i32_0: i32 = 8707i32;
    let mut i32_1: i32 = 3452i32;
    let mut i32_2: i32 = -9929i32;
    let mut i32_3: i32 = -1336i32;
    let mut u16_1: u16 = 6884u16;
    let mut i64_2: i64 = 9395i64;
    let mut i64_3: i64 = 1829i64;
    let mut u16_2: u16 = 2033u16;
    let mut i32_4: i32 = 2741i32;
    let mut i32_5: i32 = -7956i32;
    let mut i32_6: i32 = 496i32;
    let mut i32_7: i32 = -10581i32;
    let mut u16_3: u16 = 1181u16;
    let mut u64_0: u64 = 7286u64;
    let mut u64_1: u64 = 14u64;
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    crate::hp::ParryHotter::alohomora(i32_7, i32_6, i32_5, i32_4);
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    crate::hp::RomTiddle::foo3(romtiddle_2_ref_0, i64_3, i64_2);
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    crate::hp::ParryHotter::alohomora(i32_3, i32_2, i32_1, i32_0);
    crate::hp::RomTiddle::foo3(romtiddle_0_ref_0, i64_1, i64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1679() {
    rusty_monitor::set_test_id(1679);
    let mut i32_0: i32 = 4817i32;
    let mut i32_1: i32 = 9193i32;
    let mut i64_0: i64 = 24178i64;
    let mut u16_0: u16 = 3292u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut i32_2: i32 = -5215i32;
    let mut i32_3: i32 = -10603i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u16_1: u16 = 4384u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_1;
    let mut u16_2: u16 = 6244u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut u16_3: u16 = 1850u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut u16_4: u16 = 7350u16;
    let mut i32_4: i32 = 6827i32;
    let mut i32_5: i32 = -18012i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_5, i32_4);
    let mut i32_6: i32 = -14251i32;
    let mut i32_7: i32 = 9139i32;
    let mut i32_8: i32 = -26798i32;
    let mut i32_9: i32 = -12343i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_9, b: i32_8};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut u16_5: u16 = 7472u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut u16_6: u16 = 2347u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_4_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_4;
    let mut u16_7: u16 = 2797u16;
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    let mut romtiddle_7: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_7};
    let mut romtiddle_6_ref_0: &crate::hp::RomTiddle = &mut romtiddle_6;
    let mut u16_8: u16 = 4u16;
    let mut i32_10: i32 = -2106i32;
    let mut u16_9: u16 = 9321u16;
    let mut i32_11: i32 = -1252i32;
    let mut i32_12: i32 = -15293i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_11, i32_10);
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i64_1: i64 = 3729i64;
    let mut u16_10: u16 = 4635u16;
    let mut romtiddle_8: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_9};
    let mut romtiddle_5_ref_0: &crate::hp::RomTiddle = &mut romtiddle_5;
    let mut romtiddle_7_ref_0: &crate::hp::RomTiddle = &mut romtiddle_7;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_6_ref_0);
    let mut romtiddle_9: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_8};
    let mut romtiddle_8_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_8;
    crate::hp::RomTiddle::avada_kedavra(romtiddle_4_ref_0, string_2);
    let mut i32_13: i32 = crate::hp::ParryHotter::accio(parryhotter_2_ref_0, i32_7, i32_12);
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_1);
    crate::hp::RomTiddle::avada_kedavra(romtiddle_1_ref_0, string_1);
    let mut i32_14: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_13, i32_6);
    let mut romtiddle_10: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_10};
    let mut parryhotter_4: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7230() {
    rusty_monitor::set_test_id(7230);
    let mut i64_0: i64 = -14487i64;
    let mut u16_0: u16 = 0u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut u16_1: u16 = 8147u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut i32_0: i32 = -18992i32;
    let mut i32_1: i32 = -9463i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut str_0: &str = "XFKwzb9cCOQIyAdr";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i32_2: i32 = 4401i32;
    let mut i32_3: i32 = 8371i32;
    let mut i32_4: i32 = 7892i32;
    let mut i32_5: i32 = -10044i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_5, i32_4);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut u16_2: u16 = 7636u16;
    let mut u16_3: u16 = 8u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut u16_4: u16 = 5400u16;
    let mut str_1: &str = "ZM6F2yiOEhfP7L1qI";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut i64_1: i64 = -2455i64;
    let mut u16_5: u16 = 4735u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_1};
    let mut wonreasley_1_ref_0: &crate::hp::WonReasley = &mut wonreasley_1;
    let mut i32_6: i32 = -27538i32;
    let mut i32_7: i32 = 8635i32;
    let mut u16_6: u16 = 346u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_2;
    let mut i64_2: i64 = -2031i64;
    let mut u16_7: u16 = 179u16;
    let mut u64_0: u64 = 3933u64;
    let mut usize_0: usize = 5401usize;
    let mut i32_8: i32 = 17350i32;
    let mut i32_9: i32 = 1131i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_8);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_10: i32 = -7918i32;
    let mut u16_8: u16 = 8649u16;
    let mut u16_9: u16 = 3102u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    let mut u16_10: u16 = 54u16;
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_9};
    let mut romtiddle_5_ref_0: &crate::hp::RomTiddle = &mut romtiddle_5;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut u16_11: u16 = 3967u16;
    let mut romtiddle_7: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_6_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_6;
    let mut i64_3: i64 = -16115i64;
    let mut romtiddle_8: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_10};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_5_ref_0);
    let mut wonreasley_2: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_3};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut i32_11: i32 = -4892i32;
    let mut i32_12: i32 = -3749i32;
    let mut i32_13: i32 = 853i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_9, i32_12);
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i32_14: i32 = 7395i32;
    let mut i32_15: i32 = -5638i32;
    let mut i32_16: i32 = -2175i32;
    let mut parryhotter_4: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_13, i32_2);
    let mut parryhotter_3_ref_0: &crate::hp::ParryHotter = &mut parryhotter_3;
    crate::hp::ParryHotter::foo2(parryhotter_1_ref_0, i32_15, i32_7);
    let mut i32_17: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_14, i32_6);
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_2);
    crate::hp::RomTiddle::avada_kedavra(romtiddle_6_ref_0, string_3);
    let mut romtiddle_9: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_7};
    let mut romtiddle_7_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_7;
    let mut romtiddle_10: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_8};
    let mut u64_1: u64 = 684u64;
    let mut usize_1: usize = 1447usize;
    let mut i32_18: i32 = -346i32;
    let mut parryhotter_5: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_16, i32_10);
    let mut parryhotter_4_ref_0: &crate::hp::ParryHotter = &mut parryhotter_4;
    let mut romtiddle_11: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_11};
    let mut romtiddle_9_ref_0: &crate::hp::RomTiddle = &mut romtiddle_9;
    let mut string_4: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    let mut i32_19: i32 = 7394i32;
    let mut wonreasley_2_ref_0: &crate::hp::WonReasley = &mut wonreasley_2;
    let mut i32_20: i32 = 5438i32;
    let mut parryhotter_6: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_19, i32_18);
    let mut parryhotter_5_ref_0: &crate::hp::ParryHotter = &mut parryhotter_5;
    crate::hp::WonReasley::arania_exumai(wonreasley_2_ref_0, str_0_ref_0);
    let mut parryhotter_7: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_11, i32_20);
    let mut parryhotter_6_ref_0: &crate::hp::ParryHotter = &mut parryhotter_6;
    let mut i32_21: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_2_ref_0, usize_1, string_4);
    let mut romtiddle_10_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_10;
    let mut parryhotter_7_ref_0: &crate::hp::ParryHotter = &mut parryhotter_7;
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_3_ref_0, usize_0, u64_0, u64_1);
    panic!("From RustyUnit with love");
}
}