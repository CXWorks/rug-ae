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
fn rusty_test_1173() {
    rusty_monitor::set_test_id(1173);
    let mut i32_0: i32 = -5900i32;
    let mut i32_1: i32 = 6217i32;
    let mut u64_0: u64 = 3994u64;
    let mut u64_1: u64 = 6975u64;
    let mut i32_2: i32 = -5593i32;
    let mut i32_3: i32 = -3894i32;
    let mut i32_4: i32 = -2551i32;
    let mut i32_5: i32 = -9817i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_5, i32_4);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u16_0: u16 = 6476u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut u64_2: u64 = 3028u64;
    let mut u64_3: u64 = 7445u64;
    let mut usize_0: usize = 3136usize;
    let mut i32_6: i32 = -15686i32;
    let mut i32_7: i32 = 4826i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_7, i32_6);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_8: i32 = 4033i32;
    let mut i32_9: i32 = 8296i32;
    let mut i32_10: i32 = -10536i32;
    let mut i32_11: i32 = 5963i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_11, i32_10);
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut u16_1: u16 = 2803u16;
    let mut i32_12: i32 = 9310i32;
    let mut i32_13: i32 = -1857i32;
    let mut i32_14: i32 = 5357i32;
    let mut i32_15: i32 = -13191i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_15, i32_14);
    let mut parryhotter_3_ref_0: &crate::hp::ParryHotter = &mut parryhotter_3;
    let mut i32_16: i32 = crate::hp::ParryHotter::accio(parryhotter_3_ref_0, i32_13, i32_12);
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut i32_17: i32 = crate::hp::ParryHotter::accio(parryhotter_2_ref_0, i32_9, i32_8);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_1_ref_0, usize_0, u64_3, u64_2);
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    crate::hp::ParryHotter::foo2(parryhotter_0_ref_0, i32_3, i32_2);
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    let mut parryhotter_4: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_1, b: i32_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1184() {
    rusty_monitor::set_test_id(1184);
    let mut i32_0: i32 = 220i32;
    let mut i32_1: i32 = 15903i32;
    let mut i32_2: i32 = 4136i32;
    let mut i32_3: i32 = 3244i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u16_0: u16 = 5912u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut u16_1: u16 = 9484u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut u16_2: u16 = 291u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_2;
    let mut str_0: &str = "r5Za2ZHbNbVMq";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_0: i64 = -16507i64;
    let mut u16_3: u16 = 1994u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
    let mut i32_4: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_1, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1186() {
    rusty_monitor::set_test_id(1186);
    let mut u16_0: u16 = 8348u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut u16_1: u16 = 7728u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_1;
    let mut i64_0: i64 = -18055i64;
    let mut i64_1: i64 = 8274i64;
    let mut u16_2: u16 = 2107u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_1};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut i64_2: i64 = 12066i64;
    let mut u16_3: u16 = 3597u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut u16_4: u16 = 3557u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    let mut u16_5: u16 = 2054u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_5_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_5;
    let mut u64_0: u64 = 1465u64;
    let mut u64_1: u64 = 1330u64;
    let mut usize_0: usize = 1979usize;
    let mut i64_3: i64 = 12190i64;
    let mut i64_4: i64 = -1037i64;
    let mut u16_6: u16 = 233u16;
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    let mut romtiddle_6_ref_0: &crate::hp::RomTiddle = &mut romtiddle_6;
    let mut string_4: std::string::String = crate::hp::RomTiddle::name(romtiddle_6_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_4, y: i64_4};
    let mut wonreasley_1_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_1;
    let mut i32_0: i32 = -783i32;
    let mut i32_1: i32 = 14922i32;
    let mut i64_5: i64 = 10112i64;
    let mut i64_6: i64 = -17427i64;
    let mut u16_7: u16 = 4267u16;
    let mut romtiddle_7: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_7};
    let mut romtiddle_7_ref_0: &crate::hp::RomTiddle = &mut romtiddle_7;
    let mut u64_2: u64 = 1793u64;
    let mut u64_3: u64 = 1317u64;
    let mut i32_2: i32 = -7694i32;
    let mut i32_3: i32 = -560i32;
    let mut i32_4: i32 = 38i32;
    let mut i32_5: i32 = 9293i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_5, i32_4);
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    crate::hp::ParryHotter::another_number_fn(u64_3, u64_2);
    crate::hp::RomTiddle::foo3(romtiddle_7_ref_0, i64_6, i64_5);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    crate::hp::ParryHotter::foo2(parryhotter_1_ref_0, i32_1, i32_0);
    crate::hp::WonReasley::ascendio(wonreasley_1_ref_0, i64_3);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_0_ref_0, usize_0, u64_1, u64_0);
    crate::hp::RomTiddle::avada_kedavra(romtiddle_5_ref_0, string_3);
    let mut wonreasley_2: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_2};
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_0);
    crate::hp::RomTiddle::avada_kedavra(romtiddle_1_ref_0, string_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1187() {
    rusty_monitor::set_test_id(1187);
    let mut u16_0: u16 = 4530u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut u16_1: u16 = 1417u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_1;
    let mut u16_2: u16 = 7071u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut u16_3: u16 = 9119u16;
    let mut i32_0: i32 = 337i32;
    let mut i32_1: i32 = -6391i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut u64_0: u64 = 1330u64;
    let mut i64_0: i64 = -1037i64;
    let mut u16_4: u16 = 233u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_0};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut i32_2: i32 = -783i32;
    let mut i32_3: i32 = 14922i32;
    let mut u16_5: u16 = 4267u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut i32_4: i32 = 12106i32;
    let mut i32_5: i32 = -11788i32;
    let mut i32_6: i32 = -11180i32;
    let mut i32_7: i32 = -11333i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_4, b: i32_5};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i64_1: i64 = 13536i64;
    let mut u16_6: u16 = 7164u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    let mut u16_7: u16 = 6476u16;
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    let mut romtiddle_5_ref_0: &crate::hp::RomTiddle = &mut romtiddle_5;
    let mut u64_1: u64 = 7445u64;
    let mut usize_0: usize = 3136usize;
    let mut i32_8: i32 = -15686i32;
    let mut i32_9: i32 = 4826i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_2, i32_8);
    let mut i32_10: i32 = 8296i32;
    let mut i32_11: i32 = 5963i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_6, i32_10);
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i32_12: i32 = -8628i32;
    let mut i32_13: i32 = -3530i32;
    let mut i32_14: i32 = 9310i32;
    let mut i32_15: i32 = -1857i32;
    let mut i32_16: i32 = 5357i32;
    let mut parryhotter_4: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_11, i32_14);
    let mut parryhotter_3_ref_0: &crate::hp::ParryHotter = &mut parryhotter_3;
    let mut i32_17: i32 = crate::hp::ParryHotter::accio(parryhotter_2_ref_0, i32_3, i32_13);
    let mut romtiddle_7: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_7};
    crate::hp::ParryHotter::alohomora(i32_16, i32_17, i32_9, i32_12);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_0_ref_0, usize_0, u64_1, u64_0);
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_1};
    let mut i32_18: i32 = crate::hp::ParryHotter::accio(parryhotter_3_ref_0, i32_15, i32_7);
    crate::hp::RomTiddle::avada_kedavra(romtiddle_1_ref_0, string_0);
    let mut string_4: std::string::String = crate::hp::RomTiddle::name(romtiddle_5_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1198() {
    rusty_monitor::set_test_id(1198);
    let mut u16_0: u16 = 6426u16;
    let mut u16_1: u16 = 1591u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut u16_2: u16 = 5451u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_1_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_1;
    let mut i32_0: i32 = 5733i32;
    let mut i32_1: i32 = -8913i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut str_0: &str = "jVbXZ8bVVV5";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_0: i64 = 10716i64;
    let mut u16_3: u16 = 7892u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_0};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    let mut i32_2: i32 = 1183i32;
    let mut i32_3: i32 = 11929i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_4: i32 = 3145i32;
    let mut i32_5: i32 = 2779i32;
    let mut u64_0: u64 = 6858u64;
    let mut u64_1: u64 = 1285u64;
    let mut usize_0: usize = 6891usize;
    let mut i32_6: i32 = 13690i32;
    let mut i32_7: i32 = 7119i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_7, b: i32_6};
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i32_8: i32 = -9878i32;
    let mut i32_9: i32 = -7322i32;
    let mut i32_10: i32 = 7120i32;
    let mut i32_11: i32 = 9087i32;
    let mut u16_4: u16 = 2309u16;
    let mut i64_1: i64 = 21266i64;
    let mut i64_2: i64 = -5460i64;
    let mut u16_5: u16 = 4135u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_2};
    let mut wonreasley_1_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_1;
    crate::hp::WonReasley::ascendio(wonreasley_1_ref_0, i64_1);
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_11, i32_10);
    let mut parryhotter_3_ref_0: &crate::hp::ParryHotter = &mut parryhotter_3;
    let mut i32_12: i32 = crate::hp::ParryHotter::accio(parryhotter_3_ref_0, i32_9, i32_8);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_2_ref_0, usize_0, u64_1, u64_0);
    let mut i32_13: i32 = crate::hp::ParryHotter::accio(parryhotter_1_ref_0, i32_4, i32_5);
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
    crate::hp::RomTiddle::avada_kedavra(romtiddle_1_ref_0, string_0);
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1207() {
    rusty_monitor::set_test_id(1207);
    let mut i32_0: i32 = -4921i32;
    let mut i32_1: i32 = 1588i32;
    let mut i32_2: i32 = -3807i32;
    let mut i32_3: i32 = 2026i32;
    let mut u16_0: u16 = 8803u16;
    let mut u16_1: u16 = 7634u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut u64_0: u64 = 8123u64;
    let mut u64_1: u64 = 1186u64;
    let mut i32_4: i32 = 12106i32;
    let mut i32_5: i32 = -11788i32;
    let mut i32_6: i32 = -11180i32;
    let mut i32_7: i32 = -11333i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_7, b: i32_6};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i64_0: i64 = 13536i64;
    let mut u16_2: u16 = 7164u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut u16_3: u16 = 6476u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut u64_2: u64 = 3028u64;
    let mut u64_3: u64 = 7445u64;
    let mut usize_0: usize = 3136usize;
    let mut i32_8: i32 = -15686i32;
    let mut i32_9: i32 = 4826i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_9, i32_8);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_10: i32 = 4033i32;
    let mut i32_11: i32 = 8296i32;
    let mut i32_12: i32 = -10536i32;
    let mut i32_13: i32 = 5963i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_13, i32_12);
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i32_14: i32 = -3676i32;
    let mut i32_15: i32 = -5256i32;
    let mut i32_16: i32 = -8628i32;
    let mut i32_17: i32 = -3530i32;
    let mut u16_4: u16 = 2803u16;
    let mut i32_18: i32 = 9310i32;
    let mut i32_19: i32 = -1857i32;
    let mut i32_20: i32 = 5357i32;
    let mut i32_21: i32 = -13191i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_21, i32_20);
    let mut parryhotter_3_ref_0: &crate::hp::ParryHotter = &mut parryhotter_3;
    let mut i32_22: i32 = crate::hp::ParryHotter::accio(parryhotter_3_ref_0, i32_19, i32_18);
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    crate::hp::ParryHotter::alohomora(i32_17, i32_16, i32_15, i32_14);
    let mut i32_23: i32 = crate::hp::ParryHotter::accio(parryhotter_2_ref_0, i32_11, i32_10);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_1_ref_0, usize_0, u64_3, u64_2);
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut i32_24: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_5, i32_4);
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    crate::hp::ParryHotter::alohomora(i32_3, i32_2, i32_1, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1224() {
    rusty_monitor::set_test_id(1224);
    let mut u16_0: u16 = 7716u16;
    let mut i32_0: i32 = -2098i32;
    let mut i32_1: i32 = 4650i32;
    let mut u16_1: u16 = 3058u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u16_2: u16 = 6358u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut u16_3: u16 = 8741u16;
    let mut i32_2: i32 = -866i32;
    let mut i32_3: i32 = 4179i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    let mut i64_0: i64 = -11804i64;
    let mut u16_4: u16 = 1954u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut i64_1: i64 = 10119i64;
    let mut u16_5: u16 = 7827u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut u16_6: u16 = 6476u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut u64_0: u64 = 3028u64;
    let mut u64_1: u64 = 7445u64;
    let mut usize_0: usize = 3136usize;
    let mut i32_4: i32 = -15686i32;
    let mut i32_5: i32 = 4826i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_5, i32_4);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_6: i32 = 8296i32;
    let mut i32_7: i32 = -10536i32;
    let mut i32_8: i32 = 5963i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_8, i32_7);
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i32_9: i32 = -3676i32;
    let mut i32_10: i32 = -5256i32;
    let mut i32_11: i32 = -8628i32;
    let mut i32_12: i32 = -3530i32;
    let mut u16_7: u16 = 2803u16;
    let mut i32_13: i32 = 9310i32;
    let mut i32_14: i32 = -1857i32;
    let mut i32_15: i32 = 5357i32;
    let mut i32_16: i32 = -13191i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_16, i32_15);
    let mut parryhotter_3_ref_0: &crate::hp::ParryHotter = &mut parryhotter_3;
    let mut i32_17: i32 = crate::hp::ParryHotter::accio(parryhotter_3_ref_0, i32_14, i32_13);
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_7};
    crate::hp::ParryHotter::alohomora(i32_12, i32_11, i32_10, i32_9);
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_1};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    let mut i32_18: i32 = -3679i32;
    let mut i32_19: i32 = 11522i32;
    let mut i32_20: i32 = -2239i32;
    let mut parryhotter_4: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_17, b: i32_6};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_5_ref_0: &crate::hp::RomTiddle = &mut romtiddle_5;
    let mut i32_21: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_20, i32_19);
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    let mut parryhotter_5: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_21, i32_18);
    let mut i32_22: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_2_ref_0, usize_0, string_2);
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut parryhotter_5_ref_0: &crate::hp::ParryHotter = &mut parryhotter_5;
    let mut i32_23: i32 = crate::hp::ParryHotter::accio(parryhotter_5_ref_0, i32_1, i32_0);
    let mut romtiddle_7: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1229() {
    rusty_monitor::set_test_id(1229);
    let mut u16_0: u16 = 7634u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut u64_0: u64 = 8123u64;
    let mut u64_1: u64 = 1186u64;
    let mut i32_0: i32 = 12106i32;
    let mut i32_1: i32 = -11788i32;
    let mut i32_2: i32 = -11180i32;
    let mut i32_3: i32 = -11333i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i64_0: i64 = 13536i64;
    let mut u16_1: u16 = 7164u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut u16_2: u16 = 6476u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut u64_2: u64 = 3028u64;
    let mut u64_3: u64 = 7445u64;
    let mut usize_0: usize = 3136usize;
    let mut i32_4: i32 = -15686i32;
    let mut i32_5: i32 = 4826i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_5, i32_4);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_6: i32 = 4033i32;
    let mut i32_7: i32 = 8296i32;
    let mut i32_8: i32 = -10536i32;
    let mut i32_9: i32 = 5963i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_9, i32_8);
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i32_10: i32 = -3676i32;
    let mut i32_11: i32 = -5256i32;
    let mut i32_12: i32 = -8628i32;
    let mut i32_13: i32 = -3530i32;
    let mut u16_3: u16 = 2803u16;
    let mut i32_14: i32 = 9310i32;
    let mut i32_15: i32 = -1857i32;
    let mut i32_16: i32 = 5357i32;
    let mut i32_17: i32 = -13191i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_17, i32_16);
    let mut parryhotter_3_ref_0: &crate::hp::ParryHotter = &mut parryhotter_3;
    let mut i32_18: i32 = crate::hp::ParryHotter::accio(parryhotter_3_ref_0, i32_15, i32_14);
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    crate::hp::ParryHotter::alohomora(i32_13, i32_12, i32_11, i32_10);
    let mut i32_19: i32 = crate::hp::ParryHotter::accio(parryhotter_2_ref_0, i32_7, i32_6);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_1_ref_0, usize_0, u64_3, u64_2);
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut i32_20: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_1, i32_0);
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1232() {
    rusty_monitor::set_test_id(1232);
    let mut i32_0: i32 = -12027i32;
    let mut i32_1: i32 = -9567i32;
    let mut i32_2: i32 = -8078i32;
    let mut i32_3: i32 = -19065i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut str_0: &str = "wxSP";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i32_4: i32 = -1594i32;
    let mut i32_5: i32 = 20309i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_5, b: i32_4};
    let mut str_1: &str = "vJI2wf51Wz7FXyGiVKS";
    let mut i64_0: i64 = 7806i64;
    let mut u16_0: u16 = 1415u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut u16_1: u16 = 3822u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut u16_2: u16 = 1182u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut u16_3: u16 = 534u16;
    let mut i64_1: i64 = -359i64;
    let mut u16_4: u16 = 7712u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_1};
    let mut u16_5: u16 = 6476u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut u64_0: u64 = 3028u64;
    let mut u64_1: u64 = 7445u64;
    let mut i32_6: i32 = -15686i32;
    let mut i32_7: i32 = 4826i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_7, i32_6);
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i32_8: i32 = 4033i32;
    let mut i32_9: i32 = 8296i32;
    let mut wonreasley_1_ref_0: &crate::hp::WonReasley = &mut wonreasley_1;
    let mut u16_6: u16 = 6771u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut i64_2: i64 = -1556i64;
    let mut i64_3: i64 = 5230i64;
    let mut u16_7: u16 = 2884u16;
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    let mut romtiddle_5_ref_0: &crate::hp::RomTiddle = &mut romtiddle_5;
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    let mut wonreasley_2: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_2};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut str_1_ref_0: &str = &mut str_1;
    let mut i64_4: i64 = -8617i64;
    let mut u16_8: u16 = 654u16;
    let mut romtiddle_7: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_7};
    let mut romtiddle_6_ref_0: &crate::hp::RomTiddle = &mut romtiddle_6;
    let mut string_4: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut wonreasley_3: crate::hp::WonReasley = crate::hp::WonReasley {x: string_3, y: i64_4};
    let mut wonreasley_2_ref_0: &crate::hp::WonReasley = &mut wonreasley_2;
    let mut usize_0: usize = 2355usize;
    let mut i32_10: i32 = 12789i32;
    let mut i32_11: i32 = 3916i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_10, i32_9);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut u16_9: u16 = 5592u16;
    let mut romtiddle_8: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_9};
    let mut parryhotter_4: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_8, i32_11);
    let mut romtiddle_9: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_8};
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_1_ref_0, usize_0, u64_1, u64_0);
    crate::hp::WonReasley::arania_exumai(wonreasley_2_ref_0, str_1_ref_0);
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_3);
    crate::hp::WonReasley::arania_exumai(wonreasley_1_ref_0, str_0_ref_0);
    crate::hp::ParryHotter::foo2(parryhotter_0_ref_0, i32_1, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1237() {
    rusty_monitor::set_test_id(1237);
    let mut u16_0: u16 = 6808u16;
    let mut u16_1: u16 = 6508u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut u16_2: u16 = 5820u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut str_0: &str = "HBNUJxnd";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_0: i64 = -3467i64;
    let mut u16_3: u16 = 1649u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    let mut i32_0: i32 = -14232i32;
    let mut i32_1: i32 = 814i32;
    let mut i32_2: i32 = -7245i32;
    let mut i32_3: i32 = -9927i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i64_1: i64 = 12925i64;
    let mut i64_2: i64 = 1686i64;
    let mut u16_4: u16 = 1563u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_2};
    let mut wonreasley_1_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_1;
    crate::hp::WonReasley::ascendio(wonreasley_1_ref_0, i64_1);
    let mut i32_4: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_1, i32_0);
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1241() {
    rusty_monitor::set_test_id(1241);
    let mut u16_0: u16 = 1509u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut u64_0: u64 = 9306u64;
    let mut u64_1: u64 = 69u64;
    let mut usize_0: usize = 1650usize;
    let mut i32_0: i32 = 474i32;
    let mut i32_1: i32 = -4457i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u16_1: u16 = 4977u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut i32_2: i32 = -17336i32;
    let mut i32_3: i32 = 6701i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut str_0: &str = "BJSPsw";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_0: i64 = -10056i64;
    let mut u16_2: u16 = 2191u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    let mut i32_4: i32 = 11522i32;
    let mut i32_5: i32 = -2239i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_5, b: i32_4};
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_0_ref_0, usize_0, u64_1, u64_0);
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1245() {
    rusty_monitor::set_test_id(1245);
    let mut str_0: &str = "9cgq";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "tGHRN1";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut i64_0: i64 = -12766i64;
    let mut u16_0: u16 = 4024u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    let mut u16_1: u16 = 6552u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut u16_2: u16 = 7166u16;
    let mut i32_0: i32 = -15743i32;
    let mut i32_1: i32 = 10572i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut u16_3: u16 = 5975u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut i32_2: i32 = 13686i32;
    let mut i32_3: i32 = 10440i32;
    let mut i32_4: i32 = -2974i32;
    let mut i32_5: i32 = 7665i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_5, b: i32_4};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut u16_4: u16 = 2090u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut u16_5: u16 = 2459u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_4_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_4;
    let mut u16_6: u16 = 731u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    let mut romtiddle_5_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_5;
    let mut i64_1: i64 = -3531i64;
    let mut i64_2: i64 = -3106i64;
    let mut u16_7: u16 = 7679u16;
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_7};
    let mut romtiddle_6_ref_0: &crate::hp::RomTiddle = &mut romtiddle_6;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_2};
    let mut wonreasley_1_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_1;
    let mut i64_3: i64 = 16518i64;
    let mut u16_8: u16 = 92u16;
    let mut romtiddle_7: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_8};
    let mut romtiddle_7_ref_0: &crate::hp::RomTiddle = &mut romtiddle_7;
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_7_ref_0);
    let mut wonreasley_2: crate::hp::WonReasley = crate::hp::WonReasley {x: string_3, y: i64_3};
    let mut u16_9: u16 = 3764u16;
    let mut romtiddle_8: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_9};
    let mut romtiddle_8_ref_0: &crate::hp::RomTiddle = &mut romtiddle_8;
    let mut string_4: std::string::String = crate::hp::RomTiddle::name(romtiddle_8_ref_0);
    let mut u16_10: u16 = 5232u16;
    let mut romtiddle_9: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_10};
    let mut romtiddle_9_ref_0: &crate::hp::RomTiddle = &mut romtiddle_9;
    let mut usize_0: usize = 6830usize;
    let mut i32_6: i32 = -4252i32;
    let mut i32_7: i32 = -3932i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_2, i32_6);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u16_11: u16 = 4871u16;
    let mut romtiddle_10: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_5: std::string::String = crate::hp::RomTiddle::name(romtiddle_9_ref_0);
    let mut wonreasley_3: crate::hp::WonReasley = crate::hp::WonReasley {x: string_4, y: i64_1};
    let mut u16_12: u16 = 860u16;
    let mut romtiddle_11: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_11};
    let mut u16_13: u16 = 8102u16;
    let mut u64_0: u64 = 4310u64;
    let mut u64_1: u64 = 6422u64;
    let mut u16_14: u16 = 1384u16;
    let mut romtiddle_12: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_13};
    let mut romtiddle_11_ref_0: &crate::hp::RomTiddle = &mut romtiddle_11;
    let mut string_6: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut romtiddle_13: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_12};
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut romtiddle_14: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_14};
    let mut romtiddle_10_ref_0: &crate::hp::RomTiddle = &mut romtiddle_10;
    let mut i32_8: i32 = crate::hp::ParryHotter::accio(parryhotter_2_ref_0, i32_3, i32_7);
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_1_ref_0);
    crate::hp::ParryHotter::another_number_fn(u64_0, u64_1);
    let mut i32_9: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_0_ref_0, usize_0, string_1);
    crate::hp::RomTiddle::avada_kedavra(romtiddle_4_ref_0, string_5);
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_8, i32_9);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1248() {
    rusty_monitor::set_test_id(1248);
    let mut i32_0: i32 = -3537i32;
    let mut i32_1: i32 = 3894i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_1, b: i32_0};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i64_0: i64 = 19291i64;
    let mut u16_0: u16 = 1717u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut i64_1: i64 = 1637i64;
    let mut u16_1: u16 = 3704u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_1};
    let mut u16_2: u16 = 136u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut u16_3: u16 = 8347u16;
    let mut u16_4: u16 = 3123u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut u16_5: u16 = 6394u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_4_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_4;
    let mut u16_6: u16 = 9014u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    let mut romtiddle_5_ref_0: &crate::hp::RomTiddle = &mut romtiddle_5;
    let mut u16_7: u16 = 3529u16;
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_7};
    let mut romtiddle_6_ref_0: &crate::hp::RomTiddle = &mut romtiddle_6;
    let mut str_0: &str = "j3a7SLMALVOjXGIyP";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_2: i64 = -15599i64;
    let mut u16_8: u16 = 9777u16;
    let mut romtiddle_7: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_8};
    let mut romtiddle_7_ref_0: &crate::hp::RomTiddle = &mut romtiddle_7;
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_7_ref_0);
    let mut wonreasley_2: crate::hp::WonReasley = crate::hp::WonReasley {x: string_3, y: i64_2};
    let mut wonreasley_2_ref_0: &crate::hp::WonReasley = &mut wonreasley_2;
    let mut i64_3: i64 = -7749i64;
    let mut u64_0: u64 = 6422u64;
    let mut u16_9: u16 = 1384u16;
    let mut romtiddle_8: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_4: std::string::String = crate::hp::RomTiddle::name(romtiddle_5_ref_0);
    let mut u16_10: u16 = 9462u16;
    let mut romtiddle_9: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_9};
    let mut romtiddle_8_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_8;
    let mut u16_11: u16 = 5547u16;
    let mut romtiddle_10: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_10};
    let mut romtiddle_9_ref_0: &crate::hp::RomTiddle = &mut romtiddle_9;
    let mut u16_12: u16 = 5820u16;
    let mut romtiddle_11: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_11};
    let mut romtiddle_10_ref_0: &crate::hp::RomTiddle = &mut romtiddle_10;
    let mut u16_13: u16 = 4715u16;
    let mut romtiddle_12: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_12};
    let mut romtiddle_11_ref_0: &crate::hp::RomTiddle = &mut romtiddle_11;
    let mut u64_1: u64 = 7285u64;
    let mut i64_4: i64 = -9580i64;
    let mut romtiddle_13: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_13};
    let mut romtiddle_12_ref_0: &crate::hp::RomTiddle = &mut romtiddle_12;
    let mut string_5: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut wonreasley_3: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_4};
    let mut wonreasley_1_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_1;
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_3);
    crate::hp::ParryHotter::another_number_fn(u64_0, u64_1);
    let mut string_6: std::string::String = crate::hp::RomTiddle::name(romtiddle_11_ref_0);
    crate::hp::WonReasley::arania_exumai(wonreasley_2_ref_0, str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1250() {
    rusty_monitor::set_test_id(1250);
    let mut i32_0: i32 = -1171i32;
    let mut i32_1: i32 = 4851i32;
    let mut i32_2: i32 = 4405i32;
    let mut i32_3: i32 = -6169i32;
    let mut i32_4: i32 = 13686i32;
    let mut i32_5: i32 = 10440i32;
    let mut i32_6: i32 = -2974i32;
    let mut i32_7: i32 = 7665i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_7, b: i32_6};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u16_0: u16 = 2090u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut u16_1: u16 = 2459u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_1;
    let mut u16_2: u16 = 731u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_2;
    let mut i64_0: i64 = -3531i64;
    let mut i64_1: i64 = -3106i64;
    let mut u16_3: u16 = 7679u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_1};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut i64_2: i64 = 16518i64;
    let mut u16_4: u16 = 92u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_2};
    let mut i64_3: i64 = 10344i64;
    let mut u16_5: u16 = 3764u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_5_ref_0: &crate::hp::RomTiddle = &mut romtiddle_5;
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_5_ref_0);
    let mut u16_6: u16 = 5232u16;
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    let mut romtiddle_6_ref_0: &crate::hp::RomTiddle = &mut romtiddle_6;
    let mut u16_7: u16 = 7360u16;
    let mut romtiddle_7: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_7};
    let mut u16_8: u16 = 3940u16;
    let mut str_0: &str = "i7oWW";
    let mut str_1: &str = "tFHWYKWui77OJ";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut i64_4: i64 = -13016i64;
    let mut u16_9: u16 = 2081u16;
    let mut romtiddle_8: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_9};
    let mut romtiddle_8_ref_0: &crate::hp::RomTiddle = &mut romtiddle_8;
    let mut string_4: std::string::String = crate::hp::RomTiddle::name(romtiddle_8_ref_0);
    let mut wonreasley_2: crate::hp::WonReasley = crate::hp::WonReasley {x: string_4, y: i64_4};
    let mut wonreasley_2_ref_0: &crate::hp::WonReasley = &mut wonreasley_2;
    let mut u64_0: u64 = 6918u64;
    let mut u64_1: u64 = 6015u64;
    let mut i32_8: i32 = 3725i32;
    let mut i32_9: i32 = 473i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_9, i32_8);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_5: i64 = -3090i64;
    let mut u16_10: u16 = 1649u16;
    let mut romtiddle_9: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_8};
    let mut romtiddle_7_ref_0: &crate::hp::RomTiddle = &mut romtiddle_7;
    let mut string_5: std::string::String = crate::hp::RomTiddle::name(romtiddle_6_ref_0);
    let mut wonreasley_3: crate::hp::WonReasley = crate::hp::WonReasley {x: string_3, y: i64_3};
    let mut wonreasley_1_ref_0: &crate::hp::WonReasley = &mut wonreasley_1;
    let mut romtiddle_10: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_10};
    let mut romtiddle_9_ref_0: &crate::hp::RomTiddle = &mut romtiddle_9;
    let mut string_6: std::string::String = crate::hp::RomTiddle::name(romtiddle_7_ref_0);
    let mut wonreasley_4: crate::hp::WonReasley = crate::hp::WonReasley {x: string_5, y: i64_5};
    let mut wonreasley_3_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_3;
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_0);
    crate::hp::WonReasley::arania_exumai(wonreasley_1_ref_0, str_0_ref_0);
    let mut string_7: std::string::String = crate::hp::RomTiddle::name(romtiddle_9_ref_0);
    crate::hp::RomTiddle::avada_kedavra(romtiddle_2_ref_0, string_6);
    crate::hp::ParryHotter::another_number_fn(u64_0, u64_1);
    crate::hp::RomTiddle::avada_kedavra(romtiddle_1_ref_0, string_0);
    crate::hp::ParryHotter::foo2(parryhotter_0_ref_0, i32_5, i32_4);
    crate::hp::ParryHotter::alohomora(i32_3, i32_2, i32_1, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1255() {
    rusty_monitor::set_test_id(1255);
    let mut i32_0: i32 = 6119i32;
    let mut i32_1: i32 = -6060i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_1, b: i32_0};
    let mut u16_0: u16 = 9558u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut u16_1: u16 = 9557u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut u16_2: u16 = 5290u16;
    let mut i64_0: i64 = 8347i64;
    let mut u16_3: u16 = 3694u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut i32_2: i32 = 1436i32;
    let mut i32_3: i32 = -16320i32;
    let mut i32_4: i32 = -7118i32;
    let mut str_0: &str = "mi1V0U7nWJi1UcSlxUn";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_1: i64 = -5733i64;
    let mut u16_4: u16 = 8966u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_1};
    let mut wonreasley_1_ref_0: &crate::hp::WonReasley = &mut wonreasley_1;
    let mut u16_5: u16 = 6771u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut i64_2: i64 = -1556i64;
    let mut i64_3: i64 = 5230i64;
    let mut u16_6: u16 = 2884u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    let mut romtiddle_5_ref_0: &crate::hp::RomTiddle = &mut romtiddle_5;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    let mut wonreasley_2: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_3};
    let mut wonreasley_2_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_2;
    let mut str_1: &str = "nNO6lyq5Kb";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut i64_4: i64 = -8617i64;
    let mut u16_7: u16 = 654u16;
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_7};
    let mut romtiddle_6_ref_0: &crate::hp::RomTiddle = &mut romtiddle_6;
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_6_ref_0);
    let mut string_4: std::string::String = crate::hp::RomTiddle::name(romtiddle_5_ref_0);
    let mut wonreasley_3: crate::hp::WonReasley = crate::hp::WonReasley {x: string_3, y: i64_4};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut i64_5: i64 = -8289i64;
    let mut u16_8: u16 = 9229u16;
    let mut romtiddle_7: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_5: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut i32_5: i32 = -6686i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_4);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_6: i32 = -7606i32;
    let mut i32_7: i32 = -18293i32;
    let mut i32_8: i32 = -12627i32;
    let mut i32_9: i32 = -8015i32;
    let mut i64_6: i64 = 3820i64;
    let mut i64_7: i64 = 10663i64;
    let mut u16_9: u16 = 3114u16;
    let mut romtiddle_8: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_8};
    let mut romtiddle_7_ref_0: &crate::hp::RomTiddle = &mut romtiddle_7;
    let mut string_6: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut wonreasley_4: crate::hp::WonReasley = crate::hp::WonReasley {x: string_5, y: i64_7};
    let mut wonreasley_3_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_3;
    let mut i64_8: i64 = -4390i64;
    let mut u16_10: u16 = 7969u16;
    let mut romtiddle_9: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_9};
    let mut romtiddle_8_ref_0: &crate::hp::RomTiddle = &mut romtiddle_8;
    let mut string_7: std::string::String = crate::hp::RomTiddle::name(romtiddle_7_ref_0);
    let mut i64_9: i64 = -17884i64;
    let mut i64_10: i64 = -13962i64;
    let mut romtiddle_10: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_10};
    let mut romtiddle_9_ref_0: &crate::hp::RomTiddle = &mut romtiddle_9;
    crate::hp::RomTiddle::foo3(romtiddle_8_ref_0, i64_8, i64_5);
    let mut wonreasley_5: crate::hp::WonReasley = crate::hp::WonReasley {x: string_7, y: i64_2};
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_9);
    crate::hp::ParryHotter::alohomora(i32_9, i32_6, i32_8, i32_2);
    let mut wonreasley_6: crate::hp::WonReasley = crate::hp::WonReasley {x: string_6, y: i64_10};
    let mut wonreasley_5_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_5;
    crate::hp::WonReasley::ascendio(wonreasley_3_ref_0, i64_6);
    let mut i32_10: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_7, i32_5);
    crate::hp::WonReasley::arania_exumai(wonreasley_1_ref_0, str_0_ref_0);
    let mut string_8: std::string::String = crate::hp::RomTiddle::name(romtiddle_9_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1258() {
    rusty_monitor::set_test_id(1258);
    let mut i64_0: i64 = 5611i64;
    let mut i64_1: i64 = -6795i64;
    let mut u16_0: u16 = 8794u16;
    let mut u16_1: u16 = 8698u16;
    let mut i64_2: i64 = 503i64;
    let mut i64_3: i64 = -8940i64;
    let mut u16_2: u16 = 4019u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut u16_3: u16 = 1591u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut u16_4: u16 = 5451u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_2_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_2;
    let mut i32_0: i32 = 5733i32;
    let mut i32_1: i32 = -8913i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut str_0: &str = "jVbXZ8bVVV5";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u16_5: u16 = 7892u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut i32_2: i32 = 1183i32;
    let mut i32_3: i32 = 11929i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut u64_0: u64 = 6858u64;
    let mut u64_1: u64 = 1285u64;
    let mut usize_0: usize = 6891usize;
    let mut i32_4: i32 = 13690i32;
    let mut i32_5: i32 = 7119i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_5, b: i32_4};
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i32_6: i32 = 7120i32;
    let mut i32_7: i32 = 9087i32;
    let mut u16_6: u16 = 2309u16;
    let mut i64_4: i64 = 21266i64;
    let mut i64_5: i64 = -5460i64;
    let mut u16_7: u16 = 4135u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_7};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_5};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_4);
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_7, i32_6);
    let mut parryhotter_3_ref_0: &crate::hp::ParryHotter = &mut parryhotter_3;
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_2_ref_0, usize_0, u64_1, u64_0);
    crate::hp::RomTiddle::avada_kedavra(romtiddle_2_ref_0, string_0);
    crate::hp::RomTiddle::foo3(romtiddle_0_ref_0, i64_3, i64_2);
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_7: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_7_ref_0: &crate::hp::RomTiddle = &mut romtiddle_7;
    crate::hp::RomTiddle::foo3(romtiddle_7_ref_0, i64_1, i64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1259() {
    rusty_monitor::set_test_id(1259);
    let mut i32_0: i32 = -7147i32;
    let mut i32_1: i32 = 13019i32;
    let mut i32_2: i32 = 13617i32;
    let mut i32_3: i32 = -3536i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u16_0: u16 = 4530u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut u16_1: u16 = 1417u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_1;
    let mut u16_2: u16 = 7071u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut u16_3: u16 = 9119u16;
    let mut i32_4: i32 = 337i32;
    let mut i32_5: i32 = -6391i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_5, i32_4);
    let mut u64_0: u64 = 1330u64;
    let mut i64_0: i64 = -1037i64;
    let mut u16_4: u16 = 233u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_0};
    let mut i32_6: i32 = -783i32;
    let mut i32_7: i32 = 14922i32;
    let mut u16_5: u16 = 4267u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut i32_8: i32 = 12106i32;
    let mut i32_9: i32 = -11788i32;
    let mut i32_10: i32 = -11180i32;
    let mut i32_11: i32 = -11333i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_8, b: i32_9};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i64_1: i64 = 13536i64;
    let mut u16_6: u16 = 7164u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    let mut u16_7: u16 = 6476u16;
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    let mut romtiddle_5_ref_0: &crate::hp::RomTiddle = &mut romtiddle_5;
    let mut u64_1: u64 = 7445u64;
    let mut usize_0: usize = 3136usize;
    let mut i32_12: i32 = -15686i32;
    let mut i32_13: i32 = 4826i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_6, i32_12);
    let mut i32_14: i32 = 8296i32;
    let mut i32_15: i32 = 5963i32;
    let mut parryhotter_4: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_10, i32_14);
    let mut parryhotter_3_ref_0: &crate::hp::ParryHotter = &mut parryhotter_3;
    let mut i32_16: i32 = -8628i32;
    let mut i32_17: i32 = -3530i32;
    let mut i32_18: i32 = 9310i32;
    let mut i32_19: i32 = -1857i32;
    let mut i32_20: i32 = 5357i32;
    let mut parryhotter_5: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_15, i32_18);
    let mut parryhotter_4_ref_0: &crate::hp::ParryHotter = &mut parryhotter_4;
    let mut i32_21: i32 = crate::hp::ParryHotter::accio(parryhotter_3_ref_0, i32_7, i32_17);
    let mut romtiddle_7: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_7};
    crate::hp::ParryHotter::alohomora(i32_20, i32_21, i32_13, i32_16);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_1_ref_0, usize_0, u64_1, u64_0);
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_1};
    let mut i32_22: i32 = crate::hp::ParryHotter::accio(parryhotter_4_ref_0, i32_19, i32_11);
    crate::hp::RomTiddle::avada_kedavra(romtiddle_1_ref_0, string_0);
    let mut string_4: std::string::String = crate::hp::RomTiddle::name(romtiddle_5_ref_0);
    let mut i32_23: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_1, i32_0);
    panic!("From RustyUnit with love");
}
}