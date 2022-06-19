use std::collections::HashMap;

use super::bytecode::{Bytecodes, Instruction::*};

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),

    Nil
}

pub struct VirtualMachine {
    codes: Bytecodes,
    p: usize,

    stack: Vec<Value>,
    memory: HashMap<String, Value>
}

impl VirtualMachine {
    pub fn new(codes: Bytecodes) -> VirtualMachine {
        VirtualMachine { codes, p: 0, stack: vec![], memory: HashMap::new() }
    }

    pub fn run(&mut self) {
        loop {
            let code = self.codes.bc[self.p];

            match code.inst {
                LoadNumber => {
                    self.stack.push(Value::Number(self.codes.nums[code.arg]));
                },

                LoadGlob => {
                    let name = &self.codes.idents[code.arg];
                    self.stack.push(self.memory.get(name).unwrap().clone());
                },
                StoreGlob => {
                    let name = &self.codes.idents[code.arg];
                    self.memory.insert(name.clone(), self.stack.pop().unwrap());
                },

                LoadTrue => {
                    self.stack.push(Value::Boolean(true));
                },
                LoadFalse => {
                    self.stack.push(Value::Boolean(false));
                },

                BinAdd => {
                    let right = self.pop_num();
                    let left = self.pop_num();

                    self.stack.push(Value::Number(left + right));
                },
                BinMinus => {
                    let right = self.pop_num();
                    let left = self.pop_num();

                    self.stack.push(Value::Number(left - right));
                },
                BinMul => {
                    let right = self.pop_num();
                    let left = self.pop_num();

                    self.stack.push(Value::Number(left * right));
                },
                BinIntDiv => {
                    let right = self.pop_num();
                    let left = self.pop_num();

                    self.stack.push(Value::Number((left / right).floor()));
                },
                BinRealDiv => {
                    let right = self.pop_num();
                    let left = self.pop_num();

                    self.stack.push(Value::Number(left / right));
                },

                JumpAbsoluteIfFalse => {
                    if !match self.stack.pop().unwrap() {
                        Value::Boolean(x) => x,
                        Value::Nil => false,

                        _ => true
                    } {
                        // we'll add one to p after the match, so we substract one here
                        self.p = code.arg;
                    }
                },
                JumpAbsolute => {
                    self.p = code.arg;
                }

                End => break,

                _ => unimplemented!()
            }

            self.p += 1;
        }

        println!("{:?}", self.memory);
    }

    fn pop_num(&mut self) -> f64 {
        match self.stack.pop().unwrap() {
            Value::Number(x) => x,

            _ => 0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lang::{lexer::Lexer, parser::Parser, compiler::Compiler};

    use super::*;

    #[test]
    fn see() {
        let toks = Lexer::new("
            a = 1 + 3
            b = 1 / 2
            a, b = b, a
            
            if false then
                c = 1
            elseif true then
                c = 2
            end
        ").analyze();
        let ast = Parser::new(toks).parse();
        let co = Compiler::new().compile(&ast);
        let mut vm = VirtualMachine::new(co);

        vm.run();
    }
}