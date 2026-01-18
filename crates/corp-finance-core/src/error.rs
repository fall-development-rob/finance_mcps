use thiserror::Error;

#[derive(Error, Debug)]
pub enum FinanceError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Calculation error: {0}")]
    CalculationError(String),

    #[error("Division by zero in {0}")]
    DivisionByZero(String),

    #[error("Negative value not allowed for {0}")]
    NegativeValue(String),

    #[error("Value out of range: {0}")]
    OutOfRange(String),

    #[error("Missing required field: {0}")]
    MissingField(String),
}

pub type Result<T> = std::result::Result<T, FinanceError>;
