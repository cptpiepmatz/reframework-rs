use crate::api::API_REF;
use crate::managed_object::ManagedObject;
use reframework_sys::*;

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct Field {
    pub(crate) handle: REFrameworkFieldHandle,
}

unsafe impl Sync for Field {}
unsafe impl Send for Field {}

impl Field {
    pub(crate) fn s_get_data_mut<'d, T>(
        handle: REFrameworkFieldHandle,
        managed_object: &ManagedObject,
    ) -> Option<&'d mut T> {
        let api = API_REF.get().expect("is init");

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

    pub fn get_data_mut<T>(&self, managed_object: &ManagedObject) -> Option<&mut T> {
        Self::s_get_data_mut(self.handle, managed_object)
    }
}
