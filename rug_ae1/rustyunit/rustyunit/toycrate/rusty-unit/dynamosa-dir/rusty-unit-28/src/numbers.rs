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
fn rusty_test_1176() {
    rusty_monitor::set_test_id(1176);
    let mut u16_0: u16 = 2568u16;
    let mut i64_0: i64 = -9467i64;
    let mut i64_1: i64 = -8728i64;
    let mut u16_1: u16 = 9056u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_1};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut u16_2: u16 = 8278u16;
    let mut i64_2: i64 = -2312i64;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut i64_3: i64 = 6278i64;
    let mut u16_3: u16 = 5860u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut i64_4: i64 = -183i64;
    let mut i64_5: i64 = -6685i64;
    let mut u16_4: u16 = 5706u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut u64_0: u64 = 1733u64;
    let mut u64_1: u64 = 9240u64;
    let mut i32_0: i32 = -380i32;
    let mut i32_1: i32 = -12932i32;
    let mut i32_2: i32 = 3201i32;
    let mut i32_3: i32 = 6101i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u16_5: u16 = 3824u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    crate::hp::ParryHotter::foo2(parryhotter_0_ref_0, i32_1, i32_0);
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    crate::hp::RomTiddle::foo3(romtiddle_3_ref_0, i64_5, i64_4);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_3};
    let mut wonreasley_2: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_2};
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_0);
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1177() {
    rusty_monitor::set_test_id(1177);
    let mut i64_0: i64 = 3633i64;
    let mut i64_1: i64 = -6079i64;
    let mut i32_0: i32 = 738i32;
    let mut i32_1: i32 = 3315i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut u16_0: u16 = 7318u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut u16_1: u16 = 7436u16;
    let mut u16_2: u16 = 1591u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut u16_3: u16 = 5451u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_2_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_2;
    let mut i32_2: i32 = 5733i32;
    let mut i64_2: i64 = -183i64;
    let mut i64_3: i64 = -6685i64;
    let mut u16_4: u16 = 5706u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut i32_3: i32 = -380i32;
    let mut i32_4: i32 = -12932i32;
    let mut i32_5: i32 = 6101i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_2, i32_4);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    crate::hp::ParryHotter::foo2(parryhotter_0_ref_0, i32_5, i32_3);
    crate::hp::RomTiddle::foo3(romtiddle_3_ref_0, i64_3, i64_2);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_1};
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1181() {
    rusty_monitor::set_test_id(1181);
    let mut u64_0: u64 = 2623u64;
    let mut u64_1: u64 = 7693u64;
    let mut i32_0: i32 = 6530i32;
    let mut i32_1: i32 = -14599i32;
    let mut i32_2: i32 = 6533i32;
    let mut i32_3: i32 = -1363i32;
    let mut u16_0: u16 = 9880u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut i64_0: i64 = 4383i64;
    let mut i64_1: i64 = 1820i64;
    let mut u16_1: u16 = 8924u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_1};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut i64_2: i64 = -2543i64;
    let mut u16_2: u16 = 2794u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_2};
    let mut u16_3: u16 = 2039u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_3_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_3;
    let mut u16_4: u16 = 3269u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut u16_5: u16 = 7634u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_5_ref_0: &crate::hp::RomTiddle = &mut romtiddle_5;
    let mut i32_4: i32 = -11180i32;
    let mut i32_5: i32 = -11333i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_5, b: i32_4};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i64_3: i64 = 13536i64;
    let mut u16_6: u16 = 7164u16;
    let mut i64_4: i64 = -8202i64;
    let mut u16_7: u16 = 7978u16;
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_5_ref_0);
    let mut u16_8: u16 = 3232u16;
    let mut romtiddle_7: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_7};
    let mut romtiddle_6_ref_0: &crate::hp::RomTiddle = &mut romtiddle_6;
    let mut string_4: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    let mut romtiddle_8: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_8};
    let mut romtiddle_7_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_7;
    crate::hp::RomTiddle::avada_kedavra(romtiddle_3_ref_0, string_3);
    let mut wonreasley_2: crate::hp::WonReasley = crate::hp::WonReasley {x: string_4, y: i64_4};
    let mut wonreasley_1_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_1;
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_3);
    let mut wonreasley_3: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    crate::hp::ParryHotter::alohomora(i32_3, i32_2, i32_1, i32_0);
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1190() {
    rusty_monitor::set_test_id(1190);
    let mut i32_0: i32 = 4576i32;
    let mut i32_1: i32 = 6297i32;
    let mut i32_2: i32 = -18476i32;
    let mut i32_3: i32 = -453i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u16_0: u16 = 7408u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut i32_4: i32 = 7209i32;
    let mut i32_5: i32 = 2201i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_5, i32_4);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_6: i32 = -122i32;
    let mut i32_7: i32 = -10585i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_7, i32_6);
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i32_8: i32 = 8757i32;
    let mut i32_9: i32 = 12890i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_9, i32_8);
    let mut i32_10: i32 = 9376i32;
    let mut i64_0: i64 = -3628i64;
    let mut u16_1: u16 = 2250u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut u16_2: u16 = 7777u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut i64_1: i64 = 5403i64;
    let mut u16_3: u16 = 8336u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_1};
    let mut i64_2: i64 = 5212i64;
    let mut u16_4: u16 = 78u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut u16_5: u16 = 7533u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_5_ref_0: &crate::hp::RomTiddle = &mut romtiddle_5;
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_5_ref_0);
    let mut u16_6: u16 = 5451u16;
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    let mut romtiddle_6_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_6;
    let mut i32_11: i32 = -8628i32;
    let mut i32_12: i32 = -3530i32;
    let mut u16_7: u16 = 2803u16;
    let mut i32_13: i32 = -1857i32;
    let mut i32_14: i32 = 5357i32;
    let mut parryhotter_4: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_13, i32_14);
    let mut parryhotter_3_ref_0: &crate::hp::ParryHotter = &mut parryhotter_3;
    let mut i32_15: i32 = crate::hp::ParryHotter::accio(parryhotter_2_ref_0, i32_11, i32_12);
    let mut romtiddle_7: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_7};
    let mut i32_16: i32 = crate::hp::ParryHotter::accio(parryhotter_3_ref_0, i32_15, i32_10);
    let mut string_4: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_2);
    let mut romtiddle_7_ref_0: &crate::hp::RomTiddle = &mut romtiddle_7;
    let mut i32_17: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_1, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1191() {
    rusty_monitor::set_test_id(1191);
    let mut u16_0: u16 = 1152u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_0;
    let mut i64_0: i64 = -11842i64;
    let mut u16_1: u16 = 3109u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut u16_2: u16 = 4742u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut u16_3: u16 = 5813u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut u16_4: u16 = 527u16;
    let mut str_0: &str = "L13qdV9NFipP0yClU";
    let mut i32_0: i32 = 1853i32;
    let mut i32_1: i32 = 80i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_1, b: i32_0};
    let mut i64_1: i64 = -765i64;
    let mut i64_2: i64 = 19975i64;
    let mut u16_5: u16 = 9668u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut i64_3: i64 = 4948i64;
    let mut u16_6: u16 = 5266u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    let mut romtiddle_5_ref_0: &crate::hp::RomTiddle = &mut romtiddle_5;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_5_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_3};
    let mut wonreasley_1_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_1;
    let mut i32_2: i32 = -4956i32;
    let mut u16_7: u16 = 6476u16;
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_7};
    let mut romtiddle_6_ref_0: &crate::hp::RomTiddle = &mut romtiddle_6;
    let mut u64_0: u64 = 3028u64;
    let mut usize_0: usize = 3136usize;
    let mut i32_3: i32 = -15686i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_2, i32_3);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_4: i32 = 4033i32;
    let mut i32_5: i32 = 8296i32;
    let mut i32_6: i32 = -10536i32;
    let mut i32_7: i32 = 5963i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_7, i32_6);
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i32_8: i32 = -3676i32;
    let mut i32_9: i32 = -5256i32;
    let mut i32_10: i32 = 5733i32;
    let mut i32_11: i32 = -8913i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_4, i32_10);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_4: i64 = 10716i64;
    let mut romtiddle_7: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    let mut wonreasley_2: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_2};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    let mut parryhotter_4: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_8, b: i32_5};
    let mut parryhotter_3_ref_0: &crate::hp::ParryHotter = &mut parryhotter_3;
    let mut u64_1: u64 = 1407u64;
    let mut string_4: std::string::String = crate::hp::RomTiddle::name(romtiddle_6_ref_0);
    let mut wonreasley_3: crate::hp::WonReasley = crate::hp::WonReasley {x: string_3, y: i64_1};
    let mut wonreasley_2_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_2;
    crate::hp::WonReasley::ascendio(wonreasley_1_ref_0, i64_4);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_3_ref_0, usize_0, u64_0, u64_1);
    crate::hp::RomTiddle::avada_kedavra(romtiddle_0_ref_0, string_4);
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
    let mut parryhotter_5: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_9, i32_11);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1199() {
    rusty_monitor::set_test_id(1199);
    let mut i32_0: i32 = 27127i32;
    let mut i32_1: i32 = 22700i32;
    let mut i32_2: i32 = 9682i32;
    let mut i32_3: i32 = 3379i32;
    let mut i64_0: i64 = 245i64;
    let mut i64_1: i64 = -16405i64;
    let mut u16_0: u16 = 2827u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_1};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut i64_2: i64 = 12303i64;
    let mut u16_1: u16 = 6839u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_2};
    let mut u16_2: u16 = 3578u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut u16_3: u16 = 7366u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut i32_4: i32 = 6670i32;
    let mut i32_5: i32 = 2504i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_5, b: i32_4};
    let mut i32_6: i32 = -6210i32;
    let mut i32_7: i32 = -10675i32;
    let mut i32_8: i32 = -7483i32;
    let mut str_0: &str = "C";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_3: i64 = 660i64;
    let mut u16_4: u16 = 6037u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    let mut wonreasley_2: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_3};
    let mut wonreasley_2_ref_0: &crate::hp::WonReasley = &mut wonreasley_2;
    let mut i32_9: i32 = -9328i32;
    let mut i32_10: i32 = 7537i32;
    let mut u16_5: u16 = 1872u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_5_ref_0: &crate::hp::RomTiddle = &mut romtiddle_5;
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_5_ref_0);
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_6, b: i32_9};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_11: i32 = 2779i32;
    let mut u64_0: u64 = 6858u64;
    let mut u64_1: u64 = 1285u64;
    let mut usize_0: usize = 6891usize;
    let mut i32_12: i32 = 13690i32;
    let mut i32_13: i32 = 7119i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_11, b: i32_8};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_14: i32 = -9878i32;
    let mut i32_15: i32 = 7120i32;
    let mut u16_6: u16 = 2309u16;
    let mut i64_4: i64 = 21266i64;
    let mut u16_7: u16 = 4135u16;
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_4: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut wonreasley_3: crate::hp::WonReasley = crate::hp::WonReasley {x: string_3, y: i64_4};
    let mut wonreasley_1_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_1;
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_0);
    let mut romtiddle_7: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_7};
    let mut romtiddle_6_ref_0: &crate::hp::RomTiddle = &mut romtiddle_6;
    let mut string_5: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_10, i32_12);
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i32_16: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_15, i32_13);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_1_ref_0, usize_0, u64_1, u64_0);
    let mut i32_17: i32 = crate::hp::ParryHotter::accio(parryhotter_2_ref_0, i32_7, i32_14);
    crate::hp::WonReasley::arania_exumai(wonreasley_2_ref_0, str_0_ref_0);
    crate::hp::ParryHotter::alohomora(i32_3, i32_2, i32_1, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1203() {
    rusty_monitor::set_test_id(1203);
    let mut i64_0: i64 = -3284i64;
    let mut i64_1: i64 = 879i64;
    let mut i32_0: i32 = -1107i32;
    let mut i32_1: i32 = -8876i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_1, b: i32_0};
    let mut i32_2: i32 = -3068i32;
    let mut i32_3: i32 = 1175i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut u16_0: u16 = 7021u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut i32_4: i32 = -6047i32;
    let mut i32_5: i32 = -1516i32;
    let mut i32_6: i32 = 24422i32;
    let mut i32_7: i32 = -4213i32;
    let mut u16_1: u16 = 9298u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut i32_8: i32 = -10536i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_6, i32_7);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_9: i32 = -3676i32;
    let mut i32_10: i32 = -5256i32;
    let mut i32_11: i32 = -8628i32;
    let mut i32_12: i32 = -3530i32;
    let mut u16_2: u16 = 2803u16;
    let mut i32_13: i32 = 9310i32;
    let mut i32_14: i32 = -1857i32;
    let mut i32_15: i32 = -13191i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_8, i32_12);
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i32_16: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_11, i32_14);
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    crate::hp::ParryHotter::alohomora(i32_4, i32_13, i32_10, i32_9);
    let mut i32_17: i32 = crate::hp::ParryHotter::accio(parryhotter_1_ref_0, i32_5, i32_15);
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    crate::hp::RomTiddle::foo3(romtiddle_1_ref_0, i64_1, i64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1206() {
    rusty_monitor::set_test_id(1206);
    let mut i64_0: i64 = -1783i64;
    let mut i64_1: i64 = 2524i64;
    let mut u16_0: u16 = 946u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut str_0: &str = "hzMMe5rT";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_2: i64 = 11304i64;
    let mut u16_1: u16 = 1694u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_2};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    let mut u16_2: u16 = 8999u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut i32_0: i32 = 22356i32;
    let mut i32_1: i32 = -16925i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_1, b: i32_0};
    let mut u16_3: u16 = 4036u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut u64_0: u64 = 6107u64;
    let mut u64_1: u64 = 2454u64;
    let mut i32_2: i32 = -741i32;
    let mut i32_3: i32 = 6784i32;
    let mut i32_4: i32 = 5001i32;
    let mut i64_3: i64 = 9536i64;
    let mut u16_4: u16 = 803u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut u16_5: u16 = 1200u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_5_ref_0: &crate::hp::RomTiddle = &mut romtiddle_5;
    let mut i32_5: i32 = -15686i32;
    let mut i32_6: i32 = 4826i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_5, i32_4);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_7: i32 = 8296i32;
    let mut i32_8: i32 = -10536i32;
    let mut i32_9: i32 = 5963i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_8, i32_7);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_10: i32 = -5256i32;
    let mut i32_11: i32 = -8628i32;
    let mut u16_6: u16 = 2803u16;
    let mut i32_12: i32 = -13191i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_10, i32_2);
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i32_13: i32 = crate::hp::ParryHotter::accio(parryhotter_1_ref_0, i32_12, i32_3);
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    let mut i32_14: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_9, i32_13);
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_3};
    let mut i32_15: i32 = crate::hp::ParryHotter::accio(parryhotter_2_ref_0, i32_6, i32_11);
    crate::hp::ParryHotter::another_number_fn(u64_0, u64_1);
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
    crate::hp::RomTiddle::foo3(romtiddle_0_ref_0, i64_1, i64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1213() {
    rusty_monitor::set_test_id(1213);
    let mut u16_0: u16 = 6312u16;
    let mut str_0: &str = "0cQ3";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_0: i64 = -15695i64;
    let mut i64_1: i64 = -10285i64;
    let mut u16_1: u16 = 8128u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_1};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut u16_2: u16 = 3039u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut u16_3: u16 = 1814u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut i32_0: i32 = -4962i32;
    let mut i32_1: i32 = -11621i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut i64_2: i64 = -9183i64;
    let mut u16_4: u16 = 5929u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_2};
    let mut i64_3: i64 = 10426i64;
    let mut u16_5: u16 = 4530u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    let mut u16_6: u16 = 1417u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    let mut romtiddle_5_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_5;
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut u16_7: u16 = 9119u16;
    let mut wonreasley_2: crate::hp::WonReasley = crate::hp::WonReasley {x: string_3, y: i64_3};
    let mut wonreasley_1_ref_0: &crate::hp::WonReasley = &mut wonreasley_1;
    let mut i32_2: i32 = -14232i32;
    let mut i32_3: i32 = -7245i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i64_4: i64 = 12925i64;
    let mut romtiddle_7: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_7};
    let mut romtiddle_6_ref_0: &crate::hp::RomTiddle = &mut romtiddle_6;
    let mut string_4: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut wonreasley_3: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_4};
    let mut wonreasley_2_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_2;
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_0);
    crate::hp::WonReasley::arania_exumai(wonreasley_1_ref_0, str_0_ref_0);
    let mut string_5: std::string::String = crate::hp::RomTiddle::name(romtiddle_6_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1216() {
    rusty_monitor::set_test_id(1216);
    let mut i32_0: i32 = 16081i32;
    let mut i32_1: i32 = 9622i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut str_0: &str = "zx1uXgfAI64jWLPL";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_0: i64 = 4020i64;
    let mut u16_0: u16 = 5067u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut u16_1: u16 = 9039u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_1;
    let mut i64_1: i64 = -2017i64;
    let mut u16_2: u16 = 828u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_1};
    let mut u16_3: u16 = 2466u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut i64_2: i64 = -3429i64;
    let mut u16_4: u16 = 5679u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut wonreasley_2: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_2};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1217() {
    rusty_monitor::set_test_id(1217);
    let mut str_0: &str = "OJteGfYzV6aZK";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_0: i64 = 1919i64;
    let mut u16_0: u16 = 3600u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut u16_1: u16 = 7101u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_1;
    let mut u16_2: u16 = 4719u16;
    let mut u16_3: u16 = 9645u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut i64_1: i64 = -2312i64;
    let mut u16_4: u16 = 5069u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut i64_2: i64 = 6278i64;
    let mut u16_5: u16 = 5860u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    let mut i64_3: i64 = -183i64;
    let mut i64_4: i64 = -6685i64;
    let mut u16_6: u16 = 5706u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    let mut romtiddle_5_ref_0: &crate::hp::RomTiddle = &mut romtiddle_5;
    let mut u64_0: u64 = 1733u64;
    let mut u64_1: u64 = 9240u64;
    let mut i32_0: i32 = -380i32;
    let mut i32_1: i32 = -12932i32;
    let mut i32_2: i32 = 3201i32;
    let mut i32_3: i32 = 6101i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u16_7: u16 = 3824u16;
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_7};
    let mut romtiddle_6_ref_0: &crate::hp::RomTiddle = &mut romtiddle_6;
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_6_ref_0);
    crate::hp::ParryHotter::foo2(parryhotter_0_ref_0, i32_1, i32_0);
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    crate::hp::RomTiddle::foo3(romtiddle_5_ref_0, i64_4, i64_3);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut u16_8: u16 = 5820u16;
    let mut romtiddle_7: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut string_4: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut wonreasley_2: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_1};
    let mut wonreasley_1_ref_0: &crate::hp::WonReasley = &mut wonreasley_1;
    let mut u64_2: u64 = 2335u64;
    let mut u16_9: u16 = 8631u16;
    let mut romtiddle_8: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_8};
    let mut romtiddle_7_ref_0: &crate::hp::RomTiddle = &mut romtiddle_7;
    let mut i64_5: i64 = 694i64;
    let mut u16_10: u16 = 9746u16;
    let mut romtiddle_9: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_9};
    let mut romtiddle_8_ref_0: &crate::hp::RomTiddle = &mut romtiddle_8;
    let mut string_5: std::string::String = crate::hp::RomTiddle::name(romtiddle_7_ref_0);
    let mut u64_3: u64 = 3462u64;
    let mut i64_6: i64 = -3282i64;
    let mut u16_11: u16 = 2044u16;
    let mut romtiddle_10: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_10};
    let mut romtiddle_9_ref_0: &crate::hp::RomTiddle = &mut romtiddle_9;
    let mut i64_7: i64 = 12601i64;
    let mut u16_12: u16 = 7978u16;
    let mut romtiddle_11: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_11};
    let mut romtiddle_10_ref_0: &crate::hp::RomTiddle = &mut romtiddle_10;
    let mut string_6: std::string::String = crate::hp::RomTiddle::name(romtiddle_9_ref_0);
    let mut u16_13: u16 = 3232u16;
    let mut romtiddle_12: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_12};
    let mut romtiddle_11_ref_0: &crate::hp::RomTiddle = &mut romtiddle_11;
    let mut string_7: std::string::String = crate::hp::RomTiddle::name(romtiddle_8_ref_0);
    let mut romtiddle_13: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_13};
    let mut romtiddle_12_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_12;
    crate::hp::RomTiddle::avada_kedavra(romtiddle_1_ref_0, string_4);
    let mut wonreasley_3: crate::hp::WonReasley = crate::hp::WonReasley {x: string_6, y: i64_5};
    let mut wonreasley_2_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_2;
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_7);
    crate::hp::ParryHotter::another_number_fn(u64_3, u64_2);
    let mut wonreasley_4: crate::hp::WonReasley = crate::hp::WonReasley {x: string_7, y: i64_6};
    let mut string_8: std::string::String = crate::hp::RomTiddle::name(romtiddle_10_ref_0);
    crate::hp::WonReasley::arania_exumai(wonreasley_1_ref_0, str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1220() {
    rusty_monitor::set_test_id(1220);
    let mut i32_0: i32 = 19833i32;
    let mut i32_1: i32 = -16446i32;
    let mut i32_2: i32 = -4565i32;
    let mut i32_3: i32 = 3319i32;
    let mut u16_0: u16 = 7634u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut u64_0: u64 = 8123u64;
    let mut u64_1: u64 = 1186u64;
    let mut i32_4: i32 = 12106i32;
    let mut i32_5: i32 = -11788i32;
    let mut i32_6: i32 = -11180i32;
    let mut i32_7: i32 = -11333i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_7, b: i32_6};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i64_0: i64 = 13536i64;
    let mut u16_1: u16 = 7164u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut u16_2: u16 = 6476u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut u64_2: u64 = 3028u64;
    let mut u64_3: u64 = 7445u64;
    let mut usize_0: usize = 3136usize;
    let mut i32_8: i32 = -15686i32;
    let mut i32_9: i32 = 4826i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_9, i32_8);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_10: i32 = 4033i32;
    let mut i32_11: i32 = 8296i32;
    let mut i32_12: i32 = -10536i32;
    let mut i32_13: i32 = 5963i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_13, i32_12);
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i32_14: i32 = -3676i32;
    let mut i32_15: i32 = -5256i32;
    let mut i32_16: i32 = -8628i32;
    let mut i32_17: i32 = -3530i32;
    let mut u16_3: u16 = 2803u16;
    let mut i32_18: i32 = 9310i32;
    let mut i32_19: i32 = -1857i32;
    let mut i32_20: i32 = 5357i32;
    let mut i32_21: i32 = -13191i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_21, i32_20);
    let mut parryhotter_3_ref_0: &crate::hp::ParryHotter = &mut parryhotter_3;
    let mut i32_22: i32 = crate::hp::ParryHotter::accio(parryhotter_3_ref_0, i32_19, i32_18);
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    crate::hp::ParryHotter::alohomora(i32_17, i32_16, i32_15, i32_14);
    let mut i32_23: i32 = crate::hp::ParryHotter::accio(parryhotter_2_ref_0, i32_11, i32_10);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_1_ref_0, usize_0, u64_3, u64_2);
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut i32_24: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_5, i32_4);
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    crate::hp::ParryHotter::alohomora(i32_3, i32_2, i32_1, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1228() {
    rusty_monitor::set_test_id(1228);
    let mut str_0: &str = "UIltLMOyTZ";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_0: i64 = -10853i64;
    let mut u16_0: u16 = 6847u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    let mut i32_0: i32 = -7147i32;
    let mut i32_1: i32 = 13019i32;
    let mut i32_2: i32 = 13617i32;
    let mut i32_3: i32 = -3536i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u16_1: u16 = 4530u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut u16_2: u16 = 1417u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_2;
    let mut u16_3: u16 = 7071u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut u16_4: u16 = 9119u16;
    let mut i64_1: i64 = -1037i64;
    let mut u16_5: u16 = 233u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_1};
    let mut wonreasley_1_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_1;
    let mut i32_4: i32 = -783i32;
    let mut i32_5: i32 = 14922i32;
    let mut u16_6: u16 = 4267u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    let mut romtiddle_5_ref_0: &crate::hp::RomTiddle = &mut romtiddle_5;
    let mut i32_6: i32 = 12106i32;
    let mut i32_7: i32 = -11788i32;
    let mut i32_8: i32 = -11333i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_6, b: i32_7};
    let mut i64_2: i64 = 13536i64;
    let mut u16_7: u16 = 7164u16;
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_5_ref_0);
    let mut u16_8: u16 = 764u16;
    let mut romtiddle_7: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_7};
    let mut romtiddle_6_ref_0: &crate::hp::RomTiddle = &mut romtiddle_6;
    let mut i32_9: i32 = -15686i32;
    let mut i32_10: i32 = 4826i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_4, i32_9);
    let mut i32_11: i32 = 5963i32;
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i32_12: i32 = -8628i32;
    let mut i32_13: i32 = -3530i32;
    let mut i32_14: i32 = 9310i32;
    let mut i32_15: i32 = -1857i32;
    let mut i32_16: i32 = 5357i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_11, i32_14);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_17: i32 = crate::hp::ParryHotter::accio(parryhotter_2_ref_0, i32_5, i32_13);
    let mut romtiddle_8: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_8};
    crate::hp::ParryHotter::alohomora(i32_16, i32_17, i32_10, i32_12);
    let mut string_4: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut wonreasley_2: crate::hp::WonReasley = crate::hp::WonReasley {x: string_3, y: i64_2};
    let mut i32_18: i32 = crate::hp::ParryHotter::accio(parryhotter_1_ref_0, i32_15, i32_8);
    crate::hp::RomTiddle::avada_kedavra(romtiddle_2_ref_0, string_1);
    let mut string_5: std::string::String = crate::hp::RomTiddle::name(romtiddle_6_ref_0);
    let mut i32_19: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_1, i32_0);
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1233() {
    rusty_monitor::set_test_id(1233);
    let mut i32_0: i32 = 3925i32;
    let mut i32_1: i32 = -9129i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_1, b: i32_0};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_2: i32 = 1685i32;
    let mut i32_3: i32 = -1903i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    let mut i32_4: i32 = 1436i32;
    let mut i32_5: i32 = -16320i32;
    let mut i32_6: i32 = -7118i32;
    let mut str_0: &str = "mi1V0U7nWJi1UcSlxUn";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_0: i64 = -5733i64;
    let mut u16_0: u16 = 8966u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_6, i32_5);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_7: i32 = -3676i32;
    let mut i32_8: i32 = -3530i32;
    let mut u16_1: u16 = 2803u16;
    let mut i32_9: i32 = -13191i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_4, i32_8);
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut i32_10: i32 = crate::hp::ParryHotter::accio(parryhotter_2_ref_0, i32_9, i32_7);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1240() {
    rusty_monitor::set_test_id(1240);
    let mut i32_0: i32 = 14378i32;
    let mut i32_1: i32 = -2479i32;
    let mut str_0: &str = "HIRxtULk";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_0: i64 = 1836i64;
    let mut u16_0: u16 = 7939u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    let mut i32_2: i32 = 803i32;
    let mut i32_3: i32 = -5745i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u16_1: u16 = 5250u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut u16_2: u16 = 7671u16;
    let mut u16_3: u16 = 5820u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut str_1: &str = "HBNUJxnd";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut i64_1: i64 = -3467i64;
    let mut u16_4: u16 = 1649u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_1};
    let mut wonreasley_1_ref_0: &crate::hp::WonReasley = &mut wonreasley_1;
    let mut i32_4: i32 = -14232i32;
    let mut i32_5: i32 = 814i32;
    let mut i32_6: i32 = -7245i32;
    let mut i32_7: i32 = -9927i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_7, b: i32_6};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i64_2: i64 = 12925i64;
    let mut i64_3: i64 = 1686i64;
    let mut u16_5: u16 = 1563u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    let mut wonreasley_2: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_3};
    let mut wonreasley_2_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_2;
    crate::hp::WonReasley::ascendio(wonreasley_2_ref_0, i64_2);
    let mut i32_8: i32 = crate::hp::ParryHotter::accio(parryhotter_1_ref_0, i32_5, i32_4);
    crate::hp::WonReasley::arania_exumai(wonreasley_1_ref_0, str_1_ref_0);
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_1, b: i32_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1244() {
    rusty_monitor::set_test_id(1244);
    let mut i64_0: i64 = 19998i64;
    let mut i64_1: i64 = -14311i64;
    let mut u16_0: u16 = 3345u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_1};
    let mut u16_1: u16 = 5768u16;
    let mut i32_0: i32 = -10675i32;
    let mut str_0: &str = "c";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_2: i64 = 660i64;
    let mut u16_2: u16 = 6037u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_2};
    let mut wonreasley_1_ref_0: &crate::hp::WonReasley = &mut wonreasley_1;
    let mut u16_3: u16 = 1872u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut u16_4: u16 = 8053u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_3_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_3;
    let mut u16_5: u16 = 8178u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut i32_1: i32 = -9938i32;
    let mut i32_2: i32 = 13345i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_2, b: i32_1};
    let mut i32_3: i32 = -5421i32;
    let mut i32_4: i32 = -15934i32;
    let mut i32_5: i32 = -10763i32;
    let mut str_1: &str = "";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut i64_3: i64 = 5459i64;
    let mut u16_6: u16 = 7373u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    let mut romtiddle_5_ref_0: &crate::hp::RomTiddle = &mut romtiddle_5;
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_5_ref_0);
    let mut wonreasley_2: crate::hp::WonReasley = crate::hp::WonReasley {x: string_3, y: i64_3};
    let mut wonreasley_2_ref_0: &crate::hp::WonReasley = &mut wonreasley_2;
    let mut u16_7: u16 = 7360u16;
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut str_2: &str = "i7oWW";
    let mut str_3: &str = "tFHWYKWui77OJ";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut i64_4: i64 = -13016i64;
    let mut u16_8: u16 = 2081u16;
    let mut romtiddle_7: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_7};
    let mut romtiddle_6_ref_0: &crate::hp::RomTiddle = &mut romtiddle_6;
    let mut string_4: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    let mut wonreasley_3: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_4};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    let mut u64_0: u64 = 6918u64;
    let mut u64_1: u64 = 6015u64;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_5, i32_3);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut str_3_ref_0: &str = &mut str_3;
    let mut i64_5: i64 = -3090i64;
    let mut u16_9: u16 = 1649u16;
    let mut romtiddle_8: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_8};
    let mut romtiddle_7_ref_0: &crate::hp::RomTiddle = &mut romtiddle_7;
    let mut string_5: std::string::String = crate::hp::RomTiddle::name(romtiddle_6_ref_0);
    let mut wonreasley_4: crate::hp::WonReasley = crate::hp::WonReasley {x: string_4, y: i64_5};
    let mut wonreasley_3_ref_0: &crate::hp::WonReasley = &mut wonreasley_3;
    let mut romtiddle_9: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_9};
    let mut romtiddle_8_ref_0: &crate::hp::RomTiddle = &mut romtiddle_8;
    let mut string_6: std::string::String = crate::hp::RomTiddle::name(romtiddle_7_ref_0);
    let mut wonreasley_5: crate::hp::WonReasley = crate::hp::WonReasley {x: string_5, y: i64_0};
    let mut wonreasley_4_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_4;
    crate::hp::WonReasley::arania_exumai(wonreasley_2_ref_0, str_0_ref_0);
    let mut string_7: std::string::String = crate::hp::RomTiddle::name(romtiddle_8_ref_0);
    crate::hp::RomTiddle::avada_kedavra(romtiddle_3_ref_0, string_6);
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    crate::hp::ParryHotter::foo2(parryhotter_0_ref_0, i32_0, i32_4);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1249() {
    rusty_monitor::set_test_id(1249);
    let mut u16_0: u16 = 2004u16;
    let mut i64_0: i64 = -12816i64;
    let mut i32_0: i32 = -6910i32;
    let mut i32_1: i32 = 15373i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut i64_1: i64 = 14859i64;
    let mut u16_1: u16 = 7495u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut u16_2: u16 = 1051u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut i32_2: i32 = 8005i32;
    let mut i32_3: i32 = -5421i32;
    let mut i32_4: i32 = -15934i32;
    let mut i32_5: i32 = -10763i32;
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_2: i64 = 5459i64;
    let mut u16_3: u16 = 7373u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_2};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    let mut i32_6: i32 = -4252i32;
    let mut i32_7: i32 = -3932i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_7, i32_6);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i64_3: i64 = -14657i64;
    let mut u16_4: u16 = 4871u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_3};
    let mut u16_5: u16 = 860u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut u16_6: u16 = 8102u16;
    let mut u16_7: u16 = 839u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut wonreasley_2: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_1};
    let mut wonreasley_1_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_1;
    let mut i64_4: i64 = -8289i64;
    let mut u16_8: u16 = 9229u16;
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_7};
    let mut romtiddle_5_ref_0: &crate::hp::RomTiddle = &mut romtiddle_5;
    let mut string_4: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    let mut i32_8: i32 = -6686i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_5, i32_8);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_9: i32 = -7606i32;
    let mut i32_10: i32 = -12627i32;
    let mut i32_11: i32 = -8015i32;
    let mut i64_5: i64 = 3820i64;
    let mut i64_6: i64 = 10663i64;
    let mut u16_9: u16 = 3114u16;
    let mut romtiddle_7: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_8};
    let mut romtiddle_6_ref_0: &crate::hp::RomTiddle = &mut romtiddle_6;
    let mut string_5: std::string::String = crate::hp::RomTiddle::name(romtiddle_5_ref_0);
    let mut wonreasley_3: crate::hp::WonReasley = crate::hp::WonReasley {x: string_4, y: i64_4};
    let mut wonreasley_2_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_2;
    let mut i64_7: i64 = -4390i64;
    let mut romtiddle_8: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_9};
    let mut romtiddle_7_ref_0: &crate::hp::RomTiddle = &mut romtiddle_7;
    let mut string_6: std::string::String = crate::hp::RomTiddle::name(romtiddle_6_ref_0);
    let mut i64_8: i64 = -17884i64;
    let mut i64_9: i64 = -13962i64;
    let mut romtiddle_9: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_8_ref_0: &crate::hp::RomTiddle = &mut romtiddle_8;
    crate::hp::RomTiddle::foo3(romtiddle_7_ref_0, i64_8, i64_9);
    let mut wonreasley_4: crate::hp::WonReasley = crate::hp::WonReasley {x: string_6, y: i64_7};
    crate::hp::WonReasley::ascendio(wonreasley_1_ref_0, i64_5);
    crate::hp::ParryHotter::alohomora(i32_4, i32_3, i32_11, i32_9);
    let mut wonreasley_5: crate::hp::WonReasley = crate::hp::WonReasley {x: string_5, y: i64_6};
    let mut wonreasley_4_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_4;
    crate::hp::WonReasley::ascendio(wonreasley_2_ref_0, i64_0);
    let mut i32_12: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_10, i32_2);
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
    let mut string_7: std::string::String = crate::hp::RomTiddle::name(romtiddle_8_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1251() {
    rusty_monitor::set_test_id(1251);
    let mut i32_0: i32 = -19438i32;
    let mut i32_1: i32 = 15005i32;
    let mut u16_0: u16 = 8751u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut i32_2: i32 = -5593i32;
    let mut i32_3: i32 = -3894i32;
    let mut i32_4: i32 = -2551i32;
    let mut i32_5: i32 = -9817i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_5, i32_4);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u16_1: u16 = 6476u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut u64_0: u64 = 3028u64;
    let mut u64_1: u64 = 7445u64;
    let mut usize_0: usize = 3136usize;
    let mut i32_6: i32 = -15686i32;
    let mut i32_7: i32 = 4826i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_7, i32_6);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_8: i32 = 4033i32;
    let mut i32_9: i32 = 8296i32;
    let mut i32_10: i32 = -10536i32;
    let mut i32_11: i32 = 5963i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_11, i32_10);
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i32_12: i32 = -3676i32;
    let mut i32_13: i32 = -5256i32;
    let mut i32_14: i32 = -8628i32;
    let mut i32_15: i32 = -3530i32;
    let mut u16_2: u16 = 2803u16;
    let mut i32_16: i32 = 5357i32;
    let mut i32_17: i32 = -13191i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_17, i32_16);
    let mut parryhotter_3_ref_0: &crate::hp::ParryHotter = &mut parryhotter_3;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    crate::hp::ParryHotter::alohomora(i32_15, i32_14, i32_13, i32_12);
    let mut i32_18: i32 = crate::hp::ParryHotter::accio(parryhotter_2_ref_0, i32_9, i32_8);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_1_ref_0, usize_0, u64_1, u64_0);
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    crate::hp::ParryHotter::foo2(parryhotter_0_ref_0, i32_3, i32_2);
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut parryhotter_4: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_1, b: i32_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1254() {
    rusty_monitor::set_test_id(1254);
    let mut i64_0: i64 = -12829i64;
    let mut u16_0: u16 = 2581u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut str_0: &str = "L4HNMUVwPFp";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_1: i64 = -257i64;
    let mut u16_1: u16 = 5091u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_1};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut i32_0: i32 = 392i32;
    let mut i32_1: i32 = -9523i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut i64_2: i64 = -14091i64;
    let mut u16_2: u16 = 1032u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut wonreasley_2: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_2};
    let mut i32_2: i32 = -6199i32;
    let mut u16_3: u16 = 3123u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut u16_4: u16 = 6394u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_4_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_4;
    let mut u16_5: u16 = 9014u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_5_ref_0: &crate::hp::RomTiddle = &mut romtiddle_5;
    let mut u16_6: u16 = 3529u16;
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    let mut romtiddle_6_ref_0: &crate::hp::RomTiddle = &mut romtiddle_6;
    let mut str_1: &str = "j3a7SLMALVOjXGIyP";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut i64_3: i64 = -15599i64;
    let mut u16_7: u16 = 9777u16;
    let mut romtiddle_7: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_7};
    let mut romtiddle_7_ref_0: &crate::hp::RomTiddle = &mut romtiddle_7;
    let mut string_4: std::string::String = crate::hp::RomTiddle::name(romtiddle_7_ref_0);
    let mut wonreasley_3: crate::hp::WonReasley = crate::hp::WonReasley {x: string_4, y: i64_3};
    let mut wonreasley_3_ref_0: &crate::hp::WonReasley = &mut wonreasley_3;
    let mut i64_4: i64 = 3625i64;
    let mut i64_5: i64 = -7749i64;
    let mut u16_8: u16 = 652u16;
    let mut romtiddle_8: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_8};
    let mut romtiddle_8_ref_0: &crate::hp::RomTiddle = &mut romtiddle_8;
    let mut wonreasley_4: crate::hp::WonReasley = crate::hp::WonReasley {x: string_3, y: i64_4};
    let mut wonreasley_1_ref_0: &crate::hp::WonReasley = &mut wonreasley_1;
    let mut i32_3: i32 = 12789i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u16_9: u16 = 5592u16;
    let mut u16_10: u16 = 4228u16;
    let mut romtiddle_9: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_9};
    let mut romtiddle_10: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_10};
    crate::hp::WonReasley::arania_exumai(wonreasley_3_ref_0, str_1_ref_0);
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_5);
    crate::hp::WonReasley::arania_exumai(wonreasley_1_ref_0, str_0_ref_0);
    panic!("From RustyUnit with love");
}
}