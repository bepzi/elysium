use wmidi::MidiMessage;

mod processor;
use processor::ElysiumAudioProcessor;

#[cxx::bridge(namespace = "elysium::ffi")]
mod ffi {
    unsafe extern "C++" {
        include!("audio_basics.hpp");

        type AudioBufferF32;

        #[rust_name = "get_num_channels"]
        fn getNumChannels(self: &AudioBufferF32) -> i32;

        #[rust_name = "get_num_samples"]
        fn getNumSamples(self: &AudioBufferF32) -> i32;

        /*
                #[rust_name = "get_array_of_read_pointers"]
                fn getArrayOfReadPointers(self: &AudioBufferF32) -> *mut *const f32;
        */

        #[rust_name = "get_array_of_write_pointers"]
        fn getArrayOfWritePointers(self: Pin<&mut AudioBufferF32>) -> *mut *mut f32;

        /*
              fn clear(self: Pin<&mut AudioBufferF32>);
        */

        // ===================================================

        type MidiBufferIterator;

        fn next(self: Pin<&mut MidiBufferIterator>) -> &[u8];
    }

    extern "Rust" {
        type ElysiumAudioProcessor;

        #[cxx_name = "createElysiumAudioProcessor"]
        fn create_elysium_audio_processor() -> Box<ElysiumAudioProcessor>;

        #[cxx_name = "prepareToPlay"]
        fn prepare_to_play(
            self: &mut ElysiumAudioProcessor,
            sample_rate: f64,
            maximum_expected_samples_per_block: i32,
        );

        #[cxx_name = "processBlock"]
        fn process_block(
            self: &mut ElysiumAudioProcessor,
            buf: Pin<&mut AudioBufferF32>,
            midi: Pin<&mut MidiBufferIterator>,
        );
    }
}

fn create_elysium_audio_processor() -> Box<ElysiumAudioProcessor> {
    Box::new(ElysiumAudioProcessor::new())
}
