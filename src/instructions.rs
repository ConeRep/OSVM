type Word = i64;

#[derive(Clone)]
pub enum InstType {
    Push,
    Plus,
    Minus,
    Mult,
    Div,
}

#[derive(Clone)]
pub struct Inst {
    pub itype: InstType,
    pub operand: Word,
}

pub fn inst_push(value: Word) -> Inst { Inst { itype: InstType::Push, operand: value, } }
pub fn inst_plus() -> Inst { Inst { itype: InstType::Plus, operand: 0 } }
pub fn inst_minus() -> Inst { Inst { itype: InstType::Minus, operand: 0 } }
pub fn inst_mult() -> Inst { Inst { itype: InstType::Mult, operand: 0 } }
pub fn inst_div() -> Inst { Inst { itype: InstType::Div, operand: 0 } }