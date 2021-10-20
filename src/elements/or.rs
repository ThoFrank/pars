use crate::ParseElement;

pub struct Or<A, B, Out>
where
    A: ParseElement<ParseOut = Out>,
    B: ParseElement<ParseOut = Out>,
{
    pub opt_a: A,
    pub opt_b: B,
}

impl<A, B, Out> ParseElement for Or<A, B, Out>
where
    A: ParseElement<ParseOut = Out>,
    B: ParseElement<ParseOut = Out>,
{
    type ParseOut = Out;

    fn pars(&self, input: &str) -> crate::ParseResult<Self::ParseOut> {
        if let Ok(result) = self.opt_a.pars(input) {
            Ok(result)
        } else {
            self.opt_b.pars(input)
        }
    }
}

#[test]
fn or() {
    use crate::Literal;
    let parser = Literal("A").or(Literal("B"));
    let result = parser.pars("A");
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(*result, "A");
    assert_eq!(result.bytes_parsed, 1);

    let result = parser.pars("B");
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(*result, "B");
    assert_eq!(result.bytes_parsed, 1);
}
