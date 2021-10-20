use crate::{ParseElement, ParseError, ParseOk};

pub struct Not<T>(pub T);

impl<T> ParseElement for Not<T>
where
    T: ParseElement,
{
    type ParseOut = ();

    fn pars(&self, input: &str) -> crate::ParseResult<Self::ParseOut> {
        match self.0.pars(input) {
            Ok(_) => Err(ParseError {}),
            Err(_) => Ok(ParseOk {
                bytes_parsed: 0,
                result: (),
            }),
        }
    }
}
