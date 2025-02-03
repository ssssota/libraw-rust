mod utils;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn supported_cameras() -> Vec<String> {
    libraw_rs::camera_list()
        .iter()
        .map(|c| c.to_string())
        .collect()
}

#[wasm_bindgen]
pub fn load(data: &[u8]) -> Result<LibRaw, String> {
    let internal = libraw_rs::LibRaw::open_buffer(data).map_err(|e| e.to_string())?;
    Ok(LibRaw { internal })
}

#[wasm_bindgen]
pub struct LibRaw {
    pub(crate) internal: libraw_rs::LibRaw,
}

#[wasm_bindgen]
impl LibRaw {
    #[wasm_bindgen]
    pub fn unpack(&self) -> Result<(), String> {
        self.internal.unpack().map_err(|e| e.to_string())
    }

    #[wasm_bindgen]
    pub fn idata(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.internal.idata()).unwrap()
    }

    #[wasm_bindgen]
    pub fn lens(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.internal.lens()).unwrap()
    }
}
