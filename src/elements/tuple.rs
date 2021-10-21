use crate::{Or, ParseElement, ParseOk, ParseResult};

pub struct Tuple<A, B> {
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

impl<A, B, Rhs> std::ops::BitOr<Rhs> for Tuple<A, B>
where
    A: ParseElement,
    B: ParseElement,
    Rhs: ParseElement<ParseOut = (A::ParseOut, B::ParseOut)>,
{
    type Output = Or<Tuple<A, B>, Rhs, Rhs::ParseOut>;

    fn bitor(self, rhs: Rhs) -> Self::Output {
        self.or(rhs)
    }
}

#[test]
fn tup() {
    use crate::Literal;
    let parser = Literal("Hello ").tup(Literal("World!"));
    let result = parser.pars("Hello World!");
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.bytes_parsed, "Hello World!".len());
    assert_eq!(
        result.result,
        (String::from("Hello "), String::from("World!"))
    );
}
