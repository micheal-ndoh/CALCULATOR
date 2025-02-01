pub fn divide(num1: f64, num2: f64) -> Result<f64, &'static str> {
    if num2 == 0.0 {
        Err(" Cannot  be dividedby zero")
    } else {
        Ok(num1 / num2)
    }
}