use std::fmt;

#[derive(Debug)]
pub enum CalculatorError {
    InvalidInput,
    InvalidNumber(String),
    InvalidExpression,
    DivisionByZero,
    IOError(String),
}

impl std::error::Error for CalculatorError {}

impl fmt::Display for CalculatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalculatorError::InvalidInput => write!(f, "Неправильне введення"),
            CalculatorError::InvalidNumber(n) => write!(f, "Неправильне число: {}", n),
            CalculatorError::InvalidExpression => write!(f, "Неправильний вираз"),
            CalculatorError::DivisionByZero => write!(f, "Ділення на нуль"),
            CalculatorError::IOError(e) => write!(f, "Помилка введення/виведення: {}", e),
        }
    }
}