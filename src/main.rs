mod operations;

use std::io;
use operations::*;

fn main() {
    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut op = String::new();

    println!("Enter first number: ");
    io::stdin().read_line(&mut num1).expect("Invalid input");
    let num1: f64 = num1.trim().parse().expect("Invalid number");

    println!("Enter operator (+, -, *, /): ");
    io::stdin().read_line(&mut op).expect("Invalid input");
    let op = op.trim();

    println!("Enter second number: ");
    io::stdin().read_line(&mut num2).expect("Invalid input");
    let num2: f64 = num2.trim().parse().expect("Invalid number");

    let result = match op {
        "+" => add(num1, num2),
        "-" => subtract(num1, num2),
        "*" => multiply(num1, num2),
        "/" => divide(num1, num2),
        _ => {
            println!("Unsupported operator!");
            std::process::exit(1);
        }
    };

    println!("Result: {}", result);
}