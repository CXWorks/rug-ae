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
fn rusty_test_2710() {
    rusty_monitor::set_test_id(2710);
    let mut u64_0: u64 = 7064u64;
    let mut u64_1: u64 = 1926u64;
    let mut usize_0: usize = 1373usize;
    let mut i32_0: i32 = -15897i32;
    let mut i32_1: i32 = 9985i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_1, b: i32_0};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut str_0: &str = "kkafQUfDjO";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_0: i64 = -7454i64;
    let mut u16_0: u16 = 7322u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    let mut u16_1: u16 = 1567u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut u16_2: u16 = 7413u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut i32_2: i32 = 2473i32;
    let mut i32_3: i32 = -4722i32;
    let mut i32_4: i32 = 10i32;
    let mut i32_5: i32 = -4222i32;
    let mut u64_2: u64 = 7360u64;
    let mut u64_3: u64 = 1076u64;
    let mut usize_1: usize = 8127usize;
    let mut u64_4: u64 = 9922u64;
    let mut u64_5: u64 = 8058u64;
    let mut u64_6: u64 = 1597u64;
    let mut u64_7: u64 = 9986u64;
    let mut usize_2: usize = 7455usize;
    let mut i32_6: i32 = 21769i32;
    let mut i32_7: i32 = -6242i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_7, i32_6);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_8: i32 = -2229i32;
    let mut i32_9: i32 = -11954i32;
    let mut i64_1: i64 = 8272i64;
    let mut i64_2: i64 = 1736i64;
    let mut u16_3: u16 = 5955u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut i64_3: i64 = -14896i64;
    let mut i64_4: i64 = 20764i64;
    let mut u16_4: u16 = 8581u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut u16_5: u16 = 6990u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_5_ref_0: &crate::hp::RomTiddle = &mut romtiddle_5;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_5_ref_0);
    let mut u16_6: u16 = 6227u16;
    let mut i64_5: i64 = 4848i64;
    let mut i64_6: i64 = 618i64;
    let mut u16_7: u16 = 483u16;
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_7};
    let mut romtiddle_6_ref_0: &crate::hp::RomTiddle = &mut romtiddle_6;
    crate::hp::RomTiddle::foo3(romtiddle_6_ref_0, i64_6, i64_5);
    let mut romtiddle_7: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    let mut romtiddle_7_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_7;
    crate::hp::RomTiddle::avada_kedavra(romtiddle_7_ref_0, string_1);
    crate::hp::RomTiddle::foo3(romtiddle_4_ref_0, i64_4, i64_3);
    crate::hp::RomTiddle::foo3(romtiddle_3_ref_0, i64_2, i64_1);
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_9, i32_8);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_1_ref_0, usize_2, u64_7, u64_6);
    crate::hp::ParryHotter::another_number_fn(u64_5, u64_4);
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut bool_1: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_2_ref_0, usize_1, u64_3, u64_2);
    crate::hp::ParryHotter::alohomora(i32_5, i32_4, i32_3, i32_2);
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
    let mut bool_2: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_0_ref_0, usize_0, u64_1, u64_0);
    panic!("From RustyUnit with love");
}
}