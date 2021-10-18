use std::{fmt::Debug, ops::Deref};

#[derive(Debug)]
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

#[derive(Debug)]
#[non_exhaustive]
pub struct ParseError {}

pub type ParseResult<T> = Result<ParseOk<T>, ParseError>;
