use ascii_converter::*;

use crate::ascii_convert::AsciiConverter;
use std::thread;

pub struct BrainfuckInterpreter {
    byte_array: [u8; 30000],
    byte_array_position: usize,
}

impl BrainfuckInterpreter {
    //initializes a new interpreter
    pub fn new() -> BrainfuckInterpreter {
        BrainfuckInterpreter {
            byte_array: [0; 30000],
            byte_array_position: 0,
        }
    }

    pub fn get_byte_array(&self) -> [u8; 30000] {
        self.byte_array
    }

    //+[-->-[>>+>-----<<]<--<---]>-.>>>+.>>..+++[.>]<<<<.+++.------.<<-.>>>>+. gives wrong output

    //function that interprets a string and returns a result. OK if the string is valid brainfuck and Err otherwise.
    pub fn interpret(&mut self, source: &String) -> Result<String, String> {
        let mut output_array: Vec<u8> = Vec::new();
        let mut loop_array: Vec<usize> = Vec::new();
        let mut i = 0;
        // loop through source chars and interpret
        while i < source.len() {
            match source.chars().nth(i).unwrap() {
                '+' => {
                    if self.byte_array[self.byte_array_position] == 255 {
                        self.byte_array[self.byte_array_position] = 0;
                    } else {
                        self.byte_array[self.byte_array_position] += 1;
                    }
                }
                '-' => {
                    if self.byte_array[self.byte_array_position] == 0 {
                        self.byte_array[self.byte_array_position] = 255;
                    } else {
                        self.byte_array[self.byte_array_position] -= 1
                    }
                }
                '>' => self.byte_array_position += 1,
                '<' => self.byte_array_position -= 1,
                '[' => {
                    loop_array.push(i);
                }
                ']' => {
                    if self.byte_array[self.byte_array_position] > 0 {
                        i = loop_array.pop().unwrap();
                        continue;
                    }
                    loop_array.pop();
                }
                '.' => output_array.push(self.byte_array[self.byte_array_position]),
                _ => (),
            }
            i += 1;
        }

        //convert output array to ascii string with the ascii converter crate
        let result = AsciiConverter::convert(&output_array);
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn first_position_should_be_1() {
        let mut interpreter = BrainfuckInterpreter::new();
        interpreter.interpret(&String::from("+")).unwrap();

        assert_eq!(interpreter.get_byte_array()[0], 1);
    }

    #[test]
    fn first_position_should_be_2() {
        let mut interpreter = BrainfuckInterpreter::new();
        interpreter.interpret(&String::from("++")).unwrap();

        assert_eq!(interpreter.get_byte_array()[0], 2);
    }

    #[test]
    fn second_position_should_be_1() {
        let mut interpreter = BrainfuckInterpreter::new();
        interpreter.interpret(&String::from(">+")).unwrap();

        assert_eq!(interpreter.get_byte_array()[1], 1);
    }

    #[test]
    fn first_position_should_be_1_after_moving_back_and_forth() {
        let mut interpreter = BrainfuckInterpreter::new();
        interpreter.interpret(&String::from("><+")).unwrap();

        assert_eq!(interpreter.get_byte_array()[0], 1);
    }

    #[test]
    fn first_position_should_be_0_after_minus() {
        let mut interpreter = BrainfuckInterpreter::new();
        interpreter.interpret(&String::from("+-")).unwrap();

        assert_eq!(interpreter.get_byte_array()[0], 0);
    }

    #[test]
    fn second_position_should_be_2_after_looping() {
        let mut interpreter = BrainfuckInterpreter::new();
        interpreter.interpret(&String::from("++[>+<-]")).unwrap();

        assert_eq!(interpreter.get_byte_array()[1], 2);
    }

    #[test]
    fn third_position_should_be_1_after_looping_and_moving() {
        let mut interpreter = BrainfuckInterpreter::new();
        interpreter.interpret(&String::from("++[>+<-]>>+")).unwrap();

        assert_eq!(interpreter.get_byte_array()[0], 0);
        assert_eq!(interpreter.get_byte_array()[1], 2);
        assert_eq!(interpreter.get_byte_array()[2], 1);
    }

    #[test]
    fn second_position_should_be_4_with_nested_loops() {
        let mut interpreter = BrainfuckInterpreter::new();
        interpreter
            .interpret(&String::from("+[>++[>+<-]<-]"))
            .unwrap();

        assert_eq!(interpreter.get_byte_array()[0], 0);
        assert_eq!(interpreter.get_byte_array()[1], 0);
        assert_eq!(interpreter.get_byte_array()[2], 2);
    }

    #[test]
    fn test() {
        let mut interpreter = BrainfuckInterpreter::new();
        interpreter
            .interpret(&String::from("+[>++[>+[>+<-]<-]<-]"))
            .unwrap();

        for i in 0..4 {
            println!("{}: {}", i, interpreter.get_byte_array()[i]);
        }

        assert_eq!(interpreter.get_byte_array()[3], 2);
    }

    #[test]
    fn should_return_0_when_parsed_48_plus_and_one_dot() {
        let mut interpreter = BrainfuckInterpreter::new();
        assert_eq!(
            interpreter
                .interpret(&String::from(
                    "++++++++++++++++++++++++++++++++++++++++++++++++."
                ))
                .unwrap(),
            "0"
        );
    }

    #[test]
    fn xd() {
        let mut interpreter = BrainfuckInterpreter::new();
        assert_eq!(interpreter.interpret(&String::from("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.")).unwrap(), "Hello World!\n");
    }
}
