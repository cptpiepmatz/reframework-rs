use crate::api::API_REF;
use crate::error::Error;
use crate::field::Field;
use crate::method::Method;
use crate::API;
use reframework_sys::*;
use std::ffi::{c_char, c_uint, CStr, CString};

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TypeDefinition {
    pub(crate) handle: REFrameworkTypeDefinitionHandle,
}

unsafe impl Sync for TypeDefinition {}
unsafe impl Send for TypeDefinition {}

impl TypeDefinition {
    pub fn get_name(&self) -> String {
        let api = API_REF.get().expect("is init");

        let type_definition = api.sdk_type_definition();

        let c_str = unsafe { type_definition.get_name.expect("not null")(self.handle) };
        // SAFETY: shouldn't be a null pointer
        let c_str = unsafe { CString::from_raw(c_str.cast_mut()) };

        String::from(c_str.to_string_lossy())
    }

    pub fn get_full_name(&self) -> Result<String, Error> {
        let mut buffer = [0; 512];
        let mut real_size: u32 = 0;

        let api = API_REF.get().expect("is init");
        let type_definition = api.sdk_type_definition();
        // SAFETY: SDK is trusted
        let res = unsafe {
            type_definition.get_full_name.expect("not null")(
                self.handle,
                buffer.as_mut_ptr(),
                buffer.len() as c_uint,
                &mut real_size,
            )
        };
        let res: Error = res.into();

        if res != Error::None {
            return Err(res);
        }

        // SAFETY: we assume that the C function put a null-terminated string in the buffer
        let c_str = unsafe { CStr::from_ptr(buffer.as_ptr()) };
        let mut s = c_str.to_string_lossy().to_string();
        s.truncate(real_size as usize);
        Ok(s)
    }

    pub fn find_method(&self, method_name: &str) -> Option<Method> {
        let api = API_REF.get().expect("is init");

        let method_name = CString::new(method_name).expect("is valid c string");

        let type_definition = api.sdk_type_definition();
        // SAFETY: SDK is trusted
        let handle = unsafe {
            type_definition.find_method.expect("not null")(self.handle, method_name.as_ptr())
        };

        if handle.is_null() {
            return None;
        }

        Some(Method { handle })
    }

    pub fn find_field(&self, field_name: &str) -> Option<Field> {
        let api = API_REF.get().expect("is init");

        let field_name = CString::new(field_name).expect("is valid c string");

        let type_definition = api.sdk_type_definition();
        // SAFETY: SDK is trusted
        let handle = unsafe {
            type_definition.find_field.expect("not null")(self.handle, field_name.as_ptr())
        };

        if handle.is_null() {
            return None;
        }

        Some(Field { handle })
    }
}
