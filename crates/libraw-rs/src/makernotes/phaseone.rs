use crate::{impl_property, impl_serialize};

pub struct PhaseOne {
    inner: libraw_sys::libraw_p1_makernotes_t,
}

impl PhaseOne {
    pub fn new(value: libraw_sys::libraw_p1_makernotes_t) -> Self {
        PhaseOne { inner: value }
    }

    impl_property!(software, Software, Option<String>);
    impl_property!(system_type, SystemType, Option<String>);
    impl_property!(firmware_string, FirmwareString, Option<String>);
    impl_property!(system_model, SystemModel, Option<String>);
}

impl_serialize!(
    PhaseOne,
    [software, system_type, firmware_string, system_model]
);
