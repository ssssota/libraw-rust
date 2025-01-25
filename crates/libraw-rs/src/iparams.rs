use std::borrow::Cow;

use crate::impl_property;

pub struct IParams {
    inner: libraw_sys::libraw_iparams_t,
}

impl IParams {
    pub(crate) fn new(inner: libraw_sys::libraw_iparams_t) -> Self {
        IParams { inner }
    }
    impl_property!(make, Option<String>);
    impl_property!(model, Option<String>);
    impl_property!(software, Option<String>);
    impl_property!(normalized_make, Option<String>);
    impl_property!(normalized_model, Option<String>);
    impl_property!(maker_index, u32);
    impl_property!(raw_count, u32);
    impl_property!(dng_version, u32);
    impl_property!(colors, i32);
    impl_property!(filters, u32);
    impl_property!(xtrans, [[i8; 6usize]; 6usize]);
    impl_property!(xtrans_abs, [[i8; 6usize]; 6usize]);
    impl_property!(cdesc, Option<String>);
    impl_property!(xmplen, u32);
    pub fn is_foveon(&self) -> bool {
        self.inner.is_foveon != 0
    }
    pub fn xmpdata(&self) -> Option<Cow<[i8]>> {
        if self.inner.xmpdata.is_null() {
            None
        } else {
            Some(unsafe {
                Cow::Borrowed(std::slice::from_raw_parts(
                    self.inner.xmpdata,
                    self.inner.xmplen as usize,
                ))
            })
        }
    }
}
