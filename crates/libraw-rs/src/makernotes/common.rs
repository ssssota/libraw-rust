use libraw_sys::*;

use crate::impl_property;

pub struct Common {
    inner: libraw_metadata_common_t,
}

impl Common {
    pub fn new(value: libraw_metadata_common_t) -> Self {
        Common { inner: value }
    }

    impl_property!(flash_ec, FlashEC, f32);
    impl_property!(flash_gn, FlashGN, f32);
    impl_property!(camera_temperature, CameraTemperature, f32);
    impl_property!(sensor_temperature, SensorTemperature, f32);
    impl_property!(sensor_temperature2, SensorTemperature2, f32);
    impl_property!(lens_temperature, LensTemperature, f32);
    impl_property!(ambient_temperature, AmbientTemperature, f32);
    impl_property!(battery_temperature, BatteryTemperature, f32);
    impl_property!(exif_ambient_temperature, exifAmbientTemperature, f32);
    impl_property!(exif_humidity, exifHumidity, f32);
    impl_property!(exif_pressure, exifPressure, f32);
    impl_property!(exif_water_depth, exifWaterDepth, f32);
    impl_property!(exif_acceleration, exifAcceleration, f32);
    impl_property!(exif_camera_elevation_angle, exifCameraElevationAngle, f32);
    impl_property!(real_iso, real_ISO, f32);
    impl_property!(exif_exposure_index, exifExposureIndex, f32);
    impl_property!(color_space, ColorSpace, ushort);
    impl_property!(firmware, Option<String>);
    impl_property!(exposure_calibration_shift, ExposureCalibrationShift, f32);
    impl_property!(afdata, [libraw_afinfo_item_t; 4usize]);
    impl_property!(afcount, i32);
}
