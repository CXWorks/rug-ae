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
fn rusty_test_3153() {
    rusty_monitor::set_test_id(3153);
    let mut i64_0: i64 = 1519i64;
    let mut u16_0: u16 = 8541u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut i32_0: i32 = -6938i32;
    let mut i32_1: i32 = -1362i32;
    let mut i64_1: i64 = 1000i64;
    let mut u16_1: u16 = 6141u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_1};
    let mut u16_2: u16 = 6174u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut u16_3: u16 = 1970u16;
    let mut str_0: &str = "C0PhE5";
    let mut u16_4: u16 = 4891u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut u64_0: u64 = 161u64;
    let mut u64_1: u64 = 4844u64;
    let mut i32_2: i32 = -8442i32;
    let mut i32_3: i32 = 11269i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i64_2: i64 = 7880i64;
    let mut i64_3: i64 = -1341i64;
    let mut u16_5: u16 = 4862u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut i32_4: i32 = -10835i32;
    let mut i32_5: i32 = 8896i32;
    let mut i32_6: i32 = -27944i32;
    let mut i32_7: i32 = -3841i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_4, b: i32_6};
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_4: i64 = 8922i64;
    let mut u16_6: u16 = 7653u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_3};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    let mut i32_8: i32 = 8959i32;
    let mut i32_9: i32 = 2967i32;
    let mut i32_10: i32 = -3522i32;
    let mut i32_11: i32 = 1677i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_8, i32_7);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i64_5: i64 = 3205i64;
    let mut i64_6: i64 = -2706i64;
    let mut i32_12: i32 = -3142i32;
    let mut i32_13: i32 = 16568i32;
    let mut i32_14: i32 = 9501i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_10, i32_5);
    let mut parryhotter_3_ref_0: &crate::hp::ParryHotter = &mut parryhotter_3;
    crate::hp::ParryHotter::foo2(parryhotter_1_ref_0, i32_14, i32_9);
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    let mut romtiddle_6_ref_0: &crate::hp::RomTiddle = &mut romtiddle_6;
    crate::hp::RomTiddle::foo3(romtiddle_4_ref_0, i64_2, i64_6);
    let mut i32_15: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_11, i32_12);
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
    let mut i32_16: i32 = crate::hp::ParryHotter::accio(parryhotter_3_ref_0, i32_15, i32_13);
    crate::hp::RomTiddle::foo3(romtiddle_6_ref_0, i64_4, i64_5);
    let mut parryhotter_4: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut wonreasley_2: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4095() {
    rusty_monitor::set_test_id(4095);
    let mut u16_0: u16 = 8681u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut u16_1: u16 = 1434u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut u16_2: u16 = 2718u16;
    let mut u64_0: u64 = 2913u64;
    let mut u16_3: u16 = 7991u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut u16_4: u16 = 6043u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut u64_1: u64 = 3424u64;
    crate::hp::ParryHotter::another_number_fn(u64_0, u64_1);
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    panic!("From RustyUnit with love");
}
}