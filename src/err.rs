use failure_derive::*;

#[derive(Debug, Clone, PartialEq, Fail)]
pub enum ECode {
    #[fail(display = "End of Input")]
    EOF,
    #[fail(display = "{}", 0)]
    SMess(&'static str),
    #[fail(display = "{}", 0)]
    Mess(String),
}

#[derive(Debug, Clone, PartialEq, Fail)]
#[fail(display = "Parse Error on line {} : {}", code, line)]
pub struct ParseError {
    code: ECode,
    line: u64,
    col: u64,
}

impl ParseError {
    pub fn new(s: &'static str, line: u64, col: u64) -> ParseError {
        ParseError {
            code: ECode::SMess(s),
            line,
            col,
        }
    }
    pub fn code(code: ECode, line: u64, col: u64) -> ParseError {
        ParseError { code, line, col }
    }
}
