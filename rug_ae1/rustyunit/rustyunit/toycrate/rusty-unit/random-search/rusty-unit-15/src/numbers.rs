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
fn rusty_test_4951() {
    rusty_monitor::set_test_id(4951);
    let mut i32_0: i32 = 4073i32;
    let mut i32_1: i32 = 8578i32;
    let mut i64_0: i64 = 15425i64;
    let mut u16_0: u16 = 127u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut u16_1: u16 = 4802u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut u16_2: u16 = 9613u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_2;
    let mut u16_3: u16 = 1853u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut i64_1: i64 = -10008i64;
    let mut i64_2: i64 = -8359i64;
    let mut u16_4: u16 = 3190u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_2};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut i32_2: i32 = 3683i32;
    let mut i32_3: i32 = -10887i32;
    let mut i32_4: i32 = -10307i32;
    let mut i32_5: i32 = -7424i32;
    let mut i32_6: i32 = 8377i32;
    let mut i32_7: i32 = 13491i32;
    crate::hp::ParryHotter::alohomora(i32_7, i32_6, i32_5, i32_4);
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_1);
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    crate::hp::RomTiddle::avada_kedavra(romtiddle_2_ref_0, string_1);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4954() {
    rusty_monitor::set_test_id(4954);
    let mut u16_0: u16 = 3333u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut usize_0: usize = 8184usize;
    let mut i32_0: i32 = -637i32;
    let mut i32_1: i32 = 5917i32;
    let mut i32_2: i32 = -4290i32;
    let mut i32_3: i32 = 3249i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_4: i32 = 8267i32;
    let mut i32_5: i32 = 2671i32;
    let mut i32_6: i32 = -4368i32;
    let mut i32_7: i32 = -2728i32;
    let mut i32_8: i32 = -544i32;
    let mut i32_9: i32 = 16484i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_9, b: i32_8};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_10: i32 = 6523i32;
    let mut i32_11: i32 = -15051i32;
    let mut i64_0: i64 = -11977i64;
    let mut u16_1: u16 = 3036u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut u64_0: u64 = 1715u64;
    let mut u64_1: u64 = 6171u64;
    let mut usize_1: usize = 4298usize;
    let mut i32_12: i32 = -2034i32;
    let mut i32_13: i32 = -7877i32;
    let mut i32_14: i32 = 1767i32;
    let mut i32_15: i32 = 15599i32;
    let mut i32_16: i32 = 9081i32;
    let mut i32_17: i32 = -2355i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_17, b: i32_16};
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut u64_2: u64 = 8289u64;
    let mut u64_3: u64 = 2484u64;
    let mut i32_18: i32 = -10110i32;
    let mut i32_19: i32 = 7379i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_19, i32_18);
    crate::hp::ParryHotter::another_number_fn(u64_3, u64_2);
    let mut i32_20: i32 = crate::hp::ParryHotter::accio(parryhotter_2_ref_0, i32_15, i32_14);
    let mut parryhotter_4: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_13, b: i32_12};
    let mut parryhotter_3_ref_0: &crate::hp::ParryHotter = &mut parryhotter_3;
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_3_ref_0, usize_1, u64_1, u64_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_0};
    let mut parryhotter_5: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_11, b: i32_10};
    crate::hp::ParryHotter::foo2(parryhotter_1_ref_0, i32_7, i32_6);
    let mut parryhotter_5_ref_0: &crate::hp::ParryHotter = &mut parryhotter_5;
    let mut i32_21: i32 = crate::hp::ParryHotter::accio(parryhotter_5_ref_0, i32_5, i32_4);
    let mut i32_22: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_1, i32_0);
    let mut parryhotter_4_ref_0: &crate::hp::ParryHotter = &mut parryhotter_4;
    let mut i32_23: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_4_ref_0, usize_0, string_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4955() {
    rusty_monitor::set_test_id(4955);
    let mut i64_0: i64 = 4944i64;
    let mut u16_0: u16 = 6498u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut i64_1: i64 = -12360i64;
    let mut i64_2: i64 = 20669i64;
    let mut u16_1: u16 = 5274u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_2};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut u64_0: u64 = 4681u64;
    let mut u64_1: u64 = 6246u64;
    let mut usize_0: usize = 1606usize;
    let mut i32_0: i32 = -1424i32;
    let mut i32_1: i32 = 14433i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u16_2: u16 = 4139u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut u16_3: u16 = 6590u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_3_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_3;
    let mut i32_2: i32 = 1626i32;
    let mut i32_3: i32 = -16890i32;
    let mut i32_4: i32 = -13832i32;
    let mut i32_5: i32 = 8547i32;
    crate::hp::ParryHotter::alohomora(i32_5, i32_4, i32_3, i32_2);
    crate::hp::RomTiddle::avada_kedavra(romtiddle_3_ref_0, string_2);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_0_ref_0, usize_0, u64_1, u64_0);
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_1);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4958() {
    rusty_monitor::set_test_id(4958);
    let mut str_0: &str = "6mG2McJt8hove6";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_0: i64 = 12253i64;
    let mut u16_0: u16 = 6541u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    let mut u16_1: u16 = 8812u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut u16_2: u16 = 7833u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_2;
    let mut i64_1: i64 = -10458i64;
    let mut i64_2: i64 = -833i64;
    let mut u16_3: u16 = 5397u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_2};
    let mut wonreasley_1_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_1;
    let mut i32_0: i32 = 10935i32;
    let mut i32_1: i32 = 4560i32;
    let mut i32_2: i32 = 6992i32;
    let mut i32_3: i32 = -10229i32;
    let mut u16_4: u16 = 4040u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    let mut usize_0: usize = 5073usize;
    let mut i32_4: i32 = -4130i32;
    let mut i32_5: i32 = -20216i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_5, b: i32_4};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i64_3: i64 = 8228i64;
    let mut i64_4: i64 = -2369i64;
    let mut u16_5: u16 = 8023u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_5_ref_0: &crate::hp::RomTiddle = &mut romtiddle_5;
    let mut string_4: std::string::String = crate::hp::RomTiddle::name(romtiddle_5_ref_0);
    let mut wonreasley_2: crate::hp::WonReasley = crate::hp::WonReasley {x: string_4, y: i64_4};
    let mut wonreasley_2_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_2;
    let mut i64_5: i64 = 8335i64;
    let mut u16_6: u16 = 6534u16;
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    let mut romtiddle_6_ref_0: &crate::hp::RomTiddle = &mut romtiddle_6;
    let mut string_5: std::string::String = crate::hp::RomTiddle::name(romtiddle_6_ref_0);
    let mut wonreasley_3: crate::hp::WonReasley = crate::hp::WonReasley {x: string_5, y: i64_5};
    crate::hp::WonReasley::ascendio(wonreasley_2_ref_0, i64_3);
    let mut i32_6: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_0_ref_0, usize_0, string_3);
    crate::hp::ParryHotter::alohomora(i32_3, i32_2, i32_1, i32_0);
    crate::hp::WonReasley::ascendio(wonreasley_1_ref_0, i64_1);
    crate::hp::RomTiddle::avada_kedavra(romtiddle_2_ref_0, string_1);
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4962() {
    rusty_monitor::set_test_id(4962);
    let mut i64_0: i64 = 10213i64;
    let mut i64_1: i64 = -5867i64;
    let mut u16_0: u16 = 9477u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut u16_1: u16 = 3277u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut i64_2: i64 = -14106i64;
    let mut i64_3: i64 = 2223i64;
    let mut u16_2: u16 = 8712u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut i32_0: i32 = 2116i32;
    let mut i32_1: i32 = 1528i32;
    let mut i32_2: i32 = 6807i32;
    let mut i32_3: i32 = -5207i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_4: i32 = -9034i32;
    let mut i32_5: i32 = 12822i32;
    let mut i32_6: i32 = 3431i32;
    let mut i32_7: i32 = -7981i32;
    let mut str_0: &str = "5CL7U7414NRHw";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_4: i64 = -1696i64;
    let mut u16_3: u16 = 6027u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_4};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    let mut u16_4: u16 = 6436u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut str_1: &str = "e4f";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut i64_5: i64 = 5457i64;
    let mut u16_5: u16 = 1007u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_5_ref_0: &crate::hp::RomTiddle = &mut romtiddle_5;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_5_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_5};
    let mut wonreasley_1_ref_0: &crate::hp::WonReasley = &mut wonreasley_1;
    let mut i32_8: i32 = -11283i32;
    let mut i32_9: i32 = -2982i32;
    let mut i32_10: i32 = 417i32;
    let mut i32_11: i32 = 65i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_11, i32_10);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    crate::hp::ParryHotter::foo2(parryhotter_1_ref_0, i32_9, i32_8);
    crate::hp::WonReasley::arania_exumai(wonreasley_1_ref_0, str_1_ref_0);
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
    crate::hp::ParryHotter::alohomora(i32_7, i32_6, i32_5, i32_4);
    crate::hp::ParryHotter::foo2(parryhotter_0_ref_0, i32_1, i32_0);
    crate::hp::RomTiddle::foo3(romtiddle_2_ref_0, i64_3, i64_2);
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    crate::hp::RomTiddle::foo3(romtiddle_0_ref_0, i64_1, i64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4970() {
    rusty_monitor::set_test_id(4970);
    let mut str_0: &str = "boCF9GY6v4ayfeyaKv";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_0: i64 = 15262i64;
    let mut u16_0: u16 = 1341u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    let mut u64_0: u64 = 9273u64;
    let mut u64_1: u64 = 260u64;
    let mut i64_1: i64 = 3826i64;
    let mut i64_2: i64 = 1359i64;
    let mut u16_1: u16 = 5902u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_2};
    let mut wonreasley_1_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_1;
    let mut i64_3: i64 = -1480i64;
    let mut i64_4: i64 = -5880i64;
    let mut u16_2: u16 = 8384u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut wonreasley_2: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_4};
    let mut wonreasley_2_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_2;
    let mut i64_5: i64 = -11856i64;
    let mut u16_3: u16 = 3180u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut i32_0: i32 = 8254i32;
    let mut i32_1: i32 = 16463i32;
    let mut i32_2: i32 = 300i32;
    let mut i32_3: i32 = -2749i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u16_4: u16 = 6863u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut string_4: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    let mut usize_0: usize = 913usize;
    let mut i32_4: i32 = 12619i32;
    let mut i32_5: i32 = 28998i32;
    let mut i32_6: i32 = -921i32;
    let mut i32_7: i32 = 5188i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_7, i32_6);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_8: i32 = 15406i32;
    let mut i32_9: i32 = -21338i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_9, i32_8);
    crate::hp::ParryHotter::foo2(parryhotter_1_ref_0, i32_5, i32_4);
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i32_10: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_2_ref_0, usize_0, string_4);
    crate::hp::ParryHotter::foo2(parryhotter_0_ref_0, i32_1, i32_0);
    let mut wonreasley_3: crate::hp::WonReasley = crate::hp::WonReasley {x: string_3, y: i64_5};
    crate::hp::WonReasley::ascendio(wonreasley_2_ref_0, i64_3);
    crate::hp::WonReasley::ascendio(wonreasley_1_ref_0, i64_1);
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4971() {
    rusty_monitor::set_test_id(4971);
    let mut i32_0: i32 = 984i32;
    let mut i32_1: i32 = -2247i32;
    let mut i32_2: i32 = -14740i32;
    let mut i32_3: i32 = -9833i32;
    let mut u16_0: u16 = 1856u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut usize_0: usize = 408usize;
    let mut i64_0: i64 = 10597i64;
    let mut u16_1: u16 = 9248u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut str_0: &str = "kD0m9Q0J9LddaFaWtZH";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_1: i64 = 16091i64;
    let mut u16_2: u16 = 7196u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_1};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    let mut i64_2: i64 = -9927i64;
    let mut i64_3: i64 = -16277i64;
    let mut u16_3: u16 = 185u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut u16_4: u16 = 8849u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut i32_4: i32 = 854i32;
    let mut i32_5: i32 = 4263i32;
    let mut i32_6: i32 = 2994i32;
    let mut i32_7: i32 = 1810i32;
    let mut i32_8: i32 = -12049i32;
    let mut i32_9: i32 = 18706i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_9, i32_8);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i64_4: i64 = -8088i64;
    let mut u16_5: u16 = 9648u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_5_ref_0: &crate::hp::RomTiddle = &mut romtiddle_5;
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_5_ref_0);
    let mut i32_10: i32 = -12170i32;
    let mut i32_11: i32 = 9906i32;
    let mut i32_12: i32 = -2938i32;
    let mut i32_13: i32 = -2813i32;
    let mut u16_6: u16 = 5897u16;
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    crate::hp::ParryHotter::alohomora(i32_13, i32_12, i32_11, i32_10);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_3, y: i64_4};
    let mut i32_14: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_7, i32_6);
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_5, b: i32_4};
    let mut string_4: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    crate::hp::RomTiddle::foo3(romtiddle_3_ref_0, i64_3, i64_2);
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
    let mut wonreasley_2: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_0};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_15: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_1_ref_0, usize_0, string_0);
    crate::hp::ParryHotter::alohomora(i32_3, i32_2, i32_1, i32_0);
    let mut romtiddle_6_ref_0: &crate::hp::RomTiddle = &mut romtiddle_6;
    let mut string_5: std::string::String = crate::hp::RomTiddle::name(romtiddle_6_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4973() {
    rusty_monitor::set_test_id(4973);
    let mut i32_0: i32 = 2838i32;
    let mut i32_1: i32 = 24063i32;
    let mut i32_2: i32 = 8180i32;
    let mut i32_3: i32 = 891i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut str_0: &str = "XmepAWRYO";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i32_4: i32 = 1491i32;
    let mut i32_5: i32 = -1695i32;
    let mut i32_6: i32 = 9009i32;
    let mut i32_7: i32 = -9257i32;
    let mut i32_8: i32 = 14748i32;
    let mut i32_9: i32 = -743i32;
    let mut u16_0: u16 = 9166u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut u16_1: u16 = 6418u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_1;
    let mut i32_10: i32 = -13697i32;
    let mut i32_11: i32 = 2789i32;
    let mut i64_0: i64 = -16845i64;
    let mut u16_2: u16 = 8872u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut i32_12: i32 = -2346i32;
    let mut i32_13: i32 = 5110i32;
    let mut i32_14: i32 = 14430i32;
    let mut i32_15: i32 = 2635i32;
    let mut i32_16: i32 = 1021i32;
    let mut i32_17: i32 = -6018i32;
    let mut i32_18: i32 = -2211i32;
    let mut i32_19: i32 = -13160i32;
    crate::hp::ParryHotter::alohomora(i32_19, i32_18, i32_17, i32_16);
    crate::hp::ParryHotter::alohomora(i32_15, i32_14, i32_13, i32_12);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_0};
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_11, i32_10);
    crate::hp::RomTiddle::avada_kedavra(romtiddle_1_ref_0, string_0);
    crate::hp::ParryHotter::alohomora(i32_9, i32_8, i32_7, i32_6);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    crate::hp::ParryHotter::foo2(parryhotter_1_ref_0, i32_5, i32_4);
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
    let mut i32_20: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_1, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4976() {
    rusty_monitor::set_test_id(4976);
    let mut i64_0: i64 = 13247i64;
    let mut i64_1: i64 = 729i64;
    let mut u16_0: u16 = 3990u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_1};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut u16_1: u16 = 9963u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut usize_0: usize = 5611usize;
    let mut i32_0: i32 = 9190i32;
    let mut i32_1: i32 = 848i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_2: i32 = 5788i32;
    let mut i32_3: i32 = -11665i32;
    let mut i32_4: i32 = 17550i32;
    let mut i32_5: i32 = 13659i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_5, b: i32_4};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut str_0: &str = "KW7OaSBPNBc2Ic";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_2: i64 = 6159i64;
    let mut u16_2: u16 = 9663u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_2};
    let mut wonreasley_1_ref_0: &crate::hp::WonReasley = &mut wonreasley_1;
    let mut u16_3: u16 = 7535u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut usize_1: usize = 3884usize;
    let mut i32_6: i32 = -12382i32;
    let mut i32_7: i32 = 3752i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_7, i32_6);
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i32_8: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_2_ref_0, usize_1, string_3);
    crate::hp::WonReasley::arania_exumai(wonreasley_1_ref_0, str_0_ref_0);
    crate::hp::ParryHotter::foo2(parryhotter_1_ref_0, i32_3, i32_2);
    let mut i32_9: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_0_ref_0, usize_0, string_1);
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4980() {
    rusty_monitor::set_test_id(4980);
    let mut i32_0: i32 = 22421i32;
    let mut i32_1: i32 = -23295i32;
    let mut i32_2: i32 = 5110i32;
    let mut i32_3: i32 = -16860i32;
    let mut i32_4: i32 = -5719i32;
    let mut i32_5: i32 = -19437i32;
    let mut str_0: &str = "JsFAw";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_0: i64 = -8096i64;
    let mut u16_0: u16 = 2322u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    let mut i32_6: i32 = 4087i32;
    let mut i32_7: i32 = -5668i32;
    let mut i32_8: i32 = -6790i32;
    let mut i32_9: i32 = -15649i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_9, b: i32_8};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u16_1: u16 = 4004u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut i32_10: i32 = -6396i32;
    let mut i32_11: i32 = -5425i32;
    let mut i32_12: i32 = 5517i32;
    let mut i32_13: i32 = -12573i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_13, b: i32_12};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i64_1: i64 = 169i64;
    let mut i64_2: i64 = 473i64;
    let mut u16_2: u16 = 9832u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut i64_3: i64 = 6106i64;
    let mut i64_4: i64 = -10780i64;
    let mut u16_3: u16 = 1050u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut u64_0: u64 = 4125u64;
    let mut u64_1: u64 = 9223u64;
    let mut i64_5: i64 = -10i64;
    let mut i64_6: i64 = -10297i64;
    let mut u16_4: u16 = 3513u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    crate::hp::RomTiddle::foo3(romtiddle_4_ref_0, i64_6, i64_5);
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    crate::hp::RomTiddle::foo3(romtiddle_3_ref_0, i64_4, i64_3);
    crate::hp::RomTiddle::foo3(romtiddle_2_ref_0, i64_2, i64_1);
    crate::hp::ParryHotter::foo2(parryhotter_1_ref_0, i32_11, i32_10);
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    crate::hp::ParryHotter::foo2(parryhotter_0_ref_0, i32_7, i32_6);
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_5, i32_4);
    crate::hp::ParryHotter::alohomora(i32_3, i32_2, i32_1, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4982() {
    rusty_monitor::set_test_id(4982);
    let mut u64_0: u64 = 6898u64;
    let mut u64_1: u64 = 1221u64;
    let mut usize_0: usize = 6812usize;
    let mut i32_0: i32 = 9034i32;
    let mut i32_1: i32 = -7685i32;
    let mut i32_2: i32 = -1557i32;
    let mut i32_3: i32 = -11322i32;
    let mut i32_4: i32 = 17585i32;
    let mut i32_5: i32 = -2672i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_5, b: i32_4};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_6: i32 = -4574i32;
    let mut i32_7: i32 = 10432i32;
    let mut i32_8: i32 = -1893i32;
    let mut i32_9: i32 = -13655i32;
    let mut i32_10: i32 = 11351i32;
    let mut i32_11: i32 = -10056i32;
    let mut i64_0: i64 = 3196i64;
    let mut i64_1: i64 = 11591i64;
    let mut u16_0: u16 = 9023u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_1};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut i32_12: i32 = 12130i32;
    let mut i32_13: i32 = -7011i32;
    let mut i32_14: i32 = 1371i32;
    let mut i32_15: i32 = -16265i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_15, i32_14);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i64_2: i64 = 1006i64;
    let mut i64_3: i64 = -247i64;
    let mut u16_1: u16 = 5701u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut i64_4: i64 = 6458i64;
    let mut i64_5: i64 = -9794i64;
    let mut i64_6: i64 = -6680i64;
    let mut i64_7: i64 = -14981i64;
    let mut u16_2: u16 = 7653u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_7};
    let mut wonreasley_1_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_1;
    let mut u16_3: u16 = 8179u16;
    let mut i32_16: i32 = 3917i32;
    let mut i32_17: i32 = 6362i32;
    let mut u16_4: u16 = 4423u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_17, i32_16);
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    crate::hp::WonReasley::ascendio(wonreasley_1_ref_0, i64_6);
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    crate::hp::RomTiddle::foo3(romtiddle_4_ref_0, i64_5, i64_4);
    crate::hp::RomTiddle::foo3(romtiddle_1_ref_0, i64_3, i64_2);
    let mut i32_18: i32 = crate::hp::ParryHotter::accio(parryhotter_1_ref_0, i32_13, i32_12);
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_0);
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    crate::hp::ParryHotter::foo2(parryhotter_2_ref_0, i32_11, i32_10);
    crate::hp::ParryHotter::alohomora(i32_9, i32_8, i32_7, i32_6);
    let mut i32_19: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_3, i32_2);
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut parryhotter_3_ref_0: &crate::hp::ParryHotter = &mut parryhotter_3;
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_3_ref_0, usize_0, u64_1, u64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4983() {
    rusty_monitor::set_test_id(4983);
    let mut u16_0: u16 = 9053u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut u16_1: u16 = 5009u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_1;
    let mut u64_0: u64 = 99u64;
    let mut u64_1: u64 = 9283u64;
    let mut i64_0: i64 = 6421i64;
    let mut i64_1: i64 = 26717i64;
    let mut u16_2: u16 = 2908u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut i32_0: i32 = -6237i32;
    let mut i32_1: i32 = 2784i32;
    let mut i32_2: i32 = 8108i32;
    let mut i32_3: i32 = 1086i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u16_3: u16 = 7002u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut usize_0: usize = 2525usize;
    let mut i32_4: i32 = 1265i32;
    let mut i32_5: i32 = 755i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_5, b: i32_4};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i64_2: i64 = -11122i64;
    let mut i64_3: i64 = 310i64;
    let mut u16_4: u16 = 2527u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut str_0: &str = "r0MuiL6ohpzD";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_4: i64 = -27166i64;
    let mut u16_5: u16 = 2349u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_5_ref_0: &crate::hp::RomTiddle = &mut romtiddle_5;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_5_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_4};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
    crate::hp::RomTiddle::foo3(romtiddle_4_ref_0, i64_3, i64_2);
    let mut i32_6: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_1_ref_0, usize_0, string_1);
    let mut i32_7: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_1, i32_0);
    crate::hp::RomTiddle::foo3(romtiddle_2_ref_0, i64_1, i64_0);
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    crate::hp::RomTiddle::avada_kedavra(romtiddle_1_ref_0, string_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4992() {
    rusty_monitor::set_test_id(4992);
    let mut str_0: &str = "h";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_0: i64 = 1717i64;
    let mut str_1: &str = "LDj";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut i64_1: i64 = -10309i64;
    let mut u16_0: u16 = 4953u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_1};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    let mut u16_1: u16 = 4334u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut usize_0: usize = 6726usize;
    let mut i32_0: i32 = 19975i32;
    let mut i32_1: i32 = -6291i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i64_2: i64 = 10500i64;
    let mut u16_2: u16 = 7571u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut i64_3: i64 = 3038i64;
    let mut u16_3: u16 = 5046u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_3, y: i64_3};
    let mut wonreasley_2: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_2};
    let mut i32_2: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_0_ref_0, usize_0, string_1);
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_1_ref_0);
    let mut wonreasley_2_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_2;
    crate::hp::WonReasley::ascendio(wonreasley_2_ref_0, i64_0);
    let mut wonreasley_1_ref_0: &crate::hp::WonReasley = &mut wonreasley_1;
    crate::hp::WonReasley::arania_exumai(wonreasley_1_ref_0, str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4994() {
    rusty_monitor::set_test_id(4994);
    let mut str_0: &str = "l0SOwVCRI37Ld11G9";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_0: i64 = -22860i64;
    let mut u16_0: u16 = 7460u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    let mut i64_1: i64 = -6933i64;
    let mut i64_2: i64 = -1612i64;
    let mut u16_1: u16 = 7774u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut i32_0: i32 = 5615i32;
    let mut i32_1: i32 = -837i32;
    let mut u64_0: u64 = 8542u64;
    let mut u64_1: u64 = 2076u64;
    let mut usize_0: usize = 2173usize;
    let mut i32_2: i32 = -1543i32;
    let mut i32_3: i32 = 985i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u16_2: u16 = 341u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut usize_1: usize = 6926usize;
    let mut i32_4: i32 = -53i32;
    let mut i32_5: i32 = 13875i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_5, i32_4);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_6: i32 = -7378i32;
    let mut i32_7: i32 = -3811i32;
    let mut i32_8: i32 = -14175i32;
    let mut i32_9: i32 = -9901i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_9, b: i32_8};
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i32_10: i32 = 7620i32;
    let mut i32_11: i32 = 11126i32;
    let mut i32_12: i32 = 4554i32;
    let mut i32_13: i32 = 3336i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_13, b: i32_12};
    let mut parryhotter_3_ref_0: &crate::hp::ParryHotter = &mut parryhotter_3;
    let mut i64_3: i64 = 7480i64;
    let mut i64_4: i64 = 2253i64;
    let mut u16_3: u16 = 1894u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    crate::hp::RomTiddle::foo3(romtiddle_3_ref_0, i64_4, i64_3);
    let mut i32_14: i32 = crate::hp::ParryHotter::accio(parryhotter_3_ref_0, i32_11, i32_10);
    let mut i32_15: i32 = crate::hp::ParryHotter::accio(parryhotter_2_ref_0, i32_7, i32_6);
    let mut i32_16: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_1_ref_0, usize_1, string_1);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_0_ref_0, usize_0, u64_1, u64_0);
    let mut parryhotter_4: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    crate::hp::RomTiddle::foo3(romtiddle_1_ref_0, i64_2, i64_1);
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4997() {
    rusty_monitor::set_test_id(4997);
    let mut u64_0: u64 = 2955u64;
    let mut u64_1: u64 = 9899u64;
    let mut usize_0: usize = 1614usize;
    let mut i32_0: i32 = 6395i32;
    let mut i32_1: i32 = -1800i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i64_0: i64 = 1079i64;
    let mut i64_1: i64 = -2180i64;
    let mut u16_0: u16 = 2442u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut u16_1: u16 = 9905u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut usize_1: usize = 1283usize;
    let mut i32_2: i32 = -20107i32;
    let mut i32_3: i32 = -12701i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_4: i32 = 13237i32;
    let mut i32_5: i32 = -4095i32;
    let mut i32_6: i32 = -2698i32;
    let mut i32_7: i32 = 4029i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_7, b: i32_6};
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i32_8: i32 = 8230i32;
    let mut i32_9: i32 = 5517i32;
    let mut i32_10: i32 = -6463i32;
    let mut i32_11: i32 = 2477i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_11, b: i32_10};
    let mut parryhotter_3_ref_0: &crate::hp::ParryHotter = &mut parryhotter_3;
    let mut u16_2: u16 = 9363u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut usize_2: usize = 8078usize;
    let mut i32_12: i32 = 2009i32;
    let mut i32_13: i32 = 767i32;
    let mut parryhotter_4: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_13, b: i32_12};
    let mut parryhotter_4_ref_0: &crate::hp::ParryHotter = &mut parryhotter_4;
    let mut u16_3: u16 = 3066u16;
    let mut i32_14: i32 = 2080i32;
    let mut i32_15: i32 = -1017i32;
    let mut i32_16: i32 = -16223i32;
    let mut i32_17: i32 = 11563i32;
    let mut u64_2: u64 = 7306u64;
    let mut u64_3: u64 = 5013u64;
    let mut usize_3: usize = 2981usize;
    let mut i32_18: i32 = 3756i32;
    let mut i32_19: i32 = -4170i32;
    let mut parryhotter_5: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_19, b: i32_18};
    let mut parryhotter_5_ref_0: &crate::hp::ParryHotter = &mut parryhotter_5;
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_5_ref_0, usize_3, u64_3, u64_2);
    crate::hp::ParryHotter::alohomora(i32_17, i32_16, i32_15, i32_14);
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut i32_20: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_4_ref_0, usize_2, string_1);
    let mut i32_21: i32 = crate::hp::ParryHotter::accio(parryhotter_3_ref_0, i32_9, i32_8);
    crate::hp::ParryHotter::foo2(parryhotter_2_ref_0, i32_5, i32_4);
    let mut i32_22: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_1_ref_0, usize_1, string_0);
    crate::hp::RomTiddle::foo3(romtiddle_0_ref_0, i64_1, i64_0);
    let mut bool_1: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_0_ref_0, usize_0, u64_1, u64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4998() {
    rusty_monitor::set_test_id(4998);
    let mut u16_0: u16 = 1862u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut u16_1: u16 = 8679u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_1;
    let mut u16_2: u16 = 8333u16;
    let mut i32_0: i32 = 28280i32;
    let mut i32_1: i32 = -7298i32;
    let mut u16_3: u16 = 9144u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut usize_0: usize = 5636usize;
    let mut i32_2: i32 = 3104i32;
    let mut i32_3: i32 = -6896i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u16_4: u16 = 8692u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut usize_1: usize = 5967usize;
    let mut i32_4: i32 = -5976i32;
    let mut i32_5: i32 = -4043i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_5, i32_4);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut u16_5: u16 = 9240u16;
    let mut u64_0: u64 = 6705u64;
    let mut u64_1: u64 = 5167u64;
    let mut u16_6: u16 = 2858u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut i32_6: i32 = 1134i32;
    let mut i32_7: i32 = 6277i32;
    let mut u16_7: u16 = 6904u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_7};
    let mut romtiddle_5_ref_0: &crate::hp::RomTiddle = &mut romtiddle_5;
    let mut i32_8: i32 = 15822i32;
    let mut i32_9: i32 = 10649i32;
    let mut i32_10: i32 = -12397i32;
    let mut i32_11: i32 = -423i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_11, b: i32_10};
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i32_12: i32 = 8857i32;
    let mut i32_13: i32 = -5983i32;
    let mut i32_14: i32 = 10789i32;
    let mut i32_15: i32 = 2941i32;
    let mut i32_16: i32 = 8590i32;
    let mut i32_17: i32 = -10771i32;
    let mut i32_18: i32 = 5437i32;
    let mut i32_19: i32 = 10547i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_19, i32_18);
    let mut parryhotter_3_ref_0: &crate::hp::ParryHotter = &mut parryhotter_3;
    let mut u16_8: u16 = 4422u16;
    let mut u16_9: u16 = 2228u16;
    let mut i32_20: i32 = 9823i32;
    let mut i32_21: i32 = 14483i32;
    let mut i32_22: i32 = 1981i32;
    let mut i32_23: i32 = 13271i32;
    crate::hp::ParryHotter::alohomora(i32_23, i32_22, i32_21, i32_20);
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_9};
    let mut romtiddle_7: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_8};
    let mut i32_24: i32 = crate::hp::ParryHotter::accio(parryhotter_3_ref_0, i32_17, i32_16);
    crate::hp::ParryHotter::alohomora(i32_15, i32_14, i32_13, i32_12);
    crate::hp::ParryHotter::foo2(parryhotter_2_ref_0, i32_9, i32_8);
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_5_ref_0);
    let mut parryhotter_4: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_7, b: i32_6};
    let mut string_4: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    let mut romtiddle_8: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut i32_25: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_1_ref_0, usize_1, string_2);
    let mut i32_26: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_0_ref_0, usize_0, string_1);
    let mut parryhotter_5: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut romtiddle_9: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    crate::hp::RomTiddle::avada_kedavra(romtiddle_1_ref_0, string_0);
    panic!("From RustyUnit with love");
}
}