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

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2636() {
    rusty_monitor::set_test_id(2636);
    let mut i64_0: i64 = -6027i64;
    let mut u16_0: u16 = 2221u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut u16_1: u16 = 5779u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut u16_2: u16 = 3877u16;
    let mut i32_0: i32 = -9300i32;
    let mut i32_1: i32 = -15591i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_1, b: i32_0};
    let mut u16_3: u16 = 4986u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut u16_4: u16 = 2947u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_3_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_3;
    let mut i32_2: i32 = 5414i32;
    let mut i32_3: i32 = -271i32;
    let mut i32_4: i32 = 5987i32;
    let mut u16_5: u16 = 1034u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut u64_0: u64 = 4454u64;
    let mut u64_1: u64 = 8266u64;
    let mut usize_0: usize = 656usize;
    let mut i32_5: i32 = 918i32;
    let mut i32_6: i32 = 18579i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_6, i32_5);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i32_7: i32 = -688i32;
    let mut i32_8: i32 = -19885i32;
    let mut i32_9: i32 = 4900i32;
    let mut i32_10: i32 = 5744i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_10, b: i32_7};
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i64_1: i64 = -4552i64;
    let mut i64_2: i64 = 1125i64;
    let mut u16_6: u16 = 8785u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    let mut romtiddle_5_ref_0: &crate::hp::RomTiddle = &mut romtiddle_5;
    let mut i32_11: i32 = -4942i32;
    let mut i32_12: i32 = -3039i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_12, b: i32_11};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u16_7: u16 = 5670u16;
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_5_ref_0);
    let mut i32_13: i32 = 6899i32;
    let mut i32_14: i32 = -3733i32;
    let mut i32_15: i32 = 17243i32;
    let mut i32_16: i32 = -10274i32;
    let mut u16_8: u16 = 7218u16;
    let mut romtiddle_7: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_7};
    let mut romtiddle_6_ref_0: &crate::hp::RomTiddle = &mut romtiddle_6;
    let mut i64_3: i64 = -12324i64;
    let mut romtiddle_8: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_8};
    let mut romtiddle_7_ref_0: &crate::hp::RomTiddle = &mut romtiddle_7;
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_6_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_2};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    let mut u16_9: u16 = 7461u16;
    let mut i32_17: i32 = -4513i32;
    let mut i32_18: i32 = -3378i32;
    let mut i32_19: i32 = -22508i32;
    let mut i32_20: i32 = -3418i32;
    let mut parryhotter_4: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_8, b: i32_9};
    let mut parryhotter_3_ref_0: &crate::hp::ParryHotter = &mut parryhotter_3;
    let mut i32_21: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_16, i32_2);
    let mut romtiddle_9: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_9};
    let mut string_4: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    crate::hp::ParryHotter::alohomora(i32_20, i32_19, i32_4, i32_15);
    let mut romtiddle_8_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_8;
    crate::hp::RomTiddle::avada_kedavra(romtiddle_3_ref_0, string_3);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_3_ref_0, usize_0, u64_0, u64_1);
    crate::hp::RomTiddle::foo3(romtiddle_1_ref_0, i64_3, i64_1);
    crate::hp::ParryHotter::foo2(parryhotter_1_ref_0, i32_17, i32_14);
    crate::hp::ParryHotter::alohomora(i32_13, i32_21, i32_3, i32_18);
    crate::hp::RomTiddle::avada_kedavra(romtiddle_8_ref_0, string_4);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7610() {
    rusty_monitor::set_test_id(7610);
    let mut u16_0: u16 = 7318u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut u16_1: u16 = 2770u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut u16_2: u16 = 8434u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_2;
    let mut str_0: &str = "JBoG70gGOzG377B2iJ";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_0: i64 = 1861i64;
    let mut u16_3: u16 = 548u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut i32_0: i32 = 4841i32;
    let mut i32_1: i32 = 5585i32;
    let mut i32_2: i32 = -29970i32;
    let mut i32_3: i32 = -6853i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    crate::hp::ParryHotter::foo2(parryhotter_0_ref_0, i32_1, i32_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_0};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
    crate::hp::RomTiddle::avada_kedavra(romtiddle_2_ref_0, string_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8262() {
    rusty_monitor::set_test_id(8262);
    let mut i64_0: i64 = -10999i64;
    let mut i64_1: i64 = 2560i64;
    let mut u16_0: u16 = 411u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut u16_1: u16 = 2770u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut u16_2: u16 = 8434u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_2;
    let mut str_0: &str = "JBoG70gGOzG377B2iJ";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_2: i64 = 1861i64;
    let mut u16_3: u16 = 548u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_2};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
    crate::hp::RomTiddle::avada_kedavra(romtiddle_2_ref_0, string_0);
    crate::hp::RomTiddle::foo3(romtiddle_0_ref_0, i64_1, i64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6628() {
    rusty_monitor::set_test_id(6628);
    let mut i32_0: i32 = -10209i32;
    let mut i32_1: i32 = 4549i32;
    let mut i32_2: i32 = -2949i32;
    let mut i32_3: i32 = 5383i32;
    let mut i64_0: i64 = -2673i64;
    let mut i64_1: i64 = 2968i64;
    let mut u16_0: u16 = 7367u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut i32_4: i32 = 373i32;
    let mut i32_5: i32 = -4412i32;
    let mut i32_6: i32 = -9087i32;
    let mut i32_7: i32 = 8191i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_7, b: i32_6};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u64_0: u64 = 1903u64;
    let mut u64_1: u64 = 5036u64;
    let mut u16_1: u16 = 5304u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    let mut i32_8: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_5, i32_4);
    crate::hp::RomTiddle::foo3(romtiddle_0_ref_0, i64_1, i64_0);
    crate::hp::ParryHotter::alohomora(i32_3, i32_2, i32_1, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3667() {
    rusty_monitor::set_test_id(3667);
    let mut i64_0: i64 = -2673i64;
    let mut i64_1: i64 = 2968i64;
    let mut u16_0: u16 = 7367u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut i32_0: i32 = 14432i32;
    let mut i32_1: i32 = 14786i32;
    let mut i32_2: i32 = 5124i32;
    let mut i32_3: i32 = 17317i32;
    let mut i32_4: i32 = -9087i32;
    let mut i32_5: i32 = 8191i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_5, b: i32_4};
    let mut u64_0: u64 = 1903u64;
    let mut u64_1: u64 = 5036u64;
    let mut u64_2: u64 = 457u64;
    let mut u64_3: u64 = 1083u64;
    let mut usize_0: usize = 1168usize;
    let mut i32_6: i32 = -11823i32;
    let mut i32_7: i32 = 14196i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_7, b: i32_6};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut u16_1: u16 = 5304u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_1_ref_0, usize_0, u64_3, u64_2);
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    crate::hp::ParryHotter::alohomora(i32_3, i32_2, i32_1, i32_0);
    crate::hp::RomTiddle::foo3(romtiddle_0_ref_0, i64_1, i64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4287() {
    rusty_monitor::set_test_id(4287);
    let mut str_0: &str = "SBjiGef29SGXglid";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u16_0: u16 = 2327u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut u16_1: u16 = 3343u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut u16_2: u16 = 5763u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    panic!("From RustyUnit with love");
}
}