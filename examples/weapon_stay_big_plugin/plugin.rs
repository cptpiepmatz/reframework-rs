use reframework::api::API;
use reframework::hook::{Hook, PreHookResult, VMContext};
use reframework::{debug, ManagedObject, MethodParameter};
use reframework_sys::bindings::REFrameworkPluginInitializeParam;

struct PlayerWeaponCtrlStart;

impl Hook for PlayerWeaponCtrlStart {
    const TYPE_NAME: &'static str = "snow.player.PlayerWeaponCtrl";
    const METHOD_NAME: &'static str = "start";

    fn pre_fn(
        api: &API,
        vm_context: Option<&VMContext>,
        this: Option<&ManagedObject>,
        params: &[&MethodParameter],
    ) -> PreHookResult {
        debug!("we got the new hook interface!");

        PreHookResult::CallOriginal
    }

    type ReturnValue = ();
}

reframework::plugin_required_version!();

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub unsafe extern "C" fn reframework_plugin_initialize(
    param: *const REFrameworkPluginInitializeParam,
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
