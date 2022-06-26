use std::vec::IntoIter;

use super::{token::{Token, TokenKind}, ast::{StmtList, Stmt, ExprList, IdentList, Ident, Expr}};

pub struct Parser {
    toks: IntoIter<Token>,
    tok: Token
}

impl Parser {
    pub fn new(toks: Vec<Token>) -> Parser {
        let mut toks = toks.into_iter();
        let tok = toks.next().unwrap();

        Parser { toks, tok }
    }

    pub fn parse(&mut self) -> StmtList {
        let res = self.stmt_list();

        if !self.matches(TokenKind::Eof) {
            panic!("Unexpected ended. cur_tok: {:?}", self.tok);
        }

        res
    }

    #[inline]
    fn matches(&mut self, tok_kind: TokenKind) -> bool {
        self.tok.kind == tok_kind
    }

    fn eat(&mut self, tok_kind: TokenKind) {
        if self.matches(tok_kind) {
            self.tok = self.toks.next().unwrap();
        } else {
            panic!("From EAT: expected {:?}, found {:?}.", tok_kind, self.tok);
        }
    }

    // stmt_list = { stmt }
    fn stmt_list(&mut self) -> StmtList {
        let mut res = vec![];

        while ![TokenKind::Eof, TokenKind::Elseif,
            TokenKind::Else, TokenKind::End]
            .contains(&self.tok.kind)
        {
            res.push(self.stmt());
        }

        res
    }

    // stmt = assign_stmt | if_stmt | while_stmt
    fn stmt(&mut self) -> Stmt {
        match self.tok.kind {
            TokenKind::If => self.if_stmt(),
            TokenKind::Ident => self.assign_stmt(),
            TokenKind::While => self.while_stmt(),

            _ => panic!("Unknown statement. cur_tok: {:?}", self.tok.kind)
        }
    }

    // assign_stmt = ident_list '=' expr_list
    fn assign_stmt(&mut self) -> Stmt {
        let ident_list = self.ident_list();
        self.eat(TokenKind::Assign);
        let expr_list = self.expr_list();

        Stmt::Assign { ident_list, expr_list }
    }

    // if_stmt = 'if' expr 'then' stmt_list { 'elseif' expr 'then' stmt_list } [ 'else' stmt_list ] 'end'
    fn if_stmt(&mut self) -> Stmt {
        self.eat(TokenKind::If);
        let cond = *self.expr();
        self.eat(TokenKind::Then);
        let if_body = self.stmt_list();

        let mut elseif_conds = vec![];
        let mut elseif_bodies = vec![];

        while self.matches(TokenKind::Elseif) {
            self.eat(TokenKind::Elseif);
            elseif_conds.push(self.expr());
            self.eat(TokenKind::Then);
            elseif_bodies.push(self.stmt_list());
        }

        let mut else_body = vec![];
        if self.matches(TokenKind::Else) {
            self.eat(TokenKind::Else);
            else_body = self.stmt_list();
        }

        self.eat(TokenKind::End);

        Stmt::If { cond, if_body, elseif_conds, elseif_bodies, else_body }
    }

    // while_stmt = 'while' expr 'do' stmt_list 'end'
    fn while_stmt(&mut self) -> Stmt {
        self.eat(TokenKind::While);
        let cond = *self.expr();
        self.eat(TokenKind::Do);
        let body = self.stmt_list();
        self.eat(TokenKind::End);

        Stmt::While { cond, body }
    }

    // ident_list = ident { , ident }
    fn ident_list(&mut self) -> IdentList {
        let mut res = vec![self.ident()];

        while self.matches(TokenKind::Comma) {
            self.eat(TokenKind::Comma);
            res.push(self.ident());
        }

        res
    }

    // ident = Ident
    fn ident(&mut self) -> Ident {
        let val = self.tok.value.clone();

        self.eat(TokenKind::Ident);

        Ident { name: val.unwrap() }
    }

    // expr_list = expr { , expr }
    fn expr_list(&mut self) -> ExprList {
        let mut res = vec![*self.expr()];

        while self.matches(TokenKind::Comma) {
            self.eat(TokenKind::Comma);
            res.push(*self.expr());
        }

        res
    }

    // expr = expr_6 { 'or' expr_6 }
    fn expr(&mut self) -> Box<Expr> {
        let mut node = self.expr_6();

        while self.matches(TokenKind::Or) {
            self.eat(TokenKind::Or);
            
            node = Box::new(Expr::BinOp {
                op: TokenKind::Or,
                left: node,
                right: self.expr_6()
            });
        }

        node
    }

