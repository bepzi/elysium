use core::pin::Pin;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::ptr;

use crate::ffi;

pub struct ElysiumAudioProcessor {
    rng: ThreadRng,
}

impl ElysiumAudioProcessor {
    pub fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
        }
    }

    pub fn process_block(&mut self, buf: Pin<&mut ffi::AudioBufferF32>) {
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

        let raw_array = buf.get_array_of_write_pointers();

        for i in 0..channels {
            unsafe {
                let raw_channel = *raw_array.add(i);
                for j in 0..samples {
                    let sample = (self.rng.gen::<f32>() * 2.0) - 1.0;
                    ptr::write(raw_channel.add(j), sample * 0.01);
                }
            }
        }
    }
}
