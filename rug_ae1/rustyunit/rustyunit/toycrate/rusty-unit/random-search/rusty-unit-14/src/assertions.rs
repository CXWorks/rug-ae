fn foo(x: &[i32], y: usize) -> i32 {
    x[y]
}


#[cfg(test)]
mod rusty_tests {
	use crate::*;

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3216() {
    rusty_monitor::set_test_id(3216);
    let mut u16_0: u16 = 1531u16;
    let mut u64_0: u64 = 7480u64;
    let mut u64_1: u64 = 6563u64;
    let mut u16_1: u16 = 8585u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut i32_0: i32 = 15910i32;
    let mut i32_1: i32 = 25105i32;
    let mut i32_2: i32 = -262i32;
    let mut i32_3: i32 = -9239i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u16_2: u16 = 1954u16;
    let mut i32_4: i32 = 145i32;
    let mut i32_5: i32 = 6789i32;
    let mut u64_2: u64 = 8635u64;
    let mut u64_3: u64 = 9485u64;
    let mut usize_0: usize = 4408usize;
    let mut i32_6: i32 = -20261i32;
    let mut i32_7: i32 = -4341i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_7, i32_6);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_1_ref_0, usize_0, u64_3, u64_2);
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_5, b: i32_4};
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut i32_8: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_1, i32_0);
    let mut romtiddle_1_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_1;
    crate::hp::RomTiddle::avada_kedavra(romtiddle_1_ref_0, string_0);
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4612() {
    rusty_monitor::set_test_id(4612);
    let mut i64_0: i64 = 1040i64;
    let mut i64_1: i64 = 6672i64;
    let mut u16_0: u16 = 4762u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut u64_0: u64 = 5908u64;
    let mut u64_1: u64 = 9694u64;
    let mut i64_2: i64 = -380i64;
    let mut u16_1: u16 = 23u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut i64_3: i64 = 6658i64;
    let mut i64_4: i64 = -8385i64;
    let mut u16_2: u16 = 9041u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut str_0: &str = "WHlOKzX9SnmOh";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_5: i64 = -18844i64;
    let mut u16_3: u16 = 4334u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_5};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
    crate::hp::RomTiddle::foo3(romtiddle_2_ref_0, i64_4, i64_3);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_2};
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    crate::hp::RomTiddle::foo3(romtiddle_0_ref_0, i64_1, i64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4218() {
    rusty_monitor::set_test_id(4218);
    let mut i64_0: i64 = -1340i64;
    let mut i64_1: i64 = -2619i64;
    let mut u16_0: u16 = 644u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut u64_0: u64 = 5899u64;
    let mut u64_1: u64 = 2497u64;
    let mut usize_0: usize = 3781usize;
    let mut i32_0: i32 = -641i32;
    let mut i32_1: i32 = -27767i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_2: i32 = -5778i32;
    let mut i32_3: i32 = 11375i32;
    let mut i32_4: i32 = -4964i32;
    let mut i32_5: i32 = -645i32;
    let mut i32_6: i32 = 12505i32;
    let mut i32_7: i32 = -18580i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_7, i32_6);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut u16_1: u16 = 1187u16;
    let mut i32_8: i32 = -6939i32;
    let mut i32_9: i32 = -4300i32;
    let mut i32_10: i32 = -1356i32;
    let mut i32_11: i32 = 3122i32;
    crate::hp::ParryHotter::alohomora(i32_11, i32_10, i32_9, i32_8);
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    crate::hp::ParryHotter::foo2(parryhotter_1_ref_0, i32_5, i32_4);
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_0_ref_0, usize_0, u64_1, u64_0);
    crate::hp::RomTiddle::foo3(romtiddle_0_ref_0, i64_1, i64_0);
    panic!("From RustyUnit with love");
}
}