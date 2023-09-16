#![allow(clippy::not_unsafe_ptr_arg_deref)] // all safety checks are internally

#[cfg(not(windows))]
compile_error!("This crate can only be compiled on Windows.");

use reframework_sys::bindings::*;
use std::ffi::c_int;

pub mod api;
pub use api::*;
pub mod error;

#[doc(hidden)]
pub mod debug;

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
