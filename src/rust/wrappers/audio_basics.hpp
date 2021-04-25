#pragma once

#include "rust/cxx.h"

#include <juce_audio_basics/juce_audio_basics.h>

#include <cstdint>

namespace elysium::ffi {

struct MidiBufferIterator
{
    juce::MidiBufferIterator iter;
    juce::MidiBufferIterator end;

    rust::Slice<const uint8_t> nextSlice();
};

} // namespace elysium::ffi
