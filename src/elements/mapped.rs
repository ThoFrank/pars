use crate::{ParseElement, ParseOk, ParseResult};

pub struct Mapped<T, F> {
    pub(crate) element: T,
    pub(crate) func: F,
}

impl<T, Out, F> ParseElement for Mapped<T, F>
where
    T: ParseElement,
    F: Fn(T::ParseOut) -> Out,
{
    type ParseOut = Out;

    fn pars(&self, input: &str) -> ParseResult<Self::ParseOut> {
        let ParseOk {
            bytes_parsed,
            result,
            ..
        } = self.element.pars(input)?;
        Ok(ParseOk {
            bytes_parsed,
            result: (self.func)(result),
        })
    }
}

#[test]
fn map() {
    let parser = "Hello, "
        .tup("World!")
        .map(|(s1, s2): (String, String)| s1 + &s2);
    let ParseOk {
        bytes_parsed,
        result,
        ..
    } = parser.pars("Hello, World!").unwrap();
    assert_eq!(bytes_parsed, "Hello, World!".len());
    assert_eq!(result, String::from("Hello, World!"))
}
