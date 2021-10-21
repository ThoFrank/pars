use crate::{Or, ParseElement, ParseError, ParseOk, ParseResult};

pub struct OneOrMore<T>
where
    T: ParseElement,
{
    elem: T,
}

impl<T: ParseElement> OneOrMore<T> {
    pub fn new(elem: T) -> OneOrMore<T> {
        OneOrMore { elem }
    }
}

impl<T, Out> ParseElement for OneOrMore<T>
where
    T: ParseElement<ParseOut = Out>,
{
    type ParseOut = Vec<Out>;

    fn pars(&self, input: &str) -> ParseResult<Self::ParseOut> {
        let mut ret: ParseResult<Vec<Out>> = Ok(ParseOk {
            bytes_parsed: 0,
            result: vec![],
        });
        let mut input = input;
        while let Ok(current_result) = self.elem.pars(input) {
            let ParseOk {
                bytes_parsed,
                mut result,
                ..
            } = ret.unwrap();
            result.push(current_result.result);
            ret = Ok(ParseOk {
                bytes_parsed: current_result.bytes_parsed + bytes_parsed,
                result,
            });

            input = &input[current_result.bytes_parsed..]
        }
        let ret = ret.unwrap();
        if ret.result.len() == 0 {
            Err(ParseError {
                r#type: crate::result::ErrorType::ParsOnUninitialized,
            })
        } else {
            Ok(ret)
        }
    }
}

impl<T, Rhs> std::ops::BitOr<Rhs> for OneOrMore<T>
where
    Rhs: ParseElement<ParseOut = Vec<T::ParseOut>>,
    T: ParseElement,
{
    type Output = Or<OneOrMore<T>, Rhs, Vec<T::ParseOut>>;

    fn bitor(self, rhs: Rhs) -> Self::Output {
        self.or(rhs)
    }
}

#[test]
fn one_or_more() {
    use crate::Literal;
    let parser = OneOrMore::new(Literal("duck, ")).tup(Literal("duck"));
    let result = parser.pars("duck");
    assert!(result.is_err());

    let result = parser.pars("duck, duck");
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.bytes_parsed, 10);
    assert_eq!(*result, (vec!["duck, ".into()], "duck".into()));

    let result = parser.pars("duck, duck, duck, duck, duck");
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.bytes_parsed, 28);
    assert_eq!(*result, (vec!["duck, ".into(); 4], "duck".into()));
}
