#include "audio_basics.hpp"

namespace elysium::ffi {

rust::Slice<const uint8_t> MidiBufferIterator::nextSlice()
{
    if (iter == end)
        return {};

    const auto meta = *iter;
    iter++;
    return { meta.data, static_cast<size_t>(std::max(meta.numBytes, 0)) };
}

} // namespace elysium::ffi
