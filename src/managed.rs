use crate::api::API_REF;
use crate::invoke::{InvokeArg, InvokeResult, InvokeValue};
use crate::{debug, API};
use reframework_sys::REFrameworkManagedObjectHandle;
use std::ffi::{c_void, CString};
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::{any, mem, slice};

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ManagedSingleton(pub(crate) REFrameworkManagedObjectHandle);

unsafe impl Send for ManagedSingleton {}
unsafe impl Sync for ManagedSingleton {}

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ManagedString(pub(crate) REFrameworkManagedObjectHandle);

unsafe impl Send for ManagedString {}
unsafe impl Sync for ManagedString {}

impl From<String> for ManagedString {
    fn from(value: String) -> Self {
        // construct a CString and forget about it to leak it into the C code
        let c_string = CString::new(value).expect("should be a valid c string");
        let ptr = c_string.as_ptr();
        mem::forget(c_string);

        let api = API_REF.get().expect("should be init first");
        // SAFETY: SDK is trusted
        let managed_s = unsafe {
            api.sdk_functions()
                .create_managed_string_normal
                .expect("not null")(ptr)
        };

        Self(managed_s)
    }
}
