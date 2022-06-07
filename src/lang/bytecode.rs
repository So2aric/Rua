#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    LoadNumber,
    LoadString,
    LoadGlob,
    StoreGlob,
    LoadLocal,
    StoreLocal,

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
    BinOr
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct Arg(i32);

impl Arg {
    pub fn new(p: i32) -> Arg {
        Arg(p)
    }

    pub fn advance(&mut self) -> i32 {
        self.0 += 1;

        self.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Bytecode {
    pub inst: Instruction,
    pub arg: Arg
}

#[derive(Debug)]
pub struct Bytecodes {
    pub bc: Vec<Bytecode>,
    pub nums: Vec<f64>,
    pub strs: Vec<String>,
    pub idents: Vec<String>
}