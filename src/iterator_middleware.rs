pub trait IteratorMiddleware {
    type InputItem;
    type OutputItem;
    fn next(&mut self, iter: impl Iterator<Item = Self::InputItem>) -> Option<Self::OutputItem>;
}

pub struct WithMiddleware<I: Iterator, M: IteratorMiddleware> {
    iter: I,
    middleware: M,
}

impl<I, M> Iterator for WithMiddleware<I, M>
where
    I: Iterator,
    M: IteratorMiddleware<InputItem = I::Item>,
{
    type Item = M::OutputItem;

    fn next(&mut self) -> Option<Self::Item> {
        self.middleware.next(&mut self.iter)
    }
}

pub trait IteratorWithMiddlewareExtension: Sized + Iterator {
    fn with_middleware<M: IteratorMiddleware>(self, middleware: M) -> WithMiddleware<Self, M> {
        WithMiddleware {
            iter: self,
            middleware,
        }
    }
}
impl<I: Iterator> IteratorWithMiddlewareExtension for I {}
