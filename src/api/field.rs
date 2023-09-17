use crate::managed_object::ManagedObject;
use crate::API;
use reframework_sys::*;

pub struct Field<'api> {
    pub(crate) api: &'api API,

    // lifetime is bound the API, this is therefore implicitly bound too
    pub(crate) handle: REFrameworkFieldHandle,
}

impl<'api> Field<'api> {
    pub(crate) fn s_get_data_mut<T>(
        api: &'api API,
        handle: REFrameworkFieldHandle,
        managed_object: &ManagedObject,
    ) -> Option<&'api mut T> {
        let field = api.sdk_field();
        // SAFETY: SDK is trusted
        let data = unsafe {
            field.get_data_raw.expect("not null")(handle, managed_object.handle.cast(), false)
        };

        if data.is_null() {
            return None;
        }

        // SAFETY: we cannot clearly determine the type, at least now, but it is not a null pointer
        Some(unsafe { &mut *data.cast() })
    }

    pub fn get_data_mut<T: 'api>(&self, managed_object: &ManagedObject) -> Option<&mut T> {
        Self::s_get_data_mut(self.api, self.handle, managed_object)
    }
}
