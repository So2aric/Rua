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
        cond: Box<Expr>, // condition
        if_body: StmtList,
        elseif_conds: Vec<Box<Expr>>,
        elseif_bodies: Vec<StmtList>,
        else_body: StmtList
    }
}


pub trait ASTWalker {
    // type Result;

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

    fn visit_assign(&mut self, ident_list: &IdentList, expr_list: &ExprList);

    fn visit_if(&mut self,
        cond: &Box<Expr>,
        if_body: &StmtList,
        elseif_conds: &Vec<Box<Expr>>,
        elseif_bodies: &Vec<StmtList>,
        else_body: &StmtList
    );
}
