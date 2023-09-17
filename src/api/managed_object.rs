use crate::API;
use reframework_sys::*;

pub struct ManagedObject<'api> {
    pub(crate) api: &'api API,

    // lifetime is bound the API, this is therefore implicitly bound too
    pub(crate) handle: REFrameworkManagedObjectHandle,
}

