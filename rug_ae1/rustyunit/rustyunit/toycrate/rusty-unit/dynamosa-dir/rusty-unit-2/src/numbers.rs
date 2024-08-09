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
fn rusty_test_6261() {
    rusty_monitor::set_test_id(6261);
    let mut i32_0: i32 = -4921i32;
    let mut i32_1: i32 = 15104i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_1, b: i32_0};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_2: i32 = -2485i32;
    let mut i32_3: i32 = 4550i32;
    let mut i32_4: i32 = -4225i32;
    let mut i32_5: i32 = -7515i32;
    let mut i32_6: i32 = 6130i32;
    let mut u16_0: u16 = 3097u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut u16_1: u16 = 6900u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_1;
    let mut u16_2: u16 = 268u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut i32_7: i32 = -22100i32;
    let mut i32_8: i32 = -5518i32;
    let mut i32_9: i32 = -13548i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_9, b: i32_8};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut u64_0: u64 = 1419u64;
    let mut u64_1: u64 = 1943u64;
    let mut usize_0: usize = 9331usize;
    let mut i32_10: i32 = 2565i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_6, i32_10);
    let mut i64_0: i64 = -4435i64;
    let mut i64_1: i64 = 19750i64;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_1};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut str_0: &str = "hfA8I5C7rVZ";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_2: i64 = -8668i64;
    let mut u16_3: u16 = 2981u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_2};
    let mut wonreasley_1_ref_0: &crate::hp::WonReasley = &mut wonreasley_1;
    let mut u16_4: u16 = 325u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut u16_5: u16 = 8495u16;
    let mut i64_3: i64 = 2973i64;
    let mut u16_6: u16 = 3382u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    let mut romtiddle_5_ref_0: &crate::hp::RomTiddle = &mut romtiddle_5;
    let mut u64_2: u64 = 3526u64;
    let mut u64_3: u64 = 8200u64;
    let mut i32_11: i32 = 7744i32;
    let mut i32_12: i32 = -2661i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_12, b: i32_11};
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i64_4: i64 = 5111i64;
    let mut u16_7: u16 = 8911u16;
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_7};
    let mut romtiddle_6_ref_0: &crate::hp::RomTiddle = &mut romtiddle_6;
    let mut i32_13: i32 = -19732i32;
    let mut i32_14: i32 = 3064i32;
    let mut romtiddle_7: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    crate::hp::RomTiddle::foo3(romtiddle_6_ref_0, i64_4, i64_3);
    crate::hp::WonReasley::arania_exumai(wonreasley_1_ref_0, str_0_ref_0);
    crate::hp::ParryHotter::foo2(parryhotter_2_ref_0, i32_14, i32_13);
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    crate::hp::ParryHotter::another_number_fn(u64_2, u64_0);
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_0);
    let mut parryhotter_3_ref_0: &crate::hp::ParryHotter = &mut parryhotter_3;
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_1_ref_0, usize_0, u64_1, u64_3);
    let mut i32_15: i32 = crate::hp::ParryHotter::accio(parryhotter_3_ref_0, i32_3, i32_2);
    crate::hp::RomTiddle::avada_kedavra(romtiddle_1_ref_0, string_3);
    crate::hp::ParryHotter::alohomora(i32_7, i32_15, i32_5, i32_4);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_584() {
    rusty_monitor::set_test_id(584);
    let mut u16_0: u16 = 7280u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut u16_1: u16 = 413u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut i32_0: i32 = -16365i32;
    let mut i32_1: i32 = -3419i32;
    let mut i32_2: i32 = -1145i32;
    let mut i32_3: i32 = 16222i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_4: i32 = 5176i32;
    let mut i32_5: i32 = -176i32;
    let mut u64_0: u64 = 8530u64;
    let mut u64_1: u64 = 75u64;
    let mut u64_2: u64 = 8820u64;
    let mut u64_3: u64 = 5663u64;
    crate::hp::ParryHotter::another_number_fn(u64_3, u64_2);
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_5, i32_4);
    crate::hp::ParryHotter::foo2(parryhotter_0_ref_0, i32_1, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6466() {
    rusty_monitor::set_test_id(6466);
    let mut i32_0: i32 = 14266i32;
    let mut i32_1: i32 = -4397i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut i64_0: i64 = 12294i64;
    let mut u16_0: u16 = 1041u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut u16_1: u16 = 4536u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut u16_2: u16 = 4996u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut u16_3: u16 = 0u16;
    let mut usize_0: usize = 9881usize;
    let mut i32_2: i32 = -5694i32;
    let mut i32_3: i32 = 1042i32;
    let mut str_0: &str = "slPLDFj7o5";
    let mut i64_1: i64 = -2125i64;
    let mut u16_4: u16 = 1292u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut u16_5: u16 = 5670u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut i64_2: i64 = 5610i64;
    let mut u16_6: u16 = 2939u16;
    let mut u16_7: u16 = 106u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut u16_8: u16 = 1903u16;
    let mut i64_3: i64 = -9277i64;
    let mut u64_0: u64 = 4132u64;
    let mut u64_1: u64 = 5257u64;
    let mut i32_4: i32 = 1965i32;
    let mut i32_5: i32 = -13181i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_2, b: i32_4};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u16_9: u16 = 3097u16;
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut romtiddle_7: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_8};
    let mut romtiddle_5_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_5;
    let mut u16_10: u16 = 3290u16;
    let mut romtiddle_8: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_7};
    let mut romtiddle_6_ref_0: &crate::hp::RomTiddle = &mut romtiddle_6;
    let mut i32_6: i32 = -22100i32;
    let mut i32_7: i32 = -32515i32;
    let mut i32_8: i32 = -13548i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_6, b: i32_5};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i64_4: i64 = -4435i64;
    let mut i64_5: i64 = 20575i64;
    let mut u16_11: u16 = 9370u16;
    let mut romtiddle_9: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_10};
    let mut romtiddle_7_ref_0: &crate::hp::RomTiddle = &mut romtiddle_7;
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_6_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_2};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u16_12: u16 = 2981u16;
    let mut romtiddle_10: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_9};
    let mut romtiddle_9_ref_0: &crate::hp::RomTiddle = &mut romtiddle_9;
    let mut string_4: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut wonreasley_2: crate::hp::WonReasley = crate::hp::WonReasley {x: string_3, y: i64_4};
    let mut wonreasley_1_ref_0: &crate::hp::WonReasley = &mut wonreasley_1;
    let mut romtiddle_11: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_11};
    let mut u16_13: u16 = 8495u16;
    let mut i64_6: i64 = 2395i64;
    let mut i64_7: i64 = -4531i64;
    let mut u16_14: u16 = 3382u16;
    let mut romtiddle_12: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_13};
    let mut romtiddle_8_ref_0: &crate::hp::RomTiddle = &mut romtiddle_8;
    let mut string_5: std::string::String = crate::hp::RomTiddle::name(romtiddle_9_ref_0);
    let mut wonreasley_3: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_1};
    let mut wonreasley_2_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_2;
    let mut u64_2: u64 = 3526u64;
    let mut u64_3: u64 = 8200u64;
    let mut i32_9: i32 = -2661i32;
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut romtiddle_13: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_12};
    let mut romtiddle_10_ref_0: &crate::hp::RomTiddle = &mut romtiddle_10;
    let mut romtiddle_14: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_14};
    let mut romtiddle_13_ref_0: &crate::hp::RomTiddle = &mut romtiddle_13;
    crate::hp::RomTiddle::foo3(romtiddle_10_ref_0, i64_6, i64_5);
    crate::hp::WonReasley::arania_exumai(wonreasley_1_ref_0, str_0_ref_0);
    crate::hp::ParryHotter::foo2(parryhotter_1_ref_0, i32_7, i32_9);
    let mut string_6: std::string::String = crate::hp::RomTiddle::name(romtiddle_7_ref_0);
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_3);
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_7);
    let mut wonreasley_4: crate::hp::WonReasley = crate::hp::WonReasley {x: string_6, y: i64_3};
    let mut string_7: std::string::String = crate::hp::RomTiddle::name(romtiddle_8_ref_0);
    let mut i32_10: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_8, i32_3);
    let mut romtiddle_14_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_14;
    crate::hp::RomTiddle::avada_kedavra(romtiddle_5_ref_0, string_4);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_2_ref_0, usize_0, u64_0, u64_2);
    panic!("From RustyUnit with love");
}
}