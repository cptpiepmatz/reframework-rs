use reframework_sys::REFrameworkManagedObjectHandle__;
use crate::API;

pub struct ManagedObject<'api> {
    pub(crate) api: &'api API,

    // lifetime is bound the API, this is therefore implicitly bound too
    pub(crate) handle: *mut REFrameworkManagedObjectHandle__,
}

