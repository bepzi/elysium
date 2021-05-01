use crate::phasor::{Frequency, MidiNote, Phasor};

use wmidi::U7;

const BEND_SEMITONES: i8 = 2;

/// Represents a playable synth voice that can generate a
/// multi-channel signal.
#[derive(Debug, Copy, Clone)]
pub struct Voice<const CHANNELS: usize> {
    phasor: Phasor,
    last_played_at: std::time::Instant,
    playing: Option<MidiNote>,
    velocity_f64: f64,
    pitch_bend: f64,
}

impl<const CHANNELS: usize> Voice<CHANNELS> {
    /// Creates a new [`Voice`] for a given sampling rate.
    ///
    /// If the sampling rate changes later, you should opt to
    /// reconstruct a new [`Voice`].
    pub fn new(sample_rate: Frequency) -> Self {
        Self {
            phasor: Phasor::new(sample_rate, 0.0),
            last_played_at: std::time::Instant::now(),
            playing: None,
            velocity_f64: 0.0,
            pitch_bend: 0.0,
        }
    }

    #[inline]
    fn update_freq(&mut self) {
        if let Some(playing) = self.playing {
            // TODO: Eventually we'll have to handle more complex
            // frequency changes, like vibrato.
            let mut freq = playing.note.to_freq_f64();

            let steps: i8 = if self.pitch_bend >= 0.0 {
                BEND_SEMITONES
            } else {
                -BEND_SEMITONES
            };

            if let Ok(next_freq) = playing.note.step(steps) {
                let next_freq = next_freq.to_freq_f64();
                freq += (next_freq - freq).abs() * self.pitch_bend;
            }

            self.phasor.set_freq(freq);
        }
    }

    /// Updates the voice's pitch bend.
    ///
    /// `pitch_bend` should be a finite value in `[-1, 1]`.
    pub fn set_pitch_bend(&mut self, pitch_bend: f64) {
        assert!(pitch_bend.is_finite());
        self.pitch_bend = pitch_bend.clamp(-1.0, 1.0);
        self.update_freq();
    }

    /// Tells this voice to start playing a certain MIDI note.
    pub fn start_playing(&mut self, note: MidiNote) {
        self.stop_playing();

        self.last_played_at = std::time::Instant::now();
        self.playing = Some(note);
        // TODO: Should velocity sensing be logarithmic instead of linear?
        self.velocity_f64 = u8::from(note.velocity) as f64 / u8::from(U7::MAX) as f64;

        self.phasor.reset();
        self.update_freq();
    }

    /// Tells this voice to stop playing any MIDI notes.
    pub fn stop_playing(&mut self) {
        self.playing = None;
    }

    /// Gets the MIDI note that is currently playing, if any.
    ///
    /// Note that a voice may still produce meaningful sample values
    /// even if this returns `None`.
    pub fn currently_playing(&self) -> Option<MidiNote> {
        self.playing
    }

    /// Returns true if a call to `next_frame()` would produce any
    /// meaningful values.
    pub fn will_produce_values(&self) -> bool {
        // TODO: This is where we can distinguish between voices that
        // still have a tail (e.g, ADSR release) but aren't
        // technically playing anything.
        self.playing.is_some()
    }

    /// Gets the timestamp of the last time this [`Voice`] was used.
    ///
    /// If the voice has never been played, this will be the time it
    /// was created.
    pub fn last_played_at(&self) -> std::time::Instant {
        self.last_played_at
    }

    /// Gets the next set of generated samples.
    ///
    /// If the voice isn't currently active, an array of zeroes will
    /// be returned.
    pub fn next_frame(&mut self) -> [f64; CHANNELS] {
        if self.playing.is_some() {
            // TODO: Obviously this should be more interesting than a
            // mono sine wave. How should we handle things like
            // realtime parameters and complex waveforms?
            let value = self.phasor.next_phase();
            let value = (value * 2.0 * std::f64::consts::PI).sin();
            let value = value * self.velocity_f64;
            [value; CHANNELS]
        } else {
            [0.0; CHANNELS]
        }
    }
}
