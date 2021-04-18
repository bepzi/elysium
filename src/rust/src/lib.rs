mod processor;
use processor::ElysiumAudioProcessor;

#[cxx::bridge(namespace = "elysium::ffi")]
mod ffi {
    unsafe extern "C++" {
        include!("audio_buffer.hpp");

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
    }

    extern "Rust" {
        type ElysiumAudioProcessor;

        #[cxx_name = "createElysiumAudioProcessor"]
        fn create_elysium_audio_processor() -> Box<ElysiumAudioProcessor>;

        #[cxx_name = "processBlock"]
        fn process_block(self: &mut ElysiumAudioProcessor, buf: Pin<&mut AudioBufferF32>);
    }
}

fn create_elysium_audio_processor() -> Box<ElysiumAudioProcessor> {
    Box::new(ElysiumAudioProcessor::new())
}
