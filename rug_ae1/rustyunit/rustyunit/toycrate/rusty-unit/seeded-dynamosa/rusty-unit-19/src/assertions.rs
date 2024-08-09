fn foo(x: &[i32], y: usize) -> i32 {
    x[y]
}


#[cfg(test)]
mod rusty_tests {
	use crate::*;

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3492() {
//    rusty_monitor::set_test_id(3492);
    let mut i32_0: i32 = 3i32;
    let mut i32_1: i32 = 140i32;
    let mut i32_2: i32 = -9664i32;
    let mut i32_3: i32 = 7i32;
    let mut u16_0: u16 = 1u16;
    let mut i32_4: i32 = 10i32;
    let mut i32_5: i32 = 140i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_5, b: i32_4};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u64_0: u64 = 9283u64;
    let mut u64_1: u64 = 20u64;
    let mut usize_0: usize = 10usize;
    let mut i32_6: i32 = 11i32;
    let mut i32_7: i32 = 111i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_7, i32_6);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_8: i32 = 10i32;
    let mut i32_9: i32 = 18029i32;
    let mut i32_10: i32 = 10i32;
    let mut i32_11: i32 = 11i32;
    let mut i32_12: i32 = 15i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_12, b: i32_11};
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i64_0: i64 = 9223372036854775527i64;
    let mut i64_1: i64 = 3700i64;
    let mut u16_1: u16 = 8929u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_1};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut u64_2: u64 = 7383u64;
    let mut u64_3: u64 = 1017u64;
    let mut usize_1: usize = 4955usize;
    let mut i32_13: i32 = -5795i32;
    let mut i32_14: i32 = 2161i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_14, b: i32_13};
    let mut parryhotter_3_ref_0: &crate::hp::ParryHotter = &mut parryhotter_3;
    let mut u16_2: u16 = 831u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut usize_2: usize = 10usize;
    let mut i32_15: i32 = 100i32;
    let mut i32_16: i32 = -11919i32;
    let mut parryhotter_4: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_16, b: i32_15};
    let mut parryhotter_4_ref_0: &crate::hp::ParryHotter = &mut parryhotter_4;
    let mut i32_17: i32 = 40i32;
    let mut i32_18: i32 = 40i32;
    let mut i32_19: i32 = 10i32;
    let mut i32_20: i32 = 13i32;
    let mut parryhotter_5: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_20, i32_19);
    let mut parryhotter_5_ref_0: &crate::hp::ParryHotter = &mut parryhotter_5;
    let mut str_0: &str = "gNulpebPMB";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_2: i64 = 0i64;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_2};
    let mut wonreasley_1_ref_0: &crate::hp::WonReasley = &mut wonreasley_1;
    let mut i32_21: i32 = -6648i32;
    let mut i32_22: i32 = -559i32;
    let mut i32_23: i32 = 15426i32;
    let mut parryhotter_6: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_23, b: i32_8};
    let mut parryhotter_6_ref_0: &crate::hp::ParryHotter = &mut parryhotter_6;
    let mut i32_24: i32 = 7i32;
    let mut i32_25: i32 = 40i32;
    let mut i32_26: i32 = -11573i32;
    let mut i32_27: i32 = 5759i32;
    let mut parryhotter_7: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_27, i32_26);
    let mut parryhotter_7_ref_0: &crate::hp::ParryHotter = &mut parryhotter_7;
    crate::hp::ParryHotter::foo2(parryhotter_7_ref_0, i32_25, i32_24);
    let mut i32_28: i32 = crate::hp::ParryHotter::accio(parryhotter_6_ref_0, i32_22, i32_21);
    crate::hp::WonReasley::arania_exumai(wonreasley_1_ref_0, str_0_ref_0);
    crate::hp::ParryHotter::foo2(parryhotter_5_ref_0, i32_18, i32_17);
    let mut i32_29: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_4_ref_0, usize_2, string_1);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_3_ref_0, usize_1, u64_3, u64_2);
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_0);
    crate::hp::ParryHotter::foo2(parryhotter_2_ref_0, i32_10, i32_9);
    let mut bool_1: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_1_ref_0, usize_0, u64_1, u64_0);
    crate::hp::ParryHotter::alohomora(i32_3, i32_2, i32_1, i32_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8231() {
//    rusty_monitor::set_test_id(8231);
    let mut i32_0: i32 = 40i32;
    let mut i32_1: i32 = 4671i32;
    let mut i32_2: i32 = 7187i32;
    let mut i32_3: i32 = 140i32;
    let mut i32_4: i32 = 111i32;
    let mut i32_5: i32 = 32i32;
    let mut i32_6: i32 = 3i32;
    let mut i32_7: i32 = 1877i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_7, i32_6);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut str_0: &str = "1iy56EArmAVOTjQMi";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_0: i64 = -3616i64;
    let mut i64_1: i64 = -3868i64;
    let mut u16_0: u16 = 7424u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_1};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut i32_8: i32 = 13i32;
    let mut i32_9: i32 = 111i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_9, i32_8);
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_0);
    let mut i32_10: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_5, i32_4);
    crate::hp::ParryHotter::alohomora(i32_3, i32_2, i32_1, i32_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_100() {
//    rusty_monitor::set_test_id(100);
    let mut i64_0: i64 = 0i64;
    let mut i64_1: i64 = 0i64;
    let mut u16_0: u16 = 3u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_1};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut u16_1: u16 = 1u16;
    let mut u16_2: u16 = 1u16;
    let mut u16_3: u16 = 3u16;
    let mut i64_2: i64 = 9223372036854775807i64;
    let mut i64_3: i64 = 100i64;
    let mut u16_4: u16 = 1004u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut str_0: &str = "ten";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_4: i64 = 100i64;
    let mut u16_5: u16 = 1u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_4};
    let mut wonreasley_1_ref_0: &crate::hp::WonReasley = &mut wonreasley_1;
    crate::hp::WonReasley::arania_exumai(wonreasley_1_ref_0, str_0_ref_0);
    crate::hp::RomTiddle::foo3(romtiddle_1_ref_0, i64_3, i64_2);
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_0);
//    panic!("From RustyUnit with love");
}
}