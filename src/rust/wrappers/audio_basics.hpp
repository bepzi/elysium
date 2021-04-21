#pragma once

#include "rust/cxx.h"

#include <juce_audio_basics/juce_audio_basics.h>

#include <cstdint>

namespace elysium {
namespace ffi {

struct MidiBufferIterator
{
    juce::MidiBufferIterator iter;
    juce::MidiBufferIterator end;

    rust::Slice<const uint8_t> next();
};

} // namespace ffi
} // namespace elysium
