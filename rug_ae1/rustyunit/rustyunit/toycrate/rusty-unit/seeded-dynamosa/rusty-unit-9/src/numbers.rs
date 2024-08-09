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
#[timeout(30000)]fn rusty_test_197() {
//    rusty_monitor::set_test_id(197);
    let mut u16_0: u16 = 1u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut u16_1: u16 = 9214u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut u16_2: u16 = 1u16;
    let mut u16_3: u16 = 2671u16;
    let mut str_0: &str = "Lord Voldemort";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_0: i64 = 5392i64;
    let mut u16_4: u16 = 9841u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1648() {
//    rusty_monitor::set_test_id(1648);
    let mut u64_0: u64 = 20u64;
    let mut u16_0: u16 = 1u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut str_0: &str = "at";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u16_1: u16 = 8465u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut u16_2: u16 = 1u16;
    let mut i32_0: i32 = 4350i32;
    let mut i32_1: i32 = 10405i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut u64_1: u64 = 4217u64;
    let mut u64_2: u64 = 20u64;
    let mut u64_3: u64 = 7131u64;
    let mut usize_0: usize = 10usize;
    let mut i32_2: i32 = 3i32;
    let mut i32_3: i32 = 13i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_1_ref_0, usize_0, u64_0, u64_3);
    crate::hp::ParryHotter::another_number_fn(u64_2, u64_1);
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8095() {
//    rusty_monitor::set_test_id(8095);
    let mut i32_0: i32 = 140i32;
    let mut i32_1: i32 = 12484i32;
    let mut u64_0: u64 = 20u64;
    let mut u64_1: u64 = 1455u64;
    let mut u16_0: u16 = 1u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut i32_2: i32 = -2090i32;
    let mut i32_3: i32 = 40i32;
    let mut i32_4: i32 = 32i32;
    let mut i32_5: i32 = 140i32;
    let mut i32_6: i32 = 20i32;
    let mut i32_7: i32 = 7i32;
    let mut i32_8: i32 = 11i32;
    let mut i32_9: i32 = -18442i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_9, b: i32_8};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_10: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_7, i32_6);
    crate::hp::ParryHotter::alohomora(i32_5, i32_2, i32_4, i32_3);
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_156() {
//    rusty_monitor::set_test_id(156);
    let mut u64_0: u64 = 1038u64;
    let mut u64_1: u64 = 4425u64;
    let mut u64_2: u64 = 7301u64;
    let mut u64_3: u64 = 3193u64;
    let mut u64_4: u64 = 7812u64;
    let mut u64_5: u64 = 20u64;
    let mut u64_6: u64 = 7845u64;
    let mut u64_7: u64 = 2117u64;
    let mut u64_8: u64 = 7895u64;
    let mut u64_9: u64 = 20u64;
    let mut u64_10: u64 = 20u64;
    let mut u64_11: u64 = 746u64;
    let mut u64_12: u64 = 20u64;
    let mut u64_13: u64 = 20u64;
    let mut u64_14: u64 = 7485u64;
    let mut u64_15: u64 = 20u64;
    let mut u64_16: u64 = 20u64;
    let mut u64_17: u64 = 8487u64;
    let mut u64_18: u64 = 1225u64;
    let mut u64_19: u64 = 8391u64;
    let mut u64_20: u64 = 1717u64;
    let mut u64_21: u64 = 2274u64;
    crate::hp::ParryHotter::another_number_fn(u64_21, u64_20);
    crate::hp::ParryHotter::another_number_fn(u64_19, u64_18);
    crate::hp::ParryHotter::another_number_fn(u64_17, u64_16);
    crate::hp::ParryHotter::another_number_fn(u64_15, u64_14);
    crate::hp::ParryHotter::another_number_fn(u64_13, u64_12);
    crate::hp::ParryHotter::another_number_fn(u64_11, u64_10);
    crate::hp::ParryHotter::another_number_fn(u64_9, u64_8);
    crate::hp::ParryHotter::another_number_fn(u64_7, u64_6);
    crate::hp::ParryHotter::another_number_fn(u64_5, u64_4);
    crate::hp::ParryHotter::another_number_fn(u64_3, u64_2);
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6286() {
//    rusty_monitor::set_test_id(6286);
    let mut u16_0: u16 = 3u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut u16_1: u16 = 1u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_1;
    let mut i64_0: i64 = 3700i64;
    let mut i64_1: i64 = 100i64;
    let mut u16_2: u16 = 1u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut u16_3: u16 = 1u16;
    let mut u16_4: u16 = 1u16;
    let mut i32_0: i32 = 11i32;
    let mut i32_1: i32 = 10538i32;
    let mut i32_2: i32 = -8279i32;
    let mut i32_3: i32 = 15i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    crate::hp::ParryHotter::foo2(parryhotter_0_ref_0, i32_1, i32_0);
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    crate::hp::RomTiddle::foo3(romtiddle_2_ref_0, i64_1, i64_0);
    crate::hp::RomTiddle::avada_kedavra(romtiddle_1_ref_0, string_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_217() {
//    rusty_monitor::set_test_id(217);
    let mut str_0: &str = "Lord Voldemort";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u16_0: u16 = 4046u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut u16_1: u16 = 3u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut i32_0: i32 = 32i32;
    let mut i32_1: i32 = 40i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut str_1: &str = "xfATvPoxM0qpT4";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut i32_2: i32 = -1274i32;
    let mut i32_3: i32 = -6368i32;
    let mut i32_4: i32 = 100i32;
    let mut i32_5: i32 = -8982i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_5, b: i32_4};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_6: i32 = crate::hp::ParryHotter::accio(parryhotter_1_ref_0, i32_3, i32_2);
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
//    panic!("From RustyUnit with love");
}
}