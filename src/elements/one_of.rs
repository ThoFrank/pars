use crate::{Or, ParseElement, ParseError};

pub struct OneOf<T>(Vec<T>);

impl<T> OneOf<T>
where
    T: ParseElement,
{
    pub fn new(iter: impl IntoIterator<Item = T>) -> Self {
        OneOf(iter.into_iter().collect())
    }
}

impl<T, Out> ParseElement for OneOf<T>
where
    T: ParseElement<ParseOut = Out>,
{
    type ParseOut = Out;

    fn pars(&self, input: &str) -> crate::ParseResult<Self::ParseOut> {
        for parser in &self.0 {
            let result = parser.pars(input);
            if result.is_ok() {
                return result;
            }
        }
        Err(ParseError {})
    }
}

impl<T, Rhs, Out> std::ops::BitOr<Rhs> for OneOf<T>
where
    Rhs: ParseElement<ParseOut = Out>,
    T: ParseElement<ParseOut = Out>,
{
    type Output = Or<OneOf<T>, Rhs, Out>;

    fn bitor(self, rhs: Rhs) -> Self::Output {
        self.or(rhs)
    }
}

#[test]
fn one_of() {
    //let parser = OneOf::new(('a'..'z').map(|c| ))
}
