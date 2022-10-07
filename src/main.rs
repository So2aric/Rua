use std::{fs,env};
use rua::lang::{lexer,parser,compiler,vm};

fn main() {

        let args:Vec<String> = env::args().collect();
        let path = &args[1];
        let content = fs::read_to_string(path).expect("error");

        let mut lexer = lexer::Lexer::new(&content);
        let mut compiler = compiler::Compiler::new();
        let mut parse = parser::Parser::new(lexer.analyze());

        let res = compiler.compile(&parse.parse());

        let mut vm = vm::VirtualMachine::new(res);
        vm.run();

}
