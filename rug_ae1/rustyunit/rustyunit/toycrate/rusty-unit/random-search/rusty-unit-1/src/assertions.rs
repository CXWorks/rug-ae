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
fn rusty_test_1787() {
    rusty_monitor::set_test_id(1787);
    let mut u64_0: u64 = 6515u64;
    let mut u64_1: u64 = 9029u64;
    let mut i32_0: i32 = -8707i32;
    let mut i32_1: i32 = -7963i32;
    let mut i32_2: i32 = 4258i32;
    let mut i32_3: i32 = 1694i32;
    let mut u64_2: u64 = 916u64;
    let mut u64_3: u64 = 4849u64;
    let mut usize_0: usize = 7102usize;
    let mut i32_4: i32 = 3630i32;
    let mut i32_5: i32 = 11273i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_5, b: i32_4};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u64_4: u64 = 9305u64;
    let mut u64_5: u64 = 6106u64;
    let mut usize_1: usize = 6778usize;
    let mut i32_6: i32 = -2203i32;
    let mut i32_7: i32 = 13571i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_7, b: i32_6};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_8: i32 = 6148i32;
    let mut i32_9: i32 = -6141i32;
    let mut i32_10: i32 = 22004i32;
    let mut i32_11: i32 = -5094i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_11, b: i32_10};
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i32_12: i32 = crate::hp::ParryHotter::accio(parryhotter_2_ref_0, i32_9, i32_8);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_1_ref_0, usize_1, u64_5, u64_4);
    let mut bool_1: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_0_ref_0, usize_0, u64_3, u64_2);
    crate::hp::ParryHotter::alohomora(i32_3, i32_2, i32_1, i32_0);
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2026() {
    rusty_monitor::set_test_id(2026);
    let mut i64_0: i64 = 5372i64;
    let mut i64_1: i64 = 8745i64;
    let mut u16_0: u16 = 8003u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_1};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut u16_1: u16 = 7703u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut i64_2: i64 = -14976i64;
    let mut i64_3: i64 = -1611i64;
    let mut u16_2: u16 = 4839u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut i64_4: i64 = -9018i64;
    let mut i64_5: i64 = 1291i64;
    let mut u16_3: u16 = 8915u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut i64_6: i64 = -4362i64;
    let mut i64_7: i64 = -4i64;
    let mut u16_4: u16 = 233u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut i64_8: i64 = -1984i64;
    let mut i64_9: i64 = 10495i64;
    let mut u16_5: u16 = 3683u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_5_ref_0: &crate::hp::RomTiddle = &mut romtiddle_5;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_5_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_9};
    let mut wonreasley_1_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_1;
    let mut u64_0: u64 = 9105u64;
    let mut u64_1: u64 = 4509u64;
    let mut usize_0: usize = 7853usize;
    let mut i32_0: i32 = -15027i32;
    let mut i32_1: i32 = 6244i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u64_2: u64 = 4511u64;
    let mut u64_3: u64 = 3576u64;
    let mut u16_6: u16 = 6710u16;
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    let mut romtiddle_6_ref_0: &crate::hp::RomTiddle = &mut romtiddle_6;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_6_ref_0);
    let mut usize_1: usize = 9109usize;
    let mut i32_2: i32 = -6318i32;
    let mut i32_3: i32 = -10719i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_4: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_1_ref_0, usize_1, string_2);
    crate::hp::ParryHotter::another_number_fn(u64_3, u64_2);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_0_ref_0, usize_0, u64_1, u64_0);
    crate::hp::WonReasley::ascendio(wonreasley_1_ref_0, i64_8);
    crate::hp::RomTiddle::foo3(romtiddle_4_ref_0, i64_7, i64_6);
    crate::hp::RomTiddle::foo3(romtiddle_3_ref_0, i64_5, i64_4);
    crate::hp::RomTiddle::foo3(romtiddle_2_ref_0, i64_3, i64_2);
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_0);
    panic!("From RustyUnit with love");
}
}