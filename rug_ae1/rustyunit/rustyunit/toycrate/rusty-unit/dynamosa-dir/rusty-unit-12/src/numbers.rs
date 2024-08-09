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
fn rusty_test_7996() {
    rusty_monitor::set_test_id(7996);
    let mut i64_0: i64 = -4087i64;
    let mut i64_1: i64 = -5702i64;
    let mut u16_0: u16 = 0u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_1};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut u64_0: u64 = 9538u64;
    let mut i32_0: i32 = 7469i32;
    let mut i32_1: i32 = -7984i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_1, b: i32_0};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u64_1: u64 = 5732u64;
    let mut u16_1: u16 = 5171u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut i32_2: i32 = 13246i32;
    let mut i32_3: i32 = 5611i32;
    let mut i32_4: i32 = -4660i32;
    let mut str_0: &str = "cUGLVLRPo";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u16_2: u16 = 13u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut u16_3: u16 = 9022u16;
    let mut i32_5: i32 = 11965i32;
    let mut i32_6: i32 = -8180i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_6, i32_5);
    let mut u16_4: u16 = 6364u16;
    let mut i32_7: i32 = -12482i32;
    let mut i32_8: i32 = -11871i32;
    let mut i32_9: i32 = -17532i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_9, i32_8);
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i32_10: i32 = -2474i32;
    let mut i32_11: i32 = 2359i32;
    let mut i32_12: i32 = 8221i32;
    let mut i32_13: i32 = 7327i32;
    let mut i32_14: i32 = 16846i32;
    let mut i32_15: i32 = 15924i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_15, b: i32_14};
    let mut parryhotter_3_ref_0: &crate::hp::ParryHotter = &mut parryhotter_3;
    let mut i64_2: i64 = -8755i64;
    let mut i64_3: i64 = 19327i64;
    let mut u16_5: u16 = 4422u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut u64_2: u64 = 2152u64;
    let mut u64_3: u64 = 6195u64;
    let mut usize_0: usize = 9143usize;
    let mut i32_16: i32 = 1211i32;
    let mut parryhotter_4: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_16, b: i32_2};
    let mut parryhotter_4_ref_0: &crate::hp::ParryHotter = &mut parryhotter_4;
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_4_ref_0, usize_0, u64_3, u64_2);
    crate::hp::RomTiddle::foo3(romtiddle_3_ref_0, i64_3, i64_2);
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut parryhotter_5: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_13, b: i32_11};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut u16_6: u16 = 1531u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    let mut romtiddle_5_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_5;
    let mut parryhotter_4_ref_0: &crate::hp::ParryHotter = &mut parryhotter_5;
    let mut i32_17: i32 = 16746i32;
    let mut i32_18: i32 = 1433i32;
    let mut i32_19: i32 = -8629i32;
    crate::hp::ParryHotter::alohomora(i32_10, i32_17, i32_18, i32_12);
    crate::hp::ParryHotter::foo2(parryhotter_1_ref_0, i32_19, i32_7);
    let mut i32_18: i32 = 1433i32;
    let mut i32_19: i32 = -8481i32;
    let mut i32_20: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_4, i32_3);
    crate::hp::ParryHotter::another_number_fn(u64_0, u64_1);
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_50() {
    rusty_monitor::set_test_id(50);
    let mut u64_0: u64 = 4715u64;
    let mut u64_1: u64 = 2500u64;
    let mut i32_0: i32 = 3066i32;
    let mut i32_1: i32 = -8013i32;
    let mut u16_0: u16 = 543u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut usize_0: usize = 3181usize;
    let mut i32_2: i32 = 3876i32;
    let mut i32_3: i32 = 3139i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u16_1: u16 = 8185u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut str_0: &str = "9nSaqtMFGswF";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_0: i64 = -10233i64;
    let mut u16_2: u16 = 7803u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_0};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut i32_4: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_0_ref_0, usize_0, string_0);
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_1, b: i32_0};
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    panic!("From RustyUnit with love");
}
}