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
fn rusty_test_5047() {
    rusty_monitor::set_test_id(5047);
    let mut u64_0: u64 = 3725u64;
    let mut u64_1: u64 = 6444u64;
    let mut i32_0: i32 = -7949i32;
    let mut i32_1: i32 = -5078i32;
    let mut i32_2: i32 = 10334i32;
    let mut i32_3: i32 = 1928i32;
    let mut u16_0: u16 = 7607u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut u16_1: u16 = 2561u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_1;
    crate::hp::RomTiddle::avada_kedavra(romtiddle_1_ref_0, string_0);
    crate::hp::ParryHotter::alohomora(i32_3, i32_2, i32_1, i32_0);
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5150() {
    rusty_monitor::set_test_id(5150);
    let mut i32_0: i32 = 7885i32;
    let mut i32_1: i32 = 1511i32;
    let mut i32_2: i32 = -4543i32;
    let mut i32_3: i32 = -3599i32;
    let mut i32_4: i32 = -5203i32;
    let mut i32_5: i32 = -5636i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_5, i32_4);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u16_0: u16 = 7535u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_0;
    let mut i32_6: i32 = 1447i32;
    let mut i32_7: i32 = -7986i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_7, i32_6);
    let mut u16_1: u16 = 1602u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut u16_2: u16 = 8433u16;
    let mut i32_8: i32 = -11432i32;
    let mut i32_9: i32 = -1075i32;
    let mut i32_10: i32 = -15283i32;
    let mut i32_11: i32 = -851i32;
    let mut i32_12: i32 = 503i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_12, i32_11);
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i32_13: i32 = -11300i32;
    let mut i32_14: i32 = 4412i32;
    let mut i32_15: i32 = -5302i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_15, i32_14);
    let mut parryhotter_3_ref_0: &crate::hp::ParryHotter = &mut parryhotter_3;
    let mut u16_3: u16 = 9473u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut str_0: &str = "lWhKl1cevZj9Q5E7b";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_0: i64 = 11109i64;
    let mut u16_4: u16 = 2040u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    let mut u16_5: u16 = 1719u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut u64_0: u64 = 1050u64;
    let mut i32_16: i32 = -19332i32;
    let mut i32_17: i32 = -1258i32;
    let mut i32_18: i32 = 66i32;
    let mut parryhotter_4: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_10, b: i32_13};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_19: i32 = 5754i32;
    let mut i32_20: i32 = -1923i32;
    let mut u64_1: u64 = 4494u64;
    crate::hp::ParryHotter::another_number_fn(u64_0, u64_1);
    let mut parryhotter_5: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_19, b: i32_16};
    let mut i32_21: i32 = crate::hp::ParryHotter::accio(parryhotter_1_ref_0, i32_20, i32_18);
    let mut parryhotter_6: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_9, i32_8);
    let mut parryhotter_4_ref_0: &crate::hp::ParryHotter = &mut parryhotter_4;
    let mut romtiddle_5_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_5;
    crate::hp::RomTiddle::avada_kedavra(romtiddle_0_ref_0, string_1);
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
    let mut parryhotter_6_ref_0: &crate::hp::ParryHotter = &mut parryhotter_6;
    let mut i32_22: i32 = crate::hp::ParryHotter::accio(parryhotter_2_ref_0, i32_21, i32_17);
    crate::hp::ParryHotter::foo2(parryhotter_0_ref_0, i32_3, i32_2);
    let mut parryhotter_5_ref_0: &crate::hp::ParryHotter = &mut parryhotter_5;
    let mut i32_23: i32 = crate::hp::ParryHotter::accio(parryhotter_5_ref_0, i32_1, i32_0);
    panic!("From RustyUnit with love");
}
}