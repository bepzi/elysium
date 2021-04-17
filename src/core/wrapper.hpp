#pragma once

#include <juce_audio_basics/juce_audio_basics.h>

#include <cstdint>

using AudioBufferF32 = juce::AudioBuffer<float>;

int32_t get_num_channels(const AudioBufferF32 &buf);

int32_t get_num_samples(const AudioBufferF32 &buf);

const float *const *get_array_of_read_pointers(const AudioBufferF32 &buf);

float **get_array_of_write_pointers(AudioBufferF32 &buf);

void clear(AudioBufferF32 &buf);
