//! Raw types and callbacks from Unity's native audio plugin SDK.
pub use unity_native_plugin_sys::audio::*;

/// Export one mixer effect from a `cdylib`. Invoke once per plugin.
/// The name must fit in 31 UTF-8 bytes plus a terminating zero.
/// Supply SDK callback field names (e.g. `create`, `reset`, `process`, `release`)
/// and matching `extern "system"` functions. Omitted callbacks are null.
#[macro_export]
macro_rules! unity_audio_effect {
    ($name:expr, $api_version:expr, $plugin_version:expr, $($field:ident: $callback:path),+ $(,)?) => {
        const _: () = {
            use $crate::audio::{UnityAudioEffectDefinition, UnityAudioParameterDefinition};

            static mut EFFECT: UnityAudioEffectDefinition = UnityAudioEffectDefinition {
                structsize: ::core::mem::size_of::<UnityAudioEffectDefinition>() as u32,
                paramstructsize: ::core::mem::size_of::<UnityAudioParameterDefinition>() as u32,
                apiversion: $api_version,
                pluginversion: $plugin_version,
                name: {
                    let bytes = ($name).as_bytes();
                    let mut name = [0; 32];
                    assert!(bytes.len() < name.len(), "effect name exceeds 31 bytes");
                    let mut i = 0;
                    while i < bytes.len() {
                        assert!(bytes[i] != 0, "effect name contains a null byte");
                        name[i] = bytes[i] as ::core::ffi::c_char;
                        i += 1;
                    }
                    name
                },
                $($field: Some($callback),)+
                // All remaining SDK fields are numbers, pointers or nullable callbacks.
                ..unsafe { ::core::mem::zeroed() }
            };
            static mut TABLE: [*mut UnityAudioEffectDefinition; 1] = [&raw mut EFFECT];

            macro_rules! export_registration {
                ($abi:literal) => {
                    #[unsafe(no_mangle)]
                    unsafe extern $abi fn UnityGetAudioEffectDefinitions(
                        definitions: *mut *mut *mut UnityAudioEffectDefinition,
                    ) -> ::core::ffi::c_int {
                        unsafe { definitions.write((&raw mut TABLE).cast()) };
                        1
                    }
                };
            }
            #[cfg(target_vendor = "uwp")]
            export_registration!("system");
            #[cfg(not(target_vendor = "uwp"))]
            export_registration!("C");
        };
    };
}
