use super::token::TokenKind;

pub type ExprList = Vec<Expr>;

#[derive(Debug)]
pub enum Expr {
    BinOp {
        op: TokenKind,
        left: Box<Expr>,
        right: Box<Expr>
    },
    UnaryOp {
        op: TokenKind,
        node: Box<Expr>
    },

    Ident(Ident),
    Number(f64),
    String(String),
    Boolean(bool),

    FuncDecl(FuncDecl),
    FuncCall(FuncCall)
}


pub type IdentList = Vec<Ident>;

#[derive(Debug)]
pub struct Ident {
    pub name: String
}


#[derive(Debug)]
pub struct FuncDecl {
    pub ident: Ident,
    pub args: IdentList,
    pub body: Box<StmtList>
}


#[derive(Debug)]
pub struct FuncCall {
    pub ident: Ident,
    pub args: ExprList
}


pub type StmtList = Vec<Stmt>;

#[derive(Debug)]
pub enum Stmt {
    Assign {
        ident_list: IdentList,
        expr_list: ExprList
    },
    If {
        cond: Expr, // condition
        if_body: StmtList,
        elseif_conds: Vec<Box<Expr>>,
        elseif_bodies: Vec<StmtList>,
        else_body: StmtList
    },
    While {
        cond: Expr,
        body: StmtList
    }
}
