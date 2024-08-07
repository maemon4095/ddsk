pub trait IteratorTerminateWithExtension: Sized + Iterator
where
    Self::Item: PartialEq,
{
    fn terminate_with(self, termination: Self::Item) -> TerminateWith<Self> {
        TerminateWith {
            iter: self,
            termination: Some(termination),
        }
    }
}

impl<I> IteratorTerminateWithExtension for I
where
    I::Item: PartialEq,
    I: Iterator,
{
}

pub struct TerminateWith<I>
where
    I::Item: PartialEq,
    I: Iterator,
{
    iter: I,
    termination: Option<I::Item>,
}

impl<I> Iterator for TerminateWith<I>
where
    I::Item: PartialEq,
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(termination) = &self.termination else {
            return None;
        };
        let Some(item) = self.iter.next() else {
            return None;
        };

        if item == *termination {
            self.termination = None;
        }
        Some(item)
    }
}
