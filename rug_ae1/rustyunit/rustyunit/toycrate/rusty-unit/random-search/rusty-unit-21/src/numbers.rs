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
    let mut i32_0: i32 = 6701i32;
    let mut i32_1: i32 = -26423i32;
    let mut i32_2: i32 = -1460i32;
    let mut i32_3: i32 = -3245i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_4: i32 = 2351i32;
    let mut i32_5: i32 = 5712i32;
    let mut i32_6: i32 = 1227i32;
    let mut i32_7: i32 = 9633i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_7, b: i32_6};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_8: i32 = -23609i32;
    let mut i32_9: i32 = -7883i32;
    let mut u64_0: u64 = 4979u64;
    let mut u64_1: u64 = 9899u64;
    let mut u16_0: u16 = 9518u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut usize_0: usize = 5208usize;
    let mut i32_10: i32 = -7085i32;
    let mut i32_11: i32 = 728i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_11, b: i32_10};
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i64_0: i64 = -549i64;
    let mut i64_1: i64 = -10163i64;
    let mut u16_1: u16 = 3679u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_1};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut i64_2: i64 = 8618i64;
    let mut u16_2: u16 = 3342u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut u16_3: u16 = 2338u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut u16_4: u16 = 3100u16;
    let mut u16_5: u16 = 9535u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    let mut u16_6: u16 = 5352u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    let mut romtiddle_5_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_5;
    let mut i32_12: i32 = 10893i32;
    let mut i32_13: i32 = -4655i32;
    let mut i32_14: i32 = 15204i32;
    let mut i32_15: i32 = -5918i32;
    crate::hp::ParryHotter::alohomora(i32_15, i32_14, i32_13, i32_12);
    crate::hp::RomTiddle::avada_kedavra(romtiddle_5_ref_0, string_3);
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut string_4: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_2};
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_0);
    let mut i32_16: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_2_ref_0, usize_0, string_0);
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_9, b: i32_8};
    crate::hp::ParryHotter::foo2(parryhotter_1_ref_0, i32_5, i32_4);
    let mut i32_17: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_1, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4953() {
    rusty_monitor::set_test_id(4953);
    let mut i32_0: i32 = -7534i32;
    let mut i32_1: i32 = 1688i32;
    let mut i32_2: i32 = -11106i32;
    let mut i32_3: i32 = 3470i32;
    let mut i32_4: i32 = 7681i32;
    let mut i32_5: i32 = -8876i32;
    let mut i32_6: i32 = -15965i32;
    let mut i32_7: i32 = 29280i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_7, i32_6);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_8: i32 = -24133i32;
    let mut i32_9: i32 = -10973i32;
    let mut i32_10: i32 = 5346i32;
    let mut i32_11: i32 = 1090i32;
    let mut i32_12: i32 = -3389i32;
    let mut i32_13: i32 = 4299i32;
    let mut u16_0: u16 = 3343u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut usize_0: usize = 732usize;
    let mut i32_14: i32 = 2831i32;
    let mut i32_15: i32 = -12234i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_15, b: i32_14};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut u64_0: u64 = 5605u64;
    let mut u64_1: u64 = 4793u64;
    let mut usize_1: usize = 9435usize;
    let mut u64_2: u64 = 2160u64;
    let mut u64_3: u64 = 3888u64;
    let mut i32_16: i32 = -4648i32;
    let mut i32_17: i32 = -11521i32;
    let mut u16_1: u16 = 1653u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut usize_2: usize = 5062usize;
    let mut i32_18: i32 = -17305i32;
    let mut i32_19: i32 = 7731i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_19, b: i32_18};
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i32_20: i32 = 3867i32;
    let mut i32_21: i32 = 8242i32;
    let mut i32_22: i32 = -17884i32;
    let mut i32_23: i32 = -9633i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_23, b: i32_22};
    let mut parryhotter_3_ref_0: &crate::hp::ParryHotter = &mut parryhotter_3;
    crate::hp::ParryHotter::foo2(parryhotter_3_ref_0, i32_21, i32_20);
    let mut i32_24: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_2_ref_0, usize_2, string_1);
    let mut parryhotter_4: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_17, b: i32_16};
    crate::hp::ParryHotter::another_number_fn(u64_3, u64_2);
    let mut parryhotter_4_ref_0: &crate::hp::ParryHotter = &mut parryhotter_4;
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_4_ref_0, usize_1, u64_1, u64_0);
    let mut i32_25: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_1_ref_0, usize_0, string_0);
    crate::hp::ParryHotter::alohomora(i32_13, i32_12, i32_11, i32_10);
    let mut parryhotter_5: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_9, i32_8);
    crate::hp::ParryHotter::foo2(parryhotter_0_ref_0, i32_5, i32_4);
    let mut parryhotter_5_ref_0: &crate::hp::ParryHotter = &mut parryhotter_5;
    crate::hp::ParryHotter::foo2(parryhotter_5_ref_0, i32_3, i32_2);
    let mut parryhotter_6: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4955() {
    rusty_monitor::set_test_id(4955);
    let mut i32_0: i32 = -8205i32;
    let mut i32_1: i32 = -18379i32;
    let mut i32_2: i32 = -926i32;
    let mut i32_3: i32 = 2575i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_4: i32 = 10634i32;
    let mut i32_5: i32 = -12290i32;
    let mut i32_6: i32 = -24073i32;
    let mut i32_7: i32 = 2057i32;
    let mut u16_0: u16 = 6737u16;
    let mut u16_1: u16 = 6572u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut u16_2: u16 = 1745u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_1_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_1;
    let mut i32_8: i32 = 23650i32;
    let mut i32_9: i32 = -537i32;
    let mut i32_10: i32 = 1342i32;
    let mut i32_11: i32 = -1376i32;
    let mut u16_3: u16 = 7050u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut u16_4: u16 = 4503u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_3_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_3;
    let mut u64_0: u64 = 9231u64;
    let mut u64_1: u64 = 7758u64;
    let mut u16_5: u16 = 9287u16;
    let mut i32_12: i32 = -14971i32;
    let mut i32_13: i32 = -1778i32;
    let mut i32_14: i32 = -5680i32;
    let mut i32_15: i32 = 4347i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_15, i32_14);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut u64_2: u64 = 7802u64;
    let mut u64_3: u64 = 2750u64;
    crate::hp::ParryHotter::another_number_fn(u64_3, u64_2);
    crate::hp::ParryHotter::foo2(parryhotter_1_ref_0, i32_13, i32_12);
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    crate::hp::RomTiddle::avada_kedavra(romtiddle_3_ref_0, string_1);
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_11, b: i32_10};
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_9, i32_8);
    crate::hp::RomTiddle::avada_kedavra(romtiddle_1_ref_0, string_0);
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    crate::hp::ParryHotter::alohomora(i32_7, i32_6, i32_5, i32_4);
    crate::hp::ParryHotter::foo2(parryhotter_0_ref_0, i32_1, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4957() {
    rusty_monitor::set_test_id(4957);
    let mut i32_0: i32 = -7997i32;
    let mut i32_1: i32 = 1085i32;
    let mut i32_2: i32 = 2719i32;
    let mut i32_3: i32 = 4825i32;
    let mut u64_0: u64 = 361u64;
    let mut u64_1: u64 = 3980u64;
    let mut usize_0: usize = 7955usize;
    let mut i32_4: i32 = -9472i32;
    let mut i32_5: i32 = -14572i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_5, b: i32_4};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_6: i32 = -4197i32;
    let mut i32_7: i32 = 2112i32;
    let mut i32_8: i32 = -6959i32;
    let mut i32_9: i32 = 101i32;
    let mut i64_0: i64 = 6167i64;
    let mut i64_1: i64 = 2816i64;
    let mut u16_0: u16 = 3745u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_1};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut u16_1: u16 = 7847u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut i32_10: i32 = -3363i32;
    let mut i32_11: i32 = 7373i32;
    let mut i32_12: i32 = -1132i32;
    let mut i32_13: i32 = 3120i32;
    let mut i32_14: i32 = -5484i32;
    let mut i32_15: i32 = 1293i32;
    let mut i32_16: i32 = 285i32;
    let mut i32_17: i32 = -759i32;
    let mut i32_18: i32 = -7258i32;
    let mut i32_19: i32 = -3287i32;
    let mut u16_2: u16 = 6082u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut u16_3: u16 = 9382u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut u16_4: u16 = 4186u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_4_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_4;
    let mut i64_2: i64 = 22420i64;
    let mut u16_5: u16 = 2526u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_5_ref_0: &crate::hp::RomTiddle = &mut romtiddle_5;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_5_ref_0);
    let mut i32_20: i32 = -18766i32;
    let mut i32_21: i32 = -2956i32;
    let mut i64_3: i64 = 11603i64;
    let mut i64_4: i64 = -1095i64;
    let mut u16_6: u16 = 6666u16;
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    let mut romtiddle_6_ref_0: &crate::hp::RomTiddle = &mut romtiddle_6;
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_6_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_3, y: i64_4};
    let mut wonreasley_1_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_1;
    let mut u64_2: u64 = 1201u64;
    let mut u64_3: u64 = 4809u64;
    let mut usize_1: usize = 1631usize;
    let mut i32_22: i32 = -19917i32;
    let mut i32_23: i32 = 14260i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_23, i32_22);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_1_ref_0, usize_1, u64_3, u64_2);
    crate::hp::WonReasley::ascendio(wonreasley_1_ref_0, i64_3);
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_21, i32_20);
    let mut wonreasley_2: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_2};
    crate::hp::RomTiddle::avada_kedavra(romtiddle_4_ref_0, string_1);
    let mut string_4: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    crate::hp::ParryHotter::alohomora(i32_19, i32_18, i32_17, i32_16);
    crate::hp::ParryHotter::alohomora(i32_15, i32_14, i32_13, i32_12);
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_11, b: i32_10};
    let mut string_5: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_0);
    crate::hp::ParryHotter::alohomora(i32_9, i32_8, i32_7, i32_6);
    let mut bool_1: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_0_ref_0, usize_0, u64_1, u64_0);
    crate::hp::ParryHotter::alohomora(i32_3, i32_2, i32_1, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4961() {
    rusty_monitor::set_test_id(4961);
    let mut i32_0: i32 = -21784i32;
    let mut i32_1: i32 = 10095i32;
    let mut i32_2: i32 = -11394i32;
    let mut i32_3: i32 = 5470i32;
    let mut i32_4: i32 = 3681i32;
    let mut i32_5: i32 = 9237i32;
    let mut u16_0: u16 = 5738u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut u64_0: u64 = 3915u64;
    let mut u64_1: u64 = 1121u64;
    let mut i64_0: i64 = 25316i64;
    let mut u16_1: u16 = 6087u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut i64_1: i64 = 11964i64;
    let mut i64_2: i64 = -9848i64;
    let mut u16_2: u16 = 7611u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut u64_2: u64 = 6798u64;
    let mut u64_3: u64 = 1491u64;
    let mut usize_0: usize = 8738usize;
    let mut i32_6: i32 = -11266i32;
    let mut i32_7: i32 = 1038i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_7, b: i32_6};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_8: i32 = -17551i32;
    let mut i32_9: i32 = 11851i32;
    let mut i32_10: i32 = 9713i32;
    let mut i32_11: i32 = 26i32;
    let mut i32_12: i32 = 2488i32;
    let mut i32_13: i32 = 11453i32;
    let mut u64_4: u64 = 9560u64;
    let mut u64_5: u64 = 9695u64;
    let mut usize_1: usize = 1196usize;
    let mut i32_14: i32 = -27822i32;
    let mut i32_15: i32 = 14800i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_15, b: i32_14};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut str_0: &str = "y4WuRLXNFhtY4gD4";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_3: i64 = 6914i64;
    let mut u16_3: u16 = 7974u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_3};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    let mut i32_16: i32 = -2794i32;
    let mut i32_17: i32 = -5628i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_17, b: i32_16};
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_1_ref_0, usize_1, u64_5, u64_4);
    crate::hp::ParryHotter::alohomora(i32_13, i32_12, i32_11, i32_10);
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_9, i32_8);
    let mut bool_1: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_0_ref_0, usize_0, u64_3, u64_2);
    crate::hp::RomTiddle::foo3(romtiddle_2_ref_0, i64_2, i64_1);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut parryhotter_4: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_5, i32_4);
    crate::hp::ParryHotter::alohomora(i32_3, i32_2, i32_1, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4971() {
    rusty_monitor::set_test_id(4971);
    let mut u16_0: u16 = 6417u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut u16_1: u16 = 2299u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_1;
    let mut i32_0: i32 = 14966i32;
    let mut i32_1: i32 = -11084i32;
    let mut u16_2: u16 = 4965u16;
    let mut i32_2: i32 = 17475i32;
    let mut i32_3: i32 = -12984i32;
    let mut i32_4: i32 = 30282i32;
    let mut i32_5: i32 = 8175i32;
    let mut i32_6: i32 = 2866i32;
    let mut i32_7: i32 = 240i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_7, i32_6);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i64_0: i64 = -837i64;
    let mut i64_1: i64 = -8609i64;
    let mut u16_3: u16 = 3139u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut i64_2: i64 = 2620i64;
    let mut i64_3: i64 = -7793i64;
    let mut u16_4: u16 = 2593u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut i32_8: i32 = -457i32;
    let mut i32_9: i32 = -5029i32;
    let mut i32_10: i32 = 11241i32;
    let mut i32_11: i32 = 11960i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_11, i32_10);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_12: i32 = crate::hp::ParryHotter::accio(parryhotter_1_ref_0, i32_9, i32_8);
    crate::hp::RomTiddle::foo3(romtiddle_3_ref_0, i64_3, i64_2);
    crate::hp::RomTiddle::foo3(romtiddle_2_ref_0, i64_1, i64_0);
    crate::hp::ParryHotter::foo2(parryhotter_0_ref_0, i32_5, i32_4);
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_1, b: i32_0};
    crate::hp::RomTiddle::avada_kedavra(romtiddle_1_ref_0, string_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4980() {
    rusty_monitor::set_test_id(4980);
    let mut u16_0: u16 = 9121u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut i32_0: i32 = 9843i32;
    let mut i32_1: i32 = -752i32;
    let mut i64_0: i64 = 1179i64;
    let mut i64_1: i64 = 329i64;
    let mut u16_1: u16 = 6575u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_1};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut i64_2: i64 = -767i64;
    let mut u16_2: u16 = 7709u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut u64_0: u64 = 4922u64;
    let mut u64_1: u64 = 7916u64;
    let mut usize_0: usize = 2137usize;
    let mut i64_3: i64 = 8949i64;
    let mut u16_3: u16 = 6366u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut i64_4: i64 = 8193i64;
    let mut u16_4: u16 = 3650u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    let mut i32_2: i32 = 10647i32;
    let mut i32_3: i32 = 12418i32;
    let mut i32_4: i32 = -2233i32;
    let mut i32_5: i32 = -2977i32;
    let mut u64_2: u64 = 9228u64;
    let mut u64_3: u64 = 7857u64;
    let mut i32_6: i32 = 14269i32;
    let mut i32_7: i32 = -11958i32;
    let mut u16_5: u16 = 7792u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_5_ref_0: &crate::hp::RomTiddle = &mut romtiddle_5;
    let mut string_4: std::string::String = crate::hp::RomTiddle::name(romtiddle_5_ref_0);
    let mut usize_1: usize = 4417usize;
    let mut u64_4: u64 = 8590u64;
    let mut u64_5: u64 = 8838u64;
    let mut i32_8: i32 = -10276i32;
    let mut i32_9: i32 = -17691i32;
    let mut i32_10: i32 = -12456i32;
    let mut i32_11: i32 = -5303i32;
    let mut i32_12: i32 = 5266i32;
    let mut i32_13: i32 = -6391i32;
    let mut i32_14: i32 = 56i32;
    let mut i32_15: i32 = 7072i32;
    let mut u64_6: u64 = 554u64;
    let mut u64_7: u64 = 7387u64;
    let mut i32_16: i32 = 5866i32;
    let mut i32_17: i32 = 3533i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_17, b: i32_16};
    crate::hp::ParryHotter::another_number_fn(u64_7, u64_6);
    crate::hp::ParryHotter::alohomora(i32_15, i32_14, i32_13, i32_12);
    crate::hp::ParryHotter::alohomora(i32_11, i32_10, i32_9, i32_8);
    crate::hp::ParryHotter::another_number_fn(u64_5, u64_4);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_18: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_0_ref_0, usize_1, string_4);
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_7, i32_6);
    crate::hp::ParryHotter::another_number_fn(u64_3, u64_2);
    crate::hp::ParryHotter::alohomora(i32_5, i32_4, i32_3, i32_2);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_3, y: i64_4};
    let mut wonreasley_2: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_3};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_1_ref_0, usize_0, u64_1, u64_0);
    let mut wonreasley_3: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_2};
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_0);
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut string_5: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4985() {
    rusty_monitor::set_test_id(4985);
    let mut u16_0: u16 = 1647u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut u64_0: u64 = 7517u64;
    let mut u64_1: u64 = 3735u64;
    let mut i32_0: i32 = 1830i32;
    let mut i32_1: i32 = 4117i32;
    let mut i32_2: i32 = 17589i32;
    let mut i32_3: i32 = -7357i32;
    let mut u16_1: u16 = 503u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut usize_0: usize = 9705usize;
    let mut i32_4: i32 = 16014i32;
    let mut i32_5: i32 = -1308i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_5, b: i32_4};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i64_0: i64 = -3989i64;
    let mut i64_1: i64 = 3093i64;
    let mut u16_2: u16 = 9556u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut str_0: &str = "sy2UH93Z4aNnp";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_2: i64 = -3618i64;
    let mut u16_3: u16 = 6483u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_2};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    let mut u16_4: u16 = 4206u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    let mut usize_1: usize = 1392usize;
    let mut i32_6: i32 = 14978i32;
    let mut i32_7: i32 = 613i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_7, b: i32_6};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_8: i32 = 4961i32;
    let mut i32_9: i32 = 3943i32;
    let mut i32_10: i32 = -3684i32;
    let mut i32_11: i32 = -13162i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_11, i32_10);
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut u64_2: u64 = 930u64;
    let mut u64_3: u64 = 2467u64;
    let mut i32_12: i32 = 1282i32;
    let mut i32_13: i32 = -8931i32;
    let mut i32_14: i32 = -330i32;
    let mut i32_15: i32 = 17187i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_15, b: i32_14};
    let mut parryhotter_4: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_13, b: i32_12};
    crate::hp::ParryHotter::another_number_fn(u64_3, u64_2);
    let mut i32_16: i32 = crate::hp::ParryHotter::accio(parryhotter_2_ref_0, i32_9, i32_8);
    let mut i32_17: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_1_ref_0, usize_1, string_2);
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
    crate::hp::RomTiddle::foo3(romtiddle_2_ref_0, i64_1, i64_0);
    let mut i32_18: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_0_ref_0, usize_0, string_0);
    crate::hp::ParryHotter::alohomora(i32_3, i32_2, i32_1, i32_0);
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4990() {
    rusty_monitor::set_test_id(4990);
    let mut str_0: &str = "q";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_0: i64 = -15503i64;
    let mut u16_0: u16 = 3401u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    let mut i64_1: i64 = 7225i64;
    let mut i64_2: i64 = 25765i64;
    let mut u16_1: u16 = 1894u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut u64_0: u64 = 3412u64;
    let mut u64_1: u64 = 3237u64;
    let mut usize_0: usize = 860usize;
    let mut i32_0: i32 = 19532i32;
    let mut i32_1: i32 = -16317i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_2: i32 = -4695i32;
    let mut i32_3: i32 = -15585i32;
    let mut i32_4: i32 = -19588i32;
    let mut i32_5: i32 = 5467i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_5, b: i32_4};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut u16_2: u16 = 4236u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut i64_3: i64 = 1230i64;
    let mut i64_4: i64 = 15021i64;
    let mut u16_3: u16 = 2819u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut str_1: &str = "BgHxgs9w0tzNKgesb";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut i64_5: i64 = 16893i64;
    let mut u16_4: u16 = 6220u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_5};
    let mut wonreasley_1_ref_0: &crate::hp::WonReasley = &mut wonreasley_1;
    let mut u16_5: u16 = 8218u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_5_ref_0: &crate::hp::RomTiddle = &mut romtiddle_5;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_5_ref_0);
    crate::hp::WonReasley::arania_exumai(wonreasley_1_ref_0, str_1_ref_0);
    crate::hp::RomTiddle::foo3(romtiddle_3_ref_0, i64_4, i64_3);
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    crate::hp::ParryHotter::foo2(parryhotter_1_ref_0, i32_3, i32_2);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_0_ref_0, usize_0, u64_1, u64_0);
    crate::hp::RomTiddle::foo3(romtiddle_1_ref_0, i64_2, i64_1);
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4998() {
    rusty_monitor::set_test_id(4998);
    let mut i32_0: i32 = -913i32;
    let mut i32_1: i32 = -18025i32;
    let mut i32_2: i32 = -3343i32;
    let mut i32_3: i32 = -12725i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u16_0: u16 = 2252u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut u16_1: u16 = 801u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_1;
    let mut i64_0: i64 = 15662i64;
    let mut i64_1: i64 = 8106i64;
    let mut u16_2: u16 = 3677u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut i32_4: i32 = -6548i32;
    let mut i32_5: i32 = 6893i32;
    let mut i32_6: i32 = 8775i32;
    let mut i32_7: i32 = -49i32;
    let mut i64_2: i64 = 15864i64;
    let mut u16_3: u16 = 9594u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut u64_0: u64 = 8519u64;
    let mut u64_1: u64 = 7539u64;
    let mut usize_0: usize = 2271usize;
    let mut i32_8: i32 = -4067i32;
    let mut i32_9: i32 = 9477i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_9, i32_8);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut u16_4: u16 = 6252u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    let mut usize_1: usize = 1603usize;
    let mut u16_5: u16 = 2162u16;
    let mut u64_2: u64 = 6828u64;
    let mut u64_3: u64 = 9054u64;
    let mut i32_10: i32 = -9142i32;
    let mut i32_11: i32 = -27i32;
    let mut i32_12: i32 = 10769i32;
    let mut i32_13: i32 = 1868i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_13, b: i32_12};
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut u64_4: u64 = 2886u64;
    let mut u64_5: u64 = 5116u64;
    let mut i32_14: i32 = -3986i32;
    let mut i32_15: i32 = -19443i32;
    let mut u16_6: u16 = 589u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_15, b: i32_14};
    crate::hp::ParryHotter::another_number_fn(u64_5, u64_4);
    let mut i32_16: i32 = crate::hp::ParryHotter::accio(parryhotter_2_ref_0, i32_11, i32_10);
    crate::hp::ParryHotter::another_number_fn(u64_3, u64_2);
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut parryhotter_3_ref_0: &crate::hp::ParryHotter = &mut parryhotter_3;
    let mut i32_17: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_3_ref_0, usize_1, string_2);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_1_ref_0, usize_0, u64_1, u64_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_2};
    crate::hp::ParryHotter::alohomora(i32_7, i32_6, i32_5, i32_4);
    crate::hp::RomTiddle::foo3(romtiddle_2_ref_0, i64_1, i64_0);
    crate::hp::RomTiddle::avada_kedavra(romtiddle_1_ref_0, string_0);
    crate::hp::ParryHotter::foo2(parryhotter_0_ref_0, i32_1, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4999() {
    rusty_monitor::set_test_id(4999);
    let mut str_0: &str = "3IZFdEX06k";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_0: i64 = -4437i64;
    let mut u16_0: u16 = 2001u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    let mut u64_0: u64 = 6877u64;
    let mut u64_1: u64 = 8960u64;
    let mut usize_0: usize = 7123usize;
    let mut u16_1: u16 = 4965u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut usize_1: usize = 5630usize;
    let mut i32_0: i32 = -4205i32;
    let mut i32_1: i32 = 6740i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_2: i32 = 5074i32;
    let mut i32_3: i32 = -2047i32;
    let mut i32_4: i32 = -15839i32;
    let mut i32_5: i32 = 12100i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_5, b: i32_4};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_6: i32 = 974i32;
    let mut i32_7: i32 = 671i32;
    let mut i32_8: i32 = 4135i32;
    let mut i32_9: i32 = 11092i32;
    let mut u16_2: u16 = 8081u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut u16_3: u16 = 7402u16;
    let mut u64_2: u64 = 7219u64;
    let mut u64_3: u64 = 3312u64;
    let mut usize_2: usize = 1128usize;
    let mut i32_10: i32 = 14541i32;
    let mut i32_11: i32 = -10034i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_11, b: i32_10};
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i64_1: i64 = 19145i64;
    let mut i64_2: i64 = -12771i64;
    let mut u16_4: u16 = 1146u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_2};
    let mut wonreasley_1_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_1;
    let mut i32_12: i32 = -4114i32;
    let mut i32_13: i32 = 3037i32;
    let mut u16_5: u16 = 4029u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    let mut u16_6: u16 = 7344u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    let mut romtiddle_5_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_5;
    let mut i32_14: i32 = 1128i32;
    let mut i32_15: i32 = 3924i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_15, i32_14);
    crate::hp::RomTiddle::avada_kedavra(romtiddle_5_ref_0, string_3);
    let mut parryhotter_4: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_13, b: i32_12};
    crate::hp::WonReasley::ascendio(wonreasley_1_ref_0, i64_1);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_2_ref_0, usize_2, u64_3, u64_2);
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut string_4: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    crate::hp::ParryHotter::alohomora(i32_9, i32_8, i32_7, i32_6);
    let mut i32_16: i32 = crate::hp::ParryHotter::accio(parryhotter_1_ref_0, i32_3, i32_2);
    let mut i32_17: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_0_ref_0, usize_1, string_1);
    let mut parryhotter_4_ref_0: &crate::hp::ParryHotter = &mut parryhotter_4;
    let mut bool_1: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_4_ref_0, usize_0, u64_1, u64_0);
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
    panic!("From RustyUnit with love");
}
}