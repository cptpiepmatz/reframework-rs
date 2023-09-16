use std::ffi::{OsStr, OsString};
use std::os::windows::ffi::{EncodeWide, OsStrExt};
use std::sync::OnceLock;
use winapi::um::debugapi::OutputDebugStringW;

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{
        use std::fmt::Write;

        let mut s = std::ffi::OsString::new();
        write!(s, $($arg)*).expect("infallible");
        $crate::debug::output_debug_string(&s);
    }};
}

use crate::api::API_REF;
pub(crate) use debug;

static DEBUG_PREFIX: OnceLock<EncodeWide> = OnceLock::new();

/// Wrapper of [OutputDebugStringW].
///
/// To be used via the [debug!] macro.
pub fn output_debug_string(s: &OsStr) {
    let s = s.encode_wide();

    let buffer: Vec<u16> = match (DEBUG_PREFIX.get(), API_REF.get()) {
        (Some(prefix), _) => prefix.clone().chain(s).chain(Some(0)).collect(),
        (None, None) => s.chain(Some(0)).collect(),
        (None, Some(api)) => {
            let name = api.name;
            let prefix = format!("[REF: {name}] ");
            let prefix: &'static str = prefix.leak();
            let prefix = OsStr::new(prefix);
            let prefix = prefix.encode_wide();

            let prefix = match DEBUG_PREFIX.set(prefix) {
                Ok(_) => DEBUG_PREFIX.get().expect("just set").clone(),
                Err(v) => v,
            };

            prefix.chain(s).chain(Some(0)).collect()
        }
    };

    // SAFETY: this string is validly prepared
    unsafe { OutputDebugStringW(buffer.as_ptr()) }
}
