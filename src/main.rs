use std::io;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    loop {
        let num1 = input("Enter first number: ");
        let operation = input("Enter operation (+, -, *, /, ^, %, !): ");
        let num2 = if operation.trim() != "!" {
            input("Enter second number: ")
        } else {
            "0".to_string()
        };

        let num1: f64 = match num1.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid input for the first number.");
                return;
            }
        };

        let num2: f64 = match num2.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid input for the second number.");
                return;
            }
        };

        let answer = if operation.trim() == "+" {
            num1 + num2
        } else if operation.trim() == "-" {
            num1 - num2
        } else if operation.trim() == "*" {
            num1 * num2
        } else if operation.trim() == "/" {
            if num2 == 0.0 {
                println!("number cannot be divided by zero");
                return;
            } else {
                num1 / num2
            }
        } else if operation.trim() == "^" {
            num1.powf(num2)
        } else if operation.trim() == "%" {
            num1 % num2
        } else if operation.trim() == "!" {
            factorial(num1 as u64) as f64
        } else {
            println!("Invalid operation");
            return;
        };

        println!("The answer to {} {} {} is {}", num1, operation, num2, answer);
        log(num1, &operation, num2, answer);
    }
}

fn input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input
}

fn log(num1: f64, operation: &str, num2: f64, answer: f64) {
    let log_entry = format!("{} {} {} = {}\n", num1, operation, num2, answer);
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("history.txt")
        .expect("Cannot open file");
    file.write_all(log_entry.as_bytes()).expect("Cannot write to file");
}

fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}