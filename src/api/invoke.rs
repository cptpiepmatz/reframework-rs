use crate::error::Error;
use crate::managed::{ManagedSingleton, ManagedString};
use crate::managed_object::ManagedObject;
use crate::private::Sealed;
use crate::NativeSingleton;
use reframework_sys::REFrameworkManagedObjectHandle;
use std::ffi::c_void;
use std::fmt::{Debug, Display, Formatter};
use std::{fmt, mem};

#[derive(Debug)]
pub enum InvokeResult<T: From<InvokeValue>> {
    Ok(T),
    Err(Error),

    /// Exception occurred while invoking method.
    ///
    /// This variant is different than the [Error::Exception] in terms of it's origin.
    /// [Error::Exception] is a return value by the underlying `invoke` function.
    /// If that function returns a value that is not [Error::None], that error will returned.
    ///
    /// If this value is [Error::None] but the returned [InvokeRet] has `exception_thrown == true`,
    /// then this variant will be used.
    ///
    /// It is unclear which variant will occur.
    ExceptionOccurred,
}

impl<T> InvokeResult<T>
where
    T: From<InvokeValue>,
{
    pub fn unwrap(self) -> T {
        Result::from(self).unwrap()
    }

    pub fn expect(self, msg: &str) -> T {
        Result::from(self).expect(msg)
    }
}

impl<T> From<InvokeResult<T>> for Result<T, Error>
where
    T: From<InvokeValue>,
{
    fn from(value: InvokeResult<T>) -> Self {
        match value {
            InvokeResult::Ok(v) => Result::Ok(v),
            InvokeResult::Err(e) => Result::Err(e),
            InvokeResult::ExceptionOccurred => Result::Err(Error::Exception),
        }
    }
}

#[repr(C)]
pub union InvokeValue {
    pub as_bytes: [u8; 128],
    pub as_byte: u8,
    pub as_u16: u16,
    pub as_u32: u32,
    pub as_f32: f32,
    pub as_u64: u64,
    pub as_f64: f64,
    pub as_mut_ptr: *mut std::ffi::c_void,
}

impl Debug for InvokeValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("InvokeValue");

        unsafe {
            let truncated_bytes = TruncatedBytes(&self.as_bytes);
            s.field("as_bytes", &truncated_bytes);
            s.field("as_byte", &self.as_byte);
            s.field("as_u16", &self.as_u16);
            s.field("as_u32", &self.as_u32);
            s.field("as_f32", &self.as_f32);
            s.field("as_u64", &self.as_u64);
            s.field("as_f64", &self.as_f64);
            s.field("as_mut_ptr", &self.as_mut_ptr);
        }

        s.finish()
    }
}

impl Default for InvokeValue {
    fn default() -> Self {
        Self { as_bytes: [0; 128] }
    }
}

impl From<InvokeValue> for Option<ManagedObject> {
    fn from(value: InvokeValue) -> Self {
        // SAFETY: this is not safe and we have no way to back this up, we just hope that the
        //         caller knows what type should come out
        let ptr = unsafe { value.as_mut_ptr };

        if ptr.is_null() {
            return None;
        }

        let handle: REFrameworkManagedObjectHandle = ptr.cast();
        Some(ManagedObject { handle })
    }
}

impl From<InvokeValue> for bool {
    fn from(value: InvokeValue) -> Self {
        unsafe { value.as_byte != 0 }
    }
}

impl From<InvokeValue> for usize {
    fn from(value: InvokeValue) -> Self {
        unsafe { value.as_u64 as usize }
    }
}

impl From<InvokeValue> for u32 {
    fn from(value: InvokeValue) -> Self {
        unsafe { value.as_u32 }
    }
}

impl From<InvokeValue> for i32 {
    fn from(value: InvokeValue) -> Self {
        unsafe { mem::transmute(value.as_u32) }
    }
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct InvokeRet {
    pub(crate) value: InvokeValue,
    pub(crate) exception_thrown: bool,
}

struct TruncatedBytes<'b>(&'b [u8]);

impl<'b> Debug for TruncatedBytes<'b> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for b in self.0[0..4].iter() {
            write!(f, "{b}, ")?;
        }
        write!(f, "...]")
    }
}

pub trait InvokeObj: InvokeArg {}
impl InvokeObj for *mut c_void {}
impl InvokeObj for ManagedObject {}
impl InvokeObj for NativeSingleton {}
impl InvokeObj for ManagedSingleton {}

pub trait InvokeArg: Sealed {
    /// Get internal pointer.
    ///
    /// # Safety
    ///
    /// All non-primitive implementors of this trait only hold a single pointer.
    /// This pointer will be returned here.
    unsafe fn as_mut_ptr(&mut self) -> *mut c_void;
}

impl InvokeArg for *mut c_void {
    unsafe fn as_mut_ptr(&mut self) -> *mut c_void {
        *self
    }
}

impl InvokeArg for ManagedObject {
    unsafe fn as_mut_ptr(&mut self) -> *mut c_void {
        self.handle.cast()
    }
}

impl InvokeArg for NativeSingleton {
    unsafe fn as_mut_ptr(&mut self) -> *mut c_void {
        self.0.cast()
    }
}

impl InvokeArg for ManagedSingleton {
    unsafe fn as_mut_ptr(&mut self) -> *mut c_void {
        self.0.cast()
    }
}

impl InvokeArg for ManagedString {
    unsafe fn as_mut_ptr(&mut self) -> *mut c_void {
        self.0.cast()
    }
}

impl InvokeArg for i32 {
    unsafe fn as_mut_ptr(&mut self) -> *mut c_void {
        let ptr: *mut i32 = self;
        ptr.cast()
    }
}

impl InvokeArg for u32 {
    unsafe fn as_mut_ptr(&mut self) -> *mut c_void {
        let ptr: *mut u32 = self;
        ptr.cast()
    }
}

impl InvokeArg for usize {
    unsafe fn as_mut_ptr(&mut self) -> *mut c_void {
        let ptr: *mut usize = self;
        ptr.cast()
    }
}

// TODO: implement this in a correct way for primitives
