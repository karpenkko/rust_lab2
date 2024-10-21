mod memory;
pub use memory::StandardMemory;

use crate::error::CalculatorError;
use crate::operations::Operation;

pub struct Calculator {
    pub memory: StandardMemory,
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            memory: StandardMemory::new(),
        }
    }

    pub fn execute_two_number_operation(
        &mut self,
        operation: Operation,
        a: f64,
        b: f64,
    ) -> Result<f64, CalculatorError> {
        let result = match operation {
            Operation::Add => Ok(a + b),
            Operation::Subtract => Ok(a - b),
            Operation::Multiply => Ok(a * b),
            Operation::Divide => {
                if b == 0.0 {
                    Err(CalculatorError::DivisionByZero)
                } else {
                    Ok(a / b)
                }
            }
        }?;
        
        self.memory.set(result);
        Ok(result)
    }

    pub fn execute_memory_operation(
        &mut self,
        operation: Operation,
        value: f64,
    ) -> Result<f64, CalculatorError> {
        let memory_value = self.memory.get();
        let result = match operation {
            Operation::Add => Ok(memory_value + value),
            Operation::Subtract => Ok(memory_value - value),
            Operation::Multiply => Ok(memory_value * value),
            Operation::Divide => {
                if value == 0.0 {
                    Err(CalculatorError::DivisionByZero)
                } else {
                    Ok(memory_value / value)
                }
            }
        }?;
        
        self.memory.set(result);
        Ok(result)
    }

    pub fn evaluate_expression(&mut self, expression: &str) -> Result<f64, CalculatorError> {
        let result = self.evaluate_prefix_expression(expression)?;
        self.memory.set(result);
        Ok(result)
    }

    fn evaluate_prefix_expression(&self, expression: &str) -> Result<f64, CalculatorError> {
        let tokens: Vec<&str> = expression.split_whitespace().collect();
        let mut stack: Vec<f64> = Vec::new();

        for token in tokens.iter().rev() {
            match *token {
                "+" | "-" | "*" | "/" => {
                    if stack.len() < 2 {
                        return Err(CalculatorError::InvalidExpression);
                    }
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    let result = match *token {
                        "+" => Ok(a + b),
                        "-" => Ok(a - b),
                        "*" => Ok(a * b),
                        "/" => {
                            if b == 0.0 {
                                Err(CalculatorError::DivisionByZero)
                            } else {
                                Ok(a / b)
                            }
                        }
                        _ => unreachable!(),
                    }?;
                    stack.push(result);
                }
                _ => {
                    let number = token
                        .parse()
                        .map_err(|_| CalculatorError::InvalidNumber(token.to_string()))?;
                    stack.push(number);
                }
            }
        }

        if stack.len() != 1 {
            return Err(CalculatorError::InvalidExpression);
        }

        Ok(stack.pop().unwrap())
    }
}