    // expr_6 = expr_5 { 'and' expr_5 }
    fn expr_6(&mut self) -> Box<Expr> {
        let mut node = self.expr_5();

        while self.matches(TokenKind::And) {
            self.eat(TokenKind::And);
            
            node = Box::new(Expr::BinOp {
                op: TokenKind::And,
                left: node,
                right: self.expr_5()
            });
        }

        node
    }

    // expr_5 = expr_4 { ('<' | '>' | '<=' | '>=' | '~=' | '==') expr_4 }
    fn expr_5(&mut self) -> Box<Expr> {
        let mut node = self.expr_4();

        while [TokenKind::Lt, TokenKind::Gt,
            TokenKind::Le, TokenKind::Ge,
            TokenKind::Eq, TokenKind::UnEq]
            .contains(&self.tok.kind)
        {
            let temp = self.tok.kind;
            self.eat(self.tok.kind);

            node = Box::new(Expr::BinOp { op: temp, left: node, right: self.expr_4() });
        }

        node
    }

    // expr_4 = expr_3 { '..' expr_3 }
    fn expr_4(&mut self) -> Box<Expr> {
        let node = self.expr_3();

        if self.matches(TokenKind::Concat) {
            self.eat(TokenKind::Concat);

            Box::new(Expr::BinOp {
                op: TokenKind::Concat,
                left: node,
                right: self.expr_4()
            })
        } else {
            node
        }
    }

    // expr_3 = expr_2 { ('+' | '-') expr_2 }
    fn expr_3(&mut self) -> Box<Expr> {
        let mut node = self.expr_2();

        while [TokenKind::Plus, TokenKind::Minus]
            .contains(&self.tok.kind)
        {
            let temp = self.tok.kind;
            self.eat(self.tok.kind);

            node = Box::new(Expr::BinOp { op: temp, left: node, right: self.expr_2() });
        }

        node
    }

    // expr_2 = expr_1 { ('*' | '/' | '//' | '%') expr_1 }
    fn expr_2(&mut self) -> Box<Expr> {
        let mut node = self.expr_1();

        while [TokenKind::Mul, TokenKind::RealDiv,
            TokenKind::IntDiv, TokenKind::Mod]
            .contains(&self.tok.kind)
        {
            let temp = self.tok.kind;
            self.eat(self.tok.kind);

            node = Box::new(Expr::BinOp { op: temp, left: node, right: self.expr_1() });
        }

        node
    }

    // expr_1 = { ('not' | '#' | '-') } expr_0
    fn expr_1(&mut self) -> Box<Expr> {
        if [TokenKind::Not, TokenKind::Len, TokenKind::Minus]
            .contains(&self.tok.kind)
        {
            let temp = self.tok.kind;
            self.eat(self.tok.kind);

            Box::new(Expr::UnaryOp { op: temp, node: self.expr_1() })
        } else {
            self.expr_0()
        }
    }

    // expr_0 = factor { '^' factor }
    fn expr_0(&mut self) -> Box<Expr> {
        let node = self.factor();

        if self.matches(TokenKind::Pow) {
            self.eat(TokenKind::Pow);

            Box::new(Expr::BinOp {
                op: TokenKind::Pow,
                left: node,
                right: self.expr_0()
            })
        } else {
            node
        }
    }

    // factor = Ident | Number | String | '(' expr ')' | False | True
    fn factor(&mut self) -> Box<Expr> {
        let node = match self.tok.kind {
            TokenKind::Ident => {
                Box::new(Expr::Ident(Ident {
                    name: self.tok.value.clone().unwrap().clone()
                }))
            },
            TokenKind::Number => {
                Box::new(Expr::Number(
                    self.tok.value.clone().unwrap().parse().unwrap()
                ))
            },
            TokenKind::String => {
                Box::new(Expr::String(
                    self.tok.value.clone().unwrap().parse().unwrap()
                ))
            },
            TokenKind::Lpar => {
                self.eat(TokenKind::Lpar);
                
                self.expr()
            },
            TokenKind::False => {
                Box::new(Expr::Boolean(false))
            },
            TokenKind::True => {
                Box::new(Expr::Boolean(true))
            },

            _ => panic!("Unexpected in factor: {:?}", self.tok.kind)
        };

        self.eat(self.tok.kind);

        node
    }
}


#[cfg(test)]
mod tests {
    use crate::lang::lexer::Lexer;

    use super::*;

    #[test]
    fn see() {
        let toks = Lexer::new(r#"
            a = 1 + 3 ^ 4 ^ 2
            b = 6 * (5 - 2)
            c, d = "wow" .. "yeah", 1

            if a + b > 3 then
                c = 5
            elseif b < 5 then
                b = 1
            elseif a - 1 == 0 then
                a = a + 1
                b = b - 2
            else
                d = "Oh no"
            end
        "#).analyze();
        let mut parser = Parser::new(toks);

        println!("{:#?}", parser.parse());
    }
}