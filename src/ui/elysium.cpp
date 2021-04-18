#include "elysium.hpp"

using namespace juce;

namespace elysium {

Elysium::Elysium()
    : AudioProcessor(BusesProperties().withOutput("Output", AudioChannelSet::stereo(), true)),
      rustPlugin(ffi::createElysiumAudioProcessor())
{
}

const String Elysium::getName() const
{
    return JucePlugin_Name;
}

void Elysium::prepareToPlay(double sampleRate, int maximumExpectedSamplesPerBlock)
{
    ignoreUnused(sampleRate, maximumExpectedSamplesPerBlock);
}

void Elysium::releaseResources() { }

void Elysium::processBlock(AudioBuffer<float> &buffer, MidiBuffer &midiMessages)
{
    ScopedNoDenormals noDenormals;
    ignoreUnused(midiMessages);
    rustPlugin->processBlock(buffer);
}

double Elysium::getTailLengthSeconds() const
{
    return 0.0;
}

bool Elysium::acceptsMidi() const
{
    return true;
}

bool Elysium::producesMidi() const
{
    return false;
}

AudioProcessorEditor *Elysium::createEditor()
{
    return nullptr;
}

bool Elysium::hasEditor() const
{
    return false;
}

int Elysium::getNumPrograms()
{
    return 1;
}

int Elysium::getCurrentProgram()
{
    return 0;
}

void Elysium::setCurrentProgram(int index)
{
    ignoreUnused(index);
}

const String Elysium::getProgramName(int index)
{
    ignoreUnused(index);
    return {};
}

void Elysium::changeProgramName(int index, const String &newName)
{
    ignoreUnused(index, newName);
}

void Elysium::getStateInformation(MemoryBlock &destData)
{
    ignoreUnused(destData);
}

void Elysium::setStateInformation(const void *data, int sizeInBytes)
{
    ignoreUnused(data, sizeInBytes);
}

} // namespace elysium

AudioProcessor *JUCE_CALLTYPE createPluginFilter()
{
    return new elysium::Elysium();
}
