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

    pub fn scale(&self, scale_type: crate::ScaleType) -> crate::Scale {
        crate::Scale::new(*self.inner, scale_type).unwrap()
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

    pub fn add_interval(&self, interval: &crate::Interval) -> Result<Self, crate::MusicError> {
        Ok(Self {
            inner: (*self.inner).clone().add_interval(interval)?.into_arc(),
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
}
