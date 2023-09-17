#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use bindings::REFRAMEWORK_PLUGIN_VERSION_MAJOR;
pub use bindings::REFRAMEWORK_PLUGIN_VERSION_MINOR;
pub use bindings::REFRAMEWORK_PLUGIN_VERSION_PATCH;

pub use bindings::REFRAMEWORK_RENDERER_D3D11;
pub use bindings::REFRAMEWORK_RENDERER_D3D12;

pub use bindings::REFRAMEWORK_ERROR_UNKNOWN;
pub use bindings::REFRAMEWORK_ERROR_NONE;
pub use bindings::REFRAMEWORK_ERROR_OUT_TOO_SMALL;
pub use bindings::REFRAMEWORK_ERROR_EXCEPTION;
pub use bindings::REFRAMEWORK_ERROR_IN_ARGS_SIZE_MISMATCH;

pub use bindings::REFRAMEWORK_HOOK_CALL_ORIGINAL;
pub use bindings::REFRAMEWORK_HOOK_SKIP_ORIGINAL;

pub use bindings::REFrameworkResult;

pub use bindings::lua_State;

pub use bindings::REFInitializedCb;
pub use bindings::REFLuaStateCreatedCb;
pub use bindings::REFLuaStateDestroyedCb;
pub use bindings::REFOnPresentCb;
pub use bindings::REFOnPreApplicationEntryCb;
pub use bindings::REFOnPostApplicationEntryCb;
pub use bindings::REFOnDeviceResetCb;
pub use bindings::REFOnMessageCb;

pub use bindings::REFCreateScriptState;
pub use bindings::REFDeleteScriptState;

pub use bindings::REFOnInitializeFn;
pub use bindings::REFOnLuaStateCreatedFn;
pub use bindings::REFOnLuaStateDestroyedFn;
pub use bindings::REFOnPresentFn;
pub use bindings::REFOnPreApplicationEntryFn;
pub use bindings::REFOnPostApplicationEntryFn;
pub use bindings::REFLuaLockUnlockFn;
pub use bindings::REFOnDeviceResetFn;
pub use bindings::REFOnMessageFn;



pub use bindings::REFrameworkPluginVersion;

pub use bindings::REFPluginRequiredVersionFn;

pub use bindings::REFrameworkPluginFunctions;

pub use bindings::REFrameworkRendererData;

pub use bindings::REFrameworkTypeDefinitionHandle;
pub use bindings::REFrameworkTypeDefinitionHandle__;
pub use bindings::REFrameworkMethodHandle;
pub use bindings::REFrameworkMethodHandle__;
pub use bindings::REFrameworkFieldHandle;
pub use bindings::REFrameworkFieldHandle__;
pub use bindings::REFrameworkPropertyHandle;
pub use bindings::REFrameworkPropertyHandle__;
pub use bindings::REFrameworkManagedObjectHandle;
pub use bindings::REFrameworkManagedObjectHandle__;
pub use bindings::REFrameworkTDBHandle;
pub use bindings::REFrameworkTDBHandle__;
pub use bindings::REFrameworkHandle;
pub use bindings::REFrameworkHandle__;
pub use bindings::REFrameworkResourceHandle;
pub use bindings::REFrameworkResourceHandle__;
pub use bindings::REFrameworkResourceManagerHandle;
pub use bindings::REFrameworkResourceManagerHandle__;
pub use bindings::REFrameworkVMContextHandle;
pub use bindings::REFrameworkVMContextHandle__;
pub use bindings::REFrameworkTypeInfoHandle;
pub use bindings::REFrameworkTypeInfoHandle__;
pub use bindings::REFrameworkReflectionPropertyHandle;
pub use bindings::REFrameworkReflectionPropertyHandle__;
pub use bindings::REFrameworkReflectionMethodHandle;
pub use bindings::REFrameworkReflectionMethodHandle__;

pub use bindings::REFRAMEWORK_CREATE_INSTANCE_FLAGS_NONE;
pub use bindings::REFRAMEWORK_CREATE_INSTANCE_FLAGS_SIMPLIFY;

pub use bindings::REFRAMEWORK_VM_OBJ_TYPE_NULL;
pub use bindings::REFRAMEWORK_VM_OBJ_TYPE_OBJECT;
pub use bindings::REFRAMEWORK_VM_OBJ_TYPE_ARRAY;
pub use bindings::REFRAMEWORK_VM_OBJ_TYPE_STRING;
pub use bindings::REFRAMEWORK_VM_OBJ_TYPE_DELEGATE;
pub use bindings::REFRAMEWORK_VM_OBJ_TYPE_VALTYPE;

pub use bindings::REFrameworkVMObjType;
pub use bindings::REFrameworkInvokeMethod;
pub use bindings::REFrameworkReflectionPropertyMethod;

pub use bindings::REFrameworkTDBTypeDefinition;

pub use bindings::REFrameworkMethodParameter;

pub use bindings::REFrameworkTDBMethod;

pub use bindings::REFrameworkTDBField;

pub use bindings::REFrameworkTDBProperty;

pub use bindings::REFrameworkTDB;

pub use bindings::REFrameworkManagedObject;

pub use bindings::REFrameworkNativeSingleton;

pub use bindings::REFrameworkManagedSingleton;

pub use bindings::REFrameworkResourceManager;

pub use bindings::REFrameworkResource;

pub use bindings::REFrameworkTypeInfo;

pub use bindings::REFrameworkVMContext;

pub use bindings::REFrameworkReflectionMethod;

pub use bindings::REFrameworkReflectionProperty;

pub use bindings::REFPreHookFn;
pub use bindings::REFPostHookFn;

pub use bindings::REFrameworkSDKFunctions;

pub use bindings::REFrameworkSDKData;

pub use bindings::REFrameworkPluginInitializeParam;
