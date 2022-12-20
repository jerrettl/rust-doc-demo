//! Module for parsing input data into a valid [`Calculation`].

use std::str::FromStr;

use super::calculation::{to_operation, Calculation};

/// Given an input string, create an instance of [`Calculation`]. If the
/// string is invalid, return [`None`].
pub fn parse_string(input: &str) -> Option<Calculation> {
    // Split the string.
    let tokens: Vec<&str> = input.split_whitespace().collect();

    if tokens.len() != 3 {
        return None;
    }

    // Parse first term
    let term_a = match u32::from_str(tokens[0]) {
        Ok(term) => term,
        Err(_) => return None,
    };

    // Parse operation
    let operation = match to_operation(tokens[1]) {
        Some(operation) => operation,
        None => return None,
    };

    // Parse second term
    let term_b = match u32::from_str(tokens[2]) {
        Ok(term) => term,
        Err(_) => return None,
    };

    let calculation = Calculation {
        term_a,
        term_b,
        operation,
    };

    Some(calculation)
}
