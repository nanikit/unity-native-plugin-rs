use core::ptr;
use unity_native_plugin::audio::*;

unity_native_plugin::unity_audio_effect!(
    "Half Gain", UNITY_AUDIO_PLUGIN_API_VERSION, 1,
    create: create, process: process, release: release,
);

unsafe extern "system" fn create(state: *mut UnityAudioEffectState) -> i32 {
    unsafe {
        (*state).__bindgen_anon_1.__bindgen_anon_1.effectdata =
            Box::into_raw(Box::new(0.5f32)).cast();
    }
    UNITY_AUDIODSP_OK
}

unsafe extern "system" fn process(
    state: *mut UnityAudioEffectState,
    input: *mut f32,
    output: *mut f32,
    frames: u32,
    in_channels: i32,
    out_channels: i32,
) -> i32 {
    if in_channels != out_channels {
        return UNITY_AUDIODSP_ERR_UNSUPPORTED;
    }
    unsafe {
        let gain = *(*state)
            .__bindgen_anon_1
            .__bindgen_anon_1
            .effectdata
            .cast::<f32>();
        for index in 0..frames as usize * in_channels as usize {
            output.add(index).write(input.add(index).read() * gain);
        }
    }
    UNITY_AUDIODSP_OK
}

unsafe extern "system" fn release(state: *mut UnityAudioEffectState) -> i32 {
    unsafe {
        let data = &mut (*state).__bindgen_anon_1.__bindgen_anon_1.effectdata;
        drop(Box::from_raw(
            core::mem::replace(data, ptr::null_mut()).cast::<f32>(),
        ));
    }
    UNITY_AUDIODSP_OK
}
