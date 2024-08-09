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
fn rusty_test_467() {
    rusty_monitor::set_test_id(467);
    let mut u16_0: u16 = 0u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut usize_0: usize = 2083usize;
    let mut i32_0: i32 = -1137i32;
    let mut i32_1: i32 = -12638i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_1, b: i32_0};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_2: i32 = -593i32;
    let mut i32_3: i32 = -24i32;
    let mut i32_4: i32 = 6432i32;
    let mut i32_5: i32 = 573i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_5, b: i32_4};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut str_0: &str = "Qu0Ghdxc6";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_0: i64 = -1184i64;
    let mut u16_1: u16 = 6211u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_0};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    let mut str_1: &str = "dtmLiCWp4";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut i64_1: i64 = 1368i64;
    let mut u16_2: u16 = 3462u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_1};
    let mut wonreasley_1_ref_0: &crate::hp::WonReasley = &mut wonreasley_1;
    let mut i64_2: i64 = 9697i64;
    let mut i64_3: i64 = 2432i64;
    let mut u16_3: u16 = 2757u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut wonreasley_2: crate::hp::WonReasley = crate::hp::WonReasley {x: string_3, y: i64_3};
    let mut wonreasley_2_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_2;
    let mut u16_4: u16 = 6178u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut i32_6: i32 = -2750i32;
    let mut i32_7: i32 = -2751i32;
    let mut i32_8: i32 = -15124i32;
    let mut i32_9: i32 = 3146i32;
    let mut u16_5: u16 = 6886u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_5_ref_0: &crate::hp::RomTiddle = &mut romtiddle_5;
    let mut string_4: std::string::String = crate::hp::RomTiddle::name(romtiddle_5_ref_0);
    let mut usize_1: usize = 2421usize;
    let mut i32_10: i32 = 13100i32;
    let mut i32_11: i32 = -8975i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_11, b: i32_10};
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i32_12: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_2_ref_0, usize_1, string_4);
    crate::hp::ParryHotter::alohomora(i32_9, i32_8, i32_7, i32_6);
    let mut string_5: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    crate::hp::WonReasley::ascendio(wonreasley_2_ref_0, i64_2);
    crate::hp::WonReasley::arania_exumai(wonreasley_1_ref_0, str_1_ref_0);
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
    let mut i32_13: i32 = crate::hp::ParryHotter::accio(parryhotter_1_ref_0, i32_3, i32_2);
    let mut i32_14: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_0_ref_0, usize_0, string_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2275() {
    rusty_monitor::set_test_id(2275);
    let mut i64_0: i64 = -7792i64;
    let mut i64_1: i64 = -35i64;
    let mut u16_0: u16 = 4661u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut u16_1: u16 = 8154u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut u16_2: u16 = 4005u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_2;
    let mut i32_0: i32 = -18165i32;
    let mut i32_1: i32 = -9085i32;
    let mut u64_0: u64 = 6715u64;
    let mut u64_1: u64 = 5089u64;
    let mut usize_0: usize = 6774usize;
    let mut i32_2: i32 = 4505i32;
    let mut i32_3: i32 = 3517i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u64_2: u64 = 5647u64;
    let mut u64_3: u64 = 6645u64;
    let mut i32_4: i32 = -2221i32;
    let mut i32_5: i32 = 12089i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_5, i32_4);
    crate::hp::ParryHotter::another_number_fn(u64_3, u64_2);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_0_ref_0, usize_0, u64_1, u64_0);
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_1, b: i32_0};
    crate::hp::RomTiddle::avada_kedavra(romtiddle_2_ref_0, string_0);
    crate::hp::RomTiddle::foo3(romtiddle_0_ref_0, i64_1, i64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2588() {
    rusty_monitor::set_test_id(2588);
    let mut u16_0: u16 = 9921u16;
    let mut i64_0: i64 = -7222i64;
    let mut i64_1: i64 = 11694i64;
    let mut u16_1: u16 = 7137u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_1};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut i64_2: i64 = 9770i64;
    let mut u16_2: u16 = 7281u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut i32_0: i32 = 9385i32;
    let mut i32_1: i32 = -2250i32;
    let mut i32_2: i32 = 1915i32;
    let mut i32_3: i32 = 5052i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_4: i32 = 1131i32;
    let mut i32_5: i32 = -4277i32;
    let mut i32_6: i32 = -13576i32;
    let mut i32_7: i32 = -13158i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_7, b: i32_6};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    crate::hp::ParryHotter::foo2(parryhotter_1_ref_0, i32_5, i32_4);
    let mut i32_8: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_1, i32_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_2};
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_0);
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2592() {
    rusty_monitor::set_test_id(2592);
    let mut i32_0: i32 = 3209i32;
    let mut i32_1: i32 = 1301i32;
    let mut i32_2: i32 = 11316i32;
    let mut i32_3: i32 = -4252i32;
    let mut i64_0: i64 = -1859i64;
    let mut i64_1: i64 = 5054i64;
    let mut u16_0: u16 = 5814u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_1};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut str_0: &str = "5ct7J6pcBZ2y";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_2: i64 = -10527i64;
    let mut u16_1: u16 = 5104u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_2};
    let mut wonreasley_1_ref_0: &crate::hp::WonReasley = &mut wonreasley_1;
    let mut i64_3: i64 = 413i64;
    let mut i64_4: i64 = -8833i64;
    let mut u16_2: u16 = 929u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    crate::hp::RomTiddle::foo3(romtiddle_2_ref_0, i64_4, i64_3);
    crate::hp::WonReasley::arania_exumai(wonreasley_1_ref_0, str_0_ref_0);
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_0);
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3418() {
    rusty_monitor::set_test_id(3418);
    let mut i32_0: i32 = -9614i32;
    let mut i32_1: i32 = -5227i32;
    let mut u16_0: u16 = 9347u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut i64_0: i64 = 19770i64;
    let mut i64_1: i64 = -4879i64;
    let mut u16_1: u16 = 3476u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut i64_2: i64 = -13970i64;
    let mut u16_2: u16 = 1548u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut i64_3: i64 = 14934i64;
    let mut i64_4: i64 = 1986i64;
    let mut u16_3: u16 = 6450u16;
    let mut i64_5: i64 = -1893i64;
    let mut u16_4: u16 = 9747u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_5};
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    crate::hp::RomTiddle::foo3(romtiddle_4_ref_0, i64_4, i64_3);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_2};
    crate::hp::RomTiddle::foo3(romtiddle_1_ref_0, i64_1, i64_0);
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4564() {
    rusty_monitor::set_test_id(4564);
    let mut u64_0: u64 = 9100u64;
    let mut u64_1: u64 = 2167u64;
    let mut usize_0: usize = 2715usize;
    let mut i32_0: i32 = -17268i32;
    let mut i32_1: i32 = 11110i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u64_2: u64 = 1267u64;
    let mut u64_3: u64 = 7190u64;
    let mut u16_0: u16 = 4235u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut usize_1: usize = 6535usize;
    let mut i32_2: i32 = -3520i32;
    let mut i32_3: i32 = 3051i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut u64_4: u64 = 4023u64;
    let mut u64_5: u64 = 8450u64;
    let mut usize_2: usize = 8498usize;
    let mut i32_4: i32 = -7806i32;
    let mut i32_5: i32 = -15716i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_5, i32_4);
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut u16_1: u16 = 7939u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_2_ref_0, usize_2, u64_5, u64_4);
    let mut i32_6: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_1_ref_0, usize_1, string_0);
    crate::hp::ParryHotter::another_number_fn(u64_3, u64_2);
    let mut bool_1: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_0_ref_0, usize_0, u64_1, u64_0);
    panic!("From RustyUnit with love");
}
}