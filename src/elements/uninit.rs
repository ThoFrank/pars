use std::{cell::RefCell, rc::Rc};

use crate::ParseElement;

#[derive(Clone)]
pub struct Uninit<Out>(
    Rc<RefCell<Option<Box<dyn ParseElement<ParseOut = Out>>>>>
);

impl<Out: Sized> Uninit<Out> {
    pub fn new() -> Self{
        Uninit(Rc::default())
    }

    pub fn init(&self, val: impl ParseElement<ParseOut = Out> + 'static)
    {
        *self.0.borrow_mut() = Some(Box::new(val));
    }
}

impl<Out> ParseElement for Uninit<Out>
{
    type ParseOut = Out;

    fn pars(&self, input: &str) -> crate::ParseResult<Self::ParseOut> {
        match &*self.0.borrow(){
            Some(elem) => elem.pars(input),
            None => unreachable!("Execution of this means a Grammar rule was not initialized!"),
        }
    }
}