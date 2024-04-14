//! Module for processing image files and creating images from color data.

mod color_map_utils;

use std::fs::File;
use std::io::Read;
use std::io::Cursor;
use flate2::read::GzDecoder;
use image::{ImageBuffer, Rgb, RgbImage};
use color_map_utils::map_block_color;
use color_map_utils::extract_colors_data;
use wasm_bindgen::prelude::*;

/// Processes an image file, extracts color data, and saves the resulting image.
///
/// # Arguments
///
/// * `input_path` - The path to the input image file.
/// * `output_path` - The path where the resulting image will be saved.
///
/// # Returns
///
/// A `Result` indicating success or failure. If successful, returns `Ok(())`.
/// If an error occurs, returns `Err` with an error message.
pub fn process_image_file(input_path: &str, output_path: &str) -> Result<(), String> {
    let file = File::open(input_path).map_err(|_| "Failed to open input file")?;
    let mut decoder = GzDecoder::new(file);
    let mut buffer = Vec::new();
    decoder.read_to_end(&mut buffer).map_err(|_| "Failed to read compressed data")?;
    let value = fastnbt::from_bytes(&buffer).map_err(|_| "Failed to parse NBT data")?;
    println!("value: {:?}", value);
    let color_data = extract_colors_data(&value).ok_or("No color data found in NBT")?;
    create_and_save_image(&color_data, output_path)
}

/// Creates an image from color data and saves it to the specified output path.
///
/// # Arguments
///
/// * `color_data` - A slice of bytes representing the color data.
/// * `output_path` - The path where the resulting image will be saved.
///
/// # Returns
///
/// A `Result` indicating success or failure. If successful, returns `Ok(())`.
/// If an error occurs, returns `Err` with an error message.
pub fn create_and_save_image(color_data: &[u8], output_path: &str) -> Result<(), String> {
    let image_size = compute_image_size(color_data.len())?;
    let image = build_image(color_data, image_size, image_size);
    image.save(output_path).map_err(|_| "Failed to save image".to_string())
}

/// Computes the size of the image based on the length of the color data.
///
/// # Arguments
///
/// * `data_length` - The length of the color data.
///
/// # Returns
///
/// A `Result` containing the computed image size if the data length is a perfect square.
/// If the data length is not a perfect square, returns an `Err` with an error message.
fn compute_image_size(data_length: usize) -> Result<usize, String> {
    let side_length = (data_length as f64).sqrt() as usize;
    if side_length * side_length != data_length {
        Err("Data does not contain a perfect square of pixels.".into())
    } else {
        Ok(side_length)
    }
}

/// Builds an RGB image from the provided color data.
///
/// # Arguments
///
/// * `data` - A slice of bytes representing the color data.
/// * `width` - The width of the resulting image.
/// * `height` - The height of the resulting image.
///
/// # Returns
///
/// An `RgbImage` built from the color data.
fn build_image(data: &[u8], width: usize, height: usize) -> RgbImage {
    let mut image: RgbImage = ImageBuffer::new(width as u32, height as u32);
    for (y, row) in data.chunks(width).enumerate() {
        for (x, &block_id) in row.iter().enumerate() {
            let color = map_block_color(block_id);
            image.put_pixel(x as u32, y as u32, Rgb([color.0, color.1, color.2]));
        }
    }
    image
}

/// Processes an image from memory, extracts color data, and returns the resulting image bytes.
///
/// # Arguments
///
/// * `input_data` - A slice of bytes representing the input image data.
///
/// # Returns
///
/// A `Result` containing a `Vec<u8>` of the resulting image bytes if successful.
/// If an error occurs, returns a `JsValue` with an error message.
#[wasm_bindgen]
pub fn process_image_from_memory(input_data: &[u8]) -> Result<Vec<u8>, JsValue> {
    let mut decoder = GzDecoder::new(input_data);
    let mut buffer = Vec::new();
    decoder.read_to_end(&mut buffer).map_err(|_| JsValue::from_str("Failed to read compressed data"))?;
    let value = fastnbt::from_bytes(&buffer).map_err(|_| JsValue::from_str("Failed to parse NBT data"))?;
    let color_data = extract_colors_data(&value).ok_or(JsValue::from_str("No color data found in NBT"))?;
    create_image_bytes(&color_data)
}

/// Creates image bytes from the provided color data.
///
/// # Arguments
///
/// * `color_data` - A slice of bytes representing the color data.
///
/// # Returns
///
/// A `Result` containing a `Vec<u8>` of the resulting image bytes if successful.
/// If an error occurs, returns a `JsValue` with an error message.
fn create_image_bytes(color_data: &[u8]) -> Result<Vec<u8>, JsValue> {
    let image_size = compute_image_size(color_data.len()).map_err(|e| JsValue::from_str(&e))?;
    let image = build_image(color_data, image_size, image_size);
    let mut image_bytes = Vec::new();
    let mut cursor = Cursor::new(&mut image_bytes);
    image.write_to(&mut cursor, image::ImageOutputFormat::Png).map_err(|_| JsValue::from_str("Failed to write image to buffer"))?;
    Ok(image_bytes)
}