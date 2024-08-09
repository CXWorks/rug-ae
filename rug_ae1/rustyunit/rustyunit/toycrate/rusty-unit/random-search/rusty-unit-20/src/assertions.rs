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
fn rusty_test_1872() {
    rusty_monitor::set_test_id(1872);
    let mut i64_0: i64 = -12213i64;
    let mut i64_1: i64 = -2300i64;
    let mut u16_0: u16 = 233u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut u64_0: u64 = 3101u64;
    let mut u64_1: u64 = 6213u64;
    let mut usize_0: usize = 6892usize;
    let mut i32_0: i32 = 795i32;
    let mut i32_1: i32 = -5865i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u16_1: u16 = 1214u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut usize_1: usize = 6415usize;
    let mut i32_2: i32 = 15704i32;
    let mut i32_3: i32 = -9447i32;
    let mut i32_4: i32 = 3328i32;
    let mut i32_5: i32 = 1121i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_5, i32_4);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_6: i32 = 9297i32;
    let mut i32_7: i32 = -5696i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_7, b: i32_6};
    crate::hp::ParryHotter::foo2(parryhotter_1_ref_0, i32_3, i32_2);
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i32_8: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_2_ref_0, usize_1, string_0);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_0_ref_0, usize_0, u64_1, u64_0);
    crate::hp::RomTiddle::foo3(romtiddle_0_ref_0, i64_1, i64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4642() {
    rusty_monitor::set_test_id(4642);
    let mut u64_0: u64 = 5353u64;
    let mut u64_1: u64 = 1941u64;
    let mut u16_0: u16 = 8455u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut i32_0: i32 = 27860i32;
    let mut i32_1: i32 = 3501i32;
    let mut i64_0: i64 = -18038i64;
    let mut i64_1: i64 = -2858i64;
    let mut u16_1: u16 = 9515u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_1};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut i32_2: i32 = -4298i32;
    let mut i32_3: i32 = 494i32;
    let mut i32_4: i32 = 7404i32;
    let mut i32_5: i32 = 11447i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_5, i32_4);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u16_2: u16 = 966u16;
    let mut i32_6: i32 = -5293i32;
    let mut i32_7: i32 = -9828i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_7, i32_6);
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    crate::hp::ParryHotter::foo2(parryhotter_0_ref_0, i32_3, i32_2);
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_0);
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_1, b: i32_0};
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4128() {
    rusty_monitor::set_test_id(4128);
    let mut i32_0: i32 = 1566i32;
    let mut i32_1: i32 = -1630i32;
    let mut i32_2: i32 = 10149i32;
    let mut i32_3: i32 = -18290i32;
    let mut i64_0: i64 = -2488i64;
    let mut i64_1: i64 = 17530i64;
    let mut u16_0: u16 = 824u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut u64_0: u64 = 6799u64;
    let mut u64_1: u64 = 4930u64;
    let mut usize_0: usize = 3807usize;
    let mut i32_4: i32 = 3897i32;
    let mut i32_5: i32 = -20635i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_5, b: i32_4};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_6: i32 = 1879i32;
    let mut i32_7: i32 = -927i32;
    let mut i32_8: i32 = -2914i32;
    let mut i32_9: i32 = -1279i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_9, b: i32_8};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_10: i32 = -13173i32;
    let mut i32_11: i32 = -5464i32;
    let mut i32_12: i32 = -9955i32;
    let mut i32_13: i32 = -6823i32;
    crate::hp::ParryHotter::alohomora(i32_13, i32_12, i32_11, i32_10);
    let mut i32_14: i32 = crate::hp::ParryHotter::accio(parryhotter_1_ref_0, i32_7, i32_6);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_0_ref_0, usize_0, u64_1, u64_0);
    crate::hp::RomTiddle::foo3(romtiddle_0_ref_0, i64_1, i64_0);
    crate::hp::ParryHotter::alohomora(i32_3, i32_2, i32_1, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1117() {
    rusty_monitor::set_test_id(1117);
    let mut i32_0: i32 = 11i32;
    let mut i32_1: i32 = -4598i32;
    let mut i64_0: i64 = -1229i64;
    let mut i64_1: i64 = 3122i64;
    let mut u16_0: u16 = 3929u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut i32_2: i32 = -3239i32;
    let mut i32_3: i32 = 12592i32;
    let mut i32_4: i32 = 2947i32;
    let mut i32_5: i32 = 7029i32;
    let mut i32_6: i32 = -2913i32;
    let mut i32_7: i32 = -2615i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_7, b: i32_6};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u64_0: u64 = 3146u64;
    let mut u64_1: u64 = 4043u64;
    let mut usize_0: usize = 301usize;
    let mut i32_8: i32 = -19128i32;
    let mut i32_9: i32 = -2591i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_9, i32_8);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_10: i32 = -2668i32;
    let mut i32_11: i32 = 16712i32;
    let mut i32_12: i32 = -5375i32;
    let mut i32_13: i32 = -3147i32;
    let mut i32_14: i32 = 9955i32;
    let mut i32_15: i32 = 13063i32;
    let mut i32_16: i32 = 5339i32;
    let mut i32_17: i32 = 1814i32;
    let mut u64_2: u64 = 6194u64;
    let mut u64_3: u64 = 5428u64;
    let mut u64_4: u64 = 8487u64;
    let mut u64_5: u64 = 822u64;
    let mut i64_2: i64 = 1936i64;
    let mut i64_3: i64 = 383i64;
    let mut u16_1: u16 = 1993u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut u64_6: u64 = 9666u64;
    let mut u64_7: u64 = 6861u64;
    let mut usize_1: usize = 3962usize;
    let mut i32_18: i32 = -23322i32;
    let mut i32_19: i32 = -5208i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_19, b: i32_18};
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut u16_2: u16 = 6101u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut i64_4: i64 = -12822i64;
    let mut i64_5: i64 = -9588i64;
    let mut u16_3: u16 = 6478u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_5};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut str_0: &str = "doVviLsjXRSWMRzWag";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_6: i64 = -6559i64;
    let mut u16_4: u16 = 889u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_6};
    let mut wonreasley_1_ref_0: &crate::hp::WonReasley = &mut wonreasley_1;
    crate::hp::WonReasley::arania_exumai(wonreasley_1_ref_0, str_0_ref_0);
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_4);
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_2_ref_0, usize_1, u64_7, u64_6);
    crate::hp::RomTiddle::foo3(romtiddle_1_ref_0, i64_3, i64_2);
    crate::hp::ParryHotter::another_number_fn(u64_5, u64_4);
    crate::hp::ParryHotter::another_number_fn(u64_3, u64_2);
    crate::hp::ParryHotter::alohomora(i32_17, i32_16, i32_15, i32_14);
    crate::hp::ParryHotter::alohomora(i32_13, i32_12, i32_11, i32_10);
    let mut bool_1: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_1_ref_0, usize_0, u64_1, u64_0);
    let mut i32_20: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_5, i32_4);
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    crate::hp::RomTiddle::foo3(romtiddle_0_ref_0, i64_1, i64_0);
    let mut parryhotter_3_ref_0: &crate::hp::ParryHotter = &mut parryhotter_3;
    crate::hp::ParryHotter::foo2(parryhotter_3_ref_0, i32_1, i32_0);
    panic!("From RustyUnit with love");
}
}