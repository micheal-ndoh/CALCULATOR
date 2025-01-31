use std::io;
use std::fs::OpenOptions;
use std::io::prelude::*;

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
                continue;
            }
        };

        let num2: f64 = match num2.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid on the second number.");
                continue;
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
                continue;
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
            continue;
        };

        println!("Answer to {} {} {} is {}", num1, operation, num2, answer);
        log(num1, &operation, num2, answer);
    }
}

fn input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    input
}

fn log(num1: f64, operation: &str, num2: f64, answer: f64) {
    let log_entry = format!("{} {} {} = {}", num1, operation, num2, answer);
    let mut file = OpenOptions::new()
        .append(true)
        .create_new(true)
        .open("history.txt")
        .expect("Cannot open file");
    file.write_all(log_entry.as_bytes()).expect("Cannot write to file");
}

fn factorial(num1: u64) -> u64 {
    if num1 == 0 {
        1
    } else {
        num1 * factorial(num1 - 1)
    }
}