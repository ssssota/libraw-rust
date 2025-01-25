use libraw_sys::*;

use crate::impl_property;

pub struct Kodak {
    inner: libraw_kodak_makernotes_t,
}

impl Kodak {
    pub fn new(value: libraw_kodak_makernotes_t) -> Self {
        Kodak { inner: value }
    }

    impl_property!(black_level_top, BlackLevelTop, u16);
    impl_property!(black_level_bottom, BlackLevelBottom, u16);
    impl_property!(offset_left, i16);
    impl_property!(offset_top, i16);
    impl_property!(clip_black, clipBlack, u16);
    impl_property!(clip_white, clipWhite, u16);
    impl_property!(romm_cam_daylight, romm_camDaylight, [[f32; 3]; 3]);
    impl_property!(romm_cam_tungsten, romm_camTungsten, [[f32; 3]; 3]);
    impl_property!(romm_cam_fluorescent, romm_camFluorescent, [[f32; 3]; 3]);
    impl_property!(romm_cam_flash, romm_camFlash, [[f32; 3]; 3]);
    impl_property!(romm_cam_custom, romm_camCustom, [[f32; 3]; 3]);
    impl_property!(romm_cam_auto, romm_camAuto, [[f32; 3]; 3]);
    impl_property!(val_018_percent, val018percent, u16);
    impl_property!(val_100_percent, val100percent, u16);
    impl_property!(val_170_percent, val170percent, u16);
    impl_property!(maker_note_kodak_8a, MakerNoteKodak8a, i16);
    impl_property!(iso_calibration_gain, ISOCalibrationGain, f32);
    impl_property!(analog_iso, AnalogISO, f32);
}
