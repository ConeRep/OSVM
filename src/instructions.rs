type Word = i64;

#[derive(Clone)]
pub enum InstType {
    Push,
    Plus,
}

#[derive(Clone)]
pub struct Inst {
    pub itype: InstType,
    pub operand: Word,
}

pub fn inst_push(value: Word) -> Inst { Inst { itype: InstType::Push, operand: value, } }
pub fn inst_plus() -> Inst { Inst { itype: InstType::Plus, operand: 0 } }