#include "elysium.hpp"

#include <cstdio>

using namespace juce;

static constexpr auto *THREAD_SAFETY_WARNING =
        "The audio thread failed to get exclusive access to the "
        "AudioProcessor; this is probably because the host is not "
        "correctly handling potential data races.";

namespace elysium {

ElysiumAudioProcessor::ElysiumAudioProcessor()
    : AudioProcessor(BusesProperties().withOutput("Output", AudioChannelSet::stereo(), true)),
      impl(ffi::createElysiumAudioProcessor())
{
}

const String ElysiumAudioProcessor::getName() const
{
    return JucePlugin_Name;
}

void ElysiumAudioProcessor::prepareToPlay(double sampleRate, int maximumExpectedSamplesPerBlock)
{
    std::lock_guard l(implLock);
    impl->prepareToPlay(sampleRate, maximumExpectedSamplesPerBlock);
}

void ElysiumAudioProcessor::releaseResources() { }

void ElysiumAudioProcessor::processBlock(AudioBuffer<float> &buffer, MidiBuffer &midiMessages)
{
    if (ELYSIUM_UNLIKELY(!implLock.try_lock())) {
        // We can't allow access to the Rust implementation if another
        // thread is already accessing it. Rust assumes we won't screw
        // up its mutability and aliasing guarantees.
        //
        // Note that we deliberately only try_lock(), because we can't
        // block the audio thread.
        std::fprintf(stderr, "%s\n", THREAD_SAFETY_WARNING);
        return;
    }

    ffi::MidiBufferIterator iter = { midiMessages.cbegin(), midiMessages.cend() };
    {
        ScopedNoDenormals noDenormals;
        impl->processBlock(buffer, iter);
    }

    implLock.unlock();
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

} // namespace elysium

AudioProcessor *JUCE_CALLTYPE createPluginFilter()
{
    return new elysium::ElysiumAudioProcessor();
}
