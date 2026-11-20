cmake_minimum_required(VERSION 3.16)
project(unity_audio_host LANGUAGES CXX)
enable_testing()

add_executable(audio_host host.cpp)
target_compile_features(audio_host PRIVATE cxx_std_11)
target_include_directories(audio_host PRIVATE ../../unity-native-plugin-sys/sdk/audio)
target_link_libraries(audio_host PRIVATE ${CMAKE_DL_LIBS})

set(cargo_dir "${CMAKE_CURRENT_BINARY_DIR}/cargo")
add_custom_target(audio_plugin ALL
    COMMAND cargo build -p unity-native-plugin --features audio --example audio_gain
            --target-dir "${cargo_dir}"
    WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}/../.."
    VERBATIM)
add_test(NAME stereo_block_at_half_gain COMMAND audio_host
    "${cargo_dir}/debug/examples/${CMAKE_SHARED_LIBRARY_PREFIX}audio_gain${CMAKE_SHARED_LIBRARY_SUFFIX}")
