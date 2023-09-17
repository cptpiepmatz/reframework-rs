use crate::field::Field;
use crate::type_definition::TypeDefinition;
use crate::API;
use reframework_sys::*;

pub struct ManagedObject<'api> {
    pub(crate) api: &'api API,

    // lifetime is bound the API, this is therefore implicitly bound too
    pub(crate) handle: REFrameworkManagedObjectHandle,
}

impl<'api> ManagedObject<'api> {
    pub fn get_type_definition(&self) -> Option<TypeDefinition> {
        let managed_object = self.api.sdk_managed_object();
        // SAFETY: SDK is trusted
        let handle = unsafe { managed_object.get_type_definition.expect("not null")(self.handle) };

        if handle.is_null() {
            return None;
        }

        Some(TypeDefinition {
            api: self.api,
            handle,
        })
    }

    pub fn get_field_mut<T>(&self, field_name: &str) -> Option<&mut T> {
        let type_definition = self.get_type_definition()?;
        let field = type_definition.find_field(field_name)?;
        Field::s_get_data_mut(self.api, field.handle, self)
    }
}
