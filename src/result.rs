use std::{fmt::{Debug, Display}, ops::Deref};

#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct ParseOk<T> {
    pub bytes_parsed: usize,
    pub result: T,
}

impl<T> Deref for ParseOk<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.result
    }
}

#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct ParseError {}

pub type ParseResult<T> = Result<ParseOk<T>, ParseError>;

impl std::error::Error for ParseError{}
impl Display for ParseError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parsing Error occured!")
    }
}