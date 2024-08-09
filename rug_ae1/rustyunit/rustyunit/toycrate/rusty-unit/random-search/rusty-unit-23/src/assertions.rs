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
fn rusty_test_230() {
    rusty_monitor::set_test_id(230);
    let mut i32_0: i32 = -4584i32;
    let mut i32_1: i32 = 8395i32;
    let mut i32_2: i32 = -9053i32;
    let mut i32_3: i32 = 5943i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u64_0: u64 = 4769u64;
    let mut u64_1: u64 = 3073u64;
    let mut i32_4: i32 = 2768i32;
    let mut i32_5: i32 = -15818i32;
    let mut i32_6: i32 = -8826i32;
    let mut i32_7: i32 = -16962i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_7, i32_6);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i64_0: i64 = 6829i64;
    let mut i64_1: i64 = -2248i64;
    let mut u16_0: u16 = 3339u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut i64_2: i64 = -5543i64;
    let mut i64_3: i64 = -14638i64;
    let mut u16_1: u16 = 9275u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut u16_2: u16 = 3608u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    crate::hp::RomTiddle::foo3(romtiddle_1_ref_0, i64_3, i64_2);
    crate::hp::RomTiddle::foo3(romtiddle_0_ref_0, i64_1, i64_0);
    crate::hp::ParryHotter::foo2(parryhotter_1_ref_0, i32_5, i32_4);
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    let mut i32_8: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_1, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1668() {
    rusty_monitor::set_test_id(1668);
    let mut i32_0: i32 = -777i32;
    let mut i32_1: i32 = 13409i32;
    let mut u16_0: u16 = 962u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut usize_0: usize = 1708usize;
    let mut i32_2: i32 = 2225i32;
    let mut i32_3: i32 = 3111i32;
    let mut i32_4: i32 = -6678i32;
    let mut i32_5: i32 = 3451i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_5, i32_4);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_6: i32 = -14159i32;
    let mut i32_7: i32 = 11117i32;
    let mut u16_1: u16 = 7544u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut u16_2: u16 = 5435u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_2;
    let mut i32_8: i32 = -10084i32;
    let mut i32_9: i32 = 3497i32;
    let mut i32_10: i32 = 1105i32;
    let mut i32_11: i32 = -7179i32;
    let mut str_0: &str = "6";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_0: i64 = 1042i64;
    let mut u16_3: u16 = 1u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_0};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    let mut i32_12: i32 = -1156i32;
    let mut i32_13: i32 = -2858i32;
    let mut i32_14: i32 = -1504i32;
    let mut i32_15: i32 = -5151i32;
    let mut i32_16: i32 = -4264i32;
    let mut i32_17: i32 = 575i32;
    let mut u16_4: u16 = 8121u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    let mut u16_5: u16 = 3445u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_5_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_5;
    crate::hp::RomTiddle::avada_kedavra(romtiddle_5_ref_0, string_3);
    crate::hp::ParryHotter::alohomora(i32_17, i32_16, i32_15, i32_14);
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_13, i32_12);
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    crate::hp::ParryHotter::foo2(parryhotter_1_ref_0, i32_11, i32_10);
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_9, i32_8);
    crate::hp::RomTiddle::avada_kedavra(romtiddle_2_ref_0, string_1);
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_7, b: i32_6};
    crate::hp::ParryHotter::foo2(parryhotter_0_ref_0, i32_3, i32_2);
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i32_18: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_2_ref_0, usize_0, string_0);
    let mut parryhotter_3_ref_0: &crate::hp::ParryHotter = &mut parryhotter_3;
    crate::hp::ParryHotter::foo2(parryhotter_3_ref_0, i32_1, i32_0);
    panic!("From RustyUnit with love");
}
}