use std::io;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() {
    loop {
        let num1 = input("Enter first number: ");
        let operation = input("Enter operation (+, -, *, /, ^, %): ");
        let num2 = input("Enter second number: ");

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
                println!("Invalid input for the second number.");
                continue;
            }
        };

        let Answer = if operation.trim() == "+" {
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
        } else {
            println!("Invalid operation");
            continue;
        };

        println!("Answer to {} {} {} is {}", num1, operation, num2, Answer);
        log(num1, &operation, num2, Answer);
    }
}

fn input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input
}

fn log(num1: f64, operation: &str, num2: f64, Answer: f64) {
    let log_entry = format!("{} {} {} = {}\n", num1, operation.trim(), num2, Answer);
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("history.txt")
        .expect("Unable to open file");
    file.write_all(log_entry.as_bytes()).expect("Unable to write data");
}
