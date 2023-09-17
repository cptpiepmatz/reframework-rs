use crate::hook::{DuplicateHookError, Hook, HookMetadataRegistryEntry, HOOK_METADATA_REGISTRY};
use crate::API;
use reframework_sys::*;

pub struct Method<'api> {
    pub(crate) api: &'api API,

    // lifetime is bound the API, this is therefore implicitly bound too
    pub(crate) handle: REFrameworkMethodHandle,
}

impl<'api> Method<'api> {
    pub fn add_hook<H>(&self, hook: H) -> Result<(), DuplicateHookError>
    where
        H: Hook,
    {
        let registry = HOOK_METADATA_REGISTRY.get_or_init(Default::default);
        let mut registry = registry.write().expect("plugins don't survive panics");
        let registry_key = H::registry_key();
        if registry.contains_key(&registry_key) {
            return Err(DuplicateHookError {
                type_name: H::TYPE_NAME,
                method_name: H::METHOD_NAME,
            });
        }

        let method = self.api.sdk_method();

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
            self.api.sdk_functions().add_hook.expect("not null")(
                self.handle,
                pre_fn,
                post_fn,
                H::IGNORE_JMP,
            )
        };

        crate::debug::debug!("Hooked into {}#{}", H::TYPE_NAME, H::METHOD_NAME);

        Ok(())
    }

    pub unsafe fn add_hook_raw(
        &self,
        pre_fn: REFPreHookFn,
        post_fn: REFPostHookFn,
        ignore_jmp: bool,
    ) {
        let functions = &(*self.api.sdk.functions);
        functions.add_hook.expect("not null")(self.handle, pre_fn, post_fn, ignore_jmp);
    }

    pub fn is_static(&self) -> bool {
        // SAFETY: SDK is trusted
        unsafe { self.api.sdk_method().is_static.expect("not null")(self.handle) }
    }
}
