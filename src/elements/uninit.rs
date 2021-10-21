use std::{cell::RefCell, rc::{Rc, Weak}};

use crate::{Or, ParseElement};

pub struct Uninit<Out>(Rc<RefCell<Option<Box<dyn ParseElement<ParseOut = Out>>>>>);
pub struct UnInitRef<Out>(Weak<RefCell<Option<Box<dyn ParseElement<ParseOut = Out>>>>>);

impl<Out: Sized> Uninit<Out> {
    pub fn new() -> Self {
        Uninit(Rc::default())
    }

    pub fn init(&self, val: impl ParseElement<ParseOut = Out> + 'static) {
        *self.0.borrow_mut() = Some(Box::new(val));
    }
}

impl<Out> Uninit<Out>{
    pub fn weak(&self) -> UnInitRef<Out> {
        UnInitRef(Rc::downgrade(&self.0))
    }
}

impl<Out, Rhs> std::ops::BitOr<Rhs> for Uninit<Out>
where
    Rhs: ParseElement<ParseOut = Out>,
{
    type Output = Or<Uninit<Out>, Rhs, Out>;

    fn bitor(self, rhs: Rhs) -> Self::Output {
        self.or(rhs)
    }
}

impl<Out> ParseElement for Uninit<Out> {
    type ParseOut = Out;

    fn pars(&self, input: &str) -> crate::ParseResult<Self::ParseOut> {
        match &*self.0.borrow() {
            Some(elem) => elem.pars(input),
            None => unreachable!("Execution of this means a Grammar rule was not initialized!"),
        }
    }
}

impl<Out> ParseElement for UnInitRef<Out> {
    type ParseOut = Out;

    fn pars(&self, input: &str) -> crate::ParseResult<Self::ParseOut> {
        match &*(self.0.upgrade().unwrap()).borrow() {
            Some(elem) => elem.pars(input),
            None => unreachable!("Execution of this means a Grammar rule was not initialized!"),
        }
    }
}
