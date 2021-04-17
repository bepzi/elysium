#include "wrapper.hpp"

int32_t get_num_channels(const AudioBufferF32 &buf)
{
    return static_cast<int32_t>(buf.getNumChannels());
}

int32_t get_num_samples(const AudioBufferF32 &buf)
{
    return static_cast<int32_t>(buf.getNumSamples());
}

const float *const *get_array_of_read_pointers(const AudioBufferF32 &buf)
{
    return buf.getArrayOfReadPointers();
}

float **get_array_of_write_pointers(AudioBufferF32 &buf)
{
    return buf.getArrayOfWritePointers();
}

void clear(AudioBufferF32 &buf)
{
    buf.clear();
}
