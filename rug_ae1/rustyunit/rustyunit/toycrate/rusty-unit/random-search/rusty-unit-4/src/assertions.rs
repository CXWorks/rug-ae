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
fn rusty_test_1525() {
    rusty_monitor::set_test_id(1525);
    let mut i64_0: i64 = -10368i64;
    let mut i64_1: i64 = -6238i64;
    let mut u16_0: u16 = 3536u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut usize_0: usize = 809usize;
    let mut i32_0: i32 = -1089i32;
    let mut i32_1: i32 = -967i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u64_0: u64 = 329u64;
    let mut u64_1: u64 = 3130u64;
    let mut usize_1: usize = 1114usize;
    let mut i32_2: i32 = -11626i32;
    let mut i32_3: i32 = -11436i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut u16_1: u16 = 3918u16;
    let mut i32_4: i32 = -6234i32;
    let mut i32_5: i32 = -5937i32;
    let mut i32_6: i32 = 10937i32;
    let mut i32_7: i32 = 12273i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_7, b: i32_6};
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i32_8: i32 = crate::hp::ParryHotter::accio(parryhotter_2_ref_0, i32_5, i32_4);
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_1_ref_0, usize_1, u64_1, u64_0);
    let mut i32_9: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_0_ref_0, usize_0, string_0);
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    crate::hp::RomTiddle::foo3(romtiddle_1_ref_0, i64_1, i64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1403() {
    rusty_monitor::set_test_id(1403);
    let mut i32_0: i32 = -7999i32;
    let mut i32_1: i32 = -1270i32;
    let mut i32_2: i32 = -6284i32;
    let mut i32_3: i32 = -1219i32;
    let mut i64_0: i64 = -11496i64;
    let mut i64_1: i64 = 12597i64;
    let mut u16_0: u16 = 3055u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_1};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut u64_0: u64 = 7846u64;
    let mut u64_1: u64 = 4542u64;
    let mut usize_0: usize = 637usize;
    let mut i32_4: i32 = 9023i32;
    let mut i32_5: i32 = -1940i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_5, i32_4);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i64_2: i64 = 4203i64;
    let mut i64_3: i64 = 12649i64;
    let mut u16_1: u16 = 7574u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_3};
    let mut wonreasley_1_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_1;
    crate::hp::WonReasley::ascendio(wonreasley_1_ref_0, i64_2);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_0_ref_0, usize_0, u64_1, u64_0);
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_0);
    crate::hp::ParryHotter::alohomora(i32_3, i32_2, i32_1, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4880() {
    rusty_monitor::set_test_id(4880);
    let mut i32_0: i32 = 12307i32;
    let mut i32_1: i32 = -16247i32;
    let mut i32_2: i32 = -3719i32;
    let mut i32_3: i32 = 3772i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u64_0: u64 = 1936u64;
    let mut u64_1: u64 = 9942u64;
    let mut usize_0: usize = 4088usize;
    let mut i32_4: i32 = 8654i32;
    let mut i32_5: i32 = -12354i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_5, b: i32_4};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut str_0: &str = "yciK3XEoRYQf";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_0: i64 = -6746i64;
    let mut u16_0: u16 = 2649u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    let mut i32_6: i32 = -13054i32;
    let mut i32_7: i32 = 3089i32;
    let mut i32_8: i32 = 10990i32;
    let mut i32_9: i32 = 739i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_9, b: i32_8};
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    crate::hp::ParryHotter::foo2(parryhotter_2_ref_0, i32_7, i32_6);
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_1_ref_0, usize_0, u64_1, u64_0);
    crate::hp::ParryHotter::foo2(parryhotter_0_ref_0, i32_1, i32_0);
    panic!("From RustyUnit with love");
}
}