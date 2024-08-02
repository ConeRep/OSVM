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

pub fn inst_type_as_string(inst_type: InstType) -> String {
    match inst_type {
        InstType::Push => return "Push".to_string(),
        InstType::Plus => return "Plus".to_string(),
        InstType::Minus => return "Minus".to_string(),
        InstType::Mult => return "Mult".to_string(),
        InstType::Div => return "Div".to_string(),
        _ => {
            assert!(false, "inst_type_as_string: Unreachable");
            return "Unreachable".to_string()
        }
    }
}

pub fn inst_push(value: Word) -> Inst { Inst { itype: InstType::Push, operand: value, } }
pub fn inst_plus() -> Inst { Inst { itype: InstType::Plus, operand: 0 } }
pub fn inst_minus() -> Inst { Inst { itype: InstType::Minus, operand: 0 } }
pub fn inst_mult() -> Inst { Inst { itype: InstType::Mult, operand: 0 } }
pub fn inst_div() -> Inst { Inst { itype: InstType::Div, operand: 0 } }