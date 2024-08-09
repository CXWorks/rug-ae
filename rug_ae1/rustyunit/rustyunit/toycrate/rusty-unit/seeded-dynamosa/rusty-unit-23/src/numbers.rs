struct PerfectNumber {
    sum: i32
}

impl PerfectNumber {
    fn is_perfect(&mut self, n: i32, div: i32) -> bool {
        let result = self.find_perfect(n, div);
        result == n
    }

    fn find_perfect(&mut self, n: i32, mut div: i32) -> i32 {
        if div <= n / 2 {
            if n % div == 0 {
                self.sum += div;
            }
            div += 1;
            self.find_perfect(n, div);
        }
        return self.sum;
    }
}
#[cfg(test)]
mod rusty_tests {
	use crate::*;

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6310() {
//    rusty_monitor::set_test_id(6310);
    let mut i64_0: i64 = -11204i64;
    let mut u16_0: u16 = 6763u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut u16_1: u16 = 1u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut u16_2: u16 = 3u16;
    let mut i64_1: i64 = 5610i64;
    let mut i32_0: i32 = -8887i32;
    let mut i32_1: i32 = 11i32;
    let mut i32_2: i32 = 2968i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_2, i32_1);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_3: i32 = 1i32;
    let mut i32_4: i32 = 13i32;
    let mut i32_5: i32 = 32i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_5, b: i32_4};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut str_0: &str = "Rom Tiddle";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_2: i64 = 15599i64;
    let mut u16_3: u16 = 3u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_2};
    let mut wonreasley_1_ref_0: &crate::hp::WonReasley = &mut wonreasley_1;
    let mut u64_0: u64 = 20u64;
    let mut u64_1: u64 = 8780u64;
    let mut usize_0: usize = 8386usize;
    let mut i32_6: i32 = 1i32;
    let mut i32_7: i32 = -10825i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_7, b: i32_6};
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut u16_4: u16 = 1u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut i32_8: i32 = 1230i32;
    let mut i32_9: i32 = 100i32;
    let mut i32_10: i32 = -7926i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_10, i32_9);
    let mut parryhotter_3_ref_0: &crate::hp::ParryHotter = &mut parryhotter_3;
    let mut u16_5: u16 = 1u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut usize_1: usize = 4955usize;
    let mut i32_11: i32 = 32i32;
    let mut i32_12: i32 = 7i32;
    let mut parryhotter_4: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_12, i32_11);
    let mut parryhotter_4_ref_0: &crate::hp::ParryHotter = &mut parryhotter_4;
    let mut i32_13: i32 = -1795i32;
    let mut i32_14: i32 = -5815i32;
    let mut i32_15: i32 = 20i32;
    let mut i32_16: i32 = 7i32;
    let mut u16_6: u16 = 1354u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    let mut i64_3: i64 = -11216i64;
    let mut u16_7: u16 = 1u16;
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut wonreasley_2: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_3};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut u16_8: u16 = 3u16;
    let mut romtiddle_7: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_7};
    let mut romtiddle_5_ref_0: &crate::hp::RomTiddle = &mut romtiddle_5;
    let mut u16_9: u16 = 1u16;
    let mut romtiddle_8: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_8};
    let mut romtiddle_7_ref_0: &crate::hp::RomTiddle = &mut romtiddle_7;
    let mut string_4: std::string::String = crate::hp::RomTiddle::name(romtiddle_5_ref_0);
    let mut romtiddle_9: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_9};
    let mut romtiddle_6_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_6;
    let mut string_5: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_1);
    crate::hp::ParryHotter::alohomora(i32_16, i32_3, i32_13, i32_14);
    let mut i32_17: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_1_ref_0, usize_0, string_3);
    let mut i32_18: i32 = crate::hp::ParryHotter::accio(parryhotter_4_ref_0, i32_0, i32_8);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_0_ref_0, usize_1, u64_0, u64_1);
    crate::hp::WonReasley::arania_exumai(wonreasley_1_ref_0, str_0_ref_0);
    let mut i32_19: i32 = crate::hp::ParryHotter::accio(parryhotter_2_ref_0, i32_15, i32_18);
    let mut romtiddle_9_ref_0: &crate::hp::RomTiddle = &mut romtiddle_9;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_621() {
//    rusty_monitor::set_test_id(621);
    let mut i32_0: i32 = 40i32;
    let mut i32_1: i32 = 1i32;
    let mut u64_0: u64 = 4529u64;
    let mut u64_1: u64 = 20u64;
    let mut usize_0: usize = 7158usize;
    let mut i32_2: i32 = 11i32;
    let mut i32_3: i32 = 111i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u64_2: u64 = 3993u64;
    let mut u64_3: u64 = 944u64;
    let mut i64_0: i64 = 111i64;
    let mut i64_1: i64 = 3700i64;
    let mut str_0: &str = "lDuVJvv";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u16_0: u16 = 7190u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut u64_4: u64 = 4955u64;
    let mut u64_5: u64 = 9143u64;
    let mut i64_2: i64 = 3700i64;
    let mut u16_1: u16 = 3u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut u16_2: u16 = 975u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_2;
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_2};
    crate::hp::ParryHotter::another_number_fn(u64_5, u64_4);
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    crate::hp::RomTiddle::foo3(romtiddle_0_ref_0, i64_1, i64_0);
    crate::hp::ParryHotter::another_number_fn(u64_3, u64_2);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_0_ref_0, usize_0, u64_1, u64_0);
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7531() {
//    rusty_monitor::set_test_id(7531);
    let mut i64_0: i64 = 0i64;
    let mut i64_1: i64 = 9223372036854775807i64;
    let mut u16_0: u16 = 5955u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_1};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut i32_0: i32 = 32i32;
    let mut i32_1: i32 = -363i32;
    let mut i32_2: i32 = 3000i32;
    let mut i32_3: i32 = 1i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_4: i32 = 3000i32;
    let mut i32_5: i32 = 20i32;
    let mut i32_6: i32 = 3000i32;
    let mut i32_7: i32 = 3708i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_7, i32_6);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_8: i32 = crate::hp::ParryHotter::accio(parryhotter_1_ref_0, i32_5, i32_4);
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_1, b: i32_0};
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_163() {
//    rusty_monitor::set_test_id(163);
    let mut i32_0: i32 = 3i32;
    let mut i32_1: i32 = -2521i32;
    let mut i32_2: i32 = 100i32;
    let mut i32_3: i32 = -11241i32;
    let mut i32_4: i32 = 13i32;
    let mut i32_5: i32 = -4035i32;
    let mut i32_6: i32 = -1205i32;
    let mut i32_7: i32 = 6804i32;
    let mut i32_8: i32 = 3000i32;
    let mut i32_9: i32 = -24698i32;
    let mut i32_10: i32 = 10i32;
    let mut i32_11: i32 = 15i32;
    let mut i32_12: i32 = 3000i32;
    let mut i32_13: i32 = 1i32;
    let mut i32_14: i32 = 13i32;
    let mut i32_15: i32 = 40i32;
    let mut i32_16: i32 = 7i32;
    let mut i32_17: i32 = 3i32;
    let mut i32_18: i32 = 111i32;
    let mut i32_19: i32 = 3i32;
    let mut i32_20: i32 = 2359i32;
    let mut i32_21: i32 = -5162i32;
    let mut i32_22: i32 = 7i32;
    let mut i32_23: i32 = -4286i32;
    let mut i32_24: i32 = -6374i32;
    let mut i32_25: i32 = 3i32;
    let mut i32_26: i32 = 3000i32;
    let mut i32_27: i32 = -3882i32;
    crate::hp::ParryHotter::alohomora(i32_27, i32_26, i32_25, i32_24);
    crate::hp::ParryHotter::alohomora(i32_23, i32_22, i32_21, i32_20);
    crate::hp::ParryHotter::alohomora(i32_19, i32_18, i32_17, i32_16);
    crate::hp::ParryHotter::alohomora(i32_15, i32_14, i32_13, i32_12);
    crate::hp::ParryHotter::alohomora(i32_11, i32_10, i32_9, i32_8);
    crate::hp::ParryHotter::alohomora(i32_7, i32_6, i32_5, i32_4);
    crate::hp::ParryHotter::alohomora(i32_3, i32_2, i32_1, i32_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8493() {
//    rusty_monitor::set_test_id(8493);
    let mut i32_0: i32 = 1i32;
    let mut i32_1: i32 = 100i32;
    let mut i32_2: i32 = 3i32;
    let mut i32_3: i32 = -174i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    let mut u16_0: u16 = 5955u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut i32_4: i32 = 32i32;
    let mut i32_5: i32 = 100i32;
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_6: i32 = 20i32;
    let mut i32_7: i32 = 3000i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_7, i32_5);
    let mut i32_8: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_6, i32_4);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    crate::hp::ParryHotter::foo2(parryhotter_1_ref_0, i32_1, i32_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7568() {
//    rusty_monitor::set_test_id(7568);
    let mut u16_0: u16 = 3u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut i64_0: i64 = -11216i64;
    let mut i64_1: i64 = 100i64;
    let mut u16_1: u16 = 1u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_1};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut u16_2: u16 = 3u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut u16_3: u16 = 1u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut u16_4: u16 = 1u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_4_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_4;
    crate::hp::RomTiddle::avada_kedavra(romtiddle_4_ref_0, string_1);
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_0);
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
//    panic!("From RustyUnit with love");
}
}