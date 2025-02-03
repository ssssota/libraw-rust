use crate::{impl_property, impl_serialize};

pub struct Common {
    inner: libraw_sys::libraw_metadata_common_t,
}

impl Common {
    pub fn new(value: libraw_sys::libraw_metadata_common_t) -> Self {
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
    impl_property!(color_space, ColorSpace, libraw_sys::ushort);
    impl_property!(firmware, Option<String>);
    impl_property!(
        exposure_calibration_shift,
        ExposureCalibrationShift,
        Option<f32>
    );
    impl_property!(afdata, [libraw_sys::libraw_afinfo_item_t; 4usize]);
    impl_property!(afcount, i32);
}

impl_serialize!(
    Common,
    [
        flash_ec,
        flash_gn,
        camera_temperature,
        sensor_temperature,
        sensor_temperature2,
        lens_temperature,
        ambient_temperature,
        battery_temperature,
        exif_ambient_temperature,
        exif_humidity,
        exif_pressure,
        exif_water_depth,
        exif_acceleration,
        exif_camera_elevation_angle,
        real_iso,
        exif_exposure_index,
        color_space,
        firmware,
        exposure_calibration_shift,
        // afdata,
        afcount
    ]
);
