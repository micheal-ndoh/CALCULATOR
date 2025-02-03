pub struct Calculator {
pub  num1: f64,
pub num2: f64
}

impl Calculator {
    pub fn add(&self, num2: f64) -> f64 {
        self.num1 + num2
    }

    pub fn subtract(&self, num2: f64) -> f64 {
        self.num1 - num2
    }

    pub fn multiply(&self, num2: f64) -> f64 {
        self.num1 * num2
    }

    pub fn divide(&self, num2: f64) -> Result<f64, &'static str> {
        if num2 == 0.0 {
            Err("Cannot be divided by zero")
        } else {
            Ok(self.num1 / num2)
        }
    }

    pub fn power(&self, num2: f64) -> f64 {
        self.num1.powf(num2)
    }

    pub fn factorial(&self) -> u64 {
        let mut result = 1;
        let mut n = self.num1 as u64;
        while n > 1 {
            result *= n;
            n -= 1;
        }
        result
    }

    pub fn modulus(&self, num2: f64) -> f64 {
        self.num1 % num2
    }
}