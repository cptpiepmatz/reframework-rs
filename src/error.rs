use reframework_sys::REFrameworkResult;
use std::any;
use std::error;
use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;

#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    Unknown = -1,
    /// No Error
    #[deprecated]
    None = 0,
    OutTooSmall = 1,
    Exception = 2,
    InArgsSizeMismatch = 3,
}

#[doc(hidden)]
impl From<REFrameworkResult> for Error {
    fn from(value: REFrameworkResult) -> Self {
        match value {
            -1 => Self::Unknown,
            0 => Self::None,
            1 => Self::OutTooSmall,
            2 => Self::Exception,
            3 => Self::InArgsSizeMismatch,
            _ => unimplemented!(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Unknown => write!(f, "unknown REFramework error"),
            Error::None => write!(f, "no REFramework error"),
            Error::OutTooSmall => write!(f, "out-buffer was too small"),
            Error::Exception => write!(f, "some exception occurred"),
            Error::InArgsSizeMismatch => write!(f, "input arguments size mismatch"),
        }
    }
}

impl error::Error for Error {}

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

impl<P> error::Error for NullPtrError<P> {}
