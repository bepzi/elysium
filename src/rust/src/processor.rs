use core::pin::Pin;
use std::convert::TryFrom;

use dasp_sample::Sample;
use dasp_signal::Signal;

use crate::ffi;

const DEFAULT_SAMPLE_RATE: f64 = 41000.0;
const DEFAULT_FREQUENCY: f64 = 220.0;

pub struct ElysiumAudioProcessor {
    sample_rate: f64,
    freq: f64,
    signal: Option<Box<dyn Signal<Frame = f64>>>,
}

impl ElysiumAudioProcessor {
    pub fn new() -> Self {
        Self {
            sample_rate: DEFAULT_SAMPLE_RATE,
            freq: DEFAULT_FREQUENCY,
            signal: None,
        }
    }

    // Will be called on the main thread.
    pub fn prepare_to_play(&mut self, sample_rate: f64, _maximum_expected_samples_per_block: i32) {
        self.sample_rate = sample_rate.max(0.0);
        self.signal = Some(Box::new(
            dasp_signal::rate(self.sample_rate)
                .const_hz(self.freq)
                .square(),
        ));
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

        assert!(!audio.is_empty());
        assert!(self.signal.is_some());

        let signal = self.signal.as_mut().unwrap();
        let samples = (0..audio[0].len()).map(|_| {
            let s = signal.next() * 0.01;
            s.to_sample::<f32>()
        }).collect::<Vec<f32>>();

        for channel in audio.iter_mut() {
            channel.copy_from_slice(&samples);
        }
    }
}
