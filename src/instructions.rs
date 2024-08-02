pub type Word = i64;

#[derive(Clone, Copy)]
pub enum InstType {
    // Mathematical Operations
    Push,
    Plus,
    Minus,
    Mult,
    Div,

    // Keywords
    Jump,
    Halt,
}

#[derive(Clone, Copy)]
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
        InstType::Jump => return "Jump".to_string(),
        InstType::Halt => return "Halt".to_string(),
        _ => {
            assert!(false, "inst_type_as_string: Unreachable");
            return "Unreachable".to_string()
        }
    }
}

// Mathematical Operations
pub fn inst_push(value: Word) -> Inst { Inst { itype: InstType::Push, operand: value, } }
pub fn inst_plus() -> Inst { Inst { itype: InstType::Plus, operand: 0 } }
pub fn inst_minus() -> Inst { Inst { itype: InstType::Minus, operand: 0 } }
pub fn inst_mult() -> Inst { Inst { itype: InstType::Mult, operand: 0 } }
pub fn inst_div() -> Inst { Inst { itype: InstType::Div, operand: 0 } }

// Keywords
pub fn inst_jmp(addr: Word) -> Inst { Inst { itype: InstType::Jump, operand: addr } }
pub fn inst_halt() -> Inst { Inst { itype: InstType::Halt, operand: 0 } }