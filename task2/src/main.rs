use std::io;

// Define the Operation enum
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

// Write the calculate function signature
fn calculate(op: Operation) -> f64 {
    // Implement the calculate function using pattern matching
    match op {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => {
            if b != 0.0 {
                a / b
            } else {
                println!("Error: Division by zero!");
                std::f64::NAN
            }
        }
    }
}

fn main() {
    println!("Simple Calculator");
    println!("Enter the first number:");
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("Failed to read line");
    let num1: f64 = num1.trim().parse().expect("Please enter a number");

    println!("Enter the operation (+, -, *, /):");
    let mut operation = String::new();
    io::stdin().read_line(&mut operation).expect("Failed to read line");
    let operation = operation.trim();

    println!("Enter the second number:");
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).expect("Failed to read line");
    let num2: f64 = num2.trim().parse().expect("Please enter a number");

    // Create an Operation enum instance with the parsed input values
    let op = match operation {
        "+" => Operation::Add(num1, num2),
        "-" => Operation::Subtract(num1, num2),
        "*" => Operation::Multiply(num1, num2),
        "/" => Operation::Divide(num1, num2),
        _ => {
            println!("Invalid operation");
            return;
        }
    };

    // Call the calculate function with the created Operation enum instance
    let result = calculate(op);

    // Print the result to the console
    println!("Result: {}", result);
}
