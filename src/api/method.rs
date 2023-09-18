use crate::api::API_REF;
use crate::error::Error;
use crate::hook::{DuplicateHookError, Hook, HookMetadataRegistryEntry, HOOK_METADATA_REGISTRY};
use crate::invoke::{InvokeArg, InvokeObj, InvokeResult, InvokeRet, InvokeValue};
use crate::managed_object::ManagedObject;
use crate::API;
use reframework_sys::*;
use std::ffi::{c_uint, c_void};
use std::mem::size_of;

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct Method {
    pub(crate) handle: REFrameworkMethodHandle,
}

unsafe impl Sync for Method {}
unsafe impl Send for Method {}

impl Method {
    pub fn add_hook<H>(&self, hook: H) -> Result<(), DuplicateHookError>
    where
        H: Hook,
    {
        let api = API_REF.get().expect("is init");

        let registry = HOOK_METADATA_REGISTRY.get_or_init(Default::default);
        let mut registry = registry.write().expect("plugins don't survive panics");
        let registry_key = H::registry_key();
        if registry.contains_key(&registry_key) {
            return Err(DuplicateHookError {
                type_name: H::TYPE_NAME,
                method_name: H::METHOD_NAME,
            });
        }

        let method = api.sdk_method();

        // TODO: check if this is the only way to guess whether a function is "native"
        let is_native = H::TYPE_NAME.starts_with("via.");
        let is_static = self.is_static();

        // SAFETY: SDK is trusted
        let param_count = unsafe { method.get_num_params.expect("not null")(self.handle) };
        let param_count = param_count as usize;

        registry.insert(
            registry_key,
            HookMetadataRegistryEntry {
                has_context: !is_native,
                has_this: !is_static,
                param_count,
            },
        );

        let pre_fn = H::pre_fn_raw as *const ();
        // SAFETY: here we cast the pre_fn_raw that uses a thin wrapper with the same memory layout
        //         to the function interface that we want, by this we can expose only our wrapper
        //         without exposing the underlying type
        let pre_fn: REFPreHookFn = Some(unsafe { std::mem::transmute(pre_fn) });

        let post_fn = H::post_fn_raw as *const ();
        // SAFETY: same as the pre_fn
        let post_fn: REFPostHookFn = Some(unsafe { std::mem::transmute(post_fn) });

        // SAFETY: SDK is trusted
        unsafe {
            api.sdk_functions().add_hook.expect("not null")(
                self.handle,
                pre_fn,
                post_fn,
                H::IGNORE_JMP,
            )
        };

        crate::debug::debug!("Hooked into {}#{}", H::TYPE_NAME, H::METHOD_NAME);

        Ok(())
    }

    pub fn is_static(&self) -> bool {
        let api = API_REF.get().expect("is init");

        // SAFETY: SDK is trusted
        unsafe { api.sdk_method().is_static.expect("not null")(self.handle) }
    }

    pub fn get_function(&self) -> *mut c_void {
        let api = API_REF.get().expect("is init");

        // SAFETY: SDK is trusted
        unsafe { api.sdk_method().get_function.expect("not null")(self.handle) }
    }

    pub fn invoke<T>(
        &self,
        this_ptr: impl InvokeObj,
        args: &mut [&dyn InvokeArg],
    ) -> InvokeResult<T>
    where
        T: From<InvokeValue>,
    {
        let mut out = InvokeRet::default();
        let out_ptr: *mut InvokeRet = &mut out;
        let mut args: Vec<*mut c_void> = args.iter_mut().map(|arg| unsafe { arg.as_ptr() }).collect();
        let in_args_size = args.len() * size_of::<*mut c_void>();

        let api = API_REF.get().expect("is init");
        let method = api.sdk_method();
        let result = unsafe {
            method.invoke.expect("not null")(
                self.handle,
                this_ptr.as_ptr(),
                args.as_mut_ptr(),
                in_args_size as c_uint,
                out_ptr.cast(),
                size_of::<InvokeRet>() as c_uint,
            )
        };
        let result: Error = result.into();

        if result != Error::None {
            return InvokeResult::Err(result);
        }

        if out.exception_thrown {
            return InvokeResult::ExceptionOccurred;
        }

        InvokeResult::Ok(out.value.into())
    }
}
