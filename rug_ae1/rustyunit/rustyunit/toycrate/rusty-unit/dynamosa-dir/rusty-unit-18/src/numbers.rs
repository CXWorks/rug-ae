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
fn rusty_test_2165() {
    rusty_monitor::set_test_id(2165);
    let mut str_0: &str = "EpQ8vOCpd";
    let mut u16_0: u16 = 9803u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut u16_1: u16 = 9392u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut u16_2: u16 = 1504u16;
    let mut i32_0: i32 = 13718i32;
    let mut i32_1: i32 = -6318i32;
    let mut i32_2: i32 = 7362i32;
    let mut i32_3: i32 = -6652i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_4: i32 = -16094i32;
    let mut i32_5: i32 = -6826i32;
    let mut i32_6: i32 = 19633i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_6, i32_5);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_7: i32 = -11558i32;
    let mut i32_8: i32 = 7862i32;
    let mut str_1: &str = "V";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut i64_0: i64 = 12521i64;
    let mut i64_1: i64 = 16495i64;
    let mut u16_3: u16 = 949u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut romtiddle_1_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_1;
    let mut u16_4: u16 = 6272u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut u16_5: u16 = 3081u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut u16_6: u16 = 9362u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_4_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_4;
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i32_9: i32 = 9846i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_7, i32_1);
    let mut i64_2: i64 = 13559i64;
    let mut u16_7: u16 = 1783u16;
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    let mut romtiddle_5_ref_0: &crate::hp::RomTiddle = &mut romtiddle_5;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_1};
    let mut u16_8: u16 = 8761u16;
    let mut romtiddle_7: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_7};
    let mut romtiddle_6_ref_0: &crate::hp::RomTiddle = &mut romtiddle_6;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_5_ref_0);
    let mut u16_9: u16 = 8212u16;
    let mut romtiddle_8: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_8};
    let mut romtiddle_7_ref_0: &crate::hp::RomTiddle = &mut romtiddle_7;
    let mut romtiddle_9: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_9};
    let mut i32_10: i32 = 3842i32;
    let mut u64_0: u64 = 6980u64;
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_6_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_2};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    let mut i32_11: i32 = 3118i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_9, b: i32_0};
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i32_12: i32 = -9994i32;
    let mut parryhotter_3_ref_0: &crate::hp::ParryHotter = &mut parryhotter_3;
    let mut u64_1: u64 = 52u64;
    crate::hp::ParryHotter::another_number_fn(u64_0, u64_1);
    let mut i32_13: i32 = crate::hp::ParryHotter::accio(parryhotter_3_ref_0, i32_8, i32_10);
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_1_ref_0);
    crate::hp::RomTiddle::avada_kedavra(romtiddle_1_ref_0, string_2);
    let mut string_4: std::string::String = crate::hp::RomTiddle::name(romtiddle_7_ref_0);
    crate::hp::RomTiddle::avada_kedavra(romtiddle_4_ref_0, string_3);
    crate::hp::ParryHotter::alohomora(i32_11, i32_13, i32_4, i32_12);
    let mut wonreasley_2: crate::hp::WonReasley = crate::hp::WonReasley {x: string_4, y: i64_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5497() {
    rusty_monitor::set_test_id(5497);
    let mut u64_0: u64 = 6980u64;
    let mut u64_1: u64 = 7178u64;
    let mut str_0: &str = "6";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_0: i64 = 719i64;
    let mut u16_0: u16 = 3808u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    let mut i64_1: i64 = 2994i64;
    let mut i64_2: i64 = 5849i64;
    let mut u16_1: u16 = 4537u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_2};
    let mut wonreasley_1_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_1;
    crate::hp::WonReasley::ascendio(wonreasley_1_ref_0, i64_1);
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    panic!("From RustyUnit with love");
}
}