use std::{io, ops::Add};
mod add;
mod subtract;
mod multiply;
mod divide;
mod power;
mod factorial;
mod modulus;
use add::add;
use subtract::subtract;
use divide::divide;
use factorial::factorial;
use multiply::multiply;
use power::power;
use modulus::modulus;
mod calculator;
use calculator::Calculator;

fn input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input
}

fn main() {
    let num1 = input("Enter the first number: ");
    let num2 = input("Enter the second number: ");
    let operator = input("Enter the operator (+ - * / ^ ! %): ");

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

    let calculator = Calculator { num1, num2 };

    let answer = if operator.trim() == "+" {
        calculator.add(num2)
    } else if operator.trim() == "-" {
        calculator.subtract(num2)
    } else if operator.trim() == "*" {
        calculator.multiply(num2)
    } else if operator.trim() == "/" {
        match calculator.divide(num2) {
            Ok(result) => result,
            Err(e) => {
                println!("{}", e);
                return; 
            }
        }
    } else if operator.trim() == "^" {
        calculator.power(num2)
    } else if operator.trim() == "%" {
        calculator.modulus(num2)
    } else if operator.trim() == "!" {
        calculator.factorial() as f64
    } else {
        println!("Invalid operator, enter a (+ - * / ^ ! %)");
        return;
    };

    println!("The answer to {} {} {} is: {}", num1, operator, num2, answer);
}