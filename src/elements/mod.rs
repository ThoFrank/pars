mod literal;
mod num;
mod or;
mod tuple;
mod zero_or_more;
mod optional;
use crate::result::ParseResult;
pub use literal::Literal;
pub use num::Integer;
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
        return Tuple {
            first: self,
            second: other,
        };
    }

    fn or<T>(self, other: T) -> Or<Self, T, Self::ParseOut>
    where
        Self: Sized,
        T: ParseElement<ParseOut = Self::ParseOut>,
    {
        return Or {
            opt_a: self,
            opt_b: other,
        };
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
