A simple brainfuck interpreter implemented in rust.

run by providing brainfuck string to be interpreted kike so:
```
cargo run -- "[input]"
```

Hello world:
```
Cargo run -- "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++."
Output: "Hello world!"
```

the , operator has not been implemented, so it will just be ignored if present in the provided string.
