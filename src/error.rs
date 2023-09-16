use std::any;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;

pub struct NullPtrError<P>(PhantomData<P>);

impl<P> NullPtrError<P> {
    pub fn new() -> Self {
        NullPtrError(PhantomData)
    }
}

impl<P> Default for NullPtrError<P> {
    fn default() -> Self {
        Self::new()
    }
}

impl<P> Debug for NullPtrError<P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "NullPtrError({})", any::type_name::<P>())
    }
}

impl<P> Display for NullPtrError<P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "pointer to {} was a null pointer", any::type_name::<P>())
    }
}

impl<P> Error for NullPtrError<P> {}
