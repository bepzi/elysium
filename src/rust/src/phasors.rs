use wmidi::{Channel, Note, Velocity, U7};

/// Alias for a played MIDI note.
#[derive(Debug, Copy, Clone)]
pub struct MidiNote {
    pub channel: Channel,
    pub note: Note,
    pub velocity: Velocity,
}

// =============================================================================

/// Semantic alias for frequency.
pub type Frequency = f64;

/// A cyclic [phasor] that yields values from `[0, 1)` given a
/// sampling rate and a frequency.
///
/// # Examples
///
/// ```
/// # use crate::elysium::phasors::Phasor;
/// // A phasor wraps around once it reaches 1.0
/// let mut phasor = Phasor::new(2.0, 1.0);
/// assert_eq!(phasor.next_phase(), 0.0);
/// assert_eq!(phasor.next_phase(), 0.5);
/// assert_eq!(phasor.next_phase(), 0.0);
///
/// // Create a phasor for audio sampling rates and frequencies
/// let mut phasor = Phasor::new(44100.0, 440.0);
/// let val = phasor.next_phase();
/// assert_eq!(val, 0.0);
/// assert_eq!(phasor.next_phase(), 0.009977324263038548);
///
/// // Shift from [0, 1) to [0, 2pi), then take the sine
/// let val = (val * 2.0 * std::f64::consts::PI).sin();
/// assert_eq!(val, 0.0);
/// ```
///
/// [phasor]: https://en.wikipedia.org/wiki/Phasor
#[derive(Debug, Clone, Copy)]
pub struct Phasor {
    sample_rate: Frequency,
    freq: Frequency,
    phase: f64,
    phase_incr: f64,
}

impl Phasor {
    #[inline]
    fn update_phase_increment(&mut self) {
        assert!(self.freq >= 0.0 && self.freq.is_finite());
        assert!(self.sample_rate > 0.0 && self.sample_rate.is_finite());
        assert!(self.freq <= self.sample_rate);
        self.phase_incr = self.freq / self.sample_rate;
    }

    /// Constructs a new [`Phasor`].
    ///
    /// # Panics
    ///
    /// This function will panic if `sample_rate` is `<= 0.0` or
    /// `freq` is `< 0.0`, or if either are infinity or `NaN`. `freq`
    /// must also be less than or equal to `sample_rate`.
    pub fn new(sample_rate: Frequency, freq: Frequency) -> Self {
        let mut phasor = Self {
            sample_rate,
            freq,
            phase: 0.0,
            phase_incr: 0.0,
        };
        phasor.update_phase_increment();
        phasor
    }

    /// Resets the phasor's position to its initial value of `0.0`.
    ///
    /// This does not affect its current sampling rate or frequency.
    pub fn reset(&mut self) {
        self.phase = 0.0;
    }

    /// Gets the current sampling rate.
    pub fn sample_rate(&self) -> Frequency {
        self.sample_rate
    }

    /// Changes the sampling rate, but not the frequency.
    ///
    /// As `sample_rate` increases, the phasor will advance by
    /// smaller and smaller amounts.
    ///
    /// # Panics
    ///
    /// This function will panic if `sample_rate` is `<= 0.0`,
    /// infinity, or `NaN`, or if `sample_rate` is less than the
    /// frequency.
    pub fn set_sample_rate(&mut self, sample_rate: Frequency) {
        self.sample_rate = sample_rate;
        self.update_phase_increment();
    }

    /// Gets the current frequency.
    pub fn freq(&self) -> Frequency {
        self.freq
    }

    /// Changes the frequency, but not the sampling rate.
    ///
    /// As `freq` increases, the phasor will advance by larger and
    /// larger amounts.
    ///
    /// If `freq` is `0.0`, the phasor will not advance.
    ///
    /// # Panics
    ///
    /// This function will panic if `freq` is `< 0.0`, infinity,
    /// or `NaN`, or if `freq` is greater than the sampling rate.
    pub fn set_freq(&mut self, freq: Frequency) {
        self.freq = freq;
        self.update_phase_increment();
    }

    /// Gets the next phase value.
    pub fn next_phase(&mut self) -> f64 {
        let value = self.phase;
        self.phase += self.phase_incr;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }
        value
    }
}

// =============================================================================

const BEND_SEMITONES: i8 = 2;

#[derive(Debug, Copy, Clone)]
pub struct Voice<const CHANNELS: usize> {
    phasor: Phasor,
    last_played_at: std::time::Instant,
    playing: Option<MidiNote>,
    velocity_f64: f64,
    pitch_bend: f64,
}

impl<const CHANNELS: usize> Voice<CHANNELS> {
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

    pub fn set_pitch_bend(&mut self, pitch_bend: f64) {
        self.pitch_bend = pitch_bend;
        self.update_freq();
    }

    pub fn start_playing(&mut self, note: MidiNote) {
        self.last_played_at = std::time::Instant::now();
        self.playing = Some(note);
        // TODO: Should velocity sensing be logarithmic instead of linear?
        self.velocity_f64 = u8::from(note.velocity) as f64 / u8::from(U7::MAX) as f64;

        self.phasor.reset();
        self.update_freq();
    }

    pub fn stop_playing(&mut self) {
        self.playing = None;
    }

    pub fn currently_playing(&self) -> Option<MidiNote> {
        self.playing
    }

    pub fn last_played_at(&self) -> std::time::Instant {
        self.last_played_at
    }

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
