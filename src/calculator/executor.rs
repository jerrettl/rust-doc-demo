//! Module for all execution of math operations.

use super::calculation::{Calculation, Operation};

/// Take the provided [`Calculation`] and create a 32-bit integer result by performing its
/// operation with its two terms. See [`Calculation`] for more details
/// on how this is structured.
pub fn execute_calculation(c: &Calculation) -> u32 {
    match c.operation {
        Operation::Add => c.term_a + c.term_b,
        Operation::Subtract => c.term_a - c.term_b,
        Operation::Multiply => c.term_a * c.term_b,
        Operation::Divide => c.term_a / c.term_b,
    }
}
