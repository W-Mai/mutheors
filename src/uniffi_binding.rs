trait IntoArc {
    fn into_arc(self) -> std::sync::Arc<Self>;
}

impl<T> IntoArc for T {
    fn into_arc(self) -> std::sync::Arc<Self> {
        std::sync::Arc::new(self)
    }
}

#[derive(uniffi::Object, Clone)]
struct Chord {
    inner: std::sync::Arc<crate::Chord>,
}

#[cfg_attr(feature = "bindgen", uniffi::export)]
impl Chord {
    pub fn quality(&self) -> crate::ChordQuality {
        self.inner.quality()
    }

    pub fn root(&self) -> Tuning {
        Tuning {
            inner: self.inner.root().into_arc(),
        }
    }

    pub fn with_root(&self, root: &Tuning) -> Self {
        Self {
            inner: (*self.inner).clone().with_root(*root.inner).into_arc(),
        }
    }

    pub fn with_octave(&self, octave: i8) -> Self {
        Self {
            inner: (*self.inner).clone().with_octave(octave).into_arc(),
        }
    }

    #[uniffi::constructor]
    pub fn new(root: &Tuning, quality: crate::ChordQuality) -> Result<Self, crate::MusicError> {
        Ok(Self {
            inner: crate::Chord::new(*root.inner, quality)?.into_arc(),
        })
    }

    /// Adding Extended interval
    pub fn with_extension(&self, tunings: Vec<std::sync::Arc<Tuning>>) -> Self {
        Self {
            inner: (*self.inner)
                .clone()
                .with_extension(&tunings.into_iter().map(|i| *i.inner).collect::<Vec<_>>())
                .into_arc(),
        }
    }

    pub fn invert(&self, inversion: &crate::Inversion) -> Self {
        let mut self_copy = (*self.inner).clone();
        self_copy.invert(*inversion);
        Self {
            inner: self_copy.into_arc(),
        }
    }

    pub fn intervals(&self) -> Vec<std::sync::Arc<Interval>> {
        self.inner
            .intervals()
            .into_iter()
            .map(|i| {
                Interval {
                    inner: i.into_arc(),
                }
                .into_arc()
            })
            .collect()
    }

    /// Getting Chord composition tones
    pub fn components(&self) -> Vec<std::sync::Arc<Tuning>> {
        self.inner
            .components()
            .into_iter()
            .map(|i| {
                Tuning {
                    inner: i.into_arc(),
                }
                .into_arc()
            })
            .collect()
    }

    pub fn simple(&self) -> Self {
        Self {
            inner: (*self.inner).clone().simple().into_arc(),
        }
    }

    pub fn function(&self, scale: &Scale) -> crate::ChordFunction {
        self.inner.function(&scale.inner)
    }

    #[uniffi::constructor]
    pub fn from_symbol(symbol: &str) -> Result<Self, crate::MusicError> {
        Ok(Self {
            inner: crate::Chord::from_symbol(symbol)?.into_arc(),
        })
    }

    pub fn in_scales(&self) -> Vec<std::sync::Arc<Scale>> {
        self.inner
            .in_scales()
            .into_iter()
            .map(|i| {
                Scale {
                    inner: i.into_arc(),
                }
                .into_arc()
            })
            .collect()
    }

    pub fn to_string(&self) -> String {
        self.inner.to_string()
    }

    pub fn add(&self, n: u8) -> Self {
        Self {
            inner: (*self.inner).clone().add(n).into_arc(),
        }
    }

    pub fn dom(&self, n: u8) -> Self {
        Self {
            inner: (*self.inner).clone().dom(n).into_arc(),
        }
    }

    pub fn maj(&self, n: u8) -> Self {
        Self {
            inner: (*self.inner).clone().maj(n).into_arc(),
        }
    }
}

#[derive(uniffi::Object, Clone)]
struct DurationBaseObject {
    inner: std::sync::Arc<crate::DurationBase>,
}

#[uniffi::export]
impl DurationBaseObject {
    pub fn in_quarters(&self) -> f32 {
        self.inner.in_quarters()
    }

    #[uniffi::constructor]
    pub fn rom_quarters(value: f32) -> Result<Self, crate::MusicError> {
        Ok(Self {
            inner: crate::DurationBase::from_quarters(value)?.into_arc(),
        })
    }

    pub fn in_whole(&self) -> f32 {
        self.inner.in_whole()
    }

    #[uniffi::constructor]
    pub fn from_whole(value: f32) -> Result<Self, crate::MusicError> {
        Ok(Self {
            inner: crate::DurationBase::from_whole(value)?.into_arc(),
        })
    }

    pub fn inner(&self) -> crate::DurationBase {
        *self.inner
    }
}

#[derive(uniffi::Object, Clone)]
struct Interval {
    inner: std::sync::Arc<crate::Interval>,
}

#[uniffi::export]
impl Interval {
    pub fn semitones(&self) -> i8 {
        self.inner.semitones()
    }

    pub fn semitones_mod(&self) -> i8 {
        self.inner.semitones().rem_euclid(12)
    }

    #[uniffi::constructor]
    pub fn from_quality_degree(
        quality: crate::IntervalQuality,
        degree: u8,
    ) -> Result<Self, crate::MusicError> {
        Ok(Self {
            inner: crate::Interval::from_quality_degree(quality, degree)?.into_arc(),
        })
    }

    #[uniffi::constructor]
    pub fn from_semitones(semitones: i8) -> Result<Self, crate::MusicError> {
        Ok(Self {
            inner: crate::Interval::from_semitones(semitones)?.into_arc(),
        })
    }

