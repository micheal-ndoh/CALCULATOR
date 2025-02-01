use std::io;
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

fn input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn main() {
    let num1 = input("Enter first number: ");
    let operator = input("Enter operator (+, -, *, /, ^, %, !): ");
    let num2 = if operator.trim() != "!" {
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

    let answer = if operator.trim() == "+" {
        add::add(num1, num2)
    } else if operator.trim() == "-" {
        subtract::subtract(num1, num2)
    } else if operator.trim() == "*" {
        multiply::multiply(num1, num2)
    } else if operator.trim() == "/" {
        match divide::divide(num1, num2) {
            Ok(result) => result,
            Err(e) => {
                println!("{}", e);
                return;
            }
        }
    } else if operator.trim() == "^" {
        power::power(num1, num2)
    } else if operator.trim() == "%" {
        modulus::modulus(num1, num2)
    } else if operator.trim() == "!" {
        factorial::factorial(num1 as u64) as f64
    } else {
        println!("Invalid operatorc, enter a (+ - * / ^ ! % ");
        return;
    };

    println!("The answer to {} {} {} is: {}", num1, operator, num2, answer);
}