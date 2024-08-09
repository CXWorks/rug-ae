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
#[timeout(30000)]fn rusty_test_1078() {
//    rusty_monitor::set_test_id(1078);
    let mut i64_0: i64 = 9223372036854775807i64;
    let mut u16_0: u16 = 5146u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut u64_0: u64 = 5468u64;
    let mut usize_0: usize = 4955usize;
    let mut i32_0: i32 = 8735i32;
    let mut i32_1: i32 = 1i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut i32_2: i32 = -13622i32;
    let mut i32_3: i32 = 140i32;
    let mut i32_4: i32 = 18616i32;
    let mut i32_5: i32 = 140i32;
    let mut i32_6: i32 = 3i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_2, i32_5);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_7: i32 = 20i32;
    let mut i32_8: i32 = 100i32;
    let mut i32_9: i32 = 7i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_7, i32_6);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut u64_1: u64 = 20u64;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_9, b: i32_3};
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_1_ref_0, usize_0, u64_0, u64_1);
    crate::hp::ParryHotter::foo2(parryhotter_2_ref_0, i32_4, i32_8);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7613() {
//    rusty_monitor::set_test_id(7613);
    let mut i32_0: i32 = 32i32;
    let mut i32_1: i32 = 20i32;
    let mut i32_2: i32 = -11804i32;
    let mut i32_3: i32 = 10i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i64_0: i64 = 111i64;
    let mut i32_4: i32 = 3i32;
    let mut i32_5: i32 = 3891i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_5, b: i32_4};
    let mut u16_0: u16 = 8455u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut u16_1: u16 = 2174u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut u16_2: u16 = 3u16;
    let mut i32_6: i32 = 100i32;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut i32_7: i32 = 111i32;
    let mut i32_8: i32 = -13622i32;
    let mut i32_9: i32 = 140i32;
    let mut i32_10: i32 = -9129i32;
    let mut i32_11: i32 = -11055i32;
    let mut i32_12: i32 = 18616i32;
    let mut i32_13: i32 = 140i32;
    let mut i32_14: i32 = 3i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_9, i32_11);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_15: i32 = 20i32;
    let mut i32_16: i32 = 100i32;
    let mut i32_17: i32 = 7i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_7, i32_13);
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i32_18: i32 = 15i32;
    let mut i32_19: i32 = 40i32;
    let mut parryhotter_4: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_8, b: i32_18};
    let mut parryhotter_3_ref_0: &crate::hp::ParryHotter = &mut parryhotter_3;
    crate::hp::ParryHotter::foo2(parryhotter_1_ref_0, i32_14, i32_12);
    crate::hp::ParryHotter::foo2(parryhotter_3_ref_0, i32_16, i32_15);
    crate::hp::ParryHotter::alohomora(i32_17, i32_10, i32_19, i32_6);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    crate::hp::ParryHotter::foo2(parryhotter_0_ref_0, i32_1, i32_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_59() {
//    rusty_monitor::set_test_id(59);
    let mut u16_0: u16 = 1u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut usize_0: usize = 8801usize;
    let mut i32_0: i32 = -2585i32;
    let mut i32_1: i32 = 13i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_1, b: i32_0};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i64_0: i64 = 100i64;
    let mut u16_1: u16 = 4010u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut i64_1: i64 = 3980i64;
    let mut u16_2: u16 = 7808u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut u64_0: u64 = 20u64;
    let mut u64_1: u64 = 20u64;
    let mut usize_1: usize = 4955usize;
    let mut i32_2: i32 = 3307i32;
    let mut i32_3: i32 = 111i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_1_ref_0, usize_1, u64_1, u64_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_1};
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_0};
    let mut i32_4: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_0_ref_0, usize_0, string_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5116() {
//    rusty_monitor::set_test_id(5116);
    let mut i32_0: i32 = 14036i32;
    let mut i32_1: i32 = 10423i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut u16_0: u16 = 3u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut i32_2: i32 = 13i32;
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut i32_3: i32 = -13622i32;
    let mut i32_4: i32 = 140i32;
    let mut i32_5: i32 = -9129i32;
    let mut i32_6: i32 = -11055i32;
    let mut i32_7: i32 = 3i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_5);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_8: i32 = -2230i32;
    let mut i32_9: i32 = 20i32;
    let mut i32_10: i32 = 100i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_6, i32_10);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_11: i32 = 15i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_8, b: i32_7};
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    crate::hp::ParryHotter::foo2(parryhotter_1_ref_0, i32_9, i32_2);
    crate::hp::ParryHotter::foo2(parryhotter_0_ref_0, i32_4, i32_11);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6048() {
//    rusty_monitor::set_test_id(6048);
    let mut i32_0: i32 = 10i32;
    let mut i32_1: i32 = 140i32;
    let mut i32_2: i32 = -4732i32;
    let mut i32_3: i32 = 3000i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u16_0: u16 = 1u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut u16_1: u16 = 1u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_1;
    let mut i32_4: i32 = 13i32;
    let mut i32_5: i32 = 9406i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_5, b: i32_4};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_6: i32 = 14715i32;
    let mut i32_7: i32 = 15i32;
    let mut u16_2: u16 = 1129u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut i64_0: i64 = 9223372036854775807i64;
    let mut i64_1: i64 = -22270i64;
    let mut u16_3: u16 = 3395u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    crate::hp::RomTiddle::foo3(romtiddle_3_ref_0, i64_1, i64_0);
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_7, b: i32_6};
    crate::hp::ParryHotter::foo2(parryhotter_0_ref_0, i32_1, i32_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3995() {
//    rusty_monitor::set_test_id(3995);
    let mut str_0: &str = "OUYDScL";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_0: i64 = 0i64;
    let mut i64_1: i64 = 0i64;
    let mut u16_0: u16 = 7850u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_1};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut i32_0: i32 = 111i32;
    let mut i32_1: i32 = -2462i32;
    let mut u16_1: u16 = 1u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut usize_0: usize = 184usize;
    let mut i32_2: i32 = 1i32;
    let mut i32_3: i32 = 11i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_4: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_0_ref_0, usize_0, string_1);
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_1, b: i32_0};
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_155() {
//    rusty_monitor::set_test_id(155);
    let mut i32_0: i32 = 111i32;
    let mut i32_1: i32 = 21801i32;
    let mut i32_2: i32 = 111i32;
    let mut i32_3: i32 = 32i32;
    let mut i32_4: i32 = 7i32;
    let mut i32_5: i32 = -3945i32;
    let mut i32_6: i32 = 20i32;
    let mut i32_7: i32 = -2835i32;
    let mut i32_8: i32 = 4974i32;
    let mut i32_9: i32 = 342i32;
    let mut i32_10: i32 = 11249i32;
    let mut i32_11: i32 = 7626i32;
    let mut i32_12: i32 = 3000i32;
    let mut i32_13: i32 = -14855i32;
    let mut i32_14: i32 = 11i32;
    let mut i32_15: i32 = 100i32;
    let mut i32_16: i32 = 884i32;
    let mut i32_17: i32 = -11086i32;
    let mut i32_18: i32 = 111i32;
    let mut i32_19: i32 = 1i32;
    let mut i32_20: i32 = -17170i32;
    let mut i32_21: i32 = 6987i32;
    let mut i32_22: i32 = 32i32;
    let mut i32_23: i32 = 8642i32;
    let mut i32_24: i32 = 7i32;
    let mut i32_25: i32 = 7i32;
    let mut i32_26: i32 = 617i32;
    let mut i32_27: i32 = 7i32;
    crate::hp::ParryHotter::alohomora(i32_27, i32_26, i32_25, i32_24);
    crate::hp::ParryHotter::alohomora(i32_23, i32_22, i32_21, i32_20);
    crate::hp::ParryHotter::alohomora(i32_19, i32_18, i32_17, i32_16);
    crate::hp::ParryHotter::alohomora(i32_15, i32_14, i32_13, i32_12);
    crate::hp::ParryHotter::alohomora(i32_11, i32_10, i32_9, i32_8);
    crate::hp::ParryHotter::alohomora(i32_7, i32_6, i32_5, i32_4);
    crate::hp::ParryHotter::alohomora(i32_3, i32_2, i32_1, i32_0);
//    panic!("From RustyUnit with love");
}
}