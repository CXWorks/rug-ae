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
#[timeout(30000)]fn rusty_test_1187() {
//    rusty_monitor::set_test_id(1187);
    let mut u16_0: u16 = 1u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut u16_1: u16 = 5725u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut u16_2: u16 = 3u16;
    let mut i32_0: i32 = 10i32;
    let mut i32_1: i32 = 111i32;
    let mut i32_2: i32 = 910i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_2, i32_1);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_3: i32 = -3589i32;
    let mut i32_4: i32 = 20i32;
    let mut u16_3: u16 = 5342u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_2_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_2;
    let mut i32_5: i32 = 40i32;
    let mut i32_6: i32 = -8783i32;
    let mut i32_7: i32 = 3i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_5, i32_0);
    crate::hp::ParryHotter::alohomora(i32_7, i32_3, i32_4, i32_6);
    crate::hp::RomTiddle::avada_kedavra(romtiddle_2_ref_0, string_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1195() {
//    rusty_monitor::set_test_id(1195);
    let mut i32_0: i32 = 111i32;
    let mut i32_1: i32 = 15i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut str_0: &str = "juyv";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i32_2: i32 = 13i32;
    let mut i32_3: i32 = 2i32;
    let mut i32_4: i32 = 15i32;
    let mut i32_5: i32 = 140i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_5, b: i32_4};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut u16_0: u16 = 7620u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut i32_6: i32 = 111i32;
    let mut i32_7: i32 = 12808i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_6, i32_3);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_8: i32 = -5867i32;
    let mut i32_9: i32 = 100i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_2, b: i32_7};
    crate::hp::ParryHotter::foo2(parryhotter_1_ref_0, i32_8, i32_9);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_94() {
//    rusty_monitor::set_test_id(94);
    let mut i64_0: i64 = -9336i64;
    let mut u16_0: u16 = 4989u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut u16_1: u16 = 7708u16;
    let mut i32_0: i32 = 40i32;
    let mut i32_1: i32 = 7i32;
    let mut i32_2: i32 = 13i32;
    let mut i32_3: i32 = 100i32;
    let mut u16_2: u16 = 1u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut usize_0: usize = 4955usize;
    let mut i32_4: i32 = 100i32;
    let mut i32_5: i32 = 140i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_5, b: i32_4};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u64_0: u64 = 20u64;
    let mut u64_1: u64 = 20u64;
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    let mut i32_6: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_0_ref_0, usize_0, string_1);
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_7: i32 = crate::hp::ParryHotter::accio(parryhotter_1_ref_0, i32_1, i32_0);
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_87() {
//    rusty_monitor::set_test_id(87);
    let mut i32_0: i32 = 140i32;
    let mut i32_1: i32 = 40i32;
    let mut i32_2: i32 = 100i32;
    let mut i32_3: i32 = 32i32;
    let mut u64_0: u64 = 8498u64;
    let mut u64_1: u64 = 8701u64;
    let mut i32_4: i32 = -233i32;
    let mut i32_5: i32 = 13i32;
    let mut i32_6: i32 = 13i32;
    let mut i32_7: i32 = 7i32;
    let mut i32_8: i32 = 1i32;
    let mut i32_9: i32 = 6154i32;
    let mut i32_10: i32 = -7141i32;
    let mut i32_11: i32 = 140i32;
    let mut u16_0: u16 = 1u16;
    let mut i32_12: i32 = 3i32;
    let mut i32_13: i32 = 140i32;
    let mut i32_14: i32 = 6817i32;
    let mut i32_15: i32 = 10i32;
    let mut i64_0: i64 = 100i64;
    let mut i64_1: i64 = -6318i64;
    let mut u16_1: u16 = 3u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    crate::hp::RomTiddle::foo3(romtiddle_0_ref_0, i64_1, i64_0);
    crate::hp::ParryHotter::alohomora(i32_15, i32_14, i32_13, i32_12);
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    crate::hp::ParryHotter::alohomora(i32_11, i32_10, i32_9, i32_8);
    crate::hp::ParryHotter::alohomora(i32_7, i32_6, i32_5, i32_4);
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_286() {
//    rusty_monitor::set_test_id(286);
    let mut i32_0: i32 = 3i32;
    let mut i32_1: i32 = 111i32;
    let mut i32_2: i32 = 25030i32;
    let mut i32_3: i32 = 7i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u64_0: u64 = 2074u64;
    let mut u64_1: u64 = 644u64;
    let mut i64_0: i64 = 111i64;
    let mut i64_1: i64 = 0i64;
    let mut u16_0: u16 = 3u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut str_0: &str = "THTDKPQV";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u16_1: u16 = 5987u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut u16_2: u16 = 7271u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut i32_4: i32 = 13i32;
    let mut i32_5: i32 = -2929i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_5, i32_4);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    crate::hp::RomTiddle::foo3(romtiddle_0_ref_0, i64_1, i64_0);
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    crate::hp::ParryHotter::foo2(parryhotter_0_ref_0, i32_1, i32_0);
//    panic!("From RustyUnit with love");
}
}