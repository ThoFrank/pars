use crate::{Or, ParseElement, ParseOk, ParseResult};

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

impl<T, F, Rhs, Out> std::ops::BitOr<Rhs> for Mapped<T, F>
where
    T: ParseElement,
    Rhs: ParseElement<ParseOut = Out>,
    F: Fn(T::ParseOut) -> Out
{
    type Output = Or<Mapped<T, F>, Rhs, Out>;

    fn bitor(self, rhs: Rhs) -> Self::Output {
        self.or(rhs)
    }
}

#[test]
fn map() {
    use crate::Literal;
    let parser = Literal("Hello, ")
        .tup(Literal("World!"))
        .map(|(s1, s2): (String, String)| s1 + &s2);
    let ParseOk {
        bytes_parsed,
        result,
        ..
    } = parser.pars("Hello, World!").unwrap();
    assert_eq!(bytes_parsed, "Hello, World!".len());
    assert_eq!(result, String::from("Hello, World!"))
}
