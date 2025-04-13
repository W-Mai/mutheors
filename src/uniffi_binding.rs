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

    pub fn root(&self) -> crate::Tuning {
        self.inner.root()
    }

    pub fn with_root(&self, root: &crate::Tuning) -> Self {
        Self {
            inner: (*self.inner).clone().with_root(*root).into_arc(),
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
