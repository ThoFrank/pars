use crate::{ParseElement, ParseError, ParseOk, ParseResult};

pub struct Literal<T>(pub T);

impl<T> ParseElement for Literal<T>
where
    T: AsRef<str>,
{
    type ParseOut = String;

    fn pars(&self, input: &str) -> ParseResult<String> {
        if input.starts_with(self.0.as_ref()) {
            Ok(ParseOk {
                bytes_parsed: self.0.as_ref().len(),
                result: String::from(self.0.as_ref()),
            })
        } else {
            Err(ParseError {})
        }
    }
}

impl Into<Literal<&str>> for &'static str {
    fn into(self) -> Literal<&'static str> {
        Literal(self)
    }
}

impl Into<Literal<String>> for String {
    fn into(self) -> Literal<String> {
        Literal(self)
    }
}

#[test]
fn literal() {
    let parser: Literal<_> = "test".into();
    let result = parser.pars("test");
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.bytes_parsed, "test".len());
    assert_eq!(result.result, String::from("test"));
}
