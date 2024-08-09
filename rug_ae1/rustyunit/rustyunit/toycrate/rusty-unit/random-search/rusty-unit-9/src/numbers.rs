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
fn rusty_test_900() {
    rusty_monitor::set_test_id(900);
    let mut u64_0: u64 = 1922u64;
    let mut u64_1: u64 = 7291u64;
    let mut u64_2: u64 = 1652u64;
    let mut u64_3: u64 = 4446u64;
    let mut usize_0: usize = 2488usize;
    let mut i32_0: i32 = -11i32;
    let mut i32_1: i32 = 12435i32;
    let mut i32_2: i32 = 1130i32;
    let mut i32_3: i32 = 14330i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_4: i32 = -15781i32;
    let mut i32_5: i32 = -3655i32;
    let mut i32_6: i32 = 9055i32;
    let mut i32_7: i32 = -5268i32;
    let mut u64_4: u64 = 7454u64;
    let mut u64_5: u64 = 2370u64;
    let mut u64_6: u64 = 4742u64;
    let mut u64_7: u64 = 3345u64;
    let mut i32_8: i32 = -13263i32;
    let mut i32_9: i32 = 6233i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_9, i32_8);
    crate::hp::ParryHotter::another_number_fn(u64_7, u64_6);
    crate::hp::ParryHotter::another_number_fn(u64_5, u64_4);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    crate::hp::ParryHotter::foo2(parryhotter_1_ref_0, i32_7, i32_6);
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_5, i32_4);
    let mut i32_10: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_1, i32_0);
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_2_ref_0, usize_0, u64_3, u64_2);
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4572() {
    rusty_monitor::set_test_id(4572);
    let mut i64_0: i64 = -4209i64;
    let mut i64_1: i64 = 1674i64;
    let mut u16_0: u16 = 9572u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_1};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut i32_0: i32 = 6537i32;
    let mut i32_1: i32 = 13156i32;
    let mut i32_2: i32 = -13886i32;
    let mut i32_3: i32 = -4997i32;
    let mut i32_4: i32 = 5542i32;
    let mut i32_5: i32 = 6105i32;
    let mut i32_6: i32 = 12128i32;
    let mut i32_7: i32 = -8125i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_7, i32_6);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_8: i32 = 14i32;
    let mut i32_9: i32 = 13724i32;
    let mut i32_10: i32 = -872i32;
    let mut i32_11: i32 = 2671i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_11, i32_10);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut u16_1: u16 = 6857u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut u16_2: u16 = 6973u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_2;
    let mut i64_2: i64 = 3452i64;
    let mut i64_3: i64 = -2559i64;
    let mut u16_3: u16 = 4600u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut u64_0: u64 = 3293u64;
    let mut u64_1: u64 = 3706u64;
    let mut i32_12: i32 = -9162i32;
    let mut i32_13: i32 = -638i32;
    let mut i32_14: i32 = -21086i32;
    let mut i32_15: i32 = -2887i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_15, i32_14);
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i32_16: i32 = 750i32;
    let mut i32_17: i32 = -21380i32;
    let mut i32_18: i32 = -1164i32;
    let mut i32_19: i32 = -12940i32;
    let mut u64_2: u64 = 9132u64;
    let mut u64_3: u64 = 6482u64;
    let mut usize_0: usize = 3652usize;
    let mut i32_20: i32 = 7579i32;
    let mut i32_21: i32 = 3393i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_21, i32_20);
    let mut parryhotter_3_ref_0: &crate::hp::ParryHotter = &mut parryhotter_3;
    let mut i32_22: i32 = 547i32;
    let mut i32_23: i32 = 21529i32;
    let mut i32_24: i32 = 14212i32;
    let mut i32_25: i32 = -13406i32;
    let mut parryhotter_4: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_25, b: i32_24};
    let mut parryhotter_4_ref_0: &crate::hp::ParryHotter = &mut parryhotter_4;
    let mut i32_26: i32 = -479i32;
    let mut i32_27: i32 = -12803i32;
    let mut i32_28: i32 = -20565i32;
    let mut i32_29: i32 = 13098i32;
    let mut i64_4: i64 = -14719i64;
    let mut i64_5: i64 = 8190i64;
    let mut u16_4: u16 = 8827u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    crate::hp::RomTiddle::foo3(romtiddle_4_ref_0, i64_5, i64_4);
    crate::hp::ParryHotter::alohomora(i32_29, i32_28, i32_27, i32_26);
    let mut i32_30: i32 = crate::hp::ParryHotter::accio(parryhotter_4_ref_0, i32_23, i32_22);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_3_ref_0, usize_0, u64_3, u64_2);
    crate::hp::ParryHotter::alohomora(i32_19, i32_18, i32_17, i32_16);
    crate::hp::ParryHotter::foo2(parryhotter_2_ref_0, i32_13, i32_12);
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    crate::hp::RomTiddle::foo3(romtiddle_3_ref_0, i64_3, i64_2);
    crate::hp::RomTiddle::avada_kedavra(romtiddle_2_ref_0, string_1);
    crate::hp::ParryHotter::foo2(parryhotter_1_ref_0, i32_9, i32_8);
    crate::hp::ParryHotter::foo2(parryhotter_0_ref_0, i32_5, i32_4);
    crate::hp::ParryHotter::alohomora(i32_3, i32_2, i32_1, i32_0);
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_107() {
    rusty_monitor::set_test_id(107);
    let mut u16_0: u16 = 9611u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut u16_1: u16 = 2613u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_1;
    let mut i32_0: i32 = -14049i32;
    let mut i32_1: i32 = -979i32;
    let mut u64_0: u64 = 4721u64;
    let mut u64_1: u64 = 162u64;
    let mut usize_0: usize = 4419usize;
    let mut i32_2: i32 = 8818i32;
    let mut i32_3: i32 = -4824i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u64_2: u64 = 271u64;
    let mut u64_3: u64 = 9647u64;
    let mut u16_2: u16 = 9884u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut usize_1: usize = 335usize;
    let mut i32_4: i32 = 7782i32;
    let mut i32_5: i32 = -4307i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_5, i32_4);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_6: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_1_ref_0, usize_1, string_1);
    crate::hp::ParryHotter::another_number_fn(u64_3, u64_2);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_0_ref_0, usize_0, u64_1, u64_0);
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    crate::hp::RomTiddle::avada_kedavra(romtiddle_1_ref_0, string_0);
    panic!("From RustyUnit with love");
}
}