mod error;
use std::usize;

use error::*;

const OSVM_STACK_CAPACITY: usize = 1024;

type Word = i64;

#[derive(Clone)]
struct OSVM {
    stack: Vec<usize>,
}

enum InstType {
    Push,
    Plus,
}

struct Inst {
    itype: InstType,
    operand: Word,
}

fn inst_push(value: Word) -> Inst {
    Inst {
        itype: InstType::Push,
        operand: value,
    }
}

fn inst_plus() -> Inst {
    Inst {
        itype: InstType::Plus,
        operand: 0
    }
}

fn osvm_execute_inst(osvm: &mut OSVM, inst: Inst) -> Err {
    match inst.itype {
        InstType::Push => {
            if osvm.stack.len() >= OSVM_STACK_CAPACITY {
                return Err::ErrStackOverflow
            }
            osvm.stack.push(inst.operand as usize);
        }
        InstType::Plus => {
            if osvm.stack.len() < 2 {
                return Err::ErrStackUnderflow
            }
            let a = osvm.stack.pop();
            let b = osvm.stack.pop();
            let add = match (a, b) {
                (Some(x), Some(y)) => Some(x + y),
                _ => None,
            };

            osvm.stack.push(add.unwrap());
        }
        _ => return Err::ErrIllegalInst
    }

    Err::ErrOK
}

fn osvm_dump(osvm: &mut OSVM) {
    println!("Stack:");
    if osvm.stack.len() > 0 {
        for i in 0..osvm.stack.len() {
            println!("  {:?}", osvm.stack);
        }
    } else {
        println!("  [empty]");
    }
}

fn main() {
    let osvm: &mut OSVM = &mut OSVM {
        stack: Vec::with_capacity(OSVM_STACK_CAPACITY),
    };

    osvm_dump(osvm);
    osvm_execute_inst(osvm, inst_push(69));
    osvm_dump(osvm);
    osvm_execute_inst(osvm, inst_push(420));
    osvm_dump(osvm);
    osvm_execute_inst(osvm, inst_plus());
    osvm_dump(osvm);
}
