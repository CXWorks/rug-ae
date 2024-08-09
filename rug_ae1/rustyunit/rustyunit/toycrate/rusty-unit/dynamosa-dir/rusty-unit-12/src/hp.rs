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
fn rusty_test_206() {
    rusty_monitor::set_test_id(206);
    let mut u16_0: u16 = 5992u16;
    let mut u16_1: u16 = 9577u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_1};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut u16_2: u16 = 1801u16;
    let mut i64_0: i64 = 6610i64;
    let mut u16_3: u16 = 4426u16;
    let mut romtiddle_1: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_3};
    let mut romtiddle_1_ref_0: &crate::hp::RomTiddle = &mut romtiddle_1;
    let mut string_0: std::string::String = crate::hp::RomTiddle::name(romtiddle_1_ref_0);
    let mut wonreasley_0: crate::hp::WonReasley = crate::hp::WonReasley {x: string_0, y: i64_0};
    let mut wonreasley_0_ref_0: &mut crate::hp::WonReasley = &mut wonreasley_0;
    let mut str_0: &str = "iGz0zNZ3qsFx";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_1: i64 = -10461i64;
    let mut u16_4: u16 = 8858u16;
    let mut romtiddle_2: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_4};
    let mut romtiddle_2_ref_0: &crate::hp::RomTiddle = &mut romtiddle_2;
    let mut string_1: std::string::String = crate::hp::RomTiddle::name(romtiddle_2_ref_0);
    let mut wonreasley_1: crate::hp::WonReasley = crate::hp::WonReasley {x: string_1, y: i64_1};
    let mut wonreasley_1_ref_0: &crate::hp::WonReasley = &mut wonreasley_1;
    let mut i64_2: i64 = 1961i64;
    let mut romtiddle_3: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_2};
    let mut string_2: std::string::String = crate::hp::RomTiddle::name(romtiddle_0_ref_0);
    let mut romtiddle_4: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    crate::hp::WonReasley::ascendio(wonreasley_0_ref_0, i64_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_751() {
    rusty_monitor::set_test_id(751);
    let mut i32_0: i32 = -12482i32;
    let mut i32_1: i32 = -14047i32;
    let mut i32_2: i32 = -11871i32;
    let mut i32_3: i32 = -17532i32;
    let mut parryhotter_0: crate::hp::ParryHotter = crate::hp::ParryHotter::new(i32_3, i32_2);
    let mut parryhotter_0_ref_0: &crate::hp::ParryHotter = &mut parryhotter_0;
    let mut i32_4: i32 = 7327i32;
    let mut i32_5: i32 = 16846i32;
    let mut i32_6: i32 = 15924i32;
    let mut parryhotter_1: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_6, b: i32_5};
    let mut parryhotter_1_ref_0: &crate::hp::ParryHotter = &mut parryhotter_1;
    let mut u16_0: u16 = 4422u16;
    let mut romtiddle_0: crate::hp::RomTiddle = crate::hp::RomTiddle {horcrux: u16_0};
    let mut romtiddle_0_ref_0: &crate::hp::RomTiddle = &mut romtiddle_0;
    let mut i32_7: i32 = -4979i32;
    let mut i32_8: i32 = 1211i32;
    let mut parryhotter_2: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_8, b: i32_7};
    let mut parryhotter_2_ref_0: &crate::hp::ParryHotter = &mut parryhotter_2;
    let mut i32_9: i32 = 16746i32;
    let mut parryhotter_3: crate::hp::ParryHotter = crate::hp::ParryHotter {a: i32_9, b: i32_0};
    let mut i32_10: i32 = crate::hp::ParryHotter::accio(parryhotter_0_ref_0, i32_1, i32_4);
    panic!("From RustyUnit with love");
}
}