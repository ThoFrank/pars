mod literal;
mod mapped;
mod num;
mod optional;
mod or;
mod tuple;
mod zero_or_more;
use crate::result::ParseResult;
pub use literal::Literal;
pub use mapped::Mapped;
pub use num::Integer;
pub use optional::Optional;
pub use or::Or;
pub use tuple::Tuple;
pub use zero_or_more::ZeroOrMore;
pub trait ParseElement {
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

/*
impl<T, Out> std::ops::BitOr for T
where
    T: ParseElement<ParseOut = Out>,
{
    type Output = Or<Self, Self, Out>;

    fn bitor(self, rhs: Self) -> Self::Output {
        todo!()
    }
}
*/
