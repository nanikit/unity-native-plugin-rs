#!/bin/sh
set -eu
cd "$(dirname "$0")"

# On Win32 the SDK spells its fixed-width integers as unsigned int/long long
# and clang retains __stdcall on DSP callbacks. Map those callbacks to Rust's
# system ABI so the same source also works on Win64 and non-Windows hosts.
bindgen \
  --output src/plugin_api_audio.rs \
  --with-derive-default --no-derive-debug \
  --no-layout-tests --no-prepend-enum-name \
  --allowlist-type 'UnityAudio.*' --allowlist-var 'UNITY_AUDIO.*' \
  sdk/audio/AudioPluginInterface.h -- -x c++ --target=i686-pc-windows-msvc -DNDEBUG
sed -i.bak 's/extern "stdcall"/extern "system"/g' src/plugin_api_audio.rs
rm src/plugin_api_audio.rs.bak
