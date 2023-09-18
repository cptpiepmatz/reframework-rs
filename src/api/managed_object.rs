use crate::api::API_REF;
use crate::field::Field;
use crate::invoke::{InvokeArg, InvokeResult, InvokeValue};
use crate::type_definition::TypeDefinition;
use reframework_sys::*;

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ManagedObject {
    pub(crate) handle: REFrameworkManagedObjectHandle,
}

unsafe impl Sync for ManagedObject {}
unsafe impl Send for ManagedObject {}

impl ManagedObject {
    pub fn get_type_definition(&self) -> Option<TypeDefinition> {
        let api = API_REF.get().expect("is init");

        let managed_object = api.sdk_managed_object();
        // SAFETY: SDK is trusted
        let handle = unsafe { managed_object.get_type_definition.expect("not null")(self.handle) };

        if handle.is_null() {
            return None;
        }

        Some(TypeDefinition { handle })
    }

    pub fn get_field_mut<T>(&self, field_name: &str) -> Option<&mut T> {
        let type_definition = self.get_type_definition()?;
        let field = type_definition.find_field(field_name)?;
        Field::s_get_data_mut(field.handle, self)
    }

    // pub fn invoke<T>(
    //     &self,
    //     method_name: &str,
    //     args: &mut [&dyn InvokeArg],
    // ) -> Option<InvokeResult<T>>
    // where
    //     T: From<InvokeValue>,
    // {
    //     let type_definition = self.get_type_definition()?;
    //     let method = type_definition.find_method(method_name)?;
    //     Some(method.invoke(self.handle, args))
    // }
}
