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
fn rusty_test_6800() {
    rusty_monitor::set_test_id(6800);
    let mut i32_0: i32 = -13981i32;
    let mut i32_1: i32 = 5835i32;
    let mut i32_2: i32 = -10039i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_2, b: i32_1};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u64_0: u64 = 9684u64;
    let mut usize_0: usize = 1022usize;
    let mut i32_3: i32 = 9428i32;
    let mut i32_4: i32 = 512i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_4, b: i32_3};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_5: i32 = -15883i32;
    let mut i32_6: i32 = 12051i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_6, b: i32_5};
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i32_7: i32 = 3279i32;
    let mut i32_8: i32 = -6280i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_8, b: i32_7};
    let mut i32_9: i32 = 6838i32;
    let mut i32_10: i32 = -11680i32;
    let mut u64_1: u64 = 6953u64;
    let mut i32_11: i32 = 2227i32;
    let mut i32_12: i32 = 16412i32;
    let mut parryhotter_4: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_9, b: i32_11};
    let mut parryhotter_3_ref_0: &crate::hp::ParryHotter = &mut parryhotter_3;
    let mut u16_0: u16 = 3846u16;
    let mut u16_1: u16 = 9539u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut i32_13: i32 = 2729i32;
    let mut parryhotter_5: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_10, b: i32_12};
    let mut parryhotter_4_ref_0: &crate::hp::ParryHotter = &mut parryhotter_4;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_1_ref_0, usize_0, u64_0, u64_1);
    let mut i32_14: i32 = crate::hp::ParryHotter::accio(parryhotter_4_ref_0, i32_13, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2336() {
    rusty_monitor::set_test_id(2336);
    let mut i32_0: i32 = -9706i32;
    let mut i32_1: i32 = 7831i32;
    let mut u16_0: u16 = 1008u16;
    let mut i32_2: i32 = 2539i32;
    let mut i32_3: i32 = 5775i32;
    let mut u16_1: u16 = 9523u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut str_0: &str = "PQNVjKUjVRvOGw";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u16_2: u16 = 9348u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut i32_4: i32 = -14607i32;
    let mut i32_5: i32 = 19856i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_5, i32_4);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i64_0: i64 = 2846i64;
    let mut u16_3: u16 = 9997u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut str_1: &str = "JQ1KB";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    crate::hp::ParryHotter::foo2(parryhotter_0_ref_0, i32_3, i32_0);
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_2, b: i32_1};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_757() {
    rusty_monitor::set_test_id(757);
    let mut u16_0: u16 = 4075u16;
    let mut u16_1: u16 = 1031u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut usize_0: usize = 3726usize;
    let mut i32_0: i32 = -101i32;
    let mut i32_1: i32 = -10968i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_2: i32 = 3005i32;
    let mut i32_3: i32 = -12159i32;
    let mut u16_2: u16 = 5311u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut i32_4: i32 = -5189i32;
    let mut i32_5: i32 = 15814i32;
    let mut i32_6: i32 = 6904i32;
    let mut i32_7: i32 = 2412i32;
    let mut i32_8: i32 = -21121i32;
    let mut i32_9: i32 = -10203i32;
    let mut i32_10: i32 = -6333i32;
    let mut i32_11: i32 = -3887i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_11, i32_10);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_12: i32 = crate::hp::ParryHotter::accio(parryhotter_1_ref_0, i32_9, i32_8);
    crate::hp::ParryHotter::alohomora(i32_7, i32_6, i32_5, i32_4);
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    let mut i32_13: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_0_ref_0, usize_0, string_0);
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8187() {
    rusty_monitor::set_test_id(8187);
    let mut u16_0: u16 = 5897u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut u16_1: u16 = 9557u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut i64_0: i64 = -2392i64;
    let mut i64_1: i64 = 8375i64;
    let mut u16_2: u16 = 1767u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    crate::hp::RomTiddle::foo3(romtiddle_2_ref_0, i64_1, i64_0);
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    panic!("From RustyUnit with love");
}
}