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

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_235() {
    rusty_monitor::set_test_id(235);
    let mut u16_0: u16 = 1367u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut u16_1: u16 = 6258u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut u16_2: u16 = 49u16;
    let mut u64_0: u64 = 85u64;
    let mut i32_0: i32 = -1358i32;
    let mut i32_1: i32 = 912i32;
    let mut i32_2: i32 = -3691i32;
    let mut i32_3: i32 = -12175i32;
    let mut u64_1: u64 = 9053u64;
    let mut u16_3: u16 = 2626u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut u16_4: u16 = 1483u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_2_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_2;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    crate::hp::RomTiddle::avada_kedavra(romtiddle_2_ref_0, string_0);
    crate::hp::ParryHotter::another_number_fn(u64_0, u64_1);
    crate::hp::ParryHotter::alohomora(i32_1, i32_2, i32_3, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6477() {
    rusty_monitor::set_test_id(6477);
    let mut str_0: &str = "dsKGPljUMswgIlP3";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u16_0: u16 = 3065u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut i32_0: i32 = -4347i32;
    let mut i32_1: i32 = 4440i32;
    let mut i32_2: i32 = 4964i32;
    let mut i64_0: i64 = -6956i64;
    let mut u16_1: u16 = 5156u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut u16_2: u16 = 7038u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_2;
    let mut i64_1: i64 = 8315i64;
    let mut u16_3: u16 = 783u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_1};
    let mut i32_3: i32 = 4363i32;
    let mut i32_4: i32 = -11005i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_4, i32_3);
    let mut u16_4: u16 = 7754u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut u16_5: u16 = 5585u16;
    let mut i32_5: i32 = -8972i32;
    let mut i32_6: i32 = -3719i32;
    let mut i32_7: i32 = -4088i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_7, i32_6);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut u16_6: u16 = 2626u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut u16_7: u16 = 44u16;
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    let mut romtiddle_5_ref_0: &crate::hp::RomTiddle = &mut romtiddle_5;
    let mut i32_8: i32 = -5313i32;
    let mut i32_9: i32 = 3265i32;
    let mut i32_10: i32 = 5479i32;
    let mut u64_0: u64 = 2552u64;
    let mut u64_1: u64 = 1975u64;
    let mut u16_8: u16 = 2589u16;
    let mut romtiddle_7: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_7};
    let mut romtiddle_6_ref_0: &crate::hp::RomTiddle = &mut romtiddle_6;
    let mut romtiddle_8: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_8};
    let mut u16_9: u16 = 5136u16;
    let mut str_1: &str = "UdPYR2yWwdENv6YIS";
    let mut i32_11: i32 = 13696i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_1, b: i32_8};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut str_1_ref_0: &str = &mut str_1;
    let mut romtiddle_9: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_9};
    let mut romtiddle_7_ref_0: &crate::hp::RomTiddle = &mut romtiddle_7;
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_0};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    let mut i32_12: i32 = -1810i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_12, i32_0);
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    crate::hp::ParryHotter::foo2(parryhotter_0_ref_0, i32_5, i32_9);
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
    let mut string_4: std::string::String = crate::hp::RomTiddle::name(romtiddle_7_ref_0);
    crate::hp::RomTiddle::avada_kedavra(romtiddle_2_ref_0, string_3);
    let mut romtiddle_9_ref_0: &crate::hp::RomTiddle = &mut romtiddle_9;
    let mut string_5: std::string::String = crate::hp::RomTiddle::name(romtiddle_5_ref_0);
    let mut i32_13: i32 = crate::hp::ParryHotter::accio(parryhotter_1_ref_0, i32_10, i32_2);
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    let mut wonreasley_1_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_1;
    let mut parryhotter_3_ref_0: &crate::hp::ParryHotter = &mut parryhotter_3;
    let mut i32_14: i32 = crate::hp::ParryHotter::accio(parryhotter_2_ref_0, i32_13, i32_11);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6932() {
    rusty_monitor::set_test_id(6932);
    let mut u16_0: u16 = 2u16;
    let mut u16_1: u16 = 1673u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut u16_2: u16 = 4440u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut u16_3: u16 = 8016u16;
    let mut u16_4: u16 = 4954u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut u16_5: u16 = 4430u16;
    let mut u16_6: u16 = 5431u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    let mut i32_0: i32 = -22410i32;
    let mut i32_1: i32 = 9362i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_2: i32 = -3640i32;
    let mut i32_3: i32 = 11468i32;
    let mut i32_4: i32 = -288i32;
    let mut i32_5: i32 = -6024i32;
    let mut i32_6: i32 = -696i32;
    let mut i64_0: i64 = 3580i64;
    let mut i64_1: i64 = -6956i64;
    let mut u16_7: u16 = 5156u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_7};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut u16_8: u16 = 7038u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_8};
    let mut romtiddle_5_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_5;
    let mut i64_2: i64 = 8315i64;
    let mut u16_9: u16 = 783u16;
    let mut i64_0: i64 = 3580i64;
    let mut i64_1: i64 = -6956i64;
    let mut u16_7: u16 = 5156u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_9};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut u16_8: u16 = 7038u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_5_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_2;
    let mut i64_2: i64 = 8315i64;
    let mut u16_9: u16 = 783u16;
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut i32_7: i32 = 4363i32;
    let mut i32_8: i32 = -11005i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_2, i32_4);
    let mut u16_10: u16 = 7754u16;
    let mut romtiddle_7: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut u16_11: u16 = 5585u16;
    let mut i32_9: i32 = -3719i32;
    let mut i32_10: i32 = -4088i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_9, i32_7);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i64_3: i64 = -2401i64;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut u16_12: u16 = 44u16;
    let mut romtiddle_8: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_10};
    let mut romtiddle_7_ref_0: &crate::hp::RomTiddle = &mut romtiddle_7;
    let mut i32_11: i32 = 5479i32;
    let mut i32_12: i32 = 4600i32;
    let mut u16_13: u16 = 2589u16;
    let mut romtiddle_9: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_12};
    let mut romtiddle_8_ref_0: &crate::hp::RomTiddle = &mut romtiddle_8;
    let mut romtiddle_10: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_13};
    let mut str_0: &str = "UdPYR2yWwdENv6YIS";
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_8, b: i32_11};
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut str_0_ref_0: &str = &mut str_0;
    let mut romtiddle_11: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_11};
    let mut romtiddle_6_ref_0: &crate::hp::RomTiddle = &mut romtiddle_6;
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_8_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_3};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    let mut i32_13: i32 = -2263i32;
    let mut parryhotter_3_ref_0: &crate::hp::ParryHotter = &mut parryhotter_3;
    crate::hp::ParryHotter::foo2(parryhotter_1_ref_0, i32_13, i32_10);
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
    let mut string_4: std::string::String = crate::hp::RomTiddle::name(romtiddle_7_ref_0);
    crate::hp::RomTiddle::avada_kedavra(romtiddle_5_ref_0, string_2);
    let mut romtiddle_11_ref_0: &crate::hp::RomTiddle = &mut romtiddle_11;
    let mut string_5: std::string::String = crate::hp::RomTiddle::name(romtiddle_6_ref_0);
    let mut i32_14: i32 = crate::hp::ParryHotter::accio(parryhotter_2_ref_0, i32_6, i32_12);
    crate::hp::RomTiddle::foo3(romtiddle_11_ref_0, i64_2, i64_1);
    let mut i32_15: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_3, i32_5);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_419() {
    rusty_monitor::set_test_id(419);
    let mut i64_0: i64 = 1081i64;
    let mut i32_0: i32 = 14540i32;
    let mut i32_1: i32 = 5213i32;
    let mut i32_2: i32 = -3514i32;
    let mut i32_3: i32 = 11336i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i64_1: i64 = 8564i64;
    let mut u16_0: u16 = 7674u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut i64_2: i64 = 2913i64;
    let mut u16_1: u16 = 9673u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut str_0: &str = "4tTaYzu8SQY";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_2};
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_1};
    let mut i32_4: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_1, i32_0);
    let mut wonreasley_1_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_1;
    crate::hp::WonReasley::ascendio(wonreasley_1_ref_0, i64_0);
    panic!("From RustyUnit with love");
}
}