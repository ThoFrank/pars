use crate::{Or, ParseElement, ParseOk};

pub struct Optional<T: ParseElement> {
    elem: T,
}

impl<T: ParseElement> Optional<T> {
    pub fn new(elem: T) -> Optional<T> {
        Optional { elem }
    }
}

impl<T> ParseElement for Optional<T>
where
    T: ParseElement,
{
    type ParseOut = Option<T::ParseOut>;

    fn pars(&self, input: &str) -> crate::ParseResult<Self::ParseOut> {
        if let Ok(ParseOk {
            bytes_parsed,
            result,
        }) = self.elem.pars(input)
        {
            Ok(ParseOk {
                bytes_parsed,
                result: Some(result),
            })
        } else {
            Ok(ParseOk {
                bytes_parsed: 0,
                result: None,
            })
        }
    }
}

impl<T, Rhs> std::ops::BitOr<Rhs> for Optional<T>
where
    T: ParseElement,
    Rhs: ParseElement<ParseOut = Option<T::ParseOut>>
{
    type Output = Or<Optional<T>, Rhs, Rhs::ParseOut>;

    fn bitor(self, rhs: Rhs) -> Self::Output {
        self.or(rhs)
    }
}

#[test]
fn optional() {
    use crate::Literal;
    let parser = Optional::new(Literal("Maybe"));

    let result = parser.pars("");
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(*result, None);
    assert_eq!(result.bytes_parsed, 0);

    let result = parser.pars("Maybe");
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(*result, Some("Maybe".into()));
    assert_eq!(result.bytes_parsed, 5);
}
