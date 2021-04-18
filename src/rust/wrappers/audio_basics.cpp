#include "audio_basics.hpp"

namespace elysium {
namespace ffi {

rust::Slice<const uint8_t> MidiBufferIterator::next()
{
    if (iter == end)
        return {};

    const auto meta = *iter;
    iter++;
    return { meta.data, static_cast<size_t>(std::max(meta.numBytes, 0)) };
}

} // namespace ffi
} // namespace elysium
