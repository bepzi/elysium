#include "elysium.hpp"

#include <cstdio>
#include <cstdlib>

using namespace juce;

static constexpr auto *THREAD_SAFETY_WARNING =
        "The audio thread failed to get exclusive access to the "
        "AudioProcessor; this is probably because the host is not "
        "correctly handling potential data races.\n";

static constexpr auto *NUM_CHANNELS_WARNING =
        "The plugin asked the host for %zu channels, but was given "
        "%zu instead.\n";

static constexpr auto *NUM_SAMPLES_WARNING =
        "The host told the plugin to expect at most %zu samples, "
        "but gave %zu instead.\n";

namespace elysium {

ElysiumAudioProcessor::ElysiumAudioProcessor()
    : AudioProcessor(BusesProperties().withOutput("Output", AudioChannelSet::stereo(), true)),
      impl(ffi::createStereoAudioProcessor()),
      expectedNumSamples(0)
{
    static_assert(CHANNELS > 0);
}

ElysiumAudioProcessor::~ElysiumAudioProcessor() = default;

// NOLINTNEXTLINE(readability-const-return-type)
const String ElysiumAudioProcessor::getName() const
{
    return JucePlugin_Name;
}

void ElysiumAudioProcessor::prepareToPlay(double sampleRate, int maximumExpectedSamplesPerBlock)
{
    expectedNumSamples = maximumExpectedSamplesPerBlock;
    impl.lock().getMut()->prepareToPlay(sampleRate, maximumExpectedSamplesPerBlock);
}

void ElysiumAudioProcessor::releaseResources() { }

void ElysiumAudioProcessor::processBlock(AudioBuffer<float> &buffer, MidiBuffer &midiMessages)
{
    const auto guard = impl.try_lock();
    if (ELYSIUM_UNLIKELY(!guard)) {
        // We can't allow access to the Rust implementation if another
        // thread is already accessing it. Rust assumes we won't screw
        // up its mutability and aliasing guarantees.
        //
        // Note that we deliberately only try_lock(), because we can't
        // block the audio thread.
        std::fprintf(stderr, THREAD_SAFETY_WARNING);
        buffer.clear();
        std::abort();
    }

    const auto numChannels = static_cast<size_t>(std::max(buffer.getNumChannels(), 0));
    if (ELYSIUM_UNLIKELY(numChannels != CHANNELS)) {
        // For some reason the host gave us a different buffer size than we asked for??
        std::fprintf(stderr, NUM_CHANNELS_WARNING, CHANNELS, numChannels);
        buffer.clear();
        std::abort();
    }

    const auto numSamples = static_cast<size_t>(std::max(buffer.getNumSamples(), 0));
    if (ELYSIUM_UNLIKELY(numSamples > expectedNumSamples)) {
        // For some reason the host gave us more samples than it told us to expect??
        std::fprintf(stderr, NUM_SAMPLES_WARNING, expectedNumSamples, numSamples);
        buffer.clear();
        std::abort();
    }

    for (size_t i = 0; i < numChannels; ++i)
        channels[i] = { buffer.getWritePointer(i), numSamples };

    rust::Slice<rust::Slice<float>> audioData = { channels.data(), numChannels };
    ffi::MidiBufferIterator midiIter = { midiMessages.cbegin(), midiMessages.cend() };
    {
        ScopedNoDenormals noDenormals;
        guard->getMut()->processBlock(audioData, midiIter);
    }
}

double ElysiumAudioProcessor::getTailLengthSeconds() const
{
    return 0.0;
}

bool ElysiumAudioProcessor::acceptsMidi() const
{
    return true;
}

bool ElysiumAudioProcessor::producesMidi() const
{
    return false;
}

AudioProcessorEditor *ElysiumAudioProcessor::createEditor()
{
    return nullptr;
}

bool ElysiumAudioProcessor::hasEditor() const
{
    return false;
}

int ElysiumAudioProcessor::getNumPrograms()
{
    return 1;
}

int ElysiumAudioProcessor::getCurrentProgram()
{
    return 0;
}

void ElysiumAudioProcessor::setCurrentProgram(int index)
{
    ignoreUnused(index);
}

// NOLINTNEXTLINE(readability-const-return-type)
const String ElysiumAudioProcessor::getProgramName(int index)
{
    ignoreUnused(index);
    return {};
}

void ElysiumAudioProcessor::changeProgramName(int index, const String &newName)
{
    ignoreUnused(index, newName);
}

void ElysiumAudioProcessor::getStateInformation(MemoryBlock &destData)
{
    ignoreUnused(destData);
}

void ElysiumAudioProcessor::setStateInformation(const void *data, int sizeInBytes)
{
    ignoreUnused(data, sizeInBytes);
}

bool ElysiumAudioProcessor::isBusesLayoutSupported(const BusesLayout &layout) const
{
    return layout.getMainOutputChannelSet() == juce::AudioChannelSet::stereo();
}

} // namespace elysium

AudioProcessor *JUCE_CALLTYPE createPluginFilter()
{
    return new elysium::ElysiumAudioProcessor();
}
