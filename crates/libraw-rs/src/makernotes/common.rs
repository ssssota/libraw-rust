use libraw_sys::*;

use crate::impl_property;

pub struct Common {
    inner: libraw_metadata_common_t,
}

impl Common {
    pub fn new(value: libraw_metadata_common_t) -> Self {
        Common { inner: value }
    }

    impl_property!(flash_ec, FlashEC, Option<f32>);
    impl_property!(flash_gn, FlashGN, Option<f32>);
    impl_property!(camera_temperature, CameraTemperature, Option<f32>);
    impl_property!(sensor_temperature, SensorTemperature, Option<f32>);
    impl_property!(sensor_temperature2, SensorTemperature2, Option<f32>);
    impl_property!(lens_temperature, LensTemperature, Option<f32>);
    impl_property!(ambient_temperature, AmbientTemperature, Option<f32>);
    impl_property!(battery_temperature, BatteryTemperature, Option<f32>);
    impl_property!(
        exif_ambient_temperature,
        exifAmbientTemperature,
        Option<f32>
    );
    impl_property!(exif_humidity, exifHumidity, Option<f32>);
    impl_property!(exif_pressure, exifPressure, Option<f32>);
    impl_property!(exif_water_depth, exifWaterDepth, Option<f32>);
    impl_property!(exif_acceleration, exifAcceleration, Option<f32>);
    impl_property!(
        exif_camera_elevation_angle,
        exifCameraElevationAngle,
        Option<f32>
    );
    impl_property!(real_iso, real_ISO, Option<f32>);
    impl_property!(exif_exposure_index, exifExposureIndex, Option<f32>);
    impl_property!(color_space, ColorSpace, ushort);
    impl_property!(firmware, Option<String>);
    impl_property!(
        exposure_calibration_shift,
        ExposureCalibrationShift,
        Option<f32>
    );
    impl_property!(afdata, [libraw_afinfo_item_t; 4usize]);
    impl_property!(afcount, i32);
}
