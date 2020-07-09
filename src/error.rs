use std::{
    error::Error,
    fmt::{Formatter, Result, Display},
};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct InvalidOpCode(u8);

impl InvalidOpCode {
    pub fn new(op_code: u8) -> Self {
        Self(op_code)
    }
}

impl Display for InvalidOpCode {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Invalid op code {:#X}", self.0)
    }
}

impl Error for InvalidOpCode {}
