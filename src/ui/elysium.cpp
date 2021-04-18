#include "elysium.hpp"

using namespace juce;

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
    impl->prepareToPlay(sampleRate, maximumExpectedSamplesPerBlock);
}

void ElysiumAudioProcessor::releaseResources() { }

void ElysiumAudioProcessor::processBlock(AudioBuffer<float> &buffer, MidiBuffer &midiMessages)
{
    ScopedNoDenormals noDenormals;
    ignoreUnused(midiMessages);
    impl->processBlock(buffer);
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
