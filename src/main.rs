mod instructions;
use instructions::*;
mod error;
use error::*;

use std::{process::exit, usize, vec};

const OSVM_STACK_CAPACITY: usize = 1024;

#[derive(Clone)]
struct OSVM {
    stack: Vec<usize>,
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
            let new_num = match (a, b) {
                (Some(x), Some(y)) => Some(y + x),
                _ => None,
            };

            osvm.stack.push(new_num.unwrap());
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
        }
        _ => return Err::ErrIllegalInst
    }

    Err::ErrOK
}

fn osvm_dump(osvm: &mut OSVM) {
    println!("Stack:");
    if osvm.stack.len() > 0 {
        for _i in 0..osvm.stack.len() {
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

    let program: Vec<Inst> = vec![
        inst_push(69),
        inst_push(420),
        inst_plus(),
        inst_push(420),
        inst_push(69),
        inst_minus(),
        inst_push(82),
        inst_push(300),
        inst_mult(),
        inst_push(10),
        inst_push(0),
        inst_div(),
    ];

    for i in 0..program.len() {
        println!("\n{}", inst_type_as_string(program[i].itype.clone()));
        let err: Err = osvm_execute_inst(osvm, program[i].clone());
        if err != Err::ErrOK {
            eprintln!("Err: {}", err_as_string(err));
            osvm_dump(osvm);
            exit(1);
        }
        osvm_dump(osvm);
    }
}
