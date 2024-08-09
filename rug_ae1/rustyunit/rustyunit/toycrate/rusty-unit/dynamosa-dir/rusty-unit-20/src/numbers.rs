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
fn rusty_test_6806() {
    rusty_monitor::set_test_id(6806);
    let mut u16_0: u16 = 2334u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut i32_0: i32 = 11541i32;
    let mut i32_1: i32 = 3273i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut i64_0: i64 = -2394i64;
    let mut i64_1: i64 = 6085i64;
    let mut u16_1: u16 = 76u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_1};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut u16_2: u16 = 2730u16;
    let mut u64_0: u64 = 1321u64;
    let mut u64_1: u64 = 3742u64;
    let mut i32_2: i32 = 15029i32;
    let mut i32_3: i32 = -4824i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_0);
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_897() {
    rusty_monitor::set_test_id(897);
    let mut str_0: &str = "BduZdj3A0TriVPZKh1i";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "9EAubBlr4FdtWogE";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut i64_0: i64 = -1420i64;
    let mut u16_0: u16 = 8165u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    let mut i64_1: i64 = 4068i64;
    let mut u16_1: u16 = 7254u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_1};
    let mut u64_0: u64 = 6616u64;
    let mut u64_1: u64 = 6607u64;
    let mut u16_2: u16 = 7105u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut usize_0: usize = 7200usize;
    let mut u16_3: u16 = 3760u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut i64_2: i64 = -18715i64;
    let mut u16_4: u16 = 52u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut u16_5: u16 = 3819u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut u16_6: u16 = 5798u16;
    let mut i32_0: i32 = 21490i32;
    let mut i32_1: i32 = -9992i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_1, b: i32_0};
    let mut i32_2: i32 = 13160i32;
    let mut i32_3: i32 = -4415i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_4: i32 = -10560i32;
    let mut u16_7: u16 = 3310u16;
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_7};
    let mut romtiddle_6_ref_0: &crate::hp::RomTiddle = &mut romtiddle_6;
    let mut i32_5: i32 = 8765i32;
    let mut i32_6: i32 = -1555i32;
    let mut i32_7: i32 = -19889i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_7, i32_6);
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut romtiddle_5_ref_0: &crate::hp::RomTiddle = &mut romtiddle_5;
    let mut i32_8: i32 = 4447i32;
    let mut i32_9: i32 = 2133i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_5, i32_9);
    let mut parryhotter_4: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_4, i32_8);
    let mut parryhotter_3_ref_0: &crate::hp::ParryHotter = &mut parryhotter_3;
    let mut i32_10: i32 = -7045i32;
    let mut i32_11: i32 = -9136i32;
    let mut i32_12: i32 = -4422i32;
    let mut i32_13: i32 = -15925i32;
    let mut i32_14: i32 = -5729i32;
    let mut i32_15: i32 = 14864i32;
    let mut i32_16: i32 = -12204i32;
    let mut parryhotter_5: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_13, b: i32_16};
    let mut parryhotter_4_ref_0: &crate::hp::ParryHotter = &mut parryhotter_4;
    let mut i32_17: i32 = crate::hp::ParryHotter::accio(parryhotter_1_ref_0, i32_14, i32_15);
    let mut parryhotter_6: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_11, b: i32_12};
    let mut romtiddle_7: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_4_ref_0, usize_0, u64_0, u64_1);
    let mut wonreasley_2: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_2};
    let mut wonreasley_1_ref_0: &crate::hp::WonReasley = &mut wonreasley_1;
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_1_ref_0);
    crate::hp::WonReasley::arania_exumai(wonreasley_1_ref_0, str_0_ref_0);
    let mut i32_18: i32 = crate::hp::ParryHotter::accio(parryhotter_3_ref_0, i32_17, i32_10);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_532() {
    rusty_monitor::set_test_id(532);
    let mut i32_0: i32 = -3153i32;
    let mut i32_1: i32 = -15372i32;
    let mut i32_2: i32 = 13559i32;
    let mut i32_3: i32 = 994i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut str_0: &str = "4izqrfgW20W";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u16_0: u16 = 6076u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut u16_1: u16 = 9623u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut usize_0: usize = 4309usize;
    let mut i32_4: i32 = -8999i32;
    let mut i32_5: i32 = 9430i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_5, i32_4);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut u64_0: u64 = 735u64;
    let mut u64_1: u64 = 92u64;
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    let mut i32_6: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_1_ref_0, usize_0, string_0);
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut i32_7: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_1, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_164() {
    rusty_monitor::set_test_id(164);
    let mut str_0: &str = "4izqrfgW20W";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u16_0: u16 = 6076u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut u16_1: u16 = 9623u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut usize_0: usize = 4309usize;
    let mut i32_0: i32 = -8999i32;
    let mut i32_1: i32 = 9430i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u64_0: u64 = 735u64;
    let mut u64_1: u64 = 92u64;
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    let mut i32_2: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_0_ref_0, usize_0, string_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7192() {
    rusty_monitor::set_test_id(7192);
    let mut i32_0: i32 = 5027i32;
    let mut i32_1: i32 = -11584i32;
    let mut i64_0: i64 = -4854i64;
    let mut u16_0: u16 = 5373u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut i64_1: i64 = -9965i64;
    let mut u16_1: u16 = 4485u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut u16_2: u16 = 5271u16;
    let mut i64_2: i64 = 1826i64;
    let mut i64_3: i64 = 3867i64;
    let mut u16_3: u16 = 3515u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    crate::hp::RomTiddle::foo3(romtiddle_2_ref_0, i64_3, i64_2);
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_1};
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_1, b: i32_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3113() {
    rusty_monitor::set_test_id(3113);
    let mut i32_0: i32 = 5027i32;
    let mut i32_1: i32 = -11584i32;
    let mut u64_0: u64 = 2580u64;
    let mut u64_1: u64 = 757u64;
    let mut i64_0: i64 = -9965i64;
    let mut u16_0: u16 = 4485u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut i32_2: i32 = 574i32;
    let mut i32_3: i32 = 2215i32;
    let mut i32_4: i32 = -2954i32;
    let mut i32_5: i32 = 15114i32;
    let mut u16_1: u16 = 5271u16;
    let mut i64_1: i64 = 1826i64;
    let mut i64_2: i64 = 3867i64;
    let mut u16_2: u16 = 3515u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    crate::hp::RomTiddle::foo3(romtiddle_1_ref_0, i64_2, i64_1);
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    crate::hp::ParryHotter::alohomora(i32_5, i32_4, i32_3, i32_2);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_1, b: i32_0};
    panic!("From RustyUnit with love");
}
}