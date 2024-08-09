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
fn rusty_test_4482() {
    rusty_monitor::set_test_id(4482);
    let mut u64_0: u64 = 5890u64;
    let mut u64_1: u64 = 9111u64;
    let mut usize_0: usize = 9673usize;
    let mut i32_0: i32 = 11800i32;
    let mut i32_1: i32 = -200i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i64_0: i64 = 7912i64;
    let mut i64_1: i64 = -16613i64;
    let mut u16_0: u16 = 1385u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_1};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut u16_1: u16 = 6230u16;
    let mut i64_2: i64 = 1081i64;
    let mut i64_3: i64 = 9576i64;
    let mut u16_2: u16 = 2335u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_3};
    let mut wonreasley_1_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_1;
    let mut u64_2: u64 = 9657u64;
    let mut u64_3: u64 = 1787u64;
    crate::hp::ParryHotter::another_number_fn(u64_3, u64_2);
    crate::hp::WonReasley::ascendio(wonreasley_1_ref_0, i64_2);
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_0);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_0_ref_0, usize_0, u64_1, u64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3845() {
    rusty_monitor::set_test_id(3845);
    let mut i32_0: i32 = 5628i32;
    let mut i32_1: i32 = -4609i32;
    let mut i32_2: i32 = -344i32;
    let mut i32_3: i32 = -12414i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_4: i32 = 4946i32;
    let mut i32_5: i32 = 2106i32;
    let mut u64_0: u64 = 8680u64;
    let mut u64_1: u64 = 6365u64;
    let mut u64_2: u64 = 955u64;
    let mut u64_3: u64 = 9215u64;
    let mut usize_0: usize = 3808usize;
    let mut i32_6: i32 = -972i32;
    let mut i32_7: i32 = 13440i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_7, i32_6);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut u64_4: u64 = 4610u64;
    let mut u64_5: u64 = 1740u64;
    let mut usize_1: usize = 5455usize;
    let mut i32_8: i32 = 1852i32;
    let mut i32_9: i32 = -9594i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_9, i32_8);
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut u16_0: u16 = 8447u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_2_ref_0, usize_1, u64_5, u64_4);
    let mut bool_1: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_1_ref_0, usize_0, u64_3, u64_2);
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_5, i32_4);
    let mut i32_10: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_1, i32_0);
    panic!("From RustyUnit with love");
}
}