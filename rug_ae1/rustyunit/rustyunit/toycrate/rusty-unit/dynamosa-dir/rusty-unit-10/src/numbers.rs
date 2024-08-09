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
fn rusty_test_636() {
    rusty_monitor::set_test_id(636);
    let mut usize_0: usize = 5471usize;
    let mut i32_0: i32 = -5737i32;
    let mut i32_1: i32 = -2741i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u64_0: u64 = 711u64;
    let mut i32_2: i32 = -9317i32;
    let mut i32_3: i32 = 9465i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut str_0: &str = "yNYwkQaIPxL";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_0: i64 = -10846i64;
    let mut u16_0: u16 = 8937u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    let mut u16_1: u16 = 5798u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut u64_1: u64 = 4198u64;
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_0_ref_0, usize_0, u64_1, u64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_217() {
    rusty_monitor::set_test_id(217);
    let mut u16_0: u16 = 4651u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut i32_0: i32 = 1290i32;
    let mut i32_1: i32 = -16027i32;
    let mut i32_2: i32 = -7706i32;
    let mut i32_3: i32 = 7248i32;
    let mut u16_1: u16 = 914u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut str_0: &str = "fiU6RpcQwF5PUKZZZ";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u16_2: u16 = 7769u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    crate::hp::ParryHotter::alohomora(i32_3, i32_2, i32_1, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_646() {
    rusty_monitor::set_test_id(646);
    let mut i32_0: i32 = -4235i32;
    let mut i32_1: i32 = -13086i32;
    let mut i32_2: i32 = -26844i32;
    let mut i32_3: i32 = 14311i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_4: i32 = 14575i32;
    let mut i32_5: i32 = 7305i32;
    let mut i32_6: i32 = 11530i32;
    let mut i32_7: i32 = -5872i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_7, i32_6);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut u64_0: u64 = 7363u64;
    let mut u64_1: u64 = 3555u64;
    let mut usize_0: usize = 6163usize;
    let mut i32_8: i32 = -7733i32;
    let mut i32_9: i32 = 1051i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_9, i32_8);
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_2_ref_0, usize_0, u64_1, u64_0);
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_5, b: i32_4};
    let mut i32_10: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_1, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3782() {
    rusty_monitor::set_test_id(3782);
    let mut i64_0: i64 = 12417i64;
    let mut i64_1: i64 = 12629i64;
    let mut u16_0: u16 = 3406u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut u16_1: u16 = 6696u16;
    let mut i32_0: i32 = -13996i32;
    let mut i32_1: i32 = 5427i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_1;
    let mut u64_0: u64 = 3392u64;
    let mut i32_2: i32 = -7276i32;
    let mut i32_3: i32 = -12536i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut u16_2: u16 = 5347u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut i32_4: i32 = -6539i32;
    let mut i32_5: i32 = -2047i32;
    let mut u64_1: u64 = 446u64;
    let mut i32_6: i32 = -12479i32;
    let mut i32_7: i32 = 749i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_5, i32_6);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    crate::hp::ParryHotter::foo2(parryhotter_0_ref_0, i32_4, i32_7);
    crate::hp::RomTiddle::foo3(romtiddle_0_ref_0, i64_1, i64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5321() {
    rusty_monitor::set_test_id(5321);
    let mut i32_0: i32 = 5439i32;
    let mut i32_1: i32 = 7602i32;
    let mut u64_0: u64 = 3231u64;
    let mut u64_1: u64 = 375u64;
    let mut i32_2: i32 = -6539i32;
    let mut i32_3: i32 = -2047i32;
    let mut u64_2: u64 = 427u64;
    let mut u64_3: u64 = 446u64;
    let mut usize_0: usize = 9427usize;
    let mut i32_4: i32 = -12479i32;
    let mut i32_5: i32 = 749i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_5, i32_4);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_0_ref_0, usize_0, u64_3, u64_2);
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_583() {
    rusty_monitor::set_test_id(583);
    let mut i64_0: i64 = 7866i64;
    let mut i64_1: i64 = 301i64;
    let mut u16_0: u16 = 4084u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_1};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut u16_1: u16 = 4518u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut u16_2: u16 = 8944u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut u16_3: u16 = 951u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_3_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_3;
    let mut u64_0: u64 = 427u64;
    let mut u64_1: u64 = 446u64;
    let mut usize_0: usize = 9427usize;
    let mut i32_0: i32 = -12479i32;
    let mut i32_1: i32 = 749i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_0_ref_0, usize_0, u64_1, u64_0);
    crate::hp::RomTiddle::avada_kedavra(romtiddle_3_ref_0, string_2);
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_0);
    panic!("From RustyUnit with love");
}
}