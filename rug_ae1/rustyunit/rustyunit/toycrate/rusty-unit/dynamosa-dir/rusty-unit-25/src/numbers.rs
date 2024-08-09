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
fn rusty_test_3054() {
    rusty_monitor::set_test_id(3054);
    let mut u16_0: u16 = 6788u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut u16_1: u16 = 9165u16;
    let mut u16_2: u16 = 5988u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut i64_0: i64 = 21062i64;
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut i64_1: i64 = -4634i64;
    let mut u16_3: u16 = 1508u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut u16_4: u16 = 8038u16;
    let mut romtiddle_3_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_3;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    crate::hp::RomTiddle::foo3(romtiddle_0_ref_0, i64_0, i64_1);
    crate::hp::RomTiddle::avada_kedavra(romtiddle_3_ref_0, string_0);
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_52() {
    rusty_monitor::set_test_id(52);
    let mut str_0: &str = "m77nuwNZe1KByCfK";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u16_0: u16 = 3097u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut u16_1: u16 = 978u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut u16_2: u16 = 9844u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut usize_0: usize = 2786usize;
    let mut i32_0: i32 = -4349i32;
    let mut i32_1: i32 = -6487i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_1, b: i32_0};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_2: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_0_ref_0, usize_0, string_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2023() {
    rusty_monitor::set_test_id(2023);
    let mut u16_0: u16 = 6749u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut u64_0: u64 = 9098u64;
    let mut u64_1: u64 = 673u64;
    let mut str_0: &str = "YduOlp772U";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_0: i64 = 13752i64;
    let mut u16_1: u16 = 5664u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    let mut i64_1: i64 = 3609i64;
    let mut u16_2: u16 = 9679u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut i32_0: i32 = 9582i32;
    let mut i32_1: i32 = 6903i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_1, b: i32_0};
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_1};
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_640() {
    rusty_monitor::set_test_id(640);
    let mut u16_0: u16 = 8425u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut u16_1: u16 = 6641u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut u64_0: u64 = 5272u64;
    let mut u64_1: u64 = 9726u64;
    let mut usize_0: usize = 6992usize;
    let mut i32_0: i32 = -1103i32;
    let mut i32_1: i32 = -16473i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_1, b: i32_0};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i64_0: i64 = -2148i64;
    let mut i64_1: i64 = -15147i64;
    let mut u16_2: u16 = 6236u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_1};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_0);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_0_ref_0, usize_0, u64_1, u64_0);
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_476() {
    rusty_monitor::set_test_id(476);
    let mut u16_0: u16 = 7532u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut u64_0: u64 = 5792u64;
    let mut u64_1: u64 = 2157u64;
    let mut usize_0: usize = 6513usize;
    let mut i32_0: i32 = 10463i32;
    let mut i32_1: i32 = 14370i32;
    let mut u64_2: u64 = 6837u64;
    let mut u64_3: u64 = 260u64;
    let mut u16_1: u16 = 4472u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut i32_2: i32 = -453i32;
    let mut i32_3: i32 = -13767i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    crate::hp::ParryHotter::another_number_fn(u64_3, u64_2);
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_1_ref_0, usize_0, u64_1, u64_0);
    panic!("From RustyUnit with love");
}
}