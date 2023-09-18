use crate::api::managed_object::ManagedObject;
use crate::api::API_REF;
use crate::{MethodParameter, API};
use reframework_sys::*;
use std::collections::HashMap;
use std::error::Error;
use std::ffi::{c_int, c_void};
use std::fmt::{Display, Formatter};
use std::slice;
use std::sync::{OnceLock, RwLock};

type HookMetadataRegistryKey = (&'static str, &'static str);

pub(crate) static HOOK_METADATA_REGISTRY: OnceLock<
    RwLock<HashMap<HookMetadataRegistryKey, HookMetadataRegistryEntry>>,
> = OnceLock::new();

pub(crate) struct HookMetadataRegistryEntry {
    pub has_context: bool,
    pub has_this: bool,
    pub param_count: usize,
}

pub trait Hook {
    const TYPE_NAME: &'static str;
    const METHOD_NAME: &'static str;

    fn pre_fn(
        api: &API,
        vm_context: Option<&VMContext>,
        this: Option<&ManagedObject>,
        params: &[&MethodParameter],
    ) -> PreHookResult {
        PreHookResult::CallOriginal
    }

    unsafe extern "C" fn pre_fn_raw(
        argc: c_int,
        argv: *mut *mut c_void,
        args_tys: *mut TypeDefinitionHandle,
    ) -> c_int {
        let registry = HOOK_METADATA_REGISTRY
            .get()
            .expect("registry is initialized on add_hook");
        let registry = registry.read().expect("plugins don't survive panics");
        let registry_key = Self::registry_key();
        let entry = registry
            .get(&registry_key)
            .expect("got initialized by add_hook");

        let offset = entry.has_context as u32 + entry.has_this as u32;
        let params: &[&MethodParameter] = unsafe {
            slice::from_raw_parts(argv.offset(offset as isize).cast(), entry.param_count)
        };

        let api = API_REF.get().expect("already init");

        let mut vm_context: Option<VMContext> = None;
        let mut this: Option<ManagedObject> = None;
        // FIXME: this ManagedObject is totally wrong
        unsafe {
            match (entry.has_context, entry.has_this) {
                (false, false) => (),
                (false, true) => {
                    this = Some(ManagedObject {
                        handle: *argv.cast(),
                    });
                }
                (true, false) => (), // TODO: prepare VMContext properly,
                (true, true) => {
                    // TODO: do the VMContext here too
                    this = Some(ManagedObject {
                        handle: *argv.offset(1).cast(),
                    });
                }
            }
        }

        Self::pre_fn(api, vm_context.as_ref(), this.as_ref(), params) as c_int
    }

    type ReturnValue;

    fn post_fn(api: &API, ret_val: &mut Self::ReturnValue) {}

    const IGNORE_JMP: bool = false;

    unsafe extern "C" fn post_fn_raw(ret_val: *mut *mut c_void, ret_ty: TypeDefinitionHandle) {
        let ret_val = &mut *ret_val.cast();

        let api = API_REF.get().expect("already init");

        Self::post_fn(api, ret_val)
    }

    fn registry_key() -> HookMetadataRegistryKey {
        (Self::TYPE_NAME, Self::METHOD_NAME)
    }
}

#[repr(C)]
pub enum PreHookResult {
    CallOriginal = REFRAMEWORK_HOOK_CALL_ORIGINAL as isize,
    SkipOriginal = REFRAMEWORK_HOOK_SKIP_ORIGINAL as isize,
}

#[derive(Debug)]
pub struct DuplicateHookError {
    pub type_name: &'static str,
    pub method_name: &'static str,
}

impl Display for DuplicateHookError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "hook metadata registry already has an entry for {}#{}",
            self.type_name, self.method_name
        )
    }
}

impl Error for DuplicateHookError {}

// TODO: probably incorrect type here
#[repr(transparent)]
pub struct VMContext(REFrameworkVMContext);

#[repr(transparent)]
pub struct TypeDefinitionHandle(REFrameworkTypeDefinitionHandle);
