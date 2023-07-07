use image::{DynamicImage, ImageOutputFormat};
use std::io::Cursor;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn crop(bytes: &[u8], width: i32, height: i32) -> Result<Vec<u8>, JsValue> {
    let mut image = image::load_from_memory(&bytes)
        .map_err(|e| JsValue::from_str(&format!("Failed to load the image: {}", e)))?;
    let cropped_image = crop_image(&mut image, width, height)
        .map_err(|e| JsValue::from_str(&format!("Failed to crop the image: {}", e)))?;

    let mut cropped_buffer = Vec::new();
    let format = ImageOutputFormat::Jpeg(50);
    cropped_image
        .write_to(&mut Cursor::new(&mut cropped_buffer), format)
        .map_err(|e| JsValue::from_str(&format!("Failed to write the image: {}", e)))?;

    Ok(cropped_buffer)
}

fn crop_image(
    image: &mut DynamicImage,
    target_width: i32,
    target_height: i32,
) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    let image_width = image.width();
    let image_height = image.height();

    let aspect_ratio = target_width as f64 / target_height as f64;
    let image_aspect_ratio = image_width as f64 / image_height as f64;

    let (x, y, width, height) = if image_aspect_ratio > aspect_ratio {
        // Image is wider, crop horizontally
        let new_width = (image_height as f64 * aspect_ratio) as u32;
        let x = (image_width - new_width) / 2;

        (x, 0, new_width, image_height)
    } else {
        // Image is taller, crop vertically
        let new_height = (image_width as f64 / aspect_ratio) as u32;
        let y = (image_height - new_height) / 2;

        (0, y, image_width, new_height)
    };

    let cropped_image = image.crop_imm(x, y, width, height);

    Ok(cropped_image)
}
