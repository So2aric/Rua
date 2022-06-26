use std::collections::HashMap;

use super::{ast::{IdentList, ExprList, Expr, StmtList, Ident, Stmt}, bytecode::{Bytecode, Instruction, Bytecodes}, token::TokenKind};

pub struct Compiler {
    codes: Vec<Bytecode>,
    num_map: HashMap<String, usize>,
    num_map_ptr: usize,
    str_map: HashMap<String, usize>,
    str_map_ptr: usize,

    ident_map: HashMap<String, usize>,
    ident_map_ptr: usize
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            codes: vec![],
            num_map: HashMap::new(),
            num_map_ptr: 0,
            str_map: HashMap::new(),
            str_map_ptr: 0,

            ident_map: HashMap::new(),
            ident_map_ptr: 0
        }
    }

    pub fn compile(&mut self, node: &StmtList) -> Bytecodes {
        self.visit_stmt_list(node);

        let mut bc = self.codes.clone();
        bc.push(Bytecode { inst: Instruction::End, arg: 0 });

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
            },
            Stmt::While { cond, body } => {
                self.visit_while(cond, body);
            }
        }
    }

    fn visit_while(&mut self,
        cond: &Expr,
        body: &StmtList
    ) {
        let cond_pos = self.codes.len() - 1;
        self.visit_expr(cond);
        self.codes.push(Bytecode { inst: Instruction::JumpAbsoluteIfFalse, arg: 0 });
        let jmp_pos = self.codes.len() - 1;

        self.visit_stmt_list(body);
        self.codes.push(Bytecode { inst: Instruction::JumpAbsolute, arg: cond_pos });
        self.codes[jmp_pos].arg = self.codes.len() - 1;
    }

    fn visit_if(&mut self,
        cond: &Expr,
        if_body: &StmtList,
        elseif_conds: &Vec<Box<Expr>>,
        elseif_bodies: &Vec<StmtList>,
        else_body: &StmtList
    ) {
        self.visit_expr(cond);
        self.codes.push(Bytecode { inst: Instruction::JumpAbsoluteIfFalse, arg: 0 });
        let if_pos = self.codes.len() - 1;
        
        self.visit_stmt_list(if_body);
        self.codes.push(Bytecode { inst: Instruction::JumpAbsolute, arg: 0 });
        self.codes[if_pos].arg = self.codes.len() - 1;

        let mut s_pos = vec![self.codes.len() - 1];

        elseif_conds.iter().zip(elseif_bodies.iter())
            .for_each(|(cond, body)| {
                self.visit_expr(cond);
                self.codes.push(Bytecode { inst: Instruction::JumpAbsoluteIfFalse, arg: 0 });
                let if_pos = self.codes.len() - 1;
                
                self.visit_stmt_list(body);
                self.codes.push(Bytecode { inst: Instruction::JumpAbsolute, arg: 0 });
                self.codes[if_pos].arg = self.codes.len() - 1;
                s_pos.push(self.codes.len() - 1);
            });

        self.visit_stmt_list(else_body);
        
        for i in s_pos {
            self.codes[i].arg = self.codes.len() - 1;
        }
    }

    fn visit_assign(&mut self,
        ident_list: &IdentList,
        expr_list: &ExprList
    ) {
        for expr in expr_list {
            self.visit_expr(expr);
        }
        for ident in ident_list.iter().rev() {
            self.visit_ident(ident, Instruction::StoreGlob);
        }


    }

    fn visit_ident(&mut self, ident: &Ident, inst: Instruction) {
        let val = ident.name.clone();

        let arg = if self.ident_map.contains_key(&val) {
            *self.ident_map.get(&val).unwrap()
        } else {
            let arg = self.ident_map_ptr;
            self.ident_map.insert(val, arg);
            self.ident_map_ptr += 1;
            arg
        };

        self.codes.push(Bytecode {
            inst: inst,
            arg
        });
    }

    fn visit_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::BinOp { op, left, right } => {
                self.visit_expr(left);
                self.visit_expr(right);
                
                match op {
                    TokenKind::Plus => self.codes.push(Bytecode {
                        inst: Instruction::BinAdd,
                        arg: 0
                    }),
                    TokenKind::Minus => self.codes.push(Bytecode {
                        inst: Instruction::BinMinus,
                        arg: 0
                    }),
                    TokenKind::Mul => self.codes.push(Bytecode {
                        inst: Instruction::BinMul,
                        arg: 0
                    }),
                    TokenKind::RealDiv => self.codes.push(Bytecode {
                        inst: Instruction::BinRealDiv,
                        arg: 0
                    }),
                    TokenKind::IntDiv => self.codes.push(Bytecode {
                        inst: Instruction::BinIntDiv,
                        arg: 0
                    }),
                    TokenKind::Mod => self.codes.push(Bytecode {
                        inst: Instruction::BinMod,
                        arg: 0
                    }),
                    TokenKind::Concat => self.codes.push(Bytecode {
                        inst: Instruction::BinConcat,
                        arg: 0
                    }),
                    TokenKind::Pow => self.codes.push(Bytecode {
                        inst: Instruction::BinPow,
                        arg: 0
                    }),

                    TokenKind::Lt => self.codes.push(Bytecode {
                        inst: Instruction::BinLt,
                        arg: 0
                    }),

                    _ => panic!("BinOP???")
                }
            },

            Expr::UnaryOp { op, node } => {
                self.visit_expr(node);

                match op {
                    TokenKind::Not => self.codes.push(Bytecode {
                        inst: Instruction::UnaryNot,
                        arg: 0
                    }),
                    TokenKind::Minus => self.codes.push(Bytecode {
                        inst: Instruction::UnaryMinus,
                        arg: 0
                    }),
                    TokenKind::Len => self.codes.push(Bytecode {
                        inst: Instruction::UnaryLen,
                        arg: 0
                    }),

                    _ => panic!("UnaryOP???")
                }
            },

            Expr::Number(x) => {
                let b = self.make_num_code(Instruction::LoadNumber, *x);
                self.codes.push(b);
            },

            Expr::Ident(x) => {
                self.visit_ident(x, Instruction::LoadGlob);
            },

            Expr::Boolean(x) => {
                self.codes.push(Bytecode {
                    inst: if *x {
                        Instruction::LoadTrue
                    } else {
                        Instruction::LoadFalse
                    },
                    arg: 0
                });
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
            self.num_map_ptr += 1;
            
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