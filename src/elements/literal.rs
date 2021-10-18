use crate::{ParseElement, ParseError, ParseOk, ParseResult};

pub type Literal<'a> = &'a str;

impl ParseElement for &str {
    type ParseOut = String;

    fn pars(&self, input: &str) -> ParseResult<String> {
        if input.starts_with(self) {
            Ok(ParseOk {
                bytes_parsed: self.len(),
                result: String::from(*self),
            })
        } else {
            Err(ParseError {})
        }
    }
}

#[test]
fn literal() {
    let parser = "test";
    let result = parser.pars("test");
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.bytes_parsed, "test".len());
    assert_eq!(result.result, String::from("test"));
}
