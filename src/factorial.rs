pub fn factorial(num1: u64) -> u64 {
    match num1 {
        0 | 1 => 1,
        _ => num1 * factorial(num1 - 1),
    }
}