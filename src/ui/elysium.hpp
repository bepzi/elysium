#pragma once

#include "elysium_rust.hpp"
#include "owning_mutex.hpp"
#include "utils.hpp"

#include <juce_audio_processors/juce_audio_processors.h>

namespace elysium {

class ElysiumAudioProcessor : public juce::AudioProcessor
{
public:
    ELYSIUM_DISABLE_COPY_MOVE(ElysiumAudioProcessor)

    ElysiumAudioProcessor();

    ~ElysiumAudioProcessor() override;

    const juce::String getName() const override;

    void prepareToPlay(double sampleRate, int maximumExpectedSamplesPerBlock) override;

    void releaseResources() override;

    void processBlock(juce::AudioBuffer<float> &buffer, juce::MidiBuffer &midiMessages) override;

    double getTailLengthSeconds() const override;

    bool acceptsMidi() const override;

    bool producesMidi() const override;

    juce::AudioProcessorEditor *createEditor() override;

    bool hasEditor() const override;

    int getNumPrograms() override;

    int getCurrentProgram() override;

    void setCurrentProgram(int index) override;

    const juce::String getProgramName(int index) override;

    void changeProgramName(int index, const juce::String &newName) override;

    void getStateInformation(juce::MemoryBlock &destData) override;

    void setStateInformation(const void *data, int sizeInBytes) override;

protected:
    bool isBusesLayoutSupported(const BusesLayout &layout) const override;

private:
    // SAFETY: This mutable reference is technically accessible from
    // multiple threads at once. Rust assumes that there is exactly
    // one mutable reference, and therefore that there can't be any
    // data races. It's crucial that we respect Rust's invariants, as
    // the Rust compiler will assume that we're being diligent and
    // correct about whatever's happening here in C++ land.
    //
    // JUCE, when using the Standalone build, seems to be reasonably
    // well-behaved: it really does treat the audio thread and the
    // main thread as separate, and doesn't try to access the
    // AudioProcessor while another thread is using it. We cannot
    // expect plugin hosts to respect that, however.
    //
    // We need to ensure that there really is exactly one mutable
    // reference by enforcing that only one thread can "own" the Rust
    // implementation at a time. And we have to do this WITHOUT
    // blocking the audio thread!
    OwningMutex<rust::Box<ffi::StereoAudioProcessor>> impl;

    static constexpr size_t CHANNELS = 2;
    std::array<rust::Slice<float>, CHANNELS> channels;

    size_t expectedNumSamples;
};

} // namespace elysium
