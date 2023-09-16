use crate::api::{Method, TDB};
use reframework_sys::bindings::REFrameworkMethodHandle__;
use std::ffi::CString;

impl<'api> TDB<'api> {
    pub fn find_method(&self, type_name: &str, name: &str) -> Option<Method> {
        let type_name = CString::new(type_name).expect("`type_name` is a valid C string");
        let name = CString::new(name).expect("`name` is a valid C String");

        let tdb = self.api.sdk_tdb();
        let method: *mut REFrameworkMethodHandle__ = unsafe {
            tdb.find_method.expect("not null")(self.handle, type_name.as_ptr(), name.as_ptr())
        };

        if method.is_null() {
            return None;
        }

        Some(Method {
            api: self.api,
            handle: method,
        })
    }
}
