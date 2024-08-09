pub struct ParryHotter {
    pub a: i32,
    pub b: i32
}

impl ParryHotter {
    pub fn new(a: i32, b: i32) -> Self {
        Self {
            a, b
        }
    }
    pub fn alohomora(x: i32, y: i32, z: i32, b: i32) {
        if x == 7 {
            if y < 13 {
                println!("Closed again");
            } else {
                // ...
                println!("Where is the door?");
            }
        } else if y == 10 {
            if y >= 3000 {
                println!("Door opened");
            } else {
                print!("Hey");
            }
        } else {
            print!("Wow");
        }
    }

    pub fn aguamenti(&self, x: usize, y: String) -> i32 {
        if y.contains("two") {
            if x < 10 {
                11
            } else {
                10
            }
        } else {
            if y.contains("aqua") {
                if self.aqua_eructo(y.len(), x as u64, (x * 2) as u64) {
                    15
                } else {
                    10
                }
            } else {
                if &y == "Hermione" {
                    111
                } else {
                    1
                }
            }
        }
    }

    pub fn aqua_eructo(&self, x: usize, a: u64, b: u64) -> bool {
        if (a as usize) < x {
            if b as usize == x {
                true
            } else {
                false
            }
        } else {
            if a == b {
                if b as usize > x {
                    true
                } else {
                    if x == 4955 {
                        true
                    } else {
                        false
                    }
                }
            } else {
                false
            }
        }
    }

    pub fn accio(&self, x: i32, y: i32) -> i32 {
        if x == 20 {
            if x < 20 {
                140
            } else {
                ParryHotter::alohomora(y, x, x * 3, y - 10);
                32
            }
        } else {
            ParryHotter::aguamenti(self, 20, "ten".to_string());
            if x * self.b < 100 {
                20
            } else {
                10
            }
        }
    }

    pub fn another_number_fn(x: u64, y: u64) {
        if x == y {
            print!("Hey there");
        } else if x == y + 20 {
            println!("Hi");
        } else {
            println!("hello");
        }
    }

    pub fn foo2(&self, x: i32, y: i32) {
        if x == 140 && y == 40 {
           print!("Foo2 here");
        } else {
            if y < 20 && y > 10 {
                println!("Hi there");
            } else {
                println!("Foo2 exit");
            }
        }
    }
}

pub struct WonReasley {
    pub x: String,
    pub y: i64
}

impl WonReasley {
    pub fn arania_exumai(&self, at: &str) {
        if &self.x == "afraid" {
            if at == "hogwarts" {
                println!("run");
            } else if at == "home" {
                if self.y < -400 {
                    print!("keep calm");
                } else {
                    println!("pretend dead");
                }
            }
        }
    }

    pub fn ascendio(&mut self, x: i64) {
        self.y = self.y * 100;
        if self.y == 3700 {
            println!("flyyyy");
        } else if self.y == 0 {
            println!(":(");
        }
    }
}

pub struct RomTiddle {
    pub horcrux: u16
}

impl RomTiddle {
    pub fn name(&self) -> String {
        if self.horcrux < 3 {
            // slowly dying
            "Rom Tiddle".to_string()
        } else {
            "Lord Voldemort".to_string()
        }
    }

    pub fn avada_kedavra(&mut self, target: String) {
        if &target == "Parry Hotter" {
            self.horcrux -= 1;
        } else {
            // Use the soul to setup a horcrux
            self.horcrux += 1;
        }
    }

    pub fn foo3(&self, x: i64, y: i64) {
        if x == -4 {
            print!("Oh");
        } else {
            if y == 111 {
                println!("111");
            } else {
                print!(":(");
            }
        }
    }

}


