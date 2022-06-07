use std::collections::HashMap;

use super::{ast::{IdentList, ExprList, Expr, StmtList, Ident, Stmt}, bytecode::{Bytecode, Instruction, Arg, Bytecodes}, token::TokenKind};

pub struct Compiler {
    codes: Vec<Bytecode>,
    num_map: HashMap<String, Arg>,
    num_map_ptr: Arg,
    str_map: HashMap<String, Arg>,
    str_map_ptr: Arg,

    ident_map: HashMap<String, Arg>,
    ident_map_ptr: Arg
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            codes: vec![],
            num_map: HashMap::new(),
            num_map_ptr: Arg::new(0),
            str_map: HashMap::new(),
            str_map_ptr: Arg::new(0),

            ident_map: HashMap::new(),
            ident_map_ptr: Arg::new(0)
        }
    }

    pub fn compile(&mut self, node: &StmtList) -> Bytecodes {
        self.visit_stmt_list(node);

        let bc = self.codes.clone();

        // to make the keys ordered
        // there should be a better method
        let mut nums = self.num_map.clone().into_iter()
            .collect::<Vec<_>>();
        nums.sort_by(|a, b| {
                a.1.cmp(&b.1)
        });
        let nums = nums.into_iter().map(|v| {
            v.0.parse().unwrap()
        }).collect();
        
        let mut strs = self.str_map.clone().into_iter()
            .collect::<Vec<_>>();
        strs.sort_by(|a, b| {
                a.1.cmp(&b.1)
        });
        let strs = strs.into_iter().map(|v| {
            v.0
        }).collect();

        let mut idents = self.ident_map.clone().into_iter()
            .collect::<Vec<_>>();
        idents.sort_by(|a, b| {
                a.1.cmp(&b.1)
        });
        let idents = idents.into_iter().map(|v| {
            v.0
        }).collect();

        Bytecodes { bc, nums, strs, idents }
    }

    fn visit_stmt_list(&mut self, node: &StmtList) {
        for stmt in node {
            self.visit_stmt(stmt);
        }
    }

    fn visit_stmt(&mut self, node: &Stmt) {
        match node {
            Stmt::Assign { ident_list, expr_list } => {
                self.visit_assign(ident_list, expr_list);
            },
            Stmt::If {
                cond,
                if_body,
                elseif_conds,
                elseif_bodies,
                else_body
            } => {
                self.visit_if(cond, if_body, elseif_conds, elseif_bodies, else_body);
            }
        }
    }

    fn visit_if(&mut self,
        cond: &Expr,
        if_body: &StmtList,
        elseif_conds: &Vec<Box<Expr>>,
        elseif_bodies: &Vec<StmtList>,
        else_body: &StmtList
    ) {
        unimplemented!()
    }

    fn visit_assign(&mut self,
        ident_list: &IdentList,
        expr_list: &ExprList
    ) {
        for ident in ident_list {
            self.visit_ident(ident);
        }
        for expr in expr_list {
            self.visit_expr(expr);
        }
        
        self.codes.push(Bytecode {
            inst: Instruction::StoreGlob,
            arg: Arg::new(0)
        });
    }

    fn visit_ident(&mut self, ident: &Ident) {
        let val = dbg!(ident).name.clone();

        let arg = if self.num_map.contains_key(&val) {
            *self.ident_map.get(&val).unwrap()
        } else {
            let arg = self.ident_map_ptr;
            self.ident_map.insert(val, arg);
            self.ident_map_ptr.advance();
            arg
        };

        self.codes.push(Bytecode {
            inst: Instruction::LoadGlob,
            arg
        });
    }

    fn visit_expr(&mut self, expr: &Expr) {
        match dbg!(expr) {
            Expr::BinOp { op, left, right } => {
                self.visit_expr(left);
                self.visit_expr(right);
                
                match op {
                    TokenKind::Plus => self.codes.push(Bytecode {
                        inst: Instruction::BinAdd,
                        arg: Arg::new(0)
                    }),
                    TokenKind::Minus => self.codes.push(Bytecode {
                        inst: Instruction::BinMinus,
                        arg: Arg::new(0)
                    }),
                    TokenKind::Mul => self.codes.push(Bytecode {
                        inst: Instruction::BinMul,
                        arg: Arg::new(0)
                    }),
                    TokenKind::RealDiv => self.codes.push(Bytecode {
                        inst: Instruction::BinRealDiv,
                        arg: Arg::new(0)
                    }),
                    TokenKind::IntDiv => self.codes.push(Bytecode {
                        inst: Instruction::BinIntDiv,
                        arg: Arg::new(0)
                    }),
                    TokenKind::Mod => self.codes.push(Bytecode {
                        inst: Instruction::BinMod,
                        arg: Arg::new(0)
                    }),
                    TokenKind::Concat => self.codes.push(Bytecode {
                        inst: Instruction::BinConcat,
                        arg: Arg::new(0)
                    }),
                    TokenKind::Pow => self.codes.push(Bytecode {
                        inst: Instruction::BinPow,
                        arg: Arg::new(0)
                    }),

                    _ => panic!("BinOP???")
                }
            },

            Expr::UnaryOp { op, node } => {
                self.visit_expr(node);

                match op {
                    TokenKind::Not => self.codes.push(Bytecode {
                        inst: Instruction::UnaryNot,
                        arg: Arg::new(0)
                    }),
                    TokenKind::Minus => self.codes.push(Bytecode {
                        inst: Instruction::UnaryMinus,
                        arg: Arg::new(0)
                    }),
                    TokenKind::Len => self.codes.push(Bytecode {
                        inst: Instruction::UnaryLen,
                        arg: Arg::new(0)
                    }),

                    _ => panic!("UnaryOP???")
                }
            },

            Expr::Number(x) => {
                let b = self.make_num_code(Instruction::LoadNumber, *x);
                self.codes.push(b);
            }

            _ => unimplemented!()
        }
    }

    fn make_num_code(&mut self, inst: Instruction, val: f64) -> Bytecode {
        let val = val.to_string();
        if self.num_map.contains_key(&val) {
            Bytecode {
                inst,
                arg: *self.num_map.get(&val).unwrap()
            }
        } else {
            let arg = self.num_map_ptr;
            self.num_map.insert(val, arg);
            self.num_map_ptr.advance();
            
            Bytecode {
                inst,
                arg
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lang::{parser::Parser, lexer::Lexer};

    use super::*;

    #[test]
    fn see() {
        let toks = Lexer::new(r#"
            a = 1 + 3 ^ 4 ^ 2
            b = 6 * (5 - 2)
        "#).analyze();
        let mut parser = Parser::new(toks);
        let mut compiler = Compiler::new();
        let res = compiler.compile(&parser.parse());

        println!("{:#?}", res);
    }
}