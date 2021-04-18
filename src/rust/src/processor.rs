use core::pin::Pin;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::ptr;

use crate::ffi;

const DEFAULT_SAMPLE_RATE: f64 = 41000.0;
const DEFAULT_FREQUENCY: f64 = 440.0;

pub struct ElysiumAudioProcessor {
    rng: ThreadRng,
    sample_rate: f64,
    // TODO: Make a dedicated Phasor struct
    freq: f64,
    angle_delta: f64,
    current_angle: f64,
}

impl ElysiumAudioProcessor {
    pub fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
            sample_rate: DEFAULT_SAMPLE_RATE,
            freq: DEFAULT_FREQUENCY,
            angle_delta: 0.0,
            current_angle: 0.0,
        }
    }

    pub fn prepare_to_play(&mut self, sample_rate: f64, _maximum_expected_samples_per_block: i32) {
        self.sample_rate = sample_rate.max(0.0);
        self.angle_delta = (self.freq / self.sample_rate) * 2.0 * std::f64::consts::PI;
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

        for j in 0..samples {
            let white_noise = (self.rng.gen::<f64>() * 2.0) - 1.0;
            let sine_sample = self.current_angle.sin();
            self.current_angle += self.angle_delta;

            for i in 0..channels {
                unsafe {
                    let channel = *raw_array.add(i);
                    ptr::write(channel.add(j), ((white_noise + sine_sample) * 0.01) as f32);
                }
            }
        }
    }
}
