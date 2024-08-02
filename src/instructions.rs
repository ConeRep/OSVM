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
    JumpIf,
    Equal,
    Dupl,
    Halt,

    // Debug
    PrintDebug,
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
        InstType::JumpIf => return "JumpIf".to_string(),
        InstType::Dupl => return "Dupl".to_string(),
        InstType::Equal => return "Equal".to_string(),
        InstType::Halt => return "Halt".to_string(),

        InstType::PrintDebug => return "PrintDebug".to_string(),
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
pub fn inst_jmp_if(addr: Word) -> Inst { Inst { itype: InstType::JumpIf, operand: addr } }
pub fn inst_dupl(value: Word) -> Inst { Inst { itype: InstType::Dupl, operand: value } }
pub fn inst_halt() -> Inst { Inst { itype: InstType::Halt, operand: 0 } }

// Debug
pub fn inst_print_debug() -> Inst { Inst { itype: InstType::PrintDebug, operand: 0 } }