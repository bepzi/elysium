const TWO_PI: f64 = 2.0 * std::f64::consts::PI;

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
/// # use crate::elysium::processor::phasor::Phasor;
/// // A phasor wraps around once it reaches 1.0
/// let mut phasor = Phasor::new(2.0, 1.0);
/// assert_eq!(phasor.next(), Some(0.5));
/// assert_eq!(phasor.next(), Some(0.0));
/// assert_eq!(phasor.next(), Some(0.5));
///
/// // Create a phasor for audio sampling rates and frequencies
/// let mut phasor = Phasor::new(44100.0, 440.0);
/// let val = phasor.next().unwrap();
/// assert_eq!(val, 0.009977324263038548);
///
/// // Shift from [0, 1) to [0, 2pi), then take the sine
/// let val = (val * 2.0 * std::f64::consts::PI).sin();
/// assert_eq!(val, 0.06264832417874368);
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

impl Default for Phasor {
    fn default() -> Self {
        Self::new(44100.0, 440.0)
    }
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
    /// `freq` is `< 0.0`, or if either are infinity or
    /// `NaN`. `freq` must be less than or equal to `sample_rate`.
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

    /// Gets the current sampling rate.
    pub fn get_sample_rate(&self) -> f64 {
        return self.sample_rate;
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
    pub fn get_freq(&self) -> f64 {
        return self.freq;
    }
}

impl Iterator for Phasor {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        self.phase += self.phase_incr;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }
        Some(self.phase)
    }
}

pub struct FnMutPhasor {
    pub phasor: Phasor,
    pub fn_mut: Box<dyn FnMut(f64) -> f64>,
}

impl Iterator for FnMutPhasor {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        Some((self.fn_mut)(self.phasor.next().unwrap()))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SinePhasor {
    pub phasor: Phasor,
}

impl Default for SinePhasor {
    fn default() -> Self {
        Self {
            phasor: Phasor::default(),
        }
    }
}

impl Iterator for SinePhasor {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        Some((self.phasor.next().unwrap() * TWO_PI).sin())
    }
}