    #[uniffi::constructor]
    pub fn between(start: crate::PitchClass, end: crate::PitchClass) -> Self {
        Self {
            inner: crate::Interval::between(start, end).into_arc(),
        }
    }

    /// Interstitial inversion (e.g. Major 3rd -> minor 6th)
    pub fn invert(&self) -> Self {
        let self_copy = (*self.inner).clone().invert();
        Self {
            inner: self_copy.into_arc(),
        }
    }

    /// Consonance of the interval
    pub fn consonance(&self) -> crate::Consonance {
        self.inner.consonance()
    }

    /// Get the interval name
    /// e.g.
    /// - M3 (major third)
    /// - m6 (minor sixth)
    /// - Aug4 (augmented fourth)
    /// - Dim5 (diminished fifth)
    pub fn name(&self) -> String {
        self.inner.name()
    }
}

#[derive(uniffi::Object, Clone)]
struct Scale {
    inner: std::sync::Arc<crate::Scale>,
}

#[uniffi::export]
impl Scale {
    #[uniffi::constructor]
    pub fn new(root: &Tuning, scale_type: crate::ScaleType) -> Result<Self, crate::MusicError> {
        Ok(Self {
            inner: crate::Scale::new(*root.inner, scale_type)?.into_arc(),
        })
    }

    pub fn scale_type(&self) -> crate::ScaleType {
        self.inner.scale_type()
    }

    pub fn root(&self) -> Tuning {
        Tuning {
            inner: self.inner.root().into_arc(),
        }
    }

    pub fn generate_tunings(
        &self,
        octaves: u8,
    ) -> Result<Vec<std::sync::Arc<Tuning>>, crate::MusicError> {
        Ok(self
            .inner
            .generate_tunings(octaves)?
            .into_iter()
            .map(|i| {
                Tuning {
                    inner: i.into_arc(),
                }
                .into_arc()
            })
            .collect())
    }

    pub fn contains(&self, tuning: &Tuning) -> bool {
        self.inner.contains(&*tuning.inner)
    }

    pub fn degree(&self, degree: u8) -> Result<Tuning, crate::MusicError> {
        Ok(Tuning {
            inner: self.inner.degree(degree)?.into_arc(),
        })
    }

    pub fn interval_count(&self) -> u8 {
        self.inner.interval_count()
    }

    pub fn semitone_count(&self) -> u8 {
        self.inner.semitone_count()
    }

    pub fn chord(
        &self,
        degree: u8,
        quality: crate::ChordQuality,
    ) -> Result<Chord, crate::MusicError> {
        Ok(Chord {
            inner: self.inner.chord(degree, quality)?.into_arc(),
        })
    }

    pub fn degree_chord(&self, degree: u8) -> Result<Chord, crate::MusicError> {
        Ok(Chord {
            inner: self.inner.degree_chord(degree)?.into_arc(),
        })
    }

    pub fn characteristic_interval(&self) -> Option<std::sync::Arc<Interval>> {
        self.inner.characteristic_interval().map(|i| {
            Interval {
                inner: i.into_arc(),
            }
            .into_arc()
        })
    }

    pub fn characteristic_tuning(&self) -> Option<std::sync::Arc<Tuning>> {
        self.characteristic_interval()
            .and_then(|i| self.root().add_interval(&i).ok())
            .map(|i| i.into_arc())
    }

    pub fn modal_tonic(&self) -> Tuning {
        self.root()
    }
}

#[derive(uniffi::Object, Clone)]
struct Tuning {
    inner: std::sync::Arc<crate::Tuning>,
}

#[uniffi::export]
impl Tuning {
    #[uniffi::constructor]
    pub fn new(class: crate::PitchClass, octave: i8) -> Self {
        Self {
            inner: crate::Tuning::new(class, octave).into_arc(),
        }
    }

    pub fn with_octave(&self, octave: i8) -> Self {
        Self {
            inner: (*self.inner).clone().with_octave(octave).into_arc(),
        }
    }

    pub fn with_freq(&self, freq: f32) -> Self {
        Self {
            inner: (*self.inner).clone().with_freq(freq).into_arc(),
        }
    }

    pub fn frequency(&self) -> f32 {
        self.inner.frequency()
    }

    pub fn scale(&self, scale_type: crate::ScaleType) -> Scale {
        Scale::new(self, scale_type).unwrap()
    }

    pub fn common_chord(&self, degree: u8) -> Chord {
        Chord {
            inner: self.inner.common_chord(degree).into_arc(),
        }
    }

    pub fn class_semitones(&self) -> i8 {
        self.inner.class_semitones()
    }

    pub fn number(&self) -> i8 {
        self.inner.number()
    }

    pub fn add_interval(&self, interval: &Interval) -> Result<Self, crate::MusicError> {
        Ok(Self {
            inner: (*self.inner)
                .clone()
                .add_interval(&interval.inner)?
                .into_arc(),
        })
    }

    pub fn sharp(&self) -> Self {
        Self {
            inner: (*self.inner).clone().sharp().into_arc(),
        }
    }

    pub fn flat(&self) -> Self {
        Self {
            inner: (*self.inner).clone().flat().into_arc(),
        }
    }

    pub fn simple(&self) -> Self {
        Self {
            inner: (*self.inner).clone().simple().into_arc(),
        }
    }

    pub fn to_string(&self) -> String {
        self.inner.to_string()
    }
}
