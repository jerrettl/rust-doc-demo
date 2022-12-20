//! A small demonstration application in Rust that shows how to write
//! documentation.
//! 
//! This application is a very simple calculator with the ability to add,
//! subtract, multiply, or divide two integers, creating an integer result.

use std::io;

pub mod calculator;

use calculator::{executor, parser};

pub fn main() {
    print_introduction();

    let input = get_input();

    match input {
        Some(string) => calculate(string.as_str()),
        None => print_error(),
    }
}

/// Prints the introduction to the program, explaining how it is used
/// and its limitations.
pub fn print_introduction() {
    println!("Welcome to the calculator! Time to calculate!");
    println!("This calculator is very simple. It can only perform one operation at a time.");
    println!("Some valid input examples:");
    println!("   3 + 5");
    println!("   1234 / 4");
    println!("   900 * 32");
    println!("   9999999 - 10000000");
    println!();
}

/// Create a prompt for the user in the terminal and save their input into
/// a [`String`] with error checking.
pub fn get_input() -> Option<String> {
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => Some(input),
        Err(_) => None,
    }
}

/// Convert a string into a u32 result and print it. Prints an error in the case of failure.
pub fn calculate(input: &str) {
    match parser::parse_string(input) {
        Some(calculation) => {
            let result = executor::execute_calculation(&calculation);
            println!("The answer is here!");
            println!(
                "{} {} {} = {}",
                calculation.term_a, calculation.operation, calculation.term_b, result
            );
        }
        None => print_error(),
    }
}

/// Print a generic error.
pub fn print_error() {
    println!("Oh no! It looks like I can't figure out the answer to that one. Try again!");
}
