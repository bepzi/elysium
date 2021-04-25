use core::pin::Pin;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;

use dasp_signal::Signal;
use wmidi::{MidiMessage, Note, Velocity, U7};

use crate::ffi;

const DEFAULT_SAMPLE_RATE: f64 = 41000.0;

pub struct ElysiumAudioProcessor {
    sample_rate: f64,
    // TODO: Use a datastructure that doesn't allocate
    voices: HashMap<Note, Box<dyn Signal<Frame = f64>>>,
    midi_state: MidiState,
}

impl ElysiumAudioProcessor {
    pub fn new() -> Self {
        Self {
            sample_rate: DEFAULT_SAMPLE_RATE,
            voices: HashMap::new(),
            midi_state: MidiState::new(),
        }
    }

    // Will be called on the main thread.
    pub fn prepare_to_play(&mut self, sample_rate: f64, _maximum_expected_samples_per_block: i32) {
        self.sample_rate = sample_rate.max(0.0);
        self.voices = HashMap::new();
        self.midi_state = MidiState::new();
    }

    // Will be called on the audio thread.
    pub fn process_block(
        &mut self,
        audio: &mut [&mut [f32]],
        midi: Pin<&mut ffi::MidiBufferIterator>,
    ) {
        if audio.is_empty() {
            return;
        }

        self.midi_state.process_midi_messages(midi);

        for off in &self.midi_state.notes_turned_off {
            self.voices.remove(off);
        }

        for (on, velocity) in &self.midi_state.notes_turned_on {
            self.voices.insert(
                *on,
                // TODO: Don't heap allocate for each NoteOn event
                Box::new(
                    dasp_signal::rate(self.sample_rate)
                        .const_hz(on.to_freq_f64())
                        .square()
                        // TODO: Should this be logarithmic rather than linear?
                        .scale_amp(u8::from(*velocity) as f64 / u8::from(U7::MAX) as f64)
                        .scale_amp(0.02),
                ),
            );
        }

        // Pull a single frame's worth of samples from each active
        // voice, then sum them together to get the final list of
        // samples.
        let mut equilibrium = vec![0.0f64; audio[0].len()];

        let voice_sample_iters = self
            .voices
            .values_mut()
            .map(|voice| (0..audio[0].len()).map(move |_| voice.next()));

        for voice_iter in voice_sample_iters {
            for (i, sample) in voice_iter.enumerate() {
                equilibrium[i] += sample;
            }
        }

        let samples: Vec<f32> = equilibrium.into_iter().map(|f| f as f32).collect();
        for channel in audio.iter_mut() {
            channel.copy_from_slice(&samples);
        }
    }
}

#[derive(Debug)]
struct MidiState {
    // TODO: Use a datastructure that doesn't allocate
    notes_turned_on: HashMap<Note, Velocity>,
    notes_turned_off: HashSet<Note>,
}

impl MidiState {
    fn new() -> Self {
        Self {
            notes_turned_on: HashMap::new(),
            notes_turned_off: HashSet::new(),
        }
    }

    fn process_midi_messages(&mut self, mut midi: Pin<&mut ffi::MidiBufferIterator>) {
        self.notes_turned_on.clear();
        self.notes_turned_off.clear();

        let mut raw_midi_message: &[u8] = midi.as_mut().next_slice();
        while !raw_midi_message.is_empty() {
            if let Ok(message) = MidiMessage::try_from(raw_midi_message) {
                match message {
                    MidiMessage::NoteOn(_, note, velocity) => {
                        self.notes_turned_off.insert(note);
                        self.notes_turned_on.insert(note, velocity);
                    }

                    MidiMessage::NoteOff(_, note, _) => {
                        self.notes_turned_on.remove(&note);
                        self.notes_turned_off.insert(note);
                    }
                    _ => {}
                }
            }

            raw_midi_message = midi.as_mut().next_slice();
        }
    }
}