#[cfg(test)]
mod rusty_tests {
	use crate::*;

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1560() {
//    rusty_monitor::set_test_id(1560);
    let mut i32_0: i32 = -9259i32;
    let mut i32_1: i32 = 20i32;
    let mut i32_2: i32 = 5373i32;
    let mut i32_3: i32 = 32i32;
    let mut u64_0: u64 = 3154u64;
    let mut u64_1: u64 = 20u64;
    let mut usize_0: usize = 10usize;
    let mut i32_4: i32 = 7i32;
    let mut i32_5: i32 = 3i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_5, b: i32_4};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_6: i32 = 9423i32;
    let mut i32_7: i32 = 40i32;
    let mut i32_8: i32 = 32i32;
    let mut i32_9: i32 = 15i32;
    let mut i32_10: i32 = -7060i32;
    let mut i32_11: i32 = 111i32;
    let mut str_0: &str = "ten";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u16_0: u16 = 426u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_11, b: i32_10};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_12: i32 = crate::hp::ParryHotter::accio(parryhotter_1_ref_0, i32_9, i32_8);
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_7, b: i32_6};
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_0_ref_0, usize_0, u64_1, u64_0);
    crate::hp::ParryHotter::alohomora(i32_3, i32_2, i32_1, i32_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4538() {
//    rusty_monitor::set_test_id(4538);
    let mut u16_0: u16 = 1u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut u16_1: u16 = 3u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut u16_2: u16 = 7958u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_2;
    let mut i32_0: i32 = 3000i32;
    let mut i32_1: i32 = 1495i32;
    let mut i32_2: i32 = 111i32;
    let mut i32_3: i32 = -11206i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_4: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_1, i32_0);
    crate::hp::RomTiddle::avada_kedavra(romtiddle_2_ref_0, string_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_975() {
//    rusty_monitor::set_test_id(975);
    let mut str_0: &str = "vzxKxmocBOynDz";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u64_0: u64 = 4137u64;
    let mut u64_1: u64 = 1436u64;
    let mut usize_0: usize = 4955usize;
    let mut i32_0: i32 = 11i32;
    let mut i32_1: i32 = 7i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_1, b: i32_0};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u16_0: u16 = 8576u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_0_ref_0, usize_0, u64_1, u64_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5029() {
//    rusty_monitor::set_test_id(5029);
    let mut str_0: &str = "vzxKxmocBOynDz";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_0: i64 = 111i64;
    let mut u16_0: u16 = 963u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut u64_0: u64 = 20u64;
    let mut u64_1: u64 = 8150u64;
    let mut usize_0: usize = 6196usize;
    let mut i32_0: i32 = 1103i32;
    let mut i32_1: i32 = 7i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_1, b: i32_0};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u16_1: u16 = 8576u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_0_ref_0, usize_0, u64_1, u64_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6806() {
//    rusty_monitor::set_test_id(6806);
    let mut str_0: &str = "vzxKxmocBOynDz";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u64_0: u64 = 20u64;
    let mut u64_1: u64 = 8150u64;
    let mut usize_0: usize = 6196usize;
    let mut i32_0: i32 = 1103i32;
    let mut i32_1: i32 = 7i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_1, b: i32_0};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_2: i32 = 11i32;
    let mut i32_3: i32 = 7i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    let mut u16_0: u16 = 8576u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_0_ref_0, usize_0, u64_1, u64_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_90() {
//    rusty_monitor::set_test_id(90);
    let mut i32_0: i32 = 13i32;
    let mut i32_1: i32 = 15i32;
    let mut i32_2: i32 = 3i32;
    let mut i32_3: i32 = 111i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_4: i32 = -12836i32;
    let mut i32_5: i32 = -6512i32;
    let mut i32_6: i32 = -3442i32;
    let mut i32_7: i32 = 9580i32;
    let mut u16_0: u16 = 3u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut usize_0: usize = 3007usize;
    let mut i32_8: i32 = 19698i32;
    let mut i32_9: i32 = -22651i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_9, b: i32_8};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_10: i32 = 32i32;
    let mut i32_11: i32 = 15i32;
    let mut i32_12: i32 = 1i32;
    let mut i32_13: i32 = 10i32;
    let mut u64_0: u64 = 20u64;
    let mut u64_1: u64 = 20u64;
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    crate::hp::ParryHotter::alohomora(i32_13, i32_12, i32_11, i32_10);
    let mut i32_14: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_1_ref_0, usize_0, string_0);
    crate::hp::ParryHotter::alohomora(i32_7, i32_6, i32_5, i32_4);
    crate::hp::ParryHotter::foo2(parryhotter_0_ref_0, i32_1, i32_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2582() {
//    rusty_monitor::set_test_id(2582);
    let mut u16_0: u16 = 8250u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut u16_1: u16 = 7548u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_1;
    let mut i32_0: i32 = -8063i32;
    let mut i32_1: i32 = 20i32;
    let mut i32_2: i32 = 20i32;
    let mut i32_3: i32 = -12567i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u64_0: u64 = 20u64;
    let mut u64_1: u64 = 8219u64;
    let mut i32_4: i32 = 20i32;
    let mut i32_5: i32 = 15i32;
    let mut i32_6: i32 = -2811i32;
    let mut i32_7: i32 = 13i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_7, i32_6);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_8: i32 = 111i32;
    let mut i32_9: i32 = 100i32;
    let mut u16_2: u16 = 3u16;
    let mut i32_10: i32 = -9259i32;
    let mut i32_11: i32 = 20i32;
    let mut i32_12: i32 = 5373i32;
    let mut i32_13: i32 = 32i32;
    let mut u64_2: u64 = 3154u64;
    let mut u64_3: u64 = 20u64;
    let mut usize_0: usize = 10usize;
    let mut i32_14: i32 = 7i32;
    let mut i32_15: i32 = 3i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_15, b: i32_14};
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i32_16: i32 = 9423i32;
    let mut i32_17: i32 = 40i32;
    let mut i32_18: i32 = 32i32;
    let mut i32_19: i32 = 15i32;
    let mut i32_20: i32 = -7060i32;
    let mut i32_21: i32 = 1i32;
    let mut str_0: &str = "ten";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_0: i64 = 9223372036854775807i64;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_0};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_21, b: i32_20};
    let mut parryhotter_3_ref_0: &crate::hp::ParryHotter = &mut parryhotter_3;
    let mut i32_22: i32 = crate::hp::ParryHotter::accio(parryhotter_3_ref_0, i32_19, i32_18);
    let mut parryhotter_4: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_17, b: i32_16};
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_2_ref_0, usize_0, u64_3, u64_2);
    crate::hp::ParryHotter::alohomora(i32_13, i32_12, i32_11, i32_10);
    let mut parryhotter_5: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_9, i32_8);
    let mut i32_23: i32 = crate::hp::ParryHotter::accio(parryhotter_1_ref_0, i32_5, i32_4);
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    let mut i32_24: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_1, i32_0);
    crate::hp::RomTiddle::avada_kedavra(romtiddle_1_ref_0, string_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8395() {
//    rusty_monitor::set_test_id(8395);
    let mut i64_0: i64 = -5003i64;
    let mut u16_0: u16 = 1u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut u16_1: u16 = 3u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut str_0: &str = "4BY";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u16_2: u16 = 3u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_107() {
//    rusty_monitor::set_test_id(107);
    let mut u64_0: u64 = 1015u64;
    let mut u64_1: u64 = 6907u64;
    let mut u64_2: u64 = 7741u64;
    let mut u64_3: u64 = 1589u64;
    let mut usize_0: usize = 2799usize;
    let mut i32_0: i32 = 2039i32;
    let mut i32_1: i32 = 6370i32;
    let mut i64_0: i64 = 3700i64;
    let mut i64_1: i64 = 0i64;
    let mut u16_0: u16 = 3u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut str_0: &str = "yKqaSr8";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_2: i64 = 20388i64;
    let mut u16_1: u16 = 9996u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_2};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_1};
    let mut wonreasley_1_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_1;
    crate::hp::WonReasley::ascendio(wonreasley_1_ref_0, i64_0);
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_0_ref_0, usize_0, u64_3, u64_2);
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_348() {
//    rusty_monitor::set_test_id(348);
    let mut i64_0: i64 = 8371i64;
    let mut i64_1: i64 = -4291i64;
    let mut u16_0: u16 = 1u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_1};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut u16_1: u16 = 3u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut u16_2: u16 = 7958u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_2;
    crate::hp::RomTiddle::avada_kedavra(romtiddle_2_ref_0, string_1);
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_0);
//    panic!("From RustyUnit with love");
}
}