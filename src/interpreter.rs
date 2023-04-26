use std::cmp::Ordering;
use colored::Colorize;

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {}
    }

    pub fn interpret(&self, source: String) -> Result<String, String> {
        let result = match source.as_str() {
            "Hello" => String::from("Hello World!"),
            _ => return Err(String::from("Unknown command"))
        };
        Ok(result)
    }
}
