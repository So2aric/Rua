#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    LoadNumber,
    LoadString,
    LoadGlob,
    StoreGlob,
    LoadLocal,
    StoreLocal,

    LoadTrue,
    LoadFalse,
    LoadNil,

    UnaryNot,
    UnaryMinus,
    UnaryLen,

    BinAdd,
    BinMinus,
    BinMul,
    BinRealDiv,
    BinIntDiv,
    BinPow,
    BinConcat,
    BinMod,

    BinLt,
    BinLe,
    BinEq,
    BinAnd,
    BinOr,

    JumpAbsoluteIfFalse,
    JumpAbsolute,

    FuncDecl,
    Return,
    FuncCall,

    End
}

#[derive(Debug, Clone, Copy)]
pub struct Bytecode {
    pub inst: Instruction,
    pub arg: usize
}

#[derive(Debug)]
pub struct Bytecodes {
    pub bc: Vec<Bytecode>,
    pub nums: Vec<f64>,
    pub strs: Vec<String>,
    pub idents: Vec<String>
}