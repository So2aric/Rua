#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
    Number,
    Ident,
    String,

    Plus,       // +
    Minus,      // -
    Mul,        // *
    Pow,        // ^
    RealDiv,    // /
    IntDiv,     // //
    Mod,        // %
    Concat,     // ..
    Len,        // #

    Lpar,       // (
    Rpar,       // )
    Lsqr,       // [
    Rsqr,       // ]
    Lbrc,       // {
    Rbrc,       // }

    Dot,        // .
    Assign,     // =
    Arg,        // ...
    Comma,      // ,
    Colon,      // :
    Semi,       // ;

    Eq,         // ==
    UnEq,       // ~=
    Lt,         // <
    Le,         // <=
    Gt,         // >
    Ge,         // >=

    If,
    Else,
    Elseif,
    Then,
    Do,
    While,
    End,
    Function,

    True,
    False,
    And,
    Not,
    Or,

    Eof
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Location {
    line: i32,
    column: i32
}

impl Location {
    pub fn new() -> Location {
        Location { line: 1, column: 1 }
    }

    pub fn advance(&mut self) {
        self.column += 1;
    }

    pub fn new_line(&mut self) {
        self.line += 1;
        self.column = 1;
    }
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub value: Option<String>,
    pub loc: Location
}