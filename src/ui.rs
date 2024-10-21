use std::io::{self, Write};

use crate::error::CalculatorError;
use crate::operations::Operation;

pub struct UserInterface;

#[derive(Debug)]
pub enum MenuOption {
    TwoNumberOperation(Operation),
    MemoryOperation(Operation),
    ShowMemory,
    ResetMemory,
    EvaluateExpression,
    Exit,
}

impl UserInterface {
    pub fn new() -> Self {
        Self
    }

    pub fn display_menu(&self) {
        println!("\nОперації:");
        println!("1. Додавання двох чисел");
        println!("2. Віднімання двох чисел");
        println!("3. Множення двох чисел");
        println!("4. Ділення двох чисел");
        println!("5. Додавання до числа в пам'яті");
        println!("6. Віднімання від числа в пам'яті");
        println!("7. Множення на число в пам'яті");
        println!("8. Ділення на число в пам'яті");
        println!("9. Відобразити пам'ять");
        println!("10. Скинути пам'ять");
        println!("11. Оцінити вираз у польській нотації");
        println!("0. Вийти");
    }

    pub fn get_menu_choice(&self) -> Result<MenuOption, CalculatorError> {
        let mut input = String::new();
        print!("Виберіть опцію: ");
        io::stdout().flush().map_err(|e| CalculatorError::IOError(e.to_string()))?;
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| CalculatorError::IOError(e.to_string()))?;

        match input.trim() {
            "0" => Ok(MenuOption::Exit),
            "1" => Ok(MenuOption::TwoNumberOperation(Operation::Add)),
            "2" => Ok(MenuOption::TwoNumberOperation(Operation::Subtract)),
            "3" => Ok(MenuOption::TwoNumberOperation(Operation::Multiply)),
            "4" => Ok(MenuOption::TwoNumberOperation(Operation::Divide)),
            "5" => Ok(MenuOption::MemoryOperation(Operation::Add)),
            "6" => Ok(MenuOption::MemoryOperation(Operation::Subtract)),
            "7" => Ok(MenuOption::MemoryOperation(Operation::Multiply)),
            "8" => Ok(MenuOption::MemoryOperation(Operation::Divide)),
            "9" => Ok(MenuOption::ShowMemory),
            "10" => Ok(MenuOption::ResetMemory),
            "11" => Ok(MenuOption::EvaluateExpression),
            _ => Err(CalculatorError::InvalidInput),
        }
    }

    pub fn read_one_number(&self) -> Result<f64, CalculatorError> {
        print!("Введіть число: ");
        io::stdout().flush().map_err(|e| CalculatorError::IOError(e.to_string()))?;
        
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| CalculatorError::IOError(e.to_string()))?;
        
        input
            .trim()
            .parse()
            .map_err(|_| CalculatorError::InvalidNumber(input.trim().to_string()))
    }

    pub fn read_two_numbers(&self) -> Result<(f64, f64), CalculatorError> {
        print!("Введіть два числа (через пробіл): ");
        io::stdout().flush().map_err(|e| CalculatorError::IOError(e.to_string()))?;
        
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| CalculatorError::IOError(e.to_string()))?;
        
        let numbers: Vec<&str> = input.trim().split_whitespace().collect();
        if numbers.len() != 2 {
            return Err(CalculatorError::InvalidInput);
        }

        let a = numbers[0]
            .parse()
            .map_err(|_| CalculatorError::InvalidNumber(numbers[0].to_string()))?;
        let b = numbers[1]
            .parse()
            .map_err(|_| CalculatorError::InvalidNumber(numbers[1].to_string()))?;

        Ok((a, b))
    }

    pub fn read_expression(&self) -> Result<String, CalculatorError> {
        print!("Введіть вираз у польській нотації: ");
        io::stdout().flush().map_err(|e| CalculatorError::IOError(e.to_string()))?;
        
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| CalculatorError::IOError(e.to_string()))?;
        
        Ok(input.trim().to_string())
    }
}