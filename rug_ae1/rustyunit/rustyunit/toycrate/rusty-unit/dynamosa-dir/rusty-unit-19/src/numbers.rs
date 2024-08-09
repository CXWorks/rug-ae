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
fn rusty_test_7327() {
    rusty_monitor::set_test_id(7327);
    let mut i64_0: i64 = 14119i64;
    let mut str_0: &str = "yNwd43fM2igmpw";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_1: i64 = -14076i64;
    let mut u16_0: u16 = 3333u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_1};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    let mut u16_1: u16 = 9929u16;
    let mut u16_2: u16 = 2u16;
    let mut u16_3: u16 = 1798u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut u16_4: u16 = 386u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut u16_5: u16 = 3658u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_3_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_3;
    let mut u16_6: u16 = 9587u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut i32_0: i32 = 454i32;
    let mut i32_1: i32 = -4988i32;
    let mut i32_2: i32 = 2891i32;
    let mut i64_2: i64 = 2232i64;
    let mut u16_7: u16 = 7603u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_7};
    let mut romtiddle_5_ref_0: &crate::hp::RomTiddle = &mut romtiddle_5;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_5_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_2};
    let mut i32_3: i32 = -22003i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_2, i32_3);
    let mut u64_0: u64 = 8815u64;
    let mut i32_4: i32 = -1121i32;
    let mut i32_5: i32 = 19167i32;
    let mut i32_6: i32 = -27230i32;
    let mut i32_7: i32 = 9205i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_6);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u16_8: u16 = 476u16;
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut u16_9: u16 = 5197u16;
    let mut u64_1: u64 = 3174u64;
    let mut usize_0: usize = 2321usize;
    let mut i32_8: i32 = -7163i32;
    let mut i32_9: i32 = 6500i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_7, b: i32_5};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut romtiddle_7: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_8};
    let mut romtiddle_8: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_9};
    let mut romtiddle_6_ref_0: &crate::hp::RomTiddle = &mut romtiddle_6;
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    let mut i64_3: i64 = 1818i64;
    let mut i64_4: i64 = 16046i64;
    let mut romtiddle_9: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_8_ref_0: &crate::hp::RomTiddle = &mut romtiddle_8;
    let mut string_4: std::string::String = crate::hp::RomTiddle::name(romtiddle_6_ref_0);
    let mut wonreasley_2: crate::hp::WonReasley = crate::hp::WonReasley {x: string_3, y: i64_3};
    let mut wonreasley_1_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_1;
    let mut i32_10: i32 = -11920i32;
    let mut i32_11: i32 = 1603i32;
    let mut i32_12: i32 = 7530i32;
    let mut i32_13: i32 = -5238i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_8, i32_11);
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i32_14: i32 = crate::hp::ParryHotter::accio(parryhotter_1_ref_0, i32_12, i32_4);
    crate::hp::WonReasley::ascendio(wonreasley_1_ref_0, i64_4);
    let mut wonreasley_3: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_0};
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_2_ref_0, usize_0, u64_0, u64_1);
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
    let mut parryhotter_4: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_0, b: i32_10};
    let mut parryhotter_5: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_13, b: i32_9};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7627() {
    rusty_monitor::set_test_id(7627);
    let mut i32_0: i32 = 781i32;
    let mut i32_1: i32 = -2018i32;
    let mut i32_2: i32 = -2601i32;
    let mut i32_3: i32 = 17809i32;
    let mut u16_0: u16 = 1262u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut i64_0: i64 = 10987i64;
    let mut u16_1: u16 = 6228u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut i32_4: i32 = -136i32;
    let mut i32_5: i32 = -13195i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_5, i32_4);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u16_2: u16 = 1712u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut u16_3: u16 = 7852u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    crate::hp::ParryHotter::alohomora(i32_3, i32_2, i32_1, i32_0);
    panic!("From RustyUnit with love");
}
}