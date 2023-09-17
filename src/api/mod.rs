use crate::error::NullPtrError;
use reframework_sys::bindings::*;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::sync::OnceLock;

pub mod hook;
pub mod managed_object;
pub mod method;
pub mod tdb;

pub(crate) static API_REF: OnceLock<API> = OnceLock::new();

// TODO: make this Debug
#[derive(Clone)]
pub struct API {
    pub name: &'static str,
    pub param: &'static REFrameworkPluginInitializeParam,
    // TODO: remove this `pub`
    pub sdk: &'static REFrameworkSDKData,
    // TODO: handle the `m_lua_mtx` field, if necessary
}

// TODO: remove this or make sure this is safe
unsafe impl Sync for API {}

unsafe impl Send for API {}

pub struct TDB<'api> {
    api: &'api API,

    // lifetime is bound the API, this is therefore implicitly bound too
    pub handle: *mut REFrameworkTDBHandle__,
}

pub struct Method<'api> {
    api: &'api API,

    // lifetime is bound the API, this is therefore implicitly bound too
    pub handle: *mut REFrameworkMethodHandle__,
}

pub struct MethodParameter<'api> {
    api: &'api API,

    // lifetime is bound the API, this is therefore implicitly bound too
    type_definition_handle: *mut REFrameworkTypeDefinitionHandle,
}

pub struct ManagedObject<'api> {
    api: &'api API,

    // lifetime is bound the API, this is therefore implicitly bound too
    handle: *mut REFrameworkManagedObjectHandle__,
}

pub struct PluginInitializeParam(REFrameworkPluginInitializeParam);

impl API {
    pub fn initialize(
        name: &'static str,
        param: *const PluginInitializeParam,
    ) -> Result<Self, APIInitError> {
        if param.is_null() {
            return Err(APIInitError::ParamIsNull(NullPtrError::new()));
        }

        if API_REF.get().is_some() {
            return Err(APIInitError::AlreadyInitialized);
        }

        // SAFETY: we checked that this is not null, also this new-type has the same memory layout
        let param: &REFrameworkPluginInitializeParam = unsafe { &(*param.cast()) };

        // SAFETY: if param is not null, then this is also not null
        let sdk = unsafe { &(*param.sdk) };

        let api = Self { name, param, sdk };
        let _ = API_REF.set(api.clone());
        crate::debug::debug!("Initialized Plugin");

        Ok(api)
    }

    pub fn tdb(&self) -> TDB {
        let functions = unsafe { &(*self.sdk.functions) };
        let handle = unsafe { functions.get_tdb.expect("not null")() };

        TDB { api: self, handle }
    }
}

impl API {
    #[inline]
    pub(crate) fn sdk_functions(&self) -> &REFrameworkSDKFunctions {
        // SAFETY: SDK is trusted
        unsafe { &*self.sdk.functions }
    }

    #[inline]
    pub(crate) fn sdk_tdb(&self) -> &REFrameworkTDB {
        // SAFETY: SDK is trusted
        unsafe { &*self.sdk.tdb }
    }

    #[inline]
    pub(crate) fn sdk_type_definition(&self) -> &REFrameworkTDBTypeDefinition {
        // SAFETY: SDK is trusted
        unsafe { &*self.sdk.type_definition }
    }

    #[inline]
    pub(crate) fn sdk_method(&self) -> &REFrameworkTDBMethod {
        // SAFETY: SDK is trusted
        unsafe { &*self.sdk.method }
    }

    #[inline]
    pub(crate) fn sdk_field(&self) -> &REFrameworkTDBField {
        // SAFETY: SDK is trusted
        unsafe { &*self.sdk.field }
    }

    #[inline]
    pub(crate) fn sdk_property(&self) -> &REFrameworkTDBProperty {
        // SAFETY: SDK is trusted
        unsafe { &*self.sdk.property }
    }

    #[inline]
    pub(crate) fn sdk_managed_object(&self) -> &REFrameworkManagedObject {
        // SAFETY: SDK is trusted
        unsafe { &*self.sdk.managed_object }
    }

    #[inline]
    pub(crate) fn sdk_resource_manager(&self) -> &REFrameworkResourceManager {
        // SAFETY: SDK is trusted
        unsafe { &*self.sdk.resource_manager }
    }

    #[inline]
    pub(crate) fn sdk_resource(&self) -> &REFrameworkResource {
        // SAFETY: SDK is trusted
        unsafe { &*self.sdk.resource }
    }

    #[inline]
    pub(crate) fn sdk_type_info(&self) -> &REFrameworkTypeInfo {
        // SAFETY: SDK is trusted
        unsafe { &*self.sdk.type_info }
    }

    #[inline]
    pub(crate) fn sdk_vm_context(&self) -> &REFrameworkVMContext {
        // SAFETY: SDK is trusted
        unsafe { &*self.sdk.vm_context }
    }

    #[inline]
    pub(crate) fn sdk_reflection_method(&self) -> &REFrameworkReflectionMethod {
        // SAFETY: SDK is trusted
        unsafe { &*self.sdk.reflection_method }
    }

    #[inline]
    pub(crate) fn sdk_reflection_property(&self) -> &REFrameworkReflectionProperty {
        // SAFETY: SDK is trusted
        unsafe { &*self.sdk.reflection_property }
    }
}

#[derive(Debug)]
pub enum APIInitError {
    ParamIsNull(NullPtrError<REFrameworkPluginInitializeParam>),
    AlreadyInitialized,
}

impl Display for APIInitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "could not initialize API: ")?;
        match self {
            APIInitError::ParamIsNull(e) => Display::fmt(e, f),
            APIInitError::AlreadyInitialized => write!(f, "already initialized"),
        }
    }
}

impl Error for APIInitError {}
