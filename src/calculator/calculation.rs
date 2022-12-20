//! The structs and data surrounding calculations.

/// Contains all the data in an operation. This struct outlines the
/// capabilities of the calculator, being able to perform an operation
/// only between two 32-bit integers at a time.
pub struct Calculation {
    /// This is the first term of a calculation.
    pub term_a: u32,

    /// This is the second term of a calculation.
    pub term_b: u32,

    /// This is the operation between the two terms.
    pub operation: Operation,
}

/// Outlines the four possible operations of the calculator.
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Operation::Add => write!(f, "+"),
            Operation::Subtract => write!(f, "-"),
            Operation::Multiply => write!(f, "*"),
            Operation::Divide => write!(f, "/"),
        }
    }
}

/// Convert a string (such as "+" or "/") to an [`Operation`].
pub fn to_operation(string: &str) -> Option<Operation> {
    match string {
        "+" => Some(Operation::Add),
        "-" => Some(Operation::Subtract),
        "*" => Some(Operation::Multiply),
        "/" => Some(Operation::Divide),
        _ => None,
    }
}
