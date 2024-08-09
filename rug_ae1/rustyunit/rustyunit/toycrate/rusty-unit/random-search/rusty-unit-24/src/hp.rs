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
fn rusty_test_1364() {
    rusty_monitor::set_test_id(1364);
    let mut i32_0: i32 = -12261i32;
    let mut i32_1: i32 = -3856i32;
    let mut i32_2: i32 = -549i32;
    let mut i32_3: i32 = -14294i32;
    let mut i32_4: i32 = 6595i32;
    let mut i32_5: i32 = -2414i32;
    let mut i32_6: i32 = -8989i32;
    let mut i32_7: i32 = 3275i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_7, b: i32_6};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_8: i32 = 8342i32;
    let mut i32_9: i32 = -19825i32;
    let mut u64_0: u64 = 2003u64;
    let mut u64_1: u64 = 1283u64;
    let mut i32_10: i32 = -8720i32;
    let mut i32_11: i32 = -5521i32;
    let mut i64_0: i64 = -373i64;
    let mut i64_1: i64 = -8321i64;
    let mut u16_0: u16 = 3239u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_1};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_0);
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_11, b: i32_10};
    crate::hp::ParryHotter::another_number_fn(u64_1, u64_0);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    crate::hp::ParryHotter::foo2(parryhotter_1_ref_0, i32_9, i32_8);
    let mut i32_12: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_5, i32_4);
    crate::hp::ParryHotter::alohomora(i32_3, i32_2, i32_1, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2184() {
    rusty_monitor::set_test_id(2184);
    let mut u16_0: u16 = 8585u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut usize_0: usize = 662usize;
    let mut i32_0: i32 = 9012i32;
    let mut i32_1: i32 = 3494i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u16_1: u16 = 8572u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut usize_1: usize = 8803usize;
    let mut i32_2: i32 = 1788i32;
    let mut i32_3: i32 = 13191i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut i64_0: i64 = -1142i64;
    let mut i64_1: i64 = 7485i64;
    let mut u16_2: u16 = 1232u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut i32_4: i32 = -2580i32;
    let mut i32_5: i32 = -991i32;
    let mut i32_6: i32 = -12304i32;
    let mut i32_7: i32 = 18305i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_7, i32_6);
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut u16_3: u16 = 4580u16;
    let mut i32_8: i32 = -2486i32;
    let mut i32_9: i32 = -3091i32;
    let mut i32_10: i32 = 6468i32;
    let mut i32_11: i32 = 5961i32;
    let mut u16_4: u16 = 1275u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut i32_12: i32 = -1734i32;
    let mut i32_13: i32 = 3138i32;
    let mut i32_14: i32 = -4498i32;
    let mut i32_15: i32 = 25114i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_15, b: i32_14};
    let mut parryhotter_3_ref_0: &crate::hp::ParryHotter = &mut parryhotter_3;
    let mut u16_5: u16 = 7388u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    let mut usize_2: usize = 6123usize;
    let mut i32_16: i32 = 1624i32;
    let mut i32_17: i32 = 2169i32;
    let mut parryhotter_4: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_17, i32_16);
    let mut parryhotter_4_ref_0: &crate::hp::ParryHotter = &mut parryhotter_4;
    let mut str_0: &str = "Ep4q";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_2: i64 = -9873i64;
    let mut u16_6: u16 = 7932u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    let mut romtiddle_5_ref_0: &crate::hp::RomTiddle = &mut romtiddle_5;
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_5_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_3, y: i64_2};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    let mut u16_7: u16 = 0u16;
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_7};
    let mut romtiddle_6_ref_0: &crate::hp::RomTiddle = &mut romtiddle_6;
    let mut string_4: std::string::String = crate::hp::RomTiddle::name(romtiddle_6_ref_0);
    let mut u16_8: u16 = 4982u16;
    let mut romtiddle_7: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_8};
    let mut romtiddle_7_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_7;
    crate::hp::RomTiddle::avada_kedavra(romtiddle_7_ref_0, string_4);
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
    let mut i32_18: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_4_ref_0, usize_2, string_2);
    crate::hp::ParryHotter::foo2(parryhotter_3_ref_0, i32_13, i32_12);
    let mut string_5: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    crate::hp::ParryHotter::alohomora(i32_11, i32_10, i32_9, i32_8);
    let mut romtiddle_8: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut i32_19: i32 = crate::hp::ParryHotter::accio(parryhotter_2_ref_0, i32_5, i32_4);
    crate::hp::RomTiddle::foo3(romtiddle_2_ref_0, i64_1, i64_0);
    let mut i32_20: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_1_ref_0, usize_1, string_1);
    let mut i32_21: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_0_ref_0, usize_0, string_0);
    panic!("From RustyUnit with love");
}
}