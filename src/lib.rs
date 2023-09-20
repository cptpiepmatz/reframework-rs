#![allow(clippy::not_unsafe_ptr_arg_deref)] // all safety checks are internally
#![deny(improper_ctypes_definitions)]

#[cfg(not(all(target_os = "windows", target_pointer_width = "64")))]
compile_error!("This crate can only be compiled on 64-Bit Windows.");

use reframework_sys::*;
use std::ffi::{c_int, c_void};

pub mod api;
pub use api::*;

pub mod error;
pub mod managed;

#[doc(hidden)]
pub mod debug;

#[repr(transparent)]
pub struct PluginVersion(REFrameworkPluginVersion);

pub fn plugin_required_version(version: *mut PluginVersion) {
    // SAFETY: This function is always called by the surrounding framework, which guarantees
    //         that the passed pointer is valid.
    //         Additionally, the cast is safe because the data layout of PluginVersion and
    //         REFrameworkPluginVersion is the same.
    let version: &mut REFrameworkPluginVersion =
        unsafe { &mut *version.cast::<REFrameworkPluginVersion>() };

    version.major = REFRAMEWORK_PLUGIN_VERSION_MAJOR as c_int;
    version.minor = REFRAMEWORK_PLUGIN_VERSION_MINOR as c_int;
    version.patch = REFRAMEWORK_PLUGIN_VERSION_PATCH as c_int;
}

#[macro_export]
macro_rules! plugin_required_version {
    () => {
        #[no_mangle]
        #[allow(clippy::not_unsafe_ptr_arg_deref)]
        pub extern "C" fn reframework_plugin_required_version(version: *mut $crate::PluginVersion) {
            $crate::plugin_required_version(version)
        }
    };
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VMContext(pub(crate) REFrameworkVMContextHandle);

unsafe impl Send for VMContext {}
unsafe impl Sync for VMContext {}

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NativeSingleton(pub(crate) *mut c_void);

unsafe impl Send for NativeSingleton {}
unsafe impl Sync for NativeSingleton {}

mod private {
    pub trait Sealed {}
    impl<T> Sealed for T {}
}
