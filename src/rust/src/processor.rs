use core::pin::Pin;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;

use wmidi::{Channel, MidiMessage, Note, Velocity};

use crate::ffi;
use crate::phasors::{MidiNote, Voice};

const DEFAULT_SAMPLE_RATE: f64 = 41000.0;
const MAX_NUM_VOICES: usize = 16;

pub struct ElysiumAudioProcessor {
    sample_rate: f64,
    // TODO: Use a datastructure that doesn't allocate
    voices: Vec<Voice>,
    midi_state: MidiState,
}

impl Default for ElysiumAudioProcessor {
    fn default() -> Self {
        Self {
            sample_rate: DEFAULT_SAMPLE_RATE,
            voices: Vec::new(),
            midi_state: MidiState::new(),
        }
    }
}

impl ElysiumAudioProcessor {
    // Will be called on the main thread.
    pub fn prepare_to_play(&mut self, sample_rate: f64, _maximum_expected_samples_per_block: i32) {
        self.sample_rate = sample_rate.max(0.0);
        self.voices = vec![Voice::new(self.sample_rate); MAX_NUM_VOICES];
        self.midi_state = MidiState::new();
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

        // TODO: Can't I just reduce the voice iterators down? (Maybe
        // I need to make the voices stereo first, to handle that
        // usecase).
        let mut equilibrium = vec![0.0f64; audio[0].len()];

        let voice_sample_iters = self
            .voices
            .iter_mut()
            .filter(|voice| voice.currently_playing().is_some())
            .map(|voice| (0..audio[0].len()).map(move |_| voice.next().unwrap()));

        for voice_iter in voice_sample_iters {
            for (i, sample) in voice_iter.enumerate() {
                equilibrium[i] += sample;
            }
        }

        let samples: Vec<f32> = equilibrium
            .into_iter()
            .map(|f| (f * 0.075) as f32)
            .collect();
        for channel in audio.iter_mut() {
            channel.copy_from_slice(&samples);
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
