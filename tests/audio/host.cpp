#ifdef _WIN32
#include <windows.h>
#else
#include <dlfcn.h>
#endif
#include "AudioPluginInterface.h"
#include <algorithm>
#include <cstdio>
#include <cstring>

static int fail(const char* message)
{
    std::fprintf(stderr, "%s\n", message);
    return 1;
}

int main(int argc, char** argv)
{
    if (argc != 2)
        return fail("Expected the path to the Rust audio plugin");
#ifdef _WIN32
    auto library = LoadLibraryA(argv[1]);
    auto symbol = library ? GetProcAddress(library, "UnityGetAudioEffectDefinitions") : nullptr;
#else
    auto library = dlopen(argv[1], RTLD_NOW);
    auto symbol = library ? dlsym(library, "UnityGetAudioEffectDefinitions") : nullptr;
#endif
    if (!symbol)
        return fail("Could not load UnityGetAudioEffectDefinitions from the plugin");
    auto get_definitions = reinterpret_cast<decltype(&UnityGetAudioEffectDefinitions)>(symbol);
    UnityAudioEffectDefinition** effects = nullptr;
    if (get_definitions(&effects) != 1 || !effects || !effects[0])
        return fail("Expected one registered effect");
    const auto& effect = *effects[0];
    UnityAudioEffectDefinition** again = nullptr;
    if (get_definitions(&again) != 1 || again != effects)
        return fail("Registration must return the same persistent table");
    if (effect.structsize != sizeof(effect) ||
        effect.paramstructsize != sizeof(UnityAudioParameterDefinition) ||
        effect.apiversion != UNITY_AUDIO_PLUGIN_API_VERSION || effect.pluginversion != 1 ||
        std::strncmp(effect.name, "Half Gain", sizeof(effect.name)) != 0)
        return fail("Unexpected effect definition layout, API version or name");

    UnityAudioEffectState state{};
    state.structsize = sizeof(state);
    state.samplerate = 48000;
    state.dspbuffersize = 3;
    state.hostapiversion = UNITY_AUDIO_PLUGIN_API_VERSION;
    if (!effect.create || effect.create(&state) != UNITY_AUDIODSP_OK || !state.effectdata)
        return fail("Could not create the effect instance");
    float stereo[] = {1, -1, 0.5f, -0.5f, -2, 2};
    float output[6]{};
    const float expected[] = {0.5f, -0.5f, 0.25f, -0.25f, -1, 1};
    if (!effect.process || effect.process(&state, stereo, output, state.dspbuffersize, 2, 2) != UNITY_AUDIODSP_OK ||
        !std::equal(output, output + 6, expected))
        return fail("Expected every sample of the stereo block at half gain");
    if (!effect.release || effect.release(&state) != UNITY_AUDIODSP_OK || state.effectdata)
        return fail("Could not release the effect instance");
    return 0;
}
