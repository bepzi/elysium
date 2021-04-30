use core::pin::Pin;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;

use wmidi::{Channel, MidiMessage, Note, Velocity};

use crate::ffi;
use crate::phasors::{MidiNote, Voice};

const DEFAULT_SAMPLE_RATE: f64 = 41000.0;
const MAX_NUM_VOICES: usize = 16;

pub struct ElysiumAudioProcessor<const CHANNELS: usize> {
    sample_rate: f64,
    voices: [Voice; MAX_NUM_VOICES],
    midi_state: MidiState,
    sample_buffer: [Vec<f64>; CHANNELS],
}

impl<const CHANNELS: usize> Default for ElysiumAudioProcessor<CHANNELS> {
    fn default() -> Self {
        Self {
            sample_rate: DEFAULT_SAMPLE_RATE,
            voices: [Voice::new(DEFAULT_SAMPLE_RATE); MAX_NUM_VOICES],
            midi_state: MidiState::new(),
            sample_buffer: array_init::array_init(|_| Vec::new()),
        }
    }
}

impl<const CHANNELS: usize> ElysiumAudioProcessor<CHANNELS> {
    // Will be called on the main thread.
    pub fn prepare_to_play(&mut self, sample_rate: f64, maximum_expected_samples_per_block: i32) {
        self.sample_rate = sample_rate.max(0.0);

        for voice in &mut self.voices {
            *voice = Voice::new(self.sample_rate);
        }

        self.midi_state = MidiState::new();

        for channel in &mut self.sample_buffer {
            *channel = vec![0.0; maximum_expected_samples_per_block.max(0) as usize];
        }
    }

    // Will be called on the audio thread.
    #[inline(always)]
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
            for voice in &mut self.voices {
                if let Some(note) = voice.currently_playing() {
                    if note.note == *off {
                        voice.stop_playing();
                        break;
                    }
                }
            }
        }

        for (on, velocity) in &self.midi_state.notes_turned_on {
            let note = MidiNote {
                channel: Channel::Ch1,
                note: *on,
                velocity: *velocity,
            };

            let mut found_unused_voice = false;
            for voice in &mut self.voices {
                if voice.currently_playing().is_none() {
                    voice.start_playing(note);
                    found_unused_voice = true;
                    break;
                }
            }

            if !found_unused_voice {
                // TODO: What I really wanted to do was store a
                // mutable reference so that I could just break from
                // the loop and immediately call
                // start_playing(). Fortunately, this code is the slow
                // path, so I don't think it matters.
                let mut least_recently_used = std::time::Instant::now();
                for voice in &self.voices {
                    if voice.last_played_at() < least_recently_used {
                        least_recently_used = voice.last_played_at();
                    }
                }

                for voice in &mut self.voices {
                    if voice.last_played_at() == least_recently_used {
                        voice.start_playing(note);
                        found_unused_voice = true;
                        break;
                    }
                }
            }

            assert!(found_unused_voice)
        }

        // Pull a single frame's worth of samples from each active
        // voice, then sum them together to get the final list of
        // samples.

        let voice_sample_iters = self
            .voices
            .iter_mut()
            .filter(|voice| voice.currently_playing().is_some())
            .map(|voice| (0..audio[0].len()).map(move |_| voice.next().unwrap()));

        // TODO: Ignoring the other channel buffer for now, pretending
        // we're doing everything in mono
        let buffer = &mut self.sample_buffer[0];

        // TODO: Need to handle the possibility that
        // maximum_expected_samples_per_block was actually greater
        // than the total number of samples we were given in this
        // callback.
        assert!(buffer.len() == audio[0].len());

        for voice_iter in voice_sample_iters {
            for (i, sample) in voice_iter.enumerate() {
                buffer[i] += sample;
            }
        }

        let samples: Vec<f32> = buffer.iter().map(|f| (f * 0.075) as f32).collect();
        for channel in audio.iter_mut() {
            channel.copy_from_slice(&samples);
        }

        for channel in &mut self.sample_buffer {
            channel.fill(0.0);
        }
    }
}

#[derive(Debug)]
struct MidiState {
    // TODO: Use a datastructure that doesn't allocate
    //
    // TODO: Need to take MIDI channels into account too. Should
    // probably just store entire MidiNote structs.
    notes_turned_on: HashMap<Note, Velocity>,
    notes_turned_off: HashSet<Note>,
}

impl MidiState {
    fn new() -> Self {
        let mut s = Self {
            notes_turned_on: HashMap::new(),
            notes_turned_off: HashSet::new(),
        };

        s.notes_turned_on.reserve(128);
        s.notes_turned_off.reserve(128);
        s
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
