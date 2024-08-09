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
#[timeout(30000)]fn rusty_test_5217() {
//    rusty_monitor::set_test_id(5217);
    let mut u64_0: u64 = 7449u64;
    let mut u64_1: u64 = 2079u64;
    let mut i32_0: i32 = 40i32;
    let mut i32_1: i32 = -3391i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u16_0: u16 = 3u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut u16_1: u16 = 1u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut u16_2: u16 = 9301u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_2;
    let mut u16_3: u16 = 8071u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut i32_2: i32 = 3000i32;
    let mut i32_3: i32 = 1i32;
    crate::hp::RomTiddle::avada_kedavra(romtiddle_2_ref_0, string_0);
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    crate::hp::ParryHotter::foo2(parryhotter_0_ref_0, i32_2, i32_3);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_430() {
//    rusty_monitor::set_test_id(430);
    let mut u64_0: u64 = 4306u64;
    let mut u64_1: u64 = 228u64;
    let mut usize_0: usize = 2usize;
    let mut i32_0: i32 = 10i32;
    let mut i32_1: i32 = 15i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_1, b: i32_0};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u64_2: u64 = 20u64;
    let mut u64_3: u64 = 1735u64;
    let mut i64_0: i64 = -809i64;
    let mut i64_1: i64 = 3700i64;
    let mut u16_0: u16 = 3521u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut i32_2: i32 = 7i32;
    let mut i32_3: i32 = 32i32;
    let mut i32_4: i32 = 1i32;
    let mut i32_5: i32 = 20i32;
    let mut u64_4: u64 = 1040u64;
    let mut u64_5: u64 = 1621u64;
    crate::hp::ParryHotter::another_number_fn(u64_5, u64_4);
    crate::hp::ParryHotter::alohomora(i32_5, i32_4, i32_3, i32_2);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_1};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_0);
    crate::hp::ParryHotter::another_number_fn(u64_3, u64_2);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_0_ref_0, usize_0, u64_1, u64_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1179() {
//    rusty_monitor::set_test_id(1179);
    let mut i32_0: i32 = -546i32;
    let mut i32_1: i32 = 10i32;
    let mut i32_2: i32 = 13163i32;
    let mut i32_3: i32 = 3i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u16_0: u16 = 2833u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut u16_1: u16 = 4914u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut i32_4: i32 = 3i32;
    let mut i32_5: i32 = 20i32;
    let mut u64_0: u64 = 4306u64;
    let mut u64_1: u64 = 228u64;
    let mut usize_0: usize = 2usize;
    let mut i32_6: i32 = 10i32;
    let mut i32_7: i32 = 15i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_7, b: i32_6};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut u64_2: u64 = 20u64;
    let mut u64_3: u64 = 1735u64;
    let mut i64_0: i64 = -809i64;
    let mut i64_1: i64 = 3700i64;
    let mut u16_2: u16 = 3521u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut i32_8: i32 = 7i32;
    let mut i32_9: i32 = 32i32;
    let mut i32_10: i32 = 1i32;
    let mut i32_11: i32 = 20i32;
    let mut u64_4: u64 = 1040u64;
    let mut u64_5: u64 = 1621u64;
    crate::hp::ParryHotter::another_number_fn(u64_5, u64_4);
    crate::hp::ParryHotter::alohomora(i32_11, i32_10, i32_9, i32_8);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_1};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_0);
    crate::hp::ParryHotter::another_number_fn(u64_3, u64_2);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_1_ref_0, usize_0, u64_1, u64_0);
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut i32_12: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_5, i32_4);
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_1, b: i32_0};
//    panic!("From RustyUnit with love");
}
}