use crate::{DurationBase, UniFfiTag};
use std::sync::Arc;
use uniffi::{FfiConverterArc, MetadataBuffer, RustBuffer};

trait IntoArc {
    fn into_arc(self) -> Arc<Self>;
}

impl<T> IntoArc for T {
    fn into_arc(self) -> Arc<Self> {
        Arc::new(self)
    }
}

#[cfg_attr(feature = "bindgen", uniffi::export)]
impl DurationBase {
    #[uniffi::method(name = "in_quarters")]
    pub fn ffi_in_quarters(self: Arc<Self>) -> f32 {
        self.in_quarters()
    }

    #[uniffi::constructor(name = "from_quarters")]
    pub fn ffi_from_quarters(value: f32) -> Self {
        DurationBase::from_quarters(value).unwrap()
    }

    #[uniffi::method(name = "in_whole")]
    pub fn ffi_in_whole(self: Self) -> f32 {
        self.in_whole()
    }

    #[uniffi::constructor(name = "from_whole")]
    pub fn ffi_from_whole(value: f32) -> Self {
        Self::from_whole(value).unwrap()
    }
}

unsafe impl FfiConverterArc<UniFfiTag> for DurationBase {
    type FfiType = RustBuffer;

    fn lower(obj: Arc<Self>) -> Self::FfiType {
        todo!()
    }

    fn try_lift(v: Self::FfiType) -> uniffi::Result<Arc<Self>> {
        todo!()
    }

    fn write(obj: Arc<Self>, buf: &mut Vec<u8>) {
        todo!()
    }

    fn try_read(buf: &mut &[u8]) -> uniffi::Result<Arc<Self>> {
        todo!()
    }

    const TYPE_ID_META: MetadataBuffer =
        ::uniffi::MetadataBuffer::from_code(::uniffi::metadata::codes::TYPE_INTERFACE)
            .concat_str("mutheors")
            .concat_str("DurationBase");
}

const UNIFFI_META_CONST_MUTHEORS_INTERFACE_DURATION_BASE: ::uniffi::MetadataBuffer =
    ::uniffi::MetadataBuffer::from_code(::uniffi::metadata::codes::INTERFACE)
        .concat_str("mutheors")
        .concat_str("DurationBase")
        .concat_long_str("DurationBase");

#[cfg_attr(feature = "bindgen", uniffi::export)]
pub fn get_duration_base() -> DurationBase {
    let a: <Arc<DurationBase> as ::uniffi::Lift<crate::UniFfiTag>>::FfiType;

    DurationBase::Quarter
}
