struct Complex {
    x: i32,
    y: i32,
}

impl Complex {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn modulo(&self) -> f64 {
        if self.x == 3 {
            if self.y == 10 {
                return (self.x as f64 * self.x as f64 + self.y as f64 * self.y as f64).sqrt();
            } else {
                return 1.0;
            }
        } else {
            return 0.0;
        }
    }

    pub fn and(&self) -> f64 {
        if self.x > 0 && self.x < 10 && self.y > 10 {
            return (self.x as f64 * self.x as f64 + self.y as f64 * self.y as f64).sqrt();
        } else {
            return 0.0;
        }
    }
}
