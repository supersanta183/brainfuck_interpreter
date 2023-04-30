pub mod interpreter;
pub mod ascii_convert;

use std::{env, cmp::Ordering};
use colored::Colorize;

use crate::interpreter::BrainfuckInterpreter;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if the program was called with the correct number of arguments, stop if not
    match args.len().cmp(&2) {
        Ordering::Less => {
            println!("{} Cargo run was called with no arguments", "Error:".red());
            return;
        },
        Ordering::Equal => (),
        Ordering::Greater => {
            println!("{} Cargo run was called with too many arguments", "Error:".red());
            return;
        }
    }

    // The first argument is the name of the program
    let program_name = &args[0];
    println!("Program name: {}", program_name);

    // The second argument is the parameter
    let parameter = String::from(&args[1]);

    let mut interpreter = BrainfuckInterpreter::new();
    match interpreter.interpret(parameter) {
        Ok(result) => println!("Output: {}", result),
        Err(error) => println!("Error {}", error)
    };
}
