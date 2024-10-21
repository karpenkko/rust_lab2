mod calculator;
mod error;
mod operations;
mod ui;

use calculator::Calculator;
use error::CalculatorError;
use ui::{MenuOption, UserInterface};

fn main() {
    let mut calculator = Calculator::new();
    let ui = UserInterface::new();

    loop {
        ui.display_menu();
        
        match ui.get_menu_choice() {
            Ok(choice) => {
                match handle_menu_option(&mut calculator, &ui, choice) {
                    Ok(_) => (),
                    Err(e) => println!("Помилка: {}", e),
                }
            }
            Err(e) => println!("Помилка: {}", e),
        }
    }
}

fn handle_menu_option(
    calculator: &mut Calculator,
    ui: &UserInterface,
    choice: MenuOption,
) -> Result<(), CalculatorError> {
    match choice {
        MenuOption::Exit => std::process::exit(0),
        MenuOption::TwoNumberOperation(operation) => {
            let (a, b) = ui.read_two_numbers()?;
            let result = calculator.execute_two_number_operation(operation, a, b)?;
            println!("Результат: {}", result);
        }
        MenuOption::MemoryOperation(operation) => {
            let number = ui.read_one_number()?;
            let result = calculator.execute_memory_operation(operation, number)?;
            println!("Результат: {}", result);
        }
        MenuOption::ShowMemory => {
            println!("Пам'ять: {}", calculator.memory.get());
        }
        MenuOption::ResetMemory => {
            calculator.memory.reset();
            println!("Пам'ять скинута.");
        }
        MenuOption::EvaluateExpression => {
            let expression = ui.read_expression()?;
            let result = calculator.evaluate_expression(&expression)?;
            println!("Результат: {}", result);
        }
    }
    Ok(())
}