use napi_derive::napi;

#[napi]
pub fn supported_cameras() -> Vec<&'static str> {
    libraw_rs::camera_list()
}
