#![allow(clippy::missing_safety_doc)]

use reframework::api::managed_object::ManagedObject;
use reframework::api::API;
use reframework::hook::{Hook, PreHookResult, VMContext};
use reframework::{MethodParameter, PluginInitializeParam};

struct PlayerWeaponCtrlStart;

impl Hook for PlayerWeaponCtrlStart {
    const TYPE_NAME: &'static str = "snow.player.PlayerWeaponCtrl";
    const METHOD_NAME: &'static str = "start";

    fn pre_fn(
        _api: &API,
        _vm_context: Option<&VMContext>,
        this: Option<&mut ManagedObject>,
        _params: &[&MethodParameter],
    ) -> Option<PreHookResult> {
        let this = this.expect("not native and not static");
        if let Some(field) = this.get_field_mut::<f32>("_bodyConstScale") {
            *field = 1.0;
        }

        Some(PreHookResult::CallOriginal)
    }

    type ReturnValue = ();
}

reframework::plugin_required_version!();

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub unsafe extern "C" fn reframework_plugin_initialize(
    param: *const PluginInitializeParam,
) -> bool {
    let api = API::initialize("Weapon Stay Big", param).expect("should init");
    let tdb = api.tdb();
    let method = tdb
        .find_method(
            PlayerWeaponCtrlStart::TYPE_NAME,
            PlayerWeaponCtrlStart::METHOD_NAME,
        )
        .expect("should be available");

    method
        .add_hook(PlayerWeaponCtrlStart)
        .expect("no duplicate");

    true
}
