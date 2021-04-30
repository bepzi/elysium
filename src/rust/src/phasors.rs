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
/// Note that although the [`Iterator`] trait returns an
/// [`Option`], it is always safe to call `unwrap()` as it will
/// never return `None`.
///
/// # Examples
///
/// ```
/// # use crate::elysium::phasors::Phasor;
/// // A phasor wraps around once it reaches 1.0
/// let mut phasor = Phasor::new(2.0, 1.0);
/// assert_eq!(phasor.next(), Some(0.0));
/// assert_eq!(phasor.next(), Some(0.5));
/// assert_eq!(phasor.next(), Some(0.0));
///
/// // Create a phasor for audio sampling rates and frequencies
/// let mut phasor = Phasor::new(44100.0, 440.0);
/// let val = phasor.next().unwrap();
/// assert_eq!(val, 0.0);
/// assert_eq!(phasor.next(), Some(0.009977324263038548));
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
    pub fn get_sample_rate(&self) -> Frequency {
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

    /// Gets the current frequency.
    pub fn get_freq(&self) -> Frequency {
        self.freq
    }
}

impl Iterator for Phasor {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.phase;
        self.phase += self.phase_incr;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }
        Some(value)
    }
}

// =============================================================================

#[derive(Debug, Copy, Clone)]
pub struct Voice {
    phasor: Phasor,
    last_played_at: std::time::Instant,
    note: Option<MidiNote>,
    velocity: f64,
}

impl Voice {
    pub fn new(sample_rate: Frequency) -> Self {
        Self {
            phasor: Phasor::new(sample_rate, 0.0),
            last_played_at: std::time::Instant::now(),
            note: None,
            velocity: 0.0,
        }
    }

    pub fn start_playing(&mut self, note: MidiNote) {
        self.phasor.reset();
        self.phasor.set_freq(note.note.to_freq_f64());
        self.last_played_at = std::time::Instant::now();
        self.note = Some(note);
        self.velocity = u8::from(note.velocity) as f64 / u8::from(U7::MAX) as f64
    }

    pub fn stop_playing(&mut self) {
        self.note = None;
    }

    pub fn currently_playing(&self) -> Option<MidiNote> {
        self.note
    }

    pub fn last_played_at(&self) -> std::time::Instant {
        self.last_played_at
    }
}

impl Iterator for Voice {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.note.is_some() {
            // TODO: Obviously this should be more interesting than
            // just a sine wave. How should we handle things like
            // realtime parameters and complex waveforms?
            let value = self.phasor.next().unwrap();
            let value = (value * 2.0 * std::f64::consts::PI).sin();
            Some(value * self.velocity)
        } else {
            Some(0.0)
        }
    }
}
