use std::io;
use rua::lang::{lexer,parser,ast};

fn main() {
    while true {

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Fail to read text");

        let mut lexer = lexer::Lexer::new(&input);

        let mut parse = parser::Parser::new(lexer.analyze());
        
    }
    
}
