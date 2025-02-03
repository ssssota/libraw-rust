use crate::{impl_property, impl_serialize};

pub struct Hasselblad {
    inner: libraw_sys::libraw_hasselblad_makernotes_t,
}

impl Hasselblad {
    pub fn new(value: libraw_sys::libraw_hasselblad_makernotes_t) -> Self {
        Hasselblad { inner: value }
    }

    impl_property!(base_iso, BaseISO, i32);
    impl_property!(gain, Gain, f64);
    impl_property!(sensor, Sensor, [i8; 8]);
    impl_property!(sensor_unit, SensorUnit, [i8; 64]);
    impl_property!(host_body, HostBody, [i8; 64]);
    impl_property!(sensor_code, SensorCode, i32);
    impl_property!(sensor_sub_code, SensorSubCode, i32);
    impl_property!(coating_code, CoatingCode, i32);
    impl_property!(uncropped, uncropped, i32);
    impl_property!(
        capture_sequence_initiator,
        CaptureSequenceInitiator,
        [i8; 32]
    );
    impl_property!(sensor_unit_connector, SensorUnitConnector, [i8; 64]);
    impl_property!(format, format, i32);
    impl_property!(n_ifd_cm, nIFD_CM, [i32; 2]);
    impl_property!(recommended_crop, RecommendedCrop, [i32; 2]);
    impl_property!(mn_color_matrix, mnColorMatrix, [[f64; 3]; 4]);
}

impl_serialize!(
    Hasselblad,
    [
        base_iso,
        gain,
        sensor,
        // sensor_unit,
        // host_body,
        sensor_code,
        sensor_sub_code,
        coating_code,
        uncropped,
        capture_sequence_initiator,
        // sensor_unit_connector,
        format,
        n_ifd_cm,
        recommended_crop,
        mn_color_matrix
    ]
);
