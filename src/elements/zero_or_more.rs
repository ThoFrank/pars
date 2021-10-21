use crate::{Or, ParseElement, ParseOk, ParseResult, Tuple};

pub struct ZeroOrMore<T>
where
    T: ParseElement,
{
    elem: T,
}

impl<T: ParseElement> ZeroOrMore<T> {
    pub fn new(elem: T) -> ZeroOrMore<T> {
        ZeroOrMore { elem }
    }
}

impl<T, Out> ParseElement for ZeroOrMore<T>
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
        ret
    }
}

impl<T, Rhs> std::ops::BitOr<Rhs> for ZeroOrMore<T>
where
    Rhs: ParseElement<ParseOut = Vec<T::ParseOut>>,
    T: ParseElement,
{
    type Output = Or<ZeroOrMore<T>, Rhs, Vec<T::ParseOut>>;

    fn bitor(self, rhs: Rhs) -> Self::Output {
        self.or(rhs)
    }
}

impl<T, Rhs> std::ops::Add<Rhs> for ZeroOrMore<T>
where
    Rhs: ParseElement,
    T: ParseElement,
{
    type Output = Tuple<ZeroOrMore<T>, Rhs>;

    fn add(self, rhs: Rhs) -> Self::Output {
        self.tup(rhs)
    }
}

#[test]
fn zero_or_more() {
    use crate::Literal;
    let parser = ZeroOrMore::new(Literal("duck, ")).tup(Literal("duck"));
    let result = parser.pars("duck");
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.bytes_parsed, 4);
    assert_eq!(*result, (vec![], "duck".into()));

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
