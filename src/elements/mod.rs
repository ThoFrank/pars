mod literal;
mod mapped;
mod not;
mod num;
mod one_of;
mod one_or_more;
mod optional;
mod or;
mod tuple;
mod uninit;
mod zero_or_more;
mod any;
use crate::result::ParseResult;
pub use literal::Literal;
pub use mapped::Mapped;
pub use not::Not;
pub use num::Integer;
pub use one_of::OneOf;
pub use one_or_more::OneOrMore;
pub use optional::Optional;
pub use or::Or;
pub use tuple::Tuple;
pub use uninit::Uninit;
pub use zero_or_more::ZeroOrMore;
pub use any::Any;
pub trait ParseElement: {
    type ParseOut;

    fn pars(&self, input: &str) -> ParseResult<Self::ParseOut>;

    fn tup<T: ParseElement>(self, other: T) -> Tuple<Self, T>
    where
        Self: Sized,
    {
        Tuple {
            first: self,
            second: other,
        }
    }

    fn or<T>(self, other: T) -> Or<Self, T, Self::ParseOut>
    where
        Self: Sized,
        T: ParseElement<ParseOut = Self::ParseOut>,
    {
        Or {
            opt_a: self,
            opt_b: other,
        }
    }

    fn map<F>(self, func: F) -> Mapped<Self, F>
    where
        Self: Sized,
    {
        Mapped {
            element: self,
            func,
        }
    }
}

impl<T> ParseElement for &T
where
    T: ParseElement,
{
    type ParseOut = T::ParseOut;

    fn pars(&self, input: &str) -> ParseResult<Self::ParseOut> {
        T::pars(&self, input)
    }
}
