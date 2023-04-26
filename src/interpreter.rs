
pub struct Interpreter {}

impl Interpreter {
    //initializes a new interpreter
    pub fn new() -> Interpreter {
        Interpreter {}
    }

    //function that interprets a string and returns a result. OK if the string is valid brainfuck and Err otherwise.
    pub fn interpret(&self, source: String) -> Result<String, String> {
        let result = match self.eval(source) {
            Ok(_) => todo!(),
            Err(_) => todo!(),
        };

        Ok(result)
    }

    //evaluates the source string. OK returns the interpretted value and Err returns an error code.
    fn eval(&self, source: String) -> Result<String, i64> {
        todo!()
    }
}
