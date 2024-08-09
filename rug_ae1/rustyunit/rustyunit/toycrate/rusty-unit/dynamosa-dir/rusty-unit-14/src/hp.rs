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
fn rusty_test_632() {
    rusty_monitor::set_test_id(632);
    let mut u64_0: u64 = 4460u64;
    let mut u64_1: u64 = 228u64;
    let mut usize_0: usize = 266usize;
    let mut i32_0: i32 = 12848i32;
    let mut i32_1: i32 = 1379i32;
    let mut u16_0: u16 = 5694u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut i32_2: i32 = -9085i32;
    let mut i32_3: i32 = -12307i32;
    let mut i32_4: i32 = 2926i32;
    let mut i32_5: i32 = 15233i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_5, i32_4);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_6: i32 = -9859i32;
    let mut i32_7: i32 = 260i32;
    let mut i64_0: i64 = -11642i64;
    let mut i64_1: i64 = 5656i64;
    let mut u16_1: u16 = 7277u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_1};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_0);
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_7, i32_6);
    crate::hp::ParryHotter::foo2(parryhotter_0_ref_0, i32_3, i32_2);
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_1, b: i32_0};
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut bool_0: bool = crate::hp::ParryHotter::aqua_eructo(parryhotter_2_ref_0, usize_0, u64_1, u64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1747() {
    rusty_monitor::set_test_id(1747);
    let mut i32_0: i32 = 9947i32;
    let mut i32_1: i32 = 8891i32;
    let mut i64_0: i64 = -8425i64;
    let mut i64_1: i64 = 1799i64;
    let mut u16_0: u16 = 3764u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_1};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut i32_2: i32 = -8007i32;
    let mut i32_3: i32 = 735i32;
    let mut i32_4: i32 = -799i32;
    let mut i32_5: i32 = -4408i32;
    let mut u16_1: u16 = 7846u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut u16_2: u16 = 101u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_2_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_2;
    crate::hp::RomTiddle::avada_kedavra(romtiddle_2_ref_0, string_1);
    crate::hp::ParryHotter::alohomora(i32_5, i32_4, i32_3, i32_2);
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_0);
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_1, b: i32_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_316() {
    rusty_monitor::set_test_id(316);
    let mut i64_0: i64 = -19220i64;
    let mut i64_1: i64 = 7124i64;
    let mut u16_0: u16 = 6405u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut usize_0: usize = 6618usize;
    let mut i32_0: i32 = 651i32;
    let mut i32_1: i32 = -5469i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_1, b: i32_0};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_2: i32 = -10496i32;
    let mut i32_3: i32 = 6408i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_3, b: i32_2};
    let mut u16_1: u16 = 6124u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut i32_4: i32 = crate::hp::ParryHotter::aguamenti(parryhotter_0_ref_0, usize_0, string_0);
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    crate::hp::RomTiddle::foo3(romtiddle_1_ref_0, i64_1, i64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4191() {
    rusty_monitor::set_test_id(4191);
    let mut u16_0: u16 = 7691u16;
    let mut i64_0: i64 = -7791i64;
    let mut u16_1: u16 = 5650u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut str_0: &str = "43AX3dLlVJ3rf";
    let mut u16_2: u16 = 2u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut u16_3: u16 = 3768u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut i32_0: i32 = -9584i32;
    let mut i32_1: i32 = 8280i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_1, i32_0);
    let mut u16_4: u16 = 304u16;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_3_ref_0: &crate::hp::RomTiddle = &mut romtiddle_3;
    let mut u16_5: u16 = 5147u16;
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_5};
    let mut romtiddle_4_ref_0: &crate::hp::RomTiddle = &mut romtiddle_4;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_4_ref_0);
    let mut u16_6: u16 = 9880u16;
    let mut u64_0: u64 = 1025u64;
    let mut u64_1: u64 = 601u64;
    let mut i32_2: i32 = -12708i32;
    let mut i32_3: i32 = 15429i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut u16_7: u16 = 6906u16;
    let mut romtiddle_5: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_7};
    let mut romtiddle_5_ref_0: &crate::hp::RomTiddle = &mut romtiddle_5;
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_5_ref_0);
    let mut i32_4: i32 = 155i32;
    let mut i32_5: i32 = -16332i32;
    let mut i32_6: i32 = 425i32;
    let mut i32_7: i32 = 2999i32;
    let mut i32_8: i32 = -13869i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_8, i32_7);
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i64_1: i64 = 3572i64;
    let mut i64_2: i64 = -2786i64;
    let mut u16_8: u16 = 7293u16;
    let mut romtiddle_6: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_8};
    let mut romtiddle_6_ref_0: &crate::hp::RomTiddle = &mut romtiddle_6;
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_6_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_3, y: i64_2};
    let mut string_3: std::string::String = crate::hp::RomTiddle::name(romtiddle_3_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_1};
    let mut wonreasley_1_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_1;
    let mut i32_9: i32 = 7817i32;
    let mut i32_10: i32 = 2837i32;
    let mut i32_11: i32 = 20061i32;
    let mut i32_12: i32 = 22646i32;
    let mut i32_13: i32 = 11507i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_11, b: i32_10};
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut u16_9: u16 = 290u16;
    let mut romtiddle_7: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_6};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_4: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_3: i64 = -10158i64;
    let mut u16_10: u16 = 9684u16;
    let mut romtiddle_8: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_9};
    let mut romtiddle_7_ref_0: &crate::hp::RomTiddle = &mut romtiddle_7;
    let mut string_5: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut wonreasley_2: crate::hp::WonReasley = crate::hp::WonReasley {x: string_2, y: i64_3};
    let mut wonreasley_0_ref_0: &crate::hp::WonReasley = &mut wonreasley_0;
    let mut i64_4: i64 = -3156i64;
    let mut i64_5: i64 = 6418i64;
    let mut romtiddle_9: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_10};
    let mut romtiddle_8_ref_0: &crate::hp::RomTiddle = &mut romtiddle_8;
    crate::hp::WonReasley::arania_exumai(wonreasley_0_ref_0, str_0_ref_0);
    let mut i32_14: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_4, i32_12);
    crate::hp::ParryHotter::another_number_fn(u64_0, u64_1);
    crate::hp::WonReasley::ascendio(wonreasley_1_ref_0, i64_4);
    crate::hp::ParryHotter::foo2(parryhotter_2_ref_0, i32_6, i32_9);
    let mut parryhotter_4: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_5, i32_13);
    let mut wonreasley_3: crate::hp::WonReasley = crate::hp::WonReasley {x: string_4, y: i64_5};
    let mut wonreasley_2_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_2;
    let mut romtiddle_10: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_9_ref_0: &mut crate::hp::RomTiddle = &mut romtiddle_9;
    panic!("From RustyUnit with love");
}
}