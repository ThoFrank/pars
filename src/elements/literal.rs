use crate::{Or, ParseElement, ParseError, ParseOk, ParseResult, Tuple};

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
            Err(ParseError::new())
        }
    }
}

impl<T, U> std::ops::BitOr<T> for Literal<U>
where
    T: ParseElement<ParseOut = String>,
    U: AsRef<str>,
{
    type Output = Or<Literal<U>, T, String>;

    fn bitor(self, rhs: T) -> Self::Output {
        self.or(rhs)
    }
}

impl<T, U> std::ops::Add<T> for Literal<U>
where
    T: ParseElement,
    U: AsRef<str>,
{
    type Output = Tuple<Literal<U>, T>;

    fn add(self, rhs: T) -> Self::Output {
        self.tup(rhs)
    }
}

#[test]
fn literal() {
    let parser: Literal<_> = Literal("test");
    let result = parser.pars("test");
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.bytes_parsed, "test".len());
    assert_eq!(result.result, String::from("test"));
}

#[test]
fn test_or_operator() {
    let parser = Literal("Cat") | Literal("Dog");
    let expected: ParseResult<String> = Ok(ParseOk {
        bytes_parsed: 3,
        result: "Cat".into(),
    });
    assert_eq!(parser.pars("Cat"), expected);
}
