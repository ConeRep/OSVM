mod instructions;
use instructions::*;
mod error;
use error::*;

use std::{os, process::exit, usize, vec};

const OSVM_STACK_CAPACITY: usize = 1024;
const OSVM_PROGRAM_CAPACITY: usize = 1024;

#[derive(Clone)]
struct OSVM {
    stack: Vec<usize>,

    program: Vec<Inst>,
    ip: usize,
    
    halt: bool,
}

fn osvm_execute_inst(osvm: &mut OSVM) -> Err {
    if osvm.ip < 0 || osvm.ip >= osvm.program.len() {
        return Err::ErrIllegalInstAccess
    }

    let inst: Inst = osvm.program[osvm.ip];

    match inst.itype {
        InstType::Push => {
            if osvm.stack.len() >= OSVM_STACK_CAPACITY {
                return Err::ErrStackOverflow
            }
            osvm.stack.push(inst.operand as usize);
            osvm.ip += 1;
        }
        InstType::Plus => {
            if osvm.stack.len() < 2 {
                return Err::ErrStackUnderflow
            }
            let a = osvm.stack.pop();
            let b = osvm.stack.pop();
            let new_num = match (a, b) {
                (Some(x), Some(y)) => Some(y + x),
                _ => None,
            };

            osvm.stack.push(new_num.unwrap());
            osvm.ip += 1;
        }
        InstType::Minus => {
            if osvm.stack.len() < 2 {
                return Err::ErrStackUnderflow
            }
            let a = osvm.stack.pop();
            let b = osvm.stack.pop();
            let new_num = match (a, b) {
                (Some(x), Some(y)) => Some(y - x),
                _ => None,
            };

            osvm.stack.push(new_num.unwrap());
            osvm.ip += 1;
        }
        InstType::Mult => {
            if osvm.stack.len() < 2 {
                return Err::ErrStackUnderflow
            }
            let a = osvm.stack.pop();
            let b = osvm.stack.pop();
            let new_num = match (a, b) {
                (Some(x), Some(y)) => Some(y * x),
                _ => None,
            };

            osvm.stack.push(new_num.unwrap());
            osvm.ip += 1;
        }
        InstType::Div => {
            if osvm.stack.len() < 2 {
                return Err::ErrStackUnderflow
            }

            let a = osvm.stack.pop();
            let b = osvm.stack.pop();

            if a == Some(0) || b == Some(0)   {
                return Err::ErrDivByZero
            }

            let new_num = match (a, b) {
                (Some(x), Some(y)) => Some(y / x),
                _ => None,
            };

            osvm.stack.push(new_num.unwrap());
            osvm.ip += 1;
        }
        InstType::Jump => {
            osvm.ip = inst.operand as usize;
        }
        InstType::JumpIf => {
            if osvm.stack.len() < 1 {
                return Err::ErrStackUnderflow;
            }

            let a = osvm.stack.pop();

            if a.is_some() {
                osvm.ip = inst.operand as usize;
            } else {
                osvm.ip += 1;
            }

        }
        InstType::Equal => {
            if osvm.stack.len() < 2 {
                return Err::ErrStackUnderflow
            }

            let a = osvm.stack.pop();
            let b = osvm.stack.pop();

            let new_num = match (a, b) {
                (Some(x), Some(y)) => Some(y == x),
                _ => None,
            };

            osvm.stack.push(new_num.unwrap() as usize);
            osvm.ip += 1;
        }
        InstType::Dupl => {
            if osvm.stack.len() >= OSVM_STACK_CAPACITY {
                return Err::ErrStackOverflow;
            }
        
            if osvm.stack.len() == 0 {
                return Err::ErrStackUnderflow;
            }
        
            if inst.operand as usize >= osvm.stack.len() {
                return Err::ErrIllegalOperand;
            }
        
            let index = osvm.stack.len() - 1 - inst.operand as usize;
            let value_to_duplicate = osvm.stack[index];
        
            osvm.stack.push(value_to_duplicate);
            osvm.ip += 1;
        }
        InstType::Halt => {
            osvm.ip = 0;
            osvm.halt = true;
        }

        InstType::PrintDebug => {
            if osvm.stack.len() < 1 {
                return Err::ErrStackUnderflow;
            }

            println!("{:?}", osvm.stack.pop());
            osvm.ip += 1;
        }
        _ => return Err::ErrIllegalInst
    }

    Err::ErrOK
}

fn osvm_dump_stack(osvm: &mut OSVM) {
    println!("Stack:");
    if osvm.stack.len() > 0 {
        for _i in 0..osvm.stack.len() {
            println!("  {:?}", osvm.stack);
        }
    } else {
        println!("  [empty]");
    }
}

fn osvm_load_program_from_memory(osvm: &mut OSVM, program: &Vec<Inst>) {
    assert!(program.len() < OSVM_PROGRAM_CAPACITY);
    osvm.program.extend_from_slice(program);
}

fn main() {
    let osvm: &mut OSVM = &mut OSVM {
        stack: Vec::with_capacity(OSVM_STACK_CAPACITY),

        program: Vec::with_capacity(OSVM_PROGRAM_CAPACITY),
        ip: 0,
        
        halt: false,
    };

    let program: Vec<Inst> =  vec![
        inst_push(0),
        inst_push(1),
        inst_dupl(1),
        inst_dupl(1),
        inst_plus(),
        inst_jmp(2),
        inst_halt()
    ];

    osvm_load_program_from_memory(osvm, &program);

    for _i in 0..69 {
        let err: Err = osvm_execute_inst(osvm);
        if err != Err::ErrOK {
            eprintln!("Err: {}", err_as_string(err));
            exit(1);
        }
    }
    osvm_dump_stack(osvm);
}
