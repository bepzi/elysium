use core::pin::Pin;
use ffi::*;
use rand::Rng;
use std::ptr;

#[cxx::bridge(namespace = "elysium::ffi")]
mod ffi {
    unsafe extern "C++" {
        include!("audio_buffer.hpp");

        type AudioBufferF32;

        #[rust_name = "get_num_channels"]
        fn getNumChannels(self: &AudioBufferF32) -> i32;

        #[rust_name = "get_num_samples"]
        fn getNumSamples(self: &AudioBufferF32) -> i32;

        #[rust_name = "get_array_of_read_pointers"]
        fn getArrayOfReadPointers(self: &AudioBufferF32) -> *mut *const f32;

        #[rust_name = "get_array_of_write_pointers"]
        fn getArrayOfWritePointers(self: Pin<&mut AudioBufferF32>) -> *mut *mut f32;

        fn clear(self: Pin<&mut AudioBufferF32>);
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
    Box::new(ElysiumAudioProcessor)
}

struct ElysiumAudioProcessor;

impl ElysiumAudioProcessor {
    fn process_block(&mut self, buf: Pin<&mut AudioBufferF32>) {
        let channels = {
            let count = buf.get_num_channels();
            if count < 0 {
                0
            } else {
                count as usize
            }
        };

        let samples = {
            let count = buf.get_num_samples();
            if count < 0 {
                0
            } else {
                count as usize
            }
        };

        let mut rng = rand::thread_rng();
        let raw_array = buf.get_array_of_write_pointers();

        for i in 0..channels {
            unsafe {
                let raw_channel = *raw_array.add(i);
                for j in 0..samples {
                    let sample = (rng.gen::<f32>() * 2.0) - 1.0;
                    ptr::write(raw_channel.add(j), sample * 0.01);
                }
            }
        }
    }
}
