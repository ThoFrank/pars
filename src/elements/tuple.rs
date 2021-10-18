use crate::{ParseElement, ParseOk, ParseResult};

pub struct Tuple<A: ParseElement + Sized, B: ParseElement> {
    pub first: A,
    pub second: B,
}

impl<A: ParseElement, B: ParseElement> ParseElement for Tuple<A, B> {
    type ParseOut = (A::ParseOut, B::ParseOut);

    fn pars(&self, input: &str) -> ParseResult<Self::ParseOut> {
        let first = self.first.pars(input)?;
        let second = self.second.pars(&input[first.bytes_parsed..])?;
        Ok(ParseOk {
            bytes_parsed: first.bytes_parsed + second.bytes_parsed,
            result: (first.result, second.result),
        })
    }
}

#[test]
fn tup() {
    let parser = "Hello ".tup("World!");
    let result = parser.pars("Hello World!");
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.bytes_parsed, "Hello World!".len());
    assert_eq!(
        result.result,
        (String::from("Hello "), String::from("World!"))
    );
}
