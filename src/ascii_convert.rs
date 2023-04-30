
pub struct AsciiConverter {
}

impl AsciiConverter {
    pub fn convert(input: &Vec<u8>) -> String {
        let mut output = String::new();
        for x in input.iter() {
            output.push(*x as char);
        }
        output.clone()
    }
}