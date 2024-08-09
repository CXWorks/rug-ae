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
fn rusty_test_3009() {
    rusty_monitor::set_test_id(3009);
    let mut i32_0: i32 = -6233i32;
    let mut i32_1: i32 = -9672i32;
    let mut str_0: &str = "IPn6jLOBAYuY5V5M";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u16_0: u16 = 8232u16;
    let mut i32_2: i32 = 4992i32;
    let mut i32_3: i32 = 17082i32;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut u16_1: u16 = 4205u16;
    let mut u16_2: u16 = 4481u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut i32_4: i32 = -8392i32;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut i64_0: i64 = 10059i64;
    let mut i64_1: i64 = 19831i64;
    let mut u16_3: u16 = 5912u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_1};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut u16_4: u16 = 6208u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_4_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_4;
    let mut u16_5: u16 = 1646u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_5_ref_0: &crate::hp::RomTiddle = &mut romtiddle_5;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_5_ref_0);
    let mut i32_5: i32 = 27404i32;
    let mut i32_6: i32 = 8117i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_6, i32_5);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_7: i32 = -8203i32;
    let mut i32_8: i32 = 3878i32;
    let mut i32_9: i32 = 5793i32;
    let mut i32_10: i32 = -22453i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_10, b: i32_9};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i64_2: i64 = -8667i64;
    let mut i32_11: i32 = -4642i32;
    let mut i32_12: i32 = 3875i32;
    let mut i32_13: i32 = -5524i32;
    let mut i32_14: i32 = 3078i32;
    let mut i64_3: i64 = 21139i64;
    let mut u16_6: u16 = 8445u16;
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    let mut romtiddle_6_ref_0: &crate::hp::RomTiddle = &mut romtiddle_6;
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_6_ref_0);
    let mut u64_0: u64 = 3219u64;
    let mut u64_1: u64 = 8327u64;
    let mut u64_2: u64 = 1407u64;
    let mut u64_3: u64 = 7753u64;
    let mut usize_0: usize = 2442usize;
    let mut i32_15: i32 = 8426i32;
    let mut i32_16: i32 = -14444i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_16, i32_15);
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut str_1: &str = "UDv1fPDNdjnWWGjF";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut i64_4: i64 = 17408i64;
    let mut u16_7: u16 = 1195u16;
    let mut romtiddle_7: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_7};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_4: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_4, y: i64_4};
    let mut wonreasley_1_ref_0: &crate::hp::WonReasley = &mut wonreasley_1;
    crate::hp::WonReasley::arania_exumai(wonreasley_1_ref_0, str_1_ref_0);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_2_ref_0, usize_0, u64_3, u64_2);
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    let mut wonreasley_2: crate::hp::WonReasley = crate::hp::WonReasley {x: string_3, y: i64_3};
    crate::hp::ParryHotter::alohomora(i32_14, i32_13, i32_12, i32_11);
    let mut wonreasley_2_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_2;
    crate::hp::WonReasley::ascendio(wonreasley_2_ref_0, i64_2);
    let mut i32_17: i32 = crate::hp::ParryHotter::accio(parryhotter_1_ref_0, i32_8, i32_7);
    crate::hp::RomTiddle::avada_kedavra(romtiddle_4_ref_0, string_2);
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_0);
    crate::hp::ParryHotter::foo2(parryhotter_0_ref_0, i32_2, i32_3);
    let mut romtiddle_7_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_7;
    crate::hp::ParryHotter::alohomora(i32_4, i32_17, i32_0, i32_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_349() {
    rusty_monitor::set_test_id(349);
    let mut i64_0: i64 = 29598i64;
    let mut u16_0: u16 = 9338u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut u64_0: u64 = 3029u64;
    let mut u64_1: u64 = 6346u64;
    let mut u16_1: u16 = 8301u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut u16_2: u16 = 967u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut i32_0: i32 = -8842i32;
    let mut i32_1: i32 = -4266i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_2: i32 = -18745i32;
    let mut i32_3: i32 = 216i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7867() {
    rusty_monitor::set_test_id(7867);
    let mut u64_0: u64 = 7493u64;
    let mut u64_1: u64 = 9365u64;
    let mut usize_0: usize = 7804usize;
    let mut i32_0: i32 = 13499i32;
    let mut i32_1: i32 = -1742i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_1, b: i32_0};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u16_0: u16 = 5200u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut u16_1: u16 = 5597u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_1;
    let mut i32_2: i32 = 3450i32;
    let mut i32_3: i32 = 231i32;
    let mut i32_4: i32 = -14914i32;
    let mut i32_5: i32 = -12134i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_5, b: i32_4};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_6: i32 = crate::hp::ParryHotter::accio(parryhotter_1_ref_0, i32_3, i32_2);
    crate::hp::RomTiddle::avada_kedavra(romtiddle_1_ref_0, string_0);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_0_ref_0, usize_0, u64_1, u64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7470() {
    rusty_monitor::set_test_id(7470);
    let mut u16_0: u16 = 9202u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut u64_0: u64 = 4987u64;
    let mut u64_1: u64 = 2613u64;
    let mut usize_0: usize = 2975usize;
    let mut i32_0: i32 = 6023i32;
    let mut i32_1: i32 = 9163i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u16_1: u16 = 118u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut u16_2: u16 = 4205u16;
    let mut u16_3: u16 = 4481u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut i64_0: i64 = 19831i64;
    let mut u16_4: u16 = 666u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_0};
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_0_ref_0, usize_0, u64_1, u64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6507() {
    rusty_monitor::set_test_id(6507);
    let mut i32_0: i32 = 10677i32;
    let mut i32_1: i32 = -2754i32;
    let mut i32_2: i32 = 4996i32;
    let mut i32_3: i32 = 3588i32;
    let mut i64_0: i64 = 111i64;
    let mut i64_1: i64 = 17307i64;
    let mut u16_0: u16 = 7137u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut i64_2: i64 = -5971i64;
    let mut u16_1: u16 = 3786u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_2};
    let mut u16_2: u16 = 2919u16;
    let mut i64_3: i64 = -12667i64;
    let mut u16_3: u16 = 4021u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_3};
    let mut i64_4: i64 = -2211i64;
    let mut u16_4: u16 = 336u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut i64_5: i64 = 7998i64;
    let mut u16_5: u16 = 4145u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    let mut u16_6: u16 = 9246u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    let mut romtiddle_5_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_5;
    let mut i32_4: i32 = 8177i32;
    let mut i32_5: i32 = -16114i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_5, b: i32_4};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut wonreasley_1_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_1;
    let mut i64_6: i64 = -12978i64;
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    crate::hp::WonReasley::ascendio(wonreasley_1_ref_0, i64_6);
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_5);
    let mut wonreasley_2: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_4};
    crate::hp::RomTiddle::foo3(romtiddle_0_ref_0, i64_1, i64_0);
    crate::hp::ParryHotter::alohomora(i32_3, i32_2, i32_1, i32_0);
    panic!("From RustyUnit with love");
}
}