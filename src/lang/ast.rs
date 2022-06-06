use super::token::TokenKind;

pub trait AST {}


pub enum ASTNode {
    StmtList(StmtList),
    Expr(Expr)
}
impl AST for ASTNode {}


pub type ExprList = Vec<Expr>;
impl AST for ExprList {}

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

    FuncDecl(FuncDecl),
    FuncCall(FuncCall)
}
impl AST for Expr {}


pub type IdentList = Vec<Ident>;
impl AST for IdentList {}

#[derive(Debug)]
pub struct Ident {
    pub name: String
}
impl AST for Ident {}


#[derive(Debug)]
pub struct FuncDecl {
    pub ident: Ident,
    pub args: IdentList,
    pub body: Box<StmtList>
}
impl AST for FuncDecl {}


#[derive(Debug)]
pub struct FuncCall {
    pub ident: Ident,
    pub args: ExprList
}
impl AST for FuncCall {}


pub type StmtList = Vec<Stmt>;
impl AST for StmtList {}

#[derive(Debug)]
pub enum Stmt {
    Assign {
        ident_list: IdentList,
        expr_list: ExprList
    }
}
impl AST for Stmt {}
