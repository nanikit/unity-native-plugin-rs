unity-native-plugin-rs
====

* based on Unity 6000.3.8f1
* bindgen 0.72.1

## Audio bindings

The `audio` feature exposes the raw audio SDK in `unity_native_plugin_sys::audio`.
The generated bindings are included in the crate. The SDK header and its MIT
license come from [Unity NativeAudioPlugins, API 0x010403](https://github.com/Unity-Technologies/NativeAudioPlugins/tree/bc7893edbba4c8a592777e590e34f21a11d762b4).

To regenerate, run `sh bindgen_audio.sh` with bindgen 0.72.1, libclang and
headers for the i686 Windows MSVC target.
