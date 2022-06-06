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
            panic!("Unexpected ended.");
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

        while !self.matches(TokenKind::Eof) {
            res.push(self.stmt());
        }

        res
    }

    // stmt = ident_list '=' expr_list
    fn stmt(&mut self) -> Stmt {
        Stmt::Assign { ident_list: self.ident_list(), expr_list: self.expr_list() }
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
        let mut res = vec![self.expr()];

        while self.matches(TokenKind::Comma) {
            self.eat(TokenKind::Comma);
            res.push(self.expr());
        }

        res
    }

    fn expr(&mut self) -> Expr {
        unimplemented!()
    }
}