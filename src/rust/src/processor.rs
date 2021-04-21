use core::pin::Pin;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::convert::TryFrom;

use crate::ffi;

const DEFAULT_SAMPLE_RATE: f64 = 41000.0;
const DEFAULT_FREQUENCY: f64 = 440.0;
const TWO_PI: f64 = std::f64::consts::PI * 2.0;

pub struct ElysiumAudioProcessor {
    rng: ThreadRng,
    sample_rate: f64,

    // TODO: Make a dedicated Phasor state struct
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

    // Will be called on the main thread.
    pub fn prepare_to_play(&mut self, sample_rate: f64, _maximum_expected_samples_per_block: i32) {
        self.sample_rate = sample_rate.max(0.0);
        self.angle_delta = (self.freq / self.sample_rate) * TWO_PI;
    }

    // Will be called on the audio thread.
    pub fn process_block(
        &mut self,
        audio: &mut [&mut [f32]],
        mut midi: Pin<&mut ffi::MidiBufferIterator>,
    ) {
        let mut raw_midi_message: &[u8] = midi.as_mut().next();
        while !raw_midi_message.is_empty() {
            if let Ok(message) = wmidi::MidiMessage::try_from(raw_midi_message) {
                println!("MIDI MESSAGE: ${:?}", message);
            }
            raw_midi_message = midi.as_mut().next();
        }

        for j in 0..audio[0].len() {
            let white_noise = (self.rng.gen::<f64>() * 2.0) - 1.0;
            let sine_sample = self.current_angle.sin();
            self.current_angle += self.angle_delta;

            for i in 0..audio.len() {
                audio[i][j] = ((white_noise + sine_sample) * 0.01) as f32;
            }
        }
    }
}
