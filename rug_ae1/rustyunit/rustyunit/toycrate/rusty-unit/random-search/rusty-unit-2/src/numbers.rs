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
fn rusty_test_3208() {
    rusty_monitor::set_test_id(3208);
    let mut i32_0: i32 = 1704i32;
    let mut i32_1: i32 = -6131i32;
    let mut u16_0: u16 = 1293u16;
    let mut u64_0: u64 = 8558u64;
    let mut u64_1: u64 = 9725u64;
    let mut i32_2: i32 = 13028i32;
    let mut i32_3: i32 = 7021i32;
    let mut i32_4: i32 = -14527i32;
    let mut i32_5: i32 = 14846i32;
    let mut i32_6: i32 = 12552i32;
    let mut i32_7: i32 = -14205i32;
    let mut i32_8: i32 = 15242i32;
    let mut i32_9: i32 = 11093i32;
    let mut i32_10: i32 = -1554i32;
    let mut i32_11: i32 = -9083i32;
    let mut u16_1: u16 = 900u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut u16_2: u16 = 148u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_1_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_1;
    crate::hp::RomTiddle::avada_kedavra(romtiddle_1_ref_0, string_0);
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_11, b: i32_10};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_12: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_9, i32_8);
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_7, b: i32_6};
    crate::hp::ParryHotter::alohomora(i32_5, i32_4, i32_3, i32_2);
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1927() {
    rusty_monitor::set_test_id(1927);
    let mut i32_0: i32 = -8289i32;
    let mut i32_1: i32 = -2344i32;
    let mut i64_0: i64 = 487i64;
    let mut u16_0: u16 = 1150u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut i32_2: i32 = -2592i32;
    let mut i32_3: i32 = -52i32;
    let mut i32_4: i32 = 16i32;
    let mut i32_5: i32 = -11675i32;
    let mut i32_6: i32 = 15690i32;
    let mut i32_7: i32 = -1967i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_7, i32_6);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u16_1: u16 = 171u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut usize_0: usize = 3983usize;
    let mut i32_8: i32 = -3192i32;
    let mut i32_9: i32 = 919i32;
    let mut u16_2: u16 = 3245u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut usize_1: usize = 2049usize;
    let mut i32_10: i32 = 17091i32;
    let mut i32_11: i32 = 8542i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_11, i32_10);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i64_1: i64 = 8902i64;
    let mut i64_2: i64 = 12643i64;
    let mut u16_3: u16 = 3300u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_3, y: i64_2};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_1);
    let mut i32_12: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_1_ref_0, usize_1, string_2);
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_9, b: i32_8};
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i32_13: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_2_ref_0, usize_0, string_1);
    crate::hp::ParryHotter::foo2(parryhotter_0_ref_0, i32_5, i32_4);
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut parryhotter_3_ref_0: &crate::hp::ParryHotter = &mut parryhotter_3;
    crate::hp::ParryHotter::foo2(parryhotter_3_ref_0, i32_1, i32_0);
    panic!("From RustyUnit with love");
}
}