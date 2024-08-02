#[derive(PartialEq)]
pub enum Err {
    ErrOK = 0,
    ErrStackOverflow,
    ErrStackUnderflow,
    ErrIllegalInst,
}

pub fn err_as_string(err: Err) -> String {
    match err {
        Err::ErrOK => return "Ok".to_string(),
        Err::ErrStackOverflow => return "ErrStackOverflow".to_string(),
        Err::ErrStackUnderflow => return "ErrStackUnderflow".to_string(),
        Err::ErrIllegalInst => return "ErrIllegalInst".to_string(),
        _ => {
            assert!(false, "err_as_string: Unreachable");
            return "Unreachable".to_string();
        },
    };
}