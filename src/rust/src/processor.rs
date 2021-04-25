use core::pin::Pin;
use std::convert::TryFrom;
use wmidi::MidiMessage;

// use dasp_envelope;
use dasp_sample::Sample;
use dasp_signal::Signal;
// use dasp_signal::envelope::SignalEnvelope;

use crate::ffi;

const DEFAULT_SAMPLE_RATE: f64 = 41000.0;
const DEFAULT_FREQUENCY: f64 = 220.0;

pub struct ElysiumAudioProcessor {
    sample_rate: f64,
    freq: f64,
    signal: Option<Box<dyn Signal<Frame = f64>>>,
    midi_state: MidiState,
}

impl ElysiumAudioProcessor {
    pub fn new() -> Self {
        Self {
            sample_rate: DEFAULT_SAMPLE_RATE,
            freq: DEFAULT_FREQUENCY,
            signal: None,
            midi_state: MidiState::new(),
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
        midi: Pin<&mut ffi::MidiBufferIterator>,
    ) {
        self.midi_state.process_midi_messages(midi);

        if !self.midi_state.notes_on.is_empty() {
            println!("MIDI ON {:?}", self.midi_state.notes_on);
        }
        if !self.midi_state.notes_turned_off.is_empty() {
            println!("MIDI OFF {:?}", self.midi_state.notes_turned_off);
        }

        assert!(!audio.is_empty());
        assert!(self.signal.is_some());

        let signal = self.signal.as_mut().unwrap();
        let samples = (0..audio[0].len())
            .map(|_| {
                let s = signal.next() * 0.01;
                s.to_sample::<f32>()
            })
            .collect::<Vec<f32>>();

        for channel in audio.iter_mut() {
            channel.copy_from_slice(&samples);
        }
    }
}

struct MidiState {
    notes_on: Vec<MidiMessage<'static>>,
    notes_turned_off: Vec<MidiMessage<'static>>,
}

impl MidiState {
    fn new() -> Self {
        Self {
            notes_on: vec![],
            notes_turned_off: vec![],
        }
    }

    fn process_midi_messages(&mut self, mut midi: Pin<&mut ffi::MidiBufferIterator>) {
        self.notes_turned_off.clear();

        let mut raw_midi_message: &[u8] = midi.as_mut().next_slice();
        while !raw_midi_message.is_empty() {
            if let Ok(message) = MidiMessage::try_from(raw_midi_message) {
                match message {
                    MidiMessage::NoteOn(_, _, _) => {
                        self.notes_on.push(message.drop_unowned_sysex().unwrap())
                    }
                    MidiMessage::NoteOff(_, _, _) => {
                        let owned = message.drop_unowned_sysex().unwrap();

                        // Pair the NoteOn with its NoteOff, but don't
                        // worry about checking the velocity.
                        self.notes_on.retain(|note| match (note, &owned) {
                            (
                                &MidiMessage::NoteOn(ch1, n1, _),
                                &MidiMessage::NoteOff(ch2, n2, _),
                            ) => ch1 != ch2 || n1 != n2,
                            _ => true,
                        });

                        self.notes_turned_off.push(owned);
                    }
                    _ => {}
                }
            }

            raw_midi_message = midi.as_mut().next_slice();
        }
    }
}
