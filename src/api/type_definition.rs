use crate::field::Field;
use crate::API;
use reframework_sys::*;
use std::ffi::{CStr, CString};

pub struct TypeDefinition<'api> {
    pub(crate) api: &'api API,

    // lifetime is bound the API, this is therefore implicitly bound too
    pub(crate) handle: REFrameworkTypeDefinitionHandle,
}

impl<'api> TypeDefinition<'api> {
    pub fn find_field(&self, field_name: &str) -> Option<Field> {
        let field_name = CString::new(field_name).expect("`field_name` has a 0 byte");

        let type_definition = self.api.sdk_type_definition();
        // SAFETY: SDK is trusted
        let handle = unsafe {
            type_definition.find_field.expect("not null")(self.handle, field_name.as_ptr())
        };

        if handle.is_null() {
            return None;
        }

        Some(Field {
            api: self.api,
            handle,
        })
    }
}
