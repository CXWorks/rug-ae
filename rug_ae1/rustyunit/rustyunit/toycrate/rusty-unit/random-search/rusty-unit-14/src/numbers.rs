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
fn rusty_test_4950() {
    rusty_monitor::set_test_id(4950);
    let mut str_0: &str = "6";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_0: i64 = 4025i64;
    let mut u16_0: u16 = 7931u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    let mut u16_1: u16 = 2750u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut i64_1: i64 = -6458i64;
    let mut u16_2: u16 = 983u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut i32_0: i32 = 2067i32;
    let mut i32_1: i32 = 2245i32;
    let mut i64_2: i64 = -5608i64;
    let mut i64_3: i64 = -1114i64;
    let mut u16_3: u16 = 4968u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut u16_4: u16 = 1466u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    let mut u16_5: u16 = 5502u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_5_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_5;
    let mut u64_0: u64 = 5515u64;
    let mut u64_1: u64 = 9737u64;
    let mut usize_0: usize = 4624usize;
    let mut i32_2: i32 = -7395i32;
    let mut i32_3: i32 = -8176i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u16_6: u16 = 4275u16;
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    let mut romtiddle_6_ref_0: &crate::hp::RomTiddle = &mut romtiddle_6;
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_6_ref_0);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_0_ref_0, usize_0, u64_1, u64_0);
    crate::hp::RomTiddle::avada_kedavra(romtiddle_5_ref_0, string_2);
    crate::hp::RomTiddle::foo3(romtiddle_3_ref_0, i64_3, i64_2);
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_1};
    let mut string_4: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4954() {
    rusty_monitor::set_test_id(4954);
    let mut i32_0: i32 = 2267i32;
    let mut i32_1: i32 = 20494i32;
    let mut i32_2: i32 = 21658i32;
    let mut i32_3: i32 = 7607i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_4: i32 = 11209i32;
    let mut i32_5: i32 = 6694i32;
    let mut str_0: &str = "zFRJ84UcyB6uNTgYnjG";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_0: i64 = 9840i64;
    let mut u16_0: u16 = 8301u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    let mut i32_6: i32 = -8476i32;
    let mut i32_7: i32 = 5121i32;
    let mut i64_1: i64 = -7478i64;
    let mut i64_2: i64 = -5204i64;
    let mut u16_1: u16 = 8266u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_2};
    let mut wonreasley_1_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_1;
    let mut i32_8: i32 = 10378i32;
    let mut i32_9: i32 = 10873i32;
    let mut i32_10: i32 = -8436i32;
    let mut i32_11: i32 = 6934i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_11, b: i32_10};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut u16_2: u16 = 8262u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut u16_3: u16 = 3023u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut usize_0: usize = 5718usize;
    let mut i32_12: i32 = 3097i32;
    let mut i32_13: i32 = -1249i32;
    let mut u16_4: u16 = 2861u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    let mut u16_5: u16 = 5395u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_5_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_5;
    let mut u16_6: u16 = 2789u16;
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    let mut romtiddle_6_ref_0: &crate::hp::RomTiddle = &mut romtiddle_6;
    let mut string_4: std::string::String = crate::hp::RomTiddle::name(romtiddle_6_ref_0);
    let mut u16_7: u16 = 1776u16;
    let mut romtiddle_7: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_7};
    let mut romtiddle_7_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_7;
    crate::hp::RomTiddle::avada_kedavra(romtiddle_7_ref_0, string_4);
    crate::hp::RomTiddle::avada_kedavra(romtiddle_5_ref_0, string_3);
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_13, i32_12);
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i32_14: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_2_ref_0, usize_0, string_2);
    let mut string_5: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    crate::hp::ParryHotter::foo2(parryhotter_1_ref_0, i32_9, i32_8);
    crate::hp::WonReasley::ascendio(wonreasley_1_ref_0, i64_1);
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_7, i32_6);
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
    let mut parryhotter_3_ref_0: &crate::hp::ParryHotter = &mut parryhotter_3;
    crate::hp::ParryHotter::foo2(parryhotter_3_ref_0, i32_5, i32_4);
    crate::hp::ParryHotter::foo2(parryhotter_0_ref_0, i32_1, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4955() {
    rusty_monitor::set_test_id(4955);
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u64_0: u64 = 1546u64;
    let mut u64_1: u64 = 4838u64;
    let mut usize_0: usize = 5666usize;
    let mut i32_0: i32 = 28498i32;
    let mut i32_1: i32 = -4524i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_1, b: i32_0};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i64_0: i64 = -13952i64;
    let mut u16_0: u16 = 8488u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut i32_2: i32 = -7955i32;
    let mut i32_3: i32 = 9013i32;
    let mut u16_1: u16 = 2338u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut i32_4: i32 = -5273i32;
    let mut i32_5: i32 = 7735i32;
    let mut u16_2: u16 = 5775u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut u64_2: u64 = 3472u64;
    let mut u64_3: u64 = 2297u64;
    crate::hp::ParryHotter::another_number_fn(u64_3, u64_2);
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_5, b: i32_4};
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_6: i32 = crate::hp::ParryHotter::accio(parryhotter_1_ref_0, i32_3, i32_2);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_0_ref_0, usize_0, u64_1, u64_0);
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4957() {
    rusty_monitor::set_test_id(4957);
    let mut i32_0: i32 = 4248i32;
    let mut i32_1: i32 = -2566i32;
    let mut i32_2: i32 = -11138i32;
    let mut i32_3: i32 = 14179i32;
    let mut i32_4: i32 = -26879i32;
    let mut i32_5: i32 = -9633i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_5, i32_4);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_6: i32 = -3232i32;
    let mut i32_7: i32 = -22882i32;
    let mut i32_8: i32 = 6604i32;
    let mut i32_9: i32 = 3486i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_9, b: i32_8};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut u64_0: u64 = 9206u64;
    let mut u64_1: u64 = 4498u64;
    let mut i64_0: i64 = 9538i64;
    let mut i64_1: i64 = 10203i64;
    let mut u16_0: u16 = 1422u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut i64_2: i64 = 2817i64;
    let mut i64_3: i64 = 10712i64;
    let mut u16_1: u16 = 1828u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_3};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_2);
    crate::hp::RomTiddle::foo3(romtiddle_0_ref_0, i64_1, i64_0);
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    crate::hp::ParryHotter::foo2(parryhotter_1_ref_0, i32_7, i32_6);
    let mut i32_10: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_3, i32_2);
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_1, b: i32_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4968() {
    rusty_monitor::set_test_id(4968);
    let mut u16_0: u16 = 1126u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut u64_0: u64 = 5637u64;
    let mut u64_1: u64 = 4658u64;
    let mut u64_2: u64 = 2272u64;
    let mut u64_3: u64 = 3843u64;
    let mut usize_0: usize = 7895usize;
    let mut i32_0: i32 = -1077i32;
    let mut i32_1: i32 = 5082i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_2: i32 = 3886i32;
    let mut i32_3: i32 = -4140i32;
    let mut u16_1: u16 = 4790u16;
    let mut i32_4: i32 = 9569i32;
    let mut i32_5: i32 = -21074i32;
    let mut i32_6: i32 = 4383i32;
    let mut i32_7: i32 = -1836i32;
    let mut i32_8: i32 = -5496i32;
    let mut i32_9: i32 = 2436i32;
    let mut i32_10: i32 = -10315i32;
    let mut i32_11: i32 = -78i32;
    let mut i64_0: i64 = -12527i64;
    let mut i64_1: i64 = 1285i64;
    let mut u16_2: u16 = 2849u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_1};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut u16_3: u16 = 8904u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut usize_1: usize = 226usize;
    let mut i32_12: i32 = 1002i32;
    let mut i32_13: i32 = 7339i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_13, b: i32_12};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_14: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_1_ref_0, usize_1, string_1);
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_0);
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_11, b: i32_10};
    crate::hp::ParryHotter::alohomora(i32_9, i32_8, i32_7, i32_6);
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    crate::hp::ParryHotter::foo2(parryhotter_2_ref_0, i32_5, i32_4);
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_0_ref_0, usize_0, u64_3, u64_2);
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4969() {
    rusty_monitor::set_test_id(4969);
    let mut i32_0: i32 = -6261i32;
    let mut i32_1: i32 = 6158i32;
    let mut i32_2: i32 = -15471i32;
    let mut i32_3: i32 = -13072i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u64_0: u64 = 1081u64;
    let mut u64_1: u64 = 9553u64;
    let mut usize_0: usize = 639usize;
    let mut i32_4: i32 = -14783i32;
    let mut i32_5: i32 = 7652i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_5, i32_4);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i64_0: i64 = 6929i64;
    let mut u16_0: u16 = 4683u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut u64_2: u64 = 5794u64;
    let mut u64_3: u64 = 5929u64;
    let mut u64_4: u64 = 1576u64;
    let mut u64_5: u64 = 5107u64;
    let mut str_0: &str = "45zqUPzkRvMffwHpw";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_1: i64 = 5867i64;
    let mut u16_1: u16 = 397u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_1};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
    crate::hp::ParryHotter::another_number_fn(u64_5, u64_4);
    crate::hp::ParryHotter::another_number_fn(u64_3, u64_2);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_1_ref_0, usize_0, u64_1, u64_0);
    crate::hp::ParryHotter::foo2(parryhotter_0_ref_0, i32_1, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4972() {
    rusty_monitor::set_test_id(4972);
    let mut str_0: &str = "lr5asMB01";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_0: i64 = 826i64;
    let mut u16_0: u16 = 5242u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut i64_1: i64 = 7392i64;
    let mut i64_2: i64 = -3441i64;
    let mut u16_1: u16 = 3976u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_2};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut i32_0: i32 = 5769i32;
    let mut i32_1: i32 = -2862i32;
    let mut i32_2: i32 = 6521i32;
    let mut i32_3: i32 = 4310i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u16_2: u16 = 5819u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut u16_3: u16 = 9402u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut i32_4: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_1, i32_0);
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_1);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut wonreasley_1_ref_0: &crate::hp::WonReasley = &mut wonreasley_1;
    crate::hp::WonReasley::arania_exumai(wonreasley_1_ref_0, str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4973() {
    rusty_monitor::set_test_id(4973);
    let mut i64_0: i64 = 2680i64;
    let mut i64_1: i64 = -11400i64;
    let mut u16_0: u16 = 8099u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut u64_0: u64 = 5210u64;
    let mut u64_1: u64 = 2525u64;
    let mut u64_2: u64 = 5139u64;
    let mut u64_3: u64 = 6705u64;
    let mut usize_0: usize = 5832usize;
    let mut i32_0: i32 = 12971i32;
    let mut i32_1: i32 = -3718i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_1, b: i32_0};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_2: i32 = 1723i32;
    let mut i32_3: i32 = -6529i32;
    let mut i32_4: i32 = 4015i32;
    let mut i32_5: i32 = -8197i32;
    let mut i32_6: i32 = 2900i32;
    let mut i32_7: i32 = 4973i32;
    let mut i32_8: i32 = -7625i32;
    let mut i32_9: i32 = 13374i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_9, b: i32_8};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut u16_1: u16 = 8730u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut usize_1: usize = 4950usize;
    let mut i32_10: i32 = -10759i32;
    let mut i32_11: i32 = 5594i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_11, i32_10);
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut u16_2: u16 = 2663u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut u64_4: u64 = 6954u64;
    let mut u64_5: u64 = 8423u64;
    let mut usize_2: usize = 7133usize;
    let mut i64_2: i64 = 12122i64;
    let mut i64_3: i64 = 7483i64;
    let mut u16_3: u16 = 8360u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut u64_6: u64 = 5565u64;
    let mut u64_7: u64 = 7072u64;
    let mut i32_12: i32 = -3934i32;
    let mut i32_13: i32 = 2742i32;
    let mut u64_8: u64 = 2669u64;
    let mut u64_9: u64 = 9312u64;
    let mut usize_3: usize = 287usize;
    let mut i32_14: i32 = -18635i32;
    let mut i32_15: i32 = 1686i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_15, b: i32_14};
    let mut parryhotter_3_ref_0: &crate::hp::ParryHotter = &mut parryhotter_3;
    let mut i32_16: i32 = 12119i32;
    let mut i32_17: i32 = 273i32;
    let mut i32_18: i32 = -1868i32;
    let mut i32_19: i32 = 6210i32;
    let mut parryhotter_4: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_19, b: i32_18};
    let mut parryhotter_4_ref_0: &crate::hp::ParryHotter = &mut parryhotter_4;
    let mut i32_20: i32 = crate::hp::ParryHotter::accio(parryhotter_4_ref_0, i32_17, i32_16);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_3_ref_0, usize_3, u64_9, u64_8);
    let mut parryhotter_5: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_13, i32_12);
    crate::hp::ParryHotter::another_number_fn(u64_7, u64_6);
    crate::hp::RomTiddle::foo3(romtiddle_3_ref_0, i64_3, i64_2);
    let mut parryhotter_5_ref_0: &crate::hp::ParryHotter = &mut parryhotter_5;
    let mut bool_1: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_5_ref_0, usize_2, u64_5, u64_4);
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut i32_21: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_2_ref_0, usize_1, string_0);
    crate::hp::ParryHotter::foo2(parryhotter_1_ref_0, i32_7, i32_6);
    crate::hp::ParryHotter::alohomora(i32_5, i32_4, i32_3, i32_2);
    let mut bool_2: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_0_ref_0, usize_0, u64_3, u64_2);
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    crate::hp::RomTiddle::foo3(romtiddle_0_ref_0, i64_1, i64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4977() {
    rusty_monitor::set_test_id(4977);
    let mut u16_0: u16 = 198u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut usize_0: usize = 6776usize;
    let mut i32_0: i32 = 12531i32;
    let mut i32_1: i32 = -10320i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u16_1: u16 = 3356u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut usize_1: usize = 1941usize;
    let mut i32_2: i32 = -14743i32;
    let mut i32_3: i32 = 973i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_4: i32 = -12784i32;
    let mut i32_5: i32 = 6156i32;
    let mut i32_6: i32 = 4413i32;
    let mut i32_7: i32 = -4087i32;
    let mut i64_0: i64 = 7262i64;
    let mut i64_1: i64 = 4463i64;
    let mut u16_2: u16 = 157u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut u16_3: u16 = 3546u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    crate::hp::RomTiddle::foo3(romtiddle_2_ref_0, i64_1, i64_0);
    crate::hp::ParryHotter::alohomora(i32_7, i32_6, i32_5, i32_4);
    let mut i32_8: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_1_ref_0, usize_1, string_1);
    let mut i32_9: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_0_ref_0, usize_0, string_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4986() {
    rusty_monitor::set_test_id(4986);
    let mut i32_0: i32 = 6251i32;
    let mut i32_1: i32 = -340i32;
    let mut i32_2: i32 = -22227i32;
    let mut i32_3: i32 = 1156i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i64_0: i64 = 4202i64;
    let mut u16_0: u16 = 2655u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut i64_1: i64 = 1022i64;
    let mut u16_1: u16 = 9508u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut i64_2: i64 = -6949i64;
    let mut i64_3: i64 = -8036i64;
    let mut u16_2: u16 = 7526u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut i64_4: i64 = 6755i64;
    let mut i64_5: i64 = -3310i64;
    let mut u16_3: u16 = 1279u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_5};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut u64_0: u64 = 7292u64;
    let mut u64_1: u64 = 7893u64;
    let mut usize_0: usize = 7841usize;
    let mut i32_4: i32 = -11867i32;
    let mut i32_5: i32 = -16886i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_5, b: i32_4};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_6: i32 = 14197i32;
    let mut i32_7: i32 = -7070i32;
    let mut i32_8: i32 = 206i32;
    let mut i32_9: i32 = -16531i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_9, i32_8);
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i32_10: i32 = crate::hp::ParryHotter::accio(parryhotter_2_ref_0, i32_7, i32_6);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_1_ref_0, usize_0, u64_1, u64_0);
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_4);
    crate::hp::RomTiddle::foo3(romtiddle_2_ref_0, i64_3, i64_2);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_1};
    let mut wonreasley_2: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut i32_11: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_1, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4987() {
    rusty_monitor::set_test_id(4987);
    let mut i32_0: i32 = -13806i32;
    let mut i32_1: i32 = -9765i32;
    let mut i32_2: i32 = 14725i32;
    let mut i32_3: i32 = 4253i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_4: i32 = -899i32;
    let mut i32_5: i32 = -19371i32;
    let mut u64_0: u64 = 5928u64;
    let mut u64_1: u64 = 4453u64;
    let mut i32_6: i32 = -12701i32;
    let mut i32_7: i32 = 6070i32;
    let mut i64_0: i64 = 8675i64;
    let mut i64_1: i64 = -7983i64;
    let mut u16_0: u16 = 92u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_1};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut u16_1: u16 = 1096u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut usize_0: usize = 934usize;
    let mut i32_8: i32 = -20i32;
    let mut i32_9: i32 = -18538i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_9, b: i32_8};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut u16_2: u16 = 3231u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut usize_1: usize = 3225usize;
    let mut i32_10: i32 = 14457i32;
    let mut i32_11: i32 = 14590i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_11, b: i32_10};
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut str_0: &str = "MgcMxVJwmayFP";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_2: i64 = 10173i64;
    let mut u16_3: u16 = 4519u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_3, y: i64_2};
    let mut wonreasley_1_ref_0: &crate::hp::WonReasley = &mut wonreasley_1;
    let mut i64_3: i64 = 6848i64;
    let mut i64_4: i64 = 11311i64;
    let mut u16_4: u16 = 4143u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut string_4: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    let mut wonreasley_2: crate::hp::WonReasley = crate::hp::WonReasley {x: string_4, y: i64_4};
    let mut wonreasley_2_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_2;
    let mut str_1: &str = "EHh7Z9oECAIl";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut i64_5: i64 = 15453i64;
    let mut u16_5: u16 = 2152u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_5_ref_0: &crate::hp::RomTiddle = &mut romtiddle_5;
    let mut string_5: std::string::String = crate::hp::RomTiddle::name(romtiddle_5_ref_0);
    let mut wonreasley_3: crate::hp::WonReasley = crate::hp::WonReasley {x: string_5, y: i64_5};
    let mut wonreasley_3_ref_0: &crate::hp::WonReasley = &mut wonreasley_3;
    let mut i32_12: i32 = 1230i32;
    let mut i32_13: i32 = -14621i32;
    let mut i32_14: i32 = 2695i32;
    let mut i32_15: i32 = 27720i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_15, i32_14);
    let mut parryhotter_3_ref_0: &crate::hp::ParryHotter = &mut parryhotter_3;
    let mut i32_16: i32 = crate::hp::ParryHotter::accio(parryhotter_3_ref_0, i32_13, i32_12);
    crate::hp::WonReasley::arania_exumai(wonreasley_3_ref_0, str_1_ref_0);
    crate::hp::WonReasley::ascendio(wonreasley_2_ref_0, i64_3);
    crate::hp::WonReasley::arania_exumai(wonreasley_1_ref_0, str_0_ref_0);
    let mut i32_17: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_2_ref_0, usize_1, string_2);
    let mut i32_18: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_1_ref_0, usize_0, string_1);
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_0);
    let mut parryhotter_4: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_7, b: i32_6};
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    let mut parryhotter_4_ref_0: &crate::hp::ParryHotter = &mut parryhotter_4;
    crate::hp::ParryHotter::foo2(parryhotter_4_ref_0, i32_5, i32_4);
    crate::hp::ParryHotter::foo2(parryhotter_0_ref_0, i32_1, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4991() {
    rusty_monitor::set_test_id(4991);
    let mut i64_0: i64 = 107i64;
    let mut i64_1: i64 = 4951i64;
    let mut u16_0: u16 = 5940u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_1};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut u16_1: u16 = 448u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut usize_0: usize = 6421usize;
    let mut i32_0: i32 = -4651i32;
    let mut i32_1: i32 = 2146i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_1, b: i32_0};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i64_2: i64 = -5973i64;
    let mut i64_3: i64 = -15664i64;
    let mut u16_2: u16 = 9799u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_3};
    let mut wonreasley_1_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_1;
    let mut u16_3: u16 = 1799u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut usize_1: usize = 878usize;
    let mut i32_2: i32 = -9285i32;
    let mut i32_3: i32 = 21372i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i64_4: i64 = 5043i64;
    let mut i64_5: i64 = 2274i64;
    let mut u16_4: u16 = 399u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut string_4: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    let mut wonreasley_2: crate::hp::WonReasley = crate::hp::WonReasley {x: string_4, y: i64_5};
    let mut wonreasley_2_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_2;
    let mut u64_0: u64 = 4697u64;
    let mut u64_1: u64 = 2497u64;
    let mut usize_2: usize = 8075usize;
    let mut i32_4: i32 = -5434i32;
    let mut i32_5: i32 = 12099i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_5, i32_4);
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i32_6: i32 = -2350i32;
    let mut i32_7: i32 = 563i32;
    let mut i32_8: i32 = 9550i32;
    let mut i32_9: i32 = -10715i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_9, b: i32_8};
    let mut parryhotter_3_ref_0: &crate::hp::ParryHotter = &mut parryhotter_3;
    let mut i32_10: i32 = crate::hp::ParryHotter::accio(parryhotter_3_ref_0, i32_7, i32_6);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_2_ref_0, usize_2, u64_1, u64_0);
    crate::hp::WonReasley::ascendio(wonreasley_2_ref_0, i64_4);
    let mut i32_11: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_1_ref_0, usize_1, string_3);
    crate::hp::WonReasley::ascendio(wonreasley_1_ref_0, i64_2);
    let mut i32_12: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_0_ref_0, usize_0, string_1);
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4999() {
    rusty_monitor::set_test_id(4999);
    let mut u16_0: u16 = 7739u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut u16_1: u16 = 1667u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_1;
    let mut i64_0: i64 = -7516i64;
    let mut i64_1: i64 = -5372i64;
    let mut u16_2: u16 = 3267u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut usize_0: usize = 1105usize;
    let mut i32_0: i32 = 17358i32;
    let mut i32_1: i32 = -6560i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_1, b: i32_0};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i64_2: i64 = -1715i64;
    let mut i64_3: i64 = 5074i64;
    let mut u16_3: u16 = 3574u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut u16_4: u16 = 4447u16;
    let mut u16_5: u16 = 7441u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    let mut usize_1: usize = 1162usize;
    let mut i32_2: i32 = 8452i32;
    let mut i32_3: i32 = 21455i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i64_4: i64 = -14221i64;
    let mut i64_5: i64 = -8252i64;
    let mut u16_6: u16 = 9278u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    let mut romtiddle_5_ref_0: &crate::hp::RomTiddle = &mut romtiddle_5;
    crate::hp::RomTiddle::foo3(romtiddle_5_ref_0, i64_5, i64_4);
    let mut i32_4: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_1_ref_0, usize_1, string_2);
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    crate::hp::RomTiddle::foo3(romtiddle_3_ref_0, i64_3, i64_2);
    let mut i32_5: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_0_ref_0, usize_0, string_1);
    let mut romtiddle_6_ref_0: &crate::hp::RomTiddle = &mut romtiddle_6;
    crate::hp::RomTiddle::foo3(romtiddle_6_ref_0, i64_1, i64_0);
    crate::hp::RomTiddle::avada_kedavra(romtiddle_1_ref_0, string_0);
    panic!("From RustyUnit with love");
}
}