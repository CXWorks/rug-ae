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
#[timeout(30000)]fn rusty_test_67() {
//    rusty_monitor::set_test_id(67);
    let mut u16_0: u16 = 3u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut u16_1: u16 = 8694u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_1;
    let mut i64_0: i64 = -1995i64;
    let mut i64_1: i64 = -5024i64;
    let mut u16_2: u16 = 6538u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_1};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut i64_2: i64 = 13991i64;
    let mut i64_3: i64 = 2409i64;
    let mut u16_3: u16 = 3u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut i64_4: i64 = 9426i64;
    let mut u16_4: u16 = 2653u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    let mut i32_0: i32 = -6960i32;
    let mut i32_1: i32 = 15i32;
    let mut i32_2: i32 = 628i32;
    let mut i32_3: i32 = 7i32;
    crate::hp::ParryHotter::alohomora(i32_3, i32_2, i32_1, i32_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_4};
    crate::hp::RomTiddle::foo3(romtiddle_3_ref_0, i64_3, i64_2);
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_0);
    crate::hp::RomTiddle::avada_kedavra(romtiddle_1_ref_0, string_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4717() {
//    rusty_monitor::set_test_id(4717);
    let mut i64_0: i64 = -25497i64;
    let mut i64_1: i64 = 100i64;
    let mut u16_0: u16 = 3u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_1};
    let mut u16_1: u16 = 7487u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut u16_2: u16 = 1u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut u16_3: u16 = 7446u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut u16_4: u16 = 9675u16;
    let mut str_0: &str = "Stg8uxx3";
    let mut i32_0: i32 = 40i32;
    let mut i32_1: i32 = 140i32;
    let mut i32_2: i32 = 111i32;
    let mut i32_3: i32 = 32i32;
    let mut i32_4: i32 = 3i32;
    let mut i32_5: i32 = 1877i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_5, i32_4);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut str_1: &str = "1iy56EArmAVOTjQMi";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut i64_2: i64 = -802i64;
    let mut u16_5: u16 = 1u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_2};
    let mut wonreasley_1_ref_0: &crate::hp::WonReasley = &mut wonreasley_1;
    let mut i64_3: i64 = -3616i64;
    let mut i64_4: i64 = -3868i64;
    let mut u16_6: u16 = 7424u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    let mut romtiddle_5_ref_0: &crate::hp::RomTiddle = &mut romtiddle_5;
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_5_ref_0);
    let mut wonreasley_2: crate::hp::WonReasley = crate::hp::WonReasley {x: string_3, y: i64_4};
    let mut wonreasley_2_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_2;
    let mut i32_6: i32 = 13i32;
    let mut i32_7: i32 = 111i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_7, i32_6);
    crate::hp::WonReasley::ascendio(wonreasley_2_ref_0, i64_3);
    crate::hp::WonReasley::arania_exumai(wonreasley_1_ref_0, str_1_ref_0);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i64_5: i64 = 111i64;
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u16_7: u16 = 1u16;
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_4: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut wonreasley_3: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_5};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    let mut u64_0: u64 = 20u64;
    let mut u64_1: u64 = 20u64;
    let mut u16_8: u16 = 3u16;
    let mut romtiddle_7: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_7};
    let mut romtiddle_6_ref_0: &crate::hp::RomTiddle = &mut romtiddle_6;
    let mut string_5: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut u16_9: u16 = 3u16;
    let mut romtiddle_8: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_8};
    let mut romtiddle_7_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_7;
    let mut u64_2: u64 = 5009u64;
    let mut u64_3: u64 = 6614u64;
    let mut u16_10: u16 = 1u16;
    let mut romtiddle_9: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_9};
    let mut romtiddle_8_ref_0: &crate::hp::RomTiddle = &mut romtiddle_8;
    let mut i64_6: i64 = 0i64;
    let mut i64_7: i64 = -4952i64;
    let mut u16_11: u16 = 1u16;
    let mut romtiddle_10: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_10};
    let mut romtiddle_9_ref_0: &crate::hp::RomTiddle = &mut romtiddle_9;
    let mut string_6: std::string::String = crate::hp::RomTiddle::name(romtiddle_6_ref_0);
    let mut wonreasley_4: crate::hp::WonReasley = crate::hp::WonReasley {x: string_4, y: i64_6};
    let mut wonreasley_3_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_3;
    let mut romtiddle_11: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_11};
    let mut string_7: std::string::String = crate::hp::RomTiddle::name(romtiddle_8_ref_0);
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_2);
    crate::hp::RomTiddle::avada_kedavra(romtiddle_7_ref_0, string_6);
    crate::hp::ParryHotter::another_number_fn(u64_3, u64_0);
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
    crate::hp::ParryHotter::foo2(parryhotter_1_ref_0, i32_1, i32_0);
    let mut romtiddle_11_ref_0: &crate::hp::RomTiddle = &mut romtiddle_11;
    crate::hp::RomTiddle::foo3(romtiddle_9_ref_0, i64_7, i64_0);
    let mut i32_8: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_3, i32_2);
    let mut romtiddle_10_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_10;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_364() {
//    rusty_monitor::set_test_id(364);
    let mut i32_0: i32 = 16041i32;
    let mut i32_1: i32 = 10i32;
    let mut i32_2: i32 = 20i32;
    let mut i32_3: i32 = 113i32;
    let mut i32_4: i32 = 15i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_4, i32_3);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_5: i32 = 23059i32;
    let mut i32_6: i32 = 106i32;
    let mut i32_7: i32 = -622i32;
    let mut i32_8: i32 = -13462i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_8, i32_7);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_9: i32 = 6346i32;
    let mut i32_10: i32 = 4173i32;
    let mut i32_11: i32 = 111i32;
    let mut i32_12: i32 = 40i32;
    let mut u16_0: u16 = 8510u16;
    let mut u64_0: u64 = 20u64;
    let mut u64_1: u64 = 6280u64;
    let mut usize_0: usize = 4955usize;
    let mut i32_13: i32 = 7i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_0, b: i32_13};
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i32_14: i32 = 11i32;
    let mut i32_15: i32 = 3745i32;
    let mut i32_16: i32 = 100i32;
    let mut i32_17: i32 = -12589i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_17, b: i32_16};
    let mut parryhotter_3_ref_0: &crate::hp::ParryHotter = &mut parryhotter_3;
    let mut i32_18: i32 = crate::hp::ParryHotter::accio(parryhotter_3_ref_0, i32_15, i32_14);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_2_ref_0, usize_0, u64_1, u64_0);
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    crate::hp::ParryHotter::alohomora(i32_12, i32_11, i32_10, i32_9);
    let mut i32_19: i32 = crate::hp::ParryHotter::accio(parryhotter_1_ref_0, i32_6, i32_5);
    let mut i32_20: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_2, i32_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3835() {
//    rusty_monitor::set_test_id(3835);
    let mut u16_0: u16 = 1u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut i32_0: i32 = 32i32;
    let mut i32_1: i32 = 10677i32;
    let mut u64_0: u64 = 2796u64;
    let mut u64_1: u64 = 20u64;
    let mut usize_0: usize = 3172usize;
    let mut i32_2: i32 = 4909i32;
    let mut i32_3: i32 = 32i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u16_1: u16 = 5636u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut i32_4: i32 = -14131i32;
    let mut i32_5: i32 = 3000i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_5, b: i32_4};
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_0_ref_0, usize_0, u64_1, u64_0);
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7000() {
//    rusty_monitor::set_test_id(7000);
    let mut i32_0: i32 = 3000i32;
    let mut i32_1: i32 = 40i32;
    let mut i32_2: i32 = 10i32;
    let mut i32_3: i32 = 18736i32;
    let mut str_0: &str = "4ZV6T2QSDCKW5oSAu0";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_0: i64 = -800i64;
    let mut u16_0: u16 = 242u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    let mut u16_1: u16 = 3u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut u16_2: u16 = 1u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut i32_4: i32 = -871i32;
    let mut i32_5: i32 = -585i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_5, i32_4);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_6: i32 = 140i32;
    let mut i32_7: i32 = 1i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_7, i32_6);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_8: i32 = 140i32;
    let mut i32_9: i32 = -137i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_9, b: i32_8};
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i64_1: i64 = 7012i64;
    let mut i32_10: i32 = 19949i32;
    let mut i32_11: i32 = 20i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_11, i32_10);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_1};
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
    crate::hp::ParryHotter::alohomora(i32_3, i32_2, i32_1, i32_0);
//    panic!("From RustyUnit with love");
}
}