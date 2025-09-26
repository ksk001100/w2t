use image::ImageFormat;
use js_sys::Array;
use std::io::Cursor;
use w2t_core::make_transparent;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn convert_to_transparent(image_data: &[u8], threshold: u8) -> Result<Vec<u8>, JsValue> {
    // 画像を読み込み
    let img = image::load_from_memory(image_data)
        .map_err(|e| JsValue::from_str(&format!("Failed to load image: {}", e)))?;

    // 共通ロジックを使用
    let transparent_img = make_transparent(&img, threshold);

    // PNGとしてエンコード
    let mut output = Vec::new();
    {
        let mut cursor = Cursor::new(&mut output);
        transparent_img
            .write_to(&mut cursor, ImageFormat::Png)
            .map_err(|e| JsValue::from_str(&format!("Failed to encode PNG: {}", e)))?;
    }

    Ok(output)
}

#[wasm_bindgen]
pub struct BatchResult {
    threshold: u8,
    data: Vec<u8>,
}

#[wasm_bindgen]
impl BatchResult {
    #[wasm_bindgen(getter)]
    pub fn threshold(&self) -> u8 {
        self.threshold
    }

    #[wasm_bindgen(getter)]
    pub fn data(&self) -> Vec<u8> {
        self.data.clone()
    }
}

#[wasm_bindgen]
pub fn convert_all_thresholds(
    image_data: &[u8],
    start: u8,
    end: u8,
    step: u8,
) -> Result<Array, JsValue> {
    if step == 0 {
        return Err(JsValue::from_str("Step cannot be zero"));
    }

    // 画像を読み込み
    let img = image::load_from_memory(image_data)
        .map_err(|e| JsValue::from_str(&format!("Failed to load image: {}", e)))?;

    let results = Array::new();
    let mut current = start;

    while current <= end {
        // 各閾値で変換
        let transparent_img = make_transparent(&img, current);

        // PNGとしてエンコード
        let mut output = Vec::new();
        {
            let mut cursor = Cursor::new(&mut output);
            transparent_img
                .write_to(&mut cursor, ImageFormat::Png)
                .map_err(|e| {
                    JsValue::from_str(&format!(
                        "Failed to encode PNG for threshold {}: {}",
                        current, e
                    ))
                })?;
        }

        let result = BatchResult {
            threshold: current,
            data: output,
        };

        results.push(&JsValue::from(result));

        if current == 255 || current > 255 - step {
            break;
        }
        current = (current as u16 + step as u16).min(255) as u8;
    }

    Ok(results)
}
