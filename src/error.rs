#[derive(PartialEq)]
pub enum Err {
    ErrOK = 0,
    ErrStackOverflow,
    ErrStackUnderflow,
    ErrIllegalInst,
    ErrIllegalInstAccess,
    ErrDivByZero,
}

pub fn err_as_string(err: Err) -> String {
    match err {
        Err::ErrOK => return "Ok".to_string(),
        Err::ErrStackOverflow => return "ErrStackOverflow".to_string(),
        Err::ErrStackUnderflow => return "ErrStackUnderflow".to_string(),
        Err::ErrIllegalInst => return "ErrIllegalInst".to_string(),
        Err::ErrIllegalInstAccess => return "ErrIllegalInstAccess".to_string(),
        Err::ErrDivByZero => return "ErrDivByZero".to_string(),
        _ => {
            assert!(false, "err_as_string: Unreachable");
            return "Unreachable".to_string();
        },
    };
}