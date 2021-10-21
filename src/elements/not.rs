use crate::{Or, ParseElement, ParseError, ParseOk, Tuple};

pub struct Not<T>(pub T);

impl<T> ParseElement for Not<T>
where
    T: ParseElement,
{
    type ParseOut = ();

    fn pars(&self, input: &str) -> crate::ParseResult<Self::ParseOut> {
        match self.0.pars(input) {
            Ok(_) => Err(ParseError::new()),
            Err(_) => Ok(ParseOk {
                bytes_parsed: 0,
                result: (),
            }),
        }
    }
}

impl<T, Rhs> std::ops::BitOr<Rhs> for Not<T>
where
    T: ParseElement,
    Rhs: ParseElement<ParseOut = ()>,
{
    type Output = Or<Not<T>, Rhs, ()>;

    fn bitor(self, rhs: Rhs) -> Self::Output {
        self.or(rhs)
    }
}

impl<T, Rhs> std::ops::Add<Rhs> for Not<T>
where
    T: ParseElement,
    Rhs: ParseElement,
{
    type Output = Tuple<Not<T>, Rhs>;

    fn add(self, rhs: Rhs) -> Self::Output {
        self.tup(rhs)
    }
}