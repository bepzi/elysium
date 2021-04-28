#![deny(clippy::all)]
#![deny(clippy::correctness)]
#![warn(clippy::style)]
#![warn(clippy::complexity)]
#![warn(clippy::perf)]

pub mod phasors;
pub mod processor;
use processor::ElysiumAudioProcessor;

#[cxx::bridge(namespace = "elysium::ffi")]
mod ffi {
    unsafe extern "C++" {
        include!("audio_basics.hpp");

        type MidiBufferIterator;

        #[rust_name = "next_slice"]
        fn nextSlice(self: Pin<&mut MidiBufferIterator>) -> &[u8];
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
            audio: &mut [&mut [f32]],
            midi: Pin<&mut MidiBufferIterator>,
        );
    }
}

fn create_elysium_audio_processor() -> Box<ElysiumAudioProcessor> {
    Box::new(ElysiumAudioProcessor::default())
}
