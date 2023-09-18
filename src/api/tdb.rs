use crate::api::method::Method;
use crate::api::API_REF;
use crate::type_definition::TypeDefinition;
use crate::API;
use reframework_sys::*;
use std::ffi::CString;

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TDB {
    pub(crate) handle: REFrameworkTDBHandle,
}

unsafe impl Sync for TDB {}
unsafe impl Send for TDB {}

impl TDB {
    pub fn find_type(&self, type_name: &str) -> Option<TypeDefinition> {
        let api = API_REF.get().expect("is init");
        let tdb = api.sdk_tdb();

        let type_name = CString::new(type_name).expect("`type_name` is a valid C string");

        let handle = unsafe { tdb.find_type.expect("not null")(self.handle, type_name.as_ptr()) };

        if handle.is_null() {
            return None;
        }

        Some(TypeDefinition { handle })
    }

    pub fn find_method(&self, type_name: &str, name: &str) -> Option<Method> {
        let api = API_REF.get().expect("is init");

        let type_name = CString::new(type_name).expect("`type_name` is a valid C string");
        let name = CString::new(name).expect("`name` is a valid C String");

        let tdb = api.sdk_tdb();
        let method: *mut REFrameworkMethodHandle__ = unsafe {
            tdb.find_method.expect("not null")(self.handle, type_name.as_ptr(), name.as_ptr())
        };

        if method.is_null() {
            return None;
        }

        Some(Method { handle: method })
    }
}
